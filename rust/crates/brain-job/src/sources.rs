use crate::InputObject;
use anyhow::{bail, Context};
use std::{fs::OpenOptions, path::Path, process::Command};

pub type InputObjects = Box<dyn Iterator<Item = anyhow::Result<InputObject>> + Send>;
pub type Source = fn(&Path, bool) -> InputObjects;
pub const SOURCES: &[(&str, Source)] = &[("deadlock-data", deadlock_data)];
pub const DEADLOCK_DATA_URL: &str = "https://github.com/deadlock-wiki/deadlock-data.git";

fn git(repository: &Path, args: &[&str]) -> anyhow::Result<Vec<u8>> {
    let result = Command::new("git")
        .args([
            "-c",
            "credential.helper=",
            "-c",
            "core.hooksPath=/dev/null",
            "-c",
            "protocol.file.allow=never",
            "-c",
            "protocol.ext.allow=never",
        ])
        .arg("-C")
        .arg(repository)
        .args(args)
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .context("Git-Aufruf fehlgeschlagen")?;
    if !result.status.success() {
        bail!("Git-Aufruf fehlgeschlagen");
    }
    Ok(result.stdout)
}

fn read_revision(repository: &Path, reference: &str) -> anyhow::Result<InputObject> {
    let revision = String::from_utf8(git(repository, &["rev-parse", "--verify", reference])?)?;
    let revision = revision.trim().to_owned();
    if ![40, 64].contains(&revision.len()) || !revision.bytes().all(|b| b.is_ascii_hexdigit()) {
        bail!("Ungültige Git-Revision");
    }
    let path = "data/json/hero-data.json";
    let bytes = git(repository, &["show", &format!("{revision}:{path}")])?;
    Ok(InputObject {
        revision,
        path: path.to_owned(),
        bytes,
    })
}

fn refresh(repository: &Path) -> anyhow::Result<InputObject> {
    let parent = repository.parent().context("Quellen-Cache fehlt")?;
    std::fs::create_dir_all(parent)?;
    let lock = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(parent.join(".deadlock-data.lock"))?;
    lock.lock()
        .context("Quellen-Cache konnte nicht gesperrt werden")?;
    if !repository.exists() {
        let destination = repository
            .file_name()
            .and_then(|s| s.to_str())
            .context("Ungültiger Quellen-Cache")?;
        git(
            parent,
            &[
                "clone",
                "--no-checkout",
                "--depth=1",
                "--no-tags",
                DEADLOCK_DATA_URL,
                destination,
            ],
        )?;
    }
    let origin = String::from_utf8(git(repository, &["remote", "get-url", "origin"])?)?;
    anyhow::ensure!(
        origin.trim() == DEADLOCK_DATA_URL,
        "Unzulässiger Quellenursprung"
    );
    git(
        repository,
        &["fetch", "--no-tags", "--depth=1", DEADLOCK_DATA_URL, "HEAD"],
    )?;
    read_revision(repository, "FETCH_HEAD^{commit}")
}

fn deadlock_data(repository: &Path, fetch: bool) -> InputObjects {
    let repository = repository.to_path_buf();
    Box::new(std::iter::once_with(move || {
        if fetch {
            refresh(&repository)
        } else {
            read_revision(&repository, "HEAD^{commit}")
        }
    }))
}

pub fn fetch(source: &str, cache: &Path) -> InputObjects {
    match SOURCES.iter().find(|(name, _)| *name == source) {
        Some((_, loader)) => loader(&cache.join(source), true),
        None => Box::new(std::iter::once_with(|| {
            Err(anyhow::anyhow!("Unbekannte Quelle"))
        })),
    }
}
