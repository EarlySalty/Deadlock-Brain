use crate::{
    AbilityModel, AbilityRole, DamagePlan, DamageType, HeroModel, ItemModel, ReasonerConfig,
    SlotType, WeaponProfile,
};

pub fn purchase_bonus_value(item: &ItemModel, hero: &HeroModel) -> f64 {
    let bonuses = match item.slot {
        SlotType::Weapon => &hero.purchase_bonuses.weapon,
        SlotType::Vitality => &hero.purchase_bonuses.vitality,
        SlotType::Spirit => &hero.purchase_bonuses.spirit,
    };
    let bonus = bonuses
        .iter()
        .find(|bonus| bonus.tier == item.tier)
        .map(|bonus| bonus.value)
        .unwrap_or_default();
    match item.slot {
        SlotType::Weapon => bonus / 100.0 * hero.damage_plan.weapon_dps,
        SlotType::Spirit => {
            let spirit_scale = hero
                .scaling
                .iter()
                .filter_map(|stat| stat.per_spirit)
                .sum::<f64>();
            bonus * (spirit_scale + hero.damage_plan.spirit_dps / 100.0).max(0.01)
        }
        SlotType::Vitality => bonus * hero.base_health.max(1.0) / 100.0,
    }
}

pub fn combat_window_value(item: &ItemModel, hero: &HeroModel, cfg: &ReasonerConfig) -> f64 {
    let active = property_effect(&item.properties, hero, cfg);
    let passive = property_effect(&item.passive_properties, hero, cfg);
    passive + active * condition_factor(item, cfg)
}

pub fn condition_factor(item: &ItemModel, cfg: &ReasonerConfig) -> f64 {
    match &item.condition {
        crate::ConditionKind::None => 1.0,
        crate::ConditionKind::ActiveCooldown { uptime, cooldown } => {
            if *cooldown > 0.0 {
                (*uptime).clamp(0.0, 1.0)
            } else {
                1.0
            }
        }
        crate::ConditionKind::ActionBound { .. } => 1.0,
        crate::ConditionKind::RampUp { ramp_seconds } => {
            (1.0 - ramp_seconds / cfg.combat_window_seconds.max(1.0)).max(0.0)
        }
        crate::ConditionKind::StateBound { threshold } => (1.0 - threshold).clamp(0.0, 1.0),
        crate::ConditionKind::MeleeBound => 0.5,
        crate::ConditionKind::ShotBound => 0.75,
    }
}

pub fn proc_stacks(tick_rate: f64, proc_cooldown: f64, window: f64) -> f64 {
    if tick_rate <= 0.0 || proc_cooldown <= 0.0 || window <= 0.0 {
        return 0.0;
    }
    (window / tick_rate).min(window / proc_cooldown)
}

pub fn per_slot_value(combat: f64, purchase: f64) -> f64 {
    combat + purchase
}

pub fn per_soul_value(combat: f64, cost: i64) -> f64 {
    if cost > 0 {
        combat / cost as f64
    } else {
        0.0
    }
}

pub fn active_value(item: &ItemModel, hero: &HeroModel, cfg: &ReasonerConfig) -> f64 {
    let mut active = item.clone();
    active.passive_properties.clear();
    active.condition = crate::ConditionKind::None;
    combat_window_value(&active, hero, cfg)
}

pub fn passive_value(item: &ItemModel, hero: &HeroModel, cfg: &ReasonerConfig) -> f64 {
    let mut passive = item.clone();
    passive.properties.clear();
    passive.condition = crate::ConditionKind::None;
    combat_window_value(&passive, hero, cfg)
}

pub fn imbue_target(item: &ItemModel, hero: &HeroModel, cfg: &ReasonerConfig) -> Option<i64> {
    if !item.imbueable {
        return None;
    }
    hero.abilities
        .iter()
        .map(|ability| (ability_dps(ability, hero, cfg), ability.ability_id))
        .max_by(|left, right| left.0.total_cmp(&right.0))
        .map(|(_, ability_id)| ability_id)
}

pub fn buy_phase(item: &ItemModel) -> crate::BuyPhase {
    if item.cost <= 1600 {
        crate::BuyPhase::Lane
    } else if item.cost <= 6400 {
        crate::BuyPhase::Core
    } else {
        crate::BuyPhase::Late
    }
}

pub fn weapon_dps(weapon: &WeaponProfile, window: f64) -> f64 {
    if window <= 0.0 || weapon.shots_per_second <= 0.0 || weapon.clip_size <= 0.0 {
        return 0.0;
    }
    if weapon.sustained_dps > 0.0 {
        return weapon.sustained_dps;
    }
    let cycle = weapon.clip_size / weapon.shots_per_second;
    let cycle_with_reload = cycle + weapon.reload_duration.max(0.0);
    if cycle_with_reload <= 0.0 {
        return 0.0;
    }
    let magazines = (window / cycle_with_reload).max(1.0);
    weapon.bullet_damage * weapon.clip_size * magazines / window
}

pub fn ability_dps(ability: &AbilityModel, hero: &HeroModel, cfg: &ReasonerConfig) -> f64 {
    let scaling = ability
        .scaling
        .iter()
        .map(|stat| stat.per_spirit.unwrap_or(stat.per_level).abs())
        .sum::<f64>();
    if scaling <= 0.0 {
        return 0.0;
    }
    let channel = ability.channel_time.unwrap_or(1.0).max(1.0);
    let recharge = ability.cooldown.max(channel).max(1.0);
    let casts = (cfg.combat_window_seconds / recharge).min(
        cfg.combat_window_seconds * cfg.channel_uptime / channel * ability.charges.max(1) as f64,
    );
    let role_factor = if ability
        .roles
        .iter()
        .any(|role| matches!(role, AbilityRole::Damage | AbilityRole::Ultimate))
    {
        1.0
    } else if ability
        .roles
        .iter()
        .any(|role| matches!(role, AbilityRole::Sustain))
    {
        0.35
    } else {
        0.2
    };
    let hero_factor = if matches!(ability.damage_type, DamageType::Weapon) {
        hero.damage_plan.weapon_share.max(0.1)
    } else {
        (1.0 - hero.damage_plan.weapon_share).max(0.1)
    };
    scaling * casts / cfg.combat_window_seconds.max(1.0) * role_factor * hero_factor
}

pub fn damage_plan(hero: &HeroModel, cfg: &ReasonerConfig) -> DamagePlan {
    let weapon_dps = weapon_dps(&hero.weapon, cfg.combat_window_seconds);
    let spirit_dps = hero
        .abilities
        .iter()
        .map(|ability| ability_dps(ability, hero, cfg))
        .sum::<f64>();
    let total = weapon_dps + spirit_dps;
    let weapon_share = if total > 0.0 { weapon_dps / total } else { 0.0 };
    let primary_axis = if weapon_share > 0.6 {
        DamageType::Weapon
    } else if weapon_share < 0.4 {
        DamageType::Spirit
    } else {
        DamageType::Hybrid
    };
    DamagePlan {
        weapon_dps,
        spirit_dps,
        weapon_share,
        primary_axis,
    }
}

fn property_effect(
    properties: &std::collections::BTreeMap<String, f64>,
    hero: &HeroModel,
    cfg: &ReasonerConfig,
) -> f64 {
    properties
        .iter()
        .map(|(name, value)| property_value(name, *value, hero, cfg))
        .sum()
}

fn property_value(name: &str, value: f64, hero: &HeroModel, cfg: &ReasonerConfig) -> f64 {
    let lower = name.to_ascii_lowercase();
    let magnitude = value.abs();
    let percent = magnitude / 100.0;
    if lower.contains("proccooldown") || lower.contains("proc_cooldown") {
        return 0.0;
    }
    if lower.contains("spiritpower") || lower.contains("techpower") {
        let scaling = hero
            .scaling
            .iter()
            .filter_map(|stat| stat.per_spirit)
            .sum::<f64>()
            .max(0.01);
        return magnitude * scaling * cfg.combat_window_seconds * 0.1;
    }
    if lower.contains("weapon")
        || lower.contains("bullet")
        || lower.contains("firerate")
        || lower.contains("ammo")
        || lower.contains("reload")
    {
        let factor = if magnitude > 1.0 { percent } else { magnitude };
        return hero.damage_plan.weapon_dps * factor;
    }
    if lower.contains("damage")
        || lower.contains("dps")
        || lower.contains("bleed")
        || lower.contains("burn")
        || lower.contains("shred")
        || lower.contains("lifesteal")
    {
        return magnitude
            * hero
                .damage_plan
                .weapon_dps
                .max(hero.damage_plan.spirit_dps)
                .max(1.0)
            / 100.0;
    }
    if lower.contains("health")
        || lower.contains("shield")
        || lower.contains("resist")
        || lower.contains("armor")
        || lower.contains("regen")
    {
        return magnitude * hero.base_health.max(1.0) / 100.0;
    }
    if lower.contains("cooldown") || lower.contains("duration") || lower.contains("charge") {
        return magnitude * hero.damage_plan.spirit_dps.max(1.0) / 100.0;
    }
    magnitude / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ConditionKind, PurchaseBonuses, ScalingStat, TierBonus};

    fn hero() -> HeroModel {
        HeroModel {
            hero_id: 25,
            name: "Warden".to_string(),
            archetype: "brawler".to_string(),
            base_health: 600.0,
            level_curve: Vec::new(),
            purchase_bonuses: PurchaseBonuses {
                spirit: vec![TierBonus {
                    tier: 2,
                    value: 7.0,
                    value_type: "spirit".to_string(),
                }],
                weapon: vec![TierBonus {
                    tier: 2,
                    value: 10.0,
                    value_type: "weapon".to_string(),
                }],
                vitality: vec![TierBonus {
                    tier: 2,
                    value: 8.0,
                    value_type: "health".to_string(),
                }],
            },
            scaling: vec![ScalingStat {
                stat: "SpiritPower".to_string(),
                per_level: 0.0,
                per_spirit: Some(0.3),
            }],
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
            abilities: Vec::new(),
            damage_plan: DamagePlan {
                weapon_dps: 40.0,
                spirit_dps: 10.0,
                weapon_share: 0.8,
                primary_axis: DamageType::Weapon,
            },
        }
    }

    #[test]
    fn purchase_bonus_uses_slot_and_tier() {
        let item = ItemModel {
            item_id: 1,
            name: "x".to_string(),
            slot: SlotType::Weapon,
            tier: 2,
            cost: 1600,
            is_active: false,
            shopable: true,
            disabled: false,
            damage_axis: DamageType::Weapon,
            defense_kind: Vec::new(),
            properties: Default::default(),
            passive_properties: Default::default(),
            condition: ConditionKind::None,
            proc_cooldown: None,
            imbueable: false,
        };
        assert_eq!(purchase_bonus_value(&item, &hero()), 4.0);
    }

    #[test]
    fn proc_stacks_respect_both_rates() {
        assert_eq!(proc_stacks(0.1, 0.7, 7.0), 10.0);
    }

    #[test]
    fn condition_factor_applies_ramp_and_active_uptime() {
        let cfg = ReasonerConfig::default();
        let ramp = ItemModel {
            item_id: 1,
            name: "x".to_string(),
            slot: SlotType::Spirit,
            tier: 1,
            cost: 800,
            is_active: false,
            shopable: true,
            disabled: false,
            damage_axis: DamageType::Spirit,
            defense_kind: Vec::new(),
            properties: Default::default(),
            passive_properties: Default::default(),
            condition: ConditionKind::RampUp { ramp_seconds: 20.0 },
            proc_cooldown: None,
            imbueable: false,
        };
        assert_eq!(condition_factor(&ramp, &cfg), 0.5);
        let active = ItemModel {
            condition: ConditionKind::ActiveCooldown {
                uptime: 0.25,
                cooldown: 20.0,
            },
            ..ramp
        };
        assert_eq!(condition_factor(&active, &cfg), 0.25);
    }

    #[test]
    fn weapon_dps_uses_magazine_and_reload() {
        let value = weapon_dps(&hero().weapon, 40.0);
        assert!((value - 33.3333333333).abs() < 1e-9);
    }

    #[test]
    fn per_slot_value_adds_combat_and_purchase_effect() {
        assert_eq!(per_slot_value(12.0, 3.0), 15.0);
    }

    #[test]
    fn per_soul_value_uses_item_cost() {
        assert_eq!(per_soul_value(160.0, 1600), 0.1);
        assert_eq!(per_soul_value(160.0, 0), 0.0);
    }

    #[test]
    fn buy_phase_follows_soul_thresholds() {
        let item = ItemModel {
            item_id: 1,
            name: "x".to_string(),
            slot: SlotType::Spirit,
            tier: 1,
            cost: 800,
            is_active: false,
            shopable: true,
            disabled: false,
            damage_axis: DamageType::Spirit,
            defense_kind: Vec::new(),
            properties: Default::default(),
            passive_properties: Default::default(),
            condition: ConditionKind::None,
            proc_cooldown: None,
            imbueable: false,
        };
        assert_eq!(buy_phase(&item), crate::BuyPhase::Lane);
        let item = ItemModel { cost: 3200, ..item };
        assert_eq!(buy_phase(&item), crate::BuyPhase::Core);
        let item = ItemModel {
            cost: 12800,
            ..item
        };
        assert_eq!(buy_phase(&item), crate::BuyPhase::Late);
    }

    #[test]
    fn imbue_target_chooses_highest_ability_dps() {
        let mut hero = hero();
        hero.abilities = vec![
            AbilityModel {
                ability_id: 1,
                class_name: "low".to_string(),
                slot: 1,
                roles: vec![AbilityRole::Damage],
                scaling: vec![crate::ScalingStat {
                    stat: "Spirit".to_string(),
                    per_level: 1.0,
                    per_spirit: None,
                }],
                channel_time: None,
                charges: 1,
                cooldown: 20.0,
                scaling_step: None,
                damage_type: DamageType::Spirit,
            },
            AbilityModel {
                ability_id: 2,
                class_name: "high".to_string(),
                slot: 2,
                roles: vec![AbilityRole::Damage],
                scaling: vec![crate::ScalingStat {
                    stat: "Spirit".to_string(),
                    per_level: 3.0,
                    per_spirit: None,
                }],
                channel_time: None,
                charges: 1,
                cooldown: 5.0,
                scaling_step: None,
                damage_type: DamageType::Spirit,
            },
        ];
        let item = ItemModel {
            item_id: 1,
            name: "imbue".to_string(),
            slot: SlotType::Weapon,
            tier: 2,
            cost: 1600,
            is_active: false,
            shopable: true,
            disabled: false,
            damage_axis: DamageType::Hybrid,
            defense_kind: Vec::new(),
            properties: Default::default(),
            passive_properties: Default::default(),
            condition: ConditionKind::None,
            proc_cooldown: None,
            imbueable: true,
        };
        assert_eq!(
            imbue_target(&item, &hero, &ReasonerConfig::default()),
            Some(2)
        );
    }

    #[test]
    fn damage_plan_detects_weapon_core() {
        let value = damage_plan(&hero(), &ReasonerConfig::default());
        assert_eq!(value.primary_axis, DamageType::Weapon);
        assert!(value.weapon_share > 0.6);
    }
}
