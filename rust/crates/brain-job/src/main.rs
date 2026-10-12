use brain_job::config::Config;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
struct Arguments {
    #[arg(long)]
    config: PathBuf,
    #[command(subcommand)]
    command: Job,
}
#[derive(Subcommand)]
enum Job {
    IngestGamefiles,
    IngestDocs,
}

#[tokio::main]
async fn main() -> std::process::ExitCode {
    let arguments = Arguments::parse();
    let result = async {
        let config = Config::load(&arguments.config)?;
        match arguments.command {
            Job::IngestGamefiles => brain_job::ingest(&config).await,
            Job::IngestDocs => brain_job::docs::ingest(&config).await,
        }
    }
    .await;
    match result {
        Ok(counts) => {
            println!("{}", counts.value());
            std::process::ExitCode::SUCCESS
        }
        Err(_) => {
            eprintln!(
                "Spielwertimport fehlgeschlagen (ingest_failed). Details werden nicht ausgegeben."
            );
            std::process::ExitCode::FAILURE
        }
    }
}
