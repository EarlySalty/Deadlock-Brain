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
    AuditReceipts,
}

#[tokio::main]
async fn main() -> std::process::ExitCode {
    let arguments = Arguments::parse();
    let auditing = matches!(&arguments.command, Job::AuditReceipts);
    let result = async {
        let config = Config::load(&arguments.config)?;
        match arguments.command {
            Job::IngestGamefiles => brain_job::ingest(&config).await,
            Job::AuditReceipts => brain_job::receipt_audit::run(&config).await,
        }
    }
    .await;
    match result {
        Ok(counts) => {
            println!("{}", counts.value());
            std::process::ExitCode::SUCCESS
        }
        Err(_) if auditing => {
            eprintln!("Belegprüfung fehlgeschlagen oder unvollständig (audit_failed). Details werden nicht ausgegeben.");
            std::process::ExitCode::FAILURE
        }
        Err(_) => {
            eprintln!(
                "Spielwertimport fehlgeschlagen (ingest_failed). Details werden nicht ausgegeben."
            );
            std::process::ExitCode::FAILURE
        }
    }
}
