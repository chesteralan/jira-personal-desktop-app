use std::sync::Arc;

use tauri::State;
use tracing::info;

use crate::infrastructure::credentials::{self, AuthMethod, JiraCredentials};
use crate::infrastructure::database::Database;
use crate::infrastructure::error::AppError;
use crate::infrastructure::jira::client::JiraClient;
use crate::sync::engine::SyncEngine;

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

/// Connect to Jira via API-token: verify credentials, store in keychain, fetch issues.
#[tauri::command]
pub async fn jira_connect(
    db: State<'_, Arc<Database>>,
    input: ConnectInput,
) -> Result<ConnectResult, AppError> {
    let base_url = input.base_url.trim().trim_end_matches('/').to_string();
    if base_url.is_empty() {
        return Err(AppError::Validation("Jira URL is required".to_string()));
    }
    if !base_url.starts_with("https://") {
        return Err(AppError::Validation(
            "Jira URL must start with https://".to_string(),
        ));
    }
    if input.email.trim().is_empty() {
        return Err(AppError::Validation("Email is required".to_string()));
    }
    if input.api_token.trim().is_empty() {
        return Err(AppError::Validation("API token is required".to_string()));
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
    db.set_workspace_meta("auth_method", "api_token")?;

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

/// Disconnect from Jira: clear all credentials and cached data.
#[tauri::command]
pub fn jira_disconnect(db: State<'_, Arc<Database>>) -> Result<(), AppError> {
    credentials::clear_all_credentials()?;
    db.clear_issues()?;
    db.set_workspace_meta("user_display_name", "")?;
    db.set_workspace_meta("jira_base_url", "")?;
    db.set_workspace_meta("last_synced_at", "")?;
    db.set_workspace_meta("auth_method", "")?;
    info!("Disconnected from Jira");
    Ok(())
}

/// Build a JiraClient from whatever auth method is currently stored.
pub fn make_active_client() -> Result<JiraClient, AppError> {
    match credentials::active_auth_method()? {
        Some(AuthMethod::ApiToken) => {
            let creds = credentials::load_credentials()?
                .ok_or_else(|| AppError::Internal("API-token credentials missing".to_string()))?;
            JiraClient::new(&creds.base_url, &creds.email, &creds.api_token)
        }
        Some(AuthMethod::OAuth) => {
            let creds = credentials::load_oauth_credentials()?
                .ok_or_else(|| AppError::Internal("OAuth credentials missing".to_string()))?;
            JiraClient::with_oauth(&creds.access_token, &creds.cloud_id)
        }
        None => Err(AppError::Internal("Not connected to Jira".to_string())),
    }
}

/// Sync: trigger background sync engine for an immediate cycle.
#[tauri::command]
pub async fn jira_sync(
    db: State<'_, Arc<Database>>,
    engine: State<'_, SyncEngine>,
) -> Result<u32, AppError> {
    // Verify we have some credentials.
    let _ = make_active_client()?;

    engine.trigger_sync();

    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    let count = db.count_issues(None)?;
    Ok(count)
}

/// Check if any credentials exist, restore the session, and trigger a background sync.
#[tauri::command]
pub async fn jira_restore_session(
    db: State<'_, Arc<Database>>,
    engine: State<'_, SyncEngine>,
) -> Result<bool, AppError> {
    let client = match make_active_client() {
        Ok(c) => c,
        Err(_) => return Ok(false),
    };

    match client.get_myself().await {
        Ok(user) => {
            db.set_workspace_meta("user_display_name", &user.display_name)?;
            // Preserve site URL from whatever was stored.
            info!("Session restored for {}", user.display_name);
            engine.trigger_sync();
            Ok(true)
        }
        Err(_) => {
            // If OAuth, try refreshing the token first.
            if credentials::active_auth_method()? == Some(AuthMethod::OAuth) {
                if let Some(creds) = credentials::load_oauth_credentials()? {
                    if let Ok(tokens) = crate::infrastructure::jira::oauth::refresh_tokens(
                        &creds.client_id,
                        &creds.client_secret,
                        &creds.refresh_token,
                    )
                    .await
                    {
                        if let Some(new_refresh) = tokens.refresh_token {
                            let _ = credentials::update_oauth_tokens(
                                &tokens.access_token,
                                &new_refresh,
                            );
                            // Retry with new token.
                            if let Ok(client) =
                                JiraClient::with_oauth(&tokens.access_token, &creds.cloud_id)
                            {
                                if let Ok(user) = client.get_myself().await {
                                    db.set_workspace_meta("user_display_name", &user.display_name)?;
                                    info!(
                                        "Session restored after token refresh for {}",
                                        user.display_name
                                    );
                                    engine.trigger_sync();
                                    return Ok(true);
                                }
                            }
                        }
                    }
                }
            }
            Ok(false)
        }
    }
}
