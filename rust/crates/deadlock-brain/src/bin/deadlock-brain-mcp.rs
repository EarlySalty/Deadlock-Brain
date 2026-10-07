use std::{
    env,
    io::{self, BufRead, Write},
};

use anyhow::{anyhow, ensure, Context, Result};
use chrono::{DateTime, NaiveDate};
use postgres::{Client, NoTls};
use serde::Deserialize;
use serde_json::{json, Value};

const SERVER_NAME: &str = "dl-brain";
const SERVER_VERSION: &str = "0.1.0";
const DEFAULT_PROTOCOL_VERSION: &str = "2025-06-18";

#[derive(Debug, Deserialize)]
struct RpcRequest {
    #[serde(default)]
    id: Option<Value>,
    method: String,
    #[serde(default)]
    params: Value,
}

fn main() -> Result<()> {
    serve(io::stdin().lock(), io::stdout())
}

fn serve<R: BufRead, W: Write>(reader: R, mut writer: W) -> Result<()> {
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let request = match serde_json::from_str::<RpcRequest>(&line) {
            Ok(request) => request,
            Err(_) => {
                write_json(
                    &mut writer,
                    &json!({"jsonrpc":"2.0","id":Value::Null,"error":{"code":-32700,"message":"Ungültige JSON RPC Nachricht."}}),
                )?;
                continue;
            }
        };
        let Some(id) = request.id.clone() else {
            continue;
        };
        let response = match handle_request(&request) {
            Ok(result) => json!({"jsonrpc":"2.0","id":id,"result":result}),
            Err(error) => json!({
                "jsonrpc":"2.0",
                "id":id,
                "error":{"code":-32602,"message":error.to_string()}
            }),
        };
        write_json(&mut writer, &response)?;
    }
    Ok(())
}

fn write_json(writer: &mut impl Write, value: &Value) -> Result<()> {
    serde_json::to_writer(&mut *writer, value)?;
    writer.write_all(b"\n")?;
    writer.flush()?;
    Ok(())
}

fn handle_request(request: &RpcRequest) -> Result<Value> {
    match request.method.as_str() {
        "initialize" => {
            let requested = request
                .params
                .get("protocolVersion")
                .and_then(Value::as_str)
                .unwrap_or(DEFAULT_PROTOCOL_VERSION);
            Ok(json!({
                "protocolVersion": requested,
                "capabilities":{"tools":{"listChanged":false}},
                "serverInfo":{"name":SERVER_NAME,"version":SERVER_VERSION}
            }))
        }
        "ping" => Ok(json!({})),
        "tools/list" => Ok(json!({"tools":tool_specs()})),
        "tools/call" => call_tool(&request.params),
        _ => Err(anyhow!("MCP Methode wird nicht unterstützt.")),
    }
}

fn tool_specs() -> Vec<Value> {
    vec![
        json!({
            "name":"patch_history_v1",
            "description":"Revisionssichere Patch Historie. Gelöschte oder frühere Fassungen bleiben als Historie erkennbar.",
            "inputSchema":{
                "type":"object",
                "properties":{
                    "query":{"type":"string"},
                    "entity":{"type":"string","default":""},
                    "known_at":{"type":"string","default":""},
                    "limit":{"type":"integer","minimum":1,"maximum":500,"default":100}
                },
                "required":["query"],
                "additionalProperties":false
            },
            "annotations":{"readOnlyHint":true}
        }),
        json!({
            "name":"patch_history",
            "description":"Liest die Patch Historie einer Entity aus brain.patch_changes.",
            "inputSchema":{
                "type":"object",
                "properties":{
                    "entity":{"type":"string"},
                    "ability":{"type":["string","null"]},
                    "stat":{"type":["string","null"]},
                    "since":{"type":["string","null"]},
                    "limit":{"type":"integer","minimum":1,"maximum":500,"default":100}
                },
                "required":["entity"],
                "additionalProperties":false
            },
            "annotations":{"readOnlyHint":true}
        }),
        json!({
            "name":"patch_search",
            "description":"Sucht in Patch Zeilen und Entity Namen.",
            "inputSchema":{
                "type":"object",
                "properties":{
                    "text":{"type":"string"},
                    "limit":{"type":"integer","minimum":1,"maximum":500,"default":50}
                },
                "required":["text"],
                "additionalProperties":false
            },
            "annotations":{"readOnlyHint":true}
        }),
        json!({
            "name":"list_patches",
            "description":"Listet bekannte Patches mit Datum und Titel, neueste zuerst.",
            "inputSchema":{
                "type":"object",
                "properties":{
                    "limit":{"type":"integer","minimum":1,"maximum":500,"default":100}
                },
                "additionalProperties":false
            },
            "annotations":{"readOnlyHint":true}
        }),
        json!({
            "name":"entity_summary",
            "description":"Fasst Anzahl, Zeitraum, Abilities und Stats einer Entity zusammen.",
            "inputSchema":{
                "type":"object",
                "properties":{"entity":{"type":"string"}},
                "required":["entity"],
                "additionalProperties":false
            },
            "annotations":{"readOnlyHint":true}
        }),
    ]
}

fn call_tool(params: &Value) -> Result<Value> {
    let name = required_text(params.get("name"), "name")?;
    let arguments = params
        .get("arguments")
        .filter(|value| value.is_object())
        .cloned()
        .unwrap_or_else(|| json!({}));
    let output = match name.as_str() {
        "patch_history_v1" => patch_history_v1(&arguments),
        "patch_history" => patch_history(&arguments),
        "patch_search" => patch_search(&arguments),
        "list_patches" => list_patches(&arguments),
        "entity_summary" => entity_summary(&arguments),
        _ => Err(anyhow!("Unbekanntes MCP Werkzeug.")),
    };
    match output {
        Ok(value) => Ok(json!({
            "content":[{"type":"text","text":serde_json::to_string_pretty(&value)?}],
            "structuredContent":value,
            "isError":false
        })),
        Err(error) => Ok(json!({
            "content":[{"type":"text","text":error.to_string()}],
            "isError":true
        })),
    }
}

fn connect_read_only() -> Result<Client> {
    let dsn = env::var("DEADLOCK_CENTRAL_DSN")
        .map_err(|_| anyhow!("Datenbank Verbindung ist nicht konfiguriert."))?;
    let mut client = Client::connect(&dsn, NoTls)
        .map_err(|_| anyhow!("Datenbank Verbindung fehlgeschlagen."))?;
    client
        .batch_execute("SET statement_timeout='8s'; SET default_transaction_read_only=on;")
        .map_err(|_| anyhow!("Read only Datenbank Modus konnte nicht gesetzt werden."))?;
    Ok(client)
}

fn patch_history_v1(args: &Value) -> Result<Value> {
    let query = required_text(args.get("query"), "query")?;
    let entity = optional_text(args.get("entity"));
    let known_at = optional_text(args.get("known_at"));
    if let Some(value) = &known_at {
        DateTime::parse_from_rfc3339(value)
            .context("known_at muss RFC3339 mit Zeitzone sein.")?;
    }
    let limit = limit(args.get("limit"), 100)?;
    let mut client = connect_read_only()?;
    let rows = client
        .query(
            r#"
            WITH known AS (
                SELECT DISTINCT ON (event_hash) *
                FROM brain.patch_history_v1
                WHERE ($3::text IS NULL OR observed_at <= $3::text::timestamptz)
                ORDER BY event_hash, revision_id DESC
            )
            SELECT to_jsonb(k)::text
            FROM known k
            WHERE strpos(lower(COALESCE(raw_line,'')), lower($1)) > 0
              AND ($2::text IS NULL OR lower(entity_name) = lower($2))
            ORDER BY source_published_at ASC NULLS LAST, revision_id ASC
            LIMIT $4
            "#,
            &[&query, &entity, &known_at, &(limit + 1)],
        )
        .map_err(|_| anyhow!("Patch Historie konnte nicht gelesen werden."))?;
    let truncated = rows.len() as i64 > limit;
    let hits = rows
        .into_iter()
        .take(limit as usize)
        .map(|row| serde_json::from_str::<Value>(&row.get::<_, String>(0)))
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(json!({
        "query":query,
        "hits":hits,
        "truncated":truncated,
        "scope":"observed_imported_patch_corpus_not_proof_of_first_game_occurrence",
        "time_semantics":"source_published_at is publication; observed_at is this revision's observation. baseline does not establish earlier first-seen.",
        "deleted_semantics":"deleted means removed from the imported table, not reverted in the game",
        "no_hits_semantics":"not found in this corpus, not proof that the change never occurred",
        "source":"brain.patch_history_v1",
        "model_called":false
    }))
}

fn patch_history(args: &Value) -> Result<Value> {
    let entity = required_text(args.get("entity"), "entity")?;
    let ability = optional_text(args.get("ability"));
    let stat = optional_text(args.get("stat"));
    let since = optional_text(args.get("since"));
    if let Some(value) = &since {
        NaiveDate::parse_from_str(value, "%Y-%m-%d")
            .context("since muss YYYY-MM-DD sein.")?;
    }
    let limit = limit(args.get("limit"), 100)?;
    let mut client = connect_read_only()?;
    let rows = client
        .query(
            r#"
            SELECT to_jsonb(row_data)::text
            FROM (
                SELECT patch_title, patch_date, entity_type, entity_name, ability_name,
                       stat_name, old_value, new_value, change_type, numeric_direction,
                       raw_line, patch_url, confidence
                FROM brain.patch_changes
                WHERE lower(entity_name) = lower($1)
                  AND ($2::text IS NULL OR ability_name ILIKE '%' || $2 || '%')
                  AND ($3::text IS NULL OR stat_name ILIKE '%' || $3 || '%')
                  AND ($4::text IS NULL OR patch_date >= $4::text::date)
                ORDER BY patch_date DESC, stat_name DESC
                LIMIT $5
            ) row_data
            ORDER BY patch_date, stat_name
            "#,
            &[&entity, &ability, &stat, &since, &limit],
        )
        .map_err(|_| anyhow!("Patch Historie konnte nicht gelesen werden."))?;
    rows_to_json(rows)
}

fn patch_search(args: &Value) -> Result<Value> {
    let text = required_text(args.get("text"), "text")?;
    let limit = limit(args.get("limit"), 50)?;
    let mut client = connect_read_only()?;
    let rows = client
        .query(
            r#"
            SELECT to_jsonb(row_data)::text
            FROM (
                SELECT patch_title, patch_date, entity_type, entity_name, ability_name,
                       stat_name, old_value, new_value, change_type, numeric_direction,
                       raw_line, patch_url, confidence
                FROM brain.patch_changes
                WHERE raw_line ILIKE '%' || $1 || '%'
                   OR entity_name ILIKE '%' || $1 || '%'
                ORDER BY patch_date DESC
                LIMIT $2
            ) row_data
            "#,
            &[&text, &limit],
        )
        .map_err(|_| anyhow!("Patch Suche konnte nicht ausgeführt werden."))?;
    rows_to_json(rows)
}

fn list_patches(args: &Value) -> Result<Value> {
    let limit = limit(args.get("limit"), 100)?;
    let mut client = connect_read_only()?;
    let rows = client
        .query(
            r#"
            SELECT jsonb_build_object(
                'patch_date', patch_date,
                'patch_title', patch_title
            )::text
            FROM (
                SELECT DISTINCT patch_date, patch_title
                FROM brain.patch_changes
                ORDER BY patch_date DESC
                LIMIT $1
            ) patches
            ORDER BY patch_date DESC
            "#,
            &[&limit],
        )
        .map_err(|_| anyhow!("Patch Liste konnte nicht gelesen werden."))?;
    rows_to_json(rows)
}

fn entity_summary(args: &Value) -> Result<Value> {
    let entity = required_text(args.get("entity"), "entity")?;
    let mut client = connect_read_only()?;
    let row = client
        .query_one(
            r#"
            SELECT jsonb_build_object(
                'entity_name', $1::text,
                'change_count', count(*)::int,
                'first_patch_date', min(patch_date),
                'last_patch_date', max(patch_date),
                'abilities', COALESCE(
                    array_agg(DISTINCT ability_name ORDER BY ability_name)
                        FILTER (WHERE ability_name IS NOT NULL),
                    ARRAY[]::text[]
                ),
                'stats', COALESCE(
                    array_agg(DISTINCT stat_name ORDER BY stat_name)
                        FILTER (WHERE stat_name IS NOT NULL),
                    ARRAY[]::text[]
                )
            )::text
            FROM brain.patch_changes
            WHERE lower(entity_name) = lower($1)
            "#,
            &[&entity],
        )
        .map_err(|_| anyhow!("Entity Zusammenfassung konnte nicht gelesen werden."))?;
    Ok(serde_json::from_str::<Value>(&row.get::<_, String>(0))?)
}

fn rows_to_json(rows: Vec<postgres::Row>) -> Result<Value> {
    let rows = rows
        .into_iter()
        .map(|row| serde_json::from_str::<Value>(&row.get::<_, String>(0)))
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(Value::Array(rows))
}

fn required_text(value: Option<&Value>, name: &str) -> Result<String> {
    let text = value
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| anyhow!("{name} darf nicht leer sein."))?;
    Ok(text.to_string())
}

fn optional_text(value: Option<&Value>) -> Option<String> {
    value
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn limit(value: Option<&Value>, default: i64) -> Result<i64> {
    let value = value.and_then(Value::as_i64).unwrap_or(default);
    ensure!((1..=500).contains(&value), "limit muss zwischen 1 und 500 liegen.");
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_contract_is_stable() {
        let names = tool_specs()
            .into_iter()
            .filter_map(|tool| tool.get("name").and_then(Value::as_str).map(str::to_string))
            .collect::<Vec<_>>();
        assert_eq!(
            names,
            vec![
                "patch_history_v1",
                "patch_history",
                "patch_search",
                "list_patches",
                "entity_summary"
            ]
        );
    }

    #[test]
    fn limits_and_dates_fail_closed() {
        assert!(limit(Some(&json!(0)), 10).is_err());
        assert!(limit(Some(&json!(501)), 10).is_err());
        assert!(NaiveDate::parse_from_str("2026-09-19", "%Y-%m-%d").is_ok());
        assert!(DateTime::parse_from_rfc3339("2026-09-19T00:00:00Z").is_ok());
    }

    #[test]
    fn initialize_and_tool_list_work_without_database() {
        let init = handle_request(&RpcRequest {
            id: Some(json!(1)),
            method: "initialize".to_string(),
            params: json!({"protocolVersion":"2025-06-18"}),
        })
        .unwrap();
        assert_eq!(init["serverInfo"]["name"], SERVER_NAME);

        let listed = handle_request(&RpcRequest {
            id: Some(json!(2)),
            method: "tools/list".to_string(),
            params: json!({}),
        })
        .unwrap();
        assert_eq!(listed["tools"].as_array().unwrap().len(), 5);
    }
}
