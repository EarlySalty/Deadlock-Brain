mod claims;
mod db;
mod gemini;
mod loop_runner;
mod queue;
mod schema;
mod transcript_claims;
mod transcripts;
mod video_classification;

use std::{fs, path::PathBuf};

use anyhow::Context;
use clap::{Parser, Subcommand};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Parser)]
#[command(
    name = "deadlock-brain-yt",
    about = "Deadlock-Brain YouTube ingestion for curated feeds, transcript handling, classification, and claim storage."
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
    FetchTranscripts {
        #[arg(long, default_value_t = 20)]
        limit: usize,
    },
    ClassifyVideos {
        #[arg(long, default_value_t = 20)]
        limit: usize,
    },
    #[command(about = "Prepare or ingest transcript claim batches")]
    TranscriptClaims {
        #[command(subcommand)]
        action: TranscriptClaimsAction,
    },
}

#[derive(Debug, Subcommand)]
enum TranscriptClaimsAction {
    #[command(about = "Select transcript batches for external claim extraction")]
    Prepare {
        #[arg(long, default_value_t = 20)]
        limit: usize,
        #[arg(long, value_name = "PATH")]
        out: Option<PathBuf>,
        #[arg(long, value_enum, default_value_t = transcript_claims::PrepareOrder::Recent)]
        order: transcript_claims::PrepareOrder,
        #[arg(
            long,
            default_value_t = 150_000,
            hide_default_value = true,
            help = "Maximum transcript characters [default: 150000] (0 = kein Limit)"
        )]
        max_chars: usize,
    },
    #[command(about = "Insert externally verified transcript claims")]
    Ingest {
        #[arg(long = "in", value_name = "PATH")]
        input: PathBuf,
        #[arg(long)]
        write: bool,
        #[arg(long)]
        no_backup: bool,
        #[arg(long, default_value = "claude")]
        model: String,
        #[arg(long, default_value = "youtube_claims_de_transcript_v1")]
        prompt_version: String,
    },
    #[command(about = "Mark zero-yield transcript claim attempts without LLM work")]
    BackfillAttempts {
        #[arg(
            long,
            default_value_t = 150_000,
            hide_default_value = true,
            help = "Maximum transcript characters [default: 150000] (0 = kein Limit)"
        )]
        max_chars: usize,
        #[arg(long, default_value = "youtube_claims_de_transcript_v1")]
        prompt_version: String,
        #[arg(long)]
        write: bool,
        #[arg(long)]
        no_backup: bool,
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
        Commands::FetchTranscripts { limit } => {
            let conn = db::open_db(cli.db)?;
            print_json(&transcripts::fetch_transcripts(&conn, limit)?)
        }
        Commands::ClassifyVideos { limit } => {
            let conn = db::open_db(cli.db)?;
            print_json(&video_classification::classify_videos(&conn, limit)?)
        }
        Commands::TranscriptClaims { action } => match action {
            TranscriptClaimsAction::Prepare {
                limit,
                out,
                order,
                max_chars,
            } => {
                let conn = db::open_db(cli.db)?;
                let max_chars = if max_chars == 0 {
                    None
                } else {
                    Some(max_chars)
                };
                let summary = transcript_claims::prepare(&conn, limit, order, max_chars)?;
                if let Some(out) = out {
                    let content = serde_json::to_string_pretty(&summary)?;
                    fs::write(&out, format!("{content}\n"))
                        .with_context(|| format!("write prepare output {}", out.display()))?;
                    Ok(())
                } else {
                    print_json(&summary)
                }
            }
            TranscriptClaimsAction::Ingest {
                input,
                write,
                no_backup,
                model,
                prompt_version,
            } => {
                let mut conn = db::open_db(cli.db)?;
                let summary = transcript_claims::ingest(
                    &mut conn,
                    &input,
                    transcript_claims::IngestOptions {
                        write,
                        no_backup,
                        model,
                        prompt_version,
                    },
                )?;
                print_json(&summary)
            }
            TranscriptClaimsAction::BackfillAttempts {
                max_chars,
                prompt_version,
                write,
                no_backup,
            } => {
                let max_chars = if max_chars == 0 {
                    None
                } else {
                    Some(max_chars)
                };
                let mut conn = db::open_db(cli.db)?;
                let summary = transcript_claims::backfill_attempts(
                    &mut conn,
                    transcript_claims::BackfillAttemptsOptions {
                        write,
                        no_backup,
                        prompt_version,
                        max_chars,
                    },
                )?;
                print_json(&summary)
            }
        },
    }
}

fn print_json<T: Serialize>(value: &T) -> anyhow::Result<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
