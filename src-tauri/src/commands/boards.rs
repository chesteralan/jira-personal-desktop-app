use std::sync::Arc;

use tauri::State;
use tracing::info;

use crate::commands::auth::make_active_client;
use crate::domain::board::{Board, IssueTransition, SavedBoard};
use crate::domain::issue::IssueView;
use crate::infrastructure::database::Database;
use crate::infrastructure::error::AppError;

/// Fetch all boards visible to the user.
#[tauri::command]
pub async fn list_boards() -> Result<Vec<Board>, AppError> {
    let client = make_active_client()?;
    client.fetch_boards().await
}

/// Get the user's saved boards from local storage.
#[tauri::command]
pub fn list_saved_boards(db: State<'_, Arc<Database>>) -> Result<Vec<SavedBoard>, AppError> {
    db.list_saved_boards()
}

/// Save a board for quick access.
#[tauri::command]
pub fn save_board(db: State<'_, Arc<Database>>, board: SavedBoard) -> Result<(), AppError> {
    db.save_board(&board)?;
    info!("Saved board {} ({})", board.name, board.board_id);
    Ok(())
}

/// Remove a saved board.
#[tauri::command]
pub fn unsave_board(db: State<'_, Arc<Database>>, board_id: u32) -> Result<bool, AppError> {
    let removed = db.unsave_board(board_id)?;
    if removed {
        info!("Unsaved board {board_id}");
    }
    Ok(removed)
}

/// Fetch issues for a specific board.
#[tauri::command]
pub async fn get_board_issues(board_id: u32) -> Result<Vec<IssueView>, AppError> {
    let client = make_active_client()?;
    let issues = client.fetch_board_issues(board_id).await?;
    Ok(issues.iter().map(IssueView::from).collect())
}

/// Get available transitions for an issue.
#[tauri::command]
pub async fn get_transitions(issue_key: String) -> Result<Vec<IssueTransition>, AppError> {
    let client = make_active_client()?;
    client.get_transitions(&issue_key).await
}

/// Transition an issue to a new status.
#[tauri::command]
pub async fn transition_issue(issue_key: String, transition_id: String) -> Result<(), AppError> {
    let client = make_active_client()?;
    client.transition_issue(&issue_key, &transition_id).await
}

/// Add a comment to an issue.
#[tauri::command]
pub async fn add_comment(issue_key: String, body: String) -> Result<(), AppError> {
    let client = make_active_client()?;
    client.add_comment(&issue_key, &body).await
}
