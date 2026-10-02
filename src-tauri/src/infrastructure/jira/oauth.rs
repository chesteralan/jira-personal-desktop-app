//! Jira Cloud OAuth 2.0 3LO helpers.
//! Handles authorization URL generation, token exchange, refresh, and
//! accessible-resources lookup. The local callback server captures the
//! authorization code from the browser redirect.

use reqwest::Client;
use serde::Deserialize;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use tracing::{debug, info, warn};

use crate::infrastructure::error::AppError;

const AUTH_URL: &str = "https://auth.atlassian.com/authorize";
const TOKEN_URL: &str = "https://auth.atlassian.com/oauth/token";
const RESOURCES_URL: &str = "https://api.atlassian.com/oauth/token/accessible-resources";

/// Scopes needed for reading/writing Jira issues and boards.
/// Classic scopes cover the Jira Platform REST API; granular scopes are
/// required by the Jira Software (Agile) REST API for board access.
const SCOPES: &str = "read:jira-work write:jira-work read:jira-user offline_access \
    read:board-scope:jira-software read:project:jira read:issue-details:jira";

/// Default port for the local callback server.
const CALLBACK_PORT: u16 = 17042;

/// Generate the authorization URL the user should visit.
pub fn build_auth_url(client_id: &str, state: &str) -> String {
    let redirect_uri = format!("http://localhost:{CALLBACK_PORT}/callback");
    format!(
        "{AUTH_URL}?client_id={}&scope={}&redirect_uri={}&state={}&response_type=code&prompt=consent",
        urlencoded(client_id),
        urlencoded(SCOPES),
        urlencoded(&redirect_uri),
        urlencoded(state),
    )
}

/// Spin up a one-shot local HTTP server that waits for the OAuth callback.
/// Returns `(authorization_code, state)` once the browser redirects.
pub async fn wait_for_callback() -> Result<(String, String), AppError> {
    let addr = format!("127.0.0.1:{CALLBACK_PORT}");
    let listener = TcpListener::bind(&addr).await.map_err(|e| {
        AppError::Internal(format!("Failed to bind callback server on {addr}: {e}"))
    })?;
    info!("OAuth callback server listening on {addr}");

    let (tx, rx) = oneshot::channel::<Result<(String, String), AppError>>();

    // Accept exactly one connection, extract code and state, respond, and shut down.
    tokio::spawn(async move {
        match listener.accept().await {
            Ok((mut stream, _)) => {
                let mut buf = vec![0u8; 4096];
                let n = stream.read(&mut buf).await.unwrap_or(0);
                let request = String::from_utf8_lossy(&buf[..n]);

                let result = parse_callback_request(&request);

                // Send a friendly HTML response regardless of outcome.
                let (status, body) = match &result {
                    Ok(_) => ("200 OK", "Authorization successful! You can close this tab and return to Jira Personal."),
                    Err(e) => ("400 Bad Request", {
                        warn!("OAuth callback error: {e}");
                        "Authorization failed. Please try again."
                    }),
                };

                let response = format!(
                    "HTTP/1.1 {status}\r\nContent-Type: text/html\r\nConnection: close\r\n\r\n\
                     <html><body style=\"font-family:system-ui;text-align:center;padding:48px\">\
                     <h2>{body}</h2></body></html>"
                );
                let _ = stream.write_all(response.as_bytes()).await;
                let _ = stream.shutdown().await;

                let _ = tx.send(result);
            }
            Err(e) => {
                let _ = tx.send(Err(AppError::Internal(format!(
                    "Callback accept failed: {e}"
                ))));
            }
        }
    });

    rx.await
        .map_err(|_| AppError::Internal("Callback channel closed".to_string()))?
}

/// Parse the HTTP request line to extract `code` and `state` query parameters.
fn parse_callback_request(request: &str) -> Result<(String, String), AppError> {
    // First line: GET /callback?code=XXX&state=YYY HTTP/1.1
    let first_line = request
        .lines()
        .next()
        .ok_or_else(|| AppError::Internal("Empty callback request".to_string()))?;

    let path = first_line
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| AppError::Internal("Malformed callback request".to_string()))?;

    let query = path
        .split('?')
        .nth(1)
        .ok_or_else(|| AppError::Internal("No query parameters in callback".to_string()))?;

    let mut code = None;
    let mut state = None;

    for param in query.split('&') {
        if let Some((key, value)) = param.split_once('=') {
            match key {
                "code" => code = Some(value.to_string()),
                "state" => state = Some(value.to_string()),
                _ => {}
            }
        }
    }

    let code = code.ok_or_else(|| AppError::Internal("Missing code in callback".to_string()))?;
    let state = state.ok_or_else(|| AppError::Internal("Missing state in callback".to_string()))?;

    Ok((code, state))
}

// ── Token exchange ──────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: Option<u64>,
    pub scope: Option<String>,
}

/// Exchange an authorization code for an access token (and optional refresh token).
pub async fn exchange_code(
    client_id: &str,
    client_secret: &str,
    code: &str,
) -> Result<TokenResponse, AppError> {
    let redirect_uri = format!("http://localhost:{CALLBACK_PORT}/callback");
    let client = Client::new();

    debug!("Exchanging authorization code for tokens");
    let resp = client
        .post(TOKEN_URL)
        .json(&serde_json::json!({
            "grant_type": "authorization_code",
            "client_id": client_id,
            "client_secret": client_secret,
            "code": code,
            "redirect_uri": redirect_uri,
        }))
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("Token exchange network error: {e}")))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_else(|_| "no body".to_string());
        return Err(AppError::Internal(format!(
            "Token exchange failed ({status}): {body}"
        )));
    }

    let tokens: TokenResponse = resp
        .json()
        .await
        .map_err(|e| AppError::Internal(format!("Token parse error: {e}")))?;

    info!("OAuth tokens obtained successfully");
    Ok(tokens)
}

/// Refresh an access token using a refresh token (rotating refresh tokens).
pub async fn refresh_tokens(
    client_id: &str,
    client_secret: &str,
    refresh_token: &str,
) -> Result<TokenResponse, AppError> {
    let client = Client::new();

    debug!("Refreshing OAuth tokens");
    let resp = client
        .post(TOKEN_URL)
        .json(&serde_json::json!({
            "grant_type": "refresh_token",
            "client_id": client_id,
            "client_secret": client_secret,
            "refresh_token": refresh_token,
        }))
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("Token refresh network error: {e}")))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_else(|_| "no body".to_string());
        return Err(AppError::Internal(format!(
            "Token refresh failed ({status}): {body}"
        )));
    }

    let tokens: TokenResponse = resp
        .json()
        .await
        .map_err(|e| AppError::Internal(format!("Token refresh parse error: {e}")))?;

    info!("OAuth tokens refreshed successfully");
    Ok(tokens)
}

// ── Accessible resources ────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct AccessibleResource {
    pub id: String,
    pub name: String,
    pub url: String,
    pub scopes: Vec<String>,
}

/// Retrieve the list of Jira sites the user has authorized.
pub async fn get_accessible_resources(
    access_token: &str,
) -> Result<Vec<AccessibleResource>, AppError> {
    let client = Client::new();
    let resp = client
        .get(RESOURCES_URL)
        .header("Authorization", format!("Bearer {access_token}"))
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("Accessible resources error: {e}")))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_else(|_| "no body".to_string());
        return Err(AppError::Internal(format!(
            "Accessible resources failed ({status}): {body}"
        )));
    }

    let resources: Vec<AccessibleResource> = resp
        .json()
        .await
        .map_err(|e| AppError::Internal(format!("Resources parse error: {e}")))?;

    info!("Found {} accessible resources", resources.len());
    Ok(resources)
}

fn urlencoded(s: &str) -> String {
    s.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                String::from(b as char)
            }
            _ => format!("%{b:02X}"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_callback_extracts_code_and_state() {
        let request = "GET /callback?code=abc123&state=xyz789 HTTP/1.1\r\nHost: localhost\r\n";
        let (code, state) = parse_callback_request(request).unwrap();
        assert_eq!(code, "abc123");
        assert_eq!(state, "xyz789");
    }

    #[test]
    fn parse_callback_missing_code_fails() {
        let request = "GET /callback?state=xyz HTTP/1.1\r\n";
        assert!(parse_callback_request(request).is_err());
    }

    #[test]
    fn auth_url_contains_required_params() {
        let url = build_auth_url("my-client-id", "random-state");
        assert!(url.contains("client_id=my-client-id"));
        assert!(url.contains("state=random-state"));
        assert!(url.contains("response_type=code"));
        assert!(url.contains("prompt=consent"));
        assert!(url.contains("offline_access"));
    }
}
