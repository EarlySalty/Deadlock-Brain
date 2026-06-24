use std::time::SystemTimeError;

#[derive(Debug, thiserror::Error)]
pub enum SourcesError {
    #[error(transparent)]
    Core(#[from] deadlock_brain_core::CoreError),

    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("system clock error: {0}")]
    Time(#[from] SystemTimeError),

    #[error("{0}")]
    InvalidInput(String),

    #[error("Zentrale Deadlock-DB nicht gefunden: {0}")]
    CentralDbNotFound(String),

    #[error("{0}")]
    Invariant(String),
}

impl SourcesError {
    pub(crate) fn invalid_input(message: impl Into<String>) -> Self {
        Self::InvalidInput(message.into())
    }

    pub(crate) fn invariant(message: impl Into<String>) -> Self {
        Self::Invariant(message.into())
    }
}

pub type Result<T> = std::result::Result<T, SourcesError>;
