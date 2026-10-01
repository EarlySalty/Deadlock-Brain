use std::path::PathBuf;

use anyhow::Result;
use sqlx::postgres::PgPool;

pub fn repo_root() -> PathBuf {
    if let Ok(directory) = std::env::current_dir() {
        if let Some(root) = directory
            .ancestors()
            .find(|p| p.join("config").is_dir() && p.join("rust").is_dir())
        {
            return root.to_path_buf();
        }
    }
    deadlock_brain_core::config::repo_root()
}

pub fn default_feed_config_path() -> PathBuf {
    repo_root().join("config/youtube_feeds.json")
}

/// Ein Infisical-Snapshot pro Prozess, auch bei geerbtem einmaligem Secret-FD.
static SECRETS: tokio::sync::OnceCell<Vec<(String, zeroize::Zeroizing<String>)>> =
    tokio::sync::OnceCell::const_new();
pub async fn runtime_secrets() -> Result<&'static [(String, zeroize::Zeroizing<String>)]> {
    Ok(SECRETS
        .get_or_try_init(|| async {
            dl_token_secrets::values(&repo_root().join("config/infisical.json")).await
        })
        .await?
        .as_slice())
}
pub async fn pg_pool() -> Result<PgPool> {
    let values = runtime_secrets().await?;
    let config: serde_json::Value =
        serde_json::from_slice(&std::fs::read(repo_root().join("config/infisical.json"))?)?;
    let name = config["database_secret"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Datenbankname fehlt in Konfiguration."))?;
    let dsn = values
        .iter()
        .find(|(k, _)| k == name)
        .map(|(_, v)| v)
        .ok_or_else(|| anyhow::anyhow!("Datenbankzugang fehlt."))?;
    let options: sqlx::postgres::PgConnectOptions = dsn
        .parse()
        .map_err(|_| anyhow::anyhow!("Ungültiger Datenbankzugang."))?;
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(4)
        .acquire_timeout(std::time::Duration::from_secs(15))
        .connect_with(options)
        .await
        .map_err(|_| anyhow::anyhow!("Verbindung zur zentralen Datenbank fehlgeschlagen."))
}

pub fn now_epoch_seconds() -> i64 {
    deadlock_brain_core::now_epoch_seconds().unwrap_or(0)
}
