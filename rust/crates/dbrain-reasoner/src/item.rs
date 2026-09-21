use crate::mechanics;
use crate::{
    BuyPhase, Confidence, DamageType, HeroModel, ItemModel, ItemScore, MetaIndex, PatchDelta,
    ReasonerConfig, ReasonerError, Result, ScoredItem,
};

pub fn build_item_model(loaded: &ItemModel) -> Result<ItemModel> {
    if loaded.item_id == 0 || loaded.name.trim().is_empty() {
        return Err(ReasonerError::Data(
            "Geladenes Item-Modell enthält keine ID oder keinen Namen".to_string(),
        ));
    }
    let mut loaded = loaded.clone();
    if !loaded.is_active {
        let mut properties = loaded.passive_properties.clone();
        properties.extend(
            loaded
                .properties
                .iter()
                .map(|(key, value)| (key.clone(), *value)),
        );
        if matches!(
            loaded.condition,
            crate::ConditionKind::SpiritDamageToHeroes { .. }
        ) {
            // A normalized mechanism must remain invariant under display-text changes.
        } else if let Some(condition) =
            crate::damage_conditions::spirit_refresh_condition(&properties, &loaded.description)
        {
            loaded.condition = condition;
        } else if crate::item_interactions::has_nearby_aura(&loaded) {
            loaded.condition = crate::ConditionKind::None;
        } else if let Some(condition) = crate::data::condition_from_properties(&loaded.properties) {
            loaded.condition = condition;
        }
    }
    Ok(loaded)
}

pub fn score_item(
    item: &ItemModel,
    hero: &HeroModel,
    meta: &MetaIndex,
    deltas: &[PatchDelta],
    cfg: &ReasonerConfig,
) -> ScoredItem {
    if item.disabled || !item.shopable {
        return ScoredItem {
            item: item.clone(),
            score: ItemScore {
                combat_value: 0.0,
                per_slot_value: 0.0,
                per_soul_value: 0.0,
                purchase_bonus_value: 0.0,
                condition_factor: 0.0,
                active_value: 0.0,
                passive_value: 0.0,
                meta_support: 0.0,
                total: 0.0,
            },
            confidence: Confidence::Low,
            buy_phase: BuyPhase::Late,
            sources: Vec::new(),
        };
    }
    let refresh = crate::combat::damage_refresh_summary(item, hero, cfg);
    let condition_factor = refresh
        .map(|(factor, _)| factor)
        .unwrap_or_else(|| mechanics::condition_factor_for_hero(item, hero, cfg));
    // Provenienz der Spirit->Feuerrate-Konversion einmal an der Modellgrenze
    // ermitteln (nicht pro Tick), um unbekannte/fehlerhafte Assetwerte sichtbar
    // im Confidence-/Assumptions-Pfad zu halten.
    let rate_provenance = mechanics::spirit_weapon_rate_provenance(hero);
    let scaling_warnings = mechanics::hero_scaling_warnings(hero);
    let fire_rate = spirit_fire_rate_value(item, hero, cfg);
    let active_value = mechanics::active_value(item, hero, cfg)
        + fire_rate.active_dps
        + refresh.map_or(
            0.0,
            |(factor, healing)| if factor > 0.0 { healing / factor } else { 0.0 },
        );
    let passive_value = mechanics::passive_value(item, hero, cfg) + fire_rate.passive_dps;
    let combat_value = passive_value + active_value * condition_factor;
    let purchase_bonus_value =
        mechanics::purchase_bonus_value_with_config(item, hero, cfg) + fire_rate.purchase_dps;
    let per_slot_value = mechanics::per_slot_value(combat_value, purchase_bonus_value);
    let per_soul_value = mechanics::per_soul_value(per_slot_value, item.cost);
    let meta_support = meta_value(item.item_id, meta);
    let patch_support = patch_value(item, hero, deltas);
    let total = per_slot_value + meta_support + patch_support;
    let unmeasured = item.properties.iter().any(|(name, value)| {
        *value != 0.0
            && (name.contains("OnKill")
                || name.starts_with("ParrySuccess")
                || name.contains("HealAmpReceivePenalty")
                || name.contains("HealAmpRegenPenalty"))
    });
    // Ohne validierten Ersatz darf aus nicht endlichen Konversionsdaten keine
    // bestaetigte Konversion mit hoher Sicherheit entstehen.
    let confidence = if meta.sample_ok.contains(&item.item_id)
        && !unmeasured
        && !rate_provenance.unknown_nonfinite
        && scaling_warnings.is_empty()
    {
        Confidence::High
    } else {
        Confidence::Low
    };
    let mut sources: Vec<crate::Evidence> = scaling_warnings
        .into_iter()
        .map(|detail| crate::Evidence {
            kind: crate::EvidenceKind::Mechanic,
            detail,
        })
        .collect();
    if rate_provenance.unknown_nonfinite {
        sources.push(crate::Evidence {
            kind: crate::EvidenceKind::Mechanic,
            detail: "Spirit → Feuerrate: nicht endliche Konversionsdaten im Helden-Snapshot und kein valider Ersatz; die Konversion gilt als unbekannt (0), keine bestätigte Null-Konversion mit hoher Sicherheit.".into(),
        });
    } else if rate_provenance.recovered_from_nonfinite {
        sources.push(crate::Evidence {
            kind: crate::EvidenceKind::Mechanic,
            detail: "Spirit → Feuerrate: primärer ERoundsPerSecond nicht endlich; Konversion aus validem EFireRate-Prozentalias zurückgewonnen und als Recovery gekennzeichnet.".into(),
        });
    }
    if let Some((factor, healing)) = refresh {
        sources.push(crate::Evidence {
            kind: crate::EvidenceKind::Mechanic,
            detail: format!("Spirit-Treffer auf Helden → zielgebundener Refresh → {:.6} mittlere aktive Stapel → {:.6} nutzbare Heilung/s im begrenzten Vergleich. Keine Auslösung durch proc-ausgeschlossene Treffer, keine gespeicherte Überheilung; Inventarzusammenspiel wird im Planner erneut simuliert.", factor, healing),
        });
    }
    if fire_rate.weapon_dps_in_score != 0.0 {
        sources.push(crate::Evidence { kind: crate::EvidenceKind::Mechanic, detail: format!("Spirit Power → Waffenwerte aus Assets-Snapshot: {} Item-SP + {} Kaufbonus-SP → {:+.6} Schaden/Schuss, {:+.6} Magazin, {:+.6} Schuss/s → {:+.6} Waffen-DPS nach Nachladezyklus und Zustandsfaktor. EBulletDamage/EClipSize als absolute Zielstats; ERoundsPerSecond direkt, EFireRate nur Prozent-Fallback, nie beide addiert.", fire_rate.spirit_power, fire_rate.purchase_spirit_power, fire_rate.bullet_damage, fire_rate.clip_size, fire_rate.rounds_per_second, fire_rate.weapon_dps_in_score) });
    }
    if combat_value > 0.0 {
        sources.push(crate::Evidence {
            kind: crate::EvidenceKind::Mechanic,
            detail: format!("Kampffenster {} s, Kanalanteil {}; Slot-Wert in Schaden oder effektivem Leben pro Sekunde. total nutzt den Slot-Wert; frühe Käufe nutzen per_soul_value.",cfg.combat_window_seconds,cfg.channel_uptime),
        });
    }
    if matches!(item.condition, crate::ConditionKind::StateBound { .. }) {
        sources.push(crate::Evidence { kind:crate::EvidenceKind::Mechanic, detail:"Annahme für hit_rate: gleichverteiltes Restleben zwischen 0 und 100 Prozent, keine gemessene Trefferquote.".into() });
    }
    sources.push(crate::Evidence { kind:crate::EvidenceKind::Mechanic,detail:"Kaufreihenfolge nach Kostenband: günstige Anfangskäufe, Aufbau, Kern und Spätspiel. Nahkampf und Bewegung werden aus dem Heldenmodell abgeleitet; ein Kaufbonus bleibt auch bei bedingter Itemwirkung erhalten.".into() });
    if item.properties.keys().any(|name| {
        name.contains("OnKill")
            || name.starts_with("ParrySuccess")
            || name.contains("HealAmpReceivePenalty")
            || name.contains("HealAmpRegenPenalty")
    }) {
        sources.push(crate::Evidence {kind:crate::EvidenceKind::Mechanic,detail:"Bedingte Heilung nach Kills oder Paraden und verhinderte Gegnerheilung sind ohne Kampf- und Gegnerdaten nicht quantifiziert. Diese Wirkung wird nicht als garantierte Dauerheilung eingerechnet.".into()});
    }
    if item.properties.contains_key("DamageThreshold")
        && item.properties.contains_key("ImmunityDuration")
    {
        sources.push(crate::Evidence {kind:crate::EvidenceKind::Mechanic,detail:"Auslösung aus Spirit-Schaden im vorgegebenen Zeitfenster abgeschätzt. Der periodische Brandwert ist eine Obergrenze je Ziel; ohne erreichte Schadensschwelle wird kein garantierter Proc angenommen.".into()});
    }
    if item.properties.contains_key("HealthStealPctHero")
        || item.passive_properties.contains_key("HealthStealPctHero")
    {
        sources.push(crate::Evidence { kind:crate::EvidenceKind::Mechanic, detail:"Max-HP-Entzug zählt Verlust am Ziel und eigenen Lebensgewinn. Annahme: Ziel hat das Basisleben des Helden; keine Gegnerdaten vorhanden. Bewertung als Bruttowirkung der Procs im Fenster, ohne Rückgabe nach Debuff-Ende.".into() });
    }
    if meta_support != 0.0 {
        sources.push(crate::Evidence {
            kind: crate::EvidenceKind::Meta,
            detail: "Meta als Nebensignal".to_string(),
        });
    }
    if patch_support != 0.0 {
        sources.push(crate::Evidence {
            kind: crate::EvidenceKind::Patch,
            detail: "Patch-Delta".to_string(),
        });
    }
    ScoredItem {
        item: item.clone(),
        score: ItemScore {
            combat_value,
            per_slot_value,
            per_soul_value,
            purchase_bonus_value,
            condition_factor,
            active_value,
            passive_value,
            meta_support,
            total,
        },
        confidence,
        buy_phase: mechanics::buy_phase_for_hero(item, hero),
        sources,
    }
}

#[derive(Debug, serde::Serialize)]
pub struct SpiritFireRateValue {
    pub spirit_power: f64,
    pub purchase_spirit_power: f64,
    pub rounds_per_second: f64,
    pub bullet_damage: f64,
    pub clip_size: f64,
    pub active_dps: f64,
    pub passive_dps: f64,
    pub purchase_dps: f64,
    pub weapon_dps_in_score: f64,
}

pub fn spirit_fire_rate_value(
    item: &ItemModel,
    hero: &HeroModel,
    cfg: &ReasonerConfig,
) -> SpiritFireRateValue {
    // Einzige Quelle der Spirit->Feuerrate-Konversion (siehe mechanics); keine
    // eigene Ableitung mehr im Score-Pfad. Endliche negative Skalen bleiben
    // signiert (Downside), nicht endliche werden verworfen.
    let per_spirit = mechanics::spirit_weapon_rate_per_spirit(hero);
    let is_spirit = |name: &str| {
        [
            "TechPower",
            "SpiritPower",
            "BonusSpirit",
            "BonusSpiritPower",
            "SpiritPowerInnate",
        ]
        .iter()
        .any(|alias| name.eq_ignore_ascii_case(alias))
    };
    let is_conditional = |name: &str| {
        item.conditional_properties.contains(name)
            || (item.is_active && !item.passive_properties.contains_key(name))
    };
    let mut properties = item.passive_properties.clone();
    properties.extend(
        item.properties
            .iter()
            .map(|(name, value)| (name.clone(), *value)),
    );
    let passive_spirit = properties
        .iter()
        .filter(|(name, value)| is_spirit(name) && value.is_finite() && !is_conditional(name))
        .map(|(_, value)| value)
        .sum::<f64>();
    let active_spirit = properties
        .iter()
        .filter(|(name, value)| is_spirit(name) && value.is_finite() && is_conditional(name))
        .map(|(_, value)| value)
        .sum::<f64>();
    let bonus = if item.slot == crate::SlotType::Spirit {
        hero.purchase_bonuses
            .spirit
            .iter()
            .find(|bonus| bonus.tier == item.tier)
            .map_or(0.0, |bonus| bonus.value)
    } else {
        0.0
    };
    // Grenzwert relativ zum realen Basiszustand: innewohnender base_spirit_power
    // fliesst wie in damage_plan/combat genau einmal ein, der Item-Zuwachs kommt
    // obendrauf. Die weapon_dps-Formel ist nichtlinear (Rate wirkt auf Zyklus und
    // Reload); ein Grenzwert relativ zur nackten Waffe waere falsch.
    let dps = |item_spirit: f64| {
        let mut weapon = mechanics::weapon_with_spirit(hero, hero.base_spirit_power + item_spirit);
        weapon.sustained_dps = 0.0;
        mechanics::weapon_dps(&weapon, cfg.combat_window_seconds)
    };
    let factor = mechanics::condition_factor_for_hero(item, hero, cfg);
    let passive_dps = dps(passive_spirit) - dps(0.0);
    let active_dps = dps(passive_spirit + active_spirit) - dps(passive_spirit);
    let purchase_dps = factor
        * (dps(passive_spirit + active_spirit + bonus) - dps(passive_spirit + active_spirit))
        + (1.0 - factor) * (dps(passive_spirit + bonus) - dps(passive_spirit));
    SpiritFireRateValue {
        spirit_power: passive_spirit + active_spirit,
        purchase_spirit_power: bonus,
        rounds_per_second: (passive_spirit + active_spirit + bonus) * per_spirit,
        bullet_damage: (passive_spirit + active_spirit + bonus)
            * mechanics::weapon_spirit_scaling(hero).bullet_damage,
        clip_size: (passive_spirit + active_spirit + bonus)
            * mechanics::weapon_spirit_scaling(hero).clip_size,
        active_dps,
        passive_dps,
        purchase_dps,
        weapon_dps_in_score: passive_dps + active_dps * factor + purchase_dps,
    }
}

pub const CORE_SCORE_QUANTILE: f64 = 0.5;

pub fn core_score_threshold(scored: &[ScoredItem], cost: i64) -> Option<f64> {
    let mut values = scored
        .iter()
        .filter(|item| item.item.cost == cost && item.item.shopable && !item.item.disabled)
        .map(|item| item.score.per_slot_value)
        .filter(|value| value.is_finite())
        .collect::<Vec<_>>();
    values.sort_by(f64::total_cmp);
    if values.is_empty() {
        return None;
    }
    let position = (values.len() - 1) as f64 * CORE_SCORE_QUANTILE;
    let lower = position.floor() as usize;
    let upper = position.ceil() as usize;
    Some(values[lower] + (values[upper] - values[lower]) * position.fract())
}

pub fn above_core_threshold(item: &ScoredItem, scored: &[ScoredItem]) -> bool {
    core_score_threshold(scored, item.item.cost)
        .is_some_and(|threshold| item.score.per_slot_value > threshold)
}

pub fn score_items(
    hero: &HeroModel,
    items: &[ItemModel],
    meta: &MetaIndex,
    deltas: &[PatchDelta],
    cfg: &ReasonerConfig,
) -> Vec<ScoredItem> {
    let mut scored = items
        .iter()
        .filter(|item| item.shopable && !item.disabled)
        .map(|item| score_item(item, hero, meta, deltas, cfg))
        .collect::<Vec<_>>();
    scored.sort_by(|left, right| {
        right
            .score
            .total
            .total_cmp(&left.score.total)
            .then_with(|| left.item.item_id.cmp(&right.item.item_id))
    });
    scored
}

fn meta_value(item_id: i64, meta: &MetaIndex) -> f64 {
    let Some(support) = meta.by_item.get(&item_id) else {
        return 0.0;
    };
    let prevalence = if support.prevalence > 1.0 {
        (1.0 + support.prevalence).ln() / 10.0
    } else {
        support.prevalence.clamp(0.0, 1.0)
    };
    let winrate = support.winrate_pp.unwrap_or_default().abs().min(100.0) / 100.0;
    let lift = support.lift_pp.unwrap_or_default().abs().min(100.0) / 100.0;
    (prevalence * 0.15
        + winrate * 0.05
        + lift * 0.05
        + support.author_hits.max(0) as f64 * 0.1
        + support.claim_hits.max(0) as f64 * 0.02)
        .min(0.5)
}

fn patch_value(item: &ItemModel, hero: &HeroModel, deltas: &[PatchDelta]) -> f64 {
    deltas
        .iter()
        .filter_map(|delta| match delta.target {
            crate::DeltaTarget::Item(id) if id == item.item_id => {
                Some(delta.sign as f64 * delta.magnitude.abs().min(100.0) / 10.0)
            }
            crate::DeltaTarget::Hero(id) if id == hero.hero_id => {
                let axis_match = match hero.damage_plan.primary_axis {
                    DamageType::Weapon => delta.mechanic.contains("weapon"),
                    DamageType::Spirit => delta.mechanic.contains("spirit"),
                    DamageType::Hybrid | DamageType::None => true,
                };
                axis_match.then_some(delta.sign as f64 * delta.magnitude.abs() / 20.0)
            }
            _ => None,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn fix_e_snapshot_spirit_fire_rate_reaches_weapon_score_once() {
        let cfg = ReasonerConfig::default();
        let meta = MetaIndex {
            by_item: Default::default(),
            sample_ok: Default::default(),
        };
        let mut warden = hero();
        warden.scaling = crate::data::scaling_stats(Some(
            &serde_json::json!({"EFireRate":{"scaling_stat":"ETechPower","scale":0.25},"ERoundsPerSecond":{"scaling_stat":"ETechPower","scale":0.01}}),
        ));
        let item = ItemModel {
            property_spirit_scaling: Default::default(),
            property_damage_types: Default::default(),
            component_items: Vec::new(),
            class_name: String::new(),
            description: String::new(),
            item_id: 1,
            name: "Spirit fixture".into(),
            slot: crate::SlotType::Weapon,
            tier: 2,
            cost: 1600,
            is_active: false,
            shopable: true,
            disabled: false,
            damage_axis: DamageType::Spirit,
            defense_kind: vec![],
            properties: BTreeMap::from([("TechPower".into(), 40.0)]),
            passive_properties: BTreeMap::new(),
            conditional_properties: Default::default(),
            condition: ConditionKind::None,
            proc_cooldown: None,
            imbueable: false,
        };
        let with_scaling = score_item(&item, &warden, &meta, &[], &cfg);
        warden.scaling.clear();
        let without = score_item(&item, &warden, &meta, &[], &cfg);
        let expected = 200.0 / (20.0 / 5.4 + 2.0) - 200.0 / 6.0;
        assert!((with_scaling.score.total - without.score.total - expected).abs() < 1e-10);
        assert!(
            (with_scaling.score.passive_value - without.score.passive_value - expected).abs()
                < 1e-10
        );
    }

    #[test]
    fn fix_e_fire_rate_percentage_fallback_and_passive_spirit() {
        let cfg = ReasonerConfig::default();
        let meta = MetaIndex {
            by_item: Default::default(),
            sample_ok: Default::default(),
        };
        let mut warden = hero();
        warden.scaling = crate::data::scaling_stats(Some(
            &serde_json::json!({"EFireRate":{"scaling_stat":"ETechPower","scale":0.25}}),
        ));
        let item = ItemModel {
            property_spirit_scaling: Default::default(),
            property_damage_types: Default::default(),
            component_items: Vec::new(),
            class_name: String::new(),
            description: String::new(),
            item_id: 1,
            name: "Passive Spirit".into(),
            slot: crate::SlotType::Weapon,
            tier: 2,
            cost: 1600,
            is_active: false,
            shopable: true,
            disabled: false,
            damage_axis: DamageType::Spirit,
            defense_kind: vec![],
            properties: BTreeMap::from([("TechPower".into(), 40.0)]),
            passive_properties: BTreeMap::from([("TechPower".into(), 40.0)]),
            conditional_properties: Default::default(),
            condition: ConditionKind::ActiveCooldown {
                uptime: 0.1,
                cooldown: 10.0,
            },
            proc_cooldown: None,
            imbueable: false,
        };
        let with_scaling = score_item(&item, &warden, &meta, &[], &cfg);
        warden.scaling.clear();
        let without = score_item(&item, &warden, &meta, &[], &cfg);
        let expected = 200.0 / (20.0 / 5.5 + 2.0) - 200.0 / 6.0;
        // Die passiven Werte gelten auch dann durchgehend, wenn ein anderer
        // Effekt desselben Items einen aktiven Cooldown hat. Nur der explizit
        // bedingte Gegenfall unten darf auf zehn Prozent reduziert werden.
        assert!((with_scaling.score.total - without.score.total - expected).abs() < 1e-10);
        assert!(
            (with_scaling.score.passive_value - without.score.passive_value - expected).abs()
                < 1e-10
        );
        assert!((with_scaling.score.active_value - without.score.active_value).abs() < 1e-10);
        let mut flags_only = item.clone();
        flags_only.passive_properties.clear();
        flags_only.conditional_properties.insert("TechPower".into());
        warden.scaling = crate::data::scaling_stats(Some(
            &serde_json::json!({"EFireRate":{"scaling_stat":"ETechPower","scale":0.25}}),
        ));
        let flagged = spirit_fire_rate_value(&flags_only, &warden, &cfg);
        assert_eq!(flagged.passive_dps, 0.0);
        assert!((flagged.weapon_dps_in_score - expected * 0.1).abs() < 1e-10);
    }

    fn passive_spirit_item(spirit: f64) -> ItemModel {
        ItemModel {
            property_spirit_scaling: Default::default(),
            property_damage_types: Default::default(),
            component_items: Vec::new(),
            class_name: String::new(),
            description: String::new(),
            item_id: 1,
            name: "Passive spirit item".into(),
            slot: crate::SlotType::Weapon,
            tier: 2,
            cost: 1600,
            is_active: false,
            shopable: true,
            disabled: false,
            damage_axis: DamageType::Spirit,
            defense_kind: vec![],
            properties: std::collections::BTreeMap::from([("TechPower".into(), spirit)]),
            passive_properties: std::collections::BTreeMap::new(),
            conditional_properties: Default::default(),
            condition: ConditionKind::None,
            proc_cooldown: None,
            imbueable: false,
        }
    }

    #[test]
    fn r1_item_marginal_anchors_base_spirit_exactly_once() {
        // Astra-Gegenprobe: bullet 10, clip 16, reload 2, Basisrate 4, k=0.01,
        // passives Item +100 Spirit. Der Grenzwert muss relativ zum realen
        // Basiszustand (inkl. base_spirit_power) gebildet werden.
        let cfg = ReasonerConfig::default();
        let mut hero = hero();
        hero.weapon.bullet_damage = 10.0;
        hero.weapon.clip_size = 16.0;
        hero.weapon.reload_duration = 2.0;
        hero.weapon.shots_per_second = 4.0;
        hero.scaling = vec![crate::ScalingStat {
            stat: "ERoundsPerSecond".into(),
            per_level: 0.0,
            per_spirit: 0.01.into(),
        }];

        // Basis-Spirit 50: DPS(5.5) - DPS(4.5) = 512/135.
        hero.base_spirit_power = 50.0;
        let value = spirit_fire_rate_value(&passive_spirit_item(100.0), &hero, &cfg);
        assert!((value.passive_dps - 512.0 / 135.0).abs() < 1e-9);

        // Kontrollfall Basis-Spirit 0: DPS(5) - DPS(4) = 160/39.
        hero.base_spirit_power = 0.0;
        let control = spirit_fire_rate_value(&passive_spirit_item(100.0), &hero, &cfg);
        assert!((control.passive_dps - 160.0 / 39.0).abs() < 1e-9);
    }

    #[test]
    fn passive_spirit_aliases_are_unconditional_and_not_counted_twice() {
        let cfg = ReasonerConfig::default();
        let mut hero = hero();
        hero.scaling = vec![crate::ScalingStat {
            stat: "EBulletDamage".into(),
            per_level: 0.0,
            per_spirit: Some(0.1),
        }];
        for alias in [
            "TechPower",
            "SpiritPower",
            "BonusSpirit",
            "BonusSpiritPower",
            "SpiritPowerInnate",
        ] {
            let mut item = passive_spirit_item(100.0);
            item.properties.clear();
            item.passive_properties.insert(alias.into(), 100.0);
            let only_passive = spirit_fire_rate_value(&item, &hero, &cfg);
            item.properties.insert(alias.into(), 100.0);
            let mirrored = spirit_fire_rate_value(&item, &hero, &cfg);
            assert_eq!(only_passive.spirit_power, 100.0);
            assert_eq!(mirrored.spirit_power, 100.0);
            assert!(mirrored.passive_dps > 0.0, "Waffenschaden aus {alias}");
            assert_eq!(mirrored.active_dps, 0.0);
            assert_eq!(
                mirrored.weapon_dps_in_score,
                only_passive.weapon_dps_in_score
            );
            assert_eq!(mirrored.bullet_damage, 10.0);
        }
    }

    #[test]
    fn r2_nonfinite_conversion_is_visible_as_unknown_and_lowers_confidence() {
        let cfg = ReasonerConfig::default();
        let meta = MetaIndex {
            by_item: Default::default(),
            sample_ok: std::collections::BTreeSet::from([1]),
        };
        let mut hero = hero();
        // Nicht endlicher Primaerwert ohne validen Ersatz -> unbekannt.
        hero.scaling = vec![crate::ScalingStat {
            stat: "ERoundsPerSecond".into(),
            per_level: 0.0,
            per_spirit: f64::NAN.into(),
        }];
        let scored = score_item(&passive_spirit_item(100.0), &hero, &meta, &[], &cfg);
        assert_eq!(scored.confidence, Confidence::Low);
        assert!(scored
            .sources
            .iter()
            .any(|evidence| evidence.detail.contains("unbekannt")));

        // Valider Konverter beim sonst gleichen Item -> High moeglich, kein
        // Unbekannt-Nachweis.
        hero.scaling = vec![crate::ScalingStat {
            stat: "ERoundsPerSecond".into(),
            per_level: 0.0,
            per_spirit: 0.01.into(),
        }];
        let ok = score_item(&passive_spirit_item(100.0), &hero, &meta, &[], &cfg);
        assert_eq!(ok.confidence, Confidence::High);
        assert!(!ok
            .sources
            .iter()
            .any(|evidence| evidence.detail.contains("unbekannt")));
    }

    use std::collections::BTreeMap;

    use super::*;
    use crate::{AbilityModel, ConditionKind, DamagePlan, PurchaseBonuses, WeaponProfile};

    fn hero() -> HeroModel {
        HeroModel {
            base_spirit_power: 0.0,
            standard_level_up_upgrades: Default::default(),
            standard_upgrade_levels: Default::default(),
            level_rewards: Default::default(),
            cost_bonuses: Default::default(),
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
                item_proc_disabled: false,
                duration_scaling: Default::default(),
                upgrades: Default::default(),
                properties: Default::default(),
                ability_id: 1,
                class_name: "ability1".to_string(),
                slot: 1,
                roles: Vec::new(),
                scaling: Vec::new(),
                channel_time: None,
                charges: 0,
                cooldown: 0.0,
                scaling_step: None,
                damage_type: DamageType::Weapon,
                base_effect: 0.0,
                tick_rate: None,
                duration: None,
            }],
            damage_plan: DamagePlan {
                weapon_dps: 40.0,
                spirit_dps: 10.0,
                weapon_share: 0.8,
                primary_axis: DamageType::Weapon,
            },
        }
    }

    #[test]
    fn total_ranks_slots_independently_of_soul_efficiency() {
        let item = ItemModel {
            property_spirit_scaling: Default::default(),
            property_damage_types: Default::default(),
            component_items: Vec::new(),
            class_name: String::new(),
            description: String::new(),
            item_id: 1,
            name: "fixture".into(),
            slot: crate::SlotType::Weapon,
            tier: 2,
            cost: 800,
            is_active: false,
            shopable: true,
            disabled: false,
            damage_axis: DamageType::Weapon,
            defense_kind: vec![],
            properties: [("WeaponDamage".into(), 10.0)].into_iter().collect(),
            passive_properties: Default::default(),
            conditional_properties: Default::default(),
            condition: ConditionKind::None,
            proc_cooldown: None,
            imbueable: false,
        };
        let expensive = ItemModel {
            property_spirit_scaling: Default::default(),
            property_damage_types: Default::default(),
            component_items: Vec::new(),
            class_name: String::new(),
            description: String::new(),
            item_id: 2,
            cost: 6400,
            ..item.clone()
        };
        let meta = MetaIndex {
            by_item: Default::default(),
            sample_ok: Default::default(),
        };
        let cfg = ReasonerConfig::default();
        let cheap = score_item(&item, &hero(), &meta, &[], &cfg);
        let costly = score_item(&expensive, &hero(), &meta, &[], &cfg);
        assert_eq!(cheap.score.total, costly.score.total);
        assert_eq!(
            cheap.score.per_soul_value,
            8.0 * costly.score.per_soul_value
        );
        assert_eq!(cheap.score.total, cheap.score.per_slot_value);
        let mut bonus_hero = hero();
        bonus_hero.purchase_bonuses.weapon.push(crate::TierBonus {
            tier: 2,
            value: 20.0,
            value_type: "WeaponDamage".into(),
        });
        let with_bonus = score_item(&item, &bonus_hero, &meta, &[], &cfg);
        assert_eq!(with_bonus.score.combat_value, cheap.score.combat_value);
        assert!(with_bonus.score.purchase_bonus_value > 0.0);
        assert!(
            (with_bonus.score.per_soul_value
                - cheap.score.per_soul_value
                - with_bonus.score.purchase_bonus_value / item.cost as f64)
                .abs()
                < 1e-12
        );
        let mut disabled = expensive.clone();
        disabled.disabled = true;
        assert_eq!(
            score_items(&hero(), &[item, disabled], &meta, &[], &cfg).len(),
            1
        );
        let values = [10.0, 20.0, 30.0, 40.0].map(|value| {
            let mut scored = cheap.clone();
            scored.score.per_slot_value = value;
            scored
        });
        assert_eq!(core_score_threshold(&values, 800), Some(25.0));
        assert!(!above_core_threshold(&values[1], &values));
        assert!(above_core_threshold(&values[2], &values));
    }

    #[test]
    fn build_item_model_validates_loaded_model_without_parsing_payload_again() {
        let item = ItemModel {
            property_spirit_scaling: Default::default(),
            property_damage_types: Default::default(),
            component_items: Vec::new(),
            class_name: String::new(),
            description: String::new(),
            item_id: 1,
            name: "Veil Walker".to_string(),
            slot: crate::SlotType::Weapon,
            tier: 2,
            cost: 1600,
            is_active: false,
            shopable: true,
            disabled: false,
            damage_axis: DamageType::Weapon,
            defense_kind: Vec::new(),
            properties: BTreeMap::new(),
            passive_properties: BTreeMap::new(),
            conditional_properties: Default::default(),
            condition: ConditionKind::None,
            proc_cooldown: None,
            imbueable: false,
        };
        assert_eq!(build_item_model(&item).unwrap(), item);
    }

    #[test]
    fn score_separates_active_passive_and_meta() {
        let item = ItemModel {
            property_spirit_scaling: Default::default(),
            property_damage_types: Default::default(),
            component_items: Vec::new(),
            class_name: String::new(),
            description: String::new(),
            item_id: 1,
            name: "x".to_string(),
            slot: crate::SlotType::Weapon,
            tier: 2,
            cost: 1600,
            is_active: true,
            shopable: true,
            disabled: false,
            damage_axis: DamageType::Weapon,
            defense_kind: Vec::new(),
            properties: [("WeaponDamage".to_string(), 10.0)].into_iter().collect(),
            passive_properties: [("PassiveHealth".to_string(), 100.0)].into_iter().collect(),
            conditional_properties: Default::default(),
            condition: ConditionKind::ActiveCooldown {
                uptime: 0.5,
                cooldown: 20.0,
            },
            proc_cooldown: None,
            imbueable: false,
        };
        let meta = MetaIndex {
            by_item: [(
                1,
                crate::MetaSupport {
                    prevalence: 0.8,
                    winrate_pp: Some(2.0),
                    lift_pp: Some(1.0),
                    author_hits: 1,
                    claim_hits: 0,
                    avg_buy_time_relative: None,
                },
            )]
            .into_iter()
            .collect(),
            sample_ok: [1].into_iter().collect(),
        };
        let score = score_item(&item, &hero(), &meta, &[], &ReasonerConfig::default()).score;
        assert!(score.active_value > 0.0);
        assert!(score.passive_value > 0.0);
        assert_eq!(score.condition_factor, 0.5);
        assert!(score.meta_support < 0.5);
    }

    #[tokio::test]
    #[ignore = "benötigt echten Postgres-Snapshot über DEADLOCK_CENTRAL_DSN"]
    async fn scores_warden_reference_items_from_real_snapshot() {
        let dsn = std::env::var("DEADLOCK_CENTRAL_DSN").expect("DEADLOCK_CENTRAL_DSN fehlt");
        let pool = sqlx::postgres::PgPoolOptions::new()
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
            .unwrap_or_else(|_| panic!("Verbindung für Warden-Echtdatenlauf fehlgeschlagen"));
        let ctx = crate::ReasonerCtx {
            pool,
            ai: None,
            config: ReasonerConfig::default(),
        };
        let hero = crate::hero::load_built_hero_model(&ctx, "Warden")
            .await
            .unwrap();
        assert!(
            hero.damage_plan.weapon_share > 0.6,
            "{:?}",
            hero.damage_plan
        );
        let willpower = hero
            .abilities
            .iter()
            .find(|ability| ability.class_name == "ability_warden_high_alert")
            .unwrap();
        let step = willpower
            .scaling_step
            .as_ref()
            .expect("Willpower-Skalierungsstufe fehlt");
        assert_eq!(step.upgrade_index, 2);
        assert_eq!((step.from, step.to), (0.8, 3.5));
        let items = crate::data::load_item_models(&ctx).await.unwrap();
        let meta = MetaIndex {
            by_item: BTreeMap::new(),
            sample_ok: std::collections::BTreeSet::new(),
        };
        let scored = score_items(&hero, &items, &meta, &[], &ctx.config);
        println!("WARDEN_MODEL {}", serde_json::to_string(&hero).unwrap());
        for (rank, item) in scored.iter().enumerate() {
            println!(
                "WARDEN_SCORE {} {}",
                rank + 1,
                serde_json::to_string(item).unwrap()
            );
        }
        let mut below = Vec::new();
        for name in [
            "Veil Walker",
            "Mercurial Magnum",
            "Siphon Bullets",
            "Quicksilver Reload",
        ] {
            let score = scored
                .iter()
                .find(|item| item.item.name == name)
                .unwrap_or_else(|| panic!("{name}"));
            let threshold =
                core_score_threshold(&scored, score.item.cost).expect("Kern-Schwelle fehlt");
            println!(
                "WARDEN_REFERENCE {name}: score={} threshold={threshold}",
                score.score.per_slot_value
            );
            if !above_core_threshold(score, &scored) {
                below.push(name);
            }
        }
        assert!(below.is_empty(), "Unter der Kern-Schwelle: {below:?}");
    }
}
