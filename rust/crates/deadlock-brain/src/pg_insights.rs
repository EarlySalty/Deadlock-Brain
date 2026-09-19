use std::{collections::BTreeSet, env};

use anyhow::{anyhow, ensure, Context, Result};
use chrono::{NaiveDate, TimeZone, Utc};
use postgres::{Client, NoTls, Transaction};
use serde::Deserialize;
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct ImportInsightOptions { pub dsn_env: String, pub dry_run: bool }

#[derive(Debug, Deserialize)]
struct InsightInput {
    insight_hash: Option<String>, insight_type: String,
    entity_type: Option<String>, entity_name: Option<String>, subject: Option<String>,
    summary: String, reason: Option<String>, validity_status: Option<String>,
    currentness: Option<String>, trust_tier: Option<String>, confidence: Option<f64>,
    source_patch_event_ids: Option<Vec<i64>>, source_urls: Option<Vec<String>>,
    source_references: Option<Value>, payload: Option<Value>, metadata: Option<Value>,
}

#[derive(Debug)]
struct PreparedInsight {
    insight_hash: String, insight_type: String, entity_type: Option<String>,
    entity_name: Option<String>, subject: Option<String>, summary: String,
    reason: Option<String>, validity_status: String, currentness: String,
    confidence: f64, occurred_at: Option<String>, source_patch_event_ids: Vec<i64>,
    source_urls: Vec<String>, source_references: String, payload: String, metadata: String,
}

pub fn import_insights_json(raw: &str, options: &ImportInsightOptions) -> Result<Value> {
    let mut prepared = parse_inputs(raw)?.into_iter().map(prepare_input).collect::<Result<Vec<_>>>()?;
    if options.dry_run {
        return Ok(json!({"dry_run":true,"target":"postgres","dsn_env":options.dsn_env,
            "insights":prepared.len(),"writes":false,"evidence_validation":"not_run",
            "trust_tier":"curated_agent_inference","default_validity":"needs_review"}));
    }
    let dsn = env::var(&options.dsn_env).map_err(|_| anyhow!("Database connection is not configured; DSN redacted"))?;
    let mut client = Client::connect(&dsn, NoTls).map_err(|_| anyhow!("Database connection failed; DSN redacted"))?;
    client.batch_execute("SET statement_timeout='15s'")?;
    ensure_pg_schema(&mut client)?;
    let mut tx = client.transaction()?;
    for insight in &mut prepared { resolve_event_references(&mut tx, insight)?; }
    let ids = upsert_insights(&mut tx, &prepared)?;
    let knowledge_events = materialize_insight_knowledge_events(&mut tx, &ids)?;
    tx.commit()?;
    Ok(json!({"dry_run":false,"target":"postgres","dsn_env":options.dsn_env,
        "imported":{"insight_records":ids.len(),"knowledge_events_from_insights":knowledge_events},
        "policy":{"trust":"Agenten-Ableitungen werden niemals durch Confidence zu Patchhistorie.",
            "validation":"Ereignis-IDs und deren Quellen wurden aufgelöst, die Semantik bleibt ungeprüft.",
            "scope":"Nur diese importierten Insights wurden materialisiert."}}))
}

fn parse_inputs(raw: &str) -> Result<Vec<InsightInput>> {
    let value: Value = serde_json::from_str(raw).context("lese Insight-JSON")?;
    match value {
        Value::Array(_) => serde_json::from_value(value).context("lese Insight-Array"),
        Value::Object(_) => Ok(vec![serde_json::from_value(value).context("lese einzelnen Insight")?]),
        _ => Err(anyhow!("Insight-JSON muss ein Objekt oder Array sein.")),
    }
}

fn prepare_input(input: InsightInput) -> Result<PreparedInsight> {
    ensure!(!input.insight_type.trim().is_empty(), "insight_type fehlt.");
    ensure!(!input.summary.trim().is_empty(), "summary fehlt.");
    let confidence = input.confidence.unwrap_or(0.7);
    ensure!(confidence.is_finite() && (0.0..=1.0).contains(&confidence), "confidence muss endlich und zwischen 0 und 1 sein.");
    let ids = input.source_patch_event_ids.unwrap_or_default().into_iter().collect::<BTreeSet<_>>();
    ensure!(ids.iter().all(|id| *id > 0), "Patch-Event-IDs müssen positiv sein.");
    let source_patch_event_ids = ids.into_iter().collect::<Vec<_>>();
    let source_urls = input.source_urls.unwrap_or_default().into_iter().collect::<BTreeSet<_>>().into_iter().collect::<Vec<_>>();
    ensure!(source_urls.iter().all(|url| url.starts_with("https://") || url.starts_with("http://")), "Quellen müssen HTTP(S)-Referenzen sein.");
    let raw_currentness = input.currentness.unwrap_or_else(|| "needs_revalidation".to_string());
    let (currentness, occurred_at, patch_date) = normalize_currentness(&raw_currentness);
    let mut payload = object(input.payload);
    if let Some(date) = patch_date { payload.insert("patch_date".into(), json!(date)); }
    payload.insert("source_patch_event_ids".into(), json!(source_patch_event_ids));
    payload.insert("source_urls".into(), json!(source_urls));
    let mut metadata = object(input.metadata);
    metadata.insert("importer".into(), json!("agent_insight_v2"));
    metadata.insert("requested_trust_tier".into(), json!(input.trust_tier));
    metadata.insert("requested_validity_status".into(), json!(input.validity_status));
    metadata.insert("requested_currentness".into(), json!(raw_currentness));
    metadata.insert("requested_source_references".into(), json!(input.source_references));
    metadata.insert("semantic_verification".into(), json!("not_performed"));
    let payload = serde_json::to_string(&payload)?;
    let metadata = serde_json::to_string(&metadata)?;
    let hash_basis = json!(["agent_insight_v2", input.insight_type, input.entity_type,
        input.entity_name, input.summary, currentness, occurred_at, source_patch_event_ids, payload]);
    let insight_hash = input.insight_hash.unwrap_or_else(|| hex::encode(Sha256::digest(hash_basis.to_string().as_bytes())));
    ensure!(!insight_hash.trim().is_empty(), "insight_hash fehlt.");
    Ok(PreparedInsight {
        insight_hash, insight_type:input.insight_type, entity_type:input.entity_type,
        entity_name:input.entity_name, subject:input.subject, summary:input.summary,
        reason:input.reason, validity_status:if input.validity_status.as_deref() == Some("rejected") { "rejected" } else { "needs_review" }.into(),
        currentness, confidence, occurred_at, source_patch_event_ids, source_urls,
        source_references:"[]".into(), payload, metadata,
    })
}

fn object(value: Option<Value>) -> Map<String, Value> {
    match value { Some(Value::Object(map)) => map, Some(other) => { let mut map=Map::new(); map.insert("value".into(),other); map }, None => Map::new() }
}

fn normalize_currentness(raw: &str) -> (String, Option<String>, Option<String>) {
    if let Ok(date) = NaiveDate::parse_from_str(raw.trim(), "%Y-%m-%d") {
        return ("patch_scoped".into(), Some(Utc.from_utc_datetime(&date.and_hms_opt(0,0,0).expect("valid midnight")).to_rfc3339()), Some(raw.trim().into()));
    }
    ("needs_revalidation".into(), None, None)
}

fn ensure_pg_schema(client: &mut Client) -> Result<()> {
    let row=client.query_one("SELECT to_regclass('brain.insight_records')::text, to_regclass('brain.knowledge_events')::text", &[])?;
    ensure!(row.get::<_,Option<String>>(0).is_some() && row.get::<_,Option<String>>(1).is_some(), "Zentrales Insight-Schema fehlt. Erst Central-DB-Migration 0013 anwenden.");
    Ok(())
}

fn resolve_event_references(tx: &mut Transaction<'_>, insight: &mut PreparedInsight) -> Result<()> {
    let rows=tx.query("SELECT id, patch_url, event_hash, patch_external_id FROM brain.patch_events WHERE id=ANY($1) ORDER BY id FOR SHARE", &[&insight.source_patch_event_ids])?;
    ensure!(rows.len() == insight.source_patch_event_ids.len(), "Mindestens eine referenzierte Patch-Event-ID existiert nicht mehr; Import abgebrochen.");
    let mut refs=Vec::new();
    for row in rows {
        refs.push(json!({"patch_event_id":row.get::<_,i64>(0), "url":row.get::<_,Option<String>>(1),
            "event_hash":row.get::<_,String>(2), "patch_external_id":row.get::<_,String>(3),
            "reference_validation":"exists_not_semantically_verified"}));
    }
    for url in &insight.source_urls { refs.push(json!({"url":url,"reference_validation":"external_unverified"})); }
    insight.source_references=serde_json::to_string(&refs)?;
    Ok(())
}

fn upsert_insights(tx: &mut Transaction<'_>, insights: &[PreparedInsight]) -> Result<Vec<i64>> {
    let mut ids=Vec::new();
    for i in insights {
        let row=tx.query_opt(r#"
            INSERT INTO brain.insight_records AS existing(
                insight_hash,insight_type,entity_type,entity_name,subject,summary,reason,
                validity_status,currentness,trust_tier,confidence,occurred_at,
                source_patch_event_ids,source_urls,source_references,payload,metadata
            ) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,'curated_agent_inference',$10,$11::text::timestamptz,
                $12,$13,$14::text::jsonb,$15::text::jsonb,$16::text::jsonb)
            ON CONFLICT(insight_hash) DO UPDATE SET
                insight_type=EXCLUDED.insight_type,entity_type=EXCLUDED.entity_type,
                entity_name=EXCLUDED.entity_name,subject=EXCLUDED.subject,summary=EXCLUDED.summary,
                reason=EXCLUDED.reason,validity_status=EXCLUDED.validity_status,currentness=EXCLUDED.currentness,
                trust_tier=EXCLUDED.trust_tier,confidence=EXCLUDED.confidence,occurred_at=EXCLUDED.occurred_at,
                source_patch_event_ids=EXCLUDED.source_patch_event_ids,source_urls=EXCLUDED.source_urls,
                source_references=EXCLUDED.source_references,payload=EXCLUDED.payload,metadata=EXCLUDED.metadata,updated_at=now()
            WHERE existing.trust_tier IN ('curated_agent_inference','unverified')
            RETURNING id
        "#, &[&i.insight_hash,&i.insight_type,&i.entity_type,&i.entity_name,&i.subject,&i.summary,&i.reason,
            &i.validity_status,&i.currentness,&i.confidence,&i.occurred_at,&i.source_patch_event_ids,
            &i.source_urls,&i.source_references,&i.payload,&i.metadata])?
            .context("Ein bestehender höher eingestufter Datensatz darf nicht durch Agenten-Import überschrieben werden.")?;
        ids.push(row.get(0));
    }
    Ok(ids)
}

fn materialize_insight_knowledge_events(tx: &mut Transaction<'_>, ids: &[i64]) -> Result<u64> {
    Ok(tx.execute(r#"
        INSERT INTO brain.knowledge_events(
            event_hash,event_source,source_table,source_legacy_id,entity_type,entity_name,
            subject,event_type,validity_status,currentness,trust_tier,source_url,occurred_at,
            observed_at,raw_text,normalized_text,confidence,source_references,payload,metadata
        ) SELECT 'insight:'||ir.insight_hash,'curated_insight','insight_records',ir.id,
            ir.entity_type,ir.entity_name,ir.subject,ir.insight_type,ir.validity_status,ir.currentness,
            ir.trust_tier,ir.source_urls[1],ir.occurred_at,ir.observed_at,ir.summary,ir.summary,
            ir.confidence,ir.source_references,ir.payload,ir.metadata
        FROM brain.insight_records ir WHERE ir.id=ANY($1)
        ON CONFLICT(event_hash) DO UPDATE SET
            source_legacy_id=EXCLUDED.source_legacy_id,entity_type=EXCLUDED.entity_type,
            entity_name=EXCLUDED.entity_name,subject=EXCLUDED.subject,event_type=EXCLUDED.event_type,
            validity_status=EXCLUDED.validity_status,currentness=EXCLUDED.currentness,
            trust_tier=EXCLUDED.trust_tier,source_url=EXCLUDED.source_url,occurred_at=EXCLUDED.occurred_at,
            observed_at=EXCLUDED.observed_at,raw_text=EXCLUDED.raw_text,normalized_text=EXCLUDED.normalized_text,
            confidence=EXCLUDED.confidence,source_references=EXCLUDED.source_references,
            payload=EXCLUDED.payload,metadata=EXCLUDED.metadata,updated_at=now()
    "#, &[&ids])?)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn prepare(extra: Value) -> Result<PreparedInsight> {
        let mut input=json!({"insight_type":"patch_analysis","summary":"Conditional consequence"});
        for (key,value) in extra.as_object().unwrap() { input[key]=value.clone(); }
        prepare_input(serde_json::from_value(input).unwrap())
    }
    #[test] fn high_confidence_cannot_promote_to_history() {
        let p=prepare(json!({"confidence":1.0,"trust_tier":"trusted","validity_status":"patch_history"})).unwrap();
        assert_eq!(p.validity_status,"needs_review");
        assert!(p.metadata.contains("not_performed"));
        assert!(p.metadata.contains("trusted"));
    }
    #[test] fn dates_are_patch_scoped_not_permanently_current() {
        let p=prepare(json!({"currentness":"2026-09-16"})).unwrap();
        assert_eq!(p.currentness,"patch_scoped");
        assert!(p.payload.contains("2026-09-16"));
        assert_eq!(prepare(json!({"currentness":"current_until_superseded"})).unwrap().currentness,"needs_revalidation");
    }
    #[test] fn rejects_out_of_range_confidence() {
        assert!(prepare(json!({"confidence":1.1})).is_err());
        assert!(prepare(json!({"confidence":-0.1})).is_err());
    }
    #[test] fn event_ids_are_positive_sorted_and_unique() {
        assert!(prepare(json!({"source_patch_event_ids":[-1]})).is_err());
        assert_eq!(prepare(json!({"source_patch_event_ids":[2,1,2]})).unwrap().source_patch_event_ids,vec![1,2]);
    }
    #[test] fn event_urls_are_not_paired_by_array_position() {
        let p=prepare(json!({"source_patch_event_ids":[1],"source_urls":["https://example.com/unrelated"]})).unwrap();
        assert_eq!(p.source_references,"[]");
    }
    #[test] fn hash_includes_patch_scope() {
        assert_ne!(prepare(json!({"currentness":"2026-09-16"})).unwrap().insight_hash,
            prepare(json!({"currentness":"2026-09-17"})).unwrap().insight_hash);
    }
}
