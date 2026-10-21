use super::*;
use crate::{
    ingest,
    patch_extract::{PatchExtractor, SOURCE},
    GameFactsStore,
};
use brain_storage::PgStore;
use sqlx::{postgres::PgPoolOptions, PgPool};

#[path = "../../../brain-storage/tests/support/scratch_pg.rs"]
mod scratch_pg;

fn config() -> Config {
    Config {
        infisical_config: "/etc/deadlock-brain/infisical.json".into(),
        max_posts: 5000,
        max_row_bytes: 8 * 1024 * 1024,
        timeout_seconds: 5,
    }
}

fn stream(config: Config, options: PgConnectOptions) -> InputObjects {
    spawn(config, async move { Ok(options) })
}

async fn setup() -> (scratch_pg::ScratchPg, PgPool, PgConnectOptions) {
    let pg = scratch_pg::ScratchPg::start();
    let options = PgConnectOptions::new_without_pgpass()
        .host(&pg.directory.join("socket").to_string_lossy())
        .port(55439)
        .username("brain_core_test")
        .database("postgres")
        .password("")
        .application_name("brain-job-patchnotes")
        .disable_statement_logging();
    let admin = PgPoolOptions::new()
        .max_connections(3)
        .connect_with(options.clone())
        .await
        .unwrap();
    sqlx::raw_sql(
        "CREATE SCHEMA patchnotes;
         CREATE TABLE patchnotes.changelog_posts (
             id bigint PRIMARY KEY, title text, url text, posted_at timestamptz,
             raw_content text, translated_content text
         );
         CREATE ROLE patchnotes_feed_reader LOGIN;
         GRANT USAGE ON SCHEMA patchnotes TO patchnotes_feed_reader;
         GRANT SELECT ON patchnotes.changelog_posts TO patchnotes_feed_reader;",
    )
    .execute(&admin)
    .await
    .unwrap();
    let reader = options
        .username("patchnotes_feed_reader")
        .options([("default_transaction_read_only", "on")]);
    (pg, admin, reader)
}

async fn assert_reader_closed(admin: &PgPool) {
    for _ in 0..2000 {
        let active: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM pg_stat_activity WHERE usename = 'patchnotes_feed_reader'",
        )
        .fetch_one(admin)
        .await
        .unwrap();
        if active == 0 {
            return;
        }
        tokio::task::yield_now().await;
    }
    panic!("reader backend did not close");
}

async fn store(admin: &PgPool) -> GameFactsStore {
    sqlx::raw_sql(
        "CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest NOLOGIN;
         CREATE ROLE brain_service NOLOGIN; CREATE ROLE brain_readonly NOLOGIN;
         GRANT CREATE ON DATABASE postgres TO brain_migrate;
         GRANT USAGE,CREATE ON SCHEMA public TO brain_migrate;",
    )
    .execute(admin)
    .await
    .unwrap();
    PgStore::new(admin.clone())
        .migrate_gamefacts()
        .await
        .unwrap();
    GameFactsStore::new(admin.clone())
}

#[test]
fn source_config_is_explicit_bounded_absolute_and_registered() {
    let pg = scratch_pg::ScratchPg::start();
    let path = pg.directory.join("deadlock_patchnotes_db");
    let valid = serde_json::json!({
        "infisical_config": "/etc/deadlock-brain/infisical.json",
        "max_posts": 5000, "max_row_bytes": 8388608, "timeout_seconds": 15
    });
    std::fs::write(&path, valid.to_string()).unwrap();
    assert!(Config::load(&path).is_ok());
    assert!(Config::load(Path::new("relative")).is_err());
    assert!(Config::load(&pg.directory.join("missing")).is_err());
    for (field, bad) in [
        ("infisical_config", serde_json::json!("relative")),
        ("max_posts", serde_json::json!(0)),
        ("max_posts", serde_json::json!(5001)),
        ("max_row_bytes", serde_json::json!(8388609)),
        ("max_row_bytes", serde_json::json!(crate::limits::RAW_BYTES)),
        ("timeout_seconds", serde_json::json!(61)),
        ("dsn", serde_json::json!("not-a-secret")),
    ] {
        let mut invalid = valid.clone();
        invalid[field] = bad;
        std::fs::write(&path, invalid.to_string()).unwrap();
        assert!(load(&path, true).next().unwrap().is_err());
    }
    for field in [
        "infisical_config",
        "max_posts",
        "max_row_bytes",
        "timeout_seconds",
    ] {
        let mut invalid = valid.clone();
        invalid.as_object_mut().unwrap().remove(field);
        std::fs::write(&path, invalid.to_string()).unwrap();
        assert!(Config::load(&path).is_err());
    }
    std::fs::write(&path, vec![b' '; 8193]).unwrap();
    assert!(Config::load(&path).is_err());
    assert!(parse_options("postgres://reader:synthetic@127.0.0.1/feed?sslmode=require").is_ok());
    for invalid in [
        "postgres://reader@127.0.0.1/feed",
        "postgres://reader@127.0.0.1/feed:synthetic@misleading",
        "postgres://reader:synthetic@127.0.0.1/feed?unknown=synthetic",
        "postgres://reader:synthetic@127.0.0.1/feed?options=synthetic",
        "postgres://reader:synthetic@127.0.0.1/feed?user=",
        "postgres://reader:synthetic@127.0.0.1/feed?password=",
        "postgres://reader:@127.0.0.1/feed",
    ] {
        assert!(parse_options(invalid).is_err());
    }
    assert_eq!(
        crate::sources::SOURCES
            .iter()
            .filter(|(name, _)| *name == "deadlock_patchnotes_db")
            .count(),
        1
    );
    assert!(
        crate::sources::fetch("deadlock_patchnotes_db", &pg.directory)
            .next()
            .unwrap()
            .is_err()
    );
}

#[tokio::test]
async fn real_postgres_envelopes_stream_and_second_ingest_adds_zero_raw_objects() {
    let (_pg, admin, reader) = setup().await;
    sqlx::raw_sql(
        "INSERT INTO patchnotes.changelog_posts
         SELECT id, 'Patch ' || id, 'https://forums.playdeadlock.com/threads/patch.' || id || '/',
                '2026-10-12 19:20:21.123456+02'::timestamptz,
                '- General Changes: ' || repeat('x', CASE WHEN id = 132 THEN 64006 ELSE 5200 END),
                'not part of the envelope'
         FROM generate_series(1,132) id;
         UPDATE patchnotes.changelog_posts SET posted_at = NULL, raw_content = NULL WHERE id = 2;",
    )
    .execute(&admin)
    .await
    .unwrap();
    let first: Vec<_> = stream(config(), reader.clone())
        .map(Result::unwrap)
        .collect();
    assert_eq!(first.len(), 132);
    let body: serde_json::Value = serde_json::from_slice(&first[0].bytes).unwrap();
    assert_eq!(body.as_object().unwrap().len(), 5);
    assert_eq!(body["posted_at"], "2026-10-12T17:20:21.123456Z");
    assert_eq!(body["id"], 1);
    assert_eq!(first[0].path, "1");
    assert_eq!(
        first[0].revision,
        format!("sha256:{:x}", Sha256::digest(&first[0].bytes))
    );
    let nulls: serde_json::Value = serde_json::from_slice(&first[1].bytes).unwrap();
    assert!(nulls["posted_at"].is_null());
    assert!(nulls["raw_content"].is_null());
    assert!(body.get("translated_content").is_none());
    let second: Vec<_> = stream(config(), reader.clone())
        .map(Result::unwrap)
        .collect();
    assert!(first
        .iter()
        .zip(&second)
        .all(|(a, b)| a.bytes == b.bytes && a.revision == b.revision));
    let store = store(&admin).await;
    let report = ingest(
        &store,
        "deadlock_patchnotes_db",
        stream(config(), reader.clone()),
        &[&PatchExtractor],
    )
    .await
    .unwrap();
    assert_eq!(report.counts.accepted, 131);
    assert_eq!(report.counts.ignored, 1);
    assert_eq!(report.counts.rejected, 0);
    let before: i64 = sqlx::query_scalar("SELECT count(*) FROM raw.object")
        .fetch_one(&admin)
        .await
        .unwrap();
    assert_eq!(before, 132);
    let repeated = ingest(
        &store,
        "deadlock_patchnotes_db",
        stream(config(), reader.clone()),
        &[&PatchExtractor],
    )
    .await
    .unwrap();
    assert_eq!(repeated.counts.accepted, 0);
    assert_eq!(repeated.counts.ignored, 132);
    let after: i64 = sqlx::query_scalar("SELECT count(*) FROM raw.object")
        .fetch_one(&admin)
        .await
        .unwrap();
    assert_eq!(after - before, 0);
    sqlx::query("UPDATE patchnotes.changelog_posts SET title = 'Changed' WHERE id = 1")
        .execute(&admin)
        .await
        .unwrap();
    let changed = stream(config(), reader.clone()).next().unwrap().unwrap();
    assert_eq!(changed.path, first[0].path);
    assert_ne!(changed.revision, first[0].revision);
    sqlx::query("DROP TABLE patchnotes.changelog_posts")
        .execute(&admin)
        .await
        .unwrap();
    assert!(ingest(
        &store,
        "deadlock_patchnotes_db",
        stream(config(), reader),
        &[&PatchExtractor]
    )
    .await
    .is_err());
    let status: String =
        sqlx::query_scalar("SELECT status FROM ops.ingest_run ORDER BY id DESC LIMIT 1")
            .fetch_one(&admin)
            .await
            .unwrap();
    assert_eq!(status, "failed");
    let total: i64 = sqlx::query_scalar("SELECT count(*) FROM raw.object")
        .fetch_one(&admin)
        .await
        .unwrap();
    assert_eq!(total, 132);
}

#[tokio::test]
async fn content_poison_is_captured_unchanged_and_does_not_block_valid_posts() {
    let (_pg, admin, reader) = setup().await;
    sqlx::raw_sql(
        "INSERT INTO patchnotes.changelog_posts
         SELECT id, 'Patch', 'https://forums.playdeadlock.com/threads/patch.' || id || '/',
                '2026-10-12T17:20:21Z'::timestamptz, 'General content', NULL
         FROM generate_series(0,6) id;
         UPDATE patchnotes.changelog_posts SET title = NULL WHERE id = 1;
         UPDATE patchnotes.changelog_posts SET url = NULL WHERE id = 2;
         UPDATE patchnotes.changelog_posts SET posted_at = 'infinity' WHERE id = 4;
         UPDATE patchnotes.changelog_posts SET posted_at = '0001-01-01 BC' WHERE id = 5;",
    )
    .execute(&admin)
    .await
    .unwrap();
    sqlx::query("UPDATE patchnotes.changelog_posts SET raw_content = repeat('x', $1) WHERE id = 3")
        .bind((crate::limits::RAW_BYTES + 1) as i32)
        .execute(&admin)
        .await
        .unwrap();
    let objects: Vec<_> = stream(config(), reader.clone())
        .map(Result::unwrap)
        .collect();
    assert_eq!(objects.len(), 7);
    let body =
        |index: usize| serde_json::from_slice::<serde_json::Value>(&objects[index].bytes).unwrap();
    assert!(body(1)["title"].is_null());
    assert!(body(2)["url"].is_null());
    assert_eq!(body(4)["posted_at"], "infinity");
    assert!(body(5)["posted_at"].as_str().unwrap().contains("BC"));
    assert_eq!(objects[0].path, "0");
    assert!(objects[3].bytes.len() > crate::limits::RAW_BYTES);
    let store = store(&admin).await;
    let report = ingest(
        &store,
        "deadlock_patchnotes_db",
        stream(config(), reader),
        &[&PatchExtractor],
    )
    .await
    .unwrap();
    assert_eq!(
        report.counts,
        crate::Counts {
            accepted: 1,
            rejected: 5,
            ignored: 1
        }
    );
    let raw: Vec<u8> = sqlx::query_scalar("SELECT bytes FROM raw.object WHERE path = '3'")
        .fetch_one(&admin)
        .await
        .unwrap();
    assert_eq!(raw, objects[3].bytes);
    let raw_count: i64 = sqlx::query_scalar("SELECT count(*) FROM raw.object")
        .fetch_one(&admin)
        .await
        .unwrap();
    assert_eq!(raw_count, 7);
    let status: String = sqlx::query_scalar("SELECT status FROM ops.ingest_run WHERE id = $1")
        .bind(report.run_id)
        .fetch_one(&admin)
        .await
        .unwrap();
    assert_eq!(status, "succeeded");
    let codes: Vec<String> =
        sqlx::query_scalar("SELECT code FROM ops.ingest_rejection WHERE run_id = $1 ORDER BY code")
            .bind(report.run_id)
            .fetch_all(&admin)
            .await
            .unwrap();
    assert_eq!(
        codes,
        [
            "date_invalid",
            "date_invalid",
            "raw_too_large",
            "title_invalid",
            "url_invalid"
        ]
    );
}

#[tokio::test]
async fn oversized_posts_are_logged_counted_and_later_posts_are_ingested() {
    if std::env::var_os("BRAIN_P4_OVERSIZE_LOG_CHILD").is_none() {
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "patch_extract::source::tests::oversized_posts_are_logged_counted_and_later_posts_are_ingested",
                "--nocapture",
                "--test-threads=1",
            ])
            .env("BRAIN_P4_OVERSIZE_LOG_CHILD", "1")
            .output()
            .unwrap();
        assert!(output.status.success());
        let stderr = String::from_utf8(output.stderr).unwrap();
        let logs: Vec<serde_json::Value> = stderr
            .lines()
            .filter_map(|line| serde_json::from_str(line).ok())
            .collect();
        assert_eq!(logs.len(), 3);
        for (entry, id, size, skipped) in [(&logs[0], 1, 8388615, 1), (&logs[1], 3, 8388609, 2)] {
            assert_eq!(entry["event"], "patchnotes_source_skip");
            assert_eq!(entry["source"], SOURCE);
            assert_eq!(entry["post_id"], id);
            assert_eq!(entry["reason"], "transport_row_too_large");
            assert_eq!(entry["byte_size"], size);
            assert_eq!(entry["skipped_oversized_posts"], skipped);
        }
        assert_eq!(logs[2]["event"], "patchnotes_source_summary");
        assert_eq!(logs[2]["source"], SOURCE);
        assert_eq!(logs[2]["scanned_posts"], 4);
        assert_eq!(logs[2]["emitted_posts"], 2);
        assert_eq!(logs[2]["skipped_oversized_posts"], 2);
        println!("P4_OVERSIZE_CONTINUATION_PROOF: skipped=2; exact post IDs/reason/byte sizes and summary verified; later posts accepted=2; raw paths=2,4; run=succeeded");
        return;
    }
    let (_pg, admin, reader) = setup().await;
    sqlx::raw_sql(
        "INSERT INTO patchnotes.changelog_posts
         SELECT id, 'Patch', 'https://forums.playdeadlock.com/threads/patch.' || id || '/',
                '2026-10-12T17:20:21Z'::timestamptz, 'General content', NULL
         FROM generate_series(1,4) id;
         UPDATE patchnotes.changelog_posts
         SET title = repeat(chr(228), 3), url = NULL, raw_content = repeat('x', 8388609)
         WHERE id = 1;
         UPDATE patchnotes.changelog_posts
         SET title = NULL, url = NULL, raw_content = repeat('x', 8388609)
         WHERE id = 3;",
    )
    .execute(&admin)
    .await
    .unwrap();
    let store = store(&admin).await;
    let report = ingest(&store, SOURCE, stream(config(), reader), &[&PatchExtractor])
        .await
        .unwrap();
    assert_eq!(
        report.counts,
        crate::Counts {
            accepted: 2,
            rejected: 0,
            ignored: 0
        }
    );
    let paths: Vec<String> = sqlx::query_scalar("SELECT path FROM raw.object ORDER BY path")
        .fetch_all(&admin)
        .await
        .unwrap();
    assert_eq!(paths, ["2", "4"]);
    let status: String = sqlx::query_scalar("SELECT status FROM ops.ingest_run WHERE id = $1")
        .bind(report.run_id)
        .fetch_one(&admin)
        .await
        .unwrap();
    assert_eq!(status, "succeeded");
}

#[tokio::test]
async fn worst_bounded_rows_backpressure_and_ingest_stay_below_300_mb() {
    let (_pg, admin, reader) = setup().await;
    sqlx::raw_sql(
        "INSERT INTO patchnotes.changelog_posts
         SELECT id, 'Patch', 'https://forums.playdeadlock.com/threads/patch.' || id || '/',
                '2026-10-12T17:20:21Z'::timestamptz, '', NULL FROM generate_series(1,4) id;
         UPDATE patchnotes.changelog_posts SET raw_content = repeat(chr(1),
             8388608 - octet_length(title) - octet_length(url));",
    )
    .execute(&admin)
    .await
    .unwrap();
    let size: i64 = sqlx::query_scalar(
        "SELECT min(octet_length(title)::bigint + octet_length(url) + octet_length(raw_content))
         FROM patchnotes.changelog_posts",
    )
    .fetch_one(&admin)
    .await
    .unwrap();
    assert_eq!(size, i64::from(config().max_row_bytes));
    let store = store(&admin).await;
    let slow = |reader| {
        stream(config(), reader).inspect(|object| {
            assert!(object.as_ref().unwrap().bytes.len() > 48 * 1024 * 1024 - 1024);
            std::thread::sleep(Duration::from_millis(200));
        })
    };
    let first = ingest(
        &store,
        "deadlock_patchnotes_db",
        slow(reader.clone()),
        &[&PatchExtractor],
    )
    .await
    .unwrap();
    assert_eq!(
        first.counts,
        crate::Counts {
            accepted: 0,
            rejected: 4,
            ignored: 0
        }
    );
    let before: i64 = sqlx::query_scalar("SELECT count(*) FROM raw.object")
        .fetch_one(&admin)
        .await
        .unwrap();
    let second = ingest(
        &store,
        "deadlock_patchnotes_db",
        slow(reader),
        &[&PatchExtractor],
    )
    .await
    .unwrap();
    let after: i64 = sqlx::query_scalar("SELECT count(*) FROM raw.object")
        .fetch_one(&admin)
        .await
        .unwrap();
    assert_eq!(before, 4);
    assert_eq!(after - before, 0);
    assert_eq!(
        second.counts,
        crate::Counts {
            accepted: 0,
            rejected: 4,
            ignored: 0
        }
    );
    let status = std::fs::read_to_string("/proc/self/status").unwrap();
    let peak_kib: u64 = status
        .lines()
        .find_map(|line| line.strip_prefix("VmHWM:"))
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let peak_bytes = peak_kib * 1024;
    println!("P4 RSS peak={peak_bytes} bytes; bounded row={size} bytes; escaped envelopes>48 MiB-1 KiB; channel=0; demand-driven; slow consumer=200 ms; first raw={before}; second raw={after}; delta={}", after - before);
    assert!(
        peak_bytes < 300_000_000,
        "ingest peak RSS={peak_bytes} bytes"
    );
}

#[tokio::test]
async fn real_postgres_bounds_role_failure_and_drop_cancel_workers() {
    let (_pg, admin, reader) = setup().await;
    sqlx::raw_sql(
        "INSERT INTO patchnotes.changelog_posts
         SELECT id, 'Patch', 'https://forums.playdeadlock.com/threads/patch.' || id || '/',
                '2026-10-12T17:20:21Z'::timestamptz, repeat('x', 64006), NULL FROM generate_series(1,132) id;",
    )
    .execute(&admin)
    .await
    .unwrap();
    let mut source = stream(config(), reader.clone());
    assert!(source.next().unwrap().is_ok());
    drop(source);
    assert_reader_closed(&admin).await;
    sqlx::query(
        "UPDATE patchnotes.changelog_posts SET raw_content = repeat('x', 8388609) WHERE id = 1",
    )
    .execute(&admin)
    .await
    .unwrap();
    let following = stream(config(), reader.clone()).next().unwrap().unwrap();
    assert_eq!(following.path, "2");
    sqlx::query(
        "UPDATE patchnotes.changelog_posts SET raw_content = repeat('x', 64006) WHERE id = 1",
    )
    .execute(&admin)
    .await
    .unwrap();
    let mut few = config();
    few.max_posts = 1;
    let mut source = stream(few, reader.clone());
    assert!(source.next().unwrap().is_ok());
    assert!(source.next().unwrap().is_err());
    assert!(source.next().is_none());
    drop(source);
    sqlx::query("GRANT UPDATE ON patchnotes.changelog_posts TO patchnotes_feed_reader")
        .execute(&admin)
        .await
        .unwrap();
    assert!(stream(config(), reader.clone()).next().unwrap().is_err());
    sqlx::query("REVOKE UPDATE ON patchnotes.changelog_posts FROM patchnotes_feed_reader")
        .execute(&admin)
        .await
        .unwrap();
    let mut transaction = admin.begin().await.unwrap();
    sqlx::query("LOCK TABLE patchnotes.changelog_posts IN ACCESS EXCLUSIVE MODE")
        .execute(&mut *transaction)
        .await
        .unwrap();
    let source = stream(config(), reader.clone());
    let mut blocked = false;
    for _ in 0..2000 {
        let waiting: i64 = sqlx::query_scalar("SELECT count(*) FROM pg_stat_activity WHERE usename = 'patchnotes_feed_reader' AND wait_event_type = 'Lock'").fetch_one(&admin).await.unwrap();
        if waiting > 0 {
            blocked = true;
            break;
        }
        tokio::task::yield_now().await;
    }
    assert!(blocked);
    drop(source);
    transaction.rollback().await.unwrap();
    assert_reader_closed(&admin).await;
    sqlx::query("DROP TABLE patchnotes.changelog_posts")
        .execute(&admin)
        .await
        .unwrap();
    let errors: Vec<_> = stream(config(), reader.clone()).collect();
    assert_eq!(errors.len(), 1);
    let error = format!("{:#}", errors[0].as_ref().unwrap_err());
    assert!(!error.contains("postgres"));
    assert!(!error.contains("patchnotes_feed_reader"));
    assert!(stream(config(), reader.port(1)).next().unwrap().is_err());
}
