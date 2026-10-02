//! Persist and restore the main window position and size.

use std::sync::Arc;

use tauri::{PhysicalPosition, PhysicalSize, WebviewWindow};
use tracing::{info, warn};

use crate::infrastructure::database::Database;

/// Save the current window geometry to the database.
pub fn save(window: &WebviewWindow, db: &Database) {
    let Ok(pos) = window.outer_position() else {
        return;
    };
    let Ok(size) = window.outer_size() else {
        return;
    };

    let state = format!("{},{},{},{}", pos.x, pos.y, size.width, size.height);
    if let Err(e) = db.set_workspace_meta("window_state", &state) {
        warn!("Failed to save window state: {e}");
    }
}

/// Restore the window geometry from the database, if saved.
pub fn restore(window: &WebviewWindow, db: &Database) {
    let Ok(Some(state)) = db.get_workspace_meta("window_state") else {
        return;
    };

    let parts: Vec<&str> = state.split(',').collect();
    if parts.len() != 4 {
        return;
    }

    let Ok(x) = parts[0].parse::<i32>() else {
        return;
    };
    let Ok(y) = parts[1].parse::<i32>() else {
        return;
    };
    let Ok(w) = parts[2].parse::<u32>() else {
        return;
    };
    let Ok(h) = parts[3].parse::<u32>() else {
        return;
    };

    // Sanity: don't restore tiny windows.
    if w < 400 || h < 300 {
        return;
    }

    let _ = window.set_position(PhysicalPosition::new(x, y));
    let _ = window.set_size(PhysicalSize::new(w, h));
    info!("Restored window state: {x},{y} {w}x{h}");
}

/// Attach window event listeners to persist geometry on move/resize.
pub fn attach_save_handlers(window: &WebviewWindow, db: Arc<Database>) {
    window.on_window_event(move |event| match event {
        tauri::WindowEvent::Moved(_) | tauri::WindowEvent::Resized(_) => {
            // We need the window to read its current geometry, but we only
            // have the DB here. We write a debounced meta key from the event data.
            if let tauri::WindowEvent::Moved(pos) = event {
                // Save position component; size may arrive separately.
                let _ = db.set_workspace_meta("_win_x", &pos.x.to_string());
                let _ = db.set_workspace_meta("_win_y", &pos.y.to_string());
            }
            if let tauri::WindowEvent::Resized(size) = event {
                let _ = db.set_workspace_meta("_win_w", &size.width.to_string());
                let _ = db.set_workspace_meta("_win_h", &size.height.to_string());
            }
            // Compose full state from individual components.
            if let (Ok(Some(x)), Ok(Some(y)), Ok(Some(w)), Ok(Some(h))) = (
                db.get_workspace_meta("_win_x"),
                db.get_workspace_meta("_win_y"),
                db.get_workspace_meta("_win_w"),
                db.get_workspace_meta("_win_h"),
            ) {
                let _ = db.set_workspace_meta("window_state", &format!("{x},{y},{w},{h}"));
            }
        }
        _ => {}
    });
}
