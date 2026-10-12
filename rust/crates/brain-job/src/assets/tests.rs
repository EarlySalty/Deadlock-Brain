use super::*;
use sha2::{Digest, Sha256};
fn hash(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}
fn raw(bytes: &[u8]) -> RawObject {
    RawObject {
        id: 1,
        source: "assets".into(),
        revision: "r1".into(),
        path: "items/en.json".into(),
        content_sha256: Sha256::digest(bytes).into(),
        bytes: bytes.to_vec(),
    }
}
fn accepted(bytes: &[u8]) -> FactSet {
    let Outcome::Accepted(set) = Assets.extract(&raw(bytes)) else {
        panic!("expected accepted Assets object");
    };
    brain_storage::gamefacts::receipts::verify(bytes, &set).expect("lexical receipts");
    set
}
fn code(bytes: &[u8]) -> &'static str {
    let Outcome::Rejected(error) = Assets.extract(&raw(bytes)) else {
        panic!("expected rejected Assets object");
    };
    error.code
}
#[test]
fn original_json_lexemes_ids_and_classes_are_preserved() {
    let bytes = br#" [{"id":18446744073709551615,"class_name":"Viscous_Goo_Grenade","type":"ability","name":"Car\/bine","image":"a/b","properties":{"Damage":{"value":26.000}}}] "#;
    let set = accepted(bytes);
    assert_eq!(set.entities[0].key, "viscous_goo_grenade");
    for (stat, token) in [
        ("identity_game_id", "18446744073709551615"),
        ("identity_key", r#""Viscous_Goo_Grenade""#),
        ("identity_image", r#""a/b""#),
        ("ability_damage", "26.000"),
    ] {
        let fact = set.facts.iter().find(|f| f.stat_key == stat).expect("fact");
        assert_eq!(fact.receipt.value_sha256, hash(token.as_bytes()));
    }
    assert!(
        matches!(&set.facts[0].value, FactValue::Text(value) if value == "18446744073709551615")
    );
    assert!(matches!(&set.facts[1].value, FactValue::Text(k) if k == "Viscous_Goo_Grenade"));
    assert_eq!(set.aliases[1].alias, "car/bine");
    assert_eq!(set.aliases[1].receipt.locator, "/0/name");
    assert_eq!(set.aliases[1].receipt.value_sha256, hash(br#""Car\/bine""#));
}
#[test]
fn missing_identities_and_normalized_collisions_reject_without_overwrite() {
    assert_eq!(
        code(br#"{"id":1,"name":"No key","type":"ability"}"#),
        "assets_identity"
    );
    for bytes in [
        br#"{"id":1,"class_name":"a","type":"ability","properties":{"Damage":{"value":1,"value":2}}}"#.as_slice(),
        br#"{"id":1,"class_name":"a","type":"ability","properties":{"Damage":{"value":1},"damage":{"value":2}}}"#,
        br#"{"id":1,"class_name":"a","type":"ability","properties":{"Weapon-Damage":{"value":1},"weapon damage":{"value":2}}}"#,
        br#"[{"id":1,"class_name":"a","type":"ability"},{"id":1,"class_name":"b","type":"ability"}]"#,
        br#"[{"id":1,"class_name":"A","type":"ability"},{"id":2,"class_name":"a","type":"ability"}]"#,
    ] { assert_eq!(code(bytes), "key_collision"); }
}
#[test]
fn scale_usage_and_toxic_bullets_mapping_retain_source_pointers() {
    let bytes = br#"{"id":3696726732,"class_name":"upgrade_toxic_bullets","type":"upgrade","cost":3200,"properties":{"DotHealthPercent":{"value":"1.9"},"HealAmpReceivePenaltyPercent":{"value":"-35","usage_flags":["ConditionallyApplied"],"scale_function":{"class_name":"scale_function_tech_damage","specific_stat_scale_type":"ETechPower","stat_scale":0.006,"scaling_stats":["ETechPower"]}}}}"#;
    let records = records(bytes).expect("records");
    assert_eq!(records[0].id, 3696726732);
    let fields = records[0].fields("de").expect("fields");
    let cost = fields.iter().find(|f| f.stat == "item.cost").expect("cost");
    assert_eq!(cost.leaf.text, "3200");
    assert_eq!(cost.unit, Some("souls"));
    let fact = fields
        .iter()
        .find(|field| field.stat == "item.HealAmpReceivePenaltyPercent")
        .expect("effect");
    assert_eq!(fact.condition["scale_value"], "0.006");
    assert_eq!(
        fact.condition["scale_value_locator"],
        "/properties/HealAmpReceivePenaltyPercent/scale_function/stat_scale"
    );
    assert_eq!(fact.condition["scale_stat_0"], "ETechPower");
    assert_eq!(fact.condition["usage_flag_0"], "ConditionallyApplied");
    assert_eq!(code(bytes), "assets_qualified_unsupported");
    assert_eq!(code(br#"{"id":1,"class_name":"a","type":"ability","properties":{"Damage":{"value":1,"usage_flags":null}}}"#), "assets_qualified_unsupported");
}
#[test]
fn upgrades_remain_conservative_and_qualified_occurrences_are_not_flattened() {
    let bytes = br#"{"id":1,"class_name":"a","type":"ability","heroes":[13,25],"properties":{"Radius":{"value":"1.5m"}},"upgrades":[{"property_upgrades":[{"name":"Damage","bonus":0.9,"upgrade_type":"EAddToScale"},{"name":"Damage","bonus":1,"scale_stat_filter":null}]}]}"#;
    let data = records(bytes).expect("records");
    let fields = data[0].fields("en").expect("fields");
    assert!(fields.iter().all(|f| !f.stat.starts_with("ability.")));
    assert_eq!(code(bytes), "assets_qualified_unsupported");
    let bytes = br#"{"id":1,"class_name":"a","type":"ability","upgrades":[{"property_upgrades":[{"name":"AbilityCooldown","bonus":"-15"},{"name":"AbilityCooldown","bonus":"-15"}]}]}"#;
    let records = records(bytes).expect("records");
    let fields = records[0].fields("en").expect("fields");
    let upgrades: Vec<_> = fields
        .iter()
        .filter(|field| field.condition.get("upgrade_tier").is_some())
        .collect();
    for (index, field) in upgrades.iter().enumerate() {
        assert_eq!(
            field.condition,
            json!({"upgrade_tier":1,"operation":"delta","upgrade_entry":index})
        );
        assert_eq!(field.leaf.value_sha256(), hash(br#""-15""#));
    }
    assert_eq!(upgrades.len(), 2);
    assert_eq!(code(bytes), "assets_qualified_unsupported");
    assert_eq!(code(br#"{"id":1,"class_name":"a","type":"ability","upgrades":[{"property_upgrades":[{"name":"Damage","bonus":1},{"name":"damage","bonus":2}]}]}"#), "key_collision");
}
#[test]
fn references_do_not_require_a_catalog_or_invent_resolved_links() {
    let bytes = br#"{"id":2,"class_name":"Upgrade_Parent","type":"upgrade","component_items":["UPGRADE_BASE","upgrade_unknown"]}"#;
    let set = accepted(bytes);
    assert_eq!(set.entities.len(), 1);
    assert!(set.relations.is_empty());
    let refs: Vec<_> = set
        .facts
        .iter()
        .filter(|f| f.stat_key.starts_with("component_item_"))
        .collect();
    assert_eq!(refs.len(), 2);
    assert_eq!(refs[0].entity_key, "upgrade_parent");
    assert!(matches!(&refs[0].value, FactValue::Text(value) if value == "upgrade_base"));
    assert_eq!(refs[1].receipt.locator, "/component_items/1");
    for components in [
        json!(["UPGRADE_PARENT"]),
        json!(["upgrade_base", "Upgrade_Base"]),
        json!([1]),
        json!({"0":"upgrade_base"}),
        json!("upgrade_base"),
    ] {
        let bytes = serde_json::to_vec(&json!({"id":2,"class_name":"upgrade_parent","type":"upgrade","component_items":components})).expect("json");
        assert!(matches!(Assets.extract(&raw(&bytes)), Outcome::Rejected(_)));
    }
}
#[test]
fn object_members_cannot_impersonate_upgrade_array_indices() {
    for upgrades in [
        json!({"0":{"property_upgrades":[{"name":"Damage","bonus":1}]}}),
        json!([{"property_upgrades":{"0":{"name":"Damage","bonus":1}}}]),
    ] {
        let bytes = serde_json::to_vec(
            &json!({"id":1,"class_name":"a","type":"ability","upgrades":upgrades}),
        )
        .expect("json");
        assert_eq!(code(&bytes), "assets_upgrade_type");
    }
}
#[test]
fn central_validation_receives_untruncated_values_and_numeric_lexemes() {
    for value in [json!("ü".repeat(limits::TEXT_BYTES)), json!("x\u{0000}y")] {
        let bytes =
            serde_json::to_vec(&json!({"id":1,"class_name":"a","type":"ability","image":value}))
                .expect("json");
        assert!(matches!(code(&bytes), "value_too_large" | "nul_text"));
    }
    assert_eq!(code(br#"{"id":1,"class_name":"a","type":"ability","properties":{"Damage":{"value":1e999}}}"#), "invalid_number");
    let set = accepted(br#"{"id":1,"class_name":"a","type":"ability","properties":{"Damage":{"value":"1.2300e+2"}}}"#);
    assert!(matches!(set.facts[2].value, FactValue::Number(123.0)));
}
#[test]
fn pointer_escaping_localization_and_source_selection_follow_the_contract() {
    let records = records(
        br#"{"id":1,"class_name":"a","type":"ability","properties":{"A~/B":{"value":"1.0"}}}"#,
    )
    .expect("records");
    let fields = records[0].fields("en").expect("fields");
    let field = fields
        .iter()
        .find(|f| f.stat == "ability.A~/B")
        .expect("property");
    assert_eq!(field.leaf.pointer, "/properties/A~0~1B/value");
    let mut raw = raw(br#"{"id":1,"class_name":"a","type":"ability","name":"Kugel"}"#);
    raw.path = "items/de.json".into();
    let Outcome::Accepted(set) = Assets.extract(&raw) else {
        panic!("localized alias");
    };
    assert_eq!(set.aliases[1].lang, "de");
    for (source, path) in [("stub", "items/en.json"), ("assets", "hero-data.json")] {
        assert!(!Assets.accepts(source, path));
    }
    assert!(matches!(
        Assets.extract(&super::tests::raw(
            br#"{"id":1,"class_name":"weapon","type":"weapon"}"#
        )),
        Outcome::Ignored
    ));
    assert_eq!(
        code(br#"{"id":1,"class_name":"a","type":"ability","description":{"desc":"Text"}}"#),
        "assets_qualified_unsupported"
    );
}
