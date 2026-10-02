//! Background synchronization engine.
//! Coordinates periodic Jira data fetches, incremental upserts,
//! and emits status events to the frontend.

pub mod engine;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncStatus {
    pub state: SyncState,
    pub last_synced_at: Option<String>,
    pub issue_count: Option<u32>,
    pub error: Option<String>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncState {
    #[default]
    Idle,
    Syncing,
    Success,
    Error,
    Offline,
}
