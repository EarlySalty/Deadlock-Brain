use crate::{AbilityModel, ConditionKind, HeroModel, ItemModel, ReasonerConfig, SlotType};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct CombatScenarioEvaluation {
    pub name: String,
    pub weapon_damage: f64,
    pub ability_damage: f64,
    pub proc_damage: f64,
    pub effective_health: f64,
    pub utility: f64,
    pub shots: f64,
    pub reloads: usize,
    pub casts: BTreeMap<i64, usize>,
    pub channel_seconds: f64,
    pub spirit_power: f64,
    pub sequence: Vec<String>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct InventoryEvaluation {
    pub shop_bonuses: BTreeMap<String, f64>,
    pub score: f64,
    pub weapon_damage: f64,
    pub ability_damage: f64,
    pub proc_damage: f64,
    pub effective_health: f64,
    pub utility: f64,
    pub scenarios: Vec<CombatScenarioEvaluation>,
    pub assumptions: Vec<String>,
    pub unknown_effects: Vec<String>,
}
#[derive(Default)]
struct Stats {
    spirit: f64,
    weapon: f64,
    rate: f64,
    clip: f64,
    flat_clip: f64,
    reload: f64,
    health: f64,
    health_pct: f64,
    bullet_resist: f64,
    spirit_resist: f64,
    bullet_shred: f64,
    spirit_shred: f64,
    bullet_leech: f64,
    spirit_leech: f64,
    regeneration: f64,
    shield: f64,
    cooldown: f64,
    duration: f64,
    slow: f64,
    speed: f64,
    control: f64,
}
fn value(item: &ItemModel, name: &str) -> f64 {
    item.properties
        .get(name)
        .or_else(|| item.passive_properties.get(name))
        .copied()
        .filter(|v| v.is_finite())
        .unwrap_or_default()
}
fn conditional(item: &ItemModel, name: &str) -> bool {
    !name.starts_with("Passive")
        && (item.conditional_properties.contains(name)
            || (item.is_active != item.passive_properties.contains_key(name))
            || name.contains("ProcDamage")
            || name.contains("Reduction") && name.contains("Armor"))
}
fn apply(stats: &mut Stats, name: &str, v: f64) -> bool {
    match name {
        "TechPower" | "SpiritPower" | "BonusSpirit" | "BonusSpiritPower" | "SpiritPowerInnate" => stats.spirit += v,
        "WeaponPower"
        | "WeaponDamage"
        | "BaseAttackDamagePercent"
        | "BaseAttackDamagePercentBonus" => stats.weapon += v,
        "BonusFireRate" | "FireRate" | "ActivatedFireRate" => stats.rate += v,
        "BonusClipSizePercent" | "ClipSizePercent" => stats.clip += v,
        "BonusClipSize" => stats.flat_clip += v,
        "ReloadSpeed" | "ReloadSpeedPercent" | "ReloadSpeedBonus" | "BonusReloadSpeed" => stats.reload += v,
        "BonusHealth" | "PassiveHealth" | "Health" => stats.health += v,
        "BonusHealthPercent" | "MaxHealthPercent" => stats.health_pct += v,
        "BulletArmor" | "BulletResist" | "BulletResistPercent" => {
            stats.bullet_resist = 1.0 - (1.0 - stats.bullet_resist) * (1.0 - v / 100.0)
        }
        "TechArmor" | "TechResist" | "SpiritResist" | "SpiritResistPercent" => {
            stats.spirit_resist = 1.0 - (1.0 - stats.spirit_resist) * (1.0 - v / 100.0)
        }
        "BulletArmorReduction" | "BulletResistReduction" => stats.bullet_shred += v.abs(),
        "TechArmorReduction" | "SpiritResistReduction" => stats.spirit_shred += v.abs(),
        "BulletLifestealPercent" => stats.bullet_leech += v,
        "AbilityLifestealPercent" | "AbilityLifestealPercentHero" => stats.spirit_leech += v,
        "HealthRegen" | "HealthRegenBonus" => stats.regeneration += v,
        "BulletShieldMaxHealth" | "TechShieldMaxHealth" | "CombatBarrier" => stats.shield += v,
        "CooldownReduction" => stats.cooldown = 1.0 - (1.0 - stats.cooldown) * (1.0 - v / 100.0),
        "AbilityDurationPercent" | "TechDuration" | "BonusAbilityDurationPercent" => stats.duration += v,
        "SlowPercent" | "MovementSlow" | "MovementSpeedSlow" => {
            stats.slow = stats.slow.max(v.abs())
        }
        "BonusMoveSpeed" | "MoveSpeed" => stats.speed += v,
        "StunDuration" | "RootDuration" | "SilenceDuration" => stats.control = stats.control.max(v),
        _ => return false,
    }
    true
}
fn damage_scale(ability: &AbilityModel, spirit: f64) -> f64 {
    ability
        .scaling
        .iter()
        .filter_map(|s| s.per_spirit.map(|scale| (s.stat.as_str(), scale)))
        .map(|(name, scale)| {
            let duration = ability.duration.or(ability.channel_time).unwrap_or(1.0);
            match name {
                "Damage" | "ImpactDamage" | "BaseDamage" | "Spirit" => scale * spirit,
                "PulseDPS" | "DamagePerSecond" | "DPS" => scale * spirit * duration,
                "TickDamage" | "DamagePerTick" => {
                    scale * spirit * duration / ability.tick_rate.unwrap_or(1.0).max(0.01)
                }
                _ => 0.0,
            }
        })
        .sum()
}
fn active(
    item: &ItemModel,
    time: f64,
    health_fraction: f64,
    fired: bool,
    last_cast: f64,
    spirit_damage: f64,
) -> bool {
    match &item.condition {
        ConditionKind::None => !item.is_active,
        ConditionKind::StateBound { threshold } => {
            let text = item.description.to_ascii_lowercase();
            if text.contains("above") || text.contains("over ") {
                health_fraction > *threshold
            } else if text.contains("below") || text.contains("under ") {
                health_fraction < *threshold
            } else {
                false
            }
        }
        ConditionKind::RampUp { ramp_seconds } => time >= *ramp_seconds && fired,
        ConditionKind::ShotBound => fired,
        ConditionKind::MeleeBound => false,
        ConditionKind::ActionBound { action } => match action.as_str() {
            "shot" | "shoot" => fired,
            "activation" | "cast" => time - last_cast <= value(item, "AbilityDuration").max(0.2),
            "spirit_damage_threshold" => {
                spirit_damage >= value(item, "DamageThreshold")
                    && value(item, "DamageThreshold") > 0.0
            }
            _ => false,
        },
        ConditionKind::ActiveCooldown { uptime, cooldown } => {
            let duration = value(item, "AbilityDuration").max(*uptime * *cooldown);
            *cooldown > 0.0 && time % *cooldown < duration
        }
    }
}

pub fn evaluate_inventory(
    hero: &HeroModel,
    items: &[ItemModel],
    cfg: &ReasonerConfig,
) -> InventoryEvaluation {
    let mut seen = BTreeSet::new();
    let held: Vec<_> = items.iter().filter(|i| seen.insert(i.item_id)).collect();
    let mut unknown = BTreeSet::new();
    for item in &held {
        let mut unique = BTreeSet::new();
        for (name, v) in item.properties.iter().chain(item.passive_properties.iter()) {
            if !unique.insert(name) || *v == 0.0 {
                continue;
            }
            if !v.is_finite() {
                unknown.insert(format!("{}: ungültiger Wert für {name}", item.name));
                continue;
            }
            if !apply(&mut Stats::default(), name, *v)
                && !metadata(name)
                && !name.contains("ProcDamage")
                && name != "AmmoReloadPercent"
            {
                unknown.insert(format!("{}: {name} nicht quantifiziert", item.name));
            }
        }
        if matches!(
            item.condition,
            ConditionKind::ActionBound { .. }
                | ConditionKind::MeleeBound
                | ConditionKind::StateBound { .. }
        ) {
            unknown.insert(format!(
                "{}: Auslöser {:?} nur bei belegtem Ereignis; Nahkampf/Parade nicht simuliert",
                item.name, item.condition
            ));
        }
    }
    let mut result = InventoryEvaluation { shop_bonuses: shop_bonuses(hero, &held), assumptions: vec![
        "Begrenzter deterministischer Einzelzielvergleich, keine vollständige Spielsimulation oder Gewinnwahrscheinlichkeit.".into(),
        "Drei gleich gewichtete Szenarien: gesundes Duell, steigender Lebensdruck, bewegliches Ziel. Gleiche Annahmen für alle Helden.".into(),
        "Alle Fähigkeiten und aktiven Items sind anfangs bereit; Basis-Fähigkeitsstufe, keine erfundenen Skill-Upgrades. Kanalisieren und Schießen teilen die verfügbare Kampfzeit.".into(),
        "Gegner startet ohne Resistenzen; eingehender Schaden ist zur Hälfte Waffen- und Spirit-Schaden. Lifesteal zählt höchstens den angenommenen Lebensverlust.".into(),
        "Utility-Modellannahme: Verlangsamung und Lauftempo helfen beim Zielkontakt, bei beweglichem Ziel stärker. Überlappende Verlangsamungen nutzen nur den stärksten Wert.".into(),
        "Shopboni nach Wert der aktuell gehaltenen Items je Kategorie; Verkäufe entfernen deren Bonusanteil. Fehlende Schwellen ergeben keinen erfundenen Bonus.".into(),
        "Fähigkeits- und Itemladungen, Imbue-spezifische Zusatzskalierung und komplexe Gegnerreaktionen sind noch nicht vollständig modelliert.".into(),
        "Zeitschritt 0,2 Sekunden; unbekannte Wirkungen werden nicht als garantierter Nutzen addiert.".into(),
    ], unknown_effects: unknown.into_iter().collect(), ..InventoryEvaluation::default() };
    for (name, pressure, moving) in [
        ("Duell", false, false),
        ("Unter Druck", true, false),
        ("Bewegliches Ziel", false, true),
    ] {
        let scenario = simulate(hero, &held, cfg, name, pressure, moving);
        result.weapon_damage += scenario.weapon_damage / 3.0;
        result.ability_damage += scenario.ability_damage / 3.0;
        result.proc_damage += scenario.proc_damage / 3.0;
        result.effective_health += scenario.effective_health / 3.0;
        result.utility += scenario.utility / 3.0;
        result.scenarios.push(scenario);
    }
    let window = cfg.combat_window_seconds.clamp(1.0, 120.0);
    result.score = (result.weapon_damage
        + result.ability_damage
        + result.proc_damage
        + result.effective_health)
        / window;
    result
}
fn metadata(name: &str) -> bool {
    matches!(
        name,
        "AbilityCastDelay"
            | "AbilityCastRange"
            | "AbilityChannelTime"
            | "AbilityCharges"
            | "AbilityCooldown"
            | "AbilityCooldownBetweenCharge"
            | "AbilityDuration"
            | "AbilityPostCastDuration"
            | "AbilityResourceCost"
            | "AbilityUnitTargetLimit"
            | "ChannelMoveSpeed"
            | "MaxStacks"
            | "TickRate"
            | "ProcCooldown"
            | "DamageThreshold"
            | "DamageThresholdDuration"
    )
}
fn simulate(
    hero: &HeroModel,
    items: &[&ItemModel],
    cfg: &ReasonerConfig,
    name: &str,
    pressure: bool,
    moving: bool,
) -> CombatScenarioEvaluation {
    let window = cfg.combat_window_seconds.clamp(1.0, 120.0);
    let dt = 0.2;
    let mut out = CombatScenarioEvaluation {
        name: name.into(),
        ..CombatScenarioEvaluation::default()
    };
    let mut ability_ready = vec![0.0; hero.abilities.len()];
    let mut proc_ready = vec![0.0; items.len()];
    let mut buff_ready = vec![0.0; items.len()];
    let mut instant_reload_ready = vec![0.0; items.len()];
    let mut ability_buffs_until = vec![0.0; hero.abilities.len()];
    let mut buff_until = vec![0.0; items.len()];
    let mut channel_until: f64 = 0.0;
    let mut reload_until: f64 = 0.0;
    let mut ammo: f64 = hero.weapon.clip_size.max(0.0);
    let mut initialized = false;
    let mut last_cast = f64::NEG_INFINITY;
    let mut fired = false;
    let mut spirit_events: Vec<(f64, f64)> = Vec::new();
    let mut pending_damage: Vec<(f64, f64)> = Vec::new();
    let mut health_sum = 0.0;
    let mut leech_sum = 0.0;
    let spirit_rate = hero
        .scaling
        .iter()
        .find(|s| s.stat == "ERoundsPerSecond")
        .and_then(|s| s.per_spirit)
        .or_else(|| {
            hero.scaling
                .iter()
                .find(|s| s.stat == "EFireRate")
                .and_then(|s| s.per_spirit)
                .map(|v| v * hero.weapon.shots_per_second / 100.0)
        })
        .unwrap_or(0.0);
    let steps = (window / dt).ceil() as usize;
    for step in 0..steps {
        let time = step as f64 * dt;
        let duration = dt.min(window - time);
        let health_fraction = if pressure {
            (1.0 - 0.85 * time / window).max(0.15)
        } else {
            1.0
        };
        let mut stats = Stats::default();
        for (idx, item) in items.iter().enumerate() {
            let threshold_window = value(item, "DamageThresholdDuration").max(dt);
            let recent_damage = spirit_events
                .iter()
                .filter(|(t, _)| time - *t <= threshold_window)
                .map(|(_, d)| *d)
                .sum();
            let enabled = active(item, time, health_fraction, fired, last_cast, recent_damage);
            let cooldown = value(item, "AbilityCooldown").max(item.proc_cooldown.unwrap_or(0.0));
            let buff_duration = value(item, "AbilityDuration");
            if enabled && time >= buff_ready[idx] && buff_duration > 0.0 && !item.is_active {
                buff_until[idx] = time + buff_duration;
                buff_ready[idx] = time + cooldown.max(dt);
            }
            let enabled = if cooldown > 0.0 && buff_duration > 0.0 && !item.is_active {
                time < buff_until[idx]
            } else {
                enabled
            };
            let mut keys = BTreeSet::new();
            for (key, v) in item.properties.iter().chain(item.passive_properties.iter()) {
                if !keys.insert(key) || !v.is_finite() || conditional(item, key) && !enabled {
                    continue;
                }
                apply(&mut stats, key, *v);
            }
        }
        for (idx, ability) in hero.abilities.iter().enumerate() {
            if time < ability_buffs_until[idx] {
                for (key, v) in &ability.properties {
                    apply(&mut stats, key, *v);
                }
            }
        }
        apply_shop(hero, items, &mut stats);
        let clip = (hero.weapon.clip_size * (1.0 + stats.clip / 100.0) + stats.flat_clip).max(1.0);
        if !initialized {
            ammo = clip;
            initialized = true;
        }
        let rate = ((hero.weapon.shots_per_second + stats.spirit * spirit_rate)
            * (1.0 + stats.rate / 100.0))
            .max(0.0);
        let bullet = hero.weapon.bullet_damage
            * (1.0 + stats.weapon / 100.0).max(0.0)
            * (1.0 + stats.bullet_shred / 100.0);
        let contact = if moving { 0.65 } else { 1.0 };
        let utility_contact =
            ((stats.slow / 100.0 * 0.5 + stats.speed.max(0.0) / 20.0).min(1.0 - contact)).max(0.0);
        let hit = contact + utility_contact;
        let weapon_opportunity = bullet * rate * hit;
        if time >= channel_until {
            let mut best = None;
            for (idx, ability) in hero.abilities.iter().enumerate() {
                if time < ability_ready[idx] || ability.ability_id <= 0 {
                    continue;
                }
                let damage = (ability.base_effect + damage_scale(ability, stats.spirit)).max(0.0);
                let cast_time = ability.channel_time.unwrap_or(0.0).max(0.0)
                    + ability
                        .properties
                        .get("AbilityCastDelay")
                        .copied()
                        .unwrap_or(0.0)
                        .max(0.0);
                let complete = if cast_time > 0.0 {
                    (window - time).min(cast_time) / cast_time
                } else {
                    1.0
                };
                let utility = ability
                    .properties
                    .iter()
                    .filter(|(key, _)| {
                        matches!(
                            key.as_str(),
                            "BonusFireRate"
                                | "BonusMoveSpeed"
                                | "BulletShieldMaxHealth"
                                | "TechShieldMaxHealth"
                                | "CombatBarrier"
                                | "SlowPercent"
                                | "RootDuration"
                                | "StunDuration"
                        )
                    })
                    .map(|(_, v)| v.max(0.0))
                    .sum::<f64>();
                let gain = damage * complete * (1.0 + stats.spirit_shred / 100.0) + utility
                    - weapon_opportunity * cast_time;
                if gain > 0.0 && best.as_ref().is_none_or(|(_, g, _, _)| gain > *g) {
                    best = Some((idx, gain, damage * complete, cast_time));
                }
            }
            if let Some((idx, _, damage, cast_time)) = best {
                let ability = &hero.abilities[idx];
                let effect_time = if ability.channel_time.is_some()
                    || ability.tick_rate.is_some()
                    || ability
                        .scaling
                        .iter()
                        .any(|s| s.stat.ends_with("DPS") || s.stat.ends_with("PerSecond"))
                {
                    ability
                        .duration
                        .or(ability.channel_time)
                        .unwrap_or(0.0)
                        .min(window - time)
                } else {
                    0.0
                };
                if effect_time > 0.0 {
                    pending_damage.push((time + effect_time, damage / effect_time));
                } else {
                    let dealt = damage * (1.0 + stats.spirit_shred / 100.0) * hit;
                    out.ability_damage += dealt;
                    leech_sum += dealt * stats.spirit_leech.max(0.0) / 100.0;
                    spirit_events.push((time, dealt));
                }
                *out.casts.entry(ability.ability_id).or_default() += 1;
                ability_ready[idx] =
                    time + ability.cooldown.max(window) * (1.0 - stats.cooldown.clamp(0.0, 0.8));
                if ability.cooldown > 0.0 {
                    ability_ready[idx] =
                        time + ability.cooldown * (1.0 - stats.cooldown.clamp(0.0, 0.8));
                }
                channel_until = time + cast_time;
                last_cast = time;
                ability_buffs_until[idx] =
                    time + ability.duration.unwrap_or(0.0) * (1.0 + stats.duration / 100.0);
                if out.sequence.len() < 16 {
                    out.sequence
                        .push(format!("{time:.1}s: {}", ability.class_name));
                }
            }
        }
        for (idx, item) in items.iter().enumerate() {
            let reload_percent = value(item, "AmmoReloadPercent");
            let cooldown = value(item, "AbilityCooldown").max(item.proc_cooldown.unwrap_or(0.0));
            if reload_percent > 0.0
                && cooldown > 0.0
                && ammo < clip * 0.1
                && time >= instant_reload_ready[idx]
                && item.is_active
            {
                ammo = (ammo + clip * reload_percent / 100.0).min(clip);
                reload_until = time;
                instant_reload_ready[idx] = time + cooldown;
                if out.sequence.len() < 16 {
                    out.sequence
                        .push(format!("{time:.1}s: {} füllt Magazin", item.name));
                }
            }
        }
        let tick_damage = pending_damage
            .iter()
            .map(|(end, dps)| (end - time).clamp(0.0, duration) * dps)
            .sum::<f64>()
            * (1.0 + stats.spirit_shred / 100.0)
            * hit;
        if tick_damage > 0.0 {
            out.ability_damage += tick_damage;
            spirit_events.push((time, tick_damage));
            leech_sum += tick_damage * stats.spirit_leech.max(0.0) / 100.0;
        }
        pending_damage.retain(|(end, _)| *end > time + duration);
        let mut shots = 0.0;
        if time < channel_until {
            out.channel_seconds += duration;
        } else if time >= reload_until && rate > 0.0 {
            if ammo <= 0.0001 {
                reload_until = time
                    + hero.weapon.reload_duration.max(0.0) / (1.0 + stats.reload / 100.0).max(0.1);
                ammo = clip;
                out.reloads += 1;
                if out.sequence.len() < 16 {
                    out.sequence.push(format!("{time:.1}s: Nachladen"));
                }
            } else {
                shots = (rate * duration).min(ammo);
                ammo -= shots;
            }
        }
        fired = shots > 0.0;
        let gun_damage = shots * bullet * hit;
        out.shots += shots;
        out.weapon_damage += gun_damage;
        out.utility += shots * bullet * utility_contact;
        for (idx, item) in items.iter().enumerate() {
            if time < proc_ready[idx] {
                continue;
            }
            let trigger = match item.condition {
                ConditionKind::ShotBound => fired,
                ConditionKind::None => time == last_cast,
                _ => false,
            };
            if !trigger {
                continue;
            }
            let damage = item
                .properties
                .iter()
                .filter(|(key, _)| key.contains("ProcDamage"))
                .map(|(_, v)| *v)
                .sum::<f64>();
            if damage > 0.0 {
                if let Some(cooldown) = item.proc_cooldown.filter(|v| *v > 0.0) {
                    out.proc_damage += damage * (1.0 + stats.spirit_shred / 100.0) * hit;
                    proc_ready[idx] = time + cooldown;
                }
            }
        }
        let health = (hero.base_health * (1.0 + stats.health_pct / 100.0) + stats.health).max(1.0);
        let mitigation = 0.5 * (1.0 - stats.bullet_resist.clamp(-1.0, 0.9))
            + 0.5 * (1.0 - stats.spirit_resist.clamp(-1.0, 0.9));
        health_sum += (health + stats.shield.max(0.0)) / mitigation * duration / window;
        leech_sum += gun_damage * stats.bullet_leech.max(0.0) / 100.0
            + stats.regeneration.max(0.0) * duration;
        out.spirit_power += stats.spirit * duration / window;
    }
    out.effective_health = health_sum
        + leech_sum.min(if pressure {
            hero.base_health * 0.85
        } else {
            hero.base_health * 0.3
        });
    out
}
pub fn shop_bonuses(hero: &HeroModel, items: &[&ItemModel]) -> BTreeMap<String, f64> {
    [
        ("weapon", SlotType::Weapon),
        ("spirit", SlotType::Spirit),
        ("vitality", SlotType::Vitality),
    ]
    .into_iter()
    .map(|(name, slot)| {
        let spent = items
            .iter()
            .filter(|item| item.slot == slot)
            .map(|item| item.cost)
            .sum::<i64>();
        let bonus = hero
            .cost_bonuses
            .get(name)
            .into_iter()
            .flatten()
            .filter(|b| b.gold_threshold <= spent && b.bonus.is_finite())
            .max_by_key(|b| b.gold_threshold)
            .map_or(0.0, |b| b.bonus);
        (name.into(), bonus)
    })
    .collect()
}
fn apply_shop(hero: &HeroModel, items: &[&ItemModel], stats: &mut Stats) {
    let bonuses = shop_bonuses(hero, items);
    stats.weapon += bonuses["weapon"];
    stats.spirit += bonuses["spirit"];
    stats.health_pct += bonuses["vitality"];
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AbilityRole, CostBonus, DamagePlan, DamageType, PurchaseBonuses, WeaponProfile};
    pub(crate) fn hero() -> HeroModel {
        HeroModel {
            hero_id: 1,
            name: "Testheld".into(),
            cost_bonuses: BTreeMap::new(),
            archetype: String::new(),
            base_health: 600.0,
            level_curve: vec![],
            purchase_bonuses: PurchaseBonuses {
                weapon: vec![],
                spirit: vec![],
                vitality: vec![],
            },
            scaling: vec![],
            weapon: WeaponProfile {
                bullet_damage: 10.0,
                shots_per_second: 5.0,
                clip_size: 20.0,
                reload_duration: 2.0,
                range: 20.0,
                falloff_start_range: 20.0,
                falloff_end_range: 40.0,
                sustained_dps: 0.0,
            },
            abilities: vec![],
            damage_plan: DamagePlan {
                weapon_dps: 0.0,
                spirit_dps: 0.0,
                weapon_share: 1.0,
                primary_axis: DamageType::Weapon,
            },
        }
    }
    pub(crate) fn item(id: i64, key: &str, v: f64) -> ItemModel {
        ItemModel {
            item_id: id,
            name: format!("Item {id}"),
            class_name: format!("item_{id}"),
            component_items: vec![],
            description: String::new(),
            slot: SlotType::Weapon,
            tier: 1,
            cost: 800,
            is_active: false,
            shopable: true,
            disabled: false,
            damage_axis: DamageType::Weapon,
            defense_kind: vec![],
            properties: BTreeMap::from([(key.into(), v)]),
            passive_properties: BTreeMap::new(),
            conditional_properties: BTreeSet::new(),
            condition: ConditionKind::None,
            proc_cooldown: None,
            imbueable: false,
        }
    }
    #[test]
    fn damage_and_fire_rate_are_joint_not_additive() {
        let mut hero = hero();
        hero.weapon.reload_duration = 0.0;
        hero.weapon.clip_size = 10000.0;
        let cfg = ReasonerConfig::default();
        let damage = item(1, "WeaponPower", 100.0);
        let rate = item(2, "BonusFireRate", 100.0);
        let base = evaluate_inventory(&hero, &[], &cfg).weapon_damage;
        let one = evaluate_inventory(&hero, &[damage.clone()], &cfg).weapon_damage;
        let two = evaluate_inventory(&hero, &[rate.clone()], &cfg).weapon_damage;
        let both = evaluate_inventory(&hero, &[damage, rate], &cfg).weapon_damage;
        assert!(both - base > (one - base) + (two - base));
    }
    #[test]
    fn shop_uses_cumulative_threshold_not_sum_of_item_tiers() {
        let mut hero = hero();
        hero.cost_bonuses.insert(
            "weapon".into(),
            vec![
                CostBonus {
                    gold_threshold: 800,
                    bonus: 9.0,
                },
                CostBonus {
                    gold_threshold: 1600,
                    bonus: 12.0,
                },
            ],
        );
        let a = item(1, "Unknown", 0.0);
        let b = item(2, "Unknown", 0.0);
        let evaluation = evaluate_inventory(&hero, &[a.clone(), b], &ReasonerConfig::default());
        assert_eq!(evaluation.shop_bonuses["weapon"], 12.0);
        assert_eq!(
            evaluate_inventory(&hero, &[a.clone(), a], &ReasonerConfig::default()).shop_bonuses
                ["weapon"],
            9.0
        );
    }
    #[test]
    fn spirit_changes_whole_weapon_rate_and_magazine_value() {
        let mut hero = hero();
        hero.scaling.push(crate::ScalingStat {
            stat: "ERoundsPerSecond".into(),
            per_level: 0.0,
            per_spirit: Some(0.1),
        });
        let cfg = ReasonerConfig::default();
        let spirit = item(1, "TechPower", 100.0);
        let magazine = item(2, "BonusClipSizePercent", 100.0);
        let plain = evaluate_inventory(&hero, &[], &cfg);
        let with_spirit = evaluate_inventory(&hero, &[spirit.clone()], &cfg);
        assert!(with_spirit.weapon_damage > plain.weapon_damage);
        let combined = evaluate_inventory(&hero, &[spirit, magazine], &cfg);
        assert!(combined.weapon_damage > with_spirit.weapon_damage);
        assert!(combined.scenarios[0].reloads < with_spirit.scenarios[0].reloads);
    }
    #[test]
    fn above_and_below_health_conditions_have_opposite_windows() {
        let mut below = item(1, "BonusFireRate", 100.0);
        below.condition = ConditionKind::StateBound { threshold: 0.65 };
        below.description = "While below 65% health".into();
        let mut above = below.clone();
        above.description = "While above 65% health".into();
        assert!(!active(&below, 0.0, 1.0, true, 0.0, 0.0));
        assert!(active(&above, 0.0, 1.0, true, 0.0, 0.0));
        assert!(active(&below, 0.0, 0.3, true, 0.0, 0.0));
        assert!(!active(&above, 0.0, 0.3, true, 0.0, 0.0));
        below.description.clear();
        assert!(!active(&below, 0.0, 0.3, true, 0.0, 0.0));
    }
    #[test]
    fn channel_consumes_shared_time_and_damage_is_window_bounded() {
        let mut hero = hero();
        hero.weapon.clip_size = 1000.0;
        hero.abilities.push(AbilityModel {
            ability_id: 2,
            properties: BTreeMap::new(),
            class_name: "channel".into(),
            slot: 1,
            roles: vec![AbilityRole::Damage],
            scaling: vec![],
            channel_time: Some(4.0),
            charges: 1,
            cooldown: 30.0,
            scaling_step: None,
            damage_type: DamageType::Spirit,
            base_effect: 1000.0,
            tick_rate: Some(0.2),
            duration: Some(4.0),
        });
        let cfg = ReasonerConfig {
            combat_window_seconds: 8.0,
            ..ReasonerConfig::default()
        };
        let evaluation = evaluate_inventory(&hero, &[], &cfg);
        let duel = &evaluation.scenarios[0];
        assert!((duel.channel_seconds - 4.0).abs() < 0.01);
        assert_eq!(duel.casts[&2], 1);
        assert!(duel.shots <= 20.1);
        assert!((duel.ability_damage - 1000.0).abs() < 0.01);
        let short = evaluate_inventory(
            &hero,
            &[],
            &ReasonerConfig {
                combat_window_seconds: 2.0,
                ..cfg
            },
        );
        assert!((short.scenarios[0].ability_damage - 500.0).abs() < 0.01);
    }
    #[test]
    fn proc_cooldown_caps_fast_weapon_and_unknowns_remain_visible() {
        let mut hero = hero();
        hero.weapon.shots_per_second = 100.0;
        hero.weapon.clip_size = 100000.0;
        let mut proc = item(1, "BoloProcDamage", 100.0);
        proc.condition = ConditionKind::ShotBound;
        proc.proc_cooldown = Some(2.0);
        proc.properties.insert("MysteryEffect".into(), 99.0);
        let result = evaluate_inventory(
            &hero,
            &[proc],
            &ReasonerConfig {
                combat_window_seconds: 4.0,
                ..ReasonerConfig::default()
            },
        );
        assert!(result.scenarios[0].proc_damage <= 200.01);
        assert!(result
            .unknown_effects
            .iter()
            .any(|s| s.contains("MysteryEffect")));
    }
}
