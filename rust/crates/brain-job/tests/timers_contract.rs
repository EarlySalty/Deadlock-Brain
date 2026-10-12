use brain_job::{
    extract::{timers, Atom},
    git::Source,
};
use serde_json::json;

#[test]
fn timers_have_occurrence_aware_receipts_and_no_guessed_spawn_values() {
    let path = "data/json/misc-data.json";
    let bytes = br#"{"citadel_breakable_prop_drop_gold":{"InitialSpawnTime":180.0,"RespawnTime":180.0},"neutral_camp_midboss":{"InitialSpawnDelayInSeconds":0,"SpawnIntervalInSeconds":420,"SpawnIntervalMin":300,"SpawnIntervalChange":-60},"citadel_item_pickup_idol":{"WalkBackModifier":{"WaitTimeLimit":180}}}"#;
    let facts = timers::extract(Source::ALL[0], path, bytes).unwrap();
    assert_eq!(facts.len(), 5);
    assert!(facts.iter().all(|fact| fact.kind == "timer"
        && fact.unit == Some("s")
        && fact.locator_kind == "json_pointer"
        && fact.hash.len() == 32));
    assert!(facts.iter().all(|fact| fact.entity != "urn"));
    assert!(facts
        .iter()
        .filter(|fact| fact.entity == "mid_boss")
        .all(|fact| fact.stat != "timer.first_spawn_seconds"));
    timers::prove(Source::ALL[0], path, bytes, &facts).unwrap();
    let mut corrupted = facts;
    corrupted[0].condition = json!({"source_entity":"another_box"});
    assert!(timers::prove(Source::ALL[0], path, bytes, &corrupted).is_err());

    let path = "game/citadel/pak01_dir/scripts/misc.vdata";
    let bytes = br#"<!-- kv3 encoding:text --> { citadel_breakable_prop_drop_gold = { m_flInitialSpawnTime = 180.0 m_flRespawnTime = 180.0 } neutral_camp_midboss = { m_iSpawnIntervalInSeconds = 420 m_iSpawnIntervalChange = -60 m_iSpawnIntervalMin = 300 m_iInitialSpawnDelayInSeconds = 0 } }"#;
    let facts = timers::extract(Source::ALL[1], path, bytes).unwrap();
    assert_eq!(facts.len(), 5);
    assert_eq!(
        facts[0].locator,
        "/citadel_breakable_prop_drop_gold/0/m_flInitialSpawnTime/0"
    );
    assert_eq!(facts[0].locator_kind, "kv_pointer");
    assert!(matches!(&facts[0].value, Atom::Number(value) if value == "180.0"));
    timers::prove(Source::ALL[1], path, bytes, &facts).unwrap();
    let mut corrupted = facts;
    corrupted[0].value = Atom::Number("181.0".into());
    assert!(timers::prove(Source::ALL[1], path, bytes, &corrupted).is_err());
    assert!(timers::extract(Source::ALL[1], path,
        br#"<!-- kv3 encoding:text --> { citadel_breakable_prop_drop_gold = { m_flInitialSpawnTime = 180.0 } ignored = { malformed = invalid } }"#).is_err());
}

#[test]
fn rejuvenator_times_preserve_source_values_and_do_not_invent_first_spawn() {
    let path = "data/json/generic-data.json";
    let bytes = br#"{"IdolParams":{"IdolDropDuration":12.5},"RejuvParams":{"RejuvinatorBuffDuration":180,"RejuvinatorDropDuration":6,"RejuvinatorExpirationWarningTiming":30}}"#;
    let facts = timers::extract(Source::ALL[0], path, bytes).unwrap();
    assert_eq!(facts.len(), 3);
    assert!(facts.iter().all(|fact| fact.entity == "rejuvenator"));
    assert!(facts
        .iter()
        .all(|fact| fact.stat != "timer.first_spawn_seconds"));
    timers::prove(Source::ALL[0], path, bytes, &facts).unwrap();
    assert!(timers::extract(Source::ALL[1], path, bytes)
        .unwrap()
        .is_empty());
    assert!(timers::extract(
        Source::ALL[0],
        "data/json/convars.json",
        br#"{"citadel_midboss_initial_spawn_time_override":{"value":-1}}"#
    )
    .unwrap()
    .is_empty());
}

#[test]
#[ignore = "requires an explicitly supplied public source checkout"]
fn public_timer_documents_are_extracted_and_verified_when_explicitly_supplied() {
    let root = std::env::var_os("BRAIN_P3_PUBLIC_DEADLOCK_DATA_ROOT")
        .expect("set BRAIN_P3_PUBLIC_DEADLOCK_DATA_ROOT to a verified public checkout");
    let tracking = std::env::var_os("BRAIN_P3_PUBLIC_GAMETRACKING_ROOT")
        .expect("set BRAIN_P3_PUBLIC_GAMETRACKING_ROOT to a verified public checkout");
    for (source, root, paths) in [
        (
            Source::ALL[0],
            root,
            ["data/json/misc-data.json", "data/json/generic-data.json"],
        ),
        (
            Source::ALL[1],
            tracking,
            [
                "game/citadel/pak01_dir/scripts/misc.vdata",
                "game/citadel/pak01_dir/scripts/generic_data.vdata",
            ],
        ),
    ] {
        for (path, count) in paths.into_iter().zip([11, 3]) {
            let bytes = std::fs::read(std::path::Path::new(&root).join(path)).unwrap();
            assert!(brain_job::extract::supported(source, path));
            let facts = brain_job::extract::extract(source, path, &bytes).unwrap();
            assert_eq!(facts.len(), count, "{path}");
            assert!(facts.iter().all(|fact| fact.entity != "urn"));
            brain_job::extract::prove(source, path, &bytes, &facts).unwrap();
        }
    }
}
