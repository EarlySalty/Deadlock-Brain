use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{AbilityModel, AbilityStep, HeroModel, ReasonerConfig, ScalingStat};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProgressionEvidence {
    pub earned_souls: i64,
    pub reached_level: i64,
    pub standard_boons: usize,
    pub ability_ranks: BTreeMap<i64, usize>,
    pub applied_order_steps: usize,
    pub unspent_ability_points: i64,
    pub unspent_unlocks: i64,
    pub assumptions: Vec<String>,
    pub unknown_effects: Vec<String>,
}

pub fn coherent_order(hero: &HeroModel, order: &[AbilityStep]) -> (Vec<AbilityStep>, Vec<String>) {
    if hero.abilities.is_empty() {
        return (
            order.to_vec(),
            vec!["Skill-IDs ohne vollständiges Fähigkeitsmodell nicht prüfbar.".into()],
        );
    }
    let mut output = Vec::new();
    let mut unlocked = BTreeSet::new();
    let mut ranks = BTreeMap::<i64, usize>::new();
    for step in order {
        let ability = hero
            .abilities
            .iter()
            .find(|ability| ability.ability_id == step.ability_id);
        let valid = ability.is_some()
            && match step.currency_type {
                2 => step.delta == -1 && unlocked.insert(step.ability_id),
                1 => {
                    let rank = ranks.entry(step.ability_id).or_default();
                    *rank += 1;
                    step.delta < 0
                        && unlocked.contains(&step.ability_id)
                        && ability.is_some_and(|ability| {
                            ability.upgrades.is_empty() || *rank <= ability.upgrades.len()
                        })
                }
                _ => false,
            };
        if !valid {
            return (output.clone(),vec![format!("Quellenmangel in der Skillfolge: ab Buchung {} unbekannte Fähigkeit, wiederholte Freischaltung oder ungültiges Upgrade. Nur die {} vorherigen kohärenten Schritte werden bewertet und veröffentlicht; {} weitere Rohbuchungen bleiben erhalten, werden aber nicht angewandt.",output.len()+1,output.len(),order.len()-output.len())]);
        }
        output.push(step.clone());
    }
    (output, Vec::new())
}

pub fn at_souls(
    base: &HeroModel,
    order: &[AbilityStep],
    earned_souls: i64,
    cfg: &ReasonerConfig,
) -> (HeroModel, ProgressionEvidence) {
    let mut hero = base.clone();
    let mut evidence = ProgressionEvidence {
        earned_souls,
        ..Default::default()
    };
    if base.level_curve.is_empty() || base.level_rewards.is_empty() {
        evidence.assumptions.push("Keine vollständigen Levelbelohnungen im Snapshot: Basisfähigkeiten als ausdrücklich ungestuftes Vergleichsszenario.".into());
        return (hero, evidence);
    }
    for level in base
        .level_curve
        .iter()
        .filter(|level| level.required_souls <= earned_souls)
    {
        evidence.reached_level = evidence.reached_level.max(level.level);
        for currency in base.level_rewards.get(&level.level).into_iter().flatten() {
            match currency.as_str() {
                "EAbilityPoints" => evidence.unspent_ability_points += 1,
                "EAbilityUnlocks" => evidence.unspent_unlocks += 1,
                other => evidence.unknown_effects.push(format!(
                    "Level {}: Belohnung {other} nicht verrechnet.",
                    level.level
                )),
            }
        }
        evidence.standard_boons += usize::from(base.standard_upgrade_levels.contains(&level.level));
    }
    let boons = evidence.standard_boons as f64;
    for (name, value) in &base.standard_level_up_upgrades {
        if *value == 0.0 {
            continue;
        }
        match name.as_str() {
            "MODIFIER_VALUE_BASE_BULLET_DAMAGE_FROM_LEVEL" => {
                hero.weapon.bullet_damage += boons * value
            }
            "MODIFIER_VALUE_BASE_HEALTH_FROM_LEVEL" => hero.base_health += boons * value,
            "MODIFIER_VALUE_TECH_POWER" => hero.base_spirit_power += boons * value,
            "MODIFIER_VALUE_BOON_COUNT" => {}
            _ => evidence.unknown_effects.push(format!(
                "Levelbonus {name} ({value} je Boon) nicht quantifiziert."
            )),
        }
    }
    evidence.assumptions.push("Fortschritt verwendet die tatsächlich gespeicherten Seelenschwellen und Levelbelohnungen. Verdiente Seelen sind vom ausgegebenen Geld getrennt; Verkäufe senken weder Level noch Skillrang. Keine Aussage über Spielminuten.".into());
    if order.is_empty() {
        evidence.assumptions.push("Keine belegte Skillfolge: alle Basisfähigkeiten bleiben ein Vergleichsszenario; es werden keine Fähigkeitenränge erfunden.".into());
        hero.damage_plan = crate::mechanics::damage_plan(&hero, cfg);
        return (hero, evidence);
    }
    let mut unlocked = BTreeSet::new();
    for step in order {
        let Some(cost) = step.delta.checked_neg().filter(|cost| *cost > 0) else {
            evidence.unknown_effects.push("Skillfolge enthält eine nicht unterstützte Gutschrift oder Nullbuchung; Anwendung endet vor diesem Schritt.".into());
            break;
        };
        let Some(ability) = hero
            .abilities
            .iter_mut()
            .find(|ability| ability.ability_id == step.ability_id)
        else {
            evidence.unknown_effects.push(format!("Skillfolge verweist auf fehlende Fähigkeit {}; spätere Schritte werden nicht vorgezogen.", step.ability_id));
            break;
        };
        match step.currency_type {
            2 if evidence.unspent_unlocks >= cost => {
                if cost != 1 || !unlocked.insert(step.ability_id) {
                    evidence.unknown_effects.push(format!(
                        "Ungültige Freischaltung für Fähigkeit {}.",
                        step.ability_id
                    ));
                    break;
                }
                evidence.unspent_unlocks -= cost;
                evidence.ability_ranks.insert(step.ability_id, 0);
            }
            1 if evidence.unspent_ability_points >= cost => {
                if !unlocked.contains(&step.ability_id) {
                    evidence.unknown_effects.push(format!(
                        "Upgrade vor Freischaltung für Fähigkeit {} nicht angewendet.",
                        step.ability_id
                    ));
                    break;
                }
                let rank = evidence.ability_ranks.entry(step.ability_id).or_default();
                let Some(upgrade) = ability.upgrades.get(*rank).cloned() else {
                    evidence.unknown_effects.push(format!("Fähigkeit {}: Snapshot für Upgrade {} fehlt; spätere Schritte werden nicht vorgezogen.", step.ability_id, *rank + 1));
                    break;
                };
                apply_upgrade(ability, &upgrade, &mut evidence.unknown_effects);
                *rank += 1;
                evidence.unspent_ability_points -= cost;
            }
            1 | 2 => break,
            other => {
                evidence.unknown_effects.push(format!(
                    "Unbekannte Skillwährung {other}; spätere Schritte werden nicht vorgezogen."
                ));
                break;
            }
        }
        evidence.applied_order_steps += 1;
    }
    hero.abilities
        .retain(|ability| unlocked.contains(&ability.ability_id));
    hero.weapon.sustained_dps =
        crate::mechanics::weapon_dps(&hero.weapon, cfg.combat_window_seconds);
    hero.damage_plan = crate::mechanics::damage_plan(&hero, cfg);
    (hero, evidence)
}

fn number(value: &Value) -> Option<f64> {
    value
        .as_f64()
        .or_else(|| {
            let text = value.as_str()?.trim();
            let end = text
                .char_indices()
                .find(|(_, chr)| !chr.is_ascii_digit() && !matches!(chr, '.' | '-' | '+'))
                .map(|(index, _)| index)
                .unwrap_or(text.len());
            text[..end].parse().ok()
        })
        .filter(|value| value.is_finite())
}

fn apply_upgrade(ability: &mut AbilityModel, upgrade: &Value, unknown: &mut Vec<String>) {
    let Some(properties) = upgrade.get("property_upgrades").and_then(Value::as_array) else {
        unknown.push(format!(
            "{}: Upgrade enthält keine auswertbaren Properties.",
            ability.class_name
        ));
        return;
    };
    for property in properties {
        let Some(name) = property.get("name").and_then(Value::as_str) else {
            continue;
        };
        let Some(bonus) = property.get("bonus").and_then(number) else {
            unknown.push(format!(
                "{}: Upgrade {name} besitzt keinen numerischen Bonus.",
                ability.class_name
            ));
            continue;
        };
        let kind = property
            .get("upgrade_type")
            .and_then(Value::as_str)
            .unwrap_or("EAddToBase");
        match kind {
            "EAddToBase" => *ability.properties.entry(name.into()).or_default() += bonus,
            "EMultiplyBase" => {
                if let Some(value) = ability.properties.get_mut(name) {
                    *value *= bonus;
                } else {
                    unknown.push(format!(
                        "{}: Basis für Multiplikation von {name} fehlt.",
                        ability.class_name
                    ));
                }
            }
            "EAddToScale" | "EMultiplyScale" => {
                if property
                    .get("scale_stat_filter")
                    .and_then(Value::as_str)
                    .is_some_and(|filter| filter != "ETechPower")
                {
                    unknown.push(format!(
                        "{}: Nicht-Spirit-Skalierungsupgrade {name} nicht quantifiziert.",
                        ability.class_name
                    ));
                    continue;
                }
                if let Some(scale) = ability
                    .scaling
                    .iter_mut()
                    .find(|scale| scale.stat == name && scale.per_spirit.is_some())
                {
                    let value = scale.per_spirit.unwrap_or_default();
                    scale.per_spirit = Some(if kind == "EAddToScale" {
                        value + bonus
                    } else {
                        value * bonus
                    });
                } else if kind == "EAddToScale" {
                    ability.scaling.push(ScalingStat {
                        stat: name.into(),
                        per_level: 0.0,
                        per_spirit: Some(bonus),
                    });
                } else {
                    unknown.push(format!(
                        "{}: Ausgangsskalierung für {name} fehlt.",
                        ability.class_name
                    ));
                }
            }
            _ => unknown.push(format!(
                "{}: Upgradeart {kind} für {name} nicht quantifiziert.",
                ability.class_name
            )),
        }
    }
    crate::data::refresh_ability_derived(ability);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hero() -> HeroModel {
        serde_json::from_value(serde_json::json!({
            "hero_id":25,"name":"Fortschrittsprüfung","archetype":"hybrid","base_health":600.0,
            "level_curve":[{"level":1,"required_souls":0},{"level":2,"required_souls":200},{"level":3,"required_souls":500},{"level":4,"required_souls":900},{"level":5,"required_souls":1400},{"level":6,"required_souls":2000}],
            "level_rewards":{"1":["EAbilityUnlocks"],"2":["EAbilityPoints"],"3":["EAbilityUnlocks"],"4":["EAbilityPoints"],"5":["EAbilityUnlocks"],"6":["EAbilityPoints"]},
            "standard_upgrade_levels":[2,3,4,5,6],"standard_level_up_upgrades":{"MODIFIER_VALUE_BASE_HEALTH_FROM_LEVEL":60.0,"MODIFIER_VALUE_BASE_BULLET_DAMAGE_FROM_LEVEL":0.28,"MODIFIER_VALUE_TECH_POWER":1.1},
            "purchase_bonuses":{"weapon":[],"spirit":[],"vitality":[]},"scaling":[],
            "weapon":{"bullet_damage":20.0,"shots_per_second":4.0,"clip_size":16.0,"reload_duration":2.0,"range":20.0,"falloff_start_range":20.0,"falloff_end_range":50.0,"sustained_dps":50.0},
            "damage_plan":{"weapon_dps":50.0,"spirit_dps":0.0,"weapon_share":1.0,"primary_axis":"Weapon"},
            "abilities":[{"ability_id":10,"class_name":"flask","slot":1,"roles":["Damage"],"scaling":[],"channel_time":null,"charges":1,"cooldown":12.0,"scaling_step":null,"damage_type":"Spirit","base_effect":60.0,"properties":{"Damage":60.0,"AbilityCooldown":12.0},"upgrades":[{"property_upgrades":[{"name":"Damage","bonus":"35"}]},{"property_upgrades":[{"name":"AbilityCooldown","bonus":"-7"}]}]},
            {"ability_id":20,"class_name":"second","slot":2,"roles":[],"scaling":[],"channel_time":null,"charges":1,"cooldown":10.0,"scaling_step":null,"damage_type":"Spirit","base_effect":20.0,"properties":{"Damage":20.0,"AbilityCooldown":10.0},"upgrades":[]}]
        })).unwrap()
    }

    #[test]
    fn actual_reward_thresholds_gate_unlocks_and_upgrades_without_spending_gold() {
        let hero = hero();
        let order = vec![
            AbilityStep {
                ability_id: 10,
                currency_type: 2,
                delta: -1,
            },
            AbilityStep {
                ability_id: 10,
                currency_type: 1,
                delta: -1,
            },
            AbilityStep {
                ability_id: 20,
                currency_type: 2,
                delta: -1,
            },
            AbilityStep {
                ability_id: 10,
                currency_type: 1,
                delta: -2,
            },
        ];
        let cfg = ReasonerConfig::default();
        let (early, early_evidence) = at_souls(&hero, &order, 199, &cfg);
        assert_eq!(early.abilities.len(), 1);
        assert_eq!(early.abilities[0].base_effect, 60.0);
        assert_eq!(early_evidence.applied_order_steps, 1);
        let (upgraded, evidence) = at_souls(&hero, &order, 200, &cfg);
        assert_eq!(upgraded.abilities[0].base_effect, 95.0);
        assert_eq!(upgraded.base_health, 660.0);
        assert_eq!(upgraded.base_spirit_power, 1.1);
        assert_eq!(evidence.ability_ranks.get(&10), Some(&1));
        let (not_ready, evidence) = at_souls(&hero, &order, 1999, &cfg);
        assert_eq!(not_ready.abilities.len(), 2);
        assert_eq!(not_ready.abilities[0].cooldown, 12.0);
        assert_eq!(evidence.unspent_ability_points, 1);
        let (ready, evidence) = at_souls(&hero, &order, 2000, &cfg);
        assert_eq!(ready.abilities[0].cooldown, 5.0);
        assert_eq!(evidence.ability_ranks.get(&10), Some(&2));
        assert_eq!(evidence.unspent_ability_points, 0);
        assert_eq!(ready.base_health, 900.0);
        assert!((ready.weapon.bullet_damage - 21.4).abs() < 1e-9);
    }

    #[test]
    fn scaling_upgrades_change_scaling_and_unknown_types_stay_visible() {
        let mut ability = hero().abilities.remove(0);
        let mut unknown = Vec::new();
        apply_upgrade(
            &mut ability,
            &serde_json::json!({"property_upgrades":[{"name":"Damage","bonus":0.3,"upgrade_type":"EAddToScale","scale_stat_filter":"ETechPower"},{"name":"Damage","bonus":2.0,"upgrade_type":"EMultiplyScale"},{"name":"Mystery","bonus":4.0,"upgrade_type":"EUnknown"}]}),
            &mut unknown,
        );
        assert_eq!(ability.base_effect, 60.0);
        assert_eq!(
            ability
                .scaling
                .iter()
                .find(|scale| scale.stat == "Damage")
                .unwrap()
                .per_spirit,
            Some(0.6)
        );
        assert!(unknown.iter().any(|message| message.contains("EUnknown")));
    }

    #[test]
    fn repeated_or_foreign_source_tail_is_excluded_from_the_shared_published_order() {
        let hero = hero();
        let order = vec![
            AbilityStep {
                ability_id: 10,
                currency_type: 2,
                delta: -1,
            },
            AbilityStep {
                ability_id: 10,
                currency_type: 1,
                delta: -1,
            },
            AbilityStep {
                ability_id: 20,
                currency_type: 2,
                delta: -1,
            },
            AbilityStep {
                ability_id: 10,
                currency_type: 2,
                delta: -1,
            },
        ];
        let (coherent, notes) = coherent_order(&hero, &order);
        assert_eq!(coherent, &order[..3]);
        assert!(notes[0].contains("Quellenmangel"));
        let mut unknown = order;
        unknown[3].ability_id = 999;
        assert_eq!(coherent_order(&hero, &unknown).0, coherent);
    }
}
