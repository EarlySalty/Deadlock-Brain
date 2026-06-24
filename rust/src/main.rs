mod claims;
mod db;
mod gemini;
mod loop_runner;
mod queue;
mod schema;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Parser)]
#[command(
    name = "deadlock-brain-yt",
    about = "Deadlock-Brain YouTube-Ingestion: holt Videos aus den kuratierten Feeds, lässt sie von Gemini analysieren und speichert die Erkenntnisse in der SQLite-Wissens-DB."
)]
struct Cli {
    #[arg(long, global = true, value_name = "PATH")]
    db: Option<PathBuf>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    GeminiLogin,
    Ingest {
        #[arg(long, default_value_t = 5)]
        limit: usize,
    },
    Claims {
        entity: String,
        #[arg(long)]
        pretty: bool,
    },
    Smoke {
        #[arg(long)]
        url: String,
    },
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error:#}");
        std::process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::GeminiLogin => gemini::run_login(),
        Commands::Ingest { limit } => {
            let conn = db::open_db(cli.db)?;
            print_json(&loop_runner::run_ingest(&conn, limit)?)
        }
        Commands::Claims { entity, pretty } => {
            let conn = db::open_db(cli.db)?;
            let rows = claims::query_claims(&conn, &entity)?;
            if pretty {
                for row in rows {
                    println!(
                        "- [{}] {} (source={}, confidence={:.2})",
                        row.claim_type,
                        row.assertion,
                        row.source_channel.unwrap_or_default(),
                        row.confidence
                    );
                }
                Ok(())
            } else {
                print_json(&rows)
            }
        }
        Commands::Smoke { url } => {
            let prompt = claims::build_prompt(&url);
            match gemini::analyze_url(&url, &prompt) {
                Ok(raw_text) => print_json(&json!({
                    "status": "ok",
                    "raw_text": raw_text,
                    "claims": claims::parse_model_claims(&raw_text),
                })),
                Err(error) => {
                    println!(
                        "{}",
                        serde_json::to_string(&json!({
                            "status": "error",
                            "kind": error.kind.to_string(),
                            "message": error.message,
                        }))?
                    );
                    std::process::exit(1);
                }
            }
        }
    }
}

fn print_json<T: Serialize>(value: &T) -> anyhow::Result<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
