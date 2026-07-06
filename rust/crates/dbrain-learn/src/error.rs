#[derive(Debug, thiserror::Error)]
pub enum LearnError {
    #[error(transparent)]
    Core(#[from] deadlock_brain_core::CoreError),

    #[error("Postgres error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("{0}")]
    InvalidInput(String),

    #[error("MiniMax response did not include message content.")]
    EmptyMiniMaxResponse,
}

pub type Result<T> = std::result::Result<T, LearnError>;
