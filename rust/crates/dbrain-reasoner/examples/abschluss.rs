use std::{collections::BTreeMap, path::Path};

use dbrain_reasoner::{meta, *};
use serde_json::json;
use sqlx::Row;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let pool = deadlock_brain_core::pg::pg_pool_read_only().await?;
    let read_only: String = sqlx::query_scalar("SHOW default_transaction_read_only")
        .fetch_one(&pool)
        .await?;
    assert_eq!(read_only, "on");
    let output = std::env::args().nth(1).ok_or("Ausgabepfad fehlt")?;
    if std::env::args().nth(2).as_deref() == Some("catalog") {
        let catalog: serde_json::Value = sqlx::query_scalar("SELECT jsonb_build_object('measured_at', now(), 'authors', (SELECT jsonb_agg(jsonb_build_object('author_account_id', author_account_id, 'last_checked_at', last_checked_at, 'last_checked_status', last_checked_status) ORDER BY author_account_id) FROM tierlist.watched_build_authors), 'build_rows', (SELECT count(*) FROM tierlist.hero_build_sources), 'warden_779996_versions', (SELECT jsonb_agg(version ORDER BY version) FROM tierlist.hero_build_sources WHERE hero_build_id=779996))").fetch_one(&pool).await?;
        std::fs::write(output, serde_json::to_vec_pretty(&catalog)?)?;
        return Ok(());
    }
    let mut ctx = ReasonerCtx {
        pool: pool.clone(),
        ai: None,
        config: ReasonerConfig {
            use_ai: false,
            ..Default::default()
        },
    };
    let options = ReasonerOptions {
        seed_path: None,
        persist: false,
    };
    let first = reason_build_with_options(&ctx, "Warden", options).await?;
    ctx.config.patch_tag = first.patch_tag.clone();
    let items = load_item_models(&ctx).await?;
    let tiers = items
        .iter()
        .map(|item| (item.item_id, item.tier))
        .collect::<BTreeMap<_, _>>();
    let source_rows = sqlx::query("SELECT DISTINCT ON (hero_build_id) hero_id, hero_build_id, version, details FROM tierlist.hero_build_sources ORDER BY hero_build_id, version DESC NULLS LAST, fetched_at DESC NULLS LAST").fetch_all(&pool).await?;
    let sources = source_rows
        .iter()
        .map(|row| meta::AuthorBuildLayoutSource {
            hero_id: row.get("hero_id"),
            build_id: row.get("hero_build_id"),
            version: row.get("version"),
            details: row.get("details"),
        })
        .collect::<Vec<_>>();
    let layouts = meta::derive_core_layouts(&sources, &tiers);
    let heroes = sqlx::query(
        "SELECT hero_id, name FROM brain.hero_catalog ORDER BY (hero_id=25) DESC, hero_id",
    )
    .fetch_all(&pool)
    .await?;
    let mut reports = Vec::new();
    for row in heroes {
        let name: String = row.get("name");
        let hero_id: i64 = row.get("hero_id");
        if std::env::args().nth(2).as_deref() == Some("warden") && hero_id != 25 {
            continue;
        }
        let build = reason_build_with_options(&ctx, &name, options).await?;
        let authors = load_author_builds(&ctx, hero_id).await?;
        let report = backtest::backtest_hero_with_build(hero_id, &name, &build, &authors);
        let synergy_rows = load_synergies(&ctx, hero_id).await?;
        let pair_support = meta::combination_support(
            &synergy_rows,
            &load_meta_rows(&ctx, hero_id).await?,
            &ctx.config,
        );
        let pair_items = build
            .core
            .iter()
            .filter(|item| {
                item.sources
                    .iter()
                    .any(|source| source.detail.starts_with("Zusammen mit"))
            })
            .count();
        let layout = layouts.for_hero(hero_id);
        eprintln!(
            "{}: {} Käufe, {} Referenzen, Recall {:.4}, Jaccard {:.4}, {} Paarbelege",
            name,
            build.core.len(),
            authors.len(),
            report.aggregate.reference_recall,
            report.aggregate.core_jaccard,
            pair_items
        );
        let mut entry = json!({"hero_id":hero_id,"name":name,"build":build,"layout":layout,"backtest":report,"synergy_rows":synergy_rows.len(),"qualified_pairs":pair_support.len(),"selected_pair_evidence":pair_items});
        if hero_id == 25 {
            let model = load_hero_model(&ctx, &name).await?;
            let reference = sources
                .iter()
                .find(|source| source.build_id == 779996)
                .expect("Referenz 779996 fehlt");
            let ids = meta::core_item_ids(&reference.details);
            let author = AuthorBuild {
                author: "Build 779996".into(),
                version: reference.version,
                published_at: None,
                last_updated_at: None,
                patch_tag: None,
                core_item_ids: ids.clone(),
                buy_order: ids.clone(),
            };
            let seed = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../../.tasks/2026-09-12-build-reasoner/referenz");
            let seed_authors = meta::load_seed_builds_for_hero(&seed, &items, &name)?;
            entry["reference_779996"] = json!({"version":reference.version,"ids":ids,"metrics":backtest::backtest_metrics(&build,&author)});
            let weapon_ids = items
                .iter()
                .filter(|item| {
                    item.slot == SlotType::Weapon && author.core_item_ids.contains(&item.item_id)
                })
                .map(|item| item.item_id)
                .collect::<std::collections::BTreeSet<_>>();
            let weapon_hits = build
                .core
                .iter()
                .filter(|item| weapon_ids.contains(&item.item_id))
                .map(|item| json!({"id":item.item_id,"name":item.name}))
                .collect::<Vec<_>>();
            entry["weapon_reference"] = json!({"count":weapon_ids.len(),"ids":weapon_ids,"hit_count":weapon_hits.len(),"hits":weapon_hits});
            eprintln!(
                "Warden 779996: {}; Kern: {}",
                entry["reference_779996"]["metrics"],
                build
                    .core
                    .iter()
                    .map(|item| item.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            entry["seed"] = json!(seed_authors.iter().map(|author|json!({"ids":author.core_item_ids,"metrics":backtest::backtest_metrics(&build,author)})).collect::<Vec<_>>());
            entry["model"] = json!(model);
            let meta_rows = load_meta_rows(&ctx, hero_id).await?;
            let index = meta::build_meta_index(&meta_rows, &authors, &[], &ctx.config);
            entry["item_scores"] =
                json!(item::score_items(&model, &items, &index, &[], &ctx.config));
            entry["spirit_fire_rate"] = json!(items
                .iter()
                .filter_map(|item| {
                    let value = item::spirit_fire_rate_value(item, &model, &ctx.config);
                    (value.weapon_dps_in_score > 0.0)
                        .then(|| json!({"item_id":item.item_id,"name":item.name,"effect":value}))
                })
                .collect::<Vec<_>>());
            entry["patch_impact"] = serde_json::to_value(
                reason_patch_impact_with_options(&ctx, &name, options).await?,
            )?;
        }
        reports.push(entry);
    }
    std::fs::write(
        output,
        serde_json::to_vec_pretty(
            &json!({"read_only":read_only,"patch_tag":ctx.config.patch_tag,"reference_contract":"Aktuelle Version je Autoren-Build; vollständige Pflichtkaufkurve; separate Warden-Seed-Messung. Autoren dienen zugleich als Eingangssignal: Übereinstimmung, keine unabhängige Meta-Prognose.","heroes":reports}),
        )?,
    )?;
    pool.close().await;
    Ok(())
}
