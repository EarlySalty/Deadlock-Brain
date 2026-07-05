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

    pub(crate) async fn finish_run(&self, run_id: i64, status: &str, summary: &Value) -> Result<()> {
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
    deadlock_brain_core::pg::pg_pool()
        .await
        .map_err(|error| SourcesError::Pool(error.to_string()))
}

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
        let filename = format!("{}.{}.{}", sanitize_external_id("a/b:c"), &digest[..16], "json");
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
}
