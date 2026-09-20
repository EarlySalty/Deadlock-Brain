use super::*;
use serde_json::json;

pub(crate) static SCRATCH_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

/// Jeder Lauf darf eine eigene, reservierte Testdatenbank verwenden. Die
/// Prüfung geschieht vor jeder destruktiven Fixture-Initialisierung.
pub(crate) fn assert_scratch_database(database: &str) {
    let isolated = database
        .strip_prefix("reasoner_a_fix_")
        .is_some_and(|suffix| {
            !suffix.is_empty()
                && suffix
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
        });
    assert!(
        database == "reasoner_a_fix" || isolated,
        "Reasoner-Tests benötigen eine reservierte Wegwerf-Datenbank"
    );
}

#[test]
fn isolated_scratch_names_are_accepted() {
    assert_scratch_database("reasoner_a_fix");
    assert_scratch_database("reasoner_a_fix_20260921_ci");
}

#[test]
fn production_and_ambiguous_database_names_are_rejected() {
    for name in [
        "deadlock",
        "postgres",
        "reasoner_a_fix_",
        "xreasoner_a_fix",
        "reasoner_a_fix_prod-name",
        "reasoner_a_fix_ä",
    ] {
        assert!(std::panic::catch_unwind(|| assert_scratch_database(name)).is_err());
    }
}

#[tokio::test]
#[ignore = "benötigt Wegwerf-DB reasoner_a_fix über REASONER_SCRATCH_DSN"]
async fn fix_e_loader_keeps_exact_field_dates_and_current_hero_scaling() {
    let Some((_guard, ctx)) = scratch_context().await else {
        panic!("Scratch-DSN fehlt")
    };
    sqlx::raw_sql("UPDATE brain.entity_snapshots SET fetched_at=to_timestamp(100),payload=payload || '{\"scaling_stats\":{\"EFireRate\":{\"scaling_stat\":\"ETechPower\",\"scale\":0.25},\"ERoundsPerSecond\":{\"scaling_stat\":\"ETechPower\",\"scale\":0.01}}}'::jsonb WHERE entity_type='hero';
        UPDATE brain.entity_snapshots SET fetched_at=to_timestamp(120),payload=payload || '{\"class_name\":\"fixture_item\",\"properties\":{\"WeaponDamage\":{\"value\":20}}}'::jsonb WHERE id=2;
        INSERT INTO brain.entity_snapshots VALUES (3,'deadlock_data','item_card','Fixture Item','fixture_item','{\"Key\":\"fixture_item\",\"Info2\":{\"Cooldown\":10}}',to_timestamp(200));
        CREATE TABLE brain.hero_stat_profiles (id bigint,entity_id bigint);
        CREATE TABLE brain.hero_stat_values (profile_id bigint,entity_id bigint,stat_key text,numeric_value float8);
        INSERT INTO brain.hero_stat_profiles VALUES (1,25);
        INSERT INTO brain.hero_stat_values VALUES (1,25,'spirit_scaling.EFireRate',999);")
        .execute(&ctx.pool).await.unwrap();
    let (hero, hero_snapshots) = data::load_hero_model_with_snapshots(&ctx, "Warden")
        .await
        .unwrap();
    assert_eq!(hero.scaling.len(), 2);
    assert_eq!(hero.scaling[0].per_spirit, Some(0.25));
    assert_eq!(
        hero_snapshots[0].fields["scaling.EFireRate"].fetched_at,
        Some(100.0)
    );
    let (items, snapshots) = data::load_item_models_with_snapshots(&ctx).await.unwrap();
    let item = items.iter().find(|item| item.item_id == 100).unwrap();
    let snapshot = snapshots
        .iter()
        .find(|snapshot| snapshot.name == "Fixture Item")
        .unwrap();
    assert_eq!(item.proc_cooldown, Some(10.0));
    assert_eq!(
        snapshot.fields["properties.WeaponDamage"].fetched_at,
        Some(120.0)
    );
    assert_eq!(snapshot.fields["proc_cooldown"].fetched_at, Some(200.0));
    assert_eq!(
        snapshot.fields["proc_cooldown"].source,
        "deadlock_data/item_card"
    );
}

#[tokio::test]
#[ignore = "benötigt lesenden Central-Pool über DEADLOCK_CENTRAL_DSN"]
async fn fix_e_live_warden_evidence() {
    let dsn = std::env::var("DEADLOCK_CENTRAL_DSN").expect("Central-DSN fehlt");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .after_connect(|connection, _| {
            Box::pin(async move {
                sqlx::query("SET default_transaction_read_only=on")
                    .execute(connection)
                    .await?;
                Ok(())
            })
        })
        .connect(&dsn)
        .await
        .expect("Central-Verbindung fehlgeschlagen");
    let read_only: String = sqlx::query_scalar("SHOW default_transaction_read_only")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(read_only, "on");
    let ctx = effective_context(&ReasonerCtx {
        pool,
        ai: None,
        config: ReasonerConfig {
            use_ai: false,
            ..Default::default()
        },
    })
    .await
    .unwrap();
    let seed = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../.tasks/2026-09-12-build-reasoner/referenz");
    let (mut hero, mut items, meta, snapshots) = load_reasoning_inputs(&ctx, "Warden", Some(&seed))
        .await
        .unwrap();
    assert_eq!(
        hero.scaling
            .iter()
            .find(|stat| stat.stat == "EFireRate")
            .unwrap()
            .per_spirit,
        Some(0.25)
    );
    assert_eq!(
        hero.scaling
            .iter()
            .find(|stat| stat.stat == "ERoundsPerSecond")
            .unwrap()
            .per_spirit,
        Some(0.01)
    );
    let before = item::score_items(&hero, &items, &meta.index, &[], &ctx.config);
    let events = data::load_patch_events_for_snapshots(&ctx, 25, &snapshots)
        .await
        .unwrap();
    let mut deltas = patch::compute_patch_delta_with_snapshots(&hero, &events, &snapshots);
    patch::apply_scored_patch_delta(&mut hero, &mut items, &mut deltas, &meta.index, &ctx.config);
    assert_eq!(
        hero.scaling
            .iter()
            .find(|stat| stat.stat == "EFireRate")
            .unwrap()
            .per_spirit,
        Some(0.21)
    );
    assert_eq!(
        hero.standard_level_up_upgrades["MODIFIER_VALUE_BASE_BULLET_DAMAGE_FROM_LEVEL"],
        0.25
    );
    let flask = hero
        .abilities
        .iter()
        .find(|ability| ability.ability_id == 2656490109)
        .unwrap();
    assert_eq!(flask.properties["ForwardVelocity"], 560.0);
    let willpower = hero
        .abilities
        .iter()
        .find(|ability| ability.ability_id == 2751689917)
        .unwrap();
    let willpower_t3 = willpower.upgrades[2]["property_upgrades"]
        .as_array()
        .unwrap();
    assert_eq!(
        willpower_t3
            .iter()
            .find(|property| property["name"] == "CombatBarrier")
            .unwrap()["bonus"]
            .as_f64(),
        Some(2.1)
    );
    assert_eq!(
        willpower_t3
            .iter()
            .find(|property| property["name"] == "StatusResistancePercent")
            .unwrap()["bonus"]
            .as_f64(),
        Some(30.0)
    );
    let magnum = items
        .iter()
        .find(|item| item.name == "Mercurial Magnum")
        .unwrap();
    assert_eq!(magnum.properties["BulletsBonusMagicDamage"], 20.0);
    assert_eq!(
        magnum.property_spirit_scaling["BulletsBonusMagicDamage"],
        0.38
    );
    let overflow = items
        .iter()
        .find(|item| item.name == "Spiritual Overflow")
        .unwrap();
    assert_eq!(overflow.properties["BonusSpirit"], 30.0);
    assert_eq!(overflow.properties["BonusFireRate"], 25.0);
    assert!((overflow.properties["BuildUpPerShot"] - 0.4875).abs() < 1e-12);
    let mut after = item::score_items(&hero, &items, &meta.index, &[], &ctx.config);
    finish_scores(&mut after);
    let build = reason_build_with_options(
        &ctx,
        "Warden",
        ReasonerOptions {
            seed_path: Some(&seed),
            persist: false,
        },
    )
    .await
    .unwrap();
    assert!(build.core.iter().all(|entry| {
        after
            .iter()
            .find(|item| item.item.item_id == entry.item_id)
            .unwrap()
            .score
            .total
            > 0.0
    }));
    assert!(build
        .situations
        .iter()
        .find(|block| block.label == "Optional")
        .is_none_or(|block| block.items.len() <= 12));
    let mut without_scaling = hero.clone();
    without_scaling
        .scaling
        .retain(|stat| stat.stat != "EFireRate" && stat.stat != "ERoundsPerSecond");
    let mut table = Vec::new();
    let empty_meta = MetaIndex {
        by_item: Default::default(),
        sample_ok: Default::default(),
    };
    for scored in &after {
        let base = before
            .iter()
            .find(|item| item.item.item_id == scored.item.item_id)
            .unwrap();
        let old = item::score_item(
            &scored.item,
            &without_scaling,
            &meta.index,
            &[],
            &ctx.config,
        );
        let fire = item::spirit_fire_rate_value(&scored.item, &hero, &ctx.config);
        let baseline = item::score_item(
            &scored.item,
            &without_scaling,
            &empty_meta,
            &[],
            &ctx.config,
        );
        if ["Mercurial Magnum", "Boundless Spirit", "Improved Spirit"]
            .contains(&scored.item.name.as_str())
        {
            assert!(fire.rounds_per_second > 0.0 && fire.weapon_dps_in_score > 0.0);
        }
        assert!(scored.score.total > 0.0 || scored.confidence != Confidence::High);
        table.push(json!({"name":scored.item.name,"item_id":scored.item.item_id,"rank":table.len()+1,"core":build.core.iter().any(|entry| entry.item_id==scored.item.item_id),"phase":scored.buy_phase,"confidence":scored.confidence,"total":scored.score.total,"without_patch":base.score.total,"without_fire_rate":old.score.total,"review_b_baseline":baseline.score.total,"meta_support":scored.score.meta_support,"spirit_fire_rate":fire}));
    }
    let seeds = meta::load_seed_builds(&seed, &items).unwrap();
    let row: Value = sqlx::query_scalar("SELECT to_jsonb(hbs) FROM tierlist.hero_build_sources hbs WHERE hero_id=25 AND hero_build_id=779996 ORDER BY version DESC LIMIT 1").fetch_one(&ctx.pool).await.unwrap();
    let reference = data::author_build(&row);
    let mut core_reference = reference.clone();
    core_reference.core_item_ids = row["details"]["modCategories"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|category| {
            category["name"]
                .as_str()
                .unwrap_or_default()
                .to_ascii_lowercase()
                .contains("core")
        })
        .flat_map(|category| category["mods"].as_array().unwrap())
        .filter_map(|item| item["abilityId"].as_i64())
        .collect();
    let e_build: BuildObject = serde_json::from_str(include_str!(
        "../../../../.tasks/2026-09-12-build-reasoner/WARDEN-E-BUILD.json"
    ))
    .unwrap();
    let mut old_layout = meta.core_layouts.for_hero(25).clone();
    for band in old_layout.bands.values_mut() {
        band.target = band.median.round() as usize;
    }
    old_layout.flex_slots = old_layout.total_target().saturating_sub(12);
    let old_build =
        composer::compose_build_with_layout(&hero, &after, &deltas, &ctx.config, &old_layout)
            .unwrap();
    let comparisons_f = [
        ("E", &e_build),
        ("F vor Fix", &old_build),
        ("F Fixrunde 1", &build),
    ]
    .into_iter()
    .map(|(stage, build)| {
        json!({"stage":stage,"core":build.core,
            "seed":backtest::backtest_metrics(build,&seeds[0]),
            "reference_core":backtest::backtest_metrics(build,&core_reference),
            "reference_loader":backtest::backtest_metrics(build,&reference)})
    })
    .collect::<Vec<_>>();
    let item_evidence = after
        .iter()
        .map(|item| {
            json!({"id":item.item.item_id,"name":item.item.name,
        "tier":item.item.tier,"slot":item.item.slot,"total":item.score.total,
        "per_soul":item.score.per_soul_value,"shopable":item.item.shopable,"disabled":item.item.disabled})
        })
        .collect::<Vec<_>>();
    println!(
        "FIX_F_EVIDENCE={}",
        json!({"layout":meta.core_layouts.for_hero(25),"comparisons":comparisons_f,
        "reference_core_ids":core_reference.core_item_ids,"reference_loader_ids":reference.core_item_ids,
        "seed_ids":seeds[0].core_item_ids,"items":item_evidence,"build":build,"read_only":read_only})
    );
    let comparisons = seeds.iter().map(|author| json!({"source":"Seed","author":author.author,"metrics":backtest::backtest_metrics(&build,author),"jaccard":backtest::core_jaccard(&build,author)}))
        .chain(std::iter::once(json!({"source":"Build 779996","version":reference.version,"metrics":backtest::backtest_metrics(&build,&reference),"jaccard":backtest::core_jaccard(&build,&reference)}))).collect::<Vec<_>>();
    println!(
        "FIX_E_EVIDENCE={}",
        json!({"read_only":read_only,"damage_plan":hero.damage_plan,"weapon":hero.weapon,"scaling":hero.scaling,"events":events.len(),"deltas":deltas.len(),"applied":deltas.iter().filter(|delta|delta.application.is_some()).collect::<Vec<_>>(),"table":table,"comparisons":comparisons,"build_779996_version":reference.version})
    );
}

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
    assert_scratch_database(&database);
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
    let effective = effective_context(&ctx).await.unwrap();
    let (hero, items, meta, snapshots) = load_reasoning_inputs(&effective, "Warden", None)
        .await
        .unwrap();
    let events = data::load_patch_events_for_snapshots(&ctx, 25, &snapshots)
        .await
        .unwrap();
    // Der Persistenzvertrag gilt für die tatsächlich geplanten Familienscores,
    // nicht für das globale Ranking vor der Familienkonditionierung.
    let planned = plan_build(&hero, &items, &meta, &events, &snapshots, &effective.config).unwrap();
    assert!(build
        .family
        .as_ref()
        .is_some_and(|family| family.eligible_for_planning));
    let scored = planned.scored;
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
    let expected = scored.iter().find(|item| item.item.item_id == 100).unwrap();
    assert_eq!(total, expected.score.total);
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
        let rows_per_patch = if table == "reasoner_item_scores" {
            scored.len() as i64
        } else {
            1
        };
        assert_eq!(count, 2 * rows_per_patch);
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
