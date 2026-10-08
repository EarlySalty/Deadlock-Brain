use std::{
    collections::BTreeMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant},
};

use serde_json::{json, Value};

use crate::{
    calculate_hero, calculate_hero_with_deadline, calculation_models_from_payloads,
    compare_hero_curves, compare_sheet_scenarios, hero_growth, project_hero, rank_heroes,
    AbilityBlockKind, AbilityPropertyKind, AbilityScaleClass, AbilityScaleInput, BoonRange,
    CalculationModels, CalculationScenario, CalculationTarget, ContributionComparison,
    CurveComparisonStatus, CurveLeader, DamageModifiers, GrowthMetric, MeasuredValue,
    MetricDirection, ModelSource, PerShotContribution, ProgressionInput, ReloadConvention,
    SheetComparisonInput, SheetVariant, SpiritInput,
};
use brain_contracts::RequestDeadline;

fn source(kind: &str) -> ModelSource {
    ModelSource {
        client_version: 1,
        document_id: format!("recorded-unversioned-reference-{kind}"),
        original_url: format!("https://api.deadlock-api.com/v1/assets/{kind}"),
        kind: kind.into(),
        language: "english".into(),
        json_pointer: String::new(),
    }
}

fn raw_assets() -> Value {
    serde_json::from_str(include_str!("../testdata/calculation/recorded-assets.json")).unwrap()
}

fn models() -> CalculationModels {
    let raw = raw_assets();
    assert!(raw["provenance"]["client_version"].is_null());
    calculation_models_from_payloads(
        &raw["heroes"],
        &raw["items"],
        &source("heroes"),
        &source("items"),
    )
    .unwrap()
}

fn scenario() -> CalculationScenario {
    CalculationScenario {
        progression: ProgressionInput::Boons(0),
        expected_level: Some(1),
        expected_unspent_ap: Some(0),
        spirit: SpiritInput::Total(0.0),
        weapon_bonus_percent: None,
        fire_rate_bonus_percent: None,
        item_ids: Vec::new(),
        purchases: Vec::new(),
        inventory_rules: None,
        max_active_items: None,
        ability_order: Vec::new(),
        imbues: BTreeMap::new(),
        secondary_fire: false,
        use_abilities: false,
        target: CalculationTarget {
            health: 2000.0,
            regeneration: 0.0,
            shields: [0.0; 3],
            is_hero: true,
            bullet: DamageModifiers::default(),
            spirit: DamageModifiers::default(),
            changes: Vec::new(),
        },
        hit_fraction: 1.0,
        headshot_fraction: 0.0,
        headshot_bonus: None,
        distance_source_units: None,
        window_seconds: 120.0,
        reload_convention: ReloadConvention::AfterFireInterval,
    }
}

fn number(result: &crate::CalculationResult, key: &str) -> f64 {
    result.metrics[key]
        .value()
        .unwrap_or_else(|| panic!("{key}: {:?}", result.metrics[key]))
}

fn close(left: f64, right: f64) {
    assert!((left - right).abs() < 1e-8, "{left} != {right}");
}

#[test]
fn recorded_three_heroes_match_versioned_probe_values_without_relabelling_raw_provenance() {
    let models = models();
    let probe: Value = serde_json::from_str(include_str!(
        "../../../../.tasks/2026-10-06-brain-abschluss/G/API-PROBEN.json"
    ))
    .unwrap();
    assert_eq!(probe["client_version"], 6759);
    for row in probe["heroes"].as_array().unwrap() {
        let id = row["hero_id"].as_i64().unwrap();
        let result = calculate_hero(&models, id, &scenario()).unwrap();
        close(
            number(&result, "weapon_dps"),
            row["api"]["damage_per_second"].as_f64().unwrap(),
        );
        close(
            number(&result, "bullet_damage"),
            row["api"]["bullet_damage"].as_f64().unwrap(),
        );
        close(
            number(&result, "damage_per_magazine"),
            row["api"]["damage_per_magazine"].as_f64().unwrap(),
        );
        close(
            result.api_weapon_metrics["damage_per_second_with_reload"]
                .value()
                .unwrap(),
            row["api"]["damage_per_second_with_reload"]
                .as_f64()
                .unwrap(),
        );
        let raw_cycle = row["api"]["damage_per_magazine"].as_f64().unwrap()
            / row["api"]["damage_per_second_with_reload"]
                .as_f64()
                .unwrap();
        let independent_cycle = row["api"]["clip_size"].as_f64().unwrap()
            / row["api"]["bullets_per_second"].as_f64().unwrap()
            + row["api"]["reload_duration"].as_f64().unwrap();
        close(raw_cycle - independent_cycle, 0.25);
        assert!(
            number(&result, "weapon_dps_with_reload")
                > result.api_weapon_metrics["damage_per_second_with_reload"]
                    .value()
                    .unwrap()
        );
        println!(
            "API_ABGLEICH {} {}",
            row["name"],
            serde_json::to_string(&result.metrics).unwrap()
        );
    }
}

#[test]
fn boon_and_total_spirit_bookends_keep_wardens_unrounded_base() {
    let models = models();
    for (id, base, gain, hp, hp_gain) in [
        (13, 5.26, 0.143, 730.0, 33.0),
        (25, 17.34, 0.25, 805.0, 60.0),
        (7, 5.64, 0.14, 730.0, 35.0),
    ] {
        for boons in [0, 35] {
            for spirit in [0.0, 38.0] {
                let mut scenario = scenario();
                scenario.progression = ProgressionInput::Boons(boons);
                scenario.expected_level = None;
                scenario.expected_unspent_ap = None;
                scenario.spirit = SpiritInput::Total(spirit);
                let result = calculate_hero(&models, id, &scenario).unwrap();
                close(number(&result, "bullet_damage"), base + boons as f64 * gain);
                close(number(&result, "health"), hp + boons as f64 * hp_gain);
                close(number(&result, "spirit_power"), spirit);
                if boons == 35 {
                    assert_eq!(result.progression.reached_level, 36);
                    assert_eq!(result.progression.earned_souls, 48600);
                    assert_eq!(result.progression.unspent_ability_points, 32);
                }
                if id == 13 {
                    close(number(&result, "clip_size"), 25.0 + spirit * 0.5);
                }
                if id == 25 {
                    close(
                        number(&result, "shots_per_second"),
                        1.0 / 0.2625 + spirit * 0.008,
                    );
                }
                if id == 7 {
                    close(
                        number(&result, "starting.sprint_speed"),
                        1.6 + spirit * 0.05,
                    );
                }
                println!(
                    "BOOKEND {id} {boons} {spirit}: HP={} Bullet={} RPM={} DPM={}",
                    number(&result, "health"),
                    number(&result, "bullet_damage"),
                    number(&result, "rpm"),
                    number(&result, "damage_per_magazine")
                );
            }
        }
    }
}

#[test]
fn discrete_weapon_kills_include_last_shot_and_reload_convention() {
    let models = models();
    let mut input = scenario();
    for convention in [
        ReloadConvention::AfterFireInterval,
        ReloadConvention::AfterLastShot,
    ] {
        input.reload_convention = convention;
        for id in [13, 25, 7] {
            input.ability_order = if id == 13 {
                vec![crate::AbilityStep {
                    ability_id: 2948410412,
                    currency_type: 2,
                    delta: -1,
                }]
            } else {
                Vec::new()
            };
            let result = calculate_hero(&models, id, &input).unwrap();
            assert!(result.combat.as_ref().unwrap().casts.is_empty());
            let damage = number(&result, "damage_per_shot");
            let clip = number(&result, "clip_size") as usize;
            let required = (input.target.health / damage).ceil() as usize;
            let reloads = (required - 1) / clip;
            let intervals = required
                - 1
                - if convention == ReloadConvention::AfterLastShot {
                    reloads
                } else {
                    0
                };
            let expected = intervals as f64 / number(&result, "shots_per_second")
                + reloads as f64 * number(&result, "reload_duration");
            println!(
                "TTK_ABGLEICH {id} {convention:?}: {} = {expected}",
                number(&result, "ttk")
            );
            close(number(&result, "ttk"), expected);
            assert_eq!(result.combat.as_ref().unwrap().shots, required as f64);
            assert_eq!(result.combat.as_ref().unwrap().reloads, reloads);
            close(result.combat.as_ref().unwrap().time_resolution_seconds, 0.0);
        }
    }
}

#[test]
fn burst_weapon_uses_real_intra_and_inter_burst_gaps() {
    let models = models();
    let mut input = scenario();
    for (hp, ttk) in [(21.0, 0.084), (31.0, 0.168), (41.0, 0.5145)] {
        input.target.health = hp;
        let result = calculate_hero(&models, 2, &input).unwrap();
        close(number(&result, "ttk"), ttk);
        close(number(&result, "weapon_dps"), 62.973760932944614);
    }
}

#[test]
fn pellet_damage_and_single_reload_apply_once() {
    let models = models();
    let mut input = scenario();
    input.target.health = 291.7;
    let result = calculate_hero(&models, 6, &input).unwrap();
    close(number(&result, "bullet_damage"), 3.6);
    close(number(&result, "damage_per_shot"), 32.4);
    close(number(&result, "damage_per_magazine"), 291.6);
    close(number(&result, "reload_duration"), 0.705 + 9.0 * 0.3525);
    close(number(&result, "ttk"), 9.0 * 0.63 + 0.705 + 9.0 * 0.3525);
    input.progression = ProgressionInput::Boons(35);
    input.expected_level = None;
    input.expected_unspent_ap = None;
    let result = calculate_hero(&models, 6, &input).unwrap();
    close(number(&result, "bullet_damage"), 3.6 + 35.0 * 0.1);
    close(number(&result, "damage_per_shot"), (3.6 + 35.0 * 0.1) * 9.0);
}

#[test]
fn missing_fields_stay_unknown_without_zero_or_blocking_independent_base_metrics() {
    let models = models();
    let mut input = scenario();
    input.spirit = SpiritInput::Derived;
    let result = calculate_hero(&models, 25, &input).unwrap();
    assert!(result.metrics["spirit_power"].value().is_none());
    assert!(result.metrics["spirit_resist"].value().is_none());
    close(number(&result, "health"), 805.0);
    close(number(&result, "bullet_damage"), 17.34);
    close(number(&result, "damage_per_magazine"), 294.78);
    close(number(&result, "effective_damage_per_shot"), 17.34);
    assert!(result.metrics["shots_per_second"].value().is_none());
    let haze = calculate_hero(&models, 13, &input).unwrap();
    close(number(&haze, "weapon_dps"), 50.095238095238095);
    assert!(haze.metrics["clip_size"].value().is_none());
    let wraith = calculate_hero(&models, 7, &input).unwrap();
    close(number(&wraith, "weapon_dps"), 59.682539682539684);
    assert!(wraith.metrics["starting.sprint_speed"].value().is_none());
    input.spirit = SpiritInput::Total(0.0);
    input.distance_source_units = Some(1500.0);
    let result = calculate_hero(&models, 25, &input).unwrap();
    close(number(&result, "weapon_dps"), 66.05714285714285);
    assert!(result.metrics["effective_damage_per_shot"]
        .value()
        .is_none());
    assert!(result.metrics["ttk"].value().is_none());
    input.distance_source_units = None;
    input.secondary_fire = true;
    let result = calculate_hero(&models, 25, &input).unwrap();
    assert!(result.metrics["weapon_dps"].value().is_none());
}

#[test]
fn wrong_progression_version_and_nonfinite_inputs_are_rejected() {
    let mut models = models();
    let mut input = scenario();
    input.expected_level = Some(2);
    assert!(calculate_hero(&models, 25, &input).is_err());
    input.expected_level = Some(1);
    input.hit_fraction = f64::NAN;
    assert!(calculate_hero(&models, 25, &input).is_err());
    input.hit_fraction = 1.0;
    models.heroes.get_mut(&25).unwrap().source.client_version = 2;
    assert!(calculate_hero(&models, 25, &input).is_err());
}

#[test]
fn ranks_cover_active_population_and_share_ties() {
    let mut models = models();
    let input = scenario();
    let ranking = rank_heroes(&models, &input, "health", MetricDirection::HigherIsBetter).unwrap();
    assert_eq!(ranking.population_total, 5);
    assert_eq!(ranking.population_valid, 5);
    assert_eq!(
        ranking.ranks.iter().find(|r| r.hero_id == 25).unwrap().rank,
        1
    );
    for id in [13, 7, 2] {
        let rank = ranking.ranks.iter().find(|r| r.hero_id == id).unwrap();
        assert_eq!(rank.rank, 3);
        close(rank.percentile.unwrap(), 30.0);
    }
    models
        .heroes
        .get_mut(&25)
        .unwrap()
        .starting_stats
        .remove("max_health");
    let ranking = rank_heroes(&models, &input, "health", MetricDirection::HigherIsBetter).unwrap();
    assert_eq!(ranking.population_total, 5);
    assert_eq!(ranking.population_valid, 4);
    assert!(ranking.missing.contains_key(&25));
}

#[test]
fn censored_ttk_never_gets_finite_rank() {
    let models = models();
    let mut input = scenario();
    input.target.health = 1_000_000.0;
    input.window_seconds = 1.0;
    let ranking = rank_heroes(&models, &input, "ttk", MetricDirection::LowerIsBetter).unwrap();
    assert_eq!(ranking.population_total, 5);
    assert_eq!(ranking.population_valid, 0);
    assert!(ranking.ranks.is_empty());
    assert_eq!(ranking.missing.len(), 5);
    for (&id, reason) in &ranking.missing {
        let result = calculate_hero(&models, id, &input).unwrap();
        let MeasuredValue::Unknown {
            reason: metric_reason,
            ..
        } = &result.metrics["ttk"]
        else {
            panic!("uncertified TTK received a rankable metric for hero {id}");
        };
        assert_eq!(reason, metric_reason);
        assert_eq!(result.combat.as_ref().unwrap().first_ttk, None);
        if id == 13 {
            assert!(result.metrics["simulated_weapon_damage"].value().is_none());
        } else {
            assert_eq!(reason, "not_killed_within_window");
        }
    }
}

#[test]
fn resist_shred_amp_damage_stages_preserve_negative_effective_resist() {
    let modifiers = DamageModifiers {
        resist: 0.3,
        independent_resists: vec![0.2],
        point_shreds: vec![0.15],
        relative_reductions: vec![0.2],
        amplifications: vec![0.25],
        damage_reductions: vec![0.1],
    };
    let factors = crate::damage_breakdown(100.0, &modifiers).unwrap();
    close(factors.effective_resist, (1.0 - 0.7 * 0.8 - 0.15) * 0.8);
    close(
        factors.effective_damage,
        100.0 * 1.25 * (1.0 - factors.effective_resist) * 0.9,
    );
    let factors = crate::damage_breakdown(
        100.0,
        &DamageModifiers {
            resist: -0.5,
            ..Default::default()
        },
    )
    .unwrap();
    close(factors.effective_damage, 150.0);
}

#[test]
fn target_resist_changes_shields_regen_and_explicit_headshots_reach_same_simulator() {
    let models = models();
    let mut input = scenario();
    input.target.health = 30.0;
    input.target.bullet.resist = 0.5;
    input.target.shields = [5.0, 7.0, 0.0];
    let result = calculate_hero(&models, 25, &input).unwrap();
    close(number(&result, "ttk"), 4.0 * 0.2625);
    input.target.shields = [0.0; 3];
    input.target.bullet.resist = 0.0;
    input.headshot_fraction = 1.0;
    input.headshot_bonus = Some(1.0);
    let result = calculate_hero(&models, 25, &input).unwrap();
    close(number(&result, "ttk"), 0.0);
    close(number(&result, "effective_damage_per_shot"), 34.68);
    input.headshot_fraction = 0.0;
    input.headshot_bonus = None;
    input.target.health = 34.0;
    input.target.regeneration = 3.0;
    let result = calculate_hero(&models, 25, &input).unwrap();
    close(number(&result, "ttk"), 0.525);
    input.target.health = 30.0;
    input.target.regeneration = 0.0;
    input.target.changes = vec![crate::TargetChange {
        at_seconds: 0.2,
        bullet: DamageModifiers {
            resist: 1.0,
            ..Default::default()
        },
        spirit: DamageModifiers::default(),
    }];
    let result = calculate_hero(&models, 25, &input).unwrap();
    assert!(result.metrics["ttk"].value().is_none());
}

#[test]
fn conversion_keeps_zero_negative_properties_and_exact_original_pointers() {
    let raw = raw_assets();
    let mut heroes = raw["heroes"].clone();
    let index = heroes
        .as_array()
        .unwrap()
        .iter()
        .position(|hero| hero["id"] == 13)
        .unwrap();
    heroes[index]["starting_stats"]["tech_power"] = json!({"value":0});
    heroes[index]["starting_stats"]["sprint_speed"]["value"] = json!(-0.25);
    let models = calculation_models_from_payloads(
        &heroes,
        &raw["items"],
        &source("heroes"),
        &source("items"),
    )
    .unwrap();
    let hero = &models.heroes[&13];
    assert_eq!(hero.starting_stats["tech_power"], 0.0);
    assert_eq!(hero.starting_stats["sprint_speed"], -0.25);
    assert_eq!(hero.raw, heroes[index]);
    assert_eq!(hero.source.json_pointer, format!("/{index}"));
}

#[test]
fn recorded_fixation_stacks_follow_each_shot_without_confirming_unknown_rules() {
    let models = models();
    let mut input = scenario();
    input.use_abilities = true;
    input.window_seconds = 0.5;
    input.target.health = 1_000_000.0;
    input.ability_order = vec![crate::AbilityStep {
        ability_id: 1080948381,
        currency_type: 2,
        delta: -1,
    }];
    let result = calculate_hero(&models, 13, &input).unwrap();
    let combat = result.combat.as_ref().unwrap();
    let fixation = models.heroes[&13]
        .model
        .abilities
        .iter()
        .find(|ability| ability.ability_id == 1080948381)
        .unwrap();
    close(combat.shots, 5.0);
    close(
        combat.weapon_damage,
        5.0 * 5.26 + 10.0 * fixation.properties["DamageBonusFixedPerStack"],
    );
    close(combat.elapsed_seconds, 0.5);
    close(combat.time_resolution_seconds, 0.2);
    assert!(result.metrics["simulated_weapon_damage"].value().is_none());
    assert!(result.metrics["ttk"].value().is_none());
    println!(
        "FIXATION_ABGLEICH shots={} weapon_damage={} damage_per_stack={}",
        combat.shots, combat.weapon_damage, fixation.properties["DamageBonusFixedPerStack"]
    );
}

#[test]
fn disabled_casts_preserve_recorded_fixation_and_its_ranking_uncertainty() {
    let models = models();
    let mut input = scenario();
    input.window_seconds = 0.5;
    input.target.health = 1_000_000.0;
    input.ability_order = vec![crate::AbilityStep {
        ability_id: 1080948381,
        currency_type: 2,
        delta: -1,
    }];
    let result = calculate_hero(&models, 13, &input).unwrap();
    let combat = result.combat.as_ref().unwrap();
    let fixation = models.heroes[&13]
        .model
        .abilities
        .iter()
        .find(|ability| ability.ability_id == 1080948381)
        .unwrap();
    close(combat.shots, 5.0);
    close(
        combat.weapon_damage,
        5.0 * 5.26 + 10.0 * fixation.properties["DamageBonusFixedPerStack"],
    );
    assert!(combat.casts.is_empty());
    assert!(result.metrics["simulated_weapon_damage"].value().is_none());
    let ranked = rank_heroes(
        &models,
        &input,
        "simulated_weapon_damage",
        MetricDirection::HigherIsBetter,
    )
    .unwrap();
    assert!(ranked.missing.contains_key(&13));
    assert!(!ranked.ranks.iter().any(|rank| rank.hero_id == 13));
    input.use_abilities = true;
    let enabled = calculate_hero(&models, 13, &input).unwrap();
    close(combat.weapon_damage, enabled.combat.unwrap().weapon_damage);
}

#[test]
fn api_ids_and_ability_source_versions_are_not_silently_coerced() {
    let raw = raw_assets();
    for id in [-1.0, 1.5, 0.0] {
        let mut heroes = raw["heroes"].clone();
        heroes[0]["id"] = json!(id);
        assert!(calculation_models_from_payloads(
            &heroes,
            &raw["items"],
            &source("heroes"),
            &source("items")
        )
        .is_err());
        let mut items = raw["items"].clone();
        items[0]["id"] = json!(id);
        assert!(calculation_models_from_payloads(
            &raw["heroes"],
            &items,
            &source("heroes"),
            &source("items")
        )
        .is_err());
    }
    let mut models = models();
    models
        .heroes
        .get_mut(&13)
        .unwrap()
        .ability_sources
        .values_mut()
        .next()
        .unwrap()
        .client_version = 2;
    assert!(calculate_hero(&models, 13, &scenario()).is_err());
}

#[test]
fn real_skill_upgrades_charged_spirit_and_zero_scaling_use_shared_projection() {
    let models = models();
    let mut input = scenario();
    input.progression = ProgressionInput::Boons(35);
    input.expected_level = None;
    input.expected_unspent_ap = Some(29);
    input.spirit = SpiritInput::Total(38.0);
    input.ability_order = vec![
        crate::AbilityStep {
            ability_id: 1999680326,
            currency_type: 2,
            delta: -1,
        },
        crate::AbilityStep {
            ability_id: 1999680326,
            currency_type: 1,
            delta: -1,
        },
        crate::AbilityStep {
            ability_id: 1999680326,
            currency_type: 1,
            delta: -2,
        },
    ];
    let result = calculate_hero(&models, 7, &input).unwrap();
    let properties = &result.ability_properties[&1999680326];
    close(properties["AbilityCharges"].value().unwrap(), 4.0);
    close(properties["Damage"].value().unwrap(), 85.0 + 38.0 * 0.95);
    assert_eq!(properties["AbilityCooldown"].unit(), "s");
    input.spirit = SpiritInput::Derived;
    input.ability_order.clear();
    input.expected_unspent_ap = None;
    let result = calculate_hero(&models, 13, &input).unwrap();
    close(
        result.ability_properties[&1080948381]["ProcDamage"]
            .value()
            .unwrap(),
        0.0,
    );
    assert!(result.ability_properties[&2948410412]["Damage"]
        .value()
        .is_none());
    let mut charged_models = models.clone();
    let charged = crate::item_model_from_payload(&json!({
        "id": 1, "class_name": "projection_contract_item", "name": "Prüfgegenstand",
        "cost": 800, "item_slot_type": "spirit", "shopable": true,
        "properties": {"BonusAbilityCharges":{"value":1}, "BonusSpiritForChargedAbilities":{"value":10},
            "CooldownReduction":{"value":20}, "CooldownReductionOnChargedAbilities":{"value":25},
            "CooldownBetweenChargeReduction":{"value":40}}
    })).unwrap();
    charged_models.item_sources.insert(1, source("items"));
    charged_models.items.push(charged);
    input.item_ids = vec![1];
    input.spirit = SpiritInput::Total(38.0);
    let result = calculate_hero(&charged_models, 7, &input).unwrap();
    let properties = &result.ability_properties[&1999680326];
    close(
        properties["Damage"].value().unwrap(),
        45.0 + (38.0 + 10.0) * 0.55,
    );
    close(properties["AbilityCharges"].value().unwrap(), 3.0);
    close(
        properties["AbilityCooldown"].value().unwrap(),
        models.heroes[&7]
            .model
            .abilities
            .iter()
            .find(|a| a.ability_id == 1999680326)
            .unwrap()
            .cooldown
            * 0.8
            * 0.75,
    );
}

#[test]
fn recorded_item_purchase_and_missing_shop_curve_do_not_share_false_zeroes() {
    let fixture: Value =
        serde_json::from_str(include_str!("../testdata/calculation/glass-cannon.json")).unwrap();
    assert!(fixture["provenance"]["client_version"].is_null());
    let payload = &fixture["payload"];
    let item = crate::item_model_from_payload(payload).unwrap();
    let mut models = models();
    models.item_sources.insert(item.item_id, source("items"));
    models.item_payloads.insert(item.item_id, payload.clone());
    models.items.push(item.clone());
    let rules = crate::inventory::InventoryRules {
        max_slots: 1,
        upgrade_components: BTreeMap::new(),
        resale_fraction: 0.5,
    };
    let transition = crate::inventory::Inventory::default()
        .preview_purchase_with_active_limit(&item, &models.items, &rules, &[], 1)
        .unwrap();
    let mut input = scenario();
    input.item_ids = vec![item.item_id];
    input.purchases = vec![transition];
    input.inventory_rules = Some(rules);
    input.max_active_items = Some(1);
    let result = calculate_hero(&models, 25, &input).unwrap();
    close(result.shop_bonuses["weapon"].value().unwrap(), 54.0);
    close(
        number(&result, "damage_per_shot"),
        17.34 * (1.0 + (54.0 + item.properties["BaseAttackDamagePercent"]) / 100.0),
    );
    let expected_health = 805.0 * (1.0 - item.properties["MaxHealthLossPercent"].abs() / 100.0);
    close(number(&result, "health"), expected_health);
    let mut invalid = input.clone();
    invalid.purchases[0].net_cost += 1;
    assert!(calculate_hero(&models, 25, &invalid).is_err());
    models
        .heroes
        .get_mut(&25)
        .unwrap()
        .model
        .cost_bonuses
        .remove("weapon");
    let result = calculate_hero(&models, 25, &input).unwrap();
    assert!(result.shop_bonuses["weapon"].value().is_none());
    assert!(result.metrics["damage_per_shot"].value().is_none());
    close(number(&result, "health"), expected_health);
}

#[test]
fn quantized_weapon_bonus_does_not_fabricate_a_planner_gain() {
    let mut hero: crate::HeroModel = serde_json::from_value(json!({
        "hero_id":1,"name":"Prüfheld","archetype":"weapon","base_health":600.0,
        "level_curve":[{"level":1,"required_souls":0},{"level":2,"required_souls":200},{"level":3,"required_souls":500}],
        "level_rewards":{"1":["EAbilityUnlocks"],"2":["EAbilityPoints"],"3":["EAbilityUnlocks"]},
        "purchase_bonuses":{"spirit":[],"weapon":[],"vitality":[]},"scaling":[],
        "weapon":{"bullet_damage":30.0,"shots_per_second":4.0,"clip_size":16.0,"reload_duration":2.0,"range":20.0,"falloff_start_range":20.0,"falloff_end_range":50.0,"sustained_dps":80.0},
        "abilities":[],"damage_plan":{"weapon_dps":80.0,"spirit_dps":0.0,"weapon_share":1.0,"primary_axis":"Weapon"}
    })).unwrap();
    for (id, damage, cooldown) in [(10, 60.0, 10.0), (20, 200.0, 2.0)] {
        hero.abilities.push(serde_json::from_value(json!({"ability_id":id,"class_name":format!("ability_{id}"),"slot":1,"roles":["Damage"],"scaling":[],"channel_time":null,"charges":1,"cooldown":cooldown,"scaling_step":null,"damage_type":"Spirit","base_effect":damage,"properties":{"Damage":damage,"AbilityCooldown":cooldown}})).unwrap());
    }
    let cfg = crate::ReasonerConfig::default();
    let order = vec![
        crate::AbilityStep {
            ability_id: 10,
            currency_type: 2,
            delta: -1,
        },
        crate::AbilityStep {
            ability_id: 20,
            currency_type: 2,
            delta: -1,
        },
    ];
    let (hero, _) = crate::progression::at_souls(&hero, &order, 1600, &cfg);
    let mut items = Vec::new();
    for (id, damage, cost) in [(1, 100.0, 100), (2, 20.0, 1000)] {
        items.push(crate::item_model_from_payload(&json!({"id":id,"class_name":format!("item_{id}"),"name":format!("Item {id}"),"cost":cost,"item_slot_type":"weapon","shopable":true,"properties":{"BaseAttackDamagePercent":{"value":damage}}})).unwrap());
    }
    items[0].imbueable = true;
    let bindings = BTreeMap::from([(1, 10)]);
    let before =
        crate::combat::evaluate_inventory_with_bindings(&hero, &items[..1], &cfg, &bindings);
    let after = crate::combat::evaluate_inventory_with_bindings(&hero, &items, &cfg, &bindings);
    assert_eq!(after.scenarios[0].imbue_targets.get(&1), Some(&10));
    close(after.score, before.score);
    assert_eq!(after.scenarios[0].shots, before.scenarios[0].shots);
    assert_eq!(
        after.scenarios[1].targets_defeated,
        before.scenarios[1].targets_defeated
    );
    println!(
        "PLANER_QUANTISIERUNG before={} after={} delta={}",
        before.score,
        after.score,
        after.score - before.score
    );
}

#[test]
fn zero_weapon_or_rate_axis_is_known_and_relative_zero_growth_is_undefined() {
    let models = sheet_models();
    for (weapon, rate) in [(Some(-100.0), None), (None, Some(-100.0))] {
        let input = CalculationScenario {
            weapon_bonus_percent: weapon,
            fire_rate_bonus_percent: rate,
            ..sheet_scenario()
        };
        let growth = hero_growth(&models, 25, &input, range(0, 35), &deadline()).unwrap();
        assert!(growth
            .points
            .iter()
            .all(|point| point.metrics[&GrowthMetric::WeaponDps].value() == Some(0.0)));
        assert!(matches!(
            growth.early_to_late.relative[&GrowthMetric::WeaponDps],
            MeasuredValue::NotApplicable { .. }
        ));
        assert!(growth.early_to_late.relative[&GrowthMetric::Health]
            .value()
            .is_some());
    }
}

fn sheet_assets() -> Value {
    serde_json::from_str(include_str!("../testdata/calculation/sheet-6759.json")).unwrap()
}

fn sheet_models() -> CalculationModels {
    let raw = sheet_assets();
    let origin = |kind: &str, hash: &str| ModelSource {
        client_version: raw["provenance"]["client_version"].as_i64().unwrap(),
        document_id: format!("recorded-6759-{hash}"),
        ..source(kind)
    };
    let mut models = calculation_models_from_payloads(
        &raw["heroes"],
        &raw["items"],
        &origin(
            "heroes_all",
            raw["provenance"]["heroes_all_sha256"].as_str().unwrap(),
        ),
        &origin("items", raw["provenance"]["items_sha256"].as_str().unwrap()),
    )
    .unwrap();
    let rebind = |source: &mut ModelSource, pointer: &str| {
        source.json_pointer = match source.json_pointer.splitn(3, '/').nth(2) {
            Some(suffix) => format!("{pointer}/{suffix}"),
            None => pointer.to_owned(),
        };
    };
    for (id, hero) in &mut models.heroes {
        rebind(
            &mut hero.source,
            raw["provenance"]["hero_pointers"][id.to_string()]
                .as_str()
                .unwrap(),
        );
        for (id, source) in &mut hero.ability_sources {
            rebind(
                source,
                raw["provenance"]["item_pointers"][id.to_string()]
                    .as_str()
                    .unwrap(),
            );
        }
    }
    for (id, weapon) in &mut models.weapons {
        let pointer = raw["provenance"]["item_pointers"][id.to_string()]
            .as_str()
            .unwrap();
        rebind(&mut weapon.source, pointer);
        for metric in weapon.raw_metrics.values_mut() {
            if let MeasuredValue::Known { sources, .. } | MeasuredValue::Unknown { sources, .. } =
                metric
            {
                for source in sources {
                    rebind(source, pointer);
                }
            }
        }
    }
    for (id, source) in &mut models.item_sources {
        rebind(
            source,
            raw["provenance"]["item_pointers"][id.to_string()]
                .as_str()
                .unwrap(),
        );
    }
    models
}

fn controlled_deadline(duration: Duration) -> (RequestDeadline, Arc<Mutex<Instant>>) {
    let clock = Arc::new(Mutex::new(Instant::now()));
    let source = clock.clone();
    let deadline = RequestDeadline::after_with_clock(duration, move || *source.lock().unwrap());
    (deadline, clock)
}

fn deadline() -> RequestDeadline {
    controlled_deadline(Duration::from_secs(60)).0
}

fn deadline_with_check_limit(limit: usize) -> (RequestDeadline, Arc<AtomicUsize>) {
    let anchor = Instant::now();
    let checks = Arc::new(AtomicUsize::new(0));
    let observed = checks.clone();
    let deadline = RequestDeadline::after_with_clock(Duration::from_secs(1), move || {
        if observed.fetch_add(1, Ordering::SeqCst) > limit {
            anchor + Duration::from_secs(1)
        } else {
            anchor
        }
    });
    (deadline, checks)
}

fn sheet_scenario() -> CalculationScenario {
    CalculationScenario {
        expected_level: None,
        expected_unspent_ap: None,
        spirit: SpiritInput::Total(38.0),
        ..scenario()
    }
}

fn range(min_boons: usize, max_boons: usize) -> BoonRange {
    BoonRange {
        min_boons,
        max_boons,
    }
}

fn max_rank_order(ability_ids: &[i64]) -> Vec<crate::AbilityStep> {
    ability_ids
        .iter()
        .flat_map(|ability_id| {
            [(2, -1), (1, -1), (1, -2), (1, -5)].map(|(currency_type, delta)| crate::AbilityStep {
                ability_id: *ability_id,
                currency_type,
                delta,
            })
        })
        .collect()
}

#[test]
fn sheet_fixture_retains_recorded_binding_original_objects_and_negative_values() {
    let raw = sheet_assets();
    let models = sheet_models();
    assert_eq!(models.client_version, 6759);
    assert_eq!(models.heroes[&27].source.json_pointer, "/20");
    assert_eq!(
        models.heroes[&27].ability_sources[&2566573207].json_pointer,
        "/264"
    );
    for hero in raw["heroes"].as_array().unwrap() {
        assert_eq!(&models.heroes[&hero["id"].as_i64().unwrap()].raw, hero);
    }
    for item in raw["items"].as_array().unwrap() {
        assert_eq!(&models.item_payloads[&item["id"].as_i64().unwrap()], item);
    }
    let proc = models
        .items
        .iter()
        .find(|item| item.item_id == 395867183)
        .unwrap();
    close(proc.properties["AbilityCooldownBetweenCharge"], -1.0);
    let result = project_hero(&models, 27, &sheet_scenario(), &deadline()).unwrap();
    let negative = &result.ability_views[&3319782965].properties["AbilityCooldownBetweenCharge"];
    close(negative.base.value().unwrap(), -1.0);
    close(negative.value.value().unwrap(), -1.0);
    assert_eq!(negative.raw["value"], "-1.0");
    println!("SHEET_HERKUNFT {}", raw["provenance"]);
}

#[test]
fn recorded_growth_matches_the_same_scalar_and_simulator_at_every_boon() {
    let models = sheet_models();
    let input = sheet_scenario();
    let deadline = deadline();
    for id in [13, 25, 7, 27] {
        let growth = hero_growth(&models, id, &input, range(0, 35), &deadline).unwrap();
        assert_eq!(growth.points.len(), 36);
        assert_eq!(growth.per_boon.len(), 35);
        assert_eq!(growth.scenario, input);
        for point in &growth.points {
            let mut scenario = input.clone();
            scenario.progression = ProgressionInput::Boons(point.boons);
            let scalar = project_hero(&models, id, &scenario, &deadline).unwrap();
            assert!(scalar.combat.is_none());
            close(number(&scalar, "spirit_power"), 38.0);
            for metric in [
                GrowthMetric::WeaponDps,
                GrowthMetric::DamagePerMagazine,
                GrowthMetric::Health,
            ] {
                assert_eq!(point.metrics[&metric], scalar.metrics[metric.name()]);
            }
            if [0, 20, 35].contains(&point.boons) {
                let simulated =
                    calculate_hero_with_deadline(&models, id, &scenario, &deadline).unwrap();
                for metric in [
                    GrowthMetric::WeaponDps,
                    GrowthMetric::DamagePerMagazine,
                    GrowthMetric::Health,
                ] {
                    assert_eq!(point.metrics[&metric], simulated.metrics[metric.name()]);
                }
                println!("WACHSTUM_ABNAHME {}", serde_json::to_string(&json!({"hero_id":id,"boons":point.boons,"total_spirit":38.0,"metrics":point.metrics})).unwrap());
            }
        }
        for change in &growth.per_boon {
            for metric in [
                GrowthMetric::WeaponDps,
                GrowthMetric::DamagePerMagazine,
                GrowthMetric::Health,
            ] {
                let before = growth.points[change.from_boons].metrics[&metric]
                    .value()
                    .unwrap();
                let after = growth.points[change.to_boons].metrics[&metric]
                    .value()
                    .unwrap();
                close(change.absolute[&metric].value().unwrap(), after - before);
                close(
                    change.relative[&metric].value().unwrap(),
                    (after - before) / before,
                );
            }
        }
        let later = hero_growth(&models, id, &input, range(20, 35), &deadline).unwrap();
        assert_eq!(later.points, growth.points[20..]);
        assert_eq!(later.base, growth.base);
        assert_eq!(later.per_boon[0].from_boons, 19);
        for metric in [
            GrowthMetric::WeaponDps,
            GrowthMetric::DamagePerMagazine,
            GrowthMetric::Health,
        ] {
            let early = later.points[0].metrics[&metric].value().unwrap();
            let late = later.points[15].metrics[&metric].value().unwrap();
            close(
                later.early_to_late.absolute[&metric].value().unwrap(),
                late - early,
            );
            close(
                later.early_to_late.relative[&metric].value().unwrap(),
                (late - early) / early,
            );
        }
        println!(
            "WACHSTUM_20_35 {}",
            serde_json::to_string(&json!({"hero_id":id,"change":later.early_to_late})).unwrap()
        );
    }
}

#[test]
fn growth_rejects_invalid_ranges_and_distinguishes_zero_baselines_from_missing_data() {
    let mut models = sheet_models();
    let input = sheet_scenario();
    let deadline = deadline();
    for range in [range(20, 19), range(0, 36), range(0, usize::MAX)] {
        assert!(hero_growth(&models, 13, &input, range, &deadline).is_err());
    }
    let hero = models.heroes.get_mut(&13).unwrap();
    hero.model.base_health = 0.0;
    hero.starting_stats.insert("max_health".into(), 0.0);
    let growth = hero_growth(&models, 13, &input, range(0, 2), &deadline).unwrap();
    close(
        growth.early_to_late.absolute[&GrowthMetric::Health]
            .value()
            .unwrap(),
        66.0,
    );
    assert!(matches!(
        growth.early_to_late.relative[&GrowthMetric::Health],
        MeasuredValue::NotApplicable { .. }
    ));
    models
        .heroes
        .get_mut(&13)
        .unwrap()
        .model
        .standard_level_up_upgrades
        .remove("MODIFIER_VALUE_BASE_HEALTH_FROM_LEVEL");
    let growth = hero_growth(&models, 13, &input, range(0, 2), &deadline).unwrap();
    assert!(matches!(
        growth.early_to_late.relative[&GrowthMetric::Health],
        MeasuredValue::Unknown { .. }
    ));
    assert!(growth.points[2].metrics[&GrowthMetric::WeaponDps]
        .value()
        .is_some());
}

#[test]
fn real_health_curves_keep_exact_ties_and_no_overtake() {
    let models = sheet_models();
    let input = sheet_scenario();
    let deadline = deadline();
    let comparison = compare_hero_curves(
        &models,
        [13, 7],
        &input,
        range(0, 35),
        GrowthMetric::Health,
        &deadline,
    )
    .unwrap();
    assert_eq!(comparison.status, CurveComparisonStatus::NoOvertake);
    assert_eq!(comparison.tied_boons, [0]);
    assert_eq!(comparison.points[0].leader, CurveLeader::Tie);
    assert!(comparison.points[1..]
        .iter()
        .all(|point| point.leader == CurveLeader::Right));
    assert!(comparison.overtakes.is_empty());
    let comparison = compare_hero_curves(
        &models,
        [13, 25],
        &input,
        range(0, 35),
        GrowthMetric::Health,
        &deadline,
    )
    .unwrap();
    assert_eq!(comparison.status, CurveComparisonStatus::NoOvertake);
    assert!(comparison.tied_boons.is_empty());
    assert!(compare_hero_curves(
        &models,
        [13, 13],
        &input,
        range(0, 1),
        GrowthMetric::Health,
        &deadline
    )
    .is_err());
    println!(
        "KURVEN_ABNAHME {}",
        serde_json::to_string(&comparison).unwrap()
    );
}

#[test]
fn controlled_curve_edges_distinguish_all_ties_and_ties_between_strict_leaders() {
    let mut models = sheet_models();
    let input = sheet_scenario();
    let deadline = deadline();
    let left = models.heroes.get_mut(&13).unwrap();
    left.model
        .standard_level_up_upgrades
        .insert("MODIFIER_VALUE_BASE_HEALTH_FROM_LEVEL".into(), 35.0);
    let tied = compare_hero_curves(
        &models,
        [13, 7],
        &input,
        range(0, 4),
        GrowthMetric::Health,
        &deadline,
    )
    .unwrap();
    assert_eq!(tied.status, CurveComparisonStatus::AllTied);
    assert_eq!(tied.tied_boons, [0, 1, 2, 3, 4]);
    let left = models.heroes.get_mut(&13).unwrap();
    left.model.base_health = 733.0;
    left.starting_stats.insert("max_health".into(), 733.0);
    left.model
        .standard_level_up_upgrades
        .insert("MODIFIER_VALUE_BASE_HEALTH_FROM_LEVEL".into(), 32.0);
    let crossed = compare_hero_curves(
        &models,
        [13, 7],
        &input,
        range(0, 4),
        GrowthMetric::Health,
        &deadline,
    )
    .unwrap();
    assert_eq!(crossed.status, CurveComparisonStatus::Overtakes);
    assert_eq!(crossed.overtakes.len(), 1);
    assert_eq!(crossed.overtakes[0].last_strict_boons, 0);
    assert_eq!(crossed.overtakes[0].first_strict_boons, 2);
    assert_eq!(crossed.overtakes[0].tied_boons, [1]);
    assert_eq!(crossed.overtakes[0].from, CurveLeader::Left);
    assert_eq!(crossed.overtakes[0].to, CurveLeader::Right);
}

#[test]
fn controlled_discrete_magazines_produce_multiple_overtakes_without_interpolation() {
    let mut models = sheet_models();
    for (id, damage, spirit) in [(13, 11.0, 0.0), (7, 10.0, 1.0)] {
        let hero = models.heroes.get_mut(&id).unwrap();
        hero.starting_stats.insert("tech_power".into(), spirit);
        hero.model.base_spirit_power = spirit;
        hero.model.scaling.retain(|scale| scale.stat != "EClipSize");
        hero.model.scaling.push(crate::ScalingStat {
            stat: "EClipSize".into(),
            per_level: 0.0,
            per_spirit: Some(0.5),
        });
        hero.model
            .standard_level_up_upgrades
            .insert("MODIFIER_VALUE_TECH_POWER".into(), 1.0);
        hero.model
            .standard_level_up_upgrades
            .insert("MODIFIER_VALUE_BASE_BULLET_DAMAGE_FROM_LEVEL".into(), 0.1);
        let weapon = models
            .weapons
            .get_mut(&hero.primary_weapon.unwrap())
            .unwrap();
        weapon.profile.bullet_damage = damage;
        weapon.profile.clip_size = 1.0;
        weapon.raw["weapon_info"]["bullet_damage"] = json!(damage);
        weapon.raw["weapon_info"]["clip_size"] = json!(1);
    }
    let input = CalculationScenario {
        spirit: SpiritInput::Derived,
        ..sheet_scenario()
    };
    let comparison = compare_hero_curves(
        &models,
        [13, 7],
        &input,
        range(0, 4),
        GrowthMetric::DamagePerMagazine,
        &deadline(),
    )
    .unwrap();
    assert_eq!(comparison.status, CurveComparisonStatus::Overtakes);
    assert_eq!(
        comparison
            .points
            .iter()
            .map(|point| point.leader)
            .collect::<Vec<_>>(),
        [
            CurveLeader::Left,
            CurveLeader::Right,
            CurveLeader::Left,
            CurveLeader::Right,
            CurveLeader::Left
        ]
    );
    assert_eq!(comparison.overtakes.len(), 4);
    for (index, crossing) in comparison.overtakes.iter().enumerate() {
        assert_eq!(crossing.last_strict_boons, index);
        assert_eq!(crossing.first_strict_boons, index + 1);
        assert!(crossing.tied_boons.is_empty());
    }
}

#[test]
fn controlled_progression_gap_breaks_overtake_claims_instead_of_bridging_missing_points() {
    let mut models = sheet_models();
    let left = models.heroes.get_mut(&13).unwrap();
    left.model.base_health = 734.0;
    left.starting_stats.insert("max_health".into(), 734.0);
    let next_souls = left
        .model
        .level_curve
        .iter()
        .find(|point| point.level == 4)
        .unwrap()
        .required_souls;
    left.model
        .level_curve
        .iter_mut()
        .find(|point| point.level == 3)
        .unwrap()
        .required_souls = next_souls;
    let comparison = compare_hero_curves(
        &models,
        [13, 7],
        &sheet_scenario(),
        range(0, 4),
        GrowthMetric::Health,
        &deadline(),
    )
    .unwrap();
    assert_eq!(comparison.status, CurveComparisonStatus::Incomplete);
    assert_eq!(comparison.points[1].leader, CurveLeader::Left);
    assert_eq!(comparison.points[2].leader, CurveLeader::Unknown);
    assert_eq!(comparison.points[3].leader, CurveLeader::Right);
    assert!(comparison.overtakes.is_empty());
}

#[test]
fn every_new_calculation_entry_observes_shared_cancellation_and_expiry() {
    let models = sheet_models();
    let input = sheet_scenario();
    let sheet = SheetComparisonInput {
        scenario: input.clone(),
        contributions: None,
        sheet_shred: None,
        mode: ContributionComparison::HoldFixed,
    };
    let (cancelled, _) = controlled_deadline(Duration::from_secs(1));
    let clone = cancelled.clone();
    assert_eq!(clone.remaining(), Ok(Duration::from_secs(1)));
    cancelled.cancel();
    let (expired, clock) = controlled_deadline(Duration::from_secs(1));
    let expired_clone = expired.clone();
    *clock.lock().unwrap() += Duration::from_secs(1);
    assert_eq!(
        expired.check(),
        Err(brain_contracts::PortError::BudgetExceeded)
    );
    assert_eq!(expired_clone.check(), expired.check());
    for deadline in [&clone, &expired_clone] {
        assert!(project_hero(&models, 27, &input, deadline).is_err());
        assert!(calculate_hero_with_deadline(&models, 27, &input, deadline).is_err());
        assert!(hero_growth(&models, 27, &input, range(0, 35), deadline).is_err());
        assert!(compare_hero_curves(
            &models,
            [13, 25],
            &input,
            range(0, 35),
            GrowthMetric::WeaponDps,
            deadline
        )
        .is_err());
        assert!(compare_sheet_scenarios(&models, 27, &sheet, deadline).is_err());
        let hero = &models.heroes[&27];
        assert!(crate::combat::simulate_calculation_with_deadline(
            &hero.model,
            &[],
            &input,
            &models.weapons[&hero.primary_weapon.unwrap()].timing,
            &hero.starting_stats,
            deadline
        )
        .is_err());
    }
}

#[test]
fn calculation_deadline_expires_inside_real_simulation_and_inventory_coverage() {
    let models = sheet_models();
    for use_abilities in [false, true] {
        let input = CalculationScenario {
            use_abilities,
            target: CalculationTarget {
                health: 1_000_000.0,
                ..sheet_scenario().target
            },
            window_seconds: 5.0,
            ..sheet_scenario()
        };
        let (projection_deadline, projection_checks) = deadline_with_check_limit(usize::MAX);
        project_hero(&models, 27, &input, &projection_deadline).unwrap();
        let projection_checks = projection_checks.load(Ordering::SeqCst) - 1;
        let (positive, positive_checks) = deadline_with_check_limit(usize::MAX);
        let result = calculate_hero_with_deadline(&models, 27, &input, &positive).unwrap();
        assert!(result.combat.is_some());
        let limit = projection_checks + 3;
        assert!(positive_checks.load(Ordering::SeqCst) > limit + 2);
        let (interrupted, interrupted_checks) = deadline_with_check_limit(limit);
        let clone = interrupted.clone();
        let result = calculate_hero_with_deadline(&models, 27, &input, &clone);
        assert!(matches!(result, Err(crate::ReasonerError::Data(_))));
        let consumed = interrupted_checks.load(Ordering::SeqCst);
        assert!(consumed > limit + 2);
        assert_eq!(
            interrupted.check(),
            Err(brain_contracts::PortError::BudgetExceeded)
        );
        println!(
            "RECHENFRIST use_abilities={use_abilities} projection_checks={projection_checks} positive_checks={} limit={limit} interrupted_checks={consumed}",
            positive_checks.load(Ordering::SeqCst)
        );
    }
}

#[test]
fn growth_curve_and_sheet_do_not_replace_a_deadline_between_projections() {
    let models = sheet_models();
    let input = sheet_scenario();
    let sheet = sheet_input();
    for entry in ["growth", "curve", "sheet"] {
        let (positive, positive_checks) = deadline_with_check_limit(usize::MAX);
        let run = |deadline: &RequestDeadline| match entry {
            "growth" => hero_growth(&models, 27, &input, range(0, 35), deadline).map(|_| ()),
            "curve" => compare_hero_curves(
                &models,
                [13, 25],
                &input,
                range(0, 35),
                GrowthMetric::WeaponDps,
                deadline,
            )
            .map(|_| ()),
            _ => compare_sheet_scenarios(&models, 27, &sheet, deadline).map(|_| ()),
        };
        run(&positive).unwrap();
        let positive_checks = positive_checks.load(Ordering::SeqCst);
        let limit = positive_checks / 2;
        let (interrupted, interrupted_checks) = deadline_with_check_limit(limit);
        let clone = interrupted.clone();
        assert!(matches!(run(&clone), Err(crate::ReasonerError::Data(_))));
        assert!(interrupted_checks.load(Ordering::SeqCst) > limit + 1);
        assert_eq!(
            interrupted.check(),
            Err(brain_contracts::PortError::BudgetExceeded)
        );
        println!(
            "GETEILTE_FRIST {entry} positive_checks={positive_checks} limit={limit} interrupted_checks={}",
            interrupted_checks.load(Ordering::SeqCst)
        );
    }
}

#[test]
fn recorded_yamato_five_blocks_keep_zero_non_spirit_scaling_and_separate_healing_state() {
    let models = sheet_models();
    let input = sheet_scenario();
    let deadline = deadline();
    for boons in [0, 20, 35] {
        let scenario = CalculationScenario {
            progression: ProgressionInput::Boons(boons),
            ..input.clone()
        };
        let result = project_hero(&models, 27, &scenario, &deadline).unwrap();
        let melee = &result.ability_views[&3334760137];
        assert_eq!(melee.reference, "/items/weapon_melee");
        assert_eq!(melee.kind, AbilityBlockKind::Melee);
        assert_eq!(melee.source.json_pointer, "/262");
        close(
            melee.properties["light_melee_damage"]
                .value
                .value()
                .unwrap(),
            55.0 + boons as f64 * 1.58,
        );
        let fly = &result.ability_views[&2566573207];
        assert_eq!(fly.kind, AbilityBlockKind::Damage);
        close(fly.properties["Damage"].base.value().unwrap(), 0.0);
        assert_eq!(fly.properties["Damage"].raw["value"], "0");
        let scale = fly.properties["Damage"].scale.as_ref().unwrap();
        assert_eq!(scale.class, AbilityScaleClass::SingleStat);
        assert_eq!(scale.input, AbilityScaleInput::LightMeleeDamage);
        close(scale.coefficient.unwrap(), 1.2);
        close(
            fly.properties["Damage"].value.value().unwrap(),
            (55.0 + boons as f64 * 1.58) * 1.2,
        );
        close(
            result.ability_views[&3255651252].properties["FullChargeDamage"]
                .value
                .value()
                .unwrap(),
            215.3,
        );
        let crimson = &result.ability_views[&2366960452];
        close(crimson.properties["Damage"].value.value().unwrap(), 69.06);
        assert_eq!(
            crimson.properties["HealFixedHealth"].kind,
            AbilityPropertyKind::Healing
        );
        close(
            crimson.properties["HealFixedHealth"].base.value().unwrap(),
            55.0,
        );
        assert!(crimson.properties["HealFixedHealth"]
            .value
            .value()
            .is_none());
        assert_eq!(
            crimson.properties["HealFixedHealth"]
                .scale
                .as_ref()
                .unwrap()
                .class,
            AbilityScaleClass::HealingSpirit
        );
        assert_eq!(
            result.ability_views[&3319782965].kind,
            AbilityBlockKind::State
        );
        close(
            result.ability_views[&3319782965].properties["AbilityDuration"]
                .value
                .value()
                .unwrap(),
            5.0,
        );
        close(
            result.ability_views[&3319782965].properties["WeaponDamageBonus"]
                .base
                .value()
                .unwrap(),
            0.0,
        );
        println!(
            "DNS_ABNAHME {}",
            serde_json::to_string(
                &json!({"boons":boons,"hero_id":27,"ability_views":result.ability_views})
            )
            .unwrap()
        );
    }
    let source = &project_hero(&models, 27, &input, &deadline)
        .unwrap()
        .ability_views[&2566573207]
        .properties["Damage"]
        .value;
    let MeasuredValue::Known { sources, .. } = source else {
        panic!("Flying Slash nicht projiziert")
    };
    assert!(sources
        .iter()
        .any(|source| source.json_pointer == "/264/properties/Damage"));
    assert!(sources
        .iter()
        .any(|source| source.kind == "heroes_all" && source.json_pointer == "/20"));
    let mut missing = models.clone();
    missing
        .heroes
        .get_mut(&27)
        .unwrap()
        .starting_stats
        .remove("light_melee_damage");
    let result = project_hero(&missing, 27, &input, &deadline).unwrap();
    assert!(result.ability_views[&2566573207].properties["Damage"]
        .value
        .value()
        .is_none());
    close(number(&result, "health"), 730.0);
    close(
        result.ability_views[&3255651252].properties["FullChargeDamage"]
            .value
            .value()
            .unwrap(),
        215.3,
    );
}

#[test]
fn starting_spirit_and_boon_bonuses_accumulate_without_erasing_unknown_inputs() {
    let mut raw = sheet_assets();
    let hero = raw["heroes"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|hero| hero["id"] == 27)
        .unwrap();
    hero["scaling_stats"]["ELightMeleeDamage"] = json!({"scaling_stat":"ETechPower","scale":2.0});
    hero["scaling_stats"]["EOOCHealthRegen"] = json!({"scaling_stat":"ETechPower","scale":0.25});
    hero["starting_stats"]["ooc_health_regen"]["value"] = json!(3.0);
    hero["standard_level_up_upgrades"]["MODIFIER_VALUE_OUT_OF_COMBAT_HEALTH_REGEN"] = json!(0.4);
    hero["starting_stats"]
        .as_object_mut()
        .unwrap()
        .remove("tech_power");
    let origin = |kind: &str| ModelSource {
        client_version: 6759,
        ..source(kind)
    };
    let models = calculation_models_from_payloads(
        &raw["heroes"],
        &raw["items"],
        &origin("heroes_all"),
        &origin("items"),
    )
    .unwrap();
    let deadline = deadline();
    for boons in [0, 20, 35] {
        for spirit in [0.0, 38.0] {
            let input = CalculationScenario {
                progression: ProgressionInput::Boons(boons),
                spirit: SpiritInput::Total(spirit),
                ..sheet_scenario()
            };
            let projected = project_hero(&models, 27, &input, &deadline).unwrap();
            let simulated = calculate_hero_with_deadline(&models, 27, &input, &deadline).unwrap();
            let melee = 55.0 + 2.0 * spirit + boons as f64 * 1.58;
            let regeneration = 3.0 + 0.25 * spirit + boons as f64 * 0.4;
            for result in [&projected, &simulated] {
                close(number(result, "starting.light_melee_damage"), melee);
                close(number(result, "starting.ooc_health_regen"), regeneration);
                close(
                    result.ability_views[&2566573207].properties["Damage"]
                        .value
                        .value()
                        .unwrap(),
                    melee * 1.2,
                );
            }
            println!("SPIRIT_BOON_MUTATION boons={boons} spirit={spirit} melee={melee} regeneration={regeneration} flying_slash={}", melee * 1.2);
        }
        let input = CalculationScenario {
            progression: ProgressionInput::Boons(boons),
            spirit: SpiritInput::Derived,
            ..sheet_scenario()
        };
        let result = project_hero(&models, 27, &input, &deadline).unwrap();
        for metric in ["starting.light_melee_damage", "starting.ooc_health_regen"] {
            assert!(matches!(
                result.metrics[metric],
                MeasuredValue::Unknown { .. }
            ));
        }
        assert!(matches!(
            result.ability_views[&2566573207].properties["Damage"].value,
            MeasuredValue::Unknown { .. }
        ));
    }
}

#[test]
fn zero_light_melee_coefficient_needs_no_missing_or_unknown_melee_metric() {
    for missing_metric in [true, false] {
        let mut raw = sheet_assets();
        let hero = raw["heroes"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|hero| hero["id"] == 27)
            .unwrap();
        if missing_metric {
            hero["starting_stats"]
                .as_object_mut()
                .unwrap()
                .remove("light_melee_damage");
        } else {
            hero["scaling_stats"]["ELightMeleeDamage"] =
                json!({"scaling_stat":"ETechPower","scale":2.0});
            hero["starting_stats"]
                .as_object_mut()
                .unwrap()
                .remove("tech_power");
        }
        let ability = raw["items"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|item| item["id"] == 2566573207_i64)
            .unwrap();
        ability["properties"]["Damage"]["value"] = json!(17.0);
        ability["properties"]["Damage"]["scale_function"]["stat_scale"] = json!(0.0);
        let origin = |kind: &str| ModelSource {
            client_version: 6759,
            ..source(kind)
        };
        let models = calculation_models_from_payloads(
            &raw["heroes"],
            &raw["items"],
            &origin("heroes_all"),
            &origin("items"),
        )
        .unwrap();
        let input = CalculationScenario {
            spirit: SpiritInput::Derived,
            ..sheet_scenario()
        };
        let result = project_hero(&models, 27, &input, &deadline()).unwrap();
        assert!(result
            .metrics
            .get("starting.light_melee_damage")
            .and_then(MeasuredValue::value)
            .is_none());
        close(
            result.ability_views[&2566573207].properties["Damage"]
                .value
                .value()
                .unwrap(),
            17.0,
        );
    }
}

#[test]
fn recorded_yamato_upgrades_keep_scale_filters_healing_gaps_and_state_bonuses() {
    let models = sheet_models();
    let mut input = sheet_scenario();
    input.progression = ProgressionInput::Boons(35);
    input.ability_order = max_rank_order(&[3255651252, 2566573207, 2366960452, 3319782965]);
    let result = project_hero(&models, 27, &input, &deadline()).unwrap();
    let power = &result.ability_views[&3255651252];
    assert_eq!(power.applied_rank, 3);
    close(
        power.properties["FullChargeDamage"].base.value().unwrap(),
        295.0,
    );
    close(
        power.properties["FullChargeDamage"]
            .scale
            .as_ref()
            .unwrap()
            .coefficient
            .unwrap(),
        2.35,
    );
    close(
        power.properties["FullChargeDamage"].value.value().unwrap(),
        384.3,
    );
    let crimson = &result.ability_views[&2366960452];
    close(crimson.properties["Damage"].value.value().unwrap(), 91.86);
    close(
        crimson.properties["HealMaxHealth"].base.value().unwrap(),
        6.0,
    );
    assert!(crimson.properties["HealFixedHealth"]
        .value
        .value()
        .is_none());
    assert!(crimson.properties["HealMaxHealth"].value.value().is_none());
    let shadow = &result.ability_views[&3319782965];
    assert_eq!(shadow.kind, AbilityBlockKind::State);
    assert_eq!(
        shadow.properties["WeaponDamageBonus"].kind,
        AbilityPropertyKind::State
    );
    close(
        shadow.properties["WeaponDamageBonus"]
            .value
            .value()
            .unwrap(),
        7.0,
    );
    close(
        shadow.properties["AbilityDuration"].value.value().unwrap(),
        8.0,
    );
    close(
        shadow.properties["BulletResist"].value.value().unwrap(),
        60.0,
    );
    close(shadow.properties["TechResist"].value.value().unwrap(), 60.0);
    close(
        shadow.properties["AbilityCooldown"].value.value().unwrap(),
        130.0,
    );
    println!(
        "DNS_T3_ABNAHME {}",
        serde_json::to_string(&result.ability_views).unwrap()
    );
    input.spirit = SpiritInput::Total(168.0);
    let result = project_hero(&models, 27, &input, &deadline()).unwrap();
    close(
        result.ability_views[&3255651252].properties["FullChargeDamage"]
            .value
            .value()
            .unwrap(),
        689.8,
    );
}

#[test]
fn recorded_fixation_non_spirit_upgrade_retains_flat_base_and_unknown_conversion() {
    let models = sheet_models();
    let mut input = sheet_scenario();
    let base = project_hero(&models, 13, &input, &deadline()).unwrap();
    close(
        base.ability_views[&1080948381].properties["DamageBonusFixedPerStack"]
            .value
            .value()
            .unwrap(),
        0.2,
    );
    input.progression = ProgressionInput::Boons(35);
    input.ability_order = max_rank_order(&[1080948381]);
    let result = project_hero(&models, 13, &input, &deadline()).unwrap();
    let property = &result.ability_views[&1080948381].properties["DamageBonusFixedPerStack"];
    close(property.base.value().unwrap(), 0.31);
    assert_eq!(
        property.scale.as_ref().unwrap().input,
        AbilityScaleInput::BaseWeaponDamageIncrease
    );
    close(
        property.scale.as_ref().unwrap().coefficient.unwrap(),
        0.00035,
    );
    assert!(property.value.value().is_none());
    assert!(result
        .progression
        .unknown_effects
        .iter()
        .any(|effect| effect.contains("Nicht-Spirit")));
    println!(
        "FIXATION_SCALE_ABNAHME {}",
        serde_json::to_string(property).unwrap()
    );
}

fn sheet_input() -> SheetComparisonInput {
    let mut scenario = sheet_scenario();
    scenario.weapon_bonus_percent = Some(50.0);
    scenario.fire_rate_bonus_percent = Some(25.0);
    SheetComparisonInput {
        scenario,
        contributions: Some(vec![PerShotContribution::AbilityProperty {
            ability_id: 2566573207,
            property: "Damage".into(),
            occurrences_per_shot: 1.0,
        }]),
        sheet_shred: Some(0.2),
        mode: ContributionComparison::HoldFixed,
    }
}

#[test]
fn recorded_sheet_counterfactuals_use_three_preserved_formulas_and_explicit_q() {
    let models = sheet_models();
    let input = sheet_input();
    let deadline = deadline();
    let baseline = project_hero(&models, 27, &sheet_scenario(), &deadline).unwrap();
    let b = number(&baseline, "damage_per_shot");
    let r0 = number(&baseline, "shots_per_second");
    let q = 66.0;
    let result = compare_sheet_scenarios(&models, 27, &input, &deadline).unwrap();
    for (variant, expected) in [
        (SheetVariant::Full, r0 * 1.25 * (b * 1.5 + q) * 1.2),
        (SheetVariant::WithoutWeaponBonus, r0 * 1.25 * (b + q) * 1.2),
        (SheetVariant::WithoutFireRateBonus, r0 * (b * 1.5 + q) * 1.2),
        (SheetVariant::Baseline, r0 * (q + b)),
    ] {
        close(
            result.variants[&variant].sheet_dps.value().unwrap(),
            expected,
        );
        close(
            result.variants[&variant]
                .additional_damage_per_shot
                .value()
                .unwrap(),
            q,
        );
    }
    let mut reevaluate = input.clone();
    reevaluate.mode = ContributionComparison::Reevaluate;
    let recalculated = compare_sheet_scenarios(&models, 27, &reevaluate, &deadline).unwrap();
    for variant in result.variants.keys() {
        assert_eq!(
            result.variants[variant].sheet_dps,
            recalculated.variants[variant].sheet_dps
        );
    }
    println!(
        "SCRATCHPAD_ABNAHME {}",
        serde_json::to_string(&result).unwrap()
    );
    let mut haze_input = input;
    haze_input.contributions = Some(vec![PerShotContribution::AbilityProperty {
        ability_id: 1080948381,
        property: "DamageBonusFixedPerStack".into(),
        occurrences_per_shot: 1.0,
    }]);
    let haze = compare_sheet_scenarios(&models, 13, &haze_input, &deadline).unwrap();
    close(
        haze.variants[&SheetVariant::Full]
            .additional_damage_per_shot
            .value()
            .unwrap(),
        0.2,
    );
}

#[test]
fn missing_scratchpad_inputs_are_not_zeroes_and_only_block_dependent_formulas() {
    let models = sheet_models();
    let mut input = sheet_input();
    input.contributions = None;
    let result = compare_sheet_scenarios(&models, 27, &input, &deadline()).unwrap();
    assert!(result
        .variants
        .values()
        .all(|variant| variant.sheet_dps.value().is_none()));
    assert!(result.variants.values().all(|variant| matches!(
        variant.additional_damage_per_shot,
        MeasuredValue::Unknown { .. }
    )));
    input = sheet_input();
    input.sheet_shred = None;
    let result = compare_sheet_scenarios(&models, 27, &input, &deadline()).unwrap();
    assert!(result.variants[&SheetVariant::WithoutWeaponBonus]
        .sheet_dps
        .value()
        .is_none());
    assert!(result.variants[&SheetVariant::WithoutFireRateBonus]
        .sheet_dps
        .value()
        .is_none());
    assert!(result.variants[&SheetVariant::Baseline]
        .sheet_dps
        .value()
        .is_some());
    input.contributions = Some(Vec::new());
    let result = compare_sheet_scenarios(&models, 27, &input, &deadline()).unwrap();
    close(
        result.variants[&SheetVariant::Full]
            .additional_damage_per_shot
            .value()
            .unwrap(),
        0.0,
    );
    input = sheet_input();
    input.contributions = Some(vec![PerShotContribution::AbilityProperty {
        ability_id: 3319782965,
        property: "WeaponDamageBonus".into(),
        occurrences_per_shot: 1.0,
    }]);
    assert!(compare_sheet_scenarios(&models, 27, &input, &deadline()).is_err());
    input = sheet_input();
    input.contributions = Some(vec![PerShotContribution::AbilityProperty {
        ability_id: 2366960452,
        property: "HealFixedHealth".into(),
        occurrences_per_shot: 1.0,
    }]);
    assert!(compare_sheet_scenarios(&models, 27, &input, &deadline()).is_err());
    input = sheet_input();
    input.scenario.weapon_bonus_percent = Some(f64::NAN);
    assert!(compare_sheet_scenarios(&models, 27, &input, &deadline()).is_err());
}

#[test]
fn total_weapon_override_unblocks_only_its_axis_when_shop_curve_is_missing() {
    let mut models = sheet_models();
    models
        .heroes
        .get_mut(&25)
        .unwrap()
        .model
        .cost_bonuses
        .remove("weapon");
    let mut item = models
        .items
        .iter()
        .find(|item| item.item_id == 395867183)
        .unwrap()
        .clone();
    item.slot = crate::SlotType::Weapon;
    models
        .items
        .retain(|existing| existing.item_id != item.item_id);
    models.items.push(item);
    let mut input = sheet_scenario();
    input.item_ids = vec![395867183];
    let result = project_hero(&models, 25, &input, &deadline()).unwrap();
    assert!(result.metrics["weapon_bonus_percent"].value().is_none());
    assert!(result.metrics["damage_per_shot"].value().is_none());
    input.weapon_bonus_percent = Some(0.0);
    let result = project_hero(&models, 25, &input, &deadline()).unwrap();
    close(number(&result, "weapon_bonus_percent"), 0.0);
    close(number(&result, "damage_per_shot"), 17.34);
    assert!(result.shop_bonuses["weapon"].value().is_none());
}

#[test]
fn recorded_proc_counterfactuals_reevaluate_events_without_claiming_raw_sheet_dps() {
    let models = sheet_models();
    let mut scenario = sheet_scenario();
    scenario.item_ids = vec![395867183];
    scenario.target.health = 1_000_000.0;
    scenario.target.spirit.resist = 0.25;
    scenario.window_seconds = 5.0;
    scenario.fire_rate_bonus_percent = Some(100.0);
    let mut input = SheetComparisonInput {
        scenario,
        contributions: Some(vec![PerShotContribution::SimulatedProc]),
        sheet_shred: Some(0.2),
        mode: ContributionComparison::HoldFixed,
    };
    let deadline = deadline();
    let probe = calculate_hero_with_deadline(&models, 25, &input.scenario, &deadline).unwrap();
    println!("PROC_ABDECKUNG {}", serde_json::to_string(&json!({"unknowns":probe.unknowns,"proc_damage":probe.metrics.get("simulated_proc_damage"),"combat":probe.combat})).unwrap());
    let fixed = compare_sheet_scenarios(&models, 25, &input, &deadline).unwrap();
    input.mode = ContributionComparison::Reevaluate;
    let reevaluated = compare_sheet_scenarios(&models, 25, &input, &deadline).unwrap();
    for variant in reevaluated.variants.values() {
        assert!(variant.sheet_dps.value().is_none());
        assert_eq!(
            variant.additional_damage_per_shot.unit(),
            "effective_damage/shot"
        );
        assert!(matches!(
            variant.simulated_proc_damage.as_ref().unwrap(),
            MeasuredValue::Unknown { .. }
        ));
        assert!(matches!(
            variant.simulated_damage_per_second.as_ref().unwrap(),
            MeasuredValue::Unknown { .. }
        ));
        assert!(variant
            .unknowns
            .iter()
            .any(|reason| reason.contains("Mystic Shot: Radius")));
        assert!(variant.combat.as_ref().unwrap().proc_damage > 0.0);
    }
    assert_eq!(
        fixed.variants[&SheetVariant::Full].additional_damage_per_shot,
        fixed.variants[&SheetVariant::WithoutFireRateBonus].additional_damage_per_shot,
    );
    let full = reevaluated.variants[&SheetVariant::Full]
        .combat
        .as_ref()
        .unwrap();
    let without_rate = reevaluated.variants[&SheetVariant::WithoutFireRateBonus]
        .combat
        .as_ref()
        .unwrap();
    close(full.proc_damage, 166.95);
    close(full.shots, 18.0);
    assert_ne!(full.proc_damage, without_rate.proc_damage);
    assert_ne!(
        full.proc_damage / full.shots,
        without_rate.proc_damage / without_rate.shots
    );
    println!(
        "PROC_GEGENVERGLEICH {}",
        serde_json::to_string(&reevaluated).unwrap()
    );
}

#[test]
fn proc_without_known_deterministic_chance_or_cooldown_stays_unconfirmed() {
    let mut models = sheet_models();
    let mut input = sheet_scenario();
    input.item_ids = vec![395867183];
    input.target.health = 1_000_000.0;
    input.window_seconds = 1.0;
    let item = models
        .items
        .iter_mut()
        .find(|item| item.item_id == 395867183)
        .unwrap();
    item.properties.insert("ProcChance".into(), 50.0);
    let result = calculate_hero_with_deadline(&models, 25, &input, &deadline()).unwrap();
    assert!(matches!(
        result.metrics["simulated_proc_damage"],
        MeasuredValue::Unknown { .. }
    ));
    assert!(result
        .unknowns
        .iter()
        .any(|reason| reason.contains("ProcChance")));
    let item = models
        .items
        .iter_mut()
        .find(|item| item.item_id == 395867183)
        .unwrap();
    item.properties.insert("ProcChance".into(), 100.0);
    item.proc_cooldown = None;
    let result = calculate_hero_with_deadline(&models, 25, &input, &deadline()).unwrap();
    assert!(matches!(
        result.metrics["simulated_proc_damage"],
        MeasuredValue::Unknown { .. }
    ));
    assert!(result
        .unknowns
        .iter()
        .any(|reason| reason.contains("ProcCooldown")));
}
