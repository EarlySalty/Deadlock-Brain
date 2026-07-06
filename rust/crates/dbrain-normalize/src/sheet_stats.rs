use std::collections::HashSet;

use serde_json::{json, Value};
use sqlx::PgPool;

use crate::util::{
    build_hero_index, normalize_alias, parse_float, payload_values, row_number, value_to_string,
};
use crate::Result;

const SEPARATOR_CHARS: &[char] = &['|', 'l', 'I'];

pub async fn normalize_sheet_stats(pool: &PgPool, rebuild: bool) -> Result<Value> {
    let deleted = if rebuild {
        let values = sqlx::query!("DELETE FROM brain.hero_stat_values")
            .execute(pool)
            .await?
            .rows_affected() as i64;
        let profiles = sqlx::query!("DELETE FROM brain.hero_stat_profiles")
            .execute(pool)
            .await?
            .rows_affected() as i64;
        json!({"hero_stat_values": values, "hero_stat_profiles": profiles})
    } else {
        json!({"hero_stat_values": 0, "hero_stat_profiles": 0})
    };

    let hero_index = build_hero_index(pool, true).await?;
    let rows = load_rows(pool).await?;
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
            row.canonical_name.clone().unwrap_or_default().trim().to_string()
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

        let profile_id = upsert_profile(pool, ProfileUpsert {
            snapshot_id: row.id,
            legacy_snapshot_id: row.legacy_snapshot_id,
            entity_id,
            hero_name: &hero_name,
            source: &row.source,
            external_id: &row.external_id,
            payload_hash: &row.payload_hash,
            row_number: row_number(payload.get("row_number")),
        }).await?;
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
            if upsert_value(pool, ValueUpsert {
                profile_id,
                entity_id,
                hero_name: &hero_name,
                stat_key: &stat_key,
                stat_label: label.trim(),
                numeric_value,
                raw_value: &raw_text,
            }).await? {
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
    legacy_snapshot_id: i64,
    source: String,
    external_id: String,
    canonical_name: Option<String>,
    payload_hash: String,
    payload_json: String,
}

async fn load_rows(pool: &PgPool) -> Result<Vec<SheetStatRow>> {
    let rows = sqlx::query!(
        r#"
        SELECT id AS "id!", COALESCE(legacy_sqlite_id, id) AS "legacy_snapshot_id!",
               source AS "source!", external_id AS "external_id!",
               canonical_name AS "canonical_name?", payload_hash AS "payload_hash!",
               payload::text AS "payload_json!"
        FROM brain.entity_snapshots
        WHERE entity_type = 'hero_stats_sheet'
        ORDER BY id
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|row| SheetStatRow {
            id: row.id,
            legacy_snapshot_id: row.legacy_snapshot_id,
            source: row.source,
            external_id: row.external_id,
            canonical_name: row.canonical_name,
            payload_hash: row.payload_hash,
            payload_json: row.payload_json,
        })
        .collect())
}

struct ProfileUpsert<'a> {
    snapshot_id: i64,
    legacy_snapshot_id: i64,
    entity_id: Option<i64>,
    hero_name: &'a str,
    source: &'a str,
    external_id: &'a str,
    payload_hash: &'a str,
    row_number: Option<i64>,
}

async fn upsert_profile(pool: &PgPool, input: ProfileUpsert<'_>) -> Result<i64> {
    let id = sqlx::query_scalar!(
        r#"
        INSERT INTO brain.hero_stat_profiles(
          snapshot_id, legacy_snapshot_id, entity_id, hero_name, source, external_id, payload_hash,
          row_number, created_at, updated_at
        )
        VALUES($1, $2, $3, $4, $5, $6, $7, $8, now(), now())
        ON CONFLICT (snapshot_id) DO UPDATE SET
          entity_id = EXCLUDED.entity_id,
          hero_name = EXCLUDED.hero_name,
          source = EXCLUDED.source,
          external_id = EXCLUDED.external_id,
          payload_hash = EXCLUDED.payload_hash,
          row_number = EXCLUDED.row_number,
          updated_at = EXCLUDED.updated_at
        RETURNING id
        "#,
        input.snapshot_id,
        input.legacy_snapshot_id,
        input.entity_id,
        input.hero_name,
        input.source,
        input.external_id,
        input.payload_hash,
        input.row_number,
    )
    .fetch_one(pool)
    .await?;
    Ok(id)
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

async fn upsert_value(pool: &PgPool, input: ValueUpsert<'_>) -> Result<bool> {
    let result = sqlx::query!(
        r#"
        INSERT INTO brain.hero_stat_values(
          profile_id, legacy_profile_id, entity_id, hero_name, stat_key, stat_label, numeric_value,
          raw_value, created_at, updated_at
        )
        VALUES($1, $2, $3, $4, $5, $6, $7, $8, now(), now())
        ON CONFLICT (profile_id, stat_key) DO UPDATE SET
          entity_id = EXCLUDED.entity_id,
          hero_name = EXCLUDED.hero_name,
          stat_label = EXCLUDED.stat_label,
          numeric_value = EXCLUDED.numeric_value,
          raw_value = EXCLUDED.raw_value,
          updated_at = EXCLUDED.updated_at
        "#,
        input.profile_id,
        input.profile_id,
        input.entity_id,
        input.hero_name,
        input.stat_key,
        input.stat_label,
        input.numeric_value,
        input.raw_value,
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

fn is_stat_column(label: &str, raw_value: &Value) -> bool {
    let label_text = label.trim();
    let value_text = value_to_string(Some(raw_value)).trim().to_string();
    if label_text.is_empty() || value_text.is_empty() {
        return false;
    }
    if matches!(label_text.to_lowercase().as_str(), "hero name" | "hero labs") {
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
