use std::sync::Arc;

use tauri::State;
use tracing::info;

use crate::infrastructure::credentials::{self, OAuthCredentials};
use crate::infrastructure::database::Database;
use crate::infrastructure::error::AppError;
use crate::infrastructure::jira::client::JiraClient;
use crate::infrastructure::jira::oauth;
use crate::sync::engine::SyncEngine;

/// Input from the frontend OAuth setup form.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuthSetupInput {
    pub client_id: String,
    pub client_secret: String,
}

/// Result after starting OAuth: the URL to open in the browser.
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuthStartResult {
    pub auth_url: String,
}

/// Site info returned after OAuth completes.
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuthSite {
    pub cloud_id: String,
    pub name: String,
    pub url: String,
}

/// Result after OAuth connection completes.
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuthConnectResult {
    pub display_name: String,
    pub email: Option<String>,
    pub issue_count: u32,
    pub site_url: String,
}

/// Start the OAuth flow: open the auth URL and wait for callback.
/// This combines starting the callback server, opening the browser,
/// exchanging the code, and completing the connection.
#[tauri::command]
pub async fn oauth_start(
    db: State<'_, Arc<Database>>,
    engine: State<'_, SyncEngine>,
    input: OAuthSetupInput,
) -> Result<OAuthConnectResult, AppError> {
    let state = uuid::Uuid::new_v4().to_string();
    let auth_url = oauth::build_auth_url(&input.client_id, &state);

    // Open the auth URL in the user's default browser.
    if let Err(e) = open::that(&auth_url) {
        tracing::warn!("Failed to open browser: {e}. URL: {auth_url}");
    }

    // Wait for the OAuth callback from the browser.
    let (code, returned_state) = oauth::wait_for_callback().await?;

    // Verify state parameter to prevent CSRF.
    if returned_state != state {
        return Err(AppError::Internal(
            "OAuth state mismatch — possible CSRF attack".to_string(),
        ));
    }

    // Exchange authorization code for tokens.
    let tokens = oauth::exchange_code(&input.client_id, &input.client_secret, &code).await?;
    let refresh_token = tokens.refresh_token.ok_or_else(|| {
        AppError::Internal("No refresh token received. Ensure offline_access scope.".to_string())
    })?;

    // Get accessible resources to find the Jira site cloud ID.
    let resources = oauth::get_accessible_resources(&tokens.access_token).await?;
    let site = resources.into_iter().next().ok_or_else(|| {
        AppError::Internal("No accessible Jira sites found for this account.".to_string())
    })?;

    // Store OAuth credentials in keychain.
    credentials::store_oauth_credentials(&OAuthCredentials {
        access_token: tokens.access_token.clone(),
        refresh_token,
        cloud_id: site.id.clone(),
        site_url: site.url.clone(),
        client_id: input.client_id,
        client_secret: input.client_secret,
    })?;

    // Create a client and verify + fetch data.
    let client = JiraClient::with_oauth(&tokens.access_token, &site.id)?;
    let user = client.get_myself().await?;
    info!("OAuth connected as {}", user.display_name);

    let issues = client.fetch_my_issues().await?;
    let issue_count = issues.len() as u32;

    db.clear_issues()?;
    for issue in &issues {
        db.upsert_issue(issue)?;
    }

    db.set_workspace_meta("user_display_name", &user.display_name)?;
    db.set_workspace_meta("jira_base_url", &site.url)?;
    db.set_workspace_meta("last_synced_at", &chrono::Utc::now().to_rfc3339())?;
    db.set_workspace_meta("auth_method", "oauth")?;

    engine.trigger_sync();

    Ok(OAuthConnectResult {
        display_name: user.display_name,
        email: user.email_address,
        issue_count,
        site_url: site.url,
    })
}

/// Disconnect OAuth: clear credentials and cached data.
#[tauri::command]
pub fn oauth_disconnect(db: State<'_, Arc<Database>>) -> Result<(), AppError> {
    credentials::clear_oauth_credentials()?;
    db.clear_issues()?;
    db.set_workspace_meta("user_display_name", "")?;
    db.set_workspace_meta("jira_base_url", "")?;
    db.set_workspace_meta("last_synced_at", "")?;
    db.set_workspace_meta("auth_method", "")?;
    info!("OAuth disconnected");
    Ok(())
}

/// Manually refresh OAuth tokens.
#[tauri::command]
pub async fn oauth_refresh() -> Result<(), AppError> {
    let creds = credentials::load_oauth_credentials()?
        .ok_or_else(|| AppError::Internal("No OAuth credentials found".to_string()))?;

    let tokens =
        oauth::refresh_tokens(&creds.client_id, &creds.client_secret, &creds.refresh_token).await?;

    let new_refresh = tokens
        .refresh_token
        .ok_or_else(|| AppError::Internal("No refresh token in refresh response".to_string()))?;

    credentials::update_oauth_tokens(&tokens.access_token, &new_refresh)?;
    Ok(())
}
