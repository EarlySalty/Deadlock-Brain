use crate::InputObject;
use anyhow::{ensure, Context};
use std::{
    fs::{File, OpenOptions},
    path::Path,
    process::Command,
};

pub const REPOSITORY_URL: &str = "https://github.com/EarlySalty/Deadlock-Docs.git";
const SSH_REPOSITORY_URL: &str = "git@github.com:EarlySalty/Deadlock-Docs.git";

fn git(repository: &Path, args: &[&str]) -> anyhow::Result<Vec<u8>> {
    let output = Command::new("git")
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
        .context("Git-Aufruf für Dokumente fehlgeschlagen")?;
    ensure!(
        output.status.success(),
        "Git-Aufruf für Dokumente fehlgeschlagen"
    );
    Ok(output.stdout)
}

fn snapshot(repository: &Path, reference: &str) -> anyhow::Result<(String, Vec<String>)> {
    let revision = String::from_utf8(git(repository, &["rev-parse", "--verify", reference])?)?;
    let revision = revision.trim().to_owned();
    ensure!(
        revision.len() == 40
            && revision
                .bytes()
                .all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')),
        "Ungültige Dokumentenrevision"
    );
    let listing = git(
        repository,
        &[
            "ls-tree",
            "-r",
            "-z",
            "--name-only",
            &revision,
            "--",
            "public",
            "internal",
        ],
    )?;
    let mut paths = Vec::new();
    for entry in listing.split(|b| *b == 0).filter(|entry| !entry.is_empty()) {
        let path = std::str::from_utf8(entry).context("Ungültiger Dokumentenpfad")?;
        if path.ends_with(".html") && (path.starts_with("public/") || path.starts_with("internal/"))
        {
            paths.push(path.to_owned());
        }
    }
    Ok((revision, paths))
}

fn refresh(repository: &Path) -> anyhow::Result<(File, String, Vec<String>)> {
    let parent = repository.parent().context("Dokumenten-Cache fehlt")?;
    std::fs::create_dir_all(parent)?;
    let lock = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(parent.join(".deadlock-docs.lock"))?;
    lock.lock()
        .context("Dokumenten-Cache konnte nicht gesperrt werden")?;
    if !repository.exists() {
        let destination = repository
            .file_name()
            .and_then(|name| name.to_str())
            .context("Ungültiger Dokumenten-Cache")?;
        git(
            parent,
            &[
                "clone",
                "--no-checkout",
                "--depth=1",
                "--no-tags",
                REPOSITORY_URL,
                destination,
            ],
        )?;
    }
    let origin = String::from_utf8(git(repository, &["remote", "get-url", "origin"])?)?;
    ensure!(
        [REPOSITORY_URL, SSH_REPOSITORY_URL].contains(&origin.trim()),
        "Unzulässiger Dokumentenursprung"
    );
    git(
        repository,
        &["fetch", "--no-tags", "--depth=1", REPOSITORY_URL, "HEAD"],
    )?;
    let (revision, paths) = snapshot(repository, "FETCH_HEAD^{commit}")?;
    Ok((lock, revision, paths))
}

pub fn load(
    repository: &Path,
    fetch: bool,
) -> Box<dyn Iterator<Item = anyhow::Result<InputObject>> + Send> {
    let repository = repository.to_path_buf();
    let mut initialized = false;
    let mut revision = String::new();
    let mut paths = Vec::new().into_iter();
    let mut lock = None;
    Box::new(std::iter::from_fn(move || {
        if !initialized {
            initialized = true;
            let result = if fetch {
                refresh(&repository).map(|(guard, value, entries)| {
                    lock = Some(guard);
                    (value, entries)
                })
            } else {
                snapshot(&repository, "HEAD^{commit}")
            };
            match result {
                Ok((value, entries)) => {
                    revision = value;
                    paths = entries.into_iter();
                }
                Err(error) => return Some(Err(error)),
            }
        }
        let Some(path) = paths.next() else {
            drop(lock.take());
            return None;
        };
        Some(
            git(&repository, &["show", &format!("{revision}:{path}")]).map(|bytes| InputObject {
                revision: revision.clone(),
                path,
                bytes,
            }),
        )
    }))
}
