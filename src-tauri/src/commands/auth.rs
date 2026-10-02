use tauri::State;
use tracing::info;

use crate::infrastructure::credentials::{self, JiraCredentials};
use crate::infrastructure::database::Database;
use crate::infrastructure::error::AppError;
use crate::infrastructure::jira::client::JiraClient;

/// Input from the frontend connect form.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectInput {
    pub base_url: String,
    pub email: String,
    pub api_token: String,
}

/// Result returned after a successful connection.
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectResult {
    pub display_name: String,
    pub email: Option<String>,
    pub issue_count: u32,
}

/// Connect to Jira: verify credentials, store in keychain, fetch issues.
#[tauri::command]
pub async fn jira_connect(
    db: State<'_, Database>,
    input: ConnectInput,
) -> Result<ConnectResult, AppError> {
    let base_url = input.base_url.trim_end_matches('/').to_string();
    if !base_url.starts_with("https://") {
        return Err(AppError::Internal(
            "Jira URL must start with https://".to_string(),
        ));
    }

    let client = JiraClient::new(&base_url, &input.email, &input.api_token)?;
    let user = client.get_myself().await?;
    info!("Connected as {}", user.display_name);

    let issues = client.fetch_my_issues().await?;
    let issue_count = issues.len() as u32;

    db.clear_issues()?;
    for issue in &issues {
        db.upsert_issue(issue)?;
    }

    db.set_workspace_meta("user_display_name", &user.display_name)?;
    db.set_workspace_meta("jira_base_url", &base_url)?;
    db.set_workspace_meta("last_synced_at", &chrono::Utc::now().to_rfc3339())?;

    credentials::store_credentials(&JiraCredentials {
        base_url,
        email: input.email,
        api_token: input.api_token,
    })?;

    Ok(ConnectResult {
        display_name: user.display_name,
        email: user.email_address,
        issue_count,
    })
}

/// Disconnect from Jira: clear credentials and cached data.
#[tauri::command]
pub fn jira_disconnect(db: State<'_, Database>) -> Result<(), AppError> {
    credentials::clear_credentials()?;
    db.clear_issues()?;
    db.set_workspace_meta("user_display_name", "")?;
    db.set_workspace_meta("jira_base_url", "")?;
    db.set_workspace_meta("last_synced_at", "")?;
    info!("Disconnected from Jira");
    Ok(())
}

/// Sync: re-fetch issues using stored credentials.
#[tauri::command]
pub async fn jira_sync(db: State<'_, Database>) -> Result<u32, AppError> {
    let creds = credentials::load_credentials()?
        .ok_or_else(|| AppError::Internal("Not connected to Jira".to_string()))?;

    let client = JiraClient::new(&creds.base_url, &creds.email, &creds.api_token)?;
    let issues = client.fetch_my_issues().await?;
    let count = issues.len() as u32;

    db.clear_issues()?;
    for issue in &issues {
        db.upsert_issue(issue)?;
    }
    db.set_workspace_meta("last_synced_at", &chrono::Utc::now().to_rfc3339())?;
    info!("Synced {} issues", count);

    Ok(count)
}

/// Check if credentials exist and try to restore the session on startup.
#[tauri::command]
pub async fn jira_restore_session(db: State<'_, Database>) -> Result<bool, AppError> {
    let creds = credentials::load_credentials()?;
    let Some(creds) = creds else {
        return Ok(false);
    };

    match JiraClient::new(&creds.base_url, &creds.email, &creds.api_token) {
        Ok(client) => match client.get_myself().await {
            Ok(user) => {
                db.set_workspace_meta("user_display_name", &user.display_name)?;
                db.set_workspace_meta("jira_base_url", &creds.base_url)?;
                info!("Session restored for {}", user.display_name);
                Ok(true)
            }
            Err(_) => Ok(false),
        },
        Err(_) => Ok(false),
    }
}
