use std::{
    fs,
    path::{Path, PathBuf},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use deadlock_brain_core::http::{HttpClient, HttpGetOptions};
use rusqlite::Connection;
use serde_json::{json, Value};

use crate::{
    store::{json_bytes, run_source, EntitySnapshotInput, SourceDocumentInput, SourceStore},
    util::form_urlencode,
    Result, SourcesError,
};

pub const SOURCE: &str = "deadlock_wiki";
pub const DEFAULT_API_URL: &str = "https://deadlock.wiki/api.php";

#[derive(Debug, Clone)]
pub struct PullWikiPageOptions {
    pub title: String,
    pub enabled: bool,
    pub cache_ttl_seconds: u64,
    pub min_delay_seconds: f64,
    pub api_url: String,
}

impl Default for PullWikiPageOptions {
    fn default() -> Self {
        Self {
            title: String::new(),
            enabled: false,
            cache_ttl_seconds: 604_800,
            min_delay_seconds: 5.0,
            api_url: DEFAULT_API_URL.to_string(),
        }
    }
}

pub fn pull_wiki_page(
    conn: &Connection,
    raw_dir: &Path,
    http: &HttpClient,
    options: PullWikiPageOptions,
) -> Result<Value> {
    run_source(conn, raw_dir, "wiki", |store| {
        pull_wiki_page_inner(store, http, &options)
    })
}

fn pull_wiki_page_inner(
    store: &SourceStore<'_>,
    http: &HttpClient,
    options: &PullWikiPageOptions,
) -> Result<Value> {
    if !options.enabled {
        return Err(SourcesError::invalid_input(
            "Wiki-Netzwerkzugriff ist deaktiviert. Setze DEADLOCK_BRAIN_WIKI_ENABLED=1 \
oder nutze die CLI-Option --allow-wiki-network.",
        ));
    }

    let limiter = WikiRateLimiter {
        state_path: http.cache_dir().join("wiki_last_request.txt"),
        min_delay_seconds: options.min_delay_seconds,
    };
    limiter.wait()?;
    let params = [
        ("action", "query".to_string()),
        ("format", "json".to_string()),
        ("prop", "extracts|revisions".to_string()),
        ("explaintext", "1".to_string()),
        ("rvprop", "ids|timestamp|content".to_string()),
        ("rvslots", "main".to_string()),
        ("titles", options.title.clone()),
        ("redirects", "1".to_string()),
    ];
    let url = format!("{}?{}", options.api_url, form_urlencode(&params));
    let result = http.get(
        &url,
        HttpGetOptions {
            cache_ttl_seconds: Some(options.cache_ttl_seconds),
            timeout: Duration::from_secs(30),
            ..HttpGetOptions::default()
        },
    )?;
    let payload: Value = serde_json::from_str(&result.text())?;
    let raw = json_bytes(&payload)?;
    let raw_path = store.write_raw(SOURCE, &options.title, &raw, "json")?;
    let metadata = json!({
        "from_cache": result.from_cache,
        "cache_ttl_seconds": options.cache_ttl_seconds,
    });
    let document_id = store.upsert_source_document(SourceDocumentInput {
        source: SOURCE,
        external_id: &options.title,
        title: Some(&options.title),
        url: Some(&url),
        content_type: "application/json",
        raw_path: &raw_path,
        content: &raw,
        metadata: &metadata,
    })?;
    store.upsert_entity_snapshot(
        &EntitySnapshotInput {
            source: SOURCE.to_string(),
            entity_type: "wiki_page".to_string(),
            external_id: options.title.clone(),
            canonical_name: Some(options.title.clone()),
            payload: payload.clone(),
        },
        Some(document_id),
    )?;
    let pages = payload
        .get("query")
        .and_then(|query| query.get("pages"))
        .and_then(Value::as_object)
        .map(|pages| pages.len())
        .unwrap_or(0);
    Ok(json!({
        "title": options.title.clone(),
        "from_cache": result.from_cache,
        "pages": pages,
    }))
}

#[derive(Debug, Clone)]
struct WikiRateLimiter {
    state_path: PathBuf,
    min_delay_seconds: f64,
}

impl WikiRateLimiter {
    fn wait(&self) -> Result<()> {
        if let Some(parent) = self.state_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let last = fs::read_to_string(&self.state_path)
            .ok()
            .and_then(|value| value.trim().parse::<f64>().ok())
            .unwrap_or(0.0);
        let now = now_seconds_f64()?;
        let wait_for = self.min_delay_seconds - (now - last);
        if wait_for.is_finite() && wait_for > 0.0 {
            thread::sleep(Duration::from_secs_f64(wait_for));
        }
        fs::write(&self.state_path, now_seconds_f64()?.to_string())?;
        Ok(())
    }
}

fn now_seconds_f64() -> Result<f64> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64())
}
