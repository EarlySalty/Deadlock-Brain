use super::*;
use serde_json::json;

pub(crate) static SCRATCH_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

async fn fix2_read_only_facade(command: &str) {
    let Some((_guard, ctx)) = scratch_context().await else {
        panic!("Fixrunde 2 benötigt eine isolierte Scratch-DSN");
    };
    let filter = BacktestFilter {
        hero: Some("Warden".into()),
        patch_tag: None,
    };
    let expected = match command {
        "build" => serde_json::to_value(reason_build(&ctx, "Warden").await.unwrap()).unwrap(),
        "patch-impact" => {
            serde_json::to_value(reason_patch_impact(&ctx, "Warden").await.unwrap()).unwrap()
        }
        "backtest" => {
            serde_json::to_value(reason_backtest(&ctx, filter.clone()).await.unwrap()).unwrap()
        }
        _ => unreachable!(),
    };
    sqlx::raw_sql("TRUNCATE brain.reasoner_builds, brain.reasoner_item_scores, brain.reasoner_patch_deltas, brain.reasoner_backtests; SET default_transaction_read_only=on;")
        .execute(&ctx.pool).await.unwrap();
    let read_only: String = sqlx::query_scalar("SHOW default_transaction_read_only")
        .fetch_one(&ctx.pool)
        .await
        .unwrap();
    assert_eq!(read_only, "on");
    let options = ReasonerOptions {
        persist: false,
        ..Default::default()
    };
    let (actual, default_failed) = match command {
        "build" => (
            serde_json::to_value(
                reason_build_with_options(&ctx, "Warden", options)
                    .await
                    .unwrap(),
            )
            .unwrap(),
            matches!(
                reason_build(&ctx, "Warden").await,
                Err(ReasonerError::Db(_))
            ),
        ),
        "patch-impact" => (
            serde_json::to_value(
                reason_patch_impact_with_options(&ctx, "Warden", options)
                    .await
                    .unwrap(),
            )
            .unwrap(),
            matches!(
                reason_patch_impact(&ctx, "Warden").await,
                Err(ReasonerError::Db(_))
            ),
        ),
        "backtest" => (
            serde_json::to_value(
                reason_backtest_with_options(&ctx, filter.clone(), options)
                    .await
                    .unwrap(),
            )
            .unwrap(),
            matches!(
                reason_backtest(&ctx, filter).await,
                Err(ReasonerError::Db(_))
            ),
        ),
        _ => unreachable!(),
    };
    assert_eq!(actual, expected);
    assert!(default_failed);
    for table in [
        "reasoner_builds",
        "reasoner_item_scores",
        "reasoner_patch_deltas",
        "reasoner_backtests",
    ] {
        let count: i64 = sqlx::query_scalar(&format!("SELECT count(*) FROM brain.{table}"))
            .fetch_one(&ctx.pool)
            .await
            .unwrap();
        assert_eq!(count, 0, "{table}");
    }
}

#[tokio::test]
#[ignore = "benötigt Wegwerf-DB reasoner_a_fix über REASONER_SCRATCH_DSN"]
async fn fix2_build_no_persist_on_read_only_connection() {
    fix2_read_only_facade("build").await;
}

#[tokio::test]
#[ignore = "benötigt Wegwerf-DB reasoner_a_fix über REASONER_SCRATCH_DSN"]
async fn fix2_patch_impact_no_persist_on_read_only_connection() {
    fix2_read_only_facade("patch-impact").await;
}

#[tokio::test]
#[ignore = "benötigt Wegwerf-DB reasoner_a_fix über REASONER_SCRATCH_DSN"]
async fn fix2_backtest_no_persist_on_read_only_connection() {
    fix2_read_only_facade("backtest").await;
}

#[tokio::test]
#[ignore = "benötigt Wegwerf-DB reasoner_a_fix über REASONER_SCRATCH_DSN"]
async fn fix_patch_tag_fallback_matches_sync_without_valid_patch() {
    let Some((_guard, ctx)) = scratch_context().await else {
        return;
    };
    assert_eq!(
        effective_context(&ctx).await.unwrap().config.patch_tag,
        dbrain_builds::latest_patch_tag(&ctx.pool).await.unwrap()
    );
    sqlx::query("UPDATE brain.patch_events SET patch_external_id=NULL, posted_at=NULL")
        .execute(&ctx.pool)
        .await
        .unwrap();
    assert_eq!(
        effective_context(&ctx).await.unwrap().config.patch_tag,
        "unknown"
    );
    assert_eq!(
        effective_context(&ctx).await.unwrap().config.patch_tag,
        dbrain_builds::latest_patch_tag(&ctx.pool).await.unwrap()
    );
    sqlx::raw_sql("UPDATE brain.patch_events SET patch_external_id='', posted_at='2026-09-12T00:30:00Z'; SET TIME ZONE 'Pacific/Honolulu';").execute(&ctx.pool).await.unwrap();
    assert_eq!(
        effective_context(&ctx).await.unwrap().config.patch_tag,
        "2026-09-12"
    );
    assert_eq!(
        effective_context(&ctx).await.unwrap().config.patch_tag,
        dbrain_builds::latest_patch_tag(&ctx.pool).await.unwrap()
    );
    let mut explicit = ctx.clone();
    explicit.config.patch_tag = "chosen-patch".into();
    assert_eq!(
        effective_context(&explicit).await.unwrap().config.patch_tag,
        "chosen-patch"
    );
    sqlx::query("DROP TABLE brain.patch_events")
        .execute(&ctx.pool)
        .await
        .unwrap();
    assert!(matches!(
        effective_context(&ctx).await,
        Err(ReasonerError::Db(_))
    ));
}

async fn scratch_context() -> Option<(tokio::sync::MutexGuard<'static, ()>, ReasonerCtx)> {
    let dsn = std::env::var("REASONER_SCRATCH_DSN").ok()?;
    let guard = SCRATCH_LOCK.lock().await;
    let options: sqlx::postgres::PgConnectOptions = dsn.parse().unwrap();
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .unwrap();
    let database: String = sqlx::query_scalar("SELECT current_database()")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(
        database, "reasoner_a_fix",
        "Fix-D-Tests benötigen die gesperrte Wegwerf-DB"
    );
    sqlx::raw_sql("DROP SCHEMA IF EXISTS brain CASCADE; DROP SCHEMA IF EXISTS tierlist CASCADE;")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::raw_sql(include_str!("fixtures/fix_d.sql"))
        .execute(&pool)
        .await
        .unwrap();
    for _ in 0..2 {
        sqlx::raw_sql(include_str!(
            "../../../../scripts/migrations/2026-09-12-reasoner.sql"
        ))
        .execute(&pool)
        .await
        .unwrap();
    }
    Some((
        guard,
        ReasonerCtx {
            pool,
            ai: None,
            config: ReasonerConfig {
                use_ai: false,
                ..ReasonerConfig::default()
            },
        },
    ))
}

#[tokio::test]
#[ignore = "benötigt Wegwerf-DB reasoner_a_fix über REASONER_SCRATCH_DSN"]
async fn fix_facade_build_persists_scores_and_keeps_patch_history() {
    let Some((_guard, ctx)) = scratch_context().await else {
        return;
    };
    let build = reason_build(&ctx, "Warden").await.unwrap();
    let stored: Option<Value> = sqlx::query_scalar(
        "SELECT build FROM brain.reasoner_builds WHERE hero_id=25 AND patch_tag='patch-1'",
    )
    .fetch_optional(&ctx.pool)
    .await
    .unwrap();
    assert_eq!(stored, Some(serde_json::to_value(&build).unwrap()));
    let used_ai: bool = sqlx::query_scalar("SELECT used_ai FROM brain.reasoner_builds")
        .fetch_one(&ctx.pool)
        .await
        .unwrap();
    assert!(!used_ai);
    let (mut hero, mut items, meta) =
        load_reasoning_inputs(&effective_context(&ctx).await.unwrap(), "Warden", None)
            .await
            .unwrap();
    let deltas = patch::compute_patch_delta(&hero, &load_patch_events(&ctx, 25).await.unwrap());
    patch::apply_patch_delta(&mut hero, &mut items, &deltas);
    let scored = item::score_items(&hero, &items, &meta.index, &deltas, &ctx.config);
    assert!(!scored.is_empty());
    let rows = sqlx::query("SELECT * FROM brain.reasoner_item_scores ORDER BY item_id")
        .fetch_all(&ctx.pool)
        .await
        .unwrap();
    assert_eq!(rows.len(), scored.len());
    for item in &scored {
        let row = rows
            .iter()
            .find(|row| row.get::<i64, _>("item_id") == item.item.item_id)
            .unwrap();
        for (field, value) in [
            ("combat_value", item.score.combat_value),
            ("per_slot_value", item.score.per_slot_value),
            ("per_soul_value", item.score.per_soul_value),
            ("purchase_bonus", item.score.purchase_bonus_value),
            ("condition_factor", item.score.condition_factor),
            ("active_value", item.score.active_value),
            ("passive_value", item.score.passive_value),
            ("meta_support", item.score.meta_support),
            ("total", item.score.total),
        ] {
            assert_eq!(row.get::<f64, _>(field), value, "{field}");
        }
        assert_eq!(
            row.get::<String, _>("confidence"),
            format!("{:?}", item.confidence)
        );
        assert_eq!(
            row.get::<String, _>("buy_phase"),
            format!("{:?}", item.buy_phase)
        );
    }
    reason_build(&ctx, "Warden").await.unwrap();
    sqlx::query("UPDATE brain.reasoner_item_scores SET total=-999")
        .execute(&ctx.pool)
        .await
        .unwrap();
    reason_build(&ctx, "Warden").await.unwrap();
    let total: f64 =
        sqlx::query_scalar("SELECT total FROM brain.reasoner_item_scores WHERE item_id=100")
            .fetch_one(&ctx.pool)
            .await
            .unwrap();
    assert_eq!(total, scored[0].score.total);
    sqlx::query("UPDATE brain.patch_events SET patch_external_id='patch-2'")
        .execute(&ctx.pool)
        .await
        .unwrap();
    reason_build(&ctx, "Warden").await.unwrap();
    for table in ["reasoner_builds", "reasoner_item_scores"] {
        let count: i64 = sqlx::query_scalar(&format!(
            "SELECT count(DISTINCT patch_tag) FROM brain.{table}"
        ))
        .fetch_one(&ctx.pool)
        .await
        .unwrap();
        assert_eq!(count, 2);
        let count: i64 = sqlx::query_scalar(&format!("SELECT count(*) FROM brain.{table}"))
            .fetch_one(&ctx.pool)
            .await
            .unwrap();
        assert_eq!(count, 2);
    }
}

#[tokio::test]
#[ignore = "benötigt Wegwerf-DB reasoner_a_fix über REASONER_SCRATCH_DSN"]
async fn fix_facade_patch_impact_and_backtest_persist_idempotently() {
    let Some((_guard, ctx)) = scratch_context().await else {
        return;
    };
    let impact = reason_patch_impact(&ctx, "Warden").await.unwrap();
    assert!(!impact.deltas.is_empty());
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM brain.reasoner_patch_deltas")
        .fetch_one(&ctx.pool)
        .await
        .unwrap();
    assert_eq!(count as usize, impact.deltas.len());
    let row = sqlx::query("SELECT * FROM brain.reasoner_patch_deltas")
        .fetch_one(&ctx.pool)
        .await
        .unwrap();
    assert_eq!(row.get::<String, _>("target_kind"), "hero");
    assert_eq!(row.get::<i64, _>("target_id"), 25);
    assert_eq!(row.get::<i16, _>("sign"), i16::from(impact.deltas[0].sign));
    assert_eq!(row.get::<f64, _>("magnitude"), impact.deltas[0].magnitude);
    assert_eq!(row.get::<String, _>("note"), impact.deltas[0].note);
    reason_patch_impact(&ctx, "Warden").await.unwrap();
    let filter = BacktestFilter {
        hero: Some("Warden".into()),
        patch_tag: None,
    };
    let report = reason_backtest(&ctx, filter.clone()).await.unwrap();
    let before: Vec<Value> =
        sqlx::query_scalar("SELECT to_jsonb(r) FROM brain.reasoner_backtests r ORDER BY author")
            .fetch_all(&ctx.pool)
            .await
            .unwrap();
    assert!(!before.is_empty());
    assert_eq!(before[0]["detail"]["hero_name"], report.heroes[0].hero_name);
    assert!(before[0]["order_proximity"].is_null());
    reason_backtest(&ctx, filter).await.unwrap();
    let after: Vec<Value> =
        sqlx::query_scalar("SELECT to_jsonb(r) FROM brain.reasoner_backtests r ORDER BY author")
            .fetch_all(&ctx.pool)
            .await
            .unwrap();
    assert_eq!(before, after);
    let count_after: i64 = sqlx::query_scalar("SELECT count(*) FROM brain.reasoner_patch_deltas")
        .fetch_one(&ctx.pool)
        .await
        .unwrap();
    assert_eq!(count, count_after);
    let filtered = reason_backtest(
        &ctx,
        BacktestFilter {
            hero: Some("Warden".into()),
            patch_tag: Some("explicit-patch".into()),
        },
    )
    .await
    .unwrap();
    assert!(filtered.heroes[0].per_author.is_empty());
    let tags: Vec<String> = sqlx::query_scalar(
        "SELECT DISTINCT patch_tag FROM brain.reasoner_backtests ORDER BY patch_tag",
    )
    .fetch_all(&ctx.pool)
    .await
    .unwrap();
    assert_eq!(tags, ["explicit-patch", "patch-1"]);
    let detail: Value = sqlx::query_scalar(
        "SELECT detail FROM brain.reasoner_backtests WHERE patch_tag='explicit-patch'",
    )
    .fetch_one(&ctx.pool)
    .await
    .unwrap();
    assert_eq!(detail, serde_json::to_value(&filtered.heroes[0]).unwrap());
}

#[tokio::test]
#[ignore = "benötigt Wegwerf-DB reasoner_a_fix über REASONER_SCRATCH_DSN"]
async fn fix_build_write_failure_rolls_back_and_reaches_caller() {
    let Some((_guard, ctx)) = scratch_context().await else {
        return;
    };
    sqlx::query(
        "ALTER TABLE brain.reasoner_item_scores ADD CONSTRAINT reject_score CHECK (total < -1e100)",
    )
    .execute(&ctx.pool)
    .await
    .unwrap();
    assert!(matches!(
        reason_build(&ctx, "Warden").await,
        Err(ReasonerError::Db(_))
    ));
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM brain.reasoner_builds")
        .fetch_one(&ctx.pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn fix_fallback_skill_order_keeps_unlocks_and_upgrade_costs() {
    let order =
        fallback_ability_order(&[json!(10), json!(10), json!(20), json!(10), json!(10)]).unwrap();
    assert_eq!(
        order
            .iter()
            .map(|step| (step.ability_id, step.currency_type, step.delta))
            .collect::<Vec<_>>(),
        [
            (10, 2, -1),
            (10, 1, -1),
            (20, 2, -1),
            (10, 1, -2),
            (10, 1, -5)
        ]
    );
}

#[test]
fn fix_fallback_skill_order_preserves_explicit_steps() {
    let steps = vec![
        json!({"ability_id":10,"currency_type":2,"delta":-1}),
        json!({"ability_id":10,"currency_type":1,"delta":-2}),
    ];
    assert_eq!(
        serde_json::to_value(fallback_ability_order(&steps).unwrap()).unwrap(),
        json!(steps)
    );
}

#[test]
fn fix_fallback_skill_order_rejects_invalid_or_excess_steps() {
    assert!(fallback_ability_order(&vec![json!(10); 5]).is_err());
    assert!(fallback_ability_order(&[json!({"ability_id":10})]).is_err());
    assert!(fallback_ability_order(&[json!(0)]).is_err());
}

#[test]
fn fix_migration_retains_all_scores_and_idempotent_backtests() {
    let ddl = include_str!("../../../../scripts/migrations/2026-09-12-reasoner.sql");
    let spec = include_str!("../../../../.tasks/2026-09-12-build-reasoner/ARCHITEKTUR.md");
    assert!(spec.contains("order_proximity double precision,"));
    for field in ["per_soul_value", "active_value", "passive_value"] {
        assert!(ddl.contains(field), "Score-Komponente fehlt: {field}");
    }
    assert!(ddl.contains("(hero_id, patch_tag, author)"));
    assert!(!ddl.contains("order_proximity double precision NOT NULL"));
    for (sql, table, key) in [
        (UPSERT_BUILD, "reasoner_builds", "hero_id, patch_tag"),
        (
            UPSERT_SCORE,
            "reasoner_item_scores",
            "hero_id, patch_tag, item_id",
        ),
        (
            UPSERT_DELTA,
            "reasoner_patch_deltas",
            "hero_id, patch_tag, target_kind, target_id, mechanic",
        ),
        (
            UPSERT_BACKTEST,
            "reasoner_backtests",
            "hero_id, patch_tag, author",
        ),
    ] {
        assert!(sql.starts_with(&format!("INSERT INTO brain.{table} ")));
        assert!(sql.contains(&format!("ON CONFLICT ({key}) DO UPDATE SET")));
        assert!(!sql.contains("DELETE"));
    }
    for field in [
        "combat_value",
        "per_slot_value",
        "per_soul_value",
        "purchase_bonus",
        "condition_factor",
        "active_value",
        "passive_value",
        "meta_support",
        "total",
        "confidence",
        "buy_phase",
    ] {
        assert!(UPSERT_SCORE.contains(&format!("{field}=EXCLUDED.{field}")));
    }
}
