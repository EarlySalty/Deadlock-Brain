use crate::{invalid, sha256_hex, EntityRow, PatchLineRow, Result};
use sqlx::{PgConnection, Row};
use std::collections::BTreeMap;

pub const LEGACY_TABLES: [&str; 4] = [
    "entities",
    "entity_aliases",
    "patch_event_enrichments",
    "patch_events",
];

pub struct LegacyRead {
    pub patch_lines: Vec<PatchLineRow>,
    pub entities: Vec<EntityRow>,
    pub schema_sha256: String,
    pub table_counts: BTreeMap<String, i64>,
}

pub async fn read_legacy(connection: &mut PgConnection) -> Result<LegacyRead> {
    sqlx::query("BEGIN ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *connection)
        .await?;
    let result = read_inside(connection).await;
    sqlx::query("ROLLBACK").execute(&mut *connection).await?;
    result
}

async fn read_inside(connection: &mut PgConnection) -> Result<LegacyRead> {
    let columns: Vec<String> = sqlx::query_scalar(
        "SELECT table_name||'.'||column_name||':'||data_type||':'||is_nullable
         FROM information_schema.columns
         WHERE table_schema='brain_legacy' AND table_name = ANY($1)
         ORDER BY table_name, ordinal_position",
    )
    .bind(LEGACY_TABLES.to_vec())
    .fetch_all(&mut *connection)
    .await?;
    let tables: std::collections::BTreeSet<&str> =
        columns.iter().filter_map(|c| c.split('.').next()).collect();
    if tables.len() != LEGACY_TABLES.len() {
        return Err(invalid("brain_legacy archive tables missing"));
    }
    let schema_sha256 = sha256_hex(columns.join("\n").as_bytes());
    let mut table_counts = BTreeMap::new();
    for table in LEGACY_TABLES {
        let count: i64 = sqlx::query_scalar(&format!("SELECT count(*) FROM brain_legacy.{table}"))
            .fetch_one(&mut *connection)
            .await?;
        table_counts.insert(table.to_string(), count);
    }
    let rows = sqlx::query(
        "SELECT e.id, e.patch_external_id, e.patch_title, e.patch_url,
                extract(epoch FROM e.posted_at)::bigint AS posted_epoch,
                to_char(e.posted_at AT TIME ZONE 'UTC', 'YYYY-MM-DD\"T\"HH24:MI:SS\"Z\"') AS posted_iso,
                e.source_kind, e.line_index, e.section, e.entity_name, e.raw_line,
                e.metadata->>'source_language' AS language,
                n.stat_name, n.old_value, n.new_value, n.unit
         FROM brain_legacy.patch_events e
         LEFT JOIN brain_legacy.patch_event_enrichments n ON n.patch_event_id = e.id
         ORDER BY e.patch_external_id, e.line_index, e.id, n.id",
    )
    .fetch_all(&mut *connection)
    .await?;
    let mut patch_lines = Vec::with_capacity(rows.len());
    let mut last_event = None;
    for row in rows {
        let event_id: i64 = row.try_get("id")?;
        if last_event == Some(event_id) {
            return Err(invalid("patch event with multiple enrichments"));
        }
        last_event = Some(event_id);
        patch_lines.push(PatchLineRow {
            event_id,
            patch_external_id: row
                .try_get::<Option<String>, _>("patch_external_id")?
                .ok_or_else(|| invalid("patch event without external id"))?,
            patch_title: row.try_get("patch_title")?,
            patch_url: row.try_get("patch_url")?,
            posted_at_epoch: row.try_get("posted_epoch")?,
            posted_at_iso: row.try_get("posted_iso")?,
            source_kind: row.try_get("source_kind")?,
            line_index: row.try_get("line_index")?,
            section: row.try_get("section")?,
            entity_name: row.try_get("entity_name")?,
            raw_line: row.try_get("raw_line")?,
            language: row.try_get("language")?,
            stat_name: row.try_get("stat_name")?,
            old_value: row.try_get("old_value")?,
            new_value: row.try_get("new_value")?,
            unit: row.try_get("unit")?,
        });
    }
    let alias_rows = sqlx::query(
        "SELECT entity_id, alias, alias_kind FROM brain_legacy.entity_aliases
         ORDER BY entity_id, alias, alias_kind, id",
    )
    .fetch_all(&mut *connection)
    .await?;
    let mut aliases: BTreeMap<i64, Vec<(String, Option<String>)>> = BTreeMap::new();
    for row in alias_rows {
        let entity: Option<i64> = row.try_get("entity_id")?;
        let entity = entity.ok_or_else(|| invalid("alias without entity"))?;
        aliases
            .entry(entity)
            .or_default()
            .push((row.try_get("alias")?, row.try_get("alias_kind")?));
    }
    let entity_rows = sqlx::query(
        "SELECT id, entity_type, canonical_name, primary_external_id, source, metadata
         FROM brain_legacy.entities ORDER BY id",
    )
    .fetch_all(&mut *connection)
    .await?;
    let mut entities = Vec::with_capacity(entity_rows.len());
    for row in entity_rows {
        let id: i64 = row.try_get("id")?;
        entities.push(EntityRow {
            entity_id: id,
            entity_type: row.try_get("entity_type")?,
            canonical_name: row.try_get("canonical_name")?,
            primary_external_id: row.try_get("primary_external_id")?,
            source: row.try_get("source")?,
            metadata: row
                .try_get::<Option<serde_json::Value>, _>("metadata")?
                .unwrap_or(serde_json::Value::Null),
            aliases: aliases.remove(&id).unwrap_or_default(),
        });
    }
    if !aliases.is_empty() {
        return Err(invalid("aliases reference unknown entities"));
    }
    Ok(LegacyRead {
        patch_lines,
        entities,
        schema_sha256,
        table_counts,
    })
}
