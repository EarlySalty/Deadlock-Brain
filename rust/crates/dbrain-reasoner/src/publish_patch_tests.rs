#[test]
fn repeated_api_confirmation_preserves_original_input_identity_and_resume_validation() {
    let (mut build, mut original) = bound_fixture(origin_fixture());
    let previous = validate_build_provenance(&build).unwrap().clone();
    let caller = publication_caller(&build).unwrap();
    let mut current = bound_fixture(origin_fixture()).1;
    current.provenance.checked_at = 4000;
    for field in current
        .snapshots
        .iter_mut()
        .flat_map(|snapshot| snapshot.fields.values_mut())
    {
        field.fetched_at = Some(4000.0);
    }
    let checked = calculation_provenance(
        &current.hero,
        &current.items,
        &current.snapshots,
        &previous.config,
    )
    .unwrap()
    .unwrap();
    assert_eq!(checked.origin, previous.origin);
    assert_eq!(checked.input_sha256, previous.input_sha256);
    bind_calculated_build(&mut build, checked, previous.purchase_plan.as_ref().unwrap()).unwrap();
    assert_eq!(publication_caller(&build).unwrap(), caller);
    validate_publish_current_models(&build, &original, &current, 3500).unwrap();
    assert_eq!(validate_build_provenance(&build).unwrap(), &previous);
    assert_eq!(current.provenance.mirrored_at, 2000);
    assert_eq!(original.provenance.checked_at, 2000);
    let serialized = serde_json::to_value(&build).unwrap();
    let resumed: BuildObject = serde_json::from_value(serialized.clone()).unwrap();
    validate_publish_current_models(&resumed, &original, &current, 3500).unwrap();
    current.provenance.checked_at = 5000;
    for field in current
        .snapshots
        .iter_mut()
        .flat_map(|snapshot| snapshot.fields.values_mut())
    {
        field.fetched_at = Some(5000.0);
    }
    original.provenance.checked_at = 5000;
    for field in original
        .snapshots
        .iter_mut()
        .flat_map(|snapshot| snapshot.fields.values_mut())
    {
        field.fetched_at = Some(5000.0);
    }
    validate_publish_current_models(&resumed, &original, &current, 4500).unwrap();
    assert_eq!(validate_build_provenance(&resumed).unwrap(), &previous);
    assert_eq!(publication_caller(&resumed).unwrap(), caller);
    assert_eq!(serde_json::to_value(&resumed).unwrap(), serialized);
    assert_eq!(original.provenance.mirrored_at, 2000);
    assert_eq!(current.provenance.mirrored_at, 2000);
}

#[test]
fn renewed_confirmation_rejects_stale_snapshots_and_changed_bound_inputs() {
    let (build, mut original) = bound_fixture(origin_fixture());
    let mut current = bound_fixture(origin_fixture()).1;
    current.provenance.checked_at = 4000;
    for field in current
        .snapshots
        .iter_mut()
        .flat_map(|snapshot| snapshot.fields.values_mut())
    {
        field.fetched_at = Some(4000.0);
    }
    validate_publish_current_models(&build, &original, &current, 3500).unwrap();
    for target in [
        crate::DeltaTarget::Hero(build.hero_id),
        crate::DeltaTarget::Ability(build.ability_order[0].ability_id),
        crate::DeltaTarget::Item(build.core[0].item_id),
    ] {
        let field = current
            .snapshots
            .iter_mut()
            .find(|snapshot| snapshot.target == target)
            .unwrap()
            .fields
            .values_mut()
            .next()
            .unwrap();
        field.fetched_at = Some(2000.0);
        validate_current_game_values(&build, &original, &current).unwrap();
        assert!(validate_publish_current_models(&build, &original, &current, 3500).is_err());
        current
            .snapshots
            .iter_mut()
            .find(|snapshot| snapshot.target == target)
            .unwrap()
            .fields
            .values_mut()
            .next()
            .unwrap()
            .fetched_at = Some(4000.0);
    }
    assert!(validate_publish_current_models(&build, &original, &current, 4001).is_err());
    original.items[0].cost += 1;
    assert!(validate_publish_current_models(&build, &original, &current, 3500).is_err());
    original.items[0].cost -= 1;
    current.hero.weapon.bullet_damage += 1.0;
    assert!(validate_publish_current_models(&build, &original, &current, 3500).is_err());
    current.hero.weapon.bullet_damage -= 1.0;
    let mut forged = build.clone();
    forged.provenance.as_mut().unwrap().input_sha256 = "0".repeat(64);
    assert!(validate_publish_current_models(&forged, &original, &current, 3500).is_err());
    validate_publish_current_models(&build, &original, &current, 3500).unwrap();
}

#[test]
fn publication_rejects_a_purchase_relevant_patch_delta_not_confirmed_by_original_game_values() {
    let (bound, mut mirrored) = bound_fixture(origin_fixture());
    let cfg = bound.provenance.as_ref().unwrap().config.clone();
    let snapshot = mirrored
        .snapshots
        .iter_mut()
        .find(|snapshot| snapshot.target == crate::DeltaTarget::Item(1))
        .unwrap();
    snapshot.name = "Item 1".into();
    snapshot.fields = [(
        "properties.BaseAttackDamagePercent".into(),
        crate::SnapshotField {
            value: 10.0,
            fetched_at: Some(2000.0),
            source: "deadlock_assets_api/item_or_ability".into(),
            label: "Base attack damage percent".into(),
        },
    )]
    .into_iter()
    .collect();
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
    let plan = |events: &[serde_json::Value]| {
        crate::plan_build(
            &mirrored.hero,
            &mirrored.items,
            &meta,
            events,
            &mirrored.snapshots,
            &cfg,
        )
        .unwrap()
    };
    let validate = |planned: &crate::PlannedBuild| {
        validate_current_game_values(&planned.build, &mirrored, &mirrored)?;
        validate_publish_models_with_plan(
            &planned.build,
            &planned.purchase_plan,
            &mirrored.hero,
            &mirrored.items,
            &mirrored.snapshots,
            2000.0,
            &cfg,
        )
    };
    validate(&plan(&[])).unwrap();
    let changed = plan(&[serde_json::json!({
        "entity_name": "Item 1",
        "raw_line": "Base attack damage percent increased from 10 to 11",
        "posted_at": "1970-01-01T00:41:40Z",
        "posted_at_epoch": 2500,
    })]);
    assert!(changed.deltas.iter().any(|delta| delta.application.is_some()));
    assert!(changed.build.core.iter().any(|item| item.item_id == 1));
    assert!(validate(&changed).is_err());
}
