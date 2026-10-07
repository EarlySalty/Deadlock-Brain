use anyhow::{ensure, Result};
use brain_maintenance::{
    digest,
    integration::{
        runner::Runner,
        runtime_config::{read_bounded, RuntimeConfig},
    },
};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    config: Option<PathBuf>,
    #[arg(long)]
    infisical_config: Option<PathBuf>,
    #[command(subcommand)]
    command: Task,
}
#[derive(Subcommand)]
enum Task {
    Tick {
        #[arg(long)]
        job_id: Option<String>,
    },
    ResumeEntityProfiles {
        #[arg(long)]
        original_release_id: String,
        #[arg(long)]
        original_release_sha256: String,
        #[arg(long)]
        expected_serve_config_sha256: String,
    },
    Status,
    Migrate,
    ImportReviewed {
        #[arg(long)]
        public_docs: bool,
    },
    ImportC9Release {
        #[arg(long)]
        documents: PathBuf,
        #[arg(long)]
        expected_sha256: String,
        #[arg(long, value_parser = ["docs", "second-brain"])]
        kind: String,
    },
    RegisterConfig,
    WriteServeConfig {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        expected_sha256: String,
    },
    ProjectHtml {
        #[arg(long)]
        input: PathBuf,
    },
    Query {
        #[arg(long)]
        text: String,
    },
}
#[tokio::main]
async fn main() {
    if run().await.is_err() {
        eprintln!("Dokumentpflege fehlgeschlagen: maintenance_command_failed.");
        std::process::exit(1);
    }
}
async fn run() -> Result<()> {
    let cli = Cli::parse();
    if let Task::ProjectHtml { input } = &cli.command {
        let bytes = read_bounded(input, 4 * 1024 * 1024)?;
        let html = std::str::from_utf8(&bytes)?;
        let text = brain_maintenance::html::visible_text(html);
        let projection = dbrain_retrieval::html_projection::project_html(html)?;
        println!(
            "{}",
            serde_json::to_string(
                &serde_json::json!({"html_sha256":digest(&bytes),"text_sha256":digest(text.as_bytes()),"visible_text":text,
            "reader_text_sha256":projection.semantic_sha256,"reader_text":projection.text,
            "parser_revision":dbrain_retrieval::html_projection::HTML_PROJECTION_VERSION})
            )?
        );
        return Ok(());
    }
    let path = cli
        .config
        .ok_or_else(|| anyhow::anyhow!("runtime_config_required"))?;
    let runtime = RuntimeConfig::load(&path)?;
    if let Some(path) = cli.infisical_config {
        ensure!(
            path == runtime.infisical_config,
            "infisical_config_mismatch"
        );
    }
    if let Task::WriteServeConfig {
        input,
        expected_sha256,
    } = &cli.command
    {
        let input = input.clone();
        let expected_sha256 = expected_sha256.clone();
        let result = tokio::task::spawn_blocking(move || {
            brain_maintenance::integration::config_writer::write_serve_config(
                &runtime,
                &input,
                &expected_sha256,
            )
        })
        .await??;
        println!("{}", serde_json::to_string(&result)?);
        return Ok(());
    }
    let runner = Runner::open(runtime).await?;
    let check_tick = matches!(&cli.command, Task::Tick { .. });
    let result = match cli.command {
        Task::Tick { job_id } => runner.tick_for_job(job_id.as_deref()).await?,
        Task::ResumeEntityProfiles {
            original_release_id,
            original_release_sha256,
            expected_serve_config_sha256,
        } => {
            runner
                .resume_entity_profiles(
                    &original_release_id,
                    &original_release_sha256,
                    &expected_serve_config_sha256,
                )
                .await?
        }
        Task::Status => runner.status().await?,
        Task::ImportReviewed { public_docs } => runner.import_reviewed(public_docs).await?,
        Task::ImportC9Release {
            documents,
            expected_sha256,
            kind,
        } => {
            runner
                .import_c9_release(&documents, &expected_sha256, &kind)
                .await?
        }
        Task::RegisterConfig => runner.register_config().await?,
        Task::Migrate => {
            runner.migrate().await?;
            serde_json::json!({"status":"migrated"})
        }
        Task::Query { text } => runner.query(&text).await?,
        Task::ProjectHtml { .. } => unreachable!(),
        Task::WriteServeConfig { .. } => unreachable!(),
    };
    println!("{}", serde_json::to_string_pretty(&result)?);
    if check_tick {
        ensure!(tick_succeeded(&result), "maintenance_tick_failed");
    }
    Ok(())
}

fn tick_succeeded(result: &serde_json::Value) -> bool {
    result
        .get("isolated_errors")
        .and_then(serde_json::Value::as_array)
        .is_some_and(Vec::is_empty)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn tick_failure_is_not_success() {
        assert!(tick_succeeded(&json!({"isolated_errors":[],"jobs":[]})));
        assert!(!tick_succeeded(
            &json!({"isolated_errors":[{"code":"ENTITY_PROFILE_REFRESH_FAILED"}],"jobs":[]})
        ));
        assert!(tick_succeeded(
            &json!({"isolated_errors":[],"jobs":[{"status":"failed"}]})
        ));
        assert!(!tick_succeeded(&json!({})));
    }
}
