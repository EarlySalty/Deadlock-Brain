use super::*;
use sha2::{Digest, Sha256};

fn raw(bytes: Vec<u8>) -> RawObject {
    RawObject {
        id: 1,
        source: "assets".into(),
        revision: "capture".into(),
        path: "heroes/en/0.json".into(),
        content_sha256: Sha256::digest(&bytes).into(),
        bytes,
    }
}

#[test]
fn explicit_captured_fields_produce_164_reference_only_relations() {
    let bytes = include_bytes!("../../tests/fixtures/p2-slots/heroes-en.json");
    assert_eq!(
        brain_storage::entity_profile::derivation::sha256(bytes),
        "5c163d19407aa683a6ebbae6e79d58885d71b3af3c0343e28cf0ec2e7fc75c31"
    );
    let heroes: Vec<&RawValue> = serde_json::from_slice(bytes).expect("real heroes");
    assert_eq!(heroes.len(), 41);
    let mut total = 0;
    for hero in heroes {
        let raw = raw(hero.get().as_bytes().to_vec());
        let Outcome::Accepted(set) = HeroSlots.extract(&raw) else {
            panic!("captured hero")
        };
        assert!(set.entities.is_empty());
        assert!(set.aliases.is_empty());
        assert_eq!(set.relations.len(), 4);
        brain_storage::gamefacts::receipts::verify(&raw.bytes, &set).expect("exact receipts");
        for (position, relation) in set.relations.iter().enumerate() {
            let slot = position + 1;
            assert_eq!(relation.receipt.locator, format!("/items/signature{slot}"));
            let qualifier = set
                .facts
                .iter()
                .find(|f| f.stat_key == format!("p2_slot_{slot}_slot"))
                .expect("scalar slot");
            assert!(
                matches!(&qualifier.value, FactValue::Text(value) if *value == slot.to_string())
            );
            assert_eq!(qualifier.receipt.locator, relation.receipt.locator);
            assert_eq!(
                qualifier.receipt.value_sha256,
                relation.receipt.value_sha256
            );
        }
        total += set.relations.len();
    }
    assert_eq!(total, 164);
}

#[test]
fn malformed_missing_colliding_and_duplicate_references_never_get_guessed() {
    let valid = serde_json::json!({"class_name":"hero_haze", "items":{
        "signature1":"ability_sleep_dagger", "signature2":"ability_smoke_bomb",
        "signature3":"ability_stacking_damage", "signature4":"ability_bullet_flurry"}});
    for value in [
        serde_json::json!([]),
        serde_json::json!({"id":13,"name":"Haze","items":valid["items"]}),
        {
            let mut value = valid.clone();
            value["items"]["signature4"] = serde_json::Value::Null;
            value
        },
        {
            let mut value = valid.clone();
            value["items"]["signature4"] = value["items"]["signature1"].clone();
            value
        },
        {
            let mut value = valid.clone();
            value["items"]["signature4"] = serde_json::json!("x".repeat(limits::KEY_BYTES + 1));
            value
        },
    ] {
        assert!(matches!(
            HeroSlots.extract(&raw(serde_json::to_vec(&value).expect("fixture"))),
            Outcome::Rejected(_)
        ));
    }
    let mut bytes = serde_json::to_vec(&valid).expect("fixture");
    bytes.pop();
    bytes.extend_from_slice(br#", "class_name":"hero_other"}"#);
    assert!(
        matches!(HeroSlots.extract(&raw(bytes)), Outcome::Rejected(error) if error.code == "key_collision")
    );
}

#[test]
fn duplicate_hero_records_reject_both_inputs_without_hiding_a_valid_peer() {
    struct Cache(std::path::PathBuf);
    impl Drop for Cache {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.0).expect("own test cache cleanup");
        }
    }
    let path = std::env::temp_dir().join(format!("p2-slots-source-{}", std::process::id()));
    std::fs::create_dir(&path).expect("new own source cache");
    let cache = Cache(path);
    for language in ["en", "de"] {
        std::fs::write(cache.0.as_path().join(format!("{language}.json")), b"[]")
            .expect("legacy cache");
    }
    let bytes = br#"[{"class_name":"hero_haze","items":{"signature1":"a","signature2":"b","signature3":"c","signature4":"d"}}, {"class_name":"Hero Haze","items":{"signature1":"e","signature2":"f","signature3":"g","signature4":"h"}}, {"class_name":"hero_wraith","items":{"signature1":"i","signature2":"j","signature3":"k","signature4":"l"}}]"#;
    std::fs::write(cache.0.as_path().join("heroes-en.json"), bytes)
        .expect("public-shaped response");
    let mut rejected = 0;
    let mut accepted = 0;
    let mut observations = 0;
    let mut archived = Vec::new();
    for object in crate::assets::source::load(cache.0.as_path(), false) {
        let object = object.expect("bounded source object");
        if object.path.starts_with("heroes/") && object.path.ends_with(".response") {
            archived.extend_from_slice(&object.bytes);
        }
        let mut raw = raw(object.bytes);
        raw.path = object.path;
        raw.revision = object.revision;
        match HeroSlots.extract(&raw) {
            Outcome::Accepted(set) if set.relations.is_empty() => observations += 1,
            Outcome::Accepted(set) => {
                accepted += 1;
                assert_eq!(set.relations.len(), 4);
            }
            Outcome::Rejected(reason) => {
                rejected += 1;
                assert_eq!(reason.code, "assets_hero_slot_ambiguous");
            }
            Outcome::Ignored => {}
        }
    }
    assert_eq!(archived, bytes);
    assert_eq!((accepted, rejected, observations), (1, 2, 1));
}
