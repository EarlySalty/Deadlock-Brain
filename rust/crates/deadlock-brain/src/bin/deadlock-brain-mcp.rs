#![forbid(unsafe_code)]

use std::io::{self, BufRead, Write};

use anyhow::{anyhow, ensure, Context, Result};
use chrono::NaiveDate;
use deadlock_brain_core::pg;
use serde_json::{json, Value};
use sqlx::{postgres::PgPool, QueryBuilder};

const SERVER_NAME: &str = "dl-brain";
const SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<()> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()?;
    let pool = runtime.block_on(pg::pg_pool_read_only())?;
    serve_stdio(&runtime, &pool)
}

fn serve_stdio(runtime: &tokio::runtime::Runtime, pool: &PgPool) -> Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::BufWriter::new(io::stdout().lock());
    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let request: Value = match serde_json::from_str(&line) {
            Ok(value) => value,
            Err(_) => {
                write_response(
                    &mut stdout,
                    &json!({"jsonrpc":"2.0","id":null,"error":{"code":-32700,"message":"Ungültiges JSON."}}),
                )?;
                continue;
            }
        };
        let Some(method) = request.get("method").and_then(Value::as_str) else {
            write_response(
                &mut stdout,
                &json!({"jsonrpc":"2.0","id":request.get("id"),"error":{"code":-32600,"message":"Ungültige JSON-RPC-Anfrage."}}),
            )?;
            continue;
        };
        if method.starts_with("notifications/") {
            continue;
        }
        let id = request.get("id").cloned().unwrap_or(Value::Null);
        let params = request.get("params").cloned().unwrap_or_else(|| json!({}));
        let response = match method {
            "initialize" => initialize_response(id, &params),
            "ping" => json!({"jsonrpc":"2.0","id":id,"result":{}}),
            "tools/list" => json!({"jsonrpc":"2.0","id":id,"result":{"tools":tool_specs()}}),
            "tools/call" => {
                let called = runtime.block_on(call_tool(pool, &params));
                let (text, is_error) = match called {
                    Ok(value) => (serde_json::to_string(&value)?, false),
                    Err(error) => (json!({"error":error.to_string()}).to_string(), true),
                };
                json!({"jsonrpc":"2.0","id":id,"result":{"content":[{"type":"text","text":text}],"isError":is_error}})
            }
            _ => {
                json!({"jsonrpc":"2.0","id":id,"error":{"code":-32601,"message":"Methode nicht gefunden."}})
            }
        };
        write_response(&mut stdout, &response)?;
    }
    Ok(())
}

fn write_response(stdout: &mut impl Write, response: &Value) -> Result<()> {
    serde_json::to_writer(&mut *stdout, response)?;
    stdout.write_all(b"\n")?;
    stdout.flush()?;
    Ok(())
}

fn initialize_response(id: Value, params: &Value) -> Value {
    let requested = params
        .get("protocolVersion")
        .and_then(Value::as_str)
        .unwrap_or("2025-03-26");
    let version = match requested {
        "2024-11-05" | "2025-03-26" | "2025-06-18" => requested,
        _ => "2025-03-26",
    };
    json!({
        "jsonrpc":"2.0",
        "id":id,
        "result":{
            "protocolVersion":version,
            "capabilities":{"tools":{"listChanged":false}},
            "serverInfo":{"name":SERVER_NAME,"version":SERVER_VERSION},
            "instructions":"Read-only Patchhistorie, gespeicherte Caption-Belege und eigenständige Patchanalysen aus der zentralen Brain-Datenbank. Analysehypothesen sind keine bestätigten Spieldaten."
        }
    })
}

fn tool_specs() -> Value {
    json!([
        {"name":"patch_history","description":"Liest Patchänderungen einer Entity mit optionalen Filtern für Fähigkeit, Stat und Datum.","inputSchema":{"type":"object","properties":{"entity":{"type":"string"},"ability":{"type":"string"},"stat":{"type":"string"},"since":{"type":"string","format":"date"},"limit":{"type":"integer"}},"required":["entity"]}},
        {"name":"patch_search","description":"Sucht in Originalzeilen und Entity-Namen der Patchhistorie.","inputSchema":{"type":"object","properties":{"text":{"type":"string"},"limit":{"type":"integer"}},"required":["text"]}},
        {"name":"list_patches","description":"Listet bekannte Patches mit Datum und Titel, neueste zuerst.","inputSchema":{"type":"object","properties":{"limit":{"type":"integer"}}}},
        {"name":"entity_summary","description":"Fasst Anzahl, ersten und letzten Patch sowie betroffene Fähigkeiten und Stats einer Entity zusammen.","inputSchema":{"type":"object","properties":{"entity":{"type":"string"}},"required":["entity"]}},
        {"name":"change_lookup","description":"Sucht wörtlich UND-verknüpfte Begriffe in Patchquellen, löst Entity-Aliase auf und unterstützt inklusive Datumsgrenzen sowie Pagination.","inputSchema":{"type":"object","properties":{"text":{"type":"string"},"entity":{"type":"string"},"since":{"type":"string","format":"date"},"until":{"type":"string","format":"date"},"limit":{"type":"integer"},"offset":{"type":"integer"}},"required":["text"]}},
        {"name":"video_evidence","description":"Liest gespeicherte Caption-Segmente mit Zeitmarken, Quellenhash und Sprache. Caption-Text ist kein Bildbeleg und bestätigt nicht die Richtigkeit einer Videoaussage.","inputSchema":{"type":"object","properties":{"video_id":{"type":"string"},"text":{"type":"string"},"limit":{"type":"integer"}},"required":["video_id"]}},
        {"name":"patch_insight","description":"Liest die jüngste gespeicherte eigenständige Patchanalyse. Belegte Änderungen und strategische Hypothesen bleiben getrennt; der aktuelle Quellenstand wird separat ausgewiesen.","inputSchema":{"type":"object","properties":{"patch_url":{"type":"string","format":"uri"}},"required":["patch_url"]}}
    ])
}

async fn call_tool(pool: &PgPool, params: &Value) -> Result<Value> {
    let name = required_string(params, "name")?;
    let args = params
        .get("arguments")
        .cloned()
        .unwrap_or_else(|| json!({}));
    match name {
        "patch_history" => patch_history(pool, &args).await,
        "patch_search" => patch_search(pool, &args).await,
        "list_patches" => list_patches(pool, &args).await,
        "entity_summary" => entity_summary(pool, &args).await,
        "change_lookup" => lookup_changes(pool, &args).await,
        "video_evidence" => read_video_evidence(pool, &args).await,
        "patch_insight" => read_insight(pool, &args).await,
        _ => Err(anyhow!("Unbekanntes Tool.")),
    }
}

async fn patch_history(pool: &PgPool, args: &Value) -> Result<Value> {
    let entity = checked_text(required_string(args, "entity")?, "entity", 160)?;
    let ability = optional_text(args, "ability", 160)?;
    let stat = optional_text(args, "stat", 160)?;
    let since = optional_date(args, "since")?;
    let limit = bounded_int(args, "limit", 100, 500, 1)?;
    let mut query = QueryBuilder::<sqlx::Postgres>::new(
        "SELECT jsonb_build_object('patch_title',c.patch_title,'patch_date',c.patch_date,'entity_type',c.entity_type,'entity_name',c.entity_name,'ability_name',c.ability_name,'stat_name',c.stat_name,'old_value',c.old_value,'new_value',c.new_value,'change_type',c.change_type,'numeric_direction',c.numeric_direction,'raw_line',c.raw_line,'patch_url',c.patch_url,'confidence',c.confidence)::text FROM brain.patch_changes c WHERE lower(c.entity_name)=lower(",
    );
    query.push_bind(entity).push(")");
    if let Some(ability) = ability {
        query
            .push(" AND c.ability_name ILIKE ")
            .push_bind(literal_pattern(&ability));
    }
    if let Some(stat) = stat {
        query
            .push(" AND c.stat_name ILIKE ")
            .push_bind(literal_pattern(&stat));
    }
    if let Some(since) = since {
        query
            .push(" AND c.patch_date >= ")
            .push_bind(since)
            .push("::date");
    }
    query
        .push(" ORDER BY c.patch_date DESC,c.stat_name DESC LIMIT ")
        .push_bind(limit);
    let rows: Vec<String> = query.build_query_scalar().fetch_all(pool).await?;
    json_rows(rows)
}

async fn patch_search(pool: &PgPool, args: &Value) -> Result<Value> {
    let text = checked_text(required_string(args, "text")?, "text", 512)?;
    let limit = bounded_int(args, "limit", 50, 500, 1)?;
    let rows: Vec<String> = sqlx::query_scalar(
        "SELECT jsonb_build_object('patch_title',c.patch_title,'patch_date',c.patch_date,'entity_type',c.entity_type,'entity_name',c.entity_name,'ability_name',c.ability_name,'stat_name',c.stat_name,'old_value',c.old_value,'new_value',c.new_value,'change_type',c.change_type,'numeric_direction',c.numeric_direction,'raw_line',c.raw_line,'patch_url',c.patch_url,'confidence',c.confidence)::text FROM brain.patch_changes c WHERE c.raw_line ILIKE $1 OR c.entity_name ILIKE $1 ORDER BY c.patch_date DESC LIMIT $2",
    )
    .bind(format!("%{text}%"))
    .bind(limit)
    .fetch_all(pool)
    .await?;
    json_rows(rows)
}

async fn list_patches(pool: &PgPool, args: &Value) -> Result<Value> {
    let limit = bounded_int(args, "limit", 100, 500, 1)?;
    let rows: Vec<String> = sqlx::query_scalar(
        "SELECT jsonb_build_object('patch_date',patch_date,'patch_title',patch_title)::text FROM (SELECT DISTINCT patch_date,patch_title FROM brain.patch_changes ORDER BY patch_date DESC LIMIT $1) p",
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;
    json_rows(rows)
}

async fn entity_summary(pool: &PgPool, args: &Value) -> Result<Value> {
    let entity = checked_text(required_string(args, "entity")?, "entity", 160)?;
    let row: String = sqlx::query_scalar(
        "SELECT jsonb_build_object('entity_name',$1,'change_count',count(*)::int,'first_patch_date',min(patch_date),'last_patch_date',max(patch_date),'abilities',coalesce(array_agg(DISTINCT ability_name ORDER BY ability_name) FILTER (WHERE ability_name IS NOT NULL),ARRAY[]::text[]),'stats',coalesce(array_agg(DISTINCT stat_name ORDER BY stat_name) FILTER (WHERE stat_name IS NOT NULL),ARRAY[]::text[]))::text FROM brain.patch_changes WHERE lower(entity_name)=lower($1)",
    )
    .bind(entity)
    .fetch_one(pool)
    .await?;
    serde_json::from_str(&row).context("Entity-Zusammenfassung ist kein gültiges JSON")
}

async fn lookup_changes(pool: &PgPool, args: &Value) -> Result<Value> {
    let text = required_string(args, "text")?.trim().to_owned();
    let terms: Vec<String> =
        text.split_whitespace()
            .map(str::to_owned)
            .fold(Vec::new(), |mut unique, term| {
                if !unique.contains(&term) {
                    unique.push(term);
                }
                unique
            });
    ensure!(
        !terms.is_empty() && text.chars().count() <= 256 && terms.len() <= 12,
        "text muss 1 bis 12 Suchwörter und höchstens 256 Zeichen enthalten."
    );
    let entity = optional_text(args, "entity", 160)?;
    let since = optional_date(args, "since")?;
    let until = optional_date(args, "until")?;
    ensure!(
        since
            .as_ref()
            .zip(until.as_ref())
            .map_or(true, |(start, end)| start <= end),
        "since darf nicht nach until liegen."
    );
    let limit = bounded_int(args, "limit", 50, 200, 1)?;
    let offset = bounded_int(args, "offset", 0, 100_000, 0)?;

    let mut query = QueryBuilder::<sqlx::Postgres>::new("WITH ");
    if let Some(entity) = &entity {
        query
            .push("requested_entities AS (SELECT DISTINCT e.id,e.canonical_name FROM brain.entities e LEFT JOIN brain.entity_aliases a ON a.entity_id=e.id WHERE lower(e.canonical_name)=lower(")
            .push_bind(entity)
            .push(") OR lower(a.alias)=lower(")
            .push_bind(entity)
            .push(")), requested_names AS (SELECT lower(")
            .push_bind(entity)
            .push("::text) AS name UNION SELECT lower(canonical_name) FROM requested_entities UNION SELECT lower(a.alias) FROM brain.entity_aliases a JOIN requested_entities e ON e.id=a.entity_id), ");
    }
    query.push(
        "matched AS (SELECT DISTINCT c.patch_title,c.patch_date,c.entity_type,c.entity_name,c.ability_name,c.stat_name,c.old_value,c.new_value,c.change_type,c.numeric_direction,c.raw_line,c.patch_url,c.confidence FROM brain.patch_changes c WHERE ",
    );
    for (index, term) in terms.iter().enumerate() {
        if index > 0 {
            query.push(" AND ");
        }
        query
            .push("concat_ws(' ',c.raw_line,c.entity_name,c.ability_name,c.stat_name) ILIKE ")
            .push_bind(literal_pattern(term))
            .push(" ESCAPE '!'");
    }
    if entity.is_some() {
        query.push(" AND lower(c.entity_name) IN (SELECT name FROM requested_names)");
    }
    if let Some(since) = &since {
        query
            .push(" AND c.patch_date >= ")
            .push_bind(since)
            .push("::date");
    }
    if let Some(until) = &until {
        query
            .push(" AND c.patch_date <= ")
            .push_bind(until)
            .push("::date");
    }
    query.push(
        "), page AS (SELECT * FROM matched ORDER BY patch_date ASC NULLS LAST,patch_url,entity_name,raw_line LIMIT ",
    );
    query
        .push_bind(limit)
        .push(" OFFSET ")
        .push_bind(offset)
        .push("), indexed_page AS (SELECT p.*,i.first_indexed_at FROM page p LEFT JOIN LATERAL (SELECT min(e.created_at)::text AS first_indexed_at FROM brain.patch_events e WHERE e.patch_url=p.patch_url AND e.raw_line=p.raw_line) i ON true) SELECT jsonb_build_object('match_count',(SELECT count(*) FROM matched),'earliest_matching_patch',(SELECT min(patch_date) FROM matched),'latest_matching_patch',(SELECT max(patch_date) FROM matched),'changes',coalesce((SELECT jsonb_agg(to_jsonb(p) ORDER BY p.patch_date ASC NULLS LAST,p.patch_url,p.entity_name,p.raw_line) FROM indexed_page p),'[]'::jsonb))::text");
    let row: String = query.build_query_scalar().fetch_one(pool).await?;
    let mut result: Value = serde_json::from_str(&row)?;
    let count = result["match_count"].as_i64().unwrap_or_default();
    let page_len = result["changes"].as_array().map_or(0, Vec::len) as i64;
    let next_offset = offset + page_len;
    result["query"] = json!(text);
    result["entity"] = json!(entity);
    result["since"] = json!(since);
    result["until"] = json!(until);
    result["next_offset"] = if next_offset < count {
        json!(next_offset)
    } else {
        Value::Null
    };
    result["date_semantics"] = json!({
        "patch_date":"Datum des Patch-Beitrags, nicht unabhängig bestätigter Live-Zeitpunkt.",
        "first_indexed_at":"Früheste erhaltene Indexierungszeit dieser exakten Patchzeile; kann nach Rebuilds später sein."
    });
    result["coverage"] = json!(
        "Treffer im vorhandenen Index und Suchfenster, keine Garantie vollständiger Spielhistorie."
    );
    result["authority"] =
        json!("Patchquellen; keine Creator-Bewertung und keine abgeleitete Meta-Prognose.");
    Ok(result)
}

async fn read_video_evidence(pool: &PgPool, args: &Value) -> Result<Value> {
    let video_id = required_string(args, "video_id")?;
    ensure!(
        !video_id.is_empty()
            && video_id.len() <= 128
            && video_id
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-')),
        "Ungültige Video-ID."
    );
    let text = optional_text(args, "text", 512)?.unwrap_or_default();
    let limit = bounded_int(args, "limit", 20, 100, 1)?;
    let row: Option<String> = sqlx::query_scalar(
        "WITH current_evidence AS (SELECT e.*,v.url AS source_url FROM brain.youtube_transcript_evidence e JOIN brain.youtube_transcripts t ON t.video_id=e.video_id AND t.source_kind=e.source_kind AND t.content_hash=e.text_sha256 JOIN brain.youtube_videos v ON v.video_id=e.video_id AND v.metadata->>'transcript_evidence_hash'=e.raw_sha256 WHERE e.video_id=$1 ORDER BY e.observed_at DESC,e.raw_sha256 LIMIT 1), matching_segments AS (SELECT s.value AS segment,s.ordinality FROM current_evidence e CROSS JOIN LATERAL jsonb_array_elements(e.segments) WITH ORDINALITY AS s(value,ordinality) WHERE s.value->>'text' ILIKE $2 ESCAPE '!'), page AS (SELECT segment,ordinality FROM matching_segments ORDER BY ordinality LIMIT $3) SELECT jsonb_build_object('video_id',e.video_id,'source_url',e.source_url,'language',e.language,'source_kind',e.source_kind,'timing_status',e.timing_status,'evidence_hash',e.raw_sha256,'transcript_hash',e.text_sha256,'parser_version',e.parser_version,'match_count',(SELECT count(*) FROM matching_segments),'segments',coalesce((SELECT jsonb_agg(p.segment ORDER BY p.ordinality) FROM page p),'[]'::jsonb))::text FROM current_evidence e",
    )
    .bind(video_id)
    .bind(literal_pattern(text.trim()))
    .bind(limit)
    .fetch_optional(pool)
    .await?;
    Ok(json!({
        "found":row.is_some(),
        "evidence":row.map(|value|serde_json::from_str::<Value>(&value)).transpose()?,
        "evidence_type":"caption_text_not_visual_observation",
        "search_scope":"Wörtliche Suche innerhalb einzelner Caption-Segmente; fehlende Zeitmarken bleiben null.",
        "authority":"Belegt, was in den Untertiteln steht, nicht die Richtigkeit der Spielaussage."
    }))
}

async fn read_insight(pool: &PgPool, args: &Value) -> Result<Value> {
    let patch_url = checked_text(required_string(args, "patch_url")?, "patch_url", 2048)?;
    let row: Option<String> = sqlx::query_scalar(
        "SELECT jsonb_build_object('id',r.id,'patch_url',r.patch_url,'patch_date',r.patch_date,'created_at',r.created_at,'context_hash',r.context_hash,'status',CASE WHEN r.result#>>'{context,patch_source_hash}'=p.current_hash THEN r.status ELSE 'stale' END,'source_revision_current',(r.result#>>'{context,patch_source_hash}'=p.current_hash),'newer_indexed_patch_exists',EXISTS(SELECT 1 FROM brain.patch_changes c WHERE c.patch_date>r.patch_date),'result',r.result-'context','warnings',r.warnings)::text FROM brain.patch_insight_runs r CROSS JOIN LATERAL (SELECT md5(coalesce(string_agg(to_jsonb(c)::text,chr(10) ORDER BY to_jsonb(c)::text),'')) AS current_hash FROM brain.patch_changes c WHERE c.patch_url=r.patch_url) p WHERE r.patch_url=$1 ORDER BY r.created_at DESC,r.id DESC LIMIT 1",
    )
    .bind(patch_url)
    .fetch_optional(pool)
    .await?;
    Ok(json!({
        "found":row.is_some(),
        "analysis":row.map(|value|serde_json::from_str::<Value>(&value)).transpose()?,
        "authority":"Eigene, quellengebundene Analyse zum gespeicherten Patchstand. Hypothesen sind keine bestätigten Meta-Fakten.",
        "current_patch_verified":false
    }))
}

fn json_rows(rows: Vec<String>) -> Result<Value> {
    rows.into_iter()
        .map(|row| serde_json::from_str(&row).context("Datenbankzeile ist kein gültiges JSON"))
        .collect::<Result<Vec<Value>>>()
        .map(Value::Array)
}

fn required_string<'a>(value: &'a Value, name: &str) -> Result<&'a str> {
    value
        .get(name)
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("{name} fehlt oder ist kein Text."))
}

fn optional_text(value: &Value, name: &str, max_chars: usize) -> Result<Option<String>> {
    let Some(raw) = value.get(name) else {
        return Ok(None);
    };
    if raw.is_null() {
        return Ok(None);
    }
    let text = raw
        .as_str()
        .ok_or_else(|| anyhow!("{name} muss Text sein."))?
        .trim();
    ensure!(text.chars().count() <= max_chars, "{name} ist zu lang.");
    Ok((!text.is_empty()).then(|| text.to_owned()))
}

fn checked_text(value: &str, name: &str, max_chars: usize) -> Result<String> {
    let text = value.trim();
    ensure!(!text.is_empty(), "{name} fehlt.");
    ensure!(text.chars().count() <= max_chars, "{name} ist zu lang.");
    Ok(text.to_owned())
}

fn optional_date(args: &Value, name: &str) -> Result<Option<String>> {
    let Some(value) = optional_text(args, name, 10)? else {
        return Ok(None);
    };
    let parsed = NaiveDate::parse_from_str(&value, "%Y-%m-%d")
        .with_context(|| format!("{name} muss ein ISO-Datum YYYY-MM-DD sein."))?;
    ensure!(
        parsed.format("%Y-%m-%d").to_string() == value,
        "{name} muss ein ISO-Datum YYYY-MM-DD sein."
    );
    Ok(Some(value))
}

fn bounded_int(args: &Value, name: &str, default: i64, max: i64, min: i64) -> Result<i64> {
    let Some(value) = args.get(name) else {
        return Ok(default);
    };
    let value = value
        .as_i64()
        .ok_or_else(|| anyhow!("{name} muss eine Ganzzahl sein."))?;
    Ok(value.clamp(min, max))
}

fn literal_pattern(value: &str) -> String {
    format!(
        "%{}%",
        value
            .replace('!', "!!")
            .replace('%', "!%")
            .replace('_', "!_")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_search_escapes_sql_wildcards() {
        assert_eq!(literal_pattern("20%_!"), "%20!%!_!!%");
    }

    #[test]
    fn parses_strict_iso_dates() {
        assert_eq!(
            optional_date(&json!({"since":"2026-09-18"}), "since").unwrap(),
            Some("2026-09-18".into())
        );
        assert!(optional_date(&json!({"since":"2026-9-18"}), "since").is_err());
    }

    #[test]
    fn tool_list_contains_read_only_history_contracts() {
        let tools = tool_specs();
        assert_eq!(tools.as_array().unwrap().len(), 7);
        assert!(tools
            .as_array()
            .unwrap()
            .iter()
            .any(|tool| tool["name"] == "change_lookup"));
        assert!(tools
            .as_array()
            .unwrap()
            .iter()
            .any(|tool| tool["name"] == "video_evidence"));
    }
}
