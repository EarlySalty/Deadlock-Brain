#![forbid(unsafe_code)]

//! Retrieval-Crate fuer Context-, Timeline-, Review-, Quality- und Item-Abfragen.

use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet, HashSet},
};

pub use deadlock_brain_core as core;

use regex::Regex;
use rusqlite::{
    params, params_from_iter,
    types::{Value as SqlValue, ValueRef},
    Connection, OptionalExtension, Row,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map as JsonMap, Value as JsonValue};
use sha2::{Digest, Sha256};

const ASSETS_SOURCE: &str = "deadlock_assets_api";
const MAX_EVENTS: i64 = 500;
const MAX_TIMELINE_EVENTS: i64 = 2000;
const MAX_ALIASES: i64 = 80;
const MAX_TIMELINE_ALIASES: i64 = 120;
const MAX_SHEET_ROWS_PER_SOURCE: i64 = 5;
const MIN_ENTITY_MATCH_SCORE: i64 = 100;
const LINEAGE_NAME_LIMIT: i64 = 120;
const SAMPLE_LIMIT: i64 = 20;
const GENERAL_ENTITY_SCAN_LIMIT: usize = 50;
const LOW_CONFIDENCE_THRESHOLD: f64 = 0.5;
const PROMPT_VERSION: &str = "review_context_de_v1";
const ASK_PROMPT_TEMPLATE: &str = r#"Du bist ein erfahrener Deadlock-Coach und Analyst. Beantworte die folgende Frage – oder erstelle den gewünschten Build – AUSSCHLIESSLICH auf Basis der unten gelieferten, geprüften Fakten. Erfinde keine Werte, Items, Fähigkeiten oder Patch-Stände. Wenn die Fakten etwas nicht hergeben, sage das offen, statt zu raten.

FRAGE: {{query}}
ERKANNTE ABSICHT: {{intent}}

Die Fakten unten sind nach Vertrauensgrad geordnet. Halte dich strikt an diese Rangfolge:
1. ground_truth – gesicherte Spieldaten (offizielle Werte, Skalierung, Item- und Ability-Karten, Patch-Verlauf). Das ist die harte Wahrheit; bei jedem Widerspruch schlägt sie alles andere.
2. creator_knowledge.verified – Creator-Aussagen, die gegen die Spieldaten geprüft wurden. Belastbar und als Quelle nutzbar.
3. creator_knowledge.flagged – nur teilweise oder gar nicht bestätigt. Höchstens mit klarem Vorbehalt erwähnen ("ein Creator meint …, unbestätigt").
4. creator_knowledge.refuted – nachweislich FALSCH. Niemals als wahr verwenden. Wenn die Frage es berührt, stelle den Irrtum aktiv richtig; die korrekte Tatsache steht in der Begründung oder in ground_truth.

Regeln:
- Nenne konkrete Zahlenwerte nur, wenn sie in ground_truth oder verified belegt sind.
- Zitiere bei Creator-Wissen die Quelle (Video-Titel), sofern vorhanden.
- Beziehe dich auf den aktuellen Patch-Stand und markiere erkennbar veraltete Aussagen als solche.
- Antworte auf Deutsch, präzise und ohne Floskeln.

FAKTEN (JSON, vertrauenssortiert):
{{ordered_context_json}}"#;
const ASK_TRUST_LEGEND: &str = "Vertrauensstufen: 'ground_truth' = gesicherte Spieldaten (höchste Priorität). 'creator_knowledge.verified' = gegen die Spieldaten geprüfte Creator-Aussagen. 'creator_knowledge.flagged' = nur teilweise oder unbestätigt, nur mit Vorbehalt nutzen. 'creator_knowledge.refuted' = nachweislich falsch, nicht verwenden. Bei Widerspruch gilt immer ground_truth.";
const CLAIM_KEYWORD_STOPWORDS: &[&str] = &[
    "about",
    "also",
    "build",
    "counter",
    "does",
    "from",
    "have",
    "hero",
    "item",
    "need",
    "should",
    "that",
    "this",
    "when",
    "with",
    "without",
    "aber",
    "auch",
    "dass",
    "eine",
    "einen",
    "einer",
    "fuer",
    "gegen",
    "hero",
    "item",
    "kann",
    "oder",
    "soll",
    "ueber",
    "wenn",
    "wird",
];

const PRIMARY_STAT_KEYS: &[&str] = &[
    "base_hp",
    "max_level_hp",
    "hp_gain",
    "base_regen",
    "base_move_speed",
    "base_sprint",
    "base_stamina",
    "base_ammo",
    "base_bullet_dmg",
    "base_fire_rate",
    "base_dps",
    "max_gun_dps",
    "max_gun_damage",
    "dmg_gain",
    "spirit_gain",
    "total_bullet_ratio",
    "total_spirit_ratio",
    "aggregate_growth",
    "hp_growth_increase",
    "dps_growth_increase",
];

const PUBLIC_ENTITY_TYPES: &[&str] = &["hero", "item", "item_special", "ability"];
const IMPACT_KINDS: &[&str] = &[
    "numeric_buff",
    "numeric_nerf",
    "functional_change",
    "rework",
    "bugfix",
    "added",
    "removed",
    "unknown",
];
const IMPACT_LEVELS: &[&str] = &["high", "medium", "low", "unknown"];
const BUFF_WORDS: &[&str] = &[
    "buff",
    "increased",
    "increase",
    "raised",
    "improved",
    "faster",
    "stronger",
];
const NERF_WORDS: &[&str] = &[
    "nerf",
    "reduced",
    "decreased",
    "lowered",
    "slower",
    "weaker",
];
const FUNCTIONAL_WORDS: &[&str] = &[
    "now",
    "changed",
    "adjusted",
    "upgrades from",
    "no longer grants",
    "grants",
];
const HIGH_IMPACT_WORDS: &[&str] = &["crash", "exploit", "reworked", "new hero", "new item"];

#[derive(Debug, thiserror::Error)]
pub enum RetrievalError {
    #[error(transparent)]
    Core(#[from] core::CoreError),

    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("MiniMax response did not include message content.")]
    EmptyMiniMaxResponse,

    #[error("{0}")]
    Invalid(String),
}

pub type Result<T> = std::result::Result<T, RetrievalError>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryPlan {
    pub entities: Vec<JsonValue>,
    pub threats: Vec<String>,
    pub intent: String,
    pub fetch: Vec<String>,
    pub filters: JsonValue,
    pub raw_query: String,
    pub language: String,
}

#[derive(Debug, Clone)]
pub struct AnalysisRunMinimaxOptions {
    pub limit_events: i64,
    pub config: core::minimax::MiniMaxConfig,
    pub dry_run: bool,
}

#[derive(Debug, Clone)]
pub struct AskContextOptions {
    pub limit_events: i64,
    pub include_unverified: bool,
    pub max_claims: usize,
}

#[derive(Debug, Clone)]
struct AskClaimRecord {
    id: i64,
    claim_text: String,
    evidence_quote: String,
    claim_type: String,
    entity_name: String,
    status: String,
    verifier_confidence: f64,
    source_video: JsonValue,
    verifier: JsonMap<String, JsonValue>,
    match_sources: BTreeSet<String>,
}

#[derive(Debug, Clone, Default)]
struct AskClaimBuckets {
    verified: Vec<AskClaimRecord>,
    flagged: Vec<AskClaimRecord>,
    refuted: Vec<AskClaimRecord>,
    unverified: Vec<AskClaimRecord>,
    omitted: AskClaimOmitted,
}

#[derive(Debug, Clone, Default)]
struct AskClaimOmitted {
    verified: usize,
    flagged: usize,
    refuted: usize,
    unverified: usize,
}

#[derive(Debug, Clone, Default)]
struct AskEntityClaimMatch {
    names: Vec<String>,
    matched: bool,
}

pub fn status(conn: &Connection) -> Result<JsonValue> {
    let source_documents = fetch_all(
        conn,
        r#"
        SELECT source, COUNT(*) AS documents
        FROM source_documents
        GROUP BY source
        ORDER BY source
        "#,
        vec![],
    )?;
    let entity_snapshots = fetch_all(
        conn,
        r#"
        SELECT source, entity_type, COUNT(*) AS snapshots
        FROM entity_snapshots
        GROUP BY source, entity_type
        ORDER BY source, entity_type
        "#,
        vec![],
    )?;
    let patch_events = fetch_all(
        conn,
        r#"
        SELECT source_kind, entity_type, COUNT(*) AS events
        FROM patch_events
        GROUP BY source_kind, entity_type
        ORDER BY source_kind, entity_type
        "#,
        vec![],
    )?;
    let entities = fetch_all(
        conn,
        r#"
        SELECT entity_type, COUNT(*) AS entities
        FROM entities
        GROUP BY entity_type
        ORDER BY entity_type
        "#,
        vec![],
    )?;
    let mut derived = Vec::new();
    for table in [
        "patch_event_enrichments",
        "entity_lineage",
        "legacy_entities",
        "hero_stat_profiles",
        "hero_stat_values",
        "analysis_notes",
        "build_learning_notes",
        "player_match_decision_notes",
    ] {
        if table_exists(conn, table)? {
            derived.push(json!({"table": table, "count": scalar_i64(conn, &format!("SELECT COUNT(*) FROM {}", quote_identifier(table)), vec![])?}));
        }
    }

    Ok(json!({
        "source_documents": source_documents,
        "entity_snapshots": entity_snapshots,
        "patch_events": patch_events,
        "entities": entities,
        "derived_data": derived,
    }))
}

pub fn context(conn: &Connection, query: &str, limit_events: i64) -> Result<JsonValue> {
    build_entity_context(conn, query, limit_events)
}

pub fn build_entity_context(conn: &Connection, query: &str, limit_events: i64) -> Result<JsonValue> {
    let query = query.trim().to_string();
    let query_norm = normalize_alias(&query);
    let limit = clamp_i64(limit_events, 1, MAX_EVENTS);
    let best_match = find_best_entity_match(conn, &query, &query_norm)?;
    let fallback_used = best_match.is_none();
    let aliases = match best_match.as_ref().and_then(|row| row.get("id").and_then(JsonValue::as_i64)) {
        Some(entity_id) => load_aliases(conn, entity_id, MAX_ALIASES)?,
        None => Vec::new(),
    };
    let lineage = related_names_for_query(conn, &query, best_match.as_ref())?;
    let mut lineage_names = BTreeSet::new();
    for name in lineage_lookup_names(conn, &query, best_match.as_ref())? {
        lineage_names.insert(name);
    }
    for name in legacy_lookup_names(conn, &query)? {
        lineage_names.insert(name);
    }
    let lineage_names = lineage_names.into_iter().collect::<Vec<_>>();
    let events = load_patch_events(
        conn,
        &query,
        best_match.as_ref(),
        &aliases,
        &lineage_names,
        limit,
        PatchEventMode::Retrieval,
    )?;
    let event_ids = events
        .iter()
        .filter_map(|event| event.get("id").and_then(JsonValue::as_i64))
        .collect::<Vec<_>>();
    let event_hashes = events
        .iter()
        .filter_map(|event| value_to_nonempty_string(event.get("event_hash")))
        .collect::<Vec<_>>();

    Ok(json!({
        "query": query,
        "query_norm": query_norm,
        "best_match": best_match,
        "aliases": aliases,
        "lineage": lineage,
        "patch_events": events,
        "enrichments": load_enrichments_bundle(conn, &event_ids, &event_hashes)?,
        "sheet_stats": load_sheet_stats(conn, &query, best_match.as_ref(), &aliases)?,
        "fallback": {
            "used": fallback_used,
            "strategy": if fallback_used { JsonValue::String("patch_events.entity_name LIKE".to_string()) } else { JsonValue::Null },
        },
    }))
}

pub fn timeline(conn: &Connection, query: &str) -> Result<JsonValue> {
    build_entity_timeline(conn, query, MAX_TIMELINE_EVENTS, true)
}

pub fn build_entity_timeline(
    conn: &Connection,
    query: &str,
    limit_events: i64,
    ascending: bool,
) -> Result<JsonValue> {
    let query = query.trim().to_string();
    let query_norm = normalize_alias(&query);
    let limit = clamp_i64(limit_events, 1, MAX_TIMELINE_EVENTS);
    let best_match = find_best_entity_match(conn, &query, &query_norm)?;
    let aliases = match best_match.as_ref().and_then(|row| row.get("id").and_then(JsonValue::as_i64)) {
        Some(entity_id) => load_aliases(conn, entity_id, MAX_TIMELINE_ALIASES)?,
        None => Vec::new(),
    };
    let lineage = related_names_for_query(conn, &query, best_match.as_ref())?;
    let mut lineage_names = BTreeSet::new();
    for name in lineage_lookup_names(conn, &query, best_match.as_ref())? {
        lineage_names.insert(name);
    }
    for name in legacy_lookup_names(conn, &query)? {
        lineage_names.insert(name);
    }
    let lineage_names = lineage_names.into_iter().collect::<Vec<_>>();
    let events = load_patch_events(
        conn,
        &query,
        best_match.as_ref(),
        &aliases,
        &lineage_names,
        limit,
        PatchEventMode::Timeline { ascending },
    )?;
    let event_ids = events
        .iter()
        .filter_map(|event| event.get("id").and_then(JsonValue::as_i64))
        .collect::<Vec<_>>();
    let enrichments = load_enrichments_by_event_id(conn, &event_ids)?;
    let mut classified_events = Vec::new();
    for mut event in events {
        let enrichment = event
            .get("id")
            .and_then(JsonValue::as_i64)
            .and_then(|id| enrichments.get(&id).cloned());
        let impact = classify_patch_event_impact_map(&event, enrichment.as_ref());
        event.insert(
            "enrichment".to_string(),
            enrichment
                .map(JsonValue::Object)
                .unwrap_or(JsonValue::Null),
        );
        for (key, value) in impact {
            event.insert(key, value);
        }
        classified_events.push(event);
    }
    classified_events.sort_by(compare_event_sort_key);
    let patches = group_events_by_patch(&classified_events, ascending);

    Ok(json!({
        "query": query,
        "query_norm": query_norm,
        "best_match": best_match,
        "aliases": aliases,
        "lineage": lineage,
        "patches": patches,
        "event_count": classified_events.len(),
        "patch_count": patches.len(),
        "impact_summary": impact_summary(&classified_events),
        "enrichments": {
            "available": table_exists(conn, "patch_event_enrichments")?,
            "matched": classified_events.iter().filter(|event| event.get("enrichment").is_some_and(|value| !value.is_null())).count(),
        },
        "fallback": {
            "used": best_match.is_none(),
            "strategy": if best_match.is_none() { JsonValue::String("patch_events.entity_name LIKE".to_string()) } else { JsonValue::Null },
        },
    }))
}

pub fn classify_patch_event_impact(
    event: &JsonValue,
    enrichment: Option<&JsonValue>,
) -> JsonValue {
    let event_obj = event.as_object().cloned().unwrap_or_default();
    let enrichment_obj = enrichment.and_then(JsonValue::as_object).cloned();
    JsonValue::Object(classify_patch_event_impact_map(&event_obj, enrichment_obj.as_ref()))
}

pub fn review(conn: &Connection, query: &str) -> Result<JsonValue> {
    build_review_context(conn, query, 80)
}

pub fn build_review_context(conn: &Connection, query: &str, limit_events: i64) -> Result<JsonValue> {
    let retrieval_context = build_entity_context(conn, query, limit_events)?;
    let ctx = retrieval_context.as_object().cloned().unwrap_or_default();
    let best_match = ctx
        .get("best_match")
        .and_then(JsonValue::as_object)
        .cloned()
        .unwrap_or_default();
    let events = json_array_objects(ctx.get("patch_events"));
    let enrichments = ctx
        .get("enrichments")
        .and_then(JsonValue::as_object)
        .and_then(|value| value.get("rows"))
        .map(|value| json_array_objects(Some(value)))
        .unwrap_or_default();

    let entity_summary = build_entity_summary(&ctx);
    let current_stat_hints = build_current_stat_hints(&ctx);
    let timeline_signals = build_timeline_signals(&events, &enrichments);
    let source_references = build_source_references(&ctx, &current_stat_hints);
    let open_questions = build_open_questions(
        &ctx,
        &entity_summary,
        &current_stat_hints,
        &timeline_signals,
    );

    Ok(json!({
        "query": ctx.get("query").cloned().unwrap_or_else(|| json!(query)),
        "context_kind": "analysis_review",
        "entity_summary": entity_summary,
        "lineage": build_lineage_summary(&ctx),
        "current_stat_hints": current_stat_hints,
        "timeline_signals": timeline_signals,
        "open_questions": open_questions,
        "source_references": source_references,
        "prompt_de": build_prompt_de(&best_match, query),
        "retrieval_meta": {
            "limit_events_requested": limit_events,
            "events_loaded": events.len(),
            "enrichments_loaded": enrichments.len(),
            "fallback_used": ctx.get("fallback").and_then(JsonValue::as_object).and_then(|value| value.get("used")).and_then(JsonValue::as_bool).unwrap_or(false),
            "sheet_stats_available": ctx.get("sheet_stats").and_then(JsonValue::as_object).and_then(|value| value.get("available")).and_then(JsonValue::as_bool).unwrap_or(false),
        },
    }))
}

pub fn ask_context(conn: &Connection, query: &str, opts: &AskContextOptions) -> Result<JsonValue> {
    let plan = analyze_query(conn, query)?;
    let base = build_review_context(conn, query, opts.limit_events)?;
    let entity_match = resolve_ask_entity_match(conn, query, &plan)?;
    let (claims, entity_matched, keyword_matched) =
        load_ask_claims(conn, query, &entity_match.names)?;
    let claim_buckets = partition_ask_claims(claims, opts.include_unverified, opts.max_claims);

    let entity = base.get("entity_summary").cloned().unwrap_or(JsonValue::Null);
    let item_ground_truth = ask_item_ground_truth(conn, &entity)?;
    let ground_truth = json!({
        "stats": base.get("current_stat_hints").cloned().unwrap_or(JsonValue::Null),
        "timeline": base.get("timeline_signals").cloned().unwrap_or(JsonValue::Null),
        "lineage": base.get("lineage").cloned().unwrap_or(JsonValue::Null),
        "item": item_ground_truth,
    });
    let mut creator_knowledge = JsonMap::new();
    creator_knowledge.insert("verified".to_string(), claims_to_json(&claim_buckets.verified));
    creator_knowledge.insert("flagged".to_string(), claims_to_json(&claim_buckets.flagged));
    creator_knowledge.insert("refuted".to_string(), claims_to_json(&claim_buckets.refuted));
    if opts.include_unverified {
        creator_knowledge.insert(
            "unverified".to_string(),
            claims_to_json(&claim_buckets.unverified),
        );
    }
    creator_knowledge.insert("omitted".to_string(), omitted_to_json(&claim_buckets.omitted));

    let mut result = JsonMap::new();
    result.insert("query".to_string(), json!(query));
    result.insert("intent".to_string(), json!(plan.intent.clone()));
    result.insert("entity".to_string(), entity);
    result.insert("ground_truth".to_string(), ground_truth);
    result.insert(
        "creator_knowledge".to_string(),
        JsonValue::Object(creator_knowledge),
    );
    result.insert(
        "sources".to_string(),
        base.get("source_references").cloned().unwrap_or_else(|| json!([])),
    );
    result.insert("trust_legend".to_string(), json!(ASK_TRUST_LEGEND));
    result.insert(
        "retrieval_meta".to_string(),
        json!({
            "claims_scanned": entity_matched + keyword_matched,
            "claims_selected": claim_buckets.verified.len()
                + claim_buckets.flagged.len()
                + claim_buckets.refuted.len()
                + claim_buckets.unverified.len(),
            "entity_matched": entity_matched,
            "entity_match_resolved": entity_match.matched,
            "keyword_matched": keyword_matched,
            "intent": plan.intent,
            "base_prompt_de_available": base.get("prompt_de").and_then(JsonValue::as_str).is_some_and(|value| !value.trim().is_empty()),
        }),
    );

    let prompt = render_ask_prompt(&JsonValue::Object(result.clone()))?;
    result.insert("prompt".to_string(), JsonValue::String(prompt));
    Ok(JsonValue::Object(result))
}

pub fn quality(conn: &Connection) -> Result<JsonValue> {
    run_quality_checks(conn)
}

pub fn run_quality_checks(conn: &Connection) -> Result<JsonValue> {
    let checks = vec![
        check_required_tables(conn)?,
        check_entity_counts(conn)?,
        check_alias_collisions(conn)?,
        check_patch_events_unknown_entities(conn)?,
        check_enrichment_table(conn)?,
        check_lineage_table(conn)?,
        check_legacy_entities_table(conn)?,
        check_low_confidence_enrichments(conn)?,
        check_sheet_profiles_without_entity(conn)?,
        check_general_events_with_known_entity_names(conn)?,
    ];
    let worst = checks
        .iter()
        .filter_map(|check| check.get("severity").and_then(JsonValue::as_str))
        .map(severity_rank)
        .max()
        .unwrap_or(0);
    let warnings = checks
        .iter()
        .filter(|check| check.get("severity").and_then(JsonValue::as_str) == Some("warning"))
        .count();
    let errors = checks
        .iter()
        .filter(|check| check.get("severity").and_then(JsonValue::as_str) == Some("error"))
        .count();
    Ok(json!({
        "ok": worst < severity_rank("warning"),
        "summary": {"checks": checks.len(), "warnings": warnings, "errors": errors},
        "checks": checks,
    }))
}

pub fn item(conn: &Connection, query: &str) -> Result<JsonValue> {
    build_item_context(conn, query)
}

pub fn build_item_context(conn: &Connection, query: &str) -> Result<JsonValue> {
    let payload = load_entity_payload(conn, query, "item")?
        .or_else(|| load_entity_payload(conn, query, "item_special").ok().flatten())
        .ok_or_else(|| RetrievalError::Invalid(format!("Kein Item-Payload fuer {query} gefunden.")))?;
    Ok(JsonValue::Object(item_summary(&payload)))
}

pub fn summarize_item_payload(payload: &JsonValue) -> JsonValue {
    JsonValue::Object(item_summary(payload.as_object().unwrap_or(&JsonMap::new())))
}

pub fn analysis_save_review(
    conn: &Connection,
    review_context: &JsonValue,
    result_text: Option<&str>,
    model: Option<&str>,
    confidence: Option<f64>,
    status: &str,
    provider_metadata: Option<&JsonValue>,
) -> Result<JsonValue> {
    save_review_analysis_note(
        conn,
        review_context,
        result_text,
        model,
        confidence,
        status,
        provider_metadata,
    )
}

pub fn analysis_save_review_for_query(
    conn: &Connection,
    query: &str,
    limit_events: i64,
    result_text: Option<&str>,
    model: Option<&str>,
    confidence: Option<f64>,
) -> Result<JsonValue> {
    let review_context = build_review_context(conn, query, limit_events)?;
    let status = if result_text.is_some() {
        "analysis_ready"
    } else {
        "context_ready"
    };
    save_review_analysis_note(
        conn,
        &review_context,
        result_text,
        model,
        confidence,
        status,
        None,
    )
}

pub fn analysis_run_minimax(
    conn: &Connection,
    query: &str,
    options: AnalysisRunMinimaxOptions,
) -> Result<JsonValue> {
    let review_context = build_review_context(conn, query, options.limit_events)?;
    let prompt = review_context
        .get("prompt_de")
        .and_then(JsonValue::as_str)
        .unwrap_or_default();
    let compact_context = compact_context_for_model(&review_context);
    let request = core::minimax::build_review_request(prompt, &compact_context, &options.config);
    let endpoint = format!("{}/chat/completions", options.config.base_url.trim_end_matches('/'));
    let request_value = serde_json::to_value(&request)?;

    if options.dry_run {
        return Ok(json!({
            "dry_run": true,
            "query": query,
            "model": options.config.model,
            "base_url": options.config.base_url,
            "endpoint": endpoint,
            "api_key_present": options.config.api_key_present(),
            "request": request_value,
        }));
    }

    let client = core::minimax::MiniMaxClient::new(options.config.clone())?;
    let response = client.chat(&request)?;
    let result_text = core::minimax::extract_minimax_text(&response);
    if result_text.is_empty() {
        return Err(RetrievalError::EmptyMiniMaxResponse);
    }
    let provider_metadata = core::minimax::minimax_usage_summary(&response);
    let note = save_review_analysis_note(
        conn,
        &review_context,
        Some(&result_text),
        Some(&options.config.model),
        None,
        "analysis_ready",
        Some(&provider_metadata),
    )?;
    Ok(json!({
        "note": note,
        "query": query,
        "model": options.config.model,
        "endpoint": endpoint,
        "result_text": result_text,
        "provider_metadata": provider_metadata,
    }))
}

pub fn analysis_list(conn: &Connection, query: Option<&str>, limit: i64) -> Result<JsonValue> {
    let max_rows = clamp_i64(limit, 1, 500);
    let mut sql = r#"
        SELECT id, query, entity_type, entity_name, context_kind, context_hash,
               prompt_version, model, confidence, status, created_at, updated_at
        FROM analysis_notes
    "#
    .to_string();
    let mut params = Vec::new();
    if let Some(query) = query.filter(|value| !value.trim().is_empty()) {
        sql.push_str(" WHERE query LIKE ? OR entity_name LIKE ?");
        params.push(SqlValue::Text(format!("%{query}%")));
        params.push(SqlValue::Text(format!("%{query}%")));
    }
    sql.push_str(" ORDER BY updated_at DESC, id DESC LIMIT ?");
    params.push(SqlValue::Integer(max_rows));
    Ok(JsonValue::Array(
        fetch_all(conn, &sql, params)?
            .into_iter()
            .map(JsonValue::Object)
            .collect(),
    ))
}

pub fn search_mechanic_notes(conn: &Connection, _query: &str, _limit: i64) -> Result<JsonValue> {
    if !table_exists(conn, "mechanic_notes")? || !table_exists(conn, "vector_embeddings")? {
        return Ok(JsonValue::Array(Vec::new()));
    }
    todo!("Vektorsuche: spätere Wave")
}

pub fn analyze_query(conn: &Connection, query: &str) -> Result<QueryPlan> {
    let known_heroes = fetch_all(
        conn,
        "SELECT canonical_name FROM entities WHERE entity_type='hero'",
        vec![],
    )?
    .into_iter()
    .filter_map(|row| value_to_nonempty_string(row.get("canonical_name")))
    .collect::<Vec<_>>();
    let known_items = fetch_all(
        conn,
        "SELECT canonical_name FROM entities WHERE entity_type='item' LIMIT 60",
        vec![],
    )?
    .into_iter()
    .filter_map(|row| value_to_nonempty_string(row.get("canonical_name")))
    .collect::<Vec<_>>();

    let mut intent = "hero_overview".to_string();
    let mut entities = Vec::new();
    let mut threats = Vec::new();
    let query_lower = query.to_lowercase();
    for hero in &known_heroes {
        if query_lower.contains(&hero.to_lowercase()) {
            if entities.is_empty() {
                entities.push(json!({"name": hero, "type": "hero", "raw": hero}));
            } else {
                threats.push(hero.clone());
            }
        }
    }
    if entities.is_empty() {
        for item in &known_items {
            if query_lower.contains(&item.to_lowercase()) {
                entities.push(json!({"name": item, "type": "item", "raw": item}));
                intent = "item_question".to_string();
            }
        }
    }
    Ok(QueryPlan {
        fetch: intent_fetch(&intent),
        entities,
        threats,
        intent,
        filters: json!({}),
        raw_query: query.to_string(),
        language: "de".to_string(),
    })
}

fn intent_fetch(intent: &str) -> Vec<String> {
    let values = match intent {
        "build_recommendation" => vec![
            "builds",
            "build_notes",
            "item_wpa",
            "hero_stats",
            "patch_impact_notes",
        ],
        "item_question" => vec!["item_data", "item_wpa", "patch_events"],
        "patch_changes" => vec!["patch_events", "enrichments", "patch_impact_notes"],
        "mechanics_question" => vec![
            "hidden_mechanics",
            "boons_ap",
            "shop_bonuses",
            "damage_calc",
        ],
        "meta_question" | "hero_comparison" => {
            vec!["hero_rankings", "hero_stats", "patch_impact_notes"]
        }
        "match_coaching" => vec!["match_data", "build_notes"],
        _ => vec![
            "hero_stats",
            "hero_rankings",
            "patch_events",
            "patch_impact_notes",
        ],
    };
    values.into_iter().map(str::to_string).collect()
}

fn find_best_entity_match(
    conn: &Connection,
    query: &str,
    query_norm: &str,
) -> Result<Option<JsonMap<String, JsonValue>>> {
    if query_norm.is_empty()
        || !table_exists(conn, "entities")?
        || !table_exists(conn, "entity_aliases")?
    {
        return Ok(None);
    }
    let like_norm = format!("%{query_norm}%");
    let like_query = format!("%{query}%");
    let rows = fetch_all(
        conn,
        r#"
        SELECT
          e.id,
          e.entity_type,
          e.canonical_name,
          e.primary_external_id,
          e.source,
          e.metadata_json,
          MAX(
            CASE
              WHEN lower(e.canonical_name)=lower(?) THEN 120
              WHEN a.alias_norm=? AND a.alias_kind='canonical' THEN 115
              WHEN a.alias_norm=? THEN 110
              WHEN a.alias_norm LIKE ? THEN 80
              WHEN lower(e.canonical_name) LIKE lower(?) THEN 70
              ELSE 50
            END
          ) AS score,
          GROUP_CONCAT(DISTINCT a.alias_kind) AS matched_alias_kinds
        FROM entities e
        LEFT JOIN entity_aliases a ON a.entity_id=e.id
        WHERE
          lower(e.canonical_name)=lower(?)
          OR lower(e.canonical_name) LIKE lower(?)
          OR a.alias_norm=?
          OR a.alias_norm LIKE ?
        GROUP BY e.id
        ORDER BY score DESC, e.entity_type, length(e.canonical_name), e.canonical_name
        LIMIT 1
        "#,
        vec![
            SqlValue::Text(query.to_string()),
            SqlValue::Text(query_norm.to_string()),
            SqlValue::Text(query_norm.to_string()),
            SqlValue::Text(like_norm.clone()),
            SqlValue::Text(like_query.clone()),
            SqlValue::Text(query.to_string()),
            SqlValue::Text(like_query),
            SqlValue::Text(query_norm.to_string()),
            SqlValue::Text(like_norm),
        ],
    )?;
    let Some(mut row) = rows.into_iter().next() else {
        return Ok(None);
    };
    let score = row.get("score").and_then(JsonValue::as_i64).unwrap_or(0);
    if score < MIN_ENTITY_MATCH_SCORE {
        return Ok(None);
    }
    let metadata = loads_json_object(row.remove("metadata_json").as_ref());
    let alias_kinds = split_group_concat(row.get("matched_alias_kinds"));
    row.insert("metadata".to_string(), JsonValue::Object(metadata));
    row.insert("score".to_string(), json!(score));
    row.insert(
        "matched_alias_kinds".to_string(),
        JsonValue::Array(alias_kinds.into_iter().map(JsonValue::String).collect()),
    );
    Ok(Some(row))
}

fn load_aliases(conn: &Connection, entity_id: i64, limit: i64) -> Result<Vec<JsonMap<String, JsonValue>>> {
    fetch_all(
        conn,
        r#"
        SELECT alias, alias_norm, alias_kind, source, external_id, snapshot_id
        FROM entity_aliases
        WHERE entity_id=?
        ORDER BY
          CASE alias_kind
            WHEN 'canonical' THEN 0
            WHEN 'snapshot_name' THEN 1
            WHEN 'class_name' THEN 2
            WHEN 'class_name_short' THEN 3
            WHEN 'external_id' THEN 4
            ELSE 5
          END,
          length(alias),
          alias
        LIMIT ?
        "#,
        vec![SqlValue::Integer(entity_id), SqlValue::Integer(limit)],
    )
}

fn related_names_for_query(
    conn: &Connection,
    query: &str,
    best_match: Option<&JsonMap<String, JsonValue>>,
) -> Result<Vec<JsonMap<String, JsonValue>>> {
    if !table_exists(conn, "entity_lineage")? {
        return Ok(Vec::new());
    }
    let mut names = BTreeSet::new();
    let query = query.trim();
    if !query.is_empty() {
        names.insert(query.to_string());
    }
    if let Some(canonical) = best_match
        .and_then(|row| value_to_nonempty_string(row.get("canonical_name")))
    {
        names.insert(canonical);
    }
    let norms = names
        .iter()
        .map(|name| normalize_alias(name))
        .filter(|name| !name.is_empty())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    if norms.is_empty() {
        return Ok(Vec::new());
    }
    let placeholders = placeholders(norms.len());
    let sql = format!(
        r#"
        SELECT *
        FROM entity_lineage
        WHERE source_name_norm IN ({placeholders})
           OR target_name_norm IN ({placeholders})
           OR owner_name_norm IN ({placeholders})
        ORDER BY confidence DESC, id ASC
        LIMIT ?
        "#
    );
    let mut params = Vec::new();
    for _ in 0..3 {
        params.extend(norms.iter().cloned().map(SqlValue::Text));
    }
    params.push(SqlValue::Integer(LINEAGE_NAME_LIMIT));
    let rows = fetch_all(conn, &sql, params)?
        .into_iter()
        .map(decode_metadata)
        .collect();
    Ok(rows)
}

fn lineage_lookup_names(
    conn: &Connection,
    query: &str,
    best_match: Option<&JsonMap<String, JsonValue>>,
) -> Result<Vec<String>> {
    let mut names = BTreeSet::new();
    let query = query.trim();
    if !query.is_empty() {
        names.insert(query.to_string());
    }
    if let Some(canonical) = best_match
        .and_then(|row| value_to_nonempty_string(row.get("canonical_name")))
    {
        names.insert(canonical);
    }
    for row in related_names_for_query(conn, query, best_match)? {
        for key in ["source_name", "target_name", "owner_name"] {
            if let Some(value) = value_to_nonempty_string(row.get(key)) {
                names.insert(value);
            }
        }
    }
    Ok(names.into_iter().filter(|name| !name.is_empty()).collect())
}

fn legacy_lookup_names(conn: &Connection, query: &str) -> Result<Vec<String>> {
    if !table_exists(conn, "legacy_entities")? {
        return Ok(Vec::new());
    }
    let query_norm = normalize_alias(query);
    if query_norm.is_empty() {
        return Ok(Vec::new());
    }
    let rows = fetch_all(
        conn,
        r#"
        SELECT canonical_name
        FROM legacy_entities
        WHERE name_norm=? OR name_norm LIKE ?
        ORDER BY confidence DESC, event_count DESC, canonical_name
        LIMIT 40
        "#,
        vec![
            SqlValue::Text(query_norm.clone()),
            SqlValue::Text(format!("%{query_norm}%")),
        ],
    )?;
    Ok(rows
        .into_iter()
        .filter_map(|row| value_to_nonempty_string(row.get("canonical_name")))
        .collect())
}

#[derive(Clone, Copy)]
enum PatchEventMode {
    Retrieval,
    Timeline { ascending: bool },
}

fn load_patch_events(
    conn: &Connection,
    query: &str,
    best_match: Option<&JsonMap<String, JsonValue>>,
    aliases: &[JsonMap<String, JsonValue>],
    lineage_names: &[String],
    limit: i64,
    mode: PatchEventMode,
) -> Result<Vec<JsonMap<String, JsonValue>>> {
    if !table_exists(conn, "patch_events")? {
        return Ok(Vec::new());
    }
    let mut params = Vec::new();
    let where_clause = if let Some(best_match) = best_match {
        let mut names = event_lookup_names(best_match, aliases);
        names.extend(lineage_names.iter().map(|name| name.to_lowercase()));
        let names = names.into_iter().collect::<BTreeSet<_>>().into_iter().collect::<Vec<_>>();
        if names.is_empty() {
            return Ok(Vec::new());
        }
        params.extend(names.into_iter().map(SqlValue::Text));
        format!("lower(entity_name) IN ({})", placeholders(params.len()))
    } else if lineage_names.is_empty() {
        params.push(SqlValue::Text(format!("%{query}%")));
        "entity_name LIKE ?".to_string()
    } else {
        let names = lineage_names
            .iter()
            .map(|name| name.to_lowercase())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        params.extend(names.iter().cloned().map(SqlValue::Text));
        params.push(SqlValue::Text(format!("%{query}%")));
        format!(
            "(lower(entity_name) IN ({}) OR entity_name LIKE ?)",
            placeholders(names.len())
        )
    };

    let order_by = match mode {
        PatchEventMode::Retrieval => "patch_snapshot_id DESC, line_index".to_string(),
        PatchEventMode::Timeline { ascending } => {
            let direction = if ascending { "ASC" } else { "DESC" };
            format!("COALESCE(posted_at, '') {direction}, patch_snapshot_id {direction}, line_index {direction}")
        }
    };
    let sql_limit = match mode {
        PatchEventMode::Retrieval => MAX_EVENTS,
        PatchEventMode::Timeline { .. } => limit,
    };
    params.push(SqlValue::Integer(sql_limit));
    let sql = format!(
        r#"
        SELECT
          id,
          patch_snapshot_id,
          patch_external_id,
          patch_title,
          patch_url,
          source_kind,
          posted_at,
          line_index,
          section,
          entity_type,
          entity_name,
          subject,
          change_type,
          raw_line,
          normalized_line,
          old_value,
          new_value,
          confidence,
          metadata_json,
          event_hash,
          created_at
        FROM patch_events
        WHERE {where_clause}
        ORDER BY {order_by}
        LIMIT ?
        "#
    );
    let mut rows = fetch_all(conn, &sql, params)?;
    for row in &mut rows {
        let metadata = loads_json_object(row.remove("metadata_json").as_ref());
        row.insert("metadata".to_string(), JsonValue::Object(metadata));
    }
    if matches!(mode, PatchEventMode::Retrieval) {
        rows.sort_by(|left, right| compare_event_sort_key(right, left));
        rows.truncate(limit as usize);
    }
    Ok(rows)
}

fn event_lookup_names(
    best_match: &JsonMap<String, JsonValue>,
    aliases: &[JsonMap<String, JsonValue>],
) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    if let Some(name) = value_to_nonempty_string(best_match.get("canonical_name")) {
        names.insert(name.to_lowercase());
    }
    for alias in aliases {
        let alias_kind = value_to_string(alias.get("alias_kind"));
        if matches!(alias_kind.as_str(), "canonical" | "snapshot_name" | "class_name_short") {
            if let Some(value) = value_to_nonempty_string(alias.get("alias")) {
                names.insert(value.to_lowercase());
            }
        }
    }
    names
}

fn load_enrichments_bundle(
    conn: &Connection,
    event_ids: &[i64],
    event_hashes: &[String],
) -> Result<JsonValue> {
    let table = "patch_event_enrichments";
    if !table_exists(conn, table)? {
        return Ok(json!({"available": false, "rows": []}));
    }
    let columns = table_columns(conn, table)?;
    let mut rows = Vec::new();
    if columns.contains(&"patch_event_id".to_string()) && !event_ids.is_empty() {
        rows.extend(select_by_values_i64(conn, table, "patch_event_id", event_ids)?);
    } else if columns.contains(&"event_id".to_string()) && !event_ids.is_empty() {
        rows.extend(select_by_values_i64(conn, table, "event_id", event_ids)?);
    } else if columns.contains(&"event_hash".to_string()) && !event_hashes.is_empty() {
        rows.extend(select_by_values_text(conn, table, "event_hash", event_hashes)?);
    }
    let rows = dedupe_rows(rows.into_iter().map(decode_json_fields).collect());
    Ok(json!({"available": true, "rows": rows}))
}

fn load_enrichments_by_event_id(
    conn: &Connection,
    event_ids: &[i64],
) -> Result<BTreeMap<i64, JsonMap<String, JsonValue>>> {
    if event_ids.is_empty() || !table_exists(conn, "patch_event_enrichments")? {
        return Ok(BTreeMap::new());
    }
    let columns = table_columns(conn, "patch_event_enrichments")?;
    if !columns.contains(&"patch_event_id".to_string()) {
        return Ok(BTreeMap::new());
    }
    let mut enrichments = BTreeMap::new();
    for row in select_by_values_i64(conn, "patch_event_enrichments", "patch_event_id", event_ids)? {
        let decoded = decode_json_fields(row);
        if let Some(patch_event_id) = decoded.get("patch_event_id").and_then(JsonValue::as_i64) {
            enrichments.insert(patch_event_id, decoded);
        }
    }
    Ok(enrichments)
}

fn load_sheet_stats(
    conn: &Connection,
    query: &str,
    best_match: Option<&JsonMap<String, JsonValue>>,
    aliases: &[JsonMap<String, JsonValue>],
) -> Result<JsonValue> {
    let names = sheet_lookup_names(query, best_match, aliases);
    let mut sources = Vec::new();
    let snapshot_rows = load_sheet_snapshot_stats(conn, &names)?;
    if !snapshot_rows.is_empty() {
        sources.push(json!({"source": "entity_snapshots", "rows": snapshot_rows}));
    }
    for table in candidate_sheet_tables(conn)? {
        let rows = load_matching_sheet_table_rows(conn, &table, &names)?;
        if !rows.is_empty() {
            sources.push(json!({"source": table, "rows": rows}));
        }
    }
    Ok(json!({"available": !sources.is_empty(), "sources": sources}))
}

fn load_sheet_snapshot_stats(
    conn: &Connection,
    names: &[String],
) -> Result<Vec<JsonMap<String, JsonValue>>> {
    if !table_exists(conn, "entity_snapshots")? {
        return Ok(Vec::new());
    }
    let lowered = names
        .iter()
        .map(|name| name.to_lowercase())
        .filter(|name| !name.is_empty())
        .collect::<Vec<_>>();
    if lowered.is_empty() {
        return Ok(Vec::new());
    }
    let sql = format!(
        r#"
        SELECT id, source, entity_type, external_id, canonical_name, payload_json, fetched_at
        FROM entity_snapshots
        WHERE
          (source='deadlock_stats_sheet' OR entity_type='hero_stats_sheet')
          AND lower(canonical_name) IN ({})
        ORDER BY fetched_at DESC, id DESC
        LIMIT ?
        "#,
        placeholders(lowered.len())
    );
    let mut params = lowered.into_iter().map(SqlValue::Text).collect::<Vec<_>>();
    params.push(SqlValue::Integer(MAX_SHEET_ROWS_PER_SOURCE));
    let mut rows = fetch_all(conn, &sql, params)?;
    for row in &mut rows {
        let payload = loads_json_object(row.remove("payload_json").as_ref());
        let values = payload
            .get("values")
            .cloned()
            .unwrap_or(JsonValue::Object(payload));
        row.insert("values".to_string(), values);
    }
    Ok(rows)
}

fn candidate_sheet_tables(conn: &Connection) -> Result<Vec<String>> {
    let rows = fetch_all(
        conn,
        r#"
        SELECT name
        FROM sqlite_master
        WHERE type='table'
          AND name NOT LIKE 'sqlite_%'
          AND name NOT IN (
            'source_documents',
            'source_runs',
            'entity_snapshots',
            'entities',
            'entity_aliases',
            'patch_events',
            'patch_event_enrichments'
          )
        ORDER BY name
        "#,
        vec![],
    )?;
    Ok(rows
        .into_iter()
        .filter_map(|row| value_to_nonempty_string(row.get("name")))
        .filter(|name| {
            let lowered = name.to_lowercase();
            lowered.contains("sheet") || lowered.contains("stat")
        })
        .collect())
}

fn load_matching_sheet_table_rows(
    conn: &Connection,
    table: &str,
    names: &[String],
) -> Result<Vec<JsonMap<String, JsonValue>>> {
    let columns = table_columns(conn, table)?;
    if columns.is_empty() || names.is_empty() {
        return Ok(Vec::new());
    }
    let preferred = columns
        .iter()
        .filter(|column| {
            matches!(
                column.to_lowercase().as_str(),
                "entity_name" | "canonical_name" | "hero_name" | "hero" | "name"
            )
        })
        .cloned()
        .collect::<Vec<_>>();
    let text_columns = if preferred.is_empty() {
        columns.iter().take(columns.len().min(8)).cloned().collect::<Vec<_>>()
    } else {
        preferred
    };
    let mut clauses = Vec::new();
    let mut params = Vec::new();
    for column in &text_columns {
        for name in names {
            clauses.push(format!("{} LIKE ?", quote_identifier(column)));
            params.push(SqlValue::Text(format!("%{name}%")));
        }
    }
    if clauses.is_empty() {
        return Ok(Vec::new());
    }
    params.push(SqlValue::Integer(MAX_SHEET_ROWS_PER_SOURCE));
    let sql = format!(
        r#"
        SELECT *
        FROM {}
        WHERE {}
        LIMIT ?
        "#,
        quote_identifier(table),
        clauses.join(" OR ")
    );
    Ok(fetch_all(conn, &sql, params)?
        .into_iter()
        .map(decode_json_fields)
        .collect())
}

fn sheet_lookup_names(
    query: &str,
    best_match: Option<&JsonMap<String, JsonValue>>,
    aliases: &[JsonMap<String, JsonValue>],
) -> Vec<String> {
    let mut names = BTreeSet::new();
    let query = query.trim();
    if !query.is_empty() {
        names.insert(query.to_string());
    }
    if let Some(canonical) = best_match
        .and_then(|row| value_to_nonempty_string(row.get("canonical_name")))
    {
        names.insert(canonical);
    }
    for alias in aliases {
        let alias_kind = value_to_string(alias.get("alias_kind"));
        if matches!(alias_kind.as_str(), "canonical" | "snapshot_name" | "class_name_short") {
            if let Some(value) = value_to_nonempty_string(alias.get("alias")) {
                if !value.chars().all(|ch| ch.is_ascii_digit()) {
                    names.insert(value);
                }
            }
        }
    }
    names.into_iter().filter(|name| !name.is_empty()).collect()
}

fn classify_patch_event_impact_map(
    event: &JsonMap<String, JsonValue>,
    enrichment: Option<&JsonMap<String, JsonValue>>,
) -> JsonMap<String, JsonValue> {
    let empty = JsonMap::new();
    let enrichment = enrichment.unwrap_or(&empty);
    let change_type = value_to_string(event.get("change_type")).to_lowercase();
    let line = value_to_nonempty_string(event.get("normalized_line"))
        .or_else(|| value_to_nonempty_string(event.get("raw_line")))
        .unwrap_or_default();
    let lower_line = line.to_lowercase();
    let flags = coerce_flags(enrichment.get("flags"));
    let stat_name = value_to_string(enrichment.get("stat_name")).to_lowercase();
    let confidence = safe_f64(
        enrichment.get("confidence"),
        safe_f64(event.get("confidence"), 0.0),
    );

    if change_type == "added"
        || lower_line.contains("added")
        || lower_line.contains("new item")
        || lower_line.contains("new hero")
    {
        return impact("added", non_numeric_level("added", &lower_line, confidence), confidence, flags);
    }
    if change_type == "removed"
        || lower_line.contains("removed")
        || lower_line.contains("no longer")
        || flags.iter().any(|flag| flag == "removed_grant")
    {
        return impact(
            "removed",
            non_numeric_level("removed", &lower_line, confidence),
            confidence,
            flags,
        );
    }
    if change_type == "bugfix" || lower_line.contains("fixed") || lower_line.contains("bug") {
        return impact(
            "bugfix",
            non_numeric_level("bugfix", &lower_line, confidence),
            confidence,
            flags,
        );
    }
    if change_type == "rework"
        || lower_line.contains("reworked")
        || lower_line.contains("rescaled")
        || lower_line.contains("moved from")
    {
        return impact(
            "rework",
            non_numeric_level("rework", &lower_line, confidence),
            confidence,
            flags,
        );
    }
    if flags.iter().any(|flag| flag == "movement") || stat_name == "item_tier" {
        return impact(
            "rework",
            non_numeric_level("rework", &lower_line, confidence),
            confidence,
            flags,
        );
    }

    let old_value = first_present([enrichment.get("old_value"), event.get("old_value")]);
    let new_value = first_present([enrichment.get("new_value"), event.get("new_value")]);
    let numeric_delta = numeric_delta(old_value.as_ref(), new_value.as_ref());

    if change_type == "buff"
        || BUFF_WORDS.iter().any(|word| lower_line.contains(word))
        || has_direction(&flags, "increased")
    {
        let kind = if numeric_delta.is_some() {
            "numeric_buff"
        } else {
            "functional_change"
        };
        return impact(
            kind,
            level_for_kind(kind, &lower_line, confidence, numeric_delta.as_ref()),
            confidence,
            flags,
        );
    }
    if change_type == "nerf"
        || NERF_WORDS.iter().any(|word| lower_line.contains(word))
        || has_direction(&flags, "reduced")
    {
        let kind = if numeric_delta.is_some() {
            "numeric_nerf"
        } else {
            "functional_change"
        };
        return impact(
            kind,
            level_for_kind(kind, &lower_line, confidence, numeric_delta.as_ref()),
            confidence,
            flags,
        );
    }
    if let Some(delta) = numeric_delta.as_ref().and_then(|value| value.get("delta")).copied() {
        if delta > 0.0 {
            return impact(
                "numeric_buff",
                level_for_kind("numeric_buff", &lower_line, confidence, numeric_delta.as_ref()),
                confidence,
                flags,
            );
        }
        if delta < 0.0 {
            return impact(
                "numeric_nerf",
                level_for_kind("numeric_nerf", &lower_line, confidence, numeric_delta.as_ref()),
                confidence,
                flags,
            );
        }
    }
    if change_type == "changed" || FUNCTIONAL_WORDS.iter().any(|word| lower_line.contains(word)) {
        return impact(
            "functional_change",
            level_for_kind("functional_change", &lower_line, confidence, None),
            confidence,
            flags,
        );
    }
    if stat_name == "upgrades_from"
        || (!stat_name.is_empty() && (old_value.is_some() || new_value.is_some()))
    {
        return impact(
            "functional_change",
            level_for_kind("functional_change", &lower_line, confidence, None),
            confidence,
            flags,
        );
    }
    impact("unknown", "unknown", confidence, flags)
}

fn impact(kind: &str, level: &str, confidence: f64, flags: Vec<String>) -> JsonMap<String, JsonValue> {
    let kind = if IMPACT_KINDS.contains(&kind) {
        kind
    } else {
        "unknown"
    };
    let level = if IMPACT_LEVELS.contains(&level) {
        level
    } else {
        "unknown"
    };
    let confidence = (confidence.clamp(0.0, 1.0) * 1000.0).round() / 1000.0;
    let mut result = JsonMap::new();
    result.insert("impact_kind".to_string(), json!(kind));
    result.insert("impact_level".to_string(), json!(level));
    result.insert("impact_confidence".to_string(), json!(confidence));
    result.insert(
        "impact_flags".to_string(),
        JsonValue::Array(flags.into_iter().map(JsonValue::String).collect()),
    );
    result
}

fn level_for_kind(
    kind: &str,
    lower_line: &str,
    confidence: f64,
    numeric_delta: Option<&BTreeMap<String, f64>>,
) -> &'static str {
    if confidence <= 0.0
        && !matches!(
            kind,
            "functional_change" | "bugfix" | "added" | "removed" | "rework"
        )
    {
        return "unknown";
    }
    if kind == "bugfix" {
        return if ["crash", "exploit"]
            .iter()
            .any(|word| lower_line.contains(word))
        {
            "medium"
        } else {
            "low"
        };
    }
    if HIGH_IMPACT_WORDS.iter().any(|word| lower_line.contains(word)) {
        return "high";
    }
    if matches!(kind, "rework" | "added" | "removed") {
        return "high";
    }
    if kind == "functional_change" {
        return "medium";
    }
    let Some(numeric_delta) = numeric_delta else {
        return "low";
    };
    let relative = numeric_delta.get("relative").copied().unwrap_or(0.0).abs();
    if relative >= 0.25 {
        "high"
    } else if relative >= 0.10 {
        "medium"
    } else {
        "low"
    }
}

fn non_numeric_level(kind: &str, lower_line: &str, confidence: f64) -> &'static str {
    level_for_kind(kind, lower_line, confidence, None)
}

fn numeric_delta(
    old_value: Option<&JsonValue>,
    new_value: Option<&JsonValue>,
) -> Option<BTreeMap<String, f64>> {
    let old_number = extract_number(old_value?)?;
    let new_number = extract_number(new_value?)?;
    let delta = new_number - old_number;
    let relative = if old_number == 0.0 {
        0.0
    } else {
        (delta / old_number).abs()
    };
    Some(BTreeMap::from([
        ("old".to_string(), old_number),
        ("new".to_string(), new_number),
        ("delta".to_string(), delta),
        ("relative".to_string(), relative),
    ]))
}

fn extract_number(value: &JsonValue) -> Option<f64> {
    let re = Regex::new(r"[-+]?\d+(?:\.\d+)?").ok()?;
    let text = value_to_string(Some(value));
    let matched = re.find(&text)?;
    matched.as_str().parse::<f64>().ok()
}

fn has_direction(flags: &[String], direction: &str) -> bool {
    let expected = format!("direction:{direction}");
    flags.iter().any(|flag| flag == &expected)
}

fn first_present(values: [Option<&JsonValue>; 2]) -> Option<JsonValue> {
    values
        .into_iter()
        .flatten()
        .find(|value| !value.is_null() && !value_to_string(Some(value)).is_empty())
        .cloned()
}

fn coerce_flags(value: Option<&JsonValue>) -> Vec<String> {
    match value {
        None | Some(JsonValue::Null) => Vec::new(),
        Some(JsonValue::Array(items)) => {
            let mut values = items.iter().map(|item| value_to_string(Some(item))).collect::<Vec<_>>();
            values.sort();
            values
        }
        Some(value) => vec![value_to_string(Some(value))],
    }
}

fn group_events_by_patch(events: &[JsonMap<String, JsonValue>], ascending: bool) -> Vec<JsonValue> {
    let mut grouped: BTreeMap<i64, JsonMap<String, JsonValue>> = BTreeMap::new();
    let mut order = Vec::new();
    for event in events {
        let patch_snapshot_id = event
            .get("patch_snapshot_id")
            .and_then(JsonValue::as_i64)
            .unwrap_or(0);
        grouped.entry(patch_snapshot_id).or_insert_with(|| {
            order.push(patch_snapshot_id);
            let mut patch = JsonMap::new();
            patch.insert("patch_snapshot_id".to_string(), json!(patch_snapshot_id));
            patch.insert(
                "patch_external_id".to_string(),
                event.get("patch_external_id").cloned().unwrap_or(JsonValue::Null),
            );
            patch.insert(
                "title".to_string(),
                event.get("patch_title").cloned().unwrap_or(JsonValue::Null),
            );
            patch.insert(
                "url".to_string(),
                event.get("patch_url").cloned().unwrap_or(JsonValue::Null),
            );
            patch.insert(
                "date".to_string(),
                event.get("posted_at").cloned().unwrap_or(JsonValue::Null),
            );
            patch.insert(
                "source".to_string(),
                event.get("source_kind").cloned().unwrap_or(JsonValue::Null),
            );
            patch.insert("events".to_string(), JsonValue::Array(Vec::new()));
            patch
        });
        if let Some(events) = grouped
            .get_mut(&patch_snapshot_id)
            .and_then(|patch| patch.get_mut("events"))
            .and_then(JsonValue::as_array_mut)
        {
            events.push(JsonValue::Object(event.clone()));
        }
    }
    if !ascending {
        order.reverse();
    }
    order
        .into_iter()
        .filter_map(|id| grouped.remove(&id).map(JsonValue::Object))
        .collect()
}

fn impact_summary(events: &[JsonMap<String, JsonValue>]) -> JsonValue {
    let mut by_kind: BTreeMap<String, i64> = BTreeMap::new();
    let mut by_level: BTreeMap<String, i64> = BTreeMap::new();
    for event in events {
        let kind = value_to_nonempty_string(event.get("impact_kind")).unwrap_or_else(|| "unknown".to_string());
        let level = value_to_nonempty_string(event.get("impact_level")).unwrap_or_else(|| "unknown".to_string());
        *by_kind.entry(kind).or_insert(0) += 1;
        *by_level.entry(level).or_insert(0) += 1;
    }
    json!({"by_kind": by_kind, "by_level": by_level})
}

fn build_lineage_summary(ctx: &JsonMap<String, JsonValue>) -> JsonValue {
    let rows = ctx
        .get("lineage")
        .map(|value| json_array_objects(Some(value)))
        .unwrap_or_default();
    let mut names = BTreeSet::new();
    let mut relation_counts: BTreeMap<String, i64> = BTreeMap::new();
    for row in &rows {
        let relation = value_to_nonempty_string(row.get("relation_type")).unwrap_or_else(|| "unknown".to_string());
        *relation_counts.entry(relation).or_insert(0) += 1;
        for key in ["source_name", "target_name", "owner_name"] {
            if let Some(value) = value_to_nonempty_string(row.get(key)) {
                names.insert(value);
            }
        }
    }
    json!({
        "available": !rows.is_empty(),
        "relations": rows.iter().take(12).cloned().collect::<Vec<_>>(),
        "related_names": names.into_iter().collect::<Vec<_>>(),
        "relation_counts": relation_counts,
        "omitted_relation_count": rows.len().saturating_sub(12),
    })
}

fn build_entity_summary(ctx: &JsonMap<String, JsonValue>) -> JsonValue {
    let best_match = ctx
        .get("best_match")
        .and_then(JsonValue::as_object)
        .cloned()
        .unwrap_or_default();
    let aliases = ctx
        .get("aliases")
        .map(|value| json_array_objects(Some(value)))
        .unwrap_or_default();
    let metadata = best_match
        .get("metadata")
        .and_then(JsonValue::as_object)
        .cloned()
        .unwrap_or_default();
    let mut human_aliases = Vec::new();
    for alias in aliases {
        let kind = value_to_string(alias.get("alias_kind"));
        let value = value_to_string(alias.get("alias")).trim().to_string();
        if !value.is_empty()
            && matches!(kind.as_str(), "canonical" | "snapshot_name" | "class_name_short")
            && !human_aliases.contains(&value)
        {
            human_aliases.push(value);
        }
    }
    human_aliases.truncate(12);
    json!({
        "matched": !best_match.is_empty(),
        "name": best_match.get("canonical_name").cloned().unwrap_or_else(|| ctx.get("query").cloned().unwrap_or(JsonValue::Null)),
        "entity_type": best_match.get("entity_type").cloned().unwrap_or(JsonValue::Null),
        "source": best_match.get("source").cloned().unwrap_or(JsonValue::Null),
        "external_id": best_match.get("primary_external_id").cloned().unwrap_or(JsonValue::Null),
        "match_score": best_match.get("score").cloned().unwrap_or(JsonValue::Null),
        "matched_alias_kinds": best_match.get("matched_alias_kinds").cloned().unwrap_or_else(|| json!([])),
        "aliases": human_aliases,
        "metadata_hints": {
            "disabled": metadata.get("disabled").cloned().unwrap_or(JsonValue::Null),
            "document_kinds": metadata.get("document_kinds").cloned().unwrap_or_else(|| json!([])),
        },
    })
}

fn build_current_stat_hints(ctx: &JsonMap<String, JsonValue>) -> JsonValue {
    let sheet_stats = ctx
        .get("sheet_stats")
        .and_then(JsonValue::as_object)
        .cloned()
        .unwrap_or_default();
    let sources = sheet_stats
        .get("sources")
        .map(|value| json_array_objects(Some(value)))
        .unwrap_or_default();
    let value_rows = sheet_rows_for_source(&sources, "hero_stat_values");
    let snapshot_rows = sheet_rows_for_source(&sources, "entity_snapshots");
    let profile_rows = sheet_rows_for_source(&sources, "hero_stat_profiles");
    let hints = merged_sheet_stat_hints(&value_rows, &snapshot_rows);
    json!({
        "available": sheet_stats.get("available").and_then(JsonValue::as_bool).unwrap_or(false),
        "source_count": sources.len(),
        "profile": profile_rows.first().map(compact_profile).unwrap_or(JsonValue::Null),
        "hints": hints.iter().take(18).cloned().collect::<Vec<_>>(),
        "omitted_hint_count": hints.len().saturating_sub(18),
        "source_tables": sources.iter().filter_map(|source| value_to_nonempty_string(source.get("source"))).collect::<Vec<_>>(),
    })
}

fn build_timeline_signals(
    events: &[JsonMap<String, JsonValue>],
    enrichments: &[JsonMap<String, JsonValue>],
) -> JsonValue {
    let enrichments_by_event_id = index_enrichments(enrichments);
    let mut change_type_counts = BTreeMap::new();
    let mut source_counts = BTreeMap::new();
    let mut section_counts = BTreeMap::new();
    let mut dates = Vec::new();
    let mut recent_events = Vec::new();
    let mut stat_changes = Vec::new();
    let mut ability_mentions: BTreeMap<String, i64> = BTreeMap::new();
    let mut low_confidence_events = 0;

    for event in events {
        let change_type = value_to_nonempty_string(event.get("change_type")).unwrap_or_else(|| "unknown".to_string());
        let source_kind = value_to_nonempty_string(event.get("source_kind")).unwrap_or_else(|| "unknown".to_string());
        let section = value_to_nonempty_string(event.get("section")).unwrap_or_else(|| "Unsectioned".to_string());
        *change_type_counts.entry(change_type).or_insert(0) += 1;
        *source_counts.entry(source_kind).or_insert(0) += 1;
        *section_counts.entry(section).or_insert(0) += 1;
        if let Some(date) = value_to_nonempty_string(event.get("posted_at")) {
            dates.push(date);
        }
        let event_id = event.get("id").and_then(JsonValue::as_i64);
        let mut event_enrichments = event_id
            .and_then(|id| enrichments_by_event_id.get(&id).cloned())
            .unwrap_or_default();
        if event_enrichments.is_empty() {
            event_enrichments.push(JsonMap::new());
        }
        if event_enrichments
            .iter()
            .any(|enrichment| safe_f64(enrichment.get("confidence"), 0.0) < 0.5)
        {
            low_confidence_events += 1;
        }
        recent_events.push(compact_event(event, &event_enrichments));
        for enrichment in &event_enrichments {
            if let Some(ability) = value_to_nonempty_string(enrichment.get("ability_name")) {
                *ability_mentions.entry(ability).or_insert(0) += 1;
            }
            if has_structured_stat_change(enrichment) {
                stat_changes.push(compact_stat_change(event, enrichment));
            }
        }
    }
    dates.sort();
    let ability_mentions = sort_count_map_limited(ability_mentions, 12);
    json!({
        "event_count": events.len(),
        "date_range": {
            "newest": dates.last().cloned(),
            "oldest": dates.first().cloned(),
        },
        "latest_patch": latest_patch(events),
        "change_type_counts": sort_count_map(change_type_counts),
        "source_counts": sort_count_map(source_counts),
        "top_sections": sort_count_map_limited(section_counts, 8),
        "recent_events": recent_events.iter().take(24).cloned().collect::<Vec<_>>(),
        "stat_changes": stat_changes.iter().take(24).cloned().collect::<Vec<_>>(),
        "omitted_recent_event_count": recent_events.len().saturating_sub(24),
        "omitted_stat_change_count": stat_changes.len().saturating_sub(24),
        "ability_mentions": ability_mentions,
        "low_confidence_event_count": low_confidence_events,
    })
}

fn build_source_references(
    ctx: &JsonMap<String, JsonValue>,
    current_stat_hints: &JsonValue,
) -> Vec<JsonValue> {
    let mut references = Vec::new();
    let mut seen = HashSet::new();
    let best_match = ctx
        .get("best_match")
        .and_then(JsonValue::as_object)
        .cloned()
        .unwrap_or_default();
    if !best_match.is_empty() {
        append_reference(
            &mut references,
            &mut seen,
            json!({
                "kind": "entity",
                "source": best_match.get("source").cloned().unwrap_or(JsonValue::Null),
                "label": best_match.get("canonical_name").cloned().unwrap_or(JsonValue::Null),
                "external_id": best_match.get("primary_external_id").cloned().unwrap_or(JsonValue::Null),
            }),
        );
    }
    for event in ctx
        .get("patch_events")
        .map(|value| json_array_objects(Some(value)))
        .unwrap_or_default()
    {
        append_reference(
            &mut references,
            &mut seen,
            json!({
                "kind": "patch_event",
                "source": event.get("source_kind").cloned().unwrap_or(JsonValue::Null),
                "label": event.get("patch_title").cloned().unwrap_or(JsonValue::Null),
                "url": event.get("patch_url").cloned().unwrap_or(JsonValue::Null),
                "posted_at": event.get("posted_at").cloned().unwrap_or(JsonValue::Null),
                "patch_event_id": event.get("id").cloned().unwrap_or(JsonValue::Null),
                "line_index": event.get("line_index").cloned().unwrap_or(JsonValue::Null),
            }),
        );
        if references.len() >= 40 {
            break;
        }
    }
    let profile = current_stat_hints
        .get("profile")
        .and_then(JsonValue::as_object)
        .cloned()
        .unwrap_or_default();
    if !profile.is_empty() {
        append_reference(
            &mut references,
            &mut seen,
            json!({
                "kind": "sheet_stats",
                "source": profile.get("source").cloned().unwrap_or(JsonValue::Null),
                "label": profile.get("hero_name").cloned().unwrap_or(JsonValue::Null),
                "external_id": profile.get("external_id").cloned().unwrap_or(JsonValue::Null),
                "snapshot_id": profile.get("snapshot_id").cloned().unwrap_or(JsonValue::Null),
                "row_number": profile.get("row_number").cloned().unwrap_or(JsonValue::Null),
            }),
        );
    }
    references.truncate(40);
    references
}

fn build_open_questions(
    ctx: &JsonMap<String, JsonValue>,
    entity_summary: &JsonValue,
    current_stat_hints: &JsonValue,
    timeline_signals: &JsonValue,
) -> Vec<String> {
    let mut questions = Vec::new();
    if !entity_summary
        .get("matched")
        .and_then(JsonValue::as_bool)
        .unwrap_or(false)
    {
        questions.push("Entity konnte nicht kanonisch gematcht werden; Patch-Treffer sind nur Fallback-Suche.".to_string());
    }
    if ctx
        .get("fallback")
        .and_then(JsonValue::as_object)
        .and_then(|value| value.get("used"))
        .and_then(JsonValue::as_bool)
        .unwrap_or(false)
    {
        questions.push("Retrieval nutzt Fallback-Matching; Namen und Aliase vor einer Review-Aussage pruefen.".to_string());
    }
    if timeline_signals
        .get("event_count")
        .and_then(JsonValue::as_u64)
        .unwrap_or(0)
        == 0
    {
        questions.push("Keine Patch-Events gefunden; Trend- oder Balance-Aussagen waeren spekulativ.".to_string());
    }
    if !ctx
        .get("enrichments")
        .and_then(JsonValue::as_object)
        .and_then(|value| value.get("available"))
        .and_then(JsonValue::as_bool)
        .unwrap_or(false)
    {
        questions.push("Patch-Event-Enrichments fehlen; Stat-Aenderungen sind nur aus Rohzeilen ableitbar.".to_string());
    } else if timeline_signals
        .get("low_confidence_event_count")
        .and_then(JsonValue::as_u64)
        .unwrap_or(0)
        > 0
    {
        questions.push("Ein Teil der Patchzeilen ist unstrukturiert oder low-confidence; Rohzeilen gegenlesen.".to_string());
    }
    if entity_summary
        .get("entity_type")
        .and_then(JsonValue::as_str)
        == Some("hero")
        && current_stat_hints
            .get("hints")
            .and_then(JsonValue::as_array)
            .map(Vec::is_empty)
            .unwrap_or(true)
    {
        questions.push("Keine aktuellen Sheet-Stat-Hints fuer den Hero gefunden.".to_string());
    }
    if current_stat_hints
        .get("omitted_hint_count")
        .and_then(JsonValue::as_u64)
        .unwrap_or(0)
        > 0
    {
        questions.push("Sheet-Stats wurden gekuerzt; fuer Detailanalyse ggf. Rohkontext nachladen.".to_string());
    }
    if timeline_signals
        .get("omitted_recent_event_count")
        .and_then(JsonValue::as_u64)
        .unwrap_or(0)
        > 0
        || timeline_signals
            .get("omitted_stat_change_count")
            .and_then(JsonValue::as_u64)
            .unwrap_or(0)
            > 0
    {
        questions.push("Timeline wurde gekuerzt; fuer historische Vollanalyse hoeheres limit_events nutzen.".to_string());
    }
    questions
}

fn build_prompt_de(best_match: &JsonMap<String, JsonValue>, query: &str) -> String {
    let name = value_to_nonempty_string(best_match.get("canonical_name")).unwrap_or_else(|| query.to_string());
    let entity_type = value_to_nonempty_string(best_match.get("entity_type")).unwrap_or_else(|| "Entity".to_string());
    format!(
        "Du bist ein Deadlock-Analyseassistent. Nutze ausschliesslich den bereitgestellten Review-Kontext und kennzeichne Unsicherheiten klar. Behalte Namen von Items, Heroes und Abilities exakt auf Englisch; erklaere Bewertung, Patch-Interpretation und offene Fragen auf Deutsch. Ziel: Erstelle eine kompakte Review fuer {name} ({entity_type}) mit aktueller Stat-Einordnung, relevanten Timeline-Signalen, moeglichen Balance- oder Build-Implikationen und Quellenhinweisen. Erfinde keine Zahlen oder Patchdetails, die nicht im Kontext stehen."
    )
}

fn resolve_ask_entity_match(
    conn: &Connection,
    query: &str,
    plan: &QueryPlan,
) -> Result<AskEntityClaimMatch> {
    let mut candidates = Vec::new();
    for entity in &plan.entities {
        if let Some(name) = value_to_nonempty_string(entity.get("name")) {
            candidates.push(name);
        } else if let Some(raw) = value_to_nonempty_string(entity.get("raw")) {
            candidates.push(raw);
        }
    }
    candidates.push(query.to_string());

    for candidate in candidates {
        let query_norm = normalize_alias(&candidate);
        let Some(best_match) = find_best_entity_match(conn, &candidate, &query_norm)? else {
            continue;
        };
        let mut names = BTreeSet::new();
        if let Some(name) = value_to_nonempty_string(best_match.get("canonical_name")) {
            names.insert(name);
        }
        if let Some(entity_id) = best_match.get("id").and_then(JsonValue::as_i64) {
            for alias in load_aliases(conn, entity_id, MAX_ALIASES)? {
                if let Some(value) = value_to_nonempty_string(alias.get("alias")) {
                    names.insert(value);
                }
                if let Some(value) = value_to_nonempty_string(alias.get("alias_norm")) {
                    names.insert(value);
                }
            }
        }
        return Ok(AskEntityClaimMatch {
            names: names.into_iter().collect(),
            matched: true,
        });
    }

    Ok(AskEntityClaimMatch::default())
}

fn ask_item_ground_truth(conn: &Connection, entity: &JsonValue) -> Result<JsonValue> {
    let entity_type = entity
        .get("entity_type")
        .and_then(JsonValue::as_str)
        .unwrap_or_default();
    if entity_type != "item" && entity_type != "item_special" {
        return Ok(JsonValue::Null);
    }
    let Some(name) = entity.get("name").and_then(JsonValue::as_str) else {
        return Ok(JsonValue::Null);
    };
    match build_item_context(conn, name) {
        Ok(value) => Ok(value),
        Err(RetrievalError::Invalid(_)) => Ok(JsonValue::Null),
        Err(error) => Err(error),
    }
}

fn load_ask_claims(
    conn: &Connection,
    query: &str,
    entity_names: &[String],
) -> Result<(Vec<AskClaimRecord>, usize, usize)> {
    if !tables_exist(conn, &["youtube_learning_claims", "youtube_videos"])? {
        return Ok((Vec::new(), 0, 0));
    }

    let mut by_id = BTreeMap::new();
    let entity_rows = load_entity_claim_rows(conn, entity_names)?;
    let entity_matched = entity_rows.len();
    merge_claim_rows(&mut by_id, entity_rows, "entity");

    let keywords = ask_query_keywords(query);
    let keyword_rows = load_keyword_claim_rows(conn, &keywords)?;
    let keyword_matched = keyword_rows.len();
    merge_claim_rows(&mut by_id, keyword_rows, "keyword");

    Ok((by_id.into_values().collect(), entity_matched, keyword_matched))
}

fn load_entity_claim_rows(
    conn: &Connection,
    entity_names: &[String],
) -> Result<Vec<JsonMap<String, JsonValue>>> {
    let names = entity_names
        .iter()
        .map(|name| name.trim().to_lowercase())
        .filter(|name| !name.is_empty())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    if names.is_empty() {
        return Ok(Vec::new());
    }
    let sql = format!(
        r#"
        SELECT c.id, c.video_id, c.entity_type, c.entity_name, c.claim_type,
               c.claim_text, c.evidence_quote, c.verifier_confidence, c.status,
               c.verifier_json, v.title AS source_video_title
        FROM youtube_learning_claims c
        LEFT JOIN youtube_videos v ON v.video_id=c.video_id
        WHERE lower(c.entity_name) IN ({})
        ORDER BY c.verifier_confidence DESC, c.id
        "#,
        placeholders(names.len())
    );
    fetch_all(
        conn,
        &sql,
        names.into_iter().map(SqlValue::Text).collect(),
    )
}

fn load_keyword_claim_rows(
    conn: &Connection,
    keywords: &[String],
) -> Result<Vec<JsonMap<String, JsonValue>>> {
    if keywords.is_empty() {
        return Ok(Vec::new());
    }
    let mut clauses = Vec::new();
    let mut values = Vec::new();
    for keyword in keywords {
        clauses.push("(lower(c.claim_text) LIKE ? OR lower(c.evidence_quote) LIKE ? OR lower(c.verifier_json) LIKE ?)".to_string());
        let pattern = format!("%{}%", keyword);
        values.push(SqlValue::Text(pattern.clone()));
        values.push(SqlValue::Text(pattern.clone()));
        values.push(SqlValue::Text(pattern));
    }
    let sql = format!(
        r#"
        SELECT c.id, c.video_id, c.entity_type, c.entity_name, c.claim_type,
               c.claim_text, c.evidence_quote, c.verifier_confidence, c.status,
               c.verifier_json, v.title AS source_video_title
        FROM youtube_learning_claims c
        LEFT JOIN youtube_videos v ON v.video_id=c.video_id
        WHERE {}
        ORDER BY c.verifier_confidence DESC, c.id
        "#,
        clauses.join(" OR ")
    );
    let rows = fetch_all(conn, &sql, values)?;
    Ok(rows
        .into_iter()
        .filter(|row| keyword_row_starts_word(row, keywords))
        .collect())
}

fn keyword_row_starts_word(row: &JsonMap<String, JsonValue>, keywords: &[String]) -> bool {
    let claim_text = value_to_string(row.get("claim_text")).to_lowercase();
    let evidence_quote = value_to_string(row.get("evidence_quote")).to_lowercase();
    let verifier_json = value_to_string(row.get("verifier_json")).to_lowercase();
    keywords.iter().any(|keyword| {
        keyword_starts_word(&claim_text, keyword)
            || keyword_starts_word(&evidence_quote, keyword)
            || keyword_starts_word(&verifier_json, keyword)
    })
}

fn keyword_starts_word(haystack_lower: &str, token_lower: &str) -> bool {
    if token_lower.is_empty() {
        return false;
    }
    haystack_lower.match_indices(token_lower).any(|(index, _)| {
        index == 0
            || haystack_lower[..index]
                .chars()
                .next_back()
                .is_some_and(|previous| !previous.is_ascii_alphanumeric())
    })
}

fn merge_claim_rows(
    by_id: &mut BTreeMap<i64, AskClaimRecord>,
    rows: Vec<JsonMap<String, JsonValue>>,
    match_source: &str,
) {
    for row in rows {
        let Some(mut claim) = ask_claim_from_row(row, match_source) else {
            continue;
        };
        if let Some(existing) = by_id.get_mut(&claim.id) {
            existing.match_sources.insert(match_source.to_string());
        } else {
            claim.match_sources.insert(match_source.to_string());
            by_id.insert(claim.id, claim);
        }
    }
}

fn ask_claim_from_row(
    mut row: JsonMap<String, JsonValue>,
    match_source: &str,
) -> Option<AskClaimRecord> {
    let id = row.get("id").and_then(JsonValue::as_i64)?;
    let verifier = loads_json_object(row.remove("verifier_json").as_ref());
    let mut match_sources = BTreeSet::new();
    match_sources.insert(match_source.to_string());
    Some(AskClaimRecord {
        id,
        claim_text: value_to_string(row.get("claim_text")),
        evidence_quote: value_to_string(row.get("evidence_quote")),
        claim_type: value_to_string(row.get("claim_type")),
        entity_name: value_to_string(row.get("entity_name")),
        status: value_to_string(row.get("status")).to_lowercase(),
        verifier_confidence: safe_f64(row.get("verifier_confidence"), 0.0),
        source_video: json!({
            "video_id": row.get("video_id").cloned().unwrap_or(JsonValue::Null),
            "title": row.get("source_video_title").cloned().unwrap_or(JsonValue::Null),
        }),
        verifier,
        match_sources,
    })
}

fn ask_query_keywords(query: &str) -> Vec<String> {
    let mut current = String::new();
    let mut tokens = BTreeSet::new();
    for character in query.to_lowercase().chars() {
        if character.is_ascii_alphanumeric() {
            current.push(character);
        } else {
            push_ask_keyword(&mut tokens, &mut current);
        }
    }
    push_ask_keyword(&mut tokens, &mut current);
    tokens.into_iter().take(12).collect()
}

fn push_ask_keyword(tokens: &mut BTreeSet<String>, current: &mut String) {
    if current.len() >= 4 && !CLAIM_KEYWORD_STOPWORDS.contains(&current.as_str()) {
        tokens.insert(current.clone());
    }
    current.clear();
}

fn partition_ask_claims(
    claims: Vec<AskClaimRecord>,
    include_unverified: bool,
    max_claims: usize,
) -> AskClaimBuckets {
    let mut all = AskClaimBuckets::default();
    for claim in claims {
        match claim.status.as_str() {
            "accepted" => all.verified.push(claim),
            "needs_review" => all.flagged.push(claim),
            "rejected" => all.refuted.push(claim),
            "unverified" => all.unverified.push(claim),
            _ => {}
        }
    }
    sort_ask_claims(&mut all.verified);
    sort_ask_claims(&mut all.flagged);
    sort_ask_claims(&mut all.refuted);
    sort_ask_claims(&mut all.unverified);

    let take = ask_claim_take_counts(&all, include_unverified, max_claims);
    let total_verified = all.verified.len();
    let total_flagged = all.flagged.len();
    let total_refuted = all.refuted.len();
    let total_unverified = all.unverified.len();
    AskClaimBuckets {
        verified: all.verified.into_iter().take(take.verified).collect(),
        flagged: all.flagged.into_iter().take(take.flagged).collect(),
        refuted: all.refuted.into_iter().take(take.refuted).collect(),
        unverified: if include_unverified {
            all.unverified.into_iter().take(take.unverified).collect()
        } else {
            Vec::new()
        },
        omitted: AskClaimOmitted {
            verified: total_verified.saturating_sub(take.verified),
            flagged: total_flagged.saturating_sub(take.flagged),
            refuted: total_refuted.saturating_sub(take.refuted),
            unverified: total_unverified.saturating_sub(take.unverified),
        },
    }
}

fn sort_ask_claims(claims: &mut [AskClaimRecord]) {
    claims.sort_by(|left, right| {
        right
            .verifier_confidence
            .total_cmp(&left.verifier_confidence)
            .then_with(|| left.id.cmp(&right.id))
    });
}

fn ask_claim_take_counts(
    claims: &AskClaimBuckets,
    include_unverified: bool,
    max_claims: usize,
) -> AskClaimOmitted {
    let mut take = AskClaimOmitted::default();
    let mut remaining = max_claims;
    if remaining == 0 {
        return take;
    }

    let reserve_flagged = usize::from(!claims.flagged.is_empty());
    let reserve_refuted = usize::from(!claims.refuted.is_empty());
    let reserve_unverified = usize::from(include_unverified && !claims.unverified.is_empty());
    let reserve = (reserve_flagged + reserve_refuted + reserve_unverified).min(remaining.saturating_sub(1));

    take.verified = claims.verified.len().min(remaining.saturating_sub(reserve));
    remaining = remaining.saturating_sub(take.verified);

    if reserve_flagged > 0 && remaining > 0 {
        take.flagged = 1;
        remaining -= 1;
    }
    if reserve_refuted > 0 && remaining > 0 {
        take.refuted = 1;
        remaining -= 1;
    }
    if reserve_unverified > 0 && remaining > 0 {
        take.unverified = 1;
        remaining -= 1;
    }

    while remaining > 0 {
        let before = remaining;
        if take.verified < claims.verified.len() {
            take.verified += 1;
            remaining -= 1;
        }
        if remaining > 0 && take.flagged < claims.flagged.len() {
            take.flagged += 1;
            remaining -= 1;
        }
        if remaining > 0 && take.refuted < claims.refuted.len() {
            take.refuted += 1;
            remaining -= 1;
        }
        if include_unverified && remaining > 0 && take.unverified < claims.unverified.len() {
            take.unverified += 1;
            remaining -= 1;
        }
        if remaining == before {
            break;
        }
    }
    take
}

fn claims_to_json(claims: &[AskClaimRecord]) -> JsonValue {
    JsonValue::Array(claims.iter().map(ask_claim_to_json).collect())
}

fn ask_claim_to_json(claim: &AskClaimRecord) -> JsonValue {
    json!({
        "claim_text": claim.claim_text,
        "evidence_quote": claim.evidence_quote,
        "claim_type": claim.claim_type,
        "entity_name": claim.entity_name,
        "status": claim.status,
        "verifier_confidence": claim.verifier_confidence,
        "source_video": claim.source_video,
        "verdict": claim.verifier.get("verdict").cloned().unwrap_or(JsonValue::Null),
        "db_evidence": claim.verifier.get("db_evidence").cloned().unwrap_or(JsonValue::Null),
        "db_value": claim.verifier.get("db_value").cloned().unwrap_or(JsonValue::Null),
        "match_sources": claim.match_sources.iter().cloned().collect::<Vec<_>>(),
    })
}

fn omitted_to_json(omitted: &AskClaimOmitted) -> JsonValue {
    json!({
        "verified": omitted.verified,
        "flagged": omitted.flagged,
        "refuted": omitted.refuted,
        "unverified": omitted.unverified,
    })
}

fn render_ask_prompt(bundle: &JsonValue) -> Result<String> {
    let ordered_context = json!({
        "ground_truth": bundle.get("ground_truth").cloned().unwrap_or(JsonValue::Null),
        "creator_knowledge": {
            "verified": bundle.pointer("/creator_knowledge/verified").cloned().unwrap_or_else(|| json!([])),
            "flagged": bundle.pointer("/creator_knowledge/flagged").cloned().unwrap_or_else(|| json!([])),
            "refuted": bundle.pointer("/creator_knowledge/refuted").cloned().unwrap_or_else(|| json!([])),
            "unverified": bundle.pointer("/creator_knowledge/unverified").cloned().unwrap_or_else(|| json!([])),
        },
        "query": bundle.get("query").cloned().unwrap_or(JsonValue::Null),
        "sources": bundle.get("sources").cloned().unwrap_or_else(|| json!([])),
    });
    let ordered_context_json = serde_json::to_string_pretty(&ordered_context)?;
    let mut rendered = ASK_PROMPT_TEMPLATE
        .replace("{{query}}", &value_to_string(bundle.get("query")))
        .replace("{{intent}}", &value_to_string(bundle.get("intent")))
        .replace("{{ordered_context_json}}", &ordered_context_json);
    if !rendered.contains(&ordered_context_json) {
        rendered.push_str("\n\n```json\n");
        rendered.push_str(&ordered_context_json);
        rendered.push_str("\n```");
    }
    Ok(rendered)
}

fn sheet_rows_for_source(
    sources: &[JsonMap<String, JsonValue>],
    source_name: &str,
) -> Vec<JsonMap<String, JsonValue>> {
    for source in sources {
        if source.get("source").and_then(JsonValue::as_str) == Some(source_name) {
            return source
                .get("rows")
                .map(|value| json_array_objects(Some(value)))
                .unwrap_or_default();
        }
    }
    Vec::new()
}

fn merged_sheet_stat_hints(
    value_rows: &[JsonMap<String, JsonValue>],
    snapshot_rows: &[JsonMap<String, JsonValue>],
) -> Vec<JsonValue> {
    let snapshot_hints = stat_hints_from_snapshot_rows(snapshot_rows);
    let normalized_hints = stat_hints_from_normalized_rows(value_rows);
    let mut by_key: BTreeMap<String, JsonMap<String, JsonValue>> = BTreeMap::new();
    for hint in snapshot_hints {
        if let Some(key) = value_to_nonempty_string(hint.get("stat_key")) {
            by_key.insert(key, hint);
        }
    }
    for hint in normalized_hints {
        let Some(key) = value_to_nonempty_string(hint.get("stat_key")) else {
            continue;
        };
        if let Some(existing) = by_key.get_mut(&key) {
            existing.insert(
                "numeric_value".to_string(),
                hint.get("numeric_value").cloned().unwrap_or(JsonValue::Null),
            );
            if existing
                .get("value")
                .map(|value| value.is_null() || value_to_string(Some(value)).is_empty())
                .unwrap_or(true)
            {
                existing.insert(
                    "value".to_string(),
                    hint.get("value").cloned().unwrap_or(JsonValue::Null),
                );
            }
        } else {
            by_key.insert(key, hint);
        }
    }
    let mut ordered_keys = PRIMARY_STAT_KEYS
        .iter()
        .filter(|key| by_key.contains_key(**key))
        .map(|key| (*key).to_string())
        .collect::<Vec<_>>();
    for key in by_key.keys() {
        if !ordered_keys.contains(key) {
            ordered_keys.push(key.clone());
        }
    }
    ordered_keys
        .into_iter()
        .filter_map(|key| by_key.remove(&key).map(JsonValue::Object))
        .collect()
}

fn stat_hints_from_normalized_rows(
    rows: &[JsonMap<String, JsonValue>],
) -> Vec<JsonMap<String, JsonValue>> {
    let mut by_key = BTreeMap::new();
    for row in rows {
        if let Some(key) = value_to_nonempty_string(row.get("stat_key")) {
            by_key.insert(key, row.clone());
        }
    }
    let mut ordered_keys = PRIMARY_STAT_KEYS
        .iter()
        .filter(|key| by_key.contains_key(**key))
        .map(|key| (*key).to_string())
        .collect::<Vec<_>>();
    for key in by_key.keys() {
        if !ordered_keys.contains(key) {
            ordered_keys.push(key.clone());
        }
    }
    ordered_keys
        .into_iter()
        .filter_map(|key| by_key.get(&key).map(compact_stat_value_map))
        .collect()
}

fn stat_hints_from_snapshot_rows(
    rows: &[JsonMap<String, JsonValue>],
) -> Vec<JsonMap<String, JsonValue>> {
    let Some(first) = rows.first() else {
        return Vec::new();
    };
    let values = first
        .get("values")
        .and_then(JsonValue::as_object)
        .cloned()
        .unwrap_or_default();
    let mut hints = Vec::new();
    for (label, raw_value) in values {
        let value = value_to_string(Some(&raw_value)).trim().to_string();
        let stat_key = stat_key(&label);
        if value.is_empty() || stat_key.is_empty() {
            continue;
        }
        let mut hint = JsonMap::new();
        hint.insert("stat_key".to_string(), json!(stat_key));
        hint.insert("stat_label".to_string(), json!(label));
        hint.insert("value".to_string(), json!(value));
        hint.insert("numeric_value".to_string(), float_or_null(&value));
        hints.push(hint);
    }
    hints
}

fn compact_profile(row: &JsonMap<String, JsonValue>) -> JsonValue {
    json!({
        "hero_name": row.get("hero_name").cloned().unwrap_or(JsonValue::Null),
        "source": row.get("source").cloned().unwrap_or(JsonValue::Null),
        "external_id": row.get("external_id").cloned().unwrap_or(JsonValue::Null),
        "snapshot_id": row.get("snapshot_id").cloned().unwrap_or(JsonValue::Null),
        "row_number": row.get("row_number").cloned().unwrap_or(JsonValue::Null),
    })
}

fn compact_stat_value_map(row: &JsonMap<String, JsonValue>) -> JsonMap<String, JsonValue> {
    let mut compact = JsonMap::new();
    compact.insert(
        "stat_key".to_string(),
        row.get("stat_key").cloned().unwrap_or(JsonValue::Null),
    );
    compact.insert(
        "stat_label".to_string(),
        row.get("stat_label").cloned().unwrap_or(JsonValue::Null),
    );
    compact.insert(
        "value".to_string(),
        row.get("raw_value").cloned().unwrap_or(JsonValue::Null),
    );
    compact.insert(
        "numeric_value".to_string(),
        row.get("numeric_value").cloned().unwrap_or(JsonValue::Null),
    );
    compact
}

fn stat_key(label: &str) -> String {
    let lowered = label.trim().to_lowercase();
    if lowered.is_empty() || lowered.starts_with("column ") {
        return String::new();
    }
    if lowered
        .chars()
        .all(|character| matches!(character, '|' | 'l' | 'i' | ' '))
    {
        return String::new();
    }
    let mut result = String::new();
    let mut last_was_underscore = false;
    for character in lowered.chars() {
        if character.is_alphanumeric() {
            result.push(character);
            last_was_underscore = false;
        } else if !last_was_underscore {
            result.push('_');
            last_was_underscore = true;
        }
    }
    result.trim_matches('_').to_string()
}

fn index_enrichments(
    enrichments: &[JsonMap<String, JsonValue>],
) -> BTreeMap<i64, Vec<JsonMap<String, JsonValue>>> {
    let mut indexed: BTreeMap<i64, Vec<JsonMap<String, JsonValue>>> = BTreeMap::new();
    for enrichment in enrichments {
        if let Some(id) = enrichment
            .get("patch_event_id")
            .or_else(|| enrichment.get("event_id"))
            .and_then(JsonValue::as_i64)
        {
            indexed.entry(id).or_default().push(enrichment.clone());
        }
    }
    indexed
}

fn compact_event(
    event: &JsonMap<String, JsonValue>,
    enrichments: &[JsonMap<String, JsonValue>],
) -> JsonValue {
    json!({
        "patch_event_id": event.get("id").cloned().unwrap_or(JsonValue::Null),
        "posted_at": event.get("posted_at").cloned().unwrap_or(JsonValue::Null),
        "patch_title": event.get("patch_title").cloned().unwrap_or(JsonValue::Null),
        "source_kind": event.get("source_kind").cloned().unwrap_or(JsonValue::Null),
        "section": event.get("section").cloned().unwrap_or(JsonValue::Null),
        "change_type": event.get("change_type").cloned().unwrap_or(JsonValue::Null),
        "line": event.get("normalized_line").cloned().or_else(|| event.get("raw_line").cloned()).unwrap_or(JsonValue::Null),
        "structured_changes": enrichments.iter().filter(|enrichment| !enrichment.is_empty()).map(compact_enrichment).collect::<Vec<_>>(),
    })
}

fn compact_enrichment(enrichment: &JsonMap<String, JsonValue>) -> JsonValue {
    json!({
        "stat_name": enrichment.get("stat_name").cloned().unwrap_or(JsonValue::Null),
        "old_value": enrichment.get("old_value").cloned().unwrap_or(JsonValue::Null),
        "new_value": enrichment.get("new_value").cloned().unwrap_or(JsonValue::Null),
        "unit": enrichment.get("unit").cloned().unwrap_or(JsonValue::Null),
        "ability_name": enrichment.get("ability_name").cloned().unwrap_or(JsonValue::Null),
        "secondary_entity_name": enrichment.get("secondary_entity_name").cloned().unwrap_or(JsonValue::Null),
        "confidence": enrichment.get("confidence").cloned().unwrap_or(JsonValue::Null),
    })
}

fn compact_stat_change(
    event: &JsonMap<String, JsonValue>,
    enrichment: &JsonMap<String, JsonValue>,
) -> JsonValue {
    let mut compact = compact_enrichment(enrichment)
        .as_object()
        .cloned()
        .unwrap_or_default();
    compact.insert(
        "patch_event_id".to_string(),
        event.get("id").cloned().unwrap_or(JsonValue::Null),
    );
    compact.insert(
        "posted_at".to_string(),
        event.get("posted_at").cloned().unwrap_or(JsonValue::Null),
    );
    compact.insert(
        "patch_title".to_string(),
        event.get("patch_title").cloned().unwrap_or(JsonValue::Null),
    );
    compact.insert(
        "change_type".to_string(),
        event.get("change_type").cloned().unwrap_or(JsonValue::Null),
    );
    compact.insert(
        "line".to_string(),
        event
            .get("normalized_line")
            .cloned()
            .or_else(|| event.get("raw_line").cloned())
            .unwrap_or(JsonValue::Null),
    );
    JsonValue::Object(compact)
}

fn has_structured_stat_change(enrichment: &JsonMap<String, JsonValue>) -> bool {
    value_to_nonempty_string(enrichment.get("stat_name")).is_some()
        && (enrichment.get("old_value").is_some_and(|value| !value.is_null())
            || enrichment.get("new_value").is_some_and(|value| !value.is_null()))
}

fn latest_patch(events: &[JsonMap<String, JsonValue>]) -> JsonValue {
    let Some(event) = events.first() else {
        return JsonValue::Null;
    };
    json!({
        "posted_at": event.get("posted_at").cloned().unwrap_or(JsonValue::Null),
        "patch_title": event.get("patch_title").cloned().unwrap_or(JsonValue::Null),
        "patch_url": event.get("patch_url").cloned().unwrap_or(JsonValue::Null),
        "source_kind": event.get("source_kind").cloned().unwrap_or(JsonValue::Null),
    })
}

fn append_reference(references: &mut Vec<JsonValue>, seen: &mut HashSet<String>, reference: JsonValue) {
    let Some(mut object) = reference.as_object().cloned() else {
        return;
    };
    object.retain(|_, value| !value.is_null());
    let kind = value_to_string(object.get("kind"));
    let identity_value = ["url", "external_id", "patch_event_id"]
        .into_iter()
        .find_map(|key| value_to_nonempty_string(object.get(key)))
        .unwrap_or_default();
    let identity = format!("{kind}\0{identity_value}");
    if seen.insert(identity) {
        references.push(JsonValue::Object(object));
    }
}

fn sort_count_map(map: BTreeMap<String, i64>) -> BTreeMap<String, i64> {
    let mut rows = map.into_iter().collect::<Vec<_>>();
    rows.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
    rows.into_iter().collect()
}

fn sort_count_map_limited(map: BTreeMap<String, i64>, limit: usize) -> BTreeMap<String, i64> {
    let mut rows = map.into_iter().collect::<Vec<_>>();
    rows.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
    rows.truncate(limit);
    rows.into_iter().collect()
}

fn check_required_tables(conn: &Connection) -> Result<JsonValue> {
    let required = [
        "source_documents",
        "entity_snapshots",
        "entities",
        "entity_aliases",
        "patch_events",
    ];
    let mut missing = Vec::new();
    for table in required {
        if !table_exists(conn, table)? {
            missing.push(table.to_string());
        }
    }
    if missing.is_empty() {
        Ok(result("ok", "required_tables", "Required normalized tables are present.", 0, Vec::new(), None))
    } else {
        Ok(result(
            "error",
            "required_tables",
            &format!("Missing required normalized tables: {}", missing.join(", ")),
            missing.len() as i64,
            missing.into_iter().map(|table| json!({"table": table})).collect(),
            None,
        ))
    }
}

fn check_entity_counts(conn: &Connection) -> Result<JsonValue> {
    if !tables_exist(conn, &["entity_snapshots", "entities"])? {
        return Ok(result(
            "error",
            "entity_counts",
            "Cannot check entity counts because required tables are missing.",
            0,
            Vec::new(),
            None,
        ));
    }
    let asset_snapshots = scalar_i64(
        conn,
        r#"
        SELECT COUNT(*)
        FROM entity_snapshots
        WHERE source=? AND entity_type IN ('hero', 'item_or_ability', 'rank')
        "#,
        vec![SqlValue::Text(ASSETS_SOURCE.to_string())],
    )?;
    let mut entity_counts = BTreeMap::new();
    for row in fetch_all(
        conn,
        r#"
        SELECT entity_type, COUNT(*) AS count
        FROM entities
        GROUP BY entity_type
        "#,
        vec![],
    )? {
        if let Some(entity_type) = value_to_nonempty_string(row.get("entity_type")) {
            entity_counts.insert(entity_type, row.get("count").and_then(JsonValue::as_i64).unwrap_or(0));
        }
    }
    let public_total = PUBLIC_ENTITY_TYPES
        .iter()
        .map(|entity_type| entity_counts.get(*entity_type).copied().unwrap_or(0))
        .sum::<i64>();
    let mut issues = Vec::new();
    if asset_snapshots == 0 {
        issues.push(json!({"metric": "asset_snapshots", "value": 0, "expected": "> 0"}));
    }
    if public_total == 0 {
        issues.push(json!({"metric": "public_entities", "value": 0, "expected": "> 0"}));
    }
    for (entity_type, minimum) in [("hero", 10), ("item", 20), ("ability", 20)] {
        let value = entity_counts.get(entity_type).copied().unwrap_or(0);
        if value < minimum {
            issues.push(json!({"metric": format!("entities.{entity_type}"), "value": value, "expected": format!(">= {minimum}")}));
        }
    }
    if asset_snapshots > 0 && public_total > asset_snapshots * 2 {
        issues.push(json!({
            "metric": "public_entities_to_asset_snapshots",
            "value": public_total,
            "expected": format!("not more than {}", asset_snapshots * 2),
            "asset_snapshots": asset_snapshots,
        }));
    }
    let severity = if issues.is_empty() { "ok" } else { "warning" };
    let message = if issues.is_empty() {
        "Entity counts look plausible."
    } else {
        "Entity counts are outside expected local snapshot bounds."
    };
    Ok(result(
        severity,
        "entity_counts",
        message,
        issues.len() as i64,
        issues,
        Some(json!({"asset_snapshots": asset_snapshots, "entity_counts": entity_counts})),
    ))
}

fn check_alias_collisions(conn: &Connection) -> Result<JsonValue> {
    if !tables_exist(conn, &["entity_aliases", "entities"])? {
        return Ok(result(
            "error",
            "alias_collisions",
            "Cannot check alias collisions because required tables are missing.",
            0,
            Vec::new(),
            None,
        ));
    }
    let rows = fetch_all(
        conn,
        r#"
        SELECT a.alias_norm, COUNT(DISTINCT a.entity_id) AS entities,
               GROUP_CONCAT(DISTINCT e.entity_type || ':' || e.canonical_name) AS targets,
               GROUP_CONCAT(DISTINCT a.alias) AS aliases
        FROM entity_aliases a
        JOIN entities e ON e.id=a.entity_id
        WHERE a.alias_norm <> ''
        GROUP BY a.alias_norm
        HAVING COUNT(DISTINCT a.entity_id) > 1
        ORDER BY entities DESC, a.alias_norm
        LIMIT ?
        "#,
        vec![SqlValue::Integer(SAMPLE_LIMIT)],
    )?;
    let total = scalar_i64(
        conn,
        r#"
        SELECT COUNT(*)
        FROM (
          SELECT alias_norm
          FROM entity_aliases
          WHERE alias_norm <> ''
          GROUP BY alias_norm
          HAVING COUNT(DISTINCT entity_id) > 1
        )
        "#,
        vec![],
    )?;
    Ok(result(
        if total > 0 { "warning" } else { "ok" },
        "alias_collisions",
        if total > 0 {
            "Alias collisions found across multiple entities."
        } else {
            "No alias collisions found."
        },
        total,
        rows.into_iter().map(JsonValue::Object).collect(),
        None,
    ))
}

fn check_patch_events_unknown_entities(conn: &Connection) -> Result<JsonValue> {
    if !tables_exist(conn, &["patch_events", "entities", "entity_aliases"])? {
        return Ok(result(
            "error",
            "patch_events_unknown_entities",
            "Cannot check patch events because required tables are missing.",
            0,
            Vec::new(),
            None,
        ));
    }
    let known = known_entity_names(conn)?;
    let mut unknown = Vec::new();
    let rows = fetch_all(
        conn,
        r#"
        SELECT id, patch_external_id, entity_type, entity_name, raw_line
        FROM patch_events
        WHERE entity_type <> 'general' AND entity_name IS NOT NULL AND TRIM(entity_name) <> ''
        ORDER BY id
        "#,
        vec![],
    )?;
    let mut total = 0;
    for row in rows {
        let entity_type = value_to_string(row.get("entity_type"));
        let entity_name = value_to_string(row.get("entity_name"));
        let key = (entity_type, normalize_alias(&entity_name));
        if !known.contains(&key) {
            total += 1;
            if unknown.len() < SAMPLE_LIMIT as usize {
                unknown.push(JsonValue::Object(row));
            }
        }
    }
    Ok(result(
        if total > 0 { "warning" } else { "ok" },
        "patch_events_unknown_entities",
        if total > 0 {
            "Patch events reference entities that are absent from normalized entities/aliases."
        } else {
            "Patch event entity references resolve to normalized entities or aliases."
        },
        total,
        unknown,
        None,
    ))
}

fn check_enrichment_table(conn: &Connection) -> Result<JsonValue> {
    if table_exists(conn, "patch_event_enrichments")? {
        Ok(result(
            "ok",
            "missing_enrichment_table",
            "patch_event_enrichments table is present.",
            0,
            Vec::new(),
            None,
        ))
    } else {
        Ok(result(
            "warning",
            "missing_enrichment_table",
            "Missing patch_event_enrichments table; enrichment quality cannot be evaluated.",
            0,
            Vec::new(),
            None,
        ))
    }
}

fn check_lineage_table(conn: &Connection) -> Result<JsonValue> {
    if !table_exists(conn, "entity_lineage")? {
        return Ok(result(
            "warning",
            "missing_lineage_table",
            "Missing entity_lineage table; renamed/reworked legacy names are not linked.",
            0,
            Vec::new(),
            None,
        ));
    }
    let total = scalar_i64(conn, "SELECT COUNT(*) FROM entity_lineage", vec![])?;
    let rows = fetch_all(
        conn,
        r#"
        SELECT relation_type, COUNT(*) AS count
        FROM entity_lineage
        GROUP BY relation_type
        ORDER BY relation_type
        "#,
        vec![],
    )?;
    Ok(result(
        if total == 0 { "warning" } else { "ok" },
        "lineage_table",
        if total > 0 {
            "entity_lineage is populated."
        } else {
            "entity_lineage exists but has no rows."
        },
        total,
        rows.into_iter().map(JsonValue::Object).collect(),
        None,
    ))
}

fn check_legacy_entities_table(conn: &Connection) -> Result<JsonValue> {
    if !table_exists(conn, "legacy_entities")? {
        return Ok(result(
            "warning",
            "missing_legacy_entities_table",
            "Missing legacy_entities table; old removed names outside lineage are not modeled.",
            0,
            Vec::new(),
            None,
        ));
    }
    let total = scalar_i64(conn, "SELECT COUNT(*) FROM legacy_entities", vec![])?;
    let suspect = scalar_i64(
        conn,
        "SELECT COUNT(*) FROM legacy_entities WHERE status='suspect_parser_subject'",
        vec![],
    )?;
    let samples = fetch_all(
        conn,
        r#"
        SELECT legacy_type, canonical_name, event_count, confidence, status
        FROM legacy_entities
        ORDER BY confidence DESC, event_count DESC, canonical_name
        LIMIT ?
        "#,
        vec![SqlValue::Integer(SAMPLE_LIMIT)],
    )?;
    let suspect_samples = fetch_all(
        conn,
        r#"
        SELECT legacy_type, canonical_name, event_count, confidence, status
        FROM legacy_entities
        WHERE status='suspect_parser_subject'
        ORDER BY canonical_name
        LIMIT ?
        "#,
        vec![SqlValue::Integer(SAMPLE_LIMIT)],
    )?;
    let severity = if suspect > 0 {
        "warning"
    } else if total > 0 {
        "ok"
    } else {
        "warning"
    };
    let message = if suspect > 0 {
        "legacy_entities has suspect parser subjects."
    } else if total > 0 {
        "legacy_entities is populated."
    } else {
        "legacy_entities exists but has no rows."
    };
    let sample_rows = if suspect_samples.is_empty() {
        samples
    } else {
        suspect_samples
    };
    Ok(result(
        severity,
        "legacy_entities_table",
        message,
        total,
        sample_rows.into_iter().map(JsonValue::Object).collect(),
        Some(json!({"suspect_parser_subjects": suspect})),
    ))
}

fn check_low_confidence_enrichments(conn: &Connection) -> Result<JsonValue> {
    if !table_exists(conn, "patch_event_enrichments")? {
        return Ok(result(
            "info",
            "low_confidence_unparsed_enrichments",
            "Skipped because patch_event_enrichments table is missing.",
            0,
            Vec::new(),
            None,
        ));
    }
    let rows = fetch_all(
        conn,
        r#"
        SELECT pee.patch_event_id, pee.confidence, pee.flags_json, pe.patch_external_id,
               pe.entity_type, pe.entity_name, pe.normalized_line
        FROM patch_event_enrichments pee
        LEFT JOIN patch_events pe ON pe.id=pee.patch_event_id
        WHERE pee.confidence < ? OR pee.flags_json LIKE '%unparsed%'
        ORDER BY pee.confidence ASC, pee.patch_event_id ASC
        LIMIT ?
        "#,
        vec![
            SqlValue::Real(LOW_CONFIDENCE_THRESHOLD),
            SqlValue::Integer(SAMPLE_LIMIT),
        ],
    )?;
    let total = scalar_i64(
        conn,
        r#"
        SELECT COUNT(*)
        FROM patch_event_enrichments
        WHERE confidence < ? OR flags_json LIKE '%unparsed%'
        "#,
        vec![SqlValue::Real(LOW_CONFIDENCE_THRESHOLD)],
    )?;
    Ok(result(
        if total > 0 { "warning" } else { "ok" },
        "low_confidence_unparsed_enrichments",
        if total > 0 {
            "Low-confidence or unparsed patch event enrichments found."
        } else {
            "No low-confidence or unparsed enrichments found."
        },
        total,
        rows.into_iter().map(JsonValue::Object).collect(),
        Some(json!({"confidence_threshold": LOW_CONFIDENCE_THRESHOLD})),
    ))
}

fn check_sheet_profiles_without_entity(conn: &Connection) -> Result<JsonValue> {
    if !table_exists(conn, "hero_stat_profiles")? {
        return Ok(result(
            "info",
            "sheet_profiles_without_entity",
            "Skipped because hero_stat_profiles table is missing.",
            0,
            Vec::new(),
            None,
        ));
    }
    let rows = fetch_all(
        conn,
        r#"
        SELECT id, snapshot_id, hero_name, source, external_id, row_number
        FROM hero_stat_profiles
        WHERE entity_id IS NULL
        ORDER BY hero_name
        LIMIT ?
        "#,
        vec![SqlValue::Integer(SAMPLE_LIMIT)],
    )?;
    let total = scalar_i64(
        conn,
        "SELECT COUNT(*) FROM hero_stat_profiles WHERE entity_id IS NULL",
        vec![],
    )?;
    Ok(result(
        if total > 0 { "warning" } else { "ok" },
        "sheet_profiles_without_entity",
        if total > 0 {
            "Sheet profiles without normalized hero entity found."
        } else {
            "All sheet profiles are linked to a normalized hero entity."
        },
        total,
        rows.into_iter().map(JsonValue::Object).collect(),
        None,
    ))
}

fn check_general_events_with_known_entity_names(conn: &Connection) -> Result<JsonValue> {
    if !tables_exist(conn, &["patch_events", "entities", "entity_aliases"])? {
        return Ok(result(
            "error",
            "general_events_with_known_entity_names",
            "Cannot scan general events because required tables are missing.",
            0,
            Vec::new(),
            None,
        ));
    }
    let scan_names = entity_scan_names(conn)?;
    let mut samples = Vec::new();
    let mut total = 0;
    let rows = fetch_all(
        conn,
        r#"
        SELECT id, patch_external_id, section, raw_line, normalized_line
        FROM patch_events
        WHERE entity_type='general'
        ORDER BY id
        "#,
        vec![],
    )?;
    for mut row in rows {
        let text_key = format!(
            " {} ",
            normalize_key(&format!(
                "{} {}",
                value_to_string(row.get("raw_line")),
                value_to_string(row.get("normalized_line"))
            ))
        );
        let matches = scan_names
            .iter()
            .filter(|scan| text_key.contains(&format!(" {} ", scan.name_key)))
            .map(|scan| json!({"entity_type": scan.entity_type, "entity_name": scan.entity_name, "matched_name": scan.name}))
            .collect::<Vec<_>>();
        if matches.is_empty() {
            continue;
        }
        total += 1;
        if samples.len() < GENERAL_ENTITY_SCAN_LIMIT {
            row.insert("matches".to_string(), JsonValue::Array(matches.into_iter().take(5).collect()));
            samples.push(JsonValue::Object(row));
        }
    }
    samples.truncate(SAMPLE_LIMIT as usize);
    Ok(result(
        if total > 0 { "warning" } else { "ok" },
        "general_events_with_known_entity_names",
        if total > 0 {
            "General patch events mention known local API entities and may be misclassified."
        } else {
            "No general patch events with known local API entity names found."
        },
        total,
        samples,
        None,
    ))
}

#[derive(Debug, Clone)]
struct ScanName {
    name_key: String,
    entity_type: String,
    entity_name: String,
    name: String,
}

fn known_entity_names(conn: &Connection) -> Result<HashSet<(String, String)>> {
    let mut known = HashSet::new();
    for row in fetch_all(conn, "SELECT entity_type, canonical_name FROM entities", vec![])? {
        known.insert((
            value_to_string(row.get("entity_type")),
            normalize_alias(&value_to_string(row.get("canonical_name"))),
        ));
    }
    for row in fetch_all(
        conn,
        r#"
        SELECT e.entity_type, a.alias_norm
        FROM entity_aliases a
        JOIN entities e ON e.id=a.entity_id
        "#,
        vec![],
    )? {
        known.insert((
            value_to_string(row.get("entity_type")),
            value_to_string(row.get("alias_norm")),
        ));
    }
    if table_exists(conn, "entity_lineage")? {
        for row in fetch_all(
            conn,
            "SELECT source_entity_type, source_name_norm, target_entity_type, target_name_norm FROM entity_lineage",
            vec![],
        )? {
            add_lineage_known_name(&mut known, row.get("source_entity_type"), row.get("source_name_norm"));
            add_lineage_known_name(&mut known, row.get("target_entity_type"), row.get("target_name_norm"));
        }
    }
    if table_exists(conn, "legacy_entities")? {
        for row in fetch_all(
            conn,
            "SELECT observed_entity_type, name_norm FROM legacy_entities",
            vec![],
        )? {
            known.insert((
                value_to_string(row.get("observed_entity_type")),
                value_to_string(row.get("name_norm")),
            ));
        }
    }
    Ok(known)
}

fn add_lineage_known_name(
    known: &mut HashSet<(String, String)>,
    entity_type: Option<&JsonValue>,
    name_norm: Option<&JsonValue>,
) {
    let name = value_to_string(name_norm).trim().to_string();
    if name.is_empty() {
        return;
    }
    if let Some(entity_type) = value_to_nonempty_string(entity_type) {
        known.insert((entity_type, name));
    } else {
        for fallback_type in ["item", "item_special", "ability", "hero"] {
            known.insert((fallback_type.to_string(), name.clone()));
        }
    }
}

fn entity_scan_names(conn: &Connection) -> Result<Vec<ScanName>> {
    let mut by_key: BTreeMap<(String, String, String), String> = BTreeMap::new();
    for row in fetch_all(
        conn,
        r#"
        SELECT entity_type, canonical_name
        FROM entities
        WHERE source=? AND entity_type IN ('hero', 'item', 'item_special', 'ability')
        "#,
        vec![SqlValue::Text(ASSETS_SOURCE.to_string())],
    )? {
        let entity_type = value_to_string(row.get("entity_type"));
        let canonical_name = value_to_string(row.get("canonical_name"));
        add_scan_name(&mut by_key, &entity_type, &canonical_name, &canonical_name);
    }
    for row in fetch_all(
        conn,
        r#"
        SELECT e.entity_type, e.canonical_name, a.alias
        FROM entity_aliases a
        JOIN entities e ON e.id=a.entity_id
        WHERE a.source=? AND e.entity_type IN ('hero', 'item', 'item_special', 'ability')
        "#,
        vec![SqlValue::Text(ASSETS_SOURCE.to_string())],
    )? {
        add_scan_name(
            &mut by_key,
            &value_to_string(row.get("entity_type")),
            &value_to_string(row.get("canonical_name")),
            &value_to_string(row.get("alias")),
        );
    }
    if table_exists(conn, "entity_lineage")? {
        for row in fetch_all(
            conn,
            r#"
            SELECT source_entity_type, source_name, target_entity_type, target_name
            FROM entity_lineage
            WHERE relation_type IN ('rename', 'replaced_by')
            "#,
            vec![],
        )? {
            let source_type = value_to_nonempty_string(row.get("source_entity_type")).unwrap_or_else(|| "legacy".to_string());
            let source_name = value_to_string(row.get("source_name"));
            add_scan_name(&mut by_key, &source_type, &source_name, &source_name);
            let target_type = value_to_nonempty_string(row.get("target_entity_type")).unwrap_or_else(|| "legacy".to_string());
            let target_name = value_to_string(row.get("target_name"));
            add_scan_name(&mut by_key, &target_type, &target_name, &target_name);
        }
    }
    if table_exists(conn, "legacy_entities")? {
        for row in fetch_all(
            conn,
            r#"
            SELECT legacy_type, canonical_name
            FROM legacy_entities
            WHERE confidence >= 0.5
            "#,
            vec![],
        )? {
            let legacy_type = value_to_string(row.get("legacy_type"));
            let canonical_name = value_to_string(row.get("canonical_name"));
            add_scan_name(&mut by_key, &legacy_type, &canonical_name, &canonical_name);
        }
    }
    let mut rows = by_key
        .into_iter()
        .map(|((name_key, entity_type, entity_name), name)| ScanName {
            name_key,
            entity_type,
            entity_name,
            name,
        })
        .collect::<Vec<_>>();
    rows.sort_by_key(|row| std::cmp::Reverse(row.name_key.len()));
    Ok(rows)
}

fn add_scan_name(
    by_key: &mut BTreeMap<(String, String, String), String>,
    entity_type: &str,
    entity_name: &str,
    name: &str,
) {
    let name_key = normalize_key(name);
    if name_key.len() < 4 || name_key.chars().all(|ch| ch.is_ascii_digit()) {
        return;
    }
    if matches!(name_key.as_str(), "hero" | "item" | "ability" | "weapon" | "melee") {
        return;
    }
    by_key
        .entry((name_key, entity_type.to_string(), entity_name.to_string()))
        .or_insert_with(|| name.to_string());
}

fn result(
    severity: &str,
    check: &str,
    message: &str,
    rows: i64,
    samples: Vec<JsonValue>,
    details: Option<JsonValue>,
) -> JsonValue {
    let mut result = JsonMap::new();
    result.insert("severity".to_string(), json!(severity));
    result.insert("check".to_string(), json!(check));
    result.insert("message".to_string(), json!(message));
    result.insert("rows".to_string(), json!(rows));
    result.insert("samples".to_string(), JsonValue::Array(samples));
    if let Some(details) = details {
        result.insert("details".to_string(), details);
    }
    JsonValue::Object(result)
}

fn severity_rank(severity: &str) -> i64 {
    match severity {
        "info" => 1,
        "warning" => 2,
        "error" => 3,
        _ => 0,
    }
}

fn load_entity_payload(
    conn: &Connection,
    query: &str,
    entity_type: &str,
) -> Result<Option<JsonMap<String, JsonValue>>> {
    let context = build_entity_context(conn, query, 1)?;
    let best = context
        .get("best_match")
        .and_then(JsonValue::as_object)
        .cloned()
        .unwrap_or_default();
    let best_type = value_to_string(best.get("entity_type"));
    if best_type != entity_type && !(entity_type == "item" && best_type == "item_special") {
        return Ok(None);
    }
    let Some(entity_id) = best.get("id").and_then(JsonValue::as_i64) else {
        return Ok(None);
    };
    let payload_json: Option<String> = conn
        .query_row(
            r#"
            SELECT s.payload_json
            FROM entity_aliases a
            JOIN entity_snapshots s ON s.id=a.snapshot_id
            WHERE a.entity_id=? AND s.source='deadlock_assets_api'
            ORDER BY CASE s.entity_type WHEN 'hero' THEN 0 WHEN 'item_or_ability' THEN 0 ELSE 1 END, s.id
            LIMIT 1
            "#,
            params![entity_id],
            |row| row.get(0),
        )
        .optional()?;
    Ok(payload_json
        .as_deref()
        .map(|text| loads_json_object(Some(&JsonValue::String(text.to_string())))))
}

fn item_summary(payload: &JsonMap<String, JsonValue>) -> JsonMap<String, JsonValue> {
    let mut item = JsonMap::new();
    item.insert(
        "name".to_string(),
        payload
            .get("name")
            .cloned()
            .or_else(|| payload.get("class_name").cloned())
            .unwrap_or(JsonValue::Null),
    );
    item.insert(
        "class_name".to_string(),
        payload.get("class_name").cloned().unwrap_or(JsonValue::Null),
    );
    item.insert(
        "slot".to_string(),
        payload.get("item_slot_type").cloned().unwrap_or(JsonValue::Null),
    );
    item.insert("tier".to_string(), json!(int_or_zero(payload.get("item_tier"))));
    item.insert("cost".to_string(), json!(int_or_zero(payload.get("cost"))));
    item.insert(
        "is_active".to_string(),
        json!(payload.get("is_active_item").and_then(JsonValue::as_bool).unwrap_or(false)),
    );
    item.insert(
        "activation".to_string(),
        payload.get("activation").cloned().unwrap_or(JsonValue::Null),
    );
    item.insert(
        "description".to_string(),
        json!(clean_html_text(&nested_desc(payload.get("description")))),
    );
    item.insert(
        "component_items".to_string(),
        payload.get("component_items").cloned().unwrap_or_else(|| json!([])),
    );
    item.insert(
        "properties".to_string(),
        JsonValue::Array(
            compact_properties(payload.get("properties"))
                .into_iter()
                .map(JsonValue::Object)
                .collect(),
        ),
    );
    item.insert(
        "upgrades".to_string(),
        payload.get("upgrades").cloned().unwrap_or_else(|| json!([])),
    );
    let archetypes = classify_item_archetypes(&item);
    item.insert(
        "archetypes".to_string(),
        JsonValue::Array(archetypes.into_iter().map(JsonValue::String).collect()),
    );
    item
}

fn classify_item_archetypes(item: &JsonMap<String, JsonValue>) -> Vec<String> {
    let desc = value_to_string(item.get("description"));
    let props = item
        .get("properties")
        .and_then(JsonValue::as_array)
        .cloned()
        .unwrap_or_default();
    let mut blob_parts = vec![value_to_string(item.get("name")), desc];
    for prop in props {
        if let Some(prop) = prop.as_object() {
            blob_parts.push(format!(
                "{} {} {}",
                value_to_string(prop.get("name")),
                value_to_string(prop.get("label")),
                value_to_string(prop.get("provided_property_type"))
            ));
        }
    }
    let blob = blob_parts.join(" ").to_lowercase();
    let mut archetypes = BTreeSet::new();
    let cost = int_or_zero(item.get("cost"));
    if cost >= 6400 {
        archetypes.insert("luxury");
    }
    if item.get("is_active").and_then(JsonValue::as_bool).unwrap_or(false) {
        archetypes.insert("active_burden");
    }
    add_archetype_if(&mut archetypes, &blob, &["npc", "nonplayer", "non-player", "trooper", "bonus souls", "souls", "creep"], "lane_farm");
    add_archetype_if(&mut archetypes, &blob, &["npc damage", "trooper", "non-player", "nonplayer", "creep", "chain", "ricochet"], "waveclear");
    add_archetype_if(&mut archetypes, &blob, &["bonus souls", "secure", "claim", "confirm", "last hit", "orb"], "orb_secure");
    add_archetype_if(&mut archetypes, &blob, &["close range", "weapon damage", "fire rate", "max ammo", "reload", "bonus damage", "current health damage", "bullet lifesteal", "out of combat regen"], "lane_trade");
    add_archetype_if(&mut archetypes, &blob, &["close range", "bullet lifesteal", "melee", "duel", "weapon damage", "fire rate", "slow resist"], "duel");
    add_archetype_if(&mut archetypes, &blob, &["burst", "bonus damage", "damage amp", "amplification", "current health damage", "execute", "crit"], "burst");
    add_archetype_if(&mut archetypes, &blob, &["fire rate", "weapon damage", "max ammo", "reload", "bullet procs", "ricochet", "sustained"], "sustained_dps");
    add_archetype_if(&mut archetypes, &blob, &["slow", "stun", "silence", "disarm", "root", "immobil", "knock", "teleport", "dash distance", "gravity"], "kill_setup");
    add_archetype_if(&mut archetypes, &blob, &["teleport", "dash", "move speed", "sprint speed", "stamina", "escape", "barrier"], "escape");
    add_archetype_if(&mut archetypes, &blob, &["stun", "silence", "disarm", "knock", "hex", "curse", "area", "nearby enemies", "radius"], "teamfight_engage");
    add_archetype_if(&mut archetypes, &blob, &["debuff", "cleanse", "dispel", "unstoppable", "immune", "barrier", "shield", "return fire", "healing reduction", "metal skin"], "counter");
    add_archetype_if(&mut archetypes, &blob, &["healing reduction", "anti-heal", "healbane"], "anti_heal");
    add_archetype_if(&mut archetypes, &blob, &["disarm", "return fire", "metal skin", "bullet resist", "weapon damage reduction", "fire rate slow"], "anti_carry");
    add_archetype_if(&mut archetypes, &blob, &["rescue", "barrier", "shield", "cleanse", "dispel", "heal yourself and nearby allies"], "save");
    add_archetype_if(&mut archetypes, &blob, &["stack", "escalat", "amp", "cooldown", "duration", "ability range", "radius", "spirit power", "techpower", "charges", "ricochet", "bullet procs", "max weapon damage"], "core_scaling");
    add_archetype_if(&mut archetypes, &blob, &["nearby allies", "friendly", "ally", "aura", "rescue", "heal yourself and nearby allies", "healing output"], "support");
    add_archetype_if(&mut archetypes, &blob, &["guardian", "walker", "patron", "mid boss", "midboss", "objective", "non-player", "nonplayer", "npc damage"], "objective_damage");
    add_archetype_if(&mut archetypes, &blob, &["split push", "splitpush", "lane pressure", "trooper", "wave"], "splitpush");
    archetypes.into_iter().map(str::to_string).collect()
}

fn add_archetype_if<'a>(
    archetypes: &mut BTreeSet<&'a str>,
    blob: &str,
    needles: &[&str],
    archetype: &'a str,
) {
    if needles.iter().any(|needle| blob.contains(needle)) {
        archetypes.insert(archetype);
    }
}

fn compact_properties(value: Option<&JsonValue>) -> Vec<JsonMap<String, JsonValue>> {
    let Some(properties) = value.and_then(JsonValue::as_object) else {
        return Vec::new();
    };
    let mut rows = Vec::new();
    for (name, prop) in properties {
        let Some(prop) = prop.as_object() else {
            continue;
        };
        let value = prop.get("value").cloned().unwrap_or(JsonValue::Null);
        let disable = prop.get("disable_value").cloned().unwrap_or(JsonValue::Null);
        if matches!(value_to_string(Some(&value)).as_str(), "" | "0" | "0.0" | "-1.0")
            && matches!(value_to_string(Some(&disable)).as_str(), "0" | "-1" | "-2")
        {
            continue;
        }
        let mut row = JsonMap::new();
        row.insert("name".to_string(), json!(name));
        row.insert(
            "label".to_string(),
            prop.get("label")
                .cloned()
                .or_else(|| prop.get("postvalue_label").cloned())
                .unwrap_or_else(|| json!(name)),
        );
        row.insert("value".to_string(), value);
        row.insert(
            "prefix".to_string(),
            prop.get("prefix").cloned().unwrap_or(JsonValue::Null),
        );
        row.insert(
            "postfix".to_string(),
            prop.get("postfix").cloned().unwrap_or(JsonValue::Null),
        );
        row.insert(
            "provided_property_type".to_string(),
            prop.get("provided_property_type")
                .cloned()
                .unwrap_or(JsonValue::Null),
        );
        row.insert(
            "tooltip_section".to_string(),
            prop.get("tooltip_section").cloned().unwrap_or(JsonValue::Null),
        );
        row.insert(
            "important".to_string(),
            json!(prop
                .get("tooltip_is_important")
                .and_then(JsonValue::as_bool)
                .unwrap_or(false)),
        );
        row.insert(
            "elevated".to_string(),
            json!(prop
                .get("tooltip_is_elevated")
                .and_then(JsonValue::as_bool)
                .unwrap_or(false)),
        );
        row.insert(
            "scales_with".to_string(),
            JsonValue::Array(
                scale_hint(prop.get("scale_function"))
                    .into_iter()
                    .map(JsonValue::String)
                    .collect(),
            ),
        );
        rows.push(row);
    }
    rows
}

fn scale_hint(value: Option<&JsonValue>) -> Vec<String> {
    let Some(scale_function) = value.and_then(JsonValue::as_object) else {
        return Vec::new();
    };
    if let Some(stats) = scale_function.get("scaling_stats").and_then(JsonValue::as_array) {
        return stats.iter().map(|stat| value_to_string(Some(stat))).collect();
    }
    value_to_nonempty_string(scale_function.get("specific_stat_scale_type"))
        .into_iter()
        .collect()
}

fn nested_desc(value: Option<&JsonValue>) -> String {
    match value {
        Some(JsonValue::Object(object)) => value_to_string(object.get("desc")),
        Some(value) => value_to_string(Some(value)),
        None => String::new(),
    }
}

fn clean_html_text(value: &str) -> String {
    let without_svg = Regex::new(r"(?is)<svg\b.*?</svg>")
        .map(|re| re.replace_all(value, " ").into_owned())
        .unwrap_or_else(|_| value.to_string());
    let without_tags = Regex::new(r"(?is)<[^>]+>")
        .map(|re| re.replace_all(&without_svg, " ").into_owned())
        .unwrap_or(without_svg);
    collapse_whitespace(&html_unescape_minimal(&without_tags))
}

fn html_unescape_minimal(value: &str) -> String {
    value
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
}

fn save_review_analysis_note(
    conn: &Connection,
    review_context: &JsonValue,
    result_text: Option<&str>,
    model: Option<&str>,
    confidence: Option<f64>,
    status: &str,
    provider_metadata: Option<&JsonValue>,
) -> Result<JsonValue> {
    let query = review_context
        .get("query")
        .and_then(JsonValue::as_str)
        .unwrap_or_default()
        .trim()
        .to_string();
    let entity_summary = review_context
        .get("entity_summary")
        .and_then(JsonValue::as_object)
        .cloned()
        .unwrap_or_default();
    let context_json = serde_json::to_string(review_context)?;
    let context_hash = stable_hash_text(&context_json);
    let now = core::db::now_epoch_seconds()?;
    let source_references = source_references(review_context, provider_metadata);
    let source_references_json = serde_json::to_string(&source_references)?;
    let entity_type = value_to_nonempty_string(entity_summary.get("entity_type"));
    let entity_name = value_to_nonempty_string(entity_summary.get("name"));
    let context_kind = review_context
        .get("context_kind")
        .and_then(JsonValue::as_str)
        .unwrap_or("analysis_review")
        .to_string();
    let prompt_text = review_context
        .get("prompt_de")
        .and_then(JsonValue::as_str)
        .unwrap_or_default()
        .to_string();
    conn.execute(
        r#"
        INSERT INTO analysis_notes(
          query, entity_type, entity_name, context_kind, context_hash,
          prompt_version, prompt_text, result_text, model, confidence, status,
          source_references_json, context_json, created_at, updated_at
        )
        VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)
        ON CONFLICT DO UPDATE SET
          prompt_text=excluded.prompt_text,
          result_text=excluded.result_text,
          confidence=excluded.confidence,
          source_references_json=excluded.source_references_json,
          context_json=excluded.context_json,
          updated_at=excluded.updated_at
        "#,
        params![
            query,
            entity_type,
            entity_name,
            context_kind,
            context_hash,
            PROMPT_VERSION,
            prompt_text,
            result_text,
            model,
            confidence,
            status,
            source_references_json,
            context_json,
            now,
            now,
        ],
    )?;
    let row = fetch_one(
        conn,
        r#"
        SELECT id, query, entity_type, entity_name, context_hash, prompt_version,
               model, status, created_at, updated_at
        FROM analysis_notes
        WHERE query=? AND context_hash=? AND prompt_version=? AND COALESCE(model, '')=COALESCE(?, '') AND status=?
        ORDER BY id DESC
        LIMIT 1
        "#,
        vec![
            SqlValue::Text(query.clone()),
            SqlValue::Text(context_hash.clone()),
            SqlValue::Text(PROMPT_VERSION.to_string()),
            model.map(|value| SqlValue::Text(value.to_string())).unwrap_or(SqlValue::Null),
            SqlValue::Text(status.to_string()),
        ],
    )?;
    Ok(row
        .map(JsonValue::Object)
        .unwrap_or_else(|| json!({"query": query, "context_hash": context_hash, "status": status})))
}

fn source_references(review_context: &JsonValue, provider_metadata: Option<&JsonValue>) -> Vec<JsonValue> {
    let mut references = review_context
        .get("source_references")
        .and_then(JsonValue::as_array)
        .cloned()
        .unwrap_or_default();
    if let Some(provider_metadata) = provider_metadata {
        let mut model_call = JsonMap::new();
        model_call.insert("kind".to_string(), json!("model_call"));
        if let Some(object) = provider_metadata.as_object() {
            for (key, value) in object {
                model_call.insert(key.clone(), value.clone());
            }
        }
        references.push(JsonValue::Object(model_call));
    }
    references
}

fn compact_context_for_model(review_context: &JsonValue) -> JsonValue {
    let mut timeline = review_context
        .get("timeline_signals")
        .and_then(JsonValue::as_object)
        .cloned()
        .unwrap_or_default();
    truncate_array_field(&mut timeline, "recent_events", 18);
    truncate_array_field(&mut timeline, "stat_changes", 18);
    json!({
        "query": review_context.get("query").cloned().unwrap_or(JsonValue::Null),
        "entity_summary": review_context.get("entity_summary").cloned().unwrap_or(JsonValue::Null),
        "lineage": review_context.get("lineage").cloned().unwrap_or(JsonValue::Null),
        "current_stat_hints": review_context.get("current_stat_hints").cloned().unwrap_or(JsonValue::Null),
        "timeline_signals": timeline,
        "open_questions": review_context.get("open_questions").cloned().unwrap_or(JsonValue::Null),
        "source_references": review_context.get("source_references").and_then(JsonValue::as_array).map(|items| JsonValue::Array(items.iter().take(30).cloned().collect())).unwrap_or_else(|| json!([])),
        "retrieval_meta": review_context.get("retrieval_meta").cloned().unwrap_or(JsonValue::Null),
    })
}

fn truncate_array_field(object: &mut JsonMap<String, JsonValue>, key: &str, limit: usize) {
    if let Some(value) = object.get_mut(key).and_then(JsonValue::as_array_mut) {
        value.truncate(limit);
    }
}

fn select_by_values_i64(
    conn: &Connection,
    table: &str,
    column: &str,
    values: &[i64],
) -> Result<Vec<JsonMap<String, JsonValue>>> {
    let sql = format!(
        r#"
        SELECT *
        FROM {}
        WHERE {} IN ({})
        "#,
        quote_identifier(table),
        quote_identifier(column),
        placeholders(values.len())
    );
    fetch_all(
        conn,
        &sql,
        values.iter().copied().map(SqlValue::Integer).collect(),
    )
}

fn select_by_values_text(
    conn: &Connection,
    table: &str,
    column: &str,
    values: &[String],
) -> Result<Vec<JsonMap<String, JsonValue>>> {
    let sql = format!(
        r#"
        SELECT *
        FROM {}
        WHERE {} IN ({})
        "#,
        quote_identifier(table),
        quote_identifier(column),
        placeholders(values.len())
    );
    fetch_all(
        conn,
        &sql,
        values.iter().cloned().map(SqlValue::Text).collect(),
    )
}

fn table_exists(conn: &Connection, name: &str) -> Result<bool> {
    let found: Option<i64> = conn
        .query_row(
            "SELECT 1 FROM sqlite_master WHERE type='table' AND name=?",
            params![name],
            |row| row.get(0),
        )
        .optional()?;
    Ok(found.is_some())
}

fn tables_exist(conn: &Connection, tables: &[&str]) -> Result<bool> {
    for table in tables {
        if !table_exists(conn, table)? {
            return Ok(false);
        }
    }
    Ok(true)
}

fn table_columns(conn: &Connection, table: &str) -> Result<Vec<String>> {
    let rows = fetch_all(conn, &format!("PRAGMA table_info({})", quote_identifier(table)), vec![])?;
    Ok(rows
        .into_iter()
        .filter_map(|row| value_to_nonempty_string(row.get("name")))
        .collect())
}

fn fetch_one(
    conn: &Connection,
    sql: &str,
    values: Vec<SqlValue>,
) -> Result<Option<JsonMap<String, JsonValue>>> {
    Ok(fetch_all(conn, sql, values)?.into_iter().next())
}

fn fetch_all(
    conn: &Connection,
    sql: &str,
    values: Vec<SqlValue>,
) -> Result<Vec<JsonMap<String, JsonValue>>> {
    let mut stmt = conn.prepare(sql)?;
    let rows = stmt.query_map(params_from_iter(values.iter()), row_to_json_map)?;
    let mut result = Vec::new();
    for row in rows {
        result.push(row?);
    }
    Ok(result)
}

fn scalar_i64(conn: &Connection, sql: &str, values: Vec<SqlValue>) -> Result<i64> {
    let value: Option<i64> = conn
        .query_row(sql, params_from_iter(values.iter()), |row| row.get(0))
        .optional()?;
    Ok(value.unwrap_or(0))
}

fn row_to_json_map(row: &Row<'_>) -> rusqlite::Result<JsonMap<String, JsonValue>> {
    let row_ref = row.as_ref();
    let mut result = JsonMap::new();
    for index in 0..row_ref.column_count() {
        let name = row_ref.column_name(index)?.to_string();
        let value = match row.get_ref(index)? {
            ValueRef::Null => JsonValue::Null,
            ValueRef::Integer(value) => json!(value),
            ValueRef::Real(value) => json!(value),
            ValueRef::Text(value) => JsonValue::String(String::from_utf8_lossy(value).to_string()),
            ValueRef::Blob(value) => JsonValue::String(String::from_utf8_lossy(value).to_string()),
        };
        result.insert(name, value);
    }
    Ok(result)
}

fn loads_json_object(value: Option<&JsonValue>) -> JsonMap<String, JsonValue> {
    let Some(value) = value else {
        return JsonMap::new();
    };
    if let Some(object) = value.as_object() {
        return object.clone();
    }
    let text = value_to_string(Some(value));
    if text.trim().is_empty() {
        return JsonMap::new();
    }
    serde_json::from_str::<JsonValue>(&text)
        .ok()
        .and_then(|value| value.as_object().cloned())
        .unwrap_or_default()
}

fn loads_json_list(value: Option<&JsonValue>) -> Vec<JsonValue> {
    let Some(value) = value else {
        return Vec::new();
    };
    if let Some(array) = value.as_array() {
        return array.clone();
    }
    let text = value_to_string(Some(value));
    if text.trim().is_empty() {
        return Vec::new();
    }
    serde_json::from_str::<JsonValue>(&text)
        .ok()
        .and_then(|value| value.as_array().cloned())
        .unwrap_or_default()
}

fn decode_json_fields(mut row: JsonMap<String, JsonValue>) -> JsonMap<String, JsonValue> {
    let keys = row.keys().cloned().collect::<Vec<_>>();
    for key in keys {
        if !key.ends_with("_json") {
            continue;
        }
        let Some(value) = row.remove(&key) else {
            continue;
        };
        let target = key.trim_end_matches("_json").to_string();
        let decoded = if target == "flags" || target == "samples" || target == "source_references" {
            JsonValue::Array(loads_json_list(Some(&value)))
        } else {
            JsonValue::Object(loads_json_object(Some(&value)))
        };
        row.insert(target, decoded);
    }
    row
}

fn decode_metadata(mut row: JsonMap<String, JsonValue>) -> JsonMap<String, JsonValue> {
    let metadata = loads_json_object(row.remove("metadata_json").as_ref());
    row.insert("metadata".to_string(), JsonValue::Object(metadata));
    row
}

fn dedupe_rows(rows: Vec<JsonMap<String, JsonValue>>) -> Vec<JsonMap<String, JsonValue>> {
    let mut seen = HashSet::new();
    let mut deduped = Vec::new();
    for row in rows {
        let key = serde_json::to_string(&row).unwrap_or_default();
        if seen.insert(key) {
            deduped.push(row);
        }
    }
    deduped
}

fn split_group_concat(value: Option<&JsonValue>) -> Vec<String> {
    let text = value_to_string(value);
    if text.is_empty() {
        return Vec::new();
    }
    text.split(',')
        .filter(|part| !part.is_empty())
        .map(str::to_string)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn quote_identifier(value: &str) -> String {
    format!("\"{}\"", value.replace('"', "\"\""))
}

fn placeholders(count: usize) -> String {
    std::iter::repeat_n("?", count)
        .collect::<Vec<_>>()
        .join(",")
}

fn value_to_string(value: Option<&JsonValue>) -> String {
    match value {
        None | Some(JsonValue::Null) => String::new(),
        Some(JsonValue::String(value)) => value.clone(),
        Some(JsonValue::Number(value)) => value.to_string(),
        Some(JsonValue::Bool(value)) => value.to_string(),
        Some(value) => value.to_string(),
    }
}

fn value_to_nonempty_string(value: Option<&JsonValue>) -> Option<String> {
    let value = value_to_string(value).trim().to_string();
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

fn json_array_objects(value: Option<&JsonValue>) -> Vec<JsonMap<String, JsonValue>> {
    value
        .and_then(JsonValue::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.as_object().cloned())
                .collect()
        })
        .unwrap_or_default()
}

fn safe_f64(value: Option<&JsonValue>, default: f64) -> f64 {
    value
        .and_then(|value| {
            value
                .as_f64()
                .or_else(|| value.as_str().and_then(|text| text.parse::<f64>().ok()))
        })
        .unwrap_or(default)
}

fn int_or_zero(value: Option<&JsonValue>) -> i64 {
    value
        .and_then(|value| {
            value.as_i64().or_else(|| {
                value_to_string(Some(value))
                    .replace(',', "")
                    .trim()
                    .parse::<f64>()
                    .ok()
                    .map(|value| value as i64)
            })
        })
        .unwrap_or(0)
}

fn float_or_null(value: &str) -> JsonValue {
    let cleaned = value.trim().trim_end_matches('%');
    cleaned
        .parse::<f64>()
        .map(JsonValue::from)
        .unwrap_or(JsonValue::Null)
}

fn clamp_i64(value: i64, min: i64, max: i64) -> i64 {
    value.max(min).min(max)
}

fn compare_event_sort_key(
    left: &JsonMap<String, JsonValue>,
    right: &JsonMap<String, JsonValue>,
) -> Ordering {
    date_sort_value(left.get("posted_at"))
        .partial_cmp(&date_sort_value(right.get("posted_at")))
        .unwrap_or(Ordering::Equal)
        .then_with(|| {
            left.get("patch_snapshot_id")
                .and_then(JsonValue::as_i64)
                .unwrap_or(0)
                .cmp(&right.get("patch_snapshot_id").and_then(JsonValue::as_i64).unwrap_or(0))
        })
        .then_with(|| {
            left.get("line_index")
                .and_then(JsonValue::as_i64)
                .unwrap_or(0)
                .cmp(&right.get("line_index").and_then(JsonValue::as_i64).unwrap_or(0))
        })
}

fn date_sort_value(value: Option<&JsonValue>) -> f64 {
    let text = value_to_string(value).trim().to_string();
    if text.is_empty() {
        return 0.0;
    }
    if let Ok(number) = text.parse::<f64>() {
        return number;
    }
    let date_part = text.get(0..10).unwrap_or("");
    if let Some((year, month, day)) = parse_ymd(date_part) {
        let seconds = parse_hms(&text).unwrap_or(0);
        return days_from_civil(year, month, day) as f64 * 86_400.0 + f64::from(seconds);
    }
    0.0
}

fn parse_ymd(value: &str) -> Option<(i32, u32, u32)> {
    let mut parts = value.split('-');
    let year = parts.next()?.parse::<i32>().ok()?;
    let month = parts.next()?.parse::<u32>().ok()?;
    let day = parts.next()?.parse::<u32>().ok()?;
    if (1..=12).contains(&month) && (1..=31).contains(&day) {
        Some((year, month, day))
    } else {
        None
    }
}

fn parse_hms(value: &str) -> Option<u32> {
    let time_start = value.find('T').or_else(|| value.find(' '))? + 1;
    let time = value.get(time_start..)?;
    let mut parts = time.split(':');
    let hour = parts.next()?.get(0..2)?.parse::<u32>().ok()?;
    let minute = parts.next()?.get(0..2)?.parse::<u32>().ok()?;
    let second = parts.next().and_then(|part| part.get(0..2)).and_then(|part| part.parse::<u32>().ok()).unwrap_or(0);
    Some(hour * 3600 + minute * 60 + second)
}

fn days_from_civil(year: i32, month: u32, day: u32) -> i64 {
    let year = year - i32::from(month <= 2);
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let yoe = year - era * 400;
    let month = month as i32;
    let day = day as i32;
    let doy = (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    i64::from(era * 146_097 + doe - 719_468)
}

fn normalize_alias(value: &str) -> String {
    collapse_whitespace(&value.trim().to_lowercase().replace('_', " "))
}

fn normalize_key(text: &str) -> String {
    let cleaned = clean_subject(text).to_lowercase().replace('&', " and ");
    let mut result = String::new();
    let mut last_was_space = false;
    for character in cleaned.chars() {
        if character.is_ascii_alphanumeric() {
            result.push(character);
            last_was_space = false;
        } else if !last_was_space {
            result.push(' ');
            last_was_space = true;
        }
    }
    collapse_whitespace(&result)
}

fn clean_subject(text: &str) -> String {
    let mut cleaned = text.trim().to_string();
    if let Some(stripped) = cleaned.strip_prefix("**") {
        cleaned = stripped.to_string();
    }
    if let Some(stripped) = cleaned.strip_suffix("**") {
        cleaned = stripped.to_string();
    }
    cleaned.trim_matches([' ', ':', '-', '\t']).to_string()
}

fn collapse_whitespace(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn stable_hash_text(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_conn() -> Connection {
        let temp = tempfile::NamedTempFile::new().expect("temp db");
        let conn = Connection::open(temp.path()).expect("open temp db");
        core::db::apply_pragmas(&conn).expect("pragmas");
        core::schema::ensure_schema(&conn).expect("schema");
        conn
    }

    fn insert_entity_fixture(conn: &Connection) {
        conn.execute(
            r#"
            INSERT INTO entity_snapshots(
              id, source, entity_type, external_id, canonical_name, payload_hash,
              payload_json, fetched_at, source_document_id
            )
            VALUES(1, 'deadlock_assets_api', 'item_or_ability', 'item1', 'Mystic Shot', 'hash1', ?, 100, NULL)
            "#,
            params![serde_json::to_string(&json!({
                "name": "Mystic Shot",
                "class_name": "item_mystic_shot",
                "item_slot_type": "spirit",
                "item_tier": 2,
                "cost": 1600,
                "is_active_item": false,
                "description": {"desc": "<b>Bonus Spirit Power</b> and cooldown."},
                "properties": {
                    "TechPower": {
                        "value": 12,
                        "disable_value": 1,
                        "label": "Spirit Power",
                        "provided_property_type": "ETechPower",
                        "tooltip_is_important": true,
                        "scale_function": {"scaling_stats": ["ETechPower"]}
                    }
                },
                "upgrades": []
            })).expect("payload")],
        )
        .expect("snapshot");
        conn.execute(
            r#"
            INSERT INTO entities(
              id, entity_type, canonical_name, primary_external_id, source,
              first_snapshot_id, metadata_json, created_at, updated_at
            )
            VALUES(1, 'item', 'Mystic Shot', 'item1', 'deadlock_assets_api', 1, '{}', 100, 100)
            "#,
            [],
        )
        .expect("entity");
        conn.execute(
            r#"
            INSERT INTO entity_aliases(
              entity_id, alias, alias_norm, alias_kind, source, external_id,
              snapshot_id, created_at
            )
            VALUES(1, 'Mystic Shot', 'mystic shot', 'canonical', 'deadlock_assets_api', 'item1', 1, 100)
            "#,
            [],
        )
        .expect("alias");
        conn.execute(
            r#"
            INSERT INTO patch_events(
              id, patch_snapshot_id, patch_external_id, patch_title, patch_url,
              source_kind, posted_at, line_index, section, entity_type, entity_name,
              subject, change_type, raw_line, normalized_line, old_value, new_value,
              confidence, metadata_json, event_hash, created_at
            )
            VALUES(10, 1, 'p3', 'Patch 3', 'https://example.invalid/p3', 'forum',
              '2026-01-02', 1, 'Items', 'item', 'Mystic Shot', 'Mystic Shot',
              'buff', 'Mystic Shot damage increased from 10 to 12',
              'Mystic Shot damage increased from 10 to 12', '10', '12', 0.9, '{}', 'evt10', 100)
            "#,
            [],
        )
        .expect("patch event");
        conn.execute(
            r#"
            INSERT INTO patch_event_enrichments(
              patch_event_id, stat_name, old_value, new_value, unit, ability_name,
              secondary_entity_name, confidence, flags_json, created_at, updated_at
            )
            VALUES(10, 'damage', '10', '12', NULL, NULL, NULL, 0.95, '["direction:increased"]', 100, 100)
            "#,
            [],
        )
        .expect("enrichment");
    }

    fn ask_claim_fixture(id: i64, status: &str, confidence: f64) -> AskClaimRecord {
        AskClaimRecord {
            id,
            claim_text: format!("claim {id}"),
            evidence_quote: format!("evidence {id}"),
            claim_type: "mechanic".to_string(),
            entity_name: "Mystic Shot".to_string(),
            status: status.to_string(),
            verifier_confidence: confidence,
            source_video: json!({"video_id": "vid", "title": "Video"}),
            verifier: JsonMap::new(),
            match_sources: BTreeSet::new(),
        }
    }

    #[test]
    fn ask_claim_partitioning_keeps_rejected_out_of_verified() {
        let buckets = partition_ask_claims(
            vec![
                ask_claim_fixture(1, "accepted", 0.7),
                ask_claim_fixture(2, "needs_review", 0.8),
                ask_claim_fixture(3, "rejected", 0.9),
                ask_claim_fixture(4, "accepted", 0.6),
            ],
            true,
            10,
        );

        assert_eq!(buckets.verified.len(), 2);
        assert_eq!(buckets.flagged.len(), 1);
        assert_eq!(buckets.refuted.len(), 1);
        assert!(buckets
            .verified
            .iter()
            .all(|claim| claim.status == "accepted"));
        assert_eq!(buckets.refuted[0].status, "rejected");
    }

    #[test]
    fn ask_claim_partitioning_respects_include_unverified() {
        let without_unverified = partition_ask_claims(
            vec![
                ask_claim_fixture(1, "accepted", 0.9),
                ask_claim_fixture(2, "unverified", 0.8),
            ],
            false,
            10,
        );
        let with_unverified = partition_ask_claims(
            vec![
                ask_claim_fixture(1, "accepted", 0.9),
                ask_claim_fixture(2, "unverified", 0.8),
            ],
            true,
            10,
        );

        assert!(without_unverified.unverified.is_empty());
        assert_eq!(without_unverified.omitted.unverified, 1);
        assert_eq!(with_unverified.unverified.len(), 1);
        assert_eq!(with_unverified.omitted.unverified, 0);
    }

    #[test]
    fn keyword_starts_word_filters_mid_word_matches() {
        assert!(!keyword_starts_word("crimson slash", "lash"));
        assert!(!keyword_starts_word("flash farming", "lash"));
        assert!(keyword_starts_word("lash's ground strike", "lash"));
        assert!(keyword_starts_word("denying souls", "soul"));
        assert!(keyword_starts_word("the disarming hex counter", "disarming"));
    }

    #[test]
    fn context_matches_alias_and_loads_patch_events() {
        let conn = temp_conn();
        insert_entity_fixture(&conn);

        let ctx = build_entity_context(&conn, "Mystic Shot", 30).expect("context");

        assert_eq!(ctx["best_match"]["canonical_name"], "Mystic Shot");
        assert_eq!(ctx["best_match"]["score"], 120);
        assert_eq!(ctx["patch_events"].as_array().expect("events").len(), 1);
        assert_eq!(ctx["enrichments"]["available"], true);
    }

    #[test]
    fn timeline_classifies_numeric_buff() {
        let conn = temp_conn();
        insert_entity_fixture(&conn);

        let timeline = build_entity_timeline(&conn, "Mystic Shot", 100, true).expect("timeline");
        let event = &timeline["patches"][0]["events"][0];

        assert_eq!(event["impact_kind"], "numeric_buff");
        assert_eq!(event["impact_level"], "medium");
        assert_eq!(timeline["event_count"], 1);
    }

    #[test]
    fn item_context_summarizes_assets_payload() {
        let conn = temp_conn();
        insert_entity_fixture(&conn);

        let item = build_item_context(&conn, "Mystic Shot").expect("item");

        assert_eq!(item["name"], "Mystic Shot");
        assert_eq!(item["slot"], "spirit");
        assert_eq!(item["cost"], 1600);
        assert!(item["archetypes"]
            .as_array()
            .expect("archetypes")
            .contains(&json!("core_scaling")));
    }

    #[test]
    fn analysis_save_and_list_roundtrip() {
        let conn = temp_conn();
        insert_entity_fixture(&conn);
        let review_context = build_review_context(&conn, "Mystic Shot", 30).expect("review");

        let note = analysis_save_review(
            &conn,
            &review_context,
            Some("Analyse"),
            Some("MiniMax-M3"),
            Some(0.8),
            "analysis_ready",
            None,
        )
        .expect("save");
        let notes = analysis_list(&conn, Some("Mystic"), 25).expect("list");

        assert_eq!(note["status"], "analysis_ready");
        assert_eq!(notes.as_array().expect("notes").len(), 1);
    }

    #[test]
    fn quality_reports_required_tables_present() {
        let conn = temp_conn();
        let report = run_quality_checks(&conn).expect("quality");

        let checks = report["checks"].as_array().expect("checks");
        assert!(checks.iter().any(|check| {
            check["check"] == "required_tables" && check["severity"] == "ok"
        }));
    }

    #[test]
    fn mechanic_vector_path_is_inactive_without_vector_tables() {
        let conn = temp_conn();
        let notes = search_mechanic_notes(&conn, "silence", 5).expect("notes");

        assert_eq!(notes.as_array().expect("array").len(), 0);
    }
}
