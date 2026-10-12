use brain_job::{
    git::{Blob, Source},
    patch_store::{Counts, PatchStore},
    patchnotes::{self, Feed, MAX_ENVELOPE_BYTES},
    store,
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    ConnectOptions, PgPool,
};
use std::process::Command;

struct Scratch {
    root: tempfile::TempDir,
}

impl Scratch {
    fn new() -> Self {
        let root = tempfile::Builder::new()
            .prefix("brain-patch-ingest-pg-")
            .tempdir()
            .unwrap();
        std::fs::create_dir(root.path().join("socket")).unwrap();
        let scratch = Self { root };
        let initialized = Command::new("/usr/lib/postgresql/16/bin/initdb")
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
        assert!(initialized.status.success(), "initdb failed");
        let started = Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .arg("--pgdata")
            .arg(scratch.root.path().join("data"))
            .arg("--log")
            .arg(scratch.root.path().join("postgres.log"))
            .args(["--wait", "--options"])
            .arg(format!(
                "-k {} -p 55469 -h '' -c fsync=off",
                scratch.root.path().join("socket").display()
            ))
            .arg("start")
            .output()
            .unwrap();
        assert!(started.status.success(), "pg_ctl start failed");
        scratch
    }

    async fn pool(&self, database: &str, user: &str) -> PgPool {
        PgPoolOptions::new()
            .max_connections(2)
            .connect_with(
                PgConnectOptions::new_without_pgpass()
                    .host(self.root.path().join("socket").to_str().unwrap())
                    .port(55469)
                    .database(database)
                    .username(user)
                    .password("")
                    .ssl_mode(PgSslMode::Disable)
                    .disable_statement_logging(),
            )
            .await
            .unwrap()
    }

    async fn prepare(&self) -> (PgPool, PgPool) {
        let admin = self.pool("postgres", "brain_test_dba").await;
        let directory: String = sqlx::query_scalar("SELECT current_setting('data_directory')")
            .fetch_one(&admin)
            .await
            .unwrap();
        assert_eq!(
            std::path::PathBuf::from(directory),
            self.root.path().join("data")
        );
        sqlx::raw_sql(
            "CREATE ROLE brain_migrate LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
            CREATE ROLE brain_ingest LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
            CREATE ROLE brain_service LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
            CREATE ROLE brain_tool NOLOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOREPLICATION NOBYPASSRLS;
            CREATE ROLE patch_source_owner LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;
            CREATE ROLE patchnotes_feed_reader LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOREPLICATION NOBYPASSRLS;",
        ).execute(&admin).await.unwrap();
        sqlx::query("CREATE DATABASE brain_patch_target OWNER brain_migrate")
            .execute(&admin)
            .await
            .unwrap();
        sqlx::query("CREATE DATABASE brain_patch_source OWNER patch_source_owner")
            .execute(&admin)
            .await
            .unwrap();
        let owner = self.pool("brain_patch_target", "brain_migrate").await;
        sqlx::raw_sql(include_str!(
            "../../../../scripts/migrations/2026-10-12-brain-gamefacts-v1.sql"
        ))
        .execute(&owner)
        .await
        .unwrap();
        sqlx::raw_sql(include_str!(
            "../../../../scripts/migrations/2026-10-12-brain-gamefacts-p4-patches-v1.sql"
        ))
        .execute(&owner)
        .await
        .unwrap();
        let source = self.pool("brain_patch_source", "patch_source_owner").await;
        sqlx::raw_sql(
            "REVOKE CREATE ON SCHEMA public FROM PUBLIC;
            REVOKE CREATE ON DATABASE brain_patch_source FROM PUBLIC;
            CREATE SCHEMA patchnotes;
            CREATE TABLE patchnotes.changelog_posts (
                id bigint PRIMARY KEY,title text NOT NULL,url text NOT NULL,posted_at timestamptz,
                raw_content text,translated_content text);
            GRANT USAGE ON SCHEMA patchnotes TO patchnotes_feed_reader;
            GRANT SELECT ON patchnotes.changelog_posts TO patchnotes_feed_reader;",
        )
        .execute(&source)
        .await
        .unwrap();
        owner.close().await;
        admin.close().await;
        (
            self.pool("brain_patch_target", "brain_ingest").await,
            source,
        )
    }

    async fn reader(&self) -> PgPool {
        patchnotes::connect_source(&format!(
            "postgres://patchnotes_feed_reader:fixture@localhost:55469/brain_patch_source?host={}&sslmode=disable",
            self.root.path().join("socket").display(),
        )).await.unwrap()
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let stopped = Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .arg("--pgdata")
            .arg(self.root.path().join("data"))
            .args(["--wait", "--mode=immediate", "stop"])
            .output();
        assert!(
            stopped.is_ok_and(|status| status.status.success()),
            "pg_ctl stop failed"
        );
    }
}

async fn insert_post(source: &PgPool, id: i64, date: Option<&str>, content: Option<&str>) {
    sqlx::query(
        "INSERT INTO patchnotes.changelog_posts(id,title,url,posted_at,raw_content,translated_content)
        VALUES ($1,'Patch notes',$2,$3::text::timestamptz,$4,'Must not be ingested')",
    ).bind(id).bind(format!("https://forums.playdeadlock.com/threads/patch-notes.{id}/"))
        .bind(date).bind(content).execute(source).await.unwrap();
}

async fn catalog(pool: &PgPool) {
    let (store, run, counts) = stage_catalog(
        pool,
        &"a".repeat(40),
        br#"{"hero_baba":{"Name":"Baba","MaxHealth":780.0}}"#,
        None,
    )
    .await;
    store.publish(&run).await.unwrap();
    store.finish(&counts, false).await.unwrap();
}

async fn stage_catalog(
    pool: &PgPool,
    revision: &str,
    heroes: &[u8],
    abilities: Option<&[u8]>,
) -> (store::Store, store::Run, store::Counts) {
    let store = store::Store::start(pool.clone(), 300).await.unwrap();
    let source = Source::ALL[0];
    let source_id = store.source(source).await.unwrap();
    let run = store
        .begin_run(source_id, revision)
        .await
        .unwrap()
        .unwrap();
    let mut counts = store::Counts::default();
    for (path, bytes) in std::iter::once(("data/json/hero-data.json", heroes))
        .chain(abilities.map(|bytes| ("data/json/ability-data.json", bytes)))
    {
        store
            .object(
                &run,
                source,
                path,
                Some(Blob {
                    bytes: bytes.to_vec(),
                    hash: Sha256::digest(bytes).to_vec(),
                }),
                &mut counts,
            )
            .await
            .unwrap();
    }
    (store, run, counts)
}

async fn table_counts(pool: &PgPool) -> (i64, i64, i64, i64) {
    sqlx::query_as(
        "SELECT (SELECT count(*) FROM raw.object),(SELECT count(*) FROM canon.patch_change),
        (SELECT count(*) FROM canon.fact),(SELECT count(*) FROM canon.receipt)",
    )
    .fetch_one(pool)
    .await
    .unwrap()
}

async fn scan_once(target: &PgPool, reader: &PgPool) -> Counts {
    let store = PatchStore::start(target.clone(), 300).await.unwrap();
    let mut counts = Counts::default();
    patchnotes::scan(&store, reader, MAX_ENVELOPE_BYTES, &mut counts)
        .await
        .unwrap();
    store.finish(&counts, false).await.unwrap();
    counts
}

async fn patch_checkpoint(pool: &PgPool) -> (Option<String>, i64, i64, i64) {
    sqlx::query_as(
        "SELECT s.last_complete_revision,h.published_revision_id,
            (SELECT count(*) FROM raw.run r WHERE r.source_id=s.id),
            (SELECT count(*) FROM canon.revision v WHERE v.source_id=s.id)
        FROM raw.source s JOIN canon.source_head h ON h.source_id=s.id WHERE s.kind='patchnotes'",
    )
    .fetch_one(pool)
    .await
    .unwrap()
}

async fn patch_receipts(pool: &PgPool) -> Vec<String> {
    sqlx::query_scalar(
        "SELECT row_to_json(proof)::text FROM canon.receipt proof
        JOIN raw.source s ON s.id=proof.source_id WHERE s.kind='patchnotes' ORDER BY proof.id",
    )
    .fetch_all(pool)
    .await
    .unwrap()
}

async fn patch_raw(pool: &PgPool) -> (i64, Vec<u8>, Vec<u8>, String) {
    sqlx::query_as(
        "SELECT o.id,o.body,o.content_sha256,o.revision FROM raw.object o
        JOIN raw.source s ON s.id=o.source_id WHERE s.kind='patchnotes'",
    )
    .fetch_one(pool)
    .await
    .unwrap()
}

async fn assert_two_unchanged_runs(target: &PgPool, reader: &PgPool) {
    let before = table_counts(target).await;
    let checkpoint = patch_checkpoint(target).await;
    let receipts = patch_receipts(target).await;
    let raw = patch_raw(target).await;
    for _ in 0..2 {
        let counts = scan_once(target, reader).await;
        assert_eq!(counts.raw_inserted, 0);
        assert_eq!(counts.raw_reused, 1);
        assert_eq!(counts.changes_inserted, 0);
        assert_eq!(table_counts(target).await, before);
        assert_eq!(patch_checkpoint(target).await, checkpoint);
        assert_eq!(patch_receipts(target).await, receipts);
        assert_eq!(patch_raw(target).await, raw);
    }
}

#[tokio::test]
async fn published_catalogue_reextracts_unbound_baba_from_the_same_raw_object() {
    let scratch = Scratch::new();
    let (target, source) = scratch.prepare().await;
    let content = "Baba\n- Base Health increased from 780 to 800";
    insert_post(&source, 1, Some("2026-10-12T12:00:00Z"), Some(content)).await;
    let reader = scratch.reader().await;
    let first = scan_once(&target, &reader).await;
    assert_eq!(first.raw_inserted, 1);
    assert_eq!(first.changes_inserted, 0);
    assert!(first.changes_unresolved > 0);
    let raw = patch_raw(&target).await;
    assert_eq!(raw.2, Sha256::digest(&raw.1).to_vec());
    let envelope: Value = serde_json::from_slice(&raw.1).unwrap();
    assert_eq!(envelope["raw_content"], content);
    let receipts = patch_receipts(&target).await;
    let initial_checkpoint = patch_checkpoint(&target).await;
    assert_two_unchanged_runs(&target, &reader).await;
    let (gamefiles, run, game_counts) = stage_catalog(
        &target,
        &"a".repeat(40),
        br#"{"hero_baba":{"MaxHealth":780.0}}"#,
        None,
    )
    .await;
    gamefiles.publish(&run).await.unwrap();
    gamefiles.finish(&game_counts, false).await.unwrap();
    let unnamed = scan_once(&target, &reader).await;
    assert_eq!(unnamed.raw_inserted, 0);
    assert_eq!(unnamed.raw_reused, 1);
    assert_eq!(unnamed.changes_inserted, 0);
    assert!(unnamed.changes_unresolved > 0);
    assert_eq!(patch_raw(&target).await, raw);
    let checkpoint = patch_checkpoint(&target).await;
    assert_ne!(checkpoint.0, initial_checkpoint.0);
    assert_eq!(checkpoint.2, initial_checkpoint.2 + 1);
    assert_two_unchanged_runs(&target, &reader).await;
    let (gamefiles, run, game_counts) = stage_catalog(
        &target,
        &"b".repeat(40),
        br#"{"hero_baba":{"Name":"Baba","MaxHealth":780.0}}"#,
        None,
    )
    .await;
    assert_two_unchanged_runs(&target, &reader).await;
    gamefiles.publish(&run).await.unwrap();
    gamefiles.finish(&game_counts, false).await.unwrap();
    let bound = scan_once(&target, &reader).await;
    assert_eq!(bound.raw_inserted, 0);
    assert_eq!(bound.raw_reused, 1);
    assert_eq!(bound.changes_inserted, 1);
    assert!(!bound.partial());
    assert_eq!(patch_raw(&target).await, raw);
    let current: (String, String, i64, Vec<u8>) = sqlx::query_as(
        "SELECT e.key,change.line_text,proof.raw_object_id,proof.value_sha256
        FROM serve.published_patch_change change JOIN canon.entity e ON e.id=change.entity_id
        JOIN canon.receipt proof ON proof.id=change.receipt_id",
    )
    .fetch_one(&target)
    .await
    .unwrap();
    assert_eq!(current.0, "hero_baba");
    assert_eq!(current.1, "- Base Health increased from 780 to 800");
    assert_eq!(current.2, raw.0);
    let lexical: Vec<u8> = sqlx::query_scalar(
        "SELECT sha256(convert_to((convert_from(body,'UTF8')::json->'raw_content')::text,'UTF8'))
        FROM raw.object WHERE id=$1",
    )
    .bind(raw.0)
    .fetch_one(&target)
    .await
    .unwrap();
    assert_eq!(current.3, lexical);
    let refreshed = patch_checkpoint(&target).await;
    assert_ne!(refreshed.0, checkpoint.0);
    assert_eq!(refreshed.2, checkpoint.2 + 1);
    let all_receipts = patch_receipts(&target).await;
    assert_eq!(&all_receipts[..receipts.len()], receipts.as_slice());
    assert_two_unchanged_runs(&target, &reader).await;
    let (gamefiles, run, game_counts) = stage_catalog(
        &target,
        &"c".repeat(40),
        br#"{"hero_baba":{"MaxHealth":800.00,"Name":"Baba"}}"#,
        None,
    )
    .await;
    gamefiles.publish(&run).await.unwrap();
    gamefiles.finish(&game_counts, false).await.unwrap();
    assert_two_unchanged_runs(&target, &reader).await;
    assert_eq!(patch_checkpoint(&target).await, refreshed);
    assert_eq!(patch_raw(&target).await, raw);
    reader.close().await;
    source.close().await;
    target.close().await;
}

#[tokio::test]
async fn stat_label_refresh_reuses_raw_and_rolls_back_failed_publication() {
    let scratch = Scratch::new();
    let (target, source) = scratch.prepare().await;
    catalog(&target).await;
    insert_post(
        &source,
        1,
        Some("2026-10-12T12:00:00Z"),
        Some("Baba\n- Base Health increased from 780 to 800"),
    )
    .await;
    let reader = scratch.reader().await;
    let first = scan_once(&target, &reader).await;
    assert_eq!(first.raw_inserted, 1);
    assert_eq!(first.changes_inserted, 1);
    let stat: Option<String> =
        sqlx::query_scalar("SELECT stat_key FROM serve.published_patch_change")
            .fetch_one(&target)
            .await
            .unwrap();
    assert!(stat.is_none());
    let raw = patch_raw(&target).await;
    let receipts = patch_receipts(&target).await;
    let original_checkpoint = patch_checkpoint(&target).await;
    assert_two_unchanged_runs(&target, &reader).await;
    let owner = scratch.pool("brain_patch_target", "brain_migrate").await;
    let mut labels = owner.begin().await.unwrap();
    sqlx::query("SELECT pg_advisory_xact_lock(742110026114::bigint)")
        .execute(&mut *labels)
        .await
        .unwrap();
    sqlx::query("UPDATE canon.stat SET label_en='Base Health' WHERE key='stats.max_health'")
        .execute(&mut *labels)
        .await
        .unwrap();
    labels.commit().await.unwrap();
    let refreshed = scan_once(&target, &reader).await;
    assert_eq!(refreshed.raw_inserted, 0);
    assert_eq!(refreshed.raw_reused, 1);
    assert_eq!(refreshed.changes_inserted, 1);
    let stat: String = sqlx::query_scalar("SELECT stat_key FROM serve.published_patch_change")
        .fetch_one(&target)
        .await
        .unwrap();
    assert_eq!(stat, "stats.max_health");
    assert_eq!(patch_raw(&target).await, raw);
    let bound_checkpoint = patch_checkpoint(&target).await;
    assert_ne!(bound_checkpoint.0, original_checkpoint.0);
    let bound_receipts = patch_receipts(&target).await;
    assert_eq!(&bound_receipts[..receipts.len()], receipts.as_slice());
    assert_two_unchanged_runs(&target, &reader).await;
    let mut labels = owner.begin().await.unwrap();
    sqlx::query("SELECT pg_advisory_xact_lock(742110026114::bigint)")
        .execute(&mut *labels)
        .await
        .unwrap();
    sqlx::query("UPDATE canon.stat SET label_en='Health' WHERE key='stats.max_health'")
        .execute(&mut *labels)
        .await
        .unwrap();
    labels.commit().await.unwrap();
    sqlx::raw_sql(
        "CREATE FUNCTION ops.test_reject_catalogue_refresh() RETURNS trigger LANGUAGE plpgsql
        SET search_path=pg_catalog,pg_temp AS $fixture$
        BEGIN IF EXISTS (SELECT 1 FROM raw.source WHERE id=NEW.source_id AND kind='patchnotes') THEN
            RAISE EXCEPTION 'fixture' USING ERRCODE='23514'; END IF; RETURN NEW; END $fixture$;
        CREATE TRIGGER test_reject_catalogue_refresh BEFORE UPDATE ON canon.source_head
        FOR EACH ROW EXECUTE FUNCTION ops.test_reject_catalogue_refresh();",
    )
    .execute(&owner)
    .await
    .unwrap();
    let before = table_counts(&target).await;
    let origin: i64 = sqlx::query_scalar("SELECT receipt_id FROM canon.patch_origin")
        .fetch_one(&target)
        .await
        .unwrap();
    let store = PatchStore::start(target.clone(), 300).await.unwrap();
    let mut failed = Counts::default();
    assert!(patchnotes::scan(&store, &reader, MAX_ENVELOPE_BYTES, &mut failed)
        .await
        .is_err());
    assert_eq!(failed.objects_completed, 0);
    assert_eq!(failed.raw_inserted, 0);
    assert_eq!(failed.changes_inserted, 0);
    assert_eq!(table_counts(&target).await, before);
    assert_eq!(patch_checkpoint(&target).await, bound_checkpoint);
    assert_eq!(patch_receipts(&target).await, bound_receipts);
    assert_eq!(patch_raw(&target).await, raw);
    let unchanged: (i64, String) = sqlx::query_as(
        "SELECT origin.receipt_id,change.stat_key FROM canon.patch_origin origin
        JOIN serve.published_patch_change change ON change.patch_id=origin.patch_id",
    )
    .fetch_one(&target)
    .await
    .unwrap();
    assert_eq!(unchanged, (origin, "stats.max_health".to_owned()));
    store.finish(&failed, true).await.unwrap();
    sqlx::query("DROP TRIGGER test_reject_catalogue_refresh ON canon.source_head")
        .execute(&owner)
        .await
        .unwrap();
    let recovered = scan_once(&target, &reader).await;
    assert_eq!(recovered.raw_inserted, 0);
    assert_eq!(recovered.raw_reused, 1);
    assert_eq!(recovered.changes_inserted, 1);
    let stat: Option<String> =
        sqlx::query_scalar("SELECT stat_key FROM serve.published_patch_change")
            .fetch_one(&target)
            .await
            .unwrap();
    assert!(stat.is_none());
    assert_eq!(patch_raw(&target).await, raw);
    let recovered_receipts = patch_receipts(&target).await;
    assert_eq!(
        &recovered_receipts[..bound_receipts.len()],
        bound_receipts.as_slice()
    );
    let checkpoint = patch_checkpoint(&target).await;
    assert_ne!(checkpoint.0, bound_checkpoint.0);
    assert_eq!(checkpoint.2, bound_checkpoint.2 + 1);
    assert_two_unchanged_runs(&target, &reader).await;
    owner.close().await;
    reader.close().await;
    source.close().await;
    target.close().await;
}

#[tokio::test]
async fn published_ability_owner_scope_invalidates_the_extraction_cache() {
    let scratch = Scratch::new();
    let (target, source) = scratch.prepare().await;
    let vindicta_owner = br#"{"hero_baba":{"Name":"Baba","MaxHealth":780.0},"hero_vindicta":{"Name":"Vindicta","BoundAbilities":{"1":{"Key":"ability_brew"}}}}"#;
    let baba_owner = br#"{"hero_baba":{"Name":"Baba","MaxHealth":780.0,"BoundAbilities":{"1":{"Key":"ability_brew"}}},"hero_vindicta":{"Name":"Vindicta"}}"#;
    let abilities = br#"{"ability_brew":{"Name":"Brew","AbilityCooldown":10.0}}"#;
    let (gamefiles, run, game_counts) =
        stage_catalog(&target, &"a".repeat(40), vindicta_owner, Some(abilities)).await;
    gamefiles.publish(&run).await.unwrap();
    gamefiles.finish(&game_counts, false).await.unwrap();
    insert_post(
        &source,
        1,
        Some("2026-10-12T12:00:00Z"),
        Some("Baba\nBrew\n- Cooldown reduced from 10 to 8"),
    )
    .await;
    let reader = scratch.reader().await;
    let first = scan_once(&target, &reader).await;
    assert_eq!(first.raw_inserted, 1);
    assert_eq!(first.changes_inserted, 0);
    assert!(first.changes_unresolved > 0);
    let raw = patch_raw(&target).await;
    let receipts = patch_receipts(&target).await;
    let checkpoint = patch_checkpoint(&target).await;
    assert_two_unchanged_runs(&target, &reader).await;
    let (gamefiles, run, game_counts) =
        stage_catalog(&target, &"b".repeat(40), baba_owner, None).await;
    assert_two_unchanged_runs(&target, &reader).await;
    gamefiles.publish(&run).await.unwrap();
    gamefiles.finish(&game_counts, false).await.unwrap();
    let bound = scan_once(&target, &reader).await;
    assert_eq!(bound.raw_inserted, 0);
    assert_eq!(bound.raw_reused, 1);
    assert_eq!(bound.changes_inserted, 1);
    assert!(!bound.partial());
    let change: (String, String, String, i64) = sqlx::query_as(
        "SELECT e.key,e.kind,change.line_text,proof.raw_object_id
        FROM serve.published_patch_change change JOIN canon.entity e ON e.id=change.entity_id
        JOIN canon.receipt proof ON proof.id=change.receipt_id",
    )
    .fetch_one(&target)
    .await
    .unwrap();
    assert_eq!(change.0, "ability_brew");
    assert_eq!(change.1, "ability");
    assert_eq!(change.2, "- Cooldown reduced from 10 to 8");
    assert_eq!(change.3, raw.0);
    assert_eq!(patch_raw(&target).await, raw);
    let bound_receipts = patch_receipts(&target).await;
    assert_eq!(&bound_receipts[..receipts.len()], receipts.as_slice());
    let bound_checkpoint = patch_checkpoint(&target).await;
    assert_ne!(bound_checkpoint.0, checkpoint.0);
    assert_eq!(bound_checkpoint.2, checkpoint.2 + 1);
    assert_two_unchanged_runs(&target, &reader).await;
    let (gamefiles, run, game_counts) =
        stage_catalog(&target, &"c".repeat(40), vindicta_owner, None).await;
    assert_two_unchanged_runs(&target, &reader).await;
    gamefiles.publish(&run).await.unwrap();
    gamefiles.finish(&game_counts, false).await.unwrap();
    let unbound = scan_once(&target, &reader).await;
    assert_eq!(unbound.raw_inserted, 0);
    assert_eq!(unbound.raw_reused, 1);
    assert_eq!(unbound.changes_inserted, 0);
    assert!(unbound.changes_unresolved > 0);
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM serve.published_patch_change")
        .fetch_one(&target)
        .await
        .unwrap();
    assert_eq!(count, 0);
    let preserved: i64 = sqlx::query_scalar("SELECT count(*) FROM canon.patch_change")
        .fetch_one(&target)
        .await
        .unwrap();
    assert_eq!(preserved, 1);
    assert_eq!(patch_raw(&target).await, raw);
    let unbound_checkpoint = patch_checkpoint(&target).await;
    assert_eq!(unbound_checkpoint.0, checkpoint.0);
    assert_eq!(unbound_checkpoint.2, bound_checkpoint.2 + 1);
    let all_receipts = patch_receipts(&target).await;
    assert_eq!(&all_receipts[..bound_receipts.len()], bound_receipts.as_slice());
    assert_two_unchanged_runs(&target, &reader).await;
    reader.close().await;
    source.close().await;
    target.close().await;
}

#[tokio::test]
async fn real_sql_ingest_is_receipt_backed_idempotent_and_corrections_replace_published_claims() {
    let scratch = Scratch::new();
    let (target, source) = scratch.prepare().await;
    catalog(&target).await;
    let content = "Heroes\nBaba\n- Base Health increased from 780 to 800\n- Fixed brew interaction";
    insert_post(&source, 3, Some("2026-10-12T12:00:00Z"), Some(content)).await;
    insert_post(&source, 10, Some("2026-10-11T12:00:00Z"), Some(content)).await;
    let reader = scratch.reader().await;
    let first = PatchStore::start(target.clone(), 300).await.unwrap();
    assert!(PatchStore::start(target.clone(), 300).await.is_err());
    let mut counts = Counts::default();
    patchnotes::scan(&first, &reader, MAX_ENVELOPE_BYTES, &mut counts)
        .await
        .unwrap();
    assert_eq!(counts.raw_inserted, 2);
    assert_eq!(counts.changes_inserted, 4);
    assert!(!counts.partial());
    first.finish(&counts, false).await.unwrap();
    let latest: String = sqlx::query_scalar(
        "SELECT origin.external_id FROM serve.current_patch p JOIN canon.patch_origin origin ON origin.patch_id=p.id",
    ).fetch_one(&target).await.unwrap();
    assert_eq!(latest, "3");
    let distinct_receipts: (i64, i64) = sqlx::query_as(
        "SELECT count(*),count(DISTINCT receipt_id) FROM serve.published_patch_change",
    )
    .fetch_one(&target)
    .await
    .unwrap();
    assert_eq!(distinct_receipts, (4, 4));
    let valid_receipts: bool = sqlx::query_scalar(
        "SELECT bool_and(proof.source_id=origin.source_id AND proof.revision_id=head.revision_id
            AND proof.raw_object_id=head.raw_object_id AND proof.locator='/raw_content'
            AND proof.value_sha256=sha256(convert_to((convert_from(raw.body,'UTF8')::json->'raw_content')::text,'UTF8')))
        FROM serve.published_patch_change change JOIN canon.patch_origin origin ON origin.patch_id=change.patch_id
        JOIN canon.receipt proof ON proof.id=change.receipt_id JOIN canon.receipt head ON head.id=origin.receipt_id
        JOIN raw.object raw ON raw.id=proof.raw_object_id",
    ).fetch_one(&target).await.unwrap();
    assert!(valid_receipts);
    let bodies: Vec<Vec<u8>> =
        sqlx::query_scalar("SELECT body FROM raw.object WHERE source_id=$1 ORDER BY external_id")
            .bind(first.source)
            .fetch_all(&target)
            .await
            .unwrap();
    for body in &bodies {
        let envelope: Value = serde_json::from_slice(body).unwrap();
        assert_eq!(envelope.as_object().unwrap().len(), 5);
        assert!(envelope.get("translated_content").is_none());
        assert_eq!(envelope["raw_content"], content);
    }
    let before = table_counts(&target).await;
    let second = PatchStore::start(target.clone(), 300).await.unwrap();
    let mut unchanged = Counts::default();
    patchnotes::scan(&second, &reader, MAX_ENVELOPE_BYTES, &mut unchanged)
        .await
        .unwrap();
    assert_eq!(unchanged.raw_inserted, 0);
    assert_eq!(unchanged.changes_inserted, 0);
    assert_eq!(unchanged.raw_reused, 2);
    second.finish(&unchanged, false).await.unwrap();
    assert_eq!(table_counts(&target).await, before);
    let origin: (i64, i64) = sqlx::query_as(
        "SELECT patch_id,receipt_id FROM canon.patch_origin WHERE source_id=$1 AND external_id='3'",
    )
    .bind(first.source)
    .fetch_one(&target)
    .await
    .unwrap();
    sqlx::query("UPDATE patchnotes.changelog_posts SET raw_content=$2 WHERE id=$1")
        .bind(3_i64)
        .bind("Baba\n- Base Health increased from 780 to 810")
        .execute(&source)
        .await
        .unwrap();
    let corrected = PatchStore::start(target.clone(), 300).await.unwrap();
    let mut correction_counts = Counts::default();
    patchnotes::scan(
        &corrected,
        &reader,
        MAX_ENVELOPE_BYTES,
        &mut correction_counts,
    )
    .await
    .unwrap();
    corrected.finish(&correction_counts, false).await.unwrap();
    assert_eq!(correction_counts.raw_inserted, 1);
    assert_eq!(correction_counts.changes_inserted, 1);
    let updated: (i64, i64) = sqlx::query_as(
        "SELECT patch_id,receipt_id FROM canon.patch_origin WHERE source_id=$1 AND external_id='3'",
    )
    .bind(first.source)
    .fetch_one(&target)
    .await
    .unwrap();
    assert_eq!(updated.0, origin.0);
    assert_ne!(updated.1, origin.1);
    let lines: Vec<String> =
        sqlx::query_scalar("SELECT line_text FROM serve.published_patch_change WHERE patch_id=$1")
            .bind(origin.0)
            .fetch_all(&target)
            .await
            .unwrap();
    assert_eq!(lines, vec!["- Base Health increased from 780 to 810"]);
    sqlx::query("UPDATE patchnotes.changelog_posts SET raw_content=$2 WHERE id=$1")
        .bind(3_i64)
        .bind(content)
        .execute(&source)
        .await
        .unwrap();
    let reverted = PatchStore::start(target.clone(), 300).await.unwrap();
    let mut revert_counts = Counts::default();
    patchnotes::scan(&reverted, &reader, MAX_ENVELOPE_BYTES, &mut revert_counts)
        .await
        .unwrap();
    reverted.finish(&revert_counts, false).await.unwrap();
    assert_eq!(revert_counts.raw_inserted, 0);
    assert_eq!(revert_counts.changes_inserted, 2);
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM serve.published_patch_change WHERE patch_id=$1"
        )
        .bind(origin.0)
        .fetch_one(&target)
        .await
        .unwrap(),
        2
    );
    assert_eq!(table_counts(&target).await.2, before.2);
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM canon.source_revision_patch")
            .fetch_one(&target)
            .await
            .unwrap(),
        0
    );
    reader.close().await;
    source.close().await;
    target.close().await;
}

#[tokio::test]
async fn reader_enforces_scope_snapshot_size_and_confirmation_and_keeps_missing_metadata_raw() {
    let scratch = Scratch::new();
    let (target, source) = scratch.prepare().await;
    catalog(&target).await;
    insert_post(
        &source,
        2,
        Some("2026-10-12T12:00:00+02:00"),
        Some("Baba\n- Fixed brew"),
    )
    .await;
    insert_post(&source, 9, None, None).await;
    insert_post(
        &source,
        12,
        Some("2026-10-12T12:00:00Z"),
        Some(&"x".repeat(MAX_ENVELOPE_BYTES)),
    )
    .await;
    assert!(patchnotes::connect_source(
        "postgres://patchnotes_feed_reader@localhost/brain_patch_source"
    )
    .await
    .is_err());
    assert!(patchnotes::connect_source(
        "postgres://patchnotes_feed_reader:fixture@localhost/brain_patch_source?password=fixture"
    )
    .await
    .is_err());
    let reader = scratch.reader().await;
    let mut write_attempt = reader.begin().await.unwrap();
    assert!(sqlx::query("DELETE FROM patchnotes.changelog_posts")
        .execute(&mut *write_attempt)
        .await
        .is_err());
    write_attempt.rollback().await.unwrap();
    let mut feed = Feed::begin(&reader, MAX_ENVELOPE_BYTES).await.unwrap();
    let post = feed.next_post().await.unwrap().unwrap();
    assert_eq!(post.id, 2);
    assert!(feed.next_post().await.is_err());
    assert!(feed.confirm(3).is_err());
    feed.confirm(2).unwrap();
    insert_post(
        &source,
        15,
        Some("2026-10-12T12:00:00Z"),
        Some("Baba\n- Fixed later"),
    )
    .await;
    let missing = feed.next_post().await.unwrap().unwrap();
    assert_eq!(missing.id, 9);
    assert!(missing.bytes.is_some());
    feed.confirm(9).unwrap();
    let oversized = feed.next_post().await.unwrap().unwrap();
    assert_eq!(oversized.id, 12);
    assert!(oversized.bytes.is_none());
    feed.confirm(12).unwrap();
    assert!(feed.next_post().await.unwrap().is_none());
    feed.finish().await.unwrap();
    let store = PatchStore::start(target.clone(), 300).await.unwrap();
    let mut counts = Counts::default();
    patchnotes::scan(&store, &reader, MAX_ENVELOPE_BYTES, &mut counts)
        .await
        .unwrap();
    assert_eq!(counts.raw_inserted, 3);
    assert_eq!(counts.objects_oversized, 1);
    assert_eq!(counts.metadata_missing, 1);
    assert!(counts.partial());
    store.finish(&counts, false).await.unwrap();
    let status: (String, String, Value) =
        sqlx::query_as("SELECT status,error_code,counts FROM ops.job_run WHERE id=$1")
            .bind(store.job)
            .fetch_one(&target)
            .await
            .unwrap();
    assert_eq!(status.0, "partial");
    assert_eq!(status.1, "patch_data_gaps");
    assert_eq!(status.2["objects_oversized"], 1);
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM canon.patch_origin WHERE external_id IN ('9','12')"
        )
        .fetch_one(&target)
        .await
        .unwrap(),
        0
    );
    sqlx::query("GRANT UPDATE ON patchnotes.changelog_posts TO patchnotes_feed_reader")
        .execute(&source)
        .await
        .unwrap();
    assert!(Feed::begin(&reader, MAX_ENVELOPE_BYTES).await.is_err());
    reader.close().await;
    source.close().await;
    target.close().await;
}

#[tokio::test]
async fn mismatch_rollback_safe_retries_and_fencing_never_publish_failure() {
    let scratch = Scratch::new();
    let (target, source) = scratch.prepare().await;
    catalog(&target).await;
    insert_post(
        &source,
        1,
        Some("2026-10-12T12:00:00Z"),
        Some("Baba\n- Fixed brew"),
    )
    .await;
    let reader = scratch.reader().await;
    let mut feed = Feed::begin(&reader, MAX_ENVELOPE_BYTES).await.unwrap();
    let mut post = feed.next_post().await.unwrap().unwrap();
    let store = PatchStore::start(target.clone(), 300).await.unwrap();
    let mut counts = Counts::default();
    let before = table_counts(&target).await;
    post.id = 2;
    assert!(store.object(&post, &mut counts).await.is_err());
    assert_eq!(table_counts(&target).await, before);
    post.id = 1;
    let owner = scratch.pool("brain_patch_target", "brain_migrate").await;
    sqlx::raw_sql(
        "CREATE FUNCTION ops.test_corrupt_patch() RETURNS trigger LANGUAGE plpgsql
        SET search_path=pg_catalog,pg_temp AS $fixture$
        BEGIN NEW.body := convert_to('{}','UTF8'); RETURN NEW; END $fixture$;
        CREATE TRIGGER test_corrupt_patch BEFORE INSERT ON raw.object FOR EACH ROW EXECUTE FUNCTION ops.test_corrupt_patch();",
    ).execute(&owner).await.unwrap();
    assert!(store.object(&post, &mut counts).await.is_err());
    assert_eq!(table_counts(&target).await, before);
    assert_eq!(counts.objects_completed, 0);
    sqlx::raw_sql("DROP TRIGGER test_corrupt_patch ON raw.object;
        CREATE SEQUENCE ops.test_patch_attempts;
        GRANT USAGE,SELECT ON SEQUENCE ops.test_patch_attempts TO brain_ingest;
        CREATE FUNCTION ops.test_retry_patch() RETURNS trigger LANGUAGE plpgsql
        SET search_path=pg_catalog,pg_temp AS $fixture$
        BEGIN IF nextval('ops.test_patch_attempts')=1 THEN
            RAISE EXCEPTION 'fixture' USING ERRCODE=TG_ARGV[0]; END IF; RETURN NEW; END $fixture$;
        CREATE TRIGGER test_retry_patch BEFORE INSERT ON raw.run_object FOR EACH ROW EXECUTE FUNCTION ops.test_retry_patch('40001');")
        .execute(&owner).await.unwrap();
    store.object(&post, &mut counts).await.unwrap();
    assert_eq!(counts.raw_inserted, 1);
    assert_eq!(counts.changes_inserted, 1);
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT last_value FROM ops.test_patch_attempts")
            .fetch_one(&target)
            .await
            .unwrap(),
        2
    );
    sqlx::query("DROP TRIGGER test_retry_patch ON raw.run_object")
        .execute(&owner)
        .await
        .unwrap();
    sqlx::query(
        "UPDATE ops.job_lease SET epoch=epoch+1 WHERE lease_key='brain-job:ingest-patchnotes'",
    )
    .execute(&target)
    .await
    .unwrap();
    assert!(store.object(&post, &mut counts).await.is_err());
    assert!(store.finish(&counts, false).await.is_err());
    assert_eq!(
        sqlx::query_scalar::<_, String>("SELECT status FROM ops.job_run WHERE id=$1")
            .bind(store.job)
            .fetch_one(&target)
            .await
            .unwrap(),
        "running"
    );
    feed.confirm(1).unwrap();
    assert!(feed.next_post().await.unwrap().is_none());
    feed.finish().await.unwrap();
    owner.close().await;
    reader.close().await;
    source.close().await;
    target.close().await;
}

#[tokio::test]
async fn oversized_and_metadata_corrections_withdraw_prior_claims_without_fabricated_raw() {
    let scratch = Scratch::new();
    let (target, source) = scratch.prepare().await;
    catalog(&target).await;
    insert_post(
        &source,
        1,
        Some("2026-10-12T12:00:00Z"),
        Some("Baba\n- Fixed brew"),
    )
    .await;
    let reader = scratch.reader().await;
    for index in 0..3 {
        let store = PatchStore::start(target.clone(), 300).await.unwrap();
        let mut counts = Counts::default();
        patchnotes::scan(&store, &reader, MAX_ENVELOPE_BYTES, &mut counts)
            .await
            .unwrap();
        store.finish(&counts, false).await.unwrap();
        let visible: i64 = sqlx::query_scalar("SELECT count(*) FROM serve.published_patch_change")
            .fetch_one(&target)
            .await
            .unwrap();
        assert_eq!(visible, if index == 0 { 1 } else { 0 });
        if index == 0 {
            sqlx::query("UPDATE patchnotes.changelog_posts SET posted_at=NULL WHERE id=1")
                .execute(&source)
                .await
                .unwrap();
        } else if index == 1 {
            sqlx::query("UPDATE patchnotes.changelog_posts SET raw_content=$1 WHERE id=1")
                .bind("x".repeat(MAX_ENVELOPE_BYTES))
                .execute(&source)
                .await
                .unwrap();
        } else {
            assert_eq!(counts.raw_inserted, 0);
            assert_eq!(counts.objects_oversized, 1);
        }
    }
    assert_eq!(sqlx::query_scalar::<_, i64>("SELECT count(*) FROM raw.object WHERE source_id=(SELECT id FROM raw.source WHERE kind='patchnotes')")
        .fetch_one(&target).await.unwrap(), 2);
    reader.close().await;
    source.close().await;
    target.close().await;
}
