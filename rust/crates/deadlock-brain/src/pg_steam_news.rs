use std::{
    collections::{HashMap, HashSet},
    env,
    time::Duration,
};

use anyhow::{anyhow, Context, Result};
use chrono::{NaiveDate, TimeZone, Utc};
use deadlock_brain_core::http::{HttpClient, HttpGetOptions};
use postgres::{Client, NoTls, Transaction};
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const SOURCE: &str = "steam_appnews";
const IMPORTER: &str = "steam_appnews_pg";
const APPNEWS_API: &str = "https://api.steampowered.com/ISteamNews/GetNewsForApp/v2/";

#[derive(Debug, Clone)]
pub struct ImportSteamNewsOptions {
    pub dsn_env: String,
    pub appid: u32,
    pub start_date: String,
    pub count: u32,
    pub cache_ttl_seconds: u64,
    pub include_non_official: bool,
    pub gids: Vec<String>,
    pub dry_run: bool,
}

#[derive(Debug, Deserialize)]
struct AppNewsResponse {
    appnews: AppNews,
}

#[derive(Debug, Deserialize)]
struct AppNews {
    #[serde(default)]
    newsitems: Vec<SteamNewsItem>,
}

#[derive(Debug, Clone, Deserialize)]
struct SteamNewsItem {
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
    #[serde(default)]
    feed_type: Option<i64>,
    #[serde(default)]
    appid: Option<i64>,
    #[serde(default)]
    is_external_url: Option<bool>,
}

#[derive(Debug)]
struct PreparedPatch {
    item: SteamNewsItem,
    external_id: String,
    posted_at: String,
    raw_payload_hash: String,
    snapshot_payload_text: String,
    snapshot_payload_hash: String,
    events: Vec<PreparedEvent>,
}

#[derive(Debug)]
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

struct EventParseContext<'a> {
    item: &'a SteamNewsItem,
    external_id: &'a str,
    posted_at: &'a str,
    section: Option<&'a str>,
    index: &'a EntityIndex,
}

#[derive(Debug, Clone)]
struct EntityAlias {
    key: String,
    entity_type: String,
    canonical_name: String,
}

#[derive(Debug, Default, Clone)]
struct EntityIndex {
    by_key: HashMap<String, EntityAlias>,
    aliases_desc: Vec<EntityAlias>,
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
        self.aliases_desc
            .sort_by(|a, b| b.key.len().cmp(&a.key.len()).then(a.key.cmp(&b.key)));
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

pub fn import_steam_news(http: &HttpClient, options: &ImportSteamNewsOptions) -> Result<Value> {
    let start_date = NaiveDate::parse_from_str(&options.start_date, "%Y-%m-%d")
        .with_context(|| format!("ungueltiges --start-date: {}", options.start_date))?;
    let api_url = appnews_url(options.appid, options.count);
    let response = http.get_json::<AppNewsResponse>(
        &api_url,
        HttpGetOptions {
            cache_ttl_seconds: Some(options.cache_ttl_seconds),
            timeout: Duration::from_secs(45),
            ..HttpGetOptions::default()
        },
    )?;
    let selected = select_items(
        response.appnews.newsitems,
        start_date,
        options.include_non_official,
        &options.gids,
    )?;

    if options.dry_run {
        let index = EntityIndex::default().finish();
        let prepared = prepare_patches(options.appid, selected, &index)?;
        return Ok(summary_json(
            true,
            &api_url,
            options,
            &prepared,
            PgWriteCounts::default(),
        ));
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
    let index = load_entity_index(&mut client)?;
    let prepared = prepare_patches(options.appid, selected, &index)?;

    let mut tx = client.transaction()?;
    let run_id = begin_run(&mut tx)?;
    let mut counts = PgWriteCounts::default();
    for patch in &prepared {
        let document_id = upsert_source_document(&mut tx, patch, options.appid, &api_url)?;
        counts.documents += 1;
        let snapshot_id = upsert_entity_snapshot(&mut tx, patch, document_id)?;
        counts.snapshots += 1;
        prune_direct_patch_events(&mut tx, &patch.external_id)?;
        counts.patch_events += insert_patch_events(&mut tx, patch, snapshot_id)?;
    }
    counts.knowledge_events = materialize_patch_knowledge_events(&mut tx)?;
    finish_run(
        &mut tx,
        run_id,
        &summary_json(false, &api_url, options, &prepared, counts),
    )?;
    tx.commit()?;

    Ok(summary_json(false, &api_url, options, &prepared, counts))
}

fn appnews_url(appid: u32, count: u32) -> String {
    format!("{APPNEWS_API}?appid={appid}&count={count}&maxlength=100000&format=json")
}

fn select_items(
    mut items: Vec<SteamNewsItem>,
    start_date: NaiveDate,
    include_non_official: bool,
    gids: &[String],
) -> Result<Vec<SteamNewsItem>> {
    let wanted_gids = gids.iter().map(|gid| gid.trim()).collect::<HashSet<_>>();
    let start = Utc
        .from_utc_datetime(
            &start_date
                .and_hms_opt(0, 0, 0)
                .ok_or_else(|| anyhow!("ungueltiger Startzeitpunkt"))?,
        )
        .timestamp();
    items.retain(|item| {
        if !wanted_gids.is_empty() && !wanted_gids.contains(item.gid.as_str()) {
            return false;
        }
        if item.date < start {
            return false;
        }
        include_non_official || is_official_steam_announcement(item)
    });
    items.sort_by(|a, b| a.date.cmp(&b.date).then(a.gid.cmp(&b.gid)));
    Ok(items)
}

fn is_official_steam_announcement(item: &SteamNewsItem) -> bool {
    item.feedname.as_deref() == Some("steam_community_announcements")
        || item.url.contains("steam_community_announcements")
}

fn prepare_patches(
    appid: u32,
    items: Vec<SteamNewsItem>,
    index: &EntityIndex,
) -> Result<Vec<PreparedPatch>> {
    items
        .into_iter()
        .map(|item| prepare_patch(appid, item, index))
        .collect()
}

fn prepare_patch(appid: u32, item: SteamNewsItem, index: &EntityIndex) -> Result<PreparedPatch> {
    let posted_at = timestamp_to_rfc3339(item.date)?;
    let external_id = format!("{SOURCE}:{}", item.gid);
    let clean_content = clean_steam_content(&item.contents);
    let raw_payload = json!({
        "appid": appid,
        "gid": item.gid,
        "title": item.title,
        "url": item.url,
        "date": item.date,
        "posted_at": posted_at,
        "author": item.author,
        "feedlabel": item.feedlabel,
        "feedname": item.feedname,
        "feed_type": item.feed_type,
        "item_appid": item.appid,
        "is_external_url": item.is_external_url,
        "contents": item.contents,
    });
    let raw_payload_text = json_text(&raw_payload)?;
    let raw_payload_hash = stable_hash(raw_payload_text.as_bytes());

    let snapshot_payload = json!({
        "id": item.gid,
        "appid": appid,
        "title": item.title,
        "url": item.url,
        "posted_at": posted_at,
        "raw_content": clean_content,
        "translated_content": Value::Null,
        "source_kind": "steam",
        "steam": raw_payload,
        "importer": IMPORTER,
    });
    let snapshot_payload_text = json_text(&snapshot_payload)?;
    let snapshot_payload_hash = stable_hash(snapshot_payload_text.as_bytes());
    let events = parse_events(&item, &external_id, &posted_at, &clean_content, index);

    Ok(PreparedPatch {
        item,
        external_id,
        posted_at,
        raw_payload_hash,
        snapshot_payload_text,
        snapshot_payload_hash,
        events,
    })
}

fn timestamp_to_rfc3339(timestamp: i64) -> Result<String> {
    let dt = Utc
        .timestamp_opt(timestamp, 0)
        .single()
        .ok_or_else(|| anyhow!("ungueltiger Steam-Zeitstempel: {timestamp}"))?;
    Ok(dt.to_rfc3339())
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
        "[b]", "[/b]", "[i]", "[/i]", "[u]", "[/u]", "[h1]", "[/h1]", "[h2]", "[/h2]", "[h3]",
        "[/h3]", "[code]", "[/code]",
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

fn parse_events(
    item: &SteamNewsItem,
    external_id: &str,
    posted_at: &str,
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
            let context = EventParseContext {
                item,
                external_id,
                posted_at,
                section: section.as_deref(),
                index,
            };
            if let Some(event) = parse_bullet_event(&context, line_index, trimmed, &body) {
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

fn patch_lines(content: &str) -> Vec<String> {
    let mut lines = Vec::new();
    for raw in content.lines() {
        let stripped = raw.trim();
        if stripped.starts_with("- ") && stripped.contains(" - ") {
            let pieces = stripped[2..].split(" - ").collect::<Vec<_>>();
            if pieces.len() > 1
                && pieces.iter().skip(1).all(|part| {
                    part.chars().next().is_some_and(|ch| {
                        ch.is_ascii_uppercase() || ch.is_ascii_digit() || ch == '"'
                    })
                })
            {
                for piece in pieces {
                    let piece = piece.trim();
                    if !piece.is_empty() {
                        lines.push(format!("- {piece}"));
                    }
                }
                continue;
            }
        }
        lines.push(raw.to_string());
    }
    lines
}

fn bullet_body(line: &str) -> Option<String> {
    for marker in ["- ", "* ", "• "] {
        if let Some(body) = line.strip_prefix(marker) {
            return Some(body.trim().to_string());
        }
    }
    None
}

fn section_heading(line: &str) -> Option<String> {
    let cleaned = line.trim().trim_matches(':').trim();
    if cleaned.is_empty() || cleaned.len() > 80 {
        return None;
    }
    let lower = cleaned.to_ascii_lowercase();
    if lower.contains("http") || lower.starts_with('-') {
        return None;
    }
    Some(cleaned.to_string())
}

fn parse_bullet_event(
    context: &EventParseContext<'_>,
    line_index: i64,
    raw_line: &str,
    body: &str,
) -> Option<PreparedEvent> {
    let normalized_line = normalize_patch_line(body);
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
        "steam_gid": context.item.gid,
        "steam_feedname": context.item.feedname,
        "steam_author": context.item.author,
        "source_language": "en",
        "source_posted_at": context.posted_at,
        "line_subject": subject,
        "line_remainder": remainder,
        "section": context.section,
    });
    let event_hash = stable_hash(
        format!(
            "{SOURCE}|{}|{}|{line_index}|{}|{}|{}",
            context.item.gid,
            context.external_id,
            context.section.unwrap_or_default(),
            entity_type,
            normalized_line
        )
        .as_bytes(),
    );

    Some(PreparedEvent {
        line_index,
        section: context.section.map(ToString::to_string),
        entity_type,
        entity_name,
        subject,
        change_type,
        raw_line: raw_line.to_string(),
        normalized_line,
        old_value,
        new_value,
        confidence,
        metadata,
        event_hash,
    })
}

fn normalize_patch_line(line: &str) -> String {
    line.split_whitespace().collect::<Vec<_>>().join(" ")
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

fn classify_change_type(line: &str) -> String {
    let lower = line.to_ascii_lowercase();
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
        return (None, None);
    }
    (Some(old_value), Some(new_value))
}

fn trim_value(value: &str) -> String {
    value
        .trim()
        .trim_end_matches(['.', ',', ';'])
        .chars()
        .take(80)
        .collect::<String>()
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
        &[&SOURCE],
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
        &[&run_id, &summary],
    )?;
    Ok(())
}

fn upsert_source_document(
    tx: &mut Transaction<'_>,
    patch: &PreparedPatch,
    appid: u32,
    api_url: &str,
) -> Result<i64> {
    let metadata = json_text(&json!({
        "appid": appid,
        "gid": patch.item.gid,
        "feedname": patch.item.feedname,
        "feedlabel": patch.item.feedlabel,
        "feed_type": patch.item.feed_type,
        "author": patch.item.author,
        "source_kind": "steam",
        "api_url": api_url,
        "importer": IMPORTER,
        "policy": "official Steam Community Announcements are imported as trusted patch history",
    }))?;
    let raw_path = format!("{SOURCE}/{}.json", patch.item.gid);
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
            &SOURCE,
            &patch.external_id,
            &Some(patch.item.title.clone()),
            &Some(patch.item.url.clone()),
            &raw_path,
            &patch.raw_payload_hash,
            &metadata,
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
            &SOURCE,
            &patch.external_id,
            &Some(patch.item.title.clone()),
            &patch.snapshot_payload_hash,
            &patch.snapshot_payload_text,
            &document_id,
        ],
    )?;
    Ok(row.get(0))
}

fn prune_direct_patch_events(tx: &mut Transaction<'_>, external_id: &str) -> Result<()> {
    let rows = tx.query(
        r#"
        SELECT id
        FROM brain.patch_events
        WHERE patch_external_id=$1
          AND source_kind='steam'
          AND metadata->>'importer'=$2
        "#,
        &[&external_id, &IMPORTER],
    )?;
    let ids = rows
        .into_iter()
        .map(|row| row.get::<_, i64>(0))
        .collect::<Vec<_>>();
    if ids.is_empty() {
        return Ok(());
    }
    tx.execute(
        "DELETE FROM brain.knowledge_events WHERE event_source='patch_event' AND patch_event_id = ANY($1)",
        &[&ids],
    )?;
    tx.execute("DELETE FROM brain.patch_events WHERE id = ANY($1)", &[&ids])?;
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
        let metadata = json_text(&event.metadata)?;
        let row = tx.query_one(
            r#"
            INSERT INTO brain.patch_events(
                patch_snapshot_id, legacy_patch_snapshot_id, patch_external_id,
                patch_title, patch_url, source_kind, posted_at, line_index, section,
                entity_type, entity_name, subject, change_type, raw_line, normalized_line,
                old_value, new_value, confidence, metadata, event_hash, created_at
            )
            VALUES (
                $1,$2,$3,$4,$5,'steam',$6::text::timestamptz,$7,$8,$9,$10,$11,$12,
                $13,$14,$15,$16,$17,$18::text::jsonb,$19,now()
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
            RETURNING id
            "#,
            &[
                &snapshot_id,
                &legacy_patch_snapshot_id,
                &patch.external_id,
                &Some(patch.item.title.clone()),
                &Some(patch.item.url.clone()),
                &patch.posted_at,
                &event.line_index,
                &event.section,
                &event.entity_type,
                &event.entity_name,
                &event.subject,
                &event.change_type,
                &event.raw_line,
                &event.normalized_line,
                &event.old_value,
                &event.new_value,
                &event.confidence,
                &metadata,
                &event.event_hash,
            ],
        )?;
        let _: i64 = row.get(0);
        changed += 1;
    }
    Ok(changed)
}

fn materialize_patch_knowledge_events(tx: &mut Transaction<'_>) -> Result<u64> {
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
            pe.legacy_sqlite_id,
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
                'importer', pe.metadata->>'importer'
            ),
            pe.metadata
        FROM brain.patch_events pe
        LEFT JOIN brain.entity_snapshots es ON es.id = pe.patch_snapshot_id
        LEFT JOIN brain.source_documents sd ON sd.id = es.source_document_id
        WHERE pe.source_kind='steam'
          AND pe.metadata->>'importer'=$1
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
        &[&IMPORTER],
    )?)
}

fn summary_json(
    dry_run: bool,
    api_url: &str,
    options: &ImportSteamNewsOptions,
    patches: &[PreparedPatch],
    counts: PgWriteCounts,
) -> Value {
    let total_events = patches
        .iter()
        .map(|patch| patch.events.len())
        .sum::<usize>();
    let first_patch = patches.first().map(patch_summary);
    let last_patch = patches.last().map(patch_summary);
    json!({
        "dry_run": dry_run,
        "target": "postgres",
        "dsn_env": options.dsn_env,
        "source": SOURCE,
        "api_url": api_url,
        "appid": options.appid,
        "start_date": options.start_date,
        "count_requested": options.count,
        "official_only": !options.include_non_official,
        "patches": patches.len(),
        "parsed_patch_events": total_events,
        "first_patch": first_patch,
        "last_patch": last_patch,
        "written": {
            "source_documents": counts.documents,
            "entity_snapshots": counts.snapshots,
            "patch_events": counts.patch_events,
            "knowledge_events_from_patch_events": counts.knowledge_events
        },
        "policy": {
            "order": "Steam News werden alt-nach-neu importiert.",
            "trust": "Nur offizielle steam_community_announcements sind standardmaessig trusted.",
            "history": "Bestehende historische Quellen bleiben erhalten; direkte Steam-Events ueberschreiben keine nicht-Steam-Events."
        }
    })
}

fn patch_summary(patch: &PreparedPatch) -> Value {
    json!({
        "gid": patch.item.gid,
        "title": patch.item.title,
        "posted_at": patch.posted_at,
        "url": patch.item.url,
        "events": patch.events.len()
    })
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

    #[test]
    fn steam_content_cleanup_turns_bbcode_items_into_bullets() {
        let cleaned = clean_steam_content("[h2]General[/h2][list][*]A &amp; B[*]C[/list]");

        assert_eq!(cleaned, "General\n- A & B\n- C");
    }

    #[test]
    fn parser_maps_manual_objective_aliases() {
        let mut index = EntityIndex::default();
        add_manual_objective_aliases(&mut index);
        let index = index.finish();
        let item = SteamNewsItem {
            gid: "1".to_string(),
            title: "Minor Update - 06-30-2026".to_string(),
            url: "https://steamstore-a.akamaihd.net/news/externalpost/steam_community_announcements/1".to_string(),
            contents: String::new(),
            date: 1_782_840_134,
            author: None,
            feedlabel: None,
            feedname: Some("steam_community_announcements".to_string()),
            feed_type: Some(1),
            appid: Some(1_422_450),
            is_external_url: None,
        };

        let events = parse_events(
            &item,
            "steam_appnews:1",
            "2026-06-30T17:22:14Z",
            "- King of the Hill objective has been rethemed and renamed to \"Unstable Rift\"",
            &index,
        );

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].entity_type, "objective");
        assert_eq!(events[0].entity_name.as_deref(), Some("Unstable Rift"));
        assert_eq!(events[0].change_type, "rename");
    }

    #[test]
    fn selects_official_items_old_to_new() {
        let official_new = test_item("2", "steam_community_announcements", 20);
        let official_old = test_item("1", "steam_community_announcements", 10);
        let article = test_item("3", "PC Gamer", 30);

        let selected = select_items(
            vec![official_new, article, official_old],
            NaiveDate::from_ymd_opt(1970, 1, 1).expect("date"),
            false,
            &[],
        )
        .expect("select");

        assert_eq!(
            selected
                .iter()
                .map(|item| item.gid.as_str())
                .collect::<Vec<_>>(),
            vec!["1", "2"]
        );
    }

    fn test_item(gid: &str, feedname: &str, date: i64) -> SteamNewsItem {
        SteamNewsItem {
            gid: gid.to_string(),
            title: format!("Patch {gid}"),
            url: format!("https://example.test/{feedname}/{gid}"),
            contents: "- Test".to_string(),
            date,
            author: None,
            feedlabel: None,
            feedname: Some(feedname.to_string()),
            feed_type: None,
            appid: None,
            is_external_url: None,
        }
    }
}
