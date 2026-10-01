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
struct RuntimeSecrets {
    source: PathBuf,
    values: Vec<(String, zeroize::Zeroizing<String>)>,
}
static SECRETS: tokio::sync::OnceCell<RuntimeSecrets> = tokio::sync::OnceCell::const_new();
pub async fn runtime_secrets(
    config: &std::path::Path,
) -> Result<&'static [(String, zeroize::Zeroizing<String>)]> {
    let source = config.canonicalize()?;
    let snapshot = SECRETS
        .get_or_try_init(|| async {
            Ok::<_, anyhow::Error>(RuntimeSecrets {
                values: dl_token_secrets::values(&source).await?,
                source: source.clone(),
            })
        })
        .await?;
    if snapshot.source != source {
        anyhow::bail!("Abweichende Secret-Konfiguration im selben Prozess.");
    }
    Ok(snapshot.values.as_slice())
}
pub async fn pg_pool() -> Result<PgPool> {
    let source = crate::gemini::infisical_config()?;
    let values = runtime_secrets(&source).await?;
    let config: serde_json::Value = serde_json::from_slice(&std::fs::read(source)?)?;
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
