use crate::{
    AbilityModel, AbilityRole, DamagePlan, DamageType, HeroModel, ItemModel, ReasonerConfig,
    SlotType, WeaponProfile,
};

pub fn purchase_bonus_value(item: &ItemModel, hero: &HeroModel) -> f64 {
    purchase_bonus_value_with_config(item, hero, &ReasonerConfig::default())
}

pub fn purchase_bonus_value_with_config(
    item: &ItemModel,
    hero: &HeroModel,
    cfg: &ReasonerConfig,
) -> f64 {
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
        SlotType::Spirit => bonus * spirit_power_value(hero, cfg),
        SlotType::Vitality => bonus * hero.base_health / 100.0 / cfg.combat_window_seconds.max(1.0),
    }
}

pub fn combat_window_value(item: &ItemModel, hero: &HeroModel, cfg: &ReasonerConfig) -> f64 {
    passive_value(item, hero, cfg)
        + active_value(item, hero, cfg) * condition_factor_for_hero(item, hero, cfg)
}

pub fn weapon_uptime(hero: &HeroModel) -> f64 {
    if hero.weapon.shots_per_second <= 0.0 || hero.weapon.clip_size <= 0.0 {
        return 0.0;
    }
    let firing = hero.weapon.clip_size / hero.weapon.shots_per_second;
    firing / (firing + hero.weapon.reload_duration.max(0.0))
}

pub fn hero_melees(hero: &HeroModel) -> bool {
    hero.archetype.to_ascii_lowercase().contains("melee")
        || hero
            .abilities
            .iter()
            .any(|ability| ability.class_name.to_ascii_lowercase().contains("melee"))
}

pub fn action_in_rotation(action: &str, hero: &HeroModel) -> bool {
    let action = action.to_ascii_lowercase();
    match action.as_str() {
        "shot" | "shoot" => weapon_uptime(hero) > 0.0,
        "melee" => hero_melees(hero),
        "activation" | "cast" => !hero.abilities.is_empty(),
        "dash" | "dash-jump" | "dash_jump" => hero
            .abilities
            .iter()
            .any(|ability| ability.roles.contains(&AbilityRole::Mobility)),
        _ => hero
            .abilities
            .iter()
            .any(|ability| ability.class_name.eq_ignore_ascii_case(&action)),
    }
}

pub fn hit_rate(threshold: f64) -> f64 {
    1.0 - threshold.clamp(0.0, 1.0)
}

pub fn condition_factor_for_hero(item: &ItemModel, hero: &HeroModel, cfg: &ReasonerConfig) -> f64 {
    match &item.condition {
        crate::ConditionKind::MeleeBound => {
            if hero_melees(hero) {
                1.0
            } else {
                0.0
            }
        }
        crate::ConditionKind::ShotBound => weapon_uptime(hero),
        crate::ConditionKind::ActionBound { action } => {
            if action_in_rotation(action, hero) {
                1.0
            } else {
                0.5
            }
        }
        _ => condition_factor(item, cfg),
    }
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
        crate::ConditionKind::ActionBound { .. } => 0.5,
        crate::ConditionKind::RampUp { ramp_seconds } => {
            (1.0 - ramp_seconds / cfg.combat_window_seconds.max(1.0)).max(0.0)
        }
        crate::ConditionKind::StateBound { threshold } => hit_rate(*threshold),
        crate::ConditionKind::MeleeBound | crate::ConditionKind::ShotBound => 0.0,
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
    let properties = item
        .properties
        .iter()
        .filter(|(name, _)| !item.passive_properties.contains_key(*name))
        .map(|(name, value)| (name.clone(), *value))
        .collect();
    item_property_effect(&properties, item, hero, cfg)
}

pub fn passive_value(item: &ItemModel, hero: &HeroModel, cfg: &ReasonerConfig) -> f64 {
    let factor = condition_factor_for_hero(item, hero, cfg);
    let raw: f64 = item
        .passive_properties
        .iter()
        .map(|(name, value)| {
            let lower = name.to_ascii_lowercase();
            let trigger_bound = lower.contains("shred")
                || lower.contains("armorreduction")
                || lower.contains("debuff");
            property_value(name, *value, hero, cfg) * if trigger_bound { factor } else { 1.0 }
        })
        .sum();
    let raw = raw + reload_value(&item.passive_properties, item, hero, cfg);
    let proc = proc_property_effect(&item.passive_properties, item, hero, cfg);
    raw + proc * condition_factor_for_hero(item, hero, cfg)
}

pub fn imbue_target(item: &ItemModel, hero: &HeroModel, cfg: &ReasonerConfig) -> Option<i64> {
    if !item.imbueable {
        return None;
    }
    hero.abilities
        .iter()
        .map(|ability| {
            (
                ability_dps(ability, hero, cfg) * imbue_gain(item, ability, cfg),
                ability.ability_id,
            )
        })
        .max_by(|left, right| left.0.total_cmp(&right.0))
        .map(|(_, ability_id)| ability_id)
}

pub fn imbue_gain(item: &ItemModel, ability: &AbilityModel, cfg: &ReasonerConfig) -> f64 {
    let cooldown_reduction = item
        .properties
        .get("CooldownReduction")
        .copied()
        .unwrap_or_default()
        / 100.0;
    let mut changed = ability.clone();
    changed.cooldown *= 1.0 - cooldown_reduction.clamp(0.0, 0.99);
    changed.charges += item
        .properties
        .get("BonusAbilityCharges")
        .copied()
        .unwrap_or_default() as i64;
    let before = ability_casts(ability, cfg);
    if before <= 0.0 {
        return 0.0;
    }
    let direct = item.properties.get("Damage").copied().unwrap_or_default();
    ability_casts(&changed, cfg) / before * (1.0 + direct / ability.base_effect.max(1.0))
}

pub fn scaling_souls(hero: &HeroModel) -> Option<i64> {
    let upgrade = hero
        .abilities
        .iter()
        .filter_map(|ability| ability.scaling_step.as_ref())
        .map(|step| step.upgrade_index)
        .min()?;
    let points: i64 = [1, 2, 5].iter().take((upgrade + 1).max(0) as usize).sum();
    hero.level_curve
        .iter()
        .filter(|point| ![1, 3, 5, 8].contains(&point.level))
        .nth(points.saturating_sub(1) as usize)
        .map(|point| point.required_souls)
}

pub fn buy_phase_for_hero(item: &ItemModel, hero: &HeroModel) -> crate::BuyPhase {
    if !item.defense_kind.is_empty()
        || matches!(item.condition, crate::ConditionKind::StateBound { .. })
    {
        return crate::BuyPhase::Late;
    }
    let Some(pivot) = scaling_souls(hero) else {
        return buy_phase(item);
    };
    let late = hero
        .level_curve
        .iter()
        .map(|point| point.required_souls)
        .max()
        .unwrap_or(pivot);
    if item.cost < pivot {
        crate::BuyPhase::Lane
    } else if item.cost < late {
        crate::BuyPhase::Core
    } else {
        crate::BuyPhase::Late
    }
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
    weapon.bullet_damage * weapon.clip_size / cycle_with_reload
}

pub fn ability_casts(ability: &AbilityModel, cfg: &ReasonerConfig) -> f64 {
    crate::data::ability_cast_count(ability, cfg)
}

fn is_damage_stat(name: &str) -> bool {
    let name = name.to_ascii_lowercase();
    name.contains("damage")
        || name.contains("dps")
        || name == "spirit"
        || name.contains("lifedrain")
}

fn spirit_power_value(hero: &HeroModel, cfg: &ReasonerConfig) -> f64 {
    hero.abilities
        .iter()
        .map(|ability| {
            ability
                .scaling
                .iter()
                .filter(|stat| {
                    is_damage_stat(&stat.stat) || stat.stat.to_ascii_lowercase().contains("barrier")
                })
                .filter_map(|stat| {
                    stat.per_spirit.map(|scale| {
                        let duration = ability.duration.or(ability.channel_time).unwrap_or(1.0);
                        if stat.stat.ends_with("DPS") || stat.stat.ends_with("PerSecond") {
                            scale * duration
                        } else if stat.stat == "TickDamage" || stat.stat == "DamagePerTick" {
                            scale * duration / ability.tick_rate.unwrap_or(1.0).max(0.001)
                        } else {
                            scale
                        }
                    })
                })
                .sum::<f64>()
                * ability_casts(ability, cfg)
                / cfg.combat_window_seconds.max(1.0)
        })
        .sum()
}

pub fn ability_dps(ability: &AbilityModel, _hero: &HeroModel, cfg: &ReasonerConfig) -> f64 {
    crate::data::ability_base_dps(ability, cfg)
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
    let magnitude = value;
    let percent = magnitude / 100.0;
    let window = cfg.combat_window_seconds.max(1.0);
    if lower == "healthdrainedpersecond" {
        return -magnitude.abs();
    }
    if lower.contains("weaponpowerdebuff") {
        return hero.damage_plan.weapon_dps * percent.abs();
    }
    if lower.contains("healampreceivepenalty") || lower.contains("healampregenpenalty") {
        return hero.base_health / window * percent.abs();
    }
    if is_proc_property(name)
        || lower.contains("proccooldown")
        || lower.contains("proc_cooldown")
        || lower.contains("tickrate")
        || lower.contains("maxstacks")
        || lower.starts_with("ability")
        || lower.contains("radius")
        || lower.contains("range")
        || lower.contains("duration")
        || lower.contains("perkill")
        || lower.contains("perdeath")
        || lower == "stealperhit"
        || lower.contains("channelmovespeed")
        || lower.contains("invis")
        || lower.contains("bulletspeed")
        || lower.contains("bulletvelocity")
        || lower == "ammoreloadpercent"
    {
        return 0.0;
    }
    if lower.contains("spiritpower") || lower.contains("techpower") {
        return magnitude * spirit_power_value(hero, cfg);
    }
    if lower.contains("shield")
        || lower.contains("barrier")
        || lower.contains("health")
        || lower.contains("heal")
    {
        if lower.contains("regen") || lower.contains("persecond") {
            return magnitude;
        }
        if lower.contains("percent") || lower.contains("pct") {
            return percent * hero.base_health / window;
        }
        return magnitude / window;
    }
    if lower.contains("resist") || lower.contains("armor") || lower.contains("shred") {
        let basis = if lower.contains("bullet") {
            hero.damage_plan.weapon_dps
        } else if lower.contains("tech") || lower.contains("spirit") {
            hero.damage_plan.spirit_dps
        } else {
            hero.damage_plan.weapon_dps + hero.damage_plan.spirit_dps
        };
        return basis * percent.abs();
    }
    if lower.contains("lifesteal") {
        return percent * (hero.damage_plan.weapon_dps + hero.damage_plan.spirit_dps);
    }
    if lower.contains("weapon")
        || lower.contains("baseattackdamage")
        || lower.contains("bullet")
        || lower.contains("firerate")
        || lower.contains("ammo")
        || lower.contains("reload")
        || lower.contains("clipsize")
    {
        return hero.damage_plan.weapon_dps * percent;
    }
    if lower.contains("damage")
        || lower.contains("dps")
        || lower.contains("bleed")
        || lower.contains("burn")
    {
        return if lower.contains("percent") || lower.contains("pct") {
            percent * (hero.damage_plan.weapon_dps + hero.damage_plan.spirit_dps)
        } else if lower.contains("dps") {
            magnitude
        } else {
            magnitude / window
        };
    }
    if lower.contains("cooldown") || lower.contains("duration") || lower.contains("charge") {
        return magnitude * hero.damage_plan.spirit_dps / 100.0;
    }
    if lower.contains("regen") {
        return magnitude;
    }
    0.0
}

fn is_proc_property(name: &str) -> bool {
    let name = name.to_ascii_lowercase();
    name.contains("procdamage") || name.contains("perstack") || name == "healthstealpcthero"
}

fn proc_property_effect(
    properties: &std::collections::BTreeMap<String, f64>,
    item: &ItemModel,
    hero: &HeroModel,
    cfg: &ReasonerConfig,
) -> f64 {
    let Some(cooldown) = item.proc_cooldown else {
        return 0.0;
    };
    let tick = if matches!(item.condition, crate::ConditionKind::ShotBound) {
        if hero.weapon.shots_per_second <= 0.0 {
            return 0.0;
        }
        1.0 / hero.weapon.shots_per_second
    } else {
        properties
            .iter()
            .find(|(name, _)| name.to_ascii_lowercase().ends_with("tickrate"))
            .map(|(_, value)| *value)
            .unwrap_or(0.0)
    };
    let window = cfg.combat_window_seconds.max(1.0);
    let ability_procs = if tick <= 0.0 && !matches!(item.condition, crate::ConditionKind::ShotBound)
    {
        hero.abilities
            .iter()
            .filter(|ability| ability.base_effect > 0.0)
            .filter_map(|ability| {
                let tick = ability.tick_rate?;
                let duration = ability.duration.or(ability.channel_time)?;
                Some(
                    proc_stacks(tick, cooldown, duration.min(window)) * ability_casts(ability, cfg),
                )
            })
            .max_by(f64::total_cmp)
            .unwrap_or_default()
            .min(window / cooldown.max(0.001))
    } else {
        0.0
    };
    properties
        .iter()
        .filter(|(name, _)| is_proc_property(name))
        .map(|(name, value)| {
            let stacks = proc_stacks(tick, cooldown, window).max(ability_procs).min(
                item.properties
                    .get("MaxStacks")
                    .or_else(|| item.passive_properties.get("MaxStacks"))
                    .copied()
                    .unwrap_or(f64::INFINITY),
            );
            if name.eq_ignore_ascii_case("HealthStealPctHero") {
                let target_max_health = hero.base_health;
                let target_health_lost = value / 100.0 * target_max_health * stacks;
                let own_health_gained = target_health_lost;
                (target_health_lost + own_health_gained) / window
            } else if name.to_ascii_lowercase().contains("percent") {
                value / 100.0 * (hero.damage_plan.weapon_dps + hero.damage_plan.spirit_dps) * stacks
            } else {
                value * stacks / window
            }
        })
        .sum()
}

fn item_property_effect(
    properties: &std::collections::BTreeMap<String, f64>,
    item: &ItemModel,
    hero: &HeroModel,
    cfg: &ReasonerConfig,
) -> f64 {
    property_effect(properties, hero, cfg)
        + proc_property_effect(properties, item, hero, cfg)
        + reload_value(properties, item, hero, cfg)
}

fn reload_value(
    properties: &std::collections::BTreeMap<String, f64>,
    item: &ItemModel,
    hero: &HeroModel,
    cfg: &ReasonerConfig,
) -> f64 {
    let percent = properties
        .get("AmmoReloadPercent")
        .copied()
        .unwrap_or_default()
        / 100.0;
    let cooldown = item
        .properties
        .get("AbilityCooldown")
        .or_else(|| item.passive_properties.get("AbilityCooldown"))
        .copied()
        .unwrap_or_default();
    if percent <= 0.0 || cooldown <= 0.0 {
        return 0.0;
    }
    let window = cfg.combat_window_seconds.max(1.0);
    let saved = (hero.weapon.reload_duration * percent.clamp(0.0, 1.0) * (1.0 + window / cooldown))
        .min(window * (1.0 - weapon_uptime(hero)));
    hero.weapon.bullet_damage * hero.weapon.shots_per_second * saved / window
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ConditionKind, PurchaseBonuses, ScalingStat, TierBonus};

    fn item() -> ItemModel {
        ItemModel {
            item_id: 1,
            name: "fixture".into(),
            slot: SlotType::Spirit,
            tier: 2,
            cost: 1600,
            is_active: false,
            shopable: true,
            disabled: false,
            damage_axis: DamageType::Spirit,
            defense_kind: vec![],
            properties: Default::default(),
            passive_properties: Default::default(),
            condition: ConditionKind::None,
            proc_cooldown: None,
            imbueable: false,
        }
    }

    fn channel() -> AbilityModel {
        AbilityModel {
            ability_id: 1,
            class_name: "LifeDrain".into(),
            slot: 1,
            roles: vec![AbilityRole::Damage],
            scaling: vec![ScalingStat {
                stat: "Damage".into(),
                per_level: 0.0,
                per_spirit: Some(0.3225),
            }],
            channel_time: Some(7.0),
            charges: 1,
            cooldown: 30.0,
            scaling_step: None,
            damage_type: DamageType::Spirit,
            base_effect: 0.0,
            tick_rate: None,
            duration: None,
        }
    }

    #[test]
    fn ability_casts_include_initial_charges_without_exceeding_channel_budget() {
        let mut hero = hero();
        hero.damage_plan.weapon_share = 0.0;
        let cfg = ReasonerConfig::default();
        let mut ability = channel();
        ability.base_effect = 0.3225;
        assert!(
            (ability_dps(&ability, &hero, &cfg) - 0.3225 * (1.0 + 40.0 / 30.0) / 40.0).abs()
                < 1e-12
        );
        ability.charges = 10;
        assert!(
            (ability_dps(&ability, &hero, &cfg) - 0.3225 * (40.0 * 0.55 / 7.0) / 40.0).abs()
                < 1e-12
        );
    }

    #[test]
    fn purchase_bonuses_share_damage_or_health_per_second_units() {
        let mut hero = hero();
        hero.abilities = vec![channel()];
        let cfg = ReasonerConfig::default();
        let mut item = item();
        let spirit = purchase_bonus_value(&item, &hero);
        item.properties.insert("SpiritPower".into(), 7.0);
        assert!((spirit - combat_window_value(&item, &hero, &cfg)).abs() < 1e-12);
        item.slot = SlotType::Vitality;
        assert!((purchase_bonus_value(&item, &hero) - 48.0 / 40.0).abs() < 1e-12);
    }

    #[test]
    fn spirit_purchase_values_duration_scaled_damage_per_activation() {
        let mut hero = hero();
        let mut ability = channel();
        ability.scaling[0].stat = "PulseDPS".into();
        ability.scaling[0].per_spirit = Some(1.3);
        ability.duration = Some(6.0);
        ability.channel_time = None;
        ability.cooldown = 180.0;
        hero.abilities = vec![ability];
        let value = purchase_bonus_value(&item(), &hero);
        assert!((value - 7.0 * 1.3 * 6.0 * (1.0 + 40.0 / 180.0) / 40.0).abs() < 1e-12);
    }

    #[test]
    fn property_values_distinguish_flat_shields_percent_damage_and_metadata() {
        let hero = hero();
        let cfg = ReasonerConfig::default();
        assert_eq!(
            property_value("BulletShieldMaxHealth", 100.0, &hero, &cfg),
            2.5
        );
        assert_eq!(
            property_value("BaseAttackDamagePercentBonus", 10.0, &hero, &cfg),
            4.0
        );
        assert_eq!(property_value("MaxStacks", 9999.0, &hero, &cfg), 0.0);
        assert_eq!(property_value("AbilityCooldown", 30.0, &hero, &cfg), 0.0);
        assert_eq!(
            property_value("BonusBulletSpeedPercent", 200.0, &hero, &cfg),
            0.0
        );
        let props = [
            ("BulletShieldMaxHealth".into(), 100.0),
            ("BaseAttackDamagePercentBonus".into(), 10.0),
        ]
        .into_iter()
        .collect();
        assert_eq!(property_effect(&props, &hero, &cfg), 6.5);
    }

    #[test]
    fn passive_and_active_values_follow_seven_seconds_per_thirty() {
        let mut item = item();
        item.is_active = true;
        item.properties.insert("WeaponDamage".into(), 10.0);
        item.passive_properties
            .insert("PassiveHealth".into(), 100.0);
        item.condition = ConditionKind::ActiveCooldown {
            uptime: 7.0 / 30.0,
            cooldown: 30.0,
        };
        let hero = hero();
        let cfg = ReasonerConfig::default();
        assert_eq!(active_value(&item, &hero, &cfg), 4.0);
        assert_eq!(passive_value(&item, &hero, &cfg), 2.5);
        assert!((combat_window_value(&item, &hero, &cfg) - (2.5 + 4.0 * 7.0 / 30.0)).abs() < 1e-12);
    }

    #[test]
    fn shot_proc_value_changes_with_proc_cooldown() {
        let mut item = item();
        item.condition = ConditionKind::ShotBound;
        item.proc_cooldown = Some(0.7);
        item.properties.insert("BoloProcDamage".into(), 10.0);
        let mut hero = hero();
        hero.weapon.shots_per_second = 10.0;
        hero.weapon.reload_duration = 0.0;
        let cfg = ReasonerConfig {
            combat_window_seconds: 7.0,
            ..ReasonerConfig::default()
        };
        assert!((combat_window_value(&item, &hero, &cfg) - 100.0 / 7.0).abs() < 1e-12);
        item.proc_cooldown = Some(1.4);
        assert!((combat_window_value(&item, &hero, &cfg) - 50.0 / 7.0).abs() < 1e-12);
    }

    #[test]
    fn melee_bound_effect_is_zero_without_melee_rotation() {
        let mut item = item();
        item.condition = ConditionKind::MeleeBound;
        item.properties.insert("Shred".into(), 10.0);
        assert_eq!(
            combat_window_value(&item, &hero(), &ReasonerConfig::default()),
            0.0
        );
    }

    #[test]
    fn passive_melee_shred_requires_melee_but_health_remains_passive() {
        let mut item = item();
        item.condition = ConditionKind::MeleeBound;
        item.passive_properties = [
            ("TechArmorReduction".into(), 10.0),
            ("PassiveHealth".into(), 100.0),
        ]
        .into_iter()
        .collect();
        assert_eq!(
            combat_window_value(&item, &hero(), &ReasonerConfig::default()),
            2.5
        );
    }

    #[test]
    fn short_weapon_window_does_not_invent_a_whole_magazine() {
        assert!((weapon_dps(&hero().weapon, 1.0) - 200.0 / 6.0).abs() < 1e-12);
    }

    #[test]
    fn model_conditions_cover_shot_melee_action_and_state() {
        let mut hero = hero();
        let cfg = ReasonerConfig::default();
        let mut item = item();
        item.condition = ConditionKind::ShotBound;
        assert!((condition_factor_for_hero(&item, &hero, &cfg) - 2.0 / 3.0).abs() < 1e-12);
        item.condition = ConditionKind::MeleeBound;
        assert_eq!(condition_factor_for_hero(&item, &hero, &cfg), 0.0);
        hero.archetype = "melee".into();
        assert_eq!(condition_factor_for_hero(&item, &hero, &cfg), 1.0);
        item.condition = ConditionKind::ActionBound {
            action: "dash-jump".into(),
        };
        assert_eq!(condition_factor_for_hero(&item, &hero, &cfg), 0.5);
        let mut ability = channel();
        ability.roles = vec![AbilityRole::Mobility];
        hero.abilities = vec![ability];
        assert_eq!(condition_factor_for_hero(&item, &hero, &cfg), 1.0);
        item.condition = ConditionKind::StateBound { threshold: 0.65 };
        assert!((condition_factor_for_hero(&item, &hero, &cfg) - 0.35).abs() < 1e-12);
    }

    #[test]
    fn imbue_gain_changes_target_when_channel_budget_is_exhausted() {
        let mut hero = hero();
        let mut capped = channel();
        capped.charges = 10;
        capped.scaling[0].per_spirit = Some(0.6225);
        capped.base_effect = 0.6225;
        let mut available = channel();
        available.ability_id = 2;
        available.channel_time = Some(1.0);
        available.scaling[0].per_spirit = Some(0.6225);
        available.base_effect = 0.6225;
        hero.abilities = vec![capped, available];
        let mut item = item();
        item.imbueable = true;
        let cfg = ReasonerConfig::default();
        assert_eq!(imbue_target(&item, &hero, &cfg), Some(1));
        item.properties.insert("CooldownReduction".into(), 50.0);
        assert_eq!(imbue_target(&item, &hero, &cfg), Some(2));
        assert!((imbue_gain(&item, &hero.abilities[1], &cfg) - 11.0 / 7.0).abs() < 1e-12);
    }

    #[test]
    fn buy_phase_moves_with_soul_curve_and_scaling_upgrade() {
        let mut hero = hero();
        hero.level_curve = (1..=14)
            .map(|level| crate::LevelPoint {
                level,
                required_souls: (level - 1) * 800,
            })
            .collect();
        let mut ability = channel();
        ability.scaling_step = Some(crate::ScalingStep {
            upgrade_index: 0,
            stat: "LifeDrain".into(),
            from: 0.3225,
            to: 0.6225,
        });
        hero.abilities = vec![ability];
        let mut item = item();
        item.cost = 3200;
        assert_eq!(scaling_souls(&hero), Some(800));
        assert_eq!(buy_phase_for_hero(&item, &hero), crate::BuyPhase::Core);
        hero.abilities[0]
            .scaling_step
            .as_mut()
            .unwrap()
            .upgrade_index = 2;
        assert_eq!(scaling_souls(&hero), Some(8800));
        assert_eq!(buy_phase_for_hero(&item, &hero), crate::BuyPhase::Lane);
        item.defense_kind.push("bullet_resist".into());
        assert_eq!(buy_phase_for_hero(&item, &hero), crate::BuyPhase::Late);
    }

    #[test]
    fn overlapping_passive_properties_are_counted_once() {
        let mut item = item();
        item.properties.insert("PassiveHealth".into(), 100.0);
        item.passive_properties = item.properties.clone();
        assert_eq!(
            combat_window_value(&item, &hero(), &ReasonerConfig::default()),
            2.5
        );
    }

    #[test]
    fn tick_proc_uses_item_tick_rate_and_limits_stacks() {
        let mut item = item();
        item.properties = [("TickRate".into(), 0.1), ("BoloProcDamage".into(), 10.0)]
            .into_iter()
            .collect();
        item.proc_cooldown = Some(0.7);
        let cfg = ReasonerConfig {
            combat_window_seconds: 7.0,
            ..ReasonerConfig::default()
        };
        assert!((combat_window_value(&item, &hero(), &cfg) - 100.0 / 7.0).abs() < 1e-12);
        item.properties.insert("MaxStacks".into(), 5.0);
        assert!((combat_window_value(&item, &hero(), &cfg) - 50.0 / 7.0).abs() < 1e-12);
    }

    #[test]
    fn opponent_debuffs_and_self_cost_have_opposite_signs() {
        let hero = hero();
        let cfg = ReasonerConfig::default();
        assert_eq!(
            property_value("WeaponPowerDebuff", -25.0, &hero, &cfg),
            10.0
        );
        assert_eq!(
            property_value("HealthDrainedPerSecond", 50.0, &hero, &cfg),
            -50.0
        );
    }

    #[test]
    fn instant_reload_counts_saved_reload_time_instead_of_full_weapon_dps() {
        let mut item = item();
        item.properties = [
            ("AmmoReloadPercent".into(), 100.0),
            ("AbilityCooldown".into(), 30.0),
        ]
        .into_iter()
        .collect();
        let cfg = ReasonerConfig {
            combat_window_seconds: 40.0,
            ..ReasonerConfig::default()
        };
        assert!(
            (combat_window_value(&item, &hero(), &cfg) - 50.0 * 2.0 * (1.0 + 40.0 / 30.0) / 40.0)
                .abs()
                < 1e-12
        );
    }

    #[test]
    fn ability_tick_rate_reaches_item_proc_scoring() {
        let mut item = item();
        item.properties.insert("BoloProcDamage".into(), 10.0);
        item.proc_cooldown = Some(0.7);
        let mut hero = hero();
        let mut ability = channel();
        ability.base_effect = 70.0;
        ability.tick_rate = Some(0.1);
        ability.duration = Some(7.0);
        hero.abilities = vec![ability];
        let cfg = ReasonerConfig {
            combat_window_seconds: 7.0,
            channel_uptime: 1.0,
            ..ReasonerConfig::default()
        };
        assert!((combat_window_value(&item, &hero, &cfg) - 100.0 / 7.0).abs() < 1e-12);
        item.proc_cooldown = Some(1.4);
        assert!((combat_window_value(&item, &hero, &cfg) - 50.0 / 7.0).abs() < 1e-12);
    }

    #[test]
    fn max_health_steal_counts_target_loss_and_own_gain() {
        let mut item = item();
        item.condition = ConditionKind::ShotBound;
        item.properties.insert("HealthStealPctHero".into(), 2.5);
        item.proc_cooldown = Some(0.7);
        let mut hero = hero();
        hero.weapon.shots_per_second = 10.0;
        hero.weapon.reload_duration = 0.0;
        let cfg = ReasonerConfig {
            combat_window_seconds: 7.0,
            ..ReasonerConfig::default()
        };
        let stolen = 600.0 * 0.025 * 10.0;
        assert!((combat_window_value(&item, &hero, &cfg) - (stolen + stolen) / 7.0).abs() < 1e-12);
    }

    #[test]
    fn base_damage_dps_is_independent_of_previous_damage_plan() {
        let mut hero = hero();
        let mut ability = channel();
        ability.base_effect = 60.0;
        let cfg = ReasonerConfig::default();
        let before = ability_dps(&ability, &hero, &cfg);
        hero.damage_plan.weapon_share = 0.1;
        assert_eq!(ability_dps(&ability, &hero, &cfg), before);
        assert!((before - 3.5).abs() < 1e-12);
    }

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
                base_effect: 0.0,
                tick_rate: None,
                duration: None,
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
                base_effect: 0.0,
                tick_rate: None,
                duration: None,
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
