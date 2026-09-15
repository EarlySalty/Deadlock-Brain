use std::time::{Instant, SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Result};
use clap::{Args, Subcommand};
use sqlx::postgres::PgPool;
use tokio::runtime::Runtime;

use crate::aggregate::{self, BucketAgg};
use crate::api::ApiClient;
use crate::catalog::Catalog;
use crate::clean::match_to_rows;
use crate::db;
use crate::index::PopulationIndex;
use crate::{BUCKET_ALL, MIN_CELL_OBSERVATIONS};

const MATCHES_PER_PAGE: usize = 200;
const DEFAULT_WINDOW_DAYS: i64 = 30;

#[derive(Debug, Args)]
pub struct PopulationArgs {
    #[command(subcommand)]
    command: PopulationCommand,
}

#[derive(Debug, Subcommand)]
enum PopulationCommand {
    #[command(about = "Lädt Matchdaten der Population und speichert bereinigte Spieler-Matches.")]
    Sync(SyncArgs),
    #[command(about = "Baut die Populations-Aggregate je Held und Bucket neu auf.")]
    Stats(StatsArgs),
    #[command(
        about = "Zeigt Staples, Reihenfolge, Imbue-Ziele und Skill-Reihenfolge eines Helden."
    )]
    Show(ShowArgs),
}

#[derive(Debug, Args)]
struct SyncArgs {
    #[arg(long, default_value_t = 10000)]
    matches: usize,
    #[arg(long)]
    hero: Option<String>,
    #[arg(long)]
    since: Option<i64>,
}

#[derive(Debug, Args)]
struct StatsArgs {
    #[arg(long)]
    hero: Option<String>,
    #[arg(long)]
    rebuild: bool,
}

#[derive(Debug, Args)]
struct ShowArgs {
    #[arg(long)]
    hero: String,
    #[arg(long)]
    bucket: Option<String>,
}

pub fn run_population(args: PopulationArgs) -> Result<()> {
    let runtime = Runtime::new()?;
    let client = ApiClient::new()?;
    let catalog = Catalog::fetch(&client)?;
    let pool = runtime.block_on(db::connect())?;
    runtime.block_on(db::ensure_schema(&pool))?;

    match args.command {
        PopulationCommand::Sync(sync) => run_sync(&runtime, &client, &catalog, &pool, sync),
        PopulationCommand::Stats(stats) => runtime.block_on(run_stats(&pool, &catalog, stats)),
        PopulationCommand::Show(show) => runtime.block_on(run_show(&pool, &catalog, show)),
    }
}

fn run_sync(
    runtime: &Runtime,
    client: &ApiClient,
    catalog: &Catalog,
    pool: &PgPool,
    args: SyncArgs,
) -> Result<()> {
    runtime.block_on(db::assert_writable(pool))?;
    let min_unix = args
        .since
        .unwrap_or_else(|| now_unix() - DEFAULT_WINDOW_DAYS * 86_400);
    let hero_ids = match &args.hero {
        Some(needle) => {
            let id = catalog
                .resolve_hero(needle)
                .ok_or_else(|| anyhow!("Held '{needle}' ist im Katalog nicht bekannt."))?;
            vec![id]
        }
        None => Vec::new(),
    };

    let started = Instant::now();
    let mut seen = 0usize;
    let mut inserted_total = 0usize;
    let mut skipped_total = 0usize;
    let mut cursor: Option<i64> = None;
    let mut window_low: Option<i64> = None;
    let mut window_high: Option<i64> = None;

    while seen < args.matches {
        let limit = MATCHES_PER_PAGE.min(args.matches - seen);
        let page = client.fetch_metadata_page(cursor, limit, min_unix, &hero_ids)?;
        if page.is_empty() {
            break;
        }

        let mut rows = Vec::new();
        let mut lowest = i64::MAX;
        let mut highest = i64::MIN;
        for value in &page {
            if let Some(match_id) = value.get("match_id").and_then(|id| id.as_i64()) {
                lowest = lowest.min(match_id);
                highest = highest.max(match_id);
            }
            rows.extend(match_to_rows(value, catalog));
        }

        let (inserted, skipped) = runtime.block_on(db::insert_rows(pool, &rows))?;
        inserted_total += inserted;
        skipped_total += skipped;
        seen += page.len();
        window_low = Some(window_low.map_or(lowest, |value: i64| value.min(lowest)));
        window_high = Some(window_high.map_or(highest, |value: i64| value.max(highest)));

        println!(
            "geladen {seen}/{} Matches, neu {inserted_total} übersprungen {skipped_total}",
            args.matches
        );

        if lowest == i64::MAX {
            break;
        }
        cursor = Some(lowest - 1);
        if page.len() < limit {
            break;
        }
    }

    let duration = started.elapsed();
    let run = db::SyncRun {
        requested_matches: args.matches as i32,
        hero_filter: hero_ids.first().copied(),
        since_unix: Some(min_unix),
        window_low_match_id: window_low,
        window_high_match_id: window_high,
        matches_seen: seen as i32,
        player_matches_inserted: inserted_total as i32,
        player_matches_skipped: skipped_total as i32,
        duration_ms: duration.as_millis() as i64,
    };
    runtime.block_on(db::record_sync_run(pool, &run))?;

    println!(
        "Fertig: {seen} Matches, {inserted_total} neue Spieler-Matches, {skipped_total} schon vorhanden, {:.1}s.",
        duration.as_secs_f64()
    );
    Ok(())
}

async fn run_stats(pool: &PgPool, catalog: &Catalog, args: StatsArgs) -> Result<()> {
    let hero_ids = match &args.hero {
        Some(needle) => {
            let id = catalog
                .resolve_hero(needle)
                .ok_or_else(|| anyhow!("Held '{needle}' ist im Katalog nicht bekannt."))?;
            vec![id]
        }
        None => db::distinct_hero_ids(pool).await?,
    };
    if hero_ids.is_empty() {
        println!("Keine Spieler-Matches vorhanden. Zuerst 'population sync' laufen lassen.");
        return Ok(());
    }
    let _ = args.rebuild;

    let detailed = hero_ids.len() == 1;
    for hero_id in hero_ids {
        let rows = db::load_hero_rows(pool, hero_id).await?;
        let buckets = aggregate::rebuild_hero(pool, hero_id, &rows, catalog).await?;
        if detailed {
            print_hero_detail(catalog, hero_id, &buckets);
        } else {
            let all = buckets.iter().find(|bucket| bucket.bucket == BUCKET_ALL);
            let players = all.map(|bucket| bucket.players_raw).unwrap_or(0);
            let staples = all
                .map(|bucket| bucket.items.iter().filter(|item| item.is_staple).count())
                .unwrap_or(0);
            println!(
                "{:<20} Spieler-Matches {players:>7}   Staples {staples:>2}   Buckets {}",
                catalog.hero_name(hero_id),
                buckets.len()
            );
        }
    }
    Ok(())
}

fn print_hero_detail(catalog: &Catalog, hero_id: i64, buckets: &[BucketAgg]) {
    println!("Held {} ({hero_id})", catalog.hero_name(hero_id));
    for bucket in buckets {
        print_bucket(catalog, bucket);
    }
}

fn print_bucket(catalog: &Catalog, bucket: &BucketAgg) {
    println!();
    let cell = if (bucket.players_raw as i64) < MIN_CELL_OBSERVATIONS {
        "  (dünne Zelle, unter Mindestbeobachtung)"
    } else {
        ""
    };
    println!(
        "Bucket {:<9} Spieler roh {:>6}  gewichtet {:>8.1}{cell}",
        bucket.bucket, bucket.players_raw, bucket.players_weighted
    );

    let mut staples: Vec<_> = bucket.items.iter().filter(|item| item.is_staple).collect();
    staples.sort_by(|a, b| b.prevalence_raw.total_cmp(&a.prevalence_raw));
    if staples.is_empty() {
        println!("  Staples: keine ab 70 Prozent");
    } else {
        println!("  Staples ab 70 Prozent:");
        for item in staples {
            println!(
                "    {:<26} {:>5.1}% roh / {:>5.1}% gew.  Pos {:>4.1}  ~{}min  Verkauf {:>4.1}%",
                catalog.item_name(item.item_id),
                item.prevalence_raw * 100.0,
                item.prevalence_weighted * 100.0,
                item.median_position,
                (item.median_buy_time_s / 60.0).round() as i64,
                item.sell_rate * 100.0,
            );
        }
    }

    let mut ranked: Vec<_> = bucket.items.iter().filter(|item| !item.is_staple).collect();
    ranked.sort_by(|a, b| b.prevalence_raw.total_cmp(&a.prevalence_raw));
    if !ranked.is_empty() {
        println!("  Weitere häufige Käufe:");
        for item in ranked.iter().take(8) {
            let next = item
                .next_item_id
                .map(|next| {
                    format!(
                        "  danach {} ({:.0}%)",
                        catalog.item_name(next),
                        item.next_item_share * 100.0
                    )
                })
                .unwrap_or_default();
            println!(
                "    {:<26} {:>5.1}%  Pos {:>4.1}{next}",
                catalog.item_name(item.item_id),
                item.prevalence_raw * 100.0,
                item.median_position,
            );
        }
    }

    if !bucket.imbue.is_empty() {
        println!("  Imbue-Ziele:");
        for imbue in &bucket.imbue {
            let mut flags = String::new();
            if imbue.is_split {
                flags.push_str(" [geteilt]");
            }
            if imbue.is_thin {
                flags.push_str(" [dünn]");
            }
            println!(
                "    {:<26} -> {:<22} {}/{} Imbues{flags}",
                catalog.item_name(imbue.item_id),
                catalog.ability_name(imbue.target_ability_id),
                imbue.target_count,
                imbue.total_imbues,
            );
        }
    }

    if !bucket.ability.is_empty() {
        let order = bucket
            .ability
            .iter()
            .map(|entry| catalog.ability_name(entry.ability_id))
            .collect::<Vec<_>>()
            .join(" > ");
        println!(
            "  Skill-Reihenfolge (Modus): {order}\n    genau gefolgt von {} von {} Spielern",
            bucket.ability_followers, bucket.ability_players
        );
    }
}

async fn run_show(pool: &PgPool, catalog: &Catalog, args: ShowArgs) -> Result<()> {
    let hero_id = catalog
        .resolve_hero(&args.hero)
        .ok_or_else(|| anyhow!("Held '{}' ist im Katalog nicht bekannt.", args.hero))?;
    let index = PopulationIndex::load(pool, hero_id).await?;
    let bucket = args.bucket.as_deref().unwrap_or(BUCKET_ALL);
    if !index.buckets().iter().any(|name| name == bucket) {
        println!(
            "Held {} hat keinen Bucket '{bucket}'. Verfügbar: {}",
            catalog.hero_name(hero_id),
            index.buckets().join(", ")
        );
        return Ok(());
    }

    println!(
        "Held {} ({hero_id}), Bucket {bucket}",
        catalog.hero_name(hero_id)
    );

    let staples = index.staples(bucket);
    if staples.is_empty() {
        println!("Staples: keine ab 70 Prozent");
    } else {
        println!("Staples ab 70 Prozent:");
        for item_id in &staples {
            if let Some(item) = index.item_in(bucket, *item_id) {
                println!(
                    "  {:<26} {:>5.1}%  Pos {:>4.1}",
                    catalog.item_name(*item_id),
                    item.prevalence_raw * 100.0,
                    item.median_position,
                );
            }
        }
    }

    let imbue: Vec<i64> = staples
        .iter()
        .chain(index.population_positions(bucket).iter().map(|(id, _)| id))
        .copied()
        .filter(|id| index.imbue_target_in(bucket, *id).is_some())
        .collect();
    let mut seen = std::collections::HashSet::new();
    let imbue: Vec<i64> = imbue.into_iter().filter(|id| seen.insert(*id)).collect();
    if !imbue.is_empty() {
        println!("Imbue-Ziele:");
        for item_id in imbue {
            if let Some(target) = index.imbue_target_in(bucket, item_id) {
                println!(
                    "  {:<26} -> {}",
                    catalog.item_name(item_id),
                    catalog.ability_name(target),
                );
            }
        }
    }

    let order = index.ability_order_in(bucket);
    if !order.is_empty() {
        let named = order
            .iter()
            .map(|id| catalog.ability_name(*id))
            .collect::<Vec<_>>()
            .join(" > ");
        println!("Skill-Reihenfolge (Modus): {named}");
    }
    Ok(())
}

fn now_unix() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|value| value.as_secs() as i64)
        .unwrap_or(0)
}
