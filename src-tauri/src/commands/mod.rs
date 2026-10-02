pub mod auth;

use std::sync::Arc;

use tauri::State;

use crate::domain::issue::{IssueFilter, IssueView};
use crate::domain::preference::Preferences;
use crate::domain::workspace::{ConnectionStatus, WorkspaceInfo};
use crate::infrastructure::database::Database;
use crate::infrastructure::error::AppError;

// ── Issue commands ──────────────────────────────────────────────────────

#[tauri::command]
pub fn list_issues(
    db: State<'_, Arc<Database>>,
    filter: Option<IssueFilter>,
) -> Result<Vec<IssueView>, AppError> {
    let f = filter.unwrap_or_default();
    db.list_issues(&f)
}

#[tauri::command]
pub fn get_issue(db: State<'_, Arc<Database>>, key: String) -> Result<Option<IssueView>, AppError> {
    db.get_issue(&key)
}

#[tauri::command]
pub fn get_issue_counts(db: State<'_, Arc<Database>>) -> Result<IssueCounts, AppError> {
    use crate::domain::issue::IssueStatus;

    let total = db.count_issues(None)?;
    let todo = db.count_issues(Some(&IssueStatus::Todo))?;
    let in_progress = db.count_issues(Some(&IssueStatus::InProgress))?;
    let review = db.count_issues(Some(&IssueStatus::Review))?;
    let done = db.count_issues(Some(&IssueStatus::Done))?;

    Ok(IssueCounts {
        total,
        todo,
        in_progress,
        review,
        done,
    })
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueCounts {
    pub total: u32,
    pub todo: u32,
    pub in_progress: u32,
    pub review: u32,
    pub done: u32,
}

// ── Preference commands ─────────────────────────────────────────────────

#[tauri::command]
pub fn get_preferences(db: State<'_, Arc<Database>>) -> Result<Preferences, AppError> {
    db.load_preferences()
}

#[tauri::command]
pub fn save_preferences(db: State<'_, Arc<Database>>, prefs: Preferences) -> Result<(), AppError> {
    db.save_preferences(&prefs)
}

// ── Workspace commands ──────────────────────────────────────────────────

#[tauri::command]
pub fn get_workspace_info(db: State<'_, Arc<Database>>) -> Result<WorkspaceInfo, AppError> {
    let issue_count = db.count_issues(None)?;
    let last_synced = db
        .get_workspace_meta("last_synced_at")?
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc));
    let user_name = db.get_workspace_meta("user_display_name")?;
    let base_url = db.get_workspace_meta("jira_base_url")?;

    let status = if base_url.is_some() {
        ConnectionStatus::Connected
    } else {
        ConnectionStatus::Disconnected
    };

    Ok(WorkspaceInfo {
        connection_status: status,
        last_synced_at: last_synced,
        issue_count,
        user_display_name: user_name,
        jira_base_url: base_url,
    })
}

// ── Seed command (development only) ─────────────────────────────────────

#[tauri::command]
pub fn seed_mock_data(db: State<'_, Arc<Database>>) -> Result<String, AppError> {
    use crate::domain::issue::{Issue, IssuePriority, IssueStatus};
    use chrono::{Duration, Utc};

    let now = Utc::now();
    let mock_issues = vec![
        Issue {
            id: uuid::Uuid::new_v4().to_string(),
            key: "TPT-7042".to_string(),
            summary: "Update Snowplow tracking".to_string(),
            description: Some(
                "Update the Snowplow tracking implementation for the new analytics pipeline."
                    .to_string(),
            ),
            status: IssueStatus::InProgress,
            priority: IssuePriority::High,
            project_key: "TPT".to_string(),
            project_name: "E-commerce".to_string(),
            assignee: Some("Alchie".to_string()),
            reporter: Some("PM".to_string()),
            labels: vec!["analytics".to_string(), "tracking".to_string()],
            sprint: Some("Sprint 42".to_string()),
            due_date: Some(now.date_naive()),
            created_at: now - Duration::days(3),
            updated_at: now - Duration::minutes(18),
            synced_at: now,
            web_url: Some("https://jira.example.com/browse/TPT-7042".to_string()),
        },
        Issue {
            id: uuid::Uuid::new_v4().to_string(),
            key: "PRCM-2579".to_string(),
            summary: "Checkout component update".to_string(),
            description: Some(
                "Refactor the checkout component for the new design system.".to_string(),
            ),
            status: IssueStatus::Review,
            priority: IssuePriority::Medium,
            project_key: "PRCM".to_string(),
            project_name: "Funnels".to_string(),
            assignee: Some("Alchie".to_string()),
            reporter: Some("Lead".to_string()),
            labels: vec!["frontend".to_string()],
            sprint: Some("Sprint 42".to_string()),
            due_date: None,
            created_at: now - Duration::days(5),
            updated_at: now - Duration::minutes(42),
            synced_at: now,
            web_url: Some("https://jira.example.com/browse/PRCM-2579".to_string()),
        },
        Issue {
            id: uuid::Uuid::new_v4().to_string(),
            key: "TPT-7101".to_string(),
            summary: "Fix mobile funnel issue".to_string(),
            description: Some(
                "Mobile funnel flow breaks on iOS Safari with specific viewport sizes.".to_string(),
            ),
            status: IssueStatus::Todo,
            priority: IssuePriority::High,
            project_key: "TPT".to_string(),
            project_name: "E-commerce".to_string(),
            assignee: Some("Alchie".to_string()),
            reporter: Some("QA".to_string()),
            labels: vec!["bug".to_string(), "mobile".to_string()],
            sprint: Some("Sprint 42".to_string()),
            due_date: Some((now + Duration::days(1)).date_naive()),
            created_at: now - Duration::days(1),
            updated_at: now - Duration::minutes(67),
            synced_at: now,
            web_url: Some("https://jira.example.com/browse/TPT-7101".to_string()),
        },
        Issue {
            id: uuid::Uuid::new_v4().to_string(),
            key: "TPT-6981".to_string(),
            summary: "Update product selector".to_string(),
            description: Some(
                "Align the product selector component with the latest design tokens.".to_string(),
            ),
            status: IssueStatus::InProgress,
            priority: IssuePriority::Medium,
            project_key: "TPT".to_string(),
            project_name: "Presells".to_string(),
            assignee: Some("Alchie".to_string()),
            reporter: Some("Designer".to_string()),
            labels: vec!["frontend".to_string(), "design-system".to_string()],
            sprint: Some("Sprint 41".to_string()),
            due_date: None,
            created_at: now - Duration::days(7),
            updated_at: now - Duration::minutes(125),
            synced_at: now,
            web_url: Some("https://jira.example.com/browse/TPT-6981".to_string()),
        },
        Issue {
            id: uuid::Uuid::new_v4().to_string(),
            key: "PRCM-2601".to_string(),
            summary: "Add A/B test variant for pricing page".to_string(),
            description: None,
            status: IssueStatus::Todo,
            priority: IssuePriority::Medium,
            project_key: "PRCM".to_string(),
            project_name: "Funnels".to_string(),
            assignee: Some("Alchie".to_string()),
            reporter: Some("PM".to_string()),
            labels: vec!["experiment".to_string()],
            sprint: Some("Sprint 43".to_string()),
            due_date: Some((now + Duration::days(5)).date_naive()),
            created_at: now - Duration::hours(6),
            updated_at: now - Duration::hours(2),
            synced_at: now,
            web_url: Some("https://jira.example.com/browse/PRCM-2601".to_string()),
        },
        Issue {
            id: uuid::Uuid::new_v4().to_string(),
            key: "TPT-7110".to_string(),
            summary: "Performance audit for landing pages".to_string(),
            description: Some(
                "Run Lighthouse CI and fix any regressions from the last release.".to_string(),
            ),
            status: IssueStatus::Todo,
            priority: IssuePriority::Low,
            project_key: "TPT".to_string(),
            project_name: "E-commerce".to_string(),
            assignee: Some("Alchie".to_string()),
            reporter: Some("Tech Lead".to_string()),
            labels: vec!["performance".to_string()],
            sprint: None,
            due_date: None,
            created_at: now - Duration::hours(2),
            updated_at: now - Duration::hours(1),
            synced_at: now,
            web_url: Some("https://jira.example.com/browse/TPT-7110".to_string()),
        },
    ];

    db.clear_issues()?;
    for issue in &mock_issues {
        db.upsert_issue(issue)?;
    }

    db.set_workspace_meta("user_display_name", "Alchie")?;
    db.set_workspace_meta("last_synced_at", &now.to_rfc3339())?;

    Ok(format!("Seeded {} mock issues", mock_issues.len()))
}
