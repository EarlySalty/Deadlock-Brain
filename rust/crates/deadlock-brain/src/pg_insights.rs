use std::env;

use anyhow::{anyhow, Context, Result};
use chrono::{NaiveDate, TimeZone, Utc};
use postgres::{Client, NoTls, Transaction};
use serde::Deserialize;
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct ImportInsightOptions {
    pub dsn_env: String,
    pub dry_run: bool,
}

#[derive(Debug, Deserialize)]
struct InsightInput {
    insight_hash: Option<String>,
    insight_type: String,
    entity_type: Option<String>,
    entity_name: Option<String>,
    subject: Option<String>,
    summary: String,
    reason: Option<String>,
    validity_status: Option<String>,
    currentness: Option<String>,
    trust_tier: Option<String>,
    confidence: Option<f64>,
    source_patch_event_ids: Option<Vec<i64>>,
    source_urls: Option<Vec<String>>,
    source_references: Option<Value>,
    payload: Option<Value>,
    metadata: Option<Value>,
}

struct PreparedInsight {
    insight_hash: String,
    insight_type: String,
    entity_type: Option<String>,
    entity_name: Option<String>,
    subject: Option<String>,
    summary: String,
    reason: Option<String>,
    validity_status: String,
    currentness: String,
    trust_tier: String,
    confidence: f64,
    occurred_at: Option<String>,
    source_patch_event_ids: Vec<i64>,
    source_urls: Vec<String>,
    source_references: String,
    payload: String,
    metadata: String,
}

pub fn import_insights_json(raw: &str, options: &ImportInsightOptions) -> Result<Value> {
    let inputs = parse_inputs(raw)?;
    let prepared = inputs
        .into_iter()
        .map(prepare_input)
        .collect::<Result<Vec<_>>>()?;

    if options.dry_run {
        return Ok(json!({
            "dry_run": true,
            "target": "postgres",
            "dsn_env": options.dsn_env,
            "insights": prepared.len(),
            "writes": false,
        }));
    }

    let dsn = env::var(&options.dsn_env).map_err(|_| {
        anyhow!(
            "{} ist nicht gesetzt; DSN wird nicht ausgegeben.",
            options.dsn_env
        )
    })?;
    let mut client = Client::connect(&dsn, NoTls).map_err(|_| {
        anyhow!("Konnte zentrale Postgres-DB nicht oeffnen; DSN wird nicht ausgegeben.")
    })?;
    ensure_pg_schema(&mut client)?;

    let mut tx = client.transaction()?;
    let insights = upsert_insights(&mut tx, &prepared)?;
    let knowledge_events = materialize_insight_knowledge_events(&mut tx)?;
    tx.commit()?;

    Ok(json!({
        "dry_run": false,
        "target": "postgres",
        "dsn_env": options.dsn_env,
        "imported": {
            "insight_records": insights,
            "knowledge_events_from_insights": knowledge_events,
        },
        "policy": {
            "currentness": "Datumswerte aus Spark-Inputs werden als current_until_superseded gespeichert und im Payload als patch_date behalten.",
            "trust": "Importierte Agenten-Erkenntnisse ueberschreiben keine Current-State-Snapshots."
        }
    }))
}

fn parse_inputs(raw: &str) -> Result<Vec<InsightInput>> {
    let value: Value = serde_json::from_str(raw).context("lese Insight-JSON")?;
    match value {
        Value::Array(_) => serde_json::from_value(value).context("lese Insight-Array"),
        Value::Object(_) => Ok(vec![
            serde_json::from_value(value).context("lese einzelnen Insight")?
        ]),
        _ => Err(anyhow!("Insight-JSON muss ein Objekt oder Array sein.")),
    }
}

fn prepare_input(input: InsightInput) -> Result<PreparedInsight> {
    if input.insight_type.trim().is_empty() {
        return Err(anyhow!("insight_type fehlt."));
    }
    if input.summary.trim().is_empty() {
        return Err(anyhow!("summary fehlt."));
    }

    let source_patch_event_ids = input.source_patch_event_ids.unwrap_or_default();
    let source_urls = input.source_urls.unwrap_or_default();
    let raw_currentness = input
        .currentness
        .unwrap_or_else(|| "current_until_superseded".to_string());
    let (currentness, occurred_at, patch_date) = normalize_currentness(&raw_currentness);
    let confidence = input.confidence.unwrap_or(0.7);

    let mut payload = match input.payload.unwrap_or_else(|| json!({})) {
        Value::Object(object) => object,
        other => {
            let mut object = Map::new();
            object.insert("value".to_string(), other);
            object
        }
    };
    if let Some(patch_date) = patch_date {
        payload.insert("patch_date".to_string(), Value::String(patch_date));
    }
    payload.insert(
        "source_patch_event_ids".to_string(),
        json!(source_patch_event_ids),
    );
    payload.insert("source_urls".to_string(), json!(source_urls));

    let source_references = source_references_json(
        input.source_references,
        &source_patch_event_ids,
        &source_urls,
    )?;
    let metadata = json_text(input.metadata.unwrap_or_else(|| json!({})))?;
    let payload = json_text(Value::Object(payload))?;

    let insight_hash = input.insight_hash.unwrap_or_else(|| {
        stable_hash(
            &input.insight_type,
            input.entity_type.as_deref(),
            input.entity_name.as_deref(),
            &input.summary,
            &source_patch_event_ids,
        )
    });

    Ok(PreparedInsight {
        insight_hash,
        insight_type: input.insight_type,
        entity_type: input.entity_type,
        entity_name: input.entity_name,
        subject: input.subject,
        summary: input.summary,
        reason: input.reason,
        validity_status: input
            .validity_status
            .unwrap_or_else(|| "curated_patch_insight".to_string()),
        currentness,
        trust_tier: input
            .trust_tier
            .unwrap_or_else(|| trust_tier_for_confidence(confidence).to_string()),
        confidence,
        occurred_at,
        source_patch_event_ids,
        source_urls,
        source_references,
        payload,
        metadata,
    })
}

fn normalize_currentness(raw: &str) -> (String, Option<String>, Option<String>) {
    let trimmed = raw.trim();
    if let Ok(date) = NaiveDate::parse_from_str(trimmed, "%Y-%m-%d") {
        let timestamp = Utc
            .from_utc_datetime(&date.and_hms_opt(0, 0, 0).expect("valid midnight"))
            .to_rfc3339();
        return (
            "current_until_superseded".to_string(),
            Some(timestamp),
            Some(trimmed.to_string()),
        );
    }
    (trimmed.to_string(), None, None)
}

fn trust_tier_for_confidence(confidence: f64) -> &'static str {
    if confidence >= 0.9 {
        "curated_patch_history"
    } else {
        "curated_agent_inference"
    }
}

fn source_references_json(
    explicit: Option<Value>,
    event_ids: &[i64],
    urls: &[String],
) -> Result<String> {
    if let Some(value) = explicit {
        return json_text(value);
    }
    let refs = event_ids
        .iter()
        .enumerate()
        .map(|(index, event_id)| {
            json!({
                "patch_event_id": event_id,
                "url": urls.get(index).or_else(|| urls.first()),
            })
        })
        .collect::<Vec<_>>();
    json_text(Value::Array(refs))
}

fn json_text(value: Value) -> Result<String> {
    serde_json::to_string(&value).context("serialisiere JSON")
}

fn stable_hash(
    insight_type: &str,
    entity_type: Option<&str>,
    entity_name: Option<&str>,
    summary: &str,
    source_patch_event_ids: &[i64],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(insight_type.as_bytes());
    hasher.update(b"\0");
    hasher.update(entity_type.unwrap_or("").as_bytes());
    hasher.update(b"\0");
    hasher.update(entity_name.unwrap_or("").as_bytes());
    hasher.update(b"\0");
    hasher.update(summary.as_bytes());
    hasher.update(b"\0");
    for event_id in source_patch_event_ids {
        hasher.update(event_id.to_string().as_bytes());
        hasher.update(b",");
    }
    hex::encode(hasher.finalize())
}

fn ensure_pg_schema(client: &mut Client) -> Result<()> {
    let row = client
        .query_one(
            "SELECT to_regclass('brain.insight_records')::text, to_regclass('brain.knowledge_events')::text",
            &[],
        )
        .context("pruefe brain-Insight-Schema in Postgres")?;
    let insight_records: Option<String> = row.get(0);
    let knowledge_events: Option<String> = row.get(1);
    if insight_records.is_none() || knowledge_events.is_none() {
        return Err(anyhow!(
            "Zentrales Insight-Schema fehlt. Erst die Central-DB-Migration 0013 anwenden."
        ));
    }
    Ok(())
}

fn upsert_insights(tx: &mut Transaction<'_>, insights: &[PreparedInsight]) -> Result<u64> {
    let mut changed = 0;
    for insight in insights {
        changed += tx.execute(
            r#"
            INSERT INTO brain.insight_records(
                insight_hash, insight_type, entity_type, entity_name, subject, summary, reason,
                validity_status, currentness, trust_tier, confidence, occurred_at,
                source_patch_event_ids, source_urls, source_references, payload, metadata
            )
            VALUES (
                $1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12::text::timestamptz,
                $13,$14,$15::text::jsonb,$16::text::jsonb,$17::text::jsonb
            )
            ON CONFLICT (insight_hash) DO UPDATE SET
                insight_type = EXCLUDED.insight_type,
                entity_type = EXCLUDED.entity_type,
                entity_name = EXCLUDED.entity_name,
                subject = EXCLUDED.subject,
                summary = EXCLUDED.summary,
                reason = EXCLUDED.reason,
                validity_status = EXCLUDED.validity_status,
                currentness = EXCLUDED.currentness,
                trust_tier = EXCLUDED.trust_tier,
                confidence = EXCLUDED.confidence,
                occurred_at = EXCLUDED.occurred_at,
                source_patch_event_ids = EXCLUDED.source_patch_event_ids,
                source_urls = EXCLUDED.source_urls,
                source_references = EXCLUDED.source_references,
                payload = EXCLUDED.payload,
                metadata = EXCLUDED.metadata,
                updated_at = now()
            "#,
            &[
                &insight.insight_hash,
                &insight.insight_type,
                &insight.entity_type,
                &insight.entity_name,
                &insight.subject,
                &insight.summary,
                &insight.reason,
                &insight.validity_status,
                &insight.currentness,
                &insight.trust_tier,
                &insight.confidence,
                &insight.occurred_at,
                &insight.source_patch_event_ids,
                &insight.source_urls,
                &insight.source_references,
                &insight.payload,
                &insight.metadata,
            ],
        )?;
    }
    Ok(changed)
}

fn materialize_insight_knowledge_events(tx: &mut Transaction<'_>) -> Result<u64> {
    Ok(tx.execute(
        r#"
        INSERT INTO brain.knowledge_events(
            event_hash, event_source, source_table, source_legacy_id, entity_type, entity_name,
            subject, event_type, validity_status, currentness, trust_tier, source_url,
            occurred_at, observed_at, raw_text, normalized_text, confidence, source_references,
            payload, metadata
        )
        SELECT
            'insight:' || ir.insight_hash,
            'curated_insight',
            'insight_records',
            ir.id,
            ir.entity_type,
            ir.entity_name,
            ir.subject,
            ir.insight_type,
            ir.validity_status,
            ir.currentness,
            ir.trust_tier,
            ir.source_urls[1],
            ir.occurred_at,
            ir.observed_at,
            ir.summary,
            ir.summary,
            ir.confidence,
            ir.source_references,
            ir.payload,
            ir.metadata
        FROM brain.insight_records ir
        ON CONFLICT (event_hash) DO UPDATE SET
            source_legacy_id = EXCLUDED.source_legacy_id,
            entity_type = EXCLUDED.entity_type,
            entity_name = EXCLUDED.entity_name,
            subject = EXCLUDED.subject,
            event_type = EXCLUDED.event_type,
            validity_status = EXCLUDED.validity_status,
            currentness = EXCLUDED.currentness,
            trust_tier = EXCLUDED.trust_tier,
            source_url = EXCLUDED.source_url,
            occurred_at = EXCLUDED.occurred_at,
            observed_at = EXCLUDED.observed_at,
            raw_text = EXCLUDED.raw_text,
            normalized_text = EXCLUDED.normalized_text,
            confidence = EXCLUDED.confidence,
            source_references = EXCLUDED.source_references,
            payload = EXCLUDED.payload,
            metadata = EXCLUDED.metadata,
            updated_at = now()
        "#,
        &[],
    )?)
}
