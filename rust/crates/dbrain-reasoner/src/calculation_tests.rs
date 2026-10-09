use std::{
    collections::BTreeMap,
    time::{Duration, Instant},
};

use brain_contracts::RequestDeadline;
use serde_json::Value;

use crate::{
    calculate_hero, calculation_models_from_payloads, rank_heroes, CalculationModels,
    CalculationScenario, CalculationTarget, DamageModifiers, MetricDirection, ModelSource,
    ProgressionInput, ReloadConvention, SpiritInput,
};

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

fn models() -> CalculationModels {
    let raw: Value =
        serde_json::from_str(include_str!("../testdata/calculation/recorded-assets.json")).unwrap();
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

fn close(left: f64, right: f64) {
    assert!((left - right).abs() < 1e-8, "{left} != {right}");
}

fn deadline() -> RequestDeadline {
    let now = Instant::now();
    RequestDeadline::after_with_clock(Duration::from_secs(1), move || now)
}

#[test]
fn growth_sweeps_clear_both_original_progression_assertions() {
    let models = models();
    let deadline = deadline();
    let range = crate::BoonRange {
        min_boons: 1,
        max_boons: 3,
    };
    for (progression, level, ap) in [
        (ProgressionInput::Boons(0), 1, 0),
        (ProgressionInput::Souls(900), 4, 2),
    ] {
        for assertions in [
            (Some(level), None),
            (None, Some(ap)),
            (Some(level), Some(ap)),
        ] {
            let mut input = scenario();
            input.progression = progression;
            input.expected_level = assertions.0;
            input.expected_unspent_ap = assertions.1;
            let original = input.clone();
            let growth = crate::hero_growth(&models, 2, &input, range, &deadline).unwrap();
            assert_eq!(input, original);
            assert_eq!(growth.scenario, original);
            assert_eq!(growth.points.len(), 3);
            assert_eq!(growth.per_boon.len(), 3);
            for point in std::iter::once(&growth.base).chain(&growth.points) {
                let mut direct = input.clone();
                direct.progression = ProgressionInput::Boons(point.boons);
                direct.expected_level = Some(point.boons as i64 + 1);
                direct.expected_unspent_ap = Some([0, 1, 1, 2][point.boons]);
                let projected = crate::project_hero(&models, 2, &direct, &deadline).unwrap();
                for (metric, value) in &point.metrics {
                    assert!(value.value().is_some(), "{metric:?}: {value:?}");
                    assert_eq!(value, &projected.metrics[metric.name()]);
                }
            }
            for change in growth.per_boon.iter().chain([&growth.early_to_late]) {
                assert!(change
                    .absolute
                    .values()
                    .all(|value| value.value().is_some()));
                assert!(change
                    .relative
                    .values()
                    .all(|value| value.value().is_some()));
            }
        }
    }
}

#[test]
fn curve_comparison_clears_stale_assertions_for_both_hero_sweeps() {
    let models = models();
    let deadline = deadline();
    let range = crate::BoonRange {
        min_boons: 0,
        max_boons: 3,
    };
    let input = scenario();
    for metric in [
        crate::GrowthMetric::WeaponDps,
        crate::GrowthMetric::DamagePerMagazine,
        crate::GrowthMetric::Health,
    ] {
        let comparison =
            crate::compare_hero_curves(&models, [2, 6], &input, range, metric, &deadline).unwrap();
        assert_ne!(comparison.status, crate::CurveComparisonStatus::Incomplete);
        assert_eq!(comparison.points.len(), 4);
        assert_eq!(comparison.left.scenario, input);
        assert_eq!(comparison.right.scenario, input);
        for point in &comparison.points {
            let left = comparison.left.points[point.boons].metrics[&metric]
                .value()
                .unwrap();
            let right = comparison.right.points[point.boons].metrics[&metric]
                .value()
                .unwrap();
            close(point.difference.value().unwrap(), left - right);
            assert_eq!(
                point.leader,
                if left > right {
                    crate::CurveLeader::Left
                } else if left < right {
                    crate::CurveLeader::Right
                } else {
                    crate::CurveLeader::Tie
                }
            );
        }
    }
}

#[test]
fn direct_projection_still_checks_level_and_unspent_ap_assertions() {
    let models = models();
    let deadline = deadline();
    for assertions in [(Some(2), Some(0)), (Some(1), Some(1))] {
        let mut input = scenario();
        input.expected_level = assertions.0;
        input.expected_unspent_ap = assertions.1;
        assert!(crate::project_hero(&models, 2, &input, &deadline).is_err());
        assert!(calculate_hero(&models, 2, &input).is_err());
    }
    assert!(crate::project_hero(&models, 2, &scenario(), &deadline).is_ok());
}

#[test]
fn growth_sweeps_preserve_the_skill_order_guard() {
    let models = models();
    let deadline = deadline();
    let mut input = scenario();
    input.ability_order = vec![crate::AbilityStep {
        ability_id: models.heroes[&2].model.abilities[0].ability_id,
        currency_type: 1,
        delta: -1,
    }];
    let growth = crate::hero_growth(
        &models,
        2,
        &input,
        crate::BoonRange {
            min_boons: 0,
            max_boons: 3,
        },
        &deadline,
    )
    .unwrap();
    for point in std::iter::once(&growth.base).chain(&growth.points) {
        assert!(point.metrics.values().all(|value| value.value().is_none()));
    }
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
