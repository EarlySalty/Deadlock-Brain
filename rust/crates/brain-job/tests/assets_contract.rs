use brain_job::{
    assets::extraction::{extract, prove},
    extract::Atom,
};
use serde_json::json;
use sha2::{Digest, Sha256};

#[test]
fn json_lexemes_conditions_and_component_receipts_are_exact() {
    let bytes = br#" [{"id":4294967295,"class_name":"viscous_goo_grenade","type":"ability","name":"Splatter","properties":{"AbilityCooldown":{"value":26.000}},"upgrades":[{"property_upgrades":[{"name":"Damage","bonus":"36.0"}]},{"property_upgrades":[{"name":"AbilityCooldown","bonus":-14.0}]}]},{"id":1,"class_name":"upgrade_base","type":"upgrade","cost":800},{"id":2,"class_name":"upgrade_parent","type":"upgrade","cost":3200,"item_tier":3,"item_slot_type":"weapon","component_items":["upgrade_base"]}] "#;
    let mut extracted = extract(bytes, "en").unwrap();
    prove(bytes, &extracted).unwrap();
    let name = extracted
        .facts
        .iter()
        .find(|fact| fact.stat == "identity.name")
        .unwrap();
    assert_eq!(name.locator, "/0/name");
    assert_eq!(name.hash, Sha256::digest(br#""Splatter""#).to_vec());
    assert_eq!(name.alias, Some(("Splatter".into(), "en")));
    let base = extracted
        .facts
        .iter()
        .find(|fact| fact.stat == "ability.AbilityCooldown" && fact.condition == json!({}))
        .unwrap();
    assert!(matches!(&base.value, Atom::Number(value) if value == "26.000"));
    assert_eq!(base.hash, Sha256::digest(b"26.000").to_vec());
    let upgrade = extracted
        .facts
        .iter()
        .find(|fact| fact.condition.get("upgrade_tier") == Some(&json!(1)))
        .unwrap();
    assert_eq!(upgrade.stat, "ability.Damage");
    assert_eq!(
        upgrade.condition,
        json!({"upgrade_tier":1,"operation":"delta"})
    );
    assert_eq!(upgrade.locator, "/0/upgrades/0/property_upgrades/0/bonus");
    assert_eq!(upgrade.hash, Sha256::digest(br#""36.0""#).to_vec());
    assert_eq!(extracted.edges.len(), 2);
    for edge in &extracted.edges {
        assert_eq!(edge.from, "upgrade_base");
        assert_eq!(edge.to, "upgrade_parent");
        assert_eq!(edge.proof.locator, "/2/component_items/0");
        assert_eq!(
            edge.proof.hash,
            Sha256::digest(br#""upgrade_base""#).to_vec()
        );
    }
    extracted.facts[0].hash[0] ^= 1;
    assert!(prove(bytes, &extracted).is_err());
}

#[test]
fn missing_identity_duplicate_members_and_invented_upgrades_are_rejected() {
    for bytes in [br#"[{"id":1,"name":"No key","type":"ability"}]"#.as_slice(), br#"[{"id":1,"class_name":"a","type":"ability","properties":{"Damage":{"value":1,"value":2}}}]"#, br#"[{"id":1,"class_name":"a","type":"ability"},{"id":1,"class_name":"b","type":"ability"}]"#] {
        assert!(extract(bytes, "en").is_err());
    }
    let bytes = br#"[{"id":1,"class_name":"a","type":"ability","properties":{"Radius":{"value":"1.5m"}},"upgrades":[{"property_upgrades":[{"name":"Damage","bonus":0.9,"upgrade_type":"EAddToScale"}]}]}]"#;
    let extracted = extract(bytes, "en").unwrap();
    assert_eq!(extracted.unmapped, 2);
    assert!(extracted
        .facts
        .iter()
        .all(|fact| !fact.stat.starts_with("ability.")));
    prove(bytes, &extracted).unwrap();
}

#[test]
fn conditions_are_verified_from_original_qualifier_pointers() {
    let bytes = br#"[{"id":1,"class_name":"a","type":"ability","properties":{"Damage":{"value":"1.9","usage_flags":["ConditionallyApplied"],"scale_function":{"class_name":"scale_function_tech_damage","specific_stat_scale_type":"ETechPower","stat_scale":0.006}}}}]"#;
    let mut extracted = extract(bytes, "de").unwrap();
    prove(bytes, &extracted).unwrap();
    let fact = extracted
        .facts
        .iter_mut()
        .find(|fact| fact.stat == "ability.Damage")
        .unwrap();
    assert_eq!(fact.condition["scale_value"], "0.006");
    assert_eq!(fact.condition["usage_flag_0"], "ConditionallyApplied");
    fact.condition["scale_value"] = json!("0.007");
    assert!(prove(bytes, &extracted).is_err());
}

#[test]
fn repeated_upgrades_keep_occurrence_conditions_and_escaped_names_keep_lexeme_hashes() {
    let bytes = br#"[{"id":1,"class_name":"ability_gunslinger_demon_carbine","type":"ability","name":"Car\/bine","upgrades":[{"property_upgrades":[{"name":"AbilityCooldown","bonus":"-15"},{"name":"AbilityCooldown","bonus":"-15"}]}]},{"id":2,"class_name":"weapon_alternative_rmb+lmb_activate","type":"weapon"}]"#;
    let extracted = extract(bytes, "en").unwrap();
    prove(bytes, &extracted).unwrap();
    let name = extracted
        .facts
        .iter()
        .find(|fact| fact.stat == "identity.name")
        .unwrap();
    assert_eq!(name.hash, Sha256::digest(br#""Car\/bine""#).to_vec());
    let upgrades: Vec<_> = extracted
        .facts
        .iter()
        .filter(|fact| fact.condition.get("upgrade_tier").is_some())
        .collect();
    assert_eq!(upgrades.len(), 2);
    assert_eq!(upgrades[0].condition["upgrade_entry"], 0);
    assert_eq!(upgrades[1].condition["upgrade_entry"], 1);
}

#[test]
fn every_fact_and_edge_requires_a_canonical_record_leaf_and_original_hash() {
    let bytes = br#"[{"id":1,"class_name":"upgrade_base","type":"upgrade","name":"Base\\/item"},{"id":2,"class_name":"upgrade_parent","type":"upgrade","component_items":["upgrade_base"]}]"#;
    let extracted = extract(bytes, "en").unwrap();
    prove(bytes, &extracted).unwrap();
    for locator in [
        "/0",
        "/00/class_name",
        "/+0/class_name",
        "/0/missing",
        "/2/class_name",
        "/0/class_name/child",
    ] {
        let mut mutated = extract(bytes, "en").unwrap();
        mutated.facts[0].locator = locator.into();
        mutated.facts[0].hash[0] ^= 1;
        assert!(prove(bytes, &mutated).is_err(), "fact {locator}");
    }
    for edge in 0..extracted.edges.len() {
        for locator in [
            "/1",
            "/01/component_items/0",
            "/+1/component_items/0",
            "/1/missing",
            "/2/component_items/0",
            "/1/component_items/00",
        ] {
            let mut mutated = extract(bytes, "en").unwrap();
            mutated.edges[edge].proof.locator = locator.into();
            mutated.edges[edge].proof.hash[0] ^= 1;
            assert!(prove(bytes, &mutated).is_err(), "edge {edge} {locator}");
        }
        let mut mutated = extract(bytes, "en").unwrap();
        mutated.edges[edge].proof.hash[0] ^= 1;
        assert!(prove(bytes, &mutated).is_err());
        let mut mutated = extract(bytes, "en").unwrap();
        mutated.edges[edge].to = "upgrade_base".into();
        assert!(prove(bytes, &mutated).is_err());
        let mut mutated = extract(bytes, "en").unwrap();
        mutated.edges[edge].from = "upgrade_parent".into();
        assert!(prove(bytes, &mutated).is_err());
    }
}

#[test]
#[ignore = "requires an explicitly downloaded public upstream payload"]
fn public_payload_preserves_toxic_bullets_and_real_ability_atoms() {
    let directory = std::path::PathBuf::from(
        std::env::var_os("BRAIN_ASSETS_PUBLIC_FIXTURES").expect("public fixture directory"),
    );
    for (file, language) in [("items-en.json", "en"), ("items-de.json", "de")] {
        let bytes = std::fs::read(directory.join(file)).unwrap();
        let extracted = extract(&bytes, language).unwrap();
        prove(&bytes, &extracted).unwrap();
        let keys: std::collections::BTreeSet<_> = extracted
            .facts
            .iter()
            .map(|fact| (&fact.entity, &fact.stat, fact.condition.to_string()))
            .collect();
        assert_eq!(keys.len(), extracted.facts.len());
        let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        let records = value.as_array().unwrap();
        for key in ["upgrade_toxic_bullets", "viscous_goo_grenade"] {
            let source = records
                .iter()
                .position(|record| record["class_name"] == key)
                .unwrap();
            let facts: Vec<_> = extracted
                .facts
                .iter()
                .filter(|fact| fact.entity == key)
                .collect();
            assert!(!facts.is_empty());
            for fact in facts {
                assert_eq!(
                    fact.locator
                        .split('/')
                        .nth(1)
                        .unwrap()
                        .parse::<usize>()
                        .unwrap(),
                    source
                );
            }
        }
        let cost = extracted
            .facts
            .iter()
            .find(|fact| fact.entity == "upgrade_toxic_bullets" && fact.stat == "item.cost")
            .unwrap();
        assert!(
            matches!(&cost.value, Atom::Number(number) if number == &records.iter().find(|record| record["class_name"] == cost.entity).unwrap()["cost"].to_string())
        );
        println!(
            "language={language} bytes={} facts={} edges={} unmapped={}",
            bytes.len(),
            extracted.facts.len(),
            extracted.edges.len(),
            extracted.unmapped
        );
    }
}
