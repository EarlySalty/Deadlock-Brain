//! Read-only frozen input and deterministic family evaluation. No AI or writes
//! to Central; output files are exclusively created and never overwritten.
use dbrain_reasoner::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::postgres::PgPoolOptions;
use std::{fs, path::Path};
mod support;
type Error = Box<dyn std::error::Error>;

#[derive(Serialize, Deserialize)]
struct Input {
    hero: HeroModel,
    items: Vec<ItemModel>,
    meta: meta::MetaIndexWithSources,
    snapshots: Vec<PatchSnapshot>,
    events: Vec<Value>,
}
#[derive(Serialize, Deserialize)]
struct Frozen {
    format_version: u32,
    config: ReasonerConfig,
    snapshot_guard: Value,
    heroes: Vec<Input>,
}
async fn guard(pool: &sqlx::PgPool) -> Result<Value> {
    let (pid,snapshot,isolation,readonly):(i32,String,String,String)=sqlx::query_as("SELECT pg_backend_pid(),txid_current_snapshot()::text,current_setting('transaction_isolation'),current_setting('transaction_read_only')")
        .fetch_one(pool).await.map_err(ReasonerError::Db)?;
    if isolation != "repeatable read" || readonly != "on" {
        return Err(ReasonerError::Data(
            "consistent read-only snapshot required".into(),
        ));
    }
    Ok(json!({"backend_pid":pid,"snapshot":snapshot,"isolation":isolation,"read_only":readonly}))
}
async fn freeze(output: &Path, names: &str) -> std::result::Result<(), Error> {
    if output.exists() {
        return Err("output already exists".into());
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
    let snapshot_guard = guard(&pool).await?;
    let config = ReasonerConfig {
        patch_tag: dbrain_builds::latest_patch_tag(&pool).await?,
        use_ai: false,
        ..Default::default()
    };
    let ctx = ReasonerCtx {
        pool: pool.clone(),
        ai: None,
        config: config.clone(),
    };
    let mut heroes = Vec::new();
    for name in names.split(',') {
        eprintln!("freeze inputs: {name}");
        let (hero, items, mut meta, snapshots) = load_reasoning_inputs(&ctx, name, None).await?;
        meta.author_builds = families::load_family_author_sources(&pool, hero.hero_id).await?;
        meta.observations = families::load_player_observations(&pool, hero.hero_id).await?;
        meta.observations
            .extend(meta.author_builds.iter().map(families::author_observation));
        let events = load_patch_events_for_snapshots(&ctx, hero.hero_id, &snapshots).await?;
        heroes.push(Input {
            hero,
            items,
            meta,
            snapshots,
            events,
        });
    }
    if snapshot_guard != guard(&pool).await? {
        return Err("snapshot changed".into());
    }
    let frozen = Frozen {
        format_version: 1,
        config,
        snapshot_guard,
        heroes,
    };
    support::write_new(output, &serde_json::to_vec(&frozen)?)?;
    sqlx::query("ROLLBACK").execute(&pool).await?;
    println!("frozen {} heroes; read_only=on", frozen.heroes.len());
    Ok(())
}
fn discover(input: &Path, output: &Path) -> std::result::Result<(), Error> {
    let frozen: Frozen = serde_json::from_slice(&fs::read(input)?)?;
    let mut reports = Vec::new();
    for input in frozen.heroes {
        let mut hero = input.hero;
        let mut items = input.items;
        let mut deltas =
            patch::compute_patch_delta_with_snapshots(&hero, &input.events, &input.snapshots);
        patch::apply_scored_patch_delta(
            &mut hero,
            &mut items,
            &mut deltas,
            &input.meta.index,
            &frozen.config,
        );
        let discovery = families::detect_families(
            &input.meta.observations,
            &items,
            &input.meta.population,
            &families::FamilyPolicy::for_patch(&input.events, &frozen.config),
        );
        let names = items
            .iter()
            .map(|i| (i.item_id, i.name.clone()))
            .collect::<std::collections::BTreeMap<_, _>>();
        reports.push(json!({"hero_id":hero.hero_id,"hero_name":hero.name,"discovery":discovery,"item_names":names,"patch_deltas":deltas}));
    }
    support::write_new(output, &serde_json::to_vec_pretty(&reports)?)?;
    summary(output)
}
fn summary(path: &Path) -> std::result::Result<(), Error> {
    let reports: Vec<Value> = serde_json::from_slice(&fs::read(path)?)?;
    for report in reports {
        let families = report["discovery"]["families"]
            .as_array()
            .ok_or("families missing")?;
        println!(
            "{}: observations={}, candidates={}, unassigned={}",
            report["hero_name"],
            report["discovery"]["input_observations"],
            families.len(),
            report["discovery"]["unassigned_observations"]
        );
        for family in families
            .iter()
            .filter(|f| f["eligible_for_planning"] == true)
        {
            let mut items = family["items"].as_array().ok_or("items missing")?.clone();
            items.sort_by(|a, b| {
                b["prevalence"]
                    .as_f64()
                    .unwrap_or(0.0)
                    .total_cmp(&a["prevalence"].as_f64().unwrap_or(0.0))
            });
            let names = items
                .iter()
                .take(12)
                .map(|i| {
                    format!(
                        "{} {:.0}%",
                        report["item_names"][i["item_id"].to_string()]
                            .as_str()
                            .unwrap_or("?"),
                        i["prevalence"].as_f64().unwrap_or(0.0) * 100.0
                    )
                })
                .collect::<Vec<_>>();
            println!(
                "  {} {} matches={} players={} authors={} cohesion={:.3} skill={}: {}",
                family["id"],
                family["label"],
                family["player_matches"],
                family["distinct_players"],
                family["distinct_authors"],
                family["cohesion"].as_f64().unwrap_or(0.0),
                family["skill_order_support"],
                names.join(", ")
            );
        }
    }
    Ok(())
}
fn plan(
    input: &Path,
    output: &Path,
    names: Option<&str>,
    holdout: bool,
) -> std::result::Result<(), Error> {
    let frozen: Frozen = serde_json::from_slice(&fs::read(input)?)?;
    let mut reports = Vec::new();
    for mut input in frozen.heroes {
        if names.is_some_and(|names| !names.split(',').any(|name| name == input.hero.name)) {
            continue;
        }
        let withheld = if holdout {
            let withheld = input
                .meta
                .observations
                .iter()
                .filter(|row| families::is_holdout(row))
                .cloned()
                .collect::<Vec<_>>();
            input
                .meta
                .observations
                .retain(|row| !families::is_holdout(row));
            input
                .meta
                .author_builds
                .retain(|source| !families::is_holdout(&families::author_observation(source)));
            input.meta.population = families::observed_population(&input.meta.observations);
            input.meta.hero_ability_orders.clear();
            input.meta.index = meta::build_meta_index(&[], &[], &[], &frozen.config);
            input.meta.combinations.clear();
            withheld
        } else {
            Vec::new()
        };
        eprintln!("plan: {} (holdout={holdout})", input.hero.name);
        match plan_build(&input.hero,&input.items,&input.meta,&input.events,&input.snapshots,&frozen.config) {
            Err(error)=>reports.push(json!({"hero_name":input.hero.name,"error":error.to_string(),"independent_holdout":null})),
            Ok(planned)=>{
                let assignments=planned.build.family_discovery.as_ref().map(|discovery| {
                    let items=planned.scored.iter().map(|item|item.item.clone()).collect::<Vec<_>>();
                    families::assign_holdout(discovery,&input.meta.observations,&withheld,&items)
                }).unwrap_or_default();
                let mut metrics=Vec::new();
                for build in std::iter::once(&planned.build).chain(&planned.build.variants) {
                    let context=build.family.as_ref().map(|family| families::conditioned_meta(&input.meta,family,&frozen.config)).unwrap_or_else(|| input.meta.clone());
                    let ids=build.core.iter().map(|i| i.item_id).collect::<Vec<_>>();
                    let authors=context.author_builds.iter().map(|source| {
                        let core=meta::core_item_ids(&source.details);
                        let positions=core.iter().enumerate().map(|(i,id)|(*id,i as f64+1.0)).collect::<Vec<_>>();
                        json!({"author":source.author,"core_jaccard":backtest::jaccard_at(usize::MAX,&ids,&core),"jaccard_at_12":backtest::jaccard_at(12,&ids,&core),"kendall_tau":backtest::kendall_tau(&ids,&positions)})
                    }).collect::<Vec<_>>();
                    let population=backtest::population_backtest(build,&context.population);
                    let global=backtest::population_backtest(build,&input.meta.population);
                    println!("{} {}: core={} family staples={}/{} gate={:?} Kendall={:?} Jaccard@12={:?}",input.hero.name,build.family.as_ref().map(|f| f.id.as_str()).unwrap_or("unclassified"),ids.len(),population.present_staples.len(),population.staple_count,population.staple_gate_passed,population.kendall_tau,population.jaccard_at_12);
                    println!("  {}",build.core.iter().map(|i|i.name.as_str()).collect::<Vec<_>>().join(", "));
                    let holdout_report=if holdout {
                        let id=build.family.as_ref().map(|family|family.id.as_str());
                        let keys=assignments.iter().filter(|row|row.family_id.as_deref()==id && id.is_some()).map(|row|row.observation_key.as_str()).collect::<std::collections::BTreeSet<_>>();
                        let rows=withheld.iter().filter(|row|keys.contains(row.key.as_str())).cloned().collect::<Vec<_>>();
                        let player_matches=rows.iter().filter(|row|row.source==families::ObservationSource::Player).count();
                        let prior=families::observed_population(&rows);
                        let test=backtest::population_backtest(build,&prior);
                        let author_tests=rows.iter().filter(|row|row.source==families::ObservationSource::Author).map(|row|json!({
                            "author":row.participant,"jaccard_at_12":backtest::jaccard_at(12,&ids,&row.items),
                            "kendall_tau":backtest::kendall_tau(&ids,&row.items.iter().enumerate().map(|(i,id)|(*id,i as f64+1.0)).collect::<Vec<_>>())
                        })).collect::<Vec<_>>();
                        println!("  HELD OUT: {player_matches} matches; staple_gate={:?}; Kendall={:?}; Jaccard@12={:?}",test.staple_gate_passed,test.kendall_tau,test.jaccard_at_12);
                        json!({"player_matches":player_matches,"distinct_players":rows.iter().filter(|r|r.source==families::ObservationSource::Player).map(|r|&r.participant).collect::<std::collections::BTreeSet<_>>().len(),"population":test,"authors":author_tests,
                            "sufficient_sample":player_matches>=100,"unassigned_holdout_observations":assignments.iter().filter(|a|a.family_id.is_none()).count(),
                            "method":"Participant-disjoint deterministic 20% holdout; assignment to frozen training centroids, no holdout reclustering. Internal validation, not independent expert release approval."})
                    } else {Value::Null};
                    metrics.push(json!({"family_id":build.family.as_ref().map(|f| &f.id),"family_population":population,"global_population":global,"training_authors":authors,"independent_holdout":holdout_report}));
                }
                reports.push(json!({"hero_name":input.hero.name,"build":planned.build,"metrics":metrics,"variant_scores":planned.variant_scores,"patch_deltas":planned.deltas}));
            }
        }
    }
    support::write_new(output, &serde_json::to_vec_pretty(&reports)?)?;
    Ok(())
}

fn verify_replay_bytes(runs: &[Vec<u8>]) -> std::result::Result<(String, usize), Error> {
    if runs.len() != 3 {
        return Err("exactly three independently evaluated runs required".into());
    }
    let reports: Vec<Value> = serde_json::from_slice(&runs[0])?;
    if reports.is_empty()
        || reports.iter().any(|report| {
            !report["error"].is_null()
                || report["hero_name"].as_str().is_none()
                || !report["build"].is_object()
        })
    {
        return Err("empty or failed plans are not a successful replay proof".into());
    }
    if runs[1..].iter().any(|bytes| bytes != &runs[0]) {
        return Err("replays differ; all original outputs are retained for inspection".into());
    }
    Ok((format!("{:x}", Sha256::digest(&runs[0])), reports.len()))
}

/// Repeat the existing planner, without AI, DB access, score changes or output
/// normalization. Byte equality includes every score, variant and explanation;
/// a stable failure or an empty hero selection must not pass this check.
fn replay(input: &Path, output_dir: &Path, names: Option<&str>) -> std::result::Result<(), Error> {
    let input_bytes = fs::read(input)?;
    let frozen: Frozen = serde_json::from_slice(&input_bytes)?;
    if frozen.format_version != 1 || frozen.config.use_ai {
        return Err("version 1 frozen inputs with AI disabled required".into());
    }
    if let Some(names) = names {
        for name in names.split(',') {
            if !frozen.heroes.iter().any(|hero| hero.hero.name == name) {
                return Err(format!("unknown replay hero: {name}").into());
            }
        }
    }
    // create_dir is exclusive. Never reuse or delete existing evidence.
    fs::create_dir(output_dir)?;
    let input_hash = format!("{:x}", Sha256::digest(&input_bytes));
    let mut runs = Vec::new();
    for index in 1..=3 {
        if fs::read(input)? != input_bytes {
            return Err("frozen input changed between replays".into());
        }
        let output = output_dir.join(format!("replay-{index}.json"));
        eprintln!("determinism replay {index}/3");
        plan(input, &output, names, false)?;
        if fs::read(input)? != input_bytes {
            return Err("frozen input changed during replay".into());
        }
        runs.push(fs::read(&output)?);
    }
    let checked = verify_replay_bytes(&runs);
    let hashes = runs
        .iter()
        .map(|bytes| format!("{:x}", Sha256::digest(bytes)))
        .collect::<Vec<_>>();
    let report = json!({"format_version": 1, "input_sha256": input_hash,
        "replay_sha256": hashes, "byte_identical": checked.is_ok(),
        "validation_error": checked.as_ref().err().map(|error| error.to_string()),
        "heroes": checked.as_ref().ok().map(|(_, count)| count),
        "scope": "Three new runs of the same frozen numerical planner; full output byte comparison. Not a current-patch or independent release approval."});
    support::write_new(
        output_dir.join("verification.json"),
        &serde_json::to_vec_pretty(&report)?,
    )?;
    let (hash, count) = checked?;
    println!("REPLAY VERIFIED: {count} heroes, three byte-identical outputs, sha256={hash}");
    Ok(())
}

fn inspect_input(path: &Path) -> std::result::Result<(), Error> {
    let frozen: Frozen = serde_json::from_slice(&fs::read(path)?)?;
    if frozen.format_version != 1 {
        return Err("unsupported frozen input version".into());
    }
    println!(
        "format={} patch={} snapshot={}",
        frozen.format_version, frozen.config.patch_tag, frozen.snapshot_guard
    );
    for input in frozen.heroes {
        let policy = families::FamilyPolicy::for_patch(&input.events, &frozen.config);
        let players = input
            .meta
            .observations
            .iter()
            .filter(|row| row.source == families::ObservationSource::Player)
            .collect::<Vec<_>>();
        let current = policy.patch_started_at.map(|start| {
            players
                .iter()
                .filter(|row| row.observed_at.is_some_and(|time| time >= start))
                .count()
        });
        let times = players
            .iter()
            .filter_map(|row| row.observed_at)
            .collect::<Vec<_>>();
        let snapshot_times = input
            .snapshots
            .iter()
            .flat_map(|snapshot| snapshot.fields.values())
            .filter_map(|field| field.fetched_at)
            .filter(|time| time.is_finite())
            .collect::<Vec<_>>();
        println!("{}: players={} authors={} patch_start={:?} post_patch={:?} match_range={:?}..{:?} snapshot_range={:?}..{:?}",input.hero.name,players.len(),input.meta.author_builds.len(),policy.patch_started_at,current,times.iter().min(),times.iter().max(),snapshot_times.iter().min_by(|a,b|a.total_cmp(b)),snapshot_times.iter().max_by(|a,b|a.total_cmp(b)));
    }
    Ok(())
}

fn inspect_items(path: &Path, ids: &str) -> std::result::Result<(), Error> {
    let frozen: Frozen = serde_json::from_slice(&fs::read(path)?)?;
    if frozen.format_version != 1 {
        return Err("unsupported frozen input version".into());
    }
    let requested = ids
        .split(',')
        .map(str::parse::<i64>)
        .collect::<std::result::Result<std::collections::BTreeSet<_>, _>>()?;
    let input = frozen.heroes.first().ok_or("no heroes in frozen input")?;
    for id in requested {
        match input.items.iter().find(|item| item.item_id == id) {
            Some(item) => println!(
                "{}",
                serde_json::to_string(
                    &json!({"item_id":id,"name":item.name,"class_name":item.class_name,"shopable":item.shopable,"disabled":item.disabled,"tier":item.tier,"cost":item.cost,"component_items":item.component_items,"properties":item.properties})
                )?
            ),
            None => println!("{}", json!({"item_id":id,"catalog_status":"absent"})),
        }
    }
    Ok(())
}

fn report_summary(path: &Path) -> std::result::Result<(), Error> {
    let reports: Vec<Value> = serde_json::from_slice(&fs::read(path)?)?;
    for report in reports {
        println!("HERO {}", report["hero_name"]);
        if !report["error"].is_null() {
            println!("  ERROR {}", report["error"]);
            continue;
        }
        let root = &report["build"];
        for build in std::iter::once(root).chain(root["variants"].as_array().into_iter().flatten())
        {
            let family = &build["family"];
            println!("  {} {} matches={} distinct_players={} authors={} post_patch={} share={} cohesion={} confidence={}",family["id"],family["label"],family["player_matches"],family["distinct_players"],family["distinct_authors"],family["post_patch_player_matches"],family["population_share"],family["cohesion"],build["confidence"]);
            println!("  LIMITATIONS {}", family["limitations"]);
            println!(
                "  SKILL support={} order={}",
                family["skill_order_support"], build["ability_order"]
            );
            if let Some(core) = build["core"].as_array() {
                for item in core {
                    println!(
                        "    {} ({}) confidence={} imbue={} sell={}",
                        item["name"],
                        item["item_id"],
                        item["confidence"],
                        item["imbue_target"],
                        item["sell_priority"]
                    );
                }
            }
            if let Some(metrics) = report["metrics"].as_array() {
                for metric in metrics
                    .iter()
                    .filter(|metric| metric["family_id"] == family["id"])
                {
                    for label in ["family_population", "global_population"] {
                        let test = &metric[label];
                        println!(
                            "  {label}: gate={} staples={} missing={} Kendall={} Jaccard@12={}",
                            test["staple_gate_passed"],
                            test["staple_count"],
                            test["missing_staples"],
                            test["kendall_tau"],
                            test["jaccard_at_12"]
                        );
                    }
                    let held = &metric["independent_holdout"];
                    println!("  HOLDOUT matches={} sufficient={} gate={} missing={} Kendall={} Jaccard@12={}",held["player_matches"],held["sufficient_sample"],held["population"]["staple_gate_passed"],held["population"]["missing_staples"],held["population"]["kendall_tau"],held["population"]["jaccard_at_12"]);
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod replay_tests {
    use super::*;

    fn numerical_output(score: f64) -> Vec<u8> {
        serde_json::to_vec(&json!([{"hero_name":"Synthetic replay",
            "build":{"confidence":"Low","score":score},
            "metrics":{"staple_gate_passed":false}}]))
        .unwrap()
    }

    #[test]
    fn byte_identical_outputs_preserve_red_quality_metrics() {
        let runs = vec![numerical_output(1.0); 3];
        let (hash, count) = verify_replay_bytes(&runs).unwrap();
        assert_eq!(hash.len(), 64);
        assert_eq!(count, 1);
        let unchanged: Value = serde_json::from_slice(&runs[0]).unwrap();
        assert_eq!(unchanged[0]["metrics"]["staple_gate_passed"], false);
        assert_eq!(unchanged[0]["build"]["confidence"], "Low");
    }

    #[test]
    fn a_changed_numeric_value_fails_without_rounding_or_normalization() {
        let runs = vec![
            numerical_output(1.0),
            numerical_output(1.0),
            numerical_output(1.000000001),
        ];
        assert!(verify_replay_bytes(&runs).is_err());
    }

    #[test]
    fn fewer_than_three_replays_are_not_accepted() {
        assert!(verify_replay_bytes(&vec![numerical_output(1.0); 2]).is_err());
    }

    #[test]
    fn stable_empty_or_failed_outputs_are_not_a_replay_proof() {
        assert!(verify_replay_bytes(&vec![b"[]".to_vec(); 3]).is_err());
        let error =
            serde_json::to_vec(&json!([{"hero_name":"Synthetic replay","error":"plan failed"}]))
                .unwrap();
        assert!(verify_replay_bytes(&vec![error; 3]).is_err());
    }
}

#[tokio::main]
async fn main() -> std::result::Result<(), Error> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [mode,path,names] if mode=="freeze"=>freeze(Path::new(path),names).await,
        [mode,input,output] if mode=="discover"=>discover(Path::new(input),Path::new(output)),
        [mode,input,output] if mode=="replay"=>replay(Path::new(input),Path::new(output),None),
        [mode,input,output,names] if mode=="replay"=>replay(Path::new(input),Path::new(output),Some(names)),
        [mode,input,output] if mode=="plan" || mode=="holdout"=>plan(Path::new(input),Path::new(output),None,mode=="holdout"),
        [mode,input,output,names] if mode=="plan" || mode=="holdout"=>plan(Path::new(input),Path::new(output),Some(names),mode=="holdout"),
        [mode,path] if mode=="summary"=>summary(Path::new(path)),
        [mode,path] if mode=="report"=>report_summary(Path::new(path)),
        [mode,path] if mode=="inspect"=>inspect_input(Path::new(path)),
        [mode,path,ids] if mode=="inspect-items"=>inspect_items(Path::new(path),ids),
        _=>Err("usage: family_evaluation freeze OUTPUT HERO,HERO | discover INPUT OUTPUT | plan INPUT OUTPUT [HERO,HERO] | holdout INPUT OUTPUT [HERO,HERO] | replay INPUT NEW_DIRECTORY [HERO,HERO] | summary DISCOVERY | report PLANS | inspect INPUT | inspect-items INPUT ID,ID".into()),
    }
}
