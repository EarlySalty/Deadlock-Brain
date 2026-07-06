//! Tests fuer `dbrain-normalize`.
//!
//! - Ein reiner Logik-Test (kein DB-Zugriff) fuer den Lineage-Extraktor.
//! - PG-Integrationstests gegen die Scratch-Postgres aus `DEADLOCK_CENTRAL_DSN`.
//!   Sie sind `#[ignore]` + self-skip: fehlt das DSN, kehren sie sofort zurueck,
//!   damit `cargo test` offline gruen bleibt.
//!
//! Alle Schreibpfade laufen idempotent (`rebuild=false` -> `ON CONFLICT
//! DO NOTHING`/`DO UPDATE`) und werden zusaetzlich per `id > vorher_max`-Cleanup
//! abgeraeumt, damit die Live-Zaehlerstaende (patch_events=13383, entities=905, …)
//! nach dem Testlauf unveraendert sind.

use sqlx::postgres::{PgPool, PgPoolOptions};

use crate::{
    enrich_legacy_entities, enrich_lineage, extract_lineage_candidates, normalize_entities,
    normalize_sheet_stats, normalize_sheet_tabs, parse_patchnotes, resolve_gaps, LineageEvent,
};

const SKIP_REASON: &str = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN";

/// Scratch-Pool aus `DEADLOCK_CENTRAL_DSN`. `None` (Test-Skip) wenn die Env-Var
/// fehlt oder leer ist. Das echte zentrale DSN wird nie eingebrannt.
async fn test_pool() -> Option<PgPool> {
    let dsn = std::env::var("DEADLOCK_CENTRAL_DSN").ok()?;
    if dsn.trim().is_empty() {
        return None;
    }
    Some(
        PgPoolOptions::new()
            .max_connections(4)
            .connect(&dsn)
            .await
            .expect("connect scratch postgres"),
    )
}

async fn count(pool: &PgPool, table: &str) -> i64 {
    let sql = format!("SELECT COUNT(*) FROM brain.{table}");
    sqlx::query_scalar::<_, i64>(&sql)
        .fetch_one(pool)
        .await
        .expect("count rows")
}

async fn max_id(pool: &PgPool, table: &str) -> i64 {
    let sql = format!("SELECT COALESCE(MAX(id), 0) FROM brain.{table}");
    sqlx::query_scalar::<_, i64>(&sql)
        .fetch_one(pool)
        .await
        .expect("max id")
}

/// Sicherheitsnetz: entfernt alle Zeilen, die der Test ueber `min_id` hinaus
/// eingefuegt haben koennte, damit die Live-Zaehler stabil bleiben. Bei echter
/// Idempotenz loescht das nichts.
async fn cleanup_above(pool: &PgPool, table: &str, min_id: i64) {
    let sql = format!("DELETE FROM brain.{table} WHERE id > $1");
    sqlx::query(&sql)
        .bind(min_id)
        .execute(pool)
        .await
        .expect("cleanup rows");
}

#[test]
fn extracts_direct_lineage_rename_with_owner() {
    let direct = extract_lineage_candidates(&LineageEvent {
        id: 99,
        patch_title: None,
        patch_url: None,
        source_kind: Some("forum".to_string()),
        posted_at: None,
        entity_type: Some("hero".to_string()),
        entity_name: Some("Abrams".to_string()),
        subject: None,
        change_type: Some("changed".to_string()),
        normalized_line: Some("Life Drain renamed to Siphon Life.".to_string()),
        raw_line: None,
    });
    assert_eq!(direct.len(), 1);
    assert_eq!(direct[0].relation_type, "rename");
    assert_eq!(direct[0].source_entity_type.as_deref(), Some("ability"));
    assert_eq!(direct[0].source_name, "Life Drain");
    assert_eq!(direct[0].target_name.as_deref(), Some("Siphon Life"));
    assert_eq!(direct[0].owner_name.as_deref(), Some("Abrams"));
}

#[tokio::test]
#[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
async fn parse_patchnotes_is_count_stable_and_idempotent() {
    let Some(pool) = test_pool().await else {
        eprintln!("skip: {SKIP_REASON}");
        return;
    };
    let before = count(&pool, "patch_events").await;
    let before_max = max_id(&pool, "patch_events").await;

    let summary = parse_patchnotes(&pool, false).await.expect("parse patchnotes");
    let inserted = summary["events_inserted"].as_i64().expect("events_inserted");
    let total = summary["events_total"].as_i64().expect("events_total");

    cleanup_above(&pool, "patch_events", before_max).await;
    let after = count(&pool, "patch_events").await;

    assert_eq!(after, before, "patch_events count must stay stable");
    assert_eq!(total, before, "read-path total must equal direct SQL count");
    assert_eq!(inserted, 0, "re-parse of migrated data must insert nothing");
}

#[tokio::test]
#[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
async fn normalize_entities_is_count_stable() {
    let Some(pool) = test_pool().await else {
        eprintln!("skip: {SKIP_REASON}");
        return;
    };
    let before_entities = count(&pool, "entities").await;
    let before_aliases = count(&pool, "entity_aliases").await;
    let max_entities = max_id(&pool, "entities").await;
    let max_aliases = max_id(&pool, "entity_aliases").await;

    let summary = normalize_entities(&pool, false).await.expect("normalize entities");
    let processed = summary["entities"].as_i64().expect("entities");

    cleanup_above(&pool, "entity_aliases", max_aliases).await;
    cleanup_above(&pool, "entities", max_entities).await;
    let after_entities = count(&pool, "entities").await;
    let after_aliases = count(&pool, "entity_aliases").await;

    assert_eq!(after_entities, before_entities, "entities count stable");
    assert_eq!(after_aliases, before_aliases, "entity_aliases count stable");
    assert!(processed > 0, "must upsert at least one candidate");
}

#[tokio::test]
#[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
async fn enrich_lineage_is_count_stable_and_idempotent() {
    let Some(pool) = test_pool().await else {
        eprintln!("skip: {SKIP_REASON}");
        return;
    };
    let before = count(&pool, "entity_lineage").await;
    let before_max = max_id(&pool, "entity_lineage").await;

    let summary = enrich_lineage(&pool, false).await.expect("enrich lineage");
    let inserted = summary["lineage_inserted"].as_i64().expect("lineage_inserted");
    let total = summary["lineage_total"].as_i64().expect("lineage_total");

    cleanup_above(&pool, "entity_lineage", before_max).await;
    let after = count(&pool, "entity_lineage").await;

    assert_eq!(after, before, "entity_lineage count stable");
    assert_eq!(total, before, "read-path total equals direct SQL count");
    assert_eq!(inserted, 0, "re-run must not insert new lineage rows");
}

#[tokio::test]
#[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
async fn enrich_legacy_entities_is_count_stable() {
    let Some(pool) = test_pool().await else {
        eprintln!("skip: {SKIP_REASON}");
        return;
    };
    let before = count(&pool, "legacy_entities").await;
    let before_max = max_id(&pool, "legacy_entities").await;

    let summary = enrich_legacy_entities(&pool, false)
        .await
        .expect("enrich legacy entities");
    let total = summary["legacy_total"].as_i64().expect("legacy_total");

    cleanup_above(&pool, "legacy_entities", before_max).await;
    let after = count(&pool, "legacy_entities").await;

    assert_eq!(after, before, "legacy_entities count stable");
    assert_eq!(total, before, "read-path total equals direct SQL count");
}

#[tokio::test]
#[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
async fn normalize_sheet_stats_is_count_stable() {
    let Some(pool) = test_pool().await else {
        eprintln!("skip: {SKIP_REASON}");
        return;
    };
    let before_values = count(&pool, "hero_stat_values").await;
    let before_profiles = count(&pool, "hero_stat_profiles").await;
    let max_values = max_id(&pool, "hero_stat_values").await;
    let max_profiles = max_id(&pool, "hero_stat_profiles").await;

    let summary = normalize_sheet_stats(&pool, false)
        .await
        .expect("normalize sheet stats");
    let profiles = summary["profiles"].as_i64().expect("profiles");

    cleanup_above(&pool, "hero_stat_values", max_values).await;
    cleanup_above(&pool, "hero_stat_profiles", max_profiles).await;
    let after_values = count(&pool, "hero_stat_values").await;
    let after_profiles = count(&pool, "hero_stat_profiles").await;

    assert_eq!(after_profiles, before_profiles, "hero_stat_profiles stable");
    assert_eq!(after_values, before_values, "hero_stat_values stable");
    assert!(profiles > 0, "must upsert at least one profile");
}

#[tokio::test]
#[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
async fn normalize_sheet_tabs_is_count_stable() {
    let Some(pool) = test_pool().await else {
        eprintln!("skip: {SKIP_REASON}");
        return;
    };
    let tables = [
        "sheet_hero_rankings",
        "sheet_boons_ap",
        "sheet_raw_heroes",
        "sheet_heroes_stats",
        "sheet_items",
        "sheet_shop_bonuses",
        "sheet_tab_rows",
    ];
    let mut before = Vec::new();
    let mut maxes = Vec::new();
    for table in tables {
        before.push(count(&pool, table).await);
        maxes.push(max_id(&pool, table).await);
    }

    let summary = normalize_sheet_tabs(&pool, false)
        .await
        .expect("normalize sheet tabs");

    for (index, table) in tables.iter().enumerate() {
        cleanup_above(&pool, table, maxes[index]).await;
    }
    for (index, table) in tables.iter().enumerate() {
        assert_eq!(count(&pool, table).await, before[index], "{table} count stable");
    }
    assert!(summary["sheet_items"]["snapshots"].as_i64().unwrap_or(0) > 0);
}

#[tokio::test]
#[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
async fn resolve_gaps_dry_run_reads_without_writing() {
    let Some(pool) = test_pool().await else {
        eprintln!("skip: {SKIP_REASON}");
        return;
    };
    let before_events = count(&pool, "patch_events").await;

    let summary = resolve_gaps(&pool, true).await.expect("resolve gaps dry run");

    assert_eq!(summary["dry_run"], true);
    assert!(summary["patch_events"]["changed"].is_number());
    assert!(summary["claims"]["changed"].is_number());
    assert!(summary["entity_id_backfill"]["changed"].is_number());

    let after_events = count(&pool, "patch_events").await;
    assert_eq!(after_events, before_events, "dry-run must not write");
}
