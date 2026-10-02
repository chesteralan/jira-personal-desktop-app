pub mod application;
pub mod commands;
pub mod domain;
pub mod infrastructure;
pub mod sync;

use std::fs;
use std::sync::Arc;

use infrastructure::database::Database;
use sync::engine::SyncEngine;
use tracing::info;
use tracing_subscriber::EnvFilter;

fn init_database(app: &tauri::App) -> Database {
    let app_dir = app
        .path()
        .app_data_dir()
        .expect("failed to resolve app data directory");
    fs::create_dir_all(&app_dir).expect("failed to create app data directory");
    let db_path = app_dir.join("jira-personal.db");
    Database::open(&db_path).expect("failed to open database")
}

use tauri::Manager;

/// Default background sync interval in seconds (5 minutes).
const DEFAULT_SYNC_INTERVAL: u64 = 300;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    info!("Starting Jira Personal");

    tauri::Builder::default()
        .setup(|app| {
            let db = init_database(app);
            let db_arc = Arc::new(db);

            // Share the Arc<Database> as managed state.
            app.manage(db_arc.clone());

            // Spawn background sync engine.
            let engine = SyncEngine::spawn(app.handle().clone(), db_arc, DEFAULT_SYNC_INTERVAL);
            app.manage(engine);

            info!("Application setup complete");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_issues,
            commands::get_issue,
            commands::get_issue_counts,
            commands::get_preferences,
            commands::save_preferences,
            commands::get_workspace_info,
            commands::seed_mock_data,
            commands::auth::jira_connect,
            commands::auth::jira_disconnect,
            commands::auth::jira_sync,
            commands::auth::jira_restore_session,
            commands::boards::list_boards,
            commands::boards::list_saved_boards,
            commands::boards::save_board,
            commands::boards::unsave_board,
            commands::boards::get_board_issues,
            commands::boards::get_transitions,
            commands::boards::transition_issue,
            commands::boards::add_comment,
            commands::oauth::oauth_start,
            commands::oauth::oauth_disconnect,
            commands::oauth::oauth_refresh,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Jira Personal");
}
