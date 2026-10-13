use brain_job::{
    config::Config,
    ingest,
    registry::EXTRACTORS,
    sources::{self, SOURCES},
    GameFactsStore,
};
use brain_storage::PgStore;
use std::{path::Path, process::ExitCode};

const USAGE: &str =
    "Aufruf: brain-job --config <brain-job.json> ingest <source> [--input <git-repository>] | brain-job --config <brain-job.json> audit-receipts";

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
        let objects = match input {
            Some(repository) => loader(Path::new(repository), false),
            None => sources::fetch(
                source,
                config
                    .source_cache
                    .as_deref()
                    .ok_or_else(|| anyhow::anyhow!("Quellen-Cache fehlt"))?,
            ),
        };
        let report = ingest(
            &GameFactsStore::new(pool.clone()),
            source,
            objects,
            EXTRACTORS,
        )
        .await?;
        println!(
            "run={} accepted={} rejected={} ignored={}",
            report.run_id, report.counts.accepted, report.counts.rejected, report.counts.ignored
        );
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
