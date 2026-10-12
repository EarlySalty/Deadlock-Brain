use brain_job::{ingest, registry::EXTRACTORS, sources::SOURCES, GameFactsStore};
use brain_storage::PgStore;
use serde::Deserialize;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use std::{path::PathBuf, process::ExitCode, time::Duration};

const USAGE: &str =
    "Aufruf: brain-job ingest <source> --config <local-postgres.json> --input <git-repository>";

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
    anyhow::ensure!(
        args.len() == 6 && args[0] == "ingest" && args[2] == "--config" && args[4] == "--input",
        "Ungültiger Aufruf"
    );
    let source = args[1]
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Ungültige Quelle"))?;
    let loader = SOURCES
        .iter()
        .find(|(name, _)| *name == source)
        .map(|(_, loader)| *loader)
        .ok_or_else(|| anyhow::anyhow!("Unbekannte Quelle"))?;
    anyhow::ensure!(
        std::env::var_os("PGOPTIONS").is_none(),
        "PGOPTIONS ist unzulässig"
    );
    let config: Config = serde_json::from_reader(std::fs::File::open(&args[3])?)?;
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
