use std::collections::{BTreeMap, HashMap, HashSet};

use serde_json::{json, Value};
use sqlx::PgPool;

use crate::util::{
    normalize_alias, object_json_string, optional_value_to_string, parse_int, value_bool,
    value_to_string, value_truthy, ASSETS_SOURCE,
};
use crate::Result;

const PUBLIC_ITEM_TYPES: &[&str] = &["upgrade"];
const PUBLIC_ITEM_SLOTS: &[&str] = &["weapon", "vitality", "spirit"];
const INTERNAL_NAME_PREFIXES: &[&str] = &[
    "ability_",
    "citadel_",
    "hero_",
    "item_",
    "melee_",
    "upgrade_",
    "weapon_",
];
const INTERNAL_ENTITY_TYPES: &[&str] = &["hero_internal", "ability_internal", "weapon_or_internal"];

#[derive(Debug, Clone)]
struct SnapshotRow {
    id: i64,
    entity_type: String,
    external_id: String,
    canonical_name: Option<String>,
    payload_json: String,
    document_kind: Option<String>,
}

#[derive(Debug, Clone)]
struct EntityCandidate {
    entity_type: String,
    canonical_name: String,
    external_id: Option<String>,
    snapshot_id: Option<i64>,
    source: String,
    metadata: BTreeMap<String, Value>,
    aliases: HashSet<(String, String)>,
}

#[derive(Debug, Default)]
struct AssetContext {
    active_hero_ids: HashSet<String>,
    active_hero_tokens: HashSet<String>,
    public_item_names_by_class: HashMap<String, String>,
    special_item_names_by_class: HashMap<String, String>,
    active_ability_names_by_class: HashMap<String, String>,
    internal_ability_names_by_class: HashMap<String, String>,
}

pub async fn normalize_entities(pool: &PgPool, rebuild: bool) -> Result<Value> {
    let deleted = if rebuild {
        let aliases = sqlx::query!("DELETE FROM brain.entity_aliases")
            .execute(pool)
            .await?
            .rows_affected() as i64;
        let entities = sqlx::query!("DELETE FROM brain.entities")
            .execute(pool)
            .await?
            .rows_affected() as i64;
        json!({"aliases": aliases, "entities": entities})
    } else {
        json!({"aliases": 0, "entities": 0})
    };

    let candidates = collect_asset_candidates(pool).await?;
    let mut inserted = 0_i64;
    let mut aliases = 0_i64;
    let mut skipped = 0_i64;
    let mut by_type: BTreeMap<String, i64> = BTreeMap::new();

    for candidate in candidates.into_values() {
        if candidate.canonical_name.is_empty() {
            skipped += 1;
            continue;
        }
        let entity_id = upsert_entity(pool, &candidate).await?;
        inserted += 1;
        *by_type.entry(candidate.entity_type.clone()).or_default() += 1;

        let mut alias_rows = candidate.aliases.iter().cloned().collect::<Vec<_>>();
        alias_rows.sort();
        for (alias, alias_kind) in alias_rows {
            let alias_norm = normalize_alias(&alias);
            if alias_norm.is_empty() {
                continue;
            }
            if upsert_entity_alias(pool, &candidate, entity_id, &alias, &alias_norm, &alias_kind).await? {
                aliases += 1;
            }
        }
    }

    Ok(json!({
        "deleted": deleted,
        "entities": inserted,
        "aliases": aliases,
        "skipped": skipped,
        "by_type": by_type,
    }))
}

async fn collect_asset_candidates(pool: &PgPool) -> Result<HashMap<(String, String), EntityCandidate>> {
    let rows = load_asset_rows(pool).await?;
    let context = build_asset_context(&rows);
    let mut candidates: HashMap<(String, String), EntityCandidate> = HashMap::new();
    let mut class_index: HashMap<(String, String), (String, String)> = HashMap::new();

    for row in rows {
        let Ok(payload) = serde_json::from_str::<Value>(&row.payload_json) else {
            continue;
        };
        let Some(payload) = payload.as_object() else {
            continue;
        };
        let entity_type = classify_snapshot(&row.entity_type, payload, &context);
        let Some(canonical_name) = canonical_name(&entity_type, row.canonical_name.as_deref(), payload, &context) else {
            continue;
        };
        if canonical_name.is_empty() {
            continue;
        }

        let class_name = value_to_string(payload.get("class_name")).trim().to_string();
        let class_key = if class_name.is_empty() {
            None
        } else {
            Some((entity_type.clone(), class_name.to_lowercase()))
        };
        let key = class_key
            .as_ref()
            .and_then(|key| class_index.get(key).cloned())
            .unwrap_or_else(|| (entity_type.clone(), canonical_name.clone()));

        let metadata = metadata_for(row.document_kind.as_deref(), payload);
        if let Some(candidate) = candidates.get_mut(&key) {
            candidate.metadata = merge_metadata(&candidate.metadata, &metadata);
            if let Some(class_key) = class_key {
                class_index.insert(class_key, key.clone());
            }
            add_aliases(candidate, Some(&row.external_id), row.canonical_name.as_deref(), payload);
        } else {
            let mut candidate = EntityCandidate {
                entity_type: entity_type.clone(),
                canonical_name,
                external_id: Some(row.external_id.clone()),
                snapshot_id: Some(row.id),
                source: ASSETS_SOURCE.to_string(),
                metadata,
                aliases: HashSet::new(),
            };
            add_aliases(&mut candidate, Some(&row.external_id), row.canonical_name.as_deref(), payload);
            candidates.insert(key.clone(), candidate);
            if let Some(class_key) = class_key {
                class_index.insert(class_key, key);
            }
        }
    }

    Ok(candidates)
}

async fn load_asset_rows(pool: &PgPool) -> Result<Vec<SnapshotRow>> {
    let rows = sqlx::query!(
        r#"
        SELECT es.id AS "id!", es.entity_type AS "entity_type!", es.external_id AS "external_id!",
               es.canonical_name AS "canonical_name?", es.payload::text AS "payload_json!",
               sd.external_id AS "document_kind?"
        FROM brain.entity_snapshots es
        LEFT JOIN brain.source_documents sd ON sd.id = es.source_document_id
        WHERE es.source = $1
        ORDER BY
          CASE sd.external_id
            WHEN 'heroes' THEN 0
            WHEN 'items' THEN 1
            WHEN 'raw_heroes' THEN 2
            WHEN 'raw_items' THEN 3
            ELSE 4
          END,
          es.id
        "#,
        ASSETS_SOURCE,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|row| SnapshotRow {
            id: row.id,
            entity_type: row.entity_type,
            external_id: row.external_id,
            canonical_name: row.canonical_name,
            payload_json: row.payload_json,
            document_kind: row.document_kind,
        })
        .collect())
}

fn build_asset_context(rows: &[SnapshotRow]) -> AssetContext {
    let mut context = AssetContext::default();

    for row in rows {
        let Ok(payload) = serde_json::from_str::<Value>(&row.payload_json) else {
            continue;
        };
        let Some(payload) = payload.as_object() else {
            continue;
        };
        if row.entity_type == "hero" {
            let hero_id = value_to_string(payload.get("id"))
                .trim()
                .to_string();
            let class_token = strip_prefix(payload.get("class_name"), "hero_");
            if is_active_hero(payload) {
                if !hero_id.is_empty() {
                    context.active_hero_ids.insert(hero_id);
                }
                if let Some(class_token) = class_token {
                    context.active_hero_tokens.insert(class_token.to_lowercase());
                }
            }
        }
    }

    for row in rows {
        let Ok(payload) = serde_json::from_str::<Value>(&row.payload_json) else {
            continue;
        };
        let Some(payload) = payload.as_object() else {
            continue;
        };
        let payload_type = value_to_string(payload.get("type")).to_lowercase();
        let class_name = value_to_string(payload.get("class_name")).trim().to_lowercase();
        if is_public_item(&payload_type, payload) {
            let name = optional_value_to_string(payload.get("name"))
                .or_else(|| row.canonical_name.clone())
                .unwrap_or_default()
                .trim()
                .to_string();
            if !class_name.is_empty() && !name.is_empty() {
                if is_special_item(payload) {
                    context.special_item_names_by_class.insert(class_name, name);
                } else {
                    context.public_item_names_by_class.insert(class_name, name);
                }
            }
        } else if payload_type == "ability" {
            let name = optional_value_to_string(payload.get("name"))
                .or_else(|| row.canonical_name.clone())
                .unwrap_or_default()
                .trim()
                .to_string();
            if !class_name.is_empty() && !name.is_empty() {
                if is_active_ability(payload, &context.active_hero_ids, &context.active_hero_tokens) {
                    context.active_ability_names_by_class.insert(class_name, name);
                } else if !looks_internal_name(&name) {
                    context.internal_ability_names_by_class.insert(class_name, name);
                }
            }
        }
    }

    context
}

fn classify_snapshot(
    snapshot_type: &str,
    payload: &serde_json::Map<String, Value>,
    context: &AssetContext,
) -> String {
    let payload_type = value_to_string(payload.get("type")).to_lowercase();
    if snapshot_type == "hero" || payload_type == "hero" {
        return if is_active_hero(payload) { "hero" } else { "hero_internal" }.to_string();
    }
    if snapshot_type == "rank" || payload_type == "rank" {
        return "rank".to_string();
    }
    let class_name = value_to_string(payload.get("class_name")).trim().to_lowercase();
    if payload_type == "ability" {
        if context.active_ability_names_by_class.contains_key(&class_name) {
            return "ability".to_string();
        }
        if context.internal_ability_names_by_class.contains_key(&class_name) {
            return "ability_internal".to_string();
        }
        return if is_active_ability(payload, &context.active_hero_ids, &context.active_hero_tokens) {
            "ability"
        } else {
            "ability_internal"
        }
        .to_string();
    }
    if context.public_item_names_by_class.contains_key(&class_name) {
        return "item".to_string();
    }
    if context.special_item_names_by_class.contains_key(&class_name) {
        return "item_special".to_string();
    }
    if is_public_item(&payload_type, payload) {
        return if is_special_item(payload) { "item_special" } else { "item" }.to_string();
    }
    "weapon_or_internal".to_string()
}

fn is_active_hero(payload: &serde_json::Map<String, Value>) -> bool {
    value_bool(payload.get("disabled")) != Some(true)
        && value_bool(payload.get("player_selectable")) == Some(true)
        && value_bool(payload.get("in_development")) != Some(true)
        && value_bool(payload.get("prerelease_only")) != Some(true)
        && value_bool(payload.get("assigned_players_only")) != Some(true)
}

fn is_active_ability(
    payload: &serde_json::Map<String, Value>,
    active_hero_ids: &HashSet<String>,
    active_hero_tokens: &HashSet<String>,
) -> bool {
    let name = value_to_string(payload.get("name")).trim().to_string();
    if name.is_empty() || looks_internal_name(&name) {
        return false;
    }
    let hero = value_to_string(payload.get("hero")).trim().to_string();
    if !hero.is_empty() {
        return active_hero_ids.contains(&hero);
    }
    hero_token_from_ability_class(payload.get("class_name"))
        .map(|token| active_hero_tokens.contains(&token))
        .unwrap_or(false)
}

fn hero_token_from_ability_class(value: Option<&Value>) -> Option<String> {
    let text = value_to_string(value).to_lowercase();
    for prefix in ["citadel_ability_", "ability_"] {
        if let Some(rest) = text.strip_prefix(prefix) {
            let token = rest.split('_').next().unwrap_or_default();
            if !token.is_empty() {
                return Some(token.to_string());
            }
        }
    }
    None
}

fn is_public_item(payload_type: &str, payload: &serde_json::Map<String, Value>) -> bool {
    if !PUBLIC_ITEM_TYPES.contains(&payload_type) {
        return false;
    }
    if value_bool(payload.get("disabled")) == Some(true) {
        return false;
    }
    if value_bool(payload.get("shopable")) != Some(true) {
        return false;
    }
    let slot = value_to_string(payload.get("item_slot_type")).to_lowercase();
    if !PUBLIC_ITEM_SLOTS.contains(&slot.as_str()) {
        return false;
    }
    let name = value_to_string(payload.get("name"));
    if looks_internal_name(&name) && !value_truthy(payload.get("shop_image")) {
        return false;
    }
    value_truthy(payload.get("item_tier"))
        || value_truthy(payload.get("cost"))
        || value_truthy(payload.get("shop_image"))
}

fn is_special_item(payload: &serde_json::Map<String, Value>) -> bool {
    let tier = parse_int(&value_to_string(payload.get("item_tier"))).unwrap_or(0);
    let cost = parse_int(&value_to_string(payload.get("cost"))).unwrap_or(0);
    tier > 4 || cost >= 9000
}

fn canonical_name(
    entity_type: &str,
    snapshot_name: Option<&str>,
    payload: &serde_json::Map<String, Value>,
    context: &AssetContext,
) -> Option<String> {
    let class_name = value_to_string(payload.get("class_name")).trim().to_lowercase();
    if entity_type == "item" {
        if let Some(name) = context.public_item_names_by_class.get(&class_name) {
            return Some(name.clone());
        }
    }
    if entity_type == "item_special" {
        if let Some(name) = context.special_item_names_by_class.get(&class_name) {
            return Some(name.clone());
        }
    }
    if entity_type == "ability" {
        if let Some(name) = context.active_ability_names_by_class.get(&class_name) {
            return Some(name.clone());
        }
    }
    if entity_type == "ability_internal" {
        if let Some(name) = context.internal_ability_names_by_class.get(&class_name) {
            return Some(name.clone());
        }
    }

    let name = optional_value_to_string(payload.get("name"))
        .or_else(|| snapshot_name.map(ToString::to_string))
        .or_else(|| optional_value_to_string(payload.get("class_name")))
        .unwrap_or_default()
        .trim()
        .to_string();
    if name.is_empty() {
        return None;
    }
    if matches!(
        entity_type,
        "hero" | "hero_internal" | "item" | "item_special" | "ability" | "ability_internal" | "rank"
    ) {
        return Some(name);
    }
    Some(
        optional_value_to_string(payload.get("class_name"))
            .unwrap_or(name)
            .trim()
            .to_string(),
    )
}

fn add_aliases(
    candidate: &mut EntityCandidate,
    external_id: Option<&str>,
    snapshot_name: Option<&str>,
    payload: &serde_json::Map<String, Value>,
) {
    if !INTERNAL_ENTITY_TYPES.contains(&candidate.entity_type.as_str()) {
        add_alias(candidate, Some(candidate.canonical_name.clone()), "canonical");
        add_alias(candidate, snapshot_name.map(ToString::to_string), "snapshot_name");
    }
    add_alias(candidate, external_id.map(ToString::to_string), "external_id");
    add_alias(candidate, optional_value_to_string(payload.get("id")), "external_id");
    add_alias(candidate, optional_value_to_string(payload.get("class_name")), "class_name");
}

fn add_alias(candidate: &mut EntityCandidate, value: Option<String>, alias_kind: &str) {
    let Some(value) = value else {
        return;
    };
    let alias = value.trim();
    if !alias.is_empty() {
        candidate.aliases.insert((alias.to_string(), alias_kind.to_string()));
    }
}

fn strip_prefix(value: Option<&Value>, prefix: &str) -> Option<String> {
    let text = value_to_string(value);
    text.strip_prefix(prefix).map(ToString::to_string)
}

fn looks_internal_name(name: &str) -> bool {
    let stripped = name.trim();
    if stripped.is_empty() {
        return true;
    }
    let lowered = stripped.to_lowercase();
    INTERNAL_NAME_PREFIXES.iter().any(|prefix| lowered.starts_with(prefix))
}

fn metadata_for(
    document_kind: Option<&str>,
    payload: &serde_json::Map<String, Value>,
) -> BTreeMap<String, Value> {
    let mut metadata = BTreeMap::new();
    for key in [
        "type",
        "ability_type",
        "activation",
        "item_slot_type",
        "item_tier",
        "cost",
        "shopable",
        "disabled",
        "hero",
    ] {
        if let Some(value) = payload.get(key) {
            metadata.insert(key.to_string(), value.clone());
        }
    }
    if let Some(document_kind) = document_kind {
        metadata.insert("document_kinds".to_string(), json!([document_kind]));
    }
    metadata
}

fn merge_metadata(
    existing: &BTreeMap<String, Value>,
    new: &BTreeMap<String, Value>,
) -> BTreeMap<String, Value> {
    let mut merged = existing.clone();
    let mut document_kinds = string_array_set(existing.get("document_kinds"));
    document_kinds.extend(string_array_set(new.get("document_kinds")));
    if !document_kinds.is_empty() {
        let mut kinds = document_kinds.into_iter().collect::<Vec<_>>();
        kinds.sort();
        merged.insert("document_kinds".to_string(), json!(kinds));
    }
    for (key, value) in new {
        if key != "document_kinds" && !merged.contains_key(key) {
            merged.insert(key.clone(), value.clone());
        }
    }
    merged
}

fn string_array_set(value: Option<&Value>) -> HashSet<String> {
    value
        .and_then(Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(Value::as_str)
                .map(ToString::to_string)
                .collect()
        })
        .unwrap_or_default()
}

async fn upsert_entity(pool: &PgPool, candidate: &EntityCandidate) -> Result<i64> {
    let metadata_json = object_json_string(candidate.metadata.clone())?;
    let id = sqlx::query_scalar!(
        r#"
        INSERT INTO brain.entities(
          entity_type, canonical_name, primary_external_id, source,
          first_snapshot_id, metadata, created_at, updated_at
        )
        VALUES($1, $2, $3, $4, $5, $6::text::jsonb, now(), now())
        ON CONFLICT (entity_type, canonical_name) DO UPDATE SET
          primary_external_id = COALESCE(EXCLUDED.primary_external_id, entities.primary_external_id),
          first_snapshot_id = COALESCE(entities.first_snapshot_id, EXCLUDED.first_snapshot_id),
          metadata = EXCLUDED.metadata,
          updated_at = EXCLUDED.updated_at
        RETURNING id
        "#,
        candidate.entity_type,
        candidate.canonical_name,
        candidate.external_id,
        candidate.source,
        candidate.snapshot_id,
        metadata_json,
    )
    .fetch_one(pool)
    .await?;
    Ok(id)
}

async fn upsert_entity_alias(
    pool: &PgPool,
    candidate: &EntityCandidate,
    entity_id: i64,
    alias: &str,
    alias_norm: &str,
    alias_kind: &str,
) -> Result<bool> {
    let result = sqlx::query!(
        r#"
        INSERT INTO brain.entity_aliases(
          entity_id, alias, alias_norm, alias_kind, source,
          external_id, snapshot_id, created_at
        )
        VALUES($1, $2, $3, $4, $5, $6, $7, now())
        ON CONFLICT (entity_id, alias_norm, alias_kind) DO NOTHING
        "#,
        entity_id,
        alias,
        alias_norm,
        alias_kind,
        candidate.source,
        candidate.external_id,
        candidate.snapshot_id,
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}
