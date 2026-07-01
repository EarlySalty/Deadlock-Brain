#![forbid(unsafe_code)]

//! Normalisierungs-Crate fuer Entities, Patchnotes, Sheet-Stats und Sheet-Tabs.

mod entities;
mod error;
mod forum_claims;
mod legacy;
mod lineage;
mod patch;
mod resolve_gaps;
mod sheet_stats;
mod sheet_tabs;
mod util;

use rusqlite::Connection;
use serde_json::Value;

pub use deadlock_brain_core as core;
pub use error::{NormalizeError, Result};
pub use util::normalize_alias;

pub fn parse_patchnotes(rebuild: bool) -> Result<Value> {
    let conn = deadlock_brain_core::db::open_connection(None)?;
    parse_patchnotes_with_conn(&conn, rebuild)
}

pub fn parse_patchnotes_with_conn(conn: &Connection, rebuild: bool) -> Result<Value> {
    patch::parse_patchnotes(conn, rebuild)
}

pub fn parse_forum_claims_with_conn(conn: &Connection, rebuild: bool) -> Result<Value> {
    forum_claims::parse_forum_claims(conn, rebuild)
}

pub fn normalize_entities(rebuild: bool) -> Result<Value> {
    let conn = deadlock_brain_core::db::open_connection(None)?;
    normalize_entities_with_conn(&conn, rebuild)
}

pub fn normalize_entities_with_conn(conn: &Connection, rebuild: bool) -> Result<Value> {
    entities::normalize_entities(conn, rebuild)
}

pub fn normalize_sheet_stats(rebuild: bool) -> Result<Value> {
    let conn = deadlock_brain_core::db::open_connection(None)?;
    normalize_sheet_stats_with_conn(&conn, rebuild)
}

pub fn normalize_sheet_stats_with_conn(conn: &Connection, rebuild: bool) -> Result<Value> {
    sheet_stats::normalize_sheet_stats(conn, rebuild)
}

pub fn normalize_sheet_tabs(rebuild: bool) -> Result<Value> {
    let conn = deadlock_brain_core::db::open_connection(None)?;
    normalize_sheet_tabs_with_conn(&conn, rebuild)
}

pub fn normalize_sheet_tabs_with_conn(conn: &Connection, rebuild: bool) -> Result<Value> {
    sheet_tabs::normalize_sheet_tabs(conn, rebuild)
}

pub fn resolve_gaps_with_conn(conn: &Connection, dry_run: bool) -> Result<Value> {
    resolve_gaps::resolve_gaps(conn, dry_run)
}

pub fn enrich_lineage(rebuild: bool) -> Result<Value> {
    let conn = deadlock_brain_core::db::open_connection(None)?;
    enrich_lineage_with_conn(&conn, rebuild)
}

pub fn enrich_lineage_with_conn(conn: &Connection, rebuild: bool) -> Result<Value> {
    lineage::enrich_lineage(conn, rebuild)
}

pub fn enrich_legacy_entities(rebuild: bool) -> Result<Value> {
    let conn = deadlock_brain_core::db::open_connection(None)?;
    enrich_legacy_entities_with_conn(&conn, rebuild)
}

pub fn enrich_legacy_entities_with_conn(conn: &Connection, rebuild: bool) -> Result<Value> {
    legacy::enrich_legacy_entities(conn, rebuild)
}

pub use lineage::{extract_lineage_candidates, LineageCandidate, LineageEvent};

#[cfg(test)]
mod tests;
