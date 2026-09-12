use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;
use sqlx::{postgres::PgPool, Row};

use crate::{
    AbilityModel, AbilityRole, AuthorBuild, ConditionKind, DamagePlan, DamageType, HeroModel,
    ItemModel, LevelPoint, MetaRow, PurchaseBonuses, ReasonerCtx, ReasonerError, Result,
    ScalingStat, SlotType, TierBonus, WeaponProfile,
};

async fn table_exists(pool: &PgPool, table: &str) -> Result<bool> {
    sqlx::query_scalar::<_, Option<String>>("SELECT to_regclass($1)::text")
        .bind(table)
        .fetch_one(pool)
        .await
        .map(|value| value.is_some())
        .map_err(ReasonerError::Db)
}

fn parse_json(text: String, context: &str) -> Result<Value> {
    serde_json::from_str(&text)
        .map_err(|error| ReasonerError::Data(format!("{context}: ungültiges JSON: {error}")))
}

fn number(value: Option<&Value>) -> f64 {
    value
        .and_then(|value| {
            value
                .as_f64()
                .or_else(|| value.as_i64().map(|value| value as f64))
                .or_else(|| {
                    value
                        .as_str()
                        .and_then(|value| value.trim_end_matches('%').parse().ok())
                })
        })
        .unwrap_or_default()
}

fn integer(value: Option<&Value>) -> i64 {
    number(value) as i64
}

fn string(value: Option<&Value>) -> String {
    value
        .map(|value| {
            value
                .as_str()
                .map(str::to_string)
                .unwrap_or_else(|| value.to_string().trim_matches('"').to_string())
        })
        .unwrap_or_default()
}

fn property_values(value: Option<&Value>) -> BTreeMap<String, f64> {
    let Some(properties) = value.and_then(Value::as_object) else {
        return BTreeMap::new();
    };
    properties
        .iter()
        .filter_map(|(name, property)| {
            let raw = property
                .as_object()
                .and_then(|object| object.get("value"))
                .unwrap_or(property);
            let value = number(Some(raw));
            if value == 0.0 && !raw.is_number() && raw.as_str().is_none() {
                None
            } else {
                Some((name.clone(), value))
            }
        })
        .collect()
}

fn passive_property_values(value: Option<&Value>) -> BTreeMap<String, f64> {
    let Some(properties) = value.and_then(Value::as_object) else {
        return BTreeMap::new();
    };
    properties
        .iter()
        .filter_map(|(name, property)| {
            let object = property.as_object();
            let is_passive = name.to_ascii_lowercase().contains("passive")
                || object
                    .and_then(|object| object.get("tooltip_section"))
                    .map(|value| string(Some(value)))
                    .map(|value| value.eq_ignore_ascii_case("passive"))
                    .unwrap_or(false);
            if !is_passive {
                return None;
            }
            let raw = object
                .and_then(|object| object.get("value"))
                .unwrap_or(property);
            Some((name.clone(), number(Some(raw))))
        })
        .collect()
}

fn slot_type(value: &str) -> SlotType {
    match value.to_ascii_lowercase().as_str() {
        "weapon" => SlotType::Weapon,
        "spirit" => SlotType::Spirit,
        _ => SlotType::Vitality,
    }
}

fn damage_type(value: &str) -> DamageType {
    match value.to_ascii_lowercase().as_str() {
        "weapon" | "bullet" => DamageType::Weapon,
        "spirit" | "tech" => DamageType::Spirit,
        "hybrid" => DamageType::Hybrid,
        _ => DamageType::None,
    }
}

fn description_text(payload: &Value) -> String {
    let mut texts = Vec::new();
    if let Some(description) = payload.get("description") {
        texts.extend(
            ["desc", "passive", "active"]
                .iter()
                .filter_map(|key| description.get(*key).map(|value| string(Some(value)))),
        );
    }
    if let Some(card) = payload.get("item_card") {
        texts.extend(
            ["Description", "description"]
                .iter()
                .filter_map(|key| card.get(*key).map(|value| string(Some(value)))),
        );
        for info_key in ["Info1", "Info2"] {
            if let Some(info) = card.get(info_key) {
                texts.extend(
                    ["Desc", "Description", "desc"]
                        .iter()
                        .filter_map(|key| info.get(*key).map(|value| string(Some(value)))),
                );
            }
        }
    }
    texts.join(" ")
}

fn card_number(card: Option<&Value>, path: &[&str]) -> f64 {
    let mut value = card;
    for key in path {
        value = value.and_then(|value| value.get(*key));
    }
    number(value)
}

fn card_string(card: Option<&Value>, path: &[&str]) -> String {
    let mut value = card;
    for key in path {
        value = value.and_then(|value| value.get(*key));
    }
    string(value)
}

fn classify_condition(payload: &Value, is_active: bool) -> ConditionKind {
    let description = description_text(payload).to_ascii_lowercase();
    let properties = payload.get("properties").and_then(Value::as_object);
    let cooldown = properties
        .and_then(|properties| properties.get("AbilityCooldown"))
        .map(|value| number(value.as_object().and_then(|value| value.get("value"))))
        .unwrap_or_default();
    if is_active {
        return ConditionKind::ActiveCooldown {
            uptime: if cooldown > 0.0 {
                1.0 / cooldown.max(1.0)
            } else {
                1.0
            },
            cooldown,
        };
    }
    if description.contains("melee") {
        return ConditionKind::MeleeBound;
    }
    if description.contains("bullet") || description.contains("shot") {
        return ConditionKind::ShotBound;
    }
    if description.contains("after") || description.contains("for the next") {
        return ConditionKind::ActionBound {
            action: "activation".to_string(),
        };
    }
    if let Some(seconds) = description
        .split_whitespace()
        .collect::<Vec<_>>()
        .windows(2)
        .find(|words| words[1].starts_with("second"))
        .and_then(|words| words[0].parse::<f64>().ok())
    {
        return ConditionKind::RampUp {
            ramp_seconds: seconds,
        };
    }
    if description.contains("health") && description.contains("above") {
        return ConditionKind::StateBound { threshold: 0.65 };
    }
    ConditionKind::None
}

fn ability_roles(payload: &Value) -> Vec<AbilityRole> {
    let text = description_text(payload).to_ascii_lowercase();
    let class_name = string(payload.get("class_name")).to_ascii_lowercase();
    let mut roles = Vec::new();
    if text.contains("damage") || text.contains("burn") {
        roles.push(AbilityRole::Damage);
    }
    if text.contains("stun")
        || text.contains("slow")
        || text.contains("root")
        || text.contains("bind")
    {
        roles.push(AbilityRole::Control);
    }
    if text.contains("dash") || text.contains("teleport") || class_name.contains("movement") {
        roles.push(AbilityRole::Mobility);
    }
    if text.contains("heal") || text.contains("regenerat") || text.contains("health") {
        roles.push(AbilityRole::Sustain);
    }
    if text.contains("ultimate") || class_name.ends_with("4") {
        roles.push(AbilityRole::Ultimate);
    }
    if roles.is_empty() {
        roles.push(AbilityRole::Utility);
    }
    roles
}

fn scaling_stats(value: Option<&Value>) -> Vec<ScalingStat> {
    value
        .and_then(Value::as_object)
        .map(|object| {
            object
                .iter()
                .map(|(_stat, value)| ScalingStat {
                    stat: string(value.get("scaling_stat").or_else(|| value.get("stat"))),
                    per_level: number(value.get("scale")),
                    per_spirit: value
                        .get("per_spirit")
                        .or_else(|| value.get("spirit_scale"))
                        .map(|value| number(Some(value)))
                        .or_else(|| {
                            let stat_name = string(value.get("scaling_stat"));
                            (!stat_name.is_empty()).then(|| number(value.get("scale")))
                        }),
                })
                .map(|mut stat| {
                    if stat.stat.is_empty() {
                        stat.stat = "unknown".to_string();
                    }
                    stat
                })
                .collect()
        })
        .unwrap_or_default()
}

fn ability_model(payload: &Value, slot: i64) -> Option<AbilityModel> {
    let ability_id = integer(payload.get("id"));
    let class_name = string(payload.get("class_name"));
    if ability_id == 0 || class_name.is_empty() {
        return None;
    }
    let properties = payload.get("properties").and_then(Value::as_object);
    let property_number = |name: &str| {
        properties
            .and_then(|properties| properties.get(name))
            .map(|value| number(value.as_object().and_then(|value| value.get("value"))))
            .unwrap_or_default()
    };
    Some(AbilityModel {
        ability_id,
        class_name,
        slot,
        roles: ability_roles(payload),
        scaling: scaling_stats(payload.get("scaling_stats")),
        channel_time: {
            let value = property_number("AbilityChannelTime");
            (value > 0.0).then_some(value)
        },
        charges: property_number("AbilityCharges") as i64,
        cooldown: property_number("AbilityCooldown"),
        scaling_step: None,
        damage_type: if properties
            .map(|properties| {
                properties
                    .keys()
                    .any(|key| key.contains("BaseAttackDamage"))
            })
            .unwrap_or(false)
        {
            DamageType::Weapon
        } else {
            DamageType::Spirit
        },
    })
}

fn tier_bonuses(value: Option<&Value>) -> Vec<TierBonus> {
    value
        .and_then(Value::as_array)
        .map(|values| {
            values
                .iter()
                .map(|value| TierBonus {
                    tier: integer(value.get("tier")),
                    value: number(value.get("value")),
                    value_type: string(value.get("value_type")),
                })
                .collect()
        })
        .unwrap_or_default()
}

fn purchase_bonuses(payload: &Value) -> PurchaseBonuses {
    let bonuses = payload.get("purchase_bonuses");
    PurchaseBonuses {
        spirit: tier_bonuses(bonuses.and_then(|value| value.get("spirit"))),
        weapon: tier_bonuses(bonuses.and_then(|value| value.get("weapon"))),
        vitality: tier_bonuses(bonuses.and_then(|value| value.get("vitality"))),
    }
}

fn level_curve(payload: &Value) -> Vec<LevelPoint> {
    let mut points = payload
        .get("level_info")
        .and_then(Value::as_object)
        .map(|levels| {
            levels
                .iter()
                .filter_map(|(level, value)| {
                    Some(LevelPoint {
                        level: level.parse().ok()?,
                        required_souls: integer(value.get("required_gold")),
                    })
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    points.sort_by_key(|point| point.level);
    points
}

fn base_health(payload: &Value) -> f64 {
    payload
        .get("starting_stats")
        .and_then(Value::as_object)
        .and_then(|stats| stats.get("max_health"))
        .map(|value| number(value.as_object().and_then(|value| value.get("value"))))
        .or_else(|| payload.get("base_health").map(|value| number(Some(value))))
        .unwrap_or_default()
}

fn weapon_profile(payload: &Value) -> WeaponProfile {
    let info = payload.get("weapon_info").and_then(Value::as_object);
    let get = |names: &[&str]| {
        names
            .iter()
            .find_map(|name| {
                info.and_then(|info| info.get(*name))
                    .map(|value| number(Some(value)))
            })
            .unwrap_or_default()
    };
    WeaponProfile {
        bullet_damage: get(&["bullet_damage"]),
        shots_per_second: get(&["shots_per_second", "bullets_per_second"]),
        clip_size: get(&["clip_size"]),
        reload_duration: get(&["reload_duration"]),
        range: get(&["range"]),
        falloff_start_range: get(&["damage_falloff_start_range", "falloff_start_range"]),
        falloff_end_range: get(&["damage_falloff_end_range", "falloff_end_range"]),
        sustained_dps: get(&["damage_per_second_with_reload", "damage_per_second"]),
    }
}

fn hero_model(payload: &Value, abilities: &[Value], stats: &[ScalingStat]) -> Result<HeroModel> {
    let hero_id = integer(payload.get("id"));
    let name = string(payload.get("name"));
    if hero_id == 0 || name.is_empty() {
        return Err(ReasonerError::Data(
            "Hero-Snapshot enthält keine ID oder keinen Namen".to_string(),
        ));
    }
    let parsed_abilities = abilities
        .iter()
        .enumerate()
        .filter_map(|(slot, payload)| ability_model(payload, (slot + 1) as i64))
        .collect::<Vec<_>>();
    let weapon = weapon_profile(payload);
    let spirit_dps = parsed_abilities
        .iter()
        .filter(|ability| matches!(ability.damage_type, DamageType::Spirit | DamageType::Hybrid))
        .map(|ability| {
            let channel = ability.channel_time.unwrap_or(1.0).max(1.0);
            (ability.cooldown.max(channel) / channel).max(0.0)
        })
        .sum();
    let weapon_dps = if weapon.sustained_dps > 0.0 {
        weapon.sustained_dps
    } else {
        weapon.bullet_damage * weapon.shots_per_second
    };
    let share = if weapon_dps + spirit_dps > 0.0 {
        weapon_dps / (weapon_dps + spirit_dps)
    } else {
        0.0
    };
    let primary_axis = if share > 0.6 {
        DamageType::Weapon
    } else if share < 0.4 {
        DamageType::Spirit
    } else {
        DamageType::Hybrid
    };
    let mut all_scaling = scaling_stats(payload.get("scaling_stats"));
    all_scaling.extend(stats.iter().cloned());
    Ok(HeroModel {
        hero_id,
        name,
        archetype: string(payload.get("hero_type")),
        base_health: base_health(payload),
        level_curve: level_curve(payload),
        purchase_bonuses: purchase_bonuses(payload),
        scaling: all_scaling,
        weapon,
        abilities: parsed_abilities,
        damage_plan: DamagePlan {
            weapon_dps,
            spirit_dps,
            weapon_share: share,
            primary_axis,
        },
    })
}

async fn snapshot(pool: &PgPool, entity_type: &str, name: &str) -> Result<Value> {
    let pattern = format!("%{}%", name.replace('%', ""));
    let row = sqlx::query(
        "SELECT payload::text AS payload_json FROM brain.entity_snapshots WHERE source='deadlock_assets_api' AND entity_type=$1 AND (lower(canonical_name)=lower($2) OR lower(payload->>'name')=lower($2) OR canonical_name ILIKE $3) ORDER BY fetched_at DESC, id DESC LIMIT 1",
    )
    .bind(entity_type)
    .bind(name)
    .bind(pattern)
    .fetch_optional(pool)
    .await
    .map_err(ReasonerError::Db)?;
    let row = row.ok_or_else(|| ReasonerError::MissingSnapshot(name.to_string()))?;
    let text = row
        .try_get::<String, _>("payload_json")
        .map_err(ReasonerError::Db)?;
    parse_json(text, name)
}

async fn ability_snapshots(pool: &PgPool, hero: &Value) -> Result<Vec<Value>> {
    let names = hero
        .get("items")
        .and_then(Value::as_object)
        .into_iter()
        .flat_map(|items| {
            ["signature1", "signature2", "signature3", "signature4"]
                .into_iter()
                .filter_map(|key| items.get(key).map(|value| string(Some(value))))
        })
        .filter(|name| !name.is_empty())
        .collect::<Vec<_>>();
    let mut abilities = Vec::new();
    for name in names {
        let row = sqlx::query("SELECT payload::text AS payload_json FROM brain.entity_snapshots WHERE source='deadlock_assets_api' AND entity_type='item_or_ability' AND payload->>'class_name'=$1 ORDER BY fetched_at DESC, id DESC LIMIT 1")
            .bind(name)
            .fetch_optional(pool)
            .await
            .map_err(ReasonerError::Db)?;
        if let Some(row) = row {
            let text = row
                .try_get::<String, _>("payload_json")
                .map_err(ReasonerError::Db)?;
            abilities.push(parse_json(text, "Ability")?);
        }
    }
    Ok(abilities)
}

async fn hero_name(pool: &PgPool, hero_id: i64) -> Result<String> {
    let value = sqlx::query_scalar::<_, Option<String>>(
        "SELECT canonical_name FROM brain.entities WHERE entity_type='hero' AND id=$1",
    )
    .bind(hero_id)
    .fetch_optional(pool)
    .await
    .map_err(ReasonerError::Db)?
    .flatten();
    if let Some(value) = value {
        return Ok(value);
    }
    let value = sqlx::query_scalar::<_, Option<String>>(
        "SELECT payload->>'name' FROM brain.entity_snapshots WHERE source='deadlock_assets_api' AND entity_type='hero' AND (payload->>'id')::bigint=$1 ORDER BY fetched_at DESC, id DESC LIMIT 1",
    )
    .bind(hero_id)
    .fetch_optional(pool)
    .await
    .map_err(ReasonerError::Db)?
    .flatten();
    value.ok_or_else(|| ReasonerError::HeroNotFound(hero_id.to_string()))
}

pub async fn load_hero_model(ctx: &ReasonerCtx, hero: &str) -> Result<HeroModel> {
    let payload = snapshot(&ctx.pool, "hero", hero).await?;
    let abilities = ability_snapshots(&ctx.pool, &payload).await?;
    let stats = load_hero_stat_values(ctx, integer(payload.get("id"))).await?;
    hero_model(&payload, &abilities, &stats)
}

pub async fn load_item_models(ctx: &ReasonerCtx) -> Result<Vec<ItemModel>> {
    let catalog_rows = sqlx::query(
        "SELECT item_id, name, slot_type, tier, defense_kind::text AS defense_kind_json, damage_axis FROM brain.item_catalog ORDER BY item_id",
    )
    .fetch_all(&ctx.pool)
    .await
    .map_err(ReasonerError::Db)?;
    let mut catalog = BTreeMap::new();
    for row in catalog_rows {
        let item_id = row
            .try_get::<i64, _>("item_id")
            .map_err(ReasonerError::Db)?;
        let defense: Vec<String> = row
            .try_get::<String, _>("defense_kind_json")
            .ok()
            .and_then(|value| serde_json::from_str(&value).ok())
            .unwrap_or_default();
        catalog.insert(
            item_id,
            (
                row.try_get::<String, _>("name")
                    .map_err(ReasonerError::Db)?,
                row.try_get::<String, _>("slot_type")
                    .map_err(ReasonerError::Db)?,
                row.try_get::<i64, _>("tier").map_err(ReasonerError::Db)?,
                defense,
                row.try_get::<String, _>("damage_axis")
                    .map_err(ReasonerError::Db)?,
            ),
        );
    }
    let rows = sqlx::query("SELECT payload::text AS payload_json FROM brain.entity_snapshots WHERE source='deadlock_assets_api' AND entity_type='item_or_ability' ORDER BY fetched_at DESC, id DESC")
        .fetch_all(&ctx.pool)
        .await
        .map_err(ReasonerError::Db)?;
    let card_rows = sqlx::query("SELECT external_id, canonical_name, payload::text AS payload_json FROM brain.entity_snapshots WHERE source='deadlock_data' AND entity_type='item_card' ORDER BY fetched_at DESC, id DESC")
        .fetch_all(&ctx.pool)
        .await
        .map_err(ReasonerError::Db)?;
    let mut cards = BTreeMap::new();
    for row in card_rows {
        let payload = parse_json(
            row.try_get::<String, _>("payload_json")
                .map_err(ReasonerError::Db)?,
            "Item-Card",
        )?;
        for key in [
            row.try_get::<Option<String>, _>("external_id")
                .map_err(ReasonerError::Db)?,
            row.try_get::<Option<String>, _>("canonical_name")
                .map_err(ReasonerError::Db)?,
            payload.get("Key").map(|value| string(Some(value))),
            payload.get("class_name").map(|value| string(Some(value))),
        ]
        .into_iter()
        .flatten()
        .filter(|key| !key.is_empty())
        {
            cards
                .entry(key.to_ascii_lowercase())
                .or_insert_with(|| payload.clone());
        }
    }
    let mut seen = BTreeSet::new();
    let mut items = Vec::new();
    for row in rows {
        let text = row
            .try_get::<String, _>("payload_json")
            .map_err(ReasonerError::Db)?;
        let payload = parse_json(text, "Item-Snapshot")?;
        let item_id = integer(payload.get("id"));
        if item_id == 0
            || !seen.insert(item_id)
            || payload.get("type").and_then(Value::as_str) == Some("ability")
        {
            continue;
        }
        let Some((catalog_name, catalog_slot, catalog_tier, defense_kind, catalog_axis)) =
            catalog.get(&item_id)
        else {
            continue;
        };
        let class_name = string(payload.get("class_name"));
        let item_card = cards.get(&class_name.to_ascii_lowercase());
        let mut merged_payload = payload.clone();
        if let Some(object) = merged_payload.as_object_mut() {
            if let Some(card) = item_card {
                object.insert("item_card".to_string(), card.clone());
            }
        }
        let properties = property_values(payload.get("properties"));
        let passive_properties = passive_property_values(payload.get("properties"));
        let proc_cooldown = properties
            .iter()
            .find(|(name, _)| name.to_ascii_lowercase().contains("proccooldown"))
            .map(|(_, value)| *value)
            .filter(|value| *value > 0.0)
            .or_else(|| {
                let value = card_number(item_card, &["Info2", "Cooldown"]);
                (value > 0.0).then_some(value)
            });
        let cost = integer(payload.get("cost"));
        let cost = if cost > 0 {
            cost
        } else {
            card_number(item_card, &["Cost"]) as i64
        };
        let is_active = payload
            .get("is_active_item")
            .and_then(Value::as_bool)
            .unwrap_or_else(|| {
                card_string(item_card, &["Info2", "Type"]).eq_ignore_ascii_case("active")
            });
        let shopable = payload
            .get("shopable")
            .and_then(Value::as_bool)
            .unwrap_or(true);
        let disabled = payload
            .get("disabled")
            .and_then(Value::as_bool)
            .or_else(|| payload.get("IsDisabled").and_then(Value::as_bool))
            .or_else(|| {
                item_card
                    .and_then(|card| card.get("IsDisabled"))
                    .and_then(Value::as_bool)
            })
            .unwrap_or(false);
        items.push(ItemModel {
            item_id,
            name: if catalog_name.is_empty() {
                string(payload.get("name"))
            } else {
                catalog_name.clone()
            },
            slot: slot_type(catalog_slot),
            tier: if *catalog_tier == 0 {
                integer(payload.get("item_tier"))
            } else {
                *catalog_tier
            },
            cost,
            is_active,
            shopable,
            disabled,
            damage_axis: damage_type(catalog_axis),
            defense_kind: defense_kind.clone(),
            properties,
            passive_properties,
            condition: classify_condition(&merged_payload, is_active),
            proc_cooldown,
            imbueable: payload.get("imbue").is_some()
                || item_card
                    .and_then(|card| card.get("IsImbue"))
                    .and_then(Value::as_bool)
                    .unwrap_or(false)
                || description_text(&merged_payload)
                    .to_ascii_lowercase()
                    .contains("imbued"),
        });
    }
    Ok(items)
}

pub async fn load_meta_rows(ctx: &ReasonerCtx, hero_id: i64) -> Result<Vec<MetaRow>> {
    let rows = sqlx::query("SELECT item_id, patch_tag, prevalence_builds, wins, losses, matches, avg_buy_time_relative, lift_pp FROM brain.hero_item_stats WHERE hero_id=$1 AND bracket=$2 AND patch_tag=$3 ORDER BY item_id")
        .bind(hero_id)
        .bind(&ctx.config.bracket)
        .bind(&ctx.config.patch_tag)
        .fetch_all(&ctx.pool)
        .await
        .map_err(ReasonerError::Db)?;
    rows.into_iter()
        .map(|row| {
            Ok(MetaRow {
                item_id: row.try_get("item_id").map_err(ReasonerError::Db)?,
                patch_tag: row.try_get("patch_tag").map_err(ReasonerError::Db)?,
                prevalence_builds: row
                    .try_get("prevalence_builds")
                    .map_err(ReasonerError::Db)?,
                wins: row.try_get("wins").map_err(ReasonerError::Db)?,
                losses: row.try_get("losses").map_err(ReasonerError::Db)?,
                matches: row.try_get("matches").map_err(ReasonerError::Db)?,
                avg_buy_time_relative: row
                    .try_get("avg_buy_time_relative")
                    .map_err(ReasonerError::Db)?,
                lift_pp: row.try_get("lift_pp").map_err(ReasonerError::Db)?,
            })
        })
        .collect()
}

pub async fn load_patch_events(ctx: &ReasonerCtx, hero_id: i64) -> Result<Vec<Value>> {
    let name = hero_name(&ctx.pool, hero_id).await?;
    let rows = sqlx::query("SELECT (to_jsonb(pe) || jsonb_build_object('enrichment', COALESCE(to_jsonb(pee), '{}'::jsonb)))::text AS row_json FROM brain.patch_events pe LEFT JOIN brain.patch_event_enrichments pee ON pee.patch_event_id=pe.id WHERE lower(pe.entity_name)=lower($1) OR lower(pee.secondary_entity_name)=lower($1) ORDER BY pe.posted_at DESC NULLS LAST, pe.id DESC")
        .bind(name)
        .fetch_all(&ctx.pool)
        .await
        .map_err(ReasonerError::Db)?;
    rows.into_iter()
        .map(|row| {
            let text = row
                .try_get::<String, _>("row_json")
                .map_err(ReasonerError::Db)?;
            parse_json(text, "Patch-Event")
        })
        .collect()
}

fn ids_from_details(details: &Value) -> (Vec<i64>, Vec<i64>) {
    let mut core = Vec::new();
    let mut order = Vec::new();
    for category in details
        .get("mod_categories")
        .or_else(|| details.get("modCategories"))
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        let optional = category
            .get("optional")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let category_name = string(category.get("name")).to_ascii_lowercase();
        for item in category
            .get("mods")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            let id = item
                .get("ability_id")
                .or_else(|| item.get("abilityId"))
                .map(|value| integer(Some(value)))
                .unwrap_or_default();
            if id == 0 {
                continue;
            }
            if !optional || category_name.contains("core") {
                core.push(id);
            }
            order.push(id);
        }
    }
    let ability_order = details
        .get("ability_order")
        .or_else(|| details.get("abilityOrder"))
        .and_then(|value| {
            value
                .get("currency_changes")
                .or_else(|| value.get("currencyChanges"))
        })
        .and_then(Value::as_array);
    if let Some(changes) = ability_order {
        for change in changes {
            let id = change
                .get("ability_id")
                .or_else(|| change.get("abilityId"))
                .map(|value| integer(Some(value)))
                .unwrap_or_default();
            if id != 0 {
                order.push(id);
            }
        }
    }
    (core, order)
}

fn author_build(value: &Value) -> AuthorBuild {
    let details = value.get("details").unwrap_or(value);
    let (core_item_ids, buy_order) = ids_from_details(details);
    AuthorBuild {
        author: ["author", "author_name", "authorName", "creator_name"]
            .iter()
            .find_map(|key| value.get(*key).map(|value| string(Some(value))))
            .unwrap_or_else(|| "unbekannt".to_string()),
        version: integer(value.get("version")),
        published_at: value
            .get("published_at")
            .or_else(|| value.get("publish_ts"))
            .map(|value| integer(Some(value))),
        last_updated_at: value
            .get("last_updated_at")
            .or_else(|| value.get("last_updated_ts"))
            .or_else(|| value.get("last_seen_at"))
            .map(|value| integer(Some(value))),
        patch_tag: ["patch_tag", "patchTag"]
            .iter()
            .find_map(|key| value.get(*key).map(|value| string(Some(value)))),
        core_item_ids,
        buy_order,
    }
}

pub async fn load_author_builds(
    ctx: &ReasonerCtx,
    hero_id: i64,
) -> Result<Vec<crate::AuthorBuild>> {
    let table = if table_exists(&ctx.pool, "tierlist.hero_build_sources").await? {
        "tierlist.hero_build_sources"
    } else if table_exists(&ctx.pool, "hero_build_sources").await? {
        "hero_build_sources"
    } else {
        return Ok(Vec::new());
    };
    let query = format!("SELECT to_jsonb(hbs)::text AS row_json FROM {table} hbs WHERE hbs.hero_id=$1 ORDER BY COALESCE(hbs.last_seen_at, hbs.fetched_at, hbs.last_updated_ts, hbs.publish_ts, 0) DESC LIMIT 100");
    let rows = sqlx::query(&query)
        .bind(hero_id)
        .fetch_all(&ctx.pool)
        .await
        .map_err(ReasonerError::Db)?;
    rows.into_iter()
        .map(|row| {
            let text = row
                .try_get::<String, _>("row_json")
                .map_err(ReasonerError::Db)?;
            Ok(author_build(&parse_json(text, "Autoren-Build")?))
        })
        .collect()
}

pub async fn load_claims(ctx: &ReasonerCtx, hero_id: i64) -> Result<Vec<Value>> {
    let name = hero_name(&ctx.pool, hero_id).await?;
    let mut claims = Vec::new();
    for table in ["brain.youtube_learning_claims", "brain.forum_claims"] {
        if !table_exists(&ctx.pool, table).await? {
            continue;
        }
        let query = format!("SELECT to_jsonb(c)::text AS row_json FROM {table} c WHERE lower(c.entity_name)=lower($1) OR lower(c.entity_name) LIKE lower($2) ORDER BY c.id DESC LIMIT 100");
        let rows = sqlx::query(&query)
            .bind(&name)
            .bind(format!("%{}%", name))
            .fetch_all(&ctx.pool)
            .await
            .map_err(ReasonerError::Db)?;
        for row in rows {
            let text = row
                .try_get::<String, _>("row_json")
                .map_err(ReasonerError::Db)?;
            claims.push(parse_json(text, "Claim")?);
        }
    }
    Ok(claims)
}

pub async fn load_hero_stat_values(ctx: &ReasonerCtx, hero_id: i64) -> Result<Vec<ScalingStat>> {
    if !table_exists(&ctx.pool, "brain.hero_stat_values").await? {
        return Ok(Vec::new());
    }
    let rows = sqlx::query("SELECT stat_key, COALESCE(numeric_value, 0.0)::float8 AS numeric_value FROM brain.hero_stat_values WHERE entity_id=$1 OR profile_id IN (SELECT id FROM brain.hero_stat_profiles WHERE entity_id=$1) ORDER BY stat_key")
        .bind(hero_id)
        .fetch_all(&ctx.pool)
        .await
        .map_err(ReasonerError::Db)?;
    rows.into_iter()
        .map(|row| {
            Ok(ScalingStat {
                stat: row.try_get("stat_key").map_err(ReasonerError::Db)?,
                per_level: 0.0,
                per_spirit: Some(row.try_get("numeric_value").map_err(ReasonerError::Db)?),
            })
        })
        .collect()
}

pub async fn load_synergies(ctx: &ReasonerCtx, hero_id: i64) -> Result<Vec<Value>> {
    if !table_exists(&ctx.pool, "brain.hero_item_synergies").await? {
        return Ok(Vec::new());
    }
    let rows = sqlx::query("SELECT jsonb_build_object('hero_id', hero_id, 'item_id', item_id, 'with_item_id', with_item_id, 'wins', wins, 'losses', losses, 'matches', matches, 'patch_tag', patch_tag)::text AS row_json FROM brain.hero_item_synergies WHERE hero_id=$1 AND patch_tag=$2 ORDER BY matches DESC, item_id, with_item_id")
        .bind(hero_id)
        .bind(&ctx.config.patch_tag)
        .fetch_all(&ctx.pool)
        .await
        .map_err(ReasonerError::Db)?;
    rows.into_iter()
        .map(|row| {
            let text = row
                .try_get::<String, _>("row_json")
                .map_err(ReasonerError::Db)?;
            parse_json(text, "Synergie")
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;

    #[test]
    fn parses_purchase_bonuses_with_string_values() {
        let payload = serde_json::json!({
            "purchase_bonuses": {"spirit": [{"tier": 1, "value": "4", "value_type": "MODIFIER_VALUE_TECH_POWER"}]},
            "level_info": {"1": {"required_gold": 0}},
            "id": 25,
            "name": "Warden",
            "hero_type": "brawler"
        });
        let hero = hero_model(&payload, &[], &[]).unwrap();
        assert_eq!(hero.purchase_bonuses.spirit[0].value, 4.0);
        assert_eq!(hero.level_curve[0].required_souls, 0);
    }

    #[tokio::test]
    #[ignore = "benötigt echten Postgres-Snapshot über DEADLOCK_CENTRAL_DSN"]
    async fn loads_warden_and_reference_items_from_real_snapshot() {
        let Ok(dsn) = std::env::var("DEADLOCK_CENTRAL_DSN") else {
            return;
        };
        let pool = PgPoolOptions::new()
            .max_connections(2)
            .connect(&dsn)
            .await
            .unwrap();
        let ctx = ReasonerCtx {
            pool,
            ai: None,
            config: crate::ReasonerConfig::default(),
        };
        let hero = load_hero_model(&ctx, "Warden").await.unwrap();
        assert_eq!(hero.hero_id, 25);
        let items = load_item_models(&ctx).await.unwrap();
        for name in [
            "Veil Walker",
            "Mercurial Magnum",
            "Siphon Bullets",
            "Quicksilver Reload",
        ] {
            assert!(items.iter().any(|item| item.name == name), "{name}");
        }
    }
}
