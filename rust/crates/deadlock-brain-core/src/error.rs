use std::{num, time::SystemTimeError};

use reqwest::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP request failed: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("system clock error: {0}")]
    Time(#[from] SystemTimeError),

    #[error("invalid integer env {name}={value:?}: {message}")]
    InvalidIntegerEnv {
        name: &'static str,
        value: String,
        message: String,
    },

    #[error("invalid float env {name}={value:?}: {message}")]
    InvalidFloatEnv {
        name: &'static str,
        value: String,
        message: String,
    },

    #[error("invalid HTTP header {name:?}: {message}")]
    InvalidHeader { name: String, message: String },

    #[error("HTTP {status} while loading {url}: {body}")]
    HttpStatus {
        status: StatusCode,
        url: String,
        body: String,
    },

    #[error("MiniMax API-Key fehlt. Setze MINIMAX_API_KEY oder MINIMAX_TOKEN_PLAN_KEY.")]
    MissingMiniMaxApiKey,
}

impl CoreError {
    pub(crate) fn invalid_integer_env(
        name: &'static str,
        value: String,
        error: num::ParseIntError,
    ) -> Self {
        Self::InvalidIntegerEnv {
            name,
            value,
            message: error.to_string(),
        }
    }

    pub(crate) fn invalid_float_env(
        name: &'static str,
        value: String,
        error: num::ParseFloatError,
    ) -> Self {
        Self::InvalidFloatEnv {
            name,
            value,
            message: error.to_string(),
        }
    }
}

pub type Result<T> = std::result::Result<T, CoreError>;
