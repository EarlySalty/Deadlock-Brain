use std::{path::Path, time::Duration};

use deadlock_brain_core::http::{HttpClient, HttpGetOptions};
use serde_json::{json, Map, Value};

use crate::{
    store::{
        complete_run, json_bytes, open_pool, EntitySnapshotInput, SourceDocumentInput, SourceStore,
    },
    util::{python_or_string, value_to_python_string},
    Result, SourcesError,
};

pub const SOURCE: &str = "deadlock_assets_api";
pub const BASE_URL: &str = "https://api.deadlock-api.com";
// Contract: https://api.deadlock-api.com/openapi.json (2026-09-21).
// The former raw routes no longer exist. Their CLI names remain explicit
// compatibility aliases, not a claim that the normalized response is raw KV.
pub const ENDPOINTS: &[(&str, &str)] = &[
    ("items", "/v1/assets/items"),
    ("heroes", "/v1/assets/heroes?only_active=true"),
    ("heroes_all", "/v1/assets/heroes"),
    ("raw_items", "/v1/assets/items"),
    ("raw_heroes", "/v1/assets/heroes"),
    ("ranks", "/v1/assets/ranks"),
    ("colors", "/v1/assets/colors"),
    ("build_tags", "/v1/assets/build-tags"),
    ("npc_units", "/v1/assets/npc-units"),
];

#[derive(Debug, Clone, Default)]
pub struct PullAssetsOptions {
    pub kinds: Vec<String>,
    /// Pin one upstream game revision across every endpoint in this run.
    pub client_version: Option<u32>,
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
    let selected = selected_kinds(&options.kinds)?;
    let client_version = match options.client_version {
        Some(version) if version > 0 => version,
        Some(_) => return Err(SourcesError::invalid_input("Ungültige Spielversion: 0")),
        None => {
            let versions = http.get_json::<Value>(
                &format!("{BASE_URL}/v1/assets/client-versions"),
                HttpGetOptions {
                    cache_ttl_seconds: Some(0),
                    timeout: Duration::from_secs(30),
                    ..HttpGetOptions::default()
                },
            )?;
            latest_client_version(&versions)?
        }
    };

    // Fetch and validate the complete batch before changing stored assets.
    // A failed endpoint cannot mark half of a new game revision as current.
    let mut fetched = Vec::new();
    for kind in selected {
        let endpoint = endpoint_path(&kind).ok_or_else(|| {
            SourcesError::invalid_input(format!("Unbekannter Assets-Endpoint: {kind}"))
        })?;
        let url = versioned_url(endpoint, client_version);
        let payload = http.get_json::<Value>(
            &url,
            HttpGetOptions {
                cache_ttl_seconds: Some(3600),
                timeout: Duration::from_secs(30),
                ..HttpGetOptions::default()
            },
        )?;
        validate_payload(&kind, &payload)?;
        fetched.push((kind, endpoint, url, payload));
    }

    let mut endpoints_summary = Map::new();
    let mut total_snapshots = 0usize;
    for (kind, endpoint, url, payload) in fetched {
        let raw = json_bytes(&payload)?;
        let raw_path = store.write_raw(SOURCE, &kind, &raw, "json")?;
        let title = format!("Deadlock Assets API {kind}");
        let metadata = json!({
            "endpoint": endpoint,
            "client_version": client_version,
            "representation": "normalized_assets_v1",
            "requested_kinds": options.kinds,
        });
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

        let mut snapshots = snapshots_for(&kind, &payload);
        for snapshot in &mut snapshots {
            if let Some(object) = snapshot.payload.as_object_mut() {
                object.insert("_source_client_version".into(), json!(client_version));
            }
        }
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
        "client_version": client_version,
        "representation": "normalized_assets_v1",
        "requested_kinds": options.kinds,
    }))
}

fn selected_kinds(requested: &[String]) -> Result<Vec<String>> {
    let defaults = ["items".to_string(), "heroes".to_string()];
    let requested = if requested.is_empty() {
        &defaults[..]
    } else {
        requested
    };
    let mut selected = Vec::new();
    for kind in requested {
        let canonical = match kind.as_str() {
            "raw_items" => "items",
            "raw_heroes" => "heroes_all",
            other => other,
        };
        if endpoint_path(canonical).is_none() {
            return Err(SourcesError::invalid_input(format!(
                "Unbekannter Assets-Endpoint: {kind}"
            )));
        }
        if !selected.iter().any(|value| value == canonical) {
            selected.push(canonical.to_string());
        }
    }
    Ok(selected)
}

fn latest_client_version(payload: &Value) -> Result<u32> {
    let versions = payload.as_array().ok_or_else(|| {
        SourcesError::invalid_input("Die Assets-API liefert keine Liste von Spielversionen")
    })?;
    let versions = versions
        .iter()
        .map(|value| {
            value
                .as_u64()
                .and_then(|value| u32::try_from(value).ok())
                .filter(|value| *value > 0)
                .ok_or_else(|| {
                    SourcesError::invalid_input(
                        "Die Assets-API liefert eine ungültige Spielversion",
                    )
                })
        })
        .collect::<Result<Vec<_>>>()?;
    versions.into_iter().max().ok_or_else(|| {
        SourcesError::invalid_input("Die Assets-API liefert eine leere Liste von Spielversionen")
    })
}

fn versioned_url(endpoint: &str, client_version: u32) -> String {
    let separator = if endpoint.contains('?') { '&' } else { '?' };
    format!("{BASE_URL}{endpoint}{separator}client_version={client_version}")
}

fn validate_payload(kind: &str, payload: &Value) -> Result<()> {
    if kind == "colors" && payload.as_object().is_some_and(|object| !object.is_empty()) {
        return Ok(());
    }
    let entries = payload
        .as_array()
        .filter(|entries| !entries.is_empty())
        .ok_or_else(|| {
            SourcesError::invalid_input(format!(
                "Assets-Antwort für {kind}: nichtleere Liste erwartet"
            ))
        })?;
    if entries.iter().any(|entry| !entry.is_object()) {
        return Err(SourcesError::invalid_input(format!(
            "Assets-Antwort für {kind}: ungültiger Eintrag"
        )));
    }
    if matches!(kind, "items" | "heroes" | "heroes_all") {
        let mut ids = std::collections::BTreeSet::new();
        for entry in entries {
            let id = entry
                .get("id")
                .and_then(Value::as_u64)
                .filter(|id| *id > 0)
                .ok_or_else(|| {
                    SourcesError::invalid_input(format!(
                        "Assets-Antwort für {kind}: stabile ID fehlt"
                    ))
                })?;
            if !ids.insert(id)
                || entry
                    .get("class_name")
                    .and_then(Value::as_str)
                    .is_none_or(str::is_empty)
            {
                return Err(SourcesError::invalid_input(format!(
                    "Assets-Antwort für {kind}: doppelte ID oder fehlende Klasse"
                )));
            }
        }
    }
    Ok(())
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
        "heroes" | "heroes_all" | "raw_heroes" => "hero",
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

    #[test]
    fn endpoints_follow_the_current_official_assets_contract() {
        assert_eq!(BASE_URL, "https://api.deadlock-api.com");
        assert_eq!(endpoint_path("items"), Some("/v1/assets/items"));
        assert_eq!(
            endpoint_path("heroes"),
            Some("/v1/assets/heroes?only_active=true")
        );
        assert_eq!(endpoint_path("raw_items"), endpoint_path("items"));
        assert_eq!(endpoint_path("raw_heroes"), endpoint_path("heroes_all"));
    }

    #[test]
    fn complete_hero_endpoint_keeps_the_hero_entity_type() {
        let payload = serde_json::json!([{"id": 1, "name": "Synthetic hero"}]);
        assert_eq!(snapshots_for("heroes_all", &payload)[0].entity_type, "hero");
    }

    #[test]
    fn version_selection_is_strict_and_independent_of_list_order() {
        assert_eq!(latest_client_version(&json!([12, 10, 11])).unwrap(), 12);
        for payload in [
            json!([]),
            json!({}),
            json!([0]),
            json!([-1]),
            json!([1, "2"]),
            json!([1.5]),
            json!([u64::MAX]),
        ] {
            assert!(latest_client_version(&payload).is_err());
        }
    }

    #[test]
    fn version_pin_keeps_endpoint_filters() {
        assert_eq!(
            versioned_url(endpoint_path("items").unwrap(), 12),
            "https://api.deadlock-api.com/v1/assets/items?client_version=12"
        );
        assert_eq!(
            versioned_url(endpoint_path("heroes").unwrap(), 12),
            "https://api.deadlock-api.com/v1/assets/heroes?only_active=true&client_version=12"
        );
    }

    #[test]
    fn raw_compatibility_names_are_explicit_and_do_not_duplicate_imports() {
        assert_eq!(selected_kinds(&[]).unwrap(), vec!["items", "heroes"]);
        let requested = ["items", "raw_items", "raw_heroes", "heroes_all"].map(str::to_string);
        assert_eq!(
            selected_kinds(&requested).unwrap(),
            vec!["items", "heroes_all"]
        );
        assert!(selected_kinds(&["missing".into()]).is_err());
    }

    #[test]
    fn malformed_asset_batches_fail_before_snapshot_writes() {
        let valid = json!([{"id": 1, "class_name": "synthetic_one"}]);
        assert!(validate_payload("items", &valid).is_ok());
        for invalid in [
            json!([]),
            json!({"error":"upstream failed"}),
            json!([null]),
            json!([{"name":"missing identity"}]),
            json!([{"id":1,"class_name":"a"},{"id":1,"class_name":"b"}]),
        ] {
            assert!(validate_payload("items", &invalid).is_err());
        }
        assert!(validate_payload("colors", &json!({"weapon":"#ff0000"})).is_ok());
    }

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
