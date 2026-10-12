use super::{FactSet, FactValue, Receipt, Rejection};
use std::collections::BTreeSet;

pub const KEY_BYTES: usize = 128;
pub const TEXT_BYTES: usize = 16_384;
pub const ALIAS_BYTES: usize = 256;
pub const LOCATOR_BYTES: usize = 4_096;
pub const DETAIL_BYTES: usize = 512;
pub const ROWS: usize = 4_096;
pub const NUMBER_ABS: f64 = 1_000_000_000_000_000.0;
pub const KEY_PATTERN: &str = "^[a-z][a-z0-9_]*$";

pub fn normalize_key(value: &str) -> String {
    value
        .trim_matches(|c: char| c.is_ascii_whitespace())
        .to_ascii_lowercase()
        .replace(['-', ' '], "_")
}

pub fn normalize_alias(value: &str) -> String {
    value.trim().to_lowercase()
}

fn text(value: &str, max: usize) -> Result<(), Rejection> {
    if value.len() > max {
        return Err(Rejection::new(
            "value_too_large",
            "Text überschreitet die Byte-Grenze",
        ));
    }
    if value.contains('\0') {
        return Err(Rejection::new("nul_text", "Text enthält ein NUL-Zeichen"));
    }
    Ok(())
}

fn key(value: &str) -> Result<String, Rejection> {
    let normalized = normalize_key(value);
    text(&normalized, KEY_BYTES)?;
    if !normalized
        .as_bytes()
        .first()
        .is_some_and(u8::is_ascii_lowercase)
        || !normalized
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
    {
        return Err(Rejection::new(
            "invalid_key",
            "Schlüssel muss dem ASCII-Schlüsselvertrag entsprechen",
        ));
    }
    Ok(normalized)
}

fn receipt(value: &Receipt) -> Result<(), Rejection> {
    let normalized = key(&value.locator_kind)?;
    if normalized != value.locator_kind {
        return Err(Rejection::new(
            "invalid_locator_kind",
            "Locator-Typ muss bereits normalisiert sein",
        ));
    }
    text(&value.locator, LOCATOR_BYTES)
}

fn unique(seen: &mut BTreeSet<Vec<String>>, key: Vec<String>) -> Result<(), Rejection> {
    if !seen.insert(key) {
        return Err(Rejection::new(
            "key_collision",
            "Zwei Zeilen besitzen denselben normalisierten Schlüssel",
        ));
    }
    Ok(())
}

pub fn validate(set: &FactSet) -> Result<(), Rejection> {
    if set
        .entities
        .len()
        .saturating_add(set.aliases.len())
        .saturating_add(set.facts.len())
        .saturating_add(set.relations.len())
        > ROWS
    {
        return Err(Rejection::new(
            "too_many_rows",
            "Objekt enthält zu viele kanonische Zeilen",
        ));
    }
    let mut seen = BTreeSet::new();
    for row in &set.entities {
        unique(&mut seen, vec![key(&row.key)?])?;
        key(&row.kind)?;
        receipt(&row.receipt)?;
    }
    seen.clear();
    for row in &set.aliases {
        let alias = normalize_alias(&row.alias);
        text(&alias, ALIAS_BYTES)?;
        unique(
            &mut seen,
            vec![key(&row.entity_key)?, alias, key(&row.lang)?],
        )?;
        receipt(&row.receipt)?;
    }
    seen.clear();
    for row in &set.facts {
        unique(&mut seen, vec![key(&row.entity_key)?, key(&row.stat_key)?])?;
        match &row.value {
            FactValue::Number(n) if !n.is_finite() || n.abs() > NUMBER_ABS => {
                return Err(Rejection::new(
                    "invalid_number",
                    "Zahl ist nicht endlich oder außerhalb des Wertebereichs",
                ))
            }
            FactValue::Text(t) => text(t, TEXT_BYTES)?,
            _ => {}
        }
        receipt(&row.receipt)?;
    }
    seen.clear();
    for row in &set.relations {
        unique(
            &mut seen,
            vec![key(&row.from_key)?, key(&row.kind)?, key(&row.to_key)?],
        )?;
        receipt(&row.receipt)?;
    }
    Ok(())
}

pub fn render_sql(sql: &str) -> String {
    let mut result = sql.to_owned();
    for (name, value) in [
        ("{{KEY_BYTES}}", KEY_BYTES.to_string()),
        ("{{TEXT_BYTES}}", TEXT_BYTES.to_string()),
        ("{{ALIAS_BYTES}}", ALIAS_BYTES.to_string()),
        ("{{LOCATOR_BYTES}}", LOCATOR_BYTES.to_string()),
        ("{{DETAIL_BYTES}}", DETAIL_BYTES.to_string()),
        ("{{ROWS}}", ROWS.to_string()),
        ("{{NUMBER_ABS}}", NUMBER_ABS.to_string()),
        ("{{KEY_PATTERN}}", KEY_PATTERN.to_owned()),
    ] {
        result = result.replace(name, &value);
    }
    result
}
