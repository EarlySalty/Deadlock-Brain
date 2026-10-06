use std::time::SystemTimeError;

use reqwest::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error(transparent)]
    Config(#[from] crate::bot_config::ConfigError),

    #[error(transparent)]
    ModelSelection(#[from] crate::model_resolver::SelectionError),

    #[error("KI-Anfrageparameter sind ungültig oder widersprechen dem freigegebenen Endpunkt.")]
    InvalidAiOverrides,

    #[error("Fireworks-Anfrage fehlgeschlagen: {kind}, HTTP {status}. Antwortinhalte werden nicht ausgegeben.")]
    AiResponse { status: u16, kind: &'static str },

    #[error("Der KI-Pfad hat kein freigegebenes, geprüftes Modell. Es wurde keine KI-Anfrage gesendet.")]
    NoVerifiedModel,

    #[error("Anfragemodell und Client-Momentaufnahme stimmen nicht überein. Es wurde keine KI-Anfrage gesendet.")]
    ModelSnapshotMismatch,

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

    #[error("Fireworks-Zugang fehlt im geschützten Infisical-Bootstrap.")]
    MissingFireworksApiKey,
}

pub type Result<T> = std::result::Result<T, CoreError>;
