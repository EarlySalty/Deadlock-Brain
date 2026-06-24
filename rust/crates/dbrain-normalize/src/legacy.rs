use std::collections::{BTreeMap, HashSet};

use rusqlite::{params, Connection};
use serde_json::{json, Value};

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

pub fn enrich_legacy_entities(conn: &Connection, rebuild: bool) -> Result<Value> {
    let before_total = table_count(conn, "legacy_entities")?;
    let deleted = if rebuild {
        conn.execute("DELETE FROM legacy_entities", [])? as i64
    } else {
        0
    };

    let known = known_names(conn)?;
    let mut grouped: BTreeMap<(String, String), LegacyEntry> = BTreeMap::new();
    for row in load_patch_entity_rows(conn)? {
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
        if insert_legacy(conn, &entry)? {
            inserted += 1;
            *by_type.entry(entry.legacy_type.clone()).or_default() += 1;
        }
    }

    Ok(json!({
        "legacy_inserted": inserted,
        "legacy_before": before_total,
        "legacy_total": table_count(conn, "legacy_entities")?,
        "deleted_before_build": deleted,
        "by_type": by_type,
        "rebuild": rebuild,
    }))
}

fn known_names(conn: &Connection) -> Result<HashSet<(String, String)>> {
    let mut known = HashSet::new();
    if table_exists(conn, "entities")? {
        let mut stmt = conn.prepare("SELECT entity_type, canonical_name FROM entities")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;
        for row in rows {
            let (entity_type, canonical_name) = row?;
            known.insert((entity_type, normalize_alias(&canonical_name)));
        }
    }
    if table_exists(conn, "entity_aliases")? {
        let mut stmt = conn.prepare(
            r#"
            SELECT e.entity_type, a.alias_norm
            FROM entity_aliases a
            JOIN entities e ON e.id=a.entity_id
            "#,
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;
        for row in rows {
            known.insert(row?);
        }
    }
    if table_exists(conn, "entity_lineage")? {
        let mut stmt = conn.prepare(
            "SELECT source_entity_type, source_name_norm, target_entity_type, target_name_norm FROM entity_lineage",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, Option<String>>(0)?,
                row.get::<_, Option<String>>(1)?,
                row.get::<_, Option<String>>(2)?,
                row.get::<_, Option<String>>(3)?,
            ))
        })?;
        for row in rows {
            let (source_type, source_norm, target_type, target_norm) = row?;
            add_known_lineage_name(&mut known, source_type, source_norm);
            add_known_lineage_name(&mut known, target_type, target_norm);
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

fn load_patch_entity_rows(conn: &Connection) -> Result<Vec<PatchEntityRow>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, patch_title, patch_url, posted_at, entity_type, entity_name, raw_line, normalized_line
        FROM patch_events
        WHERE entity_type <> 'general'
          AND entity_name IS NOT NULL
          AND TRIM(entity_name) <> ''
        ORDER BY id ASC
        "#,
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(PatchEntityRow {
            id: row.get("id")?,
            patch_title: row.get("patch_title")?,
            patch_url: row.get("patch_url")?,
            posted_at: row.get("posted_at")?,
            entity_type: row.get("entity_type")?,
            entity_name: row.get("entity_name")?,
            raw_line: row.get("raw_line")?,
            normalized_line: row.get("normalized_line")?,
        })
    })?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

fn insert_legacy(conn: &Connection, entry: &LegacyEntry) -> Result<bool> {
    let now = crate::util::now()?;
    let samples_json = json_string(&Value::Array(entry.samples.clone()))?;
    let changed = conn.execute(
        r#"
        INSERT INTO legacy_entities(
          legacy_type, canonical_name, name_norm, observed_entity_type,
          first_patch_event_id, last_patch_event_id, first_seen_at, last_seen_at,
          event_count, confidence, status, samples_json, created_at, updated_at
        )
        VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14)
        ON CONFLICT(legacy_type, name_norm) DO UPDATE SET
          canonical_name=excluded.canonical_name,
          observed_entity_type=excluded.observed_entity_type,
          first_patch_event_id=excluded.first_patch_event_id,
          last_patch_event_id=excluded.last_patch_event_id,
          first_seen_at=excluded.first_seen_at,
          last_seen_at=excluded.last_seen_at,
          event_count=excluded.event_count,
          confidence=excluded.confidence,
          status=excluded.status,
          samples_json=excluded.samples_json,
          updated_at=excluded.updated_at
        "#,
        params![
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
            now,
            now,
        ],
    )?;
    Ok(changed > 0)
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
