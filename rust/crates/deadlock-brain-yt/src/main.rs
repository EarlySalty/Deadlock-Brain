mod claims;
mod db;
mod gemini;
mod loop_runner;
mod queue;
mod transcript_claims;
mod transcripts;
mod video_classification;

#[cfg(test)]
mod testutil;

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
    AutoLearn {
        #[arg(long, default_value = "config/youtube_feeds.json")]
        config: PathBuf,
        #[arg(long = "discover-limit", default_value_t = 50)]
        discover_limit: usize,
        #[arg(long = "transcript-fetch-limit", default_value_t = 20)]
        transcript_fetch_limit: usize,
        #[arg(long = "analyze-limit", default_value_t = 5)]
        analyze_limit: usize,
        #[arg(long = "no-fetch-transcripts")]
        no_fetch_transcripts: bool,
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
        #[arg(long, value_enum, default_value_t = transcript_claims::PrepareMode::Normal)]
        mode: transcript_claims::PrepareMode,
        #[arg(
            long,
            default_value_t = transcript_claims::MONSTER_CHAR_THRESHOLD,
            hide_default_value = true,
            help = "Maximum transcript characters [default: 150000] (0 = kein Limit)"
        )]
        max_chars: usize,
        #[arg(
            long,
            default_value_t = transcript_claims::MONSTER_CHAR_THRESHOLD,
            hide_default_value = true,
            help = "Minimum transcript characters for monster mode [default: 150000] (0 = kein Minimum)"
        )]
        min_chars: usize,
        #[arg(long, default_value_t = 60_000)]
        chunk_chars: usize,
        #[arg(long, default_value_t = 4_000)]
        overlap_chars: usize,
    },
    #[command(about = "Insert externally verified transcript claims")]
    Ingest {
        #[arg(long = "in", value_name = "PATH")]
        input: PathBuf,
        #[arg(long)]
        write: bool,
        #[arg(long, default_value = "claude")]
        model: String,
        #[arg(long, default_value = "youtube_claims_de_transcript_v1")]
        prompt_version: String,
    },
    #[command(about = "Mark zero-yield transcript claim attempts without LLM work")]
    BackfillAttempts {
        #[arg(
            long,
            default_value_t = transcript_claims::MONSTER_CHAR_THRESHOLD,
            hide_default_value = true,
            help = "Maximum transcript characters [default: 150000] (0 = kein Limit)"
        )]
        max_chars: usize,
        #[arg(long, default_value = "youtube_claims_de_transcript_v1")]
        prompt_version: String,
        #[arg(long)]
        write: bool,
    },
    #[command(about = "Mark monster transcript claim candidates as off-topic")]
    MarkOfftopic {
        #[arg(long = "title-contains", value_name = "STR")]
        title_contains: Vec<String>,
        #[arg(long = "video-ids", value_name = "PATH")]
        video_ids: Option<PathBuf>,
        #[arg(long, default_value = "youtube_claims_de_transcript_v1")]
        prompt_version: String,
        #[arg(long)]
        write: bool,
    },
}

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("{error:#}");
        std::process::exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::GeminiLogin => gemini::run_login(),
        Commands::Ingest { limit } => {
            let pool = db::pg_pool().await?;
            print_json(&loop_runner::run_ingest(&pool, limit).await?)
        }
        Commands::Claims { entity, pretty } => {
            let pool = db::pg_pool().await?;
            let rows = claims::query_claims(&pool, &entity).await?;
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
            let pool = db::pg_pool().await?;
            print_json(&transcripts::fetch_transcripts(&pool, limit).await?)
        }
        Commands::ClassifyVideos { limit } => {
            let pool = db::pg_pool().await?;
            print_json(&video_classification::classify_videos(&pool, limit).await?)
        }
        Commands::AutoLearn {
            config,
            discover_limit,
            transcript_fetch_limit,
            analyze_limit,
            no_fetch_transcripts,
        } => {
            let pool = db::pg_pool().await?;
            let discover = queue::discover_youtube_videos(&pool, &config, discover_limit).await?;
            let transcripts = if no_fetch_transcripts {
                None
            } else {
                Some(transcripts::fetch_transcripts(&pool, transcript_fetch_limit).await?)
            };
            let classification =
                video_classification::classify_videos(&pool, transcript_fetch_limit).await?;
            let ingest =
                loop_runner::run_ingest_after_discover(&pool, analyze_limit, discover).await?;
            print_json(&json!({
                "transcripts": transcripts,
                "classification": classification,
                "ingest": ingest,
            }))
        }
        Commands::TranscriptClaims { action } => match action {
            TranscriptClaimsAction::Prepare {
                limit,
                out,
                order,
                mode,
                max_chars,
                min_chars,
                chunk_chars,
                overlap_chars,
            } => {
                let pool = db::pg_pool().await?;
                if mode == transcript_claims::PrepareMode::Monster {
                    let summary = transcript_claims::prepare_monster(
                        &pool,
                        limit,
                        order,
                        min_chars,
                        chunk_chars,
                        overlap_chars,
                    )
                    .await?;
                    if let Some(out) = out {
                        let content = serde_json::to_string_pretty(&summary)?;
                        fs::write(&out, format!("{content}\n")).with_context(|| {
                            format!("write monster prepare output {}", out.display())
                        })?;
                        Ok(())
                    } else {
                        print_json(&summary)
                    }
                } else {
                    let max_chars = if max_chars == 0 {
                        None
                    } else {
                        Some(max_chars)
                    };
                    let summary =
                        transcript_claims::prepare(&pool, limit, order, max_chars).await?;
                    if let Some(out) = out {
                        let content = serde_json::to_string_pretty(&summary)?;
                        fs::write(&out, format!("{content}\n"))
                            .with_context(|| format!("write prepare output {}", out.display()))?;
                        Ok(())
                    } else {
                        print_json(&summary)
                    }
                }
            }
            TranscriptClaimsAction::Ingest {
                input,
                write,
                model,
                prompt_version,
            } => {
                let pool = db::pg_pool().await?;
                let summary = transcript_claims::ingest(
                    &pool,
                    &input,
                    transcript_claims::IngestOptions {
                        write,
                        model,
                        prompt_version,
                    },
                )
                .await?;
                print_json(&summary)
            }
            TranscriptClaimsAction::BackfillAttempts {
                max_chars,
                prompt_version,
                write,
            } => {
                let max_chars = if max_chars == 0 {
                    None
                } else {
                    Some(max_chars)
                };
                let pool = db::pg_pool().await?;
                let summary = transcript_claims::backfill_attempts(
                    &pool,
                    transcript_claims::BackfillAttemptsOptions {
                        write,
                        prompt_version,
                        max_chars,
                    },
                )
                .await?;
                print_json(&summary)
            }
            TranscriptClaimsAction::MarkOfftopic {
                title_contains,
                video_ids,
                prompt_version,
                write,
            } => {
                let pool = db::pg_pool().await?;
                let summary = transcript_claims::mark_offtopic(
                    &pool,
                    transcript_claims::MarkOfftopicOptions {
                        write,
                        prompt_version,
                        title_contains,
                        video_ids_path: video_ids,
                    },
                )
                .await?;
                print_json(&summary)
            }
        },
    }
}

fn print_json<T: Serialize>(value: &T) -> anyhow::Result<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
