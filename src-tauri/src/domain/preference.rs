use serde::{Deserialize, Serialize};

/// User preferences persisted in SQLite.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preferences {
    pub theme: ThemeMode,
    pub sidebar_collapsed: bool,
    pub sync_interval_secs: u32,
    pub active_view: String,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            theme: ThemeMode::System,
            sidebar_collapsed: false,
            sync_interval_secs: 300,
            active_view: "today".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}

impl ThemeMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
            Self::System => "system",
        }
    }

    pub fn from_str_lossy(raw: &str) -> Self {
        match raw.to_lowercase().as_str() {
            "light" => Self::Light,
            "dark" => Self::Dark,
            _ => Self::System,
        }
    }
}
