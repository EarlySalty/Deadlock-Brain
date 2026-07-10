use std::{
    fs,
    path::{Path, PathBuf},
};

use serde_json::Value;
use sha2::{Digest, Sha256};
use sqlx::PgPool;

use crate::{Result, SourcesError};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct EntitySnapshotInput {
    pub source: String,
    pub entity_type: String,
    pub external_id: String,
    pub canonical_name: Option<String>,
    pub payload: Value,
}

#[derive(Debug)]
pub(crate) struct SourceStore<'a> {
    pool: &'a PgPool,
    raw_dir: PathBuf,
}

impl<'a> SourceStore<'a> {
    pub(crate) fn new(pool: &'a PgPool, raw_dir: &Path) -> Result<Self> {
        fs::create_dir_all(raw_dir)?;
        Ok(Self {
            pool,
            raw_dir: raw_dir.to_path_buf(),
        })
    }

    pub(crate) fn pool(&self) -> &'a PgPool {
        self.pool
    }

    pub(crate) async fn begin_run(&self, source: &str) -> Result<i64> {
        let run_id = sqlx::query_scalar!(
            r#"
            INSERT INTO brain.source_runs(source, status, started_at)
            VALUES($1, 'running', now())
            RETURNING id
            "#,
            source,
        )
        .fetch_one(self.pool)
        .await?;
        Ok(run_id)
    }

    pub(crate) async fn finish_run(
        &self,
        run_id: i64,
        status: &str,
        summary: &Value,
    ) -> Result<()> {
        let summary_json = json_string(summary)?;
        sqlx::query!(
            r#"
            UPDATE brain.source_runs
            SET status=$1, finished_at=now(), summary=$2::text::jsonb
            WHERE id=$3
            "#,
            status,
            summary_json,
            run_id,
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub(crate) fn write_raw(
        &self,
        source: &str,
        external_id: &str,
        content: &[u8],
        suffix: &str,
    ) -> Result<PathBuf> {
        let source_dir = self.raw_dir.join(source);
        fs::create_dir_all(&source_dir)?;
        let digest = stable_hash_bytes(content);
        let safe_external = sanitize_external_id(external_id);
        let suffix = suffix.trim_start_matches('.');
        let path = source_dir.join(format!("{}.{}.{}", safe_external, &digest[..16], suffix));
        if !path.exists() {
            fs::write(&path, content)?;
        }
        Ok(path)
    }

    pub(crate) async fn upsert_source_document(
        &self,
        document: SourceDocumentInput<'_>,
    ) -> Result<i64> {
        let content_hash = stable_hash_bytes(document.content);
        let raw_path = document.raw_path.to_string_lossy().into_owned();
        let metadata_json = json_string(document.metadata)?;

        let inserted = sqlx::query_scalar!(
            r#"
            INSERT INTO brain.source_documents(
              source, external_id, title, url, content_type, raw_path,
              content_hash, fetched_at, metadata
            )
            VALUES($1, $2, $3, $4, $5, $6, $7, now(), $8::text::jsonb)
            ON CONFLICT (source, external_id, content_hash) DO NOTHING
            RETURNING id
            "#,
            document.source,
            document.external_id,
            document.title,
            document.url,
            document.content_type,
            raw_path,
            content_hash,
            metadata_json,
        )
        .fetch_optional(self.pool)
        .await?;

        if let Some(id) = inserted {
            return Ok(id);
        }

        let existing = sqlx::query_scalar!(
            r#"
            SELECT id FROM brain.source_documents
            WHERE source=$1 AND external_id=$2 AND content_hash=$3
            ORDER BY id DESC LIMIT 1
            "#,
            document.source,
            document.external_id,
            content_hash,
        )
        .fetch_optional(self.pool)
        .await?;

        existing.ok_or_else(|| {
            SourcesError::invariant("source_documents row missing after INSERT ... ON CONFLICT")
        })
    }

    pub(crate) async fn upsert_entity_snapshot(
        &self,
        snapshot: &EntitySnapshotInput,
        source_document_id: Option<i64>,
    ) -> Result<()> {
        let _ = self
            .upsert_entity_snapshot_id(snapshot, source_document_id)
            .await?;
        Ok(())
    }

    pub(crate) async fn upsert_entity_snapshot_id(
        &self,
        snapshot: &EntitySnapshotInput,
        source_document_id: Option<i64>,
    ) -> Result<i64> {
        let payload_json = json_string(&snapshot.payload)?;
        let payload_hash = stable_hash_text(&payload_json);

        let inserted = sqlx::query_scalar!(
            r#"
            INSERT INTO brain.entity_snapshots(
              source, entity_type, external_id, canonical_name, payload_hash,
              payload, fetched_at, source_document_id
            )
            VALUES($1, $2, $3, $4, $5, $6::text::jsonb, now(), $7)
            ON CONFLICT (source, entity_type, external_id, payload_hash) DO NOTHING
            RETURNING id
            "#,
            snapshot.source,
            snapshot.entity_type,
            snapshot.external_id,
            snapshot.canonical_name,
            payload_hash,
            payload_json,
            source_document_id,
        )
        .fetch_optional(self.pool)
        .await?;

        if let Some(id) = inserted {
            return Ok(id);
        }

        let existing = sqlx::query_scalar!(
            r#"
            SELECT id FROM brain.entity_snapshots
            WHERE source=$1 AND entity_type=$2 AND external_id=$3 AND payload_hash=$4
            ORDER BY id DESC LIMIT 1
            "#,
            snapshot.source,
            snapshot.entity_type,
            snapshot.external_id,
            payload_hash,
        )
        .fetch_optional(self.pool)
        .await?;

        existing.ok_or_else(|| {
            SourcesError::invariant("entity_snapshots row missing after INSERT ... ON CONFLICT")
        })
    }

    pub(crate) async fn insert_many_snapshots(
        &self,
        snapshots: &[EntitySnapshotInput],
        source_document_id: Option<i64>,
    ) -> Result<usize> {
        let mut count = 0usize;
        for snapshot in snapshots {
            self.upsert_entity_snapshot(snapshot, source_document_id)
                .await?;
            count += 1;
        }
        Ok(count)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SourceDocumentInput<'a> {
    pub source: &'a str,
    pub external_id: &'a str,
    pub title: Option<&'a str>,
    pub url: Option<&'a str>,
    pub content_type: &'a str,
    pub raw_path: &'a Path,
    pub content: &'a [u8],
    pub metadata: &'a Value,
}

/// Schliesst einen Source-Run ab: `ok` mit Summary bei Erfolg, sonst `error`
/// mit der Fehlermeldung als Summary. Ersetzt die frueher synchrone
/// `run_source`-Closure-Verdrahtung (async ist mit geliehenen Store-Futures
/// ohne HRTB-Ballast einfacher inline).
pub(crate) async fn complete_run(
    store: &SourceStore<'_>,
    run_id: i64,
    outcome: Result<Value>,
) -> Result<Value> {
    match outcome {
        Ok(summary) => {
            store.finish_run(run_id, "ok", &summary).await?;
            Ok(summary)
        }
        Err(error) => {
            let summary = serde_json::json!({ "error": error.to_string() });
            store.finish_run(run_id, "error", &summary).await?;
            Err(error)
        }
    }
}

/// Oeffnet den zentralen Postgres-Pool ueber `deadlock_brain_core::pg::pg_pool`
/// und bruecke dessen `anyhow`-Fehler auf `SourcesError` (die Fehlermeldung von
/// `pg_pool` enthaelt bewusst kein DSN/Secret).
pub(crate) async fn open_pool() -> Result<PgPool> {
    let pool = deadlock_brain_core::pg::pg_pool()
        .await
        .map_err(|error| SourcesError::Pool(error.to_string()))?;
    ensure_patch_changes_view(&pool).await?;
    Ok(pool)
}

pub(crate) async fn ensure_patch_changes_view(pool: &PgPool) -> Result<()> {
    let ready: bool = sqlx::query_scalar(
        r#"
        SELECT to_regclass('brain.patch_events') IS NOT NULL
           AND to_regclass('brain.patch_event_enrichments') IS NOT NULL
           AND to_regclass('patchnotes.deadlock_changelogs') IS NOT NULL
        "#,
    )
    .fetch_one(pool)
    .await?;
    if ready {
        sqlx::query(PATCH_CHANGES_VIEW_SQL).execute(pool).await?;
    }
    Ok(())
}

pub(crate) const PATCH_CHANGES_VIEW_SQL: &str = r#"
CREATE OR REPLACE VIEW brain.patch_changes AS
WITH patch_catalog AS (
    SELECT
        lower(regexp_replace(trim(title), '\s+', ' ', 'g')) AS patch_title_key,
        url AS patch_url,
        min(posted_at)::date AS patch_date
    FROM patchnotes.deadlock_changelogs
    WHERE title IS NOT NULL OR url IS NOT NULL
    GROUP BY 1, 2
), raw_base AS (
    SELECT
        pe.id,
        lower(regexp_replace(trim(pe.patch_title), '\s+', ' ', 'g')) AS patch_title_key,
        trim(pe.patch_title) AS patch_title,
        pe.patch_url,
        pe.posted_at,
        pe.created_at,
        pe.entity_type,
        pe.entity_name,
        CASE
            WHEN pee.ability_name IS NULL THEN NULL
            WHEN lower(left(trim(pee.ability_name), length(pe.entity_name) + 1)) = lower(pe.entity_name || ':')
                THEN trim(substr(trim(pee.ability_name), length(pe.entity_name) + 2))
            ELSE trim(pee.ability_name)
        END AS ability_name_raw,
        CASE
            WHEN pee.stat_name IS NULL THEN NULL
            WHEN lower(left(trim(pee.stat_name), length(pe.entity_name) + 1)) = lower(pe.entity_name || ':')
                THEN trim(substr(trim(pee.stat_name), length(pe.entity_name) + 2))
            ELSE trim(pee.stat_name)
        END AS stat_name_raw,
        coalesce(nullif(trim(pee.old_value), ''), nullif(trim(pe.old_value), '')) AS old_value,
        coalesce(nullif(trim(pee.new_value), ''), nullif(trim(pe.new_value), '')) AS new_value,
        pe.change_type,
        CASE
            WHEN lower(left(trim(coalesce(pe.normalized_line, pe.raw_line)), length(pe.entity_name) + 1)) = lower(pe.entity_name || ':')
                THEN trim(substr(trim(coalesce(pe.normalized_line, pe.raw_line)), length(pe.entity_name) + 2))
            ELSE trim(coalesce(pe.normalized_line, pe.raw_line))
        END AS cleaned_source_line,
        lower(left(trim(coalesce(pe.normalized_line, pe.raw_line)), length(pe.entity_name) + 1)) = lower(pe.entity_name || ':') AS hero_prefixed,
        greatest(coalesce(pe.confidence, 0), coalesce(pee.confidence, 0)) AS confidence
    FROM brain.patch_events pe
    LEFT JOIN brain.patch_event_enrichments pee ON pee.patch_event_id = pe.id
), base AS (
    SELECT
        id,
        patch_title_key,
        patch_title,
        patch_url,
        posted_at,
        created_at,
        entity_type,
        entity_name,
        nullif(regexp_replace(regexp_replace(ability_name_raw, '(?i)spirits\s+caling', 'spirit scaling', 'g'), '\s+', ' ', 'g'), '') AS ability_name,
        nullif(regexp_replace(regexp_replace(stat_name_raw, '(?i)spirits\s+caling', 'spirit scaling', 'g'), '\s+', ' ', 'g'), '') AS stat_name,
        old_value,
        new_value,
        change_type,
        CASE
            WHEN stat_name_raw IS NOT NULL AND old_value IS NOT NULL AND new_value IS NOT NULL THEN
                concat_ws(
                    ' ',
                    nullif(regexp_replace(regexp_replace(ability_name_raw, '(?i)spirits\s+caling', 'spirit scaling', 'g'), '\s+', ' ', 'g'), ''),
                    nullif(regexp_replace(regexp_replace(stat_name_raw, '(?i)spirits\s+caling', 'spirit scaling', 'g'), '\s+', ' ', 'g'), ''),
                    'changed from',
                    old_value,
                    'to',
                    new_value
                )
            ELSE regexp_replace(regexp_replace(cleaned_source_line, '(?i)spirits\s+caling', 'spirit scaling', 'g'), '\s+', ' ', 'g')
        END AS raw_line,
        hero_prefixed,
        confidence
    FROM raw_base
), patch_groups AS (
    SELECT
        patch_title_key,
        (array_agg(patch_title ORDER BY (posted_at IS NULL), posted_at, id))[1] AS patch_title,
        (array_agg(patch_url ORDER BY (patch_url IS NULL), (posted_at IS NULL), posted_at, id))[1] AS patch_url,
        min(posted_at)::date AS posted_at_date,
        min(created_at)::date AS created_at_date
    FROM base
    GROUP BY patch_title_key
), patch_dates AS (
    SELECT
        pg.patch_title_key,
        pg.patch_title,
        coalesce(
            pg.posted_at_date,
            (SELECT min(pc.patch_date) FROM patch_catalog pc WHERE pc.patch_url = pg.patch_url),
            (SELECT min(pc.patch_date) FROM patch_catalog pc WHERE pc.patch_title_key = pg.patch_title_key),
            to_date(substring(pg.patch_title FROM '(\d{2}-\d{2}-\d{4})'), 'MM-DD-YYYY'),
            pg.created_at_date
        ) AS patch_date,
        pg.patch_url
    FROM patch_groups pg
), deduped AS (
    SELECT DISTINCT ON (
        b.patch_title_key,
        b.entity_type,
        b.entity_name,
        coalesce(b.ability_name, ''),
        coalesce(b.stat_name, ''),
        coalesce(b.old_value, ''),
        coalesce(b.new_value, ''),
        b.raw_line
    )
        pd.patch_title,
        pd.patch_date,
        b.entity_type,
        b.entity_name,
        b.ability_name,
        b.stat_name,
        b.old_value,
        b.new_value,
        b.change_type,
        b.raw_line,
        coalesce(b.patch_url, pd.patch_url) AS patch_url,
        b.confidence
    FROM base b
    JOIN patch_dates pd ON pd.patch_title_key = b.patch_title_key
    ORDER BY
        b.patch_title_key,
        b.entity_type,
        b.entity_name,
        coalesce(b.ability_name, ''),
        coalesce(b.stat_name, ''),
        coalesce(b.old_value, ''),
        coalesce(b.new_value, ''),
        b.raw_line,
        b.confidence DESC,
        ((b.ability_name IS NOT NULL)::int + (b.stat_name IS NOT NULL)::int) DESC,
        b.hero_prefixed DESC,
        (b.posted_at IS NULL),
        b.posted_at,
        b.id
), parsed AS (
    SELECT
        d.*,
        CASE
            WHEN d.old_value ~ '^\s*"?[+-]?(?:\d+(?:\.\d+)?|\.\d+)\s*[[:alpha:]%/]*"?\s*$'
             AND d.new_value ~ '^\s*"?[+-]?(?:\d+(?:\.\d+)?|\.\d+)\s*[[:alpha:]%/]*"?\s*$'
                THEN regexp_replace(d.old_value, '[^0-9.+-]', '', 'g')::numeric
        END AS old_number,
        CASE
            WHEN d.old_value ~ '^\s*"?[+-]?(?:\d+(?:\.\d+)?|\.\d+)\s*[[:alpha:]%/]*"?\s*$'
             AND d.new_value ~ '^\s*"?[+-]?(?:\d+(?:\.\d+)?|\.\d+)\s*[[:alpha:]%/]*"?\s*$'
                THEN regexp_replace(d.new_value, '[^0-9.+-]', '', 'g')::numeric
        END AS new_number
    FROM deduped d
)
SELECT
    patch_title,
    patch_date,
    entity_type,
    entity_name,
    ability_name,
    stat_name,
    old_value,
    new_value,
    change_type,
    CASE
        WHEN old_number IS NULL OR new_number IS NULL OR old_number = new_number THEN NULL
        WHEN new_number > old_number THEN 'increase'
        ELSE 'decrease'
    END AS numeric_direction,
    raw_line,
    patch_url,
    confidence
FROM parsed
"#;

pub(crate) fn json_bytes(value: &Value) -> Result<Vec<u8>> {
    Ok(json_string(value)?.into_bytes())
}

pub(crate) fn json_string(value: &Value) -> Result<String> {
    let compact = serde_json::to_string(value)?;
    Ok(python_json_spacing_and_ascii(&compact))
}

pub(crate) fn stable_hash_bytes(content: &[u8]) -> String {
    let digest = Sha256::digest(content);
    let mut output = String::with_capacity(digest.len() * 2);
    for byte in digest {
        output.push(hex_char(byte >> 4));
        output.push(hex_char(byte & 0x0f));
    }
    output
}

pub(crate) fn stable_hash_text(content: &str) -> String {
    stable_hash_bytes(content.as_bytes())
}

fn sanitize_external_id(external_id: &str) -> String {
    external_id
        .chars()
        .map(|ch| {
            if ch.is_alphanumeric() || matches!(ch, '-' | '_' | '.') {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

fn python_json_spacing_and_ascii(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut in_string = false;
    let mut escaped = false;

    for ch in input.chars() {
        if in_string {
            if escaped {
                output.push(ch);
                escaped = false;
                continue;
            }

            match ch {
                '\\' => {
                    output.push('\\');
                    escaped = true;
                }
                '"' => {
                    output.push('"');
                    in_string = false;
                }
                _ => push_ascii_json_char(&mut output, ch),
            }
            continue;
        }

        match ch {
            '"' => {
                output.push('"');
                in_string = true;
            }
            ',' => output.push_str(", "),
            ':' => output.push_str(": "),
            _ => push_ascii_json_char(&mut output, ch),
        }
    }

    output
}

fn push_ascii_json_char(output: &mut String, ch: char) {
    if ch.is_ascii() {
        output.push(ch);
        return;
    }

    let mut units = [0u16; 2];
    for unit in ch.encode_utf16(&mut units) {
        output.push_str("\\u");
        push_u16_hex(output, *unit);
    }
}

fn push_u16_hex(output: &mut String, value: u16) {
    for shift in [12, 8, 4, 0] {
        let nibble = ((value >> shift) & 0x0f) as u8;
        output.push(hex_char(nibble));
    }
}

fn hex_char(nibble: u8) -> char {
    match nibble {
        0..=9 => (b'0' + nibble) as char,
        _ => (b'a' + (nibble - 10)) as char,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::{PgPool, PgPoolOptions};
    use sqlx::Row;

    /// Wegwerf-Postgres aus `DEADLOCK_CENTRAL_DSN`. `None` (Test-Skip), wenn die
    /// Variable nicht gesetzt ist — identisch zum bereits portierten `dbrain-enrich`.
    async fn test_pool() -> Option<PgPool> {
        let dsn = std::env::var("DEADLOCK_CENTRAL_DSN").ok()?;
        PgPoolOptions::new()
            .max_connections(2)
            .connect(&dsn)
            .await
            .ok()
    }

    /// Eindeutige Test-Quelle, die in echten Daten nicht vorkommt — erlaubt
    /// praezises, kollisionsfreies Aufraeumen (Zaehler bleiben netto unveraendert).
    const TEST_SOURCE: &str = "__dbrain_sources_store_test__";

    async fn cleanup(pool: &PgPool) {
        let _ = sqlx::query("DELETE FROM brain.entity_snapshots WHERE source=$1")
            .bind(TEST_SOURCE)
            .execute(pool)
            .await;
        let _ = sqlx::query("DELETE FROM brain.source_documents WHERE source=$1")
            .bind(TEST_SOURCE)
            .execute(pool)
            .await;
        let _ = sqlx::query("DELETE FROM brain.source_runs WHERE source=$1")
            .bind(TEST_SOURCE)
            .execute(pool)
            .await;
    }

    #[test]
    fn json_string_matches_python_default_spacing_and_ascii_for_common_values() {
        let value = serde_json::json!({"b": "Mö", "a": [1, true, null]});
        let encoded = json_string(&value).expect("json");
        // Python-Paritaet: Nicht-ASCII wird als \uXXXX (ensure_ascii) escaped.
        // Die Escape-Sequenz wird zur Laufzeit gebaut, damit der Editor sie nicht
        // in das rohe Zeichen zurueckwandelt.
        let escaped_oe = format!("{}u{:04x}", '\\', 'ö' as u32);
        let expected = format!(r#"{{"a": [1, true, null], "b": "M{escaped_oe}"}}"#);
        assert_eq!(encoded, expected);
        assert!(!encoded.contains('ö'));
    }

    #[test]
    fn write_raw_uses_sanitized_external_id_and_content_hash_prefix() {
        // `write_raw` schreibt nur Dateien (kein DB-Zugriff); ein Dummy-Pool ist
        // nicht noetig — wir testen die reine Pfadlogik ueber eine tempdir.
        let temp = tempfile::tempdir().expect("tempdir");
        let source_dir = temp.path().join("source");
        std::fs::create_dir_all(&source_dir).expect("dir");
        let digest = stable_hash_bytes(b"payload");
        let filename = format!(
            "{}.{}.{}",
            sanitize_external_id("a/b:c"),
            &digest[..16],
            "json"
        );
        assert!(filename.starts_with("a_b_c."));
        assert!(filename.ends_with(".json"));
    }

    /// Paritaets-/Round-Trip-Beweis gegen die echte Scratch-PG: schreibt Run,
    /// Dokument und Snapshot ueber den Store, liest per SQL zurueck, prueft
    /// Idempotenz und raeumt anschliessend restlos wieder auf.
    #[tokio::test]
    #[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
    async fn store_roundtrip_writes_reads_back_and_cleans_up() {
        let Some(pool) = test_pool().await else {
            return;
        };
        cleanup(&pool).await;

        let temp = tempfile::tempdir().expect("tempdir");
        let store = SourceStore::new(&pool, temp.path()).expect("store");

        let run_id = store.begin_run(TEST_SOURCE).await.expect("begin run");
        let raw = b"payload-xyz";
        let raw_path = store
            .write_raw(TEST_SOURCE, "a/b:c", raw, "json")
            .expect("write raw");
        let file_name = raw_path
            .file_name()
            .and_then(|value| value.to_str())
            .expect("filename");
        assert!(file_name.starts_with("a_b_c."));
        assert!(raw_path.exists());

        let metadata = serde_json::json!({"kind": "test"});
        let document = SourceDocumentInput {
            source: TEST_SOURCE,
            external_id: "ext-1",
            title: Some("Titel"),
            url: Some("https://example.test/x"),
            content_type: "application/json",
            raw_path: &raw_path,
            content: raw,
            metadata: &metadata,
        };
        let doc_id = store.upsert_source_document(document).await.expect("doc");
        let doc_id_again = store.upsert_source_document(document).await.expect("doc2");
        assert_eq!(doc_id, doc_id_again, "gleicher content_hash => idempotent");

        let snapshot = EntitySnapshotInput {
            source: TEST_SOURCE.to_string(),
            entity_type: "test_entity".to_string(),
            external_id: "ext-1".to_string(),
            canonical_name: Some("Name".to_string()),
            payload: serde_json::json!({"value": 1}),
        };
        let snap_id = store
            .upsert_entity_snapshot_id(&snapshot, Some(doc_id))
            .await
            .expect("snapshot");
        assert!(snap_id > 0);
        let snap_id_again = store
            .upsert_entity_snapshot_id(&snapshot, Some(doc_id))
            .await
            .expect("snapshot2");
        assert_eq!(snap_id, snap_id_again);

        store
            .finish_run(run_id, "ok", &serde_json::json!({"done": true}))
            .await
            .expect("finish run");

        let docs: i64 =
            sqlx::query_scalar("SELECT count(*)::int8 FROM brain.source_documents WHERE source=$1")
                .bind(TEST_SOURCE)
                .fetch_one(&pool)
                .await
                .expect("count docs");
        assert_eq!(docs, 1);

        cleanup(&pool).await;
    }

    #[tokio::test]
    #[ignore = "needs central Postgres via DEADLOCK_CENTRAL_DSN"]
    async fn patch_changes_view_deduplicates_holliday_powder_keg_scaling() {
        let Some(pool) = test_pool().await else {
            return;
        };

        let rows = sqlx::query(
            r#"
            SELECT patch_date::text, old_value, new_value
            FROM brain.patch_changes
            WHERE entity_name='Holliday'
              AND raw_line ILIKE '%spirit%scal%'
              AND raw_line ILIKE '%powder%'
            ORDER BY patch_date
            "#,
        )
        .fetch_all(&pool)
        .await
        .expect("query patch_changes");
        let actual = rows
            .iter()
            .map(|row| {
                (
                    row.get::<String, _>(0),
                    row.get::<String, _>(1),
                    row.get::<String, _>(2),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            actual,
            vec![
                (
                    "2025-09-04".to_string(),
                    "1.4".to_string(),
                    "1.6".to_string()
                ),
                (
                    "2026-06-12".to_string(),
                    "1.6".to_string(),
                    "1.4".to_string()
                ),
                (
                    "2026-06-30".to_string(),
                    "1.4".to_string(),
                    "1.2".to_string()
                ),
                (
                    "2026-07-09".to_string(),
                    "1.2".to_string(),
                    "1.05".to_string()
                ),
            ]
        );

        let null_dates: i64 = sqlx::query_scalar(
            "SELECT count(*)::int8 FROM brain.patch_changes WHERE patch_date IS NULL",
        )
        .fetch_one(&pool)
        .await
        .expect("count null patch dates");
        assert_eq!(null_dates, 0);
    }
}
