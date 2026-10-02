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
    Tick,
    Status,
    Migrate,
    ImportReviewed,
    RegisterConfig,
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
        eprintln!("Dokumentpflege fehlgeschlagen. Der gespeicherte Status enthält den Fehlercode.");
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
    let runner = Runner::open(runtime).await?;
    let result = match cli.command {
        Task::Tick => runner.tick().await?,
        Task::Status => runner.status().await?,
        Task::ImportReviewed => runner.import_reviewed().await?,
        Task::RegisterConfig => runner.register_config().await?,
        Task::Migrate => {
            runner.migrate().await?;
            serde_json::json!({"status":"migrated"})
        }
        Task::Query { text } => runner.query(&text).await?,
        Task::ProjectHtml { .. } => unreachable!(),
    };
    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(())
}
