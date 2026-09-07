#![forbid(unsafe_code)]

//! Normalisierungs-Crate fuer Entities, Patchnotes, Sheet-Stats und Sheet-Tabs.
//!
//! Alle Schreib-/Lesepfade laufen ueber `sqlx`/[`PgPool`] (async) gegen das
//! `brain`-Schema.

mod entities;
mod error;
mod forum_claims;
mod legacy;
mod lineage;
mod patch;
mod reddit_claims;
mod resolve_gaps;
mod sheet_stats;
mod sheet_tabs;
mod util;

use serde_json::Value;
use sqlx::PgPool;

pub use deadlock_brain_core as core;
pub use error::{NormalizeError, Result};
pub use patch::classify_change_type;
pub use util::normalize_alias;

pub async fn parse_patchnotes(pool: &PgPool, rebuild: bool) -> Result<Value> {
    patch::parse_patchnotes(pool, rebuild).await
}

pub async fn normalize_entities(pool: &PgPool, rebuild: bool) -> Result<Value> {
    entities::normalize_entities(pool, rebuild).await
}

pub async fn parse_forum_claims(pool: &PgPool, rebuild: bool) -> Result<Value> {
    forum_claims::parse_forum_claims(pool, rebuild).await
}

pub async fn parse_reddit_claims(pool: &PgPool, rebuild: bool) -> Result<Value> {
    reddit_claims::parse_reddit_claims(pool, rebuild).await
}

pub async fn normalize_sheet_stats(pool: &PgPool, rebuild: bool) -> Result<Value> {
    sheet_stats::normalize_sheet_stats(pool, rebuild).await
}

pub async fn normalize_sheet_tabs(pool: &PgPool, rebuild: bool) -> Result<Value> {
    sheet_tabs::normalize_sheet_tabs(pool, rebuild).await
}

pub async fn resolve_gaps(pool: &PgPool, dry_run: bool) -> Result<Value> {
    resolve_gaps::resolve_gaps(pool, dry_run).await
}

pub async fn enrich_lineage(pool: &PgPool, rebuild: bool) -> Result<Value> {
    lineage::enrich_lineage(pool, rebuild).await
}

pub async fn enrich_legacy_entities(pool: &PgPool, rebuild: bool) -> Result<Value> {
    legacy::enrich_legacy_entities(pool, rebuild).await
}

pub use lineage::{extract_lineage_candidates, LineageCandidate, LineageEvent};

#[cfg(test)]
mod tests;
