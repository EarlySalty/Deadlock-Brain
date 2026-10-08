#![forbid(unsafe_code)]

#[path = "site/mod.rs"]
mod site;

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[command(flatten)]
    site: site::Args,
}

#[tokio::main]
async fn main() -> Result<()> {
    site::serve(Args::parse().site).await
}
