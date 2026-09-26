//! Opt-in I/O around the shared, offline-tested Wiki parser. Reuses HttpClient,
//! rate-limit state and SourceStore. No second facts store or publisher.
use crate::{
    store::{complete_run, SourceDocumentInput, SourceStore},
    util::form_urlencode,
    wiki::{WikiRateLimiter, DEFAULT_API_URL},
    Result, SourcesError,
};
use dbrain_s12_wiki_probe::{
    capture::{self, CaptureOptions},
    knowledge::WikiIr,
    model::Capture,
};
use deadlock_brain_core::http::{HttpClient, SourceHttpOptions};
use serde_json::{json, Value};
use sqlx::PgPool;
use std::{fs::OpenOptions, path::Path, time::Duration};

#[derive(Debug, Clone)]
pub struct WikiCaptureAccess {
    pub enabled: bool,
    pub min_delay_seconds: f64,
    pub max_response_bytes: usize,
}
impl Default for WikiCaptureAccess {
    fn default() -> Self {
        Self {
            enabled: false,
            min_delay_seconds: 5.0,
            max_response_bytes: 4 * 1024 * 1024,
        }
    }
}

/// Blocking pilot capture. Async callers must use the existing blocking-worker
/// boundary. The caller supplies explicit pilot IDs and reviewed source policy.
/// The API URL is fixed: no redirects, auth guessing, proxies or fallback hosts.
pub fn capture_with_http(
    http: &HttpClient,
    access: &WikiCaptureAccess,
    options: &CaptureOptions,
) -> Result<Capture> {
    if !access.enabled {
        return Err(SourcesError::invalid_input("Wiki network disabled"));
    }
    if !access.min_delay_seconds.is_finite()
        || access.min_delay_seconds < 5.0
        || access.max_response_bytes == 0
        || access.max_response_bytes > 8 * 1024 * 1024
    {
        return Err(SourcesError::invalid_input("invalid Wiki request limits"));
    }
    let lock = OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(http.cache_dir().join("wiki-corpus.lock"))?;
    lock.try_lock()
        .map_err(|_| SourcesError::invalid_input("Wiki capture already active"))?;
    let limiter = WikiRateLimiter {
        state_path: http.cache_dir().join("wiki_last_request.txt"),
        min_delay_seconds: access.min_delay_seconds,
    };
    options.validate().map_err(SourcesError::invalid_input)?;
    if options.request_interval_ms < 5_000 {
        return Err(SourcesError::invalid_input(
            "live Wiki interval must be at least 5000 ms",
        ));
    }
    capture::collect_bytes(options, |params, limit| {
        let started = std::time::Instant::now();
        limiter.wait_bounded(limit.timeout).map_err(|_| {
            "Wiki rate limiter unavailable or capture deadline exhausted".to_string()
        })?;
        let remaining = limit
            .timeout
            .saturating_sub(started.elapsed())
            .min(Duration::from_secs(300));
        if remaining.is_zero() {
            return Err("capture deadline exhausted".into());
        }
        let pairs: Vec<_> = params
            .iter()
            .map(|(key, value)| (key.as_str(), value.clone()))
            .collect();
        let url = format!("{DEFAULT_API_URL}?{}", form_urlencode(&pairs));
        let response = http
            .get_bounded(
                &url,
                SourceHttpOptions {
                    max_bytes: limit.max_bytes.min(access.max_response_bytes),
                    attempts: 1,
                    request_timeout: remaining.min(Duration::from_secs(30)),
                    total_timeout: remaining,
                    ..Default::default()
                },
            )
            .map_err(|_| {
                "Wiki HTTP failed or exceeded byte/deadline budget; no retry or redirect"
                    .to_string()
            })?;
        response
            .ensure_success()
            .map_err(|_| format!("Wiki HTTP {}; capture stopped", response.status))?;
        if response
            .headers
            .get("content-encoding")
            .is_some_and(|v| v != "identity")
        {
            return Err("unexpected compressed Wiki response".into());
        }
        Ok(response.content)
    })
    .map_err(SourcesError::invalid_input)
}

/// Stage approved-to-read immutable raw revisions through the existing SourceStore.
/// This deliberately creates NO entity snapshots, facts, cards or release marker.
/// Current-denial content has already been removed by the IR constructor.
pub async fn stage_sources_with_pool(pool: &PgPool, raw_dir: &Path, ir: &WikiIr) -> Result<Value> {
    if !ir.report().source_policy.raw_retention_allowed
        || ir
            .report()
            .source_policy
            .source_license
            .as_deref()
            .is_none_or(|s| s.trim().is_empty())
    {
        return Err(SourcesError::invalid_input(
            "raw staging requires explicit retention approval and source license",
        ));
    }
    let store = SourceStore::new(pool, raw_dir)?;
    let run_id = store.begin_run("wiki_knowledge_capture").await?;
    let outcome = async {
        let mut document_ids = Vec::new();
        for source in ir.sources() {
            source.validate().map_err(|_| SourcesError::invalid_input("invalid wiki source contract"))?;
            let external_id = format!("{}:revision:{}",source.logical_id,source.revision);
            let raw = source.content.as_bytes();
            let path = store.write_raw("wiki_knowledge_capture",&external_id,raw,"txt")?;
            let metadata = json!({"source_record":source,"kind":"untrusted_raw_revision","canonical_publication":false});
            // Content bytes remain in the raw artifact, not duplicated in metadata.
            let mut metadata = metadata;
            metadata["source_record"].as_object_mut().unwrap().remove("content");
            document_ids.push(store.upsert_source_document(SourceDocumentInput {
                source:"wiki_knowledge_capture",external_id:&external_id,title:None,url:None,
                content_type:"text/plain; charset=utf-8",raw_path:&path,content:raw,metadata:&metadata,
            }).await?);
        }
        Ok(json!({"document_ids":document_ids,"raw_revisions":ir.sources().len(),"published":false,"fact_writes":0,"snapshot_writes":0}))
    }.await;
    complete_run(&store, run_id, outcome).await
}
