use brain_job::{
    extract,
    git::{Blob, Source},
    store::{Counts, Store},
};
use sha2::{Digest, Sha256};
use sqlx::{postgres::PgConnectOptions, PgPool};
use std::process::Command;

struct Scratch {
    root: tempfile::TempDir,
}

impl Scratch {
    fn start() -> Self {
        let root = tempfile::Builder::new()
            .prefix("brain-p3-backfill-")
            .tempdir()
            .unwrap();
        std::fs::create_dir(root.path().join("socket")).unwrap();
        let scratch = Self { root };
        for (program, arguments) in [
            (
                "initdb",
                vec![
                    "-D".into(),
                    scratch.root.path().join("data").into_os_string(),
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
                    scratch.root.path().join("data").into_os_string(),
                    "-l".into(),
                    scratch.root.path().join("postgres.log").into_os_string(),
                    "-o".into(),
                    format!(
                        "-k {} -p 55468 -c listen_addresses=''",
                        scratch.root.path().join("socket").display()
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
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        scratch
    }

    async fn pool(&self, user: &str) -> PgPool {
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect_with(
                PgConnectOptions::new_without_pgpass()
                    .host(self.root.path().join("socket").to_str().unwrap())
                    .port(55468)
                    .username(user)
                    .database("postgres"),
            )
            .await
            .unwrap()
    }

    async fn migrate(&self) -> PgPool {
        let admin = self.pool("brain_test_dba").await;
        sqlx::raw_sql("CREATE ROLE brain_migrate LOGIN; CREATE ROLE brain_ingest LOGIN; CREATE ROLE brain_service LOGIN; CREATE ROLE brain_tool NOLOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate;")
            .execute(&admin).await.unwrap();
        let owner = self.pool("brain_migrate").await;
        for (version, sql) in [
            (
                1,
                include_str!("../../../../scripts/migrations/2026-10-12-brain-gamefacts-v1.sql"),
            ),
            (
                2,
                include_str!(
                    "../../../../scripts/migrations/2026-10-12-brain-gamefacts-p3-timers-v1.sql"
                ),
            ),
        ] {
            sqlx::raw_sql(sql).execute(&owner).await.unwrap();
            sqlx::query("INSERT INTO ops.gamefacts_schema_version(version) VALUES($1)")
                .bind(version)
                .execute(&owner)
                .await
                .unwrap();
        }
        owner.close().await;
        admin
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let output = Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .arg("-D")
            .arg(self.root.path().join("data"))
            .args(["-m", "immediate", "-w", "stop"])
            .output()
            .unwrap();
        assert!(output.status.success());
    }
}

fn blob(bytes: &[u8]) -> Blob {
    Blob {
        bytes: bytes.to_vec(),
        hash: Sha256::digest(bytes).to_vec(),
    }
}

async fn totals(pool: &PgPool) -> (i64, i64, i64) {
    sqlx::query_as("SELECT (SELECT count(*) FROM raw.object),(SELECT count(*) FROM canon.fact),(SELECT count(*) FROM canon.receipt)")
        .fetch_one(pool).await.unwrap()
}

fn cgroup_memory_proof() {
    let groups = std::fs::read_to_string("/proc/self/cgroup").unwrap();
    let memory = groups.lines().find_map(|line| {
        let mut parts = line.splitn(3, ':');
        parts.next()?;
        let controllers = parts.next()?;
        let path = parts.next()?;
        controllers
            .split(',')
            .any(|name| name == "memory")
            .then_some(path)
    });
    let (path, root, limit_name, peak_name) = if let Some(path) = memory {
        (
            path,
            format!("/sys/fs/cgroup/memory{path}"),
            "memory.limit_in_bytes",
            "memory.max_usage_in_bytes",
        )
    } else {
        let path = groups
            .lines()
            .find_map(|line| line.strip_prefix("0::"))
            .unwrap();
        (
            path,
            format!("/sys/fs/cgroup{path}"),
            "memory.max",
            "memory.peak",
        )
    };
    let limit = std::fs::read_to_string(format!("{root}/{limit_name}")).unwrap();
    let peak = std::fs::read_to_string(format!("{root}/{peak_name}")).unwrap();
    let scoped = path.contains("brain-p3-public-store-proof-");
    if scoped {
        assert_eq!(limit.trim().parse::<u64>().unwrap(), 536_870_912);
        assert!((1..300_000_000).contains(&peak.trim().parse::<u64>().unwrap()));
    }
    println!(
        "P3_MEMORY_PROOF scoped={scoped} limit_bytes={} peak_bytes={}",
        limit.trim(),
        peak.trim()
    );
}

#[derive(sqlx::FromRow)]
struct StoredReceipt {
    stat_key: String,
    locator: String,
    value_sha256: Vec<u8>,
    body: Vec<u8>,
    extractor_version: String,
}

const OLD_P1: &str = "BEGIN;
    INSERT INTO ops.job_run(job,extractor,extractor_version) VALUES ('ingest-gamefiles','hero-ability-atoms','p1-v1');
    INSERT INTO raw.source(kind,locator) VALUES ('git','synthetic://old-p1');
    INSERT INTO raw.run(source_id,job_run_id,target_revision,extractor,extractor_version) VALUES (1,1,repeat('a',40),'hero-ability-atoms','p1-v1');
    INSERT INTO canon.revision(source_id,raw_run_id,source_revision,extractor,extractor_version) VALUES (1,1,repeat('a',40),'hero-ability-atoms','p1-v1');
    INSERT INTO raw.object(source_id,external_id,revision,content_sha256,media_type,body)
        SELECT 1,path,repeat('a',40),sha256(convert_to(body,'UTF8')),'application/json',convert_to(body,'UTF8') FROM (VALUES
        ('data/json/hero-data.json','{\"hero_baba\":{\"Name\":\"Baba\",\"MaxHealth\":780.0}}'),
        ('data/json/misc-data.json','{\"citadel_breakable_prop_drop_gold\":{\"InitialSpawnTime\":180.0,\"RespawnTime\":180.0}}')) f(path,body);
    INSERT INTO raw.run_object(run_id,path,object_id,action,parse_status) VALUES
        (1,'data/json/hero-data.json',1,'upsert','succeeded'),(1,'data/json/misc-data.json',2,'upsert','skipped');
    INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version)
        SELECT 1,1,1,'json_pointer',pointer,sha256(convert_to(value,'UTF8')),'hero-ability-atoms','p1-v1' FROM (VALUES
        ('/hero_baba/Name','\"Baba\"'),('/hero_baba/MaxHealth','780.0')) f(pointer,value);
    INSERT INTO canon.entity(kind,key,identity_receipt_id) VALUES ('hero','hero_baba',1);
    INSERT INTO canon.stat(key,section,unit,value_type) VALUES ('identity.name','identity',NULL,'text'),('stats.max_health','stats','health','number');
    INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,value_text,receipt_id,valid_from_revision_id)
        VALUES (1,'identity.name',1,'data/json/hero-data.json','Baba',1,1);
    INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,value_num,receipt_id,valid_from_revision_id)
        VALUES (1,'stats.max_health',1,'data/json/hero-data.json',780.0,2,1);
    UPDATE raw.run SET status='succeeded' WHERE id=1;
    INSERT INTO canon.source_head(source_id,published_revision_id) VALUES (1,1);
    UPDATE raw.source SET last_complete_revision=repeat('a',40) WHERE id=1;
    COMMIT;";

#[tokio::test]
async fn version_backfill_reuses_raw_objects_preserves_p1_and_second_run_writes_nothing() {
    let scratch = Scratch::start();
    let admin = scratch.migrate().await;
    let pool = scratch.pool("brain_ingest").await;
    let source = Source::ALL[0];
    let hero_path = "data/json/hero-data.json";
    let timer_path = "data/json/misc-data.json";
    let hero = br#"{"hero_baba":{"Name":"Baba","MaxHealth":780.0}}"#;
    let timer =
        br#"{"citadel_breakable_prop_drop_gold":{"InitialSpawnTime":180.0,"RespawnTime":180.0}}"#;
    let target = "a".repeat(40);
    sqlx::raw_sql(OLD_P1).execute(&admin).await.unwrap();
    let source_id = 1;
    let baseline = totals(&pool).await;
    assert_eq!(baseline.0, 2);
    let store = Store::start(pool.clone(), 300).await.unwrap();
    let backfill = store.begin_run(source_id, &target).await.unwrap().unwrap();
    assert!(backfill.full);
    let mut counts = Counts::default();
    for (path, bytes) in [(hero_path, hero.as_slice()), (timer_path, timer.as_slice())] {
        store
            .object(&backfill, source, path, Some(blob(bytes)), &mut counts)
            .await
            .unwrap();
    }
    assert_eq!(counts.raw_inserted, 0);
    assert_eq!(counts.raw_reused, 2);
    store.publish(&backfill).await.unwrap();
    let p1_current: i64 = sqlx::query_scalar("SELECT count(*) FROM serve.current_fact f JOIN canon.entity e ON e.id=f.entity_id WHERE e.kind='hero'")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(p1_current, baseline.1);
    let duplicates: i64 = sqlx::query_scalar("SELECT count(*) FROM (SELECT entity_id,stat_key,source_id,source_path,condition FROM serve.current_fact GROUP BY entity_id,stat_key,source_id,source_path,condition HAVING count(*)>1) d")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(duplicates, 0);
    let facts: Vec<StoredReceipt> = sqlx::query_as("SELECT f.stat_key,r.locator,r.value_sha256,o.body,r.extractor_version FROM serve.timer f JOIN canon.receipt r ON r.id=f.receipt_id JOIN raw.object o ON o.id=r.raw_object_id WHERE f.key='soul_box' ORDER BY f.stat_key")
        .fetch_all(&pool).await.unwrap();
    assert_eq!(facts.len(), 2);
    for fact in facts {
        assert_eq!(fact.body, timer);
        assert_eq!(fact.value_sha256, Sha256::digest(b"180.0").to_vec());
        assert_eq!(fact.extractor_version, extract::VERSION);
        assert_eq!(
            fact.locator,
            if fact.stat_key == "timer.first_spawn_seconds" {
                "/citadel_breakable_prop_drop_gold/InitialSpawnTime"
            } else {
                "/citadel_breakable_prop_drop_gold/RespawnTime"
            }
        );
    }
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM serve.timer WHERE key='urn'")
            .fetch_one(&pool)
            .await
            .unwrap(),
        0
    );
    store.finish(&counts, false).await.unwrap();
    let published = totals(&pool).await;
    let second = Store::start(pool.clone(), 300).await.unwrap();
    assert!(second
        .begin_run(source_id, &target)
        .await
        .unwrap()
        .is_none());
    assert_eq!(totals(&pool).await, published);
    second.finish(&Counts::default(), false).await.unwrap();
    pool.close().await;
    admin.close().await;
}

#[tokio::test]
#[ignore = "requires explicitly supplied public source checkouts"]
async fn public_timer_objects_persist_with_raw_receipts_and_unchanged_runs_are_empty() {
    let roots = [
        std::env::var_os("BRAIN_P3_PUBLIC_DEADLOCK_DATA_ROOT")
            .expect("public deadlock-data checkout"),
        std::env::var_os("BRAIN_P3_PUBLIC_GAMETRACKING_ROOT")
            .expect("public GameTracking checkout"),
    ];
    let scratch = Scratch::start();
    let admin = scratch.migrate().await;
    let pool = scratch.pool("brain_ingest").await;
    let store = Store::start(pool.clone(), 300).await.unwrap();
    let mut counts = Counts::default();
    let mut completed = Vec::new();
    for (source, root, paths) in [
        (
            Source::ALL[0],
            &roots[0],
            ["data/json/misc-data.json", "data/json/generic-data.json"],
        ),
        (
            Source::ALL[1],
            &roots[1],
            [
                "game/citadel/pak01_dir/scripts/misc.vdata",
                "game/citadel/pak01_dir/scripts/generic_data.vdata",
            ],
        ),
    ] {
        let revision = Command::new("git")
            .arg("-C")
            .arg(root)
            .args(["rev-parse", "HEAD"])
            .output()
            .unwrap();
        assert!(revision.status.success());
        let revision = String::from_utf8(revision.stdout)
            .unwrap()
            .trim()
            .to_owned();
        assert_eq!(revision.len(), 40);
        let source_id = store.source(source).await.unwrap();
        let run = store
            .begin_run(source_id, &revision)
            .await
            .unwrap()
            .unwrap();
        for (path, expected_count) in paths.into_iter().zip([11, 3]) {
            let object = Command::new("git")
                .arg("-C")
                .arg(root)
                .arg("show")
                .arg(format!("{revision}:{path}"))
                .output()
                .unwrap();
            assert!(object.status.success());
            let bytes = object.stdout;
            store
                .object(&run, source, path, Some(blob(&bytes)), &mut counts)
                .await
                .unwrap();
            let stored: (Vec<u8>, Vec<u8>) = sqlx::query_as(
                "SELECT body,content_sha256 FROM raw.object WHERE source_id=$1 AND external_id=$2",
            )
            .bind(source_id)
            .bind(path)
            .fetch_one(&pool)
            .await
            .unwrap();
            assert_eq!(stored.0, bytes);
            assert_eq!(stored.1, Sha256::digest(&bytes).to_vec());
            let expected = extract::extract(source, path, &stored.0).unwrap();
            assert_eq!(expected.len(), expected_count);
            extract::prove(source, path, &stored.0, &expected).unwrap();
            let actual: Vec<(String, String, Vec<u8>)> = sqlx::query_as("SELECT f.stat_key,r.locator,r.value_sha256 FROM canon.fact f JOIN canon.receipt r ON r.id=f.receipt_id WHERE f.source_id=$1 AND f.source_path=$2 ORDER BY f.id")
                .bind(source_id).bind(path).fetch_all(&pool).await.unwrap();
            assert_eq!(
                actual,
                expected
                    .into_iter()
                    .map(|fact| (fact.stat, fact.locator, fact.hash))
                    .collect::<Vec<_>>()
            );
        }
        store.publish(&run).await.unwrap();
        completed.push((source_id, revision));
    }
    assert_eq!(counts.raw_inserted, 4);
    assert_eq!(counts.objects_completed, 4);
    assert_eq!(totals(&pool).await, (4, 28, 28));
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM serve.timer")
            .fetch_one(&pool)
            .await
            .unwrap(),
        28
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM serve.timer WHERE key='urn'")
            .fetch_one(&pool)
            .await
            .unwrap(),
        0
    );
    store.finish(&counts, false).await.unwrap();
    let second = Store::start(pool.clone(), 300).await.unwrap();
    for (source_id, revision) in completed {
        assert!(second
            .begin_run(source_id, &revision)
            .await
            .unwrap()
            .is_none());
    }
    assert_eq!(totals(&pool).await, (4, 28, 28));
    second.finish(&Counts::default(), false).await.unwrap();
    pool.close().await;
    admin.close().await;
    cgroup_memory_proof();
}
