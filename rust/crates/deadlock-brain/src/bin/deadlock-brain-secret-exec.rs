use std::{
    ffi::OsString,
    os::unix::process::CommandExt,
    path::PathBuf,
    process::Command,
};

use anyhow::{anyhow, Context, Result};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "deadlock-brain-secret-exec",
    about = "Lädt Deadlock Brain Runtime Secrets über den lokalen Infisical Transport und startet genau einen Prozess.",
    trailing_var_arg = true
)]
struct Args {
    #[arg(long)]
    config: Option<PathBuf>,
    #[arg(required = true, allow_hyphen_values = true)]
    command: Vec<OsString>,
}

fn main() {
    let result = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("Tokio Runtime konnte nicht gestartet werden")
        .and_then(|runtime| runtime.block_on(run()));
    if let Err(error) = result {
        eprintln!("{error:#}");
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let args = Args::parse();
    let config = args
        .config
        .unwrap_or_else(|| deadlock_brain_core::config::repo_root().join("config/infisical.json"));
    let environment = deadlock_brain_core::pg::infisical_environment(&config)
        .await
        .context("Infisical Runtime Umgebung konnte nicht geladen werden")?;

    let (program, command_args) = args
        .command
        .split_first()
        .ok_or_else(|| anyhow!("Ein Zielprogramm ist erforderlich."))?;
    let mut command = Command::new(program);
    command.args(command_args);
    for (name, value) in &environment {
        command.env(name, value.as_str());
    }

    let error = command.exec();
    Err(anyhow!("Zielprogramm konnte nicht gestartet werden: {error}"))
}
