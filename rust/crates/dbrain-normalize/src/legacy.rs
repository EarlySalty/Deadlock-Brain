use std::collections::{BTreeMap, HashSet};

use serde_json::{json, Value};
use sqlx::PgPool;

use crate::util::{json_string, normalize_alias, table_count, table_exists};
use crate::Result;

const SUSPICIOUS_PREFIXES: &[&str] = &[
    "added ",
    "removed ",
    "improved ",
    "the following ",
];

#[derive(Debug, Clone)]
struct PatchEntityRow {
    id: i64,
    patch_title: Option<String>,
    patch_url: Option<String>,
    posted_at: Option<String>,
    entity_type: String,
    entity_name: String,
    raw_line: String,
    normalized_line: String,
}

#[derive(Debug, Clone)]
struct LegacyEntry {
    legacy_type: String,
    canonical_name: String,
    name_norm: String,
    observed_entity_type: String,
    first_patch_event_id: Option<i64>,
    last_patch_event_id: Option<i64>,
    first_seen_at: Option<String>,
    last_seen_at: Option<String>,
    event_count: i64,
    confidence: f64,
    status: String,
    samples: Vec<Value>,
}

pub async fn enrich_legacy_entities(pool: &PgPool, rebuild: bool) -> Result<Value> {
    let before_total = table_count(pool, "legacy_entities").await?;
    let deleted = if rebuild {
        sqlx::query!("DELETE FROM brain.legacy_entities")
            .execute(pool)
            .await?
            .rows_affected() as i64
    } else {
        0
    };

    let known = known_names(pool).await?;
    let mut grouped: BTreeMap<(String, String), LegacyEntry> = BTreeMap::new();
    for row in load_patch_entity_rows(pool).await? {
        let entity_type = row.entity_type.clone();
        let name = clean_name(&row.entity_name);
        let name_norm = normalize_alias(&name);
        if name_norm.is_empty() || known.contains(&(entity_type.clone(), name_norm.clone())) {
            continue;
        }
        let legacy_type = legacy_type(&entity_type);
        let key = (legacy_type.clone(), name_norm.clone());
        let confidence = name_confidence(&name, &row);
        let entry = grouped.entry(key).or_insert_with(|| LegacyEntry {
            legacy_type,
            canonical_name: name.clone(),
            name_norm: name_norm.clone(),
            observed_entity_type: entity_type.clone(),
            first_patch_event_id: Some(row.id),
            last_patch_event_id: Some(row.id),
            first_seen_at: row.posted_at.clone(),
            last_seen_at: row.posted_at.clone(),
            event_count: 0,
            confidence,
            status: "legacy_candidate".to_string(),
            samples: Vec::new(),
        });
        entry.event_count += 1;
        entry.last_patch_event_id = Some(row.id);
        if row.posted_at.is_some() {
            entry.last_seen_at = row.posted_at.clone();
        }
        entry.confidence = entry.confidence.max(confidence);
        if entry.samples.len() < 5 {
            entry.samples.push(json!({
                "patch_event_id": row.id,
                "patch_title": row.patch_title,
                "patch_url": row.patch_url,
                "posted_at": row.posted_at,
                "line": if row.normalized_line.is_empty() { row.raw_line } else { row.normalized_line },
            }));
        }
    }

    let mut inserted = 0_i64;
    let mut by_type: BTreeMap<String, i64> = BTreeMap::new();
    for mut entry in grouped.into_values() {
        if entry.confidence < 0.5 {
            entry.status = "suspect_parser_subject".to_string();
        }
        if insert_legacy(pool, &entry).await? {
            inserted += 1;
            *by_type.entry(entry.legacy_type.clone()).or_default() += 1;
        }
    }

    Ok(json!({
        "legacy_inserted": inserted,
        "legacy_before": before_total,
        "legacy_total": table_count(pool, "legacy_entities").await?,
        "deleted_before_build": deleted,
        "by_type": by_type,
        "rebuild": rebuild,
    }))
}

async fn known_names(pool: &PgPool) -> Result<HashSet<(String, String)>> {
    let mut known = HashSet::new();
    if table_exists(pool, "entities").await? {
        let rows = sqlx::query!(
            r#"SELECT entity_type AS "entity_type!", canonical_name AS "canonical_name!"
               FROM brain.entities"#,
        )
        .fetch_all(pool)
        .await?;
        for row in rows {
            known.insert((row.entity_type, normalize_alias(&row.canonical_name)));
        }
    }
    if table_exists(pool, "entity_aliases").await? {
        let rows = sqlx::query!(
            r#"
            SELECT e.entity_type AS "entity_type!", a.alias_norm AS "alias_norm!"
            FROM brain.entity_aliases a
            JOIN brain.entities e ON e.id = a.entity_id
            "#,
        )
        .fetch_all(pool)
        .await?;
        for row in rows {
            known.insert((row.entity_type, row.alias_norm));
        }
    }
    if table_exists(pool, "entity_lineage").await? {
        let rows = sqlx::query!(
            r#"
            SELECT source_entity_type AS "source_entity_type?", source_name_norm AS "source_name_norm?",
                   target_entity_type AS "target_entity_type?", target_name_norm AS "target_name_norm?"
            FROM brain.entity_lineage
            "#,
        )
        .fetch_all(pool)
        .await?;
        for row in rows {
            add_known_lineage_name(&mut known, row.source_entity_type, row.source_name_norm);
            add_known_lineage_name(&mut known, row.target_entity_type, row.target_name_norm);
        }
    }
    Ok(known)
}

fn add_known_lineage_name(
    known: &mut HashSet<(String, String)>,
    entity_type: Option<String>,
    name_norm: Option<String>,
) {
    let Some(name_norm) = name_norm.filter(|value| !value.is_empty()) else {
        return;
    };
    if let Some(entity_type) = entity_type.filter(|value| !value.is_empty()) {
        known.insert((entity_type, name_norm));
    } else {
        for fallback_type in ["hero", "item", "item_special", "ability"] {
            known.insert((fallback_type.to_string(), name_norm.clone()));
        }
    }
}

async fn load_patch_entity_rows(pool: &PgPool) -> Result<Vec<PatchEntityRow>> {
    let rows = sqlx::query!(
        r#"
        SELECT id AS "id!", patch_title AS "patch_title?", patch_url AS "patch_url?",
               posted_at::text AS "posted_at?", entity_type AS "entity_type!",
               entity_name AS "entity_name!", raw_line AS "raw_line!",
               normalized_line AS "normalized_line!"
        FROM brain.patch_events
        WHERE entity_type <> 'general'
          AND entity_name IS NOT NULL
          AND TRIM(entity_name) <> ''
        ORDER BY id ASC
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|row| PatchEntityRow {
            id: row.id,
            patch_title: row.patch_title,
            patch_url: row.patch_url,
            posted_at: row.posted_at,
            entity_type: row.entity_type,
            entity_name: row.entity_name,
            raw_line: row.raw_line,
            normalized_line: row.normalized_line,
        })
        .collect())
}

async fn insert_legacy(pool: &PgPool, entry: &LegacyEntry) -> Result<bool> {
    let samples_json = json_string(&Value::Array(entry.samples.clone()))?;
    let result = sqlx::query!(
        r#"
        INSERT INTO brain.legacy_entities(
          legacy_type, canonical_name, name_norm, observed_entity_type,
          first_patch_event_id, last_patch_event_id, first_seen_at, last_seen_at,
          event_count, confidence, status, samples, created_at, updated_at
        )
        VALUES(
          $1, $2, $3, $4,
          $5, $6, CASE WHEN $7 ~ '^-?[0-9]+$' THEN to_timestamp($7::double precision) ELSE $7::text::timestamptz END, CASE WHEN $8 ~ '^-?[0-9]+$' THEN to_timestamp($8::double precision) ELSE $8::text::timestamptz END,
          $9, $10, $11, $12::text::jsonb, now(), now()
        )
        ON CONFLICT (legacy_type, name_norm) DO UPDATE SET
          canonical_name = EXCLUDED.canonical_name,
          observed_entity_type = EXCLUDED.observed_entity_type,
          first_patch_event_id = EXCLUDED.first_patch_event_id,
          last_patch_event_id = EXCLUDED.last_patch_event_id,
          first_seen_at = EXCLUDED.first_seen_at,
          last_seen_at = EXCLUDED.last_seen_at,
          event_count = EXCLUDED.event_count,
          confidence = EXCLUDED.confidence,
          status = EXCLUDED.status,
          samples = EXCLUDED.samples,
          updated_at = EXCLUDED.updated_at
        "#,
        entry.legacy_type,
        entry.canonical_name,
        entry.name_norm,
        entry.observed_entity_type,
        entry.first_patch_event_id,
        entry.last_patch_event_id,
        entry.first_seen_at,
        entry.last_seen_at,
        entry.event_count,
        entry.confidence,
        entry.status,
        samples_json,
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

fn legacy_type(entity_type: &str) -> String {
    match entity_type {
        "hero" => "legacy_hero",
        "item" | "item_special" => "legacy_item",
        "ability" | "ability_internal" => "legacy_ability",
        "" => "legacy_entity",
        other => return format!("legacy_{other}"),
    }
    .to_string()
}

fn name_confidence(name: &str, row: &PatchEntityRow) -> f64 {
    let lowered = name.to_lowercase();
    if name.is_empty() || name.len() > 70 {
        return 0.2;
    }
    if SUSPICIOUS_PREFIXES.iter().any(|prefix| lowered.starts_with(prefix)) || lowered.ends_with(" from") {
        return 0.25;
    }
    if name.contains('(') && !name.contains(')') {
        return 0.25;
    }
    if [" following ", " abilities have ", " balance note"]
        .iter()
        .any(|token| lowered.contains(token))
    {
        return 0.3;
    }
    if name.split_whitespace().count() > 6 {
        return 0.35;
    }
    if matches!(row.entity_type.as_str(), "hero" | "item" | "item_special" | "ability") {
        0.74
    } else {
        0.55
    }
}

fn clean_name(value: &str) -> String {
    value
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim_matches(|ch: char| ch == ' ' || ch == ':' || ch == '-' || ch == '\t')
        .to_string()
}
