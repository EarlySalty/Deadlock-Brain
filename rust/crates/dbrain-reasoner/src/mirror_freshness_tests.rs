#[test]
fn confirmed_originals_use_the_latest_check_for_patch_history_without_changing_raw_values() {
    let raw: serde_json::Value =
        serde_json::from_str(include_str!("../testdata/calculation/sheet-6759.json")).unwrap();
    let source = |kind: &str| crate::ModelSource {
        client_version: 6759,
        document_id: format!("recorded-mirror-check-reference-{kind}"),
        original_url: format!("https://assets.deadlock-api.com/v2/{kind}"),
        kind: kind.into(),
        language: "english".into(),
        json_pointer: format!("/{kind}"),
    };
    let models = super::calculation_models_from_payloads(
        &raw["heroes"],
        &raw["items"],
        &source("heroes"),
        &source("items"),
    )
    .unwrap();
    let original = serde_json::to_value(&models).unwrap();
    let assets = super::MirroredAssets {
        origin: None,
        provenance: super::MirrorProvenance {
            client_version: 6759,
            mirrored_at: 1000,
            checked_at: 2000,
        },
        heroes: super::mirror_records(raw["heroes"].clone(), 2000).unwrap(),
        items: super::mirror_records(raw["items"].clone(), 2000).unwrap(),
        calculation: Some(models),
    };
    assert_eq!(assets.provenance.mirrored_at, 1000);
    assert_eq!(
        serde_json::to_value(assets.calculation.as_ref().unwrap()).unwrap(),
        original,
    );
    for (records, originals) in [(&assets.heroes, &raw["heroes"]), (&assets.items, &raw["items"])] {
        for (record, original) in records.iter().zip(originals.as_array().unwrap()) {
            assert_eq!(record["_snapshot_fetched_at"], 2000);
            let mut restored = record.clone();
            restored.as_object_mut().unwrap().remove("_snapshot_fetched_at");
            assert_eq!(&restored, original);
        }
    }
    let cfg = crate::ReasonerConfig::default();
    let (mut hero, snapshots) = super::hero_model_from_mirror(&cfg, "25", &assets).unwrap();
    let damage = snapshots[0].fields["weapon.bullet_damage"].value;
    assert_eq!(snapshots[0].fields["weapon.bullet_damage"].fetched_at, Some(2000.0));
    let events = [serde_json::json!({
        "entity_name": hero.name,
        "raw_line": format!("Base bullet damage increased from {damage} to {}", damage * 1.1),
        "posted_at": "1970-01-01T00:25:00Z",
        "posted_at_epoch": 1500,
    })];
    let deltas = crate::patch::compute_patch_delta_with_snapshots(&hero, &events, &snapshots);
    assert_eq!(deltas.len(), 1);
    assert!(deltas[0].application.is_none());
    let before = serde_json::to_value(&hero).unwrap();
    crate::patch::apply_patch_delta(&mut hero, &mut [], &deltas);
    assert_eq!(serde_json::to_value(&hero).unwrap(), before);
}
