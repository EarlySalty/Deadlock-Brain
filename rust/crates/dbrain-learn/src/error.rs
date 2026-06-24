use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum LearnError {
    #[error(transparent)]
    Core(#[from] deadlock_brain_core::CoreError),

    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("{0}")]
    InvalidInput(String),

    #[error("MiniMax response did not include message content.")]
    EmptyMiniMaxResponse,

    #[error("steam db not found: {0}")]
    SteamDbNotFound(PathBuf),
}

pub type Result<T> = std::result::Result<T, LearnError>;
