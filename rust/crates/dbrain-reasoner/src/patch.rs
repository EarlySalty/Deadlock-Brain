use std::collections::BTreeMap;

use serde_json::Value;

use crate::{AbilityModel, DamageType, DeltaTarget, HeroModel, ItemModel, PatchDelta};

fn text(value: Option<&Value>) -> String {
    value
        .and_then(Value::as_str)
        .map(str::to_string)
        .unwrap_or_else(|| value.map(Value::to_string).unwrap_or_default())
}

fn lower(value: Option<&Value>) -> String {
    text(value).to_ascii_lowercase()
}

fn number(value: Option<&Value>) -> Option<f64> {
    value.and_then(|value| {
        value
            .as_f64()
            .or_else(|| value.as_i64().map(|value| value as f64))
            .or_else(|| value.as_str()?.trim_end_matches('%').parse().ok())
    })
}

fn numeric_tokens(value: &str) -> Vec<f64> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    for character in value.chars() {
        if character.is_ascii_digit() || character == '.' || character == '-' || character == '+' {
            current.push(character);
        } else if !current.is_empty() {
            if let Ok(value) = current.parse::<f64>() {
                tokens.push(value);
            }
            current.clear();
        }
    }
    if !current.is_empty() {
        if let Ok(value) = current.parse::<f64>() {
            tokens.push(value);
        }
    }
    tokens
}

fn event_text(event: &Value) -> String {
    [
        "subject",
        "raw_line",
        "normalized_line",
        "old_value",
        "new_value",
    ]
    .iter()
    .map(|key| text(event.get(*key)))
    .chain(event.get("enrichment").into_iter().flat_map(|enrichment| {
        [
            "stat_name",
            "ability_name",
            "secondary_entity_name",
            "old_value",
            "new_value",
        ]
        .iter()
        .map(move |key| text(enrichment.get(*key)))
    }))
    .collect::<Vec<_>>()
    .join(" ")
}

fn id(event: &Value, keys: &[&str]) -> Option<i64> {
    keys.iter().find_map(|key| {
        event
            .get(*key)
            .and_then(|value| value.as_i64().or_else(|| value.as_str()?.parse().ok()))
            .filter(|value| *value != 0)
    })
}

fn target(hero: &HeroModel, event: &Value) -> Option<DeltaTarget> {
    let entity_type = lower(event.get("entity_type"));
    let entity_name = lower(event.get("entity_name"));
    let hero_name = hero.name.to_ascii_lowercase();
    if entity_type == "hero" || entity_name == hero_name {
        return (entity_name.is_empty() || entity_name == hero_name)
            .then_some(DeltaTarget::Hero(hero.hero_id));
    }
    if entity_type == "ability" || entity_type == "item_or_ability" {
        if let Some(ability_id) = id(event, &["ability_id", "target_id", "entity_id"]) {
            return Some(DeltaTarget::Ability(ability_id));
        }
    }
    let enrichment = event.get("enrichment");
    id(event, &["item_id", "target_item_id", "secondary_entity_id"])
        .or_else(|| {
            id(
                enrichment.unwrap_or(&Value::Null),
                &["item_id", "target_item_id", "secondary_entity_id"],
            )
        })
        .map(DeltaTarget::Item)
}

fn mechanic(event: &Value) -> String {
    let value = event_text(event).to_ascii_lowercase();
    let candidates: &[(&str, &[&str])] = &[
        (
            "spirit_scaling",
            &[
                "spirit power scaling",
                "spirit scaling",
                "tech power scaling",
            ],
        ),
        (
            "weapon_scaling",
            &["weapon scaling", "weapon damage scaling"],
        ),
        (
            "bullet_damage",
            &["bullet damage", "bullet damage per", "weapon damage"],
        ),
        (
            "fire_rate",
            &["fire rate", "rounds per second", "attack speed"],
        ),
        ("cooldown", &["cooldown", "cooldown time"]),
        ("reload", &["reload", "reload duration"]),
        ("range", &["range", "falloff"]),
        ("health", &["health", "max hp"]),
        ("damage", &["damage", "dps"]),
    ];
    candidates
        .iter()
        .find_map(|(name, needles)| {
            needles
                .iter()
                .any(|needle| value.contains(needle))
                .then_some((*name).to_string())
        })
        .unwrap_or_else(|| "other".to_string())
}

fn sign(event: &Value, old: Option<f64>, new: Option<f64>) -> i8 {
    let change = lower(event.get("change_type"));
    if [
        "buff",
        "increase",
        "increased",
        "add",
        "added",
        "improved",
        "up",
    ]
    .iter()
    .any(|value| change.contains(value))
    {
        return 1;
    }
    if [
        "nerf",
        "decrease",
        "decreased",
        "reduced",
        "removed",
        "lowered",
        "down",
    ]
    .iter()
    .any(|value| change.contains(value))
    {
        return -1;
    }
    match (old, new) {
        (Some(old), Some(new)) if new > old => 1,
        (Some(old), Some(new)) if new < old => -1,
        _ => 0,
    }
}

fn magnitude(event: &Value) -> Option<f64> {
    let old = number(event.get("old_value"));
    let new = number(event.get("new_value"));
    if let (Some(old), Some(new)) = (old, new) {
        return Some((new - old).abs());
    }
    let line = text(event.get("raw_line")).if_empty_then(|| text(event.get("normalized_line")));
    let values = numeric_tokens(&line);
    match values.as_slice() {
        [first, second, ..] => Some((second - first).abs()),
        [first] => Some(first.abs()),
        _ => None,
    }
}

trait StringFallback {
    fn if_empty_then(self, fallback: impl FnOnce() -> String) -> String;
}

impl StringFallback for String {
    fn if_empty_then(self, fallback: impl FnOnce() -> String) -> String {
        if self.is_empty() {
            fallback()
        } else {
            self
        }
    }
}

pub fn compute_patch_delta(hero: &HeroModel, events: &[Value]) -> Vec<PatchDelta> {
    events
        .iter()
        .filter_map(|event| {
            let target = target(hero, event)?;
            let old = number(event.get("old_value"));
            let new = number(event.get("new_value"));
            let sign = sign(event, old, new);
            let magnitude = magnitude(event)?;
            (sign != 0 && magnitude.is_finite()).then_some(PatchDelta {
                target,
                mechanic: mechanic(event),
                sign,
                magnitude,
                note: text(event.get("normalized_line"))
                    .if_empty_then(|| text(event.get("raw_line"))),
            })
        })
        .collect()
}

fn matches_mechanic(key: &str, mechanic: &str) -> bool {
    let key = key.to_ascii_lowercase();
    match mechanic {
        "bullet_damage" => key.contains("damage") || key.contains("bullet"),
        "weapon_scaling" => key.contains("weapon") || key.contains("damage"),
        "spirit_scaling" => key.contains("spirit") || key.contains("tech"),
        "fire_rate" => key.contains("fire") || key.contains("round") || key.contains("attack"),
        "cooldown" => key.contains("cooldown"),
        "reload" => key.contains("reload"),
        "range" => key.contains("range") || key.contains("falloff"),
        "health" => key.contains("health") || key.contains("hp"),
        "damage" => key.contains("damage"),
        _ => false,
    }
}

fn adjust_map(values: &mut BTreeMap<String, f64>, mechanic: &str, delta: f64) {
    for (key, value) in values.iter_mut() {
        if matches_mechanic(key, mechanic) {
            *value += delta;
        }
    }
}

fn adjust_ability(ability: &mut AbilityModel, delta: &PatchDelta) {
    match delta.mechanic.as_str() {
        "cooldown" => ability.cooldown += delta.sign as f64 * delta.magnitude,
        "damage" | "spirit_scaling" | "weapon_scaling" => {
            for scaling in &mut ability.scaling {
                scaling.per_level += delta.sign as f64 * delta.magnitude;
            }
        }
        _ => {}
    }
    if let Some(step) = &mut ability.scaling_step {
        if delta.mechanic.contains("scaling") || delta.mechanic == "damage" {
            step.to += delta.sign as f64 * delta.magnitude;
        }
    }
}

pub fn apply_patch_delta(hero: &mut HeroModel, items: &mut [ItemModel], deltas: &[PatchDelta]) {
    for delta in deltas {
        let amount = delta.sign as f64 * delta.magnitude;
        match delta.target {
            DeltaTarget::Hero(hero_id) if hero_id == hero.hero_id => {
                match delta.mechanic.as_str() {
                    "bullet_damage" | "weapon_scaling" | "damage" => {
                        hero.weapon.bullet_damage += amount
                    }
                    "fire_rate" => hero.weapon.shots_per_second += amount,
                    "reload" => hero.weapon.reload_duration += amount,
                    "range" => hero.weapon.range += amount,
                    "spirit_scaling" => {
                        for scaling in &mut hero.scaling {
                            if scaling.stat.to_ascii_lowercase().contains("spirit")
                                || scaling.stat.to_ascii_lowercase().contains("tech")
                            {
                                scaling.per_spirit =
                                    Some(scaling.per_spirit.unwrap_or_default() + amount);
                            }
                        }
                    }
                    _ => {}
                }
            }
            DeltaTarget::Ability(ability_id) => {
                if let Some(ability) = hero
                    .abilities
                    .iter_mut()
                    .find(|ability| ability.ability_id == ability_id)
                {
                    adjust_ability(ability, delta);
                }
            }
            DeltaTarget::Item(item_id) => {
                if let Some(item) = items.iter_mut().find(|item| item.item_id == item_id) {
                    adjust_map(&mut item.properties, &delta.mechanic, amount);
                    adjust_map(&mut item.passive_properties, &delta.mechanic, amount);
                }
            }
            _ => {}
        }
    }
    hero.weapon.sustained_dps = if hero.weapon.sustained_dps > 0.0 {
        hero.weapon.sustained_dps
    } else {
        hero.weapon.bullet_damage * hero.weapon.shots_per_second
    };
    hero.damage_plan.weapon_dps = hero.weapon.sustained_dps;
    let total = hero.damage_plan.weapon_dps + hero.damage_plan.spirit_dps;
    hero.damage_plan.weapon_share = if total > 0.0 {
        hero.damage_plan.weapon_dps / total
    } else {
        0.0
    };
    hero.damage_plan.primary_axis = if hero.damage_plan.weapon_share > 0.6 {
        DamageType::Weapon
    } else if hero.damage_plan.weapon_share < 0.4 {
        DamageType::Spirit
    } else {
        DamageType::Hybrid
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DamagePlan, PurchaseBonuses, WeaponProfile};

    fn hero() -> HeroModel {
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
                shots_per_second: 2.0,
                clip_size: 20.0,
                reload_duration: 2.0,
                range: 30.0,
                falloff_start_range: 20.0,
                falloff_end_range: 40.0,
                sustained_dps: 20.0,
            },
            abilities: Vec::new(),
            damage_plan: DamagePlan {
                weapon_dps: 20.0,
                spirit_dps: 5.0,
                weapon_share: 0.8,
                primary_axis: DamageType::Weapon,
            },
        }
    }

    #[test]
    fn parses_real_warden_style_patch_lines() {
        let events = vec![
            serde_json::json!({
                "entity_type": "hero", "entity_name": "Warden", "change_type": "buff",
                "subject": "Willpower", "raw_line": "Willpower T3 increased from +2.5 spirit power scaling to +2.7",
                "old_value": "+2.5", "new_value": "+2.7", "normalized_line": "Willpower T3 +2.5 to +2.7"
            }),
            serde_json::json!({
                "entity_type": "hero", "entity_name": "Warden", "change_type": "nerf",
                "subject": "Bullet damage per boon", "raw_line": "Bullet damage per boon reduced from 0.34 to 0.28",
                "old_value": "0.34", "new_value": "0.28"
            }),
        ];
        let deltas = compute_patch_delta(&hero(), &events);
        assert_eq!(deltas.len(), 2);
        assert_eq!(deltas[0].mechanic, "spirit_scaling");
        assert_eq!(deltas[0].sign, 1);
        assert!((deltas[0].magnitude - 0.2).abs() < 0.0001);
        assert_eq!(deltas[1].mechanic, "bullet_damage");
        assert_eq!(deltas[1].sign, -1);
        assert!((deltas[1].magnitude - 0.06).abs() < 0.0001);
    }

    #[test]
    fn applies_weapon_delta_without_estimating_a_value() {
        let mut hero = hero();
        let delta = PatchDelta {
            target: DeltaTarget::Hero(25),
            mechanic: "bullet_damage".to_string(),
            sign: 1,
            magnitude: 0.25,
            note: String::new(),
        };
        apply_patch_delta(&mut hero, &mut [], &[delta]);
        assert_eq!(hero.weapon.bullet_damage, 10.25);
    }
}
