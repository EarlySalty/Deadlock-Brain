use std::{path::Path, time::Duration};

use deadlock_brain_core::http::{HttpClient, HttpGetOptions};
use regex::Regex;
use rusqlite::Connection;
use serde_json::{json, Map, Value};

use crate::{
    store::{run_source, EntitySnapshotInput, SourceDocumentInput, SourceStore},
    Result,
};

pub const SOURCE: &str = "deadlock_stats_sheet";

#[derive(Debug, Clone)]
pub struct PullSheetOptions {
    pub sheet_id: String,
    pub gid: Option<String>,
    pub all_tabs: bool,
    pub cache_ttl_seconds: u64,
}

#[derive(Debug, Clone)]
pub struct RefreshSheetOptions {
    pub sheet_id: String,
    pub gid: String,
}

pub fn sheet_csv_url(sheet_id: &str, gid: &str) -> String {
    format!("https://docs.google.com/spreadsheets/d/{sheet_id}/export?format=csv&gid={gid}")
}

pub fn sheet_pubhtml_url(sheet_id: &str) -> String {
    format!("https://docs.google.com/spreadsheets/d/{sheet_id}/pubhtml")
}

pub fn pull_sheet(
    conn: &Connection,
    raw_dir: &Path,
    http: &HttpClient,
    options: PullSheetOptions,
) -> Result<Value> {
    run_source(conn, raw_dir, "sheet", |store| {
        pull_sheet_inner(store, http, &options)
    })
}

pub fn refresh_sheet(
    conn: &Connection,
    raw_dir: &Path,
    http: &HttpClient,
    options: RefreshSheetOptions,
) -> Result<Value> {
    run_source(conn, raw_dir, "refresh_sheet", |store| {
        let pull = pull_sheet_inner(
            store,
            http,
            &PullSheetOptions {
                sheet_id: options.sheet_id.clone(),
                gid: Some(options.gid.clone()),
                all_tabs: true,
                cache_ttl_seconds: 0,
            },
        )?;
        Ok(json!({ "pull": pull }))
    })
}

pub(crate) fn pull_sheet_inner(
    store: &SourceStore<'_>,
    http: &HttpClient,
    options: &PullSheetOptions,
) -> Result<Value> {
    let tabs = if options.all_tabs {
        discover_sheet_tabs(http, &options.sheet_id, options.cache_ttl_seconds)?
    } else {
        Vec::new()
    };
    let tabs = if tabs.is_empty() {
        let safe_gid = match options.gid.as_deref() {
            Some(value) => value.trim().to_string(),
            None => "0".to_string(),
        };
        vec![SheetTab {
            name: "sheet".to_string(),
            gid: safe_gid,
        }]
    } else {
        tabs
    };

    let mut summaries = Vec::new();
    for tab in &tabs {
        summaries.push(pull_single_sheet(
            store,
            http,
            &options.sheet_id,
            tab,
            options.cache_ttl_seconds,
        )?);
    }

    let rows: i64 = summaries
        .iter()
        .map(|summary| summary.get("rows").and_then(Value::as_i64).unwrap_or(0))
        .sum();
    let hero_snapshots: i64 = summaries
        .iter()
        .map(|summary| {
            summary
                .get("hero_snapshots")
                .and_then(Value::as_i64)
                .unwrap_or(0)
        })
        .sum();
    let item_snapshots: i64 = summaries
        .iter()
        .map(|summary| {
            summary
                .get("item_snapshots")
                .and_then(Value::as_i64)
                .unwrap_or(0)
        })
        .sum();
    let generic_row_snapshots: i64 = summaries
        .iter()
        .map(|summary| {
            summary
                .get("generic_row_snapshots")
                .and_then(Value::as_i64)
                .unwrap_or(0)
        })
        .sum();
    let tab_names = summaries
        .iter()
        .map(|summary| summary.get("sheet_name").cloned().unwrap_or(Value::Null))
        .collect::<Vec<_>>();

    Ok(json!({
        "sheet_id": options.sheet_id.clone(),
        "tabs": summaries.len(),
        "tab_names": tab_names,
        "rows": rows,
        "hero_snapshots": hero_snapshots,
        "item_snapshots": item_snapshots,
        "generic_row_snapshots": generic_row_snapshots,
        "summaries": summaries,
    }))
}

pub fn discover_sheet_tabs(
    http: &HttpClient,
    sheet_id: &str,
    cache_ttl_seconds: u64,
) -> Result<Vec<SheetTab>> {
    let url = sheet_pubhtml_url(sheet_id);
    let result = http.get(
        &url,
        HttpGetOptions {
            cache_ttl_seconds: Some(cache_ttl_seconds),
            timeout: Duration::from_secs(45),
            ..HttpGetOptions::default()
        },
    )?;
    let pattern = Regex::new(r#"items\.push\(\{name:\s*"((?:[^"\\]|\\.)*)".*?gid:\s*"(-?\d+)""#)?;
    let mut tabs = Vec::new();
    for captures in pattern.captures_iter(&result.text()) {
        let name = captures
            .get(1)
            .map(|value| decode_js_string(value.as_str()))
            .unwrap_or_default()
            .trim()
            .to_string();
        let gid = captures
            .get(2)
            .map(|value| value.as_str().trim().to_string())
            .unwrap_or_default();
        let tab = SheetTab { name, gid };
        if !tab.name.is_empty() && !tab.gid.is_empty() && !tabs.iter().any(|existing| existing == &tab) {
            tabs.push(tab);
        }
    }
    Ok(tabs)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SheetTab {
    pub name: String,
    pub gid: String,
}

fn pull_single_sheet(
    store: &SourceStore<'_>,
    http: &HttpClient,
    sheet_id: &str,
    tab: &SheetTab,
    cache_ttl_seconds: u64,
) -> Result<Value> {
    let gid = if tab.gid.is_empty() { "0" } else { &tab.gid };
    let sheet_name = if tab.name.is_empty() { gid } else { &tab.name };
    let url = sheet_csv_url(sheet_id, gid);
    let result = http.get(
        &url,
        HttpGetOptions {
            cache_ttl_seconds: Some(cache_ttl_seconds),
            timeout: Duration::from_secs(45),
            ..HttpGetOptions::default()
        },
    )?;
    let raw_path = store.write_raw(SOURCE, &format!("gid_{gid}"), &result.content, "csv")?;
    let external_id = format!("{sheet_id}:{gid}");
    let title = format!("Deadlock community stats sheet - {sheet_name}");
    let metadata = json!({
        "sheet_id": sheet_id,
        "gid": gid,
        "sheet_name": sheet_name,
        "from_cache": result.from_cache,
    });
    let document_id = store.upsert_source_document(SourceDocumentInput {
        source: SOURCE,
        external_id: &external_id,
        title: Some(&title),
        url: Some(&url),
        content_type: "text/csv",
        raw_path: &raw_path,
        content: &result.content,
        metadata: &metadata,
    })?;

    let rows = parse_csv_rows(&result.text());
    let header_index = find_header_row(&rows);
    let mut snapshots = Vec::new();
    let mut generic_snapshots = 0usize;
    let mut item_snapshots = 0usize;

    if let Some(header_index) = header_index {
        let headers = rows[header_index]
            .iter()
            .map(|header| header.trim().to_string())
            .collect::<Vec<_>>();
        for (offset, row) in rows[(header_index + 1)..].iter().enumerate() {
            let idx = header_index + 2 + offset;
            let record = row_to_record(&headers, row);
            let hero_name = hero_name_from_record(&record);
            let item_name = item_name_from_record(&record);
            let payload = json!({
                "row_number": idx,
                "sheet_name": sheet_name,
                "gid": gid,
                "values": record,
            });
            let canonical_name = hero_name
                .as_ref()
                .filter(|value| !value.is_empty())
                .cloned()
                .or_else(|| item_name.as_ref().filter(|value| !value.is_empty()).cloned())
                .unwrap_or_else(|| format!("{sheet_name} row {idx}"));
            snapshots.push(EntitySnapshotInput {
                source: SOURCE.to_string(),
                entity_type: "stats_sheet_row".to_string(),
                external_id: format!("{gid}:{idx}"),
                canonical_name: Some(canonical_name),
                payload: payload.clone(),
            });
            generic_snapshots += 1;
            if let Some(hero_name) = hero_name.filter(|value| !value.is_empty()) {
                snapshots.push(EntitySnapshotInput {
                    source: SOURCE.to_string(),
                    entity_type: "hero_stats_sheet".to_string(),
                    external_id: format!("{}:{}", gid, hero_name.to_lowercase().replace(' ', "_")),
                    canonical_name: Some(hero_name),
                    payload: payload.clone(),
                });
            }
            if let Some(item_name) = item_name.filter(|value| !value.is_empty()) {
                snapshots.push(EntitySnapshotInput {
                    source: SOURCE.to_string(),
                    entity_type: "item_stats_sheet".to_string(),
                    external_id: format!("{}:{}", gid, item_name.to_lowercase().replace(' ', "_")),
                    canonical_name: Some(item_name),
                    payload: payload.clone(),
                });
                item_snapshots += 1;
            }
        }
    }

    let count = store.insert_many_snapshots(&snapshots, Some(document_id))?;
    let hero_snapshot_count = snapshots
        .iter()
        .filter(|snapshot| snapshot.entity_type == "hero_stats_sheet")
        .count();
    Ok(json!({
        "url": url,
        "sheet_name": sheet_name,
        "gid": gid,
        "rows": rows.len(),
        "header_row": header_index.map(|index| index + 1),
        "snapshots": count,
        "hero_snapshots": hero_snapshot_count,
        "item_snapshots": item_snapshots,
        "generic_row_snapshots": generic_snapshots,
    }))
}

fn parse_csv_rows(text: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut field = String::new();
    let mut in_quotes = false;
    let mut field_started = false;
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if in_quotes {
            if ch == '"' {
                if chars.peek().copied() == Some('"') {
                    chars.next();
                    field.push('"');
                } else {
                    in_quotes = false;
                }
            } else {
                field.push(ch);
            }
            continue;
        }

        match ch {
            '"' if !field_started => {
                in_quotes = true;
                field_started = true;
            }
            ',' => {
                row.push(std::mem::take(&mut field));
                field_started = false;
            }
            '\n' => {
                finish_csv_row(&mut rows, &mut row, &mut field, field_started);
                field_started = false;
            }
            '\r' => {
                if chars.peek().copied() == Some('\n') {
                    chars.next();
                }
                finish_csv_row(&mut rows, &mut row, &mut field, field_started);
                field_started = false;
            }
            _ => {
                field.push(ch);
                field_started = true;
            }
        }
    }

    if field_started || !field.is_empty() || !row.is_empty() {
        row.push(field);
        rows.push(row);
    }
    rows
}

fn finish_csv_row(
    rows: &mut Vec<Vec<String>>,
    row: &mut Vec<String>,
    field: &mut String,
    field_started: bool,
) {
    if row.is_empty() && field.is_empty() && !field_started {
        rows.push(Vec::new());
    } else {
        row.push(std::mem::take(field));
        rows.push(std::mem::take(row));
    }
}

fn find_header_row(rows: &[Vec<String>]) -> Option<usize> {
    for (idx, row) in rows.iter().enumerate() {
        let normalized = row
            .iter()
            .map(|cell| cell.trim().to_lowercase())
            .collect::<Vec<_>>();
        let joined = normalized.join(" ");
        if normalized.iter().any(|cell| cell == "hero name")
            || joined.contains("hero name")
            || normalized.iter().any(|cell| cell == "hero")
            || normalized.iter().any(|cell| cell == "name")
            || normalized.iter().any(|cell| cell == "game name")
            || normalized.iter().any(|cell| cell == "code name")
            || normalized.iter().any(|cell| cell == "souls")
            || normalized.iter().any(|cell| cell == "weapon")
        {
            return Some(idx);
        }
    }

    for (idx, row) in rows.iter().enumerate() {
        if row.iter().any(|cell| !cell.trim().is_empty()) {
            return Some(idx);
        }
    }
    None
}

fn row_to_record(headers: &[String], row: &[String]) -> Map<String, Value> {
    let mut record = Map::new();
    let mut used = Vec::<String>::new();
    for (idx, header) in headers.iter().enumerate() {
        let mut key = if header.trim().is_empty() {
            format!("Column {}", idx + 1)
        } else {
            header.trim().to_string()
        };
        if used.iter().any(|existing| existing == &key) {
            let mut suffix = 2usize;
            while used.iter().any(|existing| existing == &format!("{key} {suffix}")) {
                suffix += 1;
            }
            key = format!("{key} {suffix}");
        }
        used.push(key.clone());
        let value = row
            .get(idx)
            .map(|cell| cell.trim().to_string())
            .unwrap_or_default();
        record.insert(key, Value::String(value));
    }
    record
}

fn hero_name_from_record(record: &Map<String, Value>) -> Option<String> {
    if record
        .get("disabled")
        .and_then(Value::as_str)
        .map(|value| matches!(value.trim().to_lowercase().as_str(), "true" | "1" | "yes"))
        .unwrap_or(false)
    {
        return None;
    }

    for key in ["Hero Name", "hero name", "name"] {
        let value = record
            .get(key)
            .and_then(Value::as_str)
            .unwrap_or_default()
            .trim();
        if !value.is_empty() && looks_like_name(value) {
            return Some(value.to_string());
        }
    }
    None
}

fn item_name_from_record(record: &Map<String, Value>) -> Option<String> {
    for key in ["game name", "Game Name", "Item", "item", "code name", "Code Name"] {
        let value = record
            .get(key)
            .and_then(Value::as_str)
            .unwrap_or_default()
            .trim();
        if !value.is_empty() && looks_like_name(value) {
            return Some(value.to_string());
        }
    }
    None
}

fn looks_like_name(value: &str) -> bool {
    let lowered = value.to_lowercase();
    if matches!(
        lowered.as_str(),
        "true" | "false" | "name" | "hero" | "game name" | "code name"
    ) {
        return false;
    }
    if lowered.contains("tiermaker") {
        return false;
    }
    let mut chars = value.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '(' {
            if matches!(chars.peek(), Some('<' | '>')) {
                chars.next();
            }
            if matches!(chars.peek(), Some(next) if next.is_ascii_digit()) {
                return false;
            }
        }
    }
    value.chars().any(char::is_alphabetic)
}

fn decode_js_string(value: &str) -> String {
    serde_json::from_str::<String>(&format!("\"{value}\""))
        .unwrap_or_else(|_| value.replace("\\/", "/"))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use deadlock_brain_core::{http::HttpClient, schema};

    use super::*;
    use crate::store::stable_hash_bytes;

    #[test]
    fn pull_sheet_reads_cached_csv_and_creates_generic_hero_and_item_snapshots() {
        let temp = tempfile::tempdir().expect("tempdir");
        let raw_dir = temp.path().join("raw");
        let cache_dir = temp.path().join("cache");
        let conn = Connection::open_in_memory().expect("open sqlite");
        schema::ensure_schema(&conn).expect("schema");
        let http = HttpClient::new("test-agent", &cache_dir).expect("http");
        let url = sheet_csv_url("sheet123", "0");
        write_http_cache(
            &cache_dir,
            &url,
            b"Hero Name,Game Name,disabled\nInfernus,,false\n,Healing Rite,\n",
            "text/csv",
        );

        let summary = pull_sheet(
            &conn,
            &raw_dir,
            &http,
            PullSheetOptions {
                sheet_id: "sheet123".to_string(),
                gid: Some("0".to_string()),
                all_tabs: false,
                cache_ttl_seconds: 1800,
            },
        )
        .expect("pull sheet");

        assert_eq!(summary["rows"], json!(3));
        assert_eq!(summary["hero_snapshots"], json!(1));
        assert_eq!(summary["item_snapshots"], json!(1));
        let snapshots: i64 = conn
            .query_row("SELECT COUNT(*) FROM entity_snapshots", [], |row| row.get(0))
            .expect("snapshot count");
        assert_eq!(snapshots, 4);
    }

    #[test]
    fn discover_sheet_tabs_decodes_pubhtml_items() {
        let temp = tempfile::tempdir().expect("tempdir");
        let cache_dir = temp.path().join("cache");
        let http = HttpClient::new("test-agent", &cache_dir).expect("http");
        let url = sheet_pubhtml_url("sheet123");
        write_http_cache(
            &cache_dir,
            &url,
            br#"items.push({name: "Heroes\/Stats", foo: "bar", gid: "42"});"#,
            "text/html",
        );

        let tabs = discover_sheet_tabs(&http, "sheet123", 1800).expect("tabs");

        assert_eq!(
            tabs,
            vec![SheetTab {
                name: "Heroes/Stats".to_string(),
                gid: "42".to_string()
            }]
        );
    }

    fn write_http_cache(cache_dir: &Path, url: &str, content: &[u8], content_type: &str) {
        fs::create_dir_all(cache_dir).expect("cache dir");
        let path = cache_dir.join(format!("{}.bin", stable_hash_bytes(url.as_bytes())));
        fs::write(&path, content).expect("cache body");
        fs::write(
            path.with_extension("bin.json"),
            format!(r#"{{"url":"fixture","content_type":"{content_type}","fetched_at":0}}"#),
        )
        .expect("cache metadata");
    }
}
