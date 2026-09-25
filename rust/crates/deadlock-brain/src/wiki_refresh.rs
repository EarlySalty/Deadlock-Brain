//! Ein Quellenimport veröffentlicht erst nach vollständiger Prüfung eine neue Wikiwurzel.
use std::{
    fs,
    fs::OpenOptions,
    io::Write,
    os::unix::fs::symlink,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};
use chrono::Utc;
use clap::Args;
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

#[derive(Debug, Args)]
pub struct RefreshArgs {
    #[arg(long)]
    pub config: PathBuf,
    #[arg(
        long,
        help = "Nur vorhandene Daten exportieren; kein neuer Quellenabruf."
    )]
    pub skip_source_update: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    infisical_config: PathBuf,
    source_repository: PathBuf,
    raw_directory: PathBuf,
    publication_root: PathBuf,
    #[serde(default)]
    wiki: Option<dbrain_sources::wiki_corpus::WikiCorpusOptions>,
}

impl Config {
    fn read(path: &Path) -> Result<Self> {
        let bytes = fs::read(path).context("Wiki-Konfiguration ist nicht lesbar")?;
        if bytes.len() > 64 * 1024 {
            bail!("Wiki-Konfiguration ist zu groß");
        }
        let config: Self =
            serde_json::from_slice(&bytes).context("Wiki-Konfiguration ist ungültig")?;
        for path in [
            &config.infisical_config,
            &config.source_repository,
            &config.raw_directory,
            &config.publication_root,
        ] {
            if !path.is_absolute() {
                bail!("Wiki-Konfigurationspfade müssen absolut sein");
            }
        }
        Ok(config)
    }
}

/// Kein Legacy-settings-/ENV-Pfad: gewöhnliche JSON-Konfiguration und Infisical-Pool.
pub async fn run(args: &RefreshArgs) -> Result<Value> {
    let config = Config::read(&args.config)?;
    fs::create_dir_all(&config.publication_root)
        .context("Wiki-Ziel ist nicht verfügbar")?;
    let lock = OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(config.publication_root.join("refresh.lock"))?;
    lock.try_lock().context("Ein Wiki-Refresh läuft bereits")?;
    let result = refresh(&config, args.skip_source_update).await;
    if result.is_err() {
        // Kein Fehlertext aus Fremdbibliotheken im Status; der alte Snapshot bleibt sichtbar.
        atomic_json(&config.publication_root.join("last-attempt.json"), &json!({
            "state": "failed", "attempted_at": Utc::now().to_rfc3339(),
            "message": "Quellenabruf oder Wiki-Veröffentlichung fehlgeschlagen. Der tatsächlich aktive Stand steht unter current/status.json."
        })).context("Wiki-Fehlerstatus konnte nicht gespeichert werden")?;
    }
    result
}

async fn refresh(config: &Config, skip_source_update: bool) -> Result<Value> {
    let pool =
        deadlock_brain_core::pg::pg_pool_from_config(&config.infisical_config, false)
            .await?;
    let source_update = if skip_source_update {
        Value::Null
    } else {
        dbrain_sources::deadlock_data::pull_deadlock_data_with_pool(
            &pool,
            &config.raw_directory,
            dbrain_sources::PullDeadlockDataOptions {
                repo_dir: config.source_repository.clone(),
                commit: std::env::var("DBRAIN_DEADLOCK_DATA_COMMIT")
                    .context("Wiki-Quellenimport benötigt DBRAIN_DEADLOCK_DATA_COMMIT; kein implizites HEAD")?,
                update_repo: false,
            },
        )
        .await
        .context("Deadlock-Quellenimport fehlgeschlagen")?
    };
    let wiki_update = if !skip_source_update {
        if let Some(options) = config.wiki.as_ref().filter(|options| options.enabled) {
            let http = crate::http_client_async(
                "Deadlock-Brain/1.0 (+https://github.com/EarlySalty/Deadlock-Brain)"
                    .into(),
                config.raw_directory.join("http-cache"),
            )
            .await?;
            dbrain_sources::wiki_corpus::pull_wiki_corpus_with_pool(
                &pool,
                &config.raw_directory,
                &http,
                options,
            )
            .await
            .context(
                "Deadlock-Wiki-Import fehlgeschlagen; bisheriger Snapshot bleibt aktiv",
            )?
        } else {
            Value::Null
        }
    } else {
        Value::Null
    };
    let source_update = if skip_source_update {
        Value::Null
    } else {
        json!({"deadlock_data":source_update,"deadlock_wiki":wiki_update})
    };
    let snapshots = config.publication_root.join("snapshots");
    fs::create_dir_all(&snapshots)?;
    let name = format!(
        "{}-{}",
        Utc::now().format("%Y%m%dT%H%M%S%.9fZ"),
        std::process::id()
    );
    let staging = snapshots.join(format!(".building-{name}"));
    fs::create_dir(&staging)?;
    let report = match dbrain_retrieval::rebuild_game_wiki(&pool, &staging).await {
        Ok(report) => report,
        Err(error) => {
            fs::remove_dir_all(&staging)?;
            return Err(error.into());
        }
    };
    let result = publish(
        &config.publication_root,
        &staging,
        &name,
        report,
        source_update,
    );
    if result.is_err() && staging.exists() {
        fs::remove_dir_all(&staging)?;
    }
    result
}

fn publish(
    base: &Path,
    staging: &Path,
    name: &str,
    report: Value,
    source_update: Value,
) -> Result<Value> {
    let digest = validate_snapshot(staging, &report)?;
    let destination = base.join("snapshots").join(name);
    let status = json!({
        "schema_version": 1, "state": "ready", "generation": digest,
        "rendered_at": report["generated_at"], "provenance": report["provenance"],
        "entries": report["entries_written"], "counts": report["counts"],
        "source_update_performed": !source_update.is_null(),
        "snapshot": destination,
    });
    atomic_json(&staging.join("status.json"), &status)?;
    fs::rename(staging, &destination)?;
    activate(base, &destination, &status)?;
    Ok(status)
}

fn activate(base: &Path, destination: &Path, status: &Value) -> Result<()> {
    let current = base.join("current");
    let old = match fs::symlink_metadata(&current) {
        Ok(meta) if meta.file_type().is_symlink() => Some(fs::read_link(&current)?),
        Ok(_) => bail!("Aktuelle Wikiwurzel ist kein verwalteter Symlink"),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
        Err(error) => return Err(error.into()),
    };
    replace_link(&current, destination)?;
    if let Err(error) = atomic_json(&base.join("last-attempt.json"), status) {
        match old {
            Some(target) => {
                replace_link(&current, &target).context("Wiki-Rollback fehlgeschlagen")?
            }
            None => fs::remove_file(&current)
                .context("Erster Wiki-Cutover konnte nicht zurückgenommen werden")?,
        }
        return Err(error);
    }
    Ok(())
}

fn replace_link(path: &Path, target: &Path) -> Result<()> {
    let temp = path.with_extension(format!("link-{}", std::process::id()));
    symlink(target, &temp).context("Wiki-Umschaltlink konnte nicht angelegt werden")?;
    if let Err(error) = fs::rename(&temp, path) {
        fs::remove_file(&temp)?;
        return Err(error.into());
    }
    Ok(())
}

fn atomic_json(path: &Path, value: &Value) -> Result<()> {
    let temp = path.with_extension(format!("tmp-{}", std::process::id()));
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&temp)?;
    let result = (|| -> Result<()> {
        file.write_all(&serde_json::to_vec_pretty(value)?)?;
        file.sync_all()?;
        fs::rename(&temp, path)?;
        Ok(())
    })();
    if result.is_err() {
        fs::remove_file(&temp)?;
    }
    result
}

fn validate_snapshot(root: &Path, report: &Value) -> Result<String> {
    if report["entries_written"].as_u64().unwrap_or_default() == 0
        || !report["provenance"].is_object()
    {
        bail!("Wiki-Rebuild enthält keine belegten Einträge");
    }
    for relative in [
        "index.md",
        "pages/deadlock-data/hero.md",
        "pages/deadlock-data/item-card.md",
        "pages/deadlock-data/ability-card.md",
    ] {
        let metadata = fs::symlink_metadata(root.join(relative))?;
        if !metadata.is_file() || metadata.len() == 0 {
            bail!("Wiki-Pflichtdatei fehlt: {relative}");
        }
    }
    let mut paths = Vec::new();
    collect_files(root, root, &mut paths)?;
    paths.sort();
    let mut hash = Sha256::new();
    for relative in paths {
        hash.update(relative.to_string_lossy().as_bytes());
        hash.update([0]);
        hash.update(Sha256::digest(fs::read(root.join(relative))?));
    }
    Ok(format!("{:x}", hash.finalize()))
}

fn collect_files(root: &Path, directory: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let kind = entry.file_type()?;
        if kind.is_symlink() {
            bail!("Symlink im neuen Wiki-Snapshot ist nicht erlaubt");
        }
        if kind.is_dir() {
            collect_files(root, &entry.path(), out)?;
        } else if kind.is_file() {
            out.push(entry.path().strip_prefix(root)?.to_path_buf());
        } else {
            bail!("Unzulässiger Dateityp im Wiki-Snapshot");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wiki_corpus_is_opt_in_and_existing_config_stays_valid() {
        let mut value = json!({
            "infisical_config":"/test/infisical.json",
            "source_repository":"/test/source",
            "raw_directory":"/test/raw",
            "publication_root":"/test/wiki"
        });
        let config: Config = serde_json::from_value(value.clone()).unwrap();
        assert!(config.wiki.is_none());
        value["wiki"] = json!({"enabled":true});
        let config: Config = serde_json::from_value(value.clone()).unwrap();
        let wiki = config.wiki.unwrap();
        assert!(wiki.enabled);
        assert_eq!(wiki.min_delay_seconds, 5.0);
        value["wiki"]["unknown_flag"] = json!(true);
        assert!(serde_json::from_value::<Config>(value).is_err());
    }

    #[test]
    fn failed_validation_preserves_current_and_honest_dates() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let base = temp.path();
        let old = base.join("old");
        fs::create_dir(&old)?;
        symlink(&old, base.join("current"))?;
        let staging = base.join("staging");
        fs::create_dir(&staging)?;
        assert!(publish(base, &staging, "new", json!({}), Value::Null).is_err());
        assert_eq!(fs::read_link(base.join("current"))?, old);
        Ok(())
    }

    #[test]
    fn status_failure_rolls_back_current() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let base = temp.path();
        let old = base.join("old");
        let new = base.join("new");
        fs::create_dir(&old)?;
        fs::create_dir(&new)?;
        symlink(&old, base.join("current"))?;
        fs::create_dir(base.join("last-attempt.json"))?;
        assert!(activate(base, &new, &json!({"state":"ready"})).is_err());
        assert_eq!(fs::read_link(base.join("current"))?, old);
        Ok(())
    }

    #[test]
    fn snapshot_rejects_missing_required_files_and_symlinks() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let root = temp.path();
        let report = json!({"entries_written":3,"provenance":{}});
        assert!(validate_snapshot(root, &report).is_err());
        fs::create_dir_all(root.join("pages/deadlock-data"))?;
        for relative in [
            "index.md",
            "pages/deadlock-data/hero.md",
            "pages/deadlock-data/item-card.md",
            "pages/deadlock-data/ability-card.md",
        ] {
            fs::write(root.join(relative), "# Verified fixture")?;
        }
        assert_eq!(validate_snapshot(root, &report)?.len(), 64);
        symlink("/etc/passwd", root.join("escape.md"))?;
        assert!(validate_snapshot(root, &report).is_err());
        Ok(())
    }

    #[test]
    fn successful_publication_keeps_source_and_render_dates_separate() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let base = temp.path();
        fs::create_dir(base.join("snapshots"))?;
        let staging = base.join("staging");
        fs::create_dir_all(staging.join("pages/deadlock-data"))?;
        for relative in [
            "index.md",
            "pages/deadlock-data/hero.md",
            "pages/deadlock-data/item-card.md",
            "pages/deadlock-data/ability-card.md",
        ] {
            fs::write(staging.join(relative), "# Source-backed fixture")?;
        }
        let provenance = json!({"deadlock_data":{"latest_fetched_at":"2026-09-16T20:18:12Z",
            "revisions":{"revision":"2026-09-12T09:54:48Z"}}});
        let status = publish(
            base,
            &staging,
            "new",
            json!({"entries_written":3,
            "generated_at":"2026-09-20T00:00:00Z", "provenance":provenance}),
            Value::Null,
        )?;
        assert_eq!(status["provenance"], provenance);
        assert_eq!(status["rendered_at"], "2026-09-20T00:00:00Z");
        assert_eq!(status["source_update_performed"], false);
        let active: Value =
            serde_json::from_slice(&fs::read(base.join("current/status.json"))?)?;
        assert_eq!(status, active);
        Ok(())
    }
}
