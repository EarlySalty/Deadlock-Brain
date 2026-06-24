use std::collections::{BTreeMap, BTreeSet};

use rusqlite::{
    types::{ToSql, ValueRef},
    Connection, Row,
};
use serde_json::{json, Map, Number, Value};
use sha2::{Digest, Sha256};

use crate::Result;

pub(crate) fn ensure_schema(conn: &Connection) -> Result<()> {
    deadlock_brain_core::schema::ensure_schema(conn)?;
    Ok(())
}

pub(crate) fn stable_hash_text(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub(crate) fn now_epoch_seconds() -> Result<i64> {
    Ok(deadlock_brain_core::db::now_epoch_seconds()?)
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

pub(crate) fn row_to_json(row: &Row<'_>) -> rusqlite::Result<Value> {
    let mut object = Map::new();
    let row_ref = row.as_ref();
    for index in 0..row_ref.column_count() {
        let name = row_ref.column_name(index)?.to_string();
        object.insert(name, sql_value_to_json(row.get_ref(index)?));
    }
    Ok(Value::Object(object))
}

pub(crate) fn query_json_rows(
    conn: &Connection,
    sql: &str,
    params: &[&dyn ToSql],
) -> rusqlite::Result<Vec<Value>> {
    let mut stmt = conn.prepare(sql)?;
    let rows = stmt.query_map(params, row_to_json)?;
    rows.collect()
}

pub(crate) fn query_one_json(
    conn: &Connection,
    sql: &str,
    params: &[&dyn ToSql],
) -> rusqlite::Result<Option<Value>> {
    let mut rows = query_json_rows(conn, sql, params)?;
    Ok(if rows.is_empty() {
        None
    } else {
        Some(rows.remove(0))
    })
}

pub(crate) fn table_exists(conn: &Connection, name: &str) -> rusqlite::Result<bool> {
    let row = conn.query_row(
        "SELECT 1 FROM sqlite_master WHERE type='table' AND name=?1",
        [name],
        |_row| Ok(()),
    );
    match row {
        Ok(()) => Ok(true),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(false),
        Err(error) => Err(error),
    }
}

pub(crate) fn sql_value_to_json(value: ValueRef<'_>) -> Value {
    match value {
        ValueRef::Null => Value::Null,
        ValueRef::Integer(item) => json!(item),
        ValueRef::Real(item) => Number::from_f64(item).map(Value::Number).unwrap_or(Value::Null),
        ValueRef::Text(bytes) => String::from_utf8_lossy(bytes).to_string().into(),
        ValueRef::Blob(bytes) => bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>().into(),
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
    let marker = "\"insights\"";
    if !text.contains(marker) {
        return json!({});
    }
    let marker_pos = text.find(marker).unwrap_or(0);
    let search_start = marker_pos.saturating_sub(50);
    let Some(relative_start) = text[search_start..].find('{') else {
        return json!({});
    };
    let start = search_start + relative_start;
    let Some(end) = text.rfind('}') else {
        return json!({});
    };
    if end <= start {
        return json!({});
    }
    let parsed = serde_json::from_str::<Value>(&text[start..=end]).unwrap_or_else(|_| json!({}));
    if let Some(insights) = parsed.get("insights").filter(|value| value.is_object()) {
        insights.clone()
    } else if parsed.is_object() {
        parsed
    } else {
        json!({})
    }
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
