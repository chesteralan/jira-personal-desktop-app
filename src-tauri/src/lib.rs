pub mod application;
pub mod commands;
pub mod domain;
pub mod infrastructure;
pub mod sync;

use std::fs;

use infrastructure::database::Database;
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
            app.manage(db);
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running Jira Personal");
}
