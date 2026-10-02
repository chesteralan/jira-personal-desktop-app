use serde::Serialize;

/// Application-level error type surfaced through Tauri IPC.
/// Messages are safe to display in the UI — they never contain tokens or secrets.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Authentication failed: {0}")]
    Auth(String),

    #[error("Invalid input: {0}")]
    Validation(String),

    #[error("{0}")]
    Internal(String),
}

impl AppError {
    /// True when the error was caused by a network failure (offline, DNS, timeout).
    pub fn is_network(&self) -> bool {
        matches!(self, AppError::Network(_))
    }
}

/// Serializable wrapper so Tauri can return errors over IPC.
/// The serialized string is the user-facing message.
impl Serialize for AppError {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
