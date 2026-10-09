use std::collections::BTreeMap;

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
