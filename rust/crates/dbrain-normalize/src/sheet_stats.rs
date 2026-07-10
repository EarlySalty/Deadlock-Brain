use std::collections::HashSet;

use rusqlite::{params, Connection, OptionalExtension};
use serde_json::{json, Value};

use crate::util::{
    build_hero_index, get_required_id, normalize_alias, parse_float, payload_values, row_number,
    value_to_string,
};
use crate::Result;

const SEPARATOR_CHARS: &[char] = &['|', 'l', 'I'];

pub fn normalize_sheet_stats(conn: &Connection, rebuild: bool) -> Result<Value> {
    let deleted = if rebuild {
        let values = conn.execute("DELETE FROM hero_stat_values", [])? as i64;
        let profiles = conn.execute("DELETE FROM hero_stat_profiles", [])? as i64;
        json!({"hero_stat_values": values, "hero_stat_profiles": profiles})
    } else {
        json!({"hero_stat_values": 0, "hero_stat_profiles": 0})
    };

    let hero_index = build_hero_index(conn, true)?;
    let rows = load_rows(conn)?;
    let mut profiles = 0_i64;
    let mut values = 0_i64;
    let mut skipped_columns = 0_i64;
    let mut skipped_snapshots = 0_i64;
    let mut unmatched_heroes = HashSet::new();
    let mut seen_stat_keys = HashSet::new();

    for row in &rows {
        let Ok(payload) = serde_json::from_str::<Value>(&row.payload_json) else {
            skipped_snapshots += 1;
            continue;
        };
        let Some(raw_values) = payload_values(&payload) else {
            skipped_snapshots += 1;
            continue;
        };
        let hero_name = value_to_string(raw_values.get("Hero Name"))
            .trim()
            .to_string();
        let hero_name = if hero_name.is_empty() {
            row.canonical_name
                .clone()
                .unwrap_or_default()
                .trim()
                .to_string()
        } else {
            hero_name
        };
        if hero_name.is_empty() {
            skipped_snapshots += 1;
            continue;
        }

        let entity_id = hero_index.get(&normalize_alias(&hero_name)).copied();
        if entity_id.is_none() {
            unmatched_heroes.insert(hero_name.clone());
        }

        let profile_id = upsert_profile(
            conn,
            ProfileUpsert {
                snapshot_id: row.id,
                entity_id,
                hero_name: &hero_name,
                source: &row.source,
                external_id: &row.external_id,
                payload_hash: &row.payload_hash,
                row_number: row_number(payload.get("row_number")),
            },
        )?;
        profiles += 1;

        let mut stat_keys_for_profile = HashSet::new();
        for (label, raw_value) in raw_values {
            if !is_stat_column(label, raw_value) {
                skipped_columns += 1;
                continue;
            }
            let base_key = stat_key(label);
            if base_key.is_empty() {
                skipped_columns += 1;
                continue;
            }
            let stat_key = dedupe_stat_key(&base_key, &stat_keys_for_profile);
            stat_keys_for_profile.insert(stat_key.clone());
            seen_stat_keys.insert(stat_key.clone());

            let raw_text = value_to_string(Some(raw_value)).trim().to_string();
            let numeric_value = parse_number(&raw_text);
            if upsert_value(
                conn,
                ValueUpsert {
                    profile_id,
                    entity_id,
                    hero_name: &hero_name,
                    stat_key: &stat_key,
                    stat_label: label.trim(),
                    numeric_value,
                    raw_value: &raw_text,
                },
            )? {
                values += 1;
            }
        }
    }

    let mut unmatched = unmatched_heroes.into_iter().collect::<Vec<_>>();
    unmatched.sort();

    Ok(json!({
        "deleted": deleted,
        "snapshots": rows.len(),
        "profiles": profiles,
        "values": values,
        "stat_keys": seen_stat_keys.len(),
        "skipped_snapshots": skipped_snapshots,
        "skipped_columns": skipped_columns,
        "unmatched_heroes": unmatched,
    }))
}

#[derive(Debug)]
struct SheetStatRow {
    id: i64,
    source: String,
    external_id: String,
    canonical_name: Option<String>,
    payload_hash: String,
    payload_json: String,
}

fn load_rows(conn: &Connection) -> Result<Vec<SheetStatRow>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, source, external_id, canonical_name, payload_hash, payload_json
        FROM entity_snapshots
        WHERE entity_type='hero_stats_sheet'
        ORDER BY id
        "#,
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(SheetStatRow {
            id: row.get("id")?,
            source: row.get("source")?,
            external_id: row.get("external_id")?,
            canonical_name: row.get("canonical_name")?,
            payload_hash: row.get("payload_hash")?,
            payload_json: row.get("payload_json")?,
        })
    })?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

struct ProfileUpsert<'a> {
    snapshot_id: i64,
    entity_id: Option<i64>,
    hero_name: &'a str,
    source: &'a str,
    external_id: &'a str,
    payload_hash: &'a str,
    row_number: Option<i64>,
}

fn upsert_profile(conn: &Connection, input: ProfileUpsert<'_>) -> Result<i64> {
    let now = crate::util::now()?;
    conn.execute(
        r#"
        INSERT INTO hero_stat_profiles(
          snapshot_id, entity_id, hero_name, source, external_id, payload_hash,
          row_number, created_at, updated_at
        )
        VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9)
        ON CONFLICT(snapshot_id) DO UPDATE SET
          entity_id=excluded.entity_id,
          hero_name=excluded.hero_name,
          source=excluded.source,
          external_id=excluded.external_id,
          payload_hash=excluded.payload_hash,
          row_number=excluded.row_number,
          updated_at=excluded.updated_at
        "#,
        params![
            input.snapshot_id,
            input.entity_id,
            input.hero_name,
            input.source,
            input.external_id,
            input.payload_hash,
            input.row_number,
            now,
            now,
        ],
    )?;
    let id = conn
        .query_row(
            "SELECT id FROM hero_stat_profiles WHERE snapshot_id=?1",
            [input.snapshot_id],
            |row| row.get(0),
        )
        .optional()?;
    get_required_id(id, "hero_stat_profiles")
}

struct ValueUpsert<'a> {
    profile_id: i64,
    entity_id: Option<i64>,
    hero_name: &'a str,
    stat_key: &'a str,
    stat_label: &'a str,
    numeric_value: Option<f64>,
    raw_value: &'a str,
}

fn upsert_value(conn: &Connection, input: ValueUpsert<'_>) -> Result<bool> {
    let now = crate::util::now()?;
    let changed = conn.execute(
        r#"
        INSERT INTO hero_stat_values(
          profile_id, entity_id, hero_name, stat_key, stat_label, numeric_value,
          raw_value, created_at, updated_at
        )
        VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9)
        ON CONFLICT(profile_id, stat_key) DO UPDATE SET
          entity_id=excluded.entity_id,
          hero_name=excluded.hero_name,
          stat_label=excluded.stat_label,
          numeric_value=excluded.numeric_value,
          raw_value=excluded.raw_value,
          updated_at=excluded.updated_at
        "#,
        params![
            input.profile_id,
            input.entity_id,
            input.hero_name,
            input.stat_key,
            input.stat_label,
            input.numeric_value,
            input.raw_value,
            now,
            now,
        ],
    )?;
    Ok(changed > 0)
}

fn is_stat_column(label: &str, raw_value: &Value) -> bool {
    let label_text = label.trim();
    let value_text = value_to_string(Some(raw_value)).trim().to_string();
    if label_text.is_empty() || value_text.is_empty() {
        return false;
    }
    if matches!(
        label_text.to_lowercase().as_str(),
        "hero name" | "hero labs"
    ) {
        return false;
    }
    if is_column_placeholder(label_text) {
        return false;
    }
    if is_separator_label(label_text) {
        return false;
    }
    true
}

fn is_column_placeholder(label: &str) -> bool {
    let lower = label.to_lowercase();
    let Some(rest) = lower.strip_prefix("column ") else {
        return false;
    };
    !rest.is_empty() && rest.chars().all(|ch| ch.is_ascii_digit())
}

fn is_separator_label(label: &str) -> bool {
    let compact = label.split_whitespace().collect::<String>();
    if compact.len() < 3 {
        return false;
    }
    compact.chars().all(|ch| SEPARATOR_CHARS.contains(&ch))
        && compact.to_lowercase().chars().collect::<HashSet<_>>().len() == 1
}

fn stat_key(label: &str) -> String {
    let mut key = String::new();
    let mut last_underscore = true;
    for ch in label.trim().to_lowercase().chars() {
        if ch.is_ascii_alphanumeric() {
            key.push(ch);
            last_underscore = false;
        } else if !last_underscore {
            key.push('_');
            last_underscore = true;
        }
    }
    key.trim_matches('_').to_string()
}

fn dedupe_stat_key(stat_key: &str, used_keys: &HashSet<String>) -> String {
    if !used_keys.contains(stat_key) {
        return stat_key.to_string();
    }
    let mut suffix = 2;
    loop {
        let candidate = format!("{stat_key}_{suffix}");
        if !used_keys.contains(&candidate) {
            return candidate;
        }
        suffix += 1;
    }
}

fn parse_number(value: &str) -> Option<f64> {
    let mut normalized = value.trim().replace(',', "");
    if let Some(stripped) = normalized.strip_suffix('%') {
        normalized = stripped.trim().to_string();
    }
    if let Some(stripped) = normalized.strip_prefix('+') {
        normalized = stripped.trim().to_string();
    }
    if normalized.is_empty() || !is_plain_number(&normalized) {
        return None;
    }
    parse_float(&normalized)
}

fn is_plain_number(value: &str) -> bool {
    let mut chars = value.chars().peekable();
    if chars.peek() == Some(&'-') {
        let _ = chars.next();
    }
    let mut saw_digit = false;
    let mut saw_dot = false;
    for ch in chars {
        if ch.is_ascii_digit() {
            saw_digit = true;
        } else if ch == '.' && !saw_dot {
            saw_dot = true;
        } else {
            return false;
        }
    }
    saw_digit
}
