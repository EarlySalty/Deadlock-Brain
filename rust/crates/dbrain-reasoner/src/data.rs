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

fn number(value: Option<&Value>) -> Option<f64> {
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
        .filter(|value| value.is_finite())
}

fn integer(value: Option<&Value>) -> i64 {
    number(value).unwrap_or_default() as i64
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
            number(Some(raw)).map(|value| (name.clone(), value))
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
            number(Some(raw)).map(|value| (name.clone(), value))
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

fn card_number(card: Option<&Value>, path: &[&str]) -> Option<f64> {
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

fn is_imbue_marker(value: Option<&Value>) -> bool {
    value.is_some_and(|value| !value.is_null() && value.as_bool() != Some(false))
}

fn classify_condition(payload: &Value, is_active: bool) -> ConditionKind {
    let description = description_text(payload).to_ascii_lowercase();
    let properties = payload.get("properties").and_then(Value::as_object);
    let cooldown = properties
        .and_then(|properties| properties.get("AbilityCooldown"))
        .and_then(|value| number(value.as_object().and_then(|value| value.get("value"))))
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
                .map(|(stat, value)| ScalingStat {
                    stat: stat.clone(),
                    per_level: number(value.get("per_level")).unwrap_or_default(),
                    per_spirit: number(value.get("per_spirit"))
                        .or_else(|| number(value.get("spirit_scale")))
                        .or_else(|| {
                            let stat_name = string(value.get("scaling_stat"));
                            if !stat_name.eq_ignore_ascii_case("ETechPower") {
                                None
                            } else {
                                number(value.get("scale"))
                            }
                        }),
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
    let values = property_values(payload.get("properties"));
    let property_number = |name: &str| values.get(name).copied().unwrap_or_default();
    let channel_time = property_number("AbilityChannelTime");
    let duration = property_number("AbilityDuration").max(channel_time);
    let tick_rate = values
        .iter()
        .filter(|(key, value)| {
            (key.ends_with("TickRate") || key.as_str() == "PulseInterval") && **value > 0.0
        })
        .map(|(_, value)| *value)
        .min_by(f64::total_cmp);
    let base_effect = values
        .iter()
        .map(|(key, value)| match key.as_str() {
            "Damage" | "ImpactDamage" | "BaseDamage" => *value,
            "PulseDPS" | "DamagePerSecond" | "DPS" => value * duration,
            "DamagePerTick" | "TickDamage" => tick_rate
                .map(|tick| value * duration / tick)
                .unwrap_or_default(),
            _ => 0.0,
        })
        .sum();
    let mut scaling = scaling_stats(payload.get("scaling_stats"));
    if let Some(properties) = properties {
        for (name, property) in properties {
            let Some(function) = property.get("scale_function") else {
                continue;
            };
            let Some(scale) = number(function.get("stat_scale")) else {
                continue;
            };
            if scaling.iter().any(|stat| stat.stat == *name) {
                continue;
            }
            let input = string(function.get("specific_stat_scale_type"));
            let class = string(function.get("class_name"));
            if input == "ETechPower" || class == "scale_function_tech_damage" {
                scaling.push(ScalingStat {
                    stat: name.clone(),
                    per_level: 0.0,
                    per_spirit: Some(scale),
                });
            }
        }
    }
    Some(AbilityModel {
        ability_id,
        class_name,
        slot,
        roles: ability_roles(payload),
        scaling,
        channel_time: {
            let value = property_number("AbilityChannelTime");
            (value > 0.0).then_some(value)
        },
        charges: property_number("AbilityCharges") as i64,
        cooldown: if property_number("AbilityCooldownBetweenCharge") > 0.0 {
            property_number("AbilityCooldownBetweenCharge")
        } else {
            property_number("AbilityCooldown")
        },
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
        base_effect,
        tick_rate,
        duration: (duration > 0.0).then_some(duration),
    })
}

pub(crate) fn ability_cast_count(ability: &AbilityModel, cfg: &crate::ReasonerConfig) -> f64 {
    let window = cfg.combat_window_seconds.max(0.0);
    let channel = ability.channel_time.unwrap_or(1.0).max(0.001);
    let recharge = ability.cooldown.max(channel);
    (ability.charges.max(1) as f64 + window / recharge)
        .min(window * cfg.channel_uptime.clamp(0.0, 1.0) / channel)
}

pub(crate) fn ability_base_dps(ability: &AbilityModel, cfg: &crate::ReasonerConfig) -> f64 {
    ability.base_effect * ability_cast_count(ability, cfg) / cfg.combat_window_seconds.max(1.0)
}

fn tier_bonuses(value: Option<&Value>) -> Vec<TierBonus> {
    value
        .and_then(Value::as_array)
        .map(|values| {
            values
                .iter()
                .map(|value| TierBonus {
                    tier: integer(value.get("tier")),
                    value: number(value.get("value")).unwrap_or_default(),
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
        .and_then(|value| number(value.as_object().and_then(|value| value.get("value"))))
        .or_else(|| number(payload.get("base_health")))
        .unwrap_or_default()
}

fn weapon_profile(payload: &Value) -> WeaponProfile {
    let info = payload.get("weapon_info").and_then(Value::as_object);
    let get = |names: &[&str]| {
        names
            .iter()
            .find_map(|name| number(info.and_then(|info| info.get(*name))))
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
        .map(|ability| ability_base_dps(ability, &crate::ReasonerConfig::default()))
        .sum();
    let weapon_dps = if weapon.sustained_dps > 0.0 {
        weapon.sustained_dps
    } else {
        let firing = weapon.clip_size / weapon.shots_per_second;
        if firing.is_finite() && firing > 0.0 {
            weapon.bullet_damage * weapon.clip_size / (firing + weapon.reload_duration.max(0.0))
        } else {
            0.0
        }
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
        "SELECT payload::text AS payload_json FROM brain.entity_snapshots WHERE source='deadlock_assets_api' AND entity_type=$1 AND (lower(canonical_name)=lower($2) OR lower(payload->>'name')=lower($2) OR canonical_name ILIKE $3) ORDER BY (lower(canonical_name)=lower($2)) DESC NULLS LAST, (lower(payload->>'name')=lower($2)) DESC NULLS LAST, fetched_at DESC, id DESC LIMIT 1",
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
        let row = row.ok_or_else(|| ReasonerError::MissingSnapshot("Signatur-Ability".into()))?;
        {
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
        "SELECT canonical_name FROM brain.entities WHERE entity_type='hero' AND primary_external_id=$1",
    )
    .bind(hero_id.to_string())
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
    let mut payload = snapshot(&ctx.pool, "hero", hero).await?;
    if payload.get("weapon_info").is_none_or(Value::is_null) {
        if let Some(name) = payload
            .pointer("/items/weapon_primary")
            .and_then(Value::as_str)
        {
            let weapon: Option<String> = sqlx::query_scalar("SELECT payload::text FROM brain.entity_snapshots WHERE source='deadlock_assets_api' AND entity_type='item_or_ability' AND payload->>'class_name'=$1 ORDER BY fetched_at DESC,id DESC LIMIT 1")
                .bind(name).fetch_optional(&ctx.pool).await.map_err(ReasonerError::Db)?;
            let weapon = parse_json(
                weapon.ok_or_else(|| ReasonerError::MissingSnapshot("Primärwaffe".into()))?,
                "Primärwaffe",
            )?;
            payload["weapon_info"] = weapon
                .get("weapon_info")
                .filter(|value| value.is_object())
                .ok_or_else(|| ReasonerError::Data("weapon_info der Primärwaffe fehlt".into()))?
                .clone();
        }
    }
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
                card_number(item_card, &["Info2", "Cooldown"]).filter(|value| *value > 0.0)
            });
        let cost = integer(payload.get("cost"));
        let cost = if cost > 0 {
            cost
        } else {
            card_number(item_card, &["Cost"]).unwrap_or_default() as i64
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
            imbueable: is_imbue_marker(payload.get("imbue"))
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
    let mut names = sqlx::query_scalar::<_, String>(
        "SELECT lower(e.canonical_name) FROM brain.entities e WHERE e.entity_type='hero' AND e.primary_external_id=$1 UNION SELECT lower(a.alias) FROM brain.entities e JOIN brain.entity_aliases a ON a.entity_id=e.id WHERE e.entity_type='hero' AND e.primary_external_id=$1 AND a.alias_kind IN ('canonical', 'snapshot_name', 'class_name_short')",
    )
    .bind(hero_id.to_string())
    .fetch_all(&ctx.pool)
    .await
    .map_err(ReasonerError::Db)?;
    if names.is_empty() {
        names.push(hero_name(&ctx.pool, hero_id).await?.to_lowercase());
    }
    let rows = sqlx::query("SELECT (to_jsonb(pe) || jsonb_build_object('enrichment', COALESCE(to_jsonb(pee), '{}'::jsonb)))::text AS row_json FROM brain.patch_events pe LEFT JOIN brain.patch_event_enrichments pee ON pee.patch_event_id=pe.id WHERE lower(pe.entity_name)=ANY($1) OR lower(pee.secondary_entity_name)=ANY($1) ORDER BY pe.posted_at DESC NULLS LAST, pe.id DESC, to_jsonb(pee)::text")
        .bind(names)
        .fetch_all(&ctx.pool)
        .await
        .map_err(ReasonerError::Db)?;
    let mut seen = BTreeSet::new();
    rows.into_iter()
        .map(|row| {
            let text = row
                .try_get::<String, _>("row_json")
                .map_err(ReasonerError::Db)?;
            parse_json(text, "Patch-Event")
        })
        .filter(|event| match event {
            Ok(event) => seen.insert(integer(event.get("id"))),
            Err(_) => true,
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
        published_at: value.get("published_at").and_then(Value::as_i64),
        last_updated_at: value.get("last_updated_at").and_then(Value::as_i64),
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
    let rows = sqlx::query("SELECT (to_jsonb(hbs) || jsonb_build_object('published_at', EXTRACT(EPOCH FROM hbs.published_at)::bigint, 'last_updated_at', EXTRACT(EPOCH FROM hbs.last_updated_at)::bigint))::text AS row_json FROM tierlist.hero_build_sources hbs WHERE hbs.hero_id=$1 ORDER BY COALESCE(hbs.last_updated_at, hbs.published_at) DESC NULLS LAST, hbs.version DESC NULLS LAST LIMIT 100")
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
    let rows = sqlx::query("SELECT DISTINCT v.stat_key, v.numeric_value::float8 AS numeric_value FROM brain.hero_stat_values v WHERE (v.entity_id IN (SELECT id FROM brain.entities WHERE entity_type='hero' AND primary_external_id=$1) OR v.profile_id IN (SELECT p.id FROM brain.hero_stat_profiles p JOIN brain.entities e ON e.id=p.entity_id WHERE e.entity_type='hero' AND e.primary_external_id=$1)) AND v.numeric_value IS NOT NULL ORDER BY v.stat_key, numeric_value")
        .bind(hero_id.to_string())
        .fetch_all(&ctx.pool)
        .await
        .map_err(ReasonerError::Db)?;
    let mut stats = Vec::new();
    for row in rows {
        let stat: String = row.try_get("stat_key").map_err(ReasonerError::Db)?;
        let value: f64 = row.try_get("numeric_value").map_err(ReasonerError::Db)?;
        if stat.starts_with("spirit_scaling.") {
            stats.push(ScalingStat {
                stat,
                per_level: 0.0,
                per_spirit: Some(value),
            });
        } else if stat.ends_with("_per_level") || stat.ends_with("_per_boon") {
            stats.push(ScalingStat {
                stat,
                per_level: value,
                per_spirit: None,
            });
        }
    }
    Ok(stats)
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
    fn null_or_false_imbue_marker_is_not_imbueable() {
        assert!(!is_imbue_marker(None));
        assert!(!is_imbue_marker(Some(&serde_json::json!(null))));
        assert!(!is_imbue_marker(Some(&serde_json::json!(false))));
        assert!(is_imbue_marker(Some(&serde_json::json!(true))));
    }

    #[test]
    fn ability_damage_and_tick_fields_preserve_units() {
        let payload = serde_json::json!({"id":1,"class_name":"channel","properties":{"PulseDPS":{"value":70},"PulseInterval":{"value":0.5},"AbilityDuration":{"value":6},"AbilityCooldown":{"value":180}}});
        let ability = ability_model(&payload, 4).unwrap();
        assert_eq!(ability.base_effect, 420.0);
        assert_eq!(ability.tick_rate, Some(0.5));
        assert_eq!(ability.duration, Some(6.0));
        assert!(
            (ability_base_dps(&ability, &crate::ReasonerConfig::default())
                - 420.0 * (1.0 + 40.0 / 180.0) / 40.0)
                .abs()
                < 1e-12
        );
    }

    #[test]
    fn numeric_properties_keep_zero_and_skip_invalid_values() {
        let properties = serde_json::json!({
            "passive_zero": {"value": "0"},
            "passive_percent": {"value": "4%"},
            "passive_number": 2.5,
            "passive_invalid": {"value": "unknown"},
            "passive_missing": {"tooltip_section": "passive"},
            "passive_null": null,
            "passive_nan": "NaN",
            "passive_infinite": "inf"
        });
        let expected = BTreeMap::from([
            ("passive_zero".to_string(), 0.0),
            ("passive_percent".to_string(), 4.0),
            ("passive_number".to_string(), 2.5),
        ]);
        assert_eq!(property_values(Some(&properties)), expected);
        assert_eq!(passive_property_values(Some(&properties)), expected);
    }

    #[test]
    fn scaling_preserves_unknown_and_zero_values() {
        let source = serde_json::json!({"EFireRate":{"scaling_stat":"ETechPower","scale":0.25},"ERoundsPerSecond":{"scaling_stat":"ETechPower","scale":0.01}});
        let stats = scaling_stats(Some(&source));
        assert_eq!(stats[0].stat, "EFireRate");
        assert_eq!(stats[0].per_level, 0.0);
        assert_eq!(stats[0].per_spirit, Some(0.25));
        assert_eq!(stats[1].stat, "ERoundsPerSecond");
        for value in [serde_json::json!(null), serde_json::json!("unknown")] {
            let payload = serde_json::json!({"test": {"stat": "damage", "per_spirit": value}});
            assert_eq!(scaling_stats(Some(&payload))[0].per_spirit, None);
        }
        let missing = serde_json::json!({"test": {"scaling_stat": "damage"}});
        assert_eq!(scaling_stats(Some(&missing))[0].per_spirit, None);
        let zero =
            serde_json::json!({"test": {"scaling_stat": "damage", "per_spirit": "0", "scale": 2}});
        assert_eq!(scaling_stats(Some(&zero))[0].per_spirit, Some(0.0));
        let fallback = serde_json::json!({"test": {"scaling_stat": "damage", "per_spirit": "unknown", "spirit_scale": "3", "scale": 2}});
        assert_eq!(scaling_stats(Some(&fallback))[0].per_spirit, Some(3.0));
    }

    #[test]
    fn hero_damage_uses_damage_and_falls_with_longer_cooldown() {
        let payload = serde_json::json!({"id":25,"name":"Warden","weapon_info":{"bullet_damage":10,"shots_per_second":5,"clip_size":20,"reload_duration":2}});
        let mut ability = serde_json::json!({"id":1,"class_name":"test","properties":{"Damage":{"value":60},"AbilityCooldown":{"value":30}}});
        let fast = hero_model(&payload, &[ability.clone()], &[]).unwrap();
        assert!((fast.damage_plan.spirit_dps - 3.5).abs() < 1e-12);
        ability["properties"]["AbilityCooldown"]["value"] = serde_json::json!(60);
        let slow = hero_model(&payload, &[ability], &[]).unwrap();
        assert!(slow.damage_plan.spirit_dps < fast.damage_plan.spirit_dps);
        assert_eq!(fast.damage_plan.primary_axis, DamageType::Weapon);
    }

    #[test]
    fn invalid_numbers_allow_snapshot_fallbacks() {
        let mut payload = serde_json::json!({
            "starting_stats": {"max_health": {"value": "unknown"}},
            "base_health": 650,
            "weapon_info": {"shots_per_second": null, "bullets_per_second": "4"}
        });
        assert_eq!(base_health(&payload), 650.0);
        assert_eq!(weapon_profile(&payload).shots_per_second, 4.0);
        payload["starting_stats"]["max_health"]["value"] = serde_json::json!(0);
        payload["weapon_info"]["shots_per_second"] = serde_json::json!("0");
        assert_eq!(base_health(&payload), 0.0);
        assert_eq!(weapon_profile(&payload).shots_per_second, 0.0);
    }

    static SCRATCH_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

    async fn scratch_context() -> (tokio::sync::MutexGuard<'static, ()>, ReasonerCtx) {
        let guard = SCRATCH_LOCK.lock().await;
        let dsn = std::env::var("REASONER_SCRATCH_DSN").expect("REASONER_SCRATCH_DSN fehlt");
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&dsn)
            .await
            .unwrap();
        let database: String = sqlx::query_scalar("SELECT current_database()")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(database, "reasoner_a_fix");
        sqlx::raw_sql("DROP SCHEMA IF EXISTS brain CASCADE; DROP SCHEMA IF EXISTS tierlist CASCADE; CREATE SCHEMA brain; CREATE SCHEMA tierlist;
            CREATE TABLE brain.entities (id bigint, entity_type text, canonical_name text, primary_external_id text);
            CREATE TABLE brain.entity_aliases (entity_id bigint, alias text, alias_kind text);
            CREATE TABLE brain.entity_snapshots (id bigint, source text, entity_type text, canonical_name text, payload jsonb, fetched_at timestamptz);
            INSERT INTO brain.entities VALUES (175035, 'hero', 'Warden', '25'), (25, 'hero', 'Wrong Hero', '999');")
            .execute(&pool).await.unwrap();
        (
            guard,
            ReasonerCtx {
                pool,
                ai: None,
                config: crate::ReasonerConfig::default(),
            },
        )
    }

    #[tokio::test]
    #[ignore = "benötigt isolierten Postgres über REASONER_SCRATCH_DSN"]
    async fn patch_events_resolve_allowed_aliases_for_external_hero_id() {
        let (_guard, ctx) = scratch_context().await;
        sqlx::raw_sql("CREATE TABLE brain.patch_events (id bigint, entity_name text, posted_at timestamptz);
            CREATE TABLE brain.patch_event_enrichments (patch_event_id bigint, secondary_entity_name text);
            INSERT INTO brain.entity_aliases VALUES (175035, 'Canonical Alias', 'canonical'), (175035, 'hero_warden', 'snapshot_name'), (175035, 'warden_short', 'class_name_short'), (175035, 'excluded', 'external_id'), (25, 'Wrong Alias', 'snapshot_name');
            INSERT INTO brain.patch_events VALUES (1, 'WARDEN', now()), (2, 'HERO_WARDEN', now()), (3, 'warden_short', now()), (4, 'other', now()), (5, 'excluded', now()), (6, 'Wrong Alias', now()), (7, 'Canonical Alias', now());
            INSERT INTO brain.patch_event_enrichments VALUES (4, 'HeRo_WaRdEn'), (4, 'HeRo_WaRdEn');")
            .execute(&ctx.pool).await.unwrap();
        let events = load_patch_events(&ctx, 25).await.unwrap();
        let ids: BTreeSet<_> = events
            .iter()
            .map(|event| event["id"].as_i64().unwrap())
            .collect();
        assert_eq!(ids, BTreeSet::from([1, 2, 3, 4, 7]));
        assert_eq!(events.len(), ids.len());
        ctx.pool.close().await;
    }

    #[tokio::test]
    #[ignore = "benötigt isolierten Postgres über REASONER_SCRATCH_DSN"]
    async fn author_builds_use_real_timestamps_and_expose_schema_errors() {
        let (_guard, ctx) = scratch_context().await;
        assert!(load_author_builds(&ctx, 25).await.is_err());
        sqlx::raw_sql("SET TIME ZONE 'Pacific/Honolulu'; CREATE TABLE tierlist.hero_build_sources (hero_id bigint, version bigint, details jsonb, published_at timestamptz, last_updated_at timestamptz);
            INSERT INTO tierlist.hero_build_sources VALUES
            (25, 1, '{}', '2026-01-01T00:00:00Z', '2026-02-01T00:00:00Z'),
            (25, 2, '{}', '2026-03-01T00:00:00Z', NULL),
            (25, 3, '{}', '2026-03-01T00:00:00Z', NULL),
            (25, 4, '{}', NULL, NULL);")
            .execute(&ctx.pool).await.unwrap();
        let builds = load_author_builds(&ctx, 25).await.unwrap();
        assert_eq!(
            builds.iter().map(|build| build.version).collect::<Vec<_>>(),
            [3, 2, 1, 4]
        );
        assert_eq!(builds[0].published_at, Some(1772323200));
        assert_eq!(builds[0].last_updated_at, None);
        assert_eq!(builds[2].last_updated_at, Some(1769904000));
        assert_eq!(builds[3].published_at, None);
        sqlx::raw_sql("ALTER TABLE tierlist.hero_build_sources DROP COLUMN last_updated_at")
            .execute(&ctx.pool)
            .await
            .unwrap();
        assert!(load_author_builds(&ctx, 25).await.is_err());
        ctx.pool.close().await;
    }

    #[tokio::test]
    #[ignore = "benötigt isolierten Postgres über REASONER_SCRATCH_DSN"]
    async fn hero_stats_resolve_external_id_to_profile_entity() {
        let (_guard, ctx) = scratch_context().await;
        sqlx::raw_sql("CREATE TABLE brain.hero_stat_profiles (id bigint, entity_id bigint);
            CREATE TABLE brain.hero_stat_values (profile_id bigint, entity_id bigint, stat_key text, numeric_value float8);
            INSERT INTO brain.hero_stat_profiles VALUES (10, 175035), (20, 25);
            INSERT INTO brain.hero_stat_values VALUES (10, NULL, 'spirit_scaling.test', 2.7), (10, NULL, 'spirit_scaling.test', 2.7), (10,NULL,'base_hp',815), (20, 25, 'wrong', 99);")
            .execute(&ctx.pool).await.unwrap();
        let stats = load_hero_stat_values(&ctx, 25).await.unwrap();
        assert_eq!(stats.len(), 1);
        assert_eq!(stats[0].stat, "spirit_scaling.test");
        assert_eq!(stats[0].per_spirit, Some(2.7));
        ctx.pool.close().await;
    }

    #[tokio::test]
    #[ignore = "benötigt isolierten Postgres über REASONER_SCRATCH_DSN"]
    async fn missing_ability_snapshot_is_an_error_instead_of_shifting_slots() {
        let (_guard, ctx) = scratch_context().await;
        let hero = serde_json::json!({"items":{"signature1":"missing","signature2":"present"}});
        assert!(ability_snapshots(&ctx.pool, &hero).await.is_err());
        ctx.pool.close().await;
    }

    #[tokio::test]
    #[ignore = "benötigt isolierten Postgres über REASONER_SCRATCH_DSN"]
    async fn snapshot_prefers_canonical_exact_match_before_newer_fuzzy_match() {
        let (_guard, ctx) = scratch_context().await;
        sqlx::raw_sql("INSERT INTO brain.entity_snapshots VALUES
            (1, 'deadlock_assets_api', 'hero', 'Warden', '{\"id\":25}', '2026-01-01'),
            (2, 'deadlock_assets_api', 'hero', 'Super Warden', '{\"id\":999}', '2026-02-01'),
            (3, 'deadlock_assets_api', 'hero', 'Other', '{\"id\":998,\"name\":\"Warden\"}', '2026-03-01');")
            .execute(&ctx.pool).await.unwrap();
        assert_eq!(
            snapshot(&ctx.pool, "hero", "wArDeN").await.unwrap()["id"],
            25
        );
        assert_eq!(
            snapshot(&ctx.pool, "hero", "Super").await.unwrap()["id"],
            999
        );
        ctx.pool.close().await;
    }

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
            .after_connect(|connection, _| {
                Box::pin(async move {
                    sqlx::query("SET default_transaction_read_only = on")
                        .execute(connection)
                        .await?;
                    Ok(())
                })
            })
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
        assert_eq!(hero.weapon.reload_duration, 2.914);
        assert!(
            hero.damage_plan.weapon_share > 0.6,
            "{:?}",
            hero.damage_plan
        );
        let items = load_item_models(&ctx).await.unwrap();
        for name in [
            "Veil Walker",
            "Mercurial Magnum",
            "Siphon Bullets",
            "Quicksilver Reload",
        ] {
            assert!(items.iter().any(|item| item.name == name), "{name}");
        }
        let stats = load_hero_stat_values(&ctx, hero.hero_id).await.unwrap();
        assert!(!stats.is_empty());
        let events = load_patch_events(&ctx, hero.hero_id).await.unwrap();
        assert!(!events.is_empty());
        let authors = load_author_builds(&ctx, hero.hero_id).await.unwrap();
        assert!(!authors.is_empty());
        assert!(authors
            .iter()
            .any(|build| build.published_at.is_some_and(|time| time > 0)));
        println!("Echtdaten: 1 Held, {} Items, 4 Referenz-Items, {} Stat-Zeilen, {} Patch-Zeilen, {} Autoren-Builds", items.len(), stats.len(), events.len(), authors.len());
    }
}
