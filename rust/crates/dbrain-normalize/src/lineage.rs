use std::collections::BTreeMap;

use rusqlite::{params, Connection};
use serde_json::{json, Value};

use crate::util::{clean_subject, normalize_alias, object_json_string, table_count};
use crate::Result;

const LINEAGE_NAME_LIMIT: usize = 120;

#[derive(Debug, Clone, PartialEq)]
pub struct LineageEvent {
    pub id: i64,
    pub patch_title: Option<String>,
    pub patch_url: Option<String>,
    pub source_kind: Option<String>,
    pub posted_at: Option<String>,
    pub entity_type: Option<String>,
    pub entity_name: Option<String>,
    pub subject: Option<String>,
    pub change_type: Option<String>,
    pub normalized_line: Option<String>,
    pub raw_line: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LineageCandidate {
    pub patch_event_id: i64,
    pub relation_type: String,
    pub source_entity_type: Option<String>,
    pub source_name: String,
    pub source_name_norm: String,
    pub target_entity_type: Option<String>,
    pub target_name: Option<String>,
    pub target_name_norm: Option<String>,
    pub owner_entity_type: Option<String>,
    pub owner_name: Option<String>,
    pub owner_name_norm: Option<String>,
    pub confidence: f64,
    pub metadata: BTreeMap<String, Value>,
}

struct CandidateInput {
    relation_type: &'static str,
    source_type: Option<String>,
    source_name: String,
    target_type: Option<String>,
    target_name: Option<String>,
    owner_type: Option<String>,
    owner_name: Option<String>,
    confidence: f64,
}

pub fn enrich_lineage(conn: &Connection, rebuild: bool) -> Result<Value> {
    let before_total = table_count(conn, "entity_lineage")?;
    let deleted = if rebuild {
        conn.execute("DELETE FROM entity_lineage", [])? as i64
    } else {
        0
    };

    let events = load_events(conn)?;
    let mut processed = 0_i64;
    let mut inserted = 0_i64;
    let mut by_relation: BTreeMap<String, i64> = BTreeMap::new();
    for event in events {
        let candidates = extract_lineage_candidates(&event);
        processed += 1;
        for candidate in candidates {
            if insert_lineage(conn, &candidate)? {
                inserted += 1;
                *by_relation.entry(candidate.relation_type.clone()).or_default() += 1;
            }
        }
    }

    Ok(json!({
        "events_processed": processed,
        "lineage_inserted": inserted,
        "lineage_before": before_total,
        "lineage_total": table_count(conn, "entity_lineage")?,
        "deleted_before_build": deleted,
        "by_relation": by_relation,
        "rebuild": rebuild,
    }))
}

pub fn extract_lineage_candidates(event: &LineageEvent) -> Vec<LineageCandidate> {
    let text = clean_line(
        event
            .normalized_line
            .as_deref()
            .or(event.raw_line.as_deref())
            .unwrap_or_default(),
    );
    if text.is_empty() {
        return Vec::new();
    }
    let entity_type = lineage_entity_type(event.entity_type.as_deref());
    let entity_name = clean_name(
        event
            .entity_name
            .as_deref()
            .or(event.subject.as_deref())
            .unwrap_or_default(),
    );
    let entity_name = if entity_name.is_empty() { None } else { Some(entity_name) };
    let owner_type = if entity_type.as_deref() == Some("hero") {
        entity_type.clone()
    } else {
        None
    };
    let owner_name = if owner_type.is_some() { entity_name.clone() } else { None };
    let mut candidates = Vec::new();

    if let Some(new_name) = renamed_to(&text) {
        if let Some(entity_name) = entity_name.clone() {
            candidates.push(candidate(event, CandidateInput {
                relation_type: "rename",
                source_type: entity_type.clone(),
                source_name: entity_name,
                target_type: entity_type.clone(),
                target_name: Some(clean_name(&new_name)),
                owner_type: None,
                owner_name: None,
                confidence: 0.95,
            }));
            return candidates;
        }
    }

    if let Some((old_name, new_name)) = renamed_x_to_y(&text).or_else(|| renamed_old_to_new(&text)) {
        candidates.push(candidate(event, CandidateInput {
            relation_type: "rename",
            source_type: scoped_child_type(entity_type.as_deref()),
            source_name: clean_name(&old_name),
            target_type: scoped_child_type(entity_type.as_deref()),
            target_name: Some(clean_name(&new_name)),
            owner_type: owner_type.clone(),
            owner_name: owner_name.clone(),
            confidence: 0.9,
        }));
        return valid_candidates(candidates);
    }

    if let Some((old_name, new_name)) = replaced_with(&text) {
        candidates.push(candidate(event, CandidateInput {
            relation_type: "replaced_by",
            source_type: entity_type.clone(),
            source_name: clean_name(&old_name),
            target_type: entity_type.clone(),
            target_name: Some(clean_name(&new_name)),
            owner_type: None,
            owner_name: None,
            confidence: 0.75,
        }));
    }

    if is_rework_event(event, &text) {
        let mut reworked_name = entity_name.clone();
        if reworked_name.is_none() {
            reworked_name = reworked_name_from_text(&text);
        }
        if let Some(reworked_name) = reworked_name {
            let keep_owner = owner_name.as_deref() != Some(reworked_name.as_str());
            candidates.push(candidate(event, CandidateInput {
                relation_type: "rework",
                source_type: entity_type.clone(),
                source_name: reworked_name.clone(),
                target_type: entity_type,
                target_name: Some(reworked_name),
                owner_type: if keep_owner { owner_type } else { None },
                owner_name: if keep_owner { owner_name } else { None },
                confidence: if text.to_lowercase().contains("reworked") { 0.72 } else { 0.65 },
            }));
        }
    }

    valid_candidates(candidates)
}

fn load_events(conn: &Connection) -> Result<Vec<LineageEvent>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, patch_title, patch_url, source_kind, posted_at, entity_type,
               entity_name, subject, change_type, normalized_line, raw_line
        FROM patch_events
        WHERE
          change_type IN ('changed', 'rework', 'removed', 'added')
          OR lower(normalized_line) LIKE '%rename%'
          OR lower(normalized_line) LIKE '%rework%'
          OR lower(normalized_line) LIKE '%replaced%'
        ORDER BY id ASC
        "#,
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(LineageEvent {
            id: row.get("id")?,
            patch_title: row.get("patch_title")?,
            patch_url: row.get("patch_url")?,
            source_kind: row.get("source_kind")?,
            posted_at: row.get("posted_at")?,
            entity_type: row.get("entity_type")?,
            entity_name: row.get("entity_name")?,
            subject: row.get("subject")?,
            change_type: row.get("change_type")?,
            normalized_line: row.get("normalized_line")?,
            raw_line: row.get("raw_line")?,
        })
    })?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

fn candidate(event: &LineageEvent, input: CandidateInput) -> LineageCandidate {
    let mut metadata = BTreeMap::new();
    metadata.insert("patch_title".to_string(), option_json(event.patch_title.clone()));
    metadata.insert("patch_url".to_string(), option_json(event.patch_url.clone()));
    metadata.insert("posted_at".to_string(), option_json(event.posted_at.clone()));
    metadata.insert("source_kind".to_string(), option_json(event.source_kind.clone()));
    metadata.insert(
        "line".to_string(),
        option_json(event.normalized_line.clone().or_else(|| event.raw_line.clone())),
    );
    let target_name_norm = input
        .target_name
        .as_deref()
        .map(normalize_alias)
        .filter(|value| !value.is_empty());
    let owner_name_norm = input
        .owner_name
        .as_deref()
        .map(normalize_alias)
        .filter(|value| !value.is_empty());
    LineageCandidate {
        patch_event_id: event.id,
        relation_type: input.relation_type.to_string(),
        source_entity_type: input.source_type,
        source_name_norm: normalize_alias(&input.source_name),
        source_name: input.source_name,
        target_entity_type: input.target_type,
        target_name: input.target_name,
        target_name_norm,
        owner_entity_type: input.owner_type,
        owner_name: input.owner_name,
        owner_name_norm,
        confidence: input.confidence,
        metadata,
    }
}

fn insert_lineage(conn: &Connection, candidate: &LineageCandidate) -> Result<bool> {
    let now = crate::util::now()?;
    let metadata_json = object_json_string(candidate.metadata.clone())?;
    let changed = conn.execute(
        r#"
        INSERT INTO entity_lineage(
          patch_event_id, relation_type, source_entity_type, source_name, source_name_norm,
          target_entity_type, target_name, target_name_norm, owner_entity_type, owner_name,
          owner_name_norm, confidence, metadata_json, created_at, updated_at
        )
        VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15)
        ON CONFLICT DO NOTHING
        "#,
        params![
            candidate.patch_event_id,
            candidate.relation_type,
            candidate.source_entity_type,
            candidate.source_name,
            candidate.source_name_norm,
            candidate.target_entity_type,
            candidate.target_name,
            candidate.target_name_norm,
            candidate.owner_entity_type,
            candidate.owner_name,
            candidate.owner_name_norm,
            candidate.confidence,
            metadata_json,
            now,
            now,
        ],
    )?;
    Ok(changed > 0)
}

fn valid_candidates(candidates: Vec<LineageCandidate>) -> Vec<LineageCandidate> {
    candidates.into_iter().filter(valid_candidate).collect()
}

fn valid_candidate(candidate: &LineageCandidate) -> bool {
    let source = candidate.source_name.trim();
    let target = candidate.target_name.as_deref().unwrap_or_default().trim();
    if source.len() < 2 || source.len() > LINEAGE_NAME_LIMIT {
        return false;
    }
    if !target.is_empty() && target.len() > LINEAGE_NAME_LIMIT {
        return false;
    }
    !matches!(source.to_lowercase().as_str(), "it" | "this" | "the item" | "the ability")
}

fn lineage_entity_type(value: Option<&str>) -> Option<String> {
    let text = value.unwrap_or_default().trim();
    if text.is_empty() || text == "general" {
        None
    } else {
        Some(text.to_string())
    }
}

fn scoped_child_type(parent_type: Option<&str>) -> Option<String> {
    if matches!(parent_type, Some("hero" | "hero_internal")) {
        Some("ability".to_string())
    } else {
        parent_type.map(ToString::to_string)
    }
}

fn is_rework_event(event: &LineageEvent, text: &str) -> bool {
    let lower = text.to_lowercase();
    event
        .change_type
        .as_deref()
        .map(|value| value.eq_ignore_ascii_case("rework"))
        .unwrap_or(false)
        || lower.contains("reworked")
        || lower.contains("rescaled")
}

fn clean_line(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn clean_name(value: &str) -> String {
    let mut text = clean_line(value);
    if text.ends_with(')') {
        if let Some(index) = text.rfind('(') {
            text = text[..index].trim().to_string();
        }
    }
    clean_subject(Some(text.trim_matches(|ch: char| {
        ch == ' ' || ch == ':' || ch == '-' || ch == '\t' || ch == '"' || ch == '\''
    })))
}

fn renamed_to(text: &str) -> Option<String> {
    let lower = text.to_lowercase();
    let prefix = if lower.starts_with("now renamed to ") {
        "now renamed to "
    } else if lower.starts_with("renamed to ") {
        "renamed to "
    } else {
        return None;
    };
    Some(sentence_tail(&text[prefix.len()..]))
}

fn renamed_x_to_y(text: &str) -> Option<(String, String)> {
    let lower = text.to_lowercase();
    let marker = " renamed to ";
    if lower.starts_with("renamed to ") {
        return None;
    }
    let index = lower.find(marker)?;
    let old_name = text[..index].trim();
    let new_name = &text[index + marker.len()..];
    if old_name.is_empty() {
        None
    } else {
        Some((old_name.to_string(), sentence_tail(new_name)))
    }
}

fn renamed_old_to_new(text: &str) -> Option<(String, String)> {
    let lower = text.to_lowercase();
    let rest = lower.strip_prefix("renamed ")?;
    let index = rest.find(" to ")?;
    let old_start = "renamed ".len();
    let old_end = old_start + index;
    let new_start = old_end + " to ".len();
    Some((text[old_start..old_end].trim().to_string(), sentence_tail(&text[new_start..])))
}

fn replaced_with(text: &str) -> Option<(String, String)> {
    let lower = text.to_lowercase();
    let rest = lower.strip_prefix("replaced ")?;
    let index = rest.find(" with ")?;
    let old_start = "replaced ".len();
    let old_end = old_start + index;
    let new_start = old_end + " with ".len();
    Some((text[old_start..old_end].trim().to_string(), sentence_tail(&text[new_start..])))
}

fn reworked_name_from_text(text: &str) -> Option<String> {
    let lower = text.to_lowercase();
    if let Some(index) = lower.find(" has been reworked") {
        let name = clean_name(&text[..index]);
        return (!name.is_empty()).then_some(name);
    }
    if let Some(index) = lower.find(" reworked") {
        let name = clean_name(&text[..index]);
        return (!name.is_empty()).then_some(name);
    }
    None
}

fn sentence_tail(value: &str) -> String {
    value
        .trim()
        .trim_end_matches(['.', ';', ',', ')'])
        .trim()
        .to_string()
}

fn option_json(value: Option<String>) -> Value {
    value.map_or(Value::Null, Value::String)
}
