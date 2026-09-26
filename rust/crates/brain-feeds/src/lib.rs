#![forbid(unsafe_code)]

pub mod build_publish;
pub mod deadlock_assets;
pub mod patchnotes;

use brain_contracts::SourceVisibility;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FeedError {
    #[error("invalid feed: {0}")]
    Invalid(String),
    #[error("quarantined source: {0}")]
    Quarantined(String),
    #[error(transparent)]
    Ingestion(#[from] brain_ingestion::IngestionError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, FeedError>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FeedPolicy {
    pub visibility: SourceVisibility,
    pub allowed_scopes: BTreeSet<String>,
    pub provider_egress_allowed: bool,
    pub publication_allowed: bool,
    pub raw_retention_allowed: bool,
}

pub(crate) fn sha256_hex(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

pub(crate) fn configuration(source_id: &str, parser: &str, policy: &FeedPolicy) -> Result<String> {
    Ok(sha256_hex(
        serde_json::to_string(&(source_id, parser, policy))?.as_bytes(),
    ))
}

pub(crate) fn clean(value: &str) -> String {
    value
        .chars()
        .map(|c| if c.is_control() && c != '\n' { ' ' } else { c })
        .collect::<String>()
        .trim()
        .to_string()
}
