#![forbid(unsafe_code)]

//! Enrichment-Crate fuer Patch-Events, Patch-Impact-Notizen und Meta-Analysen.

use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    sync::OnceLock,
};

use deadlock_brain_core::{
    ai::{
        extract_ai_text, ChatCompletionRequest, ChatMessage, AiClient, AiConfig,
    },
    models::PatchEvent,
};
use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use sqlx::PgPool;

pub use deadlock_brain_core as core;

pub const PATCH_IMPACT_PROMPT_VERSION: &str = "patch_impact_de_v1";

const STAT_SUFFIXES: &[&str] = &[
    "spirit scaling",
    "bullet scaling",
    "weapon scaling",
    "melee scaling",
    "boon scaling",
    "base damage",
    "bonus damage",
    "bullet resist",
    "spirit resist",
    "melee resistance",
    "spirit resistance",
    "fire rate",
    "souls per minute",
    "bonus health",
    "max health",
    "health regen",
    "cooldown",
    "duration",
    "damage",
    "barrier",
    "range",
    "radius",
    "speed",
    "slow",
    "ammo",
    "cost",
    "charges",
    "stacks",
    "scaling",
    "health",
];

const FROM_TO_PATTERN: &str = r"^(?P<subject>.+?)\s+(?P<verb>increased|reduced|decreased|lowered|raised|changed|rescaled|adjusted)\s+from\s+(?P<old>.+?)\s+to\s+(?P<new>.+?)(?:[;,)]|$)";
const BARE_FROM_TO_PATTERN: &str =
    r"^(?P<subject>.+?)\s+from\s+(?P<old>.+?)\s+to\s+(?P<new>.+?)(?:[;,)]|$)";
const BY_AMOUNT_PATTERN: &str = r"^(?P<subject>.+?)\s+(?P<verb>increased|reduced|decreased|lowered|raised|improved)\s+by\s+(?P<amount>[+-]?\d+(?:\.\d+)?%?)(?:[;,)]|$)";
const MOVED_PATTERN: &str = r"^Moved\s+from\s+(?P<old>.+?)\s+to\s+(?P<new>.+?)(?:[;,)]|$)";
const UPGRADES_FROM_PATTERN: &str =
    r"^Now\s+upgrades\s+from\s+(?P<entity>.+?)(?:\s*\([^)]*\))?(?:[;,)]|$)";
const NO_LONGER_GRANTS_PATTERN: &str =
    r"^No\s+longer\s+grants\s+(?P<value>[+-]?\d+(?:\.\d+)?%?)\s+(?P<stat>.+?)(?:[;,)]|$)";
const NO_LONGER_HAS_PATTERN: &str =
    r"^No\s+longer\s+(?:has|scales\s+with|targets|procs|affects)\s+(?P<body>.+?)(?:[;,)]|$)";
const NOW_GRANTS_PATTERN: &str =
    r"^Now\s+(?:grants|provides)\s+(?P<value>[+-]?\d+(?:\.\d+)?%?)\s+(?P<stat>.+?)(?:[;,)]|$)";
const NOW_VALUE_STAT_PATTERN: &str = r"^(?P<subject>.+?)\s+now\s+(?:has|costs|scales\s+with|scales\s+per)\s+(?P<value>[+-]?\d+(?:\.\d+)?%?)\s+(?P<stat>.+?)(?:[;,)]|$)";
const NOW_TIER_PATTERN: &str = r"^Now\s+a\s+(?P<tier>T\d+)\s+item(?:[.;,)]|$)";
const VALUE_PATTERN: &str = r"^(?P<number>[+-]?\d+(?:\.\d+)?)(?P<unit>%|s|m|ms|x)?$";

static FROM_TO_RE: OnceLock<std::result::Result<Regex, String>> = OnceLock::new();
static BARE_FROM_TO_RE: OnceLock<std::result::Result<Regex, String>> = OnceLock::new();
static BY_AMOUNT_RE: OnceLock<std::result::Result<Regex, String>> = OnceLock::new();
static MOVED_RE: OnceLock<std::result::Result<Regex, String>> = OnceLock::new();
static UPGRADES_FROM_RE: OnceLock<std::result::Result<Regex, String>> = OnceLock::new();
static NO_LONGER_GRANTS_RE: OnceLock<std::result::Result<Regex, String>> = OnceLock::new();
static NO_LONGER_HAS_RE: OnceLock<std::result::Result<Regex, String>> = OnceLock::new();
static NOW_GRANTS_RE: OnceLock<std::result::Result<Regex, String>> = OnceLock::new();
static NOW_VALUE_STAT_RE: OnceLock<std::result::Result<Regex, String>> = OnceLock::new();
static NOW_TIER_RE: OnceLock<std::result::Result<Regex, String>> = OnceLock::new();
static VALUE_RE: OnceLock<std::result::Result<Regex, String>> = OnceLock::new();

#[derive(Debug, thiserror::Error)]
pub enum EnrichError {
    #[error(transparent)]
    Core(#[from] deadlock_brain_core::CoreError),

    #[error("Postgres error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("regex definition invalid for {pattern}: {message}")]
    RegexDefinition {
        pattern: &'static str,
        message: String,
    },

    #[error("regex capture missing: {0}")]
    MissingCapture(&'static str),

    #[error("required table missing: {0}")]
    MissingTable(&'static str),
}

pub type Result<T> = std::result::Result<T, EnrichError>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchEventEnrichment {
    pub patch_event_id: Option<i64>,
    pub stat_name: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub unit: Option<String>,
    pub ability_name: Option<String>,
    pub secondary_entity_name: Option<String>,
    pub confidence: f64,
    pub flags: Vec<String>,
}

impl PatchEventEnrichment {
    pub fn flags_json(&self) -> Result<String> {
        python_json_list(&self.flags)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchEventEnrichmentSummary {
    pub events_processed: usize,
    pub enrichments_inserted: usize,
    pub enrichments_matched: usize,
    pub enrichments_before: i64,
    pub enrichments_total: i64,
    pub deleted_before_build: usize,
    pub rebuild: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchImpactTarget {
    pub entity_name: String,
    pub entity_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchImpactRequest {
    pub request: ChatCompletionRequest,
    pub prompt_text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchImpactNoteSummary {
    pub id: i64,
    pub entity_name: String,
    pub status: String,
    pub hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchImpactBatchSummary {
    pub processed: usize,
    pub success: usize,
    pub failed: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaTrendSummary {
    pub processed: usize,
    pub success: usize,
    pub failed: usize,
}

/// Lokale Zeile aus `brain.patch_events`: der `PatchEvent` fuer die Regex-Parser
/// plus die synthetische `legacy_patch_event_id` (NOT NULL in der PG-Tabelle,
/// abgeleitet als `COALESCE(pe.legacy_sqlite_id, pe.id)`), die bis in
/// [`insert_enrichment`] durchgereicht werden muss.
struct PatchEventRow {
    event: PatchEvent,
    legacy_patch_event_id: i64,
}

pub async fn build_patch_event_enrichments(
    pool: &PgPool,
    rebuild: bool,
) -> Result<PatchEventEnrichmentSummary> {
    require_table(pool, "patch_event_enrichments").await?;

    let before_total = count_enrichments(pool).await?;
    let deleted = if rebuild {
        let result = sqlx::query!("DELETE FROM brain.patch_event_enrichments")
            .execute(pool)
            .await?;
        usize::try_from(result.rows_affected()).unwrap_or(usize::MAX)
    } else {
        0
    };

    let rebuild_flag = if rebuild { 1_i64 } else { 0_i64 };
    let rows = sqlx::query!(
        r#"
        SELECT pe.id AS "id!",
               COALESCE(pe.patch_snapshot_id, 0) AS "patch_snapshot_id!",
               pe.patch_external_id AS "patch_external_id!",
               pe.patch_title AS "patch_title?",
               pe.patch_url AS "patch_url?",
               pe.source_kind AS "source_kind!",
               pe.posted_at::text AS "posted_at?",
               pe.line_index AS "line_index!",
               pe.section AS "section?",
               pe.entity_type AS "entity_type!",
               pe.entity_name AS "entity_name?",
               pe.subject AS "subject?",
               pe.change_type AS "change_type!",
               pe.raw_line AS "raw_line!",
               pe.normalized_line AS "normalized_line!",
               pe.old_value AS "old_value?",
               pe.new_value AS "new_value?",
               pe.confidence AS "confidence!",
               pe.metadata::text AS "metadata_json!",
               pe.event_hash AS "event_hash!",
               extract(epoch from pe.created_at)::int8 AS "created_at!",
               COALESCE(pe.legacy_sqlite_id, pe.id) AS "legacy_patch_event_id!"
        FROM brain.patch_events pe
        LEFT JOIN brain.patch_event_enrichments pee ON pee.patch_event_id = pe.id
        WHERE ($1::int8 <> 0 OR pee.patch_event_id IS NULL)
        ORDER BY pe.id ASC
        "#,
        rebuild_flag,
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|row| PatchEventRow {
        event: PatchEvent {
            id: row.id,
            patch_snapshot_id: row.patch_snapshot_id,
            patch_external_id: row.patch_external_id,
            patch_title: row.patch_title,
            patch_url: row.patch_url,
            source_kind: row.source_kind,
            posted_at: row.posted_at,
            line_index: row.line_index,
            section: row.section,
            entity_type: row.entity_type,
            entity_name: row.entity_name,
            subject: row.subject,
            change_type: row.change_type,
            raw_line: row.raw_line,
            normalized_line: row.normalized_line,
            old_value: row.old_value,
            new_value: row.new_value,
            confidence: row.confidence,
            metadata_json: row.metadata_json,
            event_hash: row.event_hash,
            created_at: row.created_at,
        },
        legacy_patch_event_id: row.legacy_patch_event_id,
    })
    .collect::<Vec<_>>();

    let mut processed = 0_usize;
    let mut inserted = 0_usize;
    let mut matched = 0_usize;

    for row in rows {
        let enrichment = enrich_patch_event(&row.event)?;
        processed += 1;
        if enrichment.confidence > 0.0 {
            matched += 1;
        }
        if insert_enrichment(pool, &enrichment, row.legacy_patch_event_id).await? {
            inserted += 1;
        }
    }

    Ok(PatchEventEnrichmentSummary {
        events_processed: processed,
        enrichments_inserted: inserted,
        enrichments_matched: matched,
        enrichments_before: before_total,
        enrichments_total: count_enrichments(pool).await?,
        deleted_before_build: deleted,
        rebuild,
    })
}

pub fn enrich_patch_event(event: &PatchEvent) -> Result<PatchEventEnrichment> {
    let text = if event.normalized_line.is_empty() {
        event.raw_line.as_str()
    } else {
        event.normalized_line.as_str()
    };
    enrich_patch_event_line(Some(event.id), text)
}

pub fn enrich_patch_event_line(
    patch_event_id: Option<i64>,
    text: &str,
) -> Result<PatchEventEnrichment> {
    let text = clean_line(text);
    let flags: Vec<String> = Vec::new();

    if let Some(captures) = compiled_regex(&FROM_TO_RE, FROM_TO_PATTERN)?.captures(&text) {
        return from_to_enrichment(
            patch_event_id,
            capture(&captures, "subject")?,
            capture(&captures, "old")?,
            capture(&captures, "new")?,
            &capture(&captures, "verb")?.to_lowercase(),
            0.95,
            flags,
        );
    }

    if let Some(captures) = compiled_regex(&BARE_FROM_TO_RE, BARE_FROM_TO_PATTERN)?.captures(&text)
    {
        let mut bare_flags = flags;
        bare_flags.push("bare_from_to".to_string());
        return from_to_enrichment(
            patch_event_id,
            capture(&captures, "subject")?,
            capture(&captures, "old")?,
            capture(&captures, "new")?,
            "changed",
            0.88,
            bare_flags,
        );
    }

    if let Some(captures) = compiled_regex(&BY_AMOUNT_RE, BY_AMOUNT_PATTERN)?.captures(&text) {
        let (mut value, unit, value_flags) = split_value(capture(&captures, "amount")?)?;
        let mut amount_flags = flags;
        amount_flags.extend(value_flags);
        let (stat_name, ability_name, subject_flags) =
            split_stat_subject(capture(&captures, "subject")?);
        amount_flags.extend(subject_flags);
        let direction = capture(&captures, "verb")?.to_lowercase();
        amount_flags.push(format!("direction:{direction}"));
        if matches!(direction.as_str(), "reduced" | "decreased" | "lowered")
            && !value.starts_with('-')
        {
            value = format!("-{value}");
        }
        amount_flags.push("relative_delta".to_string());
        return Ok(PatchEventEnrichment {
            patch_event_id,
            stat_name: Some(stat_name),
            old_value: None,
            new_value: Some(value),
            unit,
            ability_name,
            secondary_entity_name: None,
            confidence: confidence(0.82, &amount_flags),
            flags: sorted_unique(amount_flags),
        });
    }

    if let Some(captures) = compiled_regex(&MOVED_RE, MOVED_PATTERN)?.captures(&text) {
        let (old_value, new_value, unit, value_flags) =
            split_values(capture(&captures, "old")?, capture(&captures, "new")?)?;
        let mut movement_flags = flags;
        movement_flags.extend(value_flags);
        movement_flags.push("movement".to_string());
        return Ok(PatchEventEnrichment {
            patch_event_id,
            stat_name: Some("item_tier".to_string()),
            old_value: Some(old_value),
            new_value: Some(new_value),
            unit,
            ability_name: None,
            secondary_entity_name: None,
            confidence: confidence(0.9, &movement_flags),
            flags: sorted_unique(movement_flags),
        });
    }

    if let Some(captures) =
        compiled_regex(&UPGRADES_FROM_RE, UPGRADES_FROM_PATTERN)?.captures(&text)
    {
        let secondary = clean_entity(capture(&captures, "entity")?);
        return Ok(PatchEventEnrichment {
            patch_event_id,
            stat_name: Some("upgrades_from".to_string()),
            old_value: None,
            new_value: Some(secondary.clone()),
            unit: None,
            ability_name: None,
            secondary_entity_name: Some(secondary),
            confidence: 0.92,
            flags: vec!["upgrade_source".to_string()],
        });
    }

    if let Some(captures) =
        compiled_regex(&NO_LONGER_GRANTS_RE, NO_LONGER_GRANTS_PATTERN)?.captures(&text)
    {
        let stat_name = clean_stat(capture(&captures, "stat")?);
        let (old_value, _, unit, value_flags) = split_values(capture(&captures, "value")?, "0")?;
        let mut grant_flags = flags;
        grant_flags.extend(value_flags);
        grant_flags.push("removed_grant".to_string());
        return Ok(PatchEventEnrichment {
            patch_event_id,
            stat_name: Some(stat_name),
            old_value: Some(old_value),
            new_value: Some("0".to_string()),
            unit,
            ability_name: None,
            secondary_entity_name: None,
            confidence: confidence(0.88, &grant_flags),
            flags: sorted_unique(grant_flags),
        });
    }

    if let Some(captures) = compiled_regex(&NOW_GRANTS_RE, NOW_GRANTS_PATTERN)?.captures(&text) {
        let (_, new_value, unit, value_flags) = split_values("0", capture(&captures, "value")?)?;
        let mut grant_flags = flags;
        grant_flags.extend(value_flags);
        grant_flags.push("added_grant".to_string());
        return Ok(PatchEventEnrichment {
            patch_event_id,
            stat_name: Some(clean_stat(capture(&captures, "stat")?)),
            old_value: None,
            new_value: Some(new_value),
            unit,
            ability_name: None,
            secondary_entity_name: None,
            confidence: confidence(0.82, &grant_flags),
            flags: sorted_unique(grant_flags),
        });
    }

    if let Some(captures) =
        compiled_regex(&NOW_VALUE_STAT_RE, NOW_VALUE_STAT_PATTERN)?.captures(&text)
    {
        let (new_value, unit, value_flags) = split_value(capture(&captures, "value")?)?;
        let mut value_stat_flags = flags;
        value_stat_flags.extend(value_flags);
        let subject = format!(
            "{} {}",
            capture(&captures, "subject")?,
            capture(&captures, "stat")?
        );
        let (stat_name, ability_name, subject_flags) = split_stat_subject(&subject);
        value_stat_flags.extend(subject_flags);
        value_stat_flags.push("now_value".to_string());
        return Ok(PatchEventEnrichment {
            patch_event_id,
            stat_name: Some(stat_name),
            old_value: None,
            new_value: Some(new_value),
            unit,
            ability_name,
            secondary_entity_name: None,
            confidence: confidence(0.74, &value_stat_flags),
            flags: sorted_unique(value_stat_flags),
        });
    }

    if let Some(captures) = compiled_regex(&NOW_TIER_RE, NOW_TIER_PATTERN)?.captures(&text) {
        return Ok(PatchEventEnrichment {
            patch_event_id,
            stat_name: Some("item_tier".to_string()),
            old_value: None,
            new_value: Some(capture(&captures, "tier")?.to_string()),
            unit: None,
            ability_name: None,
            secondary_entity_name: None,
            confidence: 0.78,
            flags: vec!["tier_change".to_string()],
        });
    }

    if let Some(captures) =
        compiled_regex(&NO_LONGER_HAS_RE, NO_LONGER_HAS_PATTERN)?.captures(&text)
    {
        return Ok(PatchEventEnrichment {
            patch_event_id,
            stat_name: Some(clean_stat(capture(&captures, "body")?)),
            old_value: Some("present".to_string()),
            new_value: Some("removed".to_string()),
            unit: None,
            ability_name: None,
            secondary_entity_name: None,
            confidence: 0.68,
            flags: vec!["removed_property".to_string()],
        });
    }

    if let Some(generic) = generic_event_enrichment(patch_event_id, &text) {
        return Ok(generic);
    }

    Ok(PatchEventEnrichment {
        patch_event_id,
        stat_name: None,
        old_value: None,
        new_value: None,
        unit: None,
        ability_name: None,
        secondary_entity_name: None,
        confidence: 0.0,
        flags: vec!["unparsed".to_string()],
    })
}

pub async fn list_pending_patch_impact_targets(
    pool: &PgPool,
    limit: usize,
    _model: Option<&str>,
) -> Result<Vec<PatchImpactTarget>> {
    require_table(pool, "patch_impact_notes").await?;

    let limit = i64::try_from(limit).map_or(i64::MAX, |value| value);
    let targets = sqlx::query!(
        r#"
        SELECT DISTINCT pe.entity_name AS "entity_name?", pe.entity_type AS "entity_type!"
        FROM brain.patch_events pe
        WHERE pe.entity_name NOT IN (
            SELECT entity_name FROM brain.patch_impact_notes WHERE status = 'analysis_ready'
        )
        LIMIT $1
        "#,
        limit,
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|row| PatchImpactTarget {
        entity_name: row.entity_name.map_or_else(String::new, keep_string),
        entity_type: row.entity_type,
    })
    .collect::<Vec<_>>();
    Ok(targets)
}

pub async fn build_patch_impact_context(
    pool: &PgPool,
    entity_name: &str,
    entity_type: &str,
) -> Result<Value> {
    let context = build_review_context(pool, entity_name, entity_type, 80).await?;
    let mut timeline_signals = match context.get("timeline_signals").cloned() {
        Some(value) => value,
        None => json!({}),
    };

    if let Some(recent_events) = timeline_signals
        .get_mut("recent_events")
        .and_then(Value::as_array_mut)
    {
        recent_events.truncate(30);
    }

    let patch_range = match timeline_signals.get("patch_range").cloned() {
        Some(value) => value,
        None => json!({}),
    };
    let stat_changes = match context
        .get("current_stat_hints")
        .and_then(|value| value.get("stat_changes"))
        .cloned()
    {
        Some(value) => value,
        None => json!([]),
    };
    let entity_summary = match context.get("entity_summary").cloned() {
        Some(value) => value,
        None => json!({}),
    };

    Ok(json!({
        "entity_summary": entity_summary,
        "timeline_signals": timeline_signals,
        "stat_changes": stat_changes,
        "patch_range": patch_range,
    }))
}

pub fn build_patch_impact_request(
    context: &Value,
    config: &AiConfig,
) -> Result<PatchImpactRequest> {
    let entity_name = context
        .get("entity_summary")
        .and_then(|summary| summary.get("canonical_name"))
        .and_then(Value::as_str)
        .map_or("Unknown", |value| value);

    let mut prompt =
        format!("Analysiere die Patch-Entwicklung für {entity_name} basierend auf den folgenden Daten.\n");
    prompt.push_str("Gib NUR JSON aus mit folgendem Schema: {\"trend\": \"buffs_dominant\"|\"nerfs_dominant\"|\"mixed\"|\"stable\", \"affected_areas\": [\"...\"], \"momentum\": \"rising\"|\"falling\"|\"stable\", \"key_changes\": [\"...\"], \"confidence\": 0-1}\n\n");
    prompt.push_str(&serde_json::to_string_pretty(context)?);

    Ok(PatchImpactRequest {
        request: ChatCompletionRequest {
            model: config.model.clone(),
            messages: vec![ChatMessage::user(prompt.clone())],
            max_completion_tokens: config.max_completion_tokens,
            temperature: config.temperature,
            top_p: config.top_p,
            stream: false,
        },
        prompt_text: prompt,
    })
}

pub async fn save_patch_impact_note(
    pool: &PgPool,
    context: &Value,
    prompt_text: &str,
    result_text: Option<&str>,
    model: &str,
    status: &str,
) -> Result<PatchImpactNoteSummary> {
    require_table(pool, "patch_impact_notes").await?;

    let entity_summary = match context
        .get("entity_summary")
        .and_then(Value::as_object)
        .cloned()
    {
        Some(value) => value,
        None => Map::new(),
    };
    let entity_name = string_from_object(&entity_summary, "canonical_name")
        .or_else(|| string_from_object(&entity_summary, "name"))
        .map_or_else(String::new, keep_string);
    let entity_type =
        string_from_object(&entity_summary, "entity_type").map_or_else(String::new, keep_string);
    let entity_id = entity_summary.get("entity_id").and_then(Value::as_i64);

    let patch_range = match context
        .get("patch_range")
        .and_then(Value::as_object)
        .cloned()
    {
        Some(value) => value,
        None => Map::new(),
    };
    let patch_range_start = scalar_string_from_object(&patch_range, "oldest");
    let patch_range_end = scalar_string_from_object(&patch_range, "newest");
    let event_count = patch_range
        .get("event_count")
        .and_then(Value::as_i64)
        .map_or(0, |value| value);
    let context_hash = stable_hash_text(&python_json_dumps_sort_keys(context));

    let mut final_status = status.to_string();
    let mut insights_json = "{}".to_string();
    if let Some(result_text) = result_text.filter(|text| !text.is_empty()) {
        if final_status == "analysis_ready" {
            let text = extract_json_text(result_text);
            if serde_json::from_str::<Value>(&text).is_ok() {
                insights_json = text;
            } else {
                final_status = "analysis_failed".to_string();
            }
        }
    }

    let inserted_id = sqlx::query_scalar!(
        r#"
        INSERT INTO brain.patch_impact_notes (
            entity_type, entity_name, entity_id, context_hash, prompt_version,
            prompt_text, result_text, insights, model, status,
            patch_range_start, patch_range_end, event_count, created_at, updated_at
        ) VALUES (
            $1, $2, $3, $4, $5,
            $6, $7, $8::text::jsonb, $9, $10,
            $11, $12, $13, now(), now()
        )
        ON CONFLICT (entity_name, context_hash, prompt_version, COALESCE(model, ''))
        DO UPDATE SET
            result_text = excluded.result_text,
            insights = excluded.insights,
            status = excluded.status,
            updated_at = now()
        RETURNING id AS "id!"
        "#,
        entity_type,
        entity_name,
        entity_id,
        context_hash,
        PATCH_IMPACT_PROMPT_VERSION,
        prompt_text,
        result_text,
        insights_json,
        model,
        final_status,
        patch_range_start.as_deref(),
        patch_range_end.as_deref(),
        event_count,
    )
    .fetch_one(pool)
    .await?;

    Ok(PatchImpactNoteSummary {
        id: inserted_id,
        entity_name,
        status: final_status,
        hash: context_hash,
    })
}

pub async fn run_patch_impact_batch(
    pool: &PgPool,
    config: &AiConfig,
    limit: usize,
) -> Result<PatchImpactBatchSummary> {
    let client = AiClient::new(config.clone())?;
    run_patch_impact_batch_with_chat(pool, config, limit, |request| {
        Ok(client.chat(request)?)
    })
    .await
}

pub async fn run_patch_impact_batch_with_chat<F>(
    pool: &PgPool,
    config: &AiConfig,
    limit: usize,
    mut chat: F,
) -> Result<PatchImpactBatchSummary>
where
    F: FnMut(&ChatCompletionRequest) -> Result<Value>,
{
    let targets = list_pending_patch_impact_targets(pool, limit, Some(&config.model)).await?;
    if targets.is_empty() {
        return Ok(PatchImpactBatchSummary {
            processed: 0,
            success: 0,
            failed: 0,
            message: Some("no targets pending".to_string()),
        });
    }

    let mut success = 0_usize;
    let mut failed = 0_usize;

    for target in &targets {
        let context =
            build_patch_impact_context(pool, &target.entity_name, &target.entity_type).await?;
        let request_info = build_patch_impact_request(&context, config)?;
        match chat(&request_info.request) {
            Ok(response) => {
                let result_text = extract_ai_text(&response);
                save_patch_impact_note(
                    pool,
                    &context,
                    &request_info.prompt_text,
                    Some(&result_text),
                    &config.model,
                    "analysis_ready",
                )
                .await?;
                success += 1;
            }
            Err(error) => {
                let result_text = error.to_string();
                save_patch_impact_note(
                    pool,
                    &context,
                    &request_info.prompt_text,
                    Some(&result_text),
                    &config.model,
                    "analysis_failed",
                )
                .await?;
                failed += 1;
            }
        }
    }

    Ok(PatchImpactBatchSummary {
        processed: targets.len(),
        success,
        failed,
        message: None,
    })
}

pub async fn run_meta_trend_analysis(
    pool: &PgPool,
    config: &AiConfig,
) -> Result<MetaTrendSummary> {
    let client = AiClient::new(config.clone())?;
    run_meta_trend_analysis_with_chat(pool, config, |request| Ok(client.chat(request)?)).await
}

pub async fn run_meta_trend_analysis_with_chat<F>(
    pool: &PgPool,
    config: &AiConfig,
    mut chat: F,
) -> Result<MetaTrendSummary>
where
    F: FnMut(&ChatCompletionRequest) -> Result<Value>,
{
    require_table(pool, "meta_trend_notes").await?;

    let mock_shifts = [
        ("Abrams", -3.2_f64, "falling"),
        ("Infernus", 4.1_f64, "rising"),
    ];
    let mut success = 0_usize;
    let mut failed = 0_usize;

    for (entity_name, winrate_delta, trend_direction) in mock_shifts {
        let prompt = format!(
            "Analysiere den Meta-Trend für {entity_name}. Die Winrate hat sich um {winrate_delta}% verändert (Trend: {trend_direction}).\nErkläre kurz, woran das liegen könnte (z.B. direkte Nerfs, System-Patches oder Buffs von Counter-Heroes)."
        );
        let request = ChatCompletionRequest {
            model: config.model.clone(),
            messages: vec![
                ChatMessage::system("Du bist ein professioneller Deadlock Meta-Analyst."),
                ChatMessage::user(prompt),
            ],
            max_completion_tokens: config.max_completion_tokens,
            temperature: config.temperature,
            top_p: config.top_p,
            stream: false,
        };

        match chat(&request) {
            Ok(response) => {
                let result_text = extract_ai_text(&response);
                sqlx::query!(
                    r#"
                    INSERT INTO brain.meta_trend_notes (
                        entity_name, trend_direction, winrate_delta, context,
                        result_text, status, created_at, updated_at
                    ) VALUES ($1, $2, $3, $4::text::jsonb, $5, $6, now(), now())
                    "#,
                    entity_name,
                    trend_direction,
                    winrate_delta,
                    "{}",
                    result_text,
                    "analysis_ready",
                )
                .execute(pool)
                .await?;
                success += 1;
            }
            Err(error) => {
                eprintln!("Meta Trend Fehler für {entity_name}: {error}");
                failed += 1;
            }
        }
    }

    Ok(MetaTrendSummary {
        processed: mock_shifts.len(),
        success,
        failed,
    })
}

fn from_to_enrichment(
    patch_event_id: Option<i64>,
    subject: &str,
    old_raw: &str,
    new_raw: &str,
    direction: &str,
    base_confidence: f64,
    mut flags: Vec<String>,
) -> Result<PatchEventEnrichment> {
    let (old_value, new_value, unit, value_flags) = split_values(old_raw, new_raw)?;
    flags.extend(value_flags);
    let (stat_name, ability_name, subject_flags) = split_stat_subject(subject);
    flags.extend(subject_flags);
    flags.push(format!("direction:{direction}"));
    Ok(PatchEventEnrichment {
        patch_event_id,
        stat_name: Some(stat_name),
        old_value: Some(old_value),
        new_value: Some(new_value),
        unit,
        ability_name,
        secondary_entity_name: None,
        confidence: confidence(base_confidence, &flags),
        flags: sorted_unique(flags),
    })
}

fn generic_event_enrichment(
    patch_event_id: Option<i64>,
    text: &str,
) -> Option<PatchEventEnrichment> {
    let lower = text.to_lowercase();
    if lower.is_empty() {
        return None;
    }
    let padded = format!(" {lower} ");
    if lower.starts_with("fixed ") || padded.contains(" bug ") || lower.contains("crash") {
        return Some(PatchEventEnrichment {
            patch_event_id,
            stat_name: Some("bugfix".to_string()),
            old_value: None,
            new_value: Some("fixed".to_string()),
            unit: None,
            ability_name: None,
            secondary_entity_name: None,
            confidence: 0.62,
            flags: vec!["generic_bugfix".to_string()],
        });
    }
    if lower.starts_with("added ") {
        return Some(PatchEventEnrichment {
            patch_event_id,
            stat_name: Some("added_content".to_string()),
            old_value: None,
            new_value: Some("added".to_string()),
            unit: None,
            ability_name: None,
            secondary_entity_name: None,
            confidence: 0.6,
            flags: vec!["generic_added".to_string()],
        });
    }
    if padded.contains(" now ")
        || lower.starts_with("now ")
        || lower.contains(" swapped")
        || lower.contains("adjusted ")
    {
        return Some(PatchEventEnrichment {
            patch_event_id,
            stat_name: Some("functional_change".to_string()),
            old_value: None,
            new_value: Some("changed".to_string()),
            unit: None,
            ability_name: None,
            secondary_entity_name: None,
            confidence: 0.58,
            flags: vec!["generic_functional".to_string()],
        });
    }
    None
}

async fn insert_enrichment(
    pool: &PgPool,
    enrichment: &PatchEventEnrichment,
    legacy_patch_event_id: i64,
) -> Result<bool> {
    let flags_json = enrichment.flags_json()?;
    let result = sqlx::query!(
        r#"
        INSERT INTO brain.patch_event_enrichments(
          patch_event_id, legacy_patch_event_id, stat_name, old_value, new_value, unit,
          ability_name, secondary_entity_name, confidence, flags, created_at, updated_at
        )
        VALUES(
          $1, $2, $3, $4, $5, $6, $7, $8, $9, $10::text::jsonb, now(), now()
        )
        ON CONFLICT (patch_event_id) DO NOTHING
        "#,
        enrichment.patch_event_id,
        legacy_patch_event_id,
        enrichment.stat_name.as_deref(),
        enrichment.old_value.as_deref(),
        enrichment.new_value.as_deref(),
        enrichment.unit.as_deref(),
        enrichment.ability_name.as_deref(),
        enrichment.secondary_entity_name.as_deref(),
        enrichment.confidence,
        flags_json,
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

fn split_stat_subject(subject: &str) -> (String, Option<String>, Vec<String>) {
    let cleaned = clean_stat(subject);
    let lower = cleaned.to_lowercase();
    for suffix in STAT_SUFFIXES {
        if lower == *suffix {
            return (cleaned, None, Vec::new());
        }
        let suffix_with_space = format!(" {suffix}");
        if lower.ends_with(&suffix_with_space) && cleaned.len() >= suffix.len() {
            let ability_end = cleaned.len() - suffix.len();
            let ability = cleaned[..ability_end].trim().to_string();
            if !ability.is_empty() {
                let suffix_original = &cleaned[ability_end..];
                return (
                    title_like_suffix(suffix_original),
                    Some(ability),
                    vec!["ability_prefix".to_string()],
                );
            }
        }
    }
    (cleaned, None, vec!["unknown_stat_shape".to_string()])
}

fn split_values(old_raw: &str, new_raw: &str) -> Result<(String, String, Option<String>, Vec<String>)> {
    let (old_value, old_unit, old_flags) = split_value(old_raw)?;
    let (new_value, new_unit, new_flags) = split_value(new_raw)?;
    let mut flags = old_flags;
    flags.extend(new_flags);
    let unit = if old_unit == new_unit {
        old_unit
    } else if old_unit.is_some() && new_unit.is_some() {
        flags.push("mixed_units".to_string());
        None
    } else {
        old_unit.or(new_unit)
    };
    Ok((old_value, new_value, unit, flags))
}

fn split_value(raw: &str) -> Result<(String, Option<String>, Vec<String>)> {
    let cleaned = clean_value(raw);
    let Some(captures) = compiled_regex(&VALUE_RE, VALUE_PATTERN)?.captures(&cleaned) else {
        return Ok((cleaned, None, vec!["non_numeric_value".to_string()]));
    };
    let number = capture(&captures, "number")?.to_string();
    let unit = captures.name("unit").map(|matched| matched.as_str().to_string());
    Ok((number, unit, Vec::new()))
}

fn confidence(base: f64, flags: &[String]) -> f64 {
    let mut penalty = 0.0_f64;
    if flags.iter().any(|flag| flag == "unknown_stat_shape") {
        penalty += 0.12;
    }
    if flags.iter().any(|flag| flag == "non_numeric_value") {
        penalty += 0.08;
    }
    if flags.iter().any(|flag| flag == "mixed_units") {
        penalty += 0.05;
    }
    ((base - penalty).max(0.0) * 1000.0).round() / 1000.0
}

fn clean_line(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn clean_stat(text: &str) -> String {
    let cleaned = clean_line(text)
        .trim_matches(|character| matches!(character, ' ' | ':' | '-'))
        .to_string();
    capitalize_first(&cleaned)
}

fn clean_value(text: &str) -> String {
    clean_line(text)
        .trim_matches(|character| matches!(character, ' ' | '.' | ';' | ',' | ')'))
        .chars()
        .take(160)
        .collect()
}

fn clean_entity(text: &str) -> String {
    clean_value(text)
        .trim_matches(|character| matches!(character, '"' | '\''))
        .to_string()
}

fn title_like_suffix(text: &str) -> String {
    if text.is_empty() {
        return text.to_string();
    }
    text.split_whitespace()
        .map(capitalize_first)
        .collect::<Vec<_>>()
        .join(" ")
}

fn capitalize_first(text: &str) -> String {
    let mut characters = text.chars();
    let Some(first) = characters.next() else {
        return String::new();
    };
    let mut output = first.to_uppercase().collect::<String>();
    output.push_str(characters.as_str());
    output
}

fn sorted_unique(flags: Vec<String>) -> Vec<String> {
    flags.into_iter().collect::<BTreeSet<_>>().into_iter().collect()
}

async fn count_enrichments(pool: &PgPool) -> Result<i64> {
    let count = sqlx::query_scalar!(
        r#"SELECT COUNT(*) AS "count!" FROM brain.patch_event_enrichments"#
    )
    .fetch_one(pool)
    .await?;
    Ok(count)
}

fn compiled_regex(
    cell: &'static OnceLock<std::result::Result<Regex, String>>,
    pattern: &'static str,
) -> Result<&'static Regex> {
    match cell.get_or_init(|| Regex::new(&format!("(?i){pattern}")).map_err(|error| error.to_string())) {
        Ok(regex) => Ok(regex),
        Err(message) => Err(EnrichError::RegexDefinition {
            pattern,
            message: message.clone(),
        }),
    }
}

fn capture<'a>(captures: &'a Captures<'_>, name: &'static str) -> Result<&'a str> {
    captures
        .name(name)
        .map(|matched| matched.as_str())
        .ok_or(EnrichError::MissingCapture(name))
}

fn python_json_list(items: &[String]) -> Result<String> {
    let mut output = String::from("[");
    for (index, item) in items.iter().enumerate() {
        if index > 0 {
            output.push_str(", ");
        }
        output.push_str(&serde_json::to_string(item)?);
    }
    output.push(']');
    Ok(output)
}

async fn require_table(pool: &PgPool, table_name: &'static str) -> Result<()> {
    let qualified = format!("brain.{table_name}");
    let exists: bool = sqlx::query_scalar!(
        r#"SELECT to_regclass($1) IS NOT NULL AS "exists!""#,
        qualified,
    )
    .fetch_one(pool)
    .await?;
    if exists {
        Ok(())
    } else {
        Err(EnrichError::MissingTable(table_name))
    }
}

async fn build_review_context(
    pool: &PgPool,
    query: &str,
    fallback_entity_type: &str,
    limit_events: usize,
) -> Result<Value> {
    let entity_summary = load_entity_summary(pool, query, fallback_entity_type).await?;
    let events = load_patch_events(pool, query, limit_events).await?;
    let enrichments = load_enrichments(pool, &events).await?;
    let timeline_signals = build_timeline_signals(&events, &enrichments);

    Ok(json!({
        "query": query,
        "context_kind": "analysis_review",
        "entity_summary": entity_summary,
        "lineage": {
            "available": false,
            "relations": [],
            "related_names": [],
            "relation_counts": {},
            "omitted_relation_count": 0,
        },
        "current_stat_hints": {
            "available": false,
            "source_count": 0,
            "profile": Value::Null,
            "hints": [],
            "omitted_hint_count": 0,
            "source_tables": [],
        },
        "timeline_signals": timeline_signals,
        "open_questions": [],
        "source_references": [],
        "prompt_de": Value::Null,
        "retrieval_meta": {
            "limit_events_requested": limit_events,
            "events_loaded": events.len(),
            "enrichments_loaded": enrichments.len(),
            "fallback_used": false,
            "sheet_stats_available": false,
        },
    }))
}

async fn load_entity_summary(
    pool: &PgPool,
    query: &str,
    fallback_entity_type: &str,
) -> Result<Value> {
    let row = sqlx::query!(
        r#"
        SELECT e.id AS "id!",
               e.entity_type AS "entity_type!",
               e.canonical_name AS "canonical_name!",
               e.primary_external_id AS "primary_external_id?",
               e.source AS "source!",
               e.metadata::text AS "metadata_json!"
        FROM brain.entities e
        WHERE lower(e.canonical_name) = lower($1)
        ORDER BY e.entity_type, length(e.canonical_name), e.canonical_name
        LIMIT 1
        "#,
        query,
    )
    .fetch_optional(pool)
    .await?
    .map(|row| {
        (
            row.id,
            row.entity_type,
            row.canonical_name,
            row.primary_external_id,
            row.source,
            row.metadata_json,
        )
    });

    if let Some((id, entity_type, canonical_name, external_id, source, metadata_json)) = row {
        let metadata = match serde_json::from_str::<Value>(&metadata_json) {
            Ok(value) => value,
            Err(_) => json!({}),
        };
        let disabled = match metadata.get("disabled").cloned() {
            Some(value) => value,
            None => Value::Null,
        };
        let document_kinds = match metadata.get("document_kinds").cloned() {
            Some(value) => value,
            None => json!([]),
        };
        Ok(json!({
            "matched": true,
            "name": canonical_name,
            "canonical_name": canonical_name,
            "entity_id": id,
            "entity_type": entity_type,
            "source": source,
            "external_id": external_id,
            "match_score": 120,
            "matched_alias_kinds": [],
            "aliases": [],
            "metadata_hints": {
                "disabled": disabled,
                "document_kinds": document_kinds,
            },
        }))
    } else {
        Ok(json!({
            "matched": false,
            "name": query,
            "canonical_name": query,
            "entity_id": Value::Null,
            "entity_type": fallback_entity_type,
            "source": Value::Null,
            "external_id": Value::Null,
            "match_score": Value::Null,
            "matched_alias_kinds": [],
            "aliases": [],
            "metadata_hints": {
                "disabled": Value::Null,
                "document_kinds": [],
            },
        }))
    }
}

async fn load_patch_events(
    pool: &PgPool,
    query: &str,
    limit_events: usize,
) -> Result<Vec<PatchEvent>> {
    let limit = i64::try_from(limit_events).map_or(i64::MAX, |value| value);
    let like_query = format!("%{query}%");
    let rows = sqlx::query!(
        r#"
        SELECT
          pe.id AS "id!",
          COALESCE(pe.patch_snapshot_id, 0) AS "patch_snapshot_id!",
          pe.patch_external_id AS "patch_external_id!",
          pe.patch_title AS "patch_title?",
          pe.patch_url AS "patch_url?",
          pe.source_kind AS "source_kind!",
          pe.posted_at::text AS "posted_at?",
          pe.line_index AS "line_index!",
          pe.section AS "section?",
          pe.entity_type AS "entity_type!",
          pe.entity_name AS "entity_name?",
          pe.subject AS "subject?",
          pe.change_type AS "change_type!",
          pe.raw_line AS "raw_line!",
          pe.normalized_line AS "normalized_line!",
          pe.old_value AS "old_value?",
          pe.new_value AS "new_value?",
          pe.confidence AS "confidence!",
          pe.metadata::text AS "metadata_json!",
          pe.event_hash AS "event_hash!",
          extract(epoch from pe.created_at)::int8 AS "created_at!"
        FROM brain.patch_events pe
        WHERE lower(pe.entity_name) = lower($1) OR pe.entity_name LIKE $2
        ORDER BY pe.patch_snapshot_id DESC, pe.line_index
        LIMIT $3
        "#,
        query,
        like_query,
        limit,
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|row| PatchEvent {
        id: row.id,
        patch_snapshot_id: row.patch_snapshot_id,
        patch_external_id: row.patch_external_id,
        patch_title: row.patch_title,
        patch_url: row.patch_url,
        source_kind: row.source_kind,
        posted_at: row.posted_at,
        line_index: row.line_index,
        section: row.section,
        entity_type: row.entity_type,
        entity_name: row.entity_name,
        subject: row.subject,
        change_type: row.change_type,
        raw_line: row.raw_line,
        normalized_line: row.normalized_line,
        old_value: row.old_value,
        new_value: row.new_value,
        confidence: row.confidence,
        metadata_json: row.metadata_json,
        event_hash: row.event_hash,
        created_at: row.created_at,
    })
    .collect::<Vec<_>>();
    Ok(rows)
}

async fn load_enrichments(pool: &PgPool, events: &[PatchEvent]) -> Result<Vec<Value>> {
    if events.is_empty() {
        return Ok(Vec::new());
    }
    // UNIQUE(patch_event_id) garantiert hoechstens ein Enrichment je Event.
    // Ein einziger `= ANY($1)`-Load statt N Queries; danach werden die Treffer
    // in Event-Reihenfolge zusammengefuehrt (identische JSON-Ausgabe-Reihenfolge
    // wie die alte Per-Event-Schleife).
    let ids = events.iter().map(|event| event.id).collect::<Vec<i64>>();
    let mut by_event: HashMap<i64, Value> = HashMap::new();
    let fetched = sqlx::query!(
        r#"
        SELECT patch_event_id AS "patch_event_id!", stat_name, old_value, new_value, unit,
               ability_name, secondary_entity_name, confidence AS "confidence!"
        FROM brain.patch_event_enrichments
        WHERE patch_event_id = ANY($1)
        "#,
        &ids,
    )
    .fetch_all(pool)
    .await?;
    for row in fetched {
        by_event.insert(
            row.patch_event_id,
            json!({
                "patch_event_id": row.patch_event_id,
                "stat_name": row.stat_name,
                "old_value": row.old_value,
                "new_value": row.new_value,
                "unit": row.unit,
                "ability_name": row.ability_name,
                "secondary_entity_name": row.secondary_entity_name,
                "confidence": row.confidence,
            }),
        );
    }

    let mut rows = Vec::new();
    for event in events {
        if let Some(value) = by_event.get(&event.id) {
            rows.push(value.clone());
        }
    }
    Ok(rows)
}

fn build_timeline_signals(events: &[PatchEvent], enrichments: &[Value]) -> Value {
    let mut enrichments_by_event_id: HashMap<i64, Vec<Value>> = HashMap::new();
    for enrichment in enrichments {
        if let Some(event_id) = enrichment.get("patch_event_id").and_then(Value::as_i64) {
            enrichments_by_event_id
                .entry(event_id)
                .or_default()
                .push(enrichment.clone());
        }
    }

    let mut change_type_counts = BTreeMap::<String, i64>::new();
    let mut source_counts = BTreeMap::<String, i64>::new();
    let mut section_counts = BTreeMap::<String, i64>::new();
    let mut dates = Vec::new();
    let mut recent_events = Vec::new();
    let mut stat_changes = Vec::new();
    let mut ability_mentions = BTreeMap::<String, i64>::new();
    let mut low_confidence_events = 0_i64;

    for event in events {
        *change_type_counts
            .entry(event.change_type.clone())
            .or_insert(0) += 1;
        *source_counts.entry(event.source_kind.clone()).or_insert(0) += 1;
        *section_counts
            .entry(match event.section.clone() {
                Some(value) => value,
                None => "Unsectioned".to_string(),
            })
            .or_insert(0) += 1;
        if let Some(posted_at) = &event.posted_at {
            dates.push(posted_at.clone());
        }

        let event_enrichments = match enrichments_by_event_id.get(&event.id).cloned() {
            Some(value) => value,
            None => vec![json!({})],
        };
        if event_enrichments
            .iter()
            .any(|enrichment| {
                let confidence = enrichment
                    .get("confidence")
                    .and_then(Value::as_f64)
                    .map_or(0.0, |value| value);
                confidence < 0.5
            })
        {
            low_confidence_events += 1;
        }

        recent_events.push(compact_event(event, &event_enrichments));
        for enrichment in &event_enrichments {
            if let Some(ability) = enrichment.get("ability_name").and_then(Value::as_str) {
                *ability_mentions.entry(ability.to_string()).or_insert(0) += 1;
            }
            if has_structured_stat_change(enrichment) {
                stat_changes.push(compact_stat_change(event, enrichment));
            }
        }
    }

    recent_events.truncate(24);
    stat_changes.truncate(24);

    json!({
        "event_count": events.len(),
        "date_range": {
            "newest": dates.iter().max().cloned(),
            "oldest": dates.iter().min().cloned(),
        },
        "latest_patch": events.first().map(latest_patch),
        "change_type_counts": map_counts(change_type_counts, usize::MAX),
        "source_counts": map_counts(source_counts, usize::MAX),
        "top_sections": map_counts(section_counts, 8),
        "recent_events": recent_events,
        "stat_changes": stat_changes,
        "omitted_recent_event_count": events.len().saturating_sub(24),
        "omitted_stat_change_count": 0,
        "ability_mentions": map_counts(ability_mentions, 12),
        "low_confidence_event_count": low_confidence_events,
    })
}

fn compact_event(event: &PatchEvent, enrichments: &[Value]) -> Value {
    json!({
        "patch_event_id": event.id,
        "posted_at": event.posted_at,
        "patch_title": event.patch_title,
        "source_kind": event.source_kind,
        "section": event.section,
        "change_type": event.change_type,
        "line": if event.normalized_line.is_empty() {
            event.raw_line.clone()
        } else {
            event.normalized_line.clone()
        },
        "structured_changes": enrichments
            .iter()
            .filter(|enrichment| match enrichment.as_object() {
                Some(object) => !object.is_empty(),
                None => false,
            })
            .map(compact_enrichment)
            .collect::<Vec<_>>(),
    })
}

fn compact_enrichment(enrichment: &Value) -> Value {
    json!({
        "stat_name": value_field_or_null(enrichment, "stat_name"),
        "old_value": value_field_or_null(enrichment, "old_value"),
        "new_value": value_field_or_null(enrichment, "new_value"),
        "unit": value_field_or_null(enrichment, "unit"),
        "ability_name": value_field_or_null(enrichment, "ability_name"),
        "secondary_entity_name": value_field_or_null(enrichment, "secondary_entity_name"),
        "confidence": value_field_or_null(enrichment, "confidence"),
    })
}

fn value_field_or_null(value: &Value, key: &str) -> Value {
    match value.get(key).cloned() {
        Some(value) => value,
        None => Value::Null,
    }
}

fn compact_stat_change(event: &PatchEvent, enrichment: &Value) -> Value {
    let mut compact = compact_enrichment(enrichment);
    if let Some(object) = compact.as_object_mut() {
        object.insert("patch_event_id".to_string(), json!(event.id));
        object.insert("posted_at".to_string(), json!(event.posted_at));
        object.insert("patch_title".to_string(), json!(event.patch_title));
        object.insert("change_type".to_string(), json!(event.change_type));
        object.insert(
            "line".to_string(),
            json!(if event.normalized_line.is_empty() {
                event.raw_line.clone()
            } else {
                event.normalized_line.clone()
            }),
        );
    }
    compact
}

fn has_structured_stat_change(enrichment: &Value) -> bool {
    enrichment
        .get("stat_name")
        .and_then(Value::as_str)
        .map(|value| !value.is_empty())
        .is_some_and(|value| value)
        && (enrichment.get("old_value").is_some_and(|value| !value.is_null())
            || enrichment.get("new_value").is_some_and(|value| !value.is_null()))
}

fn latest_patch(event: &PatchEvent) -> Value {
    json!({
        "posted_at": event.posted_at,
        "patch_title": event.patch_title,
        "patch_url": event.patch_url,
        "source_kind": event.source_kind,
    })
}

fn map_counts(counts: BTreeMap<String, i64>, limit: usize) -> Value {
    let mut entries = counts.into_iter().collect::<Vec<_>>();
    entries.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
    entries.truncate(limit);
    let mut map = Map::new();
    for (key, value) in entries {
        map.insert(key, json!(value));
    }
    Value::Object(map)
}

fn string_from_object(object: &Map<String, Value>, key: &str) -> Option<String> {
    object
        .get(key)
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
}

fn keep_string(value: String) -> String {
    value
}

fn scalar_string_from_object(object: &Map<String, Value>, key: &str) -> Option<String> {
    let value = object.get(key)?;
    if let Some(text) = value.as_str() {
        return Some(text.to_string());
    }
    if value.is_null() {
        return None;
    }
    Some(value.to_string())
}

fn extract_json_text(result_text: &str) -> String {
    if let Some(start) = result_text.find("```json") {
        let after_start = start + "```json".len();
        let rest = &result_text[after_start..];
        return match rest.find("```") {
            Some(end) => rest[..end].trim().to_string(),
            None => rest.trim().to_string(),
        };
    }
    if let Some(start) = result_text.find("```") {
        let after_start = start + "```".len();
        let rest = &result_text[after_start..];
        return match rest.find("```") {
            Some(end) => rest[..end].trim().to_string(),
            None => rest.trim().to_string(),
        };
    }
    result_text.to_string()
}

fn stable_hash_text(content: &str) -> String {
    hex::encode(Sha256::digest(content.as_bytes()))
}

fn python_json_dumps_sort_keys(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(value) => value.to_string(),
        Value::Number(value) => value.to_string(),
        Value::String(value) => python_json_string(value),
        Value::Array(values) => {
            let items = values
                .iter()
                .map(python_json_dumps_sort_keys)
                .collect::<Vec<_>>()
                .join(", ");
            format!("[{items}]")
        }
        Value::Object(object) => {
            let mut pairs = object.iter().collect::<Vec<_>>();
            pairs.sort_by(|left, right| left.0.cmp(right.0));
            let items = pairs
                .into_iter()
                .map(|(key, value)| {
                    format!(
                        "{}: {}",
                        python_json_string(key),
                        python_json_dumps_sort_keys(value)
                    )
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("{{{items}}}")
        }
    }
}

fn python_json_string(value: &str) -> String {
    let mut output = String::from("\"");
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\u{08}' => output.push_str("\\b"),
            '\u{0c}' => output.push_str("\\f"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            character if character <= '\u{1f}' => {
                output.push_str(&format!("\\u{:04x}", character as u32));
            }
            character if !character.is_ascii() => {
                let codepoint = character as u32;
                if codepoint <= 0xffff {
                    output.push_str(&format!("\\u{codepoint:04x}"));
                } else {
                    let value = codepoint - 0x1_0000;
                    let high = 0xd800 + ((value >> 10) & 0x3ff);
                    let low = 0xdc00 + (value & 0x3ff);
                    output.push_str(&format!("\\u{high:04x}\\u{low:04x}"));
                }
            }
            character => output.push(character),
        }
    }
    output.push('"');
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    use sqlx::postgres::PgPoolOptions;
    use std::sync::atomic::{AtomicU64, Ordering};

    static FIXTURE_COUNTER: AtomicU64 = AtomicU64::new(0);

    /// Kollisionsfreier Suffix fuer Fixture-Schluessel (UNIQUE-Spalten).
    fn unique_suffix() -> String {
        let counter = FIXTURE_COUNTER.fetch_add(1, Ordering::Relaxed);
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        format!("{nanos}-{counter}")
    }

    /// Wegwerf-Postgres aus `DEADLOCK_CENTRAL_DSN`. `None` (Test-Skip) wenn die
    /// Env-Variable fehlt oder leer ist -> `cargo test` bleibt offline gruen.
    /// Das echte zentrale DSN wird NIE eingebrannt; nur eine selbst gesetzte
    /// Scratch-Instanz.
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

    async fn insert_snapshot(pool: &PgPool) -> i64 {
        let suffix = unique_suffix();
        let id: i64 = sqlx::query_scalar(
            r#"
            INSERT INTO brain.entity_snapshots(
                source, entity_type, external_id, canonical_name, payload_hash, payload, fetched_at
            ) VALUES ($1, $2, $3, $4, $5, '{}'::jsonb, now())
            RETURNING id
            "#,
        )
        .bind("fixture")
        .bind("patch")
        .bind(format!("patch-snap-{suffix}"))
        .bind("Patch Fixture")
        .bind(format!("snap-hash-{suffix}"))
        .fetch_one(pool)
        .await
        .expect("insert snapshot");
        id
    }

    async fn insert_entity(pool: &PgPool, canonical_name: &str) -> i64 {
        let suffix = unique_suffix();
        let id: i64 = sqlx::query_scalar(
            r#"
            INSERT INTO brain.entities(
                entity_type, canonical_name, primary_external_id, source, metadata,
                created_at, updated_at
            ) VALUES ($1, $2, $3, $4, '{}'::jsonb, now(), now())
            RETURNING id
            "#,
        )
        .bind("hero")
        .bind(canonical_name)
        .bind(format!("ext-{suffix}"))
        .bind("fixture")
        .fetch_one(pool)
        .await
        .expect("insert entity");
        id
    }

    async fn insert_patch_event(
        pool: &PgPool,
        snapshot_id: i64,
        entity_name: &str,
        line: &str,
        event_hash: &str,
    ) -> i64 {
        let id: i64 = sqlx::query_scalar(
            r#"
            INSERT INTO brain.patch_events(
                patch_snapshot_id, legacy_patch_snapshot_id, patch_external_id, patch_title,
                patch_url, source_kind, posted_at, line_index, section, entity_type, entity_name,
                subject, change_type, raw_line, normalized_line, old_value, new_value, confidence,
                metadata, event_hash, created_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, now(), $7, $8, $9, $10, $11, $12, $13, $14, NULL, NULL, $15,
                '{}'::jsonb, $16, now()
            )
            RETURNING id
            "#,
        )
        .bind(snapshot_id)
        .bind(snapshot_id)
        .bind("patch-fixture")
        .bind("Patch Fixture")
        .bind("https://example.invalid/patch")
        .bind("patch_notes")
        .bind(1_i64)
        .bind("Heroes")
        .bind("hero")
        .bind(entity_name)
        .bind(entity_name)
        .bind("buff")
        .bind(line)
        .bind(line)
        .bind(1.0_f64)
        .bind(event_hash)
        .fetch_one(pool)
        .await
        .expect("insert patch event");
        id
    }

    async fn cleanup_patch_event(pool: &PgPool, event_hash: &str) {
        // Enrichments haengen per FK ON DELETE CASCADE am Event.
        sqlx::query("DELETE FROM brain.patch_events WHERE event_hash = $1")
            .bind(event_hash)
            .execute(pool)
            .await
            .expect("cleanup patch event");
    }

    async fn cleanup_snapshot(pool: &PgPool, id: i64) {
        sqlx::query("DELETE FROM brain.entity_snapshots WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await
            .expect("cleanup snapshot");
    }

    fn test_config() -> AiConfig {
        let mut settings = deadlock_brain_core::config::load_settings().expect("settings");
        settings.ai_api_key = None;
        settings.ai_model = "accounts/fireworks/models/deepseek-v4-flash".to_string();
        AiConfig::from_settings(&settings)
    }

    #[test]
    fn parses_from_to_with_ability_prefix_like_python() {
        let enrichment = enrich_patch_event_line(
            Some(7),
            "Mystic Shot cooldown reduced from 12s to 10s",
        )
        .expect("parse");

        assert_eq!(enrichment.patch_event_id, Some(7));
        assert_eq!(enrichment.stat_name.as_deref(), Some("Cooldown"));
        assert_eq!(enrichment.ability_name.as_deref(), Some("Mystic Shot"));
        assert_eq!(enrichment.old_value.as_deref(), Some("12"));
        assert_eq!(enrichment.new_value.as_deref(), Some("10"));
        assert_eq!(enrichment.unit.as_deref(), Some("s"));
        assert_eq!(enrichment.confidence, 0.95);
        assert_eq!(
            enrichment.flags,
            vec!["ability_prefix", "direction:reduced"]
        );
    }

    #[test]
    fn parses_relative_delta_and_signs_reductions() {
        let enrichment = enrich_patch_event_line(Some(8), "Fire Rate reduced by 5%")
            .expect("parse");

        assert_eq!(enrichment.stat_name.as_deref(), Some("Fire Rate"));
        assert_eq!(enrichment.new_value.as_deref(), Some("-5"));
        assert_eq!(enrichment.unit.as_deref(), Some("%"));
        assert_eq!(enrichment.confidence, 0.82);
        assert_eq!(
            enrichment.flags,
            vec!["direction:reduced", "relative_delta"]
        );
    }

    #[test]
    fn parses_movement_and_grant_patterns() {
        let moved = enrich_patch_event_line(Some(9), "Moved from T2 to T3").expect("parse moved");
        assert_eq!(moved.stat_name.as_deref(), Some("Moved"));
        assert_eq!(moved.old_value.as_deref(), Some("T2"));
        assert_eq!(moved.new_value.as_deref(), Some("T3"));
        assert_eq!(moved.confidence, 0.68);
        assert_eq!(
            moved.flags,
            vec![
                "bare_from_to",
                "direction:changed",
                "non_numeric_value",
                "unknown_stat_shape"
            ]
        );

        let grant = enrich_patch_event_line(Some(10), "No longer grants 10% Bullet Resist")
            .expect("parse grant");
        assert_eq!(grant.stat_name.as_deref(), Some("Bullet Resist"));
        assert_eq!(grant.old_value.as_deref(), Some("10"));
        assert_eq!(grant.new_value.as_deref(), Some("0"));
        assert_eq!(grant.unit.as_deref(), Some("%"));
        assert_eq!(grant.flags, vec!["removed_grant"]);
    }

    #[tokio::test]
    #[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
    async fn builds_patch_event_enrichments_incrementally_and_rebuilds() {
        let Some(pool) = test_pool().await else {
            return;
        };
        let snapshot_id = insert_snapshot(&pool).await;
        let cooldown_hash = format!("enrich-cd-{}", unique_suffix());
        let bugfix_hash = format!("enrich-fx-{}", unique_suffix());
        insert_patch_event(
            &pool,
            snapshot_id,
            "Abrams",
            "Mystic Shot cooldown reduced from 12s to 10s",
            &cooldown_hash,
        )
        .await;
        insert_patch_event(
            &pool,
            snapshot_id,
            "Abrams",
            "Fixed crash when casting Charge",
            &bugfix_hash,
        )
        .await;

        // Inkrementeller Build ueber alle offenen Events: unsere zwei frischen
        // Fixtures muessen dabei sein.
        let first = build_patch_event_enrichments(&pool, false)
            .await
            .expect("first build");
        assert!(first.enrichments_inserted >= 2);

        // Flags-Paritaet fuer das Cooldown-Fixture (per event_hash aufgeloest).
        let flags_text: String = sqlx::query_scalar(
            r#"SELECT pee.flags::text
               FROM brain.patch_event_enrichments pee
               JOIN brain.patch_events pe ON pe.id = pee.patch_event_id
               WHERE pe.event_hash = $1"#,
        )
        .bind(&cooldown_hash)
        .fetch_one(&pool)
        .await
        .expect("cooldown enrichment flags");
        let flags: Value = serde_json::from_str(&flags_text).expect("flags json");
        assert_eq!(flags, json!(["ability_prefix", "direction:reduced"]));

        // Zweiter inkrementeller Build fasst bereits angereicherte Events nicht an
        // (ON CONFLICT DO NOTHING) -> Gesamtzahl bleibt gleich.
        let before = count_enrichments(&pool).await.expect("count");
        build_patch_event_enrichments(&pool, false)
            .await
            .expect("second build");
        assert_eq!(count_enrichments(&pool).await.expect("count"), before);

        cleanup_patch_event(&pool, &cooldown_hash).await;
        cleanup_patch_event(&pool, &bugfix_hash).await;
        cleanup_snapshot(&pool, snapshot_id).await;
    }

    #[tokio::test]
    #[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
    async fn patch_impact_context_and_note_roundtrip() {
        let Some(pool) = test_pool().await else {
            return;
        };
        // Eindeutiger Entity-Name -> Kontext-Load trifft deterministisch nur unser
        // Fixture (statt des globalen `list_pending`-Nichtdeterminismus).
        let entity_name = format!("FixtureHero-{}", unique_suffix());
        let entity_id = insert_entity(&pool, &entity_name).await;
        let snapshot_id = insert_snapshot(&pool).await;
        let event_hash = format!("impact-{}", unique_suffix());
        insert_patch_event(
            &pool,
            snapshot_id,
            &entity_name,
            "Cooldown increased from 10s to 12s",
            &event_hash,
        )
        .await;
        build_patch_event_enrichments(&pool, false)
            .await
            .expect("enrich events");

        let context = build_patch_impact_context(&pool, &entity_name, "hero")
            .await
            .expect("impact context");
        let config = test_config();
        let request = build_patch_impact_request(&context, &config).expect("impact request");
        let fake = "```json\n{\"trend\":\"mixed\",\"affected_areas\":[],\"momentum\":\"stable\",\"key_changes\":[],\"confidence\":0.7}\n```";
        let summary = save_patch_impact_note(
            &pool,
            &context,
            &request.prompt_text,
            Some(fake),
            &config.model,
            "analysis_ready",
        )
        .await
        .expect("save note");
        assert_eq!(summary.entity_name, entity_name);
        assert_eq!(summary.status, "analysis_ready");

        let insights: String = sqlx::query_scalar(
            r#"SELECT insights::text FROM brain.patch_impact_notes WHERE entity_name = $1"#,
        )
        .bind(&entity_name)
        .fetch_one(&pool)
        .await
        .expect("impact note");
        assert!(insights.contains("\"trend\""));
        assert!(insights.contains("mixed"));

        sqlx::query("DELETE FROM brain.patch_impact_notes WHERE entity_name = $1")
            .bind(&entity_name)
            .execute(&pool)
            .await
            .expect("cleanup note");
        cleanup_patch_event(&pool, &event_hash).await;
        cleanup_snapshot(&pool, snapshot_id).await;
        sqlx::query("DELETE FROM brain.entities WHERE id = $1")
            .bind(entity_id)
            .execute(&pool)
            .await
            .expect("cleanup entity");
    }

    #[tokio::test]
    #[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
    async fn meta_trends_use_core_schema_table() {
        let Some(pool) = test_pool().await else {
            return;
        };
        // Mock nutzt feste Entity-Namen -> vorher etwaige Fixture-Reste raeumen.
        for name in ["Abrams", "Infernus"] {
            sqlx::query("DELETE FROM brain.meta_trend_notes WHERE entity_name = $1")
                .bind(name)
                .execute(&pool)
                .await
                .expect("pre-clean");
        }

        let config = test_config();
        let summary = run_meta_trend_analysis_with_chat(&pool, &config, |_request| {
            Ok(json!({"choices":[{"message":{"content":"ok"}}]}))
        })
        .await
        .expect("meta trend analysis");
        assert_eq!(summary.processed, 2);
        assert_eq!(summary.success, 2);
        assert_eq!(summary.failed, 0);

        let count: i64 = sqlx::query_scalar(
            r#"SELECT COUNT(*) FROM brain.meta_trend_notes
               WHERE entity_name IN ('Abrams', 'Infernus')"#,
        )
        .fetch_one(&pool)
        .await
        .expect("meta note count");
        assert_eq!(count, 2);

        for name in ["Abrams", "Infernus"] {
            sqlx::query("DELETE FROM brain.meta_trend_notes WHERE entity_name = $1")
                .bind(name)
                .execute(&pool)
                .await
                .expect("cleanup");
        }
    }
}
