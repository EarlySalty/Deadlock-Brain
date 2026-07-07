use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceDocument {
    pub id: i64,
    pub source: String,
    pub external_id: String,
    pub title: Option<String>,
    pub url: Option<String>,
    pub content_type: String,
    pub raw_path: String,
    pub content_hash: String,
    pub fetched_at: i64,
    pub metadata_json: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceRun {
    pub id: i64,
    pub source: String,
    pub status: String,
    pub started_at: i64,
    pub finished_at: Option<i64>,
    pub summary_json: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchEvent {
    pub id: i64,
    pub patch_snapshot_id: i64,
    pub patch_external_id: String,
    pub patch_title: Option<String>,
    pub patch_url: Option<String>,
    pub source_kind: String,
    pub posted_at: Option<String>,
    pub line_index: i64,
    pub section: Option<String>,
    pub entity_type: String,
    pub entity_name: Option<String>,
    pub subject: Option<String>,
    pub change_type: String,
    pub raw_line: String,
    pub normalized_line: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub confidence: f64,
    pub metadata_json: String,
    pub event_hash: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entity {
    pub id: i64,
    pub entity_type: String,
    pub canonical_name: String,
    pub primary_external_id: Option<String>,
    pub source: String,
    pub first_snapshot_id: Option<i64>,
    pub metadata_json: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityAlias {
    pub id: i64,
    pub entity_id: i64,
    pub alias: String,
    pub alias_norm: String,
    pub alias_kind: String,
    pub source: String,
    pub external_id: Option<String>,
    pub snapshot_id: Option<i64>,
    pub created_at: i64,
}
