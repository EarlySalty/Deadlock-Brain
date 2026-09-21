//! Populate only a newly reserved asset-evaluation database, using the same
//! catalog classification as production build-data sync. No source fetch here.
use anyhow::{ensure, Context, Result};
use serde_json::Value;
use sqlx::postgres::PgPoolOptions;
use std::collections::BTreeSet;

fn reserved_database(name: &str) -> bool {
    name.strip_prefix("brain_assets_eval_")
        .is_some_and(|suffix| {
            !suffix.is_empty()
                && suffix
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || byte == b'_')
        })
}

async fn run() -> Result<()> {
    let dsn = std::env::var("DEADLOCK_CENTRAL_DSN").context("authorized access required")?;
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&dsn)
        .await?;
    let name: String = sqlx::query_scalar("SELECT current_database()")
        .fetch_one(&pool)
        .await?;
    ensure!(reserved_database(&name), "not an asset-evaluation database");
    let rows: Vec<String> = sqlx::query_scalar(
        "SELECT payload::text FROM brain.entity_snapshots WHERE source='deadlock_assets_api' AND entity_type='item_or_ability' ORDER BY external_id"
    ).fetch_all(&pool).await?;
    let items: Vec<Value> = rows
        .iter()
        .map(|row| serde_json::from_str(row))
        .collect::<std::result::Result<_, _>>()?;
    ensure!(!items.is_empty(), "no imported assets");
    let mut versions = BTreeSet::new();
    for item in &items {
        versions.insert(
            item.get("_source_client_version")
                .and_then(Value::as_u64)
                .context("missing asset revision")?,
        );
    }
    ensure!(versions.len() == 1, "mixed asset revisions");
    let count = dbrain_builds::upsert_item_catalog(&pool, &Value::Array(items)).await?;
    println!(
        "SCRATCH_CATALOG_VERIFIED: {count} items; client_version={}",
        versions.first().unwrap()
    );
    pool.close().await;
    Ok(())
}

#[tokio::main]
async fn main() {
    if run().await.is_err() {
        // A connection error can include connection parameters. Never print it.
        eprintln!("Scratch-Katalog fehlgeschlagen; keine Verbindungsdaten ausgegeben.");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::reserved_database;

    #[test]
    fn catalog_writes_require_a_reserved_scratch_database() {
        assert!(reserved_database("brain_assets_eval_20260921_123_456"));
        for name in [
            "deadlock_central",
            "brain_assets_eval_",
            "brain_assets_eval_production",
            "brain_assets_eval_123;drop",
        ] {
            assert!(!reserved_database(name));
        }
    }
}
