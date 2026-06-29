#![forbid(unsafe_code)]

//! Deterministische Build-Engine aus Deadlock-API-Daten.

use anyhow::Result;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

mod api;
pub mod classify;
mod engine;
mod error;
pub mod schema;
mod sync;
mod util;

pub use sync::{sync_build_data, BuildDataSyncOptions, BuildDataSyncSummary};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BuildContext {
    pub hero_id: i64,
    pub hero_name: String,
    pub hero_archetype: String,
    pub hero_base_health: Option<f64>,
    pub playstyle: Option<String>,
    pub primary_path: BuildPath,
    pub alternative_paths: Vec<BuildPathSummary>,
    pub ability_order: Option<Vec<i64>>,
    pub generated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BuildPath {
    pub label: String,
    pub winrate: Option<f64>,
    pub sample_matches: i64,
    pub phases: Vec<BuildPhase>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BuildPathSummary {
    pub label: String,
    pub winrate: Option<f64>,
    pub sample_matches: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BuildPhase {
    pub phase: String,
    pub items: Vec<ItemDossier>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ItemDossier {
    pub item_id: i64,
    pub name: String,
    pub slot_type: String,
    pub tier: i64,
    pub defense_kind: Vec<String>,
    pub damage_axis: String,
    pub prevalence_builds: i64,
    pub winrate: Option<f64>,
    pub sample_matches: i64,
    pub lift_pp: Option<f64>,
    pub buy_phase: String,
    pub synergy_with: Vec<String>,
    pub confidence: String,
}

pub fn build_context(
    conn: &Connection,
    hero_query: &str,
    playstyle: Option<&str>,
) -> Result<BuildContext> {
    engine::build_context(conn, hero_query, playstyle)
}
