use std::collections::{BTreeMap, HashMap, HashSet};

use regex::Regex;
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::patch_identity::{canonical_snapshot_ids, SnapshotIdentity};
use crate::patch_reconcile::{
    apply_plan, insert_events, load_stored_events, plan_reconcile, retire_snapshot_events,
    ReconcilePlan,
};
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
const OBJECTIVE_ALIASES: &[(&str, &str, &[&str])] = &[
    (
        "objective",
        "Unstable Rift",
        &["Unstable Rift", "King of the Hill", "KOTH"],
    ),
    ("objective", "Urn", &["Urn", "Urns", "Soul Urn"]),
    (
        "objective_entity",
        "Rift Troopers",
        &["Rift Trooper", "Rift Troopers"],
    ),
    ("objective", "Mid Boss", &["Mid Boss", "Midboss"]),
    ("objective", "Patron", &["Patron", "Weakened Patron"]),
    ("objective", "Walker", &["Walker", "Walkers"]),
    ("objective", "Guardian", &["Guardian", "Guardians"]),
];

#[derive(Debug, Default)]
pub(crate) struct EntityIndex {
    hero_names: HashMap<String, String>,
    hero_internal_names: HashMap<String, String>,
    item_names: HashMap<String, String>,
    special_item_names: HashMap<String, String>,
    ability_names: HashMap<String, String>,
    internal_ability_names: HashMap<String, String>,
    internal_names: HashMap<String, String>,
    objective_names: HashMap<String, (String, String)>,
}

impl EntityIndex {
    pub(crate) fn canonical(
        &self,
        name: &str,
        section: Option<&str>,
    ) -> (String, Option<String>, f64) {
        let cleaned = clean_subject(Some(name));
        if normalize_key(&cleaned).is_empty() {
            return ("general".to_string(), None, 0.35);
        }
        if let Some((entity_type, name, confidence)) = self.known(&cleaned) {
            return (entity_type, Some(name), confidence);
        }
        if section == Some("Heroes") {
            return ("hero".to_string(), Some(cleaned), 0.78);
        }
        if matches!(section, Some("Items") | Some("Street Brawl")) {
            return ("item".to_string(), Some(cleaned), 0.75);
        }
        ("general".to_string(), Some(cleaned), 0.45)
    }

    pub(crate) fn known(&self, name: &str) -> Option<(String, String, f64)> {
        let key = normalize_key(name);
        if key.is_empty() {
            return None;
        }
        [
            (&self.hero_names, "hero", 0.95),
            (&self.hero_internal_names, "hero_internal", 0.75),
            (&self.item_names, "item", 0.92),
            (&self.special_item_names, "item_special", 0.85),
            (&self.ability_names, "ability", 0.9),
            (&self.internal_ability_names, "ability_internal", 0.65),
            (&self.internal_names, "weapon_or_internal", 0.7),
        ]
        .into_iter()
        .find_map(|(names, entity_type, confidence)| {
            names
                .get(&key)
                .map(|name| (entity_type.to_string(), name.clone(), confidence))
        })
        .or_else(|| {
            self.objective_names
                .get(&key)
                .map(|(entity_type, name)| (entity_type.clone(), name.clone(), 0.85))
        })
    }

    fn with_objectives(mut self) -> Self {
        for (entity_type, canonical, aliases) in OBJECTIVE_ALIASES {
            for alias in *aliases {
                self.objective_names.insert(
                    normalize_key(alias),
                    (entity_type.to_string(), canonical.to_string()),
                );
            }
        }
        self
    }
}

#[derive(Debug)]
struct PatchSnapshot {
    id: i64,
    legacy_snapshot_id: i64,
    external_id: String,
    payload_json: String,
}

#[derive(Debug, Clone)]
pub(crate) struct PatchEventInsert {
    pub(crate) patch_snapshot_id: i64,
    pub(crate) legacy_patch_snapshot_id: i64,
    pub(crate) patch_external_id: String,
    pub(crate) patch_title: Option<String>,
    pub(crate) patch_url: Option<String>,
    pub(crate) source_kind: String,
    pub(crate) posted_at: Option<String>,
    pub(crate) line_index: i64,
    pub(crate) section: Option<String>,
    pub(crate) entity_type: String,
    pub(crate) entity_name: Option<String>,
    pub(crate) subject: Option<String>,
    pub(crate) change_type: String,
    pub(crate) raw_line: String,
    pub(crate) normalized_line: String,
    pub(crate) old_value: Option<String>,
    pub(crate) new_value: Option<String>,
    pub(crate) confidence: f64,
    pub(crate) metadata: BTreeMap<String, Value>,
    pub(crate) event_hash: String,
}

pub async fn parse_patchnotes(pool: &PgPool, rebuild: bool, dry_run: bool) -> Result<Value> {
    let before_total = table_count(pool, "patch_events").await?;
    let deleted = if rebuild && !dry_run {
        clear_patch_events(pool).await?
    } else {
        0
    };
    let index = build_entity_index(pool).await?;
    let parsed = parse_all_snapshots(pool, &index).await?;
    let canonical = canonical_snapshot_ids(
        &parsed
            .iter()
            .map(ParsedSnapshot::identity)
            .collect::<Vec<_>>(),
    );
    let selected = select_cluster_events(&parsed, &canonical);
    let mut stored = load_stored_events(pool).await?;
    let mut summary = ReconcileSummary::default();
    for snapshot in &parsed {
        let rows = stored.remove(&snapshot.id).unwrap_or_default();
        let is_canonical = canonical.get(&snapshot.id) == Some(&snapshot.id);
        let events = selected.get(&snapshot.id).map(Vec::as_slice).unwrap_or(&[]);
        summary.add_skipped(snapshot, is_canonical);
        summary.snapshots.push(json!([
            snapshot.id,
            canonical.get(&snapshot.id),
            snapshot.title,
            snapshot.posted_at,
            snapshot.importer,
            snapshot.events.len(),
            events.len(),
            rows.len(),
        ]));
        if events.is_empty() {
            summary.retired_snapshots += 1;
            summary.deleted += rows.len();
            if !dry_run {
                retire_snapshot_events(pool, &rows).await?;
            }
            continue;
        }
        let plan = plan_reconcile(&rows, events);
        summary.add_plan(&plan, events);
        if !dry_run {
            apply_plan(pool, &plan, events).await?;
        }
    }
    let canonical_patches = canonical.iter().filter(|(id, root)| id == root).count();
    Ok(summary.into_json(json!({
        "dry_run": dry_run,
        "patches": parsed.len(),
        "canonical_patches": canonical_patches,
        "events_before": before_total,
        "events_total": table_count(pool, "patch_events").await?,
        "deleted_before_parse": deleted,
    })))
}

pub(crate) async fn append_unparsed_patches(
    pool: &PgPool,
    staged: &[(String, Value)],
    dry_run: bool,
) -> Result<Value> {
    let index = build_entity_index(pool).await?;
    let mut parsed = parse_all_snapshots(pool, &index).await?;
    for (position, (external_id, payload)) in staged.iter().enumerate() {
        let id = -1 - position as i64;
        parsed.push(parse_snapshot(id, id, external_id, payload, &index)?);
    }
    let canonical = canonical_snapshot_ids(
        &parsed
            .iter()
            .map(ParsedSnapshot::identity)
            .collect::<Vec<_>>(),
    );
    let stored = load_stored_events(pool).await?;
    let stored_roots = parsed
        .iter()
        .filter(|snapshot| {
            stored
                .get(&snapshot.id)
                .is_some_and(|rows| !rows.is_empty())
        })
        .filter_map(|snapshot| canonical.get(&snapshot.id).copied())
        .collect::<HashSet<_>>();
    let selected = select_cluster_events(&parsed, &canonical);
    let pending = parsed
        .iter()
        .filter(|snapshot| {
            canonical
                .get(&snapshot.id)
                .is_some_and(|root| !stored_roots.contains(root))
        })
        .flat_map(|snapshot| selected.get(&snapshot.id).into_iter().flatten())
        .collect::<Vec<_>>();
    let patches = pending
        .iter()
        .filter_map(|event| event.patch_title.clone())
        .collect::<std::collections::BTreeSet<_>>();
    if !dry_run && !pending.is_empty() {
        insert_events(pool, &pending).await?;
    }
    Ok(json!({
        "dry_run": dry_run,
        "patches": patches,
        "events_inserted": pending.len(),
    }))
}

fn select_cluster_events(
    parsed: &[ParsedSnapshot],
    canonical: &HashMap<i64, i64>,
) -> HashMap<i64, Vec<PatchEventInsert>> {
    let by_id = parsed
        .iter()
        .map(|snapshot| (snapshot.id, snapshot))
        .collect::<HashMap<_, _>>();
    let mut order = parsed.iter().collect::<Vec<_>>();
    order.sort_by_key(|snapshot| {
        (
            canonical.get(&snapshot.id) != Some(&snapshot.id),
            std::cmp::Reverse(snapshot.events.len()),
            snapshot.id,
        )
    });
    let mut covered: HashMap<i64, HashSet<String>> = HashMap::new();
    let mut selected = HashMap::new();
    for snapshot in order {
        let root_id = canonical.get(&snapshot.id).copied().unwrap_or(snapshot.id);
        let root = by_id.get(&root_id).copied().unwrap_or(snapshot);
        let seen = covered.entry(root_id).or_default();
        let events = snapshot
            .events
            .iter()
            .filter(|event| seen.insert(normalize_key(&event.normalized_line)))
            .map(|event| relabel_for_patch(event, root))
            .collect::<Vec<_>>();
        selected.insert(snapshot.id, events);
    }
    selected
}

fn relabel_for_patch(event: &PatchEventInsert, root: &ParsedSnapshot) -> PatchEventInsert {
    let mut event = event.clone();
    if event.patch_snapshot_id != root.id {
        event.patch_title = root.title.clone();
        event.patch_url = root.url.clone();
        event.posted_at = root.posted_at.clone();
    }
    event
}

struct ParsedSnapshot {
    id: i64,
    title: Option<String>,
    url: Option<String>,
    posted_at: Option<String>,
    content: String,
    importer: Option<String>,
    source_kind: String,
    events: Vec<PatchEventInsert>,
    skipped: i64,
}

impl ParsedSnapshot {
    fn identity(&self) -> SnapshotIdentity {
        SnapshotIdentity::new(
            self.id,
            self.title.as_deref(),
            self.url.as_deref(),
            self.posted_at.as_deref(),
            &self.content,
            self.events
                .iter()
                .map(|event| normalize_key(&event.normalized_line))
                .filter(|key| !key.is_empty())
                .collect(),
        )
    }
}

async fn parse_all_snapshots(pool: &PgPool, index: &EntityIndex) -> Result<Vec<ParsedSnapshot>> {
    let mut parsed = Vec::new();
    for row in load_patch_snapshots(pool).await? {
        let payload: Value = serde_json::from_str(&row.payload_json)?;
        parsed.push(parse_snapshot(
            row.id,
            row.legacy_snapshot_id,
            &row.external_id,
            &payload,
            index,
        )?);
    }
    Ok(parsed)
}

fn parse_snapshot(
    id: i64,
    legacy_snapshot_id: i64,
    external_id: &str,
    payload: &Value,
    index: &EntityIndex,
) -> Result<ParsedSnapshot> {
    let (mut events, skipped) =
        parse_patchnote_snapshot(id, legacy_snapshot_id, external_id, payload, index)?;
    let mut seen = HashSet::new();
    events.retain(|event| seen.insert(event.event_hash.clone()));
    let title = optional_string(payload, "title");
    let url = optional_string(payload, "url");
    Ok(ParsedSnapshot {
        id,
        posted_at: optional_string(payload, "posted_at")
            .or_else(|| title_date_rfc3339(title.as_deref())),
        title,
        source_kind: classify_source_kind(url.as_deref()),
        url,
        content: optional_string(payload, "raw_content")
            .or_else(|| optional_string(payload, "translated_content"))
            .unwrap_or_default(),
        importer: optional_string(payload, "importer"),
        events,
        skipped,
    })
}

#[derive(Default)]
struct ReconcileSummary {
    retired_snapshots: usize,
    unchanged: usize,
    updated: usize,
    inserted: usize,
    deleted: usize,
    skipped_lines: i64,
    source_kinds: BTreeMap<String, i64>,
    snapshots: Vec<Value>,
    planned: Vec<Value>,
}

impl ReconcileSummary {
    fn add_skipped(&mut self, snapshot: &ParsedSnapshot, is_canonical: bool) {
        self.skipped_lines += snapshot.skipped;
        if is_canonical {
            *self
                .source_kinds
                .entry(snapshot.source_kind.clone())
                .or_default() += 1;
        }
    }

    fn add_plan(&mut self, plan: &ReconcilePlan, events: &[PatchEventInsert]) {
        self.unchanged += plan.unchanged;
        self.updated += plan.updates.len();
        self.inserted += plan.inserts.len();
        self.deleted += plan.deletes.len();
        self.planned.extend(events.iter().map(|event| {
            json!([
                event.patch_title,
                event.posted_at,
                event.entity_type,
                event.entity_name,
                event.change_type,
                event.normalized_line,
                event.raw_line,
            ])
        }));
    }

    fn into_json(self, mut base: Value) -> Value {
        let Some(object) = base.as_object_mut() else {
            return base;
        };
        let dry_run = object
            .get("dry_run")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        object.insert("retired_snapshots".into(), json!(self.retired_snapshots));
        object.insert("events_unchanged".into(), json!(self.unchanged));
        object.insert("events_updated".into(), json!(self.updated));
        object.insert("events_inserted".into(), json!(self.inserted));
        object.insert("events_deleted".into(), json!(self.deleted));
        object.insert("skipped_lines".into(), json!(self.skipped_lines));
        object.insert("source_kinds".into(), json!(self.source_kinds));
        if dry_run {
            object.insert("snapshots".into(), Value::Array(self.snapshots));
            object.insert("planned_events".into(), Value::Array(self.planned));
        }
        base
    }
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
    let posted_at =
        optional_string(payload, "posted_at").or_else(|| title_date_rfc3339(title.as_deref()));
    let patch_external_id = normalize_patch_external_id(external_id);
    let source_kind = classify_source_kind(url.as_deref());
    let mut section: Option<String> = None;
    let mut current_group: Option<(String, String, f64)> = None;
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
            continue;
        }

        if is_media_or_link_line(line) {
            skipped += 1;
            continue;
        }

        let Some(bullet_body) = bullet_body_from_line(line).or_else(|| {
            (has_change_action(line) && index.canonical(line, None).0 == "general")
                .then(|| line.to_string())
        }) else {
            current_group = standalone_group_name(line).and_then(|name| index.known(&name));
            if current_group.is_none() {
                skipped += 1;
            }
            continue;
        };

        let (subject, mut body) = split_subject(&bullet_body);
        let (entity_type, entity_name, confidence, event_subject) = if let Some(subject_text) =
            subject
        {
            let (entity_type, mut entity_name, confidence) =
                index.canonical(&subject_text, section.as_deref());
            if entity_type == "general" {
                entity_name = None;
            }
            if looks_like_group_heading(&body) && !has_change_action(&body) {
                current_group = index.known(&subject_text);
                body = clean_subject(Some(&body));
            } else if current_group.as_ref().map(|group| group.1.as_str()) != entity_name.as_deref()
            {
                current_group = None;
            }
            (entity_type, entity_name, confidence, Some(subject_text))
        } else if let Some((group_type, group_name, group_confidence)) = current_group.clone() {
            (
                group_type,
                Some(group_name.clone()),
                group_confidence.max(0.7),
                Some(group_name),
            )
        } else if let Some((entity_type, entity_name, confidence)) =
            infer_entities_from_line(&bullet_body, index)
                .into_iter()
                .next()
        {
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
        let source_language = if optional_string(payload, "raw_content").is_some() {
            "en"
        } else {
            "de"
        };
        metadata.insert("source_language".to_string(), json!(source_language));
        metadata.insert(
            "line_subject".to_string(),
            event_subject.clone().map_or(Value::Null, Value::String),
        );
        metadata.insert("original_bullet".to_string(), json!(bullet_body));
        let event_hash = stable_hash_text(
            &[
                legacy_snapshot_id.to_string(),
                line_index.to_string(),
                section.clone().unwrap_or_default(),
                entity_type.clone(),
                entity_name.clone().unwrap_or_default(),
                normalized_line.clone(),
            ]
            .join("|"),
        );

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
    if stripped.starts_with("- ") && split_subject(&stripped[2..]).0.is_some() {
        let mut grouped = Vec::<String>::new();
        for part in stripped[2..]
            .split(" - ")
            .map(str::trim)
            .filter(|part| !part.is_empty())
        {
            if split_subject(part).0.is_none() {
                if let Some(current) = grouped.last_mut().filter(|current| {
                    !has_change_action(current)
                        || !part.chars().next().is_some_and(is_inline_bullet_start)
                }) {
                    current.push_str(" - ");
                    current.push_str(part);
                    continue;
                }
            }
            grouped.push(part.to_string());
        }
        return grouped
            .into_iter()
            .map(|part| format!("- {part}"))
            .collect();
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
        return Ok(index.with_objectives());
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
                index
                    .hero_names
                    .entry(key)
                    .or_insert_with(|| clean_subject(Some(&name)));
            } else {
                index
                    .item_names
                    .entry(key)
                    .or_insert_with(|| clean_subject(Some(&name)));
            }
        }
    }
    Ok(index.with_objectives())
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
                    index
                        .hero_names
                        .entry(key)
                        .or_insert_with(|| canonical.clone());
                }
                "hero_internal" => {
                    index
                        .hero_internal_names
                        .entry(key)
                        .or_insert_with(|| canonical.clone());
                }
                "item" => {
                    index
                        .item_names
                        .entry(key)
                        .or_insert_with(|| canonical.clone());
                }
                "item_special" => {
                    index
                        .special_item_names
                        .entry(key)
                        .or_insert_with(|| canonical.clone());
                }
                "ability" => {
                    index
                        .ability_names
                        .entry(key)
                        .or_insert_with(|| canonical.clone());
                }
                "ability_internal" => {
                    if !GENERIC_INTERNAL_KEYS.contains(&key.as_str()) {
                        index
                            .internal_ability_names
                            .entry(key)
                            .or_insert_with(|| canonical.clone());
                    }
                }
                "weapon_or_internal" if !GENERIC_INTERNAL_KEYS.contains(&key.as_str()) => {
                    index
                        .internal_names
                        .entry(key)
                        .or_insert_with(|| canonical.clone());
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
    let mut text = cleaned
        .trim_start_matches('#')
        .trim()
        .trim_end_matches(':')
        .trim();
    if let Some(stripped) = text.strip_prefix('[') {
        text = stripped.trim();
        if let Some(stripped) = text.strip_suffix(']') {
            text = stripped.trim();
        }
    }
    if text.len() < 2 || text.len() > 60 {
        return None;
    }
    if !text
        .chars()
        .next()
        .is_some_and(|ch| ch.is_ascii_alphabetic())
    {
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
        "new heroes" => Some("New Heroes".to_string()),
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
    if let Some(rest) = trimmed
        .strip_prefix('-')
        .or_else(|| trimmed.strip_prefix('*'))
    {
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

fn is_media_or_link_line(line: &str) -> bool {
    let text = bullet_body_from_line(line).unwrap_or_else(|| line.to_string());
    let lower = clean_subject(Some(&text)).to_lowercase();
    if lower.starts_with("view attachment") || lower.starts_with("{steam_clan_image}") {
        return true;
    }
    if lower.is_empty() || lower.contains(char::is_whitespace) {
        return false;
    }
    if lower.starts_with("http://") || lower.starts_with("https://") || lower.starts_with("www.") {
        return true;
    }
    let Some((stem, extension)) = lower.rsplit_once('.') else {
        return false;
    };
    const MEDIA_EXTENSIONS: &[&str] = &["png", "jpg", "jpeg", "gif", "webp", "mp4", "webm", "mov"];
    MEDIA_EXTENSIONS.contains(&extension)
        || (stem.contains(|ch: char| ch.is_ascii_alphabetic())
            && (2..=6).contains(&extension.len())
            && extension.chars().all(|ch| ch.is_ascii_alphabetic()))
}

fn looks_like_group_heading(text: &str) -> bool {
    let cleaned = text.trim().trim_end_matches(':');
    !cleaned.is_empty()
        && cleaned.len() < 50
        && ![
            "from",
            "to",
            "increased",
            "reduced",
            "fixed",
            "now",
            "no longer",
        ]
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
    for (key, (entity_type, name)) in &index.objective_names {
        if let Some(position) = lower.find(&format!(" {key} ")) {
            found.push((position, entity_type.clone(), name.clone(), 0.55));
        }
    }
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

pub fn has_change_action(text: &str) -> bool {
    let (_, remainder) = split_subject(text);
    if matches!(
        classify_change_type(&remainder).as_str(),
        "buff" | "nerf" | "bugfix" | "added" | "removed" | "rework" | "rename"
    ) {
        return true;
    }
    let remainder = remainder.to_ascii_lowercase();
    let change = remainder
        .rsplit_once(" - ")
        .map_or(remainder.as_str(), |(_, change)| change)
        .trim();
    remainder.contains("no longer ")
        || remainder
            .split_whitespace()
            .zip(remainder.split_whitespace().skip(1))
            .any(|(word, _)| matches!(word, "now" | "adjusted" | "changed" | "updated"))
        || change.split_once(" to ").is_some_and(|(old, new)| {
            old.chars().any(|character| character.is_ascii_digit())
                && new.chars().any(|character| character.is_ascii_digit())
        })
}

pub fn classify_change_type(text: &str) -> String {
    let lower = text.to_lowercase();
    if lower.contains("renamed") || lower.contains("retitled") {
        "rename"
    } else if lower.contains("fixed")
        || lower.contains("fix ")
        || lower.starts_with("fix")
        || lower.contains("bug")
        || lower.contains("crash")
    {
        "bugfix"
    } else if lower.contains("added") || lower.contains("new ") || lower.starts_with("new") {
        "added"
    } else if lower.contains("removed") {
        "removed"
    } else if lower.contains("reworked")
        || lower.contains("rework")
        || lower.contains("rescaled")
        || lower.contains("redesigned")
        || lower.contains("moved from")
    {
        "rework"
    } else if let Some(change_type) = numeric_change_type(text) {
        change_type
    } else if lower.contains("reduced")
        || lower.contains("decreased")
        || lower.contains("lower")
        || lower.contains("less ")
        || lower.contains("slower")
    {
        if mentions_positive_stat(&lower) {
            "nerf"
        } else {
            "buff"
        }
    } else if lower.contains("increased")
        || lower.contains("faster")
        || lower.contains("improved")
        || lower.contains("higher")
        || lower.contains("more ")
    {
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

fn numeric_change_type(text: &str) -> Option<&'static str> {
    let (old_value, new_value) = extract_old_new(text).ok()?;
    numeric_change_type_from_values(text, old_value.as_deref()?, new_value.as_deref()?)
}

fn numeric_change_type_from_values(
    text: &str,
    old_value: &str,
    new_value: &str,
) -> Option<&'static str> {
    let old_number = parse_numeric_value(old_value)?;
    let new_number = parse_numeric_value(new_value)?;
    let lower = text.to_lowercase();
    let negative_polarity = mentions_negative_stat(&lower) && !mentions_positive_stat(&lower);
    if new_number > old_number {
        Some(if negative_polarity { "nerf" } else { "buff" })
    } else if new_number < old_number {
        Some(if negative_polarity { "buff" } else { "nerf" })
    } else {
        None
    }
}

fn parse_numeric_value(value: &str) -> Option<f64> {
    let value = value.trim().trim_matches('"').trim();
    let mut end = 0;
    let mut seen_digit = false;
    let mut seen_dot = false;
    for (index, ch) in value.char_indices() {
        if index == 0 && matches!(ch, '+' | '-') {
            end = ch.len_utf8();
        } else if ch.is_ascii_digit() {
            seen_digit = true;
            end = index + ch.len_utf8();
        } else if ch == '.' && !seen_dot {
            seen_dot = true;
            end = index + ch.len_utf8();
        } else {
            break;
        }
    }
    if !seen_digit {
        return None;
    }
    let suffix = value[end..].trim();
    if !suffix
        .chars()
        .all(|ch| ch.is_ascii_alphabetic() || matches!(ch, '%' | '/'))
    {
        return None;
    }
    value[..end].parse().ok()
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
            captures
                .get(1)
                .map(|value| value.as_str().trim().to_string()),
            captures
                .get(2)
                .map(|value| value.as_str().trim().to_string()),
        ));
    }
    let from_to = Regex::new(r"(?i)\bfrom\s+(.+?)\s+to\s+(.+?)(?:[;,)]|\.?$)")?;
    if let Some(captures) = from_to.captures(text) {
        let old_value = captures
            .get(1)
            .map(|value| truncate(value.as_str().trim(), 160));
        let new_value = captures
            .get(2)
            .map(|value| truncate(value.as_str().trim(), 160));
        return Ok((old_value, new_value));
    }
    let action_to = Regex::new(
        r"(?i)\b(?:increased|decreased|reduced|raised|lowered)\s+([+-]?(?:\d+(?:\.\d+)?|\.\d+)\s*[[:alpha:]%/]*)\s+to\s+([+-]?(?:\d+(?:\.\d+)?|\.\d+)\s*[[:alpha:]%/]*)(?:[.;,)]|$)",
    )?;
    if let Some(captures) = action_to.captures(text) {
        let old_value = captures
            .get(1)
            .map(|value| truncate(value.as_str().trim(), 160));
        let new_value = captures
            .get(2)
            .map(|value| truncate(value.as_str().trim(), 160));
        return Ok((old_value, new_value));
    }
    Ok((None, None))
}

fn normalize_patch_line(text: &str) -> String {
    clean_subject(Some(text))
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn truncate(value: &str, limit: usize) -> String {
    value.chars().take(limit).collect()
}

fn title_date_rfc3339(title: Option<&str>) -> Option<String> {
    title
        .and_then(extract_mm_dd_yyyy)
        .map(|(month, day, year)| format!("{year:04}-{month:02}-{day:02}T00:00:00+00:00"))
}

fn extract_mm_dd_yyyy(value: &str) -> Option<(u32, u32, u32)> {
    for bytes in value.as_bytes().windows(10) {
        if bytes[2] == b'-'
            && bytes[5] == b'-'
            && bytes[..2].iter().all(u8::is_ascii_digit)
            && bytes[3..5].iter().all(u8::is_ascii_digit)
            && bytes[6..].iter().all(u8::is_ascii_digit)
        {
            let month = std::str::from_utf8(&bytes[..2]).ok()?.parse().ok()?;
            let day = std::str::from_utf8(&bytes[3..5]).ok()?.parse().ok()?;
            let year = std::str::from_utf8(&bytes[6..]).ok()?.parse().ok()?;
            if (1..=12).contains(&month) && (1..=31).contains(&day) {
                return Some((month, day, year));
            }
        }
    }
    None
}

impl PatchEventInsert {
    pub(crate) fn metadata_json(&self) -> Result<String> {
        object_json_string(self.metadata.clone())
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn hero_index(name: &str) -> EntityIndex {
        let mut index = EntityIndex::default();
        index
            .hero_names
            .insert(normalize_key(name), name.to_string());
        index
    }

    #[test]
    fn concrete_hero_changes_keep_nested_ability_descriptions() {
        for (hero, line) in [
            (
                "Rat King",
                "- Rat King: Rule, Ratannia! - Now falls faster while charging",
            ),
            (
                "Rat King",
                "- Rat King: Rule, Ratannia! - T1 +1m/s Move Speed to +2m/s Move Speed",
            ),
            (
                "Sinclair",
                "- Sinclair: Vexing Bolt - Now does half damage to objectives",
            ),
        ] {
            let payload = json!({
                "title": "Minor Update - 10-05-2026",
                "url": "https://store.steampowered.com/news/app/1422450/view/703281025618282632",
                "raw_content": line,
            });
            let (events, skipped) =
                parse_patchnote_snapshot(17, 17, "patch_17", &payload, &hero_index(hero)).unwrap();
            assert_eq!(skipped, 0, "{line}");
            assert_eq!(events.len(), 1, "{line}");
            assert_eq!(events[0].entity_name.as_deref(), Some(hero));
            assert_eq!(events[0].raw_line, line);
            let (_, remainder) = split_subject(line.trim_start_matches("- "));
            assert_eq!(events[0].normalized_line, remainder);
        }
        let lines = [
            "- Rat King: Rule, Ratannia! - Now falls faster while charging",
            "- Sinclair: Vexing Bolt - Now does half damage to objectives",
        ];
        let mut index = hero_index("Rat King");
        index
            .hero_names
            .insert(normalize_key("Sinclair"), "Sinclair".into());
        let payload = json!({"raw_content": lines.join(" ")});
        let (events, skipped) =
            parse_patchnote_snapshot(17, 17, "patch_17", &payload, &index).unwrap();
        assert_eq!(skipped, 0);
        assert_eq!(events.len(), lines.len());
        for (line, event) in lines.iter().zip(&events) {
            let (subject, remainder) = split_subject(line.trim_start_matches("- "));
            assert_eq!(event.entity_name, subject);
            assert_eq!(event.normalized_line, remainder);
        }
    }

    #[test]
    fn change_qualifiers_stay_with_their_change() {
        for line in [
            "- Sinclair: Damage increased from 50 to 60 - only against objectives",
            "- Sinclair: Damage increased from 50 to 60 - only after fixing a reload bug",
            "- Sinclair: Vexing Bolt - Now does half damage to objectives - only while charging",
        ] {
            assert_eq!(expand_inline_bullets(line), [line]);
            let payload = json!({"raw_content": line});
            let (events, skipped) =
                parse_patchnote_snapshot(17, 17, "patch_17", &payload, &hero_index("Sinclair"))
                    .unwrap();
            assert_eq!(skipped, 0);
            assert_eq!(events.len(), 1);
            assert_eq!(events[0].entity_name.as_deref(), Some("Sinclair"));
            assert_eq!(events[0].raw_line, line);
            assert_eq!(
                events[0].normalized_line,
                line.trim_start_matches("- Sinclair: ")
            );
        }
    }

    #[test]
    fn independent_changes_after_concrete_hero_changes_remain_separate() {
        let general = "- Fixed bugs that were causing troopers to walk too slowly in lane";
        for first in [
            "- Sinclair: Base HP Regen reduced from 2 to 1",
            "- Sinclair: Vexing Bolt - Now does half damage to objectives",
            "- Sinclair: Now does half damage to objectives",
            "- Sinclair: T1 +1m/s Move Speed to +2m/s Move Speed",
        ] {
            let flat = format!("{first} {general}");
            assert_eq!(expand_inline_bullets(&flat), [first, general]);
            let payload = json!({"raw_content": flat});
            let (events, skipped) =
                parse_patchnote_snapshot(17, 17, "patch_17", &payload, &hero_index("Sinclair"))
                    .unwrap();
            assert_eq!(skipped, 0);
            assert_eq!(events.len(), 2);
            assert_eq!(events[0].entity_name.as_deref(), Some("Sinclair"));
            assert_eq!(events[1].entity_name, None);
            assert_eq!(events[1].normalized_line, general.trim_start_matches("- "));
        }
    }

    #[test]
    fn change_actions_ignore_the_bound_entity_name() {
        assert!(!has_change_action("Improved Spirit: Item Changes"));
        assert!(!has_change_action("Improved Spirit: Passive"));
        assert!(has_change_action("Improved Spirit: Added knockback"));
        assert!(has_change_action("Improved Spirit: Now grants knockback"));
    }

    #[test]
    fn general_prefixed_and_short_now_changes_are_actions() {
        for line in [
            "Parry: can now interrupt",
            "Damage now scales",
            "Players can now parry",
            "Parry: can now interrupt - only melee attacks",
            "Matchmaking: adjusted targeting",
            "Matchmaking: updated queue rules",
            "Matchmaking: changed queue rules",
            "Voice/Text chat is now opt-in. There is a prompt pre-match for joining the chat.",
            "You can now press ESC to mute individual players.",
        ] {
            assert!(has_change_action(line), "{line}");
        }
        for line in [
            "General Changes",
            "Improved Spirit: Item Changes",
            "Now",
            "Unknown",
        ] {
            assert!(!has_change_action(line), "{line}");
        }
    }

    #[test]
    fn snapshot_parser_keeps_general_prefixed_and_short_now_changes() {
        let lines = [
            "Parry: can now interrupt",
            "Damage now scales",
            "Players can now parry",
            "Parry: can now interrupt - only melee attacks",
            "Matchmaking: adjusted targeting",
            "You can now press ESC to mute individual players.",
        ];
        let payload = json!({"raw_content": format!("- Holliday: added knockback\nGeneral\n{}", lines.join("\n"))});
        let (events, skipped) =
            parse_patchnote_snapshot(17, 17, "patch_17", &payload, &hero_index("Holliday"))
                .unwrap();
        assert_eq!(skipped, 0);
        assert_eq!(events.len(), lines.len() + 1);
        for (line, event) in lines.iter().zip(&events[1..]) {
            assert_eq!(event.raw_line, *line);
            assert_eq!(event.entity_type, "general");
            assert_eq!(event.entity_name, None);
            assert_eq!(event.section.as_deref(), Some("General"));
        }
    }

    #[test]
    fn action_named_section_resets_snapshot_entity_binding() {
        for heading in ["[ New Heroes ]", "New Heroes:", "New Heroes"] {
            let payload = json!({"raw_content": format!("Holliday\n- Added knockback\n{heading}\nPlayers can now parry")});
            let (events, skipped) =
                parse_patchnote_snapshot(17, 17, "patch_17", &payload, &hero_index("Holliday"))
                    .unwrap();
            assert_eq!(skipped, 0);
            assert_eq!(events.len(), 2);
            assert_eq!(events[0].entity_name.as_deref(), Some("Holliday"));
            assert_eq!(events[1].entity_name, None);
            assert_eq!(events[1].section.as_deref(), Some("New Heroes"));
        }
    }

    #[test]
    fn classify_numeric_decrease_as_nerf_without_stat_words() {
        assert_eq!(classify_change_type("reduced from 1.2 to 1.05"), "nerf");
    }

    #[test]
    fn classify_numeric_change_uses_stat_polarity() {
        let cases = [
            ("cooldown reduced from 10s to 8s", "buff"),
            ("cost increased from 3 to 4", "nerf"),
            ("reduced from 1.05 to 1.2", "buff"),
            ("damage reduced from 90 to 80", "nerf"),
            ("spirit scaling reduced from 1.2 to 1.05", "nerf"),
            ("spirits caling reduced from 1.2 to 1.05", "nerf"),
            ("delay increased 7 to 7.5", "nerf"),
        ];

        for (line, expected) in cases {
            assert_eq!(classify_change_type(line), expected, "{line}");
        }
    }

    #[test]
    fn keeps_importer_text_heuristics() {
        let cases = [
            ("King of the Hill renamed to Unstable Rift", "rename"),
            ("Ability redesigned around burst damage", "rework"),
            ("Fix tooltip crash", "bugfix"),
            ("New teleporters added to outer lanes", "added"),
            ("higher damage falloff", "buff"),
            ("lower cooldown on Shield", "buff"),
        ];

        for (line, expected) in cases {
            assert_eq!(classify_change_type(line), expected, "{line}");
        }
    }

    fn parse_content(content: &str) -> Vec<PatchEventInsert> {
        let payload = json!({ "raw_content": content });
        parse_patchnote_snapshot(5, 5, "patch_5", &payload, &hero_index("Yamato"))
            .expect("parse snapshot")
            .0
    }

    #[test]
    fn known_hero_header_still_owns_following_bullets() {
        let events =
            parse_content("[ Heroes ]\nYamato\n- Bullet damage growth increased from 0.4 to 0.5");
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].entity_type, "hero");
        assert_eq!(events[0].entity_name.as_deref(), Some("Yamato"));
    }

    #[test]
    fn unknown_headings_do_not_become_entities_for_later_lines() {
        let events = parse_content(
            "[ Heroes ]\nYamato\n- Bullet damage increased from 1 to 2\nLocalization additions and improvements\n- Successfully parrying objectives no longer resets your Parry cooldown\n**Custom Hideout Camera:**\n- Fixed troopers walking too slowly",
        );
        let labels = events
            .iter()
            .map(|event| (event.entity_type.as_str(), event.entity_name.as_deref()))
            .collect::<Vec<_>>();
        assert_eq!(
            labels,
            vec![
                ("hero", Some("Yamato")),
                ("general", None),
                ("general", None),
            ]
        );
    }

    #[test]
    fn media_and_link_lines_are_skipped() {
        let events = parse_content(
            "[ Heroes ]\nfinal_shop_1.png\nstore.steampowered.com\nhttps://example.com/x.mp4\n- Fixed troopers walking too slowly",
        );
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].entity_name, None);
    }

    #[test]
    fn general_lines_naming_an_objective_link_to_it() {
        let payload = json!({
            "title": "06-06-2024 Update",
            "url": "https://forums.playdeadlock.com/threads/06-06-2024-update.1/",
            "raw_content": "- Walker bomb damage reduced from 300 to 200\n- Troopers now take 28% more damage from Guardians\n- Added settings option to allow weapon zoom to be a toggle",
        });
        let index = hero_index("Yamato").with_objectives();
        let (events, _) = parse_patchnote_snapshot(5, 5, "patch_5", &payload, &index).unwrap();
        let labels = events
            .iter()
            .map(|event| (event.entity_type.as_str(), event.entity_name.as_deref()))
            .collect::<Vec<_>>();
        assert_eq!(
            labels,
            [
                ("objective", Some("Walker")),
                ("objective", Some("Guardian")),
                ("general", None),
            ]
        );
    }

    #[test]
    fn no_longer_is_an_action_but_not_a_removal() {
        let line = "Unstoppable state no longer blocks anti-heal";
        assert!(has_change_action(line));
        assert_ne!(classify_change_type(line), "removed");
        assert_eq!(classify_change_type("Removed Mystic Slow"), "removed");
    }

    #[test]
    fn parse_patchnote_snapshot_uses_title_date_when_posted_at_missing() {
        let payload = json!({
            "title": "07-09-2026 Update",
            "url": "https://forums.playdeadlock.com/threads/07-09-2026-update.1/",
            "raw_content": "- Holliday: increased from 1 to 2"
        });
        let (events, skipped) =
            parse_patchnote_snapshot(1, 1, "patch_1", &payload, &hero_index("Holliday"))
                .expect("parse snapshot");

        assert_eq!(skipped, 0);
        assert_eq!(events.len(), 1);
        assert_eq!(
            events[0].posted_at.as_deref(),
            Some("2026-07-09T00:00:00+00:00")
        );
    }
}
