use std::{path::Path, time::Duration};

use deadlock_brain_core::http::{HttpClient, HttpGetOptions};
use rusqlite::Connection;
use serde_json::{json, Map, Value};

use crate::{
    store::{json_bytes, run_source, EntitySnapshotInput, SourceDocumentInput, SourceStore},
    util::{python_or_string, value_to_python_string},
    Result, SourcesError,
};

pub const SOURCE: &str = "deadlock_assets_api";
pub const BASE_URL: &str = "https://assets.deadlock-api.com";
pub const ENDPOINTS: &[(&str, &str)] = &[
    ("items", "/v2/items"),
    ("heroes", "/v2/heroes?only_active=true"),
    ("heroes_all", "/v2/heroes"),
    ("raw_items", "/raw/items"),
    ("raw_heroes", "/raw/heroes"),
    ("ranks", "/v2/ranks"),
    ("colors", "/v1/colors"),
    ("build_tags", "/v2/build-tags"),
    ("npc_units", "/v2/npc-units"),
];

#[derive(Debug, Clone, Default)]
pub struct PullAssetsOptions {
    pub kinds: Vec<String>,
}

pub fn pull_assets(
    conn: &Connection,
    raw_dir: &Path,
    http: &HttpClient,
    options: PullAssetsOptions,
) -> Result<Value> {
    run_source(conn, raw_dir, "assets", |store| {
        pull_assets_inner(store, http, &options)
    })
}

pub(crate) fn pull_assets_inner(
    store: &SourceStore<'_>,
    http: &HttpClient,
    options: &PullAssetsOptions,
) -> Result<Value> {
    let selected = if options.kinds.is_empty() {
        vec![
            "items".to_string(),
            "heroes".to_string(),
            "raw_items".to_string(),
            "raw_heroes".to_string(),
        ]
    } else {
        options.kinds.clone()
    };

    let mut endpoints_summary = Map::new();
    let mut total_snapshots = 0usize;
    for kind in selected {
        let endpoint = endpoint_path(&kind)
            .ok_or_else(|| SourcesError::invalid_input(format!("Unbekannter Assets-Endpoint: {kind}")))?;
        let url = format!("{BASE_URL}{endpoint}");
        let payload = http.get_json::<Value>(
            &url,
            HttpGetOptions {
                cache_ttl_seconds: Some(3600),
                timeout: Duration::from_secs(30),
                ..HttpGetOptions::default()
            },
        )?;
        let raw = json_bytes(&payload)?;
        let raw_path = store.write_raw(SOURCE, &kind, &raw, "json")?;
        let title = format!("Deadlock Assets API {kind}");
        let metadata = json!({ "endpoint": endpoint });
        let document_id = store.upsert_source_document(SourceDocumentInput {
            source: SOURCE,
            external_id: &kind,
            title: Some(&title),
            url: Some(&url),
            content_type: "application/json",
            raw_path: &raw_path,
            content: &raw,
            metadata: &metadata,
        })?;

        let snapshots = snapshots_for(&kind, &payload);
        let count = store.insert_many_snapshots(&snapshots, Some(document_id))?;
        endpoints_summary.insert(
            kind,
            json!({
                "url": url,
                "items": payload_len(&payload),
                "snapshots": count,
            }),
        );
        total_snapshots += count;
    }

    Ok(json!({
        "endpoints": endpoints_summary,
        "snapshots": total_snapshots,
    }))
}

fn endpoint_path(kind: &str) -> Option<&'static str> {
    ENDPOINTS
        .iter()
        .find_map(|(candidate, path)| (*candidate == kind).then_some(*path))
}

fn payload_len(payload: &Value) -> usize {
    match payload {
        Value::Array(values) => values.len(),
        Value::Object(values) => values.len(),
        _ => 1,
    }
}

fn snapshots_for(kind: &str, payload: &Value) -> Vec<EntitySnapshotInput> {
    let Value::Array(entries) = payload else {
        return Vec::new();
    };
    let entity_type = match kind {
        "items" | "raw_items" => "item_or_ability",
        "heroes" | "raw_heroes" => "hero",
        "ranks" => "rank",
        "build_tags" => "build_tag",
        "npc_units" => "npc_unit",
        _ => kind,
    };

    let mut snapshots = Vec::new();
    for entry in entries {
        let Value::Object(object) = entry else {
            continue;
        };
        let external_id = python_or_string(object.get("id"))
            .or_else(|| python_or_string(object.get("class_name")))
            .or_else(|| python_or_string(object.get("name")))
            .unwrap_or_else(|| snapshots.len().to_string());
        let canonical_name = object
            .get("name")
            .filter(|value| crate::util::value_is_python_truthy(value))
            .or_else(|| {
                object
                    .get("class_name")
                    .filter(|value| crate::util::value_is_python_truthy(value))
            })
            .map(value_to_python_string);

        snapshots.push(EntitySnapshotInput {
            source: SOURCE.to_string(),
            entity_type: entity_type.to_string(),
            external_id,
            canonical_name,
            payload: entry.clone(),
        });
    }
    snapshots
}

#[cfg(test)]
mod tests {
    use std::fs;

    use deadlock_brain_core::{http::HttpClient, schema};

    use super::*;
    use crate::store::stable_hash_bytes;

    #[test]
    fn pull_assets_uses_cache_and_inserts_document_snapshot_and_run() {
        let temp = tempfile::tempdir().expect("tempdir");
        let db_path = temp.path().join("brain.sqlite3");
        let raw_dir = temp.path().join("raw");
        let cache_dir = temp.path().join("cache");
        let conn = Connection::open(db_path).expect("open sqlite");
        schema::ensure_schema(&conn).expect("schema");
        let http = HttpClient::new("test-agent", &cache_dir).expect("http");
        let url = format!("{BASE_URL}/v2/items");
        write_http_cache(
            &cache_dir,
            &url,
            br#"[{"id": 7, "name": "Mystic Reach"}, {"class_name": "item_cold"}]"#,
        );

        let summary = pull_assets(
            &conn,
            &raw_dir,
            &http,
            PullAssetsOptions {
                kinds: vec!["items".to_string()],
            },
        )
        .expect("pull assets");

        assert_eq!(summary["snapshots"], json!(2));
        let documents: i64 = conn
            .query_row("SELECT COUNT(*) FROM source_documents", [], |row| row.get(0))
            .expect("document count");
        let snapshots: i64 = conn
            .query_row("SELECT COUNT(*) FROM entity_snapshots", [], |row| row.get(0))
            .expect("snapshot count");
        let runs: i64 = conn
            .query_row("SELECT COUNT(*) FROM source_runs WHERE source='assets' AND status='ok'", [], |row| {
                row.get(0)
            })
            .expect("run count");

        assert_eq!(documents, 1);
        assert_eq!(snapshots, 2);
        assert_eq!(runs, 1);
    }

    fn write_http_cache(cache_dir: &Path, url: &str, content: &[u8]) {
        fs::create_dir_all(cache_dir).expect("cache dir");
        let path = cache_dir.join(format!("{}.bin", stable_hash_bytes(url.as_bytes())));
        fs::write(&path, content).expect("cache body");
        fs::write(
            path.with_extension("bin.json"),
            br#"{"url":"fixture","content_type":"application/json","fetched_at":0}"#,
        )
        .expect("cache metadata");
    }
}
