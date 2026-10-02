//! Raw Jira REST API response types. These map directly to the JSON
//! returned by Jira Cloud v3 and are never exposed to the frontend.

use serde::Deserialize;

/// GET /rest/api/3/myself
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JiraUser {
    pub account_id: String,
    pub display_name: String,
    pub email_address: Option<String>,
    pub active: bool,
}

/// GET /rest/api/3/search (paginated wrapper)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JiraSearchResponse {
    pub start_at: u32,
    pub max_results: u32,
    pub total: u32,
    pub issues: Vec<JiraIssue>,
}

#[derive(Debug, Deserialize)]
pub struct JiraIssue {
    pub id: String,
    pub key: String,
    #[serde(rename = "self")]
    pub self_url: String,
    pub fields: JiraIssueFields,
}

#[derive(Debug, Deserialize)]
pub struct JiraIssueFields {
    pub summary: String,
    pub description: Option<serde_json::Value>,
    pub status: JiraStatus,
    pub priority: Option<JiraPriority>,
    pub project: JiraProject,
    pub assignee: Option<JiraAssignee>,
    pub reporter: Option<JiraAssignee>,
    pub labels: Option<Vec<String>>,
    pub created: String,
    pub updated: String,
    pub duedate: Option<String>,
    pub sprint: Option<JiraSprint>,
}

#[derive(Debug, Deserialize)]
pub struct JiraStatus {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct JiraPriority {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct JiraProject {
    pub key: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JiraAssignee {
    pub display_name: String,
}

#[derive(Debug, Deserialize)]
pub struct JiraSprint {
    pub name: String,
}

// ── Agile / Board types ─────────────────────────────────────────────

/// GET /rest/agile/1.0/board (paginated wrapper)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JiraBoardListResponse {
    pub max_results: u32,
    pub start_at: u32,
    pub total: Option<u32>,
    pub is_last: Option<bool>,
    pub values: Vec<JiraBoard>,
}

#[derive(Debug, Deserialize)]
pub struct JiraBoard {
    pub id: u32,
    pub name: String,
    #[serde(rename = "type")]
    pub board_type: String,
    pub location: Option<JiraBoardLocation>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JiraBoardLocation {
    pub project_key: Option<String>,
}

// ── Transitions ─────────────────────────────────────────────────────

/// GET /rest/api/3/issue/{issueKey}/transitions
#[derive(Debug, Deserialize)]
pub struct JiraTransitionsResponse {
    pub transitions: Vec<JiraTransition>,
}

#[derive(Debug, Deserialize)]
pub struct JiraTransition {
    pub id: String,
    pub name: String,
}
