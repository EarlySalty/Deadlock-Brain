use std::collections::{BTreeMap, BTreeSet};

use brain_contracts::RequestDeadline;

use crate::{
    AbilityBlockKind, AbilityModel, AbilityPropertyKind, AbilityPropertyScale, AbilityPropertyView,
    AbilityScaleClass, AbilityScaleInput, BoonRange, CalculationModels, CalculationResult,
    CalculationScenario, ContributionComparison, CurveComparisonPoint, CurveComparisonStatus,
    CurveLeader, DamageModifiers, EntityKind, EntityRef, GrowthChange, GrowthMetric, GrowthPoint,
    HeroCurveComparison, HeroGrowth, HeroModel, ItemModel, MeasuredValue, MetricDirection,
    MetricRank, ModelSource, Overtake, PerShotContribution, PopulationRanking, ProgressionInput,
    ProjectedAbilityProperty, ReasonerConfig, ReasonerError, Result, SheetComparisonInput,
    SheetComparisonResult, SheetComparisonVariant, SheetVariant, SourcedHeroModel, SpiritInput,
};

fn error(message: impl Into<String>) -> ReasonerError {
    ReasonerError::Data(message.into())
}

fn known(value: f64, unit: &str, sources: &[ModelSource], rule: &str) -> MeasuredValue {
    if !value.is_finite() {
        return unknown(unit, "Rechnung ergibt keinen endlichen Wert", &[], sources);
    }
    MeasuredValue::Known {
        value,
        unit: unit.into(),
        sources: sources.to_vec(),
        rule: Some(rule.into()),
    }
}

fn unknown(unit: &str, reason: &str, fields: &[&str], sources: &[ModelSource]) -> MeasuredValue {
    MeasuredValue::Unknown {
        unit: unit.into(),
        reason: reason.into(),
        missing_fields: fields.iter().map(|field| (*field).into()).collect(),
        sources: sources.to_vec(),
    }
}

fn nonnegative(value: f64) -> bool {
    value.is_finite() && value >= 0.0
}

pub fn validate_calculation_scenario(scenario: &CalculationScenario) -> Result<()> {
    if !nonnegative(scenario.target.health)
        || scenario.target.health == 0.0
        || !nonnegative(scenario.target.regeneration)
        || !scenario.target.shields.iter().copied().all(nonnegative)
        || !scenario.window_seconds.is_finite()
        || scenario.window_seconds <= 0.0
        || !scenario.hit_fraction.is_finite()
        || !(0.0..=1.0).contains(&scenario.hit_fraction)
        || !scenario.headshot_fraction.is_finite()
        || !(0.0..=1.0).contains(&scenario.headshot_fraction)
        || scenario.headshot_bonus.is_some_and(|v| !nonnegative(v))
        || scenario
            .distance_source_units
            .is_some_and(|v| !nonnegative(v))
        || matches!(scenario.spirit, SpiritInput::Total(v) if !nonnegative(v))
        || [
            scenario.weapon_bonus_percent,
            scenario.fire_rate_bonus_percent,
        ]
        .into_iter()
        .flatten()
        .any(|value| !value.is_finite() || value < -100.0)
        || matches!(scenario.progression, ProgressionInput::Souls(v) if v < 0)
    {
        return Err(error("Ungültige Szenariowerte"));
    }
    let mut last = -1.0;
    for change in &scenario.target.changes {
        if !nonnegative(change.at_seconds) || change.at_seconds <= last {
            return Err(error(
                "Zielzustände benötigen eindeutige aufsteigende Zeitpunkte",
            ));
        }
        last = change.at_seconds;
        crate::mechanics::damage_factors(1.0, &change.bullet)?;
        crate::mechanics::damage_factors(1.0, &change.spirit)?;
    }
    crate::mechanics::damage_factors(1.0, &scenario.target.bullet)?;
    crate::mechanics::damage_factors(1.0, &scenario.target.spirit)?;
    let ids: BTreeSet<_> = scenario.item_ids.iter().copied().collect();
    if ids.len() != scenario.item_ids.len() || ids.iter().any(|id| *id <= 0) {
        return Err(error("Inventar benötigt eindeutige positive Item-IDs"));
    }
    Ok(())
}

fn inventory(models: &CalculationModels, scenario: &CalculationScenario) -> Result<Vec<ItemModel>> {
    let held = crate::inventory::Inventory {
        held_ids: scenario.item_ids.iter().copied().collect(),
        spent_souls: 0,
    };
    if !scenario.purchases.is_empty() {
        let rules = scenario
            .inventory_rules
            .as_ref()
            .ok_or_else(|| error("Kaufübergänge benötigen belegte Inventarregeln"))?;
        let active_limit = scenario
            .max_active_items
            .ok_or_else(|| error("Kaufübergänge benötigen die aktive Platzgrenze"))?;
        let mut state = crate::inventory::Inventory::default();
        for transition in &scenario.purchases {
            let item = models
                .items
                .iter()
                .find(|item| item.item_id == transition.purchased_id)
                .ok_or_else(|| error("Kaufitem fehlt im Katalog"))?;
            let verified = state.preview_purchase_with_active_limit(
                item,
                &models.items,
                rules,
                &transition.sold_ids,
                active_limit,
            )?;
            if verified != *transition {
                return Err(error(
                    "Kaufübergang widerspricht der gemeinsamen Inventarrechnung",
                ));
            }
            state.apply_transition(&verified)?;
        }
        if state.held_ids != held.held_ids {
            return Err(error(
                "Endinventar stimmt nicht mit den Kaufübergängen überein",
            ));
        }
    }
    let items = held.held_items(&models.items)?;
    if items
        .iter()
        .any(|item| !item.shopable || item.disabled || item.cost <= 0)
    {
        return Err(error(
            "Inventar enthält nicht kaufbare oder deaktivierte Items",
        ));
    }
    if let Some(rules) = &scenario.inventory_rules {
        if rules.max_slots == 0 || items.len() > rules.max_slots {
            return Err(error("Inventar überschreitet die belegte Platzgrenze"));
        }
        for item in &items {
            if rules
                .upgrade_components
                .get(&item.item_id)
                .into_iter()
                .flatten()
                .any(|component| held.held_ids.contains(component))
            {
                return Err(error(
                    "Upgrade und verbrauchte Komponente sind gleichzeitig gehalten",
                ));
            }
        }
    }
    if scenario
        .max_active_items
        .is_some_and(|limit| items.iter().filter(|i| i.is_active).count() > limit)
    {
        return Err(error("Inventar überschreitet die aktive Platzgrenze"));
    }
    Ok(items)
}

fn validate_sources(models: &CalculationModels) -> Result<()> {
    let identities_invalid = models
        .heroes
        .iter()
        .any(|(id, hero)| *id <= 0 || *id != hero.model.hero_id)
        || models
            .weapons
            .iter()
            .any(|(id, weapon)| *id <= 0 || *id != weapon.item_id)
        || models.items.iter().any(|item| item.item_id <= 0)
        || models
            .items
            .iter()
            .map(|item| item.item_id)
            .collect::<BTreeSet<_>>()
            .len()
            != models.items.len();
    if identities_invalid {
        return Err(error(
            "Rechenmodelle benötigen eindeutige positive API-Identitäten",
        ));
    }
    let sources = models
        .heroes
        .values()
        .map(|hero| &hero.source)
        .chain(models.weapons.values().map(|weapon| &weapon.source))
        .chain(models.item_sources.values())
        .chain(
            models
                .heroes
                .values()
                .flat_map(|hero| hero.ability_sources.values()),
        );
    if models.client_version <= 0
        || sources.into_iter().any(|source| {
            source.client_version != models.client_version
                || source.document_id.is_empty()
                || source.original_url.is_empty()
                || source.language.is_empty()
                || models
                    .heroes
                    .values()
                    .next()
                    .is_some_and(|first| source.language != first.source.language)
        })
    {
        return Err(error(
            "Rechenmodelle enthalten fehlende oder gemischte Versionsbelege",
        ));
    }
    Ok(())
}

fn progressed(
    hero: &HeroModel,
    scenario: &CalculationScenario,
    cfg: &ReasonerConfig,
) -> Result<(HeroModel, crate::progression::ProgressionEvidence)> {
    if hero.level_curve.is_empty() || hero.level_rewards.is_empty() {
        return Err(error("Fortschritt benötigt Levelkurve und Belohnungen"));
    }
    let (model, evidence) = match scenario.progression {
        ProgressionInput::Souls(souls) => {
            crate::progression::at_souls(hero, &scenario.ability_order, souls, cfg)
        }
        ProgressionInput::Boons(boons) => {
            crate::progression::at_boons(hero, &scenario.ability_order, boons, cfg)?
        }
    };
    if scenario
        .expected_level
        .is_some_and(|level| level != evidence.reached_level)
        || scenario
            .expected_unspent_ap
            .is_some_and(|ap| ap != evidence.unspent_ability_points)
        || !scenario.ability_order.is_empty()
            && evidence.applied_order_steps != scenario.ability_order.len()
    {
        return Err(error(
            "Level, AP oder Skillfolge widersprechen den belegten Belohnungen",
        ));
    }
    Ok((model, evidence))
}

fn stat_unit(key: &str) -> &'static str {
    match key {
        "max_health" => "health",
        "base_health_regen" | "ooc_health_regen" => "health/s",
        "max_move_speed" | "sprint_speed" | "crouch_speed" => "m/s",
        "stamina" | "ability_resource_max" => "points",
        "stamina_regen_per_second" | "ability_resource_regen_per_second" => "points/s",
        "light_melee_damage" | "heavy_melee_damage" => "damage",
        "ground_dash_distance_in_meters" | "air_dash_distance_in_meters" => "m",
        "ground_dash_duration" | "air_dash_duration" => "s",
        _ => "source_value",
    }
}

fn property_unit(raw: Option<&serde_json::Value>, key: &str) -> &'static str {
    if key == "AbilityCharges" {
        return "charges";
    }
    if raw
        .and_then(|value| value.get("postfix"))
        .and_then(serde_json::Value::as_str)
        == Some("%")
    {
        return "percent";
    }
    match raw
        .and_then(|value| value.get("css_class"))
        .and_then(serde_json::Value::as_str)
    {
        Some("tech_damage" | "bullet_damage" | "melee_damage") => {
            return if raw
                .and_then(|value| value.get("postfix"))
                .and_then(serde_json::Value::as_str)
                == Some("/stack")
            {
                "damage/stack"
            } else if key.ends_with("DPS") || key.ends_with("PerSecond") {
                "damage/s"
            } else {
                "damage"
            };
        }
        Some("healing") => {
            return if raw
                .and_then(|value| value.get("postfix"))
                .and_then(serde_json::Value::as_str)
                .is_some_and(|unit| unit.contains('%'))
            {
                "percent_max_health"
            } else {
                "health"
            };
        }
        _ => {}
    }
    match raw
        .and_then(|value| value.get("display_units"))
        .and_then(serde_json::Value::as_str)
    {
        Some("EDisplayUnit_MetersPerSecond") => return "source_speed",
        Some("EDisplayUnit_Meters") => return "source_distance",
        _ => {}
    }
    match raw
        .and_then(|value| value.get("postfix"))
        .and_then(serde_json::Value::as_str)
    {
        Some("s") => "s",
        Some("m") => "m",
        Some("%") => "percent",
        _ => "source_value",
    }
}

fn projected_starting_stats(
    sourced: &SourcedHeroModel,
    spirit: Option<f64>,
    sources: &[ModelSource],
) -> BTreeMap<String, MeasuredValue> {
    let mut result = BTreeMap::new();
    for (key, value) in &sourced.starting_stats {
        let mut value = Some(*value);
        if let Some(display) = sourced
            .raw
            .pointer(&format!("/starting_stats/{key}/display_stat_name"))
            .and_then(serde_json::Value::as_str)
        {
            if let Some(scale) = sourced
                .model
                .scaling
                .iter()
                .find(|s| s.stat == display)
                .and_then(|s| s.per_spirit)
            {
                if scale != 0.0 {
                    value = spirit.map(|spirit| value.unwrap_or_default() + scale * spirit);
                }
            }
        }
        result.insert(
            format!("starting.{key}"),
            match value {
                Some(v) => known(v, stat_unit(key), sources, "reasoner/starting-spirit/v1"),
                None => unknown(
                    stat_unit(key),
                    "Gesamt-Spirit fehlt",
                    &["/starting_stats/tech_power/value"],
                    sources,
                ),
            },
        );
    }
    result
}

struct AbilityProjectionContext<'a> {
    models: &'a CalculationModels,
    sourced: &'a SourcedHeroModel,
    stats: &'a crate::combat::InventoryStats,
    spirit_known: bool,
    metrics: &'a BTreeMap<String, MeasuredValue>,
}

fn property_kind(raw: &serde_json::Value, key: &str) -> AbilityPropertyKind {
    if matches!(
        key,
        "WeaponDamageBonus"
            | "TechPower"
            | "WeaponPower"
            | "SpiritPower"
            | "BulletResist"
            | "TechResist"
            | "MaxHealthRegen"
            | "BuffMeleeDamage"
    ) {
        return AbilityPropertyKind::State;
    }
    if raw
        .get("postfix")
        .and_then(serde_json::Value::as_str)
        .is_some_and(|unit| unit.contains('%'))
        && !key.starts_with("Heal")
    {
        return AbilityPropertyKind::State;
    }
    match raw.get("css_class").and_then(serde_json::Value::as_str) {
        Some("healing") => AbilityPropertyKind::Healing,
        Some("tech_damage" | "bullet_damage" | "melee_damage") => AbilityPropertyKind::Damage,
        Some("duration" | "cooldown") => AbilityPropertyKind::Time,
        Some("armor_bullet_color" | "armor_tech_color") => AbilityPropertyKind::State,
        _ if key.starts_with("Heal") => AbilityPropertyKind::Healing,
        _ if raw.get("postfix").and_then(serde_json::Value::as_str) == Some("s") => {
            AbilityPropertyKind::Time
        }
        _ if matches!(
            key,
            "WeaponDamageBonus"
                | "BulletResist"
                | "TechResist"
                | "MaxHealthRegen"
                | "BuffMeleeDamage"
        ) =>
        {
            AbilityPropertyKind::State
        }
        _ => AbilityPropertyKind::Other,
    }
}

fn applied_property_scale(
    ability: &AbilityModel,
    key: &str,
    rank: usize,
    raw: &serde_json::Value,
) -> (Option<AbilityPropertyScale>, Option<String>) {
    let mut scale = crate::data::ability_property_scale(raw);
    for upgrade in ability.upgrades.iter().take(rank) {
        for property in upgrade
            .get("property_upgrades")
            .and_then(serde_json::Value::as_array)
            .into_iter()
            .flatten()
        {
            if property.get("name").and_then(serde_json::Value::as_str) != Some(key) {
                continue;
            }
            let kind = property
                .get("upgrade_type")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("EAddToBase");
            if !matches!(
                kind,
                "EAddToBase" | "EMultiplyBase" | "EAddToScale" | "EMultiplyScale"
            ) {
                return (scale, Some(format!("Upgradeart {kind} ist nicht belegt")));
            }
            if !matches!(kind, "EAddToScale" | "EMultiplyScale") {
                continue;
            }
            let Some(current) = scale.as_mut() else {
                return (
                    scale,
                    Some("Skalierungsupgrade ohne belegte Skalierungsfunktion".into()),
                );
            };
            if property
                .get("scale_stat_filter")
                .and_then(serde_json::Value::as_str)
                .map(crate::data::ability_scale_input)
                .is_some_and(|filter| filter != current.input)
            {
                return (
                    scale,
                    Some(
                        "Skalierungsupgrade hat einen anderen Statfilter als die belegte Funktion"
                            .into(),
                    ),
                );
            }
            let Some(bonus) = crate::data::number(property.get("bonus")) else {
                return (
                    scale,
                    Some("Skalierungsupgrade besitzt keinen endlichen Bonus".into()),
                );
            };
            let Some(coefficient) = current.coefficient else {
                return (
                    scale,
                    Some("Ausgangskoeffizient des Skalierungsupgrades fehlt".into()),
                );
            };
            let coefficient = if kind == "EAddToScale" {
                coefficient + bonus
            } else {
                coefficient * bonus
            };
            if !coefficient.is_finite() {
                return (
                    scale,
                    Some("Skalierungsupgrade ergibt keinen endlichen Koeffizienten".into()),
                );
            }
            current.coefficient = Some(coefficient);
        }
    }
    (scale, None)
}

fn ability_property_view(
    ability: &AbilityModel,
    reference: &str,
    rank: usize,
    context: &AbilityProjectionContext<'_>,
) -> Result<AbilityPropertyView> {
    let source = context
        .sourced
        .ability_sources
        .get(&ability.ability_id)
        .ok_or_else(|| error("Quellenbeleg einer referenzierten Fähigkeit fehlt"))?;
    let payload = context
        .models
        .item_payloads
        .get(&ability.ability_id)
        .ok_or_else(|| error("Originalpayload einer referenzierten Fähigkeit fehlt"))?;
    let raw_properties = payload
        .get("properties")
        .and_then(serde_json::Value::as_object);
    let keys: BTreeSet<_> = raw_properties
        .into_iter()
        .flatten()
        .map(|(key, _)| key)
        .chain(ability.properties.keys())
        .collect();
    let mut properties = BTreeMap::new();
    for key in keys {
        let raw = raw_properties
            .and_then(|properties| properties.get(key))
            .cloned()
            .unwrap_or_default();
        let unit = property_unit(Some(&raw), key);
        let mut origin = source.clone();
        origin.json_pointer.push_str(&format!(
            "/properties/{}",
            key.replace('~', "~0").replace('/', "~1")
        ));
        let mut sources = vec![origin];
        for (index, upgrade) in ability.upgrades.iter().take(rank).enumerate() {
            for (field, property) in upgrade
                .get("property_upgrades")
                .and_then(serde_json::Value::as_array)
                .into_iter()
                .flatten()
                .enumerate()
            {
                if property.get("name").and_then(serde_json::Value::as_str) == Some(key) {
                    let mut origin = source.clone();
                    origin
                        .json_pointer
                        .push_str(&format!("/upgrades/{index}/property_upgrades/{field}"));
                    sources.push(origin);
                }
            }
        }
        let (scale, scale_issue) = applied_property_scale(ability, key, rank, &raw);
        let base = match crate::data::number(raw.get("value").or(Some(&raw))) {
            Some(_) => match ability.properties.get(key) {
                Some(value) => known(*value, unit, &sources, "reasoner/ability-flat-upgrades/v1"),
                None => unknown(
                    unit,
                    "Numerischer Basiswert wurde nicht konvertiert",
                    &[],
                    &sources,
                ),
            },
            None => unknown(
                unit,
                "Property besitzt keinen endlichen numerischen Basiswert",
                &["value"],
                &sources,
            ),
        };
        let value = if let Some(reason) = scale_issue {
            unknown(unit, &reason, &["scale_function", "upgrades"], &sources)
        } else if base.value().is_none() {
            base.clone()
        } else {
            project_property_value(
                ability,
                key,
                &base,
                scale.as_ref(),
                unit,
                &mut sources,
                context,
            )
        };
        properties.insert(
            key.clone(),
            ProjectedAbilityProperty {
                kind: property_kind(&raw, key),
                base,
                value,
                scale,
                raw,
            },
        );
    }
    let kind = if properties.iter().any(|(key, property)| {
        property.kind == AbilityPropertyKind::Damage
            && (crate::ability_damage_units(ability, key) > 0.0
                || key == "DamageBonusFixedPerStack")
    }) {
        AbilityBlockKind::Damage
    } else {
        AbilityBlockKind::State
    };
    Ok(AbilityPropertyView {
        entity: EntityRef {
            kind: EntityKind::Ability,
            api_id: ability.ability_id,
        },
        reference: reference.into(),
        kind,
        source: source.clone(),
        applied_rank: rank,
        properties,
        upgrades: ability.upgrades.clone(),
    })
}

fn project_property_value(
    ability: &AbilityModel,
    key: &str,
    base: &MeasuredValue,
    scale: Option<&AbilityPropertyScale>,
    unit: &str,
    sources: &mut Vec<ModelSource>,
    context: &AbilityProjectionContext<'_>,
) -> MeasuredValue {
    let mut value = crate::combat::ability_value(ability, key, context.stats);
    let needs_spirit = ability.scaling.iter().any(|scale| {
        scale.stat == key
            && scale
                .per_spirit
                .is_some_and(|coefficient| coefficient != 0.0)
    });
    if needs_spirit && !context.spirit_known {
        return unknown(
            unit,
            "Spiritabhängige Property ohne belegten Gesamt-Spirit",
            &["/starting_stats/tech_power/value"],
            sources,
        );
    }
    if let Some(scale) = scale {
        match (&scale.class, &scale.input) {
            (
                AbilityScaleClass::TechDamage | AbilityScaleClass::SingleStat,
                AbilityScaleInput::Spirit,
            ) => {
                if scale.coefficient.is_none() {
                    return unknown(
                        unit,
                        "Spirit-Skalierungskoeffizient fehlt",
                        &["scale_function/stat_scale"],
                        sources,
                    );
                }
                if scale.coefficient != Some(0.0) && !context.spirit_known {
                    return unknown(
                        unit,
                        "Gesamt-Spirit fehlt",
                        &["/starting_stats/tech_power/value"],
                        sources,
                    );
                }
                if let Some(spirit) = context.metrics.get("spirit_power") {
                    sources.extend(measured_sources(&[spirit]));
                } else {
                    sources.push(context.sourced.source.clone());
                }
            }
            (AbilityScaleClass::SingleStat, AbilityScaleInput::LightMeleeDamage) => {
                let Some(coefficient) = scale.coefficient else {
                    return unknown(
                        unit,
                        "Single-Stat-Koeffizient fehlt",
                        &["scale_function/stat_scale"],
                        sources,
                    );
                };
                if coefficient == 0.0 {
                    value = base.value().unwrap();
                } else {
                    let Some(melee) = context.metrics.get("starting.light_melee_damage") else {
                        return unknown(
                            unit,
                            "Leichter Nahkampfgrundwert fehlt",
                            &["/starting_stats/light_melee_damage/value"],
                            sources,
                        );
                    };
                    if let MeasuredValue::Known {
                        sources: melee_sources,
                        ..
                    } = melee
                    {
                        sources.extend(melee_sources.iter().cloned());
                    }
                    if let Some(melee) = melee.value() {
                        value = base.value().unwrap() + coefficient * melee;
                    } else {
                        return unknown(
                            unit,
                            "Leichter Nahkampfzustand ist nicht belegt",
                            &["/starting_stats/light_melee_damage/value"],
                            sources,
                        );
                    }
                }
            }
            (
                AbilityScaleClass::SingleStat,
                AbilityScaleInput::Duration | AbilityScaleInput::Cooldown,
            ) => {}
            (AbilityScaleClass::AbilityCharges, AbilityScaleInput::Charges)
                if key == "AbilityCharges" => {}
            (AbilityScaleClass::AbilityRechargeTime, AbilityScaleInput::Cooldown)
                if key == "AbilityCooldownBetweenCharge" => {}
            (
                AbilityScaleClass::AbilityWeaponDamage,
                AbilityScaleInput::BaseWeaponDamageIncrease,
            ) if scale.coefficient == Some(0.0) => {
                value = base.value().unwrap();
            }
            (
                AbilityScaleClass::AbilityWeaponDamage | AbilityScaleClass::SingleStat,
                AbilityScaleInput::BaseWeaponDamageIncrease,
            ) => {
                return unknown(
                    unit,
                    "Einheit und Konversion von EBaseWeaponDamageIncrease sind nicht belegt",
                    &[
                        "scale_function/specific_stat_scale_type",
                        "scale_function/stat_scale",
                    ],
                    sources,
                );
            }
            (AbilityScaleClass::HealingSpirit, _) => {
                return unknown(
                    unit,
                    "Funktion und Eingang der Heilskalierung sind nicht belegt",
                    &["scale_function"],
                    sources,
                );
            }
            _ => {
                return unknown(
                    unit,
                    "Skalierungsfunktion oder Stat-Eingang ist nicht belegt",
                    &["scale_function"],
                    sources,
                );
            }
        }
    }
    if key == "AbilityCooldown" && base.value().is_some_and(|value| value >= 0.0) {
        value *= crate::combat::ability_cooldown_factor(ability, context.stats);
    }
    if ability.charges > 0 {
        if key == "AbilityCharges" {
            value = (ability.charges as f64 + context.stats.bonus_ability_charges)
                .max(0.0)
                .floor();
        } else if key == "AbilityCooldownBetweenCharge"
            && base.value().is_some_and(|value| value >= 0.0)
        {
            value *= (1.0 - context.stats.charge_spacing_reduction).max(0.0);
        }
    }
    known(value, unit, sources, "reasoner/typed-ability-projection/v1")
}

pub fn calculate_hero(
    models: &CalculationModels,
    hero_id: i64,
    scenario: &CalculationScenario,
) -> Result<CalculationResult> {
    calculate_inner(models, hero_id, scenario, true, None)
}

pub fn calculate_hero_with_deadline(
    models: &CalculationModels,
    hero_id: i64,
    scenario: &CalculationScenario,
    deadline: &RequestDeadline,
) -> Result<CalculationResult> {
    calculate_inner(models, hero_id, scenario, true, Some(deadline))
}

pub fn project_hero(
    models: &CalculationModels,
    hero_id: i64,
    scenario: &CalculationScenario,
    deadline: &RequestDeadline,
) -> Result<CalculationResult> {
    calculate_inner(models, hero_id, scenario, false, Some(deadline))
}

fn check_deadline(deadline: Option<&RequestDeadline>) -> Result<()> {
    if let Some(deadline) = deadline {
        deadline
            .check()
            .map_err(|_| error("Rechenanfrage ist abgebrochen oder ihre Frist abgelaufen"))?;
    }
    Ok(())
}

fn calculate_inner(
    models: &CalculationModels,
    hero_id: i64,
    scenario: &CalculationScenario,
    simulate: bool,
    deadline: Option<&RequestDeadline>,
) -> Result<CalculationResult> {
    check_deadline(deadline)?;
    validate_calculation_scenario(scenario)?;
    validate_sources(models)?;
    let sourced = models
        .heroes
        .get(&hero_id)
        .ok_or_else(|| ReasonerError::HeroNotFound(hero_id.to_string()))?;
    let weapon_id = if scenario.secondary_fire {
        sourced.secondary_weapon
    } else {
        sourced.primary_weapon
    };
    let weapon = weapon_id.and_then(|id| models.weapons.get(&id));
    let mut base = sourced.model.clone();
    let mut secondary_pellets_known = true;
    if let Some(weapon) = weapon {
        base.weapon = weapon.profile.clone();
        if scenario.secondary_fire {
            let primary_pellets = sourced
                .primary_weapon
                .and_then(|id| models.weapons.get(&id))
                .and_then(|w| w.timing.pellets)
                .filter(|value| crate::mechanics::valid_pellet_count(*value));
            let pellets = weapon
                .timing
                .pellets
                .filter(|value| crate::mechanics::valid_pellet_count(*value));
            secondary_pellets_known = primary_pellets.is_some() && pellets.is_some();
            if let (Some(primary_pellets), Some(pellets)) = (primary_pellets, pellets) {
                for scale in &mut base.scaling {
                    if scale.stat == "EBulletDamage" {
                        scale.per_spirit = scale.per_spirit.map(|v| v / primary_pellets * pellets);
                    }
                }
            } else {
                base.scaling.retain(|scale| scale.stat != "EBulletDamage");
            }
            base.standard_level_up_upgrades
                .remove("MODIFIER_VALUE_BASE_BULLET_DAMAGE_FROM_LEVEL");
            if let Some(value) = sourced.raw.pointer("/standard_level_up_upgrades/MODIFIER_VALUE_BASE_BULLET_DAMAGE_FROM_LEVEL_ALT_FIRE")
                .and_then(serde_json::Value::as_f64)
                .zip(pellets)
                .map(|(value, pellets)| value * pellets)
            {
                base.standard_level_up_upgrades.insert("MODIFIER_VALUE_BASE_BULLET_DAMAGE_FROM_LEVEL".into(), value);
            }
        }
    }
    let items = inventory(models, scenario)?;
    for (item_id, ability_id) in &scenario.imbues {
        if !items
            .iter()
            .any(|item| item.item_id == *item_id && item.imbueable)
            || !base
                .abilities
                .iter()
                .any(|ability| ability.ability_id == *ability_id && *ability_id > 0)
        {
            return Err(error(
                "Imbue bindet kein gehaltenes Item an eine vorhandene Fähigkeit",
            ));
        }
    }
    let cfg = ReasonerConfig {
        combat_window_seconds: scenario.window_seconds,
        ..ReasonerConfig::default()
    };
    let (mut hero, progression) = progressed(&base, scenario, &cfg)?;
    if scenario.imbues.values().any(|id| {
        !hero
            .abilities
            .iter()
            .any(|ability| ability.ability_id == *id)
    }) {
        return Err(error(
            "Imbue-Fähigkeit ist im gewählten Fortschritt nicht freigeschaltet",
        ));
    }
    let mut sources = vec![sourced.source.clone()];
    if let Some(weapon) = weapon {
        sources.push(weapon.source.clone());
    }
    for item in &items {
        sources.push(
            models
                .item_sources
                .get(&item.item_id)
                .ok_or_else(|| error("Quellenbeleg eines gehaltenen Items fehlt"))?
                .clone(),
        );
    }
    let shop_curve_known = |slot: &str| {
        base.cost_bonuses.contains_key(slot)
            || !items.iter().any(|item| {
                matches!(
                    (slot, &item.slot),
                    ("weapon", crate::SlotType::Weapon)
                        | ("spirit", crate::SlotType::Spirit)
                        | ("vitality", crate::SlotType::Vitality)
                )
            })
    };
    let mut stats = crate::combat::aggregate_stats(&hero, &items);
    crate::combat::apply_scenario_bonuses(&mut stats, scenario);
    check_deadline(deadline)?;
    let derived_spirit_known = shop_curve_known("spirit")
        && sourced.starting_stats.contains_key("tech_power")
        && (progression.standard_boons == 0
            || base
                .standard_level_up_upgrades
                .contains_key("MODIFIER_VALUE_TECH_POWER"));
    let spirit = match scenario.spirit {
        SpiritInput::Total(total) => Some(total),
        SpiritInput::Derived if derived_spirit_known => Some(stats.spirit),
        SpiritInput::Derived => None,
    };
    let mut metrics = projected_starting_stats(sourced, spirit, &sources);
    for (field, bonus) in [
        (
            "light_melee_damage",
            "MODIFIER_VALUE_BASE_MELEE_DAMAGE_FROM_LEVEL",
        ),
        (
            "ooc_health_regen",
            "MODIFIER_VALUE_OUT_OF_COMBAT_HEALTH_REGEN",
        ),
    ] {
        let metric = format!("starting.{field}");
        if let (Some(MeasuredValue::Known { value, .. }), Some(gain)) = (
            metrics.get(&metric),
            sourced.model.standard_level_up_upgrades.get(bonus),
        ) {
            metrics.insert(
                metric,
                known(
                    value + progression.standard_boons as f64 * gain,
                    stat_unit(field),
                    &sources,
                    "reasoner/spirit-and-boon-starting/v1",
                ),
            );
        }
    }
    if progression.standard_boons > 0 {
        metrics.insert(
            "starting.heavy_melee_damage".into(),
            unknown(
                "damage",
                "Boon-Bezug für schweren Nahkampfschaden ist nicht belegt",
                &["/standard_level_up_upgrades/MODIFIER_VALUE_BASE_MELEE_DAMAGE_FROM_LEVEL"],
                &sources,
            ),
        );
    }
    if progression.standard_boons > 0
        && !base
            .standard_level_up_upgrades
            .contains_key("MODIFIER_VALUE_BASE_MELEE_DAMAGE_FROM_LEVEL")
    {
        metrics.insert(
            "starting.light_melee_damage".into(),
            unknown(
                "damage",
                "Boonbonus für leichten Nahkampfschaden fehlt",
                &["/standard_level_up_upgrades/MODIFIER_VALUE_BASE_MELEE_DAMAGE_FROM_LEVEL"],
                &sources,
            ),
        );
    }
    if items.iter().any(|item| {
        item.properties
            .keys()
            .chain(item.passive_properties.keys())
            .any(|key| key.contains("MeleeDamage"))
    }) {
        for field in ["light_melee_damage", "heavy_melee_damage"] {
            metrics.insert(
                format!("starting.{field}"),
                unknown(
                    "damage",
                    "Nahkampfmodifikator des gehaltenen Inventars ist nicht belegt",
                    &["/properties/*MeleeDamage*"],
                    &sources,
                ),
            );
        }
    }
    let mut unknowns = progression.unknown_effects.clone();
    let mut combat_uncertain = progression.unknown_effects.iter().any(|effect| {
        !effect.contains("MODIFIER_VALUE_BASE_MELEE_DAMAGE_FROM_LEVEL")
            && !effect.contains("MODIFIER_VALUE_OUT_OF_COMBAT_HEALTH_REGEN")
    });
    if spirit.is_none()
        && items.iter().any(|item| {
            item.property_spirit_scaling
                .values()
                .any(|coefficient| *coefficient != 0.0)
        })
    {
        combat_uncertain = true;
        unknowns.push("Spiritabhängige Itemwirkungen ohne belegten Gesamt-Spirit".into());
    }
    if scenario.use_abilities
        || hero
            .abilities
            .iter()
            .any(crate::combat::passive_damage_ability)
    {
        combat_uncertain |= !sourced.unknowns.is_empty();
        unknowns.extend(sourced.unknowns.clone());
    }
    if scenario.use_abilities
        && hero.abilities.iter().any(|ability| {
            ability
                .properties
                .get("AbilityResourceCost")
                .is_some_and(|cost| *cost > 0.0)
        })
    {
        combat_uncertain = true;
        unknowns.push("Ressourcenverbrauch und Wiederaufbau für AbilityResourceCost sind nicht als Ereignisse belegt".into());
    }
    if hero.abilities.iter().any(|ability| {
        ability.properties.contains_key("DamageBonusFixedPerStack")
            && ability
                .properties
                .get("ProcDamageStackCount")
                .is_some_and(|count| *count > 0.0)
    }) {
        combat_uncertain = true;
        unknowns.push("Wiederholte Stack-Procs am Stacklimit sind nicht belegt".into());
    }
    let mut put = |name: &str, value, unit: &str, rule: &str| {
        metrics.insert(name.into(), known(value, unit, &sources, rule));
    };
    put(
        "boons",
        progression.standard_boons as f64,
        "boons",
        "reasoner/level-rewards/v1",
    );
    put(
        "level",
        progression.reached_level as f64,
        "level",
        "reasoner/level-rewards/v1",
    );
    put(
        "earned_souls",
        progression.earned_souls as f64,
        "souls",
        "reasoner/level-rewards/v1",
    );
    put(
        "unspent_ap",
        progression.unspent_ability_points as f64,
        "points",
        "reasoner/level-rewards/v1",
    );
    put(
        "unspent_unlocks",
        progression.unspent_unlocks as f64,
        "points",
        "reasoner/level-rewards/v1",
    );
    put(
        "weapon_bonus_percent",
        stats.weapon,
        "percent",
        "reasoner/inventory-or-total-bonus/v1",
    );
    put(
        "fire_rate_bonus_percent",
        stats.rate,
        "percent",
        "reasoner/inventory-or-total-bonus/v1",
    );
    let health_known = shop_curve_known("vitality")
        && sourced.starting_stats.contains_key("max_health")
        && (progression.standard_boons == 0
            || base
                .standard_level_up_upgrades
                .contains_key("MODIFIER_VALUE_BASE_HEALTH_FROM_LEVEL"));
    if health_known {
        put(
            "health",
            (hero.base_health * (1.0 + stats.health_pct / 100.0) + stats.health)
                * (1.0 - stats.health_loss),
            "health",
            "reasoner/health-inventory/v1",
        );
    }
    if let Some(regen) = sourced.starting_stats.get("base_health_regen") {
        put(
            "health_regeneration",
            regen + stats.regeneration,
            "health/s",
            "reasoner/regen-inventory/v1",
        );
    }
    for (name, field, bonus, extra) in [
        (
            "bullet_resist",
            "bullet_resist",
            "MODIFIER_VALUE_BULLET_ARMOR_DAMAGE_RESIST",
            stats.bullet_resist,
        ),
        (
            "spirit_resist",
            "tech_resist",
            "MODIFIER_VALUE_TECH_RESIST",
            stats.spirit_resist,
        ),
    ] {
        if let Some(resist) = sourced.starting_stats.get(field) {
            if progression.standard_boons == 0
                || base.standard_level_up_upgrades.contains_key(bonus)
            {
                let per_boon = base
                    .standard_level_up_upgrades
                    .get(bonus)
                    .copied()
                    .unwrap_or_default()
                    / 100.0;
                put(
                    name,
                    1.0 - (1.0 - resist - progression.standard_boons as f64 * per_boon)
                        * (1.0 - extra),
                    "fraction",
                    "reasoner/resist-product/v1",
                );
            }
        }
    }
    for (name, unit, field) in [
        ("health", "health", "/starting_stats/max_health/value"),
        (
            "health_regeneration",
            "health/s",
            "/starting_stats/base_health_regen/value",
        ),
        (
            "bullet_resist",
            "fraction",
            "/starting_stats/bullet_resist/value",
        ),
        (
            "spirit_resist",
            "fraction",
            "/starting_stats/tech_resist/value",
        ),
    ] {
        metrics.entry(name.into()).or_insert_with(|| {
            unknown(
                unit,
                "Grundwert oder zugehöriger Boonbonus fehlt",
                &[field],
                &sources,
            )
        });
    }
    if scenario.weapon_bonus_percent.is_none() && !shop_curve_known("weapon") {
        metrics.insert(
            "weapon_bonus_percent".into(),
            unknown(
                "percent",
                "Gesamter Waffenbonus benötigt die fehlende Shopkurve",
                &["/cost_bonuses/weapon"],
                &sources,
            ),
        );
    }
    metrics.insert(
        "spirit_power".into(),
        match spirit {
            Some(v) => known(v, "spirit", &sources, "reasoner/spirit-total/v1"),
            None => unknown(
                "spirit",
                "Grund-Spirit oder Boonbonus fehlt; expliziter Gesamtwert bleibt möglich",
                &[
                    "/starting_stats/tech_power/value",
                    "/standard_level_up_upgrades/MODIFIER_VALUE_TECH_POWER",
                ],
                &sources,
            ),
        },
    );
    let refs: Vec<_> = items.iter().collect();
    let shop_bonuses = ["weapon", "spirit", "vitality"]
        .into_iter()
        .map(|slot| {
            let value = if shop_curve_known(slot) {
                known(
                    crate::combat::shop_bonuses(&hero, &refs)
                        .get(slot)
                        .copied()
                        .unwrap_or_default(),
                    if slot == "spirit" {
                        "spirit"
                    } else {
                        "percent"
                    },
                    &sources,
                    "reasoner/held-category-value/v1",
                )
            } else {
                unknown(
                    if slot == "spirit" {
                        "spirit"
                    } else {
                        "percent"
                    },
                    "Shopkurve fehlt",
                    &[&format!("/cost_bonuses/{slot}")],
                    &sources,
                )
            };
            (slot.into(), value)
        })
        .collect();
    let mut ability_stats = stats.clone();
    if let Some(spirit) = spirit {
        ability_stats.spirit = spirit;
    }
    let mut ability_properties = BTreeMap::new();
    let mut ability_views = BTreeMap::new();
    for ability in hero.abilities.iter().chain(sourced.melee_ability.iter()) {
        check_deadline(deadline)?;
        let melee = sourced
            .melee_ability
            .as_ref()
            .is_some_and(|melee| melee.ability_id == ability.ability_id);
        let reference = if melee {
            "/items/weapon_melee".to_owned()
        } else {
            format!("/items/signature{}", ability.slot)
        };
        let rank = progression
            .ability_ranks
            .get(&ability.ability_id)
            .copied()
            .unwrap_or_default();
        let mut view = ability_property_view(
            ability,
            &reference,
            rank,
            &AbilityProjectionContext {
                models,
                sourced,
                stats: &ability_stats,
                spirit_known: spirit.is_some(),
                metrics: &metrics,
            },
        )?;
        if melee {
            view.kind = AbilityBlockKind::Melee;
            for field in ["light_melee_damage", "heavy_melee_damage"] {
                if let Some(value) = metrics.get(&format!("starting.{field}")) {
                    view.properties.insert(
                        field.into(),
                        ProjectedAbilityProperty {
                            kind: AbilityPropertyKind::Damage,
                            base: match sourced.starting_stats.get(field) {
                                Some(value) => known(
                                    *value,
                                    "damage",
                                    std::slice::from_ref(&sourced.source),
                                    "reasoner/melee-starting/v1",
                                ),
                                None => unknown(
                                    "damage",
                                    "Nahkampfgrundwert fehlt",
                                    &[field],
                                    std::slice::from_ref(&sourced.source),
                                ),
                            },
                            value: value.clone(),
                            scale: None,
                            raw: sourced
                                .raw
                                .pointer(&format!("/starting_stats/{field}"))
                                .cloned()
                                .unwrap_or_default(),
                        },
                    );
                }
            }
        }
        for (key, property) in &view.properties {
            if let MeasuredValue::Unknown { reason, .. } = &property.value {
                unknowns.push(format!("Fähigkeit {}: {key}: {reason}", ability.ability_id));
                if !melee
                    && (scenario.use_abilities || crate::combat::passive_damage_ability(ability))
                    && property.kind != AbilityPropertyKind::Other
                {
                    combat_uncertain = true;
                }
            }
            if !melee
                && property.kind == AbilityPropertyKind::Damage
                && property
                    .scale
                    .as_ref()
                    .is_some_and(|scale| scale.input == AbilityScaleInput::LightMeleeDamage)
                && scenario.use_abilities
            {
                combat_uncertain = true;
                unknowns.push("Nahkampfschaden ist skalar projiziert, aber seine Trefferereignisse sind nicht als Fähigkeitsrotation belegt".into());
            }
        }
        ability_properties.insert(
            ability.ability_id,
            view.properties
                .iter()
                .map(|(key, property)| (key.clone(), property.value.clone()))
                .collect(),
        );
        ability_views.insert(ability.ability_id, view);
    }
    if sourced.melee_ability.is_none() {
        unknowns.push(
            "Referenz /items/weapon_melee fehlt oder ihre Fähigkeit ist nicht vorhanden".into(),
        );
    }
    let mut blockers = Vec::new();
    if scenario.distance_source_units.is_some() {
        blockers.push("Falloff-Einheit und Biasfunktion sind nicht belegt".into());
    }
    if scenario.headshot_fraction > 0.0 && scenario.headshot_bonus.is_none() {
        blockers.push("Kopfkrit-Kurve ist nicht belegt; expliziter Szenariobonus fehlt".into());
    }
    let mut weapon_known = false;
    if let Some(weapon) = weapon {
        let raw = &weapon.raw["weapon_info"];
        let damage_known = raw
            .get("damage_per_shot")
            .and_then(serde_json::Value::as_f64)
            .is_some()
            || raw
                .get("bullet_damage")
                .and_then(serde_json::Value::as_f64)
                .is_some()
                && weapon.timing.pellets.is_some();
        let rate_known = crate::mechanics::weapon_timing_known(&weapon.timing)
            && weapon.profile.shots_per_second.is_finite()
            && weapon.profile.shots_per_second > 0.0;
        let clip_known = raw
            .get("clip_size")
            .and_then(serde_json::Value::as_f64)
            .is_some();
        let boon_known = progression.standard_boons == 0
            || hero
                .standard_level_up_upgrades
                .contains_key("MODIFIER_VALUE_BASE_BULLET_DAMAGE_FROM_LEVEL");
        let projected = crate::mechanics::weapon_with_spirit(&hero, spirit.unwrap_or_default());
        let mut projected = crate::WeaponProfile {
            bullet_damage: projected.bullet_damage * (1.0 + stats.weapon / 100.0),
            shots_per_second: projected.shots_per_second * (1.0 + stats.rate / 100.0),
            clip_size: projected.clip_size * (1.0 + stats.clip / 100.0) + stats.flat_clip,
            ..projected
        };
        let scaling = crate::mechanics::weapon_spirit_scaling(&hero);
        let damage_known = damage_known
            && weapon
                .timing
                .pellets
                .is_some_and(crate::mechanics::valid_pellet_count)
            && secondary_pellets_known
            && (scenario.weapon_bonus_percent.is_some() || shop_curve_known("weapon"))
            && boon_known
            && (spirit.is_some() || scaling.bullet_damage == 0.0)
            && nonnegative(projected.bullet_damage);
        let rate_known = rate_known
            && (spirit.is_some() || scaling.rounds_per_second == 0.0)
            && projected.shots_per_second.is_finite()
            && projected.shots_per_second >= 0.0;
        let clip_known = clip_known
            && (spirit.is_some() || scaling.clip_size == 0.0)
            && projected.clip_size.is_finite()
            && projected.clip_size >= 1.0;
        weapon_known = damage_known && rate_known && clip_known;
        for (name, value, unit, available) in [
            (
                "damage_per_shot",
                projected.bullet_damage,
                "damage/shot",
                damage_known,
            ),
            (
                "shots_per_second",
                projected.shots_per_second,
                "shots/s",
                rate_known,
            ),
            (
                "rpm",
                projected.shots_per_second * 60.0,
                "shots/min",
                rate_known,
            ),
            (
                "clip_size",
                projected.clip_size.floor(),
                "shots",
                clip_known,
            ),
            (
                "damage_per_magazine",
                projected.bullet_damage * projected.clip_size.floor(),
                "damage/magazine",
                damage_known && clip_known,
            ),
            (
                "weapon_dps",
                projected.bullet_damage * projected.shots_per_second,
                "damage/s",
                damage_known && rate_known,
            ),
        ] {
            if available {
                metrics.insert(
                    name.into(),
                    known(value, unit, &sources, "reasoner/weapon-projection/v1"),
                );
            }
        }
        if let Some(pellets) = weapon
            .timing
            .pellets
            .filter(|value| crate::mechanics::valid_pellet_count(*value))
        {
            if damage_known {
                metrics.insert(
                    "bullet_damage".into(),
                    known(
                        projected.bullet_damage / pellets,
                        "damage/projectile",
                        &sources,
                        "reasoner/pellets/v1",
                    ),
                );
            }
            if rate_known {
                metrics.insert(
                    "bullets_per_second".into(),
                    known(
                        projected.shots_per_second * pellets,
                        "projectiles/s",
                        &sources,
                        "reasoner/pellets/v1",
                    ),
                );
            }
        }
        if raw
            .get("reload_duration")
            .and_then(serde_json::Value::as_f64)
            .is_some()
            && (weapon.timing.reload_single_bullets != Some(true)
                || clip_known && weapon.timing.reload_single_bullets_initial_delay.is_some())
            && 1.0 + stats.reload / 100.0 > 0.0
        {
            projected.reload_duration = crate::mechanics::weapon_reload_seconds(
                projected.clip_size.floor(),
                &hero.weapon,
                Some(&weapon.timing),
                stats.reload,
            );
            if nonnegative(projected.reload_duration) {
                metrics.insert(
                    "reload_duration".into(),
                    known(
                        projected.reload_duration,
                        "s",
                        &sources,
                        "reasoner/reload-scenario/v1",
                    ),
                );
                if weapon_known {
                    let mut timing = weapon.timing.clone();
                    timing.reload_single_bullets = Some(false);
                    if let Some(cycle) = crate::mechanics::weapon_cycle_seconds(
                        &projected,
                        Some(&timing),
                        scenario.reload_convention,
                    ) {
                        metrics.insert(
                            "weapon_dps_with_reload".into(),
                            known(
                                projected.bullet_damage * projected.clip_size.floor() / cycle,
                                "damage/s",
                                &sources,
                                "reasoner/reload-cycle/v1",
                            ),
                        );
                    }
                }
            } else {
                blockers.push("Nachladezeit ist ungültig".into());
            }
        } else {
            blockers.push("Nachladefelder fehlen oder Nachladefaktor ist ungültig".into());
        }
        if damage_known {
            let mut bullet_modifiers = scenario.target.bullet.clone();
            bullet_modifiers
                .point_shreds
                .push(stats.bullet_shred / 100.0);
            bullet_modifiers.amplifications.push(stats.weapon_amp);
            let factors = crate::mechanics::damage_factors(
                projected.bullet_damage
                    * (1.0
                        + scenario.headshot_fraction * scenario.headshot_bonus.unwrap_or_default()),
                &bullet_modifiers,
            )?;
            for (name, value, unit) in [
                (
                    "target_amplification_factor",
                    factors.amplification_factor,
                    "factor",
                ),
                (
                    "target_effective_bullet_resist",
                    factors.effective_resist,
                    "fraction",
                ),
                (
                    "target_reduction_factor",
                    factors.reduction_factor,
                    "factor",
                ),
                (
                    "effective_damage_per_shot",
                    factors.effective_damage * scenario.hit_fraction,
                    "damage/shot",
                ),
            ] {
                metrics.insert(
                    name.into(),
                    known(value, unit, &sources, "reasoner/damage-factors/v1"),
                );
            }
        }
        if !weapon_known {
            blockers.push("Grundwaffe, Boonbonus oder relevante Spirit-Konversion ist unvollständig oder ungültig".into());
        }
    } else {
        blockers.push("Referenzierte Primär- oder Sekundärwaffe fehlt".into());
    }
    for (name, unit) in [
        ("damage_per_shot", "damage/shot"),
        ("bullet_damage", "damage/projectile"),
        ("shots_per_second", "shots/s"),
        ("bullets_per_second", "projectiles/s"),
        ("rpm", "shots/min"),
        ("clip_size", "shots"),
        ("damage_per_magazine", "damage/magazine"),
        ("weapon_dps", "damage/s"),
        ("weapon_dps_with_reload", "damage/s"),
        ("reload_duration", "s"),
        ("effective_damage_per_shot", "damage/shot"),
    ] {
        metrics.entry(name.into()).or_insert_with(|| {
            unknown(
                unit,
                "Benötigte Waffenfelder oder Regel fehlen",
                &["/weapon_info"],
                &sources,
            )
        });
    }
    if scenario.distance_source_units.is_some()
        || scenario.headshot_fraction > 0.0 && scenario.headshot_bonus.is_none()
    {
        metrics.insert(
            "effective_damage_per_shot".into(),
            unknown(
                "damage/shot",
                &blockers.join("; "),
                &[
                    "/weapon_info/damage_falloff_bias",
                    "/weapon_info/crit_bonus_start",
                ],
                &sources,
            ),
        );
    }
    if !scenario.use_abilities {
        hero.abilities.retain(crate::combat::passive_damage_ability);
    }
    if simulate && (scenario.use_abilities || !hero.abilities.is_empty() || !items.is_empty()) {
        check_deadline(deadline)?;
        if !scenario.target.is_hero {
            combat_uncertain = true;
            unknowns.push("Fähigkeits- und Itemmultiplikatoren für nichtheldische Ziele sind nicht vollständig belegt".into());
        }
        let coverage = match deadline {
            Some(deadline) => crate::combat::evaluate_inventory_with_deadline(
                &hero,
                &items,
                &cfg,
                &scenario.imbues,
                deadline,
            )?,
            None => crate::combat::evaluate_inventory_with_bindings(
                &hero,
                &items,
                &cfg,
                &scenario.imbues,
            ),
        };
        combat_uncertain |= !coverage.unknown_effects.is_empty();
        unknowns.extend(coverage.unknown_effects);
    }
    let mut combat = None;
    if simulate && weapon_known && blockers.is_empty() && health_known {
        check_deadline(deadline)?;
        if let Some(weapon) = weapon {
            let evaluated = match deadline {
                Some(deadline) => crate::combat::simulate_calculation_with_deadline(
                    &hero,
                    &items,
                    scenario,
                    &weapon.timing,
                    &sourced.starting_stats,
                    deadline,
                )?,
                None => crate::combat::simulate_calculation(
                    &hero,
                    &items,
                    scenario,
                    &weapon.timing,
                    &sourced.starting_stats,
                ),
            };
            let uncertain = combat_uncertain;
            let simulation_rule = if evaluated.time_resolution_seconds == 0.0 {
                "reasoner/weapon-events/v1"
            } else {
                "reasoner/combat-approximation/v1"
            };
            for (name, value, unit) in [
                ("simulated_weapon_damage", evaluated.weapon_damage, "damage"),
                (
                    "simulated_ability_damage",
                    evaluated.ability_damage,
                    "damage",
                ),
                ("simulated_proc_damage", evaluated.proc_damage, "damage"),
                (
                    "simulated_damage_per_second",
                    evaluated.damage_per_second,
                    "damage/s",
                ),
                ("time_resolution", evaluated.time_resolution_seconds, "s"),
                (
                    "target_remaining_health",
                    evaluated.target_remaining_health,
                    "health",
                ),
            ] {
                metrics.insert(name.into(), if uncertain && name != "time_resolution" {
                    unknown(unit, "Unquantifizierte Kampfwirkungen; numerische Teilrechnung steht nur im Simulationsdetail", &[], &sources)
                } else {
                    known(value, unit, &sources, simulation_rule)
                });
            }
            let ttk = if uncertain {
                unknown(
                    "s",
                    "Unquantifizierte Kampfwirkungen verhindern eine bestätigte Killzeit",
                    &[],
                    &sources,
                )
            } else if let Some(ttk) = evaluated.first_ttk {
                known(ttk, "s", &sources, simulation_rule)
            } else {
                unknown("s", "not_killed_within_window", &[], &sources)
            };
            metrics.insert("ttk".into(), ttk);
            combat = Some(evaluated);
        }
    }
    metrics.entry("ttk".into()).or_insert_with(|| {
        unknown(
            "s",
            if simulate {
                "Kampfszenario wegen fehlender Werte oder Regeln nicht berechenbar"
            } else {
                "Skalare Projektion ohne angeforderte Kampfsimulation"
            },
            &[],
            &sources,
        )
    });
    unknowns.extend(blockers);
    unknowns.sort();
    unknowns.dedup();
    check_deadline(deadline)?;
    Ok(CalculationResult {
        client_version: models.client_version, hero_id, scenario: scenario.clone(), metrics,
        api_weapon_metrics: weapon.map(|w| w.raw_metrics.clone()).unwrap_or_default(),
        ability_properties, ability_views, progression, shop_bonuses, combat,
        assumptions: vec![
            format!("Erster Schuss bei 0 Sekunden; Nachladekonvention {:?}. Magazin wird auf ganze Schüsse abgerundet. Kein aus API-DPS rückwärts erfundener Zeitaufschlag.", scenario.reload_convention),
            "Einzelnachladen verwendet ausdrücklich den vollständigen Magazinzyklus aus Anfangsverzögerung und einzelnen Patronenzeiten. Ein erlaubter Abbruch ist erhalten, aber keine optimierte Abbruchrotation simuliert.".into(),
            "API-Waffenmetriken sind unveränderte Rohwerte. Szenariometriken werden separat aus Fortschritt, Gesamt-Spirit und gehaltenem Inventar berechnet.".into(),
            "Trefferanteile unter 1 oder gemischte Kopfanteile rechnen deterministischen Erwartungsschaden, keine garantierte Schussfolge. Fähigkeitsrotation und zeitweise Itemeffekte bleiben als Näherung mit Zeitauflösung ausgewiesen.".into(),
            "Fehlende angeborene Resistenzen werden nicht als Null ausgegeben. Nicht belegte Falloff- und Kritkurven sperren nur entfernungssensitive oder kopfkritabhängige Ergebnisse.".into(),
        ], unknowns,
    })
}

pub fn rank_heroes(
    models: &CalculationModels,
    scenario: &CalculationScenario,
    metric: &str,
    direction: MetricDirection,
) -> Result<PopulationRanking> {
    validate_calculation_scenario(scenario)?;
    validate_sources(models)?;
    if models.heroes.values().any(|hero| {
        ["player_selectable", "disabled", "in_development"]
            .iter()
            .any(|field| {
                hero.raw
                    .get(*field)
                    .and_then(serde_json::Value::as_bool)
                    .is_none()
            })
    }) {
        return Err(error(
            "Rangpopulation benötigt belegte Aktivitätsmerkmale für jeden Eingabehelden",
        ));
    }
    let mut values = Vec::new();
    let mut missing = BTreeMap::new();
    let mut unit = None;
    let heroes: Vec<_> = models
        .heroes
        .values()
        .filter(|hero| {
            hero.raw["player_selectable"].as_bool() == Some(true)
                && hero.raw["disabled"].as_bool() == Some(false)
                && hero.raw["in_development"].as_bool() == Some(false)
        })
        .collect();
    for hero in &heroes {
        let id = hero.model.hero_id;
        match calculate_hero(models, id, scenario) {
            Ok(result) => {
                let value = result.metrics.get(metric);
                if let Some((value, measured)) = value.and_then(|m| m.value().map(|v| (v, m))) {
                    if unit.as_ref().is_some_and(|u| u != measured.unit()) {
                        return Err(error("Rangpopulation besitzt gemischte Einheiten"));
                    }
                    unit = Some(measured.unit().to_owned());
                    values.push((id, value));
                } else {
                    missing.insert(
                        id,
                        match value {
                            Some(MeasuredValue::Unknown { reason, .. })
                            | Some(MeasuredValue::NotApplicable { reason, .. }) => reason.clone(),
                            _ => "Metrik fehlt oder besitzt keinen endlichen Wert".into(),
                        },
                    );
                }
            }
            Err(reason) => {
                missing.insert(id, reason.to_string());
            }
        }
    }
    let compare = |left: f64, right: f64| match direction {
        MetricDirection::HigherIsBetter => right.total_cmp(&left),
        MetricDirection::LowerIsBetter => left.total_cmp(&right),
    };
    values.sort_by(|left, right| compare(left.1, right.1).then(left.0.cmp(&right.0)));
    let ranks = values
        .iter()
        .map(|(hero_id, value)| {
            let better = values
                .iter()
                .filter(|(_, other)| compare(*other, *value).is_lt())
                .count();
            let worse = values
                .iter()
                .filter(|(_, other)| compare(*other, *value).is_gt())
                .count();
            let equal = values.iter().filter(|(_, other)| *other == *value).count();
            MetricRank {
                hero_id: *hero_id,
                value: *value,
                rank: 1 + better,
                percentile: Some(100.0 * (worse as f64 + equal as f64 / 2.0) / values.len() as f64),
            }
        })
        .collect();
    Ok(PopulationRanking {
        client_version: models.client_version,
        metric: metric.into(),
        unit: unit.unwrap_or_default(),
        direction,
        scenario: scenario.clone(),
        population_total: heroes.len(),
        population_valid: values.len(),
        missing,
        ranks,
    })
}

fn measured_sources(values: &[&MeasuredValue]) -> Vec<ModelSource> {
    let mut sources = Vec::new();
    for value in values {
        match value {
            MeasuredValue::Known {
                sources: origins, ..
            }
            | MeasuredValue::Unknown {
                sources: origins, ..
            } => {
                for origin in origins {
                    if !sources.contains(origin) {
                        sources.push(origin.clone());
                    }
                }
            }
            MeasuredValue::NotApplicable { .. } => {}
        }
    }
    sources
}

fn unavailable_reason(value: &MeasuredValue) -> &str {
    match value {
        MeasuredValue::Unknown { reason, .. } | MeasuredValue::NotApplicable { reason, .. } => {
            reason
        }
        MeasuredValue::Known { .. } => "Wert ist nicht endlich",
    }
}

fn measured_difference(
    after: &MeasuredValue,
    before: &MeasuredValue,
    relative: bool,
) -> MeasuredValue {
    let sources = measured_sources(&[after, before]);
    let unit = if relative { "fraction" } else { after.unit() };
    if after.unit() != before.unit() {
        return unknown(
            unit,
            "Vergleich benötigt identische Einheiten",
            &[],
            &sources,
        );
    }
    match (after.value(), before.value()) {
        (Some(_), Some(before)) if relative && before == 0.0 => MeasuredValue::NotApplicable {
            unit: unit.into(),
            reason: "Relatives Wachstum ist bei Grundwert 0 undefiniert".into(),
        },
        (Some(after), Some(before)) => known(
            if relative {
                (after - before) / before
            } else {
                after - before
            },
            unit,
            &sources,
            "reasoner/shared-projection-difference/v1",
        ),
        _ => unknown(
            unit,
            &format!(
                "Vergleichswert fehlt: {}; {}",
                unavailable_reason(before),
                unavailable_reason(after)
            ),
            &[],
            &sources,
        ),
    }
}

fn growth_change(from: &GrowthPoint, to: &GrowthPoint) -> GrowthChange {
    let mut absolute = BTreeMap::new();
    let mut relative = BTreeMap::new();
    for metric in [
        GrowthMetric::WeaponDps,
        GrowthMetric::DamagePerMagazine,
        GrowthMetric::Health,
    ] {
        absolute.insert(
            metric,
            measured_difference(&to.metrics[&metric], &from.metrics[&metric], false),
        );
        relative.insert(
            metric,
            measured_difference(&to.metrics[&metric], &from.metrics[&metric], true),
        );
    }
    GrowthChange {
        from_boons: from.boons,
        to_boons: to.boons,
        absolute,
        relative,
    }
}

pub fn hero_growth(
    models: &CalculationModels,
    hero_id: i64,
    scenario: &CalculationScenario,
    range: BoonRange,
    deadline: &RequestDeadline,
) -> Result<HeroGrowth> {
    check_deadline(Some(deadline))?;
    validate_calculation_scenario(scenario)?;
    validate_sources(models)?;
    let sourced = models
        .heroes
        .get(&hero_id)
        .ok_or_else(|| ReasonerError::HeroNotFound(hero_id.to_string()))?;
    let max_boons = sourced
        .model
        .level_curve
        .iter()
        .filter(|level| sourced.model.standard_upgrade_levels.contains(&level.level))
        .count();
    if range.min_boons > range.max_boons
        || range.max_boons > max_boons
        || sourced.model.level_curve.is_empty()
        || sourced.model.level_rewards.is_empty()
    {
        return Err(error(
            "Boonbereich liegt außerhalb der gebundenen Levelbelohnungen",
        ));
    }
    let mut points = Vec::new();
    for boons in 0..=range.max_boons {
        check_deadline(Some(deadline))?;
        let mut input = scenario.clone();
        input.progression = ProgressionInput::Boons(boons);
        let projected = project_hero(models, hero_id, &input, deadline);
        check_deadline(Some(deadline))?;
        let metrics = [
            GrowthMetric::WeaponDps,
            GrowthMetric::DamagePerMagazine,
            GrowthMetric::Health,
        ]
        .into_iter()
        .map(|metric| {
            let value = match &projected {
                Ok(result) => result.metrics[metric.name()].clone(),
                Err(reason) => unknown(
                    match metric {
                        GrowthMetric::Health => "health",
                        GrowthMetric::WeaponDps => "damage/s",
                        GrowthMetric::DamagePerMagazine => "damage/magazine",
                    },
                    &reason.to_string(),
                    &[],
                    std::slice::from_ref(&sourced.source),
                ),
            };
            (metric, value)
        })
        .collect();
        points.push(GrowthPoint { boons, metrics });
    }
    let base = points[0].clone();
    let early_to_late = growth_change(&points[range.min_boons], &points[range.max_boons]);
    let per_boon = points
        .windows(2)
        .filter(|pair| pair[1].boons >= range.min_boons)
        .map(|pair| growth_change(&pair[0], &pair[1]))
        .collect();
    let points = points
        .into_iter()
        .filter(|point| point.boons >= range.min_boons)
        .collect();
    check_deadline(Some(deadline))?;
    Ok(HeroGrowth {
        client_version: models.client_version,
        hero_id,
        scenario: scenario.clone(),
        range,
        base,
        points,
        per_boon,
        early_to_late,
    })
}

pub fn compare_hero_curves(
    models: &CalculationModels,
    hero_ids: [i64; 2],
    scenario: &CalculationScenario,
    range: BoonRange,
    metric: GrowthMetric,
    deadline: &RequestDeadline,
) -> Result<HeroCurveComparison> {
    if hero_ids[0] == hero_ids[1] {
        return Err(error(
            "Kurvenvergleich benötigt zwei verschiedene Helden-IDs",
        ));
    }
    let left = hero_growth(models, hero_ids[0], scenario, range, deadline)?;
    let right = hero_growth(models, hero_ids[1], scenario, range, deadline)?;
    let mut points = Vec::new();
    let mut overtakes = Vec::new();
    let mut tied_boons = Vec::new();
    let mut last_strict = None;
    let mut ties_between = Vec::new();
    let mut incomplete = false;
    for (lhs, rhs) in left.points.iter().zip(&right.points) {
        check_deadline(Some(deadline))?;
        let difference = measured_difference(&lhs.metrics[&metric], &rhs.metrics[&metric], false);
        let leader = match difference.value() {
            Some(value) if value > 0.0 => CurveLeader::Left,
            Some(value) if value < 0.0 => CurveLeader::Right,
            Some(_) => CurveLeader::Tie,
            None => CurveLeader::Unknown,
        };
        match leader {
            CurveLeader::Unknown => {
                incomplete = true;
                last_strict = None;
                ties_between.clear();
            }
            CurveLeader::Tie => {
                tied_boons.push(lhs.boons);
                if last_strict.is_some() {
                    ties_between.push(lhs.boons);
                }
            }
            CurveLeader::Left | CurveLeader::Right => {
                if let Some((boons, previous)) = last_strict {
                    if previous != leader {
                        overtakes.push(Overtake {
                            last_strict_boons: boons,
                            first_strict_boons: lhs.boons,
                            from: previous,
                            to: leader,
                            tied_boons: ties_between.clone(),
                        });
                    }
                }
                last_strict = Some((lhs.boons, leader));
                ties_between.clear();
            }
        }
        points.push(CurveComparisonPoint {
            boons: lhs.boons,
            difference,
            leader,
        });
    }
    let status = if incomplete {
        CurveComparisonStatus::Incomplete
    } else if tied_boons.len() == points.len() {
        CurveComparisonStatus::AllTied
    } else if overtakes.is_empty() {
        CurveComparisonStatus::NoOvertake
    } else {
        CurveComparisonStatus::Overtakes
    };
    Ok(HeroCurveComparison {
        client_version: models.client_version,
        left,
        right,
        metric,
        points,
        overtakes,
        tied_boons,
        status,
    })
}

fn additional_per_shot(
    result: &CalculationResult,
    contributions: Option<&[PerShotContribution]>,
) -> Result<MeasuredValue> {
    let Some(contributions) = contributions else {
        return Ok(unknown(
            "damage/shot",
            "Zusätzliches Schadensglied q wurde nicht angegeben",
            &["contributions"],
            &[],
        ));
    };
    if contributions
        .iter()
        .filter(|contribution| matches!(contribution, PerShotContribution::SimulatedProc))
        .count()
        > 1
    {
        return Err(error(
            "Der bereits aggregierte Proc-Schaden darf nur einmal als Beitrag eingehen",
        ));
    }
    let effective = contributions
        .iter()
        .any(|contribution| matches!(contribution, PerShotContribution::SimulatedProc));
    if effective
        && contributions
            .iter()
            .any(|contribution| matches!(contribution, PerShotContribution::AbilityProperty { .. }))
    {
        return Ok(unknown("damage/shot", "Rohe Fähigkeitsschäden und wirksame simulierte Procs sind keine dimensionsgleichen Sheetbeiträge", &["contributions"], &[]));
    }
    let unit = if effective {
        "effective_damage/shot"
    } else {
        "damage/shot"
    };
    let mut total = 0.0;
    let mut sources = Vec::new();
    let mut missing = Vec::new();
    for contribution in contributions {
        let (value, count) = match contribution {
            PerShotContribution::AbilityProperty {
                ability_id,
                property,
                occurrences_per_shot,
            } => {
                if *ability_id <= 0 || property.is_empty() || !nonnegative(*occurrences_per_shot) {
                    return Err(error("Ungültiger expliziter Schadensbeitrag je Schuss"));
                }
                let property = result.ability_views.get(ability_id).and_then(|view| view.properties.get(property))
                    .ok_or_else(|| error("Referenzierter Szenariobeitrag fehlt in der gemeinsamen Fähigkeitsprojektion"))?;
                if property.kind != AbilityPropertyKind::Damage
                    || !matches!(
                        property.value.unit(),
                        "damage" | "damage/stack" | "damage/projectile" | "damage/shot"
                    )
                {
                    return Err(error(
                        "Nur dimensionsgleiche direkte Schadenswerte sind Beiträge je Schuss",
                    ));
                }
                (&property.value, *occurrences_per_shot)
            }
            PerShotContribution::SimulatedProc => {
                let value = result.metrics.get("simulated_proc_damage");
                let shots = result
                    .combat
                    .as_ref()
                    .map(|combat| combat.shots)
                    .filter(|shots| *shots > 0.0 && shots.is_finite());
                if let (Some(value), Some(shots)) = (value, shots) {
                    (value, 1.0 / shots)
                } else {
                    missing.push("Belegter Proc-Schaden oder positive Schusszahl fehlt".to_owned());
                    continue;
                }
            }
        };
        sources.extend(
            measured_sources(&[value])
                .into_iter()
                .filter(|source| !sources.contains(source))
                .collect::<Vec<_>>(),
        );
        if let Some(value) = value.value() {
            total += value * count;
        } else {
            missing.push(unavailable_reason(value).into());
        }
    }
    Ok(if missing.is_empty() {
        known(
            total,
            unit,
            &sources,
            "reasoner/explicit-per-shot-contributions/v1",
        )
    } else {
        unknown(unit, &missing.join("; "), &["contributions"], &sources)
    })
}

pub fn compare_sheet_scenarios(
    models: &CalculationModels,
    hero_id: i64,
    input: &SheetComparisonInput,
    deadline: &RequestDeadline,
) -> Result<SheetComparisonResult> {
    if input
        .sheet_shred
        .is_some_and(|value| !value.is_finite() || value < -1.0)
    {
        return Err(error("Ungültiger dimensionsloser Sheet-Vergleichsfaktor"));
    }
    check_deadline(Some(deadline))?;
    let needs_simulation = input.contributions.as_ref().is_some_and(|contributions| {
        contributions
            .iter()
            .any(|contribution| matches!(contribution, PerShotContribution::SimulatedProc))
    });
    let evaluate = |scenario: &CalculationScenario| {
        if needs_simulation {
            calculate_hero_with_deadline(models, hero_id, scenario, deadline)
        } else {
            project_hero(models, hero_id, scenario, deadline)
        }
    };
    let full = evaluate(&input.scenario)?;
    let fixed = additional_per_shot(&full, input.contributions.as_deref())?;
    let mut variants = BTreeMap::new();
    for variant in [
        SheetVariant::Full,
        SheetVariant::WithoutWeaponBonus,
        SheetVariant::WithoutFireRateBonus,
        SheetVariant::Baseline,
    ] {
        check_deadline(Some(deadline))?;
        let mut scenario = input.scenario.clone();
        if matches!(
            variant,
            SheetVariant::WithoutWeaponBonus | SheetVariant::Baseline
        ) {
            scenario.weapon_bonus_percent = Some(0.0);
        }
        if matches!(
            variant,
            SheetVariant::WithoutFireRateBonus | SheetVariant::Baseline
        ) {
            scenario.fire_rate_bonus_percent = Some(0.0);
        }
        let result = if variant == SheetVariant::Full {
            full.clone()
        } else {
            evaluate(&scenario)?
        };
        let additional = if input.mode == ContributionComparison::HoldFixed {
            fixed.clone()
        } else {
            additional_per_shot(&result, input.contributions.as_deref())?
        };
        let damage = result.metrics["damage_per_shot"].clone();
        let rate = result.metrics["shots_per_second"].clone();
        let sources = measured_sources(&[&additional, &damage, &rate]);
        let factor = if variant == SheetVariant::Baseline {
            Some(1.0)
        } else {
            input.sheet_shred.map(|value| 1.0 + value)
        };
        let sheet_dps = if additional.unit() != "damage/shot" {
            unknown(
                "damage/s",
                "Wirksamer simulierter Proc-Schaden ersetzt keinen rohen Sheetbeitrag q",
                &["contributions/raw_damage"],
                &sources,
            )
        } else {
            match (damage.value(), rate.value(), additional.value(), factor) {
                (Some(damage), Some(rate), Some(additional), Some(factor)) => known(rate * (damage + additional) * factor, "damage/s", &sources, "sheet/reconstructed-counterfactual/v1"),
                _ => unknown("damage/s", "Sheetvergleich benötigt belegtes q, Schaden, Rate und seinen ausdrücklichen Vergleichsfaktor", &["contributions", "sheet_shred"], &sources),
            }
        };
        variants.insert(
            variant,
            SheetComparisonVariant {
                scenario,
                damage_per_shot: damage,
                shots_per_second: rate,
                additional_damage_per_shot: additional,
                sheet_dps,
                combat: result.combat,
                unknowns: result.unknowns,
                simulated_proc_damage: result.metrics.get("simulated_proc_damage").cloned(),
                simulated_damage_per_second: result
                    .metrics
                    .get("simulated_damage_per_second")
                    .cloned(),
            },
        );
    }
    check_deadline(Some(deadline))?;
    Ok(SheetComparisonResult {
        client_version: models.client_version, hero_id, input: input.clone(), variants,
        assumptions: vec![
            "Der Sheetfaktor 1+s rekonstruiert den erhaltenen Formelzwilling, nicht die allgemeine Resistregel. Schadenstypen und Zielresistenzen bleiben im gemeinsamen Kampfsimulator getrennt.".into(),
            "Festgehaltenes q ist ein algebraischer Vergleich. Neuauswertung rechnet jeden ausdrücklich angegebenen Beitrag mit der geänderten Szenarioachse erneut; gelöschte Scratchpad-Eingaben werden nicht ersetzt.".into(),
            "Proc-Schaden aus einer Simulation ist ein mittlerer wirksamer Beitrag je tatsächlich ausgewertetem Schuss. Er ist kein roher konstanter Proc und keine exakte Sheet-DPS.".into(),
        ],
    })
}

pub fn damage_breakdown(
    raw_damage: f64,
    modifiers: &DamageModifiers,
) -> Result<crate::mechanics::DamageFactors> {
    crate::mechanics::damage_factors(raw_damage, modifiers)
}
