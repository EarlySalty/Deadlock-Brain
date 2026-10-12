use brain_job::{
    extract::{self, Atom},
    git::{Blob, Source},
    store::{Counts, Store},
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    ConnectOptions, PgPool,
};
use std::{path::PathBuf, process::Command};

struct Scratch {
    root: tempfile::TempDir,
}

impl Scratch {
    fn new() -> Self {
        let root = tempfile::Builder::new()
            .prefix("brain-job-pg-")
            .tempdir()
            .unwrap();
        std::fs::create_dir(root.path().join("socket")).unwrap();
        let scratch = Self { root };
        let status = Command::new("/usr/lib/postgresql/16/bin/initdb")
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .args([
                "--auth=trust",
                "--no-locale",
                "--encoding=UTF8",
                "--username=brain_test_dba",
                "--pgdata",
            ])
            .arg(scratch.root.path().join("data"))
            .output()
            .unwrap();
        assert!(status.status.success(), "initdb failed");
        let status = Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .arg("--pgdata")
            .arg(scratch.root.path().join("data"))
            .arg("--log")
            .arg(scratch.root.path().join("postgres.log"))
            .args(["--wait", "--options"])
            .arg(format!(
                "-k {} -p 55468 -h '' -c fsync=off",
                scratch.root.path().join("socket").display()
            ))
            .arg("start")
            .output()
            .unwrap();
        assert!(status.status.success(), "pg_ctl start failed");
        scratch
    }

    async fn pool(&self, database: &str, user: &str) -> PgPool {
        PgPoolOptions::new()
            .max_connections(2)
            .connect_with(
                PgConnectOptions::new_without_pgpass()
                    .host(self.root.path().join("socket").to_str().unwrap())
                    .port(55468)
                    .database(database)
                    .username(user)
                    .password("")
                    .ssl_mode(PgSslMode::Disable)
                    .disable_statement_logging(),
            )
            .await
            .unwrap()
    }

    async fn ingest_pool(&self) -> PgPool {
        let admin = self.pool("postgres", "brain_test_dba").await;
        let directory: String = sqlx::query_scalar("SELECT current_setting('data_directory')")
            .fetch_one(&admin)
            .await
            .unwrap();
        assert_eq!(PathBuf::from(directory), self.root.path().join("data"));
        sqlx::raw_sql("CREATE ROLE brain_migrate LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE; CREATE ROLE brain_ingest LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE; CREATE ROLE brain_service LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE; CREATE ROLE brain_tool NOLOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOREPLICATION NOBYPASSRLS;").execute(&admin).await.unwrap();
        sqlx::query("CREATE DATABASE brain_job_test OWNER brain_migrate")
            .execute(&admin)
            .await
            .unwrap();
        let owner = self.pool("brain_job_test", "brain_migrate").await;
        sqlx::raw_sql(include_str!(
            "../../../../scripts/migrations/2026-10-12-brain-gamefacts-v1.sql"
        ))
        .execute(&owner)
        .await
        .unwrap();
        sqlx::query("ALTER ROLE brain_ingest SET default_transaction_isolation='repeatable read'")
            .execute(&admin)
            .await
            .unwrap();
        owner.close().await;
        admin.close().await;
        self.pool("brain_job_test", "brain_ingest").await
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let status = Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .arg("--pgdata")
            .arg(self.root.path().join("data"))
            .args(["--wait", "--mode=immediate", "stop"])
            .output();
        assert!(
            status.is_ok_and(|status| status.status.success()),
            "pg_ctl stop failed"
        );
    }
}

fn blob(bytes: &[u8]) -> Blob {
    Blob {
        bytes: bytes.to_vec(),
        hash: Sha256::digest(bytes).to_vec(),
    }
}

async fn current_health(pool: &PgPool) -> String {
    sqlx::query_scalar("SELECT value_num::text FROM serve.current_fact f JOIN canon.entity e ON e.id=f.entity_id WHERE e.key='hero_baba' AND f.stat_key='stats.max_health'")
        .fetch_one(pool).await.unwrap()
}

#[tokio::test]
async fn real_postgres_dedup_resume_proof_rollback_and_path_deletion() {
    let scratch = Scratch::new();
    let pool = scratch.ingest_pool().await;
    let source = Source::ALL[0];
    let store = Store::start(pool.clone(), 300).await.unwrap();
    assert!(Store::start(pool.clone(), 300).await.is_err());
    let source_id = store.source(source).await.unwrap();
    let mut counts = Counts::default();
    let a = br#"{"hero_baba":{"Name":"Baba","MaxHealth":780.0,"BoundAbilities":{"1":{"Key":"ability_baba_bubbling_brew"},"2":{"Key":"ability_baba_bench_run"},"3":{"Key":"ability_baba_hexing_brew"},"4":{"Key":"ability_baba_ultimate_2"}}}}"#;
    let b = br#"{"hero_baba":{"Name":"Baba","MaxHealth":800.0}}"#;
    let path = "data/json/hero-data.json";
    let run_a = store
        .begin_run(source_id, &"a".repeat(40))
        .await
        .unwrap()
        .unwrap();
    store
        .object(&run_a, source, path, Some(blob(a)), &mut counts)
        .await
        .unwrap();
    assert!(store.completed(&run_a, path).await.unwrap());
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM serve.hero_profile")
            .fetch_one(&pool)
            .await
            .unwrap(),
        0
    );
    store.publish(&run_a).await.unwrap();
    assert_eq!(current_health(&pool).await, "780.0");
    assert!(store
        .begin_run(source_id, &run_a.target)
        .await
        .unwrap()
        .is_none());
    let identity: i64 =
        sqlx::query_scalar("SELECT identity_receipt_id FROM canon.entity WHERE key='hero_baba'")
            .fetch_one(&pool)
            .await
            .unwrap();
    let profile: Value =
        sqlx::query_scalar("SELECT abilities FROM serve.hero_profile WHERE key='hero_baba'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(profile["links"].as_array().unwrap().len(), 4);
    let run_b = store
        .begin_run(source_id, &"b".repeat(40))
        .await
        .unwrap()
        .unwrap();
    store
        .object(&run_b, source, path, Some(blob(b)), &mut counts)
        .await
        .unwrap();
    assert_eq!(current_health(&pool).await, "780.0");
    store.finish(&counts, true).await.unwrap();
    let resumed_store = Store::start(pool.clone(), 300).await.unwrap();
    let resumed = resumed_store.pending(source_id).await.unwrap().unwrap();
    assert_eq!(resumed.id, run_b.id);
    assert_eq!(resumed.target, run_b.target);
    assert!(resumed_store.completed(&resumed, path).await.unwrap());
    resumed_store.publish(&resumed).await.unwrap();
    assert_eq!(current_health(&pool).await, "800.0");
    let run_c = resumed_store
        .begin_run(source_id, &"c".repeat(40))
        .await
        .unwrap()
        .unwrap();
    resumed_store
        .object(&run_c, source, path, Some(blob(a)), &mut counts)
        .await
        .unwrap();
    resumed_store.publish(&run_c).await.unwrap();
    assert_eq!(current_health(&pool).await, "780.0");
    assert_eq!(counts.raw_inserted, 2);
    assert_eq!(counts.raw_reused, 1);
    let revision: String = sqlx::query_scalar("SELECT v.source_revision FROM serve.current_fact f JOIN canon.revision v ON v.id=f.valid_from_revision_id JOIN canon.entity e ON e.id=f.entity_id WHERE e.key='hero_baba' AND f.stat_key='stats.max_health'").fetch_one(&pool).await.unwrap();
    assert_eq!(revision, run_c.target);
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT identity_receipt_id FROM canon.entity WHERE key='hero_baba'"
        )
        .fetch_one(&pool)
        .await
        .unwrap(),
        identity
    );
    let run_d = resumed_store
        .begin_run(source_id, &"d".repeat(40))
        .await
        .unwrap()
        .unwrap();
    let old_objects: i64 = sqlx::query_scalar("SELECT count(*) FROM raw.object")
        .fetch_one(&pool)
        .await
        .unwrap();
    let old_receipts: i64 = sqlx::query_scalar("SELECT count(*) FROM canon.receipt")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(resumed_store
        .object(
            &run_d,
            source,
            path,
            Some(blob(br#"{"hero_baba":{"MaxHealth":1,"MaxHealth":2}}"#)),
            &mut counts
        )
        .await
        .is_err());
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM raw.object")
            .fetch_one(&pool)
            .await
            .unwrap(),
        old_objects
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM canon.receipt")
            .fetch_one(&pool)
            .await
            .unwrap(),
        old_receipts
    );
    assert!(!resumed_store.completed(&run_d, path).await.unwrap());
    assert_eq!(current_health(&pool).await, "780.0");
    let mut corrupted = blob(a);
    corrupted.hash[0] ^= 1;
    assert!(resumed_store
        .object(&run_d, source, path, Some(corrupted), &mut counts)
        .await
        .is_err());
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM raw.object")
            .fetch_one(&pool)
            .await
            .unwrap(),
        old_objects
    );
    resumed_store
        .object(
            &run_d,
            source,
            "data/localizations/english.json",
            Some(blob(br#"{"hero_baba":"  BABA   Oma  "}"#)),
            &mut counts,
        )
        .await
        .unwrap();
    resumed_store
        .object(&run_d, source, path, None, &mut counts)
        .await
        .unwrap();
    resumed_store.publish(&run_d).await.unwrap();
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM serve.current_fact WHERE stat_key='stats.max_health'"
        )
        .fetch_one(&pool)
        .await
        .unwrap(),
        0
    );
    assert_eq!(
        sqlx::query_scalar::<_, String>(
            "SELECT alias_norm FROM serve.current_alias WHERE alias_norm='baba oma'"
        )
        .fetch_one(&pool)
        .await
        .unwrap(),
        "baba oma"
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM raw.object WHERE external_id=$1")
            .bind(path)
            .fetch_one(&pool)
            .await
            .unwrap(),
        2
    );
    let mut tx = pool.begin().await.unwrap();
    sqlx::query("SET TRANSACTION ISOLATION LEVEL READ COMMITTED")
        .execute(&mut *tx)
        .await
        .unwrap();
    sqlx::query(
        "UPDATE ops.job_lease SET epoch=epoch+1 WHERE lease_key='brain-job:ingest-gamefiles'",
    )
    .execute(&mut *tx)
    .await
    .unwrap();
    tx.commit().await.unwrap();
    assert!(resumed_store.renew().await.is_err());
    pool.close().await;
}

#[tokio::test]
async fn ability_and_weapon_source_fields_remain_distinct_in_postgres() {
    let scratch = Scratch::new();
    let pool = scratch.ingest_pool().await;
    let store = Store::start(pool.clone(), 300).await.unwrap();
    let mut counts = Counts::default();
    let cases: [(Source, &str, &[u8], &[&str]); 4] = [
        (
            Source::ALL[0],
            "data/json/ability-data.json",
            br#"{"ability_baba_bubbling_brew":{"MaxHealth":1,"max_health":2}}"#,
            &["ability.MaxHealth", "ability.max_health"],
        ),
        (
            Source::ALL[0],
            "data/json/hero-data.json",
            br#"{"hero_baba":{"MaxHealth":780,"Weapon":{"MaxHealth":1,"max_health":2}}}"#,
            &["stats.max_health", "weapon.MaxHealth", "weapon.max_health"],
        ),
        (
            Source::ALL[1],
            "game/citadel/pak01_dir/scripts/abilities.vdata",
            br#"<!-- kv3 encoding:text --> { ability_baba_bubbling_brew = { m_eAbilityType = "EAbilityType_Signature" m_mapAbilityProperties = { MaxHealth = { m_strValue = "1" } max_health = { m_strValue = "2" } } } }"#,
            &["ability.MaxHealth", "ability.max_health"],
        ),
        (
            Source::ALL[1],
            "game/citadel/pak01_dir/scripts/abilities.vdata",
            br#"ability_baba_bubbling_brew { m_eAbilityType "EAbilityType_Signature" m_mapAbilityProperties { MaxHealth { m_strValue "1" } max_health { m_strValue "2" } } }"#,
            &["ability.MaxHealth", "ability.max_health"],
        ),
    ];
    for (index, (source, path, bytes, keys)) in cases.into_iter().enumerate() {
        let facts = extract::extract(source, path, bytes).unwrap();
        extract::prove(source, path, bytes, &facts).unwrap();
        assert_eq!(
            facts
                .iter()
                .map(|fact| fact.stat.as_str())
                .collect::<Vec<_>>(),
            keys
        );
        let source_id = store.source(source).await.unwrap();
        let run = store
            .begin_run(source_id, &format!("{:040x}", index + 1))
            .await
            .unwrap()
            .unwrap();
        store
            .object(&run, source, path, Some(blob(bytes)), &mut counts)
            .await
            .unwrap();
        store.publish(&run).await.unwrap();
        let actual: Vec<(String, String, String, Vec<u8>)> = sqlx::query_as(
            "SELECT f.stat_key,f.value_num::text,r.locator,r.value_sha256
            FROM serve.current_fact f JOIN canon.receipt r ON r.id=f.receipt_id
            WHERE f.source_id=$1 AND f.source_path=$2 ORDER BY f.stat_key",
        )
        .bind(source_id)
        .bind(path)
        .fetch_all(&pool)
        .await
        .unwrap();
        let mut expected: Vec<_> = facts
            .iter()
            .map(|fact| {
                let Atom::Number(value) = &fact.value else {
                    panic!("numeric source field");
                };
                (
                    fact.stat.clone(),
                    value.clone(),
                    fact.locator.clone(),
                    fact.hash.clone(),
                )
            })
            .collect();
        expected.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(actual, expected);
    }
    assert_eq!(counts.raw_inserted, 4);
    assert_eq!(counts.objects_completed, 4);
    store.finish(&counts, false).await.unwrap();
    pool.close().await;
}

#[tokio::test]
async fn localized_ability_names_use_receipt_backed_identity_without_prefixes() {
    let scratch = Scratch::new();
    let pool = scratch.ingest_pool().await;
    let store = Store::start(pool.clone(), 300).await.unwrap();
    let source = Source::ALL[0];
    let source_id = store.source(source).await.unwrap();
    let run = store
        .begin_run(source_id, &"a".repeat(40))
        .await
        .unwrap()
        .unwrap();
    let mut counts = Counts::default();
    store
        .object(
            &run,
            source,
            "data/json/ability-data.json",
            Some(blob(
                br#"{"viscous_goo_grenade":{"Name":"Splatter","AbilityCooldown":26.0}}"#,
            )),
            &mut counts,
        )
        .await
        .unwrap();
    let bytes = br#"{"viscous_goo_grenade":"Klecks","ability_unbound":"Ausgedacht"}"#;
    store
        .object(
            &run,
            source,
            "data/localizations/german.json",
            Some(blob(bytes)),
            &mut counts,
        )
        .await
        .unwrap();
    store.publish(&run).await.unwrap();
    let names: Vec<(String, String, String, Vec<u8>)> = sqlx::query_as(
        "SELECT e.key,a.alias_norm,r.locator,r.value_sha256 FROM serve.current_alias a
        JOIN canon.entity e ON e.id=a.entity_id JOIN canon.receipt r ON r.id=a.receipt_id
        WHERE a.lang='de'",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(
        names,
        vec![(
            "viscous_goo_grenade".into(),
            "klecks".into(),
            "/viscous_goo_grenade".into(),
            Sha256::digest(br#""Klecks""#).to_vec()
        )]
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM canon.entity WHERE key='ability_unbound'"
        )
        .fetch_one(&pool)
        .await
        .unwrap(),
        0
    );
    store.finish(&counts, false).await.unwrap();
    pool.close().await;
}

#[tokio::test]
async fn localization_uses_current_source_evidence_without_replacing_cross_source_identity() {
    let scratch = Scratch::new();
    let pool = scratch.ingest_pool().await;
    let store = Store::start(pool.clone(), 300).await.unwrap();
    let mut counts = Counts::default();
    let tracking = Source::ALL[1];
    let tracking_id = store.source(tracking).await.unwrap();
    let tracking_run = store
        .begin_run(tracking_id, &"a".repeat(40))
        .await
        .unwrap()
        .unwrap();
    let tracking_path = "game/citadel/pak01_dir/scripts/abilities.vdata";
    let tracking_bytes = br#"viscous_goo_grenade { m_eAbilityType "EAbilityType_Signature" m_mapAbilityProperties { AbilityCooldown { m_strValue "26.0" } } }"#;
    store
        .object(
            &tracking_run,
            tracking,
            tracking_path,
            Some(blob(tracking_bytes)),
            &mut counts,
        )
        .await
        .unwrap();
    store.publish(&tracking_run).await.unwrap();
    let identity_query =
        "SELECT e.identity_receipt_id,r.source_id,r.revision_id,r.locator,r.value_sha256
        FROM canon.entity e JOIN canon.receipt r ON r.id=e.identity_receipt_id
        WHERE e.kind='ability' AND e.key='viscous_goo_grenade'";
    let original: (i64, i64, i64, String, Vec<u8>) = sqlx::query_as(identity_query)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(original.1, tracking_id);
    assert_eq!(original.2, tracking_run.revision);
    assert_eq!(
        original.3,
        "/viscous_goo_grenade/0/m_mapAbilityProperties/0/AbilityCooldown/0/m_strValue/0"
    );
    assert_eq!(original.4, Sha256::digest(b"26.0").to_vec());
    let original_object: (String, Vec<u8>, Vec<u8>) = sqlx::query_as(
        "SELECT o.external_id,o.body,o.content_sha256 FROM canon.receipt r
        JOIN raw.object o ON o.id=r.raw_object_id WHERE r.id=$1",
    )
    .bind(original.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(
        original_object,
        (
            tracking_path.into(),
            tracking_bytes.to_vec(),
            Sha256::digest(tracking_bytes).to_vec()
        )
    );
    let source = Source::ALL[0];
    let source_id = store.source(source).await.unwrap();
    let run = store
        .begin_run(source_id, &"b".repeat(40))
        .await
        .unwrap()
        .unwrap();
    store
        .object(
            &run,
            source,
            "data/json/ability-data.json",
            Some(blob(
                br#"{"viscous_goo_grenade":{"Name":"Splatter","AbilityCooldown":26.0}}"#,
            )),
            &mut counts,
        )
        .await
        .unwrap();
    let localized_bytes = br#"{"viscous_goo_grenade":"Glibber"}"#;
    let localized_path = "data/localizations/german.json";
    store
        .object(
            &run,
            source,
            localized_path,
            Some(blob(localized_bytes)),
            &mut counts,
        )
        .await
        .unwrap();
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM serve.current_alias WHERE lang='de'")
            .fetch_one(&pool)
            .await
            .unwrap(),
        0
    );
    store.publish(&run).await.unwrap();
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM serve.current_fact WHERE source_id=$1 AND source_path='data/json/ability-data.json'"
        )
        .bind(source_id)
        .fetch_one(&pool)
        .await
        .unwrap(),
        2
    );
    let mut previous_receipt = original.0;
    let mut revision = run.revision;
    for index in 0..2 {
        if index == 1 {
            let next = store
                .begin_run(source_id, &"c".repeat(40))
                .await
                .unwrap()
                .unwrap();
            store
                .object(
                    &next,
                    source,
                    localized_path,
                    Some(blob(localized_bytes)),
                    &mut counts,
                )
                .await
                .unwrap();
            store.publish(&next).await.unwrap();
            revision = next.revision;
        }
        let localized: (i64, i64, i64, String, String, Vec<u8>) = sqlx::query_as(
            "SELECT a.receipt_id,r.source_id,r.revision_id,a.alias_norm,r.locator,r.value_sha256
            FROM serve.current_alias a JOIN canon.entity e ON e.id=a.entity_id
            JOIN canon.receipt r ON r.id=a.receipt_id
            WHERE e.key='viscous_goo_grenade' AND a.lang='de' AND a.valid_from_revision_id=$1",
        )
        .bind(revision)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_ne!(localized.0, original.0);
        assert_ne!(localized.0, previous_receipt);
        previous_receipt = localized.0;
        assert_eq!(localized.1, source_id);
        assert_eq!(localized.2, revision);
        assert_eq!(localized.3, "glibber");
        assert_eq!(localized.4, "/viscous_goo_grenade");
        assert_eq!(localized.5, Sha256::digest(br#""Glibber""#).to_vec());
        let localized_object: (String, Vec<u8>, Vec<u8>) = sqlx::query_as(
            "SELECT o.external_id,o.body,o.content_sha256 FROM canon.receipt r
            JOIN raw.object o ON o.id=r.raw_object_id WHERE r.id=$1",
        )
        .bind(localized.0)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(
            localized_object,
            (
                localized_path.into(),
                localized_bytes.to_vec(),
                Sha256::digest(localized_bytes).to_vec()
            )
        );
        let preserved: (i64, i64, i64, String, Vec<u8>) = sqlx::query_as(identity_query)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(preserved, original);
    }
    assert_eq!(
        sqlx::query_scalar::<_, String>(
            "SELECT alias_norm FROM serve.current_alias WHERE lang='de'"
        )
        .fetch_one(&pool)
        .await
        .unwrap(),
        "glibber"
    );
    assert_eq!(counts.raw_inserted, 3);
    assert_eq!(counts.raw_reused, 1);
    store.finish(&counts, false).await.unwrap();
    pool.close().await;
}

#[tokio::test]
async fn localization_excludes_removed_deleted_unknown_and_unrelated_entity_evidence() {
    let scratch = Scratch::new();
    let pool = scratch.ingest_pool().await;
    let store = Store::start(pool.clone(), 300).await.unwrap();
    let mut counts = Counts::default();
    let tracking = Source::ALL[1];
    let tracking_id = store.source(tracking).await.unwrap();
    let unpublished = store
        .begin_run(tracking_id, &"a".repeat(40))
        .await
        .unwrap()
        .unwrap();
    store
        .object(
            &unpublished,
            tracking,
            "game/citadel/pak01_dir/scripts/abilities.vdata",
            Some(blob(br#"ability_unpublished { m_eAbilityType "EAbilityType_Signature" m_mapAbilityProperties { AbilityCooldown { m_strValue "26.0" } } }"#)),
            &mut counts,
        )
        .await
        .unwrap();
    let source = Source::ALL[0];
    let source_id = store.source(source).await.unwrap();
    let initial = store
        .begin_run(source_id, &"b".repeat(40))
        .await
        .unwrap()
        .unwrap();
    let ability_path = "data/json/ability-data.json";
    let hero_path = "data/json/hero-data.json";
    store
        .object(
            &initial,
            source,
            ability_path,
            Some(blob(
                br#"{"ability_live":{"AbilityCooldown":26},"ability_removed":{"Name":"Removed"},"ability_":{"AbilityCooldown":30}}"#,
            )),
            &mut counts,
        )
        .await
        .unwrap();
    store
        .object(
            &initial,
            source,
            hero_path,
            Some(blob(
                br#"{"hero_links":{"BoundAbilities":{"1":{"Key":"ability_reference_only"}}}}"#,
            )),
            &mut counts,
        )
        .await
        .unwrap();
    store.publish(&initial).await.unwrap();
    let next = store
        .begin_run(source_id, &"c".repeat(40))
        .await
        .unwrap()
        .unwrap();
    store
        .object(
            &next,
            source,
            ability_path,
            Some(blob(
                br#"{"ability_live":{"AbilityCooldown":27},"ability_":{"AbilityCooldown":31}}"#,
            )),
            &mut counts,
        )
        .await
        .unwrap();
    let localized_path = "data/localizations/german.json";
    let localized_bytes = r#"{"ability_live":"Aktuell","ability_removed":"Entfernt","hero_links":"Verknüpft","ability_reference_only":"Nur Referenz","ability_unpublished":"Unveröffentlicht","ability_unknown":"Unbekannt","ability_":"Präfix"}"#.as_bytes();
    store
        .object(
            &next,
            source,
            localized_path,
            Some(blob(localized_bytes)),
            &mut counts,
        )
        .await
        .unwrap();
    store.publish(&next).await.unwrap();
    let aliases: Vec<(String, String)> = sqlx::query_as(
        "SELECT e.key,a.alias_norm FROM serve.current_alias a
        JOIN canon.entity e ON e.id=a.entity_id WHERE a.lang='de' ORDER BY e.key",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(
        aliases,
        vec![
            ("ability_".into(), "präfix".into()),
            ("ability_live".into(), "aktuell".into()),
            ("hero_links".into(), "verknüpft".into())
        ]
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM canon.alias a JOIN canon.entity e ON e.id=a.entity_id
            WHERE e.key='ability_' AND a.lang='und'"
        )
        .fetch_one(&pool)
        .await
        .unwrap(),
        0
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM canon.entity WHERE key IN ('ability_reference_only','ability_unpublished','ability_removed')"
        )
        .fetch_one(&pool)
        .await
        .unwrap(),
        3
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM canon.entity WHERE key='ability_unknown'"
        )
        .fetch_one(&pool)
        .await
        .unwrap(),
        0
    );
    let deleted = store
        .begin_run(source_id, &"d".repeat(40))
        .await
        .unwrap()
        .unwrap();
    for path in [ability_path, hero_path] {
        store
            .object(&deleted, source, path, None, &mut counts)
            .await
            .unwrap();
    }
    store
        .object(
            &deleted,
            source,
            localized_path,
            Some(blob(localized_bytes)),
            &mut counts,
        )
        .await
        .unwrap();
    store.publish(&deleted).await.unwrap();
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM serve.current_alias WHERE lang='de'")
            .fetch_one(&pool)
            .await
            .unwrap(),
        0
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM serve.current_fact WHERE source_id=$1")
            .bind(source_id)
            .fetch_one(&pool)
            .await
            .unwrap(),
        0
    );
    store.finish(&counts, true).await.unwrap();
    pool.close().await;
}

#[tokio::test]
async fn localization_requires_applicable_extractor_version_evidence() {
    let scratch = Scratch::new();
    let pool = scratch.ingest_pool().await;
    let owner = scratch.pool("brain_job_test", "brain_migrate").await;
    sqlx::raw_sql(
        "CREATE FUNCTION ops.test_old_extractor() RETURNS trigger LANGUAGE plpgsql
        SET search_path=pg_catalog,pg_temp AS $fixture$
        BEGIN
            NEW.extractor_version := 'p0-v0';
            RETURN NEW;
        END
        $fixture$;",
    )
    .execute(&owner)
    .await
    .unwrap();
    for table in ["raw.run", "canon.revision", "canon.receipt"] {
        sqlx::query(&format!(
            "CREATE TRIGGER test_old_extractor BEFORE INSERT ON {table}
            FOR EACH ROW EXECUTE FUNCTION ops.test_old_extractor()"
        ))
        .execute(&owner)
        .await
        .unwrap();
    }
    let store = Store::start(pool.clone(), 300).await.unwrap();
    let source = Source::ALL[0];
    let source_id = store.source(source).await.unwrap();
    let mut counts = Counts::default();
    let stale = store
        .begin_run(source_id, &"a".repeat(40))
        .await
        .unwrap()
        .unwrap();
    let bytes = br#"{"ability_stale":{"Name":"Old","AbilityCooldown":26}}"#;
    store
        .object(
            &stale,
            source,
            "data/json/ability-data.json",
            Some(blob(bytes)),
            &mut counts,
        )
        .await
        .unwrap();
    store.publish(&stale).await.unwrap();
    for table in ["raw.run", "canon.revision", "canon.receipt"] {
        sqlx::query(&format!("DROP TRIGGER test_old_extractor ON {table}"))
            .execute(&owner)
            .await
            .unwrap();
    }
    let identity: (i64, String) = sqlx::query_as(
        "SELECT e.identity_receipt_id,r.extractor_version FROM canon.entity e
        JOIN canon.receipt r ON r.id=e.identity_receipt_id WHERE e.key='ability_stale'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(identity.1, "p0-v0");
    let current = store
        .begin_run(source_id, &"b".repeat(40))
        .await
        .unwrap()
        .unwrap();
    assert!(current.full);
    let path = "data/localizations/german.json";
    let localized = br#"{"ability_stale":"Aktuell"}"#;
    store
        .object(&current, source, path, Some(blob(localized)), &mut counts)
        .await
        .unwrap();
    store.publish(&current).await.unwrap();
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM canon.alias WHERE lang='de'")
            .fetch_one(&pool)
            .await
            .unwrap(),
        0
    );
    let refreshed = store
        .begin_run(source_id, &"c".repeat(40))
        .await
        .unwrap()
        .unwrap();
    store
        .object(
            &refreshed,
            source,
            "data/json/ability-data.json",
            Some(blob(bytes)),
            &mut counts,
        )
        .await
        .unwrap();
    store
        .object(&refreshed, source, path, Some(blob(localized)), &mut counts)
        .await
        .unwrap();
    store.publish(&refreshed).await.unwrap();
    let preserved: (i64, String) = sqlx::query_as(
        "SELECT e.identity_receipt_id,r.extractor_version FROM canon.entity e
        JOIN canon.receipt r ON r.id=e.identity_receipt_id WHERE e.key='ability_stale'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(preserved, identity);
    let receipt_version: String = sqlx::query_scalar(
        "SELECT r.extractor_version FROM serve.current_alias a
        JOIN canon.receipt r ON r.id=a.receipt_id WHERE a.lang='de'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(receipt_version, extract::VERSION);
    store.finish(&counts, false).await.unwrap();
    owner.close().await;
    pool.close().await;
}

#[tokio::test]
async fn startup_requires_v1_capability_with_singleton_and_multi_version_tables() {
    let scratch = Scratch::new();
    let pool = scratch.ingest_pool().await;
    let owner = scratch.pool("brain_job_test", "brain_migrate").await;
    for definition in [
        "singleton boolean PRIMARY KEY DEFAULT true CHECK(singleton), version integer NOT NULL CHECK(version=1)",
        "version integer PRIMARY KEY CHECK(version>0)",
    ] {
        sqlx::raw_sql(&format!(
            "DROP TABLE ops.gamefacts_schema_version;
            CREATE TABLE ops.gamefacts_schema_version({definition});
            GRANT SELECT ON ops.gamefacts_schema_version TO brain_ingest;
            INSERT INTO ops.gamefacts_schema_version(version) VALUES(1);"
        ))
        .execute(&owner)
        .await
        .unwrap();
        if !definition.starts_with("singleton") {
            sqlx::query("INSERT INTO ops.gamefacts_schema_version(version) VALUES(2)")
                .execute(&owner)
                .await
                .unwrap();
        }
        let store = Store::start(pool.clone(), 300).await.unwrap();
        store.finish(&Counts::default(), false).await.unwrap();
        sqlx::query("DELETE FROM ops.gamefacts_schema_version WHERE version=1")
            .execute(&owner)
            .await
            .unwrap();
        let count: i64 = sqlx::query_scalar("SELECT count(*) FROM ops.job_run")
            .fetch_one(&pool)
            .await
            .unwrap();
        let error = Store::start(pool.clone(), 300).await.err().unwrap();
        assert_eq!(error.to_string(), "schema_version");
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT count(*) FROM ops.job_run")
                .fetch_one(&pool)
                .await
                .unwrap(),
            count
        );
    }
    owner.close().await;
    pool.close().await;
}

#[tokio::test]
#[ignore]
async fn immutable_public_objects_persist_with_independently_resolved_receipts() {
    let root = PathBuf::from(
        std::env::var_os("BRAIN_JOB_PUBLIC_FIXTURES").expect("public fixture directory"),
    );
    let scratch = Scratch::new();
    let pool = scratch.ingest_pool().await;
    let store = Store::start(pool.clone(), 300).await.unwrap();
    let mut counts = Counts::default();
    for (source, tree, objects) in [
        (
            Source::ALL[0],
            "data-tree.json",
            [
                ("ability-data.json", "data/json/ability-data.json"),
                ("hero-data.json", "data/json/hero-data.json"),
            ],
        ),
        (
            Source::ALL[1],
            "tree.json",
            [
                (
                    "abilities.vdata",
                    "game/citadel/pak01_dir/scripts/abilities.vdata",
                ),
                (
                    "heroes.vdata",
                    "game/citadel/pak01_dir/scripts/heroes.vdata",
                ),
            ],
        ),
    ] {
        let tree: Value = serde_json::from_slice(&std::fs::read(root.join(tree)).unwrap()).unwrap();
        let revision = tree["sha"].as_str().unwrap();
        let source_id = store.source(source).await.unwrap();
        let run = store.begin_run(source_id, revision).await.unwrap().unwrap();
        for (local, path) in objects {
            let bytes = std::fs::read(root.join(local)).unwrap();
            store
                .object(&run, source, path, Some(blob(&bytes)), &mut counts)
                .await
                .unwrap();
            let stored: (Vec<u8>, String) = sqlx::query_as("SELECT content_sha256,revision FROM raw.object WHERE source_id=$1 AND external_id=$2")
                .bind(source_id).bind(path).fetch_one(&pool).await.unwrap();
            assert_eq!(
                stored,
                (Sha256::digest(&bytes).to_vec(), revision.to_owned())
            );
        }
        store.publish(&run).await.unwrap();
        assert!(store
            .begin_run(source_id, revision)
            .await
            .unwrap()
            .is_none());
    }
    assert_eq!(counts.raw_inserted, 4);
    assert_eq!(counts.objects_completed, 4);
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM canon.receipt")
            .fetch_one(&pool)
            .await
            .unwrap(),
        21946
    );
    let abilities: Value =
        serde_json::from_slice(&std::fs::read(root.join("ability-data.json")).unwrap()).unwrap();
    for key in [
        "citadel_ability_bull_heal",
        "cadence_ability_anthem",
        "drifter_blood_blast",
        "mirage_fire_beetles",
        "viscous_goo_grenade",
        "yakuza_shakedown_target",
    ] {
        let actual: (String, String, Vec<u8>, String) = sqlx::query_as(
            "SELECT f.value_num::text,r.locator,r.value_sha256,v.source_revision FROM serve.current_fact f
            JOIN canon.entity e ON e.id=f.entity_id JOIN canon.receipt r ON r.id=f.receipt_id
            JOIN canon.revision v ON v.id=r.revision_id
            WHERE e.kind='ability' AND e.key=$1 AND f.source_path='data/json/ability-data.json'
            AND f.stat_key='ability.AbilityCooldown' AND NOT f.condition ? 'upgrade_tier'",
        ).bind(key).fetch_one(&pool).await.unwrap();
        let value = &abilities[key]["AbilityCooldown"];
        let (number, locator) = if value.is_object() {
            (
                value["Value"].to_string(),
                format!("/{key}/AbilityCooldown/Value"),
            )
        } else {
            (value.to_string(), format!("/{key}/AbilityCooldown"))
        };
        let revision: Value =
            serde_json::from_slice(&std::fs::read(root.join("data-tree.json")).unwrap()).unwrap();
        assert_eq!(
            actual,
            (
                number.clone(),
                locator,
                Sha256::digest(number.as_bytes()).to_vec(),
                revision["sha"].as_str().unwrap().to_owned()
            )
        );
    }
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM raw.object")
            .fetch_one(&pool)
            .await
            .unwrap(),
        4
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM canon.entity WHERE key='upgrade_clip_size'"
        )
        .fetch_one(&pool)
        .await
        .unwrap(),
        0
    );
    store.finish(&counts, false).await.unwrap();
    pool.close().await;
}

#[tokio::test]
async fn read_committed_writes_retry_entire_objects_without_duplicate_counts() {
    let scratch = Scratch::new();
    let pool = scratch.ingest_pool().await;
    assert_eq!(
        sqlx::query_scalar::<_, String>("SHOW default_transaction_isolation")
            .fetch_one(&pool)
            .await
            .unwrap(),
        "repeatable read"
    );
    let owner = scratch.pool("brain_job_test", "brain_migrate").await;
    sqlx::raw_sql(
        "CREATE SEQUENCE ops.test_retry_attempts;
        GRANT USAGE,SELECT ON SEQUENCE ops.test_retry_attempts TO brain_ingest;
        CREATE FUNCTION ops.test_retry_object() RETURNS trigger LANGUAGE plpgsql
        SET search_path=pg_catalog,pg_temp AS $retry$
        BEGIN
            IF nextval('ops.test_retry_attempts')<=coalesce(TG_ARGV[1]::bigint,1) THEN
                RAISE EXCEPTION 'retry_fixture' USING ERRCODE=TG_ARGV[0];
            END IF;
            RETURN NEW;
        END
        $retry$;",
    )
    .execute(&owner)
    .await
    .unwrap();
    let store = Store::start(pool.clone(), 300).await.unwrap();
    let source = Source::ALL[0];
    let source_id = store.source(source).await.unwrap();
    let mut counts = Counts::default();
    for (index, code) in ["40001", "40P01", "55P03"].into_iter().enumerate() {
        sqlx::raw_sql(&format!(
            "ALTER SEQUENCE ops.test_retry_attempts RESTART WITH 1;
            CREATE TRIGGER test_retry_object BEFORE INSERT ON raw.run_object
            FOR EACH ROW EXECUTE FUNCTION ops.test_retry_object('{code}');"
        ))
        .execute(&owner)
        .await
        .unwrap();
        let target = format!("{:040x}", index + 1);
        let run = store.begin_run(source_id, &target).await.unwrap().unwrap();
        let bytes = format!(
            "{{\"hero_baba\":{{\"Name\":\"Baba\",\"MaxHealth\":{}}}}}",
            780 + index
        );
        store
            .object(
                &run,
                source,
                "data/json/hero-data.json",
                Some(blob(bytes.as_bytes())),
                &mut counts,
            )
            .await
            .unwrap();
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT last_value FROM ops.test_retry_attempts")
                .fetch_one(&pool)
                .await
                .unwrap(),
            2
        );
        assert_eq!(counts.raw_inserted, u64::try_from(index + 1).unwrap());
        assert_eq!(counts.objects_completed, counts.raw_inserted);
        assert_eq!(counts.raw_reused, 0);
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT count(*) FROM raw.object")
                .fetch_one(&pool)
                .await
                .unwrap(),
            i64::try_from(index + 1).unwrap()
        );
        sqlx::query("DROP TRIGGER test_retry_object ON raw.run_object")
            .execute(&owner)
            .await
            .unwrap();
        store.publish(&run).await.unwrap();
    }
    let run = store
        .begin_run(source_id, &"d".repeat(40))
        .await
        .unwrap()
        .unwrap();
    for (code, failures, attempts) in [("23514", 1, 1), ("40001", 3, 3)] {
        sqlx::raw_sql(&format!(
            "ALTER SEQUENCE ops.test_retry_attempts RESTART WITH 1;
            CREATE TRIGGER test_retry_object BEFORE INSERT ON raw.run_object
            FOR EACH ROW EXECUTE FUNCTION ops.test_retry_object('{code}','{failures}');"
        ))
        .execute(&owner)
        .await
        .unwrap();
        let error = store
            .object(
                &run,
                source,
                "data/json/hero-data.json",
                Some(blob(br#"{"hero_baba":{"MaxHealth":1000}}"#)),
                &mut counts,
            )
            .await
            .unwrap_err();
        assert_eq!(
            error
                .downcast_ref::<sqlx::Error>()
                .and_then(sqlx::Error::as_database_error)
                .and_then(|error| error.code())
                .as_deref(),
            Some(code)
        );
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT last_value FROM ops.test_retry_attempts")
                .fetch_one(&pool)
                .await
                .unwrap(),
            attempts
        );
        assert_eq!(counts.raw_inserted, 3);
        assert_eq!(counts.objects_completed, 3);
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT count(*) FROM raw.object")
                .fetch_one(&pool)
                .await
                .unwrap(),
            3
        );
        assert!(!store
            .completed(&run, "data/json/hero-data.json")
            .await
            .unwrap());
        sqlx::query("DROP TRIGGER test_retry_object ON raw.run_object")
            .execute(&owner)
            .await
            .unwrap();
    }
    store.finish(&counts, true).await.unwrap();
    owner.close().await;
    pool.close().await;
}
