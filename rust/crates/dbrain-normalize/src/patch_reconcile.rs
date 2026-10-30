use std::collections::{HashMap, HashSet};

use sqlx::{PgPool, Postgres, Row, Transaction};

use crate::patch::PatchEventInsert;
use crate::Result;

#[derive(Debug, Clone)]
pub(crate) struct StoredEvent {
    pub(crate) id: i64,
    pub(crate) event_hash: String,
    pub(crate) line_index: i64,
    pub(crate) normalized_line: String,
    pub(crate) entity_type: String,
    pub(crate) entity_name: Option<String>,
    pub(crate) change_type: String,
    pub(crate) patch_title: Option<String>,
}

#[derive(Debug, Default, PartialEq)]
pub(crate) struct ReconcilePlan {
    pub(crate) updates: Vec<(i64, usize)>,
    pub(crate) unchanged: usize,
    pub(crate) inserts: Vec<usize>,
    pub(crate) deletes: Vec<i64>,
}

pub(crate) fn plan_reconcile(stored: &[StoredEvent], events: &[PatchEventInsert]) -> ReconcilePlan {
    let mut plan = ReconcilePlan::default();
    let mut used = HashSet::new();
    let mut matched: Vec<Option<usize>> = vec![None; events.len()];
    let matchers: [fn(&StoredEvent, &PatchEventInsert) -> bool; 3] = [
        |stored, event| stored.event_hash == event.event_hash,
        |stored, event| {
            stored.line_index == event.line_index && stored.normalized_line == event.normalized_line
        },
        |stored, event| stored.normalized_line == event.normalized_line,
    ];
    for matcher in matchers {
        for (event_index, event) in events.iter().enumerate() {
            if matched[event_index].is_some() {
                continue;
            }
            if let Some(stored_index) = stored
                .iter()
                .enumerate()
                .find(|(index, row)| !used.contains(index) && matcher(row, event))
                .map(|(index, _)| index)
            {
                used.insert(stored_index);
                matched[event_index] = Some(stored_index);
            }
        }
    }
    for (event_index, stored_index) in matched.into_iter().enumerate() {
        match stored_index {
            Some(stored_index) if same_labels(&stored[stored_index], &events[event_index]) => {
                plan.unchanged += 1;
            }
            Some(stored_index) => plan.updates.push((stored[stored_index].id, event_index)),
            None => plan.inserts.push(event_index),
        }
    }
    plan.deletes = stored
        .iter()
        .enumerate()
        .filter(|(index, _)| !used.contains(index))
        .map(|(_, row)| row.id)
        .collect();
    plan
}

fn same_labels(stored: &StoredEvent, event: &PatchEventInsert) -> bool {
    stored.event_hash == event.event_hash
        && stored.entity_type == event.entity_type
        && stored.entity_name == event.entity_name
        && stored.change_type == event.change_type
        && stored.patch_title == event.patch_title
}

pub(crate) async fn load_stored_events(pool: &PgPool) -> Result<HashMap<i64, Vec<StoredEvent>>> {
    let rows = sqlx::query(
        r#"
        SELECT id, patch_snapshot_id, event_hash, line_index, normalized_line,
               entity_type, entity_name, change_type, patch_title
        FROM brain.patch_events
        WHERE patch_snapshot_id IS NOT NULL
        ORDER BY id ASC
        "#,
    )
    .fetch_all(pool)
    .await?;
    let mut grouped: HashMap<i64, Vec<StoredEvent>> = HashMap::new();
    for row in rows {
        grouped
            .entry(row.try_get("patch_snapshot_id")?)
            .or_default()
            .push(StoredEvent {
                id: row.try_get("id")?,
                event_hash: row.try_get("event_hash")?,
                line_index: row.try_get("line_index")?,
                normalized_line: row.try_get("normalized_line")?,
                entity_type: row.try_get("entity_type")?,
                entity_name: row.try_get("entity_name")?,
                change_type: row.try_get("change_type")?,
                patch_title: row.try_get("patch_title")?,
            });
    }
    Ok(grouped)
}

pub(crate) async fn apply_plan(
    pool: &PgPool,
    plan: &ReconcilePlan,
    events: &[PatchEventInsert],
) -> Result<()> {
    let mut tx = pool.begin().await?;
    delete_events(&mut tx, &plan.deletes).await?;
    let updated_ids = plan.updates.iter().map(|(id, _)| *id).collect::<Vec<_>>();
    park_hashes(&mut tx, &updated_ids).await?;
    for (id, event_index) in &plan.updates {
        update_event(&mut tx, *id, &events[*event_index]).await?;
    }
    for event_index in &plan.inserts {
        insert_event(&mut tx, &events[*event_index]).await?;
    }
    tx.commit().await?;
    Ok(())
}

pub(crate) async fn insert_events(pool: &PgPool, events: &[&PatchEventInsert]) -> Result<()> {
    let mut tx = pool.begin().await?;
    for event in events {
        insert_event(&mut tx, event).await?;
    }
    tx.commit().await?;
    Ok(())
}

pub(crate) async fn retire_snapshot_events(pool: &PgPool, stored: &[StoredEvent]) -> Result<()> {
    let ids = stored.iter().map(|row| row.id).collect::<Vec<_>>();
    let mut tx = pool.begin().await?;
    delete_events(&mut tx, &ids).await?;
    tx.commit().await?;
    Ok(())
}

async fn delete_events(tx: &mut Transaction<'_, Postgres>, ids: &[i64]) -> Result<()> {
    if ids.is_empty() {
        return Ok(());
    }
    sqlx::query("DELETE FROM brain.patch_events WHERE id = ANY($1)")
        .bind(ids)
        .execute(&mut **tx)
        .await?;
    Ok(())
}

async fn park_hashes(tx: &mut Transaction<'_, Postgres>, ids: &[i64]) -> Result<()> {
    if ids.is_empty() {
        return Ok(());
    }
    sqlx::query(
        "UPDATE brain.patch_events SET event_hash = 'reconcile:' || id::text WHERE id = ANY($1)",
    )
    .bind(ids)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

async fn update_event(
    tx: &mut Transaction<'_, Postgres>,
    id: i64,
    event: &PatchEventInsert,
) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE brain.patch_events SET
          patch_snapshot_id = $2, legacy_patch_snapshot_id = $3, patch_external_id = $4,
          patch_title = $5, patch_url = $6, source_kind = $7,
          posted_at = CASE WHEN $8 ~ '^-?[0-9]+$' THEN to_timestamp($8::double precision) ELSE $8::text::timestamptz END,
          line_index = $9, section = $10, entity_type = $11, entity_name = $12, subject = $13,
          change_type = $14, raw_line = $15, normalized_line = $16, old_value = $17,
          new_value = $18, confidence = $19, metadata = $20::text::jsonb, event_hash = $21
        WHERE id = $1
        "#,
    )
    .bind(id)
    .bind(event.patch_snapshot_id)
    .bind(event.legacy_patch_snapshot_id)
    .bind(&event.patch_external_id)
    .bind(&event.patch_title)
    .bind(&event.patch_url)
    .bind(&event.source_kind)
    .bind(&event.posted_at)
    .bind(event.line_index)
    .bind(&event.section)
    .bind(&event.entity_type)
    .bind(&event.entity_name)
    .bind(&event.subject)
    .bind(&event.change_type)
    .bind(&event.raw_line)
    .bind(&event.normalized_line)
    .bind(&event.old_value)
    .bind(&event.new_value)
    .bind(event.confidence)
    .bind(event.metadata_json()?)
    .bind(&event.event_hash)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

async fn insert_event(tx: &mut Transaction<'_, Postgres>, event: &PatchEventInsert) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO brain.patch_events(
          patch_snapshot_id, legacy_patch_snapshot_id, patch_external_id, patch_title, patch_url,
          source_kind, posted_at, line_index, section, entity_type,
          entity_name, subject, change_type, raw_line, normalized_line,
          old_value, new_value, confidence, metadata, event_hash, created_at
        )
        VALUES(
          $1, $2, $3, $4, $5,
          $6, CASE WHEN $7 ~ '^-?[0-9]+$' THEN to_timestamp($7::double precision) ELSE $7::text::timestamptz END, $8, $9, $10,
          $11, $12, $13, $14, $15,
          $16, $17, $18, $19::text::jsonb, $20, now()
        )
        ON CONFLICT (event_hash) DO NOTHING
        "#,
    )
    .bind(event.patch_snapshot_id)
    .bind(event.legacy_patch_snapshot_id)
    .bind(&event.patch_external_id)
    .bind(&event.patch_title)
    .bind(&event.patch_url)
    .bind(&event.source_kind)
    .bind(&event.posted_at)
    .bind(event.line_index)
    .bind(&event.section)
    .bind(&event.entity_type)
    .bind(&event.entity_name)
    .bind(&event.subject)
    .bind(&event.change_type)
    .bind(&event.raw_line)
    .bind(&event.normalized_line)
    .bind(&event.old_value)
    .bind(&event.new_value)
    .bind(event.confidence)
    .bind(event.metadata_json()?)
    .bind(&event.event_hash)
    .execute(&mut **tx)
    .await?;
    Ok(())
}
