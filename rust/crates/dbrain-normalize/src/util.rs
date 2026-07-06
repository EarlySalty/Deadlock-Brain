use std::collections::{BTreeMap, HashMap, HashSet};

use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use sqlx::PgPool;

use crate::{NormalizeError, Result};

pub const ASSETS_SOURCE: &str = "deadlock_assets_api";
pub const SHEET_SOURCE: &str = "deadlock_stats_sheet";

/// Tabellen im `brain`-Schema, deren Namen die dynamischen Helfer
/// ([`table_count`]/[`delete_all`]) interpolieren duerfen. Fremde/dynamische
/// Bezeichner werden abgewiesen, damit kein ungeprueftes `format!` in SQL landet.
const MUTABLE_TABLES: &[&str] = &[
    "patch_events",
    "entity_lineage",
    "legacy_entities",
    "patch_event_enrichments",
    "entities",
    "entity_aliases",
];

/// Quotet einen whitelisteten Tabellennamen zum voll qualifizierten
/// `brain.<name>`. Nicht gelistete Namen sind ein Fehler (kein rohes `format!`
/// von Fremdinput in SQL).
fn qualified_table(table: &str) -> Result<String> {
    if MUTABLE_TABLES.contains(&table) {
        Ok(format!("brain.{table}"))
    } else {
        Err(NormalizeError::InvalidTable(table.to_string()))
    }
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

/// Bind-Parameter fuer die wenigen echt dynamischen Laufzeit-Upserts
/// ([`crate::sheet_tabs`]): dort variiert die Spaltenliste (Statistik-Maps),
/// also wird das SQL zur Laufzeit gebaut und ueber `sqlx::query` gebunden.
/// Der `Option`-Typ traegt die Spaltentypinfo mit, damit NULLs korrekt an
/// bigint- bzw. double-Spalten gehen.
#[derive(Debug, Clone)]
pub(crate) enum SqlParam {
    Int(i64),
    IntOpt(Option<i64>),
    FloatOpt(Option<f64>),
    Text(String),
    TextOpt(Option<String>),
}

pub(crate) fn bind_params<'q>(
    mut query: sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments>,
    params: &'q [SqlParam],
) -> sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments> {
    for param in params {
        query = match param {
            SqlParam::Int(value) => query.bind(*value),
            SqlParam::IntOpt(value) => query.bind(*value),
            SqlParam::FloatOpt(value) => query.bind(*value),
            SqlParam::Text(value) => query.bind(value.as_str()),
            SqlParam::TextOpt(value) => query.bind(value.as_deref()),
        };
    }
    query
}

pub async fn table_count(pool: &PgPool, table: &str) -> Result<i64> {
    let sql = format!("SELECT COUNT(*) FROM {}", qualified_table(table)?);
    Ok(sqlx::query_scalar::<_, i64>(&sql).fetch_one(pool).await?)
}

pub async fn table_exists(pool: &PgPool, table: &str) -> Result<bool> {
    let qualified = format!("brain.{table}");
    let present: bool = sqlx::query_scalar("SELECT to_regclass($1) IS NOT NULL")
        .bind(qualified)
        .fetch_one(pool)
        .await?;
    Ok(present)
}

pub async fn delete_all(pool: &PgPool, table: &str) -> Result<i64> {
    let sql = format!("DELETE FROM {}", qualified_table(table)?);
    let result = sqlx::query(&sql).execute(pool).await?;
    Ok(result.rows_affected() as i64)
}

pub async fn clear_patch_events(pool: &PgPool) -> Result<i64> {
    for table in ["legacy_entities", "entity_lineage", "patch_event_enrichments"] {
        if table_exists(pool, table).await? {
            let _ = delete_all(pool, table).await?;
        }
    }
    delete_all(pool, "patch_events").await
}

pub async fn build_hero_index(
    pool: &PgPool,
    skip_internal_aliases: bool,
) -> Result<HashMap<String, i64>> {
    let mut candidates: HashMap<String, HashSet<i64>> = HashMap::new();
    let entity_rows = sqlx::query!(
        r#"SELECT id AS "id!", canonical_name AS "canonical_name!"
           FROM brain.entities WHERE entity_type = 'hero'"#,
    )
    .fetch_all(pool)
    .await?;
    for row in entity_rows {
        add_index_candidate(&mut candidates, &normalize_alias(&row.canonical_name), row.id);
    }

    let alias_rows = sqlx::query!(
        r#"
        SELECT a.entity_id AS "entity_id!", a.alias AS "alias!",
               a.alias_norm AS "alias_norm!", a.alias_kind AS "alias_kind!"
        FROM brain.entity_aliases a
        WHERE a.entity_id IN (SELECT id FROM brain.entities WHERE entity_type = 'hero')
        "#,
    )
    .fetch_all(pool)
    .await?;
    for row in alias_rows {
        if row.alias_kind != "canonical" && row.alias_kind != "snapshot_name" {
            continue;
        }
        if skip_internal_aliases && looks_like_internal_hero_alias(&row.alias) {
            continue;
        }
        add_index_candidate(&mut candidates, &row.alias_norm, row.entity_id);
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

fn add_index_candidate(
    candidates: &mut HashMap<String, HashSet<i64>>,
    alias_norm: &str,
    entity_id: i64,
) {
    if !alias_norm.is_empty() {
        candidates.entry(alias_norm.to_string()).or_default().insert(entity_id);
    }
}

fn looks_like_internal_hero_alias(value: &str) -> bool {
    let lowered = value.trim().to_lowercase();
    lowered.starts_with("hero_") || lowered.starts_with("hero ")
}
