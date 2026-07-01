use std::env;

use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use postgres::{Client, NoTls, Transaction};
use rusqlite::{Connection, OptionalExtension};
use serde_json::{json, Value};

#[derive(Debug, Clone)]
pub struct PgExportOptions {
    pub dsn_env: String,
    pub dry_run: bool,
}

pub fn export_to_postgres(conn: &Connection, options: &PgExportOptions) -> Result<Value> {
    let local = local_counts(conn)?;
    if options.dry_run {
        return Ok(json!({
            "dry_run": true,
            "target": "postgres",
            "dsn_env": options.dsn_env,
            "local_counts": local,
            "writes": false,
        }));
    }

    let dsn = env::var(&options.dsn_env).map_err(|_| {
        anyhow!(
            "{} ist nicht gesetzt; DSN wird nicht ausgegeben.",
            options.dsn_env
        )
    })?;
    let mut client = Client::connect(&dsn, NoTls).map_err(|_| {
        anyhow!("Konnte zentrale Postgres-DB nicht oeffnen; DSN wird nicht ausgegeben.")
    })?;
    ensure_pg_schema(&mut client)?;

    let mut tx = client.transaction()?;
    let source_documents = export_source_documents(conn, &mut tx)?;
    let entity_snapshots = export_entity_snapshots(conn, &mut tx)?;
    let patch_events = export_patch_events(conn, &mut tx)?;
    let forum_claims = export_forum_claims(conn, &mut tx)?;
    let knowledge_from_patch_events = materialize_patch_knowledge_events(&mut tx)?;
    let knowledge_from_forum_claims = materialize_forum_knowledge_events(&mut tx)?;
    let current_entity_states = materialize_current_entity_states(&mut tx)?;
    tx.commit()?;

    Ok(json!({
        "dry_run": false,
        "target": "postgres",
        "dsn_env": options.dsn_env,
        "local_counts": local,
        "exported": {
            "source_documents": source_documents,
            "entity_snapshots": entity_snapshots,
            "patch_events": patch_events,
            "forum_claims": forum_claims,
            "knowledge_events_from_patch_events": knowledge_from_patch_events,
            "knowledge_events_from_forum_claims": knowledge_from_forum_claims,
            "current_entity_states": current_entity_states,
        },
        "policy": {
            "forum_claims": "historical_quarantine bleibt erhalten",
            "current_state": "trusted/current snapshots gewinnen nach source_priority und observed_at"
        }
    }))
}

fn ensure_pg_schema(client: &mut Client) -> Result<()> {
    let row = client
        .query_one(
            "SELECT to_regclass('brain.knowledge_events')::text, to_regclass('brain.current_entity_state')::text",
            &[],
        )
        .context("pruefe brain-Schema in Postgres")?;
    let knowledge_events: Option<String> = row.get(0);
    let current_state: Option<String> = row.get(1);
    if knowledge_events.is_none() || current_state.is_none() {
        return Err(anyhow!(
            "Zentrales brain-Schema fehlt. Erst die Central-DB-Migration 0012 anwenden."
        ));
    }
    Ok(())
}

fn local_counts(conn: &Connection) -> Result<Value> {
    Ok(json!({
        "source_documents": table_count(conn, "source_documents")?,
        "entity_snapshots": table_count(conn, "entity_snapshots")?,
        "patch_events": table_count(conn, "patch_events")?,
        "forum_claims": table_count(conn, "forum_claims")?,
    }))
}

fn table_count(conn: &Connection, table: &str) -> Result<i64> {
    if !table_exists(conn, table)? {
        return Ok(0);
    }
    let sql = format!("SELECT COUNT(*) FROM {table}");
    Ok(conn.query_row(&sql, [], |row| row.get(0))?)
}

fn table_exists(conn: &Connection, table: &str) -> Result<bool> {
    Ok(conn
        .query_row(
            "SELECT 1 FROM sqlite_master WHERE type='table' AND name=?1",
            [table],
            |_| Ok(()),
        )
        .optional()?
        .is_some())
}

fn export_source_documents(conn: &Connection, tx: &mut Transaction<'_>) -> Result<u64> {
    if !table_exists(conn, "source_documents")? {
        return Ok(0);
    }
    let mut stmt = conn.prepare(
        r#"
        SELECT id, source, external_id, title, url, content_type, raw_path, content_hash,
               fetched_at, metadata_json
        FROM source_documents
        ORDER BY id
        "#,
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(SourceDocumentRow {
            id: row.get(0)?,
            source: row.get(1)?,
            external_id: row.get(2)?,
            title: row.get(3)?,
            url: row.get(4)?,
            content_type: row.get(5)?,
            raw_path: row.get(6)?,
            content_hash: row.get(7)?,
            fetched_at: unix_param(Some(row.get(8)?)),
            metadata: json_text(row.get(9)?, "{}"),
        })
    })?;

    let mut changed = 0;
    for row in rows {
        let row = row?;
        changed += tx.execute(
            r#"
            INSERT INTO brain.source_documents(
                legacy_sqlite_id, source, external_id, title, url, content_type, raw_path,
                content_hash, fetched_at, metadata
            )
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,to_timestamp($9::double precision),$10::text::jsonb)
            ON CONFLICT (source, external_id, content_hash) DO UPDATE SET
                legacy_sqlite_id = EXCLUDED.legacy_sqlite_id,
                title = EXCLUDED.title,
                url = EXCLUDED.url,
                content_type = EXCLUDED.content_type,
                raw_path = EXCLUDED.raw_path,
                fetched_at = EXCLUDED.fetched_at,
                metadata = EXCLUDED.metadata
            "#,
            &[
                &row.id,
                &row.source,
                &row.external_id,
                &row.title,
                &row.url,
                &row.content_type,
                &row.raw_path,
                &row.content_hash,
                &row.fetched_at,
                &row.metadata,
            ],
        )?;
    }
    Ok(changed)
}

fn export_entity_snapshots(conn: &Connection, tx: &mut Transaction<'_>) -> Result<u64> {
    if !table_exists(conn, "entity_snapshots")? {
        return Ok(0);
    }
    let mut stmt = conn.prepare(
        r#"
        SELECT id, source, entity_type, external_id, canonical_name, payload_hash,
               payload_json, fetched_at, source_document_id
        FROM entity_snapshots
        ORDER BY id
        "#,
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(EntitySnapshotRow {
            id: row.get(0)?,
            source: row.get(1)?,
            entity_type: row.get(2)?,
            external_id: row.get(3)?,
            canonical_name: row.get(4)?,
            payload_hash: row.get(5)?,
            payload: json_text(row.get(6)?, "{}"),
            fetched_at: unix_param(Some(row.get(7)?)),
            source_document_id: row.get(8)?,
        })
    })?;

    let mut changed = 0;
    for row in rows {
        let row = row?;
        changed += tx.execute(
            r#"
            INSERT INTO brain.entity_snapshots(
                legacy_sqlite_id, source, entity_type, external_id, canonical_name, payload_hash,
                payload, fetched_at, source_document_id, legacy_source_document_id
            )
            VALUES (
                $1,$2,$3,$4,$5,$6,$7::text::jsonb,to_timestamp($8::double precision),
                (SELECT id FROM brain.source_documents WHERE legacy_sqlite_id = $9),
                $9
            )
            ON CONFLICT (source, entity_type, external_id, payload_hash) DO UPDATE SET
                legacy_sqlite_id = EXCLUDED.legacy_sqlite_id,
                canonical_name = EXCLUDED.canonical_name,
                payload = EXCLUDED.payload,
                fetched_at = EXCLUDED.fetched_at,
                source_document_id = EXCLUDED.source_document_id,
                legacy_source_document_id = EXCLUDED.legacy_source_document_id
            "#,
            &[
                &row.id,
                &row.source,
                &row.entity_type,
                &row.external_id,
                &row.canonical_name,
                &row.payload_hash,
                &row.payload,
                &row.fetched_at,
                &row.source_document_id,
            ],
        )?;
    }
    Ok(changed)
}

fn export_patch_events(conn: &Connection, tx: &mut Transaction<'_>) -> Result<u64> {
    if !table_exists(conn, "patch_events")? {
        return Ok(0);
    }
    let mut stmt = conn.prepare(
        r#"
        SELECT id, patch_snapshot_id, patch_external_id, patch_title, patch_url, source_kind,
               posted_at, line_index, section, entity_type, entity_name, subject, change_type,
               raw_line, normalized_line, old_value, new_value, confidence, metadata_json,
               event_hash, created_at
        FROM patch_events
        ORDER BY id
        "#,
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(PatchEventRow {
            id: row.get(0)?,
            patch_snapshot_id: row.get(1)?,
            patch_external_id: row.get(2)?,
            patch_title: row.get(3)?,
            patch_url: row.get(4)?,
            source_kind: row.get(5)?,
            posted_at: text_timestamptz(row.get(6)?),
            line_index: row.get(7)?,
            section: row.get(8)?,
            entity_type: row.get(9)?,
            entity_name: row.get(10)?,
            subject: row.get(11)?,
            change_type: row.get(12)?,
            raw_line: row.get(13)?,
            normalized_line: row.get(14)?,
            old_value: row.get(15)?,
            new_value: row.get(16)?,
            confidence: row.get(17)?,
            metadata: json_text(row.get(18)?, "{}"),
            event_hash: row.get(19)?,
            created_at: unix_param(Some(row.get(20)?)),
        })
    })?;

    let mut changed = 0;
    for row in rows {
        let row = row?;
        changed += tx.execute(
            r#"
            INSERT INTO brain.patch_events(
                legacy_sqlite_id, patch_snapshot_id, legacy_patch_snapshot_id, patch_external_id,
                patch_title, patch_url, source_kind, posted_at, line_index, section, entity_type,
                entity_name, subject, change_type, raw_line, normalized_line, old_value, new_value,
                confidence, metadata, event_hash, created_at
            )
            VALUES (
                $1,
                (SELECT id FROM brain.entity_snapshots WHERE legacy_sqlite_id = $2),
                $2,$3,$4,$5,$6,$7::text::timestamptz,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,
                $18,$19::text::jsonb,$20,to_timestamp($21::double precision)
            )
            ON CONFLICT (event_hash) DO UPDATE SET
                legacy_sqlite_id = EXCLUDED.legacy_sqlite_id,
                patch_snapshot_id = EXCLUDED.patch_snapshot_id,
                legacy_patch_snapshot_id = EXCLUDED.legacy_patch_snapshot_id,
                patch_title = EXCLUDED.patch_title,
                patch_url = EXCLUDED.patch_url,
                source_kind = EXCLUDED.source_kind,
                posted_at = EXCLUDED.posted_at,
                section = EXCLUDED.section,
                entity_type = EXCLUDED.entity_type,
                entity_name = EXCLUDED.entity_name,
                subject = EXCLUDED.subject,
                change_type = EXCLUDED.change_type,
                raw_line = EXCLUDED.raw_line,
                normalized_line = EXCLUDED.normalized_line,
                old_value = EXCLUDED.old_value,
                new_value = EXCLUDED.new_value,
                confidence = EXCLUDED.confidence,
                metadata = EXCLUDED.metadata,
                created_at = EXCLUDED.created_at
            "#,
            &[
                &row.id,
                &row.patch_snapshot_id,
                &row.patch_external_id,
                &row.patch_title,
                &row.patch_url,
                &row.source_kind,
                &row.posted_at,
                &row.line_index,
                &row.section,
                &row.entity_type,
                &row.entity_name,
                &row.subject,
                &row.change_type,
                &row.raw_line,
                &row.normalized_line,
                &row.old_value,
                &row.new_value,
                &row.confidence,
                &row.metadata,
                &row.event_hash,
                &row.created_at,
            ],
        )?;
    }
    Ok(changed)
}

fn export_forum_claims(conn: &Connection, tx: &mut Transaction<'_>) -> Result<u64> {
    if !table_exists(conn, "forum_claims")? {
        return Ok(0);
    }
    let mut stmt = conn.prepare(
        r#"
        SELECT id, post_snapshot_id, thread_id, post_id, thread_title, source_url, posted_at,
               author, author_role, claim_hash, claim_index, claim_type, entity_type, entity_name,
               claim_text, evidence_quote, source_trust, validity_status, currentness, confidence,
               safety_labels_json, source_references_json, metadata_json, created_at, updated_at
        FROM forum_claims
        ORDER BY id
        "#,
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(ForumClaimRow {
            id: row.get(0)?,
            post_snapshot_id: row.get(1)?,
            thread_id: row.get(2)?,
            post_id: row.get(3)?,
            thread_title: row.get(4)?,
            source_url: row.get(5)?,
            posted_at: text_timestamptz(row.get(6)?),
            author: row.get(7)?,
            author_role: row.get(8)?,
            claim_hash: row.get(9)?,
            claim_index: row.get(10)?,
            claim_type: row.get(11)?,
            entity_type: row.get(12)?,
            entity_name: row.get(13)?,
            claim_text: row.get(14)?,
            evidence_quote: row.get(15)?,
            source_trust: row.get(16)?,
            validity_status: row.get(17)?,
            currentness: row.get(18)?,
            confidence: row.get(19)?,
            safety_labels: json_text(row.get(20)?, "[]"),
            source_references: json_text(row.get(21)?, "[]"),
            metadata: json_text(row.get(22)?, "{}"),
            created_at: unix_param(Some(row.get(23)?)),
            updated_at: unix_param(Some(row.get(24)?)),
        })
    })?;

    let mut changed = 0;
    for row in rows {
        let row = row?;
        changed += tx.execute(
            r#"
            INSERT INTO brain.forum_claims(
                legacy_sqlite_id, post_snapshot_id, legacy_post_snapshot_id, thread_id, post_id,
                thread_title, source_url, posted_at, author, author_role, claim_hash, claim_index,
                claim_type, entity_type, entity_name, claim_text, evidence_quote, source_trust,
                validity_status, currentness, confidence, safety_labels, source_references,
                metadata, created_at, updated_at
            )
            VALUES (
                $1,
                (SELECT id FROM brain.entity_snapshots WHERE legacy_sqlite_id = $2),
                $2,$3,$4,$5,$6,$7::text::timestamptz,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19,
                $20,$21::text::jsonb,$22::text::jsonb,$23::text::jsonb,
                to_timestamp($24::double precision),to_timestamp($25::double precision)
            )
            ON CONFLICT (claim_hash) DO UPDATE SET
                legacy_sqlite_id = EXCLUDED.legacy_sqlite_id,
                post_snapshot_id = EXCLUDED.post_snapshot_id,
                legacy_post_snapshot_id = EXCLUDED.legacy_post_snapshot_id,
                thread_title = EXCLUDED.thread_title,
                source_url = EXCLUDED.source_url,
                posted_at = EXCLUDED.posted_at,
                author = EXCLUDED.author,
                author_role = EXCLUDED.author_role,
                claim_type = EXCLUDED.claim_type,
                entity_type = EXCLUDED.entity_type,
                entity_name = EXCLUDED.entity_name,
                claim_text = EXCLUDED.claim_text,
                evidence_quote = EXCLUDED.evidence_quote,
                source_trust = EXCLUDED.source_trust,
                validity_status = EXCLUDED.validity_status,
                currentness = EXCLUDED.currentness,
                confidence = EXCLUDED.confidence,
                safety_labels = EXCLUDED.safety_labels,
                source_references = EXCLUDED.source_references,
                metadata = EXCLUDED.metadata,
                updated_at = EXCLUDED.updated_at
            "#,
            &[
                &row.id,
                &row.post_snapshot_id,
                &row.thread_id,
                &row.post_id,
                &row.thread_title,
                &row.source_url,
                &row.posted_at,
                &row.author,
                &row.author_role,
                &row.claim_hash,
                &row.claim_index,
                &row.claim_type,
                &row.entity_type,
                &row.entity_name,
                &row.claim_text,
                &row.evidence_quote,
                &row.source_trust,
                &row.validity_status,
                &row.currentness,
                &row.confidence,
                &row.safety_labels,
                &row.source_references,
                &row.metadata,
                &row.created_at,
                &row.updated_at,
            ],
        )?;
    }
    Ok(changed)
}

fn materialize_patch_knowledge_events(tx: &mut Transaction<'_>) -> Result<u64> {
    Ok(tx.execute(
        r#"
        INSERT INTO brain.knowledge_events(
            event_hash, event_source, source_table, source_legacy_id, source_document_id,
            snapshot_id, patch_event_id, entity_type, entity_name, subject, event_type,
            validity_status, currentness, trust_tier, source_url, occurred_at, observed_at,
            effective_from, raw_text, normalized_text, old_value, new_value, confidence,
            source_references, payload, metadata
        )
        SELECT
            'patch:' || pe.event_hash,
            'patch_event',
            'patch_events',
            pe.legacy_sqlite_id,
            sd.id,
            pe.patch_snapshot_id,
            pe.id,
            pe.entity_type,
            pe.entity_name,
            pe.subject,
            pe.change_type,
            'patch_history',
            'historical_patch_event',
            CASE
                WHEN pe.source_kind IN ('steam', 'deadlock_data') THEN 'trusted'
                WHEN pe.source_kind = 'forum' THEN 'community_patch_source'
                ELSE 'imported'
            END,
            pe.patch_url,
            pe.posted_at,
            COALESCE(pe.posted_at, pe.created_at),
            pe.posted_at,
            pe.raw_line,
            pe.normalized_line,
            pe.old_value,
            pe.new_value,
            pe.confidence,
            jsonb_build_array(jsonb_build_object('url', pe.patch_url, 'title', pe.patch_title)),
            jsonb_build_object(
                'patch_external_id', pe.patch_external_id,
                'patch_title', pe.patch_title,
                'source_kind', pe.source_kind,
                'section', pe.section
            ),
            pe.metadata
        FROM brain.patch_events pe
        LEFT JOIN brain.entity_snapshots es ON es.id = pe.patch_snapshot_id
        LEFT JOIN brain.source_documents sd ON sd.id = es.source_document_id
        ON CONFLICT (event_hash) DO UPDATE SET
            source_document_id = EXCLUDED.source_document_id,
            snapshot_id = EXCLUDED.snapshot_id,
            patch_event_id = EXCLUDED.patch_event_id,
            entity_type = EXCLUDED.entity_type,
            entity_name = EXCLUDED.entity_name,
            subject = EXCLUDED.subject,
            event_type = EXCLUDED.event_type,
            validity_status = EXCLUDED.validity_status,
            currentness = EXCLUDED.currentness,
            trust_tier = EXCLUDED.trust_tier,
            source_url = EXCLUDED.source_url,
            occurred_at = EXCLUDED.occurred_at,
            observed_at = EXCLUDED.observed_at,
            effective_from = EXCLUDED.effective_from,
            raw_text = EXCLUDED.raw_text,
            normalized_text = EXCLUDED.normalized_text,
            old_value = EXCLUDED.old_value,
            new_value = EXCLUDED.new_value,
            confidence = EXCLUDED.confidence,
            source_references = EXCLUDED.source_references,
            payload = EXCLUDED.payload,
            metadata = EXCLUDED.metadata,
            updated_at = now()
        "#,
        &[],
    )?)
}

fn materialize_forum_knowledge_events(tx: &mut Transaction<'_>) -> Result<u64> {
    Ok(tx.execute(
        r#"
        INSERT INTO brain.knowledge_events(
            event_hash, event_source, source_table, source_legacy_id, source_document_id,
            snapshot_id, forum_claim_id, entity_type, entity_name, subject, event_type,
            validity_status, currentness, trust_tier, source_url, occurred_at, observed_at,
            raw_text, normalized_text, evidence_quote, confidence, safety_labels,
            source_references, payload, metadata
        )
        SELECT
            'forum:' || fc.claim_hash,
            'forum_claim',
            'forum_claims',
            fc.legacy_sqlite_id,
            sd.id,
            fc.post_snapshot_id,
            fc.id,
            fc.entity_type,
            fc.entity_name,
            fc.thread_title,
            fc.claim_type,
            fc.validity_status,
            fc.currentness,
            fc.source_trust,
            fc.source_url,
            fc.posted_at,
            COALESCE(fc.posted_at, fc.created_at),
            fc.claim_text,
            fc.claim_text,
            fc.evidence_quote,
            fc.confidence,
            fc.safety_labels,
            fc.source_references,
            jsonb_build_object(
                'thread_id', fc.thread_id,
                'post_id', fc.post_id,
                'author', fc.author,
                'author_role', fc.author_role
            ),
            fc.metadata
        FROM brain.forum_claims fc
        LEFT JOIN brain.entity_snapshots es ON es.id = fc.post_snapshot_id
        LEFT JOIN brain.source_documents sd ON sd.id = es.source_document_id
        ON CONFLICT (event_hash) DO UPDATE SET
            source_document_id = EXCLUDED.source_document_id,
            snapshot_id = EXCLUDED.snapshot_id,
            forum_claim_id = EXCLUDED.forum_claim_id,
            entity_type = EXCLUDED.entity_type,
            entity_name = EXCLUDED.entity_name,
            subject = EXCLUDED.subject,
            event_type = EXCLUDED.event_type,
            validity_status = EXCLUDED.validity_status,
            currentness = EXCLUDED.currentness,
            trust_tier = EXCLUDED.trust_tier,
            source_url = EXCLUDED.source_url,
            occurred_at = EXCLUDED.occurred_at,
            observed_at = EXCLUDED.observed_at,
            raw_text = EXCLUDED.raw_text,
            normalized_text = EXCLUDED.normalized_text,
            evidence_quote = EXCLUDED.evidence_quote,
            confidence = EXCLUDED.confidence,
            safety_labels = EXCLUDED.safety_labels,
            source_references = EXCLUDED.source_references,
            payload = EXCLUDED.payload,
            metadata = EXCLUDED.metadata,
            updated_at = now()
        "#,
        &[],
    )?)
}

fn materialize_current_entity_states(tx: &mut Transaction<'_>) -> Result<u64> {
    Ok(tx.execute(
        r#"
        INSERT INTO brain.current_entity_state(
            entity_type, entity_name, state_kind, winning_snapshot_id, source, source_priority,
            valid_from, observed_at, content_hash, payload, metadata
        )
        SELECT DISTINCT ON (entity_type, canonical_name, state_kind)
            entity_type,
            canonical_name,
            state_kind,
            id,
            source,
            source_priority,
            fetched_at,
            fetched_at,
            payload_hash,
            payload,
            jsonb_build_object('external_id', external_id, 'legacy_sqlite_id', legacy_sqlite_id)
        FROM (
            SELECT
                es.*,
                'canonical_snapshot'::text AS state_kind,
                CASE
                    WHEN es.source IN ('deadlock_data', 'deadlock_assets_api') THEN 100
                    WHEN es.source = 'statlocker' THEN 80
                    WHEN es.source = 'google_sheet' THEN 70
                    WHEN es.source = 'deadlock_patchnotes_db' THEN 50
                    ELSE 10
                END AS source_priority
            FROM brain.entity_snapshots es
            WHERE es.canonical_name IS NOT NULL
              AND es.canonical_name <> ''
              AND es.entity_type NOT IN ('forum_thread', 'forum_post', 'patchnote')
              AND es.source <> 'playdeadlock_forum'
        ) ranked
        ORDER BY entity_type, canonical_name, state_kind, source_priority DESC, fetched_at DESC, id DESC
        ON CONFLICT (entity_type, entity_name, state_kind) DO UPDATE SET
            winning_snapshot_id = EXCLUDED.winning_snapshot_id,
            source = EXCLUDED.source,
            source_priority = EXCLUDED.source_priority,
            valid_from = EXCLUDED.valid_from,
            observed_at = EXCLUDED.observed_at,
            content_hash = EXCLUDED.content_hash,
            payload = EXCLUDED.payload,
            metadata = EXCLUDED.metadata,
            updated_at = now()
        WHERE EXCLUDED.source_priority > brain.current_entity_state.source_priority
           OR (
                EXCLUDED.source_priority = brain.current_entity_state.source_priority
            AND EXCLUDED.observed_at >= brain.current_entity_state.observed_at
           )
        "#,
        &[],
    )?)
}

#[derive(Debug)]
struct SourceDocumentRow {
    id: i64,
    source: String,
    external_id: String,
    title: Option<String>,
    url: Option<String>,
    content_type: String,
    raw_path: String,
    content_hash: String,
    fetched_at: Option<f64>,
    metadata: String,
}

#[derive(Debug)]
struct EntitySnapshotRow {
    id: i64,
    source: String,
    entity_type: String,
    external_id: String,
    canonical_name: Option<String>,
    payload_hash: String,
    payload: String,
    fetched_at: Option<f64>,
    source_document_id: Option<i64>,
}

#[derive(Debug)]
struct PatchEventRow {
    id: i64,
    patch_snapshot_id: i64,
    patch_external_id: String,
    patch_title: Option<String>,
    patch_url: Option<String>,
    source_kind: String,
    posted_at: Option<String>,
    line_index: i64,
    section: Option<String>,
    entity_type: String,
    entity_name: Option<String>,
    subject: Option<String>,
    change_type: String,
    raw_line: String,
    normalized_line: String,
    old_value: Option<String>,
    new_value: Option<String>,
    confidence: f64,
    metadata: String,
    event_hash: String,
    created_at: Option<f64>,
}

#[derive(Debug)]
struct ForumClaimRow {
    id: i64,
    post_snapshot_id: i64,
    thread_id: String,
    post_id: String,
    thread_title: Option<String>,
    source_url: String,
    posted_at: Option<String>,
    author: Option<String>,
    author_role: Option<String>,
    claim_hash: String,
    claim_index: i64,
    claim_type: String,
    entity_type: Option<String>,
    entity_name: Option<String>,
    claim_text: String,
    evidence_quote: String,
    source_trust: String,
    validity_status: String,
    currentness: String,
    confidence: f64,
    safety_labels: String,
    source_references: String,
    metadata: String,
    created_at: Option<f64>,
    updated_at: Option<f64>,
}

fn unix_param(value: Option<i64>) -> Option<f64> {
    value.map(|raw| {
        let seconds = raw as f64;
        if seconds > 10_000_000_000.0 {
            seconds / 1000.0
        } else {
            seconds
        }
    })
}

fn json_text(value: String, default: &str) -> String {
    if serde_json::from_str::<Value>(&value).is_ok() {
        value
    } else {
        default.to_string()
    }
}

fn text_timestamptz(value: Option<String>) -> Option<String> {
    let raw = value?.trim().to_string();
    if raw.is_empty() {
        return None;
    }
    if let Ok(dt) = DateTime::parse_from_rfc3339(&raw) {
        return Some(dt.with_timezone(&Utc).to_rfc3339());
    }
    for format in [
        "%Y-%m-%dT%H:%M:%S%.f%z",
        "%Y-%m-%dT%H:%M:%S%z",
        "%Y-%m-%d %H:%M:%S%z",
    ] {
        if let Ok(dt) = DateTime::parse_from_str(&raw, format) {
            return Some(dt.with_timezone(&Utc).to_rfc3339());
        }
    }
    for format in ["%Y-%m-%d %H:%M:%S", "%Y-%m-%d %H:%M"] {
        if let Ok(dt) = NaiveDateTime::parse_from_str(&raw, format) {
            return Some(Utc.from_utc_datetime(&dt).to_rfc3339());
        }
    }
    if let Ok(date) = NaiveDate::parse_from_str(&raw, "%Y-%m-%d") {
        if let Some(dt) = date.and_hms_opt(0, 0, 0) {
            return Some(Utc.from_utc_datetime(&dt).to_rfc3339());
        }
    }
    None
}
