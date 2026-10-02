//! Jira Cloud REST API client supporting Basic (API-token) and Bearer (OAuth) auth.
//! Credentials are never logged, stored in SQLite, or sent to the frontend.

use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use tracing::{debug, info};

use super::types::{
    JiraBoardListResponse, JiraLegacySearchResponse, JiraSearchResponse, JiraTransitionsResponse,
    JiraUser,
};
use crate::domain::board::{Board, IssueTransition};
use crate::domain::issue::{Issue, IssuePriority, IssueStatus};
use crate::infrastructure::error::AppError;

/// Fields requested from Jira search to keep payloads small.
const SEARCH_FIELDS: &str =
    "summary,description,status,priority,project,assignee,reporter,labels,created,updated,duedate,sprint";

pub struct JiraClient {
    client: Client,
    /// For Basic auth: `https://yourorg.atlassian.net`
    /// For OAuth: `https://api.atlassian.com/ex/jira/{cloudId}`
    base_url: String,
}

impl JiraClient {
    /// Create a client using API-token (Basic) authentication.
    pub fn new(base_url: &str, email: &str, api_token: &str) -> Result<Self, AppError> {
        let auth_value = base64_encode(&format!("{email}:{api_token}"));
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Basic {auth_value}"))
                .map_err(|e| AppError::Internal(format!("Invalid auth header: {e}")))?,
        );
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = Client::builder()
            .default_headers(headers)
            .user_agent("JiraPersonal/0.1")
            .build()
            .map_err(|e| AppError::Internal(format!("HTTP client error: {e}")))?;

        let base = base_url.trim_end_matches('/').to_string();
        info!("Jira client configured for {} (Basic)", base);

        Ok(Self {
            client,
            base_url: base,
        })
    }

    /// Create a client using OAuth 2.0 Bearer token.
    /// `cloud_id` is used to construct the API base URL.
    pub fn with_oauth(access_token: &str, cloud_id: &str) -> Result<Self, AppError> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {access_token}"))
                .map_err(|e| AppError::Internal(format!("Invalid auth header: {e}")))?,
        );
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = Client::builder()
            .default_headers(headers)
            .user_agent("JiraPersonal/0.1")
            .build()
            .map_err(|e| AppError::Internal(format!("HTTP client error: {e}")))?;

        let base_url = format!("https://api.atlassian.com/ex/jira/{cloud_id}");
        info!("Jira client configured for cloud {} (OAuth)", cloud_id);

        Ok(Self { client, base_url })
    }

    /// Verify credentials by fetching the authenticated user.
    pub async fn get_myself(&self) -> Result<JiraUser, AppError> {
        let url = format!("{}/rest/api/3/myself", self.base_url);
        debug!("GET {url}");

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Network error: {e}")))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_else(|_| "no body".to_string());
            return Err(AppError::Internal(format!(
                "Jira auth failed ({status}): {body}"
            )));
        }

        let user: JiraUser = resp
            .json()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to parse user: {e}")))?;

        info!("Authenticated as {}", user.display_name);
        Ok(user)
    }

    /// Fetch issues assigned to the current user. Uses the enhanced
    /// `/rest/api/3/search/jql` endpoint with token-based pagination.
    pub async fn fetch_my_issues(&self) -> Result<Vec<Issue>, AppError> {
        let jql = "assignee = currentUser() AND statusCategory != Done ORDER BY updated DESC";
        let mut all_issues = Vec::new();
        let page_size: u32 = 50;
        let mut next_page_token: Option<String> = None;

        loop {
            let mut url = format!(
                "{}/rest/api/3/search/jql?jql={}&maxResults={}&fields={}",
                self.base_url,
                urlencoded(jql),
                page_size,
                SEARCH_FIELDS,
            );
            if let Some(ref token) = next_page_token {
                url.push_str(&format!("&nextPageToken={}", urlencoded(token)));
            }
            debug!("GET {url}");

            let resp = self
                .client
                .get(&url)
                .send()
                .await
                .map_err(|e| AppError::Network(format!("Network error: {e}")))?;

            if !resp.status().is_success() {
                let status = resp.status();
                let body = resp.text().await.unwrap_or_else(|_| "no body".to_string());
                return Err(AppError::Internal(format!(
                    "Jira search failed ({status}): {body}"
                )));
            }

            let search: JiraSearchResponse = resp
                .json()
                .await
                .map_err(|e| AppError::Internal(format!("Failed to parse search: {e}")))?;

            for ji in &search.issues {
                all_issues.push(jira_issue_to_domain(ji, &self.base_url));
            }

            if search.is_last.unwrap_or(true) || search.next_page_token.is_none() {
                break;
            }
            next_page_token = search.next_page_token;
        }

        info!("Fetched {} issues from Jira", all_issues.len());
        Ok(all_issues)
    }

    // ── Boards ──────────────────────────────────────────────────────

    /// Fetch all boards visible to the user from the Agile API.
    pub async fn fetch_boards(&self) -> Result<Vec<Board>, AppError> {
        let mut all_boards = Vec::new();
        let mut start_at: u32 = 0;
        let page_size: u32 = 50;

        loop {
            let url = format!(
                "{}/rest/agile/1.0/board?startAt={}&maxResults={}",
                self.base_url, start_at, page_size,
            );
            debug!("GET {url}");

            let resp = self
                .client
                .get(&url)
                .send()
                .await
                .map_err(|e| AppError::Internal(format!("Network error: {e}")))?;

            if !resp.status().is_success() {
                let status = resp.status();
                let body = resp.text().await.unwrap_or_else(|_| "no body".to_string());
                return Err(AppError::Internal(format!(
                    "Jira boards failed ({status}): {body}"
                )));
            }

            let list: JiraBoardListResponse = resp
                .json()
                .await
                .map_err(|e| AppError::Internal(format!("Failed to parse boards: {e}")))?;

            for jb in &list.values {
                all_boards.push(Board {
                    id: jb.id,
                    name: jb.name.clone(),
                    board_type: jb.board_type.clone(),
                    project_key: jb.location.as_ref().and_then(|l| l.project_key.clone()),
                });
            }

            if list.is_last == Some(true) {
                break;
            }
            let fetched = start_at + list.values.len() as u32;
            if list.total.is_some_and(|t| fetched >= t) || list.values.is_empty() {
                break;
            }
            start_at = fetched;
        }

        info!("Fetched {} boards from Jira", all_boards.len());
        Ok(all_boards)
    }

    /// Fetch issues belonging to a specific board.
    pub async fn fetch_board_issues(&self, board_id: u32) -> Result<Vec<Issue>, AppError> {
        let mut all_issues = Vec::new();
        let mut start_at: u32 = 0;
        let page_size: u32 = 50;

        loop {
            let url = format!(
                "{}/rest/agile/1.0/board/{}/issue?startAt={}&maxResults={}&fields={}",
                self.base_url, board_id, start_at, page_size, SEARCH_FIELDS,
            );
            debug!("GET {url}");

            let resp = self
                .client
                .get(&url)
                .send()
                .await
                .map_err(|e| AppError::Internal(format!("Network error: {e}")))?;

            if !resp.status().is_success() {
                let status = resp.status();
                let body = resp.text().await.unwrap_or_else(|_| "no body".to_string());
                return Err(AppError::Internal(format!(
                    "Board issues failed ({status}): {body}"
                )));
            }

            let search: JiraLegacySearchResponse = resp
                .json()
                .await
                .map_err(|e| AppError::Internal(format!("Failed to parse board issues: {e}")))?;

            for ji in &search.issues {
                all_issues.push(jira_issue_to_domain(ji, &self.base_url));
            }

            let fetched = start_at + search.issues.len() as u32;
            if fetched >= search.total || search.issues.is_empty() {
                break;
            }
            start_at = fetched;
        }

        info!(
            "Fetched {} issues from board {}",
            all_issues.len(),
            board_id
        );
        Ok(all_issues)
    }

    // ── Quick actions ───────────────────────────────────────────────

    /// Get available transitions for an issue.
    pub async fn get_transitions(&self, issue_key: &str) -> Result<Vec<IssueTransition>, AppError> {
        let url = format!(
            "{}/rest/api/3/issue/{}/transitions",
            self.base_url, issue_key
        );
        debug!("GET {url}");

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Network error: {e}")))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_else(|_| "no body".to_string());
            return Err(AppError::Internal(format!(
                "Transitions failed ({status}): {body}"
            )));
        }

        let data: JiraTransitionsResponse = resp
            .json()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to parse transitions: {e}")))?;

        Ok(data
            .transitions
            .into_iter()
            .map(|t| IssueTransition {
                id: t.id,
                name: t.name,
            })
            .collect())
    }

    /// Transition an issue to a new status.
    pub async fn transition_issue(
        &self,
        issue_key: &str,
        transition_id: &str,
    ) -> Result<(), AppError> {
        let url = format!(
            "{}/rest/api/3/issue/{}/transitions",
            self.base_url, issue_key
        );
        let body = serde_json::json!({
            "transition": { "id": transition_id }
        });
        debug!("POST {url}");

        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Network error: {e}")))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body_text = resp.text().await.unwrap_or_else(|_| "no body".to_string());
            return Err(AppError::Internal(format!(
                "Transition failed ({status}): {body_text}"
            )));
        }

        info!("Transitioned {issue_key} via transition {transition_id}");
        Ok(())
    }

    /// Add a comment to an issue.
    pub async fn add_comment(&self, issue_key: &str, body_text: &str) -> Result<(), AppError> {
        let url = format!("{}/rest/api/3/issue/{}/comment", self.base_url, issue_key);
        // Jira Cloud v3 uses ADF (Atlassian Document Format) for comments.
        let body = serde_json::json!({
            "body": {
                "type": "doc",
                "version": 1,
                "content": [{
                    "type": "paragraph",
                    "content": [{
                        "type": "text",
                        "text": body_text,
                    }]
                }]
            }
        });
        debug!("POST {url}");

        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Network error: {e}")))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let resp_body = resp.text().await.unwrap_or_else(|_| "no body".to_string());
            return Err(AppError::Internal(format!(
                "Comment failed ({status}): {resp_body}"
            )));
        }

        info!("Added comment to {issue_key}");
        Ok(())
    }
}

/// Convert a Jira REST issue to our domain Issue.
fn jira_issue_to_domain(ji: &super::types::JiraIssue, base_url: &str) -> Issue {
    let now = chrono::Utc::now();

    let due_date = ji.fields.duedate.as_deref().and_then(|d| d.parse().ok());
    let created_at = chrono::DateTime::parse_from_rfc3339(&ji.fields.created)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .unwrap_or(now);
    let updated_at = chrono::DateTime::parse_from_rfc3339(&ji.fields.updated)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .unwrap_or(now);

    Issue {
        id: uuid::Uuid::new_v4().to_string(),
        key: ji.key.clone(),
        summary: ji.fields.summary.clone(),
        description: ji
            .fields
            .description
            .as_ref()
            .and_then(|v| serde_json::to_string(v).ok()),
        status: IssueStatus::from_jira(&ji.fields.status.name),
        priority: ji
            .fields
            .priority
            .as_ref()
            .map(|p| IssuePriority::from_jira(&p.name))
            .unwrap_or(IssuePriority::Unknown),
        project_key: ji.fields.project.key.clone(),
        project_name: ji.fields.project.name.clone(),
        assignee: ji.fields.assignee.as_ref().map(|a| a.display_name.clone()),
        reporter: ji.fields.reporter.as_ref().map(|r| r.display_name.clone()),
        labels: ji.fields.labels.clone().unwrap_or_default(),
        sprint: ji.fields.sprint.as_ref().map(|s| s.name.clone()),
        due_date,
        created_at,
        updated_at,
        synced_at: now,
        web_url: Some(format!("{}/browse/{}", base_url, ji.key)),
    }
}

fn base64_encode(input: &str) -> String {
    use std::io::Write;
    let mut buf = Vec::new();
    {
        let mut encoder = base64_writer(&mut buf);
        encoder.write_all(input.as_bytes()).unwrap();
    }
    String::from_utf8(buf).unwrap()
}

/// Minimal base64 encoder without pulling in another crate.
fn base64_writer(output: &mut Vec<u8>) -> Base64Writer<'_> {
    Base64Writer {
        output,
        buffer: [0; 3],
        pos: 0,
    }
}

struct Base64Writer<'a> {
    output: &'a mut Vec<u8>,
    buffer: [u8; 3],
    pos: usize,
}

const B64_CHARS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

impl<'a> std::io::Write for Base64Writer<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        for &byte in buf {
            self.buffer[self.pos] = byte;
            self.pos += 1;
            if self.pos == 3 {
                self.output.push(B64_CHARS[(self.buffer[0] >> 2) as usize]);
                self.output.push(
                    B64_CHARS[(((self.buffer[0] & 0x03) << 4) | (self.buffer[1] >> 4)) as usize],
                );
                self.output.push(
                    B64_CHARS[(((self.buffer[1] & 0x0f) << 2) | (self.buffer[2] >> 6)) as usize],
                );
                self.output
                    .push(B64_CHARS[(self.buffer[2] & 0x3f) as usize]);
                self.pos = 0;
            }
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Drop for Base64Writer<'_> {
    fn drop(&mut self) {
        match self.pos {
            1 => {
                self.output.push(B64_CHARS[(self.buffer[0] >> 2) as usize]);
                self.output
                    .push(B64_CHARS[((self.buffer[0] & 0x03) << 4) as usize]);
                self.output.push(b'=');
                self.output.push(b'=');
            }
            2 => {
                self.output.push(B64_CHARS[(self.buffer[0] >> 2) as usize]);
                self.output.push(
                    B64_CHARS[(((self.buffer[0] & 0x03) << 4) | (self.buffer[1] >> 4)) as usize],
                );
                self.output
                    .push(B64_CHARS[((self.buffer[1] & 0x0f) << 2) as usize]);
                self.output.push(b'=');
            }
            _ => {}
        }
    }
}

/// Minimal percent-encoding for JQL query strings.
fn urlencoded(input: &str) -> String {
    let mut result = String::with_capacity(input.len() * 2);
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(byte as char);
            }
            _ => {
                result.push('%');
                result.push_str(&format!("{byte:02X}"));
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_encodes_correctly() {
        assert_eq!(
            base64_encode("user@example.com:token123"),
            "dXNlckBleGFtcGxlLmNvbTp0b2tlbjEyMw=="
        );
        assert_eq!(base64_encode("a"), "YQ==");
        assert_eq!(base64_encode("ab"), "YWI=");
        assert_eq!(base64_encode("abc"), "YWJj");
    }

    #[test]
    fn urlencoded_encodes_spaces_and_specials() {
        assert_eq!(urlencoded("a b"), "a%20b");
        assert_eq!(urlencoded("key=val"), "key%3Dval");
        assert_eq!(urlencoded("hello"), "hello");
    }
}
