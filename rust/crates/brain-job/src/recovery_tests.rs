use super::ScratchPg;

#[path = "recovery_bounds_tests.rs"]
mod bounds;
#[path = "recovery_identity_tests.rs"]
mod identity;
use crate::{
    extract,
    git::{Repository, Source},
    ingest_source,
    store::{Counts, Store},
    InvalidTarget,
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use std::{
    process::{Child, Command, Stdio},
    time::Duration,
};

struct Upstream {
    root: tempfile::TempDir,
    daemon: Child,
    source: Source,
}

impl Upstream {
    async fn start(source: Source) -> Self {
        let root = tempfile::Builder::new()
            .prefix("brain-job-recovery-upstream-")
            .tempdir()
            .unwrap();
        let output = Command::new("/usr/bin/git")
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .args(["init", "--bare"])
            .arg(root.path().join("upstream.git"))
            .output()
            .unwrap();
        assert!(output.status.success());
        std::fs::create_dir(root.path().join("files")).unwrap();
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        drop(listener);
        let daemon = Command::new("/usr/lib/git-core/git-daemon")
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .args(["--export-all", "--reuseaddr", "--listen=127.0.0.1"])
            .arg(format!("--port={}", address.port()))
            .arg(format!("--base-path={}", root.path().display()))
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let mut upstream = Self {
            root,
            daemon,
            source: Source {
                key: source.key,
                url: Box::leak(format!("git://{address}/upstream.git").into_boxed_str()),
            },
        };
        tokio::time::timeout(Duration::from_secs(8), async {
            loop {
                assert!(upstream.daemon.try_wait().unwrap().is_none());
                if tokio::net::TcpStream::connect(address).await.is_ok() {
                    break;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
        upstream
    }

    fn git(&self, arguments: &[&str]) -> String {
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
            .arg(self.root.path().join("upstream.git"))
            .arg("--work-tree")
            .arg(self.root.path().join("files"))
            .args(arguments)
            .output()
            .unwrap();
        assert!(output.status.success());
        String::from_utf8(output.stdout).unwrap().trim().to_owned()
    }

    fn file(&self, path: &str, bytes: Option<&[u8]>) {
        let path = self.root.path().join("files").join(path);
        if let Some(bytes) = bytes {
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, bytes).unwrap();
        } else {
            std::fs::remove_file(path).unwrap();
        }
    }

    fn commit(&self, parent: Option<&str>) -> String {
        self.git(&["add", "-A"]);
        let tree = self.git(&["write-tree"]);
        let mut arguments = vec!["commit-tree", &tree, "-m", "fixture"];
        if let Some(parent) = parent {
            arguments.extend(["-p", parent]);
        }
        let revision = self.git(&arguments);
        self.git(&["update-ref", "refs/heads/master", &revision]);
        self.git(&["symbolic-ref", "HEAD", "refs/heads/master"]);
        revision
    }
}

impl Drop for Upstream {
    fn drop(&mut self) {
        let _ = self.daemon.kill();
        self.daemon.wait().unwrap();
    }
}

async fn database(pg: &ScratchPg) -> (PgPool, PgPool) {
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
    (admin, pg.pool("brain_ingest").await)
}

fn canonical_source(source: Source) -> Source {
    Source::ALL
        .into_iter()
        .find(|candidate| candidate.key == source.key)
        .unwrap()
}

async fn source_ingest(
    store: &Store,
    repository: &Repository,
    source: Source,
    counts: &mut Counts,
) -> anyhow::Result<()> {
    ingest_source(
        store,
        repository,
        canonical_source(source),
        1024,
        counts,
        repository.fetch(source),
    )
    .await
}

async fn snapshot(pool: &PgPool, table: &str, maximum: i64) -> Value {
    sqlx::query_scalar(&format!(
        "SELECT coalesce(jsonb_agg(to_jsonb(a) ORDER BY id),'[]') FROM {table} a WHERE id<=$1"
    ))
    .bind(maximum)
    .fetch_one(pool)
    .await
    .unwrap()
}

async fn current(pool: &PgPool) -> Value {
    sqlx::query_scalar(
        "SELECT coalesce(jsonb_agg(jsonb_build_object('entity',e.key,'path',f.source_path,
        'stat',f.stat_key,'condition',f.condition,'number',f.value_num,'text',f.value_text)
        ORDER BY e.key,f.source_path,f.stat_key,f.condition),'[]')
        FROM serve.current_fact f JOIN canon.entity e ON e.id=f.entity_id",
    )
    .fetch_one(pool)
    .await
    .unwrap()
}

async fn rejects(pool: &PgPool, statement: &str) {
    let error = sqlx::query(statement).execute(pool).await.unwrap_err();
    assert_eq!(
        error.as_database_error().unwrap().code().as_deref(),
        Some("P0001")
    );
}

async fn verify_proofs(pool: &PgPool, source: Source) {
    let identities: Vec<(String, String)> = sqlx::query_as("SELECT key,kind FROM canon.entity")
        .fetch_all(pool)
        .await
        .unwrap();
    let identities = identities
        .into_iter()
        .map(|(key, kind)| {
            (
                key,
                match kind.as_str() {
                    "hero" => "hero",
                    "ability" => "ability",
                    _ => panic!("unexpected identity"),
                },
            )
        })
        .collect();
    let objects: Vec<(i64, String, Vec<u8>, Vec<u8>)> =
        sqlx::query_as("SELECT id,external_id,body,content_sha256 FROM raw.object")
            .fetch_all(pool)
            .await
            .unwrap();
    for (id, path, bytes, hash) in objects {
        assert_eq!(hash, Sha256::digest(&bytes).to_vec());
        let assertions =
            extract::extract_with_identities(source, &path, &bytes, &identities).unwrap();
        extract::prove(source, &path, &bytes, &assertions).unwrap();
        let receipts: Vec<(String, Vec<u8>)> =
            sqlx::query_as("SELECT locator,value_sha256 FROM canon.receipt WHERE raw_object_id=$1")
                .bind(id)
                .fetch_all(pool)
                .await
                .unwrap();
        for (locator, hash) in receipts {
            assert!(assertions
                .iter()
                .any(|assertion| assertion.locator == locator && assertion.hash == hash));
        }
    }
}

async fn recovery(source: Source, oversized: bool, introduced: bool) {
    let pg = ScratchPg::start();
    let (admin, pool) = database(&pg).await;
    let upstream = Upstream::start(source).await;
    let source = upstream.source;
    let state = tempfile::tempdir().unwrap();
    let repository = Repository::open(state.path(), source, 30).await.unwrap();
    let (hero_path, ability_path, hero, ability, bad_ability) = if source.key == "deadlock-data" {
        (
            "data/json/hero-data.json",
            "data/json/ability-data.json",
            br#"{"hero_old":{"MaxHealth":780.0}}"#.as_slice(),
            br#"{"ability_old":{"AbilityCooldown":26.0}}"#.as_slice(),
            br#"{"ability_new":{"AbilityCooldown":99.0}}"#.as_slice(),
        )
    } else {
        ("game/citadel/pak01_dir/scripts/heroes.vdata", "game/citadel/pak01_dir/scripts/abilities.vdata",
         br#"hero_old { m_mapStartingStats { EMaxHealth "780.0" } }"#.as_slice(),
         br#"ability_old { m_eAbilityType "EAbilityType_Signature" m_mapAbilityProperties { AbilityCooldown { m_strValue "26.0" } } }"#.as_slice(),
         br#"ability_new { m_eAbilityType "EAbilityType_Signature" m_mapAbilityProperties { AbilityCooldown { m_strValue "99.0" } } }"#.as_slice())
    };
    upstream.file(hero_path, Some(hero));
    if !introduced {
        upstream.file(ability_path, Some(ability));
    }
    if source.key == "deadlock-data" {
        upstream.file(crate::LOCALIZATION_PATHS[0], Some(br#"{"hero_old":"Old Hero","ability_old":"Old Ability","ability_new":"New Ability"}"#));
        upstream.file(crate::LOCALIZATION_PATHS[1], Some(r#"{"hero_old":"Alte Heldin","ability_old":"Alte Fähigkeit","ability_new":"Neue Fähigkeit"}"#.as_bytes()));
    }
    let initial = upstream.commit(None);
    let mut store = Store::start(pool.clone(), 300).await.unwrap();
    let source_id = store.source(canonical_source(source)).await.unwrap();
    let mut counts = Counts::default();
    source_ingest(&store, &repository, source, &mut counts)
        .await
        .unwrap();
    assert_eq!(counts.sources_completed, 1);
    let expected = current(&pool).await;
    let aliases = super::localized_aliases(&pool).await;
    store.finish(&counts, false).await.unwrap();
    upstream.file(ability_path, Some(bad_ability));
    let oversized_bytes = vec![b'x'; 2048];
    upstream.file(
        hero_path,
        Some(if oversized { &oversized_bytes } else { b"{" }),
    );
    let bad = upstream.commit(Some(&initial));
    store = Store::start(pool.clone(), 300).await.unwrap();
    counts = Counts::default();
    let error = source_ingest(&store, &repository, source, &mut counts)
        .await
        .unwrap_err();
    assert!(error.downcast_ref::<InvalidTarget>().is_some());
    assert_eq!(counts.raw_inserted, 1);
    assert_eq!(counts.objects_completed, 1);
    assert_eq!(current(&pool).await, expected);
    store.finish(&counts, true).await.unwrap();
    let failed_run: i64 =
        sqlx::query_scalar("SELECT id FROM raw.run WHERE source_id=$1 AND target_revision=$2")
            .bind(source_id)
            .bind(&bad)
            .fetch_one(&pool)
            .await
            .unwrap();
    let revision: Value =
        sqlx::query_scalar("SELECT to_jsonb(v) FROM canon.revision v WHERE raw_run_id=$1")
            .bind(failed_run)
            .fetch_one(&pool)
            .await
            .unwrap();
    let objects: Value = sqlx::query_scalar(
        "SELECT jsonb_agg(to_jsonb(o) ORDER BY path) FROM raw.run_object o WHERE run_id=$1",
    )
    .bind(failed_run)
    .fetch_one(&pool)
    .await
    .unwrap();
    let max_raw: i64 = sqlx::query_scalar("SELECT max(id) FROM raw.object")
        .fetch_one(&pool)
        .await
        .unwrap();
    let max_receipt: i64 = sqlx::query_scalar("SELECT max(id) FROM canon.receipt")
        .fetch_one(&pool)
        .await
        .unwrap();
    let raw = snapshot(&pool, "raw.object", max_raw).await;
    let receipts = snapshot(&pool, "canon.receipt", max_receipt).await;
    let entities: Value =
        sqlx::query_scalar("SELECT jsonb_agg(to_jsonb(e) ORDER BY id) FROM canon.entity e")
            .fetch_one(&pool)
            .await
            .unwrap();
    store = Store::start(pool.clone(), 300).await.unwrap();
    counts = Counts::default();
    assert!(source_ingest(&store, &repository, source, &mut counts)
        .await
        .is_err());
    assert_eq!(counts.objects_completed, 0);
    let resumed: i64 =
        sqlx::query_scalar("SELECT id FROM raw.run WHERE source_id=$1 AND status='running'")
            .bind(source_id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(resumed, failed_run);
    store.finish(&counts, true).await.unwrap();
    rejects(
        &pool,
        &format!("UPDATE raw.run SET status='failed' WHERE id={failed_run}"),
    )
    .await;
    upstream.file(hero_path, Some(hero));
    upstream.file(ability_path, if introduced { None } else { Some(ability) });
    let corrected = upstream.commit(Some(&bad));
    store = Store::start(pool.clone(), 300).await.unwrap();
    counts = Counts::default();
    source_ingest(&store, &repository, source, &mut counts)
        .await
        .unwrap();
    assert_eq!(counts.sources_completed, 1);
    assert_eq!(counts.raw_inserted, 0);
    assert_eq!(counts.paths_deleted, u64::from(introduced));
    assert_eq!(current(&pool).await, expected);
    assert_eq!(super::localized_aliases(&pool).await, aliases);
    for table in ["fact", "alias", "relation"] {
        let leaked: i64 = sqlx::query_scalar(&format!("SELECT count(*) FROM serve.current_{table} a JOIN canon.receipt p ON p.id=a.receipt_id JOIN canon.revision v ON v.id=p.revision_id JOIN raw.run r ON r.id=v.raw_run_id WHERE r.status<>'succeeded'"))
            .fetch_one(&pool).await.unwrap();
        assert_eq!(leaked, 0);
    }
    let status: String = sqlx::query_scalar("SELECT status FROM raw.run WHERE id=$1")
        .bind(failed_run)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(status, "failed");
    for status in ["running", "partial", "succeeded", "failed"] {
        rejects(
            &pool,
            &format!("UPDATE raw.run SET status='{status}' WHERE id={failed_run}"),
        )
        .await;
    }
    rejects(
        &pool,
        &format!("UPDATE raw.run_object SET parse_status='pending' WHERE run_id={failed_run}"),
    )
    .await;
    rejects(&pool, &format!("INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version) SELECT raw_object_id,source_id,revision_id,locator_kind,'late',value_sha256,extractor,extractor_version FROM canon.receipt WHERE revision_id={}", revision["id"].as_i64().unwrap())).await;
    assert_eq!(snapshot(&pool, "raw.object", max_raw).await, raw);
    assert_eq!(
        snapshot(&pool, "canon.receipt", max_receipt).await,
        receipts
    );
    let revision_after: Value =
        sqlx::query_scalar("SELECT to_jsonb(v) FROM canon.revision v WHERE raw_run_id=$1")
            .bind(failed_run)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(revision_after, revision);
    let objects_after: Value = sqlx::query_scalar(
        "SELECT jsonb_agg(to_jsonb(o) ORDER BY path) FROM raw.run_object o WHERE run_id=$1",
    )
    .bind(failed_run)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(objects_after, objects);
    let entities_after: Value =
        sqlx::query_scalar("SELECT jsonb_agg(to_jsonb(e) ORDER BY id) FROM canon.entity e")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(entities_after, entities);
    verify_proofs(&pool, source).await;
    println!(
        "recovered {} oversized={oversized} introduced={introduced}: {}",
        source.key,
        counts.value()
    );
    store.finish(&counts, false).await.unwrap();
    let raw_count: i64 = sqlx::query_scalar("SELECT count(*) FROM raw.object")
        .fetch_one(&pool)
        .await
        .unwrap();
    let followup = upstream.commit(Some(&corrected));
    assert_ne!(followup, corrected);
    store = Store::start(pool.clone(), 300).await.unwrap();
    counts = Counts::default();
    source_ingest(&store, &repository, source, &mut counts)
        .await
        .unwrap();
    assert_eq!(counts.objects_completed, 0);
    assert_eq!(counts.raw_inserted, 0);
    assert_eq!(counts.raw_reused, 0);
    source_ingest(&store, &repository, source, &mut counts)
        .await
        .unwrap();
    assert_eq!(counts.sources_unchanged, 1);
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM raw.object")
            .fetch_one(&pool)
            .await
            .unwrap(),
        raw_count
    );
    println!("unchanged {}: {}", source.key, counts.value());
    store.finish(&counts, false).await.unwrap();
    pool.close().await;
    admin.close().await;
}

#[tokio::test]
async fn wiki_malformed_target_recovers_reverted_and_deleted_partial_assertions() {
    for introduced in [false, true] {
        recovery(Source::ALL[0], false, introduced).await;
    }
}

#[tokio::test]
async fn wiki_oversized_target_recovers_reverted_and_deleted_partial_assertions() {
    for introduced in [false, true] {
        recovery(Source::ALL[0], true, introduced).await;
    }
}

#[tokio::test]
async fn gametracking_malformed_target_recovers_reverted_and_deleted_partial_assertions() {
    for introduced in [false, true] {
        recovery(Source::ALL[1], false, introduced).await;
    }
}

#[tokio::test]
async fn gametracking_oversized_target_recovers_reverted_and_deleted_partial_assertions() {
    for introduced in [false, true] {
        recovery(Source::ALL[1], true, introduced).await;
    }
}
