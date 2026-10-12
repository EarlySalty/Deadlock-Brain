use super::{FactSet, FactValue, Receipt, Rejection};
use std::collections::BTreeSet;

pub const RAW_BYTES: usize = 4 * 1024 * 1024;
pub const KEY_BYTES: usize = 128;
pub const TEXT_BYTES: usize = 16_384;
pub const ALIAS_BYTES: usize = 256;
pub const LOCATOR_BYTES: usize = 4_096;
pub const DETAIL_BYTES: usize = 512;
pub const ROWS: usize = 4_096;
pub const NUMBER_ABS: f64 = 1_000_000_000_000_000.0;
pub const KEY_PATTERN: &str = "^[a-z][a-z0-9_]*$";

fn invalid_number() -> Rejection {
    Rejection::new(
        "invalid_number",
        "Zahl ist ungültig oder außerhalb des Wertebereichs",
    )
}

pub fn validate_numeric_lexeme(value: &str) -> Result<(), Rejection> {
    if value.len() > TEXT_BYTES {
        return Err(invalid_number());
    }
    let bytes = value.as_bytes();
    let start = usize::from(bytes.first() == Some(&b'-'));
    let mut index = start;
    if bytes.get(index) == Some(&b'0') {
        index += 1;
    } else {
        if !bytes.get(index).is_some_and(|b| matches!(b, b'1'..=b'9')) {
            return Err(invalid_number());
        }
        while bytes.get(index).is_some_and(u8::is_ascii_digit) {
            index += 1;
        }
    }
    let integer_digits = index - start;
    if bytes.get(index) == Some(&b'.') {
        index += 1;
        let fraction = index;
        while bytes.get(index).is_some_and(u8::is_ascii_digit) {
            index += 1;
        }
        if index == fraction {
            return Err(invalid_number());
        }
    }
    let mantissa_end = index;
    let mut exponent = 0_i64;
    if bytes.get(index).is_some_and(|b| matches!(b, b'e' | b'E')) {
        index += 1;
        let negative = bytes.get(index) == Some(&b'-');
        if bytes.get(index).is_some_and(|b| matches!(b, b'+' | b'-')) {
            index += 1;
        }
        let exponent_start = index;
        while let Some(b) = bytes.get(index).filter(|b| b.is_ascii_digit()) {
            exponent = (exponent * 10 + i64::from(*b - b'0')).min(TEXT_BYTES as i64 + 16);
            index += 1;
        }
        if index == exponent_start {
            return Err(invalid_number());
        }
        if negative {
            exponent = -exponent;
        }
    }
    if index != bytes.len() {
        return Err(invalid_number());
    }
    let mut digits = bytes[start..mantissa_end]
        .iter()
        .copied()
        .filter(u8::is_ascii_digit)
        .enumerate();
    let Some((leading, digit)) = digits.find(|(_, b)| *b != b'0') else {
        return Ok(());
    };
    let order = integer_digits as i64 - leading as i64 - 1 + exponent;
    if order > 15 || (order == 15 && (digit != b'1' || digits.any(|(_, b)| b != b'0'))) {
        return Err(invalid_number());
    }
    Ok(())
}

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
