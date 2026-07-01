use std::collections::HashMap;
use std::env;
use std::time::Duration;

use anyhow::{anyhow, Result};
use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use deadlock_brain_core::http::{HttpClient, HttpGetOptions};
use postgres::types::ToSql;
use postgres::{Client, NoTls, Transaction};
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const SOURCE: &str = "deadlock_patchnotes_db";
const IMPORTER: &str = "deadlock_patchnotes_db_pg";
const STEAM_APPID: u32 = 1_422_450;
const STEAM_APPNEWS_API: &str = "https://api.steampowered.com/ISteamNews/GetNewsForApp/v2/";
const STEAM_LOOKBACK_COUNT: u32 = 500;

#[derive(Debug, Clone)]
pub struct ImportPatchnoteOptions {
    pub patch_id: i64,
    pub dsn_env: String,
    pub dry_run: bool,
}

#[derive(Debug, Clone)]
struct ResolvedPatchSource {
    raw_content: String,
    source_url: Option<String>,
    source_kind: String,
    resolved_from: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct SteamAppNewsResponse {
    appnews: SteamAppNews
}

#[derive(Debug, Clone, Deserialize)]
struct SteamAppNews {
    #[serde(default)]
    newsitems: Vec<SteamAppNewsItem>,
}

#[derive(Debug, Clone, Deserialize)]
struct SteamAppNewsItem {
    gid: String,
    title: String,
    url: String,
    #[serde(default)]
    contents: String,
    date: i64,
    #[serde(default)]
    author: Option<String>,
    #[serde(default)]
    feedlabel: Option<String>,
    #[serde(default)]
    feedname: Option<String>,
}

#[derive(Debug, Clone)]
struct PatchnoteRow {
    id: i64,
    title: Option<String>,
    url: Option<String>,
    posted_at: Option<String>,
    raw_content: Option<String>,
    translated_content: Option<String>,
}

#[derive(Debug, Clone)]
struct PreparedPatch {
    row_id: i64,
    patch_external_id: String,
    source_external_id: String,
    title: Option<String>,
    url: Option<String>,
    posted_at: Option<DateTime<Utc>>,
    source_kind: String,
    raw_payload_text: String,
    raw_payload_hash: String,
    snapshot_payload_text: String,
    snapshot_payload_hash: String,
    events: Vec<PreparedEvent>,
}

#[derive(Debug, Clone)]
struct PreparedEvent {
    line_index: i64,
    section: Option<String>,
    entity_type: String,
    entity_name: Option<String>,
    subject: Option<String>,
    change_type: String,
    raw_line: String,
    normalized_line: String,
    old_value: Option<String>,
    new_value: Option<String>,
    confidence: f64,
    metadata: Value,
    event_hash: String,
}

#[derive(Debug, Clone, Copy, Default)]
struct PgWriteCounts {
    documents: u64,
    snapshots: u64,
    patch_events: u64,
    knowledge_events: u64,
}

#[derive(Debug, Default, Clone)]
struct EntityIndex {
    by_key: HashMap<String, EntityAlias>,
    aliases_desc: Vec<EntityAlias>,
}

#[derive(Debug, Clone)]
struct EntityAlias {
    key: String,
    entity_type: String,
    canonical_name: String,
}

#[derive(Debug)]
struct EventParseContext<'a> {
    row: &'a PatchnoteRow,
    source_url: Option<&'a str>,
    posted_at: Option<DateTime<Utc>>,
    section: Option<&'a str>,
    line_index: i64,
    raw_line: &'a str,
    body: &'a str,
    source_kind: &'a str,
    index: &'a EntityIndex,
}

impl EntityIndex {
    fn insert(&mut self, entity_type: &str, canonical_name: &str, alias: &str) {
        let key = normalize_key(alias);
        if key.len() < 3 || !key.chars().any(|ch| ch.is_ascii_alphabetic()) {
            return;
        }
        let alias = EntityAlias {
            key: key.clone(),
            entity_type: public_entity_type(entity_type).to_string(),
            canonical_name: canonical_name.to_string(),
        };
        self.by_key.entry(key).or_insert(alias);
    }

    fn finish(mut self) -> Self {
        self.aliases_desc = self.by_key.values().cloned().collect();
        self.aliases_desc.sort_by(|left, right| {
            right
                .key
                .len()
                .cmp(&left.key.len())
                .then(left.key.cmp(&right.key))
        });
        self
    }

    fn exact(&self, value: &str) -> Option<&EntityAlias> {
        self.by_key.get(&normalize_key(value))
    }

    fn infer(&self, value: &str) -> Option<&EntityAlias> {
        let haystack = format!(" {} ", normalize_key(value));
        self.aliases_desc
            .iter()
            .find(|alias| contains_alias(&haystack, &alias.key))
    }
}

pub fn import_patchnote(http: &HttpClient, options: &ImportPatchnoteOptions) -> Result<Value> {
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
    let patch = load_patchnote(&mut client, options.patch_id)?;
    let index = load_entity_index(&mut client)?;
    let resolved = resolve_patch_source(http, &patch)?;
    let prepared = prepare_patch(&patch, &resolved, &index)?;
    if options.dry_run {
        return Ok(summary_json(
            true,
            options,
            &prepared,
            PgWriteCounts::default(),
        ));
    }
    let mut tx = client.transaction()?;
    let run_id = begin_run(&mut tx)?;
    let document_id = upsert_source_document(&mut tx, &prepared)?;
    let snapshot_id = upsert_entity_snapshot(&mut tx, &prepared, document_id)?;
    prune_direct_patch_events(&mut tx, &prepared.patch_external_id)?;
    let patch_events = insert_patch_events(&mut tx, &prepared, snapshot_id)?;
    let knowledge_events =
        materialize_patch_knowledge_events(&mut tx, &prepared.patch_external_id)?;
    let counts = PgWriteCounts {
        documents: 1,
        snapshots: 1,
        patch_events,
        knowledge_events,
    };
    finish_run(
        &mut tx,
        run_id,
        &summary_json(false, options, &prepared, counts),
    )?;
    tx.commit()?;
    Ok(summary_json(false, options, &prepared, counts))
}

fn load_patchnote(client: &mut Client, patch_id: i64) -> Result<PatchnoteRow> {
    let row = client
        .query_opt(
            r#"
            SELECT id, title, url, posted_at::text, raw_content, translated_content
            FROM patchnotes.changelog_posts
            WHERE id = $1
            "#,
            &[&patch_id],
        )
        .map_err(|error| anyhow!("Fehler beim Lesen von changelog_posts: {error}"))?;
    let row = row.ok_or_else(|| anyhow!("Patchnote nicht gefunden: {patch_id}"))?;
    Ok(PatchnoteRow {
        id: row.get(0),
        title: row.get(1),
        url: row.get(2),
        posted_at: row.get(3),
        raw_content: row.get(4),
        translated_content: row.get(5),
    })
}

fn resolve_patch_source(http: &HttpClient, row: &PatchnoteRow) -> Result<PatchSourceResolution> {
    let posted_at = parse_posted_at(row.posted_at.as_deref())?;
    let candidates = collect_steam_links(row);
    let source_kind = classify_source_kind(row.url.as_deref());
    let should_search_steam = source_kind == "forum" || !candidates.is_empty();
    if !should_search_steam {
        return Ok(PatchSourceResolution::from_row(row));
    }

    let items = match fetch_steam_news_items(http) {
        Ok(items) => items,
        Err(_) => {
            return Ok(PatchSourceResolution::from_row(row));
        }
    };
    let item = match_steam_news_item(row, posted_at, &candidates, &items)
        .or_else(|| find_matching_official_news_item(row, posted_at, &items));

    let Some(item) = item else {
        return Ok(PatchSourceResolution::from_row(row));
    };

    Ok(PatchSourceResolution {
        raw_content: clean_steam_content(&item.contents),
        source_url: Some(item.url),
        source_kind: "steam".to_string(),
        resolved_from: Some(format!("steam_gid:{}", item.gid)),
    })
}

fn fetch_steam_news_items(http: &HttpClient) -> Result<Vec<SteamAppNewsItem>> {
    let url = appnews_url(STEAM_LOOKBACK_COUNT);
    let response: SteamAppNewsResponse = http.get_json(
        &url,
        HttpGetOptions {
            cache_ttl_seconds: Some(900),
            timeout: Duration::from_secs(45),
            ..HttpGetOptions::default()
        },
    )?;
    Ok(response.appnews.newsitems)
}

fn appnews_url(count: u32) -> String {
    format!(
        "{STEAM_APPNEWS_API}?appid={STEAM_APPID}&count={count}&maxlength=100000&format=json"
    )
}

#[derive(Debug, Clone)]
struct PatchSourceResolution {
    raw_content: String,
    source_url: Option<String>,
    source_kind: String,
    resolved_from: Option<String>,
}

impl PatchSourceResolution {
    fn from_row(row: &PatchnoteRow) -> Self {
        Self {
            raw_content: row.raw_content.clone().unwrap_or_default(),
            source_url: row.url.clone(),
            source_kind: classify_source_kind(row.url.as_deref()),
            resolved_from: None,
        }
    }
}

fn prepare_patch(
    row: &PatchnoteRow,
    resolved: &PatchSourceResolution,
    index: &EntityIndex,
) -> Result<PreparedPatch> {
    let posted_at = parse_posted_at(row.posted_at.as_deref())?;
    let posted_at_text = posted_at.map(|value| value.to_rfc3339());

    let source_external_id = match resolved.source_url.as_deref().map(str::trim) {
        Some(url) if !url.is_empty() => url.to_string(),
        _ => format!("patchnotes:{}", row.id),
    };
    let patch_external_id = row.id.to_string();
    let source_kind = resolved.source_kind.clone();
    let title = Some(
        row.title
            .clone()
            .unwrap_or_else(|| format!("Patchnotes {}", row.id)),
    );
    let url = resolved.source_url.clone().or_else(|| row.url.clone());
    let canonical_source_url = url.as_deref();
    let resolved_from = resolved.resolved_from.clone();
    let content = resolved.raw_content.clone();
    let payload = json!({
        "id": row.id,
        "title": row.title,
        "url": url,
        "changelog_url": row.url,
        "posted_at": posted_at_text,
        "raw_content": content,
        "translated_content": row.translated_content,
        "source_kind": source_kind,
        "source_url": canonical_source_url,
        "resolved_from": resolved_from,
        "importer": IMPORTER,
    });
    let raw_payload_text = json_text(&payload)?;
    let raw_payload_hash = stable_hash(raw_payload_text.as_bytes());
    let snapshot_payload = json!({
        "id": row.id,
        "title": row.title,
        "url": url,
        "changelog_url": row.url,
        "posted_at": posted_at_text,
        "raw_content": content,
        "translated_content": row.translated_content,
        "source_kind": source_kind,
        "source_url": canonical_source_url,
        "patchnotes": true,
        "importer": IMPORTER,
    });
    let snapshot_payload_text = json_text(&snapshot_payload)?;
    let snapshot_payload_hash = stable_hash(snapshot_payload_text.as_bytes());
    let events = parse_events(
        row,
        canonical_source_url,
        &source_kind,
        posted_at,
        &content,
        index,
    );
    Ok(PreparedPatch {
        row_id: row.id,
        patch_external_id,
        source_external_id,
        title,
        url,
        posted_at,
        source_kind,
        raw_payload_text,
        raw_payload_hash,
        snapshot_payload_text,
        snapshot_payload_hash,
        events,
    })
}

#[derive(Debug, Clone)]
struct SteamLinkCandidate {
    source_url: String,
    gid: Option<String>,
}

fn collect_steam_links(row: &PatchnoteRow) -> Vec<SteamLinkCandidate> {
    let mut candidates = Vec::new();
    let mut seen = HashMap::<String, bool>::new();
    let mut add_candidate = |raw_url: &str| {
        if !is_steam_news_url(raw_url) {
            return;
        }
        let normalized = normalize_url_token(raw_url);
        if seen.insert(normalized.clone(), true).is_some() {
            return;
        }
        let gid = extract_steam_gid(&normalized);
        candidates.push(SteamLinkCandidate {
            source_url: normalized,
            gid,
        });
    };

    for text in row.url.iter().chain(row.raw_content.iter()).chain(row.translated_content.iter()) {
        for raw in extract_http_tokens(text) {
            add_candidate(&raw);
        }
    }

    candidates
}

fn extract_http_tokens(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let bytes = text.as_bytes();
    let mut index = 0usize;

    while index + 4 <= bytes.len() {
        if bytes[index..].starts_with(b"http://") || bytes[index..].starts_with(b"https://") {
            let start = index;
            index += 1;
            while index < bytes.len() {
                if bytes[index].is_ascii_whitespace() {
                    break;
                }
                index += 1;
            }
            out.push(String::from_utf8_lossy(&bytes[start..index]).into_owned());
            continue;
        }
        index += 1;
    }
    out
}

fn normalize_url_token(raw: &str) -> String {
    raw.trim()
        .trim_start_matches('(')
        .trim_start_matches('<')
        .trim_start_matches('"')
        .trim_end_matches(&[')', '>', ']', '"', '\'', ';', ',', '.'][..])
        .to_string()
}

fn is_steam_news_url(url: &str) -> bool {
    let lower = url.to_lowercase();
    (lower.contains("steamcommunity.com") || lower.contains("steampowered.com") || lower.contains("steamstore-a.akamaihd.net"))
        && (lower.contains("externalpost")
            || lower.contains("announcements")
            || lower.contains("steam_community_announcements"))
}

fn extract_steam_gid(url: &str) -> Option<String> {
    for marker in [
        "externalpost/steam_community_announcements/",
        "/announcements/detail/",
    ] {
        let lower = url.to_lowercase();
        if let Some(offset) = lower.find(marker) {
            let digits = lower[offset + marker.len()..]
                .chars()
                .take_while(|ch| ch.is_ascii_digit())
                .collect::<String>();
            if !digits.is_empty() {
                return Some(digits);
            }
        }
    }
    None
}

fn match_steam_news_item(
    row: &PatchnoteRow,
    posted_at: Option<DateTime<Utc>>,
    candidates: &[SteamLinkCandidate],
    items: &[SteamAppNewsItem],
) -> Option<SteamAppNewsItem> {
    for candidate in candidates {
        if let Some(gid) = &candidate.gid {
            if let Some(item) = items.iter().find(|item| &item.gid == gid) {
                return Some(item.clone());
            }
        }
    }

    let wanted_urls = candidates
        .iter()
        .map(|candidate| normalize_steam_url(&candidate.source_url))
        .collect::<Vec<_>>();
    for item in items {
        let item_url = normalize_steam_url(&item.url);
        if wanted_urls.iter().any(|wanted| item_url.contains(wanted.as_str())) {
            return Some(item.clone());
        }
    }

    if let Some(posted_at) = posted_at {
        let wanted_rows = normalize_steam_title(row.title.as_deref());
        let mut best: Option<(i64, SteamAppNewsItem)> = None;
        for item in items {
            if !is_official_steam_item(item) {
                continue;
            }
            let Some(item_time) = steam_item_time(item) else {
                continue;
            };
            let mut score = -(item_time.timestamp() - posted_at.timestamp()).abs();
            if score < -100_000 {
                continue;
            }
            let title_score = title_similarity(&wanted_rows, &normalize_steam_title(Some(&item.title)));
            score += title_score as i64;
            if score > best.as_ref().map_or(i64::MIN, |value| value.0) {
                best = Some((score, item.clone()));
            }
        }
        if let Some((_, item)) = best {
            return Some(item);
        }
    }

    None
}

fn find_matching_official_news_item(
    row: &PatchnoteRow,
    posted_at: Option<DateTime<Utc>>,
    items: &[SteamAppNewsItem],
) -> Option<SteamAppNewsItem> {
    let title = normalize_steam_title(row.title.as_deref());
    let mut best: Option<(i64, SteamAppNewsItem)> = None;
    let max_date_diff_seconds: i64 = 60_i64 * 60_i64 * 24_i64 * 14;
    for item in items {
        if !is_official_steam_item(item) {
            continue;
        }
        let mut score = 0_i64;
        if let Some(posted_at) = posted_at {
            let Some(item_time) = steam_item_time(item) else {
                continue;
            };
            let diff = (item_time.timestamp() - posted_at.timestamp()).abs();
            if diff > max_date_diff_seconds {
                continue;
            }
            score += (max_date_diff_seconds - diff) / 300;
        }
        score += title_similarity(&title, &normalize_steam_title(Some(&item.title))) as i64;
        if score > best.as_ref().map_or(i64::MIN, |best| best.0) {
            best = Some((score, item.clone()));
        }
    }
    best.map(|(_, item)| item)
}

fn is_official_steam_item(item: &SteamAppNewsItem) -> bool {
    item.feedname.as_deref() == Some("steam_community_announcements")
        || item.url.contains("steam_community_announcements")
}

fn steam_item_time(item: &SteamAppNewsItem) -> Option<DateTime<Utc>> {
    DateTime::<Utc>::from_timestamp(item.date, 0)
}

fn title_similarity(left: &str, right: &str) -> usize {
    if left.is_empty() || right.is_empty() {
        return 0;
    }
    let left_words = left.split_ascii_whitespace().collect::<Vec<_>>();
    let mut count = 0usize;
    for left_word in left_words {
        if right.contains(left_word) {
            count += 1;
        }
    }
    count
}

fn normalize_steam_url(url: &str) -> String {
    let mut lowered = url.to_lowercase();
    if let Some(pos) = lowered.find('?') {
        lowered = lowered[..pos].to_string();
    }
    lowered
        .trim_end_matches('/')
        .trim_end_matches(".html")
        .to_string()
}

fn clean_steam_content(raw: &str) -> String {
    let mut text = raw.replace("\r\n", "\n").replace('\r', "\n");
    for (from, to) in [
        ("\\-", "\n- "),
        ("[list]", "\n"),
        ("[/list]", "\n"),
        ("[*]", "\n- "),
        ("[/*]", "\n"),
        ("<br>", "\n"),
        ("<br/>", "\n"),
        ("<br />", "\n"),
        ("</p>", "\n"),
    ] {
        text = text.replace(from, to);
    }
    for tag in [
        "[b]",
        "[/b]",
        "[i]",
        "[/i]",
        "[u]",
        "[/u]",
        "[h1]",
        "[/h1]",
        "[h2]",
        "[/h2]",
        "[h3]",
        "[/h3]",
        "[code]",
        "[/code]",
    ] {
        text = text.replace(tag, "");
    }
    text = strip_bbcode_with_value(&text);
    text = strip_html_tags(&text);
    text = decode_basic_entities(&text);
    text = separate_inline_dash_bullets(&text);
    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn strip_bbcode_with_value(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    let mut chars = value.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '[' {
            let mut tag = String::new();
            while let Some(next) = chars.peek().copied() {
                chars.next();
                if next == ']' {
                    break;
                }
                tag.push(next);
            }
            let lower = tag.to_ascii_lowercase();
            if lower.starts_with("url=")
                || lower.starts_with("/url")
                || lower.starts_with("img")
                || lower.starts_with("/img")
            {
                continue;
            }
            output.push('[');
            output.push_str(&tag);
            output.push(']');
        } else {
            output.push(ch);
        }
    }
    output
}

fn strip_html_tags(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    let mut in_tag = false;
    for ch in value.chars() {
        match ch {
            '<' => in_tag = true,
            '>' if in_tag => in_tag = false,
            _ if !in_tag => output.push(ch),
            _ => {}
        }
    }
    output
}

fn decode_basic_entities(value: &str) -> String {
    value
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&apos;", "'")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
}

fn separate_inline_dash_bullets(value: &str) -> String {
    let chars = value.chars().collect::<Vec<_>>();
    let mut output = String::with_capacity(value.len());
    let mut index = 0;
    while index < chars.len() {
        let ch = chars[index];
        if ch == '-'
            && chars.get(index + 1) == Some(&' ')
            && index > 0
            && chars[index - 1] != '\n'
            && chars.get(index + 2).is_some_and(|next| {
                next.is_ascii_uppercase() || next.is_ascii_digit() || *next == '"'
            })
        {
            output.push('\n');
        }
        output.push(ch);
        index += 1;
    }
    output
}

fn normalize_steam_title(title: Option<&str>) -> String {
    normalize_key(title.unwrap_or_default())
}

fn parse_events(
    row: &PatchnoteRow,
    source_url: Option<&str>,
    source_kind: &str,
    posted_at: Option<DateTime<Utc>>,
    content: &str,
    index: &EntityIndex,
) -> Vec<PreparedEvent> {
    let mut events = Vec::new();
    let mut section: Option<String> = None;
    let mut line_index = 0_i64;
    for line in patch_lines(content) {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(body) = bullet_body(trimmed) {
            line_index += 1;
            let event = parse_bullet_event(&EventParseContext {
                row,
                source_url,
                posted_at,
                section: section.as_deref(),
                line_index,
                raw_line: trimmed,
                body: &body,
                source_kind,
                index,
            });
            if let Some(event) = event {
                events.push(event);
            }
            continue;
        }
        if let Some(next_section) = section_heading(trimmed) {
            section = Some(next_section);
        }
    }
    events
}

fn parse_bullet_event(context: &EventParseContext<'_>) -> Option<PreparedEvent> {
    let normalized_line = normalize_patch_line(context.body);
    if normalized_line.is_empty() {
        return None;
    }
    let (subject, remainder) = split_subject(&normalized_line);
    let entity = subject
        .as_deref()
        .and_then(|subject| context.index.exact(subject))
        .or_else(|| context.index.infer(&normalized_line));
    let (entity_type, entity_name, confidence) = if let Some(entity) = entity {
        (
            entity.entity_type.clone(),
            Some(entity.canonical_name.clone()),
            if subject.is_some() { 0.88 } else { 0.72 },
        )
    } else {
        ("general".to_string(), None, 0.52)
    };
    let (old_value, new_value) = extract_old_new(&normalized_line);
    let change_type = classify_change_type(&normalized_line);
    let metadata = json!({
        "importer": IMPORTER,
        "patch_id": context.row.id,
        "source_kind": context.source_kind,
        "source_language": "en",
        "source_posted_at": context.posted_at.map(|value| value.to_rfc3339()),
        "line_subject": subject,
        "line_remainder": remainder,
        "section": context.section,
        "patch_title": context.row.title,
        "url": context.source_url.or(context.row.url.as_deref()),
        "changelog_url": context.row.url,
    });
    let event_hash = stable_hash(
        format!(
            "patchnotes|{}|{}|{}|{}|{}",
            context.row.id,
            context.line_index,
            context.section.unwrap_or_default(),
            entity_type,
            normalized_line
        )
        .as_bytes(),
    );

    Some(PreparedEvent {
        line_index: context.line_index,
        section: context.section.map(ToString::to_string),
        entity_type,
        entity_name,
        subject,
        change_type,
        raw_line: context.raw_line.to_string(),
        normalized_line,
        old_value,
        new_value,
        confidence,
        metadata,
        event_hash,
    })
}

fn parse_posted_at(raw: Option<&str>) -> Result<Option<DateTime<Utc>>> {
    let raw = raw.unwrap_or("").trim();
    if raw.is_empty() {
        return Ok(None);
    }
    let mut normalized = raw.to_string();
    if normalized.ends_with("+00") || normalized.ends_with("-00") {
        normalized.push_str(":00");
    }
    if let Ok(value) = DateTime::parse_from_rfc3339(raw) {
        return Ok(Some(value.with_timezone(&Utc)));
    }
    if let Ok(value) = DateTime::parse_from_rfc3339(&normalized) {
        return Ok(Some(value.with_timezone(&Utc)));
    }
    if let Ok(value) = DateTime::parse_from_str(raw, "%Y-%m-%d %H:%M:%S%.f %z") {
        return Ok(Some(value.with_timezone(&Utc)));
    }
    if let Ok(value) = DateTime::parse_from_str(&normalized, "%Y-%m-%d %H:%M:%S%.f %z") {
        return Ok(Some(value.with_timezone(&Utc)));
    }
    if let Ok(value) = DateTime::parse_from_str(raw, "%Y-%m-%d %H:%M:%S %z") {
        return Ok(Some(value.with_timezone(&Utc)));
    }
    if let Ok(value) = DateTime::parse_from_str(&normalized, "%Y-%m-%d %H:%M:%S %z") {
        return Ok(Some(value.with_timezone(&Utc)));
    }
    if let Ok(value) = DateTime::parse_from_str(raw, "%Y-%m-%d %H:%M:%S%.f") {
        return Ok(Some(value.with_timezone(&Utc)));
    }
    if let Ok(value) = NaiveDateTime::parse_from_str(raw, "%Y-%m-%d %H:%M:%S") {
        return Ok(Some(Utc.from_utc_datetime(&value)));
    }
    if let Ok(value) = NaiveDate::parse_from_str(raw, "%Y-%m-%d") {
        if let Some(time) = value.and_hms_opt(0, 0, 0) {
            return Ok(Some(Utc.from_utc_datetime(&time)));
        }
        return Ok(None);
    }
    if let Ok(value) = raw.parse::<i64>() {
        if let Some(value) = DateTime::<Utc>::from_timestamp(value, 0) {
            return Ok(Some(value));
        }
    }
    Ok(None)
}

fn ensure_pg_schema(client: &mut Client) -> Result<()> {
    let row = client.query_one(
        r#"
        SELECT
            to_regclass('brain.source_documents')::text,
            to_regclass('brain.entity_snapshots')::text,
            to_regclass('brain.patch_events')::text,
            to_regclass('brain.knowledge_events')::text
        "#,
        &[],
    )?;
    let missing = [
        "source_documents",
        "entity_snapshots",
        "patch_events",
        "knowledge_events",
    ]
    .iter()
    .zip([
        row.get::<_, Option<String>>(0),
        row.get::<_, Option<String>>(1),
        row.get::<_, Option<String>>(2),
        row.get::<_, Option<String>>(3),
    ])
    .filter_map(|(name, value)| value.is_none().then_some(*name))
    .collect::<Vec<_>>();
    if !missing.is_empty() {
        return Err(anyhow!(
            "Zentrales brain-Schema fehlt/unvollstaendig: {}",
            missing.join(", ")
        ));
    }
    Ok(())
}

fn load_entity_index(client: &mut Client) -> Result<EntityIndex> {
    let mut index = EntityIndex::default();
    for row in client.query(
        r#"
        SELECT entity_type, canonical_name, canonical_name AS alias
        FROM brain.entities
        UNION ALL
        SELECT e.entity_type, e.canonical_name, a.alias
        FROM brain.entity_aliases a
        JOIN brain.entities e ON e.id = a.entity_id
        "#,
        &[],
    )? {
        let entity_type: String = row.get(0);
        let canonical_name: String = row.get(1);
        let alias: String = row.get(2);
        index.insert(&entity_type, &canonical_name, &alias);
    }
    add_manual_objective_aliases(&mut index);
    Ok(index.finish())
}

fn add_manual_objective_aliases(index: &mut EntityIndex) {
    for (entity_type, canonical, aliases) in [
        (
            "objective",
            "Unstable Rift",
            &["Unstable Rift", "King of the Hill", "KOTH"][..],
        ),
        ("objective", "Urn", &["Urn", "Soul Urn"][..]),
        (
            "objective_entity",
            "Rift Troopers",
            &["Rift Trooper", "Rift Troopers"][..],
        ),
        ("objective", "Mid Boss", &["Mid Boss", "Midboss"][..]),
        ("objective", "Patron", &["Patron", "Weakened Patron"][..]),
        ("objective", "Walker", &["Walker", "Walkers"][..]),
        ("objective", "Guardian", &["Guardian", "Guardians"][..]),
    ] {
        for alias in aliases {
            index.insert(entity_type, canonical, alias);
        }
    }
}

fn begin_run(tx: &mut Transaction<'_>) -> Result<i64> {
    let row = tx.query_one(
        "INSERT INTO brain.source_runs(source, status, started_at) VALUES ($1, 'running', now()) RETURNING id",
        &[&IMPORTER],
    )?;
    Ok(row.get(0))
}

fn finish_run(tx: &mut Transaction<'_>, run_id: i64, summary: &Value) -> Result<()> {
    let summary = json_text(summary)?;
    tx.execute(
        r#"
        UPDATE brain.source_runs
        SET status='ok', finished_at=now(), summary=$2::text::jsonb
        WHERE id=$1
        "#,
        &[
            &run_id as &(dyn ToSql + Sync),
            &summary as &(dyn ToSql + Sync),
        ],
    )?;
    Ok(())
}

fn upsert_source_document(tx: &mut Transaction<'_>, patch: &PreparedPatch) -> Result<i64> {
    let metadata = json_text(&json!({
        "patch_id": patch.row_id,
        "source_kind": patch.source_kind,
        "importer": IMPORTER,
        "source_external_id": patch.source_external_id,
        "policy": "Patchnotes aus zentraler changelog_posts werden als historisch importiert."
    }))?;
    let raw_path = format!("{SOURCE}/{}.json", patch.row_id);
    let row = tx.query_one(
        r#"
        INSERT INTO brain.source_documents(
            source, external_id, title, url, content_type, raw_path,
            content_hash, fetched_at, metadata
        )
        VALUES ($1,$2,$3,$4,'application/json',$5,$6,now(),$7::text::jsonb)
        ON CONFLICT (source, external_id, content_hash) DO UPDATE SET
            title = EXCLUDED.title,
            url = EXCLUDED.url,
            content_type = EXCLUDED.content_type,
            raw_path = EXCLUDED.raw_path,
            fetched_at = EXCLUDED.fetched_at,
            metadata = EXCLUDED.metadata
        RETURNING id
        "#,
        &[
            &SOURCE as &(dyn ToSql + Sync),
            &patch.source_external_id as &(dyn ToSql + Sync),
            &patch.title as &(dyn ToSql + Sync),
            &patch.url as &(dyn ToSql + Sync),
            &raw_path as &(dyn ToSql + Sync),
            &patch.raw_payload_hash as &(dyn ToSql + Sync),
            &metadata as &(dyn ToSql + Sync),
        ],
    )?;
    Ok(row.get(0))
}

fn upsert_entity_snapshot(
    tx: &mut Transaction<'_>,
    patch: &PreparedPatch,
    document_id: i64,
) -> Result<i64> {
    let row = tx.query_one(
        r#"
        INSERT INTO brain.entity_snapshots(
            source, entity_type, external_id, canonical_name, payload_hash,
            payload, fetched_at, source_document_id
        )
        VALUES ($1,'patchnote',$2,$3,$4,$5::text::jsonb,now(),$6)
        ON CONFLICT (source, entity_type, external_id, payload_hash) DO UPDATE SET
            canonical_name = EXCLUDED.canonical_name,
            payload = EXCLUDED.payload,
            fetched_at = EXCLUDED.fetched_at,
            source_document_id = EXCLUDED.source_document_id
        RETURNING id
        "#,
        &[
            &SOURCE as &(dyn ToSql + Sync),
            &patch.source_external_id as &(dyn ToSql + Sync),
            &patch.title as &(dyn ToSql + Sync),
            &patch.snapshot_payload_hash as &(dyn ToSql + Sync),
            &patch.snapshot_payload_text as &(dyn ToSql + Sync),
            &document_id as &(dyn ToSql + Sync),
        ],
    )?;
    Ok(row.get(0))
}

fn prune_direct_patch_events(tx: &mut Transaction<'_>, patch_external_id: &str) -> Result<()> {
    let rows = tx.query(
        r#"
        SELECT id
        FROM brain.patch_events
        WHERE patch_external_id=$1
          AND metadata->>'importer'=$2
        "#,
        &[
            &patch_external_id as &(dyn ToSql + Sync),
            &IMPORTER as &(dyn ToSql + Sync),
        ],
    )?;
    let ids = rows
        .into_iter()
        .map(|row| row.get::<_, i64>(0))
        .collect::<Vec<_>>();
    if ids.is_empty() {
        return Ok(());
    }
    let delete_params: &[&(dyn ToSql + Sync)] = &[&ids as &(dyn ToSql + Sync)];
    tx.execute(
        "DELETE FROM brain.knowledge_events WHERE event_source='patch_event' AND patch_event_id = ANY($1)",
        delete_params,
    )?;
    tx.execute(
        "DELETE FROM brain.patch_events WHERE id = ANY($1)",
        delete_params,
    )?;
    Ok(())
}

fn insert_patch_events(
    tx: &mut Transaction<'_>,
    patch: &PreparedPatch,
    snapshot_id: i64,
) -> Result<u64> {
    let mut changed = 0_u64;
    let legacy_patch_snapshot_id = -snapshot_id;
    for event in &patch.events {
        let posted_at_text = patch.posted_at.map(|value| value.to_rfc3339());
        let metadata = json_text(&event.metadata)?;
        tx.execute(
            r#"
            INSERT INTO brain.patch_events(
                patch_snapshot_id, legacy_patch_snapshot_id, patch_external_id,
                patch_title, patch_url, source_kind, posted_at, line_index, section,
                entity_type, entity_name, subject, change_type, raw_line, normalized_line,
                old_value, new_value, confidence, metadata, event_hash, created_at
            )
            VALUES (
                $1,$2,$3,$4,$5,$6,$7::text::timestamptz,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19::text::jsonb,$20,now()
            )
            ON CONFLICT (event_hash) DO UPDATE SET
                patch_snapshot_id = EXCLUDED.patch_snapshot_id,
                legacy_patch_snapshot_id = EXCLUDED.legacy_patch_snapshot_id,
                patch_title = EXCLUDED.patch_title,
                patch_url = EXCLUDED.patch_url,
                source_kind = EXCLUDED.source_kind,
                posted_at = EXCLUDED.posted_at,
                line_index = EXCLUDED.line_index,
                section = EXCLUDED.section,
                entity_type = EXCLUDED.entity_type,
                entity_name = EXCLUDED.entity_name,
                subject = EXCLUDED.subject,
                change_type = EXCLUDED.change_type,
                raw_line = EXCLUDED.raw_line,
                normalized_line = EXCLUDED.normalized_line,
                old_value = EXCLUDED.old_value,
                new_value = EXCLUDED.new_value,
                confidence = EXCLUDED.confidence,
                metadata = EXCLUDED.metadata,
                created_at = EXCLUDED.created_at
            "#,
            &[
                &snapshot_id as &(dyn ToSql + Sync),
                &legacy_patch_snapshot_id as &(dyn ToSql + Sync),
                &patch.patch_external_id as &(dyn ToSql + Sync),
                &patch.title as &(dyn ToSql + Sync),
                &patch.url as &(dyn ToSql + Sync),
                &patch.source_kind as &(dyn ToSql + Sync),
                &posted_at_text as &(dyn ToSql + Sync),
                &event.line_index as &(dyn ToSql + Sync),
                &event.section as &(dyn ToSql + Sync),
                &event.entity_type as &(dyn ToSql + Sync),
                &event.entity_name as &(dyn ToSql + Sync),
                &event.subject as &(dyn ToSql + Sync),
                &event.change_type as &(dyn ToSql + Sync),
                &event.raw_line as &(dyn ToSql + Sync),
                &event.normalized_line as &(dyn ToSql + Sync),
                &event.old_value as &(dyn ToSql + Sync),
                &event.new_value as &(dyn ToSql + Sync),
                &event.confidence as &(dyn ToSql + Sync),
                &metadata as &(dyn ToSql + Sync),
                &event.event_hash as &(dyn ToSql + Sync),
            ],
        )?;
        changed += 1;
    }
    Ok(changed)
}

fn materialize_patch_knowledge_events(
    tx: &mut Transaction<'_>,
    patch_external_id: &str,
) -> Result<u64> {
    Ok(tx.execute(
        r#"
        INSERT INTO brain.knowledge_events(
            event_hash, event_source, source_table, source_legacy_id, source_document_id,
            snapshot_id, patch_event_id, entity_type, entity_name, subject, event_type,
            validity_status, currentness, trust_tier, source_url, occurred_at, observed_at,
            effective_from, raw_text, normalized_text, old_value, new_value, confidence,
            source_references, payload, metadata
        )
        SELECT
            'patch:' || pe.event_hash,
            'patch_event',
            'patch_events',
            pe.id,
            sd.id,
            pe.patch_snapshot_id,
            pe.id,
            pe.entity_type,
            pe.entity_name,
            pe.subject,
            pe.change_type,
            'patch_history',
            'historical_patch_event',
            'trusted',
            pe.patch_url,
            pe.posted_at,
            COALESCE(pe.posted_at, pe.created_at),
            pe.posted_at,
            pe.raw_line,
            pe.normalized_line,
            pe.old_value,
            pe.new_value,
            pe.confidence,
            jsonb_build_array(jsonb_build_object('url', pe.patch_url, 'title', pe.patch_title)),
            jsonb_build_object(
                'patch_external_id', pe.patch_external_id,
                'patch_title', pe.patch_title,
                'source_kind', pe.source_kind,
                'section', pe.section,
                'importer', pe.metadata->>'importer',
                'source_posted_at', pe.metadata->>'source_posted_at'
            ),
            pe.metadata
        FROM brain.patch_events pe
        LEFT JOIN brain.entity_snapshots es ON es.id = pe.patch_snapshot_id
        LEFT JOIN brain.source_documents sd ON sd.id = es.source_document_id
        WHERE pe.patch_external_id=$1
          AND pe.metadata->>'importer'=$2
        ON CONFLICT (event_hash) DO UPDATE SET
            source_document_id = EXCLUDED.source_document_id,
            snapshot_id = EXCLUDED.snapshot_id,
            patch_event_id = EXCLUDED.patch_event_id,
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
            effective_from = EXCLUDED.effective_from,
            raw_text = EXCLUDED.raw_text,
            normalized_text = EXCLUDED.normalized_text,
            old_value = EXCLUDED.old_value,
            new_value = EXCLUDED.new_value,
            confidence = EXCLUDED.confidence,
            source_references = EXCLUDED.source_references,
            payload = EXCLUDED.payload,
            metadata = EXCLUDED.metadata,
            updated_at = now()
        "#,
        &[&patch_external_id, &IMPORTER],
    )?)
}

fn summary_json(
    dry_run: bool,
    options: &ImportPatchnoteOptions,
    patch: &PreparedPatch,
    counts: PgWriteCounts,
) -> Value {
    json!({
        "dry_run": dry_run,
        "target": "postgres",
        "dsn_env": options.dsn_env,
        "source": SOURCE,
        "patch_external_id": patch.patch_external_id,
        "source_external_id": patch.source_external_id,
        "patch_id": patch.row_id,
        "source_kind": patch.source_kind,
        "parsed_patch_events": patch.events.len(),
        "written": {
            "source_documents": counts.documents,
            "entity_snapshots": counts.snapshots,
            "patch_events": counts.patch_events,
            "knowledge_events_from_patch_events": counts.knowledge_events
        },
        "policy": {
            "order": "Pro Aufruf wird genau ein Patchnote-Datensatz importiert.",
            "duplication": "Duplikate werden durch ON CONFLICT (event_hash) idempotent behandelt.",
            "source_id": "source_documents.external_id = URL if present, else patchnotes:<id>"
        }
    })
}

fn patch_lines(content: &str) -> Vec<String> {
    let normalized = cleanup_patch_content(content);
    if let Some(flat_lines) = split_flat_forum_lines(&normalized) {
        return flat_lines;
    }
    if let Some(flat_lines) = split_square_bracket_forum_lines(&normalized) {
        return flat_lines;
    }

    let mut lines = Vec::new();
    for raw in normalized.lines() {
        for part in expand_inline_bullets(raw) {
            lines.push(part);
        }
    }
    lines
}

fn cleanup_patch_content(content: &str) -> String {
    let mut text = content
        .replace("&nbsp;", " ")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&apos;", "'")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace('\u{00A0}', " ");

    text = text.replace("<br/>", "\n");
    text = text.replace("<br />", "\n");
    text = text.replace("<br>", "\n");
    text = text.replace("</p>", "\n");
    text = text.replace("<p>", "\n");
    text = text.replace("<ul>", "\n");
    text = text.replace("</ul>", "\n");
    text = text.replace("<li>", "- ");
    text = text.replace("</li>", "\n");
    text = text.replace("</b>", "");
    text = text.replace("<b>", "");
    text = text.replace("<strong>", "");
    text = text.replace("</strong>", "");

    text = text.replace("\r\n", "\n");
    text = text.replace('\r', "\n");

    text
}

fn split_flat_forum_lines(content: &str) -> Option<Vec<String>> {
    if content.is_empty()
        || content.contains('\n')
        || !content.contains(": ==")
        || !content.contains(" - ")
    {
        return None;
    }

    let mut markers = Vec::new();
    let mut search_from = 0usize;
    while let Some(marker_pos_rel) = content[search_from..].find(": ==") {
        let marker_pos = search_from + marker_pos_rel;
        if let Some(section) = extract_forum_section_name(&content[..marker_pos]) {
            markers.push((marker_pos, section));
        }
        search_from = marker_pos + 4;
    }

    if markers.is_empty() {
        return None;
    }

    let mut lines = Vec::new();
    for (idx, (marker_pos, section)) in markers.iter().enumerate() {
        lines.push(section.trim().to_string());

        let section_body_end = markers.get(idx + 1).map_or(content.len(), |next| next.0);
        let section_body = content[*marker_pos + 4..section_body_end].trim();
        let section_body = if let Some((_, next_section)) = markers.get(idx + 1) {
            if section_body.ends_with(next_section) {
                let trimmed = section_body[..section_body.len() - next_section.len()].trim_end();
                trimmed.strip_suffix('-').unwrap_or(trimmed).trim_end()
            } else {
                section_body
            }
        } else {
            section_body
        };

        for line in expand_inline_bullets(section_body) {
            lines.push(line);
        }
    }

    if lines.is_empty() {
        None
    } else {
        Some(lines)
    }
}

fn split_square_bracket_forum_lines(content: &str) -> Option<Vec<String>> {
    if content.is_empty()
        || content.contains('\n')
        || !content.starts_with('[')
        || !content.contains(" - ")
    {
        return None;
    }

    let mut lines = Vec::new();
    let mut saw_section = false;
    let mut saw_event = false;

    for raw_piece in content.split(" - ") {
        let piece = raw_piece.trim();
        if piece.is_empty() {
            continue;
        }
        if let Some(section) = extract_forum_square_section_name(piece) {
            lines.push(section);
            saw_section = true;
            continue;
        }
        if let Some((event, section)) = split_square_forum_piece_with_embedded_section(piece) {
            if let Some(event) = event {
                saw_event = true;
                lines.push(format!("- {event}"));
            }
            lines.push(section);
            saw_section = true;
            continue;
        }
        if !saw_section {
            continue;
        }
        saw_event = true;
        lines.push(format!("- {piece}"));
    }

    if saw_section && saw_event {
        Some(lines)
    } else {
        None
    }
}

fn split_square_forum_piece_with_embedded_section(piece: &str) -> Option<(Option<String>, String)> {
    let piece = piece.trim();
    if !piece.ends_with(']') {
        return None;
    }
    let close = piece.len() - 1;
    let open = piece[..close].rfind('[')?;
    if open > 0 && !piece[..open].ends_with(' ') {
        return None;
    }
    let section = extract_forum_square_section_name_inner(piece[open + 1..close].trim())?;
    let event = piece[..open].trim();
    if event.is_empty() {
        return Some((None, section));
    }
    if event.len() < 2 {
        return Some((None, section));
    }
    Some((Some(event.to_string()), section))
}

fn extract_forum_square_section_name(raw_line: &str) -> Option<String> {
    let line = raw_line.trim();
    if !line.starts_with('[') || !line.ends_with(']') {
        return None;
    }
    extract_forum_square_section_name_inner(
        line.trim_start_matches('[').trim_end_matches(']').trim(),
    )
}

fn extract_forum_square_section_name_inner(raw_line: &str) -> Option<String> {
    let inner = raw_line.trim();
    if inner.len() < 2 || inner.len() > 80 {
        return None;
    }
    if !inner.chars().next()?.is_ascii_uppercase() {
        return None;
    }
    if !inner
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, ' ' | '&' | '/' | '+' | '-' | '(' | ')' | '.'))
    {
        return None;
    }
    Some(inner.to_string())
}

fn extract_forum_section_name(before_marker: &str) -> Option<String> {
    let mut words = Vec::new();

    for token in before_marker.split_whitespace().rev() {
        let token = token
            .trim_matches(|ch: char| matches!(ch, ',' | '.' | ':' | ';' | '!' | '?' | ')' | '('));
        if token == "A" {
            break;
        }
        if !is_forum_section_word(token) {
            break;
        }

        words.push(token);
        if words.len() >= 3 {
            break;
        }
    }

    if words.is_empty() {
        return None;
    }

    words.reverse();
    let section = words.join(" ");
    if section.len() < 2 || section.len() > 80 {
        return None;
    }

    let mut words_iter = section.split_whitespace();
    let first_word = words_iter.next()?;
    if matches!(
        first_word,
        "Added"
            | "Updated"
            | "Fixed"
            | "Changed"
            | "Reduced"
            | "Increased"
            | "Decreased"
            | "Removed"
    ) {
        return None;
    }

    Some(section)
}

fn is_forum_section_word(word: &str) -> bool {
    if word.is_empty() {
        return false;
    }

    let mut chars = word.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() && first.is_ascii_uppercase() || first.is_ascii_digit()) {
        return false;
    }

    for ch in chars {
        if !(ch.is_ascii_alphanumeric() || matches!(ch, '&' | '/' | '+' | '-')) {
            return false;
        }
    }

    true
}

fn is_inline_bullet_start(ch: char) -> bool {
    ch.is_ascii_uppercase() || ch.is_ascii_digit() || ch == '"' || ch == '\''
}

fn expand_inline_bullets(raw_line: &str) -> Vec<String> {
    let stripped = raw_line.trim();
    if stripped.is_empty() {
        return Vec::new();
    }

    if is_forum_section_heading(stripped) {
        return vec![raw_line.to_string()];
    }

    let normalized = if let Some(rest) = stripped.strip_prefix("- ") {
        rest.trim()
    } else {
        stripped
    };

    if normalized.contains(" - ") {
        let pieces = normalized.split(" - ").collect::<Vec<_>>();
        if pieces.len() > 1
            && pieces
                .iter()
                .skip(1)
                .all(|part| part.chars().next().is_some_and(is_inline_bullet_start))
        {
            return pieces
                .into_iter()
                .filter_map(|piece| {
                    let piece = piece.trim();
                    if piece.is_empty() {
                        None
                    } else {
                        Some(format!("- {piece}"))
                    }
                })
                .collect();
        }
    }

    if normalized
        .chars()
        .next()
        .is_some_and(is_inline_bullet_start)
    {
        vec![format!("- {normalized}")]
    } else if stripped.starts_with("- ") || stripped.starts_with("* ") || stripped.starts_with("• ")
    {
        vec![format!("- {}", stripped[2..].trim())]
    } else if stripped.starts_with("\u{2022} ") {
        vec![format!("- {}", stripped[3..].trim())]
    } else {
        vec![raw_line.to_string()]
    }
}

fn is_forum_section_heading(line: &str) -> bool {
    let trimmed = line.trim();
    if !trimmed.ends_with(':') {
        return false;
    }
    let body = trimmed.trim_end_matches(':').trim();
    if body.is_empty() || body.len() > 80 {
        return false;
    }
    let mut chars = body.chars();
    match chars.next() {
        Some(first) if first.is_ascii_alphabetic() => {}
        _ => return false,
    }
    if body.contains("http") {
        return false;
    }

    body.chars().all(|ch| {
        ch.is_ascii_alphanumeric()
            || ch.is_ascii_whitespace()
            || matches!(ch, '&' | '/' | '+' | '-')
    })
}

fn bullet_body(line: &str) -> Option<String> {
    for marker in ["- ", "* ", "• ", "\u{2022} "] {
        if let Some(body) = line.strip_prefix(marker) {
            return Some(body.trim().to_string());
        }
    }
    None
}

fn section_heading(line: &str) -> Option<String> {
    let cleaned = line.trim().trim_matches(':').trim();
    if cleaned.starts_with('[') && cleaned.ends_with(']') {
        return extract_forum_square_section_name_inner(
            cleaned.trim_start_matches('[').trim_end_matches(']').trim(),
        );
    }
    if cleaned.is_empty() || cleaned.len() > 80 {
        return None;
    }
    let lower = cleaned.to_ascii_lowercase();
    if lower.contains("http") || lower.starts_with('-') {
        return None;
    }
    Some(cleaned.to_string())
}

fn split_subject(line: &str) -> (Option<String>, Option<String>) {
    let Some(index) = line.find(':') else {
        return (None, Some(line.to_string()));
    };
    let subject = line[..index].trim();
    let remainder = line[index + 1..].trim();
    if subject.is_empty() || remainder.is_empty() || subject.len() > 64 {
        return (None, Some(line.to_string()));
    }
    if subject.matches(' ').count() > 7 {
        return (None, Some(line.to_string()));
    }
    (Some(subject.to_string()), Some(remainder.to_string()))
}

fn classify_source_kind(url: Option<&str>) -> String {
    let lower = url.unwrap_or_default().to_lowercase();
    if lower.contains("steamcommunity.com")
        || lower.contains("steampowered.com")
        || lower.contains("steamstore-a.akamaihd.net")
    {
        "steam".to_string()
    } else if lower.contains("forums.playdeadlock.com") {
        "forum".to_string()
    } else {
        "other".to_string()
    }
}

fn normalize_patch_line(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn classify_change_type(text: &str) -> String {
    let lower = text.to_ascii_lowercase();
    if lower.contains("renamed") || lower.contains("retitled") {
        "rename"
    } else if lower.contains("reworked") || lower.contains("rework") || lower.contains("redesigned")
    {
        "rework"
    } else if lower.contains("removed") || lower.contains("no longer") {
        "removed"
    } else if lower.contains("fixed") || lower.contains("fix ") || lower.starts_with("fix") {
        "fix"
    } else if lower.contains("added") || lower.contains("new ") || lower.starts_with("new") {
        "added"
    } else if lower.contains("increased")
        || lower.contains("improved")
        || lower.contains("higher")
        || lower.contains("more ")
    {
        "buff"
    } else if lower.contains("reduced")
        || lower.contains("decreased")
        || lower.contains("lower")
        || lower.contains("less ")
        || lower.contains("slower")
    {
        "nerf"
    } else if lower.contains(" from ") && lower.contains(" to ") {
        "balance_delta"
    } else {
        "mechanic_change"
    }
    .to_string()
}

fn extract_old_new(line: &str) -> (Option<String>, Option<String>) {
    let lower = line.to_ascii_lowercase();
    let Some(from_pos) = lower.find(" from ") else {
        return (None, None);
    };
    let Some(to_rel) = lower[from_pos + 6..].find(" to ") else {
        return (None, None);
    };
    let old_start = from_pos + 6;
    let to_pos = old_start + to_rel;
    let new_start = to_pos + 4;
    let old_value = trim_value(&line[old_start..to_pos]);
    let new_value = trim_value(&line[new_start..]);
    if old_value.is_empty() || new_value.is_empty() {
        (None, None)
    } else {
        (Some(old_value), Some(new_value))
    }
}

fn trim_value(value: &str) -> String {
    value
        .trim()
        .trim_end_matches(['.', ',', ';'])
        .chars()
        .take(80)
        .collect::<String>()
}

fn json_text(value: &Value) -> Result<String> {
    Ok(serde_json::to_string(value)?)
}

fn stable_hash(content: &[u8]) -> String {
    hex::encode(Sha256::digest(content))
}

fn normalize_key(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    let mut last_space = true;
    for ch in value.chars().flat_map(char::to_lowercase) {
        if ch.is_ascii_alphanumeric() {
            output.push(ch);
            last_space = false;
        } else if !last_space {
            output.push(' ');
            last_space = true;
        }
    }
    output.trim().to_string()
}

fn contains_alias(haystack: &str, alias: &str) -> bool {
    let needle = format!(" {alias} ");
    haystack.contains(&needle)
}

fn public_entity_type(entity_type: &str) -> &str {
    if entity_type.contains("hero") {
        "hero"
    } else if entity_type.contains("item") {
        "item"
    } else if entity_type.contains("ability") {
        "ability"
    } else {
        entity_type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn posted_at(value: &str) -> String {
        parse_posted_at(Some(value))
            .expect("parse posted_at")
            .expect("value")
            .to_rfc3339()
    }

    #[test]
    fn collects_steam_link_candidates_from_forum_teaser() {
        let row = PatchnoteRow {
            id: 12,
            title: Some("05-08-2025 Update".to_string()),
            url: Some("https://forums.playdeadlock.com/threads/05-08-2025-update.63133/".to_string()),
            posted_at: Some(posted_at("2025-05-08")),
            raw_content: Some("Full patch details on Steam: https://steamstore-a.akamaihd.net/news/externalpost/steam_community_announcements/1799088287841594".to_string()),
            translated_content: None,
        };

        let candidates = collect_steam_links(&row);

        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].gid.as_deref(), Some("1799088287841594"));
    }

    #[test]
    fn match_steam_news_item_prefers_gid_candidate() {
        let row = PatchnoteRow {
            id: 12,
            title: Some("05-08-2025 Update".to_string()),
            url: Some("https://forums.playdeadlock.com/threads/05-08-2025-update.63133/".to_string()),
            posted_at: Some(posted_at("2025-05-08")),
            raw_content: Some("Full patch details on Steam: https://steamstore-a.akamaihd.net/news/externalpost/steam_community_announcements/1799088287841594".to_string()),
            translated_content: None,
        };
        let candidates = collect_steam_links(&row);
        let items = [
            SteamAppNewsItem {
                gid: "11".to_string(),
                title: "Other".to_string(),
                url: "https://steamstore-a.akamaihd.net/news/externalpost/steam_community_announcements/11".to_string(),
                contents: "- Other".to_string(),
                date: 0,
                author: None,
                feedname: None,
                feedlabel: Some("Steam".to_string()),
            },
            SteamAppNewsItem {
                gid: "1799088287841594".to_string(),
                title: "Shop Rework Update".to_string(),
                url: "https://steamstore-a.akamaihd.net/news/externalpost/steam_community_announcements/1799088287841594".to_string(),
                contents: "- General Changes: Test".to_string(),
                date: 1,
                author: None,
                feedname: Some("steam_community_announcements".to_string()),
                feedlabel: Some("Steam".to_string()),
            },
        ];

        let item = match_steam_news_item(
            &row,
            Some(
                parse_posted_at(Some("2025-05-08T00:00:00+00:00"))
                    .expect("posted_at")
                    .expect("timestamp"),
            ),
            &candidates,
            &items,
        )
        .expect("match");

        assert_eq!(item.gid, "1799088287841594");
        assert!(item.title.contains("Shop Rework"));
    }

    #[test]
    fn finds_official_steam_item_for_forum_patch_with_date_filter() {
        let row = PatchnoteRow {
            id: 12,
            title: Some("05-08-2025 Update".to_string()),
            url: Some("https://forums.playdeadlock.com/threads/05-08-2025-update.63133/".to_string()),
            posted_at: Some("2025-05-08T19:43:20+00:00".to_string()),
            raw_content: Some("Deadlock - Shop Rework Update - Steam News Full shop rework including many new items, build authoring, quickbuy redesign and several other game updates store.steampowered.com".to_string()),
            translated_content: None,
        };
        let items = [
            SteamAppNewsItem {
                gid: "1833968530895860".to_string(),
                title: "Minor Update - 05-31-2026".to_string(),
                url: "https://steamstore-a.akamaihd.net/news/externalpost/steam_community_announcements/1833968530895860".to_string(),
                contents: "- Minor".to_string(),
                date: 1780256754,
                author: None,
                feedname: Some("steam_community_announcements".to_string()),
                feedlabel: Some("Steam".to_string()),
            },
            SteamAppNewsItem {
                gid: "1799088287841594".to_string(),
                title: "Shop Rework Update".to_string(),
                url: "https://steamstore-a.akamaihd.net/news/externalpost/steam_community_announcements/1799088287841594".to_string(),
                contents: "- Shop".to_string(),
                date: 1746732792,
                author: None,
                feedname: Some("steam_community_announcements".to_string()),
                feedlabel: Some("Steam".to_string()),
            },
        ];

        let item = find_matching_official_news_item(
            &row,
            Some(
                parse_posted_at(row.posted_at.as_deref())
                    .expect("parse posted_at")
                    .expect("value"),
            ),
            &items,
        )
            .expect("found item");

        assert_eq!(item.gid, "1799088287841594");
        assert_eq!(item.title, "Shop Rework Update");
    }

    #[test]
    fn uses_steam_source_when_resolved_for_prepare() {
        let row = PatchnoteRow {
            id: 12,
            title: Some("05-08-2025 Update".to_string()),
            url: Some("https://forums.playdeadlock.com/threads/05-08-2025-update.63133/".to_string()),
            posted_at: Some(posted_at("2025-05-08")),
            raw_content: Some(
                "- General Changes: Added a new interaction."
                    .to_string(),
            ),
            translated_content: None,
        };

        let source = PatchSourceResolution {
            raw_content: clean_steam_content(
                "[list][*]- General Changes: Added a new interaction.[/*][/list]",
            ),
            source_url: Some("https://steamstore-a.akamaihd.net/news/externalpost/steam_community_announcements/1799088287841594".to_string()),
            source_kind: "steam".to_string(),
            resolved_from: Some("steam_gid:1799088287841594".to_string()),
        };
        let mut index = EntityIndex::default();
        index.insert("objective", "something", "interaction");
        let index = index.finish();
        let prepared = prepare_patch(&row, &source, &index).expect("prepare");

        assert_eq!(prepared.source_kind, "steam");
        assert_eq!(
            prepared.url,
            Some("https://steamstore-a.akamaihd.net/news/externalpost/steam_community_announcements/1799088287841594".to_string())
        );
        assert_eq!(prepared.events.len(), 1);
    }

    #[test]
    fn parse_bullet_lines_and_event_hash_is_stable() {
        let row = PatchnoteRow {
            id: 17,
            title: Some("Patch 17".to_string()),
            url: Some("https://steamcommunity.com/nachrichten/17".to_string()),
            posted_at: Some(posted_at("2026-06-30")),
            raw_content: Some("- Aegis: increased health by 10".to_string()),
            translated_content: None,
        };
        let mut index = EntityIndex::default();
        index.insert("item", "Aegis", "Aegis");
        let index = index.finish();
        let prepared = prepare_patch(&row, &PatchSourceResolution::from_row(&row), &index).expect("prepare");
        assert_eq!(prepared.events.len(), 1);
        assert_eq!(prepared.events[0].entity_name.as_deref(), Some("Aegis"));
        assert_eq!(
            prepared.events[0].event_hash,
            build_event_hash(
                17,
                1,
                "",
                &prepared.events[0].entity_type,
                &prepared.events[0].normalized_line
            )
        );
    }

    #[test]
    fn expands_forum_section_headings_without_bullet_prefix() {
        let expanded = expand_inline_bullets("General Changes:");
        assert_eq!(expanded, vec!["General Changes:"]);
    }

    #[test]
    fn section_heading_rejects_invalid_square_bracket_sections() {
        assert_eq!(section_heading("[http://forums.playdeadlock.com/threads/x]"), None);
        assert_eq!(section_heading("[b]"), None);
    }

    #[test]
    fn parses_forum_single_line_sections() {
        let row = PatchnoteRow {
            id: 80,
            title: Some("05-03-2024 Update".to_string()),
            url: Some("https://forums.playdeadlock.com/threads/05-03-2024-update.427/".to_string()),
            posted_at: Some(posted_at("2024-05-03T20:02:54+00:00")),
            raw_content: Some(
                "General Changes: == - Added a Recommend A Friend button to the dashboard - Added the Patron to the spectate-when-dead cycle if the enemy is in your base or everyone on your team is dead Gameplay Changes: == - Abrams: Base Health increased from 550 to 600 - Urn bounty increased from 900 + 160/minute to 900 + 200/minute".to_string(),
            ),
            translated_content: None,
        };
        let mut index = EntityIndex::default();
        index.insert("hero", "Abrams", "Abrams");
        index.insert("hero", "Patron", "Patron");
        let index = index.finish();

        let prepared = prepare_patch(&row, &PatchSourceResolution::from_row(&row), &index).expect("prepare");
        assert!(!prepared.events.is_empty());
        assert_eq!(
            prepared.events[0].section.as_deref(),
            Some("General Changes")
        );
        assert!(prepared
            .events
            .iter()
            .any(|event| event.section.as_deref() == Some("Gameplay Changes")));
        assert!(prepared
            .events
            .iter()
            .any(|event| event.subject.as_deref() == Some("Abrams")));
    }

    #[test]
    fn parses_forum_square_bracket_single_line_sections() {
        let row = PatchnoteRow {
            id: 69,
            title: Some("06-13-2024 Update".to_string()),
            url: Some("https://forums.playdeadlock.com/threads/06-13-2024-update.5773/".to_string()),
            posted_at: Some(posted_at("2024-06-13T14:01:34+00:00")),
            raw_content: Some(
                "[ General Changes] - Voice/Text chat is now opt-in. There is a prompt pre-match for joining the chat. - You can now press ESC to mute individual players. - The Patrons no longer care about the well being of the Urn Runner, and so they will stop asking you to protect them [ Misc Gameplay ] - Added two new underground tunnels, one on each side of the map. - Added two new teleporters, one on each outer lane"
                    .to_string(),
            ),
            translated_content: None,
        };
        let prepared = prepare_patch(&row, &PatchSourceResolution::from_row(&row), &EntityIndex::default()).expect("prepare");
        assert_eq!(prepared.events.len(), 5);
        assert_eq!(
            prepared.events[0].section.as_deref(),
            Some("General Changes")
        );
        assert!(prepared
            .events
            .iter()
            .any(|event| event.section.as_deref() == Some("Misc Gameplay")));
        assert!(prepared
            .events
            .iter()
            .any(|event| event.subject.is_none()));
    }

    #[test]
    fn parses_forum_multiline_sections_with_flat_fix_still_intact() {
        let row = PatchnoteRow {
            id: 81,
            title: Some("Patch with multiline headings".to_string()),
            url: Some("https://forums.playdeadlock.com/threads/patch-81-update".to_string()),
            posted_at: Some(posted_at("2024-06-01")),
            raw_content: Some(
                "General Changes:\nPatron: Added the spectate timer to combat flow\nGameplay Changes:\nAbrams: Base Health increased from 550 to 600"
                    .to_string(),
            ),
            translated_content: None,
        };
        let mut index = EntityIndex::default();
        index.insert("objective", "Patron", "Patron");
        index.insert("hero", "Abrams", "Abrams");
        let index = index.finish();

        let prepared = prepare_patch(&row, &PatchSourceResolution::from_row(&row), &index).expect("prepare");
        assert_eq!(prepared.events.len(), 2);
        assert_eq!(
            prepared.events[0].section.as_deref(),
            Some("General Changes")
        );
        assert_eq!(
            prepared.events[1].section.as_deref(),
            Some("Gameplay Changes")
        );
    }

    #[test]
    fn split_flat_forum_content() {
        let lines = split_flat_forum_lines(
            "General Changes: == - Added A - Gameplay Changes: == - Abrams: Base Health increased from 550 to 600",
        )
        .expect("flat");
        assert_eq!(lines.len(), 4);
        assert_eq!(lines[0], "General Changes");
        assert_eq!(lines[1], "- Added A");
        assert_eq!(lines[2], "Gameplay Changes");
        assert_eq!(lines[3], "- Abrams: Base Health increased from 550 to 600");
    }

    #[test]
    fn parse_events_have_deduplicated_hashes_by_line() {
        let row = PatchnoteRow {
            id: 17,
            title: Some("Patch 17".to_string()),
            url: Some("https://steamcommunity.com/nachrichten/17".to_string()),
            posted_at: Some(posted_at("2026-06-30")),
            raw_content: Some(
                "- Aegis: increased health by 10\n- Aegis: increased speed by 12".to_string(),
            ),
            translated_content: None,
        };
        let mut index = EntityIndex::default();
        index.insert("item", "Aegis", "Aegis");
        let index = index.finish();
        let prepared = prepare_patch(&row, &PatchSourceResolution::from_row(&row), &index).expect("prepare");
        assert_eq!(prepared.events.len(), 2);
        assert_ne!(prepared.events[0].event_hash, prepared.events[1].event_hash);
        assert_eq!(
            prepared.events[0].event_hash,
            build_event_hash(
                17,
                1,
                "",
                &prepared.events[0].entity_type,
                &prepared.events[0].normalized_line
            )
        );
        assert_eq!(
            prepared.events[1].event_hash,
            build_event_hash(
                17,
                2,
                "",
                &prepared.events[1].entity_type,
                &prepared.events[1].normalized_line
            )
        );
    }

    #[test]
    fn posted_at_allows_plain_date_and_text() {
        let parsed = parse_posted_at(Some("2026-06-30"))
            .expect("parsed")
            .expect("value");
        assert_eq!(parsed.to_rfc3339(), "2026-06-30T00:00:00+00:00");
        let fallback = parse_posted_at(Some("2026-06-30T17:22:14Z"))
            .expect("parsed")
            .expect("value");
        assert_eq!(fallback.to_rfc3339(), "2026-06-30T17:22:14+00:00");

        let db_style = parse_posted_at(Some("2025-05-08 19:43:20+00"))
            .expect("parsed")
            .expect("value");
        assert_eq!(db_style.to_rfc3339(), "2025-05-08T19:43:20+00:00");
    }

    fn build_event_hash(
        patch_id: i64,
        line_index: i64,
        section: &str,
        entity_type: &str,
        normalized_line: &str,
    ) -> String {
        stable_hash(
            format!(
                "patchnotes|{patch_id}|{line_index}|{}|{entity_type}|{normalized_line}",
                section
            )
            .as_bytes(),
        )
    }
}
