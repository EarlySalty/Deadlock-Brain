use crate::ability_interactions::{AbilityInteractions, TargetAmplification};
use crate::item_interactions::ItemInteraction;
use crate::{AbilityModel, ConditionKind, HeroModel, ItemModel, ReasonerConfig, SlotType};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CombatEndReason {
    #[default]
    WindowElapsed,
    TargetDefeated,
    SelfDefeated,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct CombatScenarioEvaluation {
    pub name: String,
    pub target_health: f64,
    pub elapsed_seconds: f64,
    pub end_reason: CombatEndReason,
    pub contact_seconds: f64,
    pub first_ttk: Option<f64>,
    pub targets_defeated: usize,
    pub target_switches: usize,
    pub kill_times: Vec<f64>,
    pub damage_per_second: f64,
    pub damage_score: f64,
    pub survival_score: f64,
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
    pub item_condition_seconds: BTreeMap<i64, f64>,
    pub item_regeneration: BTreeMap<i64, f64>,
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
    bonus_ability_charges: f64,
    charged_spirit: f64,
    charged_cooldown: f64,
    charge_spacing_reduction: f64,
    duration: f64,
    slow: f64,
    speed: f64,
    control: f64,
    enemy_weapon_penalty: f64,
    enemy_rate_slow: f64,
    weapon_amp: f64,
    spirit_amp: f64,
}
fn charged_property(name: &str) -> bool {
    matches!(
        name,
        "BonusAbilityCharges"
            | "BonusSpiritForChargedAbilities"
            | "CooldownReductionOnChargedAbilities"
            | "CooldownBetweenChargeReduction"
    )
}

fn ability_spirit(ability: &AbilityModel, stats: &Stats) -> f64 {
    stats.spirit
        + if ability.charges > 0 {
            stats.charged_spirit
        } else {
            0.0
        }
}

fn ability_cooldown_factor(ability: &AbilityModel, stats: &Stats) -> f64 {
    let reduction = if ability.charges > 0 {
        1.0 - (1.0 - stats.cooldown) * (1.0 - stats.charged_cooldown)
    } else {
        stats.cooldown
    };
    1.0 - reduction.clamp(0.0, 0.8)
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
        "BonusAbilityCharges" => stats.bonus_ability_charges += v,
        "BonusSpiritForChargedAbilities" => stats.charged_spirit += v,
        "CooldownReductionOnChargedAbilities" => {
            stats.charged_cooldown = 1.0 - (1.0 - stats.charged_cooldown) * (1.0 - v / 100.0)
        }
        "CooldownBetweenChargeReduction" => {
            stats.charge_spacing_reduction =
                1.0 - (1.0 - stats.charge_spacing_reduction) * (1.0 - v / 100.0)
        }
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
        ConditionKind::SpiritDamageToHeroes { .. } => false, // Stateful damage events below, never guaranteed uptime.
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

/// Returns observed stack occupancy and useful healing per configured second for
/// the same finite solo-item scenarios used by the planner, not a weapon-share label.
pub(crate) fn damage_refresh_summary(
    item: &ItemModel,
    hero: &HeroModel,
    cfg: &ReasonerConfig,
) -> Option<(f64, f64)> {
    if !matches!(item.condition, ConditionKind::SpiritDamageToHeroes { .. }) {
        return None;
    }
    let evaluation = evaluate_inventory_fast(hero, std::slice::from_ref(item), cfg);
    let elapsed: f64 = evaluation.scenarios.iter().map(|s| s.elapsed_seconds).sum();
    let exposure: f64 = evaluation
        .scenarios
        .iter()
        .map(|s| {
            s.item_condition_seconds
                .get(&item.item_id)
                .copied()
                .unwrap_or(0.0)
        })
        .sum();
    let healing: f64 = evaluation
        .scenarios
        .iter()
        .map(|s| {
            s.item_regeneration
                .get(&item.item_id)
                .copied()
                .unwrap_or(0.0)
        })
        .sum();
    Some((
        exposure / elapsed.max(0.2),
        healing / 3.0 / cfg.combat_window_seconds.clamp(1.0, 120.0),
    ))
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
    if detailed {
        unknown.extend(crate::mechanics::hero_scaling_warnings(hero));
        let rate = crate::mechanics::spirit_weapon_rate_provenance(hero);
        if rate.unknown_nonfinite {
            unknown.insert("Spirit → Feuerrate: ungültige Helden-Konversion ohne validen Ersatz; nicht quantifiziert".into());
        } else if rate.recovered_from_nonfinite {
            unknown.insert(
                "Spirit → Feuerrate: ungültiger Direktwert; Recovery aus Prozent-Alias".into(),
            );
        }
    }
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
            if charged_property(name) && conditional(item, name) {
                unknown.insert(format!("{}: zeitweise Wirkung von {name} auf Ladungsfähigkeiten nicht quantifiziert; kein kostenloses Wiederauffüllen angenommen", item.name));
            }
            if !ItemInteraction::from_item(item).handles_property(name)
                && !apply(&mut Stats::default(), name, *v)
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
                    && !crate::ability_interactions::quantified_property(ability, key)
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
        "Begrenzter deterministischer Kampfvergleich, keine vollständige Spielsimulation oder Gewinnwahrscheinlichkeit.".into(),
        "Inventarszenario: zwölf universelle Plätze, höchstens vier aktive Items; regulärer Wiederverkauf zur Hälfte des Gesamtpreises, kein Sofort-Rückkauf.".into(),
        "Drei gleich gewichtete Szenarien: gesundes Duell, steigender Lebensdruck, bewegliches Ziel. Gleiche Annahmen für alle Helden.".into(),
        "Eigener Tod bewertet den verbleibenden Rest des Kampffensters als Wirkungsausfall: Schadensleistung und Überlebensbeitrag werden nicht durch die kurze Niederlage hochgerechnet. Beobachteter Durchsatz und tatsächliche Kampfzeit bleiben separat sichtbar; gewonnene Einzelduelle verwenden ihre echte Tötungszeit.".into(),
        "Fähigkeiten und Itemladungen starten bereit; es gelten die übergebenen Fähigkeitsstufen und belegten Ladungs-/Abklingzeiten. Kanalisieren und Schießen teilen die verfügbare Kampfzeit.".into(),
        "Jedes Ziel hat als globale Vergleichsannahme das Basisleben des übergebenen Helden. Duell und bewegliches Einzelziel enden bei Zieltod; unter Druck folgt nach genau 1,0 s ein gleiches frisches Ziel. Wechselpause und eigene Abklingzeiten zählen zur tatsächlichen verstrichenen Zeit.".into(),
        "Schaden wird vor Auslösern und Heilung am verbleibenden Zielleben begrenzt. Zielgebundene Treffer, DoTs, Kontrolle und Schadensschwellen enden beim Zieltod; eigene HP, Munition und Abklingzeiten werden nicht zurückgesetzt.".into(),
        "PulseDPS mit Radius wird als eigene zeitlich laufende Aura interpretiert und darf ein Folgeziel während der verbleibenden Dauer treffen. Unklare andere Effektübertragung auf Folgeziele wird konservativ nicht angenommen; Mehrfachkontakte sind kein belegtes Matchmodell.".into(),
        "Schadensleistung nutzt die tatsächlich verstrichene Szenariozeit. Der separat ausgewiesene Überlebensbeitrag bleibt effektives Leben geteilt durch das ursprüngliche konfigurierte Kampffenster; kurze TTK erhöht sein Gewicht nicht.".into(),
        "Eigene Gesundheitskosten nutzen den je Fähigkeit ausdrücklich benannten Prozentbezug; unklare Kosten verhindern eine simulierte kostenlose Castfolge.".into(),
        "Gegner startet ohne Resistenzen; eingehender Schaden ist zur Hälfte Waffen- und Spirit-Schaden. Lifesteal zählt höchstens den angenommenen Lebensverlust.".into(),
        "Effektives Leben ist der zeitlich gemittelte Ressourcenbestand nach Eigenkosten und tatsächlich nutzbarer Heilung; früher Tod beendet weitere Kampfereignisse. Keine gespeicherte Überheilung.".into(),
        "Bewegliches Ziel läuft mit angenommenen 8 m/s aus Kontrollkreisen; Verlangsamung reduziert die zurückgelegte Strecke, Festsetzen stoppt sie. Unbekannte Höhenschäden werden bei Höhe null nicht addiert.".into(),
        "Utility-Modellannahme: Verlangsamung und Lauftempo helfen beim Zielkontakt, bei beweglichem Ziel stärker. Überlappende Verlangsamungen nutzen nur den stärksten Wert.".into(),
        "Shopboni nach Wert der aktuell gehaltenen Items je Kategorie; Verkäufe entfernen deren Bonusanteil. Fehlende Schwellen ergeben keinen erfundenen Bonus.".into(),
        "Komplexe Gegnerreaktionen und nicht ausgewiesene Spezialladungen bleiben unquantifiziert; Imbue-Verknüpfungen bleiben bei übergebenen Kaufbindungen unverändert.".into(),
        "Verzögerte Itemauslöser aus Ult-Schaden werden konservativ höchstens einmal je Ult-Aktivierung und Ziel gerechnet; kein unbelegtes Wiederholen bei jedem Tick.".into(),
        "Zeitschritt und TTK-Auflösung 0,2 Sekunden; Ereignisse innerhalb eines Schritts folgen der ausgewiesenen Rotation. Unbekannte Wirkungen werden nicht als garantierter Nutzen addiert.".into(),
    ], unknown_effects: unknown.into_iter().collect(), ..InventoryEvaluation::default() };
    result.assumptions.push("Rotation verwendet eine begrenzte gierige Aktionswahl: Verstärkungsnutzen wird durch Restleben, verfügbaren Waffendurchsatz und verbleibende Zeit begrenzt; keine vollständige optimale Fähigkeitsfolge. Nachladen durch dieselbe Imbue-Auslösung erfolgt vor dem Magazinbonus. Buffstats werden mit der nächsten 0,2-s-Zustandsauswertung wirksam.".into());
    if detailed {
        result.assumptions.push("Ein bestätigter Treffer je Fähigkeitsauslösung; Malice-Stapel haben einzelne Ablaufzeiten und ersetzen am Limit den ältesten Stapel. Hook-Verstärkung beginnt nach seinem Treffer; Uppercut heilt fehlendes Leben und setzt nur Hook zurück. Soul Exchange bleibt mangels eindeutiger HP-Bezugsregel unquantifiziert.".into());
        result.assumptions.push("Aura-Nähe als globale Szenarioannahme: Duell und Druckziel 8 m, bewegliches Ziel 20 m; ein gegnerischer Held, während Wechselpausen keiner. Belegter Itemradius entscheidet die Wirkung. Keine Mehrzielbehauptung.".into());
        for item in &held {
            let interaction = ItemInteraction::from_item(item);
            result
                .assumptions
                .extend(interaction.assumptions().into_iter().map(str::to_owned));
            result.unknown_effects.extend(
                interaction
                    .unknown_effects()
                    .into_iter()
                    .map(|unknown| format!("{}: {unknown}", item.name)),
            );
        }
    }
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
        result.score += (scenario.damage_score + scenario.survival_score) / 3.0;
        result.scenarios.push(scenario);
    }
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
    let interactions: Vec<_> = items
        .iter()
        .map(|item| ItemInteraction::from_item(item))
        .collect();
    let has_refresh = interactions
        .iter()
        .any(|interaction| interaction.regeneration_refresh.is_some());
    let mut refresh_states: Vec<_> = items
        .iter()
        .map(|_| crate::damage_conditions::RefreshStacks::default())
        .collect();
    let mut magazine_buffs = vec![false; items.len()];
    let mut target_amplification = TargetAmplification::default();
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
        target_health: hero.base_health.max(1.0),
        name: name.into(),
        ..CombatScenarioEvaluation::default()
    };
    let mut ability_ready = vec![0.0; hero.abilities.len()];
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
    let mut target_remaining = out.target_health;
    let mut target_generation = 0usize;
    let mut buff_targets = vec![0usize; items.len()];
    let mut next_target_at = f64::INFINITY;
    let mut last_clip = hero.weapon.clip_size;
    let mut ability_effects: Vec<(f64, String, f64)> = Vec::new();
    let mut active_bindings: Vec<Binding> = Vec::new();
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
    // Einzige Quelle der Spirit->Feuerrate-Konversion (siehe mechanics); keine
    // eigene Ableitung mehr im Sim-Pfad.
    let weapon_scaling = crate::mechanics::weapon_spirit_scaling(hero);
    let mut base_stats = Stats {
        spirit: hero.base_spirit_power,
        ..Stats::default()
    };
    let conditional_effects: Vec<Vec<(&str, f64)>> = items
        .iter()
        .enumerate()
        .map(|(idx, item)| {
            let mut keys = BTreeSet::new();
            item.properties
                .iter()
                .chain(item.passive_properties.iter())
                .filter_map(|(key, v)| {
                    if !keys.insert(key)
                        || !v.is_finite()
                        || interactions[idx].handles_property(key)
                        || !apply(&mut Stats::default(), key, *v)
                    {
                        return None;
                    }
                    if conditional(item, key) {
                        // Zeitweise Ladungs-Refills benötigen eine eigene
                        // Aktivierungsregel. Nicht als permanente Extra-Ladung erfinden.
                        (!charged_property(key)).then_some((key.as_str(), *v))
                    } else {
                        apply(&mut base_stats, key, *v);
                        None
                    }
                })
                .collect()
        })
        .collect();
    apply_shop(hero, items, &mut base_stats);
    let max_charges: Vec<i64> = hero
        .abilities
        .iter()
        .map(|ability| {
            if ability.charges > 0 {
                (ability.charges as f64 + base_stats.bonus_ability_charges)
                    .max(0.0)
                    .floor() as i64
            } else {
                1
            }
        })
        .collect();
    let mut charges = max_charges.clone();
    let mut charge_ready: Vec<Option<f64>> = vec![None; hero.abilities.len()];
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
        let spirit_event_start = spirit_events.len();
        if pressure && target_remaining <= 0.0 && time + 1e-9 >= next_target_at {
            target_remaining = out.target_health;
            out.target_switches += 1;
            target_generation += 1;
            if detailed {
                out.sequence.push(format!(
                    "{time:.1}s: nächstes Ziel mit {:.0} Leben",
                    out.target_health
                ));
            }
        }
        let target_available = target_remaining > 0.0;
        let health_fraction = if pressure {
            (1.0 - 0.85 * time / window).max(0.15)
        } else {
            1.0
        };
        let mut stats = base_stats.clone();
        stats.weapon_amp = target_amplification.weapon_multiplier(time) - 1.0;
        stats.spirit_amp = target_amplification.spirit_multiplier(time) - 1.0;
        for interaction in &interactions {
            let distance = if moving { 20.0 } else { 8.0 };
            let aura = interaction.aura_effects(
                usize::from(target_available),
                target_available
                    && interaction
                        .aura_radius_m
                        .is_some_and(|radius| distance <= radius),
            );
            stats.bullet_shred += aura.bullet_resist_reduction_percent;
            stats.enemy_rate_slow =
                1.0 - (1.0 - stats.enemy_rate_slow) * (1.0 - aura.fire_rate_slow_percent / 100.0);
        }
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
                false
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
                trigger = target_remaining / out.target_health > enemy_threshold / 100.0;
            }
            let uses_activation = item.is_active
                || item.imbueable && reload > 0.0
                || cooldown > 0.0 && buff_duration > 0.0;
            if uses_activation && trigger && time >= buff_ready[idx] {
                buff_targets[idx] = target_generation;
                activated[idx] = true;
                buff_until[idx] = time + buff_duration;
                buff_ready[idx] = time + cooldown.max(dt);
                if reload > 0.0 {
                    magazine_buffs.fill(false);
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
                    if target_effect(key)
                        && (!target_available
                            || uses_activation && buff_targets[idx] != target_generation)
                    {
                        continue;
                    }
                    apply(&mut stats, key, *v);
                }
            }
        }
        ability_effects.retain(|(end, _, _)| *end > time);
        for (_, key, v) in &ability_effects {
            apply(&mut stats, key, *v);
        }
        let weapon = weapon_scaling.project(&hero.weapon, stats.spirit);
        let clip = (weapon.clip_size * (1.0 + stats.clip / 100.0) + stats.flat_clip).max(0.0);
        last_clip = clip;
        if !initialized {
            ammo = clip;
            initialized = true;
        }
        let rate = (weapon.shots_per_second * (1.0 + stats.rate / 100.0)).max(0.0);
        let bullet = weapon.bullet_damage
            * (1.0 + stats.weapon / 100.0).max(0.0)
            * (1.0 + stats.bullet_shred / 100.0)
            * (1.0 + stats.weapon_amp);
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
            out.end_reason = CombatEndReason::SelfDefeated;
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
                            * ability_cooldown_factor(ability, &stats),
                    )
                } else {
                    None
                };
            }
        }
        if time >= channel_until && target_available {
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
                let damage = (ability.base_effect
                    + prepared[idx].damage_scale(ability_spirit(ability, &stats)))
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
                let interaction = AbilityInteractions::from_ability(ability, 1.0 + stats.duration);
                let useful_time =
                    (target_remaining / weapon_opportunity.max(1.0)).min(window - time);
                let interaction_gain = interaction.weapon_amp.map_or(0.0, |amp| {
                    weapon_opportunity * amp.fraction * amp.duration.min(useful_time)
                }) + interaction.all_damage_amp.map_or(0.0, |amp| {
                    weapon_opportunity * amp.fraction * amp.duration.min(useful_time)
                }) + interaction
                    .missing_health_heal(current_health, maximum_health);
                let imbue_gain = interactions
                    .iter()
                    .enumerate()
                    .filter(|(item_idx, _)| {
                        !magazine_buffs[*item_idx]
                            && item_targets[*item_idx] == Some(ability.ability_id)
                            && ability.ability_id > 0
                    })
                    .map(|(_, interaction)| {
                        interaction.magazine_spirit_damage(
                            weapon.bullet_damage,
                            stats.spirit,
                            ammo.min(rate * useful_time) * hit,
                            true,
                        )
                    })
                    .sum::<f64>();
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
                let gain = damage * complete * (1.0 + damage_amp / 100.0)
                    + utility
                    + interaction_gain
                    + imbue_gain
                    - weapon_opportunity * cast_time
                    - self_cost;
                if gain > 0.0 && best.as_ref().is_none_or(|(_, g, _, _)| gain > *g) {
                    best = Some((idx, gain, damage, cast_time));
                }
            }
            if let Some((idx, _, damage, cast_time)) = best {
                let ability = &hero.abilities[idx];
                let interaction = AbilityInteractions::from_ability(ability, 1.0 + stats.duration);
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
                        source_aura: ability.properties.contains_key("PulseDPS")
                            && ability.properties.get("Radius").copied().unwrap_or(0.0) > 0.0,
                        item_proc_disabled: ability.item_proc_disabled,
                        start: time + start_delay,
                        end: time + start_delay + effect_time,
                        rate: damage / effect_time,
                        ultimate_source,
                        damage_type: ability.damage_type.clone(),
                        heal: interaction.native_damage_heal_fraction
                            + ability
                                .properties
                                .get("LifeDrainHealthMult")
                                .or_else(|| ability.properties.get("HealthStealPctHero"))
                                .copied()
                                .unwrap_or(0.0)
                                / 100.0,
                    });
                } else if ability.properties.get("EscapeTime").copied().unwrap_or(0.0) > 0.0 {
                    active_bindings.push(Binding {
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
                        interaction,
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
                    * ability_cooldown_factor(ability, &stats);
                if charge_ready[idx].is_none() {
                    charge_ready[idx] = Some(time + recharge);
                }
                let spacing_factor = if ability.charges > 0 {
                    (1.0 - stats.charge_spacing_reduction).max(0.0)
                } else {
                    1.0
                };
                ability_ready[idx] = time
                    + (ability
                        .properties
                        .get("AbilityCooldownBetweenCharge")
                        .copied()
                        .unwrap_or(0.0)
                        * spacing_factor)
                        .max(dt);
                channel_until = time + cast_time;
                last_cast = time;
                let last_cast_id = (ability.ability_id > 0).then_some(ability.ability_id);
                for (item_idx, item) in items.iter().enumerate() {
                    let reload =
                        value(item, "AmmoReloadPercent").max(value(item, "ActiveReloadPercent"));
                    if !item.is_active
                        && item.imbueable
                        && reload > 0.0
                        && last_cast_id.is_some()
                        && last_cast_id == item_targets[item_idx]
                        && time >= buff_ready[item_idx]
                    {
                        magazine_buffs.fill(false);
                        ammo = (ammo + clip * reload / 100.0).min(clip);
                        reload_until = time;
                        activated[item_idx] = true;
                        buff_targets[item_idx] = target_generation;
                        buff_until[item_idx] =
                            time + value(item, "AbilityDuration").max(value(item, "BuffDuration"));
                        buff_ready[item_idx] = time
                            + value(item, "AbilityCooldown")
                                .max(value(item, "AbilityChargeUpTime"))
                                .max(item.proc_cooldown.unwrap_or(0.0))
                                .max(dt);
                        out.item_activations
                            .entry(item.item_id)
                            .or_default()
                            .push(time);
                        if detailed && out.sequence.len() < 16 {
                            out.sequence.push(format!(
                                "{time:.1}s: {} durch gebundene Fähigkeit ausgelöst",
                                item.name
                            ));
                        }
                    }
                }
                for (item_idx, interaction) in interactions.iter().enumerate() {
                    if interaction.has_magazine_buff()
                        && last_cast_id.is_some()
                        && last_cast_id == item_targets[item_idx]
                        && (activated[item_idx]
                            || (value(items[item_idx], "AmmoReloadPercent")
                                .max(value(items[item_idx], "ActiveReloadPercent"))
                                <= 0.0
                                && time >= buff_ready[item_idx]))
                    {
                        magazine_buffs[item_idx] = true;
                        if !activated[item_idx] {
                            buff_ready[item_idx] =
                                time + value(items[item_idx], "AbilityCooldown").max(dt);
                            out.item_activations
                                .entry(items[item_idx].item_id)
                                .or_default()
                                .push(time);
                        }
                    }
                }
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
                    &mut target_remaining,
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
                &mut target_remaining,
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
                let landed = target_remaining > 0.0 && hit > 0.0;
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
                    &mut target_remaining,
                );
                out.ability_damage += dealt;
                if dealt > 0.0 {
                    if let Some(source) = event.ultimate_source {
                        ultimate_events.entry(source).or_insert(event.at);
                    }
                }
                leech_sum += dealt * event.heal.max(0.0);
                if landed {
                    target_amplification.on_hit(event.interaction, event.at);
                    let own_health = (maximum_health * health_fraction - self_damage_sum
                        + leech_sum)
                        .clamp(0.0, maximum_health);
                    leech_sum += event
                        .interaction
                        .missing_health_heal(own_health, maximum_health)
                        * hit;
                    if let Some(class) = event.interaction.reset_ability_class {
                        if let Some(idx) = hero
                            .abilities
                            .iter()
                            .position(|ability| ability.class_name == class)
                        {
                            ability_ready[idx] = time;
                            charges[idx] = (charges[idx] + 1).min(max_charges[idx]);
                            if charges[idx] == max_charges[idx] {
                                charge_ready[idx] = None;
                            }
                        }
                    }
                }
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
            if *at >= window || *at > time + duration || target_remaining <= 0.0 {
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
                &mut target_remaining,
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

        for binding in &mut active_bindings {
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
                    &mut target_remaining,
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
        active_bindings.retain(|binding| binding.end > time + duration);
        let mut shots = 0.0;
        if time < channel_until {
            out.channel_seconds += duration;
        } else if time >= reload_until && rate > 0.0 && target_remaining > 0.0 {
            if ammo <= 0.0001 {
                magazine_buffs.fill(false);
                reload_until = time
                    + hero.weapon.reload_duration.max(0.0) / (1.0 + stats.reload / 100.0).max(0.1);
                ammo = clip;
                out.reloads += 1;
                if detailed && out.sequence.len() < 16 {
                    out.sequence.push(format!("{time:.1}s: Nachladen"));
                }
            } else {
                shots = (rate * duration).min(ammo);
                let bonus = interactions
                    .iter()
                    .zip(&magazine_buffs)
                    .map(|(interaction, active)| {
                        interaction.magazine_spirit_damage(
                            weapon.bullet_damage,
                            stats.spirit,
                            1.0,
                            *active,
                        )
                    })
                    .sum::<f64>()
                    * (1.0 + stats.spirit_shred / 100.0)
                    * (1.0 + stats.spirit_amp);
                if (bullet + bonus) * hit > 0.0 {
                    shots = shots.min(target_remaining / ((bullet + bonus) * hit));
                }
                ammo -= shots;
            }
        }
        fired = shots > 0.0;
        let gun_damage = (shots * bullet * hit).min(target_remaining);
        target_remaining -= gun_damage;
        out.shots += shots;
        out.weapon_damage += gun_damage;
        for (interaction, active) in interactions.iter().zip(&magazine_buffs) {
            let damage = interaction.magazine_spirit_damage(
                weapon.bullet_damage,
                stats.spirit,
                shots * hit,
                *active,
            );
            out.proc_damage += damage_event(
                damage,
                &crate::DamageType::Spirit,
                &stats,
                1.0,
                time,
                &mut spirit_events,
                &mut leech_sum,
                &mut target_remaining,
            );
        }
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
                let damage = (ability.base_effect
                    + prepared[idx].damage_scale(ability_spirit(ability, &stats)))
                    / burn_duration
                    * (burn_until[idx] - time).min(duration);
                out.ability_damage += damage_event(
                    damage,
                    &ability.damage_type,
                    &stats,
                    1.0,
                    time,
                    if ability.item_proc_disabled {
                        &mut excluded_spirit_events
                    } else {
                        &mut spirit_events
                    },
                    &mut leech_sum,
                    &mut target_remaining,
                );
            }
        }

        out.utility += if hit > 0.0 {
            gun_damage * utility_contact / hit
        } else {
            0.0
        };
        for (idx, item) in items.iter().enumerate() {
            if target_remaining <= 0.0 {
                break;
            }
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
                        &mut target_remaining,
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
            * (1.0 - stats.enemy_rate_slow.clamp(0.0, 0.9))
            + 0.5 * (1.0 - stats.spirit_resist.clamp(-1.0, 0.9));
        leech_sum += gun_damage * stats.bullet_leech.max(0.0) / 100.0
            + stats.regeneration.max(0.0) * duration;
        leech_sum = leech_sum.min(self_damage_sum + health * (1.0 - health_fraction));
        if has_refresh {
            let eligible_events: Vec<_> = spirit_events[spirit_event_start..]
                .iter()
                .filter(|(_, damage)| *damage > 0.0)
                .map(|(at, _)| (*at, target_generation))
                .collect();
            let mut refresh_heals = Vec::new();
            for (idx, interaction) in interactions.iter().enumerate() {
                if let Some((amount, seconds, max_stacks)) = interaction.regeneration_refresh {
                    let exposure = refresh_states[idx].integrate(
                        time,
                        time + duration,
                        seconds,
                        max_stacks,
                        &eligible_events,
                    );
                    *out.item_condition_seconds
                        .entry(items[idx].item_id)
                        .or_default() += exposure;
                    refresh_heals.push((items[idx].item_id, amount * exposure));
                }
            }
            let potential: f64 = refresh_heals.iter().map(|(_, amount)| *amount).sum();
            let missing = (self_damage_sum + health * (1.0 - health_fraction) - leech_sum).max(0.0);
            let effective = potential.min(missing);
            leech_sum += effective;
            if potential > 0.0 {
                for (id, amount) in refresh_heals {
                    *out.item_regeneration.entry(id).or_default() += amount * effective / potential;
                }
            }
        }
        health_sum += (health + stats.shield.max(0.0) + leech_sum - self_damage_sum) / mitigation
            * duration
            / window;
        out.spirit_power += stats.spirit * duration / window;
        out.elapsed_seconds = time + duration;
        if target_available {
            out.contact_seconds += duration;
        }
        if target_available && target_remaining <= 1e-9 {
            target_remaining = 0.0;
            target_amplification.clear();
            out.targets_defeated += 1;
            out.kill_times.push(out.elapsed_seconds);
            out.first_ttk.get_or_insert(out.elapsed_seconds);
            if detailed {
                out.sequence
                    .push(format!("{:.1}s: Ziel besiegt", out.elapsed_seconds));
            }
            if !pressure {
                out.end_reason = CombatEndReason::TargetDefeated;
                break;
            }
            next_target_at = out.elapsed_seconds + 1.0;
            pending_damage.retain(|event| event.source_aura);
            if pending_damage.is_empty() {
                channel_until = out.elapsed_seconds;
            }
            pending_hits.clear();
            active_bindings.clear();
            delayed_item_hits.clear();
            spirit_events.clear();
            buildup.fill(0.0);
            burn_until.fill(0.0);
            last_hit.fill(f64::NEG_INFINITY);
            fired = false;
            item_ultimate_seen.fill(0);
            ability_effects.retain(|(_, key, _)| !target_effect(key));
        }
    }
    let elapsed = out.elapsed_seconds.max(dt);
    out.effective_health = health_sum;
    out.spirit_power *= window / elapsed;
    out.damage_per_second = (out.weapon_damage + out.ability_damage + out.proc_damage) / elapsed;
    out.damage_score = (out.weapon_damage + out.ability_damage + out.proc_damage)
        / if out.end_reason == CombatEndReason::SelfDefeated {
            window
        } else {
            elapsed
        };
    out.survival_score = out.effective_health / window;
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
    source_aura: bool,
    item_proc_disabled: bool,
    ultimate_source: Option<u64>,
    start: f64,
    end: f64,
    rate: f64,
    damage_type: crate::DamageType,
    heal: f64,
}
struct HitEvent {
    interaction: AbilityInteractions,
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
            .map(|scale| scale * ability_spirit(ability, stats))
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
    let weapon_scaling = crate::mechanics::weapon_spirit_scaling(hero);
    let output = |s: &Stats| {
        let weapon = weapon_scaling.project(&hero.weapon, s.spirit);
        weapon.bullet_damage
            * (1.0 + s.weapon / 100.0).max(0.0)
            * weapon.shots_per_second
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
#[allow(clippy::too_many_arguments)]
fn damage_event(
    damage: f64,
    damage_type: &crate::DamageType,
    stats: &Stats,
    hit: f64,
    time: f64,
    events: &mut Vec<(f64, f64)>,
    leech: &mut f64,
    target_remaining: &mut f64,
) -> f64 {
    let (shred, lifesteal) = match damage_type {
        crate::DamageType::Spirit => (stats.spirit_shred, stats.spirit_leech),
        crate::DamageType::Weapon => (stats.bullet_shred, stats.bullet_leech),
        _ => (0.0, 0.0),
    };
    let amp = match damage_type {
        crate::DamageType::Spirit => stats.spirit_amp,
        crate::DamageType::Weapon => stats.weapon_amp,
        _ => 0.0,
    };
    let dealt = (damage.max(0.0) * (1.0 + shred / 100.0) * (1.0 + amp) * hit)
        .max(0.0)
        .min(*target_remaining);
    *target_remaining -= dealt;
    if *damage_type == crate::DamageType::Spirit && dealt > 0.0 {
        events.push((time, dealt));
    }
    *leech += dealt * lifesteal.max(0.0) / 100.0;
    dealt
}
fn target_effect(key: &str) -> bool {
    matches!(
        key,
        "SlowPercent"
            | "MovementSlow"
            | "MovementSpeedSlow"
            | "MoveSpeedSlowPct"
            | "StunDuration"
            | "RootDuration"
            | "ImmobilizeDuration"
            | "WeaponPowerDebuff"
            | "BulletArmorReduction"
            | "BulletResistReduction"
            | "TechArmorReduction"
            | "SpiritResistReduction"
    )
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
    fn refresh_regeneration() -> ItemModel {
        let mut item = item(911, "Regeneration", 4.0);
        item.properties.insert("RegenerationDuration".into(), 7.0);
        item.description = "Dealing <span>spirit damage</span> to enemy Heroes grants regeneration. Stacks when dealing damage to different heroes.".into();
        crate::item::build_item_model(&item).unwrap()
    }

    #[test]
    fn sustain_refresh_is_derived_without_item_or_hero_identity() {
        let item = refresh_regeneration();
        let json = serde_json::to_value(&item.condition).unwrap();
        assert!(json.get("SpiritDamageToHeroes").is_some(), "{json}");
        let mut renamed = item.clone();
        renamed.name = "Unbekannter Anzeigename".into();
        renamed.class_name = "anonymous".into();
        assert_eq!(
            crate::item::build_item_model(&renamed).unwrap().condition,
            item.condition
        );
    }

    #[test]
    fn sustain_refresh_uses_damage_timing_and_respects_proc_exclusion() {
        let cfg = ReasonerConfig {
            combat_window_seconds: 16.0,
            ..ReasonerConfig::default()
        };
        let mut burst = hero();
        burst.weapon.bullet_damage = 0.0;
        burst.abilities = vec![ability(1, 16.0, 1000.0)];
        let mut periodic = burst.clone();
        periodic.abilities[0].properties = BTreeMap::from([
            ("DamagePerSecond".into(), 1.0),
            ("AbilityCooldown".into(), 1000.0),
        ]);
        periodic.abilities[0].duration = Some(16.0);
        periodic.abilities[0].tick_rate = Some(0.2);
        let item = refresh_regeneration();
        let delta = |hero: &HeroModel| {
            evaluate_inventory(hero, std::slice::from_ref(&item), &cfg).scenarios[1]
                .effective_health
                - evaluate_inventory(hero, &[], &cfg).scenarios[1].effective_health
        };
        let burst_gain = delta(&burst);
        let periodic_gain = delta(&periodic);
        assert!(
            burst_gain > 0.0,
            "burst must trigger a finite-lived buff: {burst_gain}"
        );
        assert!(
            periodic_gain > burst_gain,
            "refreshes must extend useful healing: {periodic_gain} <= {burst_gain}"
        );
        periodic.abilities[0].item_proc_disabled = true;
        assert!(
            delta(&periodic).abs() < 1e-9,
            "proc-disabled damage cannot grant regeneration"
        );
        burst.abilities.clear();
        assert!(
            delta(&burst).abs() < 1e-9,
            "no spirit event must mean no regeneration"
        );
    }

    #[test]
    fn sustain_refresh_trace_scoring_and_fast_path_agree() {
        let cfg = ReasonerConfig {
            combat_window_seconds: 16.0,
            ..ReasonerConfig::default()
        };
        let mut hero = hero();
        hero.weapon.bullet_damage = 0.0;
        hero.abilities = vec![ability(1, 16.0, 1000.0)];
        let item = refresh_regeneration();
        let detailed = evaluate_inventory(&hero, std::slice::from_ref(&item), &cfg);
        let fast = evaluate_inventory_fast(&hero, std::slice::from_ref(&item), &cfg);
        assert_eq!(detailed.score, fast.score);
        assert_eq!(
            detailed.scenarios,
            fast.scenarios
                .iter()
                .zip(&detailed.scenarios)
                .map(|(f, d)| {
                    let mut f = f.clone();
                    f.sequence = d.sequence.clone();
                    f
                })
                .collect::<Vec<_>>()
        );
        assert_eq!(
            detailed.scenarios[0]
                .item_regeneration
                .get(&item.item_id)
                .copied()
                .unwrap_or(0.0),
            0.0
        );
        assert!(detailed.scenarios[1].item_regeneration[&item.item_id] > 0.0);
        let summary = damage_refresh_summary(&item, &hero, &cfg).unwrap();
        assert!(summary.0 > 0.0 && summary.0 < 1.0);
        let scored = crate::item::score_item(
            &item,
            &hero,
            &crate::MetaIndex {
                by_item: BTreeMap::new(),
                sample_ok: BTreeSet::new(),
            },
            &[],
            &cfg,
        );
        assert_eq!(scored.score.condition_factor, summary.0);
        assert!(scored.score.combat_value > 0.0);
        hero.damage_plan.weapon_share = 0.0;
        assert_eq!(
            summary,
            damage_refresh_summary(&item, &hero, &cfg).unwrap(),
            "a weapon-share label cannot replace actual events"
        );
        let mut translated = item.clone();
        translated.description.clear();
        assert_eq!(
            item.condition,
            crate::item::build_item_model(&translated)
                .unwrap()
                .condition
        );
    }

    #[test]
    fn target_death_ends_duel_and_does_not_reset_own_cooldown() {
        let mut hero = hero();
        hero.weapon.bullet_damage = 0.0;
        hero.abilities = vec![ability(1, 1000.0, 10.0)];
        let cfg = ReasonerConfig {
            combat_window_seconds: 4.0,
            ..ReasonerConfig::default()
        };
        let result = evaluate_inventory(&hero, &[], &cfg);
        let duel = &result.scenarios[0];
        let chain = &result.scenarios[1];
        assert_eq!(duel.ability_damage, 600.0);
        assert_eq!(duel.first_ttk, Some(0.2));
        assert_eq!(duel.elapsed_seconds, 0.2);
        assert_eq!(duel.damage_per_second, 3000.0);
        assert_eq!(duel.effective_health, 30.0);
        assert!((duel.survival_score - 7.5).abs() < 1e-8);
        assert_eq!(chain.ability_damage, 600.0);
        assert_eq!(chain.casts[&1], 1);
        assert_eq!(chain.target_switches, 1);
        assert_eq!(chain.elapsed_seconds, 4.0);
        assert_eq!(
            result.score,
            evaluate_inventory_fast(&hero, &[], &cfg).score
        );
    }
    #[test]
    fn faster_kills_earn_damage_credit_but_not_survival_credit() {
        let mut slow = hero();
        slow.weapon.bullet_damage = 50.0;
        let mut fast = hero();
        fast.weapon.bullet_damage = 150.0;
        let cfg = ReasonerConfig {
            combat_window_seconds: 20.0,
            ..ReasonerConfig::default()
        };
        let slow = &evaluate_inventory(&slow, &[], &cfg).scenarios[0];
        let fast = &evaluate_inventory(&fast, &[], &cfg).scenarios[0];
        assert_eq!(slow.end_reason, CombatEndReason::TargetDefeated);
        assert_eq!(fast.end_reason, CombatEndReason::TargetDefeated);
        assert!((slow.elapsed_seconds - 2.4).abs() < 1e-8);
        assert!((fast.elapsed_seconds - 0.8).abs() < 1e-8);
        assert!((slow.damage_per_second - 250.0).abs() < 1e-8);
        assert!((fast.damage_per_second - 750.0).abs() < 1e-8);
        assert!((slow.effective_health - 72.0).abs() < 1e-8);
        assert!((fast.effective_health - 24.0).abs() < 1e-8);
        assert!(fast.survival_score < slow.survival_score);
        assert!(fast.damage_score > slow.damage_score);
    }
    #[test]
    fn dead_target_has_no_dot_or_lifesteal_and_new_targets_reset_health_threshold() {
        let mut remaining = 40.0;
        let mut events = vec![];
        let mut leech = 0.0;
        let stats = Stats {
            spirit_leech: 50.0,
            ..Stats::default()
        };
        assert_eq!(
            damage_event(
                100.0,
                &DamageType::Spirit,
                &stats,
                1.0,
                0.0,
                &mut events,
                &mut leech,
                &mut remaining
            ),
            40.0
        );
        assert_eq!(
            damage_event(
                100.0,
                &DamageType::Spirit,
                &stats,
                1.0,
                1.0,
                &mut events,
                &mut leech,
                &mut remaining
            ),
            0.0
        );
        assert_eq!(leech, 20.0);
        assert_eq!(events.len(), 1);
        let mut hero = hero();
        hero.weapon.bullet_damage = 0.0;
        let mut dot = ability(1, 1000.0, 20.0);
        dot.duration = Some(4.0);
        dot.tick_rate = Some(0.2);
        hero.abilities = vec![dot];
        let result = evaluate_inventory(
            &hero,
            &[],
            &ReasonerConfig {
                combat_window_seconds: 6.0,
                ..ReasonerConfig::default()
            },
        );
        assert_eq!(result.scenarios[1].ability_damage, 600.0);
        assert_eq!(result.scenarios[1].targets_defeated, 1);
        hero.abilities.clear();
        hero.weapon.bullet_damage = 100.0;
        hero.weapon.shots_per_second = 10.0;
        hero.weapon.clip_size = 1000.0;
        let mut opening = item(1, "WeaponPower", 100.0);
        opening.properties.insert("EnemyLifeThreshold".into(), 90.0);
        opening.conditional_properties.insert("WeaponPower".into());
        let result = evaluate_inventory(
            &hero,
            &[opening],
            &ReasonerConfig {
                combat_window_seconds: 3.4,
                ..ReasonerConfig::default()
            },
        );
        let chain = &result.scenarios[1];
        assert_eq!(chain.targets_defeated, 3);
        assert!((chain.kill_times[1] - chain.kill_times[0] - 1.4).abs() < 1e-8);
    }
    #[test]
    fn own_pulse_aura_continues_without_damage_during_target_switch_pause() {
        let mut hero = hero();
        hero.weapon.bullet_damage = 0.0;
        let mut aura = ability(1, 12000.0, 20.0);
        aura.duration = Some(4.0);
        aura.tick_rate = Some(0.2);
        aura.properties.insert("PulseDPS".into(), 3000.0);
        aura.properties.insert("Radius".into(), 10.0);
        hero.abilities = vec![aura];
        let result = evaluate_inventory(
            &hero,
            &[],
            &ReasonerConfig {
                combat_window_seconds: 5.0,
                ..ReasonerConfig::default()
            },
        );
        let chain = &result.scenarios[1];
        assert_eq!(chain.casts[&1], 1);
        assert_eq!(chain.targets_defeated, 4);
        assert_eq!(chain.ability_damage, 2400.0);
        assert!(chain.contact_seconds < chain.elapsed_seconds - 3.0);
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
        assert!(result.scenarios[1].elapsed_seconds < cfg.combat_window_seconds);
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
    fn earlier_self_defeat_does_not_reward_identical_capped_damage() {
        let mut hero = hero();
        hero.base_health = 600.0;
        hero.weapon.bullet_damage = 0.0;
        let mut bomb = ability(99, 1000.0, 100.0);
        bomb.class_name = "ability_blood_bomb".into();
        bomb.properties.insert("SelfDamagePct".into(), 30.0);
        hero.abilities = vec![bomb];
        let cfg = ReasonerConfig {
            combat_window_seconds: 40.0,
            ..ReasonerConfig::default()
        };
        let lower_cost = evaluate_inventory(&hero, &[], &cfg);
        hero.abilities[0].base_effect = 1800.0;
        let higher_cost = evaluate_inventory(&hero, &[], &cfg);
        let low = &lower_cost.scenarios[1];
        let high = &higher_cost.scenarios[1];
        assert_eq!(low.end_reason, CombatEndReason::SelfDefeated);
        assert_eq!(high.end_reason, CombatEndReason::SelfDefeated);
        assert_eq!(low.ability_damage, 600.0);
        assert_eq!(high.ability_damage, 600.0);
        assert!(high.elapsed_seconds < low.elapsed_seconds);
        assert!(high.damage_per_second > low.damage_per_second);
        assert_eq!(high.damage_score, 15.0);
        assert_eq!(high.damage_score, low.damage_score);
        assert!(high.survival_score < low.survival_score);
        assert!(higher_cost.score < lower_cost.score);
        assert_eq!(
            higher_cost.score,
            evaluate_inventory_fast(&hero, &[], &cfg).score
        );
        assert_eq!(
            higher_cost.scenarios[0].end_reason,
            CombatEndReason::TargetDefeated
        );
        assert_eq!(
            higher_cost.scenarios[0].damage_score,
            higher_cost.scenarios[0].damage_per_second
        );
    }
    #[test]
    fn magazine_imbue_survives_target_switch_but_not_reload() {
        let mut hero = hero();
        hero.base_health = 100.0;
        hero.weapon.bullet_damage = 100.0;
        hero.weapon.shots_per_second = 5.0;
        hero.weapon.clip_size = 100.0;
        hero.abilities = vec![ability(10, 0.0, 100.0)];
        let mut magazine = item(7, "BulletsBonusMagicDamage", 25.0);
        magazine.imbueable = true;
        magazine
            .property_damage_types
            .insert("BulletsBonusMagicDamage".into(), DamageType::Spirit);
        let bindings = BTreeMap::from([(7, 10)]);
        let cfg = ReasonerConfig {
            combat_window_seconds: 4.0,
            ..ReasonerConfig::default()
        };
        let result = evaluate_inventory_with_bindings(
            &hero,
            std::slice::from_ref(&magazine),
            &cfg,
            &bindings,
        );
        let chain = &result.scenarios[1];
        assert!(chain.target_switches > 0);
        assert_eq!(chain.casts[&10], 1);
        assert!((chain.proc_damage - chain.targets_defeated as f64 * 20.0).abs() < 1e-8);
        assert_eq!(result.scenarios[0].weapon_damage, 80.0);
        assert_eq!(result.scenarios[0].proc_damage, 20.0);
        hero.weapon.clip_size = 1.0;
        let reload = evaluate_inventory_with_bindings(
            &hero,
            std::slice::from_ref(&magazine),
            &cfg,
            &bindings,
        );
        assert!(reload.scenarios[1].reloads > 0);
        assert!(reload.scenarios[1].proc_damage < chain.proc_damage);
        let wrong = evaluate_inventory_with_bindings(
            &hero,
            std::slice::from_ref(&magazine),
            &cfg,
            &BTreeMap::from([(7, 11)]),
        );
        assert_eq!(wrong.proc_damage, 0.0);
        assert_eq!(
            result.score,
            evaluate_inventory_refs_fast_with_bindings(
                &{
                    let mut h = hero.clone();
                    h.weapon.clip_size = 100.0;
                    h
                },
                &[&magazine],
                &cfg,
                &bindings
            )
            .score
        );
    }
    #[test]
    fn aura_range_changes_only_its_own_two_effects() {
        let hero = hero();
        let cfg = ReasonerConfig {
            combat_window_seconds: 2.0,
            ..ReasonerConfig::default()
        };
        let mut aura = item(5, "BulletArmorReduction", -10.0);
        aura.properties.extend([
            ("SingleTargetPlayerMultiplier".into(), 2.0),
            ("FireRateSlow".into(), 15.0),
            ("Radius".into(), 15.0),
        ]);
        let plain = evaluate_inventory(&hero, &[], &cfg);
        let result = evaluate_inventory(&hero, std::slice::from_ref(&aura), &cfg);
        assert!(result.scenarios[0].weapon_damage > plain.scenarios[0].weapon_damage);
        assert!(result.scenarios[0].effective_health > plain.scenarios[0].effective_health);
        assert_eq!(
            result.scenarios[2].weapon_damage,
            plain.scenarios[2].weapon_damage
        );
        assert_eq!(
            result.scenarios[2].effective_health,
            plain.scenarios[2].effective_health
        );
        let weakened = evaluate_inventory(
            &hero,
            &[aura.clone(), item(6, "WeaponPowerDebuff", -25.0)],
            &cfg,
        );
        assert!(
            (weakened.scenarios[0].effective_health - 600.0 / (0.5 * 0.75 * 0.7 + 0.5)).abs()
                < 1e-8
        );
        aura.properties.insert("Radius".into(), 7.0);
        let outside = evaluate_inventory(&hero, &[aura], &cfg);
        assert_eq!(outside.weapon_damage, plain.weapon_damage);
        assert_eq!(outside.effective_health, plain.effective_health);
    }
    #[test]
    fn magazine_and_reload_share_the_same_imbue_item_cooldown() {
        let mut hero = hero();
        hero.base_health = 10000.0;
        hero.weapon.bullet_damage = 100.0;
        hero.weapon.clip_size = 1.0;
        hero.weapon.shots_per_second = 5.0;
        hero.weapon.reload_duration = 0.2;
        hero.abilities = vec![ability(10, 1.0, 1.0)];
        let mut magazine = item(7, "BulletsBonusMagicDamage", 25.0);
        magazine.imbueable = true;
        magazine.properties.extend([
            ("AmmoReloadPercent".into(), 100.0),
            ("AbilityCooldown".into(), 15.0),
        ]);
        magazine
            .property_damage_types
            .insert("BulletsBonusMagicDamage".into(), DamageType::Spirit);
        let result = evaluate_inventory_with_bindings(
            &hero,
            &[magazine],
            &ReasonerConfig {
                combat_window_seconds: 4.0,
                ..ReasonerConfig::default()
            },
            &BTreeMap::from([(7, 10)]),
        );
        assert!(result.scenarios[0].casts[&10] > 1);
        assert_eq!(result.scenarios[0].item_activations[&7], vec![0.0]);
        assert_eq!(result.scenarios[0].proc_damage, 25.0);
    }
    #[test]
    fn hook_hit_amp_and_uppercut_reset_drive_real_casts() {
        let mut hero = hero();
        hero.base_health = 10000.0;
        let mut hook = ability(10, 0.0, 100.0);
        hook.class_name = "citadel_ability_hook".into();
        hook.properties.extend([
            ("BulletAmp".into(), 20.0),
            ("BulletAmpDuration".into(), 6.0),
        ]);
        let mut uppercut = ability(11, 1.0, 100.0);
        uppercut.class_name = "citadel_ability_uppercut".into();
        uppercut.properties.extend([
            ("RestoreHookCooldown".into(), 1.0),
            ("MissingHPHeal".into(), 18.0),
        ]);
        hero.abilities = vec![hook, uppercut];
        let cfg = ReasonerConfig {
            combat_window_seconds: 4.0,
            ..ReasonerConfig::default()
        };
        let result = evaluate_inventory(&hero, &[], &cfg);
        assert_eq!(result.scenarios[0].casts[&10], 2);
        assert_eq!(result.scenarios[0].casts[&11], 1);
        hero.abilities[1].properties.remove("RestoreHookCooldown");
        let no_reset = evaluate_inventory(&hero, &[], &cfg);
        assert_eq!(no_reset.scenarios[0].casts[&10], 1);
        hero.abilities.clear();
        assert!(result.weapon_damage > evaluate_inventory(&hero, &[], &cfg).weapon_damage);
    }
    #[test]
    fn malice_hits_amplify_following_damage_with_fast_path_parity() {
        let mut hero = hero();
        hero.base_health = 10000.0;
        let mut malice = ability(10, 30.0, 1.0);
        malice.class_name = "ability_blood_shards".into();
        malice.properties.extend([
            ("VulnerabilityPerStack".into(), 15.0),
            ("DebuffDuration".into(), 9.0),
            ("MaxStacks".into(), 5.0),
        ]);
        hero.abilities = vec![malice];
        let cfg = ReasonerConfig {
            combat_window_seconds: 4.0,
            ..ReasonerConfig::default()
        };
        let mut full = evaluate_inventory(&hero, &[], &cfg);
        let fast = evaluate_inventory_fast(&hero, &[], &cfg);
        for scenario in &mut full.scenarios {
            scenario.sequence.clear();
        }
        assert_eq!(full.scenarios, fast.scenarios);
        hero.abilities[0].properties.remove("VulnerabilityPerStack");
        let plain = evaluate_inventory(&hero, &[], &cfg);
        assert!(full.weapon_damage > plain.weapon_damage);
        assert!(full.ability_damage > plain.ability_damage);
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
        let result = evaluate_inventory(
            &hero(),
            &[loss],
            &ReasonerConfig {
                combat_window_seconds: 5.0,
                ..ReasonerConfig::default()
            },
        );
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
    fn charged_item_bonus_charges_do_not_affect_ordinary_abilities() {
        let mut hero = hero();
        hero.weapon.bullet_damage = 0.0;
        hero.base_health = 100_000.0;
        let mut spell = ability(1, 10.0, 1000.0);
        spell.charges = 1;
        hero.abilities = vec![spell];
        let cfg = ReasonerConfig {
            combat_window_seconds: 5.0,
            ..Default::default()
        };
        let bonus = item(1, "BonusAbilityCharges", 2.0);
        let plain = evaluate_inventory(&hero, &[], &cfg);
        let more = evaluate_inventory(&hero, std::slice::from_ref(&bonus), &cfg);
        assert_eq!(plain.scenarios[0].casts[&1], 1);
        assert_eq!(more.scenarios[0].casts[&1], 3);
        hero.abilities[0].charges = 0;
        let ordinary = evaluate_inventory(&hero, &[bonus], &cfg);
        assert_eq!(ordinary.scenarios[0].casts[&1], 1);
    }

    #[test]
    fn charged_item_cooldowns_and_spacing_reach_cast_events() {
        let mut hero = hero();
        hero.weapon.bullet_damage = 0.0;
        hero.base_health = 100_000.0;
        let mut spell = ability(1, 10.0, 10.0);
        spell.charges = 1;
        hero.abilities = vec![spell];
        let cfg = ReasonerConfig {
            combat_window_seconds: 25.0,
            ..Default::default()
        };
        let bonus = item(1, "CooldownReductionOnChargedAbilities", 50.0);
        let plain = evaluate_inventory(&hero, &[], &cfg);
        let more = evaluate_inventory(&hero, &[bonus], &cfg);
        assert_eq!(plain.scenarios[0].casts[&1], 3);
        assert_eq!(more.scenarios[0].casts[&1], 5);
        hero.abilities[0].charges = 3;
        hero.abilities[0].cooldown = 1000.0;
        hero.abilities[0]
            .properties
            .insert("AbilityCooldown".into(), 1000.0);
        hero.abilities[0]
            .properties
            .insert("AbilityCooldownBetweenCharge".into(), 4.0);
        let short = ReasonerConfig {
            combat_window_seconds: 3.0,
            ..Default::default()
        };
        let faster = item(1, "CooldownBetweenChargeReduction", 50.0);
        assert_eq!(
            evaluate_inventory(&hero, &[], &short).scenarios[0].casts[&1],
            1
        );
        assert_eq!(
            evaluate_inventory(&hero, &[faster], &short).scenarios[0].casts[&1],
            2
        );
    }

    #[test]
    fn charged_item_spirit_is_not_global_spirit() {
        let mut hero = hero();
        hero.weapon.bullet_damage = 0.0;
        hero.base_health = 100_000.0;
        let mut spell = ability(1, 10.0, 1000.0);
        spell.charges = 1;
        spell.scaling.push(crate::ScalingStat {
            stat: "Damage".into(),
            per_level: 0.0,
            per_spirit: Some(0.5),
        });
        hero.abilities = vec![spell];
        let cfg = ReasonerConfig {
            combat_window_seconds: 5.0,
            ..Default::default()
        };
        let bonus = item(1, "BonusSpiritForChargedAbilities", 14.0);
        let charged = evaluate_inventory(&hero, std::slice::from_ref(&bonus), &cfg);
        assert!((charged.scenarios[0].ability_damage - 17.0).abs() < 1e-9);
        hero.abilities[0].charges = 0;
        let ordinary = evaluate_inventory(&hero, std::slice::from_ref(&bonus), &cfg);
        assert!((ordinary.scenarios[0].ability_damage - 10.0).abs() < 1e-9);
        hero.abilities.clear();
        hero.weapon.bullet_damage = 10.0;
        hero.scaling.push(crate::ScalingStat {
            stat: "ERoundsPerSecond".into(),
            per_level: 0.0,
            per_spirit: Some(0.5),
        });
        let plain = evaluate_inventory(&hero, &[], &cfg);
        let equipped = evaluate_inventory(&hero, &[bonus], &cfg);
        assert_eq!(plain.weapon_damage, equipped.weapon_damage);
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
                &mut leech,
                &mut 1000.0,
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
    fn spirit_weapon_axes_match_materialized_weapon_in_combat() {
        let cfg = ReasonerConfig::default();
        for (axis, scale) in [
            ("EBulletDamage", 0.08),
            ("EClipSize", 0.5),
            ("ERoundsPerSecond", 0.01),
        ] {
            let mut hero = hero();
            hero.scaling = vec![crate::ScalingStat {
                stat: axis.into(),
                per_level: 0.0,
                per_spirit: Some(scale),
            }];
            let spirit = item(1, "TechPower", 100.0);
            let evaluated = evaluate_inventory(&hero, std::slice::from_ref(&spirit), &cfg);
            let mut materialized = hero.clone();
            materialized.weapon = crate::mechanics::weapon_with_spirit(&hero, 100.0);
            materialized.scaling.clear();
            let reference = evaluate_inventory(&materialized, std::slice::from_ref(&spirit), &cfg);
            assert!(
                (evaluated.weapon_damage - reference.weapon_damage).abs() < 1e-9,
                "Kampfpfad {axis}"
            );
            for (a, b) in evaluated.scenarios.iter().zip(&reference.scenarios) {
                assert_eq!(a.reloads, b.reloads, "Nachladen {axis}");
                assert!((a.shots - b.shots).abs() < 1e-9, "Schusszahl {axis}");
            }
            let fast = evaluate_inventory_fast(&hero, std::slice::from_ref(&spirit), &cfg);
            assert!((evaluated.score - fast.score).abs() < 1e-9);
        }
    }

    #[test]
    fn spirit_does_not_change_weapon_rate_for_a_null_converter() {
        // Basis-Held ohne ERoundsPerSecond/EFireRate: mehr Spirit erzeugt keinen
        // Waffen-DPS (Gegenprobe zu spirit_changes_whole_weapon_rate_...).
        let hero = hero();
        assert!(!hero
            .scaling
            .iter()
            .any(|s| s.stat == "ERoundsPerSecond" || s.stat == "EFireRate"));
        let cfg = ReasonerConfig::default();
        let spirit = item(1, "TechPower", 100.0);
        let plain = evaluate_inventory(&hero, &[], &cfg);
        let with_spirit = evaluate_inventory(&hero, std::slice::from_ref(&spirit), &cfg);
        assert!((with_spirit.weapon_damage - plain.weapon_damage).abs() < 1e-9);
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
        hero.base_health = 2000.0;
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
