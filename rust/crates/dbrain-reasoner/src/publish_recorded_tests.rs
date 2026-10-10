fn recorded_public_warden() -> (super::MirroredModels, crate::CalculationModels) {
    let raw: serde_json::Value =
        serde_json::from_str(include_str!("../testdata/calculation/sheet-6759.json")).unwrap();
    let source = |kind: &str, hash_key: &str| crate::ModelSource {
        client_version: raw["provenance"]["client_version"].as_i64().unwrap(),
        document_id: raw["provenance"][hash_key].as_str().unwrap().into(),
        original_url: format!("https://assets.deadlock-api.com/v2/{kind}"),
        kind: kind.into(),
        language: "english".into(),
        json_pointer: String::new(),
    };
    let calculation = super::calculation_models_from_payloads(
        &raw["heroes"],
        &raw["items"],
        &source("heroes_all", "heroes_all_sha256"),
        &source("items", "items_sha256"),
    )
    .unwrap();
    let assets = super::MirroredAssets {
        origin: Some(crate::publish::BuildDataOrigin {
            client_version: calculation.client_version,
            source_run_id: 1,
            mirrored_at: 2000,
            parser_revision: "public-recorded-fixture".into(),
            manifest_document_id: 1,
            manifest_sha256: "1".repeat(64),
            heroes_document_id: 2,
            heroes_sha256: source("heroes_all", "heroes_all_sha256").document_id,
            items_document_id: 3,
            items_sha256: source("items", "items_sha256").document_id,
        }),
        provenance: super::MirrorProvenance {
            client_version: calculation.client_version,
            mirrored_at: 2000,
            checked_at: 2000,
        },
        heroes: super::mirror_records(raw["heroes"].clone(), 2000).unwrap(),
        items: super::mirror_records(raw["items"].clone(), 2000).unwrap(),
        calculation: Some(calculation.clone()),
    };
    let (hero, mut snapshots) =
        super::hero_model_from_mirror(&crate::ReasonerConfig::default(), "Warden", &assets)
            .unwrap();
    let (items, item_snapshots) = super::item_models_from_mirror(&assets).unwrap();
    snapshots.extend(item_snapshots);
    (
        super::MirroredModels {
            hero,
            items,
            snapshots,
            provenance: assets.provenance,
            flex_slots: None,
        },
        calculation,
    )
}

#[test]
fn recorded_warden_full_publication_guard_preserves_unknown_melee_and_low_confidence() {
    let (models, calculation) = recorded_public_warden();
    let cfg = crate::ReasonerConfig {
        patch_tag: "public-recorded-6759".into(),
        use_ai: false,
        ..Default::default()
    };
    let meta = crate::meta::MetaIndexWithSources {
        index: crate::MetaIndex {
            by_item: Default::default(),
            sample_ok: Default::default(),
        },
        author_builds: vec![],
        hero_ability_orders: Default::default(),
        core_layouts: Default::default(),
        combinations: Default::default(),
        population: Default::default(),
        observations: vec![],
        family: None,
    };
    let planned = crate::plan_build(
        &models.hero,
        &models.items,
        &meta,
        &[],
        &models.snapshots,
        &cfg,
    )
    .unwrap();
    let build = planned.build;
    let raw: serde_json::Value =
        serde_json::from_str(include_str!("../testdata/calculation/sheet-6759.json")).unwrap();
    let standard = raw["items"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|item| brain_contracts::game_mode::GameMode::Normal.allows_item(item))
        .map(|item| item["id"].as_i64().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    assert!(models
        .items
        .iter()
        .all(|item| standard.contains(&item.item_id)));
    assert!(build
        .core
        .iter()
        .chain(build.situations.iter().flat_map(|block| &block.items))
        .all(|item| standard.contains(&item.item_id)));
    let payload = crate::publish::publish_task_payload(&build);
    crate::publish::validate_publish_catalog(&payload, &raw["items"]).unwrap();
    let mut nonstandard = raw["items"].clone();
    let selected_id = build.core[0].item_id;
    let unavailable = nonstandard
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|item| item["id"] == selected_id)
        .unwrap();
    assert_eq!(unavailable["shopable"], true);
    unavailable["item_tier"] = serde_json::json!(5);
    assert!(brain_contracts::game_mode::GameMode::StreetBrawl.allows_item(unavailable));
    assert!(!brain_contracts::game_mode::GameMode::Normal.allows_item(unavailable));
    assert!(crate::publish::validate_publish_catalog(&payload, &nonstandard).is_err());
    assert_eq!(build.hero_id, 25);
    assert_eq!(build.confidence, crate::Confidence::Low);
    assert!(build.family.is_none());
    assert!(build.variants.is_empty());
    assert!(!build.core.is_empty());
    assert_eq!(build.ability_order.len(), 16);
    for step in &planned.purchase_plan.steps {
        assert_eq!(step.progression.unknown_effects.len(), 1);
        assert!(step.progression.unknown_effects[0]
            .contains("MODIFIER_VALUE_BASE_MELEE_DAMAGE_FROM_LEVEL"));
        assert!(!step.evaluation.unknown_effects.is_empty());
    }
    let souls = models.hero.level_curve.last().unwrap().required_souls;
    let (progressed, evidence) =
        crate::progression::at_souls(&models.hero, &build.ability_order, souls, &cfg);
    assert_eq!(evidence.applied_order_steps, build.ability_order.len());
    let pulse_ability = progressed
        .abilities
        .iter()
        .find(|ability| ability.class_name == "ability_warden_riot_protocol")
        .unwrap();
    assert_eq!(
        pulse_ability.tick_rate,
        pulse_ability.properties.get("PulseInterval").copied()
    );
    let pulse_diagnostic = crate::combat::InventoryEvaluation {
        unknown_effects: vec![format!(
            "Fähigkeit {}: PulseInterval nicht quantifiziert",
            pulse_ability.class_name
        )],
        ..Default::default()
    };
    assert!(
        crate::combat::publication_combat_blockers(&progressed, &[], &pulse_diagnostic).is_empty()
    );
    let mut pulse_changed = models.hero.clone();
    *pulse_changed
        .abilities
        .iter_mut()
        .find(|ability| ability.class_name == "ability_warden_riot_protocol")
        .unwrap()
        .properties
        .get_mut("PulseInterval")
        .unwrap() *= 2.0;
    let (mut pulse_changed, _) =
        crate::progression::at_souls(&pulse_changed, &build.ability_order, souls, &cfg);
    let updated = pulse_changed
        .abilities
        .iter_mut()
        .find(|ability| ability.class_name == "ability_warden_riot_protocol")
        .unwrap();
    assert_eq!(
        updated.tick_rate,
        updated.properties.get("PulseInterval").copied()
    );
    assert_ne!(updated.tick_rate, pulse_ability.tick_rate);
    updated.tick_rate = pulse_ability.tick_rate;
    assert_eq!(
        crate::combat::publication_combat_blockers(&pulse_changed, &[], &pulse_diagnostic).len(),
        1
    );
    let mut changed = models.hero.clone();
    *changed
        .standard_level_up_upgrades
        .get_mut("MODIFIER_VALUE_BASE_MELEE_DAMAGE_FROM_LEVEL")
        .unwrap() += 100.0;
    let (changed, changed_evidence) =
        crate::progression::at_souls(&changed, &build.ability_order, souls, &cfg);
    assert_ne!(evidence.unknown_effects, changed_evidence.unknown_effects);
    assert_eq!(progressed.weapon, changed.weapon);
    assert_eq!(progressed.abilities, changed.abilities);
    assert_eq!(
        crate::combat::evaluate_inventory(&progressed, &[], &cfg),
        crate::combat::evaluate_inventory(&changed, &[], &cfg)
    );
    let scenario = crate::CalculationScenario {
        progression: crate::ProgressionInput::Souls(souls),
        ability_order: build.ability_order.clone(),
        expected_level: None,
        expected_unspent_ap: None,
        spirit: crate::SpiritInput::Derived,
        weapon_bonus_percent: None,
        fire_rate_bonus_percent: None,
        item_ids: Vec::new(),
        purchases: Vec::new(),
        inventory_rules: None,
        max_active_items: None,
        imbues: Default::default(),
        secondary_fire: false,
        use_abilities: true,
        target: crate::CalculationTarget {
            health: 2000.0,
            regeneration: 0.0,
            shields: [0.0; 3],
            is_hero: true,
            bullet: Default::default(),
            spirit: Default::default(),
            changes: Vec::new(),
        },
        hit_fraction: 1.0,
        headshot_fraction: 0.0,
        headshot_bonus: None,
        distance_source_units: None,
        window_seconds: 30.0,
        reload_convention: crate::ReloadConvention::AfterFireInterval,
    };
    let projected = crate::calculate_hero(&calculation, 25, &scenario).unwrap();
    let sourced = &calculation.heroes[&25];
    assert_eq!(
        projected.metrics["starting.light_melee_damage"].value(),
        Some(
            sourced.starting_stats["light_melee_damage"]
                + evidence.standard_boons as f64
                    * models.hero.standard_level_up_upgrades
                        ["MODIFIER_VALUE_BASE_MELEE_DAMAGE_FROM_LEVEL"]
        )
    );
    assert!(projected.metrics["starting.heavy_melee_damage"]
        .value()
        .is_none());
    let catalog = models
        .items
        .iter()
        .map(crate::item::build_item_model)
        .collect::<crate::Result<Vec<_>>>()
        .unwrap();
    let held = planned
        .purchase_plan
        .steps
        .last()
        .unwrap()
        .transition
        .after
        .held_items(&catalog)
        .unwrap();
    let full = crate::combat::evaluate_inventory_with_bindings(
        &progressed,
        &held,
        &cfg,
        &planned.purchase_plan.steps.last().unwrap().imbue_targets,
    );
    let blockers = crate::combat::publication_combat_blockers(&progressed, &held, &full);
    assert!(!blockers.is_empty());
    assert!(blockers
        .iter()
        .any(|effect| effect.ends_with("FireRateSlow nicht quantifiziert")));
    assert!(blockers
        .iter()
        .any(|effect| effect.ends_with("MoveSpeedBonusPct nicht quantifiziert")));
    assert!(
        crate::publish::validate_publish_current_models(&build, &models, &models, 1000).is_err()
    );

    let mut geometric = progressed.clone();
    for ability in &mut geometric.abilities {
        for key in [
            "ForwardVelocity",
            "ProjectileLifetime",
            "Radius",
            "AdditionalTargetRadius",
            "ConeAngle",
            "StaminaReduction",
            "HealthStealPct",
        ] {
            if let Some(value) = ability.properties.get_mut(key) {
                *value += 100.0;
            }
        }
    }
    let geometric_evaluation = crate::combat::evaluate_inventory_with_bindings(
        &geometric,
        &held,
        &cfg,
        &planned.purchase_plan.steps.last().unwrap().imbue_targets,
    );
    assert_eq!(full.score, geometric_evaluation.score);
    assert_eq!(full.weapon_damage, geometric_evaluation.weapon_damage);
    assert_eq!(full.ability_damage, geometric_evaluation.ability_damage);
    assert_eq!(full.effective_health, geometric_evaluation.effective_health);
    assert_eq!(full.utility, geometric_evaluation.utility);
    let serialized = serde_json::to_value(&build).unwrap();
    let restored = serde_json::from_value(serialized.clone()).unwrap();
    assert!(
        crate::publish::validate_publish_current_models(&restored, &models, &models, 1000).is_err()
    );
    assert_eq!(serde_json::to_value(&restored).unwrap(), serialized);
}
