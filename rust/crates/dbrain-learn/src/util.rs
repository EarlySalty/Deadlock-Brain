use std::collections::{BTreeMap, BTreeSet};

use serde_json::{json, Map, Number, Value};
use sha2::{Digest, Sha256};
use sqlx::{
    postgres::{PgColumn, PgPool, PgRow},
    Column, Row, TypeInfo,
};

use crate::Result;

/// Bind-Parameter fuer die dynamischen Laufzeit-Queries (`sqlx::query`).
///
/// Die Lern-Crate baut ihre Ergebniszeilen bewusst dynamisch als
/// `serde_json::Value` (Payloads sind JSON). Deshalb laufen die Queries ueber
/// `sqlx::query` (Laufzeit) statt ueber die typgepruefte `query!`-Makro-Form;
/// die Bindings werden ueber diese kleine Parameter-Abstraktion gesetzt.
#[derive(Debug, Clone)]
pub(crate) enum SqlParam {
    Int(i64),
    IntOpt(Option<i64>),
    Float(f64),
    Text(String),
    TextOpt(Option<String>),
    IntArray(Vec<i64>),
    TextArray(Vec<String>),
}

fn bind_params<'q>(
    mut query: sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments>,
    params: &'q [SqlParam],
) -> sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments> {
    for param in params {
        query = match param {
            SqlParam::Int(value) => query.bind(*value),
            SqlParam::IntOpt(value) => query.bind(*value),
            SqlParam::Float(value) => query.bind(*value),
            SqlParam::Text(value) => query.bind(value.as_str()),
            SqlParam::TextOpt(value) => query.bind(value.as_deref()),
            SqlParam::IntArray(value) => query.bind(value.as_slice()),
            SqlParam::TextArray(value) => query.bind(value.as_slice()),
        };
    }
    query
}

pub(crate) async fn query_json_rows(
    pool: &PgPool,
    sql: &str,
    params: &[SqlParam],
) -> Result<Vec<Value>> {
    let rows = bind_params(sqlx::query(sql), params)
        .fetch_all(pool)
        .await?;
    Ok(rows.iter().map(pg_row_to_json).collect())
}

pub(crate) async fn query_one_json(
    pool: &PgPool,
    sql: &str,
    params: &[SqlParam],
) -> Result<Option<Value>> {
    let row = bind_params(sqlx::query(sql), params)
        .fetch_optional(pool)
        .await?;
    Ok(row.as_ref().map(pg_row_to_json))
}

pub(crate) async fn execute_sql(pool: &PgPool, sql: &str, params: &[SqlParam]) -> Result<u64> {
    let result = bind_params(sqlx::query(sql), params).execute(pool).await?;
    Ok(result.rows_affected())
}

/// Prueft, ob eine Tabelle im `brain`-Schema existiert.
pub(crate) async fn table_exists(pool: &PgPool, name: &str) -> Result<bool> {
    let qualified = format!("brain.{name}");
    let row = sqlx::query("SELECT to_regclass($1) IS NOT NULL AS present")
        .bind(&qualified)
        .fetch_one(pool)
        .await?;
    Ok(row
        .try_get::<Option<bool>, _>("present")
        .ok()
        .flatten()
        .unwrap_or(false))
}

pub(crate) fn pg_row_to_json(row: &PgRow) -> Value {
    let mut object = Map::new();
    for column in row.columns() {
        object.insert(column.name().to_string(), decode_pg_column(row, column));
    }
    Value::Object(object)
}

fn decode_pg_column(row: &PgRow, column: &PgColumn) -> Value {
    let ordinal = column.ordinal();
    match column.type_info().name() {
        "INT8" | "INT4" | "INT2" => row
            .try_get::<Option<i64>, _>(ordinal)
            .ok()
            .flatten()
            .map(|value| json!(value))
            .unwrap_or(Value::Null),
        "FLOAT8" | "FLOAT4" => row
            .try_get::<Option<f64>, _>(ordinal)
            .ok()
            .flatten()
            .and_then(Number::from_f64)
            .map(Value::Number)
            .unwrap_or(Value::Null),
        "BOOL" => row
            .try_get::<Option<bool>, _>(ordinal)
            .ok()
            .flatten()
            .map(Value::Bool)
            .unwrap_or(Value::Null),
        _ => row
            .try_get::<Option<String>, _>(ordinal)
            .ok()
            .flatten()
            .map(Value::String)
            .unwrap_or(Value::Null),
    }
}

pub(crate) fn stable_hash_text(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub(crate) fn clamp_i64(value: i64, min: i64, max: i64) -> i64 {
    value.max(min).min(max)
}

pub(crate) fn clamp_usize(value: usize, min: usize, max: usize) -> usize {
    value.max(min).min(max)
}

pub(crate) fn as_object(value: &Value) -> Option<&Map<String, Value>> {
    value.as_object()
}

pub(crate) fn as_array(value: &Value) -> Option<&Vec<Value>> {
    value.as_array()
}

pub(crate) fn get<'a>(value: &'a Value, key: &str) -> Option<&'a Value> {
    as_object(value).and_then(|object| object.get(key))
}

pub(crate) fn get_any<'a>(value: &'a Value, keys: &[&str]) -> Option<&'a Value> {
    keys.iter().find_map(|key| get(value, key))
}

pub(crate) fn get_string(value: &Value, key: &str) -> Option<String> {
    get(value, key).and_then(value_to_non_empty_string)
}

pub(crate) fn get_any_string(value: &Value, keys: &[&str]) -> Option<String> {
    get_any(value, keys).and_then(value_to_non_empty_string)
}

pub(crate) fn value_to_string(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::String(text) => text.clone(),
        Value::Bool(flag) => flag.to_string(),
        Value::Number(number) => number.to_string(),
        other => other.to_string(),
    }
}

pub(crate) fn value_to_non_empty_string(value: &Value) -> Option<String> {
    let text = value_to_string(value).trim().to_string();
    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}

pub(crate) fn int_or_none(value: Option<&Value>) -> Option<i64> {
    value.and_then(|item| match item {
        Value::Number(number) => number.as_i64().or_else(|| number.as_f64().map(|value| value as i64)),
        Value::String(text) => text.trim().parse::<f64>().ok().map(|value| value as i64),
        Value::Bool(flag) => Some(i64::from(*flag)),
        Value::Null | Value::Array(_) | Value::Object(_) => None,
    })
}

pub(crate) fn int_or_zero(value: Option<&Value>) -> i64 {
    int_or_none(value).unwrap_or(0)
}

pub(crate) fn numeric_value(value: Option<&Value>) -> f64 {
    value
        .and_then(|item| match item {
            Value::Number(number) => number.as_f64(),
            Value::String(text) => text.replace(',', "").trim().parse::<f64>().ok(),
            Value::Bool(flag) => Some(f64::from(u8::from(*flag))),
            Value::Null | Value::Array(_) | Value::Object(_) => None,
        })
        .unwrap_or(0.0)
}

pub(crate) fn bool_value(value: Option<&Value>) -> bool {
    value
        .and_then(|item| match item {
            Value::Bool(flag) => Some(*flag),
            Value::Number(number) => number.as_i64().map(|value| value != 0),
            Value::String(text) => Some(matches!(
                text.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )),
            Value::Null | Value::Array(_) | Value::Object(_) => None,
        })
        .unwrap_or(false)
}

pub(crate) fn json_loads(value: Option<&str>, fallback: Value) -> Value {
    value
        .and_then(|text| serde_json::from_str::<Value>(text).ok())
        .unwrap_or(fallback)
}

pub(crate) fn loads_json_value(value: Option<&Value>, fallback: Value) -> Value {
    match value {
        Some(Value::Array(_)) | Some(Value::Object(_)) => value.cloned().unwrap_or(fallback),
        Some(Value::String(text)) => serde_json::from_str::<Value>(text).unwrap_or(fallback),
        Some(other) => serde_json::from_str::<Value>(&value_to_string(other)).unwrap_or(fallback),
        None => fallback,
    }
}

pub(crate) fn compact_json(value: &Value, max_depth: usize, max_list: usize) -> Value {
    if max_depth == 0 {
        if let Some(object) = value.as_object() {
            return json!({"_truncated": "object", "keys": object.keys().take(20).cloned().collect::<Vec<_>>()});
        }
        if let Some(array) = value.as_array() {
            return json!({"_truncated": "list", "count": array.len()});
        }
        return value.clone();
    }
    if let Some(object) = value.as_object() {
        let mut compact = Map::new();
        for (key, item) in object.iter().take(80) {
            compact.insert(key.clone(), compact_json(item, max_depth - 1, max_list));
        }
        return Value::Object(compact);
    }
    if let Some(array) = value.as_array() {
        return Value::Array(
            array
                .iter()
                .take(max_list)
                .map(|item| compact_json(item, max_depth - 1, max_list))
                .collect(),
        );
    }
    value.clone()
}

pub(crate) fn decode_json_fields(row: &Value) -> Value {
    let Some(object) = row.as_object() else {
        return row.clone();
    };
    let mut decoded = Map::new();
    for (key, value) in object {
        if key.ends_with("_json") {
            let target = key.trim_end_matches("_json").to_string();
            decoded.insert(target, loads_json_value(Some(value), json!({})));
        } else {
            decoded.insert(key.clone(), value.clone());
        }
    }
    Value::Object(decoded)
}

pub(crate) fn dedupe_strings(values: Vec<String>) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut result = Vec::new();
    for value in values {
        if value.is_empty() || !seen.insert(value.clone()) {
            continue;
        }
        result.push(value);
    }
    result
}

pub(crate) fn sorted_counts(values: Vec<String>) -> Value {
    let mut counts: BTreeMap<String, i64> = BTreeMap::new();
    for value in values {
        *counts.entry(value).or_default() += 1;
    }
    let mut rows: Vec<_> = counts.into_iter().collect();
    rows.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
    Value::Object(rows.into_iter().map(|(key, count)| (key, json!(count))).collect())
}

pub(crate) fn extract_insights(text: &str) -> Value {
    let mut latest = None;
    for (start, character) in text.char_indices() {
        if character != '{' {
            continue;
        }
        let Some(end) = balanced_json_end(text, start) else {
            continue;
        };
        let Some(candidate) = parse_insights_candidate(&text[start..end]) else {
            continue;
        };
        latest = Some(candidate);
    }
    latest.unwrap_or_else(|| json!({}))
}

fn balanced_json_end(text: &str, start: usize) -> Option<usize> {
    let mut depth = 0_i64;
    let mut in_string = false;
    let mut escaped = false;
    for (offset, character) in text[start..].char_indices() {
        if in_string {
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '"' {
                in_string = false;
            }
            continue;
        }
        match character {
            '"' => in_string = true,
            '{' => depth += 1,
            '}' => {
                if depth <= 0 {
                    return None;
                }
                depth -= 1;
                if depth == 0 {
                    return Some(start + offset + character.len_utf8());
                }
            }
            _ => {}
        }
    }
    None
}

fn parse_insights_candidate(raw: &str) -> Option<Value> {
    let parsed = parse_json_tolerant(raw)?;
    let insights = get(&parsed, "insights")
        .filter(|value| value.is_object())
        .cloned()
        .unwrap_or(parsed);
    if !insights.is_object() {
        return None;
    }
    if get(&insights, "core_items").is_some() || get(&insights, "hero_job").is_some() {
        Some(insights)
    } else {
        None
    }
}

fn parse_json_tolerant(raw: &str) -> Option<Value> {
    serde_json::from_str::<Value>(raw)
        .ok()
        .or_else(|| {
            let cleaned = strip_trailing_json_commas(raw);
            if cleaned == raw {
                None
            } else {
                serde_json::from_str::<Value>(&cleaned).ok()
            }
        })
}

fn strip_trailing_json_commas(raw: &str) -> String {
    let mut result = String::with_capacity(raw.len());
    let mut chars = raw.chars().peekable();
    let mut in_string = false;
    let mut escaped = false;
    while let Some(character) = chars.next() {
        if in_string {
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '"' {
                in_string = false;
            }
            result.push(character);
            continue;
        }
        if character == '"' {
            in_string = true;
            result.push(character);
            continue;
        }
        if character == ',' {
            let mut lookahead = chars.clone();
            while matches!(lookahead.peek(), Some(next) if next.is_whitespace()) {
                lookahead.next();
            }
            if matches!(lookahead.peek(), Some('}' | ']')) {
                continue;
            }
        }
        result.push(character);
    }
    result
}

pub(crate) fn clean_html_text(value: &str) -> String {
    let without_svg = regex::Regex::new("(?is)<svg\\b.*?</svg>")
        .map(|regex| regex.replace_all(value, " ").into_owned())
        .unwrap_or_else(|_| value.to_string());
    let without_tags = regex::Regex::new("(?s)<[^>]+>")
        .map(|regex| regex.replace_all(&without_svg, " ").into_owned())
        .unwrap_or(without_svg);
    collapse_ws(&html_unescape_minimal(&without_tags))
}

pub(crate) fn collapse_ws(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn html_unescape_minimal(value: &str) -> String {
    value
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
}

pub(crate) fn statlocker_hero_name(value: &str) -> String {
    value
        .trim()
        .replace(" & ", "_and_")
        .replace('&', "and")
        .replace(' ', "_")
}

pub(crate) fn statlocker_key(value: &str) -> String {
    statlocker_hero_name(value)
        .to_lowercase()
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .collect()
}

pub(crate) fn prompt_text_from_request(request: &deadlock_brain_core::minimax::ChatCompletionRequest) -> String {
    request
        .messages
        .iter()
        .map(|message| message.content.as_str())
        .collect::<Vec<_>>()
        .join("\n\n")
}
