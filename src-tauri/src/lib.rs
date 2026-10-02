pub mod application;
pub mod commands;
pub mod domain;
pub mod infrastructure;
pub mod sync;
pub mod tray;
pub mod window_state;

use std::fs;
use std::sync::Arc;

use infrastructure::database::Database;
use sync::engine::SyncEngine;
use tauri::Manager;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
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

/// Default background sync interval in seconds (5 minutes).
const DEFAULT_SYNC_INTERVAL: u64 = 300;

/// Application version from Cargo.toml (embedded at compile time).
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Set up a global panic hook that logs the panic before aborting.
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        tracing::error!("PANIC: {info}");
        default_hook(info);
    }));

    // Determine log directory — use the platform's app-log directory if possible,
    // otherwise fall back to the current directory.
    let log_dir = dirs_next::data_dir()
        .map(|d| d.join("com.petlabco.jirapersonal").join("logs"))
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
    let _ = fs::create_dir_all(&log_dir);

    // Daily rotating file appender.
    let file_appender = tracing_appender::rolling::daily(&log_dir, "jira-personal.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,hyper=warn"));

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr))
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_writer(non_blocking),
        )
        .init();

    info!("Starting Jira Personal v{APP_VERSION}");
    info!("Logs directory: {}", log_dir.display());

    // Keep the guard alive for the lifetime of the application so logs flush.
    // We move it into the tauri run closure indirectly by holding it here.
    let _log_guard = _guard;

    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};

                    if event.state() == ShortcutState::Pressed
                        && shortcut.matches(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyJ)
                    {
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(),
        )
        .setup(|app| {
            let db = init_database(app);
            let db_arc = Arc::new(db);

            // Share the Arc<Database> as managed state.
            app.manage(db_arc.clone());

            // Restore window position/size from last session.
            if let Some(window) = app.get_webview_window("main") {
                window_state::restore(&window, &db_arc);
                window_state::attach_save_handlers(&window, db_arc.clone());
            }

            // Spawn background sync engine.
            let engine = SyncEngine::spawn(app.handle().clone(), db_arc, DEFAULT_SYNC_INTERVAL);
            app.manage(engine);

            // Set up system tray.
            tray::setup_tray(app.handle())?;

            // Register global shortcut: Cmd+Shift+J (macOS) / Ctrl+Shift+J (Win/Linux).
            #[cfg(desktop)]
            {
                use tauri_plugin_global_shortcut::GlobalShortcutExt;
                let shortcut = tauri_plugin_global_shortcut::Shortcut::new(
                    Some(
                        tauri_plugin_global_shortcut::Modifiers::SUPER
                            | tauri_plugin_global_shortcut::Modifiers::SHIFT,
                    ),
                    tauri_plugin_global_shortcut::Code::KeyJ,
                );
                if let Err(e) = app.global_shortcut().register(shortcut) {
                    tracing::warn!("Failed to register global shortcut Cmd+Shift+J: {e}");
                } else {
                    info!("Global shortcut Cmd+Shift+J registered");
                }
            }

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
            commands::get_app_version,
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
