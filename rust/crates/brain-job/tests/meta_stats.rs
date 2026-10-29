use brain_job::{
    ingest, limits, meta_source, meta_stats::MetaStats, registry::EXTRACTORS, Extractor, FactSet,
    FactValue, GameFactsStore, InputObject, Outcome, RawObject,
};
use brain_storage::gamefacts::receipts;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

#[path = "../../brain-storage/tests/support/gamefacts_pg.rs"]
pub mod gamefacts_pg;
use gamefacts_pg::{count, growth, rejection_codes};

const HERO: &str = "hero-stats.json";
const ITEM: &str = "item-stats.json";
const HOUR_A: &str = "2026-10-13T19:00:00Z";
const HOUR_B: &str = "2026-10-13T20:00:00Z";

fn fixture(path: &str) -> Vec<u8> {
    std::fs::read(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/meta_stats")
            .join(path),
    )
    .unwrap()
}

fn raw(path: &str, bytes: &[u8]) -> RawObject {
    RawObject {
        id: 1,
        source: meta_source::SOURCE.into(),
        revision: HOUR_A.into(),
        path: path.into(),
        content_sha256: Sha256::digest(bytes).into(),
        bytes: bytes.to_vec(),
    }
}

fn accepted(path: &str, bytes: &[u8]) -> FactSet {
    let raw = raw(path, bytes);
    let Outcome::Accepted(set) = MetaStats.extract(&raw) else {
        panic!("{:?}", MetaStats.extract(&raw))
    };
    receipts::verify(&raw.bytes, &set).unwrap();
    set
}

fn rejected(path: &str, bytes: &[u8]) -> &'static str {
    let Outcome::Rejected(rejection) = MetaStats.extract(&raw(path, bytes)) else {
        panic!("expected rejection")
    };
    rejection.code
}

fn input(revision: &str, path: &str, bytes: Vec<u8>) -> anyhow::Result<InputObject> {
    Ok(InputObject {
        revision: revision.into(),
        path: path.into(),
        bytes,
    })
}

fn snapshot(revision: &str, hero: Vec<u8>) -> Vec<anyhow::Result<InputObject>> {
    vec![
        input(revision, HERO, hero),
        input(revision, ITEM, fixture(ITEM)),
    ]
}

#[test]
fn snapshot_hour_is_utc_and_truncated() {
    assert_eq!(meta_source::hour(0), "1970-01-01T00:00:00Z");
    assert_eq!(meta_source::hour(951_782_399), "2000-02-28T23:00:00Z");
    assert_eq!(meta_source::hour(951_868_800), "2000-03-01T00:00:00Z");
    assert_eq!(meta_source::hour(1_791_919_000), "2026-10-13T19:00:00Z");
}

#[test]
fn window_starts_the_window_days_before_the_capture_day() {
    let days = meta_source::PRIMARY_DAYS;
    assert_eq!(
        meta_source::window_start(1_791_919_000, days),
        1_789_257_600
    );
    assert_eq!(
        meta_source::window_start(1_791_849_600, days),
        1_789_257_600
    );
    assert_eq!(
        meta_source::window_start(1_791_849_599, days),
        1_789_171_200
    );
    assert_eq!(meta_source::window_start(0, days), 0);
    assert_eq!(
        meta_source::window_start(1_791_919_000, meta_source::FALLBACK_DAYS),
        1_790_640_000
    );
}

#[test]
fn replay_is_refused_so_snapshot_time_is_capture_time() {
    let objects: Vec<_> = meta_source::deadlock_api_meta(
        std::path::Path::new("/nonexistent/2026-10-13T19:00:00Z"),
        false,
    )
    .collect();
    assert_eq!(objects.len(), 1);
    assert!(objects[0].is_err());
}

#[test]
fn real_captures_become_one_cited_row_per_hero_and_item() {
    assert!(MetaStats.accepts(meta_source::SOURCE, HERO));
    assert!(MetaStats.accepts(meta_source::SOURCE, ITEM));
    assert!(MetaStats.accepts(meta_source::SOURCE, "item-stats-14d.json"));
    assert!(MetaStats.accepts(meta_source::SOURCE, "hero-stats-14d.json"));
    assert!(!MetaStats.accepts("deadlock-data", HERO));
    assert!(!MetaStats.accepts(meta_source::SOURCE, "other.json"));
    for (path, entity, prefix, rows) in [
        (HERO, "meta_hero_stats", "hero_", 41),
        (ITEM, "meta_item_stats", "item_", 156),
    ] {
        let bytes = fixture(path);
        let set = accepted(path, &bytes);
        assert_eq!(set.entities.len(), 1);
        assert_eq!(set.entities[0].key, entity);
        assert_eq!(set.facts.len(), rows);
        let parsed: Vec<Value> = serde_json::from_slice(&bytes).unwrap();
        for (fact, row) in set.facts.iter().zip(&parsed) {
            assert!(fact.stat_key.starts_with(prefix));
            let FactValue::Text(text) = &fact.value else {
                panic!("text expected")
            };
            assert_eq!(&serde_json::from_str::<Value>(text).unwrap(), row);
        }
    }
}

#[test]
fn malformed_rows_reject_the_whole_snapshot() {
    for bytes in [
        br#"[]"#.as_slice(),
        br#"[{"hero_id":1,"wins":"5"}]"#,
        br#"[{"hero_id":0,"wins":5}]"#,
        br#"[{"wins":5}]"#,
        br#"[{"hero_id":1,"Wins":5}]"#,
        br#"[1]"#,
    ] {
        assert_eq!(rejected(HERO, bytes), "malformed_meta_stats");
    }
    for bytes in [
        br#"{"hero_id":1}"#.as_slice(),
        br#"[{"hero_id":1,"wins":1,"wins":2}]"#,
    ] {
        assert_eq!(rejected(HERO, bytes), "malformed_json");
    }
    assert_eq!(
        rejected(ITEM, br#"[{"hero_id":1,"wins":1}]"#),
        "malformed_meta_stats"
    );
}

fn collision() -> Vec<u8> {
    br#"[{"hero_id":1,"wins":1},{"hero_id":1,"wins":2}]"#.to_vec()
}

fn oversized() -> Vec<u8> {
    let mut row = json!({"hero_id":1});
    for index in 0..limits::TEXT_BYTES / 8 {
        row[format!("f{index:05}")] = json!(1);
    }
    serde_json::to_vec(&json!([row])).unwrap()
}

#[test]
fn poison_values_reject_with_limit_codes() {
    assert_eq!(rejected(HERO, &collision()), "key_collision");
    assert_eq!(rejected(HERO, &oversized()), "value_too_large");
}

fn changed_hero() -> Vec<u8> {
    let mut rows: Vec<Value> = serde_json::from_slice(&fixture(HERO)).unwrap();
    rows[0]["wins"] = json!(rows[0]["wins"].as_u64().unwrap() + 1);
    serde_json::to_vec(&rows).unwrap()
}

#[tokio::test]
async fn hourly_snapshots_are_idempotent_and_keep_history() {
    let (_pg, admin, writer) = gamefacts_pg::fixture(1).await;
    let store = GameFactsStore::new(writer.clone());
    let source = meta_source::SOURCE;
    let mut objects = vec![
        input(HOUR_A, HERO, collision()),
        input(HOUR_A, HERO, oversized()),
    ];
    objects.extend(snapshot(HOUR_A, fixture(HERO)));
    let first = ingest(&store, source, objects, EXTRACTORS).await.unwrap();
    assert_eq!((first.counts.accepted, first.counts.rejected), (2, 2));
    assert_eq!(
        rejection_codes(&admin, first.run_id).await,
        ["key_collision", "value_too_large"]
    );
    let after_first = growth(&admin).await;
    let rerun = ingest(&store, source, snapshot(HOUR_A, fixture(HERO)), EXTRACTORS)
        .await
        .unwrap();
    assert_eq!((rerun.counts.accepted, rerun.counts.ignored), (0, 2));
    let unchanged = ingest(&store, source, snapshot(HOUR_B, fixture(HERO)), EXTRACTORS)
        .await
        .unwrap();
    assert_eq!(
        (unchanged.counts.accepted, unchanged.counts.ignored),
        (0, 2)
    );
    assert_eq!(growth(&admin).await, after_first);
    let second = ingest(&store, source, snapshot(HOUR_B, changed_hero()), EXTRACTORS)
        .await
        .unwrap();
    assert_eq!((second.counts.accepted, second.counts.ignored), (1, 1));
    let after_second = growth(&admin).await;
    assert_eq!(after_second.0, after_first.0 + 1);
    assert_eq!(after_second.1, after_first.1 + 41);
    assert_eq!(
        count(
            &admin,
            "SELECT count(DISTINCT run_id) FROM serve.meta_stat WHERE stat_key='hero_1'"
        )
        .await,
        2
    );
    assert_eq!(
        count(
            &admin,
            "SELECT count(*) FROM serve.meta_stat WHERE source_revision='2026-10-13T20:00:00Z'"
        )
        .await,
        41
    );
    assert_eq!(
        count(
            &admin,
            "SELECT count(*) FROM ops.ingest_run WHERE status='succeeded'"
        )
        .await,
        4
    );
    writer.close().await;
    admin.close().await;
}

#[tokio::test]
async fn reverted_snapshot_is_reapplied_and_dated_by_its_own_run() {
    let (_pg, admin, writer) = gamefacts_pg::fixture(1).await;
    let store = GameFactsStore::new(writer.clone());
    let source = meta_source::SOURCE;
    let first = ingest(&store, source, snapshot(HOUR_A, fixture(HERO)), EXTRACTORS)
        .await
        .unwrap();
    let changed = ingest(&store, source, snapshot(HOUR_B, changed_hero()), EXTRACTORS)
        .await
        .unwrap();
    assert_eq!((changed.counts.accepted, changed.counts.ignored), (1, 1));
    let raw_before = growth(&admin).await.0;
    let reverted = ingest(
        &store,
        source,
        snapshot("2026-10-13T21:00:00Z", fixture(HERO)),
        EXTRACTORS,
    )
    .await
    .unwrap();
    assert_eq!((reverted.counts.accepted, reverted.counts.ignored), (1, 1));
    assert_eq!(growth(&admin).await.0, raw_before);
    let (run, value, revision, dated_by_run): (i64, Value, String, bool) = sqlx::query_as(
        "SELECT m.run_id,m.value,m.source_revision,m.snapshot_at=date_trunc('hour',r.started,'UTC') FROM serve.meta_stat m JOIN ops.ingest_run r ON r.id=m.run_id WHERE m.stat_key='hero_1' ORDER BY m.snapshot_at DESC,m.run_id DESC LIMIT 1",
    )
    .fetch_one(&admin)
    .await
    .unwrap();
    let original: Vec<Value> = serde_json::from_slice(&fixture(HERO)).unwrap();
    let hero_1 = original
        .iter()
        .find(|row| row["hero_id"] == 1)
        .unwrap()
        .clone();
    assert_eq!(run, reverted.run_id);
    assert_eq!(value, hero_1);
    assert_eq!(revision, HOUR_A);
    assert!(dated_by_run);
    assert_eq!(
        count(
            &admin,
            &format!(
                "SELECT count(*) FROM serve.meta_stat WHERE stat_key='hero_1' AND run_id IN ({},{},{})",
                first.run_id, changed.run_id, reverted.run_id
            )
        )
        .await,
        3
    );
    writer.close().await;
    admin.close().await;
}
