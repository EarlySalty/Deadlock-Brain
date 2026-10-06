use std::{
    env,
    io::{self, BufRead, Write},
};

use anyhow::{anyhow, ensure, Context, Result};
use chrono::NaiveDate;
use postgres::{Client, NoTls, Row};
use serde::Deserialize;
use serde_json::{json, Value};

const SERVER_NAME: &str = "dl-brain";
const SERVER_VERSION: &str = "0.1.0";
const PROTOCOL_VERSION: &str = "2025-06-18";
#[cfg(test)]
const PATCH_COLUMNS: &str = "patch_title, patch_date, entity_type, entity_name, ability_name, \
    stat_name, old_value, new_value, change_type, numeric_direction, raw_line, patch_url, confidence";
const PATCH_HISTORY_SQL: &str = "SELECT to_jsonb(row_data)::text FROM (SELECT patch_title, \
    patch_date, entity_type, entity_name, ability_name, stat_name, old_value, new_value, \
    change_type, numeric_direction, raw_line, patch_url, confidence FROM brain.patch_changes \
    WHERE lower(entity_name) = lower($1) AND ($2::text IS NULL OR ability_name ILIKE '%' || $2 || '%') \
    AND ($3::text IS NULL OR stat_name ILIKE '%' || $3 || '%') \
    AND ($4::text IS NULL OR patch_date >= $4::text::date) \
    ORDER BY patch_date DESC, stat_name DESC LIMIT $5) row_data \
    ORDER BY patch_date, stat_name";
const PATCH_SEARCH_SQL: &str = "SELECT to_jsonb(row_data)::text FROM (SELECT patch_title, \
    patch_date, entity_type, entity_name, ability_name, stat_name, old_value, new_value, \
    change_type, numeric_direction, raw_line, patch_url, confidence FROM brain.patch_changes \
    WHERE raw_line ILIKE '%' || $1 || '%' OR entity_name ILIKE '%' || $1 || '%' \
    ORDER BY patch_date DESC LIMIT $2) row_data";

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
                    &json!({
                        "jsonrpc":"2.0",
                        "id":Value::Null,
                        "error":{"code":-32700,"message":"Ungültige JSON-RPC-Nachricht."}
                    }),
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
                .unwrap_or(PROTOCOL_VERSION);
            Ok(json!({
                "protocolVersion":requested,
                "capabilities":{"tools":{"listChanged":false}},
                "serverInfo":{"name":SERVER_NAME,"version":SERVER_VERSION}
            }))
        }
        "ping" => Ok(json!({})),
        "tools/list" => Ok(json!({"tools":tool_specs()})),
        "tools/call" => call_tool(&request.params),
        _ => Err(anyhow!("MCP-Methode wird nicht unterstützt.")),
    }
}

fn tool_specs() -> Vec<Value> {
    vec![
        json!({
            "name":"patch_history",
            "description":"Liest die Patch-Historie einer Entity aus brain.patch_changes.",
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
            "description":"Sucht in Patch-Zeilen und Entity-Namen.",
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
                "properties":{"limit":{"type":"integer","minimum":1,"maximum":500,"default":100}},
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
        "patch_history" => patch_history(&arguments),
        "patch_search" => patch_search(&arguments),
        "list_patches" => list_patches(&arguments),
        "entity_summary" => entity_summary(&arguments),
        _ => Err(anyhow!("Unbekanntes MCP-Werkzeug.")),
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
        .map_err(|_| anyhow!("Datenbank-Verbindung ist nicht konfiguriert."))?;
    let mut client = Client::connect(&dsn, NoTls)
        .map_err(|_| anyhow!("Datenbank-Verbindung fehlgeschlagen."))?;
    client
        .batch_execute("SET statement_timeout='8s'; SET default_transaction_read_only=on;")
        .map_err(|_| anyhow!("Read-only-Datenbankmodus konnte nicht gesetzt werden."))?;
    Ok(client)
}

fn patch_history(args: &Value) -> Result<Value> {
    let entity = required_text(args.get("entity"), "entity")?;
    let ability = optional_text(args.get("ability"));
    let stat = optional_text(args.get("stat"));
    let since = optional_text(args.get("since"));
    if let Some(value) = &since {
        NaiveDate::parse_from_str(value, "%Y-%m-%d").context("since muss YYYY-MM-DD sein.")?;
    }
    let limit = limit(args.get("limit"), 100)?;
    let mut client = connect_read_only()?;
    let rows = client
        .query(
            PATCH_HISTORY_SQL,
            &[&entity, &ability, &stat, &since, &limit],
        )
        .map_err(|_| anyhow!("Patch-Historie konnte nicht gelesen werden."))?;
    rows_to_json(rows)
}

fn patch_search(args: &Value) -> Result<Value> {
    let text = required_text(args.get("text"), "text")?;
    let limit = limit(args.get("limit"), 50)?;
    let mut client = connect_read_only()?;
    let rows = client
        .query(PATCH_SEARCH_SQL, &[&text, &limit])
        .map_err(|_| anyhow!("Patch-Suche konnte nicht ausgeführt werden."))?;
    rows_to_json(rows)
}

fn list_patches(args: &Value) -> Result<Value> {
    let limit = limit(args.get("limit"), 100)?;
    let mut client = connect_read_only()?;
    let rows = client
        .query(
            "SELECT jsonb_build_object('patch_date', patch_date, 'patch_title', patch_title)::text \
             FROM (SELECT DISTINCT patch_date, patch_title FROM brain.patch_changes \
             ORDER BY patch_date DESC LIMIT $1) patches ORDER BY patch_date DESC",
            &[&limit],
        )
        .map_err(|_| anyhow!("Patch-Liste konnte nicht gelesen werden."))?;
    rows_to_json(rows)
}

fn entity_summary(args: &Value) -> Result<Value> {
    let entity = required_text(args.get("entity"), "entity")?;
    let mut client = connect_read_only()?;
    let row = client
        .query_one(
            "SELECT jsonb_build_object(\
                'entity_name', $1::text, \
                'change_count', count(*)::int, \
                'first_patch_date', min(patch_date), \
                'last_patch_date', max(patch_date), \
                'abilities', COALESCE(array_agg(DISTINCT ability_name ORDER BY ability_name) \
                    FILTER (WHERE ability_name IS NOT NULL), ARRAY[]::text[]), \
                'stats', COALESCE(array_agg(DISTINCT stat_name ORDER BY stat_name) \
                    FILTER (WHERE stat_name IS NOT NULL), ARRAY[]::text[]) \
             )::text FROM brain.patch_changes WHERE lower(entity_name) = lower($1)",
            &[&entity],
        )
        .map_err(|_| anyhow!("Entity-Zusammenfassung konnte nicht gelesen werden."))?;
    Ok(serde_json::from_str::<Value>(&row.get::<_, String>(0))?)
}

fn rows_to_json(rows: Vec<Row>) -> Result<Value> {
    let rows = rows
        .into_iter()
        .map(|row| serde_json::from_str::<Value>(&row.get::<_, String>(0)))
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(Value::Array(rows))
}

fn required_text(value: Option<&Value>, name: &str) -> Result<String> {
    value
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .ok_or_else(|| anyhow!("{name} darf nicht leer sein."))
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
    fn tool_contract_is_stable_and_read_only() {
        let tools = tool_specs();
        let names = tools
            .iter()
            .filter_map(|tool| tool.get("name").and_then(Value::as_str))
            .collect::<Vec<_>>();
        assert_eq!(
            names,
            vec!["patch_history", "patch_search", "list_patches", "entity_summary"]
        );
        assert!(tools
            .iter()
            .all(|tool| tool["annotations"]["readOnlyHint"] == true));
    }

    #[test]
    fn patch_history_keeps_exact_entity_match_and_ordering() {
        assert!(PATCH_HISTORY_SQL.contains("lower(entity_name) = lower($1)"));
        assert!(!PATCH_HISTORY_SQL.contains("entity_name ILIKE"));
        assert!(PATCH_HISTORY_SQL.contains("ORDER BY patch_date DESC, stat_name DESC"));
        assert!(PATCH_HISTORY_SQL.ends_with("ORDER BY patch_date, stat_name"));
        assert!(PATCH_COLUMNS.contains("entity_name"));
    }

    #[test]
    fn limits_and_dates_are_validated() {
        assert_eq!(limit(None, 100).unwrap(), 100);
        assert_eq!(limit(Some(&json!(500)), 100).unwrap(), 500);
        assert!(limit(Some(&json!(0)), 10).is_err());
        assert!(limit(Some(&json!(501)), 10).is_err());
        assert!(NaiveDate::parse_from_str("2026-09-19", "%Y-%m-%d").is_ok());
        assert!(NaiveDate::parse_from_str("2026-9-19", "%Y-%m-%d").is_err());
    }

    #[test]
    fn initialize_and_tool_list_work_without_database() {
        let init = handle_request(&RpcRequest {
            id: Some(json!(1)),
            method: "initialize".to_string(),
            params: json!({"protocolVersion":PROTOCOL_VERSION}),
        })
        .unwrap();
        assert_eq!(init["serverInfo"]["name"], SERVER_NAME);

        let listed = handle_request(&RpcRequest {
            id: Some(json!(2)),
            method: "tools/list".to_string(),
            params: json!({}),
        })
        .unwrap();
        assert_eq!(listed["tools"].as_array().unwrap().len(), 4);
    }

    #[test]
    fn stdio_ignores_notifications_and_returns_json_rpc_lines() {
        let input = format!(
            "{}\n{}\n",
            json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
            json!({"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}})
        );
        let mut output = Vec::new();
        serve(input.as_bytes(), &mut output).unwrap();
        let lines = String::from_utf8(output).unwrap();
        let response: Value = serde_json::from_str(lines.trim()).unwrap();
        assert_eq!(response["id"], 1);
        assert_eq!(response["result"]["tools"].as_array().unwrap().len(), 4);
    }
}
