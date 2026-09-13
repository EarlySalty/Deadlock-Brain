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
    pub ability_procs: BTreeMap<i64, usize>,
    pub channel_seconds: f64,
    pub spirit_power: f64,
    pub sequence: Vec<String>,
    pub item_activations: BTreeMap<i64, Vec<f64>>,
    pub imbue_targets: BTreeMap<i64, i64>,
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
#[derive(Default, Clone)]
struct Stats {
    spirit: f64,
    weapon: f64,
    rate: f64,
    clip: f64,
    flat_clip: f64,
    reload: f64,
    health: f64,
    health_pct: f64,
    health_loss: f64,
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
    enemy_weapon_penalty: f64,
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
            || name.starts_with("ActiveBonus")
            || name.starts_with("Fervor")
            || (item.is_active && !item.passive_properties.contains_key(name))
            || name.contains("ProcDamage")
            || name.contains("Reduction") && name.contains("Armor"))
}
fn apply(stats: &mut Stats, name: &str, v: f64) -> bool {
    match name {
        "TechPower" | "SpiritPower" | "BonusSpirit" | "BonusSpiritPower" | "SpiritPowerInnate" => {
            stats.spirit += v
        }
        "WeaponPower"
        | "WeaponDamage"
        | "BaseAttackDamagePercent"
        | "BaseAttackDamagePercentBonus" => stats.weapon += v,
        "BonusFireRate"
        | "FireRate"
        | "ActivatedFireRate"
        | "ActiveBonusFireRate"
        | "FervorFireRate" => stats.rate += v,
        "BonusClipSizePercent" | "ClipSizePercent" => stats.clip += v,
        "BonusClipSize" => stats.flat_clip += v,
        "ReloadSpeed" | "ReloadSpeedPercent" | "ReloadSpeedBonus" | "BonusReloadSpeed" => {
            stats.reload += v
        }
        "BonusHealth" | "PassiveHealth" | "Health" => stats.health += v,
        "BonusHealthPercent" | "MaxHealthPercent" => stats.health_pct += v,
        "MaxHealthLossPercent" => {
            stats.health_loss = 1.0 - (1.0 - stats.health_loss) * (1.0 - v.abs() / 100.0)
        }
        "BulletArmor" | "BulletResist" | "BulletResistPercent" => {
            stats.bullet_resist = 1.0 - (1.0 - stats.bullet_resist) * (1.0 - v / 100.0)
        }
        "TechArmor" | "TechResist" | "SpiritResist" | "SpiritResistPercent" => {
            stats.spirit_resist = 1.0 - (1.0 - stats.spirit_resist) * (1.0 - v / 100.0)
        }
        "BulletArmorReduction" | "BulletResistReduction" => stats.bullet_shred += v.abs(),
        "TechArmorReduction" | "SpiritResistReduction" => stats.spirit_shred += v.abs(),
        "BulletLifestealPercent" | "ActiveBonusLifesteal" => stats.bullet_leech += v,
        "AbilityLifestealPercent" | "AbilityLifestealPercentHero" => stats.spirit_leech += v,
        "HealthRegen" | "HealthRegenBonus" => stats.regeneration += v,
        "BulletShieldMaxHealth" | "TechShieldMaxHealth" | "CombatBarrier" => stats.shield += v,
        "CooldownReduction" => stats.cooldown = 1.0 - (1.0 - stats.cooldown) * (1.0 - v / 100.0),
        "AbilityDurationPercent" | "TechDuration" | "BonusAbilityDurationPercent" => {
            stats.duration += v / 100.0
        }
        "SlowPercent" | "MovementSlow" | "MovementSpeedSlow" | "MoveSpeedSlowPct" => {
            stats.slow = stats.slow.max(v.abs())
        }
        "BonusMoveSpeed" | "MoveSpeed" | "ActiveBonusMoveSpeed" | "FervorMovespeed" => {
            stats.speed += v
        }
        "StunDuration" | "RootDuration" | "ImmobilizeDuration" => {
            stats.control = stats.control.max(v)
        }
        "WeaponPowerDebuff" => {
            stats.enemy_weapon_penalty = stats.enemy_weapon_penalty.max(v.abs() / 100.0)
        }
        _ => return false,
    }
    true
}
struct PreparedAbility<'a> {
    damage_terms: Vec<(f64, f64)>,
    utility_keys: Vec<&'a str>,
    periodic: f64,
    duration_scales: bool,
}
impl<'a> PreparedAbility<'a> {
    fn new(ability: &'a AbilityModel) -> Self {
        let periodic = periodic_duration(ability);
        Self {
            damage_terms: ability
                .scaling
                .iter()
                .filter_map(|stat| {
                    stat.per_spirit.map(|scale| {
                        (
                            scale,
                            if stat.stat == "Spirit" {
                                1.0
                            } else {
                                crate::ability_damage_units(ability, &stat.stat)
                            },
                        )
                    })
                })
                .collect(),
            utility_keys: ability
                .properties
                .iter()
                .filter_map(|(key, value)| {
                    apply(&mut Stats::default(), key, *value).then_some(key.as_str())
                })
                .collect(),
            periodic,
            duration_scales: scaled_periodic_duration(
                ability,
                &Stats {
                    duration: 1.0,
                    ..Stats::default()
                },
            ) != periodic,
        }
    }
    fn damage_scale(&self, spirit: f64) -> f64 {
        self.damage_terms
            .iter()
            .map(|(scale, units)| scale * spirit * units)
            .sum()
    }
    fn periodic_duration(&self, duration: f64) -> f64 {
        if self.duration_scales {
            self.periodic * (1.0 + duration)
        } else {
            self.periodic
        }
    }
}
fn active_with_text(
    item: &ItemModel,
    time: f64,
    health_fraction: f64,
    fired: bool,
    last_cast: f64,
    spirit_damage: f64,
    text: &str,
) -> bool {
    let low = value(item, "LowHealthThreshold");
    if low > 0.0 {
        return health_fraction < low / 100.0;
    }
    match &item.condition {
        ConditionKind::None => !item.is_active,
        ConditionKind::StateBound { threshold } => {
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
#[cfg(test)]
fn active(
    item: &ItemModel,
    time: f64,
    health_fraction: f64,
    fired: bool,
    last_cast: f64,
    spirit_damage: f64,
) -> bool {
    active_with_text(
        item,
        time,
        health_fraction,
        fired,
        last_cast,
        spirit_damage,
        &item.description.to_ascii_lowercase(),
    )
}

pub fn evaluate_inventory(
    hero: &HeroModel,
    items: &[ItemModel],
    cfg: &ReasonerConfig,
) -> InventoryEvaluation {
    evaluate_core(hero, &items.iter().collect::<Vec<_>>(), cfg, true, None)
}
pub fn evaluate_inventory_fast(
    hero: &HeroModel,
    items: &[ItemModel],
    cfg: &ReasonerConfig,
) -> InventoryEvaluation {
    evaluate_core(hero, &items.iter().collect::<Vec<_>>(), cfg, false, None)
}
pub fn evaluate_inventory_refs_fast(
    hero: &HeroModel,
    items: &[&ItemModel],
    cfg: &ReasonerConfig,
) -> InventoryEvaluation {
    evaluate_core(hero, items, cfg, false, None)
}
pub fn evaluate_inventory_with_bindings(
    hero: &HeroModel,
    items: &[ItemModel],
    cfg: &ReasonerConfig,
    bindings: &BTreeMap<i64, i64>,
) -> InventoryEvaluation {
    evaluate_core(
        hero,
        &items.iter().collect::<Vec<_>>(),
        cfg,
        true,
        Some(bindings),
    )
}
pub fn evaluate_inventory_refs_fast_with_bindings(
    hero: &HeroModel,
    items: &[&ItemModel],
    cfg: &ReasonerConfig,
    bindings: &BTreeMap<i64, i64>,
) -> InventoryEvaluation {
    evaluate_core(hero, items, cfg, false, Some(bindings))
}
fn evaluate_core(
    hero: &HeroModel,
    items: &[&ItemModel],
    cfg: &ReasonerConfig,
    detailed: bool,
    bindings: Option<&BTreeMap<i64, i64>>,
) -> InventoryEvaluation {
    let mut seen = BTreeSet::new();
    let mut held: Vec<_> = items
        .iter()
        .copied()
        .filter(|i| seen.insert(i.item_id))
        .collect();
    held.sort_by_key(|item| item.item_id);
    let mut unknown = BTreeSet::new();
    for item in held.iter().filter(|_| detailed) {
        let mut unique = BTreeSet::new();
        for (name, v) in item.properties.iter().chain(item.passive_properties.iter()) {
            if !unique.insert(name) || *v == 0.0 {
                continue;
            }
            if name.contains("ProcDamage") && !item.property_damage_types.contains_key(name) {
                unknown.insert(format!("{}: Schadenstyp von {name} fehlt; keine Widerstands-, Spirit-Auslöser- oder Lifesteal-Wechselwirkung angenommen",item.name));
            }
            if !v.is_finite() {
                unknown.insert(format!("{}: ungültiger Wert für {name}", item.name));
                continue;
            }
            if !apply(&mut Stats::default(), name, *v)
                && !metadata(name)
                && !name.contains("ProcDamage")
                && !matches!(name.as_str(), "AmmoReloadPercent" | "ActiveReloadPercent")
                && !(name == "Damage"
                    && (item.imbueable
                        || (item.description.to_ascii_lowercase().contains("ultimate")
                            && item
                                .properties
                                .get("DelayBeforeStun")
                                .copied()
                                .unwrap_or(0.0)
                                > 0.0)))
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
    if detailed {
        if let Some(bindings) = bindings {
            for item in held.iter().filter(|item| item.imbueable) {
                if !bindings.get(&item.item_id).is_some_and(|id| {
                    *id > 0
                        && hero
                            .abilities
                            .iter()
                            .any(|ability| ability.ability_id == *id)
                }) {
                    unknown.insert(format!("{}: feste Imbue-Bindung fehlt oder Fähigkeit ist nicht verfügbar; keine Auslösung",item.name));
                }
            }
        }
        for ability in &hero.abilities {
            if ability
                .properties
                .get("SelfDamagePct")
                .copied()
                .unwrap_or(0.0)
                > 0.0
            {
                unknown.insert(format!("Fähigkeit {}: Eigenkosten-Annahme {} aus ursprünglicher Mechanikakte, nicht im Snapshot eindeutig belegt; Gegenmessung im Spiel erforderlich",ability.class_name, match ability.class_name.as_str() { "ability_blood_bomb"=>"Prozent des Fähigkeitsschadens, Spirit-Widerstand mindert Eigenkosten", "ability_blood_shards"=>"Prozent des aktuellen Lebens ohne Widerstandsminderung", _=>"Bezug unklar; Fähigkeit wird nicht als schadensfrei nutzbar simuliert" }));
            }
            if ability.properties.contains_key("LifeDrainHealthMult")
                || ability.properties.contains_key("HealPctVsHeroes")
                || ability.properties.contains_key("HealthStealPctHero")
            {
                unknown.insert(format!("Fähigkeit {}: zusätzliches Item-Lifesteal neben eigener Heilung ist eine unbestätigte Annahme",ability.class_name));
            }
            for (key, value) in &ability.properties {
                if *value == 0.0 {
                    continue;
                }
                let damage = crate::ability_damage_units(ability, key) > 0.0
                    || matches!(
                        key.as_str(),
                        "Damage"
                            | "ImpactDamage"
                            | "BaseDamage"
                            | "StompDamage"
                            | "PulseDPS"
                            | "DamagePerSecond"
                            | "DPS"
                            | "DamagePerTick"
                            | "TickDamage"
                    );
                let timing = matches!(
                    key.as_str(),
                    "SlowDuration"
                        | "DebuffDuration"
                        | "EscapeTime"
                        | "EscapeRange"
                        | "HealPctVsHeroes"
                        | "HealthStealPctHero"
                        | "LifeDrainHealthMult"
                );
                if !damage
                    && !timing
                    && !metadata(key)
                    && !apply(&mut Stats::default(), key, *value)
                {
                    unknown.insert(format!(
                        "Fähigkeit {}: {key} nicht quantifiziert",
                        ability.class_name
                    ));
                }
            }
            if ability.properties.is_empty() {
                unknown.insert(format!(
                    "Fähigkeit {}: Rohdaten für Effektabdeckung fehlen",
                    ability.class_name
                ));
            }
        }
        if hero.cost_bonuses.is_empty() {
            unknown.insert("Shopbonus-Schwellen fehlen im Heldensnapshot".into());
        }
    }
    let mut result = InventoryEvaluation { shop_bonuses: shop_bonuses(hero, &held), assumptions: vec![
        "Begrenzter deterministischer Einzelzielvergleich, keine vollständige Spielsimulation oder Gewinnwahrscheinlichkeit.".into(),
        "Inventarszenario: zwölf universelle Plätze, höchstens vier aktive Items; regulärer Wiederverkauf zur Hälfte des Gesamtpreises, kein Sofort-Rückkauf.".into(),
        "Drei gleich gewichtete Szenarien: gesundes Duell, steigender Lebensdruck, bewegliches Ziel. Gleiche Annahmen für alle Helden.".into(),
        "Fähigkeiten und Itemladungen starten bereit; es gelten die übergebenen Fähigkeitsstufen und belegten Ladungs-/Abklingzeiten. Kanalisieren und Schießen teilen die verfügbare Kampfzeit.".into(),
        "Lebensschwellen des Ziels folgen dem bereits zugefügten Schaden relativ zum Basisleben des Helden; kein automatischer Zielwechsel. Eigene Gesundheitskosten nutzen den je Fähigkeit ausdrücklich benannten Prozentbezug; unklare Kosten verhindern eine simulierte kostenlose Castfolge.".into(),
        "Gegner startet ohne Resistenzen; eingehender Schaden ist zur Hälfte Waffen- und Spirit-Schaden. Lifesteal zählt höchstens den angenommenen Lebensverlust.".into(),
        "Effektives Leben ist der zeitlich gemittelte Ressourcenbestand nach Eigenkosten und tatsächlich nutzbarer Heilung; früher Tod beendet weitere Kampfereignisse. Keine gespeicherte Überheilung.".into(),
        "Bewegliches Ziel läuft mit angenommenen 8 m/s aus Kontrollkreisen; Verlangsamung reduziert die zurückgelegte Strecke, Festsetzen stoppt sie. Unbekannte Höhenschäden werden bei Höhe null nicht addiert.".into(),
        "Utility-Modellannahme: Verlangsamung und Lauftempo helfen beim Zielkontakt, bei beweglichem Ziel stärker. Überlappende Verlangsamungen nutzen nur den stärksten Wert.".into(),
        "Shopboni nach Wert der aktuell gehaltenen Items je Kategorie; Verkäufe entfernen deren Bonusanteil. Fehlende Schwellen ergeben keinen erfundenen Bonus.".into(),
        "Komplexe Gegnerreaktionen und nicht ausgewiesene Spezialladungen bleiben unquantifiziert; Imbue-Verknüpfungen bleiben bei übergebenen Kaufbindungen unverändert.".into(),
        "Verzögerte Itemauslöser aus Ult-Schaden werden konservativ höchstens einmal je Ult-Aktivierung und Ziel gerechnet; kein unbelegtes Wiederholen bei jedem Tick.".into(),
        "Zeitschritt 0,2 Sekunden; unbekannte Wirkungen werden nicht als garantierter Nutzen addiert.".into(),
    ], unknown_effects: unknown.into_iter().collect(), ..InventoryEvaluation::default() };
    for (name, pressure, moving) in [
        ("Duell", false, false),
        ("Unter Druck", true, false),
        ("Bewegliches Ziel", false, true),
    ] {
        let scenario = simulate(
            hero,
            &held,
            cfg,
            name,
            pressure,
            moving,
            (detailed, bindings),
        );
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
    if !detailed {
        result.assumptions.clear();
    }
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
    options: (bool, Option<&BTreeMap<i64, i64>>),
) -> CombatScenarioEvaluation {
    let (detailed, bindings) = options;
    let prepared: Vec<_> = hero.abilities.iter().map(PreparedAbility::new).collect();
    let item_texts: Vec<_> = items
        .iter()
        .map(|item| item.description.to_ascii_lowercase())
        .collect();
    let window = cfg.combat_window_seconds.clamp(1.0, 120.0);
    let dt = 0.2;
    let spirit_windows: Vec<_> = items
        .iter()
        .map(|item| match &item.condition {
            ConditionKind::ActionBound { action } if action == "spirit_damage_threshold" => {
                Some(value(item, "DamageThresholdDuration").max(dt))
            }
            _ => None,
        })
        .collect();
    let mut out = CombatScenarioEvaluation {
        name: name.into(),
        ..CombatScenarioEvaluation::default()
    };
    let mut ability_ready = vec![0.0; hero.abilities.len()];
    let max_charges: Vec<i64> = hero
        .abilities
        .iter()
        .map(|ability| ability.charges.max(1))
        .collect();
    let mut charges = max_charges.clone();
    let mut charge_ready: Vec<Option<f64>> = vec![None; hero.abilities.len()];
    let mut buildup = vec![0.0; hero.abilities.len()];
    let mut burn_until = vec![0.0; hero.abilities.len()];
    let mut last_hit = vec![f64::NEG_INFINITY; hero.abilities.len()];
    let mut proc_ready = vec![0.0; items.len()];
    let mut buff_ready = vec![0.0; items.len()];
    let item_targets: Vec<Option<i64>> = items
        .iter()
        .map(|item| {
            if let Some(bindings) = bindings {
                bindings.get(&item.item_id).copied().filter(|id| {
                    hero.abilities
                        .iter()
                        .any(|ability| ability.ability_id == *id && *id > 0)
                })
            } else {
                crate::mechanics::imbue_target(item, hero, cfg)
            }
        })
        .collect();
    out.imbue_targets = items
        .iter()
        .zip(&item_targets)
        .filter_map(|(item, target)| target.map(|target| (item.item_id, target)))
        .collect();
    let mut last_cast_id = None;
    let mut last_clip = hero.weapon.clip_size;
    let mut ability_effects: Vec<(f64, String, f64)> = Vec::new();
    let mut bindings: Vec<Binding> = Vec::new();
    let mut buff_until = vec![0.0; items.len()];
    let mut channel_until: f64 = 0.0;
    let mut reload_until: f64 = 0.0;
    let mut ammo: f64 = hero.weapon.clip_size.max(0.0);
    let mut initialized = false;
    let mut last_cast = f64::NEG_INFINITY;
    let mut fired = false;
    let mut spirit_events: Vec<(f64, f64)> = Vec::new();
    let mut pending_damage: Vec<DamagePeriod> = Vec::new();
    let mut pending_hits: Vec<HitEvent> = Vec::new();
    let mut ultimate_generation = 0u64;
    let mut item_ultimate_seen = vec![0u64; items.len()];
    let mut delayed_item_hits: Vec<(f64, usize)> = Vec::new();
    let mut health_sum = 0.0;
    let mut last_maximum_health: Option<f64> = None;
    let mut self_damage_sum = 0.0;
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
    let mut base_stats = Stats {
        spirit: hero.base_spirit_power,
        ..Stats::default()
    };
    let conditional_effects: Vec<Vec<(&str, f64)>> = items
        .iter()
        .map(|item| {
            let mut keys = BTreeSet::new();
            item.properties
                .iter()
                .chain(item.passive_properties.iter())
                .filter_map(|(key, v)| {
                    if !keys.insert(key) || !v.is_finite() || !apply(&mut Stats::default(), key, *v)
                    {
                        return None;
                    }
                    if conditional(item, key) {
                        Some((key.as_str(), *v))
                    } else {
                        apply(&mut base_stats, key, *v);
                        None
                    }
                })
                .collect()
        })
        .collect();
    apply_shop(hero, items, &mut base_stats);
    let item_proc_damage: Vec<Vec<(f64, crate::DamageType)>> = items
        .iter()
        .map(|item| {
            item.properties
                .iter()
                .filter(|(key, _)| key.contains("ProcDamage"))
                .map(|(key, value)| {
                    (
                        *value,
                        item.property_damage_types
                            .get(key)
                            .cloned()
                            .unwrap_or(crate::DamageType::None),
                    )
                })
                .collect()
        })
        .collect();
    let steps = (window / dt).ceil() as usize;
    for step in 0..steps {
        let time = step as f64 * dt;
        let duration = dt.min(window - time);
        let health_fraction = if pressure {
            (1.0 - 0.85 * time / window).max(0.15)
        } else {
            1.0
        };
        let mut stats = base_stats.clone();
        let mut activated = vec![false; items.len()];
        for (idx, item) in items.iter().enumerate() {
            let recent_damage = spirit_windows[idx].map_or(0.0, |threshold_window| {
                spirit_events
                    .iter()
                    .filter(|(t, _)| time - *t <= threshold_window)
                    .map(|(_, d)| *d)
                    .sum()
            });
            let cooldown = value(item, "AbilityCooldown")
                .max(value(item, "AbilityChargeUpTime"))
                .max(item.proc_cooldown.unwrap_or(0.0));
            let buff_duration = value(item, "AbilityDuration").max(value(item, "BuffDuration"));
            let reload = value(item, "AmmoReloadPercent").max(value(item, "ActiveReloadPercent"));
            let mut trigger = if item.is_active {
                reload <= 0.0 || ammo <= last_clip * 0.1
            } else if item.imbueable && reload > 0.0 {
                time - last_cast <= dt + 0.001
                    && last_cast_id == item_targets[idx]
                    && last_cast_id.is_some()
            } else {
                let base_maximum = last_maximum_health.unwrap_or(
                    (hero.base_health * (1.0 + base_stats.health_pct / 100.0) + base_stats.health)
                        * (1.0 - base_stats.health_loss.clamp(0.0, 0.99)),
                );
                let own_fraction = (base_maximum * health_fraction - self_damage_sum + leech_sum)
                    .clamp(0.0, base_maximum)
                    / base_maximum.max(1.0);
                active_with_text(
                    item,
                    time,
                    own_fraction,
                    fired,
                    last_cast,
                    recent_damage,
                    &item_texts[idx],
                )
            };
            let enemy_threshold = value(item, "EnemyLifeThreshold");
            if enemy_threshold > 0.0 {
                trigger = (1.0
                    - (out.weapon_damage + out.ability_damage + out.proc_damage)
                        / hero.base_health.max(1.0))
                .max(0.0)
                    > enemy_threshold / 100.0;
            }
            let uses_activation = item.is_active
                || item.imbueable && reload > 0.0
                || cooldown > 0.0 && buff_duration > 0.0;
            if uses_activation && trigger && time >= buff_ready[idx] {
                activated[idx] = true;
                buff_until[idx] = time + buff_duration;
                buff_ready[idx] = time + cooldown.max(dt);
                if reload > 0.0 {
                    ammo = (ammo + last_clip * reload / 100.0).min(last_clip);
                    reload_until = time;
                }
                out.item_activations
                    .entry(item.item_id)
                    .or_default()
                    .push(time);
                if detailed && out.sequence.len() < 16 {
                    out.sequence
                        .push(format!("{time:.1}s: {} ausgelöst", item.name));
                }
            }
            let enabled = if uses_activation {
                time < buff_until[idx]
            } else {
                trigger
            };
            if enabled {
                for (key, v) in &conditional_effects[idx] {
                    apply(&mut stats, key, *v);
                }
            }
        }
        ability_effects.retain(|(end, _, _)| *end > time);
        for (_, key, v) in &ability_effects {
            apply(&mut stats, key, *v);
        }
        let clip = (hero.weapon.clip_size * (1.0 + stats.clip / 100.0) + stats.flat_clip).max(1.0);
        last_clip = clip;
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
        let hit = target_contact(&stats, moving);
        let utility_contact = hit - contact;
        let weapon_opportunity = bullet * rate * hit;
        let maximum_health = ((hero.base_health * (1.0 + stats.health_pct / 100.0) + stats.health)
            * (1.0 - stats.health_loss.clamp(0.0, 0.99)))
        .max(0.0);
        let current_health = (maximum_health * health_fraction - self_damage_sum + leech_sum)
            .clamp(0.0, maximum_health);
        last_maximum_health = Some(maximum_health);
        if current_health <= 0.0 {
            if detailed {
                out.sequence.push(format!(
                    "{time:.1}s: eigenes Leben aufgebraucht; Kampfablauf beendet"
                ));
            }
            break;
        }
        for (idx, ability) in hero.abilities.iter().enumerate() {
            if charge_ready[idx].is_some_and(|ready| time >= ready) {
                charges[idx] = (charges[idx] + 1).min(max_charges[idx]);
                charge_ready[idx] = if charges[idx] < max_charges[idx] {
                    Some(
                        time + ability
                            .properties
                            .get("AbilityCooldown")
                            .copied()
                            .filter(|value| *value > 0.0)
                            .unwrap_or(if ability.cooldown > 0.0 {
                                ability.cooldown
                            } else {
                                window
                            })
                            * (1.0 - stats.cooldown.clamp(0.0, 0.8)),
                    )
                } else {
                    None
                };
            }
        }
        if time >= channel_until {
            let mut best = None;
            for (idx, ability) in hero.abilities.iter().enumerate() {
                if time < ability_ready[idx]
                    || charges[idx] <= 0
                    || ability.class_name.is_empty()
                    || ability
                        .properties
                        .get("BuildUpBulletPercentPerHit")
                        .is_some_and(|v| *v > 0.0)
                {
                    continue;
                }
                let raw_period = prepared[idx].periodic;
                let scaled_period = prepared[idx].periodic_duration(stats.duration);
                let damage = (ability.base_effect + prepared[idx].damage_scale(stats.spirit))
                    .max(0.0)
                    * if raw_period > 0.0 {
                        scaled_period / raw_period
                    } else {
                        1.0
                    };
                let cast_time = ability
                    .channel_time
                    .map(|time| {
                        if ability.properties.contains_key("AbilityChannelTime") {
                            ability_value(ability, "AbilityChannelTime", &stats)
                        } else {
                            time
                        }
                    })
                    .unwrap_or(0.0)
                    .max(0.0)
                    + ability
                        .properties
                        .get("AbilityCastDelay")
                        .copied()
                        .unwrap_or(0.0)
                        .max(0.0);
                let start_delay = ability
                    .properties
                    .get("AbilityCastDelay")
                    .copied()
                    .unwrap_or(0.0)
                    .max(0.0)
                    + ability
                        .properties
                        .get("ExplodeDelay")
                        .copied()
                        .unwrap_or(0.0)
                        .max(0.0)
                    + ability
                        .properties
                        .get("ArmingDuration")
                        .copied()
                        .unwrap_or(0.0)
                        .max(0.0);
                let effect_duration = scaled_period;
                let complete = if effect_duration > 0.0 {
                    ((window - time - start_delay) / effect_duration).clamp(0.0, 1.0)
                } else if time + start_delay < window {
                    1.0
                } else {
                    0.0
                };
                let utility = ability_utility_value(
                    hero,
                    ability,
                    &stats,
                    moving,
                    window,
                    damage,
                    &prepared[idx].utility_keys,
                );
                let damage_amp = match ability.damage_type {
                    crate::DamageType::Spirit => stats.spirit_shred,
                    crate::DamageType::Weapon => stats.bullet_shred,
                    _ => 0.0,
                };
                let Some(self_cost) =
                    self_damage_cost(ability, damage, current_health, stats.spirit_resist)
                else {
                    continue;
                };
                if self_cost >= current_health {
                    continue;
                }
                let gain = damage * complete * (1.0 + damage_amp / 100.0) + utility
                    - weapon_opportunity * cast_time
                    - self_cost;
                if gain > 0.0 && best.as_ref().is_none_or(|(_, g, _, _)| gain > *g) {
                    best = Some((idx, gain, damage, cast_time));
                }
            }
            if let Some((idx, _, damage, cast_time)) = best {
                let ability = &hero.abilities[idx];
                if ability.slot == 4 {
                    ultimate_generation += 1;
                }
                let ultimate_source = (ability.slot == 4 && !ability.item_proc_disabled)
                    .then_some(ultimate_generation);
                self_damage_sum +=
                    self_damage_cost(ability, damage, current_health, stats.spirit_resist)
                        .unwrap_or(0.0);
                let effect_time = prepared[idx].periodic_duration(stats.duration);
                let start_delay = ability
                    .properties
                    .get("AbilityCastDelay")
                    .copied()
                    .unwrap_or(0.0)
                    .max(0.0)
                    + ability
                        .properties
                        .get("ExplodeDelay")
                        .copied()
                        .unwrap_or(0.0)
                        .max(0.0)
                    + ability
                        .properties
                        .get("ArmingDuration")
                        .copied()
                        .unwrap_or(0.0)
                        .max(0.0);
                if effect_time > 0.0 {
                    pending_damage.push(DamagePeriod {
                        item_proc_disabled: ability.item_proc_disabled,
                        start: time + start_delay,
                        end: time + start_delay + effect_time,
                        rate: damage / effect_time,
                        ultimate_source,
                        damage_type: ability.damage_type.clone(),
                        heal: ability
                            .properties
                            .get("LifeDrainHealthMult")
                            .or_else(|| ability.properties.get("HealthStealPctHero"))
                            .copied()
                            .unwrap_or(0.0)
                            / 100.0,
                    });
                } else if ability.properties.get("EscapeTime").copied().unwrap_or(0.0) > 0.0 {
                    bindings.push(Binding {
                        start: time + start_delay,
                        end: time + start_delay + ability.properties["EscapeTime"],
                        travelled: 0.0,
                        range: ability
                            .properties
                            .get("EscapeRange")
                            .copied()
                            .unwrap_or(0.0),
                        damage,
                        damage_type: ability.damage_type.clone(),
                        root: ability_value(ability, "ImmobilizeDuration", &stats),
                    });
                } else {
                    pending_hits.push(HitEvent {
                        item_proc_disabled: ability.item_proc_disabled,
                        at: time + start_delay,
                        ultimate_source,
                        damage,
                        damage_type: ability.damage_type.clone(),
                        heal: ability
                            .properties
                            .get("HealPctVsHeroes")
                            .copied()
                            .unwrap_or(0.0)
                            / 100.0,
                    });
                }
                *out.casts
                    .entry(if ability.ability_id > 0 {
                        ability.ability_id
                    } else {
                        -ability.slot.max(1)
                    })
                    .or_default() += 1;
                charges[idx] -= 1;
                let recharge = ability
                    .properties
                    .get("AbilityCooldown")
                    .copied()
                    .filter(|value| *value > 0.0)
                    .unwrap_or(if ability.cooldown > 0.0 {
                        ability.cooldown
                    } else {
                        window
                    })
                    * (1.0 - stats.cooldown.clamp(0.0, 0.8));
                if charge_ready[idx].is_none() {
                    charge_ready[idx] = Some(time + recharge);
                }
                ability_ready[idx] = time
                    + ability
                        .properties
                        .get("AbilityCooldownBetweenCharge")
                        .copied()
                        .unwrap_or(0.0)
                        .max(dt);
                channel_until = time + cast_time;
                last_cast = time;
                last_cast_id = (ability.ability_id > 0).then_some(ability.ability_id);
                for key in &prepared[idx].utility_keys {
                    if *key == "ImmobilizeDuration"
                        && ability.properties.get("EscapeTime").copied().unwrap_or(0.0) > 0.0
                    {
                        continue;
                    }
                    let duration = effect_duration(ability, key, &stats);
                    if duration > 0.0 {
                        ability_effects.push((
                            time + duration,
                            (*key).to_string(),
                            ability_value(ability, key, &stats),
                        ));
                    }
                }
                if detailed && out.sequence.len() < 16 {
                    out.sequence
                        .push(format!("{time:.1}s: {}", ability.class_name));
                }
            }
        }
        for (idx, item) in items.iter().enumerate() {
            if activated[idx] && item.imbueable {
                let damage = value(item, "Damage")
                    + item
                        .property_spirit_scaling
                        .get("Damage")
                        .copied()
                        .unwrap_or_default()
                        * stats.spirit;
                let damage_type = item
                    .property_damage_types
                    .get("Damage")
                    .unwrap_or(&crate::DamageType::None);
                out.proc_damage += damage_event(
                    damage,
                    damage_type,
                    &stats,
                    hit,
                    time,
                    &mut spirit_events,
                    &mut leech_sum,
                );
            }
        }
        let mut ultimate_events = BTreeMap::new();
        let mut excluded_spirit_events = Vec::new();
        for event in &pending_damage {
            let elapsed = (event.end.min(time + duration) - event.start.max(time)).max(0.0);
            let dealt = damage_event(
                elapsed * event.rate,
                &event.damage_type,
                &stats,
                hit,
                event.start.max(time),
                if event.item_proc_disabled {
                    &mut excluded_spirit_events
                } else {
                    &mut spirit_events
                },
                &mut leech_sum,
            );
            out.ability_damage += dealt;
            if dealt > 0.0 {
                if let Some(source) = event.ultimate_source {
                    ultimate_events
                        .entry(source)
                        .or_insert(event.start.max(time));
                }
            }
            leech_sum += dealt * event.heal.max(0.0);
        }
        pending_damage.retain(|event| event.end > time + duration);
        for event in &pending_hits {
            if event.at < window && event.at <= time + duration {
                let dealt = damage_event(
                    event.damage,
                    &event.damage_type,
                    &stats,
                    hit,
                    event.at,
                    if event.item_proc_disabled {
                        &mut excluded_spirit_events
                    } else {
                        &mut spirit_events
                    },
                    &mut leech_sum,
                );
                out.ability_damage += dealt;
                if dealt > 0.0 {
                    if let Some(source) = event.ultimate_source {
                        ultimate_events.entry(source).or_insert(event.at);
                    }
                }
                leech_sum += dealt * event.heal.max(0.0);
            }
        }
        pending_hits.retain(|event| event.at > time + duration);
        for (idx, item) in items.iter().enumerate() {
            if value(item, "DelayBeforeStun") <= 0.0 || !item_texts[idx].contains("ultimate") {
                continue;
            }
            if let Some((&source, &at)) = ultimate_events
                .iter()
                .filter(|(source, _)| **source > item_ultimate_seen[idx])
                .max_by_key(|(source, _)| *source)
            {
                item_ultimate_seen[idx] = source;
                delayed_item_hits.push((at + value(item, "DelayBeforeStun"), idx));
            }
        }
        for (at, idx) in &delayed_item_hits {
            if *at >= window || *at > time + duration {
                continue;
            }
            let item = items[*idx];
            let damage = value(item, "Damage")
                + item
                    .property_spirit_scaling
                    .get("Damage")
                    .copied()
                    .unwrap_or(0.0)
                    * stats.spirit;
            let damage_type = item
                .property_damage_types
                .get("Damage")
                .unwrap_or(&crate::DamageType::None);
            out.proc_damage += damage_event(
                damage,
                damage_type,
                &stats,
                hit,
                *at,
                &mut spirit_events,
                &mut leech_sum,
            );
            let stun = value(item, "StunDuration");
            if stun > 0.0 {
                ability_effects.push((*at + stun, "StunDuration".into(), stun));
            }
            out.item_activations
                .entry(item.item_id)
                .or_default()
                .push(*at);
            if detailed && out.sequence.len() < 16 {
                out.sequence
                    .push(format!("{at:.1}s: {} nach Ult-Treffer", item.name));
            }
        }
        delayed_item_hits.retain(|(at, _)| *at > time + duration);

        for binding in &mut bindings {
            let elapsed = (binding.end.min(time + duration) - binding.start.max(time)).max(0.0);
            if moving && stats.control <= 0.0 {
                binding.travelled += 8.0 * (1.0 - stats.slow.clamp(0.0, 100.0) / 100.0) * elapsed;
            }
            if binding.end <= time + duration
                && binding.range > 0.0
                && binding.travelled < binding.range
            {
                out.ability_damage += damage_event(
                    binding.damage,
                    &binding.damage_type,
                    &stats,
                    hit,
                    binding.end,
                    &mut spirit_events,
                    &mut leech_sum,
                );
                ability_effects.push((
                    binding.end + binding.root,
                    "ImmobilizeDuration".into(),
                    binding.root,
                ));
                if detailed && out.sequence.len() < 16 {
                    out.sequence.push(format!(
                        "{:.1}s: Flucht verhindert, {:.2}s festgesetzt",
                        binding.end, binding.root
                    ));
                }
            }
        }
        bindings.retain(|binding| binding.end > time + duration);
        let mut shots = 0.0;
        if time < channel_until {
            out.channel_seconds += duration;
        } else if time >= reload_until && rate > 0.0 {
            if ammo <= 0.0001 {
                reload_until = time
                    + hero.weapon.reload_duration.max(0.0) / (1.0 + stats.reload / 100.0).max(0.1);
                ammo = clip;
                out.reloads += 1;
                if detailed && out.sequence.len() < 16 {
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
        for (idx, ability) in hero.abilities.iter().enumerate() {
            let per_hit = ability
                .properties
                .get("BuildUpBulletPercentPerHit")
                .copied()
                .unwrap_or(0.0);
            if per_hit <= 0.0 {
                continue;
            }
            let burn_duration = ability
                .properties
                .get("BurnDuration")
                .copied()
                .unwrap_or(0.0);
            if time - last_hit[idx]
                > ability
                    .properties
                    .get("BuildUpDuration")
                    .copied()
                    .unwrap_or(0.0)
            {
                buildup[idx] = 0.0;
            }
            if fired {
                last_hit[idx] = time;
                if burn_until[idx] > time {
                    burn_until[idx] = (burn_until[idx]
                        + shots
                            * hit
                            * ability
                                .properties
                                .get("RefillDuration")
                                .copied()
                                .unwrap_or(0.0))
                    .min(time + burn_duration);
                } else {
                    buildup[idx] += shots * hit * per_hit;
                    if buildup[idx] >= 100.0 {
                        burn_until[idx] = time + burn_duration;
                        buildup[idx] = 0.0;
                        *out.ability_procs.entry(ability.ability_id).or_default() += 1;
                    }
                }
            }
            if burn_until[idx] > time && burn_duration > 0.0 {
                let damage = (ability.base_effect + prepared[idx].damage_scale(stats.spirit))
                    / burn_duration
                    * (burn_until[idx] - time).min(duration);
                out.ability_damage += damage_event(
                    damage,
                    &ability.damage_type,
                    &stats,
                    1.0,
                    time,
                    &mut spirit_events,
                    &mut leech_sum,
                );
            }
        }

        out.utility += shots * bullet * utility_contact;
        for (idx, item) in items.iter().enumerate() {
            if time < proc_ready[idx] || item_proc_damage[idx].is_empty() {
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
            if let Some(cooldown) = item.proc_cooldown.filter(|v| *v > 0.0) {
                for (damage, damage_type) in &item_proc_damage[idx] {
                    out.proc_damage += damage_event(
                        *damage,
                        damage_type,
                        &stats,
                        hit,
                        time,
                        &mut spirit_events,
                        &mut leech_sum,
                    );
                }
                proc_ready[idx] = time + cooldown;
            }
        }
        let health = ((hero.base_health * (1.0 + stats.health_pct / 100.0) + stats.health)
            * (1.0 - stats.health_loss.clamp(0.0, 0.99)))
        .max(1.0);
        let mitigation = 0.5
            * (1.0 - stats.bullet_resist.clamp(-1.0, 0.9))
            * (1.0 - stats.enemy_weapon_penalty.clamp(0.0, 0.9))
            + 0.5 * (1.0 - stats.spirit_resist.clamp(-1.0, 0.9));
        leech_sum += gun_damage * stats.bullet_leech.max(0.0) / 100.0
            + stats.regeneration.max(0.0) * duration;
        leech_sum = leech_sum.min(self_damage_sum + health * (1.0 - health_fraction));
        health_sum += (health + stats.shield.max(0.0) + leech_sum - self_damage_sum) / mitigation
            * duration
            / window;
        out.spirit_power += stats.spirit * duration / window;
    }
    out.effective_health = health_sum;
    out
}
fn periodic_duration(ability: &AbilityModel) -> f64 {
    if ability.channel_time.is_some()
        || ability.tick_rate.is_some()
        || ability.properties.keys().any(|key| {
            matches!(
                key.as_str(),
                "DPS"
                    | "PulseDPS"
                    | "DamagePerSecond"
                    | "LifeDrainPerSecond"
                    | "AfflictionDPS"
                    | "TurretDPS"
                    | "TickDamage"
                    | "DamagePerTick"
                    | "PulseDamage"
            )
        })
    {
        ability.duration.or(ability.channel_time).unwrap_or(0.0)
    } else {
        0.0
    }
}
struct DamagePeriod {
    item_proc_disabled: bool,
    ultimate_source: Option<u64>,
    start: f64,
    end: f64,
    rate: f64,
    damage_type: crate::DamageType,
    heal: f64,
}
struct HitEvent {
    item_proc_disabled: bool,
    ultimate_source: Option<u64>,
    at: f64,
    damage: f64,
    damage_type: crate::DamageType,
    heal: f64,
}
struct Binding {
    start: f64,
    end: f64,
    travelled: f64,
    range: f64,
    damage: f64,
    damage_type: crate::DamageType,
    root: f64,
}
pub(crate) fn self_damage_cost(
    ability: &AbilityModel,
    damage: f64,
    current_health: f64,
    spirit_resist: f64,
) -> Option<f64> {
    let percent = ability
        .properties
        .get("SelfDamagePct")
        .copied()
        .unwrap_or(0.0)
        .max(0.0)
        / 100.0;
    if percent == 0.0 {
        return Some(0.0);
    }
    match ability.class_name.as_str() {
        "ability_blood_bomb" => Some(percent * damage * (1.0 - spirit_resist.clamp(-1.0, 0.9))),
        "ability_blood_shards" => Some(percent * current_health),
        _ => None,
    }
}
fn target_contact(stats: &Stats, moving: bool) -> f64 {
    if !moving || stats.control > 0.0 {
        1.0
    } else {
        (0.65 + stats.slow.clamp(0.0, 100.0) / 100.0 * 0.5 + stats.speed.max(0.0) / 20.0).min(1.0)
    }
}
fn ability_value(ability: &AbilityModel, key: &str, stats: &Stats) -> f64 {
    let value = ability.properties.get(key).copied().unwrap_or(0.0)
        + ability
            .scaling
            .iter()
            .filter(|scale| scale.stat == key)
            .filter_map(|scale| scale.per_spirit)
            .map(|scale| scale * stats.spirit)
            .sum::<f64>();
    if ability.duration_scaling.contains(key) {
        value * (1.0 + stats.duration)
    } else {
        value
    }
}
fn scaled_periodic_duration(ability: &AbilityModel, stats: &Stats) -> f64 {
    let base = periodic_duration(ability);
    if [
        "AbilityDuration",
        "AbilityChannelTime",
        "BurnDuration",
        "TurretLifetime",
        "MinShockDuration",
        "DebuffDuration",
    ]
    .iter()
    .any(|key| {
        ability.duration_scaling.contains(*key)
            && ability.properties.get(*key).copied() == Some(base)
    }) {
        base * (1.0 + stats.duration)
    } else {
        base
    }
}
fn effect_duration(ability: &AbilityModel, key: &str, stats: &Stats) -> f64 {
    let value = |key: &str| ability_value(ability, key, stats);
    let generic_duration = ability
        .duration
        .map(|duration| {
            if ability.duration_scaling.contains("AbilityDuration") {
                duration * (1.0 + stats.duration)
            } else {
                duration
            }
        })
        .unwrap_or(0.0);
    match key {
        "SlowPercent" | "MovementSlow" | "MovementSpeedSlow" | "MoveSpeedSlowPct" => {
            value("SlowDuration").max(generic_duration)
        }
        "WeaponPowerDebuff" => value("DebuffDuration").max(generic_duration),
        "ImmobilizeDuration" | "StunDuration" | "RootDuration" => value(key),
        _ => generic_duration,
    }
}
fn ability_utility_value(
    hero: &HeroModel,
    ability: &AbilityModel,
    stats: &Stats,
    moving: bool,
    window: f64,
    damage: f64,
    keys: &[&str],
) -> f64 {
    let mut prospective = stats.clone();
    let mut duration: f64 = 0.0;
    for key in keys {
        if apply(&mut prospective, key, ability_value(ability, key, stats)) {
            duration = duration.max(effect_duration(ability, key, stats));
        }
    }
    let output = |s: &Stats| {
        hero.weapon.bullet_damage
            * (1.0 + s.weapon / 100.0).max(0.0)
            * hero.weapon.shots_per_second
            * (1.0 + s.rate / 100.0).max(0.0)
            * target_contact(s, moving)
    };
    (output(&prospective) - output(stats)).max(0.0) * duration.min(window)
        + (prospective.shield - stats.shield).max(0.0)
        + (prospective.enemy_weapon_penalty - stats.enemy_weapon_penalty).max(0.0)
            * hero.base_health
            * 0.5
            * duration.min(window)
            / window
        + damage
            * ability
                .properties
                .get("HealPctVsHeroes")
                .copied()
                .unwrap_or(0.0)
                .max(0.0)
            / 100.0
}
fn damage_event(
    damage: f64,
    damage_type: &crate::DamageType,
    stats: &Stats,
    hit: f64,
    time: f64,
    events: &mut Vec<(f64, f64)>,
    leech: &mut f64,
) -> f64 {
    let (shred, lifesteal) = match damage_type {
        crate::DamageType::Spirit => (stats.spirit_shred, stats.spirit_leech),
        crate::DamageType::Weapon => (stats.bullet_shred, stats.bullet_leech),
        _ => (0.0, 0.0),
    };
    let dealt = damage.max(0.0) * (1.0 + shred / 100.0) * hit;
    if *damage_type == crate::DamageType::Spirit && dealt > 0.0 {
        events.push((time, dealt));
    }
    *leech += dealt * lifesteal.max(0.0) / 100.0;
    dealt
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
pub(crate) mod tests {
    use super::*;
    use crate::{AbilityRole, CostBonus, DamagePlan, DamageType, PurchaseBonuses, WeaponProfile};
    pub(crate) fn hero() -> HeroModel {
        HeroModel {
            hero_id: 1,
            name: "Testheld".into(),
            base_spirit_power: 0.0,
            standard_level_up_upgrades: Default::default(),
            standard_upgrade_levels: Default::default(),
            level_rewards: Default::default(),
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
            property_spirit_scaling: Default::default(),
            property_damage_types: Default::default(),
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
    fn ability(id: i64, damage: f64, cooldown: f64) -> AbilityModel {
        AbilityModel {
            item_proc_disabled: false,
            duration_scaling: Default::default(),
            ability_id: id,
            upgrades: vec![],
            properties: BTreeMap::from([
                ("Damage".into(), damage),
                ("AbilityCooldown".into(), cooldown),
            ]),
            class_name: format!("ability_{id}"),
            slot: 1,
            roles: vec![AbilityRole::Damage],
            scaling: vec![],
            channel_time: None,
            charges: 1,
            cooldown,
            scaling_step: None,
            damage_type: DamageType::Spirit,
            base_effect: damage,
            tick_rate: None,
            duration: None,
        }
    }
    #[test]
    fn self_damage_cannot_fund_free_casts_and_rotation_stops_on_death() {
        let mut hero = hero();
        let mut bomb = ability(99, 1800.0, 0.2);
        bomb.class_name = "ability_blood_bomb".into();
        bomb.properties.insert("SelfDamagePct".into(), 30.0);
        hero.abilities = vec![bomb];
        let cfg = ReasonerConfig {
            combat_window_seconds: 40.0,
            ..ReasonerConfig::default()
        };
        let result = evaluate_inventory(&hero, &[], &cfg);
        assert_eq!(result.scenarios[0].casts[&99], 1);
        assert_eq!(result.scenarios[1].casts[&99], 1);
        assert!(result.scenarios[1]
            .sequence
            .iter()
            .any(|event| event.contains("Leben aufgebraucht")));
        assert!(result.scenarios[1].shots < result.scenarios[0].shots);
        assert!(result
            .scenarios
            .iter()
            .all(|scenario| scenario.effective_health >= 0.0));
        assert_eq!(
            result.score,
            evaluate_inventory_fast(&hero, &[], &cfg).score
        );
    }
    #[test]
    fn quicksilver_reload_has_one_cast_bound_activation_and_fixed_binding() {
        let mut hero = hero();
        hero.abilities = vec![ability(10, 100.0, 30.0), ability(11, 1.0, 1.0)];
        let mut reload = item(1, "Damage", 44.0);
        reload.imbueable = true;
        reload.properties.extend([
            ("AmmoReloadPercent".into(), 100.0),
            ("BuffDuration".into(), 12.0),
            ("AbilityCooldown".into(), 18.0),
            ("BonusFireRate".into(), 10.0),
        ]);
        reload
            .passive_properties
            .insert("BonusFireRate".into(), 10.0);
        reload
            .property_damage_types
            .insert("Damage".into(), DamageType::Spirit);
        reload.property_spirit_scaling.insert("Damage".into(), 0.16);
        let cfg = ReasonerConfig {
            combat_window_seconds: 5.0,
            ..ReasonerConfig::default()
        };
        let bindings = BTreeMap::from([(1, 10)]);
        let full =
            evaluate_inventory_with_bindings(&hero, std::slice::from_ref(&reload), &cfg, &bindings);
        assert_eq!(full.scenarios[0].item_activations[&1].len(), 1);
        assert!((full.scenarios[0].proc_damage - 44.0).abs() < 1e-8);
        assert!(full.scenarios[0].casts[&11] > 1);
        assert_eq!(
            full.score,
            evaluate_inventory_refs_fast_with_bindings(&hero, &[&reload], &cfg, &bindings).score
        );
        let missing = evaluate_inventory_with_bindings(&hero, &[reload], &cfg, &BTreeMap::new());
        assert_eq!(missing.proc_damage, 0.0);
    }
    #[test]
    fn active_reload_and_fire_rate_share_activation_not_two_cooldowns() {
        let mut hero = hero();
        hero.weapon.clip_size = 5.0;
        hero.weapon.shots_per_second = 5.0;
        let mut active = item(1, "ActiveReloadPercent", 75.0);
        active.is_active = true;
        active.condition = ConditionKind::ActiveCooldown {
            uptime: 5.0 / 30.0,
            cooldown: 30.0,
        };
        active.properties.extend([
            ("ActiveBonusFireRate".into(), 34.0),
            ("ActiveBonusLifesteal".into(), 70.0),
            ("AbilityDuration".into(), 5.0),
            ("AbilityCooldown".into(), 30.0),
        ]);
        let result = evaluate_inventory(
            &hero,
            &[active],
            &ReasonerConfig {
                combat_window_seconds: 3.0,
                ..ReasonerConfig::default()
            },
        );
        let activation = &result.scenarios[0].item_activations[&1];
        assert_eq!(activation.len(), 1);
        assert!(activation[0] >= 0.8);
        assert!(result.scenarios[0].shots > 5.0);
    }
    #[test]
    fn low_health_buff_uses_property_threshold_without_description() {
        let mut frenzy = item(1, "FervorFireRate", 40.0);
        frenzy.properties.extend([
            ("LowHealthThreshold".into(), 50.0),
            ("AbilityDuration".into(), 10.0),
            ("AbilityCooldown".into(), 16.0),
        ]);
        let result = evaluate_inventory(
            &hero(),
            &[frenzy],
            &ReasonerConfig {
                combat_window_seconds: 10.0,
                ..ReasonerConfig::default()
            },
        );
        assert!(!result.scenarios[0].item_activations.contains_key(&1));
        assert!(result.scenarios[1].item_activations[&1][0] > 5.8);
    }
    #[test]
    fn max_health_loss_applies_to_bonus_health_and_healing_cap() {
        let mut loss = item(1, "MaxHealthLossPercent", -13.0);
        loss.properties.insert("BonusHealth".into(), 100.0);
        let result = evaluate_inventory(&hero(), &[loss], &ReasonerConfig::default());
        assert!((result.effective_health - 609.0).abs() < 1e-8);
    }
    #[test]
    fn slow_can_prevent_escape_and_control_improves_contact() {
        let mut hero = hero();
        let mut root = ability(10, 110.0, 34.0);
        root.properties.extend([
            ("EscapeTime".into(), 2.8),
            ("EscapeRange".into(), 20.0),
            ("ImmobilizeDuration".into(), 1.75),
        ]);
        hero.abilities = vec![root];
        let cfg = ReasonerConfig {
            combat_window_seconds: 5.0,
            ..ReasonerConfig::default()
        };
        let without = evaluate_inventory(&hero, &[], &cfg);
        let slow = item(2, "MoveSpeedSlowPct", 20.0);
        let with = evaluate_inventory(&hero, &[slow], &cfg);
        assert_eq!(without.scenarios[2].ability_damage, 0.0);
        assert!(with.scenarios[2].ability_damage > 0.0);
        assert!(with.scenarios[2].weapon_damage > without.scenarios[2].weapon_damage);
    }
    #[test]
    fn class_only_ability_still_fights_and_unknown_height_is_visible() {
        let mut hero = hero();
        let mut strike = ability(0, 60.0, 10.0);
        strike.properties = BTreeMap::from([
            ("StompDamage".into(), 60.0),
            ("StompDamagePerMeterPrimary".into(), 5.5),
        ]);
        hero.abilities = vec![strike];
        let result = evaluate_inventory(
            &hero,
            &[],
            &ReasonerConfig {
                combat_window_seconds: 2.0,
                ..ReasonerConfig::default()
            },
        );
        assert!(result.ability_damage > 0.0);
        assert!(result
            .unknown_effects
            .iter()
            .any(|text| text.contains("StompDamagePerMeterPrimary")));
    }
    #[test]
    fn bullet_buildup_drives_afterburn_instead_of_a_free_cast() {
        let mut hero = hero();
        hero.weapon.clip_size = 1000.0;
        hero.weapon.shots_per_second = 10.0;
        let mut burn = ability(20, 42.0, 0.0);
        burn.duration = Some(3.0);
        burn.properties = BTreeMap::from([
            ("BuildUpBulletPercentPerHit".into(), 8.1),
            ("BuildUpDuration".into(), 17.0),
            ("BurnDuration".into(), 3.0),
            ("RefillDuration".into(), 0.5),
        ]);
        hero.abilities = vec![burn];
        let cfg = ReasonerConfig {
            combat_window_seconds: 4.0,
            ..ReasonerConfig::default()
        };
        let result = evaluate_inventory(&hero, &[], &cfg);
        assert!(result.ability_damage > 0.0);
        assert!(!result.scenarios[0].casts.contains_key(&20));
        assert!(result.scenarios[0].ability_procs[&20] > 0);
        hero.weapon.shots_per_second = 0.0;
        assert_eq!(evaluate_inventory(&hero, &[], &cfg).ability_damage, 0.0);
    }
    #[test]
    fn finite_charges_use_recharge_cooldown_not_charge_spacing() {
        let mut hero = hero();
        let mut charged = ability(10, 50.0, 1.0);
        charged.charges = 3;
        charged.properties.extend([
            ("AbilityCooldown".into(), 10.0),
            ("AbilityCooldownBetweenCharge".into(), 1.0),
        ]);
        hero.abilities = vec![charged];
        let result = evaluate_inventory(
            &hero,
            &[],
            &ReasonerConfig {
                combat_window_seconds: 5.0,
                ..ReasonerConfig::default()
            },
        );
        assert_eq!(result.scenarios[0].casts[&10], 3);
    }
    #[test]
    fn periodic_damage_keeps_full_duration_rate_at_window_end() {
        let mut hero = hero();
        hero.abilities.push(AbilityModel {
            item_proc_disabled: false,
            duration_scaling: Default::default(),
            ability_id: 9,
            properties: BTreeMap::new(),
            upgrades: Default::default(),
            class_name: "periodic".into(),
            slot: 1,
            roles: vec![AbilityRole::Damage],
            scaling: vec![],
            channel_time: None,
            charges: 1,
            cooldown: 1.8,
            scaling_step: None,
            damage_type: DamageType::Spirit,
            base_effect: 100.0,
            tick_rate: Some(0.2),
            duration: Some(10.0),
        });
        let cfg = ReasonerConfig {
            combat_window_seconds: 2.0,
            ..ReasonerConfig::default()
        };
        let full = evaluate_inventory(&hero, &[], &cfg);
        assert!((full.scenarios[0].ability_damage - 22.0).abs() < 1e-8);
        assert_eq!(full.score, evaluate_inventory_fast(&hero, &[], &cfg).score);
    }
    #[test]
    fn typed_proc_events_respect_damage_type_and_trigger_spirit_threshold() {
        let mut hero = hero();
        hero.weapon.clip_size = 10000.0;
        let mut proc = item(1, "TestProcDamage", 100.0);
        proc.condition = ConditionKind::ShotBound;
        proc.proc_cooldown = Some(1.0);
        proc.property_damage_types
            .insert("TestProcDamage".into(), DamageType::Spirit);
        let leech = item(2, "AbilityLifestealPercent", 20.0);
        let mut threshold = item(3, "BonusFireRate", 100.0);
        threshold.condition = ConditionKind::ActionBound {
            action: "spirit_damage_threshold".into(),
        };
        threshold
            .conditional_properties
            .insert("BonusFireRate".into());
        threshold.properties.insert("DamageThreshold".into(), 50.0);
        threshold
            .properties
            .insert("DamageThresholdDuration".into(), 2.0);
        let cfg = ReasonerConfig {
            combat_window_seconds: 2.0,
            ..ReasonerConfig::default()
        };
        let typed = evaluate_inventory(
            &hero,
            &[proc.clone(), leech.clone(), threshold.clone()],
            &cfg,
        );
        assert!(typed.effective_health > 600.0);
        proc.property_damage_types.clear();
        let untyped = evaluate_inventory(&hero, &[proc, leech, threshold], &cfg);
        assert_eq!(untyped.effective_health, 600.0);
        assert!(typed.weapon_damage > untyped.weapon_damage);
    }
    #[test]
    fn weapon_damage_does_not_receive_spirit_shred_or_spirit_lifesteal() {
        let stats = Stats {
            spirit_shred: 50.0,
            spirit_leech: 50.0,
            bullet_shred: 20.0,
            bullet_leech: 10.0,
            ..Stats::default()
        };
        let mut events = vec![];
        let mut leech = 0.0;
        assert_eq!(
            damage_event(
                100.0,
                &DamageType::Weapon,
                &stats,
                1.0,
                0.0,
                &mut events,
                &mut leech
            ),
            120.0
        );
        assert_eq!(leech, 12.0);
        assert!(events.is_empty());
    }
    #[test]
    fn fast_and_explained_evaluations_have_identical_numbers() {
        let hero = hero();
        let items = vec![
            item(1, "WeaponPower", 25.0),
            item(2, "BonusFireRate", 20.0),
            item(3, "TechPower", 40.0),
        ];
        let cfg = ReasonerConfig::default();
        let full = evaluate_inventory(&hero, &items, &cfg);
        let fast = evaluate_inventory_fast(&hero, &items, &cfg);
        let refs = evaluate_inventory_refs_fast(&hero, &items.iter().collect::<Vec<_>>(), &cfg);
        assert_eq!(full.score, fast.score);
        assert_eq!(fast.score, refs.score);
        assert_eq!(full.weapon_damage, fast.weapon_damage);
        assert_eq!(full.ability_damage, fast.ability_damage);
        assert_eq!(full.effective_health, fast.effective_health);
        assert_eq!(full.shop_bonuses, fast.shop_bonuses);
        assert!(fast.unknown_effects.is_empty());
        assert!(fast.scenarios.iter().all(|s| s.sequence.is_empty()));
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
        let one = evaluate_inventory(&hero, std::slice::from_ref(&damage), &cfg).weapon_damage;
        let two = evaluate_inventory(&hero, std::slice::from_ref(&rate), &cfg).weapon_damage;
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
        let with_spirit = evaluate_inventory(&hero, std::slice::from_ref(&spirit), &cfg);
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
            item_proc_disabled: false,
            duration_scaling: Default::default(),
            ability_id: 2,
            properties: BTreeMap::new(),
            upgrades: Default::default(),
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
