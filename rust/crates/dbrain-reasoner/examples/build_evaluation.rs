use dbrain_reasoner::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::postgres::PgPoolOptions;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

type Error = Box<dyn std::error::Error>;

#[derive(Serialize, Deserialize)]
struct FrozenHero {
    hero: HeroModel,
    items: Vec<ItemModel>,
    snapshots: Vec<PatchSnapshot>,
    events: Vec<Value>,
    rows: Vec<MetaRow>,
    synergies: Vec<Value>,
    claims: Vec<Value>,
    layout: CoreLayoutStats,
    ability_orders: BTreeMap<i64, Vec<AbilityStep>>,
    live_baseline: BuildObject,
}

#[derive(Serialize, Deserialize)]
struct Frozen {
    format_version: u32,
    baseline_revision: String,
    measured_at: String,
    config: ReasonerConfig,
    sources: Vec<Value>,
    raw_snapshots: Vec<Value>,
    item_tiers: BTreeMap<i64, i64>,
    heroes: Vec<FrozenHero>,
}

fn author(row: &Value) -> std::result::Result<String, Error> {
    row["author_account_id"]
        .as_i64()
        .map(|id| id.to_string())
        .ok_or_else(|| "Autoren-ID fehlt".into())
}

fn reference(row: &Value) -> std::result::Result<AuthorBuild, Error> {
    let ids = meta::core_item_ids(&row["details"]);
    Ok(AuthorBuild {
        author: author(row)?,
        version: row["version"].as_i64().ok_or("Version fehlt")?,
        published_at: row["published_at"].as_i64(),
        last_updated_at: row["last_updated_at"].as_i64(),
        patch_tag: row["patch_tag"].as_str().map(str::to_owned),
        core_item_ids: ids.clone(),
        buy_order: ids,
    })
}

fn compose(
    frozen: &Frozen,
    input: &FrozenHero,
    excluded: &BTreeSet<String>,
    holdout: bool,
    ablation: &str,
) -> std::result::Result<BuildObject, Error> {
    let sources = frozen
        .sources
        .iter()
        .filter_map(|row| match author(row) {
            Ok(id) if !excluded.contains(&id) => Some(Ok(row)),
            Ok(_) => None,
            Err(e) => Some(Err(e)),
        })
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let own = sources
        .iter()
        .copied()
        .filter(|row| row["hero_id"].as_i64() == Some(input.hero.hero_id))
        .collect::<Vec<_>>();
    let authors = own
        .iter()
        .map(|row| reference(row))
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let layouts = sources
        .iter()
        .map(|row| {
            Ok(meta::AuthorBuildLayoutSource {
                hero_id: row["hero_id"].as_i64().ok_or("Helden-ID fehlt")?,
                build_id: row["hero_build_id"].as_i64().ok_or("Build-ID fehlt")?,
                version: row["version"].as_i64().ok_or("Version fehlt")?,
                details: row["details"].clone(),
            })
        })
        .collect::<std::result::Result<Vec<_>, Error>>()?;
    let mut core_layouts = meta::derive_core_layouts(&layouts, &frozen.item_tiers);
    let mut layout = core_layouts.for_hero(input.hero.hero_id).clone();
    layout.flex_slots = input.layout.flex_slots;
    if own.is_empty() {
        layout.source_builds = 0;
    }
    core_layouts.by_hero.insert(input.hero.hero_id, layout);
    let mut cfg = frozen.config.clone();
    if ablation == "window20" {
        cfg.combat_window_seconds = 20.0;
    }
    if ablation == "window60" {
        cfg.combat_window_seconds = 60.0;
    }
    let empty_rows = Vec::new();
    let rows = if ablation == "no_statistical_scores" {
        &empty_rows
    } else {
        &input.rows
    };
    let claims = if holdout || ablation == "no_statistical_scores" {
        &[][..]
    } else {
        &input.claims
    };
    let index = meta::build_meta_index(
        rows,
        if ablation == "no_statistical_scores" {
            &[]
        } else {
            &authors
        },
        claims,
        &cfg,
    );
    let combinations = if ablation == "no_statistical_scores" || ablation == "no_pairs" {
        BTreeMap::new()
    } else {
        meta::combination_support(&input.synergies, rows, &cfg)
    };
    let author_builds = own
        .iter()
        .map(|row| meta::AuthorBuildSource {
            hero_id: input.hero.hero_id,
            author: row["author"].as_str().unwrap_or("unbekannt").into(),
            weight: row["weight"].as_f64().unwrap_or_default(),
            details: row["details"].clone(),
        })
        .collect();
    let context = meta::MetaIndexWithSources {
        index,
        author_builds,
        hero_ability_orders: if holdout {
            BTreeMap::new()
        } else {
            input.ability_orders.clone()
        },
        core_layouts,
        combinations,
    };
    let mut hero = input.hero.clone();
    let mut items = input.items.clone();
    let mut deltas =
        patch::compute_patch_delta_with_snapshots(&hero, &input.events, &input.snapshots);
    patch::apply_scored_patch_delta(&mut hero, &mut items, &mut deltas, &context.index, &cfg);
    let mut scores = item::score_items(&hero, &items, &context.index, &[], &cfg);
    for scored in &mut scores {
        if !scored.score.total.is_finite() || scored.score.total <= 0.0 {
            scored.confidence = Confidence::Low;
        }
    }
    let mut build =
        composer::compose_build_with_sources(&hero, &scores, &deltas, &cfg, &[], &context);
    if own.is_empty() {
        build.confidence = Confidence::Low;
        build.rationale.push_str(&format!(" Für diesen Helden fehlen Builds aktiver beobachteter Autoren. Die Kaufkurve ist ein Behelf aus {} beobachteten Builds anderer Helden; ein eigener Autorenvergleich ist nicht möglich.", context.core_layouts.overall.source_builds));
    }
    Ok(build)
}

fn measure(
    input: &FrozenHero,
    build: &BuildObject,
    row: &Value,
) -> std::result::Result<Value, Error> {
    let reference = reference(row)?;
    let weapons = input
        .items
        .iter()
        .filter(|item| {
            item.slot == SlotType::Weapon && reference.core_item_ids.contains(&item.item_id)
        })
        .map(|item| item.item_id)
        .collect::<BTreeSet<_>>();
    let hits = build
        .core
        .iter()
        .filter(|item| weapons.contains(&item.item_id))
        .map(|item| json!({"id":item.item_id,"name":item.name}))
        .collect::<Vec<_>>();
    Ok(
        json!({"hero_id":input.hero.hero_id,"hero_name":input.hero.name,"build_id":row["hero_build_id"],"version":reference.version,"author_id":author(row)?,"reference_count":reference.core_item_ids.len(),"reference_weapon_count":weapons.len(),"weapon_hits":hits,"weapon_hit_count":hits.len(),"metrics":backtest::backtest_metrics(build,&reference)}),
    )
}

async fn freeze(output: &Path) -> std::result::Result<(), Error> {
    if output.exists() {
        return Err("Ausgabedatei existiert bereits".into());
    }
    let access = deadlock_brain_core::pg::pg_pool_read_only().await?;
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect_with((*access.connect_options()).clone())
        .await?;
    access.close().await;
    sqlx::query("BEGIN ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&pool)
        .await?;
    let read_only: String = sqlx::query_scalar("SHOW transaction_read_only")
        .fetch_one(&pool)
        .await?;
    if read_only != "on" {
        return Err("Messung ist nicht lesend".into());
    }
    let mut ctx = ReasonerCtx {
        pool: pool.clone(),
        ai: None,
        config: ReasonerConfig {
            use_ai: false,
            ..Default::default()
        },
    };
    let opts = ReasonerOptions {
        seed_path: None,
        persist: false,
    };
    ctx.config.patch_tag = reason_build_with_options(&ctx, "Warden", opts)
        .await?
        .patch_tag;
    let sources = load_author_source_rows(&ctx, None).await?;
    let raw_snapshots: Vec<Value> = sqlx::query_scalar("SELECT to_jsonb(s) FROM (SELECT DISTINCT ON (source,entity_type,canonical_name) * FROM brain.entity_snapshots WHERE source='deadlock_assets_api' ORDER BY source,entity_type,canonical_name,fetched_at DESC,id DESC) s").fetch_all(&pool).await?;
    let tier_rows: Vec<(i64, i64)> =
        sqlx::query_as("SELECT item_id,tier FROM brain.item_catalog WHERE tier IS NOT NULL")
            .fetch_all(&pool)
            .await?;
    let names: Vec<String> =
        sqlx::query_scalar("SELECT name FROM brain.hero_catalog ORDER BY hero_id")
            .fetch_all(&pool)
            .await?;
    let measured_at: String = sqlx::query_scalar("SELECT now()::text")
        .fetch_one(&pool)
        .await?;
    let mut heroes = Vec::new();
    for name in names {
        let (hero, items, meta, snapshots) = load_reasoning_inputs(&ctx, &name, None).await?;
        let events = load_patch_events_for_snapshots(&ctx, hero.hero_id, &snapshots).await?;
        let input = FrozenHero {
            rows: load_meta_rows(&ctx, hero.hero_id).await?,
            synergies: load_synergies(&ctx, hero.hero_id).await?,
            claims: load_claims(&ctx, hero.hero_id).await?,
            layout: meta.core_layouts.for_hero(hero.hero_id).clone(),
            ability_orders: meta.hero_ability_orders,
            live_baseline: reason_build_with_options(&ctx, &name, opts).await?,
            hero,
            items,
            snapshots,
            events,
        };
        eprintln!("Eingefroren: {name}");
        heroes.push(input);
    }
    let frozen = Frozen {
        format_version: 1,
        baseline_revision: "a57382a".into(),
        measured_at,
        config: ctx.config,
        sources,
        raw_snapshots,
        item_tiers: tier_rows.into_iter().collect(),
        heroes,
    };
    for input in &frozen.heroes {
        let replay = compose(&frozen, input, &BTreeSet::new(), false, "full")?;
        if replay != input.live_baseline {
            return Err(format!(
                "Offline-Reproduktion weicht für {} von der produktiven Fassade ab",
                input.hero.name
            )
            .into());
        }
    }
    fs::write(output, serde_json::to_vec(&frozen)?)?;
    pool.close().await;
    Ok(())
}

fn evaluate(input: &Path, output: &Path, mode: &str) -> std::result::Result<(), Error> {
    if output.exists() {
        return Err("Ausgabedatei existiert bereits".into());
    }
    let frozen: Frozen = serde_json::from_slice(&fs::read(input)?)?;
    if frozen.format_version != 1 {
        return Err("Unbekanntes Freeze-Format".into());
    }
    let holdout = mode == "holdout";
    let mut reports = Vec::new();
    for hero in &frozen.heroes {
        let own = frozen
            .sources
            .iter()
            .filter(|row| row["hero_id"].as_i64() == Some(hero.hero.hero_id))
            .collect::<Vec<_>>();
        if holdout {
            for row in own {
                let id = author(row)?;
                let collaborators = ["13446690", "34634349", "1650097169"];
                let excluded = if collaborators.contains(&id.as_str()) {
                    collaborators.into_iter().map(str::to_owned).collect()
                } else {
                    BTreeSet::from([id])
                };
                let build = compose(&frozen, hero, &excluded, true, "full")?;
                reports.push(json!({"excluded_authors":excluded,"training_sources":frozen.sources.iter().filter(|source| author(source).is_ok_and(|id| !excluded.contains(&id))).count(),"measurement":measure(hero,&build,row)?,"build":build}));
            }
        } else {
            let mut variants = Vec::new();
            let variants_to_run = if mode == "sensitivity" {
                vec![
                    "full",
                    "no_pairs",
                    "no_statistical_scores",
                    "window20",
                    "window60",
                ]
            } else {
                vec!["full"]
            };
            for variant in variants_to_run {
                let build = compose(&frozen, hero, &BTreeSet::new(), false, variant)?;
                let measurements = own
                    .iter()
                    .map(|row| measure(hero, &build, row))
                    .collect::<std::result::Result<Vec<_>, _>>()?;
                variants.push(json!({"variant":variant,"build":build,"measurements":measurements}));
            }
            reports.push(json!({"hero_id":hero.hero.hero_id,"hero_name":hero.hero.name,"reference_count":own.len(),"variants":variants}));
        }
    }
    fs::write(
        output,
        serde_json::to_vec_pretty(
            &json!({"format_version":1,"mode":mode,"baseline_revision":frozen.baseline_revision,"frozen_at":frozen.measured_at,"patch_tag":frozen.config.patch_tag,"contract":"Autorenübereinstimmung auf identischem eingefrorenem Stand, keine historische Meta-Prognose. Holdout: Autor global aus Layout, Kernrolle, Verkauf und author_hits entfernt; Claims und statistische Skillfolge ausgeschlossen. Zeitgleiche aggregierte Spielstatistik bleibt Eingabe. Warden ist Entwicklungsreferenz, kein unabhängiger Test.","reports":reports}),
        )?,
    )?;
    Ok(())
}

#[tokio::main]
async fn main() -> std::result::Result<(), Error> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [mode, output] if mode == "freeze" => freeze(Path::new(output)).await,
        [mode, input, output]
            if ["evaluate", "holdout", "sensitivity"].contains(&mode.as_str()) =>
        {
            evaluate(Path::new(input), Path::new(output), mode)
        }
        _ => Err(
            "Aufruf: build_evaluation freeze DATEI | evaluate|holdout|sensitivity FROZEN AUSGABE"
                .into(),
        ),
    }
}
