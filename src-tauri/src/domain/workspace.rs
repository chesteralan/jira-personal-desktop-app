use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents the overall workspace state exposed to the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceInfo {
    pub connection_status: ConnectionStatus,
    pub last_synced_at: Option<DateTime<Utc>>,
    pub issue_count: u32,
    pub user_display_name: Option<String>,
    pub jira_base_url: Option<String>,
}

impl Default for WorkspaceInfo {
    fn default() -> Self {
        Self {
            connection_status: ConnectionStatus::Disconnected,
            last_synced_at: None,
            issue_count: 0,
            user_display_name: None,
            jira_base_url: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
    Syncing,
    Error,
}
