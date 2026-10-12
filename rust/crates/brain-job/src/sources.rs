use crate::InputObject;
use anyhow::{bail, Context};
use std::{path::Path, process::Command};

pub type Source = fn(&Path) -> anyhow::Result<InputObject>;
pub const SOURCES: &[(&str, Source)] = &[("deadlock-data", deadlock_data)];

fn git(repository: &Path, args: &[&str]) -> anyhow::Result<Vec<u8>> {
    let result = Command::new("git")
        .arg("-C")
        .arg(repository)
        .args(args)
        .output()
        .context("Lokaler Git-Aufruf fehlgeschlagen")?;
    if !result.status.success() {
        bail!("Lokaler Git-Aufruf fehlgeschlagen");
    }
    Ok(result.stdout)
}

fn deadlock_data(repository: &Path) -> anyhow::Result<InputObject> {
    let revision = String::from_utf8(git(
        repository,
        &["rev-parse", "--verify", "HEAD^{commit}"],
    )?)?;
    let revision = revision.trim().to_owned();
    if ![40, 64].contains(&revision.len()) || !revision.bytes().all(|b| b.is_ascii_hexdigit()) {
        bail!("Ungültige lokale Git-Revision");
    }
    let path = "data/json/hero-data.json";
    let bytes = git(repository, &["show", &format!("{revision}:{path}")])?;
    Ok(InputObject {
        revision,
        path: path.to_owned(),
        bytes,
    })
}
