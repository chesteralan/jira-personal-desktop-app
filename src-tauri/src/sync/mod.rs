//! Synchronization module. Full implementation arrives in M4.
//! Currently provides the scheduling types for future background sync.

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncState {
    #[default]
    Idle,
    Syncing,
    Error,
}
