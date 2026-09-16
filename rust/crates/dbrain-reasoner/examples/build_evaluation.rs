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
mod support;
include!(concat!(env!("OUT_DIR"), "/evaluation_provenance.rs"));

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct FreezeGuard {
    backend_pid: i32,
    snapshot_id: String,
    isolation: String,
    read_only: String,
}

impl FreezeGuard {
    fn verify(&self, current: &Self) -> std::result::Result<(), Error> {
        if self.backend_pid <= 0
            || self.snapshot_id.is_empty()
            || self.isolation != "repeatable read"
            || self.read_only != "on"
            || self != current
        {
            return Err("Freeze abgebrochen: Datenbankverbindung oder Transaktionssnapshot hat sich geändert".into());
        }
        Ok(())
    }
}

async fn freeze_guard(pool: &sqlx::PgPool) -> std::result::Result<FreezeGuard, Error> {
    let (backend_pid, snapshot_id, isolation, read_only): (i32, String, String, String) =
        sqlx::query_as("SELECT pg_backend_pid(), txid_current_snapshot()::text, current_setting('transaction_isolation'), current_setting('transaction_read_only')")
            .fetch_one(pool).await?;
    Ok(FreezeGuard {
        backend_pid,
        snapshot_id,
        isolation,
        read_only,
    })
}

fn write_guarded_snapshot(
    output: &Path,
    bytes: &[u8],
    start: &FreezeGuard,
    end: &FreezeGuard,
) -> std::result::Result<(), Error> {
    start.verify(end)?;
    support::write_new(output, bytes)?;
    Ok(())
}

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
    #[serde(default)]
    snapshot_guard: Option<FreezeGuard>,
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
    population: &PopulationPrior,
) -> std::result::Result<(BuildObject, Value, Vec<ScoredItem>), Error> {
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
        population: population.clone(),
    };
    let mut hero = input.hero.clone();
    let mut items = input.items.clone();
    enrich_frozen_models(&mut hero, &mut items, &frozen.raw_snapshots)?;
    let planned = plan_build(&hero, &items, &context, &input.events, &input.snapshots, &cfg)?;
    let mut build = planned.build;
    let plan = if ablation == "plan" {
        let plan =
            composer::purchase_plan_with_sources(&planned.hero, &planned.scored, &cfg, &context)?;
        if plan
            .steps
            .iter()
            .map(|step| step.transition.purchased_id)
            .collect::<Vec<_>>()
            != build
                .core
                .iter()
                .map(|item| item.item_id)
                .collect::<Vec<_>>()
        {
            return Err(
                "Mechanischer Plan stimmt nicht mit der ausgegebenen Kaufkurve überein".into(),
            );
        }
        serde_json::to_value(plan)?
    } else {
        Value::Null
    };
    annotate_missing_authors(&mut build, &context);
    Ok((build, plan, planned.scored))
}

fn measure(
    input: &FrozenHero,
    build: &BuildObject,
    scores: &[ScoredItem],
    population: &PopulationPrior,
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
    let in_build = build
        .core
        .iter()
        .map(|item| item.item_id)
        .collect::<BTreeSet<_>>();
    let diagnostics = weapons
        .iter()
        .map(|id| {
            let rank = scores.iter().position(|scored| scored.item.item_id == *id);
            let scored = rank.map(|position| &scores[position]);
            json!({
                "id": id,
                "name": scored.map(|scored| scored.item.name.clone()),
                "in_build": in_build.contains(id),
                "rank": rank.map(|position| position + 1),
                "scored_items": scores.len(),
                "cost": scored.map(|scored| scored.item.cost),
                "total": scored.map(|scored| scored.score.total),
                "per_slot_value": scored.map(|scored| scored.score.per_slot_value),
                "per_soul_value": scored.map(|scored| scored.score.per_soul_value),
                "combat_value": scored.map(|scored| scored.score.combat_value),
                "purchase_bonus_value": scored.map(|scored| scored.score.purchase_bonus_value),
                "condition_factor": scored.map(|scored| scored.score.condition_factor),
                "meta_support": scored.map(|scored| scored.score.meta_support),
            })
        })
        .collect::<Vec<_>>();
    Ok(
        json!({"hero_id":input.hero.hero_id,"hero_name":input.hero.name,"build_id":row["hero_build_id"],"version":reference.version,"author_id":author(row)?,"reference_count":reference.core_item_ids.len(),"reference_weapon_count":weapons.len(),"weapon_hits":hits,"weapon_hit_count":hits.len(),"reference_weapon_diagnostics":diagnostics,"metrics":backtest::backtest_metrics(build,&reference),"population_backtest":backtest::population_backtest(build,population)}),
    )
}

async fn freeze(output: &Path) -> std::result::Result<(), Error> {
    if output.exists() {
        return Err("Ausgabedatei existiert bereits".into());
    }
    let access = deadlock_brain_core::pg::pg_pool_read_only().await?;
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .idle_timeout(None)
        .max_lifetime(None)
        .connect_with((*access.connect_options()).clone())
        .await?;
    access.close().await;
    sqlx::query("BEGIN ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&pool)
        .await?;
    let start_guard = freeze_guard(&pool).await?;
    start_guard.verify(&start_guard)?;
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
    let raw_snapshots: Vec<Value> = sqlx::query_scalar("SELECT to_jsonb(s) FROM (SELECT DISTINCT ON (source,entity_type,COALESCE(payload->>'id',external_id,canonical_name)) * FROM brain.entity_snapshots WHERE source IN ('deadlock_assets_api','deadlock_data') ORDER BY source,entity_type,COALESCE(payload->>'id',external_id,canonical_name),fetched_at DESC,id DESC) s").fetch_all(&pool).await?;
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
        baseline_revision: ALGORITHM_REVISION.into(),
        measured_at,
        snapshot_guard: Some(start_guard.clone()),
        config: ctx.config,
        sources,
        raw_snapshots,
        item_tiers: tier_rows.into_iter().collect(),
        heroes,
    };
    for input in &frozen.heroes {
        for item in &input.items {
            if !frozen.raw_snapshots.iter().any(|row| {
                row["source"] == "deadlock_assets_api"
                    && row["entity_type"] == "item_or_ability"
                    && row["payload"]["id"].as_i64() == Some(item.item_id)
            }) {
                return Err(format!(
                    "Rohsnapshot fehlt für Item {} ({})",
                    item.name, item.item_id
                )
                .into());
            }
        }
        let population = load_population_prior(&pool, input.hero.hero_id).await?;
        let replay = compose(&frozen, input, &BTreeSet::new(), false, "full", &population)?;
        if replay.0 != input.live_baseline {
            return Err(format!(
                "Offline-Reproduktion weicht für {} von der produktiven Fassade ab",
                input.hero.name
            )
            .into());
        }
    }
    let bytes = serde_json::to_vec(&frozen)?;
    let end_guard = freeze_guard(&pool).await?;
    write_guarded_snapshot(output, &bytes, &start_guard, &end_guard)?;
    pool.close().await;
    Ok(())
}

#[cfg(test)]
mod freeze_guard_tests {
    use super::*;

    #[test]
    fn connection_or_transaction_changes_prevent_output() {
        let start = FreezeGuard {
            backend_pid: 123,
            snapshot_id: "10:20:12".into(),
            isolation: "repeatable read".into(),
            read_only: "on".into(),
        };
        let mut changes = vec![start.clone(); 4];
        changes[0].backend_pid = 124;
        changes[1].snapshot_id = "10:21:12".into();
        changes[2].isolation = "read committed".into();
        changes[3].read_only = "off".into();
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "brain-freeze-guard-{}-{unique}.json",
            std::process::id()
        ));
        for changed in changes {
            assert!(write_guarded_snapshot(&path, b"not written", &start, &changed).is_err());
            assert!(!path.exists());
            assert!(
                changed.verify(&changed).is_err()
                    || changed.backend_pid != start.backend_pid
                    || changed.snapshot_id != start.snapshot_id
            );
        }
        write_guarded_snapshot(&path, b"same snapshot", &start, &start).unwrap();
        assert_eq!(fs::read(&path).unwrap(), b"same snapshot");
        fs::remove_file(path).unwrap();
    }
}

// Serde-Spiegel eines PopulationPrior nur fuer das eingefrorene Vorher/Nachher.
// PopulationPrior selbst bleibt ohne serde; hier wird verlustfrei ueber die
// oeffentlichen Zugriffsmethoden gespiegelt (Praevalenz>0, endliche Median-
// position, Staples) und ueber from_items wieder aufgebaut.
#[derive(Serialize, Deserialize)]
struct FrozenPopulationPrior {
    prevalence: Vec<(i64, f64)>,
    median_position: Vec<(i64, f64)>,
    staples: Vec<i64>,
}

impl FrozenPopulationPrior {
    fn from_prior(prior: &PopulationPrior) -> Self {
        let mut ids: BTreeSet<i64> = prior.ranked_by_prevalence().into_iter().collect();
        for (id, _) in prior.positions() {
            ids.insert(id);
        }
        for id in prior.staples() {
            ids.insert(id);
        }
        Self {
            prevalence: ids
                .iter()
                .map(|id| (*id, prior.prevalence(*id)))
                .filter(|(_, value)| *value > 0.0)
                .collect(),
            median_position: prior.positions(),
            staples: prior.staples(),
        }
    }

    fn into_prior(self) -> PopulationPrior {
        let prevalence: BTreeMap<i64, f64> = self.prevalence.into_iter().collect();
        let median: BTreeMap<i64, f64> = self.median_position.into_iter().collect();
        let staples: BTreeSet<i64> = self.staples.into_iter().collect();
        let mut ids: BTreeSet<i64> = prevalence.keys().copied().collect();
        ids.extend(median.keys().copied());
        ids.extend(staples.iter().copied());
        PopulationPrior::from_items(ids.into_iter().map(|id| PopulationItem {
            item_id: id,
            prevalence: prevalence.get(&id).copied().unwrap_or(0.0),
            median_position: median.get(&id).copied(),
            is_staple: staples.contains(&id),
        }))
    }
}

// Zentraler Zugriff nur lesend und technisch erzwungen (pg_pool_read_only setzt
// default_transaction_read_only=on ueber die Verbindungsoptionen).
async fn load_populations_from_db() -> std::result::Result<BTreeMap<i64, PopulationPrior>, Error> {
    let pool = deadlock_brain_core::pg::pg_pool_read_only().await?;
    let present: Option<bool> =
        sqlx::query_scalar("SELECT to_regclass('brain.population_item_stats') IS NOT NULL")
            .fetch_one(&pool)
            .await?;
    if present != Some(true) {
        pool.close().await;
        return Ok(BTreeMap::new());
    }
    let hero_ids: Vec<i64> =
        sqlx::query_scalar("SELECT DISTINCT hero_id FROM brain.population_item_stats")
            .fetch_all(&pool)
            .await?;
    let mut populations = BTreeMap::new();
    for hero_id in hero_ids {
        populations.insert(hero_id, load_population_prior(&pool, hero_id).await?);
    }
    pool.close().await;
    Ok(populations)
}

// Vorher/Nachher-Vergleiche lesen die Population aus einer eingefrorenen Datei
// (Umgebungsvariable FROZEN_POPULATIONS), damit kein Live-DB-Drift zwischen den
// beiden Messungen entsteht. Ohne die Variable wird read-only aus der DB geladen.
async fn load_populations() -> std::result::Result<BTreeMap<i64, PopulationPrior>, Error> {
    if let Some(path) = std::env::var_os("FROZEN_POPULATIONS") {
        let mirror: BTreeMap<i64, FrozenPopulationPrior> =
            serde_json::from_slice(&fs::read(&path)?)?;
        return Ok(mirror
            .into_iter()
            .map(|(id, prior)| (id, prior.into_prior()))
            .collect());
    }
    load_populations_from_db().await
}

async fn freeze_populations(output: &Path) -> std::result::Result<(), Error> {
    if output.exists() {
        return Err("Ausgabedatei existiert bereits".into());
    }
    if std::env::var_os("FROZEN_POPULATIONS").is_some() {
        return Err(
            "FROZEN_POPULATIONS darf beim Einfrieren der Population nicht gesetzt sein".into(),
        );
    }
    let populations = load_populations_from_db().await?;
    let mirror: BTreeMap<i64, FrozenPopulationPrior> = populations
        .iter()
        .map(|(id, prior)| (*id, FrozenPopulationPrior::from_prior(prior)))
        .collect();
    support::write_new(output, &serde_json::to_vec_pretty(&mirror)?)?;
    Ok(())
}

fn evaluate(
    input: &Path,
    output: &Path,
    mode: &str,
    selection: Option<&str>,
    populations: &BTreeMap<i64, PopulationPrior>,
) -> std::result::Result<(), Error> {
    if output.exists() {
        return Err("Ausgabedatei existiert bereits".into());
    }
    let frozen: Frozen = serde_json::from_slice(&fs::read(input)?)?;
    if frozen.format_version != 1 {
        return Err("Unbekanntes Freeze-Format".into());
    }
    if let Some(names) = selection {
        for name in names.split(',') {
            if !frozen
                .heroes
                .iter()
                .any(|hero| hero.hero.name.eq_ignore_ascii_case(name))
            {
                return Err(format!("Unbekannter Held in der Auswahl: {name}").into());
            }
        }
    }
    let holdout = mode == "holdout";
    let mut reports = Vec::new();
    for hero in &frozen.heroes {
        if selection.is_some_and(|names| {
            !names
                .split(',')
                .any(|name| name.eq_ignore_ascii_case(&hero.hero.name))
        }) {
            continue;
        }
        let population = populations
            .get(&hero.hero.hero_id)
            .cloned()
            .unwrap_or_default();
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
                let (build, _, scores) =
                    compose(&frozen, hero, &excluded, true, "full", &population)?;
                reports.push(json!({"excluded_authors":excluded,"training_sources":frozen.sources.iter().filter(|source| author(source).is_ok_and(|id| !excluded.contains(&id))).count(),"measurement":measure(hero,&build,&scores,&population,row)?,"build":build}));
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
            } else if mode == "plan" {
                vec!["plan"]
            } else {
                vec!["full"]
            };
            for variant in variants_to_run {
                let (build, plan, scores) =
                    compose(&frozen, hero, &BTreeSet::new(), false, variant, &population)?;
                let measurements = own
                    .iter()
                    .map(|row| measure(hero, &build, &scores, &population, row))
                    .collect::<std::result::Result<Vec<_>, _>>()?;
                variants.push(json!({"variant":variant,"build":build,"plan":plan,"measurements":measurements}));
            }
            reports.push(json!({"hero_id":hero.hero.hero_id,"hero_name":hero.hero.name,"reference_count":own.len(),"variants":variants}));
        }
        eprintln!("Ausgewertet: {} ({mode})", hero.hero.name);
    }
    support::write_new(
        output,
        &serde_json::to_vec_pretty(
            &json!({"format_version":1,"mode":mode,"algorithm_revision":ALGORITHM_REVISION,"baseline_revision":frozen.baseline_revision,"frozen_at":frozen.measured_at,"patch_tag":frozen.config.patch_tag,"contract":"Autorenübereinstimmung auf identischem eingefrorenem Stand, keine historische Meta-Prognose. Holdout: Autor global aus Layout, Kernrolle, Verkauf und author_hits entfernt; Claims und statistische Skillfolge ausgeschlossen. Zeitgleiche aggregierte Spielstatistik bleibt Eingabe. Warden ist Entwicklungsreferenz, kein unabhängiger Test.","reports":reports}),
        )?,
    )?;
    Ok(())
}

#[tokio::main]
async fn main() -> std::result::Result<(), Error> {
    if !SOURCE_CLEAN || ALGORITHM_REVISION.len() != 40 {
        return Err(
            "Messbinary ohne saubere eingecheckte Quellrevision; nach Commit neu bauen".into(),
        );
    }
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [mode, output] if mode == "freeze" => freeze(Path::new(output)).await,
        [mode, output] if mode == "freeze-populations" => {
            freeze_populations(Path::new(output)).await
        }
        [mode, input, output]
            if ["evaluate", "holdout", "sensitivity", "plan"].contains(&mode.as_str()) =>
        {
            let populations = load_populations().await?;
            evaluate(
                Path::new(input),
                Path::new(output),
                mode,
                None,
                &populations,
            )
        }
        [mode, input, output, heroes]
            if ["evaluate", "sensitivity", "plan"].contains(&mode.as_str()) =>
        {
            let populations = load_populations().await?;
            evaluate(
                Path::new(input),
                Path::new(output),
                mode,
                Some(heroes),
                &populations,
            )
        }
        _ => Err(
            "Aufruf: build_evaluation freeze DATEI | freeze-populations DATEI | evaluate|holdout|sensitivity FROZEN AUSGABE"
                .into(),
        ),
    }
}
