use std::collections::{BTreeMap, HashMap};

use regex::Regex;
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::util::{
    clean_subject, clear_patch_events, normalize_key, normalize_scan_text, object_json_string,
    stable_hash_text, table_count, value_to_string,
};
use crate::Result;

const POSITIVE_STATS: &[&str] = &[
    "damage",
    "dps",
    "health",
    "regen",
    "resistance",
    "resist",
    "range",
    "radius",
    "speed",
    "sprint",
    "fire rate",
    "spirit power",
    "lifesteal",
    "duration",
    "stamina",
    "ammo",
    "barrier",
    "heal",
    "healing",
    "scaling",
    "souls",
    "bounty",
];
const NEGATIVE_STATS: &[&str] = &["cooldown", "recharge", "delay", "cost", "falloff"];
const GENERIC_INTERNAL_KEYS: &[&str] = &["melee"];

#[derive(Debug, Default)]
pub(crate) struct EntityIndex {
    hero_names: HashMap<String, String>,
    hero_internal_names: HashMap<String, String>,
    item_names: HashMap<String, String>,
    special_item_names: HashMap<String, String>,
    ability_names: HashMap<String, String>,
    internal_ability_names: HashMap<String, String>,
    internal_names: HashMap<String, String>,
}

impl EntityIndex {
    pub(crate) fn canonical(&self, name: &str, section: Option<&str>) -> (String, Option<String>, f64) {
        let cleaned = clean_subject(Some(name));
        let key = normalize_key(&cleaned);
        if key.is_empty() {
            return ("general".to_string(), None, 0.35);
        }
        if let Some(name) = self.hero_names.get(&key) {
            return ("hero".to_string(), Some(name.clone()), 0.95);
        }
        if let Some(name) = self.hero_internal_names.get(&key) {
            return ("hero_internal".to_string(), Some(name.clone()), 0.75);
        }
        if let Some(name) = self.item_names.get(&key) {
            return ("item".to_string(), Some(name.clone()), 0.92);
        }
        if let Some(name) = self.special_item_names.get(&key) {
            return ("item_special".to_string(), Some(name.clone()), 0.85);
        }
        if let Some(name) = self.ability_names.get(&key) {
            return ("ability".to_string(), Some(name.clone()), 0.9);
        }
        if let Some(name) = self.internal_ability_names.get(&key) {
            return ("ability_internal".to_string(), Some(name.clone()), 0.65);
        }
        if let Some(name) = self.internal_names.get(&key) {
            return ("weapon_or_internal".to_string(), Some(name.clone()), 0.7);
        }
        if section == Some("Heroes") {
            return ("hero".to_string(), Some(cleaned), 0.78);
        }
        if matches!(section, Some("Items") | Some("Street Brawl")) {
            return ("item".to_string(), Some(cleaned), 0.75);
        }
        ("general".to_string(), Some(cleaned), 0.45)
    }
}

#[derive(Debug)]
struct PatchSnapshot {
    id: i64,
    legacy_snapshot_id: i64,
    external_id: String,
    payload_json: String,
}

#[derive(Debug)]
struct PatchEventInsert {
    patch_snapshot_id: i64,
    legacy_patch_snapshot_id: i64,
    patch_external_id: String,
    patch_title: Option<String>,
    patch_url: Option<String>,
    source_kind: String,
    posted_at: Option<String>,
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
    metadata: BTreeMap<String, Value>,
    event_hash: String,
}

pub async fn parse_patchnotes(pool: &PgPool, rebuild: bool) -> Result<Value> {
    let before_total = table_count(pool, "patch_events").await?;
    let deleted = if rebuild { clear_patch_events(pool).await? } else { 0 };
    let index = build_entity_index(pool).await?;
    let rows = load_patch_snapshots(pool).await?;

    let mut inserted = 0_i64;
    let mut parsed_patches = 0_i64;
    let mut skipped_lines = 0_i64;
    let mut source_kinds: BTreeMap<String, i64> = BTreeMap::new();

    for row in rows {
        let payload: Value = serde_json::from_str(&row.payload_json)?;
        let (events, skipped) = parse_patchnote_snapshot(
            row.id,
            row.legacy_snapshot_id,
            &row.external_id,
            &payload,
            &index,
        )?;
        skipped_lines += skipped;
        let source_kind = classify_source_kind(optional_string(&payload, "url").as_deref());
        *source_kinds.entry(source_kind).or_default() += 1;
        for event in events {
            if insert_patch_event(pool, &event).await? {
                inserted += 1;
            }
        }
        parsed_patches += 1;
    }

    Ok(json!({
        "patches": parsed_patches,
        "events_inserted": inserted,
        "events_total": table_count(pool, "patch_events").await?,
        "events_before": before_total,
        "skipped_lines": skipped_lines,
        "deleted_before_parse": deleted,
        "source_kinds": source_kinds,
    }))
}

async fn load_patch_snapshots(pool: &PgPool) -> Result<Vec<PatchSnapshot>> {
    let rows = sqlx::query!(
        r#"
        SELECT id AS "id!",
               COALESCE(legacy_sqlite_id, id) AS "legacy_snapshot_id!",
               external_id AS "external_id!",
               payload::text AS "payload_json!"
        FROM brain.entity_snapshots
        WHERE entity_type = 'patchnote'
        ORDER BY id ASC
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|row| PatchSnapshot {
            id: row.id,
            legacy_snapshot_id: row.legacy_snapshot_id,
            external_id: row.external_id,
            payload_json: row.payload_json,
        })
        .collect())
}

fn parse_patchnote_snapshot(
    snapshot_id: i64,
    legacy_snapshot_id: i64,
    external_id: &str,
    payload: &Value,
    index: &EntityIndex,
) -> Result<(Vec<PatchEventInsert>, i64)> {
    let content = optional_string(payload, "raw_content")
        .or_else(|| optional_string(payload, "translated_content"))
        .unwrap_or_default();
    let title = optional_string(payload, "title");
    let url = optional_string(payload, "url");
    let posted_at = optional_string(payload, "posted_at");
    let patch_external_id = normalize_patch_external_id(external_id);
    let source_kind = classify_source_kind(url.as_deref());
    let mut section: Option<String> = None;
    let mut current_group: Option<String> = None;
    let mut current_group_type: Option<String> = None;
    let mut current_group_confidence = 0.0_f64;
    let mut events = Vec::new();
    let mut skipped = 0_i64;

    for (line_index, raw_line) in iter_patch_lines(&content) {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some(section_candidate) = detect_section(line) {
            section = Some(section_candidate);
            current_group = None;
            current_group_type = None;
            current_group_confidence = 0.0;
            continue;
        }

        let Some(bullet_body) = bullet_body_from_line(line) else {
            if let Some(maybe_group) = standalone_group_name(line) {
                let (entity_type, canonical, confidence) = index.canonical(&maybe_group, section.as_deref());
                current_group = canonical.or(Some(maybe_group));
                current_group_type = Some(entity_type);
                current_group_confidence = confidence;
            } else {
                skipped += 1;
            }
            continue;
        };

        let (subject, mut body) = split_subject(&bullet_body);
        let (entity_type, entity_name, confidence, event_subject) = if let Some(subject_text) = subject {
            let (entity_type, entity_name, confidence) = index.canonical(&subject_text, section.as_deref());
            if looks_like_group_heading(&body) {
                current_group = entity_name.clone().or(Some(clean_subject(Some(&subject_text))));
                current_group_type = Some(entity_type.clone());
                current_group_confidence = confidence;
                body = clean_subject(Some(&body));
            } else if entity_type != "general" {
                current_group = entity_name.clone();
            }
            (entity_type, entity_name, confidence, Some(subject_text))
        } else if let Some(group) = current_group.clone() {
            (
                current_group_type.clone().unwrap_or_else(|| "general".to_string()),
                Some(group.clone()),
                current_group_confidence.max(0.7),
                Some(group),
            )
        } else if let Some((entity_type, entity_name, confidence)) = infer_entities_from_line(&bullet_body, index).into_iter().next() {
            (
                entity_type,
                Some(entity_name.clone()),
                confidence,
                Some(entity_name),
            )
        } else {
            ("general".to_string(), None, 0.45, None)
        };

        let normalized_line = normalize_patch_line(&body);
        if normalized_line.is_empty() {
            skipped += 1;
            continue;
        }
        let change_type = classify_change_type(&normalized_line);
        let (old_value, new_value) = extract_old_new(&normalized_line)?;
        let mut metadata = BTreeMap::new();
        let source_language = if optional_string(payload, "raw_content").is_some() { "en" } else { "de" };
        metadata.insert("source_language".to_string(), json!(source_language));
        metadata.insert("line_subject".to_string(), event_subject.clone().map_or(Value::Null, Value::String));
        metadata.insert("original_bullet".to_string(), json!(bullet_body));
        let event_hash = stable_hash_text(&[
            legacy_snapshot_id.to_string(),
            line_index.to_string(),
            section.clone().unwrap_or_default(),
            entity_type.clone(),
            entity_name.clone().unwrap_or_default(),
            normalized_line.clone(),
        ].join("|"));

        events.push(PatchEventInsert {
            patch_snapshot_id: snapshot_id,
            legacy_patch_snapshot_id: legacy_snapshot_id,
            patch_external_id: patch_external_id.clone(),
            patch_title: title.clone(),
            patch_url: url.clone(),
            source_kind: source_kind.clone(),
            posted_at: posted_at.clone(),
            line_index,
            section: section.clone(),
            entity_type,
            entity_name,
            subject: event_subject.map(|subject| clean_subject(Some(&subject))),
            change_type,
            raw_line,
            normalized_line,
            old_value,
            new_value,
            confidence,
            metadata,
            event_hash,
        });
    }

    Ok((events, skipped))
}

fn iter_patch_lines(content: &str) -> Vec<(i64, String)> {
    let mut lines = Vec::new();
    let mut virtual_index = 0_i64;
    if let Some(compact_lines) = expand_compact_square_section_lines(content) {
        for raw_line in compact_lines {
            virtual_index += 1;
            lines.push((virtual_index, raw_line));
        }
        return lines;
    }
    for raw_line in content.lines() {
        for part in expand_inline_bullets(raw_line) {
            virtual_index += 1;
            lines.push((virtual_index, part));
        }
    }
    lines
}

fn expand_inline_bullets(raw_line: &str) -> Vec<String> {
    let stripped = raw_line.trim();
    if stripped.is_empty() {
        return vec![raw_line.to_string()];
    }
    if stripped.starts_with("- ") && stripped.contains(" - ") {
        let pieces = stripped[2..].split(" - ").collect::<Vec<_>>();
        if pieces.len() > 1
            && pieces
                .iter()
                .skip(1)
                .all(|part| part.chars().next().is_some_and(is_inline_bullet_start))
        {
            return pieces
                .into_iter()
                .filter_map(|part| {
                    let part = part.trim();
                    if part.is_empty() {
                        None
                    } else {
                        Some(format!("- {part}"))
                    }
                })
                .collect();
        }
    }
    vec![raw_line.to_string()]
}

fn is_inline_bullet_start(ch: char) -> bool {
    ch.is_ascii_uppercase() || ch.is_ascii_digit() || ch == '"' || ch == '\''
}

async fn build_entity_index(pool: &PgPool) -> Result<EntityIndex> {
    if let Some(index) = build_entity_index_from_entities(pool).await? {
        return Ok(index);
    }

    let mut index = EntityIndex::default();
    let rows = sqlx::query!(
        r#"
        SELECT entity_type AS "entity_type!",
               canonical_name AS "canonical_name?",
               payload::text AS "payload_json!"
        FROM brain.entity_snapshots
        WHERE entity_type IN ('hero', 'item_or_ability', 'hero_stats_sheet')
        "#,
    )
    .fetch_all(pool)
    .await?;
    for row in rows {
        let payload = serde_json::from_str::<Value>(&row.payload_json).unwrap_or(Value::Null);
        let mut names = Vec::new();
        if let Some(canonical_name) = row.canonical_name {
            names.push(canonical_name);
        }
        if let Some(object) = payload.as_object() {
            for key in ["name", "class_name"] {
                if let Some(value) = optional_value_to_string(object.get(key)) {
                    names.push(value);
                }
            }
            if let Some(values) = object.get("values").and_then(Value::as_object) {
                if let Some(name) = optional_value_to_string(values.get("Hero Name")) {
                    names.push(name);
                }
            }
        }
        for name in names {
            let key = normalize_key(&name);
            if key.is_empty() || key.chars().all(|ch| ch.is_ascii_digit()) {
                continue;
            }
            if row.entity_type == "hero" || row.entity_type == "hero_stats_sheet" {
                index.hero_names.entry(key).or_insert_with(|| clean_subject(Some(&name)));
            } else {
                index.item_names.entry(key).or_insert_with(|| clean_subject(Some(&name)));
            }
        }
    }
    Ok(index)
}

pub(crate) async fn build_entity_index_from_entities(pool: &PgPool) -> Result<Option<EntityIndex>> {
    let rows = sqlx::query!(
        r#"
        SELECT e.entity_type AS "entity_type!",
               e.canonical_name AS "canonical_name!",
               a.alias AS "alias?"
        FROM brain.entities e
        LEFT JOIN brain.entity_aliases a ON a.entity_id = e.id
        WHERE e.entity_type IN (
          'hero', 'hero_internal', 'item', 'item_special',
          'ability', 'ability_internal', 'weapon_or_internal'
        )
        "#,
    )
    .fetch_all(pool)
    .await?;

    let mut index = EntityIndex::default();
    let mut seen_any = false;
    for row in rows {
        seen_any = true;
        let canonical = clean_subject(Some(&row.canonical_name));
        let mut names = vec![canonical.clone()];
        if let Some(alias) = row.alias {
            names.push(alias);
        }
        for name in names {
            let key = normalize_key(&name);
            if key.is_empty() || key.chars().all(|ch| ch.is_ascii_digit()) {
                continue;
            }
            match row.entity_type.as_str() {
                "hero" => {
                    index.hero_names.entry(key).or_insert_with(|| canonical.clone());
                }
                "hero_internal" => {
                    index.hero_internal_names.entry(key).or_insert_with(|| canonical.clone());
                }
                "item" => {
                    index.item_names.entry(key).or_insert_with(|| canonical.clone());
                }
                "item_special" => {
                    index.special_item_names.entry(key).or_insert_with(|| canonical.clone());
                }
                "ability" => {
                    index.ability_names.entry(key).or_insert_with(|| canonical.clone());
                }
                "ability_internal" => {
                    if !GENERIC_INTERNAL_KEYS.contains(&key.as_str()) {
                        index.internal_ability_names.entry(key).or_insert_with(|| canonical.clone());
                    }
                }
                "weapon_or_internal" if !GENERIC_INTERNAL_KEYS.contains(&key.as_str()) => {
                    index.internal_names.entry(key).or_insert_with(|| canonical.clone());
                }
                _ => {}
            }
        }
    }

    if seen_any {
        Ok(Some(index))
    } else {
        Ok(None)
    }
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

fn normalize_patch_external_id(external_id: &str) -> String {
    let trimmed = external_id.trim();
    if !trimmed.is_empty() && trimmed.chars().all(|ch| ch.is_ascii_digit()) {
        format!("patch_{trimmed}")
    } else {
        trimmed.to_string()
    }
}

fn expand_compact_square_section_lines(content: &str) -> Option<Vec<String>> {
    let trimmed = content.trim();
    if trimmed.is_empty() || trimmed.contains('\n') || !trimmed.starts_with('[') {
        return None;
    }

    let mut markers = Vec::new();
    let mut search_from = 0usize;
    while let Some(marker) = find_square_section_marker(trimmed, search_from) {
        search_from = marker.1;
        markers.push(marker);
    }
    if markers.is_empty() {
        return None;
    }

    let mut lines = Vec::new();
    let mut saw_event = false;
    for (idx, (_, section_end, section)) in markers.iter().enumerate() {
        let body_end = markers.get(idx + 1).map_or(trimmed.len(), |next| next.0);
        let body = trimmed[*section_end..body_end].trim();
        lines.push(section.clone());
        for bullet in split_compact_square_section_bullets(body) {
            lines.push(format!("- {bullet}"));
            saw_event = true;
        }
    }

    if saw_event {
        Some(lines)
    } else {
        None
    }
}

fn find_square_section_marker(content: &str, start: usize) -> Option<(usize, usize, String)> {
    let search_from = start;
    let open_rel = content[search_from..].find('[')?;
    let open = search_from + open_rel;
    let close_rel = content[open..].find(']')?;
    let close = open + close_rel;
    let section = detect_section(content[open + 1..close].trim())?;
    Some((open, close + 1, section))
}

fn split_compact_square_section_bullets(body: &str) -> Vec<String> {
    let trimmed = body.trim();
    if trimmed.is_empty() {
        return Vec::new();
    }

    for (marker, prefix) in [('*', "* "), ('-', "- ")] {
        if let Some(rest) = trimmed.strip_prefix(prefix) {
            return split_compact_square_section_bullets_with_marker(rest, marker);
        }
        let delimiter = format!(" {marker} ");
        if trimmed.contains(&delimiter) {
            return split_compact_square_section_bullets_with_marker(trimmed, marker);
        }
    }

    vec![trimmed.to_string()]
}

fn split_compact_square_section_bullets_with_marker(body: &str, marker: char) -> Vec<String> {
    let delimiter = format!(" {marker} ");
    body.split(&delimiter)
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn detect_section(line: &str) -> Option<String> {
    let cleaned = line.trim();
    if cleaned.to_lowercase().starts_with("deadlock patch notes") {
        return None;
    }
    let mut text = cleaned.trim_start_matches('#').trim().trim_end_matches(':').trim();
    if let Some(stripped) = text.strip_prefix('[') {
        text = stripped.trim();
        if let Some(stripped) = text.strip_suffix(']') {
            text = stripped.trim();
        }
    }
    if text.len() < 2 || text.len() > 60 {
        return None;
    }
    if !text.chars().next().is_some_and(|ch| ch.is_ascii_alphabetic()) {
        return None;
    }
    if !text
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, ' ' | '&' | '/' | '+' | '-'))
    {
        return None;
    }
    let label = clean_subject(Some(text));
    match label.to_lowercase().as_str() {
        "general" => Some("General".to_string()),
        "items" | "item" => Some("Items".to_string()),
        "heroes" | "hero" => Some("Heroes".to_string()),
        "map" => Some("Map".to_string()),
        "ui" => Some("UI".to_string()),
        "audio" => Some("Audio".to_string()),
        "misc" => Some("Misc".to_string()),
        "street brawl" => Some("Street Brawl".to_string()),
        _ => None,
    }
}

fn bullet_body_from_line(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if let Some(rest) = trimmed.strip_prefix('-').or_else(|| trimmed.strip_prefix('*')) {
        if rest.chars().next().is_some_and(char::is_whitespace) {
            return Some(rest.trim().to_string());
        }
    }
    let digits_len = trimmed.chars().take_while(|ch| ch.is_ascii_digit()).count();
    if digits_len > 0 {
        let rest = &trimmed[digits_len..];
        if (rest.starts_with(". ") || rest.starts_with(") ")) && rest.len() > 2 {
            return Some(rest[2..].trim().to_string());
        }
    }
    None
}

fn split_subject(text: &str) -> (Option<String>, String) {
    let Some(index) = text.find(':') else {
        return (None, text.trim().to_string());
    };
    let subject = clean_subject(Some(&text[..index]));
    if subject.len() < 2 || subject.len() > 80 {
        return (None, text.trim().to_string());
    }
    let body = text[index + 1..].trim().to_string();
    if matches!(
        subject.to_lowercase().as_str(),
        "t1" | "t2" | "t3" | "t4" | "tier 1" | "tier 2" | "tier 3" | "tier 4"
    ) {
        return (None, text.trim().to_string());
    }
    (Some(subject), body)
}

fn standalone_group_name(line: &str) -> Option<String> {
    let cleaned = clean_subject(Some(line));
    if cleaned.is_empty() || cleaned.len() > 70 {
        return None;
    }
    if cleaned.starts_with('#') || cleaned.starts_with('-') {
        return None;
    }
    let lower = cleaned.to_lowercase();
    if ["increased", "reduced", "fixed", "added", "removed"]
        .iter()
        .any(|word| lower.contains(word))
    {
        return None;
    }
    Some(cleaned)
}

fn looks_like_group_heading(text: &str) -> bool {
    let cleaned = text.trim().trim_end_matches(':');
    !cleaned.is_empty()
        && cleaned.len() < 50
        && !["from", "to", "increased", "reduced", "fixed", "now", "no longer"]
            .iter()
            .any(|word| cleaned.to_lowercase().contains(word))
}

fn infer_entities_from_line(text: &str, index: &EntityIndex) -> Vec<(String, String, f64)> {
    let mut found: Vec<(usize, String, String, f64)> = Vec::new();
    let lower = normalize_scan_text(text);
    push_inferred_entities(&mut found, &lower, &index.hero_names, 1, "hero", 0.65);
    push_inferred_entities(
        &mut found,
        &lower,
        &index.hero_internal_names,
        1,
        "hero_internal",
        0.5,
    );
    push_inferred_entities(&mut found, &lower, &index.item_names, 4, "item", 0.6);
    push_inferred_entities(
        &mut found,
        &lower,
        &index.special_item_names,
        4,
        "item_special",
        0.55,
    );
    push_inferred_entities(&mut found, &lower, &index.ability_names, 4, "ability", 0.58);
    push_inferred_entities(
        &mut found,
        &lower,
        &index.internal_ability_names,
        4,
        "ability_internal",
        0.45,
    );
    found.sort_by(|left, right| {
        left.0
            .cmp(&right.0)
            .then_with(|| left.2.cmp(&right.2))
            .then_with(|| left.1.cmp(&right.1))
    });
    found.truncate(5);
    found
        .into_iter()
        .map(|(_, entity_type, entity_name, confidence)| (entity_type, entity_name, confidence))
        .collect()
}

fn push_inferred_entities(
    found: &mut Vec<(usize, String, String, f64)>,
    lower: &str,
    entries: &HashMap<String, String>,
    min_key_len: usize,
    entity_type: &str,
    confidence: f64,
) {
    for (key, name) in entries {
        if key.len() < min_key_len {
            continue;
        }
        let needle = format!(" {key} ");
        if let Some(position) = lower.find(&needle) {
            found.push((position, entity_type.to_string(), name.clone(), confidence));
        }
    }
}

fn classify_change_type(text: &str) -> String {
    let lower = text.to_lowercase();
    if lower.contains("fixed") || lower.contains("bug") || lower.contains("crash") {
        "bugfix"
    } else if lower.contains("added") || lower.contains("new item") || lower.contains("new hero") {
        "added"
    } else if lower.contains("removed") || lower.contains("no longer") {
        "removed"
    } else if lower.contains("reworked")
        || lower.contains("changed from")
        || lower.contains("rescaled")
        || lower.contains("moved from")
    {
        "rework"
    } else if lower.contains("reduced") || lower.contains("decreased") || lower.contains("slower") {
        if mentions_positive_stat(&lower) { "nerf" } else { "buff" }
    } else if lower.contains("increased") || lower.contains("faster") || lower.contains("improved") {
        if mentions_positive_stat(&lower) || !mentions_negative_stat(&lower) {
            "buff"
        } else {
            "nerf"
        }
    } else {
        "changed"
    }
    .to_string()
}

fn mentions_positive_stat(lower: &str) -> bool {
    POSITIVE_STATS.iter().any(|stat| lower.contains(stat))
}

fn mentions_negative_stat(lower: &str) -> bool {
    NEGATIVE_STATS.iter().any(|stat| lower.contains(stat))
}

fn extract_old_new(text: &str) -> Result<(Option<String>, Option<String>)> {
    let quoted = Regex::new(r#"(?i)from\s+"([^"]+)"\s+to\s+"([^"]+)""#)?;
    if let Some(captures) = quoted.captures(text) {
        return Ok((
            captures.get(1).map(|value| value.as_str().trim().to_string()),
            captures.get(2).map(|value| value.as_str().trim().to_string()),
        ));
    }
    let from_to = Regex::new(r"(?i)\bfrom\s+(.+?)\s+to\s+(.+?)(?:[.;,)]|$)")?;
    if let Some(captures) = from_to.captures(text) {
        let old_value = captures.get(1).map(|value| truncate(value.as_str().trim(), 160));
        let new_value = captures.get(2).map(|value| truncate(value.as_str().trim(), 160));
        return Ok((old_value, new_value));
    }
    Ok((None, None))
}

fn normalize_patch_line(text: &str) -> String {
    clean_subject(Some(text)).split_whitespace().collect::<Vec<_>>().join(" ")
}

fn truncate(value: &str, limit: usize) -> String {
    value.chars().take(limit).collect()
}

async fn insert_patch_event(pool: &PgPool, event: &PatchEventInsert) -> Result<bool> {
    let metadata_json = object_json_string(event.metadata.clone())?;
    let result = sqlx::query!(
        r#"
        INSERT INTO brain.patch_events(
          patch_snapshot_id, legacy_patch_snapshot_id, patch_external_id, patch_title, patch_url,
          source_kind, posted_at, line_index, section, entity_type,
          entity_name, subject, change_type, raw_line, normalized_line,
          old_value, new_value, confidence, metadata, event_hash, created_at
        )
        VALUES(
          $1, $2, $3, $4, $5,
          $6, CASE WHEN $7 ~ '^-?[0-9]+$' THEN to_timestamp($7::double precision) ELSE $7::text::timestamptz END, $8, $9, $10,
          $11, $12, $13, $14, $15,
          $16, $17, $18, $19::text::jsonb, $20, now()
        )
        ON CONFLICT (event_hash) DO NOTHING
        "#,
        event.patch_snapshot_id,
        event.legacy_patch_snapshot_id,
        event.patch_external_id,
        event.patch_title,
        event.patch_url,
        event.source_kind,
        event.posted_at,
        event.line_index,
        event.section,
        event.entity_type,
        event.entity_name,
        event.subject,
        event.change_type,
        event.raw_line,
        event.normalized_line,
        event.old_value,
        event.new_value,
        event.confidence,
        metadata_json,
        event.event_hash,
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

fn optional_string(payload: &Value, key: &str) -> Option<String> {
    optional_value_to_string(payload.as_object().and_then(|object| object.get(key)))
}

fn optional_value_to_string(value: Option<&Value>) -> Option<String> {
    let text = value_to_string(value).trim().to_string();
    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}
