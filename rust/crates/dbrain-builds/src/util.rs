use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use serde::Serialize;
use serde_json::Value;

pub(crate) const BRACKET_BADGE_80: &str = "badge80";
pub(crate) const PATCH_TAG_CURRENT: &str = "current";

pub(crate) fn now_epoch_seconds() -> Result<i64> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64)
}

pub(crate) fn json_string<T: Serialize + ?Sized>(value: &T) -> Result<String> {
    Ok(serde_json::to_string(value)?)
}

pub(crate) fn normalize_name(value: &str) -> String {
    value
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(|character| character.to_lowercase())
        .collect()
}

pub(crate) fn value_i64(value: &Value, key: &str) -> Option<i64> {
    value.get(key).and_then(value_as_i64)
}

pub(crate) fn value_f64(value: &Value, key: &str) -> Option<f64> {
    value.get(key).and_then(value_as_f64)
}

pub(crate) fn value_string(value: &Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .map(ToString::to_string)
        .filter(|text| !text.trim().is_empty())
}

pub(crate) fn value_as_i64(value: &Value) -> Option<i64> {
    match value {
        Value::Number(number) => number
            .as_i64()
            .or_else(|| number.as_u64().and_then(|raw| i64::try_from(raw).ok())),
        Value::String(text) => text.trim().parse::<i64>().ok(),
        _ => None,
    }
}

pub(crate) fn value_as_f64(value: &Value) -> Option<f64> {
    match value {
        Value::Number(number) => number.as_f64(),
        Value::String(text) => text.trim().parse::<f64>().ok(),
        _ => None,
    }
}

pub(crate) fn winrate(wins: i64, losses: i64, matches: i64) -> Option<f64> {
    let total = if matches > 0 { matches } else { wins + losses };
    (total > 0).then_some(wins as f64 / total as f64)
}

pub(crate) fn winrate_pp(wins: i64, losses: i64, matches: i64) -> Option<f64> {
    winrate(wins, losses, matches).map(|value| value * 100.0)
}

pub(crate) fn clamp_unit(value: f64) -> f64 {
    value.clamp(0.0, 1.0)
}

#[cfg(test)]
pub(crate) fn load_fixture(name: &str) -> Value {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name);
    let content = std::fs::read_to_string(path).expect("fixture readable");
    serde_json::from_str(&content).expect("fixture json")
}
