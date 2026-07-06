use std::{path::Path, time::Duration};

use deadlock_brain_core::http::{HttpClient, HttpGetOptions};
use serde_json::{json, Map, Value};

use crate::{
    store::{complete_run, json_bytes, open_pool, EntitySnapshotInput, SourceDocumentInput, SourceStore},
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

pub async fn pull_assets(
    raw_dir: &Path,
    http: &HttpClient,
    options: PullAssetsOptions,
) -> Result<Value> {
    let pool = open_pool().await?;
    let store = SourceStore::new(&pool, raw_dir)?;
    let run_id = store.begin_run("assets").await?;
    let outcome = pull_assets_inner(&store, http, &options).await;
    complete_run(&store, run_id, outcome).await
}

pub(crate) async fn pull_assets_inner(
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
        let document_id = store
            .upsert_source_document(SourceDocumentInput {
                source: SOURCE,
                external_id: &kind,
                title: Some(&title),
                url: Some(&url),
                content_type: "application/json",
                raw_path: &raw_path,
                content: &raw,
                metadata: &metadata,
            })
            .await?;

        let snapshots = snapshots_for(&kind, &payload);
        let count = store
            .insert_many_snapshots(&snapshots, Some(document_id))
            .await?;
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
    use super::*;

    /// Pure-Logic-Port: die Snapshot-Ableitung aus einem Assets-Payload haengt
    /// nicht an der DB. Der DB-Schreibpfad (`pull_assets` -> Store) ist in
    /// `store.rs` als PG-Integration abgedeckt.
    #[test]
    fn snapshots_for_items_derives_external_ids_and_names() {
        let payload = serde_json::json!([
            {"id": 7, "name": "Mystic Reach"},
            {"class_name": "item_cold"},
            {"nothing": true }
        ]);
        let snapshots = snapshots_for("items", &payload);
        assert_eq!(snapshots.len(), 3);
        assert!(snapshots
            .iter()
            .all(|snapshot| snapshot.entity_type == "item_or_ability"));
        assert_eq!(snapshots[0].external_id, "7");
        assert_eq!(snapshots[0].canonical_name.as_deref(), Some("Mystic Reach"));
        assert_eq!(snapshots[1].external_id, "item_cold");
        // Ohne id/class_name/name faellt der External-Key auf den Index zurueck.
        assert_eq!(snapshots[2].external_id, "2");
    }

    #[test]
    fn snapshots_for_maps_kind_to_entity_type() {
        let payload = serde_json::json!([{"id": 1, "name": "Abrams"}]);
        assert_eq!(snapshots_for("heroes", &payload)[0].entity_type, "hero");
        assert_eq!(snapshots_for("ranks", &payload)[0].entity_type, "rank");
        assert!(snapshots_for("items", &serde_json::json!({})).is_empty());
    }
}
