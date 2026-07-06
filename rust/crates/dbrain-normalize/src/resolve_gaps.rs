use std::collections::{HashMap, HashSet};

use serde_json::{json, Map, Value};
use sqlx::{PgPool, Row};

use crate::{
    patch::{build_entity_index_from_entities, EntityIndex},
    util::{clean_subject, normalize_alias},
    NormalizeError, Result,
};

const SAMPLE_LIMIT: usize = 10;
const TRUE_INDEX_CONFIDENCE: f64 = 0.9;
const BACKFILL_TABLES: &[&str] = &[
    "hero_stat_profiles",
    "hero_stat_values",
    "sheet_heroes_stats",
    "sheet_raw_heroes",
    "sheet_hero_rankings",
];

#[derive(Debug)]
struct ResolvedEntity {
    entity_type: String,
    entity_name: String,
    confidence: f64,
}

pub async fn resolve_gaps(pool: &PgPool, dry_run: bool) -> Result<Value> {
    let entity_index = build_entity_index_from_entities(pool).await?;
    let patch_events = resolve_patch_events(pool, entity_index.as_ref(), dry_run).await?;
    let claims = resolve_claims(pool, entity_index.as_ref(), dry_run).await?;
    let entity_id_backfill = backfill_entity_ids(pool, dry_run).await?;

    Ok(json!({
        "dry_run": dry_run,
        "patch_events": patch_events,
        "claims": claims,
        "entity_id_backfill": entity_id_backfill,
    }))
}

async fn resolve_patch_events(
    pool: &PgPool,
    entity_index: Option<&EntityIndex>,
    dry_run: bool,
) -> Result<Value> {
    let rows = sqlx::query!(
        r#"
        SELECT id AS "id!", subject AS "subject!", section AS "section?",
               entity_type AS "entity_type!", entity_name AS "entity_name?",
               confidence AS "confidence!"
        FROM brain.patch_events
        WHERE entity_type = 'general'
          AND subject IS NOT NULL
          AND TRIM(subject) <> ''
        ORDER BY id
        "#,
    )
    .fetch_all(pool)
    .await?;

    let mut changed = 0_i64;
    let mut samples = Vec::new();
    for row in rows {
        let Some(resolved) = true_index_match(entity_index, &row.subject, row.section.as_deref())
        else {
            continue;
        };
        changed += 1;
        push_sample(
            &mut samples,
            json!({
                "id": row.id,
                "subject": row.subject,
                "section": row.section,
                "before": {
                    "entity_type": row.entity_type,
                    "entity_name": row.entity_name,
                    "confidence": row.confidence,
                },
                "after": {
                    "entity_type": resolved.entity_type,
                    "entity_name": resolved.entity_name,
                    "confidence": resolved.confidence,
                },
            }),
        );
        if !dry_run {
            sqlx::query!(
                r#"
                UPDATE brain.patch_events
                SET entity_type = $1, entity_name = $2, confidence = $3
                WHERE id = $4
                "#,
                resolved.entity_type,
                resolved.entity_name,
                resolved.confidence,
                row.id,
            )
            .execute(pool)
            .await?;
        }
    }

    Ok(json!({
        "changed": changed,
        "samples": samples,
    }))
}

async fn resolve_claims(
    pool: &PgPool,
    entity_index: Option<&EntityIndex>,
    dry_run: bool,
) -> Result<Value> {
    let rows = sqlx::query!(
        r#"
        SELECT id AS "id!", entity_type AS "entity_type?", entity_name AS "entity_name?",
               verifier::text AS "verifier_json!"
        FROM brain.youtube_learning_claims
        WHERE entity_type IS NULL
           OR verifier::text LIKE '%entity_not_resolved%'
        ORDER BY id
        "#,
    )
    .fetch_all(pool)
    .await?;

    let mut changed = 0_i64;
    let mut samples = Vec::new();
    for row in rows {
        let Some(entity_name) = row.entity_name.as_deref() else {
            continue;
        };
        let candidates = claim_candidates(entity_name);
        let mut selected = None;
        for candidate in &candidates {
            if let Some(resolved) = true_index_match(entity_index, candidate, None) {
                selected = Some((candidate.clone(), resolved));
                break;
            }
        }

        let Some((candidate, resolved)) = selected else {
            continue;
        };
        let new_verifier = updated_claim_verifier(&row.verifier_json)?;
        let new_verifier_json = serde_json::to_string(&new_verifier)?;
        let has_change = row.entity_type.as_deref() != Some(resolved.entity_type.as_str())
            || row.entity_name.as_deref() != Some(resolved.entity_name.as_str())
            || row.verifier_json != new_verifier_json;
        if !has_change {
            continue;
        }

        changed += 1;
        push_sample(
            &mut samples,
            json!({
                "id": row.id,
                "candidate": candidate,
                "candidates": candidates,
                "before": {
                    "entity_type": row.entity_type,
                    "entity_name": row.entity_name,
                },
                "after": {
                    "entity_type": resolved.entity_type,
                    "entity_name": resolved.entity_name,
                    "confidence": resolved.confidence,
                },
            }),
        );
        if !dry_run {
            sqlx::query!(
                r#"
                UPDATE brain.youtube_learning_claims
                SET entity_type = $1, entity_name = $2, verifier = $3::text::jsonb
                WHERE id = $4
                "#,
                resolved.entity_type,
                resolved.entity_name,
                new_verifier_json,
                row.id,
            )
            .execute(pool)
            .await?;
        }
    }

    Ok(json!({
        "changed": changed,
        "samples": samples,
    }))
}

async fn backfill_entity_ids(pool: &PgPool, dry_run: bool) -> Result<Value> {
    let hero_index = build_unique_hero_index(pool).await?;
    let mut total_changed = 0_i64;
    let mut total_samples = Vec::new();
    let mut table_reports = Map::new();

    for table in BACKFILL_TABLES {
        let (changed, samples) = backfill_table(pool, table, &hero_index, dry_run).await?;
        total_changed += changed;
        for sample in &samples {
            push_sample(&mut total_samples, sample.clone());
        }
        table_reports.insert(
            (*table).to_string(),
            json!({
                "changed": changed,
                "samples": samples,
            }),
        );
    }

    Ok(json!({
        "changed": total_changed,
        "tables": table_reports,
        "samples": total_samples,
    }))
}

async fn backfill_table(
    pool: &PgPool,
    table: &str,
    hero_index: &HashMap<String, i64>,
    dry_run: bool,
) -> Result<(i64, Vec<Value>)> {
    // Nur die fest whitelisteten Backfill-Tabellen duerfen in die dynamische
    // SQL interpoliert werden (kein rohes `format!` von Fremdinput).
    if !BACKFILL_TABLES.contains(&table) {
        return Err(NormalizeError::InvalidTable(table.to_string()));
    }
    let select_sql =
        format!("SELECT id, hero_name FROM brain.{table} WHERE entity_id IS NULL ORDER BY id");
    let rows = sqlx::query(&select_sql).fetch_all(pool).await?;

    let update_sql =
        format!("UPDATE brain.{table} SET entity_id = $1 WHERE id = $2 AND entity_id IS NULL");
    let mut changed = 0_i64;
    let mut samples = Vec::new();
    for row in rows {
        let row_id: i64 = row.try_get("id")?;
        let hero_name: String = row.try_get("hero_name")?;
        let key = normalize_alias(&hero_name);
        let Some(entity_id) = hero_index.get(&key).copied() else {
            continue;
        };
        changed += 1;
        push_sample(
            &mut samples,
            json!({
                "table": table,
                "id": row_id,
                "hero_name": hero_name,
                "before": {"entity_id": Value::Null},
                "after": {"entity_id": entity_id},
            }),
        );
        if !dry_run {
            sqlx::query(&update_sql)
                .bind(entity_id)
                .bind(row_id)
                .execute(pool)
                .await?;
        }
    }

    Ok((changed, samples))
}

fn true_index_match(
    entity_index: Option<&EntityIndex>,
    name: &str,
    section: Option<&str>,
) -> Option<ResolvedEntity> {
    let index = entity_index?;
    let (entity_type, entity_name, confidence) = index.canonical(name, section);
    if entity_type == "general" || confidence < TRUE_INDEX_CONFIDENCE {
        return None;
    }
    entity_name.map(|name| ResolvedEntity {
        entity_type,
        entity_name: name,
        confidence,
    })
}

fn claim_candidates(entity_name: &str) -> Vec<String> {
    let cleaned = clean_subject(Some(entity_name));
    let mut candidates = Vec::new();
    add_candidate(&mut candidates, &cleaned);
    for part in cleaned.split('/') {
        add_candidate(&mut candidates, &clean_subject(Some(part)));
    }
    if let Some(start) = cleaned.find('(') {
        add_candidate(&mut candidates, &clean_subject(Some(&cleaned[..start])));
    }
    for content in parenthetical_contents(&cleaned) {
        add_candidate(&mut candidates, &content);
    }
    if let Some(stripped) = strip_trailing_annotation(&cleaned) {
        add_candidate(&mut candidates, stripped);
    }
    candidates
}

fn add_candidate(candidates: &mut Vec<String>, candidate: &str) {
    let candidate = candidate.trim();
    if candidate.is_empty() || candidates.iter().any(|existing| existing == candidate) {
        return;
    }
    candidates.push(candidate.to_string());
}

fn parenthetical_contents(value: &str) -> Vec<String> {
    let mut contents = Vec::new();
    let mut remaining = value;
    while let Some(start) = remaining.find('(') {
        let after_start = &remaining[start + 1..];
        let Some(end) = after_start.find(')') else {
            break;
        };
        add_candidate(&mut contents, &clean_subject(Some(&after_start[..end])));
        remaining = &after_start[end + 1..];
    }
    contents
}

fn strip_trailing_annotation(value: &str) -> Option<&str> {
    let ascii = value.find(" - ");
    let en_dash = value.find(" \u{2013} ");
    let cut = match (ascii, en_dash) {
        (Some(left), Some(right)) => Some(left.min(right)),
        (Some(index), None) | (None, Some(index)) => Some(index),
        (None, None) => None,
    }?;
    let stripped = value[..cut].trim();
    if stripped.is_empty() {
        None
    } else {
        Some(stripped)
    }
}

fn updated_claim_verifier(raw: &str) -> Result<Value> {
    let mut value = serde_json::from_str::<Value>(raw)?;
    if !value.is_object() {
        let mut object = Map::new();
        object.insert("value".to_string(), value);
        value = Value::Object(object);
    }
    if let Some(object) = value.as_object_mut() {
        if let Some(Value::Array(reasons)) = object.get_mut("reasons") {
            reasons.retain(|reason| reason.as_str() != Some("entity_not_resolved"));
        }
        object.insert(
            "entity_resolution".to_string(),
            Value::String("reresolved_deadlock_data".to_string()),
        );
    }
    Ok(value)
}

async fn build_unique_hero_index(pool: &PgPool) -> Result<HashMap<String, i64>> {
    let mut candidates: HashMap<String, HashSet<i64>> = HashMap::new();
    let entity_rows = sqlx::query!(
        r#"SELECT id AS "id!", canonical_name AS "canonical_name!"
           FROM brain.entities WHERE entity_type = 'hero'"#,
    )
    .fetch_all(pool)
    .await?;
    for row in entity_rows {
        add_hero_candidate(&mut candidates, &row.canonical_name, row.id);
    }

    let alias_rows = sqlx::query!(
        r#"
        SELECT a.entity_id AS "entity_id!", a.alias AS "alias!"
        FROM brain.entity_aliases a
        JOIN brain.entities e ON e.id = a.entity_id
        WHERE e.entity_type = 'hero'
        "#,
    )
    .fetch_all(pool)
    .await?;
    for row in alias_rows {
        add_hero_candidate(&mut candidates, &row.alias, row.entity_id);
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

fn add_hero_candidate(
    candidates: &mut HashMap<String, HashSet<i64>>,
    name: &str,
    entity_id: i64,
) {
    let key = normalize_alias(name);
    if !key.is_empty() {
        candidates.entry(key).or_default().insert(entity_id);
    }
}

fn push_sample(samples: &mut Vec<Value>, sample: Value) {
    if samples.len() < SAMPLE_LIMIT {
        samples.push(sample);
    }
}
