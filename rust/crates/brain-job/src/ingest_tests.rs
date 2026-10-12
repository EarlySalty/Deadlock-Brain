use super::*;
use serde_json::Value;
use sqlx::{postgres::PgConnectOptions, PgPool};
use std::{path::Path, process::Command};

struct ScratchPg(tempfile::TempDir);

impl ScratchPg {
    fn start() -> Self {
        let root = tempfile::Builder::new()
            .prefix("brain-job-incremental-")
            .tempdir()
            .unwrap();
        std::fs::create_dir(root.path().join("socket")).unwrap();
        let pg = Self(root);
        for (program, arguments) in [
            (
                "initdb",
                vec![
                    "-D".into(),
                    pg.0.path().join("data").into_os_string(),
                    "-A".into(),
                    "trust".into(),
                    "-U".into(),
                    "brain_test_dba".into(),
                    "--no-locale".into(),
                    "--encoding=UTF8".into(),
                ],
            ),
            (
                "pg_ctl",
                vec![
                    "-D".into(),
                    pg.0.path().join("data").into_os_string(),
                    "-l".into(),
                    pg.0.path().join("postgres.log").into_os_string(),
                    "-o".into(),
                    format!(
                        "-k {} -p 55468 -c listen_addresses='' -c fsync=off",
                        pg.0.path().join("socket").display()
                    )
                    .into(),
                    "-w".into(),
                    "start".into(),
                ],
            ),
        ] {
            let output = Command::new(format!("/usr/lib/postgresql/16/bin/{program}"))
                .env_clear()
                .env("PATH", "/usr/bin:/bin")
                .args(arguments)
                .output()
                .unwrap();
            assert!(output.status.success(), "{program} failed");
        }
        pg
    }

    async fn pool(&self, user: &str) -> PgPool {
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(2)
            .connect_with(
                PgConnectOptions::new_without_pgpass()
                    .host(self.0.path().join("socket").to_str().unwrap())
                    .port(55468)
                    .username(user)
                    .database("postgres"),
            )
            .await
            .unwrap()
    }
}

impl Drop for ScratchPg {
    fn drop(&mut self) {
        let output = Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .arg("-D")
            .arg(self.0.path().join("data"))
            .args(["-m", "immediate", "-w", "stop"])
            .output()
            .unwrap();
        assert!(output.status.success());
    }
}

fn fixture_git(state: &Path, arguments: &[&str]) -> String {
    let output = Command::new("/usr/bin/git")
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_AUTHOR_NAME", "Fixture")
        .env("GIT_AUTHOR_EMAIL", "fixture@example.invalid")
        .env("GIT_COMMITTER_NAME", "Fixture")
        .env("GIT_COMMITTER_EMAIL", "fixture@example.invalid")
        .args(["-c", "gc.auto=0", "--git-dir"])
        .arg(state.join("public-gamefiles-v1/deadlock-data.git"))
        .arg("--work-tree")
        .arg(state.join("fixture"))
        .args(arguments)
        .output()
        .unwrap();
    assert!(output.status.success(), "fixture git failed");
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

fn fixture_file(state: &Path, path: &str, bytes: Option<&[u8]>) {
    let path = state.join("fixture").join(path);
    if let Some(bytes) = bytes {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, bytes).unwrap();
    } else {
        std::fs::remove_file(path).unwrap();
    }
}

fn fixture_revision(state: &Path, parent: Option<&str>) -> String {
    fixture_git(state, &["add", "-A"]);
    let tree = fixture_git(state, &["write-tree"]);
    let mut arguments = vec!["commit-tree", &tree, "-m", "fixture"];
    if let Some(parent) = parent {
        arguments.extend(["-p", parent]);
    }
    fixture_git(state, &arguments)
}

async fn localized_aliases(pool: &PgPool) -> Vec<(String, String, String)> {
    sqlx::query_as(
        "SELECT e.key,a.lang,a.alias_norm FROM serve.current_alias a
        JOIN canon.entity e ON e.id=a.entity_id WHERE a.lang IN ('en','de')
        ORDER BY e.key,a.lang",
    )
    .fetch_all(pool)
    .await
    .unwrap()
}

fn expected_aliases(hero: Option<&str>, ability: Option<&str>) -> Vec<(String, String, String)> {
    let mut expected = Vec::new();
    for (key, de, en) in [
        ("hero_old", "alte heldin", "old hero"),
        ("hero_new", "neue heldin", "new hero"),
        ("ability_old", "alte fähigkeit", "old ability"),
        ("ability_new", "neue fähigkeit", "new ability"),
    ] {
        if hero == Some(key) || ability == Some(key) {
            expected.push((key.into(), "de".into(), de.into()));
            expected.push((key.into(), "en".into(), en.into()));
        }
    }
    expected.sort();
    expected
}

#[tokio::test]
async fn real_incremental_ingest_reprocesses_unchanged_localizations_after_identity_changes_and_resume(
) {
    let pg = ScratchPg::start();
    let admin = pg.pool("brain_test_dba").await;
    sqlx::raw_sql(
        "CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest LOGIN;
        CREATE ROLE brain_service LOGIN; CREATE ROLE brain_tool NOLOGIN;
        GRANT CREATE ON DATABASE postgres TO brain_migrate;",
    )
    .execute(&admin)
    .await
    .unwrap();
    let mut connection = admin.acquire().await.unwrap();
    sqlx::query("SET ROLE brain_migrate")
        .execute(&mut *connection)
        .await
        .unwrap();
    sqlx::raw_sql(include_str!(
        "../../../../scripts/migrations/2026-10-12-brain-gamefacts-v1.sql"
    ))
    .execute(&mut *connection)
    .await
    .unwrap();
    sqlx::query("INSERT INTO ops.gamefacts_schema_version(version) VALUES(1)")
        .execute(&mut *connection)
        .await
        .unwrap();
    drop(connection);
    let pool = pg.pool("brain_ingest").await;
    let state = tempfile::tempdir().unwrap();
    let source = Source::ALL[0];
    let repository = Repository::open(state.path(), source, 30).await.unwrap();
    let mut store = Store::start(pool.clone(), 300).await.unwrap();
    let source_id = store.source(source).await.unwrap();
    let hero_path = "data/json/hero-data.json";
    let ability_path = "data/json/ability-data.json";
    let english = br#"{"hero_old":"Old Hero","hero_new":"New Hero","ability_old":"Old Ability","ability_new":"New Ability","ability_reference_only":"Reference Only"}"#;
    let german = r#"{"hero_old":"Alte Heldin","hero_new":"Neue Heldin","ability_old":"Alte Fähigkeit","ability_new":"Neue Fähigkeit","ability_reference_only":"Nur Referenz"}"#.as_bytes();
    for (path, bytes) in [
        (hero_path, br#"{"hero_old":{"MaxHealth":780}}"#.as_slice()),
        (
            ability_path,
            br#"{"ability_old":{"AbilityCooldown":26}}"#.as_slice(),
        ),
        (LOCALIZATION_PATHS[0], english.as_slice()),
        (LOCALIZATION_PATHS[1], german),
    ] {
        fixture_file(state.path(), path, Some(bytes));
    }
    let initial_target = fixture_revision(state.path(), None);
    let initial = store
        .begin_run(source_id, &initial_target)
        .await
        .unwrap()
        .unwrap();
    assert!(initial.full);
    let mut initial_counts = Counts::default();
    process(
        &store,
        &repository,
        source,
        &initial,
        1024 * 1024,
        &mut initial_counts,
    )
    .await
    .unwrap();
    assert_eq!(initial_counts.raw_inserted, 4);
    assert_eq!(initial_counts.objects_completed, 4);
    assert_eq!(
        localized_aliases(&pool).await,
        expected_aliases(Some("hero_old"), Some("ability_old"))
    );
    println!("initial: {}", initial_counts.value());
    let immutable_raw: Value =
        sqlx::query_scalar("SELECT jsonb_agg(to_jsonb(o) ORDER BY id) FROM raw.object o")
            .fetch_one(&pool)
            .await
            .unwrap();
    let immutable_receipts: Value =
        sqlx::query_scalar("SELECT jsonb_agg(to_jsonb(r) ORDER BY id) FROM canon.receipt r")
            .fetch_one(&pool)
            .await
            .unwrap();
    let last_receipt: i64 = sqlx::query_scalar("SELECT max(id) FROM canon.receipt")
        .fetch_one(&pool)
        .await
        .unwrap();
    let identities: Vec<(String, i64)> =
        sqlx::query_as("SELECT key,identity_receipt_id FROM canon.entity ORDER BY key")
            .fetch_all(&pool)
            .await
            .unwrap();
    fixture_file(state.path(), hero_path, Some(br#"{"hero_new":{"MaxHealth":800,"BoundAbilities":{"1":{"Key":"ability_reference_only"}}}}"#));
    let hero_target = fixture_revision(state.path(), Some(&initial_target));
    let hero_run = store
        .begin_run(source_id, &hero_target)
        .await
        .unwrap()
        .unwrap();
    assert!(!hero_run.full);
    sqlx::raw_sql(
        "CREATE FUNCTION ops.test_localization_failure() RETURNS trigger LANGUAGE plpgsql
        SET search_path=pg_catalog,pg_temp AS $fixture$
        BEGIN
            IF NEW.path='data/localizations/german.json' THEN
                RAISE EXCEPTION 'localization_failure' USING ERRCODE='23514';
            END IF;
            RETURN NEW;
        END
        $fixture$;
        CREATE TRIGGER test_localization_failure BEFORE INSERT ON raw.run_object
        FOR EACH ROW EXECUTE FUNCTION ops.test_localization_failure();",
    )
    .execute(&admin)
    .await
    .unwrap();
    let mut partial_counts = Counts::default();
    assert!(process(
        &store,
        &repository,
        source,
        &hero_run,
        1024 * 1024,
        &mut partial_counts
    )
    .await
    .is_err());
    assert_eq!(partial_counts.raw_inserted, 1);
    assert_eq!(partial_counts.raw_reused, 1);
    assert_eq!(partial_counts.objects_completed, 2);
    assert!(store.completed(&hero_run, hero_path).await.unwrap());
    assert!(store
        .completed(&hero_run, LOCALIZATION_PATHS[0])
        .await
        .unwrap());
    assert!(!store
        .completed(&hero_run, LOCALIZATION_PATHS[1])
        .await
        .unwrap());
    assert_eq!(
        localized_aliases(&pool).await,
        expected_aliases(Some("hero_old"), Some("ability_old"))
    );
    store.finish(&partial_counts, true).await.unwrap();
    println!("identity-change-partial: {}", partial_counts.value());
    sqlx::query("DROP TRIGGER test_localization_failure ON raw.run_object")
        .execute(&admin)
        .await
        .unwrap();
    store = Store::start(pool.clone(), 300).await.unwrap();
    let resumed = store.pending(source_id).await.unwrap().unwrap();
    assert_eq!(resumed.id, hero_run.id);
    let mut resumed_counts = Counts::default();
    process(
        &store,
        &repository,
        source,
        &resumed,
        1024 * 1024,
        &mut resumed_counts,
    )
    .await
    .unwrap();
    assert_eq!(resumed_counts.raw_inserted, 0);
    assert_eq!(resumed_counts.raw_reused, 1);
    assert_eq!(resumed_counts.objects_completed, 1);
    assert_eq!(
        localized_aliases(&pool).await,
        expected_aliases(Some("hero_new"), Some("ability_old"))
    );
    println!("identity-change-resumed: {}", resumed_counts.value());
    fixture_file(
        state.path(),
        ability_path,
        Some(br#"{"ability_new":{"AbilityCooldown":27}}"#),
    );
    let ability_target = fixture_revision(state.path(), Some(&hero_target));
    let ability_run = store
        .begin_run(source_id, &ability_target)
        .await
        .unwrap()
        .unwrap();
    let mut ability_counts = Counts::default();
    process(
        &store,
        &repository,
        source,
        &ability_run,
        1024 * 1024,
        &mut ability_counts,
    )
    .await
    .unwrap();
    assert_eq!(ability_counts.raw_inserted, 1);
    assert_eq!(ability_counts.raw_reused, 2);
    assert_eq!(ability_counts.objects_completed, 3);
    assert_eq!(
        localized_aliases(&pool).await,
        expected_aliases(Some("hero_new"), Some("ability_new"))
    );
    println!("ability-change: {}", ability_counts.value());
    let names: Vec<(String, String)> = sqlx::query_as(
        "SELECT e.key,f.value_text FROM serve.current_fact f JOIN canon.entity e ON e.id=f.entity_id
        WHERE f.source_path=ANY($1::text[]) ORDER BY e.key,f.value_text",
    ).bind(LOCALIZATION_PATHS.to_vec()).fetch_all(&pool).await.unwrap();
    assert_eq!(
        names,
        vec![
            ("ability_new".into(), "Neue Fähigkeit".into()),
            ("ability_new".into(), "New Ability".into()),
            ("hero_new".into(), "Neue Heldin".into()),
            ("hero_new".into(), "New Hero".into()),
        ]
    );
    let mut parent = ability_target;
    for (path, hero, ability) in [
        (hero_path, None, Some("ability_new")),
        (ability_path, None, None),
    ] {
        fixture_file(state.path(), path, None);
        let target = fixture_revision(state.path(), Some(&parent));
        let run = store.begin_run(source_id, &target).await.unwrap().unwrap();
        let mut counts = Counts::default();
        process(&store, &repository, source, &run, 1024 * 1024, &mut counts)
            .await
            .unwrap();
        assert_eq!(counts.raw_inserted, 0);
        assert_eq!(counts.raw_reused, 2);
        assert_eq!(counts.paths_deleted, 1);
        assert_eq!(counts.objects_completed, 3);
        assert_eq!(
            localized_aliases(&pool).await,
            expected_aliases(hero, ability)
        );
        println!("deleted {path}: {}", counts.value());
        parent = target;
    }
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM serve.current_fact")
            .fetch_one(&pool)
            .await
            .unwrap(),
        0
    );
    let unchanged_target = fixture_revision(state.path(), Some(&parent));
    let unchanged = store
        .begin_run(source_id, &unchanged_target)
        .await
        .unwrap()
        .unwrap();
    let mut unchanged_counts = Counts::default();
    process(
        &store,
        &repository,
        source,
        &unchanged,
        1024 * 1024,
        &mut unchanged_counts,
    )
    .await
    .unwrap();
    assert_eq!(unchanged_counts.raw_inserted, 0);
    assert_eq!(unchanged_counts.raw_reused, 0);
    assert_eq!(unchanged_counts.objects_completed, 0);
    assert!(store
        .begin_run(source_id, &unchanged_target)
        .await
        .unwrap()
        .is_none());
    println!("unchanged-content-followup: {}", unchanged_counts.value());
    let raw_after: Value = sqlx::query_scalar(
        "SELECT jsonb_agg(to_jsonb(o) ORDER BY id) FROM raw.object o WHERE id<=4",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(raw_after, immutable_raw);
    let receipts_after: Value = sqlx::query_scalar(
        "SELECT jsonb_agg(to_jsonb(r) ORDER BY id) FROM canon.receipt r WHERE id<=$1",
    )
    .bind(last_receipt)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(receipts_after, immutable_receipts);
    for (key, identity) in identities {
        let current: i64 =
            sqlx::query_scalar("SELECT identity_receipt_id FROM canon.entity WHERE key=$1")
                .bind(key)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(current, identity);
    }
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM raw.object")
            .fetch_one(&pool)
            .await
            .unwrap(),
        6
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM raw.object WHERE external_id=ANY($1::text[])",
        )
        .bind(LOCALIZATION_PATHS.to_vec())
        .fetch_one(&pool)
        .await
        .unwrap(),
        2
    );
    assert_eq!(
        std::fs::read(state.path().join("fixture").join(LOCALIZATION_PATHS[0])).unwrap(),
        english
    );
    assert_eq!(
        std::fs::read(state.path().join("fixture").join(LOCALIZATION_PATHS[1])).unwrap(),
        german
    );
    store.finish(&unchanged_counts, false).await.unwrap();
    pool.close().await;
    admin.close().await;
}
