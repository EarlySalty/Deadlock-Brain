use brain_job::{
    config::Config,
    freshness, ingest_then,
    registry::EXTRACTORS,
    sources::{self, SOURCES},
    GameFactsStore,
};
use brain_storage::PgStore;
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    process::ExitCode,
};

const USAGE: &str =
    "Aufruf: brain-job --config <brain-job.json> ingest <source> [--input <git-repository>] | brain-job --config <brain-job.json> audit-receipts | brain-job --config <brain-job.json> freshness [--since <date>] | brain-job --config <brain-job.json> freshness-history <git-repository>";
const FRESHNESS_SINCE: &str = "2026-08-01";

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(_) => {
            eprintln!("brain-job: Verarbeitung fehlgeschlagen. {USAGE}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> anyhow::Result<()> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() == 1 && args[0] == "--help" {
        println!("{USAGE}");
        return Ok(());
    }
    let audit_config = match args.as_slice() {
        [flag, config, command] if flag == "--config" && command == "audit-receipts" => {
            Some(config)
        }
        [command, flag, config] if command == "audit-receipts" && flag == "--config" => {
            Some(config)
        }
        _ => None,
    };
    if let Some(config) = audit_config {
        return audit(Path::new(config)).await;
    }
    if let Some(result) = freshness_command(&args).await {
        return result;
    }
    let (config_path, source, input) = match args.as_slice() {
        [flag, config, command, source] if flag == "--config" && command == "ingest" => {
            (config, source, None)
        }
        [flag, config, command, source, input_flag, input]
            if flag == "--config" && command == "ingest" && input_flag == "--input" =>
        {
            (config, source, Some(input))
        }
        [command, source, flag, config, input_flag, input]
            if command == "ingest" && flag == "--config" && input_flag == "--input" =>
        {
            (config, source, Some(input))
        }
        _ => anyhow::bail!("Ungültiger Aufruf"),
    };
    let source = source
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Ungültige Quelle"))?;
    let loader = SOURCES
        .iter()
        .find(|(name, _)| *name == source)
        .map(|(_, loader)| *loader)
        .ok_or_else(|| anyhow::anyhow!("Unbekannte Quelle"))?;
    let config = Config::load(Path::new(config_path))?;
    let pool = config.pool().await?;
    let result = async {
        PgStore::new(pool.clone()).check_gamefacts_schema().await?;
        let (objects, repository) = match input {
            Some(repository) => (loader(Path::new(repository), false), PathBuf::from(repository)),
            None => {
                let cache = config
                    .source_cache
                    .as_deref()
                    .ok_or_else(|| anyhow::anyhow!("Quellen-Cache fehlt"))?;
                (sources::fetch(source, cache), cache.join(source))
            }
        };
        let (objects, revision, observed) = tracked(source, objects);
        let recorded = std::sync::atomic::AtomicU64::new(0);
        let (watched, writer, counter) = (&observed, &pool, &recorded);
        let report = ingest_then(
            &GameFactsStore::new(pool.clone()),
            source,
            objects,
            EXTRACTORS,
            |run_id| async move {
                if let Some(observed) = watched {
                    let (paths, digests) = observed.take();
                    let changed =
                        freshness::record_patch_observations(writer, run_id, &paths, &digests)
                            .await?;
                    counter.store(changed, std::sync::atomic::Ordering::Relaxed);
                }
                Ok(())
            },
        )
        .await?;
        println!(
            "run={} accepted={} rejected={} ignored={}",
            report.run_id, report.counts.accepted, report.counts.rejected, report.counts.ignored
        );
        if let Some(revision) = revision {
            freshness::record_ingest(&pool, report.run_id, &repository, &revision).await?;
            println!("revision={revision}");
        }
        if observed.is_some() {
            println!(
                "patch_observations={}",
                recorded.load(std::sync::atomic::Ordering::Relaxed)
            );
        }
        if source == brain_job::item_lineage::SOURCE {
            match brain_job::item_lineage::log_alias_conflicts(&pool).await {
                Ok(conflicts) => println!("alias_conflicts={conflicts}"),
                Err(error) => eprintln!(
                    "{}",
                    serde_json::json!({"event":"item_lineage_alias_conflicts_unavailable","error":error.to_string()})
                ),
            }
        }
        anyhow::Ok(())
    }
    .await;
    pool.close().await;
    result
}

async fn audit(path: &Path) -> anyhow::Result<()> {
    let pool = Config::load(path)?.pool().await?;
    let result = async {
        PgStore::new(pool.clone()).check_gamefacts_schema().await?;
        let report = brain_job::receipt_audit::run(&pool).await?;
        println!(
            "run={} sampled={} verified_receipts={} rejected_receipts={} rejected_objects={}",
            report.run_id,
            report.sampled,
            report.verified_receipts,
            report.rejected_receipts,
            report.rejected_objects
        );
        anyhow::Ok(())
    }
    .await;
    pool.close().await;
    result
}

fn tracked(
    source: &str,
    objects: brain_job::InputObjects,
) -> (
    brain_job::InputObjects,
    Option<String>,
    Option<freshness::Observed>,
) {
    if source == brain_job::patch_extract::SOURCE {
        let observed = freshness::Observed::default();
        return (observed.tap(objects), None, Some(observed));
    }
    if source != freshness::SOURCE {
        return (objects, None, None);
    }
    let objects: Vec<_> = objects.collect();
    let revision = objects
        .iter()
        .find_map(|object| object.as_ref().ok().map(|object| object.revision.clone()));
    (Box::new(objects.into_iter()), revision, None)
}

async fn freshness_command(args: &[OsString]) -> Option<anyhow::Result<()>> {
    let (config, command, rest) = match args {
        [flag, config, command, rest @ ..] if flag == "--config" => (config, command, rest),
        _ => return None,
    };
    let action = match (command.to_str(), rest) {
        (Some("freshness"), []) => Freshness::Report(FRESHNESS_SINCE.to_owned()),
        (Some("freshness"), [flag, since]) if flag == "--since" => {
            Freshness::Report(since.to_str()?.to_owned())
        }
        (Some("freshness-history"), [repository]) => Freshness::History(PathBuf::from(repository)),
        _ => return None,
    };
    Some(freshness(Path::new(config), action).await)
}

enum Freshness {
    Report(String),
    History(PathBuf),
}

async fn freshness(path: &Path, action: Freshness) -> anyhow::Result<()> {
    let pool = Config::load(path)?.pool().await?;
    let result = async {
        PgStore::new(pool.clone()).check_gamefacts_schema().await?;
        match action {
            Freshness::Report(since) => freshness::report(&pool, &since).await,
            Freshness::History(repository) => {
                let (commits, runs) = freshness::backfill(&pool, &repository).await?;
                println!("commits={commits} runs={runs}");
                Ok(())
            }
        }
    }
    .await;
    pool.close().await;
    result
}
