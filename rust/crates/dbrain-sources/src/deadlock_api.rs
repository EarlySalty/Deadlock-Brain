use std::{path::Path, time::Duration};

use deadlock_brain_core::http::{HttpClient, HttpGetOptions};
use serde_json::{json, Value};

use crate::{
    store::{
        complete_run, json_bytes, open_pool, EntitySnapshotInput, SourceDocumentInput, SourceStore,
    },
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

#[derive(Debug, Clone)]
pub struct PullPlayerMatchHistoryOptions {
    pub account_id: String,
    pub hero_id: Option<u32>,
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

impl Default for PullPlayerMatchHistoryOptions {
    fn default() -> Self {
        Self {
            account_id: String::new(),
            hero_id: Some(18),
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

pub async fn pull_player_match_history(
    raw_dir: &Path,
    http: &HttpClient,
    options: PullPlayerMatchHistoryOptions,
) -> Result<Value> {
    let pool = open_pool().await?;
    let store = SourceStore::new(&pool, raw_dir)?;
    let run_id = store.begin_run("deadlock-api").await?;
    let outcome = pull_player_match_history_inner(&store, http, &options).await;
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
        (
            "include_player_info",
            options.include_player_info.to_string(),
        ),
        (
            "include_player_stats",
            options.include_player_stats.to_string(),
        ),
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

async fn pull_player_match_history_inner(
    store: &SourceStore<'_>,
    http: &HttpClient,
    options: &PullPlayerMatchHistoryOptions,
) -> Result<Value> {
    let account_id = safe_numeric_id(&options.account_id, "account_id")?;
    let url = format!("{BASE_URL}/v1/players/{account_id}/match-history");
    let payload = get_deadlock_api_json(http, &url, options.cache_ttl_seconds)?;
    let rows = match_history_rows(&payload)?;
    let filtered = filter_match_history(rows, options.hero_id);
    let external_id = format!("player-match-history:{account_id}");
    let raw = json_bytes(&payload)?;
    let raw_path = store.write_raw(SOURCE, &external_id, &raw, "json")?;
    let title = format!("Deadlock API player match history {account_id}");
    let metadata = json!({
        "account_id": account_id,
        "hero_id": options.hero_id,
        "cache_ttl_seconds": options.cache_ttl_seconds,
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
    for row in &filtered {
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
        let hero_id = history_hero_id(row);
        snapshots.push(EntitySnapshotInput {
            source: SOURCE.to_string(),
            entity_type: "deadlock_api_player_match".to_string(),
            external_id: format!("{account_id}:{match_id}"),
            canonical_name: Some(match_id.clone()),
            payload: with_source_metadata(
                row,
                json!({
                    "account_id": account_id,
                    "match_id": match_id,
                    "hero_id": hero_id,
                    "source_url": url,
                }),
            ),
        });
    }
    let stored_rows = store
        .insert_many_snapshots(&snapshots, Some(document_id))
        .await?;
    Ok(json!({
        "url": url,
        "account_id": account_id,
        "hero_id": options.hero_id,
        "api_rows": rows.len(),
        "stored_rows": stored_rows,
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
                (
                    "Referer".to_string(),
                    "https://deadlock-api.com/".to_string(),
                ),
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

fn safe_numeric_id(value: &str, name: &str) -> Result<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() || !trimmed.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(SourcesError::invalid_input(format!(
            "{name} muss eine numerische ID sein."
        )));
    }
    Ok(trimmed.to_string())
}

fn match_history_rows(payload: &Value) -> Result<&[Value]> {
    payload.as_array().map(Vec::as_slice).ok_or_else(|| {
        SourcesError::invalid_input("Deadlock API match-history ist kein JSON-Array.")
    })
}

fn filter_match_history(rows: &[Value], hero_id: Option<u32>) -> Vec<Value> {
    rows.iter()
        .filter(|row| hero_id.is_none_or(|expected| history_hero_id(row) == Some(expected)))
        .cloned()
        .collect()
}

fn history_hero_id(row: &Value) -> Option<u32> {
    let object = row.as_object()?;
    for key in ["hero_id", "heroId", "player_hero_id", "playerHeroId"] {
        let Some(value) = object.get(key) else {
            continue;
        };
        if let Some(id) = value.as_u64().and_then(|id| u32::try_from(id).ok()) {
            return Some(id);
        }
        if let Some(id) = value
            .as_str()
            .and_then(|text| text.trim().parse::<u32>().ok())
        {
            return Some(id);
        }
    }
    None
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_every_requested_hero_match() {
        let payload = json!([
            {
                "match_id": 111,
                "hero_id": 18,
                "match_result": "won",
                "start_time": 1700000000,
                "team_abandoned": false
            },
            {
                "match_id": 222,
                "hero_id": 7,
                "match_result": "lost",
                "start_time": 1700000300,
                "team_abandoned": false
            },
            {
                "match_id": 333,
                "hero_id": 18,
                "match_result": "lost",
                "start_time": 1700000600,
                "team_abandoned": true
            }
        ]);

        let filtered = filter_match_history(match_history_rows(&payload).unwrap(), Some(18));

        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0]["match_id"], json!(111));
        assert_eq!(filtered[0]["match_result"], json!("won"));
        assert_eq!(filtered[0]["start_time"], json!(1700000000));
        assert_eq!(filtered[0]["team_abandoned"], json!(false));
        assert_eq!(filtered[1]["match_id"], json!(333));
        assert_eq!(filtered[1]["match_result"], json!("lost"));
        assert_eq!(filtered[1]["start_time"], json!(1700000600));
        assert_eq!(filtered[1]["team_abandoned"], json!(true));
    }

    #[test]
    fn rejects_non_array_match_history() {
        let error = match_history_rows(&json!({"error": "schema changed"})).unwrap_err();

        assert!(error
            .to_string()
            .contains("match-history ist kein JSON-Array"));
    }
}
