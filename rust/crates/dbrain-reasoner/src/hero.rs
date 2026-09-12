use std::collections::BTreeSet;

use serde_json::Value;

use crate::data::scaling_stats;
use crate::mechanics;
use crate::{DamagePlan, HeroModel, ReasonerConfig, Result, ScalingStat, ScalingStep};

pub fn build_hero_model(
    loaded: &HeroModel,
    abilities: &[Value],
    stats: &[ScalingStat],
) -> Result<HeroModel> {
    let mut hero = loaded.clone();
    let mut merged_stats = hero.scaling.clone();
    merged_stats.extend(stats.iter().cloned());
    dedupe_stats(&mut merged_stats);
    hero.scaling = merged_stats;
    for ability in abilities {
        let ability_id = integer(ability.get("id"));
        let target_index = hero
            .abilities
            .iter()
            .position(|candidate| candidate.ability_id == ability_id)
            .ok_or_else(|| {
                crate::ReasonerError::Data(format!(
                    "Ability {ability_id} gehört nicht zum geladenen Helden"
                ))
            })?;
        {
            let target = &mut hero.abilities[target_index];
            if target.scaling_step.is_none() {
                target.scaling_step = scaling_step(ability);
            }
            if target.scaling.is_empty() {
                target.scaling = scaling_stats(ability.get("scaling_stats"));
            }
        }
    }
    if hero.weapon.bullet_damage <= 0.0
        || hero.weapon.shots_per_second <= 0.0
        || hero.weapon.clip_size <= 0.0
    {
        return Err(crate::ReasonerError::Data(
            "Waffenprofil unvollständig; Primärwaffen-Snapshot laden".into(),
        ));
    }
    hero.damage_plan = damage_plan(&hero, &ReasonerConfig::default());
    if hero.weapon.sustained_dps <= 0.0 {
        hero.weapon.sustained_dps = mechanics::weapon_dps(&hero.weapon, 40.0);
        hero.damage_plan.weapon_dps = hero.weapon.sustained_dps;
        hero.damage_plan = damage_plan(&hero, &ReasonerConfig::default());
    }
    Ok(hero)
}

pub async fn load_built_hero_model(ctx: &crate::ReasonerCtx, name: &str) -> Result<HeroModel> {
    let loaded = crate::load_hero_model(ctx, name).await?;
    let abilities = crate::load_hero_abilities(ctx, name).await?;
    let stats = crate::load_hero_stat_values(ctx, loaded.hero_id).await?;
    let mut hero = build_hero_model(&loaded, &abilities, &stats)?;
    hero.damage_plan = damage_plan(&hero, &ctx.config);
    Ok(hero)
}

pub fn damage_plan(hero: &HeroModel, cfg: &ReasonerConfig) -> DamagePlan {
    mechanics::damage_plan(hero, cfg)
}

pub fn scaling_step(ability: &Value) -> Option<ScalingStep> {
    let scaling_names = parse_scaling_names(ability);
    let upgrades = ability.get("upgrades")?.as_array()?;
    for (upgrade_index, upgrade) in upgrades.iter().enumerate() {
        let property_upgrades = upgrade
            .get("property_upgrades")
            .and_then(Value::as_array)
            .or_else(|| upgrade.as_array());
        let Some(property_upgrades) = property_upgrades else {
            continue;
        };
        for property in property_upgrades {
            let name = string(property.get("name"));
            if !is_scaling_property(&name, &scaling_names, property) {
                continue;
            }
            if string(property.get("upgrade_type")).eq_ignore_ascii_case("EAddToScale") {
                let base = ability
                    .get("properties")?
                    .get(&name)?
                    .get("scale_function")?
                    .get("stat_scale")?;
                let from = number(Some(base));
                return Some(ScalingStep {
                    upgrade_index: upgrade_index as i64,
                    stat: name,
                    from,
                    to: from + number(property.get("bonus")),
                });
            }
            if let Some((from, to)) = scale_function_values(property) {
                return Some(ScalingStep {
                    upgrade_index: upgrade_index as i64,
                    stat: name,
                    from,
                    to,
                });
            }
            let from = number(property.get("from"));
            let to = number(
                property
                    .get("to")
                    .or_else(|| property.get("value"))
                    .or_else(|| property.get("bonus")),
            );
            if from != 0.0 || to != 0.0 {
                return Some(ScalingStep {
                    upgrade_index: upgrade_index as i64,
                    stat: name,
                    from,
                    to: if to == 0.0 { from } else { to },
                });
            }
        }
    }
    None
}

fn parse_scaling_names(ability: &Value) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for stat in scaling_stats(ability.get("scaling_stats")) {
        names.insert(normalize(&stat.stat));
    }
    for (name, _) in property_entries(ability.get("properties")) {
        let lower = name.to_ascii_lowercase();
        if lower.contains("scale") {
            names.insert(normalize(&name));
        }
    }
    names
}

fn is_scaling_property(name: &str, scaling_names: &BTreeSet<String>, property: &Value) -> bool {
    let normalized = normalize(name);
    (!normalized.is_empty() && scaling_names.contains(&normalized))
        || name.to_ascii_lowercase().contains("scale")
        || string(property.get("upgrade_type")).eq_ignore_ascii_case("EAddToScale")
        || property.get("scale_function").is_some()
}

fn scale_function_values(property: &Value) -> Option<(f64, f64)> {
    let function = property.get("scale_function")?;
    if function.is_null() {
        return None;
    }
    let from = number(function.get("from").or_else(|| function.get("start")));
    let to = number(function.get("to").or_else(|| function.get("end")));
    (from != 0.0 || to != 0.0).then_some((from, to))
}

fn property_entries(value: Option<&Value>) -> impl Iterator<Item = (String, &Value)> {
    let entries = match value {
        Some(Value::Object(object)) => object
            .iter()
            .map(|(name, value)| (name.clone(), value))
            .collect::<Vec<_>>(),
        Some(Value::Array(array)) => array
            .iter()
            .filter_map(|value| {
                let name = string(
                    value
                        .get("name")
                        .or_else(|| value.get("provided_property_type")),
                );
                (!name.is_empty()).then_some((name, value))
            })
            .collect::<Vec<_>>(),
        _ => Vec::new(),
    };
    entries.into_iter()
}

fn dedupe_stats(stats: &mut Vec<ScalingStat>) {
    let mut seen = BTreeSet::new();
    stats.retain(|stat| {
        seen.insert((
            stat.stat.clone(),
            stat.per_level.to_bits(),
            stat.per_spirit.map(f64::to_bits),
        ))
    });
}

fn normalize(value: &str) -> String {
    value
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .flat_map(char::to_lowercase)
        .collect()
}

fn integer(value: Option<&Value>) -> i64 {
    number(value) as i64
}

fn number(value: Option<&Value>) -> f64 {
    value
        .and_then(|value| {
            value
                .as_f64()
                .or_else(|| value.as_i64().map(|value| value as f64))
                .or_else(|| value.as_str()?.trim_end_matches('%').parse().ok())
        })
        .unwrap_or_default()
}

fn string(value: Option<&Value>) -> String {
    value
        .and_then(Value::as_str)
        .map(str::to_string)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AbilityModel, AbilityRole, DamageType, PurchaseBonuses, WeaponProfile};
    use serde_json::json;

    fn loaded_hero() -> HeroModel {
        HeroModel {
            hero_id: 25,
            name: "Warden".to_string(),
            archetype: "brawler".to_string(),
            base_health: 600.0,
            level_curve: Vec::new(),
            purchase_bonuses: PurchaseBonuses {
                spirit: Vec::new(),
                weapon: Vec::new(),
                vitality: Vec::new(),
            },
            scaling: Vec::new(),
            weapon: WeaponProfile {
                bullet_damage: 10.0,
                shots_per_second: 5.0,
                clip_size: 20.0,
                reload_duration: 2.0,
                range: 0.0,
                falloff_start_range: 0.0,
                falloff_end_range: 0.0,
                sustained_dps: 0.0,
            },
            abilities: vec![AbilityModel {
                ability_id: 100,
                class_name: "ability1".to_string(),
                slot: 1,
                roles: vec![AbilityRole::Damage],
                scaling: Vec::new(),
                channel_time: None,
                charges: 0,
                cooldown: 10.0,
                scaling_step: None,
                damage_type: DamageType::Spirit,
                base_effect: 0.0,
                tick_rate: None,
                duration: None,
            }],
            damage_plan: DamagePlan {
                weapon_dps: 0.0,
                spirit_dps: 0.0,
                weapon_share: 0.5,
                primary_axis: DamageType::Hybrid,
            },
        }
    }

    #[test]
    fn enriches_loaded_model_without_rebuilding_payload_fields() {
        let ability = json!({
            "id": 100,
            "scaling_stats": {"SpiritPower": {"stat": "SpiritPower", "per_spirit": 0.4}},
            "upgrades": [{"property_upgrades": [{"name": "SpiritPower", "from": 0.3, "to": 0.6}]}]
        });
        let hero = build_hero_model(&loaded_hero(), &[ability], &[]).unwrap();
        assert_eq!(hero.hero_id, 25);
        assert_eq!(hero.abilities[0].scaling_step.as_ref().unwrap().to, 0.6);
        assert!(hero.weapon.sustained_dps > 0.0);
    }

    #[test]
    fn scale_function_is_only_used_when_present() {
        let ability = json!({
            "id": 100,
            "scaling_stats": {"SpiritPower": {"stat": "SpiritPower"}},
            "upgrades": [{"property_upgrades": [{"name": "SpiritPower", "bonus": 0.6, "scale_function": {"from": 0.3, "to": 0.6}}]}]
        });
        assert_eq!(scaling_step(&ability).unwrap().from, 0.3);
    }

    #[test]
    fn additive_scaling_uses_property_base_and_bonus() {
        let ability = json!({
            "properties": {"LifeDrain": {"scale_function": {"stat_scale": 0.3225}}},
            "upgrades": [{"property_upgrades": [{"name": "LifeDrain", "bonus": "0.3", "upgrade_type": "EAddToScale"}]}]
        });
        let step = scaling_step(&ability).unwrap();
        assert_eq!(step.from, 0.3225);
        assert!((step.to - 0.6225).abs() < 1e-12);
    }

    #[test]
    fn ordinary_weapon_debuff_is_not_a_scaling_step() {
        let ability = json!({"properties":{"WeaponPowerDebuff":{"value":"-25"}},"upgrades":[{"property_upgrades":[{"name":"WeaponPowerDebuff","bonus":"-25"}]}]});
        assert!(scaling_step(&ability).is_none());
    }

    #[test]
    fn enrichment_matches_identity_in_unsorted_abilities() {
        let mut loaded = loaded_hero();
        let mut second = loaded.abilities[0].clone();
        second.ability_id = 200;
        second.slot = 2;
        loaded.abilities.insert(0, second);
        let payload = json!({"id":100,"upgrades":[{"property_upgrades":[{"name":"SpiritScale","from":0.3225,"to":0.6225}]}]});
        let enriched = build_hero_model(&loaded, &[payload], &[]).unwrap();
        assert!(enriched.abilities[0].scaling_step.is_none());
        assert_eq!(
            enriched.abilities[1].scaling_step.as_ref().unwrap().to,
            0.6225
        );
    }

    #[test]
    fn enrichment_rejects_unknown_identity() {
        let payload = json!({"id":999,"upgrades":[{"property_upgrades":[{"name":"SpiritScale","to":0.6225}]}]});
        assert!(build_hero_model(&loaded_hero(), &[payload], &[]).is_err());
    }

    #[test]
    fn missing_weapon_cannot_be_replaced_by_untyped_stat_values() {
        let mut loaded = loaded_hero();
        loaded.weapon.bullet_damage = 0.0;
        loaded.weapon.shots_per_second = 0.0;
        loaded.weapon.clip_size = 0.0;
        let stats = [
            ("base_bullet_dmg", 10.0),
            ("base_fire_rate", 5.0),
            ("base_ammo", 20.0),
            ("reload_time", 2.0),
        ]
        .map(|(stat, value)| ScalingStat {
            stat: stat.into(),
            per_level: 0.0,
            per_spirit: Some(value),
        });
        assert!(build_hero_model(&loaded, &[], &stats).is_err());
    }
}
