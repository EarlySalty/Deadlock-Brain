#![forbid(unsafe_code)]

//! Retrieval-Crate fuer Context-, Timeline-, Review-, Quality- und Item-Abfragen.

use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet, HashSet},
};

pub use deadlock_brain_core as core;

use dbrain_builds::BuildContext;
use deadlock_brain_core::build_narration;
use regex::Regex;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use sqlx::{
    postgres::{PgColumn, PgPool, PgRow},
    Column, Row, TypeInfo,
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
const ASK_STRONG_ON_TOPIC_TARGET: usize = 3;
const ASK_IDF_SCALE: f64 = 10.0;
const ASK_DISTINCTIVE_KEYWORD_MIN_IDF: i64 = 8;
const ASK_PROMPT_TEMPLATE: &str = r#"Du bist ein erfahrener Deadlock-Coach und Analyst. Beantworte die folgende Frage – oder erstelle den gewünschten Build – AUSSCHLIESSLICH auf Basis der unten gelieferten, geprüften Fakten. Erfinde keine Werte, Items, Fähigkeiten oder Patch-Stände. Wenn die Fakten etwas nicht hergeben, sage das offen, statt zu raten.

FRAGE: {{query}}
ERKANNTE ABSICHT: {{intent}}

Die Fakten unten sind nach Vertrauensgrad geordnet. Halte dich strikt an diese Rangfolge:
1. ground_truth – gesicherte Spieldaten (offizielle Werte, Skalierung, Item- und Ability-Karten, Patch-Verlauf). Das ist die harte Wahrheit; bei jedem Widerspruch schlägt sie alles andere.
2. creator_knowledge.verified – Creator-Aussagen, die gegen die Spieldaten geprüft wurden. Belastbar und als Quelle nutzbar.
3. creator_knowledge.flagged – nur teilweise oder gar nicht bestätigt. Höchstens mit klarem Vorbehalt erwähnen ("ein Creator meint …, unbestätigt").
4. creator_knowledge.unverified – ungeprüft, noch nicht gegen die Spieldaten abgeglichen (erscheint nur als Notbehelf, wenn kaum bestätigtes Wissen zur Frage vorliegt). Nur als möglichen Hinweis nutzen, klar als ungeprüft kennzeichnen und keine konkreten Zahlen darauf stützen.
5. creator_knowledge.refuted – nachweislich FALSCH. Niemals als wahr verwenden. Wenn die Frage es berührt, stelle den Irrtum aktiv richtig; die korrekte Tatsache steht in der Begründung oder in ground_truth.

Regeln:
- Nenne konkrete Zahlenwerte nur, wenn sie in ground_truth oder verified belegt sind.
- Wenn ground_truth.item.current_patch_overrides vorhanden ist, gelten diese neuesten Patchwerte vor aelteren Werten aus der Item-Karte.
- Zitiere bei Creator-Wissen die Quelle (Video-Titel), sofern vorhanden.
- Beziehe dich auf den aktuellen Patch-Stand und markiere erkennbar veraltete Aussagen als solche.
- Antworte auf Deutsch, präzise und ohne Floskeln.

FAKTEN (JSON, vertrauenssortiert):
{{ordered_context_json}}"#;
const ASK_TRUST_LEGEND: &str = "Vertrauensstufen: 'ground_truth' = gesicherte Spieldaten (höchste Priorität). Neueste patch_overview- und item.current_patch_overrides-Werte haben Vorrang vor älteren Item-Kartenwerten. 'creator_knowledge.verified' = gegen die Spieldaten geprüfte Creator-Aussagen. 'creator_knowledge.flagged' = nur teilweise oder unbestätigt, nur mit Vorbehalt nutzen. 'creator_knowledge.unverified' = ungeprüft (nicht gegen Spieldaten abgeglichen), nur als möglicher Hinweis, keine Zahlen darauf stützen. 'creator_knowledge.refuted' = nachweislich falsch, nicht verwenden. Bei Widerspruch gilt immer ground_truth.";
const ASK_OOD_NOTICE: &str = "HINWEIS: Diese Frage scheint sich nicht auf Deadlock zu beziehen — es wurden keine gesicherten Spieldaten und keine geprüften Creator-Aussagen dazu gefunden. Wenn die Frage tatsächlich nichts mit Deadlock zu tun hat, weise freundlich darauf hin, dass du auf Deadlock-Wissen spezialisiert bist und dazu keine belegten Fakten vorliegen. Falls sie doch Deadlock betrifft, bitte um eine konkretere Formulierung (Held, Item, Fähigkeit oder Mechanik). Erfinde nichts.";
const ASK_BUILD_INTENT_TERMS: &[&str] = &[
    "build",
    "builds",
    "items",
    "item",
    "baue",
    "bauen",
    "guide",
    "itemization",
    "skillung",
];
const CLAIM_KEYWORD_STOPWORDS: &[&str] = &[
    "about",
    "also",
    "besser",
    "build",
    "counter",
    "does",
    "etwas",
    "from",
    "funktionieren",
    "funktioniert",
    "geht",
    "gibt",
    "have",
    "hero",
    "item",
    "kostet",
    "macht",
    "mehr",
    "need",
    "sehr",
    "should",
    "sind",
    "schneller",
    "that",
    "this",
    "when",
    "with",
    "without",
    "wann",
    "warum",
    "werden",
    "wieso",
    "works",
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
    "viel",
    "viele",
];
const ENTITY_MATCH_STOPWORDS: &[&str] = &[
    "build",
    "builds",
    "bauen",
    "items",
    "item",
    "baue",
    "itemization",
    "guide",
    "skillung",
    "beste",
    "best",
    "geaendert",
    "stark",
    "kontert",
    "counter",
    "gegen",
];
const DEADLOCK_QUERY_TERMS: &[&str] = &[
    "souls",
    "lane",
    "urn",
    "ability",
    "item",
    "build",
    "hero",
    "patch",
    "farm",
    "jungle",
    "breakable",
    "guardian",
    "walker",
    "creep",
    "trooper",
    "denying",
    "secured",
];
const GENERIC_SHORT_GAME_KEYWORDS: &[&str] = &["echo", "hex", "shot"];
const GENERIC_INTENT_BONUS_KEYWORDS: &[&str] = &[
    "ability",
    "abilities",
    "build",
    "builds",
    "counter",
    "farm",
    "farming",
    "gegen",
    "hero",
    "heroes",
    "item",
    "items",
    "kontert",
    "lane",
    "laning",
    "macro",
    "mechanic",
    "mechanics",
    "meta",
    "patch",
    "seele",
    "seelen",
    "soul",
    "souls",
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

    #[error("Postgres error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Fireworks response did not include message content.")]
    EmptyAiResponse,

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
pub struct AnalysisRunAiOptions {
    pub limit_events: i64,
    pub config: core::ai::AiConfig,
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
    relevance_score: i64,
    relevance_floor_score: i64,
    matched_keyword_count: usize,
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
    canonical_name: Option<String>,
    entity_type: Option<String>,
}

#[derive(Debug, Clone)]
struct KnownAskEntity {
    name: String,
    entity_type: String,
}

#[derive(Debug, Clone)]
struct AskKeywordSpec {
    text: String,
    match_key: String,
    entity_or_alias: bool,
    generic_ranking_only: bool,
    prefix_match: bool,
}

#[derive(Debug, Clone, Default)]
struct AskClaimRelevance {
    score: i64,
    distinct_matches: usize,
    entity_keyword_hit: bool,
    distinctive_keyword_hit: bool,
}

#[derive(Debug, Clone, Default)]
struct AskIdfWeights {
    weights: BTreeMap<String, i64>,
}

impl AskIdfWeights {
    fn keyword_weight(&self, match_key: &str) -> i64 {
        self.weights.get(match_key).copied().unwrap_or(1).max(1)
    }
}

pub async fn status(pool: &PgPool) -> Result<JsonValue> {
    let source_documents = fetch_all(
        pool,
        r#"
        SELECT source, COUNT(*) AS documents
        FROM brain.source_documents
        GROUP BY source
        ORDER BY source
        "#,
        vec![],
    ).await?;
    let entity_snapshots = fetch_all(
        pool,
        r#"
        SELECT source, entity_type, COUNT(*) AS snapshots
        FROM brain.entity_snapshots
        GROUP BY source, entity_type
        ORDER BY source, entity_type
        "#,
        vec![],
    ).await?;
    let patch_events = fetch_all(
        pool,
        r#"
        SELECT source_kind, entity_type, COUNT(*) AS events
        FROM brain.patch_events
        GROUP BY source_kind, entity_type
        ORDER BY source_kind, entity_type
        "#,
        vec![],
    ).await?;
    let entities = fetch_all(
        pool,
        r#"
        SELECT entity_type, COUNT(*) AS entities
        FROM brain.entities
        GROUP BY entity_type
        ORDER BY entity_type
        "#,
        vec![],
    ).await?;
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
        if table_exists(pool, table).await? {
            derived.push(json!({"table": table, "count": scalar_i64(pool, &format!("SELECT COUNT(*) FROM {}", qualified_table(table)), vec![]).await?}));
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

pub async fn context(pool: &PgPool, query: &str, limit_events: i64) -> Result<JsonValue> {
    build_entity_context(pool, query, limit_events).await
}

pub async fn build_entity_context(pool: &PgPool, query: &str, limit_events: i64) -> Result<JsonValue> {
    let query = query.trim().to_string();
    let query_norm = normalize_alias(&query);
    let limit = clamp_i64(limit_events, 1, MAX_EVENTS);
    let best_match = find_best_entity_match(pool, &query, &query_norm).await?;
    let fallback_used = best_match.is_none();
    let aliases = match best_match.as_ref().and_then(|row| row.get("id").and_then(JsonValue::as_i64)) {
        Some(entity_id) => load_aliases(pool, entity_id, MAX_ALIASES).await?,
        None => Vec::new(),
    };
    let lineage = related_names_for_query(pool, &query, best_match.as_ref()).await?;
    let mut lineage_names = BTreeSet::new();
    for name in lineage_lookup_names(pool, &query, best_match.as_ref()).await? {
        lineage_names.insert(name);
    }
    for name in legacy_lookup_names(pool, &query).await? {
        lineage_names.insert(name);
    }
    let lineage_names = lineage_names.into_iter().collect::<Vec<_>>();
    let events = load_patch_events(
        pool,
        &query,
        best_match.as_ref(),
        &aliases,
        &lineage_names,
        limit,
        PatchEventMode::Retrieval,
    ).await?;
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
        "enrichments": load_enrichments_bundle(pool, &event_ids, &event_hashes).await?,
        "sheet_stats": load_sheet_stats(pool, &query, best_match.as_ref(), &aliases).await?,
        "fallback": {
            "used": fallback_used,
            "strategy": if fallback_used {
                JsonValue::String("patch_events.entity_name exact/lineage".to_string())
            } else {
                JsonValue::Null
            },
        },
    }))
}

pub async fn timeline(pool: &PgPool, query: &str) -> Result<JsonValue> {
    build_entity_timeline(pool, query, MAX_TIMELINE_EVENTS, true).await
}

pub async fn build_entity_timeline(
    pool: &PgPool,
    query: &str,
    limit_events: i64,
    ascending: bool,
) -> Result<JsonValue> {
    let query = query.trim().to_string();
    let query_norm = normalize_alias(&query);
    let limit = clamp_i64(limit_events, 1, MAX_TIMELINE_EVENTS);
    let best_match = find_best_entity_match(pool, &query, &query_norm).await?;
    let aliases = match best_match.as_ref().and_then(|row| row.get("id").and_then(JsonValue::as_i64)) {
        Some(entity_id) => load_aliases(pool, entity_id, MAX_TIMELINE_ALIASES).await?,
        None => Vec::new(),
    };
    let lineage = related_names_for_query(pool, &query, best_match.as_ref()).await?;
    let mut lineage_names = BTreeSet::new();
    for name in lineage_lookup_names(pool, &query, best_match.as_ref()).await? {
        lineage_names.insert(name);
    }
    for name in legacy_lookup_names(pool, &query).await? {
        lineage_names.insert(name);
    }
    let lineage_names = lineage_names.into_iter().collect::<Vec<_>>();
    let events = load_patch_events(
        pool,
        &query,
        best_match.as_ref(),
        &aliases,
        &lineage_names,
        limit,
        PatchEventMode::Timeline { ascending },
    ).await?;
    let event_ids = events
        .iter()
        .filter_map(|event| event.get("id").and_then(JsonValue::as_i64))
        .collect::<Vec<_>>();
    let enrichments = load_enrichments_by_event_id(pool, &event_ids).await?;
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
            "available": table_exists(pool, "patch_event_enrichments").await?,
            "matched": classified_events.iter().filter(|event| event.get("enrichment").is_some_and(|value| !value.is_null())).count(),
        },
        "fallback": {
            "used": best_match.is_none(),
            "strategy": if best_match.is_none() {
                JsonValue::String("patch_events.entity_name exact/lineage".to_string())
            } else {
                JsonValue::Null
            },
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

pub async fn review(pool: &PgPool, query: &str) -> Result<JsonValue> {
    build_review_context(pool, query, 80).await
}

pub async fn build_review_context(pool: &PgPool, query: &str, limit_events: i64) -> Result<JsonValue> {
    let retrieval_context = build_entity_context(pool, query, limit_events).await?;
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

async fn build_patch_review_context(
    pool: &PgPool,
    query: &str,
    limit_events: i64,
) -> Result<JsonValue> {
    let events = load_patch_overview_events(
        pool,
        query,
        clamp_i64(limit_events, 1, MAX_TIMELINE_EVENTS),
    )
    .await?;
    let event_ids = events
        .iter()
        .filter_map(|event| event.get("id").and_then(JsonValue::as_i64))
        .collect::<Vec<_>>();
    let enrichments = load_enrichments_by_event_id(pool, &event_ids).await?;
    let enrichment_rows = enrichments.values().cloned().collect::<Vec<_>>();
    let timeline_signals = build_timeline_signals(&events, &enrichment_rows);

    Ok(json!({
        "query": query,
        "context_kind": "patch_overview",
        "entity_summary": JsonValue::Null,
        "lineage": JsonValue::Null,
        "current_stat_hints": JsonValue::Null,
        "timeline_signals": timeline_signals,
        "patch_overview": build_patch_overview_signals(&events, &enrichments),
        "open_questions": [],
        "source_references": patch_source_references(&events),
        "prompt_de": "Du bist ein Deadlock-Analyseassistent. Nutze ausschliesslich den bereitgestellten Patch-Kontext und kennzeichne Meta-Folgen als Einschaetzung, wenn sie aus Balance-Aenderungen abgeleitet sind.",
        "retrieval_meta": {
            "route": "patch_overview",
            "limit_events_requested": limit_events,
            "events_loaded": events.len(),
            "enrichments_loaded": enrichment_rows.len(),
        },
    }))
}

async fn load_patch_overview_events(
    pool: &PgPool,
    query: &str,
    limit: i64,
) -> Result<Vec<JsonMap<String, JsonValue>>> {
    if !table_exists(pool, "patch_events").await? {
        return Ok(Vec::new());
    }
    let patterns = query_patch_title_patterns(query);
    let rows = query_patch_overview_events(pool, &patterns, limit).await?;
    if rows.is_empty() && !patterns.is_empty() {
        return query_patch_overview_events(pool, &[], limit).await;
    }
    Ok(rows)
}

async fn query_patch_overview_events(
    pool: &PgPool,
    patterns: &[String],
    limit: i64,
) -> Result<Vec<JsonMap<String, JsonValue>>> {
    let mut params = patterns
        .iter()
        .cloned()
        .map(SqlValue::Text)
        .collect::<Vec<_>>();
    let title_filter = if patterns.is_empty() {
        String::new()
    } else {
        format!(
            "WHERE {}",
            patterns
                .iter()
                .map(|_| "patch_title ILIKE ?")
                .collect::<Vec<_>>()
                .join(" OR ")
        )
    };
    params.push(SqlValue::Integer(limit));
    let sql = format!(
        r#"
        SELECT
          id, patch_snapshot_id, patch_external_id, patch_title, patch_url,
          source_kind, posted_at, line_index, section, entity_type, entity_name,
          subject, change_type, raw_line, normalized_line, old_value, new_value,
          confidence, metadata::text AS metadata_json, event_hash, created_at
        FROM brain.patch_events
        WHERE patch_snapshot_id = (
          SELECT patch_snapshot_id
          FROM brain.patch_events
          {title_filter}
          GROUP BY patch_snapshot_id
          ORDER BY MAX(COALESCE(posted_at::text, '')) DESC, patch_snapshot_id DESC
          LIMIT 1
        )
        ORDER BY line_index ASC
        LIMIT ?
        "#
    );
    let mut rows = fetch_all(pool, &sql, params).await?;
    for row in &mut rows {
        let metadata = loads_json_object(row.remove("metadata_json").as_ref());
        row.insert("metadata".to_string(), JsonValue::Object(metadata));
    }
    Ok(rows)
}

fn query_patch_title_patterns(query: &str) -> Vec<String> {
    let mut patterns = BTreeSet::new();
    if let Ok(ymd) = Regex::new(r"\b(20\d{2})[-./](\d{1,2})[-./](\d{1,2})\b") {
        for capture in ymd.captures_iter(query) {
            let year = capture.get(1).map(|value| value.as_str()).unwrap_or_default();
            let month = capture.get(2).and_then(|value| value.as_str().parse::<u32>().ok());
            let day = capture.get(3).and_then(|value| value.as_str().parse::<u32>().ok());
            if let (Some(month), Some(day)) = (month, day) {
                patterns.insert(format!("%{year}-{month:02}-{day:02}%"));
                patterns.insert(format!("%{month:02}-{day:02}-{year}%"));
            }
        }
    }
    if let Ok(mdy) = Regex::new(r"\b(\d{1,2})[-./](\d{1,2})[-./](20\d{2})\b") {
        for capture in mdy.captures_iter(query) {
            let first = capture.get(1).and_then(|value| value.as_str().parse::<u32>().ok());
            let second = capture.get(2).and_then(|value| value.as_str().parse::<u32>().ok());
            let year = capture.get(3).map(|value| value.as_str()).unwrap_or_default();
            if let (Some(first), Some(second)) = (first, second) {
                patterns.insert(format!("%{first:02}-{second:02}-{year}%"));
                patterns.insert(format!("%{year}-{first:02}-{second:02}%"));
                patterns.insert(format!("%{second:02}-{first:02}-{year}%"));
                patterns.insert(format!("%{year}-{second:02}-{first:02}%"));
            }
        }
    }
    patterns.into_iter().collect()
}

#[derive(Default)]
struct PatchEntityOverview {
    entity_type: String,
    entity: String,
    total: i64,
    buffs: i64,
    nerfs: i64,
    changed: i64,
    reworks: i64,
    score: i64,
    sample_lines: Vec<String>,
}

fn build_patch_overview_signals(
    events: &[JsonMap<String, JsonValue>],
    enrichments: &BTreeMap<i64, JsonMap<String, JsonValue>>,
) -> JsonValue {
    let mut by_entity: BTreeMap<(String, String), PatchEntityOverview> = BTreeMap::new();
    let mut by_type_change: BTreeMap<String, i64> = BTreeMap::new();
    let mut item_changes = Vec::new();
    let mut objective_changes = Vec::new();

    for event in events {
        let entity_type = value_to_nonempty_string(event.get("entity_type"))
            .unwrap_or_else(|| "unknown".to_string());
        let entity = value_to_nonempty_string(event.get("entity_name"))
            .or_else(|| value_to_nonempty_string(event.get("subject")))
            .or_else(|| value_to_nonempty_string(event.get("section")))
            .unwrap_or_else(|| "General".to_string());
        let change_type = value_to_nonempty_string(event.get("change_type"))
            .unwrap_or_else(|| "unknown".to_string());
        *by_type_change
            .entry(format!("{entity_type}:{change_type}"))
            .or_insert(0) += 1;

        let entry = by_entity
            .entry((entity_type.clone(), entity.clone()))
            .or_insert_with(|| PatchEntityOverview {
                entity_type: entity_type.clone(),
                entity: entity.clone(),
                ..PatchEntityOverview::default()
            });
        entry.total += 1;
        match change_type.as_str() {
            "buff" => {
                entry.buffs += 1;
                entry.score += 1;
            }
            "nerf" => {
                entry.nerfs += 1;
                entry.score -= 1;
            }
            "rework" => entry.reworks += 1,
            _ => entry.changed += 1,
        }
        if entry.sample_lines.len() < 4 {
            if let Some(line) = value_to_nonempty_string(
                event.get("normalized_line").or_else(|| event.get("raw_line")),
            ) {
                entry.sample_lines.push(line);
            }
        }

        let event_enrichments = event
            .get("id")
            .and_then(JsonValue::as_i64)
            .and_then(|id| enrichments.get(&id).cloned())
            .map(|enrichment| vec![enrichment])
            .unwrap_or_else(|| vec![JsonMap::new()]);
        if matches!(entity_type.as_str(), "item" | "ability") {
            item_changes.push(compact_event(event, &event_enrichments));
        } else if entity_type == "general" && objective_changes.len() < 18 {
            objective_changes.push(compact_event(event, &event_enrichments));
        }
    }

    let mut summaries = by_entity.into_values().collect::<Vec<_>>();
    summaries.sort_by(|left, right| {
        right
            .total
            .cmp(&left.total)
            .then_with(|| right.score.cmp(&left.score))
            .then_with(|| left.entity.cmp(&right.entity))
    });
    let top_entities = summaries
        .iter()
        .take(24)
        .map(patch_entity_overview_json)
        .collect::<Vec<_>>();
    let mut hero_movers = summaries
        .iter()
        .filter(|entry| entry.entity_type == "hero" && entry.score != 0)
        .collect::<Vec<_>>();
    hero_movers.sort_by(|left, right| {
        right
            .score
            .abs()
            .cmp(&left.score.abs())
            .then_with(|| right.total.cmp(&left.total))
            .then_with(|| left.entity.cmp(&right.entity))
    });

    json!({
        "latest_patch": latest_patch(events),
        "event_count": events.len(),
        "counts_by_entity_type_change": by_type_change.into_iter().map(|(bucket, count)| json!({"bucket": bucket, "count": count})).collect::<Vec<_>>(),
        "top_entities": top_entities,
        "hero_movers_rough": hero_movers.into_iter().take(18).map(patch_entity_overview_json).collect::<Vec<_>>(),
        "item_and_ability_changes": item_changes,
        "objective_and_economy_changes": objective_changes,
        "interpretation_note": "hero_movers_rough ist nur eine Zaehllogik aus Buff/Nerf/Rework-Zeilen; echte Meta-Folgen muessen als Einschaetzung formuliert werden.",
    })
}

fn patch_entity_overview_json(entry: &PatchEntityOverview) -> JsonValue {
    json!({
        "entity_type": entry.entity_type,
        "entity": entry.entity,
        "total": entry.total,
        "buffs": entry.buffs,
        "nerfs": entry.nerfs,
        "changed": entry.changed,
        "reworks": entry.reworks,
        "rough_score": entry.score,
        "sample_lines": entry.sample_lines,
    })
}

fn patch_source_references(events: &[JsonMap<String, JsonValue>]) -> Vec<JsonValue> {
    let mut references = Vec::new();
    let mut seen = HashSet::new();
    for event in events {
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
    references
}

pub async fn ask_context(pool: &PgPool, query: &str, opts: &AskContextOptions) -> Result<JsonValue> {
    let plan = analyze_query(pool, query).await?;
    if is_build_engine_intent(&plan) {
        return ask_build_context(pool, query, &plan).await;
    }
    let entity_match = resolve_ask_entity_match(pool, query, &plan).await?;
    let out_of_domain = !entity_match.matched && !query_has_deadlock_vocabulary(pool, query).await?;
    let intent = if out_of_domain {
        "out_of_domain".to_string()
    } else {
        plan.intent.clone()
    };
    let context_query = resolved_plan_entity_name(&plan).unwrap_or_else(|| query.trim().to_string());
    let base = if out_of_domain {
        JsonValue::Object(JsonMap::new())
    } else if should_use_patch_overview_context(&plan, &entity_match, &intent) {
        build_patch_review_context(pool, query, opts.limit_events).await?
    } else {
        build_review_context(pool, &context_query, opts.limit_events).await?
    };
    let (claims, entity_matched, keyword_matched) = if out_of_domain {
        (Vec::new(), 0, 0)
    } else {
        load_ask_claims(pool, query, &entity_match, &intent).await?
    };
    let claim_buckets = partition_ask_claims(
        claims,
        opts.include_unverified,
        opts.max_claims,
        &intent,
        &entity_match,
    );

    let entity = base.get("entity_summary").cloned().unwrap_or(JsonValue::Null);
    let item_ground_truth = ask_item_ground_truth(pool, &entity).await?;
    let item_ground_truth = apply_current_patch_overrides(
        item_ground_truth,
        base.get("timeline_signals").unwrap_or(&JsonValue::Null),
    );
    let ground_truth = json!({
        "stats": base.get("current_stat_hints").cloned().unwrap_or(JsonValue::Null),
        "patch_overview": base.get("patch_overview").cloned().unwrap_or(JsonValue::Null),
        "timeline": base.get("timeline_signals").cloned().unwrap_or(JsonValue::Null),
        "lineage": base.get("lineage").cloned().unwrap_or(JsonValue::Null),
        "item": item_ground_truth,
    });
    let mut creator_knowledge = JsonMap::new();
    if !out_of_domain {
        creator_knowledge.insert("verified".to_string(), claims_to_json(&claim_buckets.verified));
        creator_knowledge.insert("flagged".to_string(), claims_to_json(&claim_buckets.flagged));
        creator_knowledge.insert("refuted".to_string(), claims_to_json(&claim_buckets.refuted));
        if opts.include_unverified || !claim_buckets.unverified.is_empty() {
            creator_knowledge.insert(
                "unverified".to_string(),
                claims_to_json(&claim_buckets.unverified),
            );
        }
        creator_knowledge.insert("omitted".to_string(), omitted_to_json(&claim_buckets.omitted));
    }

    let mut result = JsonMap::new();
    result.insert("query".to_string(), json!(query));
    result.insert("intent".to_string(), json!(intent.clone()));
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
            "intent": intent,
            "out_of_domain": out_of_domain,
            "context_query": context_query,
            "base_prompt_de_available": base.get("prompt_de").and_then(JsonValue::as_str).is_some_and(|value| !value.trim().is_empty()),
        }),
    );

    let prompt = render_ask_prompt(&JsonValue::Object(result.clone()))?;
    result.insert("prompt".to_string(), JsonValue::String(prompt));
    Ok(JsonValue::Object(result))
}

fn should_use_patch_overview_context(
    plan: &QueryPlan,
    entity_match: &AskEntityClaimMatch,
    intent: &str,
) -> bool {
    intent == "patch_changes" && !entity_match.matched && plan.entities.is_empty()
}

fn apply_current_patch_overrides(item: JsonValue, timeline_signals: &JsonValue) -> JsonValue {
    let mut item = item;
    let Some(item_object) = item.as_object_mut() else {
        return item;
    };
    let latest_patch = timeline_signals
        .get("latest_patch")
        .and_then(JsonValue::as_object)
        .cloned()
        .unwrap_or_default();
    if latest_patch.is_empty() {
        return item;
    }
    let latest_title = value_to_nonempty_string(latest_patch.get("patch_title"));
    let latest_posted_at = value_to_nonempty_string(latest_patch.get("posted_at"));
    let Some(stat_changes) = timeline_signals
        .get("stat_changes")
        .and_then(JsonValue::as_array)
    else {
        return item;
    };
    let updates = stat_changes
        .iter()
        .filter(|change| {
            patch_change_matches_latest(
                change,
                latest_title.as_deref(),
                latest_posted_at.as_deref(),
            )
        })
        .take(16)
        .filter_map(compact_patch_override)
        .collect::<Vec<_>>();
    if updates.is_empty() {
        return item;
    }
    item_object.insert(
        "current_patch_overrides".to_string(),
        json!({
            "source": latest_patch,
            "priority": "Diese neuesten Patchwerte ueberstimmen aeltere Werte aus der statischen Item-Karte.",
            "updates": updates,
        }),
    );
    item
}

fn patch_change_matches_latest(
    change: &JsonValue,
    latest_title: Option<&str>,
    latest_posted_at: Option<&str>,
) -> bool {
    let change_title = value_to_nonempty_string(change.get("patch_title"));
    let change_posted_at = value_to_nonempty_string(change.get("posted_at"));
    latest_title
        .zip(change_title.as_deref())
        .is_some_and(|(left, right)| left == right)
        && latest_posted_at
            .zip(change_posted_at.as_deref())
            .is_some_and(|(left, right)| left == right)
}

fn compact_patch_override(change: &JsonValue) -> Option<JsonValue> {
    let object = change.as_object()?;
    let mut compact = JsonMap::new();
    for key in [
        "stat_name",
        "old_value",
        "new_value",
        "unit",
        "change_type",
        "line",
        "patch_title",
        "posted_at",
        "confidence",
    ] {
        if let Some(value) = object.get(key).filter(|value| !value.is_null()) {
            compact.insert(key.to_string(), value.clone());
        }
    }
    (!compact.is_empty()).then_some(JsonValue::Object(compact))
}

async fn ask_build_context(pool: &PgPool, query: &str, plan: &QueryPlan) -> Result<JsonValue> {
    let hero_query = resolved_plan_entity_name(plan).unwrap_or_else(|| query.trim().to_string());
    let playstyle = detect_build_playstyle(query);
    let build_context = load_build_context(pool, &hero_query, playstyle.as_deref()).await?;
    let prompt = build_narration::build_narration_user_prompt(&build_context)
        .map_err(|err| RetrievalError::Invalid(format!("build narration prompt failed: {err}")))?;
    let narration = build_narration::narrate_build(&build_context)
        .map_err(|err| RetrievalError::Invalid(format!("build narration failed: {err}")))?;
    let known_item_names = load_known_item_names(pool).await?;
    let validation =
        build_narration::validate_narration(&narration, &build_context, &known_item_names);
    let result_text = narration.clone();

    Ok(json!({
        "query": query,
        "intent": plan.intent.clone(),
        "entity": plan.entities.first().cloned().unwrap_or(JsonValue::Null),
        "build_context": build_context,
        "result_text": result_text,
        "validation": validation,
        "prompt": prompt,
        "sources": [],
        "retrieval_meta": {
            "route": "build_engine",
            "context_query": hero_query,
            "playstyle": playstyle,
        },
    }))
}

async fn load_build_context(
    pool: &PgPool,
    hero_query: &str,
    playstyle: Option<&str>,
) -> Result<BuildContext> {
    dbrain_builds::build_context(pool, hero_query, playstyle).await
        .map_err(|err| RetrievalError::Invalid(format!("build context failed: {err}")))
}

async fn load_known_item_names(pool: &PgPool) -> Result<BTreeSet<String>> {
    let rows = fetch_all(pool, "SELECT name FROM brain.item_catalog", vec![]).await?;
    let mut names = BTreeSet::new();
    for row in rows {
        if let Some(name) = value_to_nonempty_string(row.get("name")) {
            names.insert(name);
        }
    }
    Ok(names)
}

pub fn is_build_engine_intent(plan: &QueryPlan) -> bool {
    plan.intent == "build_recommendation" && plan_primary_entity_type(plan).as_deref() == Some("hero")
}

fn plan_primary_entity_type(plan: &QueryPlan) -> Option<String> {
    plan.entities
        .first()
        .and_then(|entity| value_to_nonempty_string(entity.get("type")))
}

fn detect_build_playstyle(query: &str) -> Option<String> {
    let terms = intent_query_terms(query);
    for (style, needles) in [
        ("tank", &["tank", "tanky", "defense", "defensive"][..]),
        ("spirit", &["spirit", "ap", "caster"][..]),
        ("weapon", &["weapon", "gun", "bullet", "dps"][..]),
        ("support", &["support", "utility"][..]),
    ] {
        if contains_any_intent_term(&terms, needles) {
            return Some(style.to_string());
        }
    }
    None
}

pub async fn quality(pool: &PgPool) -> Result<JsonValue> {
    run_quality_checks(pool).await
}

pub async fn run_quality_checks(pool: &PgPool) -> Result<JsonValue> {
    let checks = vec![
        check_required_tables(pool).await?,
        check_entity_counts(pool).await?,
        check_alias_collisions(pool).await?,
        check_patch_events_unknown_entities(pool).await?,
        check_enrichment_table(pool).await?,
        check_lineage_table(pool).await?,
        check_legacy_entities_table(pool).await?,
        check_low_confidence_enrichments(pool).await?,
        check_sheet_profiles_without_entity(pool).await?,
        check_general_events_with_known_entity_names(pool).await?,
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

pub async fn item(pool: &PgPool, query: &str) -> Result<JsonValue> {
    build_item_context(pool, query).await
}

pub async fn build_item_context(pool: &PgPool, query: &str) -> Result<JsonValue> {
    let payload = match load_entity_payload(pool, query, "item").await? {
        Some(found) => Some(found),
        None => load_entity_payload(pool, query, "item_special").await.ok().flatten(),
    }
    .ok_or_else(|| RetrievalError::Invalid(format!("Kein Item-Payload fuer {query} gefunden.")))?;
    Ok(JsonValue::Object(item_summary(&payload)))
}

pub fn summarize_item_payload(payload: &JsonValue) -> JsonValue {
    JsonValue::Object(item_summary(payload.as_object().unwrap_or(&JsonMap::new())))
}

pub async fn analysis_save_review(
    pool: &PgPool,
    review_context: &JsonValue,
    result_text: Option<&str>,
    model: Option<&str>,
    confidence: Option<f64>,
    status: &str,
    provider_metadata: Option<&JsonValue>,
) -> Result<JsonValue> {
    save_review_analysis_note(
        pool,
        review_context,
        result_text,
        model,
        confidence,
        status,
        provider_metadata,
    ).await
}

pub async fn analysis_save_review_for_query(
    pool: &PgPool,
    query: &str,
    limit_events: i64,
    result_text: Option<&str>,
    model: Option<&str>,
    confidence: Option<f64>,
) -> Result<JsonValue> {
    let review_context = build_review_context(pool, query, limit_events).await?;
    let status = if result_text.is_some() {
        "analysis_ready"
    } else {
        "context_ready"
    };
    save_review_analysis_note(
        pool,
        &review_context,
        result_text,
        model,
        confidence,
        status,
        None,
    ).await
}

pub async fn analysis_run_ai(
    pool: &PgPool,
    query: &str,
    options: AnalysisRunAiOptions,
) -> Result<JsonValue> {
    let review_context = build_review_context(pool, query, options.limit_events).await?;
    let prompt = review_context
        .get("prompt_de")
        .and_then(JsonValue::as_str)
        .unwrap_or_default();
    let compact_context = compact_context_for_model(&review_context);
    let request = core::ai::build_review_request(prompt, &compact_context, &options.config);
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

    let client = core::ai::AiClient::new(options.config.clone())?;
    let response = client.chat(&request)?;
    let result_text = core::ai::extract_ai_text(&response);
    if result_text.is_empty() {
        return Err(RetrievalError::EmptyAiResponse);
    }
    let provider_metadata = core::ai::ai_usage_summary(&response);
    let note = save_review_analysis_note(
        pool,
        &review_context,
        Some(&result_text),
        Some(&options.config.model),
        None,
        "analysis_ready",
        Some(&provider_metadata),
    ).await?;
    Ok(json!({
        "note": note,
        "query": query,
        "model": options.config.model,
        "endpoint": endpoint,
        "result_text": result_text,
        "provider_metadata": provider_metadata,
    }))
}

pub async fn analysis_list(pool: &PgPool, query: Option<&str>, limit: i64) -> Result<JsonValue> {
    let max_rows = clamp_i64(limit, 1, 500);
    let mut sql = r#"
        SELECT id, query, entity_type, entity_name, context_kind, context_hash,
               prompt_version, model, confidence, status, created_at, updated_at
        FROM brain.analysis_notes
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
        fetch_all(pool, &sql, params).await?
            .into_iter()
            .map(JsonValue::Object)
            .collect(),
    ))
}

pub async fn search_mechanic_notes(pool: &PgPool, _query: &str, _limit: i64) -> Result<JsonValue> {
    if !table_exists(pool, "mechanic_notes").await? || !table_exists(pool, "vector_embeddings").await? {
        return Ok(JsonValue::Array(Vec::new()));
    }
    todo!("Vektorsuche: spätere Wave")
}

pub async fn analyze_query(pool: &PgPool, query: &str) -> Result<QueryPlan> {
    let known_entities = load_known_ask_entities(pool).await?;
    let mut entities = Vec::new();
    let mut threats = Vec::new();
    let query_norm = normalize_alias(query);

    for entity in known_entities.iter().filter(|entity| entity.entity_type == "hero") {
        if keyword_starts_word(&query_norm, &normalize_alias(&entity.name)) {
            push_query_entity(&mut entities, &mut threats, entity);
        }
    }
    if entities.is_empty() {
        for entity in known_entities
            .iter()
            .filter(|entity| matches!(entity.entity_type.as_str(), "item" | "item_special"))
        {
            if keyword_starts_word(&query_norm, &normalize_alias(&entity.name)) {
                push_query_entity(&mut entities, &mut threats, entity);
            }
        }
    }
    if entities.is_empty() {
        if let Some(best_match) = resolve_entity_from_query_tokens(pool, query).await? {
            if let (Some(name), Some(entity_type)) = (
                value_to_nonempty_string(best_match.get("canonical_name")),
                value_to_nonempty_string(best_match.get("entity_type")),
            ) {
                entities.push(json!({"name": name, "type": entity_type, "raw": query}));
            }
        }
    }

    let matched = !entities.is_empty();
    let entity_type = entities
        .first()
        .and_then(|entity| value_to_nonempty_string(entity.get("type")))
        .unwrap_or_default();
    let query_lower = query.to_lowercase();
    let intent = classify_ask_intent(&query_lower, matched, &entity_type);
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

async fn load_known_ask_entities(pool: &PgPool) -> Result<Vec<KnownAskEntity>> {
    if !table_exists(pool, "entities").await? {
        return Ok(Vec::new());
    }
    Ok(fetch_all(
        pool,
        r#"
        SELECT canonical_name, entity_type
        FROM brain.entities
        WHERE entity_type IN ('hero', 'item', 'item_special')
        ORDER BY entity_type, length(canonical_name) DESC, canonical_name
        "#,
        vec![],
    ).await?
    .into_iter()
    .filter_map(|row| {
        Some(KnownAskEntity {
            name: value_to_nonempty_string(row.get("canonical_name"))?,
            entity_type: value_to_nonempty_string(row.get("entity_type"))?,
        })
    })
    .collect())
}

fn push_query_entity(
    entities: &mut Vec<JsonValue>,
    threats: &mut Vec<String>,
    entity: &KnownAskEntity,
) {
    if entities.is_empty() {
        entities.push(json!({"name": entity.name.clone(), "type": entity.entity_type.clone(), "raw": entity.name.clone()}));
    } else if entity.entity_type == "hero" {
        threats.push(entity.name.clone());
    }
}

fn classify_ask_intent(query_lower: &str, matched: bool, entity_type: &str) -> String {
    let terms = intent_query_terms(query_lower);
    let item_entity = matches!(entity_type, "item" | "item_special");
    if item_entity && contains_any_intent_term(&terms, ASK_BUILD_INTENT_TERMS) {
        return "build_recommendation".to_string();
    }
    if item_entity && !has_explicit_ask_action(&terms) {
        return "item_question".to_string();
    }
    if contains_any_intent_term(&terms, &["geaendert", "geändert", "nerf", "buff", "patch", "update", "changelog"]) {
        return "patch_changes".to_string();
    }
    if contains_any_intent_term(&terms, &["meta", "tier", "viable", "noch stark", "noch gut"]) {
        return "meta_question".to_string();
    }
    if contains_any_intent_term(&terms, &["kontert", "counter", "gegen", "matchup", "vs"]) {
        return "matchup".to_string();
    }
    if !matched
        && contains_any_intent_term(
            &terms,
            &[
                "souls",
                "soul",
                "denying",
                "breakables",
                "breakable",
                "jungle",
                "urn",
                "secured",
                "lane",
                "farm",
            ],
        )
    {
        return "mechanics_question".to_string();
    }
    if matched && contains_any_intent_term(&terms, ASK_BUILD_INTENT_TERMS) {
        return "build_recommendation".to_string();
    }
    if item_entity {
        return "item_question".to_string();
    }
    "hero_overview".to_string()
}

fn has_explicit_ask_action(terms: &[String]) -> bool {
    contains_any_intent_term(
        terms,
        &[
            "baue",
            "bauen",
            "build",
            "builds",
            "buff",
            "changelog",
            "counter",
            "gegen",
            "geaendert",
            "geändert",
            "guide",
            "item",
            "itemization",
            "items",
            "kontert",
            "matchup",
            "meta",
            "nerf",
            "patch",
            "tier",
            "update",
            "viable",
            "vs",
            "skillung",
        ],
    )
}

fn contains_any_intent_term(terms: &[String], needles: &[&str]) -> bool {
    needles.iter().any(|needle| intent_terms_contain(terms, needle))
}

fn intent_terms_contain(terms: &[String], needle: &str) -> bool {
    let needle_terms = intent_query_terms(needle);
    if needle_terms.is_empty() {
        return false;
    }
    if needle_terms.len() == 1 {
        return terms.iter().any(|term| term == &needle_terms[0]);
    }
    terms
        .windows(needle_terms.len())
        .any(|window| window == needle_terms.as_slice())
}

fn intent_query_terms(value: &str) -> Vec<String> {
    value
        .split_whitespace()
        .map(|term| {
            term.trim_matches(|character: char| !character.is_alphanumeric())
                .to_lowercase()
        })
        .filter(|term| !term.is_empty())
        .collect()
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
        "matchup" => vec!["counterplay_claims", "matchup_claims", "hero_stats", "patch_events"],
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

async fn find_best_entity_match(
    pool: &PgPool,
    query: &str,
    query_norm: &str,
) -> Result<Option<JsonMap<String, JsonValue>>> {
    if query_norm.is_empty()
        || !table_exists(pool, "entities").await?
        || !table_exists(pool, "entity_aliases").await?
    {
        return Ok(None);
    }
    let like_norm = format!("%{query_norm}%");
    let like_query = format!("%{query}%");
    let rows = fetch_all(
        pool,
        r#"
        SELECT
          e.id,
          e.entity_type,
          e.canonical_name,
          e.primary_external_id,
          e.source,
          e.metadata::text AS metadata_json,
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
          string_agg(DISTINCT a.alias_kind, ',') AS matched_alias_kinds
        FROM brain.entities e
        LEFT JOIN brain.entity_aliases a ON a.entity_id=e.id
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
    ).await?;
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

async fn resolve_entity_from_query_tokens(
    pool: &PgPool,
    query: &str,
) -> Result<Option<JsonMap<String, JsonValue>>> {
    let tokens = entity_match_tokens(query);
    if tokens.is_empty() {
        return Ok(None);
    }
    let mut candidates = Vec::new();
    for window in tokens.windows(2) {
        if let [left, right] = window {
            candidates.push(format!("{left} {right}"));
        }
    }
    candidates.extend(tokens);

    let mut best: Option<JsonMap<String, JsonValue>> = None;
    for candidate in candidates {
        let Some(row) = find_exact_entity_match(pool, &candidate).await? else {
            continue;
        };
        let row_score = row.get("score").and_then(JsonValue::as_i64).unwrap_or(0);
        let row_len = value_to_string(row.get("canonical_name")).len();
        let replace = best
            .as_ref()
            .map(|current| {
                let current_score = current.get("score").and_then(JsonValue::as_i64).unwrap_or(0);
                let current_len = value_to_string(current.get("canonical_name")).len();
                row_score > current_score || (row_score == current_score && row_len > current_len)
            })
            .unwrap_or(true);
        if replace {
            best = Some(row);
        }
    }
    Ok(best)
}

fn entity_match_tokens(query: &str) -> Vec<String> {
    tokenize_ascii_lower(query)
        .into_iter()
        .filter(|token| !ENTITY_MATCH_STOPWORDS.contains(&token.as_str()))
        .collect()
}

async fn find_exact_entity_match(
    pool: &PgPool,
    candidate: &str,
) -> Result<Option<JsonMap<String, JsonValue>>> {
    let query_norm = normalize_alias(candidate);
    if query_norm.is_empty()
        || !table_exists(pool, "entities").await?
        || !table_exists(pool, "entity_aliases").await?
    {
        return Ok(None);
    }
    let Some(mut row) = fetch_one(
        pool,
        r#"
        SELECT
          e.id,
          e.entity_type,
          e.canonical_name,
          e.primary_external_id,
          e.source,
          e.metadata::text AS metadata_json,
          MAX(
            CASE
              WHEN lower(e.canonical_name)=lower(?) THEN 120
              WHEN a.alias_norm=? AND a.alias_kind='canonical' THEN 115
              WHEN a.alias_norm=? THEN 110
              ELSE 0
            END
          ) AS score,
          string_agg(DISTINCT a.alias_kind, ',') AS matched_alias_kinds
        FROM brain.entities e
        LEFT JOIN brain.entity_aliases a ON a.entity_id=e.id
        WHERE lower(e.canonical_name)=lower(?) OR a.alias_norm=?
        GROUP BY e.id
        ORDER BY score DESC, e.entity_type, length(e.canonical_name) DESC, e.canonical_name
        LIMIT 1
        "#,
        vec![
            SqlValue::Text(candidate.to_string()),
            SqlValue::Text(query_norm.clone()),
            SqlValue::Text(query_norm.clone()),
            SqlValue::Text(candidate.to_string()),
            SqlValue::Text(query_norm),
        ],
    ).await?
    else {
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

async fn load_aliases(pool: &PgPool, entity_id: i64, limit: i64) -> Result<Vec<JsonMap<String, JsonValue>>> {
    fetch_all(
        pool,
        r#"
        SELECT alias, alias_norm, alias_kind, source, external_id, snapshot_id
        FROM brain.entity_aliases
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
    ).await
}

async fn related_names_for_query(
    pool: &PgPool,
    query: &str,
    best_match: Option<&JsonMap<String, JsonValue>>,
) -> Result<Vec<JsonMap<String, JsonValue>>> {
    if !table_exists(pool, "entity_lineage").await? {
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
        SELECT id, patch_event_id, relation_type, source_entity_type, source_name,
               source_name_norm, target_entity_type, target_name, target_name_norm,
               owner_entity_type, owner_name, owner_name_norm, confidence,
               metadata::text AS metadata_json, created_at, updated_at
        FROM brain.entity_lineage
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
    let rows = fetch_all(pool, &sql, params).await?
        .into_iter()
        .map(decode_metadata)
        .collect();
    Ok(rows)
}

async fn lineage_lookup_names(
    pool: &PgPool,
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
    for row in related_names_for_query(pool, query, best_match).await? {
        for key in ["source_name", "target_name", "owner_name"] {
            if let Some(value) = value_to_nonempty_string(row.get(key)) {
                names.insert(value);
            }
        }
    }
    Ok(names.into_iter().filter(|name| !name.is_empty()).collect())
}

async fn legacy_lookup_names(pool: &PgPool, query: &str) -> Result<Vec<String>> {
    if !table_exists(pool, "legacy_entities").await? {
        return Ok(Vec::new());
    }
    let query_norm = normalize_alias(query);
    if query_norm.is_empty() {
        return Ok(Vec::new());
    }
    let rows = fetch_all(
        pool,
        r#"
        SELECT canonical_name
        FROM brain.legacy_entities
        WHERE name_norm=? OR name_norm LIKE ?
        ORDER BY confidence DESC, event_count DESC, canonical_name
        LIMIT 40
        "#,
        vec![
            SqlValue::Text(query_norm.clone()),
            SqlValue::Text(format!("%{query_norm}%")),
        ],
    ).await?;
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

async fn load_patch_events(
    pool: &PgPool,
    query: &str,
    best_match: Option<&JsonMap<String, JsonValue>>,
    aliases: &[JsonMap<String, JsonValue>],
    lineage_names: &[String],
    limit: i64,
    mode: PatchEventMode,
) -> Result<Vec<JsonMap<String, JsonValue>>> {
    if !table_exists(pool, "patch_events").await? {
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
        let entity_types = patch_event_entity_types(best_match);
        let name_count = names.len();
        params.extend(names.into_iter().map(SqlValue::Text));
        if entity_types.is_empty() {
            format!("lower(entity_name) IN ({})", placeholders(name_count))
        } else {
            let type_count = entity_types.len();
            params.extend(entity_types.into_iter().map(SqlValue::Text));
            format!(
                "lower(entity_name) IN ({}) AND entity_type IN ({})",
                placeholders(name_count),
                placeholders(type_count)
            )
        }
    } else if lineage_names.is_empty() {
        params.push(SqlValue::Text(query.to_lowercase()));
        "lower(entity_name)=?".to_string()
    } else {
        let names = lineage_names
            .iter()
            .map(|name| name.to_lowercase())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        params.extend(names.iter().cloned().map(SqlValue::Text));
        params.push(SqlValue::Text(query.to_lowercase()));
        format!(
            "(lower(entity_name) IN ({}) OR lower(entity_name)=?)",
            placeholders(names.len())
        )
    };

    let order_by = match mode {
        PatchEventMode::Retrieval => "patch_snapshot_id DESC, line_index".to_string(),
        PatchEventMode::Timeline { ascending } => {
            let direction = if ascending { "ASC" } else { "DESC" };
            format!("COALESCE(posted_at::text, '') {direction}, patch_snapshot_id {direction}, line_index {direction}")
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
          metadata::text AS metadata_json,
          event_hash,
          created_at
        FROM brain.patch_events
        WHERE {where_clause}
        ORDER BY {order_by}
        LIMIT ?
        "#
    );
    let mut rows = fetch_all(pool, &sql, params).await?;
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

fn patch_event_entity_types(best_match: &JsonMap<String, JsonValue>) -> Vec<String> {
    match value_to_string(best_match.get("entity_type")).as_str() {
        "hero" | "hero_internal" => vec!["hero".to_string(), "hero_internal".to_string()],
        "item" | "item_special" => vec!["item".to_string(), "item_special".to_string()],
        "ability" | "ability_internal" => {
            vec!["ability".to_string(), "ability_internal".to_string()]
        }
        "" => Vec::new(),
        entity_type => vec![entity_type.to_string()],
    }
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

async fn load_enrichments_bundle(
    pool: &PgPool,
    event_ids: &[i64],
    event_hashes: &[String],
) -> Result<JsonValue> {
    let table = "patch_event_enrichments";
    if !table_exists(pool, table).await? {
        return Ok(json!({"available": false, "rows": []}));
    }
    let columns = table_columns(pool, table).await?;
    let mut rows = Vec::new();
    if columns.contains(&"patch_event_id".to_string()) && !event_ids.is_empty() {
        rows.extend(select_by_values_i64(pool, table, "patch_event_id", event_ids).await?);
    } else if columns.contains(&"event_id".to_string()) && !event_ids.is_empty() {
        rows.extend(select_by_values_i64(pool, table, "event_id", event_ids).await?);
    } else if columns.contains(&"event_hash".to_string()) && !event_hashes.is_empty() {
        rows.extend(select_by_values_text(pool, table, "event_hash", event_hashes).await?);
    }
    let rows = dedupe_rows(rows.into_iter().map(decode_json_fields).collect());
    Ok(json!({"available": true, "rows": rows}))
}

async fn load_enrichments_by_event_id(
    pool: &PgPool,
    event_ids: &[i64],
) -> Result<BTreeMap<i64, JsonMap<String, JsonValue>>> {
    if event_ids.is_empty() || !table_exists(pool, "patch_event_enrichments").await? {
        return Ok(BTreeMap::new());
    }
    let columns = table_columns(pool, "patch_event_enrichments").await?;
    if !columns.contains(&"patch_event_id".to_string()) {
        return Ok(BTreeMap::new());
    }
    let mut enrichments = BTreeMap::new();
    for row in select_by_values_i64(pool, "patch_event_enrichments", "patch_event_id", event_ids).await? {
        let decoded = decode_json_fields(row);
        if let Some(patch_event_id) = decoded.get("patch_event_id").and_then(JsonValue::as_i64) {
            enrichments.insert(patch_event_id, decoded);
        }
    }
    Ok(enrichments)
}

async fn load_sheet_stats(
    pool: &PgPool,
    query: &str,
    best_match: Option<&JsonMap<String, JsonValue>>,
    aliases: &[JsonMap<String, JsonValue>],
) -> Result<JsonValue> {
    let names = sheet_lookup_names(query, best_match, aliases);
    let mut sources = Vec::new();
    let snapshot_rows = load_sheet_snapshot_stats(pool, &names).await?;
    if !snapshot_rows.is_empty() {
        sources.push(json!({"source": "entity_snapshots", "rows": snapshot_rows}));
    }
    for table in candidate_sheet_tables(pool).await? {
        let rows = load_matching_sheet_table_rows(pool, &table, &names).await?;
        if !rows.is_empty() {
            sources.push(json!({"source": table, "rows": rows}));
        }
    }
    Ok(json!({"available": !sources.is_empty(), "sources": sources}))
}

async fn load_sheet_snapshot_stats(
    pool: &PgPool,
    names: &[String],
) -> Result<Vec<JsonMap<String, JsonValue>>> {
    if !table_exists(pool, "entity_snapshots").await? {
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
        SELECT id, source, entity_type, external_id, canonical_name, payload::text AS payload_json, fetched_at
        FROM brain.entity_snapshots
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
    let mut rows = fetch_all(pool, &sql, params).await?;
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

async fn candidate_sheet_tables(pool: &PgPool) -> Result<Vec<String>> {
    let rows = fetch_all(
        pool,
        r#"
        SELECT table_name AS name
        FROM information_schema.tables
        WHERE table_schema='brain'
          AND table_name NOT IN (
            'source_documents',
            'source_runs',
            'entity_snapshots',
            'entities',
            'entity_aliases',
            'patch_events',
            'patch_event_enrichments'
          )
        ORDER BY table_name
        "#,
        vec![],
    ).await?;
    Ok(rows
        .into_iter()
        .filter_map(|row| value_to_nonempty_string(row.get("name")))
        .filter(|name| {
            let lowered = name.to_lowercase();
            lowered.contains("sheet") || lowered.contains("stat")
        })
        .collect())
}

async fn load_matching_sheet_table_rows(
    pool: &PgPool,
    table: &str,
    names: &[String],
) -> Result<Vec<JsonMap<String, JsonValue>>> {
    let columns = table_columns(pool, table).await?;
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
            clauses.push(format!("{}::text LIKE ?", quote_identifier(column)));
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
        qualified_table(table),
        clauses.join(" OR ")
    );
    Ok(fetch_all(pool, &sql, params).await?
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

async fn resolve_ask_entity_match(
    pool: &PgPool,
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
    if let Some(best_match) = resolve_entity_from_query_tokens(pool, query).await? {
        if let Some(name) = value_to_nonempty_string(best_match.get("canonical_name")) {
            candidates.push(name);
        }
    }

    for candidate in candidates {
        let query_norm = normalize_alias(&candidate);
        let Some(best_match) = find_best_entity_match(pool, &candidate, &query_norm).await? else {
            continue;
        };
        let mut names = BTreeSet::new();
        let canonical_name = value_to_nonempty_string(best_match.get("canonical_name"));
        let entity_type = value_to_nonempty_string(best_match.get("entity_type"));
        if let Some(name) = canonical_name.clone() {
            insert_ask_entity_match_name(&mut names, &name, true);
        }
        if let Some(entity_id) = best_match.get("id").and_then(JsonValue::as_i64) {
            for alias in load_aliases(pool, entity_id, MAX_ALIASES).await? {
                if let Some(value) = value_to_nonempty_string(alias.get("alias")) {
                    insert_ask_entity_match_name(&mut names, &value, false);
                }
                if let Some(value) = value_to_nonempty_string(alias.get("alias_norm")) {
                    insert_ask_entity_match_name(&mut names, &value, false);
                }
            }
        }
        return Ok(AskEntityClaimMatch {
            names: names.into_iter().collect(),
            matched: true,
            canonical_name,
            entity_type,
        });
    }

    Ok(AskEntityClaimMatch::default())
}

fn insert_ask_entity_match_name(names: &mut BTreeSet<String>, name: &str, allow_short: bool) {
    let name_norm = normalize_alias(name);
    if name_norm.is_empty() {
        return;
    }
    if !allow_short && (name_norm.chars().all(|character| character.is_ascii_digit()) || name_norm.len() < 4) {
        return;
    }
    names.insert(name.to_string());
}

fn resolved_plan_entity_name(plan: &QueryPlan) -> Option<String> {
    plan.entities
        .first()
        .and_then(|entity| value_to_nonempty_string(entity.get("name")))
}

async fn query_has_deadlock_vocabulary(pool: &PgPool, query: &str) -> Result<bool> {
    let query_norm = normalize_alias(query);
    if query_norm.is_empty() {
        return Ok(false);
    }
    if DEADLOCK_QUERY_TERMS
        .iter()
        .any(|term| keyword_starts_word(&query_norm, term))
    {
        return Ok(true);
    }
    if !tables_exist(pool, &["entities", "entity_aliases"]).await? {
        return Ok(false);
    }
    let rows = fetch_all(
        pool,
        r#"
        SELECT canonical_name AS name
        FROM brain.entities
        WHERE entity_type IN ('hero', 'item', 'item_special', 'ability')
        UNION
        SELECT a.alias AS name
        FROM brain.entity_aliases a
        JOIN brain.entities e ON e.id=a.entity_id
        WHERE e.entity_type IN ('hero', 'item', 'item_special', 'ability')
        "#,
        vec![],
    ).await?;
    for row in rows {
        let Some(name) = value_to_nonempty_string(row.get("name")) else {
            continue;
        };
        let name_norm = normalize_alias(&name);
        if name_norm.len() < 4 || matches!(name_norm.as_str(), "hero" | "item" | "ability") {
            continue;
        }
        if keyword_starts_word(&query_norm, &name_norm) {
            return Ok(true);
        }
    }
    Ok(false)
}

async fn ask_item_ground_truth(pool: &PgPool, entity: &JsonValue) -> Result<JsonValue> {
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
    match build_item_context(pool, name).await {
        Ok(value) => Ok(value),
        Err(RetrievalError::Invalid(_)) => Ok(JsonValue::Null),
        Err(error) => Err(error),
    }
}

async fn load_ask_claims(
    pool: &PgPool,
    query: &str,
    entity_match: &AskEntityClaimMatch,
    intent: &str,
) -> Result<(Vec<AskClaimRecord>, usize, usize)> {
    if !tables_exist(pool, &["youtube_learning_claims", "youtube_videos"]).await? {
        return Ok((Vec::new(), 0, 0));
    }

    let keywords = ask_query_keyword_specs(query, entity_match);
    let idf_weights = load_ask_idf_weights(pool, &keywords).await?;

    let mut by_id = BTreeMap::new();
    let entity_rows = load_entity_claim_rows(pool, &entity_match.names).await?;
    let entity_matched = entity_rows.len();
    merge_claim_rows(&mut by_id, entity_rows, "entity");

    let keyword_rows = load_keyword_claim_rows(pool, &keywords, entity_match, &idf_weights).await?;
    let keyword_matched = keyword_rows.len();
    merge_claim_rows(&mut by_id, keyword_rows, "keyword");

    let mut claims = by_id.into_values().collect::<Vec<_>>();
    for claim in &mut claims {
        let relevance = ask_claim_relevance(claim, &keywords, entity_match, &idf_weights);
        let noise_penalty = claim_noise_penalty(claim, entity_match);
        claim.relevance_floor_score = relevance.score + noise_penalty;
        claim.relevance_score = claim.relevance_floor_score
            + claim_intent_relevance_bonus(claim, intent, entity_match, &relevance);
        claim.matched_keyword_count = relevance.distinct_matches;
    }
    claims.retain(|claim| ask_claim_passes_minimum_relevance(claim, &keywords, entity_match, &idf_weights));
    Ok((claims, entity_matched, keyword_matched))
}

async fn load_entity_claim_rows(
    pool: &PgPool,
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
    let exact_sql = format!(
        r#"
        SELECT c.id, c.video_id, c.entity_type, c.entity_name, c.claim_type,
               c.claim_text, c.evidence_quote, c.verifier_confidence, c.status,
               c.verifier::text AS verifier_json, v.title AS source_video_title
        FROM brain.youtube_learning_claims c
        LEFT JOIN brain.youtube_videos v ON v.video_id=c.video_id
        WHERE lower(c.entity_name) IN ({})
        ORDER BY c.verifier_confidence DESC, c.id
        "#,
        placeholders(names.len())
    );
    let mut by_id = BTreeMap::new();
    for row in fetch_all(
        pool,
        &exact_sql,
        names.iter().cloned().map(SqlValue::Text).collect(),
    ).await? {
        if let Some(id) = row.get("id").and_then(JsonValue::as_i64) {
            by_id.insert(id, row);
        }
    }

    let mut clauses = Vec::new();
    let mut values = Vec::new();
    for name in &names {
        clauses.push("(lower(c.claim_text) LIKE ? OR lower(c.evidence_quote) LIKE ?)".to_string());
        let pattern = format!("%{}%", name);
        values.push(SqlValue::Text(pattern.clone()));
        values.push(SqlValue::Text(pattern));
    }
    let mut text_sql = format!(
        r#"
        SELECT c.id, c.video_id, c.entity_type, c.entity_name, c.claim_type,
               c.claim_text, c.evidence_quote, c.verifier_confidence, c.status,
               c.verifier::text AS verifier_json, v.title AS source_video_title
        FROM brain.youtube_learning_claims c
        LEFT JOIN brain.youtube_videos v ON v.video_id=c.video_id
        WHERE ({})
        "#,
        clauses.join(" OR ")
    );
    text_sql.push_str(" ORDER BY c.verifier_confidence DESC, c.id");
    for row in fetch_all(pool, &text_sql, values).await?
        .into_iter()
        .filter(|row| entity_text_row_matches_name(row, &names))
    {
        if let Some(id) = row.get("id").and_then(JsonValue::as_i64) {
            by_id.entry(id).or_insert(row);
        }
    }
    Ok(by_id.into_values().collect())
}

async fn load_keyword_claim_rows(
    pool: &PgPool,
    keywords: &[AskKeywordSpec],
    entity_match: &AskEntityClaimMatch,
    idf_weights: &AskIdfWeights,
) -> Result<Vec<JsonMap<String, JsonValue>>> {
    if keywords.is_empty() {
        return Ok(Vec::new());
    }
    let mut clauses = Vec::new();
    let mut values = Vec::new();
    for keyword in keywords {
        clauses.push("(lower(c.claim_text) LIKE ? OR lower(c.evidence_quote) LIKE ? OR lower(c.verifier::text) LIKE ?)".to_string());
        let pattern = format!("%{}%", keyword.text);
        values.push(SqlValue::Text(pattern.clone()));
        values.push(SqlValue::Text(pattern.clone()));
        values.push(SqlValue::Text(pattern));
    }
    let sql = format!(
        r#"
        SELECT c.id, c.video_id, c.entity_type, c.entity_name, c.claim_type,
               c.claim_text, c.evidence_quote, c.verifier_confidence, c.status,
               c.verifier::text AS verifier_json, v.title AS source_video_title
        FROM brain.youtube_learning_claims c
        LEFT JOIN brain.youtube_videos v ON v.video_id=c.video_id
        WHERE ({})
        ORDER BY c.verifier_confidence DESC, c.id
        "#,
        clauses.join(" OR ")
    );
    let rows = fetch_all(pool, &sql, values).await?;
    Ok(rows
        .into_iter()
        .filter(|row| keyword_row_starts_word(row, keywords))
        .filter(|row| keyword_row_passes_minimum_relevance(row, keywords, entity_match, idf_weights))
        .collect())
}

async fn load_ask_idf_weights(pool: &PgPool, keywords: &[AskKeywordSpec]) -> Result<AskIdfWeights> {
    if keywords.is_empty() {
        return Ok(AskIdfWeights::default());
    }
    let rows = fetch_all(
        pool,
        r#"
        SELECT claim_text, evidence_quote, verifier::text AS verifier_json, entity_name
        FROM brain.youtube_learning_claims
        "#,
        vec![],
    ).await?;
    if rows.is_empty() {
        return Ok(AskIdfWeights::default());
    }

    let mut document_frequency = BTreeMap::<String, usize>::new();
    for row in &rows {
        let haystack = format!(
            "{} {} {} {}",
            value_to_string(row.get("claim_text")),
            value_to_string(row.get("evidence_quote")),
            value_to_string(row.get("verifier_json")),
            value_to_string(row.get("entity_name"))
        )
        .to_lowercase();
        let mut seen_in_document = BTreeSet::new();
        for keyword in keywords {
            if keyword_spec_matches_text(&haystack, keyword) {
                seen_in_document.insert(keyword.match_key.clone());
            }
        }
        for match_key in seen_in_document {
            *document_frequency.entry(match_key).or_insert(0) += 1;
        }
    }

    let document_count = rows.len() as f64;
    let weights = keywords
        .iter()
        .map(|keyword| {
            let df = document_frequency
                .get(&keyword.match_key)
                .copied()
                .unwrap_or(0)
                .max(1) as f64;
            let weight = ((document_count / df).ln() * ASK_IDF_SCALE).round() as i64;
            (keyword.match_key.clone(), weight.max(0))
        })
        .collect::<BTreeMap<_, _>>();
    Ok(AskIdfWeights { weights })
}

fn entity_text_row_matches_name(row: &JsonMap<String, JsonValue>, names: &[String]) -> bool {
    let claim_text = value_to_string(row.get("claim_text")).to_lowercase();
    let evidence_quote = value_to_string(row.get("evidence_quote")).to_lowercase();
    names.iter().any(|name| {
        keyword_starts_word(&claim_text, name) || keyword_starts_word(&evidence_quote, name)
    })
}

fn keyword_row_starts_word(row: &JsonMap<String, JsonValue>, keywords: &[AskKeywordSpec]) -> bool {
    let claim_text = value_to_string(row.get("claim_text")).to_lowercase();
    let evidence_quote = value_to_string(row.get("evidence_quote")).to_lowercase();
    let verifier_json = value_to_string(row.get("verifier_json")).to_lowercase();
    keywords.iter().any(|keyword| {
        keyword_spec_matches_text(&claim_text, keyword)
            || keyword_spec_matches_text(&evidence_quote, keyword)
            || keyword_spec_matches_text(&verifier_json, keyword)
    })
}

fn keyword_spec_matches_text(haystack_lower: &str, keyword: &AskKeywordSpec) -> bool {
    if keyword.prefix_match {
        keyword_starts_word_prefix(haystack_lower, &keyword.text)
    } else {
        keyword_starts_word(haystack_lower, &keyword.text)
    }
}

fn keyword_starts_word(haystack_lower: &str, token_lower: &str) -> bool {
    if token_lower.is_empty() {
        return false;
    }
    haystack_lower.match_indices(token_lower).any(|(index, _)| {
        keyword_has_start_boundary(haystack_lower, index)
            && keyword_has_end_boundary(haystack_lower, index + token_lower.len())
    })
}

fn keyword_starts_word_prefix(haystack_lower: &str, token_lower: &str) -> bool {
    if token_lower.is_empty() {
        return false;
    }
    haystack_lower
        .match_indices(token_lower)
        .any(|(index, _)| keyword_has_start_boundary(haystack_lower, index))
}

fn keyword_has_start_boundary(value: &str, index: usize) -> bool {
    index == 0
        || value[..index]
            .chars()
            .next_back()
            .is_some_and(|previous| !previous.is_ascii_alphanumeric())
}

fn keyword_has_end_boundary(value: &str, end: usize) -> bool {
    if end >= value.len() {
        return true;
    }
    let mut chars = value[end..].char_indices();
    let Some((_, next)) = chars.next() else {
        return true;
    };
    if !next.is_ascii_alphanumeric() {
        return true;
    }
    if matches!(next, 's' | 'n' | 'e') {
        let suffix_end = end + next.len_utf8();
        return suffix_end >= value.len()
            || value[suffix_end..]
                .chars()
                .next()
                .is_some_and(|after| !after.is_ascii_alphanumeric());
    }
    false
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
        relevance_score: 0,
        relevance_floor_score: 0,
        matched_keyword_count: 0,
    })
}

fn ask_query_keywords(query: &str) -> Vec<String> {
    let raw_tokens = tokenize_ascii_lower(query);
    let mut keywords = Vec::new();
    let mut seen = HashSet::new();
    for window in raw_tokens.windows(2) {
        if let [left, right] = window {
            push_ask_keyword(&mut keywords, &mut seen, &format!("{left} {right}"));
        }
    }
    for token in raw_tokens
        .into_iter()
        .filter(|token| token.len() >= 4 && !CLAIM_KEYWORD_STOPWORDS.contains(&token.as_str()))
    {
        push_ask_keyword(&mut keywords, &mut seen, &token);
    }
    keywords.into_iter().take(12).collect()
}

fn push_ask_keyword(keywords: &mut Vec<String>, seen: &mut HashSet<String>, keyword: &str) {
    let keyword = normalize_alias(keyword);
    if !keyword.is_empty() && seen.insert(keyword.clone()) {
        keywords.push(keyword);
    }
}

fn ask_query_keyword_specs(
    query: &str,
    entity_match: &AskEntityClaimMatch,
) -> Vec<AskKeywordSpec> {
    let mut specs = BTreeMap::<String, AskKeywordSpec>::new();
    let ranking_only_short_tokens = entity_match.matched
        && entity_match
            .entity_type
            .as_deref()
            .is_some_and(|entity_type| matches!(entity_type, "hero" | "item" | "item_special"));
    for keyword in ask_query_keywords(query) {
        let generic_ranking_only = ranking_only_short_tokens
            && !keyword.contains(' ')
            && is_generic_short_game_keyword(&keyword);
        let (match_key, prefix_match) = ask_keyword_synonym_group(&keyword)
            .map(|(match_key, variants)| {
                let prefix_match = variants
                    .iter()
                    .find(|variant| variant.text == keyword)
                    .is_some_and(|variant| variant.prefix_match);
                (match_key.to_string(), prefix_match)
            })
            .unwrap_or_else(|| (keyword.clone(), false));
        insert_keyword_spec(
            &mut specs,
            &keyword,
            &match_key,
            false,
            generic_ranking_only,
            prefix_match,
        );
        if !keyword.contains(' ') {
            for expansion in ask_keyword_synonym_expansions(&keyword) {
                insert_keyword_spec(
                    &mut specs,
                    &expansion.text,
                    &expansion.match_key,
                    false,
                    generic_ranking_only,
                    expansion.prefix_match,
                );
            }
        }
    }
    let query_norm = normalize_alias(query);
    for name in &entity_match.names {
        let name_norm = normalize_alias(name);
        if name_norm.len() < 4 {
            continue;
        }
        let is_canonical = entity_match
            .canonical_name
            .as_deref()
            .map(normalize_alias)
            .as_deref()
            == Some(name_norm.as_str());
        if is_canonical || keyword_starts_word(&query_norm, &name_norm) {
            insert_keyword_spec(&mut specs, &name_norm, &name_norm, true, false, false);
        }
    }
    specs.into_values().take(32).collect()
}

fn insert_keyword_spec(
    specs: &mut BTreeMap<String, AskKeywordSpec>,
    text: &str,
    match_key: &str,
    entity_or_alias: bool,
    generic_ranking_only: bool,
    prefix_match: bool,
) {
    let text = normalize_alias(text);
    let match_key = normalize_alias(match_key);
    if text.is_empty() {
        return;
    }
    specs
        .entry(text.clone())
        .and_modify(|existing| {
            existing.entity_or_alias |= entity_or_alias;
            existing.generic_ranking_only &= generic_ranking_only;
            existing.prefix_match |= prefix_match;
            if existing.match_key.is_empty() {
                existing.match_key = match_key.clone();
            }
        })
        .or_insert(AskKeywordSpec {
            text,
            match_key,
            entity_or_alias,
            generic_ranking_only,
            prefix_match,
        });
}

#[derive(Debug, Clone)]
struct AskKeywordExpansion {
    text: String,
    match_key: String,
    prefix_match: bool,
}

fn ask_keyword_synonym_expansions(keyword: &str) -> Vec<AskKeywordExpansion> {
    let keyword = normalize_alias(keyword);
    let Some((match_key, variants)) = ask_keyword_synonym_group(&keyword) else {
        return Vec::new();
    };
    variants
        .into_iter()
        .filter(|variant| variant.text != keyword)
        .map(|variant| AskKeywordExpansion {
            text: variant.text.to_string(),
            match_key: match_key.to_string(),
            prefix_match: variant.prefix_match,
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
struct AskKeywordVariant {
    text: &'static str,
    prefix_match: bool,
}

fn ask_keyword_synonym_group(keyword: &str) -> Option<(&'static str, Vec<AskKeywordVariant>)> {
    match keyword {
        "deny" | "denying" | "denied" | "denies" | "denyen" | "deniet" | "deni" => Some((
            "deny",
            vec![
                AskKeywordVariant {
                    text: "deny",
                    prefix_match: true,
                },
                AskKeywordVariant {
                    text: "deni",
                    prefix_match: true,
                },
                AskKeywordVariant {
                    text: "denyen",
                    prefix_match: true,
                },
            ],
        )),
        "soul" | "souls" | "seele" | "seelen" | "seelenkugel" | "seelenkugeln" => Some((
            "soul",
            vec![
                AskKeywordVariant {
                    text: "soul",
                    prefix_match: true,
                },
                AskKeywordVariant {
                    text: "seele",
                    prefix_match: true,
                },
                AskKeywordVariant {
                    text: "seelen",
                    prefix_match: true,
                },
            ],
        )),
        "breakable" | "breakables" | "box" | "boxes" | "crate" | "crates" | "kiste" | "kisten" => {
            Some((
                "breakable",
                vec![
                    AskKeywordVariant {
                        text: "breakable",
                        prefix_match: true,
                    },
                    AskKeywordVariant {
                        text: "box",
                        prefix_match: true,
                    },
                    AskKeywordVariant {
                        text: "crate",
                        prefix_match: true,
                    },
                    AskKeywordVariant {
                        text: "kiste",
                        prefix_match: true,
                    },
                ],
            ))
        }
        _ => None,
    }
}

fn is_generic_short_game_keyword(token: &str) -> bool {
    token.len() <= 5 || GENERIC_SHORT_GAME_KEYWORDS.contains(&token)
}

fn keyword_row_passes_minimum_relevance(
    row: &JsonMap<String, JsonValue>,
    keywords: &[AskKeywordSpec],
    entity_match: &AskEntityClaimMatch,
    idf_weights: &AskIdfWeights,
) -> bool {
    let relevance = row_relevance(
        &value_to_string(row.get("claim_text")),
        &value_to_string(row.get("evidence_quote")),
        &value_to_string(row.get("entity_name")),
        keywords,
        entity_match,
        idf_weights,
    );
    relevance.distinct_matches >= 2
        || relevance.entity_keyword_hit
        || (relevance.distinct_matches >= 1 && allows_single_specific_keyword(keywords, entity_match))
}

fn ask_claim_passes_minimum_relevance(
    claim: &AskClaimRecord,
    keywords: &[AskKeywordSpec],
    entity_match: &AskEntityClaimMatch,
    idf_weights: &AskIdfWeights,
) -> bool {
    if claim.match_sources.iter().any(|source| source != "keyword") {
        return true;
    }
    let relevance = ask_claim_relevance(claim, keywords, entity_match, idf_weights);
    relevance.distinct_matches >= 2
        || relevance.entity_keyword_hit
        || (relevance.distinct_matches >= 1 && allows_single_specific_keyword(keywords, entity_match))
}

fn allows_single_specific_keyword(
    keywords: &[AskKeywordSpec],
    entity_match: &AskEntityClaimMatch,
) -> bool {
    !entity_match.matched
        && keywords
            .iter()
            .filter(|keyword| {
                !keyword.entity_or_alias
                    && !keyword.generic_ranking_only
                    && !keyword.text.contains(' ')
                    && keyword.text.len() >= 8
            })
            .count()
            == 1
}

fn ask_claim_relevance(
    claim: &AskClaimRecord,
    keywords: &[AskKeywordSpec],
    entity_match: &AskEntityClaimMatch,
    idf_weights: &AskIdfWeights,
) -> AskClaimRelevance {
    row_relevance(
        &claim.claim_text,
        &claim.evidence_quote,
        &claim.entity_name,
        keywords,
        entity_match,
        idf_weights,
    )
}

fn row_relevance(
    claim_text: &str,
    evidence_quote: &str,
    entity_name: &str,
    keywords: &[AskKeywordSpec],
    entity_match: &AskEntityClaimMatch,
    idf_weights: &AskIdfWeights,
) -> AskClaimRelevance {
    let claim_text = claim_text.to_lowercase();
    let evidence_quote = evidence_quote.to_lowercase();
    let entity_name_norm = normalize_alias(entity_name);
    let entity_name_is_canonical = entity_name_matches_canonical(&entity_name_norm, entity_match);
    let entity_name_is_match = entity_name_matches_entity(&entity_name_norm, entity_match);
    let mut score = 0;
    let mut distinct_matches = BTreeSet::new();
    let mut entity_keyword_hit = entity_name_is_match;
    let mut entity_text_hit = false;
    let mut distinctive_keyword_hit = false;
    for keyword in keywords {
        let in_claim = keyword_spec_matches_text(&claim_text, keyword);
        let in_evidence = keyword_spec_matches_text(&evidence_quote, keyword);
        let in_entity_name = keyword.entity_or_alias
            && keyword_spec_matches_text(&entity_name_norm, keyword);
        if !in_claim && !in_evidence && !in_entity_name {
            continue;
        }
        let idf_weight = idf_weights.keyword_weight(&keyword.match_key);
        if is_distinctive_bonus_keyword(keyword, idf_weight) {
            distinctive_keyword_hit = true;
        }
        let text_multiplier = if keyword.entity_or_alias && entity_name_is_canonical {
            3
        } else if keyword.entity_or_alias {
            2
        } else {
            1
        };
        let evidence_multiplier = if keyword.entity_or_alias && entity_name_is_canonical {
            2
        } else {
            1
        };
        if in_claim {
            score += idf_weight * text_multiplier;
        }
        if in_evidence {
            score += idf_weight * evidence_multiplier;
        }
        if keyword.entity_or_alias {
            entity_text_hit = true;
            entity_keyword_hit = true;
        }
        if !keyword.generic_ranking_only || keyword.entity_or_alias {
            distinct_matches.insert(keyword.match_key.clone());
        }
    }
    if entity_name_is_canonical {
        score += 8;
        entity_keyword_hit = true;
    } else if entity_name_is_match {
        score += 4;
    } else if entity_text_hit {
        score += 1;
    }
    AskClaimRelevance {
        score,
        distinct_matches: distinct_matches.len(),
        entity_keyword_hit,
        distinctive_keyword_hit,
    }
}

fn is_distinctive_bonus_keyword(keyword: &AskKeywordSpec, idf_weight: i64) -> bool {
    if keyword.generic_ranking_only {
        return false;
    }
    let generic_text = GENERIC_INTENT_BONUS_KEYWORDS.contains(&keyword.text.as_str())
        || GENERIC_INTENT_BONUS_KEYWORDS.contains(&keyword.match_key.as_str());
    if generic_text {
        return false;
    }
    if keyword.entity_or_alias && keyword.match_key.len() >= 4 {
        return true;
    }
    idf_weight >= ASK_DISTINCTIVE_KEYWORD_MIN_IDF
}

fn preferred_claim_type_bonus(preferred: bool, relevance: &AskClaimRelevance) -> i64 {
    if !preferred {
        return 0;
    }
    if relevance.distinctive_keyword_hit {
        60
    } else {
        2
    }
}

fn claim_intent_relevance_bonus(
    claim: &AskClaimRecord,
    intent: &str,
    entity_match: &AskEntityClaimMatch,
    relevance: &AskClaimRelevance,
) -> i64 {
    let claim_type = claim.claim_type.to_lowercase();
    let entity_name_norm = normalize_alias(&claim.entity_name);
    match intent {
        "build_recommendation" => {
            let preferred = matches!(claim_type.as_str(), "build" | "item_timing");
            preferred_claim_type_bonus(preferred, relevance)
                + match claim_type.as_str() {
                    "mechanic" => -220,
                    "general" | "meta" | "macro" => -220,
                    "matchup" | "counterplay" | "combo" => -220,
                    _ => 0,
                }
        }
        "item_question" => {
            if entity_name_matches_canonical(&entity_name_norm, entity_match) {
                10
            } else {
                0
            }
        }
        "matchup" | "counterplay" => {
            let preferred = matches!(claim_type.as_str(), "counterplay" | "matchup" | "combo");
            preferred_claim_type_bonus(preferred, relevance)
                + match claim_type.as_str() {
                    "mechanic" if entity_name_matches_canonical(&entity_name_norm, entity_match) => {
                        if relevance.distinctive_keyword_hit { 2 } else { -20 }
                    }
                    "mechanic" => -60,
                    "general" | "meta" | "macro" => -100,
                    "build" | "item_timing" => -100,
                    _ => 0,
                }
        }
        "patch_changes" => match claim_type.as_str() {
            "meta" | "general" | "mechanic" | "buff" | "nerf" | "rework" => 8,
            "build" | "item_timing" | "matchup" | "counterplay" => -10,
            _ => 0,
        },
        "mechanics_question" => {
            let preferred = claim_type == "mechanic";
            preferred_claim_type_bonus(preferred, relevance)
                + match claim_type.as_str() {
                    "general" => 0,
                    "build" | "item_timing" | "matchup" | "counterplay" | "combo" => -8,
                    _ => 0,
                }
        }
        "meta_question" | "hero_overview" => {
            let preferred = matches!(claim_type.as_str(), "meta" | "macro");
            preferred_claim_type_bonus(preferred, relevance)
                + match claim_type.as_str() {
                    "general" => 2,
                    _ => 0,
                }
        },
        _ => 0,
    }
}

fn claim_intent_priority(
    claim: &AskClaimRecord,
    intent: &str,
    entity_match: &AskEntityClaimMatch,
) -> i64 {
    let claim_type = claim.claim_type.to_lowercase();
    let entity_name_norm = normalize_alias(&claim.entity_name);
    match intent {
        "build_recommendation" => match claim_type.as_str() {
            "build" | "item_timing" => 3,
            "mechanic" => 1,
            _ => 0,
        },
        "matchup" | "counterplay" => match claim_type.as_str() {
            "counterplay" | "matchup" if entity_name_is_multi_entity_listing(claim) => 1,
            "counterplay" | "matchup" if multi_entity_claim_for_single_entity_intent(claim, entity_match) => 2,
            "counterplay" | "matchup" => 3,
            "mechanic" if entity_name_matches_canonical(&entity_name_norm, entity_match) => 2,
            _ => 0,
        },
        "patch_changes" => match claim_type.as_str() {
            "meta" | "general" | "mechanic" | "buff" | "nerf" | "rework" => 2,
            _ => 0,
        },
        "mechanics_question" => match claim_type.as_str() {
            "mechanic" => 3,
            "general" => 1,
            _ => 0,
        },
        "meta_question" | "hero_overview" => match claim_type.as_str() {
            "meta" | "general" => 2,
            _ => 0,
        },
        _ => 0,
    }
}

fn claim_noise_penalty(claim: &AskClaimRecord, entity_match: &AskEntityClaimMatch) -> i64 {
    if !entity_match.matched {
        return 0;
    }
    let entity_name_norm = normalize_alias(&claim.entity_name);
    if entity_name_matches_canonical(&entity_name_norm, entity_match) {
        return 0;
    }

    let mut penalty = 0;
    let keyword_only = claim.match_sources.len() == 1 && claim.match_sources.contains("keyword");
    if keyword_only && claim.matched_keyword_count <= 1 {
        penalty -= 40;
    }
    if multi_entity_claim_for_single_entity_intent(claim, entity_match) {
        penalty -= 40;
    }
    if entity_name_is_multi_entity_listing(claim) {
        penalty -= 20;
    }
    penalty
}

fn multi_entity_claim_for_single_entity_intent(
    claim: &AskClaimRecord,
    entity_match: &AskEntityClaimMatch,
) -> bool {
    let Some(canonical_name) = entity_match.canonical_name.as_deref() else {
        return false;
    };
    let entity_name_lower = claim.entity_name.to_lowercase();
    let claim_text_lower = claim.claim_text.to_lowercase();
    let canonical_lower = canonical_name.to_lowercase();
    let entity_listing = entity_name_lower.contains(" vs ")
        || entity_name_lower.matches(',').count() >= 2
        || entity_name_lower.matches('/').count() >= 2;
    let text_listing = claim_text_lower.contains(&canonical_lower)
        && (claim_text_lower.matches(',').count() >= 3 || claim_text_lower.matches(" vs ").count() >= 2);
    entity_listing || text_listing
}

fn entity_name_is_multi_entity_listing(claim: &AskClaimRecord) -> bool {
    let entity_name_lower = claim.entity_name.to_lowercase();
    entity_name_lower.contains(" vs ")
        || entity_name_lower.matches(',').count() >= 2
        || entity_name_lower.matches('/').count() >= 2
}

fn entity_name_matches_entity(entity_name_norm: &str, entity_match: &AskEntityClaimMatch) -> bool {
    if entity_name_norm.is_empty() {
        return false;
    }
    entity_match
        .names
        .iter()
        .any(|name| normalize_alias(name) == entity_name_norm)
}

fn entity_name_matches_canonical(entity_name_norm: &str, entity_match: &AskEntityClaimMatch) -> bool {
    entity_match
        .canonical_name
        .as_deref()
        .map(normalize_alias)
        .as_deref()
        == Some(entity_name_norm)
}

fn partition_ask_claims(
    claims: Vec<AskClaimRecord>,
    include_unverified: bool,
    max_claims: usize,
    intent: &str,
    entity_match: &AskEntityClaimMatch,
) -> AskClaimBuckets {
    let mut all = AskClaimBuckets::default();
    for claim in relevance_filtered_claims(claims, intent, entity_match) {
        match claim.status.as_str() {
            "accepted" => all.verified.push(claim),
            "needs_review" => all.flagged.push(claim),
            "rejected" => all.refuted.push(claim),
            "unverified" => all.unverified.push(claim),
            _ => {}
        }
    }
    let take = ask_claim_take_counts(&all, include_unverified, max_claims, intent, entity_match);
    let total_verified = all.verified.len();
    let total_flagged = all.flagged.len();
    let total_refuted = all.refuted.len();
    let total_unverified = all.unverified.len();
    AskClaimBuckets {
        verified: sorted_take_ask_claims(all.verified, take.verified, intent, entity_match),
        flagged: sorted_take_ask_claims(all.flagged, take.flagged, intent, entity_match),
        refuted: sorted_take_ask_claims(all.refuted, take.refuted, intent, entity_match),
        unverified: if include_unverified || take.unverified > 0 {
            sorted_take_ask_claims(all.unverified, take.unverified, intent, entity_match)
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

fn relevance_filtered_claims(
    claims: Vec<AskClaimRecord>,
    intent: &str,
    entity_match: &AskEntityClaimMatch,
) -> Vec<AskClaimRecord> {
    let mut claims = claims
        .into_iter()
        .filter(|claim| claim.relevance_score > 0 && claim.relevance_floor_score > 0)
        .collect::<Vec<_>>();
    let Some(top_score) = claims.iter().map(|claim| claim.relevance_floor_score).max() else {
        return Vec::new();
    };
    let floor = dynamic_relevance_floor(top_score);
    claims.retain(|claim| claim.relevance_floor_score >= floor);
    sort_ask_claims(&mut claims, intent, entity_match);
    claims
}

fn dynamic_relevance_floor(top_score: i64) -> i64 {
    let quarter = ((top_score.max(0) as f64) * 0.25).ceil() as i64;
    quarter.max(1)
}

fn sort_ask_claims(
    claims: &mut [AskClaimRecord],
    intent: &str,
    entity_match: &AskEntityClaimMatch,
) {
    claims.sort_by(|left, right| {
        claim_intent_priority(right, intent, entity_match)
            .cmp(&claim_intent_priority(left, intent, entity_match))
            .then_with(|| right.relevance_score.cmp(&left.relevance_score))
            .then_with(|| left.id.cmp(&right.id))
    });
}

fn sorted_take_ask_claims(
    mut claims: Vec<AskClaimRecord>,
    take: usize,
    intent: &str,
    entity_match: &AskEntityClaimMatch,
) -> Vec<AskClaimRecord> {
    sort_ask_claims(&mut claims, intent, entity_match);
    claims.into_iter().take(take).collect()
}

fn ask_claim_take_counts(
    claims: &AskClaimBuckets,
    include_unverified: bool,
    max_claims: usize,
    intent: &str,
    entity_match: &AskEntityClaimMatch,
) -> AskClaimOmitted {
    let mut take = AskClaimOmitted::default();
    let mut remaining = max_claims;
    if remaining == 0 {
        return take;
    }

    let reserve_flagged = flagged_reserve_count(claims, max_claims, intent, entity_match);
    let reserve_refuted = usize::from(!claims.refuted.is_empty());
    let reserve_unverified = unverified_reserve_count(
        claims,
        include_unverified,
        max_claims,
        intent,
        entity_match,
    );
    let reserve = (reserve_flagged + reserve_refuted + reserve_unverified).min(remaining.saturating_sub(1));

    take.verified = claims.verified.len().min(remaining.saturating_sub(reserve));
    remaining = remaining.saturating_sub(take.verified);

    let flagged_to_take = reserve_flagged.min(claims.flagged.len()).min(remaining);
    take.flagged = flagged_to_take;
    remaining = remaining.saturating_sub(flagged_to_take);
    if reserve_refuted > 0 && remaining > 0 {
        take.refuted = 1;
        remaining -= 1;
    }
    let unverified_to_take = reserve_unverified
        .min(claims.unverified.len())
        .min(remaining);
    take.unverified = unverified_to_take;
    remaining = remaining.saturating_sub(unverified_to_take);

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

fn flagged_reserve_count(
    claims: &AskClaimBuckets,
    max_claims: usize,
    intent: &str,
    entity_match: &AskEntityClaimMatch,
) -> usize {
    if claims.flagged.is_empty() || max_claims == 0 {
        return 0;
    }
    let max_verified_score = claims
        .verified
        .iter()
        .map(|claim| claim.relevance_score)
        .max()
        .unwrap_or(0);
    let competitive = claims
        .flagged
        .iter()
        .filter(|claim| {
            claim_intent_priority(claim, intent, entity_match) >= 2
                && claim.relevance_score >= max_verified_score
        })
        .count();
    competitive.max(1).min(claims.flagged.len()).min(ASK_STRONG_ON_TOPIC_TARGET)
}

fn unverified_reserve_count(
    claims: &AskClaimBuckets,
    include_unverified: bool,
    max_claims: usize,
    intent: &str,
    entity_match: &AskEntityClaimMatch,
) -> usize {
    if claims.unverified.is_empty() || max_claims == 0 {
        return 0;
    }
    let strong_verified_flagged = claims
        .verified
        .iter()
        .chain(claims.flagged.iter())
        .filter(|claim| strong_on_topic_claim(claim, intent, entity_match))
        .count();
    let fallback_needed = ASK_STRONG_ON_TOPIC_TARGET.saturating_sub(strong_verified_flagged);
    let fallback_available = claims
        .unverified
        .iter()
        .filter(|claim| strong_on_topic_claim(claim, intent, entity_match))
        .count();
    let fallback_take = fallback_needed.min(fallback_available);
    if fallback_take > 0 {
        fallback_take
    } else {
        usize::from(include_unverified)
    }
}

fn strong_on_topic_claim(
    claim: &AskClaimRecord,
    intent: &str,
    entity_match: &AskEntityClaimMatch,
) -> bool {
    claim.relevance_score > 0 && claim_intent_priority(claim, intent, entity_match) >= 2
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
        "relevance_score": claim.relevance_score,
        "matched_keyword_count": claim.matched_keyword_count,
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
    if bundle
        .pointer("/retrieval_meta/out_of_domain")
        .and_then(JsonValue::as_bool)
        .unwrap_or(false)
    {
        return Ok(render_ask_ood_prompt(bundle));
    }
    let ordered_context_json = serde_json::to_string_pretty(&ordered_ask_context_for_prompt(bundle))?;
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

fn render_ask_ood_prompt(bundle: &JsonValue) -> String {
    let role_line = ASK_PROMPT_TEMPLATE
        .split('.')
        .next()
        .map(|value| format!("{}.", value.trim()))
        .unwrap_or_default();
    let query = value_to_string(bundle.get("query"));
    if role_line.is_empty() {
        format!("{ASK_OOD_NOTICE}\n\nFRAGE: {query}")
    } else {
        format!("{role_line}\n\n{ASK_OOD_NOTICE}\n\nFRAGE: {query}")
    }
}

fn ordered_ask_context_for_prompt(bundle: &JsonValue) -> JsonValue {
    let mut ordered_context = JsonMap::new();
    if let Some(ground_truth) = prompt_ground_truth(bundle) {
        ordered_context.insert("ground_truth".to_string(), ground_truth);
    }
    if let Some(creator_knowledge) = prompt_creator_knowledge(bundle) {
        ordered_context.insert("creator_knowledge".to_string(), creator_knowledge);
    }
    ordered_context.insert(
        "query".to_string(),
        bundle.get("query").cloned().unwrap_or(JsonValue::Null),
    );
    if let Some(sources) = bundle.get("sources").filter(|value| prompt_value_available(value)) {
        ordered_context.insert("sources".to_string(), sources.clone());
    }
    JsonValue::Object(ordered_context)
}

fn prompt_ground_truth(bundle: &JsonValue) -> Option<JsonValue> {
    let ground_truth = bundle.get("ground_truth")?.as_object()?;
    let mut compact = JsonMap::new();
    for key in ["stats", "lineage", "item"] {
        if let Some(value) = ground_truth.get(key).filter(|value| prompt_value_available(value)) {
            compact.insert(key.to_string(), value.clone());
        }
    }
    if let Some(timeline) = ground_truth
        .get("timeline")
        .and_then(compact_timeline_for_prompt)
    {
        compact.insert("timeline".to_string(), timeline);
    }
    if compact.is_empty() {
        None
    } else {
        Some(JsonValue::Object(compact))
    }
}

fn compact_timeline_for_prompt(timeline: &JsonValue) -> Option<JsonValue> {
    let compact = prune_prompt_value(timeline)?;
    let object = compact.as_object()?;
    let has_timeline_facts = [
        "latest_patch",
        "change_type_counts",
        "source_counts",
        "top_sections",
        "recent_events",
        "stat_changes",
        "ability_mentions",
    ]
    .iter()
    .any(|key| object.contains_key(*key));
    if has_timeline_facts {
        Some(compact)
    } else {
        None
    }
}

fn prune_prompt_value(value: &JsonValue) -> Option<JsonValue> {
    match value {
        JsonValue::Null => None,
        JsonValue::Array(items) => {
            let compact = items
                .iter()
                .filter_map(prune_prompt_value)
                .collect::<Vec<_>>();
            if compact.is_empty() {
                None
            } else {
                Some(JsonValue::Array(compact))
            }
        }
        JsonValue::Object(object) => {
            if object
                .get("available")
                .and_then(JsonValue::as_bool)
                .is_some_and(|available| !available)
            {
                return None;
            }
            let mut compact = JsonMap::new();
            for (key, value) in object {
                if let Some(value) = prune_prompt_value(value) {
                    compact.insert(key.clone(), value);
                }
            }
            if compact.is_empty() {
                None
            } else {
                Some(JsonValue::Object(compact))
            }
        }
        JsonValue::String(value) if value.trim().is_empty() => None,
        _ => Some(value.clone()),
    }
}

fn prompt_creator_knowledge(bundle: &JsonValue) -> Option<JsonValue> {
    let mut compact = JsonMap::new();
    for bucket in ["verified", "flagged", "refuted", "unverified"] {
        let claims = bundle
            .pointer(&format!("/creator_knowledge/{bucket}"))
            .and_then(JsonValue::as_array)
            .map(|claims| {
                claims
                    .iter()
                    .map(|claim| compact_claim_for_prompt(claim, bucket))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if !claims.is_empty() {
            compact.insert(bucket.to_string(), JsonValue::Array(claims));
        }
    }
    if compact.is_empty() {
        None
    } else {
        Some(JsonValue::Object(compact))
    }
}

fn compact_claim_for_prompt(claim: &JsonValue, bucket: &str) -> JsonValue {
    let mut compact = JsonMap::new();
    compact.insert(
        "claim_text".to_string(),
        claim.get("claim_text").cloned().unwrap_or(JsonValue::Null),
    );
    compact.insert("bucket".to_string(), json!(bucket));
    if let Some(title) = claim
        .pointer("/source_video/title")
        .and_then(JsonValue::as_str)
        .filter(|title| !title.trim().is_empty())
    {
        compact.insert("source_video".to_string(), json!({"title": title}));
    }
    if matches!(bucket, "flagged" | "refuted") {
        if let Some(db_evidence) = claim.get("db_evidence").filter(|value| prompt_value_available(value)) {
            compact.insert(
                "db_evidence".to_string(),
                JsonValue::String(short_prompt_text(&value_to_string(Some(db_evidence)), 280)),
            );
        }
    }
    JsonValue::Object(compact)
}

fn prompt_value_available(value: &JsonValue) -> bool {
    match value {
        JsonValue::Null => false,
        JsonValue::Array(items) => !items.is_empty(),
        JsonValue::Object(object) => {
            if object
                .get("available")
                .and_then(JsonValue::as_bool)
                .is_some_and(|available| !available)
            {
                return false;
            }
            !object.is_empty()
        }
        JsonValue::String(value) => !value.trim().is_empty(),
        _ => true,
    }
}

fn short_prompt_text(value: &str, max_chars: usize) -> String {
    let mut end = value.len();
    let mut truncated = false;
    for (count, (index, _)) in value.char_indices().enumerate() {
        if count == max_chars {
            end = index;
            truncated = true;
            break;
        }
    }
    if truncated {
        format!("{}...", &value[..end])
    } else {
        value.to_string()
    }
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

async fn check_required_tables(pool: &PgPool) -> Result<JsonValue> {
    let required = [
        "source_documents",
        "entity_snapshots",
        "entities",
        "entity_aliases",
        "patch_events",
    ];
    let mut missing = Vec::new();
    for table in required {
        if !table_exists(pool, table).await? {
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

async fn check_entity_counts(pool: &PgPool) -> Result<JsonValue> {
    if !tables_exist(pool, &["entity_snapshots", "entities"]).await? {
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
        pool,
        r#"
        SELECT COUNT(*)
        FROM brain.entity_snapshots
        WHERE source=? AND entity_type IN ('hero', 'item_or_ability', 'rank')
        "#,
        vec![SqlValue::Text(ASSETS_SOURCE.to_string())],
    ).await?;
    let mut entity_counts = BTreeMap::new();
    for row in fetch_all(
        pool,
        r#"
        SELECT entity_type, COUNT(*) AS count
        FROM brain.entities
        GROUP BY entity_type
        "#,
        vec![],
    ).await? {
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

async fn check_alias_collisions(pool: &PgPool) -> Result<JsonValue> {
    if !tables_exist(pool, &["entity_aliases", "entities"]).await? {
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
        pool,
        r#"
        SELECT a.alias_norm, COUNT(DISTINCT a.entity_id) AS entities,
               string_agg(DISTINCT e.entity_type || ':' || e.canonical_name, ',') AS targets,
               string_agg(DISTINCT a.alias, ',') AS aliases
        FROM brain.entity_aliases a
        JOIN brain.entities e ON e.id=a.entity_id
        WHERE a.alias_norm <> ''
        GROUP BY a.alias_norm
        HAVING COUNT(DISTINCT a.entity_id) > 1
        ORDER BY entities DESC, a.alias_norm
        LIMIT ?
        "#,
        vec![SqlValue::Integer(SAMPLE_LIMIT)],
    ).await?;
    let total = scalar_i64(
        pool,
        r#"
        SELECT COUNT(*)
        FROM (
          SELECT alias_norm
          FROM brain.entity_aliases
          WHERE alias_norm <> ''
          GROUP BY alias_norm
          HAVING COUNT(DISTINCT entity_id) > 1
        ) AS collisions
        "#,
        vec![],
    ).await?;
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

async fn check_patch_events_unknown_entities(pool: &PgPool) -> Result<JsonValue> {
    if !tables_exist(pool, &["patch_events", "entities", "entity_aliases"]).await? {
        return Ok(result(
            "error",
            "patch_events_unknown_entities",
            "Cannot check patch events because required tables are missing.",
            0,
            Vec::new(),
            None,
        ));
    }
    let known = known_entity_names(pool).await?;
    let mut unknown = Vec::new();
    let rows = fetch_all(
        pool,
        r#"
        SELECT id, patch_external_id, entity_type, entity_name, raw_line
        FROM brain.patch_events
        WHERE entity_type <> 'general' AND entity_name IS NOT NULL AND TRIM(entity_name) <> ''
        ORDER BY id
        "#,
        vec![],
    ).await?;
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

async fn check_enrichment_table(pool: &PgPool) -> Result<JsonValue> {
    if table_exists(pool, "patch_event_enrichments").await? {
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

async fn check_lineage_table(pool: &PgPool) -> Result<JsonValue> {
    if !table_exists(pool, "entity_lineage").await? {
        return Ok(result(
            "warning",
            "missing_lineage_table",
            "Missing entity_lineage table; renamed/reworked legacy names are not linked.",
            0,
            Vec::new(),
            None,
        ));
    }
    let total = scalar_i64(pool, "SELECT COUNT(*) FROM brain.entity_lineage", vec![]).await?;
    let rows = fetch_all(
        pool,
        r#"
        SELECT relation_type, COUNT(*) AS count
        FROM brain.entity_lineage
        GROUP BY relation_type
        ORDER BY relation_type
        "#,
        vec![],
    ).await?;
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

async fn check_legacy_entities_table(pool: &PgPool) -> Result<JsonValue> {
    if !table_exists(pool, "legacy_entities").await? {
        return Ok(result(
            "warning",
            "missing_legacy_entities_table",
            "Missing legacy_entities table; old removed names outside lineage are not modeled.",
            0,
            Vec::new(),
            None,
        ));
    }
    let total = scalar_i64(pool, "SELECT COUNT(*) FROM brain.legacy_entities", vec![]).await?;
    let suspect = scalar_i64(
        pool,
        "SELECT COUNT(*) FROM brain.legacy_entities WHERE status='suspect_parser_subject'",
        vec![],
    ).await?;
    let samples = fetch_all(
        pool,
        r#"
        SELECT legacy_type, canonical_name, event_count, confidence, status
        FROM brain.legacy_entities
        ORDER BY confidence DESC, event_count DESC, canonical_name
        LIMIT ?
        "#,
        vec![SqlValue::Integer(SAMPLE_LIMIT)],
    ).await?;
    let suspect_samples = fetch_all(
        pool,
        r#"
        SELECT legacy_type, canonical_name, event_count, confidence, status
        FROM brain.legacy_entities
        WHERE status='suspect_parser_subject'
        ORDER BY canonical_name
        LIMIT ?
        "#,
        vec![SqlValue::Integer(SAMPLE_LIMIT)],
    ).await?;
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

async fn check_low_confidence_enrichments(pool: &PgPool) -> Result<JsonValue> {
    if !table_exists(pool, "patch_event_enrichments").await? {
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
        pool,
        r#"
        SELECT pee.patch_event_id, pee.confidence, pee.flags::text AS flags_json, pe.patch_external_id,
               pe.entity_type, pe.entity_name, pe.normalized_line
        FROM brain.patch_event_enrichments pee
        LEFT JOIN brain.patch_events pe ON pe.id=pee.patch_event_id
        WHERE pee.confidence < ? OR pee.flags::text LIKE '%unparsed%'
        ORDER BY pee.confidence ASC, pee.patch_event_id ASC
        LIMIT ?
        "#,
        vec![
            SqlValue::Real(LOW_CONFIDENCE_THRESHOLD),
            SqlValue::Integer(SAMPLE_LIMIT),
        ],
    ).await?;
    let total = scalar_i64(
        pool,
        r#"
        SELECT COUNT(*)
        FROM brain.patch_event_enrichments
        WHERE confidence < ? OR flags::text LIKE '%unparsed%'
        "#,
        vec![SqlValue::Real(LOW_CONFIDENCE_THRESHOLD)],
    ).await?;
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

async fn check_sheet_profiles_without_entity(pool: &PgPool) -> Result<JsonValue> {
    if !table_exists(pool, "hero_stat_profiles").await? {
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
        pool,
        r#"
        SELECT id, snapshot_id, hero_name, source, external_id, row_number
        FROM brain.hero_stat_profiles
        WHERE entity_id IS NULL
        ORDER BY hero_name
        LIMIT ?
        "#,
        vec![SqlValue::Integer(SAMPLE_LIMIT)],
    ).await?;
    let total = scalar_i64(
        pool,
        "SELECT COUNT(*) FROM brain.hero_stat_profiles WHERE entity_id IS NULL",
        vec![],
    ).await?;
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

async fn check_general_events_with_known_entity_names(pool: &PgPool) -> Result<JsonValue> {
    if !tables_exist(pool, &["patch_events", "entities", "entity_aliases"]).await? {
        return Ok(result(
            "error",
            "general_events_with_known_entity_names",
            "Cannot scan general events because required tables are missing.",
            0,
            Vec::new(),
            None,
        ));
    }
    let scan_names = entity_scan_names(pool).await?;
    let mut samples = Vec::new();
    let mut total = 0;
    let rows = fetch_all(
        pool,
        r#"
        SELECT id, patch_external_id, section, raw_line, normalized_line
        FROM brain.patch_events
        WHERE entity_type='general'
        ORDER BY id
        "#,
        vec![],
    ).await?;
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

async fn known_entity_names(pool: &PgPool) -> Result<HashSet<(String, String)>> {
    let mut known = HashSet::new();
    for row in fetch_all(pool, "SELECT entity_type, canonical_name FROM brain.entities", vec![]).await? {
        known.insert((
            value_to_string(row.get("entity_type")),
            normalize_alias(&value_to_string(row.get("canonical_name"))),
        ));
    }
    for row in fetch_all(
        pool,
        r#"
        SELECT e.entity_type, a.alias_norm
        FROM brain.entity_aliases a
        JOIN brain.entities e ON e.id=a.entity_id
        "#,
        vec![],
    ).await? {
        known.insert((
            value_to_string(row.get("entity_type")),
            value_to_string(row.get("alias_norm")),
        ));
    }
    if table_exists(pool, "entity_lineage").await? {
        for row in fetch_all(
            pool,
            "SELECT source_entity_type, source_name_norm, target_entity_type, target_name_norm FROM brain.entity_lineage",
            vec![],
        ).await? {
            add_lineage_known_name(&mut known, row.get("source_entity_type"), row.get("source_name_norm"));
            add_lineage_known_name(&mut known, row.get("target_entity_type"), row.get("target_name_norm"));
        }
    }
    if table_exists(pool, "legacy_entities").await? {
        for row in fetch_all(
            pool,
            "SELECT observed_entity_type, name_norm FROM brain.legacy_entities",
            vec![],
        ).await? {
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

async fn entity_scan_names(pool: &PgPool) -> Result<Vec<ScanName>> {
    let mut by_key: BTreeMap<(String, String, String), String> = BTreeMap::new();
    for row in fetch_all(
        pool,
        r#"
        SELECT entity_type, canonical_name
        FROM brain.entities
        WHERE source=? AND entity_type IN ('hero', 'item', 'item_special', 'ability')
        "#,
        vec![SqlValue::Text(ASSETS_SOURCE.to_string())],
    ).await? {
        let entity_type = value_to_string(row.get("entity_type"));
        let canonical_name = value_to_string(row.get("canonical_name"));
        add_scan_name(&mut by_key, &entity_type, &canonical_name, &canonical_name);
    }
    for row in fetch_all(
        pool,
        r#"
        SELECT e.entity_type, e.canonical_name, a.alias
        FROM brain.entity_aliases a
        JOIN brain.entities e ON e.id=a.entity_id
        WHERE a.source=? AND e.entity_type IN ('hero', 'item', 'item_special', 'ability')
        "#,
        vec![SqlValue::Text(ASSETS_SOURCE.to_string())],
    ).await? {
        add_scan_name(
            &mut by_key,
            &value_to_string(row.get("entity_type")),
            &value_to_string(row.get("canonical_name")),
            &value_to_string(row.get("alias")),
        );
    }
    if table_exists(pool, "entity_lineage").await? {
        for row in fetch_all(
            pool,
            r#"
            SELECT source_entity_type, source_name, target_entity_type, target_name
            FROM brain.entity_lineage
            WHERE relation_type IN ('rename', 'replaced_by')
            "#,
            vec![],
        ).await? {
            let source_type = value_to_nonempty_string(row.get("source_entity_type")).unwrap_or_else(|| "legacy".to_string());
            let source_name = value_to_string(row.get("source_name"));
            add_scan_name(&mut by_key, &source_type, &source_name, &source_name);
            let target_type = value_to_nonempty_string(row.get("target_entity_type")).unwrap_or_else(|| "legacy".to_string());
            let target_name = value_to_string(row.get("target_name"));
            add_scan_name(&mut by_key, &target_type, &target_name, &target_name);
        }
    }
    if table_exists(pool, "legacy_entities").await? {
        for row in fetch_all(
            pool,
            r#"
            SELECT legacy_type, canonical_name
            FROM brain.legacy_entities
            WHERE confidence >= 0.5
            "#,
            vec![],
        ).await? {
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

async fn load_entity_payload(
    pool: &PgPool,
    query: &str,
    entity_type: &str,
) -> Result<Option<JsonMap<String, JsonValue>>> {
    let context = build_entity_context(pool, query, 1).await?;
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
    let row = fetch_one(
        pool,
        r#"
            SELECT s.payload::text AS payload_json
            FROM brain.entity_aliases a
            JOIN brain.entity_snapshots s ON s.id=a.snapshot_id
            WHERE a.entity_id=? AND s.source='deadlock_assets_api'
            ORDER BY CASE s.entity_type WHEN 'hero' THEN 0 WHEN 'item_or_ability' THEN 0 ELSE 1 END, s.id
            LIMIT 1
            "#,
        vec![SqlValue::Integer(entity_id)],
    ).await?;
    Ok(row
        .and_then(|row| value_to_nonempty_string(row.get("payload_json")))
        .map(|text| loads_json_object(Some(&JsonValue::String(text)))))
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

async fn save_review_analysis_note(
    pool: &PgPool,
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
    sqlx::query(
        r#"
        INSERT INTO brain.analysis_notes(
          query, entity_type, entity_name, context_kind, context_hash,
          prompt_version, prompt_text, result_text, model, confidence, status,
          source_references, context, created_at, updated_at
        )
        VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12::jsonb,$13::jsonb, now(), now())
        ON CONFLICT (query, context_hash, prompt_version, COALESCE(model, ''::text), status) DO UPDATE SET
          prompt_text=excluded.prompt_text,
          result_text=excluded.result_text,
          confidence=excluded.confidence,
          source_references=excluded.source_references,
          context=excluded.context,
          updated_at=excluded.updated_at
        "#,
    )
    .bind(&query)
    .bind(entity_type.as_deref())
    .bind(entity_name.as_deref())
    .bind(&context_kind)
    .bind(&context_hash)
    .bind(PROMPT_VERSION)
    .bind(&prompt_text)
    .bind(result_text)
    .bind(model)
    .bind(confidence)
    .bind(status)
    .bind(&source_references_json)
    .bind(&context_json)
    .execute(pool)
    .await?;
    let row = fetch_one(
        pool,
        r#"
        SELECT id, query, entity_type, entity_name, context_hash, prompt_version,
               model, status, created_at, updated_at
        FROM brain.analysis_notes
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
    ).await?;
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










#[derive(Debug, Clone)]
enum SqlValue {
    Integer(i64),
    Real(f64),
    Text(String),
    Null,
}

fn qualified_table(table: &str) -> String {
    format!("brain.{}", quote_identifier(table))
}

fn to_pg_placeholders(sql: &str) -> String {
    let mut out = String::with_capacity(sql.len() + 16);
    let mut index = 0_u32;
    let mut in_string = false;
    for ch in sql.chars() {
        if ch == '\'' {
            in_string = !in_string;
            out.push(ch);
        } else if ch == '?' && !in_string {
            index += 1;
            out.push('$');
            out.push_str(&index.to_string());
        } else {
            out.push(ch);
        }
    }
    out
}

fn bind_sql<'q>(
    mut query: sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments>,
    values: &'q [SqlValue],
) -> sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments> {
    for value in values {
        query = match value {
            SqlValue::Integer(inner) => query.bind(*inner),
            SqlValue::Real(inner) => query.bind(*inner),
            SqlValue::Text(inner) => query.bind(inner.as_str()),
            SqlValue::Null => query.bind(Option::<String>::None),
        };
    }
    query
}

fn decode_pg_column(row: &PgRow, column: &PgColumn) -> JsonValue {
    let ordinal = column.ordinal();
    match column.type_info().name() {
        "INT8" => row
            .try_get::<Option<i64>, _>(ordinal)
            .ok()
            .flatten()
            .map(|value| json!(value))
            .unwrap_or(JsonValue::Null),
        "INT4" => row
            .try_get::<Option<i32>, _>(ordinal)
            .ok()
            .flatten()
            .map(|value| json!(i64::from(value)))
            .unwrap_or(JsonValue::Null),
        "INT2" => row
            .try_get::<Option<i16>, _>(ordinal)
            .ok()
            .flatten()
            .map(|value| json!(i64::from(value)))
            .unwrap_or(JsonValue::Null),
        "FLOAT8" => row
            .try_get::<Option<f64>, _>(ordinal)
            .ok()
            .flatten()
            .and_then(serde_json::Number::from_f64)
            .map(JsonValue::Number)
            .unwrap_or(JsonValue::Null),
        "FLOAT4" => row
            .try_get::<Option<f32>, _>(ordinal)
            .ok()
            .flatten()
            .and_then(|value| serde_json::Number::from_f64(f64::from(value)))
            .map(JsonValue::Number)
            .unwrap_or(JsonValue::Null),
        "BOOL" => row
            .try_get::<Option<bool>, _>(ordinal)
            .ok()
            .flatten()
            .map(JsonValue::Bool)
            .unwrap_or(JsonValue::Null),
        "JSON" | "JSONB" => row
            .try_get::<Option<JsonValue>, _>(ordinal)
            .ok()
            .flatten()
            .unwrap_or(JsonValue::Null),
        "TIMESTAMPTZ" => row
            .try_get::<Option<DateTime<Utc>>, _>(ordinal)
            .ok()
            .flatten()
            .map(|value| JsonValue::String(value.format("%Y-%m-%d %H:%M:%S%:z").to_string()))
            .unwrap_or(JsonValue::Null),
        "TIMESTAMP" => row
            .try_get::<Option<NaiveDateTime>, _>(ordinal)
            .ok()
            .flatten()
            .map(|value| JsonValue::String(value.format("%Y-%m-%d %H:%M:%S").to_string()))
            .unwrap_or(JsonValue::Null),
        "DATE" => row
            .try_get::<Option<NaiveDate>, _>(ordinal)
            .ok()
            .flatten()
            .map(|value| JsonValue::String(value.format("%Y-%m-%d").to_string()))
            .unwrap_or(JsonValue::Null),
        _ => row
            .try_get::<Option<String>, _>(ordinal)
            .ok()
            .flatten()
            .map(JsonValue::String)
            .unwrap_or(JsonValue::Null),
    }
}

fn pg_row_to_json_map(row: &PgRow) -> JsonMap<String, JsonValue> {
    let mut object = JsonMap::new();
    for column in row.columns() {
        object.insert(column.name().to_string(), decode_pg_column(row, column));
    }
    object
}

async fn select_by_values_i64(
    pool: &PgPool,
    table: &str,
    column: &str,
    values: &[i64],
) -> Result<Vec<JsonMap<String, JsonValue>>> {
    let sql = format!(
        "SELECT * FROM {} WHERE {} IN ({})",
        qualified_table(table),
        quote_identifier(column),
        placeholders(values.len())
    );
    fetch_all(
        pool,
        &sql,
        values.iter().copied().map(SqlValue::Integer).collect(),
    ).await
}

async fn select_by_values_text(
    pool: &PgPool,
    table: &str,
    column: &str,
    values: &[String],
) -> Result<Vec<JsonMap<String, JsonValue>>> {
    let sql = format!(
        "SELECT * FROM {} WHERE {} IN ({})",
        qualified_table(table),
        quote_identifier(column),
        placeholders(values.len())
    );
    fetch_all(
        pool,
        &sql,
        values.iter().cloned().map(SqlValue::Text).collect(),
    ).await
}

async fn table_exists(pool: &PgPool, name: &str) -> Result<bool> {
    let qualified = format!("brain.{name}");
    let row = sqlx::query("SELECT to_regclass($1) IS NOT NULL AS present")
        .bind(&qualified)
        .fetch_one(pool)
        .await?;
    Ok(row
        .try_get::<Option<bool>, _>("present")
        .ok()
        .flatten()
        .unwrap_or(false))
}

async fn tables_exist(pool: &PgPool, tables: &[&str]) -> Result<bool> {
    for table in tables {
        if !table_exists(pool, table).await? {
            return Ok(false);
        }
    }
    Ok(true)
}

async fn table_columns(pool: &PgPool, table: &str) -> Result<Vec<String>> {
    let rows = fetch_all(
        pool,
        "SELECT column_name AS name FROM information_schema.columns WHERE table_schema='brain' AND table_name=? ORDER BY ordinal_position",
        vec![SqlValue::Text(table.to_string())],
    ).await?;
    Ok(rows
        .into_iter()
        .filter_map(|row| value_to_nonempty_string(row.get("name")))
        .collect())
}

async fn fetch_one(
    pool: &PgPool,
    sql: &str,
    values: Vec<SqlValue>,
) -> Result<Option<JsonMap<String, JsonValue>>> {
    Ok(fetch_all(pool, sql, values).await?.into_iter().next())
}

async fn fetch_all(
    pool: &PgPool,
    sql: &str,
    values: Vec<SqlValue>,
) -> Result<Vec<JsonMap<String, JsonValue>>> {
    let pg_sql = to_pg_placeholders(sql);
    let rows = bind_sql(sqlx::query(&pg_sql), &values)
        .fetch_all(pool)
        .await?;
    Ok(rows.iter().map(pg_row_to_json_map).collect())
}

async fn scalar_i64(pool: &PgPool, sql: &str, values: Vec<SqlValue>) -> Result<i64> {
    let pg_sql = to_pg_placeholders(sql);
    let row = bind_sql(sqlx::query(&pg_sql), &values)
        .fetch_optional(pool)
        .await?;
    Ok(row
        .map(|row| row.try_get::<i64, _>(0).unwrap_or(0))
        .unwrap_or(0))
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

fn tokenize_ascii_lower(value: &str) -> Vec<String> {
    let mut current = String::new();
    let mut tokens = Vec::new();
    for character in value.to_lowercase().chars() {
        if character.is_ascii_alphanumeric() {
            current.push(character);
        } else if !current.is_empty() {
            tokens.push(std::mem::take(&mut current));
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
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

    use sqlx::postgres::PgPoolOptions;

    async fn test_pool() -> Option<PgPool> {
        let dsn = std::env::var("DEADLOCK_CENTRAL_DSN").ok()?;
        if dsn.trim().is_empty() {
            return None;
        }
        Some(
            PgPoolOptions::new()
                .max_connections(2)
                .connect(&dsn)
                .await
                .expect("connect scratch postgres"),
        )
    }

    async fn pick_hero_with_events(pool: &PgPool) -> Option<String> {
        let row = sqlx::query(
            "SELECT e.canonical_name \
             FROM brain.entities e \
             JOIN brain.patch_events pe ON lower(pe.entity_name) = lower(e.canonical_name) \
             WHERE e.entity_type = 'hero' \
             GROUP BY e.canonical_name \
             ORDER BY COUNT(*) DESC LIMIT 1",
        )
        .fetch_optional(pool)
        .await
        .expect("hero query");
        row.and_then(|row| row.try_get::<Option<String>, _>("canonical_name").ok().flatten())
    }

    #[tokio::test]
    #[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
    async fn load_known_item_names_matches_catalog() {
        let Some(pool) = test_pool().await else {
            return;
        };
        let names = load_known_item_names(&pool).await.expect("item names");
        let raw_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM brain.item_catalog")
            .fetch_one(&pool)
            .await
            .expect("count");
        let distinct_names: i64 =
            sqlx::query_scalar("SELECT COUNT(DISTINCT name) FROM brain.item_catalog WHERE trim(name) <> ''")
                .fetch_one(&pool)
                .await
                .expect("distinct count");
        // item_catalog has 251 rows; names are de-duplicated (one item name appears twice),
        // so the loaded set matches the distinct non-empty name count.
        assert_eq!(raw_count, 251);
        assert_eq!(names.len() as i64, distinct_names);
    }

    #[tokio::test]
    #[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
    async fn status_reports_core_counts() {
        let Some(pool) = test_pool().await else {
            return;
        };
        let report = status(&pool).await.expect("status");
        let events: i64 = report
            .get("patch_events")
            .and_then(JsonValue::as_array)
            .map(|rows| {
                rows.iter()
                    .filter_map(|row| row.get("events").and_then(JsonValue::as_i64))
                    .sum()
            })
            .unwrap_or(0);
        assert_eq!(events, 13383);
        assert!(report
            .get("entities")
            .and_then(JsonValue::as_array)
            .map(|rows| !rows.is_empty())
            .unwrap_or(false));
    }

    #[tokio::test]
    #[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
    async fn context_loads_patch_events_for_known_hero() {
        let Some(pool) = test_pool().await else {
            return;
        };
        let Some(hero) = pick_hero_with_events(&pool).await else {
            return;
        };
        let ctx = context(&pool, &hero, 50).await.expect("context");
        let events = ctx
            .get("patch_events")
            .and_then(JsonValue::as_array)
            .cloned()
            .unwrap_or_default();
        assert!(!events.is_empty(), "expected patch events for {hero}");
        assert!(ctx
            .get("best_match")
            .map(|value| !value.is_null())
            .unwrap_or(false));
    }

    #[tokio::test]
    #[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
    async fn timeline_patch_events_keep_best_match_entity_type() {
        let Some(pool) = test_pool().await else {
            return;
        };
        let hollow_rows: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM brain.patch_events WHERE entity_type='item' AND entity_name='Hollow Point'",
        )
        .fetch_one(&pool)
        .await
        .expect("hollow point fixture count");
        assert!(hollow_rows > 0, "fixture must contain Hollow Point item rows");

        let mut best_match = JsonMap::new();
        best_match.insert("entity_type".to_string(), json!("hero"));
        best_match.insert("canonical_name".to_string(), json!("Holliday"));
        let lineage_names = vec!["Hollow Point".to_string()];
        let events = load_patch_events(
            &pool,
            "Holliday",
            Some(&best_match),
            &[],
            &lineage_names,
            2_000,
            PatchEventMode::Timeline { ascending: true },
        )
        .await
        .expect("patch events");

        assert!(!events.is_empty(), "expected Holliday patch events");
        assert!(
            events
                .iter()
                .all(|event| value_to_string(event.get("entity_type")) == "hero"),
            "hero lookup must not return item patch_events: {:?}",
            events
                .iter()
                .filter(|event| value_to_string(event.get("entity_type")) != "hero")
                .collect::<Vec<_>>()
        );
    }

    #[tokio::test]
    #[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
    async fn build_review_context_returns_summary() {
        let Some(pool) = test_pool().await else {
            return;
        };
        let Some(hero) = pick_hero_with_events(&pool).await else {
            return;
        };
        let review = build_review_context(&pool, &hero, 40)
            .await
            .expect("review");
        assert!(review.get("entity_summary").is_some());
        assert!(review.get("source_references").is_some());
        assert!(review.get("timeline_signals").is_some());
    }

    #[tokio::test]
    #[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
    async fn ask_context_returns_structured_bundle() {
        let Some(pool) = test_pool().await else {
            return;
        };
        let Some(hero) = pick_hero_with_events(&pool).await else {
            return;
        };
        let opts = AskContextOptions {
            limit_events: 40,
            include_unverified: false,
            max_claims: 12,
        };
        let bundle = ask_context(&pool, &format!("{hero} matchup"), &opts)
            .await
            .expect("ask context");
        assert!(bundle.get("intent").is_some());
        assert!(bundle.get("ground_truth").is_some());
        assert!(bundle.get("creator_knowledge").is_some());
    }

    #[tokio::test]
    #[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
    async fn analyze_query_resolves_known_hero() {
        let Some(pool) = test_pool().await else {
            return;
        };
        let Some(hero) = pick_hero_with_events(&pool).await else {
            return;
        };
        let plan = analyze_query(&pool, &format!("{hero} build"))
            .await
            .expect("plan");
        assert!(!plan.entities.is_empty());
    }

    #[tokio::test]
    #[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
    async fn analysis_list_returns_array() {
        let Some(pool) = test_pool().await else {
            return;
        };
        let list = analysis_list(&pool, None, 10).await.expect("list");
        assert!(list.is_array());
    }

    #[tokio::test]
    #[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
    async fn quality_reports_required_tables_present() {
        let Some(pool) = test_pool().await else {
            return;
        };
        let report = quality(&pool).await.expect("quality");
        let checks = report
            .get("checks")
            .and_then(JsonValue::as_array)
            .cloned()
            .unwrap_or_default();
        assert!(!checks.is_empty());
        let required = checks
            .iter()
            .find(|check| check.get("check").and_then(JsonValue::as_str) == Some("required_tables"))
            .expect("required_tables check");
        assert_eq!(
            required.get("severity").and_then(JsonValue::as_str),
            Some("ok")
        );
    }

    #[tokio::test]
    #[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
    async fn save_review_roundtrip_inserts_and_cleans_up() {
        let Some(pool) = test_pool().await else {
            return;
        };
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let query = format!("__pgtest__{nanos}");
        let review = build_review_context(&pool, &query, 5)
            .await
            .expect("review");
        let note = analysis_save_review(&pool, &review, None, None, None, "context_ready", None)
            .await
            .expect("save review");
        let context_hash = note
            .get("context_hash")
            .and_then(JsonValue::as_str)
            .map(str::to_string);
        assert!(context_hash.is_some(), "expected context_hash in saved note");
        if let Some(hash) = context_hash {
            let deleted = sqlx::query("DELETE FROM brain.analysis_notes WHERE context_hash=$1")
                .bind(&hash)
                .execute(&pool)
                .await
                .expect("cleanup");
            assert!(deleted.rows_affected() >= 1);
        }
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
            relevance_score: 1,
            relevance_floor_score: 1,
            matched_keyword_count: 0,
        }
    }

    fn claim_row(claim_text: &str, evidence_quote: &str, entity_name: &str) -> JsonMap<String, JsonValue> {
        let mut row = JsonMap::new();
        row.insert("claim_text".to_string(), json!(claim_text));
        row.insert("evidence_quote".to_string(), json!(evidence_quote));
        row.insert("entity_name".to_string(), json!(entity_name));
        row
    }

    fn entity_match_fixture(name: &str, entity_type: &str) -> AskEntityClaimMatch {
        AskEntityClaimMatch {
            names: vec![name.to_string()],
            matched: true,
            canonical_name: Some(name.to_string()),
            entity_type: Some(entity_type.to_string()),
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
            "hero_overview",
            &AskEntityClaimMatch::default(),
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
            "hero_overview",
            &AskEntityClaimMatch::default(),
        );
        let with_unverified = partition_ask_claims(
            vec![
                ask_claim_fixture(1, "accepted", 0.9),
                ask_claim_fixture(2, "unverified", 0.8),
            ],
            true,
            10,
            "hero_overview",
            &AskEntityClaimMatch::default(),
        );

        assert!(without_unverified.unverified.is_empty());
        assert_eq!(without_unverified.omitted.unverified, 1);
        assert_eq!(with_unverified.unverified.len(), 1);
        assert_eq!(with_unverified.omitted.unverified, 0);
    }

    #[test]
    fn relevance_score_ranks_more_keyword_hits_before_confidence() {
        let entity_match = AskEntityClaimMatch::default();
        let keywords = ask_query_keyword_specs("soul denying advantage", &entity_match);
        let mut two_hits = ask_claim_fixture(1, "accepted", 0.2);
        two_hits.claim_text = "Soul denying creates a double advantage.".to_string();
        let idf = AskIdfWeights::default();
        let relevance = ask_claim_relevance(&two_hits, &keywords, &entity_match, &idf);
        two_hits.relevance_score = relevance.score;
        two_hits.relevance_floor_score = relevance.score;
        two_hits.matched_keyword_count = relevance.distinct_matches;

        let mut one_hit = ask_claim_fixture(2, "accepted", 0.99);
        one_hit.claim_text = "Soul denying matters.".to_string();
        let relevance = ask_claim_relevance(&one_hit, &keywords, &entity_match, &idf);
        one_hit.relevance_score = relevance.score;
        one_hit.relevance_floor_score = relevance.score;
        one_hit.matched_keyword_count = relevance.distinct_matches;

        let buckets = partition_ask_claims(
            vec![one_hit, two_hits],
            false,
            10,
            "mechanics_question",
            &entity_match,
        );

        assert_eq!(buckets.verified[0].id, 1);
        assert!(buckets.verified[0].relevance_score > buckets.verified[1].relevance_score);
    }

    #[test]
    fn final_verified_json_is_sorted_by_relevance_before_cutoff() {
        let entity_match = AskEntityClaimMatch::default();
        let mut low = ask_claim_fixture(701, "accepted", 0.99);
        low.claim_type = "mechanic".to_string();
        low.relevance_score = 30;
        low.relevance_floor_score = 30;
        let mut high = ask_claim_fixture(702, "accepted", 0.1);
        high.claim_type = "mechanic".to_string();
        high.relevance_score = 40;
        high.relevance_floor_score = 40;
        let mut mid = ask_claim_fixture(703, "accepted", 0.5);
        mid.claim_type = "mechanic".to_string();
        mid.relevance_score = 35;
        mid.relevance_floor_score = 35;

        let buckets = partition_ask_claims(
            vec![low, mid, high],
            false,
            2,
            "mechanics_question",
            &entity_match,
        );
        let JsonValue::Array(serialized) = claims_to_json(&buckets.verified) else {
            panic!("claims json array");
        };
        let rel_scores = serialized
            .iter()
            .filter_map(|claim| claim.get("relevance_score").and_then(JsonValue::as_i64))
            .collect::<Vec<_>>();

        assert_eq!(buckets.verified.iter().map(|claim| claim.id).collect::<Vec<_>>(), vec![702, 703]);
        assert_eq!(rel_scores, vec![40, 35]);
        assert_eq!(buckets.omitted.verified, 1);
    }

    #[test]
    fn on_intent_flagged_claims_keep_slots_over_tangential_verified_claims() {
        let entity_match = entity_match_fixture("Lash", "hero");
        let mut accepted_a = ask_claim_fixture(301, "accepted", 0.9);
        accepted_a.claim_type = "general".to_string();
        accepted_a.entity_name = "Lash".to_string();
        accepted_a.relevance_score = 24;
        accepted_a.relevance_floor_score = 24;
        let mut accepted_b = ask_claim_fixture(302, "accepted", 0.8);
        accepted_b.claim_type = "general".to_string();
        accepted_b.entity_name = "Lash".to_string();
        accepted_b.relevance_score = 23;
        accepted_b.relevance_floor_score = 23;
        let mut accepted_c = ask_claim_fixture(303, "accepted", 0.7);
        accepted_c.claim_type = "general".to_string();
        accepted_c.entity_name = "Lash".to_string();
        accepted_c.relevance_score = 22;
        accepted_c.relevance_floor_score = 22;
        let mut flagged_a = ask_claim_fixture(304, "needs_review", 0.6);
        flagged_a.claim_type = "counterplay".to_string();
        flagged_a.entity_name = "Lash".to_string();
        flagged_a.relevance_score = 30;
        flagged_a.relevance_floor_score = 30;
        let mut flagged_b = ask_claim_fixture(305, "needs_review", 0.5);
        flagged_b.claim_type = "matchup".to_string();
        flagged_b.entity_name = "Lash".to_string();
        flagged_b.relevance_score = 28;
        flagged_b.relevance_floor_score = 28;

        let buckets = partition_ask_claims(
            vec![accepted_a, accepted_b, accepted_c, flagged_a, flagged_b],
            false,
            4,
            "matchup",
            &entity_match,
        );

        assert_eq!(buckets.flagged.len(), 2);
        assert_eq!(
            buckets.flagged.iter().map(|claim| claim.id).collect::<Vec<_>>(),
            vec![304, 305]
        );
        assert_eq!(buckets.omitted.verified, 1);
    }

    #[test]
    fn thin_verified_bucket_gets_on_topic_unverified_fallback() {
        let entity_match = AskEntityClaimMatch::default();
        let mut verified = ask_claim_fixture(401, "accepted", 0.9);
        verified.claim_type = "mechanic".to_string();
        verified.relevance_score = 20;
        verified.relevance_floor_score = 20;
        let mut unverified_a = ask_claim_fixture(402, "unverified", 0.7);
        unverified_a.claim_type = "mechanic".to_string();
        unverified_a.relevance_score = 18;
        unverified_a.relevance_floor_score = 18;
        let mut unverified_b = ask_claim_fixture(403, "unverified", 0.6);
        unverified_b.claim_type = "mechanic".to_string();
        unverified_b.relevance_score = 17;
        unverified_b.relevance_floor_score = 17;
        let mut unverified_c = ask_claim_fixture(404, "unverified", 0.5);
        unverified_c.claim_type = "mechanic".to_string();
        unverified_c.relevance_score = 16;
        unverified_c.relevance_floor_score = 16;

        let buckets = partition_ask_claims(
            vec![verified, unverified_a, unverified_b, unverified_c],
            false,
            12,
            "mechanics_question",
            &entity_match,
        );

        assert_eq!(buckets.verified.len(), 1);
        assert_eq!(buckets.unverified.len(), 2);
        assert_eq!(
            buckets.unverified.iter().map(|claim| claim.id).collect::<Vec<_>>(),
            vec![402, 403]
        );
    }

    #[test]
    fn relevance_floor_drops_non_positive_and_low_tail_claims() {
        let entity_match = AskEntityClaimMatch::default();
        let mut top = ask_claim_fixture(501, "accepted", 0.9);
        top.relevance_score = 12;
        top.relevance_floor_score = 12;
        let mut kept = ask_claim_fixture(502, "accepted", 0.8);
        kept.relevance_score = 3;
        kept.relevance_floor_score = 3;
        let mut low_tail = ask_claim_fixture(503, "accepted", 0.7);
        low_tail.relevance_score = 2;
        low_tail.relevance_floor_score = 2;
        let mut zero = ask_claim_fixture(504, "accepted", 0.6);
        zero.relevance_score = 0;
        zero.relevance_floor_score = 0;
        let mut negative = ask_claim_fixture(505, "accepted", 0.5);
        negative.relevance_score = -4;
        negative.relevance_floor_score = -4;

        let buckets = partition_ask_claims(
            vec![top, kept, low_tail, zero, negative],
            false,
            12,
            "hero_overview",
            &entity_match,
        );

        assert_eq!(
            buckets.verified.iter().map(|claim| claim.id).collect::<Vec<_>>(),
            vec![501, 502]
        );
        assert!(buckets
            .verified
            .iter()
            .all(|claim| claim.relevance_score > 0));
        assert!(buckets.verified.len() < 12);
    }




    #[test]
    fn keyword_starts_word_filters_mid_word_matches() {
        assert!(!keyword_starts_word("crimson slash", "lash"));
        assert!(!keyword_starts_word("flash farming", "lash"));
        assert!(!keyword_starts_word("automatisch", "auto"));
        assert!(keyword_starts_word("lash's ground strike", "lash"));
        assert!(keyword_starts_word("denying souls", "soul"));
        assert!(keyword_starts_word("the disarming hex counter", "disarming"));
    }

    #[test]
    fn keyword_phrase_keeps_echo_shard_from_echo_noise() {
        let entity_match = entity_match_fixture("Echo Shard", "item");
        let keywords = ask_query_keyword_specs("Echo Shard", &entity_match);
        let echo_shard = claim_row("Echo Shard gives another cast.", "", "Echo Shard");
        let echo_charge = claim_row("Echo Charge improves tempo.", "", "Echo Charge");

        assert!(keyword_row_passes_minimum_relevance(
            &echo_shard,
            &keywords,
            &entity_match,
            &AskIdfWeights::default()
        ));
        assert!(!keyword_row_passes_minimum_relevance(
            &echo_charge,
            &keywords,
            &entity_match,
            &AskIdfWeights::default()
        ));
    }


    #[test]
    fn build_intent_prefers_owned_build_claim_over_general_mention() {
        let entity_match = entity_match_fixture("Seven", "hero");
        let keywords = ask_query_keyword_specs("Seven build", &entity_match);
        let mut build_claim = ask_claim_fixture(201, "accepted", 0.5);
        build_claim.claim_text = "Seven wants Arcane Surge early.".to_string();
        build_claim.entity_name = "Seven".to_string();
        build_claim.claim_type = "build".to_string();
        let idf = AskIdfWeights::default();
        let relevance = ask_claim_relevance(&build_claim, &keywords, &entity_match, &idf);
        build_claim.relevance_floor_score = relevance.score;
        build_claim.relevance_score = relevance.score
            + claim_intent_relevance_bonus(&build_claim, "build_recommendation", &entity_match, &relevance);

        let mut general_claim = ask_claim_fixture(202, "accepted", 0.99);
        general_claim.claim_text = "Seven is mentioned in a broad meta note.".to_string();
        general_claim.entity_name = "Seven".to_string();
        general_claim.claim_type = "general".to_string();
        let relevance = ask_claim_relevance(&general_claim, &keywords, &entity_match, &idf);
        general_claim.relevance_floor_score = relevance.score;
        general_claim.relevance_score = relevance.score
            + claim_intent_relevance_bonus(&general_claim, "build_recommendation", &entity_match, &relevance);

        let buckets = partition_ask_claims(
            vec![general_claim, build_claim],
            false,
            2,
            "build_recommendation",
            &entity_match,
        );

        assert_eq!(buckets.verified.first().map(|claim| claim.id), Some(201));
    }

    #[test]
    fn intent_claim_type_bonus_requires_distinctive_keyword_hit() {
        let entity_match = AskEntityClaimMatch::default();
        let keywords = ask_query_keyword_specs("surge items", &entity_match);
        let mut weights = BTreeMap::new();
        weights.insert("surge".to_string(), 24);
        weights.insert("items".to_string(), 1);
        let idf = AskIdfWeights { weights };

        let mut build_claim = ask_claim_fixture(901, "accepted", 0.5);
        build_claim.claim_type = "build".to_string();
        build_claim.claim_text = "Buy Surge before flexing into damage.".to_string();
        let build_relevance = ask_claim_relevance(&build_claim, &keywords, &entity_match, &idf);
        build_claim.relevance_floor_score = build_relevance.score;
        build_claim.relevance_score = build_relevance.score
            + claim_intent_relevance_bonus(
                &build_claim,
                "build_recommendation",
                &entity_match,
                &build_relevance,
            );

        let mut mechanic_claim = ask_claim_fixture(902, "accepted", 0.9);
        mechanic_claim.claim_type = "mechanic".to_string();
        mechanic_claim.claim_text = "Items can change your combat pattern.".to_string();
        let mechanic_relevance = ask_claim_relevance(&mechanic_claim, &keywords, &entity_match, &idf);
        let mechanic_bonus = claim_intent_relevance_bonus(
            &mechanic_claim,
            "mechanics_question",
            &entity_match,
            &mechanic_relevance,
        );
        mechanic_claim.relevance_floor_score = mechanic_relevance.score;
        mechanic_claim.relevance_score = mechanic_relevance.score
            + claim_intent_relevance_bonus(
                &mechanic_claim,
                "build_recommendation",
                &entity_match,
                &mechanic_relevance,
            );

        let buckets = partition_ask_claims(
            vec![mechanic_claim, build_claim],
            false,
            2,
            "build_recommendation",
            &entity_match,
        );

        assert!(build_relevance.distinctive_keyword_hit);
        assert!(!mechanic_relevance.distinctive_keyword_hit);
        assert!(mechanic_bonus < 60);
        assert_eq!(buckets.verified.first().map(|claim| claim.id), Some(901));
    }

    #[test]
    fn relevance_floor_uses_score_without_intent_claim_type_bonus() {
        let entity_match = AskEntityClaimMatch::default();
        let mut specific = ask_claim_fixture(911, "accepted", 0.5);
        specific.claim_type = "mechanic".to_string();
        specific.relevance_floor_score = 20;
        specific.relevance_score = 80;
        let mut generic = ask_claim_fixture(912, "accepted", 0.9);
        generic.claim_type = "mechanic".to_string();
        generic.relevance_floor_score = 4;
        generic.relevance_score = 64;

        let buckets = partition_ask_claims(
            vec![generic, specific],
            false,
            12,
            "mechanics_question",
            &entity_match,
        );

        assert_eq!(buckets.verified.iter().map(|claim| claim.id).collect::<Vec<_>>(), vec![911]);
        assert_eq!(buckets.omitted.verified, 0);
    }





    #[test]
    fn ask_prompt_preserves_timeline_details_and_hides_debug_claim_fields() {
        let bundle = json!({
            "query": "was kontert Lash",
            "intent": "matchup",
            "ground_truth": {
                "stats": {"available": false},
                "lineage": null,
                "item": null,
                "timeline": {
                    "latest_patch": {"title": "Patch 1"},
                    "change_type_counts": [{"key": "buff", "count": 2}],
                    "recent_events": [{"raw_line": "full event"}],
                    "stat_changes": [{"stat_name": "damage"}],
                    "ability_mentions": [{"key": "Ground Strike", "count": 4}]
                }
            },
            "creator_knowledge": {
                "verified": [{
                    "claim_text": "Reactive Barrier is strong into Lash engage.",
                    "claim_type": "counterplay",
                    "entity_name": "Lash",
                    "status": "accepted",
                    "verifier_confidence": 0.9,
                    "relevance_score": 42,
                    "matched_keyword_count": 2,
                    "source_video": {"video_id": "v1", "title": "Lash Counters"},
                    "verdict": "supported",
                    "db_evidence": {"debug": "hidden"},
                    "db_value": {"debug": true},
                    "match_sources": ["keyword"]
                }],
                "flagged": [],
                "refuted": [],
                "unverified": []
            },
            "sources": [],
            "retrieval_meta": {"out_of_domain": false}
        });
        let prompt = render_ask_prompt(&bundle).expect("prompt");

        assert!(prompt.contains("latest_patch"));
        assert!(prompt.contains("change_type_counts"));
        assert!(prompt.contains("recent_events"));
        assert!(prompt.contains("stat_changes"));
        assert!(prompt.contains("ability_mentions"));
        assert!(!prompt.contains("relevance_score"));
        assert!(!prompt.contains("match_sources"));
        assert!(!prompt.contains("verifier_confidence"));
        assert!(!prompt.contains("db_value"));
        assert!(!prompt.contains("status"));
        assert!(prompt.contains("Lash Counters"));
    }

    #[test]
    fn patch_overview_routing_requires_broad_patch_question() {
        let mut plan = QueryPlan {
            entities: Vec::new(),
            threats: Vec::new(),
            intent: "patch_changes".to_string(),
            fetch: Vec::new(),
            filters: JsonValue::Null,
            raw_query: "Was ist die neue Meta?".to_string(),
            language: "de".to_string(),
        };
        let mut entity_match = AskEntityClaimMatch::default();

        assert!(should_use_patch_overview_context(
            &plan,
            &entity_match,
            "patch_changes"
        ));
        plan.entities.push(json!({"name": "Seven"}));
        assert!(!should_use_patch_overview_context(
            &plan,
            &entity_match,
            "patch_changes"
        ));
        plan.entities.clear();
        entity_match.matched = true;
        assert!(!should_use_patch_overview_context(
            &plan,
            &entity_match,
            "patch_changes"
        ));
    }

    #[test]
    fn patch_overview_aggregates_meta_signals() {
        let event = |value: JsonValue| value.as_object().cloned().expect("object");
        let events = vec![
            event(json!({
                "id": 1,
                "entity_type": "hero",
                "entity_name": "Seven",
                "change_type": "buff",
                "normalized_line": "Seven damage increased",
                "patch_title": "Patch 2026-06-30",
                "posted_at": "2026-06-30"
            })),
            event(json!({
                "id": 2,
                "entity_type": "item",
                "entity_name": "Mystic Shot",
                "change_type": "nerf",
                "normalized_line": "Mystic Shot damage reduced",
                "patch_title": "Patch 2026-06-30",
                "posted_at": "2026-06-30"
            })),
            event(json!({
                "id": 3,
                "entity_type": "general",
                "entity_name": "Economy",
                "change_type": "changed",
                "normalized_line": "Soul sharing changed",
                "patch_title": "Patch 2026-06-30",
                "posted_at": "2026-06-30"
            })),
        ];

        let overview = build_patch_overview_signals(&events, &BTreeMap::new());

        assert_eq!(overview["event_count"], 3);
        assert_eq!(overview["hero_movers_rough"][0]["entity"], "Seven");
        assert_eq!(overview["hero_movers_rough"][0]["rough_score"], 1);
        assert_eq!(overview["item_and_ability_changes"].as_array().map(Vec::len), Some(1));
        assert_eq!(overview["objective_and_economy_changes"].as_array().map(Vec::len), Some(1));
    }

    #[test]
    fn current_patch_overrides_ignore_older_changes() {
        let item = json!({"name": "Mystic Shot", "damage": 10});
        let timeline = json!({
            "latest_patch": {"patch_title": "Patch 2026-06-30", "posted_at": "2026-06-30"},
            "stat_changes": [
                {
                    "stat_name": "damage",
                    "old_value": "10",
                    "new_value": "12",
                    "patch_title": "Patch 2026-06-30",
                    "posted_at": "2026-06-30"
                },
                {
                    "stat_name": "damage",
                    "old_value": "8",
                    "new_value": "10",
                    "patch_title": "Patch 2026-06-01",
                    "posted_at": "2026-06-01"
                }
            ]
        });

        let item = apply_current_patch_overrides(item, &timeline);
        let updates = item["current_patch_overrides"]["updates"]
            .as_array()
            .expect("updates");

        assert_eq!(updates.len(), 1);
        assert_eq!(updates[0]["old_value"], "10");
        assert_eq!(updates[0]["new_value"], "12");
    }








}
