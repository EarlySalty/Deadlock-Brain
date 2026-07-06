use std::{path::Path, time::Duration};

use deadlock_brain_core::http::{HttpClient, HttpGetOptions};
use serde_json::{json, Value};

use crate::{
    store::{complete_run, json_bytes, open_pool, EntitySnapshotInput, SourceDocumentInput, SourceStore},
    util::{form_urlencode, python_or_string},
    Result, SourcesError,
};

pub const SOURCE: &str = "deadlock_api";
pub const BASE_URL: &str = "https://api.deadlock-api.com";

#[derive(Debug, Clone)]
pub struct PullMatchMetadataOptions {
    pub match_ids: Vec<String>,
    pub account_ids: Vec<String>,
    pub hero_ids: Vec<String>,
    pub include_player_items: bool,
    pub include_player_info: bool,
    pub include_player_stats: bool,
    pub include_player_death_details: bool,
    pub include_objectives: bool,
    pub cache_ttl_seconds: u64,
}

impl Default for PullMatchMetadataOptions {
    fn default() -> Self {
        Self {
            match_ids: Vec::new(),
            account_ids: Vec::new(),
            hero_ids: Vec::new(),
            include_player_items: true,
            include_player_info: true,
            include_player_stats: true,
            include_player_death_details: true,
            include_objectives: true,
            cache_ttl_seconds: 21_600,
        }
    }
}

pub async fn pull_match_metadata(
    raw_dir: &Path,
    http: &HttpClient,
    options: PullMatchMetadataOptions,
) -> Result<Value> {
    let pool = open_pool().await?;
    let store = SourceStore::new(&pool, raw_dir)?;
    let run_id = store.begin_run("deadlock-api").await?;
    let outcome = pull_match_metadata_inner(&store, http, &options).await;
    complete_run(&store, run_id, outcome).await
}

async fn pull_match_metadata_inner(
    store: &SourceStore<'_>,
    http: &HttpClient,
    options: &PullMatchMetadataOptions,
) -> Result<Value> {
    let safe_match_ids = safe_ids(&options.match_ids);
    if safe_match_ids.is_empty() {
        return Err(SourcesError::invalid_input("match_ids fehlt."));
    }

    let mut params = vec![
        ("match_ids", safe_match_ids.join(",")),
        ("include_info", "true".to_string()),
        (
            "include_player_items",
            options.include_player_items.to_string(),
        ),
        ("include_player_info", options.include_player_info.to_string()),
        ("include_player_stats", options.include_player_stats.to_string()),
        (
            "include_player_death_details",
            options.include_player_death_details.to_string(),
        ),
        ("include_objectives", options.include_objectives.to_string()),
        ("limit", safe_match_ids.len().to_string()),
    ];
    let safe_account_ids = safe_ids(&options.account_ids);
    if !safe_account_ids.is_empty() {
        params.push(("account_ids", safe_account_ids.join(",")));
    }
    let safe_hero_ids = safe_ids(&options.hero_ids);
    if !safe_hero_ids.is_empty() {
        params.push(("hero_ids", safe_hero_ids.join(",")));
    }

    let url = format!("{BASE_URL}/v1/matches/metadata?{}", form_urlencode(&params));
    let payload = get_deadlock_api_json(http, &url, options.cache_ttl_seconds)?;
    let rows = payload.as_array().cloned().unwrap_or_default();
    let external_id = format!("match-metadata:{}", safe_match_ids.join(","));
    let raw = json_bytes(&payload)?;
    let raw_path = store.write_raw(SOURCE, &external_id, &raw, "json")?;
    let title = format!("Deadlock API match metadata {}", safe_match_ids.join(","));
    let metadata = json!({
        "match_ids": safe_match_ids,
        "account_ids": safe_account_ids,
        "hero_ids": safe_hero_ids,
        "cache_ttl_seconds": options.cache_ttl_seconds,
        "include_player_items": options.include_player_items,
        "include_player_stats": options.include_player_stats,
        "include_player_death_details": options.include_player_death_details,
        "include_objectives": options.include_objectives,
    });
    let document_id = store
        .upsert_source_document(SourceDocumentInput {
            source: SOURCE,
            external_id: &external_id,
            title: Some(&title),
            url: Some(&url),
            content_type: "application/json",
            raw_path: &raw_path,
            content: &raw,
            metadata: &metadata,
        })
        .await?;

    let mut snapshots = Vec::new();
    for row in &rows {
        let Value::Object(object) = row else {
            continue;
        };
        let match_id = python_or_string(object.get("match_id"))
            .or_else(|| python_or_string(object.get("matchId")))
            .unwrap_or_default()
            .trim()
            .to_string();
        if match_id.is_empty() {
            continue;
        }
        snapshots.push(EntitySnapshotInput {
            source: SOURCE.to_string(),
            entity_type: "deadlock_api_match_metadata".to_string(),
            external_id: match_id.clone(),
            canonical_name: Some(match_id.clone()),
            payload: with_source_metadata(row, json!({ "source_url": url, "match_id": match_id })),
        });
    }
    let count = store
        .insert_many_snapshots(&snapshots, Some(document_id))
        .await?;
    Ok(json!({
        "url": url,
        "matches": rows.len(),
        "snapshots": count,
    }))
}

fn get_deadlock_api_json(http: &HttpClient, url: &str, cache_ttl_seconds: u64) -> Result<Value> {
    let result = http.get(
        url,
        HttpGetOptions {
            cache_ttl_seconds: Some(cache_ttl_seconds),
            timeout: Duration::from_secs(60),
            headers: vec![
                ("Accept".to_string(), "application/json".to_string()),
                ("Referer".to_string(), "https://deadlock-api.com/".to_string()),
            ],
            ..HttpGetOptions::default()
        },
    )?;
    Ok(serde_json::from_str(&result.text())?)
}

fn safe_ids(values: &[String]) -> Vec<String> {
    let mut result = Vec::new();
    for value in values {
        let raw = value.trim();
        if !raw.is_empty() && !result.iter().any(|existing| existing == raw) {
            result.push(raw.to_string());
        }
    }
    result
}

fn with_source_metadata(payload: &Value, metadata: Value) -> Value {
    let mut copied = payload.as_object().cloned().unwrap_or_else(|| {
        let mut object = serde_json::Map::new();
        object.insert("value".to_string(), payload.clone());
        object
    });
    copied.insert("_deadlock_brain".to_string(), metadata);
    Value::Object(copied)
}
