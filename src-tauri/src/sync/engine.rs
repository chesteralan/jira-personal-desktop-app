//! The sync engine runs as a background tokio task.
//! It performs incremental syncs: upsert new/changed issues and
//! remove issues no longer assigned. On failure the last committed
//! snapshot is preserved.

use std::collections::HashSet;
use std::sync::Arc;

use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::Notify;
use tokio::time::{interval, Duration};
use tracing::{error, info, warn};

use crate::commands::auth::make_active_client;
use crate::infrastructure::database::Database;
use crate::sync::{SyncState, SyncStatus};

/// Event name emitted to the frontend whenever sync status changes.
const SYNC_EVENT: &str = "sync-status";

/// Handle to control the background sync engine.
pub struct SyncEngine {
    /// Notify to trigger an immediate sync cycle.
    trigger: Arc<Notify>,
}

impl SyncEngine {
    /// Spawn the background sync loop. Returns a handle for triggering syncs.
    pub fn spawn(app: AppHandle, db: Arc<Database>, interval_secs: u64) -> Self {
        let trigger = Arc::new(Notify::new());
        let trigger_clone = trigger.clone();

        tokio::spawn(async move {
            sync_loop(app, db, interval_secs, trigger_clone).await;
        });

        Self { trigger }
    }

    /// Trigger an immediate sync cycle (non-blocking).
    pub fn trigger_sync(&self) {
        self.trigger.notify_one();
    }
}

async fn sync_loop(app: AppHandle, db: Arc<Database>, interval_secs: u64, trigger: Arc<Notify>) {
    let mut tick = interval(Duration::from_secs(interval_secs));
    // Don't fire immediately — let the app settle first.
    tick.tick().await;

    loop {
        tokio::select! {
            _ = tick.tick() => {
                run_sync_cycle(&app, &db).await;
            }
            _ = trigger.notified() => {
                run_sync_cycle(&app, &db).await;
                // Reset the interval so we don't double-sync.
                tick.reset();
            }
        }
    }
}

async fn run_sync_cycle(app: &AppHandle, db: &Database) {
    // Build a client from whichever auth method is active.
    let client = match make_active_client() {
        Ok(c) => c,
        Err(_) => return, // No credentials — skip silently.
    };

    // Emit syncing status.
    emit_status(app, SyncState::Syncing, None, None, None);

    // Fetch issues from Jira. On network failure, preserve the cache.
    let fresh_issues = match client.fetch_my_issues().await {
        Ok(issues) => issues,
        Err(e) => {
            let is_network = format!("{e}").contains("Network error");
            if is_network {
                warn!("Sync skipped: offline or network error");
                emit_status(
                    app,
                    SyncState::Offline,
                    None,
                    None,
                    Some("Network unavailable".to_string()),
                );
            } else {
                error!("Sync failed: {e}");
                emit_status(
                    app,
                    SyncState::Error,
                    None,
                    None,
                    Some(format!("Sync error: {e}")),
                );
                notify_if_background(app, "Sync failed", &format!("{e}"));
            }
            return;
        }
    };

    // Incremental sync: upsert all fetched issues, then remove stale ones.
    let fresh_keys: HashSet<String> = fresh_issues.iter().map(|i| i.key.clone()).collect();
    let count = fresh_issues.len() as u32;

    for issue in &fresh_issues {
        if let Err(e) = db.upsert_issue(issue) {
            error!("Failed to upsert issue {}: {e}", issue.key);
        }
    }

    // Remove issues no longer in the Jira results.
    match db.list_all_issue_keys() {
        Ok(existing_keys) => {
            for key in existing_keys {
                if !fresh_keys.contains(&key) {
                    if let Err(e) = db.delete_issue_by_key(&key) {
                        error!("Failed to remove stale issue {key}: {e}");
                    }
                }
            }
        }
        Err(e) => {
            warn!("Could not list existing keys for stale removal: {e}");
        }
    }

    let now = chrono::Utc::now().to_rfc3339();
    let _ = db.set_workspace_meta("last_synced_at", &now);
    info!("Background sync complete: {count} issues");

    emit_status(app, SyncState::Success, Some(now), Some(count), None);

    // Send a desktop notification if the window is not focused.
    notify_if_background(app, "Sync complete", &format!("{count} issues synced"));
}

fn emit_status(
    app: &AppHandle,
    state: SyncState,
    last_synced_at: Option<String>,
    issue_count: Option<u32>,
    error_msg: Option<String>,
) {
    let status = SyncStatus {
        state,
        last_synced_at,
        issue_count,
        error: error_msg,
    };
    if let Err(e) = app.emit(SYNC_EVENT, &status) {
        warn!("Failed to emit sync event: {e}");
    }
}

/// Send a desktop notification only when the main window is not focused.
fn notify_if_background(app: &AppHandle, title: &str, body: &str) {
    use tauri_plugin_notification::NotificationExt;

    let focused = app
        .get_webview_window("main")
        .and_then(|w| w.is_focused().ok())
        .unwrap_or(false);

    if !focused {
        let _ = app.notification().builder().title(title).body(body).show();
    }
}
