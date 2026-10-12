use brain_job::{ingest, registry::EXTRACTORS, sources::SOURCES, GameFactsStore};
use brain_storage::PgStore;
use serde::Deserialize;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use std::{path::PathBuf, process::ExitCode, time::Duration};

const USAGE: &str =
    "Aufruf: brain-job ingest <source> --config <local-postgres.json> --input <git-repository> | brain-job audit-receipts --config <local-postgres.json>";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    socket: PathBuf,
    port: u16,
    database: String,
    user: String,
}

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
    let audit = args.len() == 3 && args[0] == "audit-receipts" && args[1] == "--config";
    anyhow::ensure!(
        audit
            || (args.len() == 6
                && args[0] == "ingest"
                && args[2] == "--config"
                && args[4] == "--input"),
        "Ungültiger Aufruf"
    );
    let ingestion = if audit {
        None
    } else {
        let source = args[1]
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Ungültige Quelle"))?;
        let loader = SOURCES
            .iter()
            .find(|(name, _)| *name == source)
            .map(|(_, loader)| *loader)
            .ok_or_else(|| anyhow::anyhow!("Unbekannte Quelle"))?;
        Some((source, loader))
    };
    anyhow::ensure!(
        std::env::var_os("PGOPTIONS").is_none(),
        "PGOPTIONS ist unzulässig"
    );
    let config: Config =
        serde_json::from_reader(std::fs::File::open(&args[if audit { 2 } else { 3 }])?)?;
    anyhow::ensure!(
        config.socket.is_absolute()
            && config.port != 0
            && [&config.database, &config.user]
                .iter()
                .all(|s| !s.trim().is_empty()
                    && s.len() <= 128
                    && !s.chars().any(char::is_control)),
        "Ungültige lokale Datenbankkonfiguration"
    );
    let options = PgConnectOptions::new_without_pgpass()
        .host(&config.socket.to_string_lossy())
        .port(config.port)
        .database(&config.database)
        .username(&config.user)
        .password("")
        .ssl_mode(PgSslMode::Disable)
        .application_name("brain-job");
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .acquire_timeout(Duration::from_secs(5))
        .connect_with(options)
        .await?;
    let result = async {
        PgStore::new(pool.clone()).check_gamefacts_schema().await?;
        if audit {
            let report = brain_job::receipt_audit::run(&pool).await?;
            println!(
                "run={} sampled={} verified_receipts={} rejected_receipts={} rejected_objects={}",
                report.run_id,
                report.sampled,
                report.verified_receipts,
                report.rejected_receipts,
                report.rejected_objects
            );
            return anyhow::Ok(());
        }
        let (source, loader) = ingestion.ok_or_else(|| anyhow::anyhow!("Ungültiger Aufruf"))?;
        let input = PathBuf::from(&args[5]);
        let report = ingest(
            &GameFactsStore::new(pool.clone()),
            source,
            std::iter::once_with(|| loader(&input)),
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
