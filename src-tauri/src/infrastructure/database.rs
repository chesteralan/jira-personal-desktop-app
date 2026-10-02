use std::path::Path;
use std::sync::Mutex;

use rusqlite::Connection;
use tracing::info;

use crate::domain::issue::{Issue, IssueFilter, IssuePriority, IssueStatus, IssueView};
use crate::domain::preference::Preferences;
use crate::infrastructure::error::AppError;

/// Thread-safe wrapper around a SQLite connection.
pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    /// Open (or create) the database at the given path and run migrations.
    pub fn open(path: &Path) -> Result<Self, AppError> {
        let conn = Connection::open(path)?;

        // WAL mode for concurrent readers during sync.
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "foreign_keys", "ON")?;

        let db = Self {
            conn: Mutex::new(conn),
        };
        db.migrate()?;
        info!("Database opened at {}", path.display());
        Ok(db)
    }

    /// Run all pending schema migrations.
    fn migrate(&self) -> Result<(), AppError> {
        let conn = self.conn.lock().expect("database lock poisoned");

        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS issues (
                id              TEXT PRIMARY KEY,
                key             TEXT NOT NULL UNIQUE,
                summary         TEXT NOT NULL,
                description     TEXT,
                status          TEXT NOT NULL DEFAULT 'unknown',
                priority        TEXT NOT NULL DEFAULT 'unknown',
                project_key     TEXT NOT NULL,
                project_name    TEXT NOT NULL,
                assignee        TEXT,
                reporter        TEXT,
                labels          TEXT NOT NULL DEFAULT '[]',
                sprint          TEXT,
                due_date        TEXT,
                created_at      TEXT NOT NULL,
                updated_at      TEXT NOT NULL,
                synced_at       TEXT NOT NULL,
                web_url         TEXT
            );

            CREATE INDEX IF NOT EXISTS idx_issues_status ON issues(status);
            CREATE INDEX IF NOT EXISTS idx_issues_priority ON issues(priority);
            CREATE INDEX IF NOT EXISTS idx_issues_project ON issues(project_key);
            CREATE INDEX IF NOT EXISTS idx_issues_updated ON issues(updated_at);

            CREATE TABLE IF NOT EXISTS preferences (
                key   TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS workspace (
                key   TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS saved_boards (
                board_id     INTEGER PRIMARY KEY,
                name         TEXT NOT NULL,
                board_type   TEXT NOT NULL,
                project_key  TEXT
            );
            ",
        )?;

        info!("Database migrations applied");
        Ok(())
    }

    // ── Issues ──────────────────────────────────────────────────────────

    /// Insert or replace an issue (upsert by key).
    pub fn upsert_issue(&self, issue: &Issue) -> Result<(), AppError> {
        let conn = self.conn.lock().expect("database lock poisoned");
        let labels_json = serde_json::to_string(&issue.labels)?;

        conn.execute(
            "INSERT INTO issues (id, key, summary, description, status, priority,
                                 project_key, project_name, assignee, reporter,
                                 labels, sprint, due_date, created_at, updated_at,
                                 synced_at, web_url)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)
             ON CONFLICT(key) DO UPDATE SET
                 summary=excluded.summary, description=excluded.description,
                 status=excluded.status, priority=excluded.priority,
                 project_key=excluded.project_key, project_name=excluded.project_name,
                 assignee=excluded.assignee, reporter=excluded.reporter,
                 labels=excluded.labels, sprint=excluded.sprint,
                 due_date=excluded.due_date, created_at=excluded.created_at,
                 updated_at=excluded.updated_at, synced_at=excluded.synced_at,
                 web_url=excluded.web_url",
            rusqlite::params![
                issue.id,
                issue.key,
                issue.summary,
                issue.description,
                serde_json::to_string(&issue.status)?.trim_matches('"'),
                serde_json::to_string(&issue.priority)?.trim_matches('"'),
                issue.project_key,
                issue.project_name,
                issue.assignee,
                issue.reporter,
                labels_json,
                issue.sprint,
                issue.due_date.map(|d| d.to_string()),
                issue.created_at.to_rfc3339(),
                issue.updated_at.to_rfc3339(),
                issue.synced_at.to_rfc3339(),
                issue.web_url,
            ],
        )?;
        Ok(())
    }

    /// Query issues with optional filters.
    pub fn list_issues(&self, filter: &IssueFilter) -> Result<Vec<IssueView>, AppError> {
        let conn = self.conn.lock().expect("database lock poisoned");

        let mut sql = String::from(
            "SELECT id, key, summary, status, priority, project_key, project_name,
                    assignee, labels, sprint, due_date, updated_at, web_url
             FROM issues WHERE 1=1",
        );
        let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

        if let Some(ref statuses) = filter.status {
            if !statuses.is_empty() {
                let placeholders: Vec<String> = statuses
                    .iter()
                    .enumerate()
                    .map(|(i, _)| format!("?{}", params.len() + i + 1))
                    .collect();
                sql.push_str(&format!(" AND status IN ({})", placeholders.join(",")));
                for s in statuses {
                    params.push(Box::new(
                        serde_json::to_string(s)
                            .unwrap_or_default()
                            .trim_matches('"')
                            .to_string(),
                    ));
                }
            }
        }

        if let Some(ref priorities) = filter.priority {
            if !priorities.is_empty() {
                let placeholders: Vec<String> = priorities
                    .iter()
                    .enumerate()
                    .map(|(i, _)| format!("?{}", params.len() + i + 1))
                    .collect();
                sql.push_str(&format!(" AND priority IN ({})", placeholders.join(",")));
                for p in priorities {
                    params.push(Box::new(
                        serde_json::to_string(p)
                            .unwrap_or_default()
                            .trim_matches('"')
                            .to_string(),
                    ));
                }
            }
        }

        if let Some(ref project) = filter.project_key {
            params.push(Box::new(project.clone()));
            sql.push_str(&format!(" AND project_key = ?{}", params.len()));
        }

        if let Some(ref sprint) = filter.sprint {
            params.push(Box::new(sprint.clone()));
            sql.push_str(&format!(" AND sprint = ?{}", params.len()));
        }

        if let Some(ref label) = filter.label {
            params.push(Box::new(format!("%\"{label}\"%")));
            sql.push_str(&format!(" AND labels LIKE ?{}", params.len()));
        }

        if let Some(ref search) = filter.search {
            let term = format!("%{search}%");
            params.push(Box::new(term.clone()));
            let idx = params.len();
            params.push(Box::new(term));
            sql.push_str(&format!(
                " AND (summary LIKE ?{} OR key LIKE ?{})",
                idx,
                idx + 1
            ));
        }

        sql.push_str(" ORDER BY updated_at DESC");

        let param_refs: Vec<&dyn rusqlite::types::ToSql> =
            params.iter().map(|p| p.as_ref()).collect();
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(param_refs.as_slice(), |row| {
            let status_str: String = row.get(3)?;
            let priority_str: String = row.get(4)?;
            let labels_str: String = row.get(8)?;
            let due_date_str: Option<String> = row.get(10)?;
            let updated_str: String = row.get(11)?;

            Ok(IssueView {
                id: row.get(0)?,
                key: row.get(1)?,
                summary: row.get(2)?,
                status: IssueStatus::from_jira(&status_str),
                priority: IssuePriority::from_jira(&priority_str),
                project_key: row.get(5)?,
                project_name: row.get(6)?,
                assignee: row.get(7)?,
                labels: serde_json::from_str(&labels_str).unwrap_or_default(),
                sprint: row.get(9)?,
                due_date: due_date_str.and_then(|s| s.parse().ok()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&updated_str)
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(|_| chrono::Utc::now()),
                web_url: row.get(12)?,
            })
        })?;

        let mut issues = Vec::new();
        for row in rows {
            issues.push(row?);
        }
        Ok(issues)
    }

    /// Get a single issue by key.
    pub fn get_issue(&self, key: &str) -> Result<Option<IssueView>, AppError> {
        let filter = IssueFilter {
            search: Some(key.to_string()),
            ..Default::default()
        };
        let mut results = self.list_issues(&filter)?;
        // Find exact key match
        if let Some(pos) = results.iter().position(|i| i.key == key) {
            Ok(Some(results.swap_remove(pos)))
        } else {
            Ok(None)
        }
    }

    /// Count issues, optionally filtered by status.
    pub fn count_issues(&self, status: Option<&IssueStatus>) -> Result<u32, AppError> {
        let conn = self.conn.lock().expect("database lock poisoned");
        let count: u32 = match status {
            Some(s) => {
                let status_str = serde_json::to_string(s)
                    .unwrap_or_default()
                    .trim_matches('"')
                    .to_string();
                conn.query_row(
                    "SELECT COUNT(*) FROM issues WHERE status = ?1",
                    [&status_str],
                    |row| row.get(0),
                )?
            }
            None => conn.query_row("SELECT COUNT(*) FROM issues", [], |row| row.get(0))?,
        };
        Ok(count)
    }

    /// Delete all issues (used before full re-sync).
    pub fn clear_issues(&self) -> Result<(), AppError> {
        let conn = self.conn.lock().expect("database lock poisoned");
        conn.execute("DELETE FROM issues", [])?;
        Ok(())
    }

    /// List all issue keys currently in the database.
    pub fn list_all_issue_keys(&self) -> Result<Vec<String>, AppError> {
        let conn = self.conn.lock().expect("database lock poisoned");
        let mut stmt = conn.prepare("SELECT key FROM issues")?;
        let keys = stmt
            .query_map([], |row| row.get(0))?
            .filter_map(|r| r.ok())
            .collect();
        Ok(keys)
    }

    /// Delete a single issue by its key.
    pub fn delete_issue_by_key(&self, key: &str) -> Result<bool, AppError> {
        let conn = self.conn.lock().expect("database lock poisoned");
        let deleted = conn.execute("DELETE FROM issues WHERE key = ?1", [key])?;
        Ok(deleted > 0)
    }

    // ── Preferences ─────────────────────────────────────────────────────

    /// Load preferences, returning defaults if none are stored.
    pub fn load_preferences(&self) -> Result<Preferences, AppError> {
        let conn = self.conn.lock().expect("database lock poisoned");
        let json: Option<String> = conn
            .query_row(
                "SELECT value FROM preferences WHERE key = 'user_preferences'",
                [],
                |row| row.get(0),
            )
            .ok();

        match json {
            Some(data) => Ok(serde_json::from_str(&data)?),
            None => Ok(Preferences::default()),
        }
    }

    /// Persist preferences.
    pub fn save_preferences(&self, prefs: &Preferences) -> Result<(), AppError> {
        let conn = self.conn.lock().expect("database lock poisoned");
        let json = serde_json::to_string(prefs)?;
        conn.execute(
            "INSERT INTO preferences (key, value) VALUES ('user_preferences', ?1)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            [&json],
        )?;
        Ok(())
    }

    // ── Workspace metadata ──────────────────────────────────────────────

    /// Get a workspace metadata value by key.
    pub fn get_workspace_meta(&self, key: &str) -> Result<Option<String>, AppError> {
        let conn = self.conn.lock().expect("database lock poisoned");
        let value: Option<String> = conn
            .query_row("SELECT value FROM workspace WHERE key = ?1", [key], |row| {
                row.get(0)
            })
            .ok();
        Ok(value)
    }

    /// Set a workspace metadata value.
    pub fn set_workspace_meta(&self, key: &str, value: &str) -> Result<(), AppError> {
        let conn = self.conn.lock().expect("database lock poisoned");
        conn.execute(
            "INSERT INTO workspace (key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            [key, value],
        )?;
        Ok(())
    }

    // ── Saved boards ────────────────────────────────────────────────

    /// Save a board for quick access.
    pub fn save_board(&self, board: &crate::domain::board::SavedBoard) -> Result<(), AppError> {
        let conn = self.conn.lock().expect("database lock poisoned");
        conn.execute(
            "INSERT OR REPLACE INTO saved_boards (board_id, name, board_type, project_key)
             VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![
                board.board_id,
                board.name,
                board.board_type,
                board.project_key
            ],
        )?;
        Ok(())
    }

    /// Remove a saved board.
    pub fn unsave_board(&self, board_id: u32) -> Result<bool, AppError> {
        let conn = self.conn.lock().expect("database lock poisoned");
        let deleted = conn.execute("DELETE FROM saved_boards WHERE board_id = ?1", [board_id])?;
        Ok(deleted > 0)
    }

    /// List all saved boards.
    pub fn list_saved_boards(&self) -> Result<Vec<crate::domain::board::SavedBoard>, AppError> {
        let conn = self.conn.lock().expect("database lock poisoned");
        let mut stmt =
            conn.prepare("SELECT board_id, name, board_type, project_key FROM saved_boards")?;
        let boards = stmt
            .query_map([], |row| {
                Ok(crate::domain::board::SavedBoard {
                    board_id: row.get(0)?,
                    name: row.get(1)?,
                    board_type: row.get(2)?,
                    project_key: row.get(3)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();
        Ok(boards)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::issue::{Issue, IssuePriority, IssueStatus};
    use chrono::Utc;

    fn test_db() -> Database {
        let conn = Connection::open_in_memory().unwrap();
        conn.pragma_update(None, "journal_mode", "WAL").unwrap();
        let db = Database {
            conn: Mutex::new(conn),
        };
        db.migrate().unwrap();
        db
    }

    fn sample_issue(key: &str) -> Issue {
        Issue {
            id: uuid::Uuid::new_v4().to_string(),
            key: key.to_string(),
            summary: format!("Test issue {key}"),
            description: None,
            status: IssueStatus::InProgress,
            priority: IssuePriority::High,
            project_key: "TPT".to_string(),
            project_name: "E-commerce".to_string(),
            assignee: Some("Alchie".to_string()),
            reporter: Some("Reporter".to_string()),
            labels: vec!["bug".to_string()],
            sprint: Some("Sprint 42".to_string()),
            due_date: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            synced_at: Utc::now(),
            web_url: Some("https://jira.example.com/browse/TPT-1".to_string()),
        }
    }

    #[test]
    fn upsert_and_list_issues() {
        let db = test_db();
        let issue = sample_issue("TPT-1");
        db.upsert_issue(&issue).unwrap();

        let results = db.list_issues(&IssueFilter::default()).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].key, "TPT-1");
        assert_eq!(results[0].summary, "Test issue TPT-1");
    }

    #[test]
    fn upsert_updates_existing() {
        let db = test_db();
        let mut issue = sample_issue("TPT-2");
        db.upsert_issue(&issue).unwrap();

        issue.summary = "Updated summary".to_string();
        db.upsert_issue(&issue).unwrap();

        let results = db.list_issues(&IssueFilter::default()).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].summary, "Updated summary");
    }

    #[test]
    fn filter_by_status() {
        let db = test_db();
        let mut issue1 = sample_issue("TPT-3");
        issue1.status = IssueStatus::Todo;
        db.upsert_issue(&issue1).unwrap();

        let mut issue2 = sample_issue("TPT-4");
        issue2.status = IssueStatus::InProgress;
        db.upsert_issue(&issue2).unwrap();

        let filter = IssueFilter {
            status: Some(vec![IssueStatus::Todo]),
            ..Default::default()
        };
        let results = db.list_issues(&filter).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].key, "TPT-3");
    }

    #[test]
    fn count_issues_works() {
        let db = test_db();
        db.upsert_issue(&sample_issue("TPT-5")).unwrap();
        db.upsert_issue(&sample_issue("TPT-6")).unwrap();

        assert_eq!(db.count_issues(None).unwrap(), 2);
        assert_eq!(db.count_issues(Some(&IssueStatus::InProgress)).unwrap(), 2);
        assert_eq!(db.count_issues(Some(&IssueStatus::Done)).unwrap(), 0);
    }

    #[test]
    fn preferences_round_trip() {
        let db = test_db();
        let prefs = db.load_preferences().unwrap();
        assert_eq!(prefs.sync_interval_secs, 300);

        let mut updated = prefs;
        updated.sync_interval_secs = 60;
        db.save_preferences(&updated).unwrap();

        let loaded = db.load_preferences().unwrap();
        assert_eq!(loaded.sync_interval_secs, 60);
    }

    #[test]
    fn workspace_meta_round_trip() {
        let db = test_db();
        assert!(db.get_workspace_meta("last_sync").unwrap().is_none());

        db.set_workspace_meta("last_sync", "2025-01-01T00:00:00Z")
            .unwrap();
        assert_eq!(
            db.get_workspace_meta("last_sync").unwrap().as_deref(),
            Some("2025-01-01T00:00:00Z")
        );
    }

    #[test]
    fn clear_issues_removes_all() {
        let db = test_db();
        db.upsert_issue(&sample_issue("TPT-7")).unwrap();
        db.upsert_issue(&sample_issue("TPT-8")).unwrap();
        assert_eq!(db.count_issues(None).unwrap(), 2);

        db.clear_issues().unwrap();
        assert_eq!(db.count_issues(None).unwrap(), 0);
    }

    #[test]
    fn list_all_issue_keys_returns_keys() {
        let db = test_db();
        db.upsert_issue(&sample_issue("TPT-10")).unwrap();
        db.upsert_issue(&sample_issue("TPT-11")).unwrap();

        let mut keys = db.list_all_issue_keys().unwrap();
        keys.sort();
        assert_eq!(keys, vec!["TPT-10", "TPT-11"]);
    }

    #[test]
    fn delete_issue_by_key_removes_one() {
        let db = test_db();
        db.upsert_issue(&sample_issue("TPT-12")).unwrap();
        db.upsert_issue(&sample_issue("TPT-13")).unwrap();

        assert!(db.delete_issue_by_key("TPT-12").unwrap());
        assert!(!db.delete_issue_by_key("TPT-99").unwrap());

        let keys = db.list_all_issue_keys().unwrap();
        assert_eq!(keys, vec!["TPT-13"]);
    }

    #[test]
    fn saved_boards_crud() {
        use crate::domain::board::SavedBoard;

        let db = test_db();
        assert!(db.list_saved_boards().unwrap().is_empty());

        let board = SavedBoard {
            board_id: 42,
            name: "Sprint Board".to_string(),
            board_type: "scrum".to_string(),
            project_key: Some("TPT".to_string()),
        };
        db.save_board(&board).unwrap();

        let boards = db.list_saved_boards().unwrap();
        assert_eq!(boards.len(), 1);
        assert_eq!(boards[0].board_id, 42);
        assert_eq!(boards[0].name, "Sprint Board");

        assert!(db.unsave_board(42).unwrap());
        assert!(!db.unsave_board(99).unwrap());
        assert!(db.list_saved_boards().unwrap().is_empty());
    }
}
