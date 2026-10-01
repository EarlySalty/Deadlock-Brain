use std::time::SystemTimeError;

use reqwest::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("{0}")]
    ModelSelection(String),
    #[error("{0}")]
    Configuration(#[from] crate::bot_config::ConfigError),
    #[error("Die KI-Anfrage muss ein JSON-Objekt sein.")]
    InvalidAiRequest,
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP request failed: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("system clock error: {0}")]
    Time(#[from] SystemTimeError),

    #[error("invalid HTTP header {name:?}: {message}")]
    InvalidHeader { name: String, message: String },

    #[error("HTTP {status} while loading {url}: {body}")]
    HttpStatus {
        status: StatusCode,
        url: String,
        body: String,
    },

    #[error("Fireworks API-Key fehlt. Setze FIREWORK_API_KEY oder FIREWORKS_API_KEY.")]
    MissingFireworksApiKey,
}

pub type Result<T> = std::result::Result<T, CoreError>;
