#[path = "site/mod.rs"]
mod site;

use anyhow::{bail, Context, Result};
use clap::Parser;
use std::{net::SocketAddr, path::PathBuf};

#[derive(Parser)]
struct Args {
    #[arg(long)]
    corpus_root: PathBuf,
    #[arg(long)]
    infisical_config: PathBuf,
    #[arg(long, default_value = "127.0.0.1:8087")]
    bind: SocketAddr,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    if !args.bind.ip().is_loopback() {
        bail!("Die Brain-Site darf nur auf Loopback lauschen.");
    }
    let pool = deadlock_brain_core::pg::pg_pool_from_config(&args.infisical_config, false).await?;
    let app = site::router(&args.corpus_root, pool).await?;
    let listener = tokio::net::TcpListener::bind(args.bind)
        .await
        .context("Der lokale Siteport ist nicht verfügbar.")?;
    axum::serve(listener, app)
        .await
        .context("Der Brain-Sitedienst wurde unterbrochen.")
}
