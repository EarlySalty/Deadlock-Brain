use std::collections::{BTreeMap, HashMap, HashSet};

use rusqlite::{Connection, OptionalExtension};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

use crate::{Result, NormalizeError};

pub const ASSETS_SOURCE: &str = "deadlock_assets_api";
pub const SHEET_SOURCE: &str = "deadlock_stats_sheet";

pub fn now() -> Result<i64> {
    Ok(deadlock_brain_core::db::now_epoch_seconds()?)
}

pub fn normalize_alias(value: &str) -> String {
    value
        .trim()
        .to_lowercase()
        .replace('_', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn clean_subject(value: Option<&str>) -> String {
    let Some(value) = value else {
        return String::new();
    };
    let mut cleaned = value.trim();
    if let Some(stripped) = cleaned.strip_prefix("**") {
        cleaned = stripped;
    }
    if let Some(stripped) = cleaned.strip_suffix("**") {
        cleaned = stripped;
    }
    cleaned.trim_matches(|ch: char| ch == ' ' || ch == ':' || ch == '-' || ch == '\t').to_string()
}

pub fn normalize_key(value: &str) -> String {
    let cleaned = clean_subject(Some(value)).to_lowercase().replace('&', " and ");
    let mut out = String::new();
    let mut last_space = true;
    for ch in cleaned.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
            last_space = false;
        } else if !last_space {
            out.push(' ');
            last_space = true;
        }
    }
    out.trim().to_string()
}

pub fn normalize_scan_text(value: &str) -> String {
    format!(" {} ", normalize_key(value))
}

pub fn stable_hash_text(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn json_string(value: &Value) -> Result<String> {
    Ok(serde_json::to_string(value)?)
}

pub fn object_json_string(map: BTreeMap<String, Value>) -> Result<String> {
    let mut object = Map::new();
    for (key, value) in map {
        object.insert(key, value);
    }
    json_string(&Value::Object(object))
}

pub fn value_to_string(value: Option<&Value>) -> String {
    match value {
        Some(Value::String(text)) => text.clone(),
        Some(Value::Number(number)) => number.to_string(),
        Some(Value::Bool(flag)) => flag.to_string(),
        Some(Value::Null) | None => String::new(),
        Some(other) => other.to_string(),
    }
}

pub fn optional_value_to_string(value: Option<&Value>) -> Option<String> {
    let text = value_to_string(value).trim().to_string();
    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}

pub fn value_bool(value: Option<&Value>) -> Option<bool> {
    match value {
        Some(Value::Bool(flag)) => Some(*flag),
        _ => None,
    }
}

pub fn value_truthy(value: Option<&Value>) -> bool {
    match value {
        Some(Value::Bool(flag)) => *flag,
        Some(Value::Number(number)) => {
            number.as_i64().map_or_else(
                || number.as_f64().is_some_and(|value| value != 0.0),
                |value| value != 0,
            )
        }
        Some(Value::String(text)) => !text.is_empty(),
        Some(Value::Array(values)) => !values.is_empty(),
        Some(Value::Object(values)) => !values.is_empty(),
        Some(Value::Null) | None => false,
    }
}

pub fn parse_float(value: &str) -> Option<f64> {
    let mut cleaned = value.replace([',', '%'], "").trim().to_string();
    if let Some(stripped) = cleaned.strip_prefix('+') {
        cleaned = stripped.trim().to_string();
    }
    if cleaned.is_empty() {
        return None;
    }
    cleaned.parse::<f64>().ok()
}

pub fn parse_int(value: &str) -> Option<i64> {
    parse_float(value).map(|value| value as i64)
}

pub fn parse_bool(value: &str) -> bool {
    matches!(value.trim().to_lowercase().as_str(), "true" | "1" | "yes")
}

pub fn payload_values(payload: &Value) -> Option<&Map<String, Value>> {
    payload.get("values").and_then(Value::as_object)
}

pub fn row_number(value: Option<&Value>) -> Option<i64> {
    match value {
        Some(Value::Number(number)) => number.as_i64(),
        Some(Value::String(text)) => text.parse::<i64>().ok(),
        _ => None,
    }
}

pub fn title_case(value: &str) -> String {
    let mut out = String::new();
    let mut upper_next = true;
    for ch in value.chars() {
        if ch.is_ascii_alphabetic() {
            if upper_next {
                out.push(ch.to_ascii_uppercase());
            } else {
                out.push(ch);
            }
            upper_next = false;
        } else {
            out.push(ch);
            upper_next = !ch.is_ascii_alphanumeric();
        }
    }
    out
}

pub fn table_count(conn: &Connection, table: &str) -> Result<i64> {
    let sql = format!("SELECT COUNT(*) FROM {table}");
    Ok(conn.query_row(&sql, [], |row| row.get(0))?)
}

pub fn table_exists(conn: &Connection, table: &str) -> Result<bool> {
    Ok(conn
        .query_row(
            "SELECT 1 FROM sqlite_master WHERE type='table' AND name=?1",
            [table],
            |_| Ok(()),
        )
        .optional()?
        .is_some())
}

pub fn delete_all(conn: &Connection, table: &'static str) -> Result<i64> {
    let sql = format!("DELETE FROM {table}");
    let count = conn.execute(&sql, [])?;
    Ok(count as i64)
}

pub fn clear_patch_events(conn: &Connection) -> Result<i64> {
    for table in ["legacy_entities", "entity_lineage", "patch_event_enrichments"] {
        if table_exists(conn, table)? {
            let _ = delete_all(conn, table)?;
        }
    }
    delete_all(conn, "patch_events")
}

pub fn build_hero_index(conn: &Connection, skip_internal_aliases: bool) -> Result<HashMap<String, i64>> {
    let mut candidates: HashMap<String, HashSet<i64>> = HashMap::new();
    let mut entity_stmt = conn.prepare(
        "SELECT id, canonical_name FROM entities WHERE entity_type='hero'",
    )?;
    let entity_rows = entity_stmt.query_map([], |row| {
        Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
    })?;
    for row in entity_rows {
        let (entity_id, canonical_name) = row?;
        add_index_candidate(&mut candidates, &normalize_alias(&canonical_name), entity_id);
    }

    let mut alias_stmt = conn.prepare(
        r#"
        SELECT entity_id, alias, alias_norm, alias_kind
        FROM entity_aliases
        WHERE entity_id IN (SELECT id FROM entities WHERE entity_type='hero')
        "#,
    )?;
    let alias_rows = alias_stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
        ))
    })?;
    for row in alias_rows {
        let (entity_id, alias, alias_norm, alias_kind) = row?;
        if alias_kind != "canonical" && alias_kind != "snapshot_name" {
            continue;
        }
        if skip_internal_aliases && looks_like_internal_hero_alias(&alias) {
            continue;
        }
        add_index_candidate(&mut candidates, &alias_norm, entity_id);
    }

    Ok(candidates
        .into_iter()
        .filter_map(|(alias, ids)| {
            if ids.len() == 1 {
                ids.into_iter().next().map(|id| (alias, id))
            } else {
                None
            }
        })
        .collect())
}

pub fn add_index_candidate(candidates: &mut HashMap<String, HashSet<i64>>, alias_norm: &str, entity_id: i64) {
    if !alias_norm.is_empty() {
        candidates.entry(alias_norm.to_string()).or_default().insert(entity_id);
    }
}

fn looks_like_internal_hero_alias(value: &str) -> bool {
    let lowered = value.trim().to_lowercase();
    lowered.starts_with("hero_") || lowered.starts_with("hero ")
}

pub fn get_required_id(row: Option<i64>, label: &'static str) -> Result<i64> {
    row.ok_or(NormalizeError::MissingRow(label))
}
