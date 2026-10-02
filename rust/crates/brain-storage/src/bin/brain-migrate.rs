//! Administrative entrypoint, intentionally separate from service startup.
//! Local Unix socket/peer auth only. No password, DSN, environment fallback, or secret file.
use brain_contracts::store::STORE_VERSION;
use brain_storage::{PgStore, CORE_SCHEMA_VERSION};
use serde::Deserialize;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use std::{path::PathBuf, process::ExitCode, time::Duration};

const USAGE: &str = "Usage: brain-migrate <check|up|guide-check|guide-up|guide-import> --config <non-secret-local-postgres.json>";
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    socket: PathBuf,
    port: u16,
    database: String,
    user: String,
    #[serde(default)]
    guide_retention_seconds: Option<i64>,
}
impl Config {
    fn validate(&self) -> Result<(), &'static str> {
        if !self.socket.is_absolute()
            || self.port == 0
            || [&self.database, &self.user]
                .iter()
                .any(|s| s.trim().is_empty() || s.len() > 128 || s.chars().any(char::is_control))
        {
            return Err("invalid local PostgreSQL configuration");
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() == 1 && args[0] == "--help" {
        println!("{USAGE}");
        return ExitCode::SUCCESS;
    }
    if args.len() != 3
        || args[1] != "--config"
        || !["check", "up", "guide-check", "guide-up", "guide-import"]
            .iter()
            .any(|a| args[0] == *a)
    {
        eprintln!("{USAGE}");
        return ExitCode::FAILURE;
    }
    match run(
        args[0].to_str().unwrap_or_default(),
        PathBuf::from(&args[2]),
    )
    .await
    {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("brain-migrate: {message}");
            ExitCode::FAILURE
        }
    }
}

async fn run(action: &str, path: PathBuf) -> Result<(), String> {
    if std::env::var_os("PGOPTIONS").is_some() {
        return Err("ambient PGOPTIONS is not allowed; use the explicit local config".into());
    }
    let file = std::fs::File::open(path).map_err(|_| "cannot read migration config")?;
    let config: Config =
        serde_json::from_reader(file).map_err(|_| "invalid migration config JSON")?;
    config.validate()?;
    // An explicit empty password prevents ambient credentials from being used. All connection
    // coordinates are mandatory; this executable is for local peer-authenticated maintenance.
    let options = PgConnectOptions::new_without_pgpass()
        .host(&config.socket.to_string_lossy())
        .port(config.port)
        .database(&config.database)
        .username(&config.user)
        .password("")
        .ssl_mode(PgSslMode::Disable)
        .application_name("brain-migrate");
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(5))
        .connect_with(options)
        .await
        .map_err(|_| "local PostgreSQL connection failed (check peer auth and role)")?;
    let store = PgStore::new(pool.clone());
    let result = match action {
        "up" => store.migrate_core().await,
        "guide-up" => store.migrate_guide().await,
        "guide-check" => store.check_guide().await,
        "guide-import" => {
            store
                .import_guide_legacy(config.guide_retention_seconds)
                .await
        }
        _ => store.check_core_schema().await,
    };
    pool.close().await;
    result.map_err(|e| e.to_string())?;
    if action.starts_with("guide-") {
        println!(
            "Serverguide-Schema v1: {}",
            match action {
                "guide-import" => "zulässiger Import abgeschlossen",
                "guide-up" => "Migration ausgeführt",
                _ => "kompatibel, nur geprüft",
            }
        );
        return Ok(());
    }
    println!(
        "core schema v{CORE_SCHEMA_VERSION} / {STORE_VERSION}: {}",
        if action.ends_with("up") {
            "upgrade verified"
        } else {
            "compatible (read-only check)"
        }
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn config_is_explicit_local_and_rejects_secrets() {
        let valid =
            r#"{"socket":"/scratch/socket","port":55441,"database":"scratch","user":"owner"}"#;
        let config: Config = serde_json::from_str(valid).unwrap();
        assert!(config.validate().is_ok());
        assert!(serde_json::from_str::<Config>(&valid.replace("\"port\":55441,", "")).is_err());
        assert!(serde_json::from_str::<Config>(&valid.replace(
            "\"user\":\"owner\"",
            "\"user\":\"owner\",\"password\":\"not-allowed\""
        ))
        .is_err());
        let remote: Config =
            serde_json::from_str(&valid.replace("/scratch/socket", "localhost")).unwrap();
        assert!(remote.validate().is_err());
    }
}
