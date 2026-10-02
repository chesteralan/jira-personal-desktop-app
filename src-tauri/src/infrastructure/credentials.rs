//! Secure credential storage using the OS keychain.
//! Credentials are never logged, stored in SQLite, or exposed to the frontend.

use tracing::info;

use crate::infrastructure::error::AppError;

const SERVICE_NAME: &str = "com.petlabco.jirapersonal";

// API-token keys
const CRED_KEY_EMAIL: &str = "jira_email";
const CRED_KEY_TOKEN: &str = "jira_api_token";
const CRED_KEY_BASE_URL: &str = "jira_base_url";

// OAuth keys
const OAUTH_KEY_ACCESS_TOKEN: &str = "oauth_access_token";
const OAUTH_KEY_REFRESH_TOKEN: &str = "oauth_refresh_token";
const OAUTH_KEY_CLOUD_ID: &str = "oauth_cloud_id";
const OAUTH_KEY_SITE_URL: &str = "oauth_site_url";
const OAUTH_KEY_CLIENT_ID: &str = "oauth_client_id";
const OAUTH_KEY_CLIENT_SECRET: &str = "oauth_client_secret";

// Auth method discriminator
const AUTH_METHOD_KEY: &str = "auth_method";

/// Which authentication method is active.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthMethod {
    ApiToken,
    OAuth,
}

/// Stored credential set for Jira API-token auth.
pub struct JiraCredentials {
    pub base_url: String,
    pub email: String,
    pub api_token: String,
}

/// Stored credential set for Jira OAuth 2.0 3LO.
pub struct OAuthCredentials {
    pub access_token: String,
    pub refresh_token: String,
    pub cloud_id: String,
    pub site_url: String,
    pub client_id: String,
    pub client_secret: String,
}

/// Detect which auth method is stored, if any.
pub fn active_auth_method() -> Result<Option<AuthMethod>, AppError> {
    match get_secret(AUTH_METHOD_KEY)? {
        Some(v) if v == "api_token" => Ok(Some(AuthMethod::ApiToken)),
        Some(v) if v == "oauth" => Ok(Some(AuthMethod::OAuth)),
        _ => Ok(None),
    }
}

// ── API-token credentials ───────────────────────────────────────────

pub fn store_credentials(creds: &JiraCredentials) -> Result<(), AppError> {
    set_secret(CRED_KEY_BASE_URL, &creds.base_url)?;
    set_secret(CRED_KEY_EMAIL, &creds.email)?;
    set_secret(CRED_KEY_TOKEN, &creds.api_token)?;
    set_secret(AUTH_METHOD_KEY, "api_token")?;
    info!("API-token credentials stored in OS keychain");
    Ok(())
}

pub fn load_credentials() -> Result<Option<JiraCredentials>, AppError> {
    let base_url = get_secret(CRED_KEY_BASE_URL)?;
    let email = get_secret(CRED_KEY_EMAIL)?;
    let api_token = get_secret(CRED_KEY_TOKEN)?;

    match (base_url, email, api_token) {
        (Some(base_url), Some(email), Some(api_token)) => Ok(Some(JiraCredentials {
            base_url,
            email,
            api_token,
        })),
        _ => Ok(None),
    }
}

pub fn clear_credentials() -> Result<(), AppError> {
    delete_secret(CRED_KEY_BASE_URL);
    delete_secret(CRED_KEY_EMAIL);
    delete_secret(CRED_KEY_TOKEN);
    delete_secret(AUTH_METHOD_KEY);
    info!("API-token credentials cleared from OS keychain");
    Ok(())
}

// ── OAuth credentials ───────────────────────────────────────────────

pub fn store_oauth_credentials(creds: &OAuthCredentials) -> Result<(), AppError> {
    set_secret(OAUTH_KEY_ACCESS_TOKEN, &creds.access_token)?;
    set_secret(OAUTH_KEY_REFRESH_TOKEN, &creds.refresh_token)?;
    set_secret(OAUTH_KEY_CLOUD_ID, &creds.cloud_id)?;
    set_secret(OAUTH_KEY_SITE_URL, &creds.site_url)?;
    set_secret(OAUTH_KEY_CLIENT_ID, &creds.client_id)?;
    set_secret(OAUTH_KEY_CLIENT_SECRET, &creds.client_secret)?;
    set_secret(AUTH_METHOD_KEY, "oauth")?;
    info!("OAuth credentials stored in OS keychain");
    Ok(())
}

pub fn load_oauth_credentials() -> Result<Option<OAuthCredentials>, AppError> {
    let access_token = get_secret(OAUTH_KEY_ACCESS_TOKEN)?;
    let refresh_token = get_secret(OAUTH_KEY_REFRESH_TOKEN)?;
    let cloud_id = get_secret(OAUTH_KEY_CLOUD_ID)?;
    let site_url = get_secret(OAUTH_KEY_SITE_URL)?;
    let client_id = get_secret(OAUTH_KEY_CLIENT_ID)?;
    let client_secret = get_secret(OAUTH_KEY_CLIENT_SECRET)?;

    match (
        access_token,
        refresh_token,
        cloud_id,
        site_url,
        client_id,
        client_secret,
    ) {
        (
            Some(access_token),
            Some(refresh_token),
            Some(cloud_id),
            Some(site_url),
            Some(client_id),
            Some(client_secret),
        ) => Ok(Some(OAuthCredentials {
            access_token,
            refresh_token,
            cloud_id,
            site_url,
            client_id,
            client_secret,
        })),
        _ => Ok(None),
    }
}

/// Update just the access and refresh tokens (after a refresh cycle).
pub fn update_oauth_tokens(access_token: &str, refresh_token: &str) -> Result<(), AppError> {
    set_secret(OAUTH_KEY_ACCESS_TOKEN, access_token)?;
    set_secret(OAUTH_KEY_REFRESH_TOKEN, refresh_token)?;
    info!("OAuth tokens refreshed in OS keychain");
    Ok(())
}

pub fn clear_oauth_credentials() -> Result<(), AppError> {
    delete_secret(OAUTH_KEY_ACCESS_TOKEN);
    delete_secret(OAUTH_KEY_REFRESH_TOKEN);
    delete_secret(OAUTH_KEY_CLOUD_ID);
    delete_secret(OAUTH_KEY_SITE_URL);
    delete_secret(OAUTH_KEY_CLIENT_ID);
    delete_secret(OAUTH_KEY_CLIENT_SECRET);
    delete_secret(AUTH_METHOD_KEY);
    info!("OAuth credentials cleared from OS keychain");
    Ok(())
}

/// Clear all credentials regardless of auth method.
pub fn clear_all_credentials() -> Result<(), AppError> {
    clear_credentials()?;
    clear_oauth_credentials()?;
    Ok(())
}

// ── Keychain helpers ────────────────────────────────────────────────

fn set_secret(key: &str, value: &str) -> Result<(), AppError> {
    let entry = keyring::Entry::new(SERVICE_NAME, key)
        .map_err(|e| AppError::Internal(format!("Keyring entry error: {e}")))?;
    entry
        .set_password(value)
        .map_err(|e| AppError::Internal(format!("Keyring store error: {e}")))?;
    Ok(())
}

fn get_secret(key: &str) -> Result<Option<String>, AppError> {
    let entry = keyring::Entry::new(SERVICE_NAME, key)
        .map_err(|e| AppError::Internal(format!("Keyring entry error: {e}")))?;
    match entry.get_password() {
        Ok(val) => Ok(Some(val)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(AppError::Internal(format!("Keyring read error: {e}"))),
    }
}

fn delete_secret(key: &str) {
    if let Ok(entry) = keyring::Entry::new(SERVICE_NAME, key) {
        let _ = entry.delete_credential();
    }
}
