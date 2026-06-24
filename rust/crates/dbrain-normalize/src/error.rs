use thiserror::Error;

#[derive(Debug, Error)]
pub enum NormalizeError {
    #[error("Core error: {0}")]
    Core(#[from] deadlock_brain_core::CoreError),

    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("missing row after upsert: {0}")]
    MissingRow(&'static str),
}

pub type Result<T> = std::result::Result<T, NormalizeError>;
