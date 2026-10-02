//! Board and saved-board domain types.

use serde::{Deserialize, Serialize};

/// A Jira board as seen by the application.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Board {
    /// Jira board ID.
    pub id: u32,
    /// Board name.
    pub name: String,
    /// Board type (scrum, kanban, simple).
    pub board_type: String,
    /// Project key the board belongs to, if any.
    pub project_key: Option<String>,
}

/// A board the user has explicitly saved for quick access.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedBoard {
    pub board_id: u32,
    pub name: String,
    pub board_type: String,
    pub project_key: Option<String>,
}

/// Available transitions for an issue (from Jira workflow).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueTransition {
    pub id: String,
    pub name: String,
}
