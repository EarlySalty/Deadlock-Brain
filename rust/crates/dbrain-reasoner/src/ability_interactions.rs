use crate::AbilityModel;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct TimedAmplification {
    pub fraction: f64,
    pub duration: f64,
    pub max_stacks: usize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct AbilityInteractions {
    pub weapon_amp: Option<TimedAmplification>,
    pub all_damage_amp: Option<TimedAmplification>,
    pub reset_ability_class: Option<&'static str>,
    pub missing_health_heal_fraction: f64,
    pub native_damage_heal_fraction: f64,
}

fn positive(ability: &AbilityModel, key: &str) -> f64 {
    ability
        .properties
        .get(key)
        .copied()
        .filter(|v| v.is_finite() && *v > 0.0)
        .unwrap_or(0.0)
}

impl AbilityInteractions {
    pub fn from_ability(ability: &AbilityModel, duration_multiplier: f64) -> Self {
        let duration = |key: &str| {
            positive(ability, key)
                * if ability.duration_scaling.contains(key) {
                    if duration_multiplier.is_finite() {
                        duration_multiplier.max(0.0)
                    } else {
                        0.0
                    }
                } else {
                    1.0
                }
        };
        let mut result = Self::default();
        match ability.class_name.as_str() {
            "citadel_ability_hook" => {
                let fraction = positive(ability, "BulletAmp") / 100.0;
                let duration = duration("BulletAmpDuration");
                if fraction > 0.0 && duration > 0.0 {
                    result.weapon_amp = Some(TimedAmplification {
                        fraction,
                        duration,
                        max_stacks: 1,
                    });
                }
            }
            "ability_blood_shards" => {
                let fraction = positive(ability, "VulnerabilityPerStack") / 100.0;
                let duration = duration("DebuffDuration");
                let max_stacks = positive(ability, "MaxStacks").floor() as usize;
                if fraction > 0.0 && duration > 0.0 && max_stacks > 0 {
                    result.all_damage_amp = Some(TimedAmplification {
                        fraction,
                        duration,
                        max_stacks,
                    });
                }
            }
            "citadel_ability_uppercut" => {
                if positive(ability, "RestoreHookCooldown") > 0.0 {
                    result.reset_ability_class = Some("citadel_ability_hook");
                }
                result.missing_health_heal_fraction =
                    (positive(ability, "MissingHPHeal") / 100.0).min(1.0);
            }
            "citadel_ability_bebop_laser_beam" => {
                result.native_damage_heal_fraction = positive(ability, "BeamLifesteal") / 100.0;
            }
            _ => {}
        }
        result
    }

    pub fn missing_health_heal(self, current: f64, maximum: f64) -> f64 {
        if !current.is_finite() || !maximum.is_finite() || current <= 0.0 || maximum <= 0.0 {
            return 0.0;
        }
        (maximum - current).max(0.0) * self.missing_health_heal_fraction
    }

    pub fn native_healing(self, actual_ability_damage: f64) -> f64 {
        if actual_ability_damage.is_finite() {
            actual_ability_damage.max(0.0) * self.native_damage_heal_fraction
        } else {
            0.0
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Stack {
    end: f64,
    fraction: f64,
}

#[derive(Debug, Clone, Default)]
pub struct TargetAmplification {
    weapon: Option<Stack>,
    all: Vec<Stack>,
}

fn valid_amplification(amp: &TimedAmplification) -> bool {
    amp.fraction.is_finite()
        && amp.fraction > 0.0
        && amp.duration.is_finite()
        && amp.duration > 0.0
        && amp.max_stacks > 0
}

impl TargetAmplification {
    pub fn clear(&mut self) {
        self.weapon = None;
        self.all.clear();
    }

    pub fn on_hit(&mut self, effect: AbilityInteractions, time: f64) {
        if !time.is_finite() {
            return;
        }
        if let Some(amp) = effect.weapon_amp.filter(valid_amplification) {
            self.weapon = Some(Stack {
                end: time + amp.duration,
                fraction: amp.fraction,
            });
        }
        self.all.retain(|stack| stack.end > time);
        if let Some(amp) = effect.all_damage_amp.filter(valid_amplification) {
            if self.all.len() >= amp.max_stacks {
                self.all.remove(0);
            }
            self.all.push(Stack {
                end: time + amp.duration,
                fraction: amp.fraction,
            });
        }
    }

    pub fn spirit_multiplier(&self, time: f64) -> f64 {
        1.0 + self
            .all
            .iter()
            .filter(|stack| stack.end > time)
            .map(|stack| stack.fraction)
            .sum::<f64>()
    }

    pub fn weapon_multiplier(&self, time: f64) -> f64 {
        self.spirit_multiplier(time)
            * (1.0
                + self
                    .weapon
                    .filter(|stack| stack.end > time)
                    .map_or(0.0, |stack| stack.fraction))
    }
}

pub fn quantified_property(ability: &AbilityModel, key: &str) -> bool {
    let effect = AbilityInteractions::from_ability(ability, 1.0);
    match ability.class_name.as_str() {
        "citadel_ability_hook" => {
            effect.weapon_amp.is_some() && matches!(key, "BulletAmp" | "BulletAmpDuration")
        }
        "ability_blood_shards" => {
            effect.all_damage_amp.is_some()
                && matches!(
                    key,
                    "VulnerabilityPerStack" | "DebuffDuration" | "MaxStacks"
                )
        }
        "citadel_ability_uppercut" => match key {
            "RestoreHookCooldown" => effect.reset_ability_class.is_some(),
            "MissingHPHeal" => effect.missing_health_heal_fraction > 0.0,
            _ => false,
        },
        "citadel_ability_bebop_laser_beam" => {
            key == "BeamLifesteal" && effect.native_damage_heal_fraction > 0.0
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    fn raw_ability(class: &str, rank: usize) -> AbilityModel {
        let raw: Value =
            serde_json::from_str(include_str!("../testdata/ability-interactions/raw.json"))
                .unwrap();
        let payload = &raw
            .as_array()
            .unwrap()
            .iter()
            .find(|row| row["payload"]["class_name"] == class)
            .unwrap()["payload"];
        let number = |v: &Value| {
            v.as_f64()
                .or_else(|| v.as_str().and_then(|s| s.parse::<f64>().ok()))
                .unwrap_or(0.0)
        };
        let mut ability = AbilityModel {
            ability_id: payload["id"].as_i64().unwrap_or(0),
            class_name: class.into(),
            slot: 1,
            properties: payload["properties"]
                .as_object()
                .unwrap()
                .iter()
                .map(|(key, value)| (key.clone(), number(&value["value"])))
                .collect(),
            duration_scaling: payload["properties"]
                .as_object()
                .unwrap()
                .iter()
                .filter(|(_, value)| {
                    value["scale_function"]["specific_stat_scale_type"] == "ETechDuration"
                })
                .map(|(key, _)| key.clone())
                .collect(),
            item_proc_disabled: false,
            upgrades: payload["upgrades"].as_array().unwrap().clone(),
            roles: vec![],
            scaling: vec![],
            channel_time: None,
            charges: 1,
            cooldown: 0.0,
            scaling_step: None,
            damage_type: crate::DamageType::Spirit,
            base_effect: 0.0,
            tick_rate: None,
            duration: None,
        };
        for upgrade in ability.upgrades.iter().take(rank) {
            for field in upgrade["property_upgrades"].as_array().unwrap() {
                *ability
                    .properties
                    .entry(field["name"].as_str().unwrap().into())
                    .or_default() += number(&field["bonus"]);
            }
        }
        ability
    }

    #[test]
    fn coverage_requires_complete_rank_effect_and_rejects_unimplemented_keys() {
        let mut hook = raw_ability("citadel_ability_hook", 1);
        assert!(quantified_property(&hook, "BulletAmp"));
        hook.properties.remove("BulletAmpDuration");
        assert!(!quantified_property(&hook, "BulletAmp"));
        assert!(!quantified_property(
            &raw_ability("citadel_ability_uppercut", 2),
            "MissingHPHeal"
        ));
        assert!(quantified_property(
            &raw_ability("citadel_ability_uppercut", 3),
            "MissingHPHeal"
        ));
        assert!(!quantified_property(
            &raw_ability("citadel_ability_uppercut", 3),
            "UppercutDamage"
        ));
        assert!(!quantified_property(
            &raw_ability("ability_health_swap", 3),
            "EnemyMinHealthPct"
        ));
    }

    #[test]
    fn real_hook_rank_one_is_weapon_only_and_expires() {
        assert_eq!(
            AbilityInteractions::from_ability(&raw_ability("citadel_ability_hook", 0), 1.0),
            AbilityInteractions::default()
        );
        let hook = AbilityInteractions::from_ability(&raw_ability("citadel_ability_hook", 1), 1.5);
        let mut target = TargetAmplification::default();
        target.on_hit(hook, 2.0);
        assert_eq!(target.weapon_multiplier(10.9), 1.2);
        assert_eq!(target.spirit_multiplier(3.0), 1.0);
        assert_eq!(target.weapon_multiplier(11.0), 1.0);
        target.on_hit(hook, 12.0);
        target.clear();
        assert_eq!(target.weapon_multiplier(12.0), 1.0);
    }

    #[test]
    fn real_uppercut_and_beam_upgrades_only_apply_at_rank_three() {
        for rank in 0..3 {
            let uppercut = AbilityInteractions::from_ability(
                &raw_ability("citadel_ability_uppercut", rank),
                1.0,
            );
            assert_eq!(uppercut.reset_ability_class, None);
            assert_eq!(uppercut.missing_health_heal(100.0, 1000.0), 0.0);
            let beam = AbilityInteractions::from_ability(
                &raw_ability("citadel_ability_bebop_laser_beam", rank),
                1.0,
            );
            assert_eq!(beam.native_healing(100.0), 0.0);
        }
        let uppercut =
            AbilityInteractions::from_ability(&raw_ability("citadel_ability_uppercut", 3), 1.0);
        assert_eq!(uppercut.reset_ability_class, Some("citadel_ability_hook"));
        assert_eq!(uppercut.missing_health_heal(100.0, 1000.0), 162.0);
        assert_eq!(uppercut.missing_health_heal(1000.0, 1000.0), 0.0);
        assert_eq!(uppercut.missing_health_heal(0.0, 1000.0), 0.0);
        let beam = AbilityInteractions::from_ability(
            &raw_ability("citadel_ability_bebop_laser_beam", 3),
            1.0,
        );
        assert_eq!(beam.native_healing(100.0), 65.0);
        assert_eq!(beam.native_healing(-100.0), 0.0);
    }

    #[test]
    fn real_malice_caps_and_expires_without_shard_multiplication() {
        let base = AbilityInteractions::from_ability(&raw_ability("ability_blood_shards", 0), 1.0);
        assert_eq!(base.all_damage_amp.unwrap().fraction, 0.07);
        let rank3 = AbilityInteractions::from_ability(&raw_ability("ability_blood_shards", 3), 1.0);
        assert_eq!(
            rank3.all_damage_amp.unwrap(),
            TimedAmplification {
                fraction: 0.15,
                duration: 9.0,
                max_stacks: 5
            }
        );
        let mut target = TargetAmplification::default();
        for time in 0..6 {
            target.on_hit(rank3, f64::from(time));
        }
        assert_eq!(target.spirit_multiplier(5.0), 1.75);
        assert_eq!(target.weapon_multiplier(5.0), 1.75);
        assert_eq!(target.spirit_multiplier(10.0), 1.6);
        assert_eq!(target.spirit_multiplier(14.0), 1.0);
    }
}
