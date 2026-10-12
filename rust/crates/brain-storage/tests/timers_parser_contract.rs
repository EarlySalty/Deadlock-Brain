#[path = "../../brain-job/src/timer_values.rs"]
mod timer_values;

use sha2::{Digest, Sha256};

fn values(
    source: &str,
    path: &str,
    bytes: &[u8],
) -> Result<Vec<timer_values::TimerValue>, &'static str> {
    timer_values::parse(source, path, bytes, bytes.len(), |key| Ok(key.to_owned()))
}

#[test]
fn numeric_timer_values_keep_exact_raw_lexemes_and_locators() {
    let json = br#"{"citadel_breakable_prop_drop_gold":{"InitialSpawnTime":180.0,"RespawnTime":180.0},"neutral_camp_midboss":{"InitialSpawnDelayInSeconds":0}}"#;
    let facts = values("deadlock-data", "data/json/misc-data.json", json).unwrap();
    assert_eq!(facts.len(), 2);
    for fact in facts {
        assert_eq!(fact.timer, "soul_box");
        assert_eq!(fact.numeric, "180.0");
        assert_eq!(fact.locator_kind, "json_pointer");
        assert_eq!(fact.value_sha256, Sha256::digest(b"180.0").to_vec());
        assert_eq!(fact.source_entity, "citadel_breakable_prop_drop_gold");
        assert!(fact.flags.is_empty());
    }
    let kv = br#"<!-- kv3 encoding:text --> { neutral_camp_midboss = { m_iSpawnIntervalInSeconds = 120 m_iSpawnIntervalMin = 60 } }"#;
    let facts = values(
        "gametracking",
        "game/citadel/pak01_dir/scripts/misc.vdata",
        kv,
    )
    .unwrap();
    assert_eq!(facts.len(), 2);
    assert_eq!(
        facts[0].locator,
        "/neutral_camp_midboss/0/m_iSpawnIntervalInSeconds/0"
    );
    assert_eq!(facts[0].locator_kind, "kv_path");
    assert_eq!(facts[0].stat, "timer.respawn_interval_seconds");
}

#[test]
fn duplicate_properties_and_wrong_timer_types_reject_the_object() {
    for json in [
        br#"{"citadel_breakable_prop_drop_gold":{"InitialSpawnTime":180,"InitialSpawnTime":190}}"#
            .as_slice(),
        br#"{"citadel_breakable_prop_drop_gold":{"InitialSpawnTime":"180"}}"#.as_slice(),
        br#"{"citadel_breakable_prop_drop_gold":{"InitialSpawnTime":[180]}}"#.as_slice(),
    ] {
        assert!(values("deadlock-data", "data/json/misc-data.json", json).is_err());
    }
    let duplicate =
        br#"{ neutral_camp_midboss = { m_iSpawnIntervalMin = 60 m_iSpawnIntervalMin = 90 } }"#;
    assert_eq!(
        values(
            "gametracking",
            "game/citadel/pak01_dir/scripts/misc.vdata",
            duplicate
        )
        .unwrap_err(),
        "key_collision"
    );
    let malformed = br#"{ neutral_camp_midboss = { m_iSpawnIntervalMin = 60 } ignored = invalid }"#;
    assert!(values(
        "gametracking",
        "game/citadel/pak01_dir/scripts/misc.vdata",
        malformed
    )
    .is_err());
}

#[test]
fn input_limit_is_supplied_by_the_caller_and_urn_spawn_is_not_invented() {
    let bytes = br#"{"RejuvParams":{"RejuvinatorBuffDuration":120.0},"IdolParams":{"IdolDropDuration":15.0}}"#;
    assert_eq!(
        timer_values::parse(
            "deadlock-data",
            "data/json/generic-data.json",
            bytes,
            bytes.len() - 1,
            |key| Ok(key.to_owned())
        )
        .unwrap_err(),
        "raw_object_limit"
    );
    let facts = values("deadlock-data", "data/json/generic-data.json", bytes).unwrap();
    assert_eq!(facts.len(), 1);
    assert_eq!(facts[0].timer, "rejuvenator");
    assert_eq!(facts[0].stat, "timer.buff_duration_seconds");
    assert!(values("other", "data/json/misc-data.json", b"malformed")
        .unwrap()
        .is_empty());
}
