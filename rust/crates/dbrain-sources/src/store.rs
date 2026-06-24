use std::{
    fs,
    path::{Path, PathBuf},
};

use deadlock_brain_core::db;
use rusqlite::{params, Connection, OptionalExtension};
use serde_json::Value;
use sha2::{Digest, Sha256};

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
    conn: &'a Connection,
    raw_dir: PathBuf,
}

impl<'a> SourceStore<'a> {
    pub(crate) fn new(conn: &'a Connection, raw_dir: &Path) -> Result<Self> {
        fs::create_dir_all(raw_dir)?;
        Ok(Self {
            conn,
            raw_dir: raw_dir.to_path_buf(),
        })
    }

    pub(crate) fn conn(&self) -> &'a Connection {
        self.conn
    }

    pub(crate) fn begin_run(&self, source: &str) -> Result<i64> {
        let started_at = db::now_epoch_seconds()?;
        self.conn.execute(
            "INSERT INTO source_runs(source, status, started_at) VALUES(?1, ?2, ?3)",
            params![source, "running", started_at],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub(crate) fn finish_run(&self, run_id: i64, status: &str, summary: &Value) -> Result<()> {
        let finished_at = db::now_epoch_seconds()?;
        self.conn.execute(
            r#"
            UPDATE source_runs
            SET status=?1, finished_at=?2, summary_json=?3
            WHERE id=?4
            "#,
            params![status, finished_at, json_string(summary)?, run_id],
        )?;
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

    pub(crate) fn upsert_source_document(
        &self,
        document: SourceDocumentInput<'_>,
    ) -> Result<i64> {
        let content_hash = stable_hash_bytes(document.content);
        let fetched_at = db::now_epoch_seconds()?;
        self.conn.execute(
            r#"
            INSERT OR IGNORE INTO source_documents(
              source, external_id, title, url, content_type, raw_path, content_hash, fetched_at, metadata_json
            )
            VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            "#,
            params![
                document.source,
                document.external_id,
                document.title,
                document.url,
                document.content_type,
                document.raw_path.to_string_lossy(),
                content_hash,
                fetched_at,
                json_string(document.metadata)?,
            ],
        )?;

        let row_id = self
            .conn
            .query_row(
                r#"
                SELECT id FROM source_documents
                WHERE source=?1 AND external_id=?2 AND content_hash=?3
                ORDER BY id DESC LIMIT 1
                "#,
                params![document.source, document.external_id, content_hash],
                |row| row.get::<_, i64>(0),
            )
            .optional()?;

        row_id.ok_or_else(|| {
            SourcesError::invariant("source_documents row missing after INSERT OR IGNORE")
        })
    }

    pub(crate) fn upsert_entity_snapshot(
        &self,
        snapshot: &EntitySnapshotInput,
        source_document_id: Option<i64>,
    ) -> Result<()> {
        let payload_json = json_string(&snapshot.payload)?;
        let payload_hash = stable_hash_text(&payload_json);
        let fetched_at = db::now_epoch_seconds()?;
        self.conn.execute(
            r#"
            INSERT OR IGNORE INTO entity_snapshots(
              source, entity_type, external_id, canonical_name, payload_hash,
              payload_json, fetched_at, source_document_id
            )
            VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            "#,
            params![
                snapshot.source,
                snapshot.entity_type,
                snapshot.external_id,
                snapshot.canonical_name,
                payload_hash,
                payload_json,
                fetched_at,
                source_document_id,
            ],
        )?;
        Ok(())
    }

    pub(crate) fn insert_many_snapshots(
        &self,
        snapshots: &[EntitySnapshotInput],
        source_document_id: Option<i64>,
    ) -> Result<usize> {
        let mut count = 0usize;
        for snapshot in snapshots {
            self.upsert_entity_snapshot(snapshot, source_document_id)?;
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

pub(crate) fn run_source<F>(
    conn: &Connection,
    raw_dir: &Path,
    run_source_name: &str,
    f: F,
) -> Result<Value>
where
    F: FnOnce(&SourceStore<'_>) -> Result<Value>,
{
    let store = SourceStore::new(conn, raw_dir)?;
    let run_id = store.begin_run(run_source_name)?;
    match f(&store) {
        Ok(summary) => {
            store.finish_run(run_id, "ok", &summary)?;
            Ok(summary)
        }
        Err(error) => {
            let summary = serde_json::json!({ "error": error.to_string() });
            store.finish_run(run_id, "error", &summary)?;
            Err(error)
        }
    }
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

    #[test]
    fn json_string_matches_python_default_spacing_and_ascii_for_common_values() {
        let value = serde_json::json!({"b": "Mö", "a": [1, true, null]});
        let encoded = json_string(&value).expect("json");
        assert_eq!(encoded, r#"{"a": [1, true, null], "b": "M\u00f6"}"#);
    }

    #[test]
    fn write_raw_uses_sanitized_external_id_and_content_hash_prefix() {
        let temp = tempfile::tempdir().expect("tempdir");
        let conn = Connection::open_in_memory().expect("open sqlite");
        deadlock_brain_core::schema::ensure_schema(&conn).expect("schema");
        let store = SourceStore::new(&conn, temp.path()).expect("store");

        let path = store
            .write_raw("source", "a/b:c", b"payload", "json")
            .expect("write raw");

        let filename = path
            .file_name()
            .and_then(|value| value.to_str())
            .expect("filename");
        assert!(filename.starts_with("a_b_c."));
        assert!(filename.ends_with(".json"));
        assert!(path.exists());
    }
}
