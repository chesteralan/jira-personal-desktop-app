//! Secure credential storage using the OS keychain.
//! Credentials are never logged, stored in SQLite, or exposed to the frontend.

use tracing::info;

use crate::infrastructure::error::AppError;

const SERVICE_NAME: &str = "com.petlabco.jirapersonal";
const CRED_KEY_EMAIL: &str = "jira_email";
const CRED_KEY_TOKEN: &str = "jira_api_token";
const CRED_KEY_BASE_URL: &str = "jira_base_url";

/// Stored credential set for Jira API-token auth.
pub struct JiraCredentials {
    pub base_url: String,
    pub email: String,
    pub api_token: String,
}

/// Store Jira credentials in the OS keychain.
pub fn store_credentials(creds: &JiraCredentials) -> Result<(), AppError> {
    set_secret(CRED_KEY_BASE_URL, &creds.base_url)?;
    set_secret(CRED_KEY_EMAIL, &creds.email)?;
    set_secret(CRED_KEY_TOKEN, &creds.api_token)?;
    info!("Credentials stored in OS keychain");
    Ok(())
}

/// Load Jira credentials from the OS keychain.
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

/// Remove all Jira credentials from the OS keychain.
pub fn clear_credentials() -> Result<(), AppError> {
    delete_secret(CRED_KEY_BASE_URL);
    delete_secret(CRED_KEY_EMAIL);
    delete_secret(CRED_KEY_TOKEN);
    info!("Credentials cleared from OS keychain");
    Ok(())
}

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
