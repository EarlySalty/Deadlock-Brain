use brain_job::{
    hero::Hero, ingest, limits, Extractor, FactSet, GameFactsStore, InputObject, Outcome, RawObject,
};
use brain_storage::{gamefacts::receipts, PgStore};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    PgPool, Row,
};

#[path = "../../brain-storage/tests/support/scratch_pg.rs"]
mod scratch_pg;

const REVISION: &str = "2a26e26535a390c309e300e2c7b2763fa5ad7abd";
const PATH: &str = "data/json/hero-data.json";
const HASH: &str = "947c26790e067109f6cca4085649ed56e446bc5f4c9c571cd396290f802a3456";

fn hash(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}

fn raw(bytes: &[u8]) -> RawObject {
    RawObject {
        id: 1,
        source: "deadlock-data".into(),
        revision: REVISION.into(),
        path: PATH.into(),
        content_sha256: hash(bytes),
        bytes: bytes.to_vec(),
    }
}

fn accepted(raw: &RawObject) -> FactSet {
    let Outcome::Accepted(set) = Hero.extract(raw) else {
        panic!("{:?}", Hero.extract(raw))
    };
    limits::validate(&set).unwrap();
    receipts::verify(&raw.bytes, &set).unwrap();
    set
}

fn rejected(bytes: &[u8], code: &str) {
    let Outcome::Rejected(rejection) = Hero.extract(&raw(bytes)) else {
        panic!("expected rejection")
    };
    assert_eq!(rejection.code, code);
}

#[test]
fn exact_source_and_supported_fields_only() {
    assert!(Hero.accepts("deadlock-data", PATH));
    for (source, path) in [
        ("other", PATH),
        ("deadlock-data", "hero-data.json"),
        ("deadlock-data", "data/json/ability-data.json"),
    ] {
        assert!(!Hero.accepts(source, path));
    }
    let set = accepted(&raw(br##"{"hero_baba":{"Name":"Baba","Type":"Mystic","MaxHealth":780,"Lore":"#lore","Role":"#role","Playstyle":"#style","WeaponNameKey":"#weapon","InDevelopment":false,"Disabled":false},"hero_testhero":{"Name":null}}"##));
    assert_eq!(set.entities.len(), 1);
    assert_eq!(set.aliases.len(), 1);
    assert_eq!(set.facts.len(), 3);
    assert!(set.relations.is_empty());
    assert_eq!(set.entities[0].key, "hero_baba");
    assert_eq!(
        (set.aliases[0].alias.as_str(), set.aliases[0].lang.as_str()),
        ("baba", "en")
    );
    assert_eq!(
        set.facts
            .iter()
            .map(|f| f.stat_key.as_str())
            .collect::<Vec<_>>(),
        ["name_en", "type", "max_health"]
    );
    rejected(br#"{"hero_other":{"Name":null}}"#, "malformed_hero");
    let unknown_type = accepted(&raw(br#"{"hero_a":{"Name":"A","Type":null}}"#));
    assert_eq!(unknown_type.facts.len(), 1);
}

#[test]
fn duplicate_decoded_keys_and_normalized_collisions_reject() {
    for bytes in [
        br#"{"hero_a":{"Name":"A"},"hero_a":{"Name":"B"}}"#.as_slice(),
        br#"{"hero_a":{"Name":"A","Name":"B"}}"#,
        br#"{"hero_a":{"Name":"A","Weapon":{"BulletDamage":1,"BulletDamage":2}}}"#,
        br#"{"hero_a":{"Name":"A","Ignored":[{"x":1,"x":2}]}}"#,
    ] {
        rejected(bytes, "malformed_json");
    }
    let escaped = format!(
        r#"{{"hero_a":{{"Name":"A","{}u004eame":"B"}}}}"#,
        char::from(92)
    );
    rejected(escaped.as_bytes(), "malformed_json");
    for bytes in [
        br#"{"hero_a":{"Name":"A"},"Hero A":{"Name":"B"}}"#.as_slice(),
        br#"{"hero_a":{"Name":"A","name":"B"}}"#,
        br#"{"hero_a":{"Name":"A","Weapon":{"BulletDamage":1,"bulletdamage":2}}}"#,
        br#"{"hero_a":{"Name":"A","Ignored":[{"Max-Health":1,"MAX HEALTH":2}]}}"#,
    ] {
        rejected(bytes, "key_collision");
    }
}

#[test]
fn malformed_supported_fields_reject_the_entire_blob() {
    for (field, value) in [
        ("Name", json!(null)),
        ("Name", json!(42)),
        ("Name", json!(" ")),
        ("Type", json!([])),
        ("Type", json!("")),
        ("MaxHealth", json!("780")),
        ("MaxHealth", json!(null)),
        ("Stamina", json!(true)),
        ("Weapon", json!([])),
        ("Weapon", json!(null)),
        ("Weapon", json!({"BulletDamage": "8.8"})),
    ] {
        let mut blob = json!({"hero_a":{"Name":"A"},"hero_b":{"Name":"B"}});
        blob["hero_b"][field] = value;
        rejected(&serde_json::to_vec(&blob).unwrap(), "malformed_hero");
    }
}

#[test]
fn oversized_text_and_alias_reject() {
    for (field, length) in [
        ("Type", limits::TEXT_BYTES + 1),
        ("Name", limits::ALIAS_BYTES + 1),
    ] {
        let mut value = json!({"hero_a":{"Name":"A"}});
        value["hero_a"][field] = json!("x".repeat(length));
        rejected(&serde_json::to_vec(&value).unwrap(), "value_too_large");
    }
}

#[test]
fn receipts_hash_original_string_and_numeric_tokens() {
    let raw = raw(
        br#"{"Hero-Baba":{"Name":"Baba","MaxHealth":7.800e+2,"Weapon":{"BulletDamage":8.800}}}"#,
    );
    let set = accepted(&raw);
    assert_eq!(set.entities[0].key, "hero_baba");
    for (stat, locator, token) in [
        ("name_en", "/Hero-Baba/Name", r#""Baba""#),
        ("max_health", "/Hero-Baba/MaxHealth", "7.800e+2"),
        (
            "weapon_bullet_damage",
            "/Hero-Baba/Weapon/BulletDamage",
            "8.800",
        ),
    ] {
        let fact = set.facts.iter().find(|f| f.stat_key == stat).unwrap();
        assert_eq!(fact.receipt.locator_kind, "json_pointer");
        assert_eq!(fact.receipt.locator, locator);
        assert_eq!(fact.receipt.value_sha256, hash(token.as_bytes()));
        assert_eq!(
            receipts::value_bytes(&raw.bytes, &fact.receipt).unwrap(),
            token.as_bytes()
        );
    }
    assert_eq!(
        set.aliases[0].receipt.value_sha256,
        set.facts[0].receipt.value_sha256
    );
    let token = format!(r#""{}u0042aba""#, char::from(92));
    let blob = format!(r#"{{"hero_baba":{{"Name":{token}}}}}"#);
    let escaped = accepted(&crate::raw(blob.as_bytes()));
    assert_eq!(escaped.aliases[0].alias, "baba");
    assert_eq!(
        escaped.facts[0].receipt.value_sha256,
        hash(token.as_bytes())
    );
}

fn actual() -> RawObject {
    let path = std::env::var_os("BRAIN_HERO_FIXTURE")
        .expect("set BRAIN_HERO_FIXTURE to the original hero-data.json");
    let bytes = std::fs::read(path).unwrap();
    assert_eq!(bytes.len(), 226_182);
    assert_eq!(format!("{:x}", Sha256::digest(&bytes)), HASH);
    raw(&bytes)
}

#[test]
#[ignore = "requires the original upstream blob via BRAIN_HERO_FIXTURE"]
fn original_blob_has_67_named_heroes_and_verified_receipts() {
    let set = accepted(&actual());
    assert_eq!(
        (set.entities.len(), set.aliases.len(), set.facts.len()),
        (67, 67, 845)
    );
    assert!(set.relations.is_empty());
    assert!(!set.entities.iter().any(|e| e.key == "hero_testhero"));
}

fn input(bytes: &[u8]) -> anyhow::Result<InputObject> {
    Ok(InputObject {
        revision: REVISION.into(),
        path: PATH.into(),
        bytes: bytes.to_vec(),
    })
}

async fn counts(pool: &PgPool) -> (i64, i64, i64) {
    sqlx::query_as("SELECT (SELECT count(*) FROM raw.object), (SELECT count(*) FROM canon.fact), (SELECT count(*) FROM canon.receipt)").fetch_one(pool).await.unwrap()
}

#[tokio::test]
#[ignore = "requires the original upstream blob via BRAIN_HERO_FIXTURE and local PostgreSQL 16"]
async fn original_blob_postgres_poison_retry_and_zero_growth_rerun() {
    let raw = actual();
    let pg = scratch_pg::ScratchPg::start();
    let options = PgConnectOptions::new_without_pgpass()
        .host(&pg.directory.join("socket").to_string_lossy())
        .port(55439)
        .username("brain_core_test")
        .database("postgres")
        .password("")
        .ssl_mode(PgSslMode::Disable);
    let admin = PgPoolOptions::new()
        .max_connections(2)
        .connect_with(options.clone())
        .await
        .unwrap();
    sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest NOLOGIN; CREATE ROLE brain_service NOLOGIN; CREATE ROLE brain_readonly NOLOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate; GRANT USAGE,CREATE ON SCHEMA public TO brain_migrate").execute(&admin).await.unwrap();
    PgStore::new(admin.clone())
        .migrate_gamefacts()
        .await
        .unwrap();
    let writer = PgPoolOptions::new()
        .max_connections(2)
        .after_connect(|c, _| {
            Box::pin(async move {
                sqlx::query("SET ROLE brain_ingest").execute(c).await?;
                Ok(())
            })
        })
        .connect_with(options)
        .await
        .unwrap();
    let store = GameFactsStore::new(writer.clone());
    let large = serde_json::to_vec(
        &json!({"hero_bad":{"Name":"Bad","Type":"x".repeat(limits::TEXT_BYTES + 1)}}),
    )
    .unwrap();
    let collision = br#"{"hero_bad":{"Name":"Bad","MaxHealth":1,"maxhealth":2}}"#;
    let first = ingest(
        &store,
        "deadlock-data",
        [input(&large), input(collision), input(&raw.bytes)],
        &[&Hero],
    )
    .await
    .unwrap();
    assert_eq!(first.counts.accepted, 1);
    assert_eq!(first.counts.rejected, 2);
    assert_eq!(first.counts.ignored, 0);
    let reasons: Vec<String> =
        sqlx::query_scalar("SELECT code FROM ops.ingest_rejection WHERE run_id=$1 ORDER BY id")
            .bind(first.run_id)
            .fetch_all(&admin)
            .await
            .unwrap();
    assert_eq!(reasons, ["value_too_large", "key_collision"]);
    let before = counts(&admin).await;
    assert_eq!(before, (3, 845, 979));
    let heroes: i64 =
        sqlx::query_scalar("SELECT count(*) FROM serve.current_entity WHERE kind='hero'")
            .fetch_one(&admin)
            .await
            .unwrap();
    assert_eq!(heroes, 67);
    let profiles: Vec<Value> = sqlx::query_scalar("SELECT identity FROM serve.hero_profile WHERE identity->>'key' IN('hero_baba','hero_bebop') ORDER BY identity->>'key'").fetch_all(&admin).await.unwrap();
    assert_eq!(
        profiles
            .iter()
            .map(|v| v["name_en"].as_str().unwrap())
            .collect::<Vec<_>>(),
        ["Baba", "Bebop"]
    );
    for (stat, baba, bebop) in [
        ("max_health", 780.0, 880.0),
        ("base_health_regen", 2.5, 2.5),
        ("max_move_speed", 6.5, 6.45),
        ("sprint_speed", 1.6, 4.0),
        ("stamina", 2.0, 3.0),
        ("light_melee_damage", 50.0, 63.0),
        ("heavy_melee_damage", 116.0, 116.0),
        ("weapon_bullet_damage", 8.8, 4.98),
        ("weapon_rounds_per_second", 3.2258, 11.905),
        ("weapon_clip_size", 15.0, 66.0),
        ("weapon_reload_time", 2.5, 2.35),
    ] {
        for (key, expected) in [("hero_baba", baba), ("hero_bebop", bebop)] {
            let row = sqlx::query("SELECT f.value_num,p.locator,p.value_sha256,p.content_sha256,p.source_revision,p.path FROM serve.current_fact f JOIN serve.current_entity e ON e.id=f.entity_id JOIN serve.published_receipt p ON p.id=f.receipt_id WHERE e.key=$1 AND f.stat_key=$2").bind(key).bind(stat).fetch_one(&admin).await.unwrap();
            assert_eq!(row.get::<f64, _>("value_num"), expected);
            assert_eq!(row.get::<String, _>("source_revision"), REVISION);
            assert_eq!(row.get::<String, _>("path"), PATH);
            assert_eq!(row.get::<Vec<u8>, _>("content_sha256"), raw.content_sha256);
            let receipt = brain_job::Receipt {
                locator_kind: "json_pointer".into(),
                locator: row.get("locator"),
                value_sha256: row.get::<Vec<u8>, _>("value_sha256").try_into().unwrap(),
            };
            assert_eq!(
                receipt.value_sha256,
                hash(&receipts::value_bytes(&raw.bytes, &receipt).unwrap())
            );
        }
    }
    let second = ingest(&store, "deadlock-data", [input(&raw.bytes)], &[&Hero])
        .await
        .unwrap();
    assert_eq!(second.counts.rejected, 0);
    assert_eq!(counts(&admin).await, before);
    let retry = ingest(
        &store,
        "deadlock-data",
        [input(&large), input(collision)],
        &[&Hero],
    )
    .await
    .unwrap();
    assert_eq!((retry.counts.accepted, retry.counts.rejected), (0, 2));
    assert_eq!(counts(&admin).await, before);
    let succeeded: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM ops.ingest_run WHERE status='succeeded' AND finished IS NOT NULL",
    )
    .fetch_one(&admin)
    .await
    .unwrap();
    assert_eq!(succeeded, 3);
    writer.close().await;
    admin.close().await;
}
