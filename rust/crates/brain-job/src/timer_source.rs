use crate::{InputObject, InputObjects};
use anyhow::Context;
use std::{fs::OpenOptions, path::Path, process::Command};

const URL: &str = "https://github.com/SteamTracking/GameTracking-Deadlock.git";
const PATHS: &[&str] = &[
    "game/citadel/pak01_dir/scripts/misc.vdata",
    "game/citadel/pak01_dir/scripts/generic_data.vdata",
];

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
        .context("Timerquelle konnte nicht gelesen werden")?;
    anyhow::ensure!(
        output.status.success(),
        "Timerquelle konnte nicht gelesen werden"
    );
    Ok(output.stdout)
}

fn revision(repository: &Path, fetch: bool) -> anyhow::Result<String> {
    let reference = if fetch {
        let parent = repository.parent().context("Quellen-Cache fehlt")?;
        std::fs::create_dir_all(parent)?;
        let lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(parent.join(".gametracking.lock"))?;
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
                    "--bare",
                    "--depth=1",
                    "--filter=blob:none",
                    "--no-tags",
                    URL,
                    destination,
                ],
            )?;
        }
        let origin = String::from_utf8(git(repository, &["remote", "get-url", "origin"])?)?;
        anyhow::ensure!(origin.trim() == URL, "Unzulässiger Quellenursprung");
        git(
            repository,
            &[
                "fetch",
                "--filter=blob:none",
                "--no-tags",
                "--depth=1",
                URL,
                "HEAD",
            ],
        )?;
        let revision = String::from_utf8(git(
            repository,
            &["rev-parse", "--verify", "FETCH_HEAD^{commit}"],
        )?)?;
        return checked_revision(&revision);
    } else {
        "HEAD^{commit}"
    };
    let revision = String::from_utf8(git(repository, &["rev-parse", "--verify", reference])?)?;
    checked_revision(&revision)
}

fn checked_revision(value: &str) -> anyhow::Result<String> {
    let value = value.trim();
    anyhow::ensure!(
        [40, 64].contains(&value.len()) && value.bytes().all(|b| b.is_ascii_hexdigit()),
        "Ungültige Git-Revision"
    );
    Ok(value.to_owned())
}

pub fn load(repository: &Path, fetch: bool) -> InputObjects {
    let repository = repository.to_path_buf();
    let mut revision = None;
    let mut index = 0;
    Box::new(std::iter::from_fn(move || {
        let path = *PATHS.get(index)?;
        index += 1;
        if revision.is_none() {
            match self::revision(&repository, fetch) {
                Ok(value) => revision = Some(value),
                Err(error) => {
                    index = PATHS.len();
                    return Some(Err(error));
                }
            }
        }
        let revision = revision.as_deref()?;
        Some(
            git(&repository, &["show", &format!("{revision}:{path}")]).map(|bytes| InputObject {
                revision: revision.to_owned(),
                path: path.to_owned(),
                bytes,
            }),
        )
    }))
}
