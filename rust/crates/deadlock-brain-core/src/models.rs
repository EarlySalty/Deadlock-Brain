use rusqlite::Row;
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

impl SourceDocument {
    pub fn from_row(row: &Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            source: row.get("source")?,
            external_id: row.get("external_id")?,
            title: row.get("title")?,
            url: row.get("url")?,
            content_type: row.get("content_type")?,
            raw_path: row.get("raw_path")?,
            content_hash: row.get("content_hash")?,
            fetched_at: row.get("fetched_at")?,
            metadata_json: row.get("metadata_json")?,
        })
    }
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

impl SourceRun {
    pub fn from_row(row: &Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            source: row.get("source")?,
            status: row.get("status")?,
            started_at: row.get("started_at")?,
            finished_at: row.get("finished_at")?,
            summary_json: row.get("summary_json")?,
        })
    }
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

impl PatchEvent {
    pub fn from_row(row: &Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            patch_snapshot_id: row.get("patch_snapshot_id")?,
            patch_external_id: row.get("patch_external_id")?,
            patch_title: row.get("patch_title")?,
            patch_url: row.get("patch_url")?,
            source_kind: row.get("source_kind")?,
            posted_at: row.get("posted_at")?,
            line_index: row.get("line_index")?,
            section: row.get("section")?,
            entity_type: row.get("entity_type")?,
            entity_name: row.get("entity_name")?,
            subject: row.get("subject")?,
            change_type: row.get("change_type")?,
            raw_line: row.get("raw_line")?,
            normalized_line: row.get("normalized_line")?,
            old_value: row.get("old_value")?,
            new_value: row.get("new_value")?,
            confidence: row.get("confidence")?,
            metadata_json: row.get("metadata_json")?,
            event_hash: row.get("event_hash")?,
            created_at: row.get("created_at")?,
        })
    }
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

impl Entity {
    pub fn from_row(row: &Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            entity_type: row.get("entity_type")?,
            canonical_name: row.get("canonical_name")?,
            primary_external_id: row.get("primary_external_id")?,
            source: row.get("source")?,
            first_snapshot_id: row.get("first_snapshot_id")?,
            metadata_json: row.get("metadata_json")?,
            created_at: row.get("created_at")?,
            updated_at: row.get("updated_at")?,
        })
    }
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

impl EntityAlias {
    pub fn from_row(row: &Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            entity_id: row.get("entity_id")?,
            alias: row.get("alias")?,
            alias_norm: row.get("alias_norm")?,
            alias_kind: row.get("alias_kind")?,
            source: row.get("source")?,
            external_id: row.get("external_id")?,
            snapshot_id: row.get("snapshot_id")?,
            created_at: row.get("created_at")?,
        })
    }
}
