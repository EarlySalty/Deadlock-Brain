use std::path::Path;

use rusqlite::{Connection, OpenFlags};
use serde_json::{json, Map, Value};

use crate::{
    store::{json_bytes, run_source, EntitySnapshotInput, SourceDocumentInput, SourceStore},
    Result, SourcesError,
};

pub const SOURCE: &str = "deadlock_patchnotes_db";

#[derive(Debug, Clone)]
pub struct PullPatchnotesOptions<'a> {
    pub central_db_path: &'a Path,
}

pub fn classify_source_kind(url: Option<&str>) -> &'static str {
    let lower = url.unwrap_or_default().to_lowercase();
    if lower.contains("steamcommunity.com")
        || lower.contains("steampowered.com")
        || lower.contains("steamstore-a.akamaihd.net")
    {
        "steam"
    } else if lower.contains("forums.playdeadlock.com") {
        "forum"
    } else {
        "other"
    }
}

pub fn pull_patchnotes(
    conn: &Connection,
    raw_dir: &Path,
    options: PullPatchnotesOptions<'_>,
) -> Result<Value> {
    run_source(conn, raw_dir, "patchnotes", |store| {
        pull_patchnotes_inner(store, options.central_db_path)
    })
}

fn pull_patchnotes_inner(store: &SourceStore<'_>, central_db_path: &Path) -> Result<Value> {
    if !central_db_path.exists() {
        return Err(SourcesError::CentralDbNotFound(
            central_db_path.to_string_lossy().into_owned(),
        ));
    }

    let central = Connection::open_with_flags(central_db_path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    let mut statement = central.prepare(
        r#"
        SELECT id, title, url, posted_at, raw_content, translated_content
        FROM changelog_posts
        WHERE raw_content IS NOT NULL AND raw_content != ''
        ORDER BY id ASC
        "#,
    )?;
    let rows = statement.query_map([], |row| {
        Ok(PatchnoteRow {
            id: row.get(0)?,
            title: row.get(1)?,
            url: row.get(2)?,
            posted_at: row.get(3)?,
            raw_content: row.get(4)?,
            translated_content: row.get(5)?,
        })
    })?;

    let mut imported = 0usize;
    let mut source_kinds = Map::new();
    for row in rows {
        let row = row?;
        let external_id = row
            .url
            .as_deref()
            .filter(|value| !value.is_empty())
            .map(ToString::to_string)
            .unwrap_or_else(|| row.id.to_string());
        let source_kind = classify_source_kind(row.url.as_deref());
        increment_counter(&mut source_kinds, source_kind);
        let payload = json!({
            "id": row.id,
            "title": row.title.clone(),
            "url": row.url.clone(),
            "posted_at": row.posted_at.clone(),
            "raw_content": row.raw_content,
            "translated_content": row.translated_content.clone(),
        });
        let raw = json_bytes(&payload)?;
        let raw_external_id = row.id.to_string();
        let raw_path = store.write_raw(SOURCE, &raw_external_id, &raw, "json")?;
        let metadata = json!({
            "central_db_path": central_db_path.to_string_lossy().into_owned(),
            "changelog_post_id": row.id,
            "source_kind": source_kind,
        });
        let document_id = store.upsert_source_document(SourceDocumentInput {
            source: SOURCE,
            external_id: &external_id,
            title: row.title.as_deref(),
            url: row.url.as_deref(),
            content_type: "application/json",
            raw_path: &raw_path,
            content: &raw,
            metadata: &metadata,
        })?;
        store.upsert_entity_snapshot(
            &EntitySnapshotInput {
                source: SOURCE.to_string(),
                entity_type: "patchnote".to_string(),
                external_id,
                canonical_name: row.title,
                payload,
            },
            Some(document_id),
        )?;
        imported += 1;
    }

    Ok(json!({
        "central_db_path": central_db_path.to_string_lossy().into_owned(),
        "patchnotes": imported,
        "source_kinds": Value::Object(source_kinds),
    }))
}

#[derive(Debug)]
struct PatchnoteRow {
    id: i64,
    title: Option<String>,
    url: Option<String>,
    posted_at: Option<String>,
    raw_content: String,
    translated_content: Option<String>,
}

fn increment_counter(counters: &mut Map<String, Value>, key: &str) {
    let next = counters.get(key).and_then(Value::as_i64).unwrap_or(0) + 1;
    counters.insert(key.to_string(), json!(next));
}

#[cfg(test)]
mod tests {
    use deadlock_brain_core::schema;
    use rusqlite::params;

    use super::*;

    #[test]
    fn pull_patchnotes_reads_central_db_and_writes_documents_snapshots_and_run() {
        let temp = tempfile::tempdir().expect("tempdir");
        let central_path = temp.path().join("central.sqlite3");
        let raw_dir = temp.path().join("raw");
        create_central_db(&central_path);
        let conn = Connection::open_in_memory().expect("open sqlite");
        schema::ensure_schema(&conn).expect("schema");

        let summary = pull_patchnotes(
            &conn,
            &raw_dir,
            PullPatchnotesOptions {
                central_db_path: &central_path,
            },
        )
        .expect("pull patchnotes");

        assert_eq!(summary["patchnotes"], json!(2));
        assert_eq!(summary["source_kinds"]["steam"], json!(1));
        assert_eq!(summary["source_kinds"]["forum"], json!(1));
        let documents: i64 = conn
            .query_row("SELECT COUNT(*) FROM source_documents", [], |row| {
                row.get(0)
            })
            .expect("documents");
        let snapshots: i64 = conn
            .query_row("SELECT COUNT(*) FROM entity_snapshots", [], |row| {
                row.get(0)
            })
            .expect("snapshots");
        assert_eq!(documents, 2);
        assert_eq!(snapshots, 2);
    }

    fn create_central_db(path: &Path) {
        let conn = Connection::open(path).expect("central open");
        conn.execute(
            r#"
            CREATE TABLE changelog_posts(
              id INTEGER PRIMARY KEY,
              title TEXT,
              url TEXT,
              posted_at TEXT,
              raw_content TEXT,
              translated_content TEXT
            )
            "#,
            [],
        )
        .expect("create changelog");
        conn.execute(
            "INSERT INTO changelog_posts VALUES(?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                1,
                "Steam Post",
                "https://steamcommunity.com/games/1422450/announcements/detail/1",
                "2026-01-01",
                "Raw",
                Option::<String>::None,
            ],
        )
        .expect("insert steam");
        conn.execute(
            "INSERT INTO changelog_posts VALUES(?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                2,
                "Forum Post",
                "https://forums.playdeadlock.com/threads/2",
                "2026-01-02",
                "Raw 2",
                "DE",
            ],
        )
        .expect("insert forum");
    }
}
