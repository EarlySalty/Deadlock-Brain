use crate::fs_safe::{self, hash, regular};
use anyhow::{ensure, Context, Result};
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

pub const OPERATOR: u32 = 1000;
pub const ORIGIN: &str = "git@github.com:EarlySalty/Deadlock-Brain.git";
pub const BINS: [&str; 7] = [
    "deadlock-brain",
    "brain-serve",
    "brain-maintain",
    "brain-legacy-import",
    "deadlock-brain-yt",
    "brain-candidate-activate",
    "brain-patchnotes-ingest",
];
pub const BUILD_ARGS: [&str; 7] = [
    "build",
    "--locked",
    "--release",
    "--jobs",
    "4",
    "--workspace",
    "--bins",
];

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Source {
    pub sha: String,
    pub tree: String,
    pub fingerprint: String,
    pub path: PathBuf,
    pub worktree_dev: u64,
    pub worktree_ino: u64,
    pub git_pointer_sha256: String,
}

impl Source {
    pub fn same_revision(&self, other: &Self) -> bool {
        self.sha == other.sha && self.tree == other.tree && self.fingerprint == other.fingerprint
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Artifact {
    pub name: String,
    pub sha256: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub format: u32,
    pub source: Source,
    pub origin: String,
    pub cargo_version: String,
    pub rustc_version: String,
    pub build_args: Vec<String>,
    pub artifacts: Vec<Artifact>,
}

pub fn command(program: &str) -> Command {
    let mut cmd = Command::new(program);
    cmd.env_clear()
        .env("HOME", "/home/nathanael")
        .env("PATH", "/home/nathanael/.cargo/bin:/usr/bin:/bin")
        .env("LANG", "C.UTF-8")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("GIT_NO_REPLACE_OBJECTS", "1")
        .env("GIT_SSH_COMMAND", "/usr/bin/ssh -oBatchMode=yes");
    cmd
}

fn git(path: &Path, args: &[&str]) -> Result<Vec<u8>> {
    ensure!(fs_safe::uid() != 0, "Git darf nicht als root laufen");
    let output = command("/usr/bin/git")
        .args([
            "-c",
            "core.fsmonitor=false",
            "-c",
            "core.sshCommand=/usr/bin/ssh -oBatchMode=yes",
            "-c",
            "protocol.ext.allow=never",
            "-C",
        ])
        .arg(path)
        .args(args)
        .output()?;
    ensure!(
        output.status.success(),
        "Git-Prüfung fehlgeschlagen (Exit {:?})",
        output.status.code()
    );
    Ok(output.stdout)
}

fn text(path: &Path, args: &[&str]) -> Result<String> {
    Ok(String::from_utf8(git(path, args)?)?.trim().to_string())
}

pub fn remote_main() -> Result<String> {
    let value = text(Path::new("/"), &["ls-remote", ORIGIN, "refs/heads/main"])?;
    let fields: Vec<_> = value.split_whitespace().collect();
    ensure!(
        fields.len() == 2 && fields[1] == "refs/heads/main",
        "Remote-main nicht eindeutig"
    );
    fs_safe::sha(fields[0])?;
    Ok(fields[0].to_string())
}

pub fn inspect(path: &Path) -> Result<Source> {
    ensure!(
        fs_safe::uid() == OPERATOR,
        "Quellprüfung benötigt den festen Betreiber-UID 1000"
    );
    ensure!(
        path.starts_with("/home/nathanael/.worktrees")
            && path != Path::new("/home/nathanael/.worktrees"),
        "Quelle muss ein eigener Worktree sein"
    );
    fs_safe::checked_path(path, OPERATOR, true)?;
    use std::os::unix::fs::MetadataExt;
    let worktree_meta = std::fs::symlink_metadata(path)?;
    let git_pointer_sha256 = fs_safe::hash(&fs_safe::bytes(&path.join(".git"), OPERATOR, 8192)?);
    ensure!(
        text(path, &["rev-parse", "--show-toplevel"])? == path.to_string_lossy(),
        "Quellwurzel stimmt nicht"
    );
    ensure!(
        regular(&path.join(".git"), OPERATOR).is_ok(),
        "Kein separater Quellworktree"
    );
    ensure!(
        text(path, &["remote", "get-url", "origin"])? == ORIGIN,
        "Falsches Quellrepository"
    );
    ensure!(
        git(
            path,
            &[
                "status",
                "--porcelain=v1",
                "--untracked-files=all",
                "--ignored"
            ]
        )?
        .is_empty(),
        "Quellworktree ist nicht vollständig sauber (auch ignorierte Dateien prüfen)"
    );
    let sha = text(path, &["rev-parse", "HEAD"])?;
    fs_safe::sha(&sha)?;
    let tree_ref = format!("{sha}^{{tree}}");
    let tree = text(path, &["rev-parse", &tree_ref])?;
    let entries = git(path, &["ls-tree", "-r", "-z", &sha])?;
    let mut fingerprint = Vec::new();
    for entry in entries.split(|b| *b == 0).filter(|s| !s.is_empty()) {
        let tab = entry
            .iter()
            .position(|b| *b == b'\t')
            .context("Ungültiger Baum")?;
        let fields = std::str::from_utf8(&entry[..tab])?
            .split_whitespace()
            .collect::<Vec<_>>();
        ensure!(
            fields.len() == 3 && fields[1] == "blob" && ["100644", "100755"].contains(&fields[0]),
            "Nicht reguläre Quelldatei"
        );
        let relative = std::str::from_utf8(&entry[tab + 1..])?;
        let mut file = regular(&path.join(relative), OPERATOR)?;
        use std::os::unix::fs::PermissionsExt;
        let mode = file.metadata()?.permissions().mode();
        ensure!(
            (mode & 0o111 != 0) == (fields[0] == "100755"),
            "Quell-Dateimodus weicht von HEAD ab: {relative}"
        );
        let size = file.metadata()?.len();
        let mut digest = Sha1::new();
        digest.update(format!("blob {size}\0").as_bytes());
        let mut sha256 = sha2::Sha256::new();
        let mut buf = [0u8; 65536];
        let mut seen = 0;
        loop {
            let n = file.read(&mut buf)?;
            if n == 0 {
                break;
            }
            seen += n as u64;
            digest.update(&buf[..n]);
            sha256.update(&buf[..n]);
        }
        ensure!(
            size == seen && format!("{:x}", digest.finalize()) == fields[2],
            "Quellbytes weichen von HEAD ab: {relative}"
        );
        fingerprint.extend_from_slice(entry);
        fingerprint.push(0);
        fingerprint.extend_from_slice(&sha256.finalize());
    }
    ensure!(
        text(path, &["rev-parse", "HEAD"])? == sha,
        "HEAD während Quellprüfung verändert"
    );
    let after_meta = std::fs::symlink_metadata(path)?;
    ensure!(
        after_meta.dev() == worktree_meta.dev()
            && after_meta.ino() == worktree_meta.ino()
            && fs_safe::hash(&fs_safe::bytes(&path.join(".git"), OPERATOR, 8192)?)
                == git_pointer_sha256,
        "Quellworktree während Prüfung ausgetauscht"
    );
    Ok(Source {
        sha,
        tree,
        fingerprint: hash(&fingerprint),
        path: path.to_path_buf(),
        worktree_dev: worktree_meta.dev(),
        worktree_ino: worktree_meta.ino(),
        git_pointer_sha256,
    })
}

pub fn inventory(path: &Path) -> Result<Vec<String>> {
    ensure!(
        fs_safe::uid() == OPERATOR,
        "Cargo-Metadaten nur unprivilegiert lesen"
    );
    let output = command("/home/nathanael/.cargo/bin/cargo")
        .current_dir("/")
        .args([
            "metadata",
            "--format-version",
            "1",
            "--no-deps",
            "--manifest-path",
        ])
        .arg(path.join("rust/Cargo.toml"))
        .output()?;
    ensure!(
        output.status.success(),
        "Binary-Inventar fehlt (Exit {:?})",
        output.status.code()
    );
    let data: serde_json::Value = serde_json::from_slice(&output.stdout)?;
    let members = data["workspace_members"]
        .as_array()
        .context("Workspace fehlt")?;
    let mut names = Vec::new();
    for package in data["packages"].as_array().context("Pakete fehlen")? {
        if !members.contains(&package["id"]) {
            continue;
        }
        for target in package["targets"].as_array().context("Targets fehlen")? {
            if target["kind"]
                .as_array()
                .is_some_and(|kinds| kinds.iter().any(|kind| kind == "bin"))
            {
                names.push(
                    target["name"]
                        .as_str()
                        .context("Binaryname fehlt")?
                        .to_string(),
                );
            }
        }
    }
    names.sort();
    ensure!(
        names.windows(2).all(|pair| pair[0] != pair[1]),
        "Mehrdeutige Binarynamen im Workspace"
    );
    ensure!(
        BINS.iter()
            .all(|name| names.iter().any(|found| found == name)),
        "Gemeinsamer Integrationsstand enthält nicht alle Pflichtbinaries"
    );
    Ok(names)
}

pub fn validate_manifest(manifest: &Manifest) -> Result<()> {
    fs_safe::sha(&manifest.source.sha)?;
    fs_safe::sha(&manifest.source.tree)?;
    ensure!(
        [2, 3].contains(&manifest.format)
            && manifest.origin == ORIGIN
            && !manifest.cargo_version.is_empty()
            && !manifest.rustc_version.is_empty(),
        "Herkunft fehlt"
    );
    ensure!(
        manifest
            .build_args
            .iter()
            .map(String::as_str)
            .eq(BUILD_ARGS),
        "Unbekannter Buildaufruf"
    );
    ensure!(
        manifest.artifacts.len() >= BINS.len()
            && manifest.artifacts.len() <= 64
            && BINS.iter().all(|name| manifest
                .artifacts
                .iter()
                .any(|artifact| artifact.name == *name)),
        "Unvollständiges gemeinsames Release"
    );
    ensure!(
        manifest.source.fingerprint.len() == 64,
        "Quellfingerprint fehlt"
    );
    let mut names = std::collections::BTreeSet::new();
    for artifact in &manifest.artifacts {
        ensure!(
            names.insert(&artifact.name)
                && !artifact.name.is_empty()
                && artifact.name.len() <= 128
                && artifact
                    .name
                    .bytes()
                    .all(|x| x.is_ascii_alphanumeric() || x == b'-' || x == b'_'),
            "Ungültiger oder doppelter Binaryname"
        );
        ensure!(
            artifact.sha256.len() == 64
                && artifact
                    .sha256
                    .bytes()
                    .all(|x| x.is_ascii_hexdigit() && !x.is_ascii_uppercase()),
            "Artefaktliste ungültig"
        );
    }
    Ok(())
}

pub fn compare_artifacts(provided: &[Artifact], compiled: &[Artifact]) -> Result<()> {
    ensure!(
        provided == compiled,
        "Bundlebytes weichen vom bestätigten Sourcebuild ab"
    );
    Ok(())
}

pub fn verify(source: &Path, bundle: &Path) -> Result<Manifest> {
    fs_safe::checked_path(bundle, OPERATOR, true)?;
    let candidate: Manifest = serde_json::from_slice(&fs_safe::bytes(
        &bundle.join("manifest.json"),
        OPERATOR,
        65536,
    )?)?;
    validate_manifest(&candidate)?;
    verify_unchanged(source, bundle, &candidate)?;
    ensure!(
        candidate.format == 3,
        "Altes Bundle ohne bestätigten Einmal-Build: build erneut ausführen"
    );
    let (compiled, _) =
        crate::install::build_proof(Path::new(crate::install::ROOT), &candidate.source.sha, 0)?
            .context("Root-eigener Buildbeleg fehlt: build ausführen")?;
    compare_artifacts(&candidate.artifacts, &compiled.artifacts)?;
    ensure!(
        candidate == compiled,
        "Buildmanifest weicht vom root-eigenen Sourcebuild ab"
    );
    verify_unchanged(source, bundle, &compiled)?;
    Ok(compiled)
}

pub fn verify_unchanged(source: &Path, bundle: &Path, manifest: &Manifest) -> Result<()> {
    fs_safe::checked_path(bundle, OPERATOR, true)?;
    let actual: Manifest = serde_json::from_slice(&fs_safe::bytes(
        &bundle.join("manifest.json"),
        OPERATOR,
        65536,
    )?)?;
    ensure!(
        actual == *manifest,
        "Bundlemanifest während Prüfung geändert"
    );
    validate_manifest(manifest)?;
    let source_state = inspect(source)?;
    ensure!(
        source_state.same_revision(&manifest.source),
        "Quelle gehört nicht zum bestätigten Quellbaum"
    );
    ensure!(
        inventory(source)?
            == manifest
                .artifacts
                .iter()
                .map(|artifact| artifact.name.clone())
                .collect::<Vec<_>>(),
        "Binary-Inventar stimmt nicht mit dem integrierten Workspace überein"
    );
    ensure!(
        inspect(source)? == source_state,
        "Quelle während Inventarprüfung verändert"
    );
    for artifact in &manifest.artifacts {
        ensure!(
            fs_safe::file_hash(&bundle.join(&artifact.name), OPERATOR)? == artifact.sha256,
            "Artefaktdrift: {}",
            artifact.name
        );
    }
    ensure!(
        remote_main()? == manifest.source.sha,
        "Remote-main hat sich geändert"
    );
    ensure!(
        inspect(source)? == source_state,
        "Quelle während Artefaktprüfung verändert"
    );
    ensure!(
        remote_main()? == manifest.source.sha,
        "Remote-main hat sich geändert"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reuse_requires_the_complete_revision_not_the_old_worktree_path() {
        let source = Source {
            sha: "a".repeat(40),
            tree: "b".repeat(40),
            fingerprint: "c".repeat(64),
            path: "/home/nathanael/.worktrees/first".into(),
            worktree_dev: 1,
            worktree_ino: 2,
            git_pointer_sha256: "d".repeat(64),
        };
        let mut other = source.clone();
        other.path = "/home/nathanael/.worktrees/second".into();
        other.worktree_ino = 3;
        other.git_pointer_sha256 = "e".repeat(64);
        assert_ne!(source, other);
        assert!(source.same_revision(&other));
        for field in ["sha", "tree", "fingerprint"] {
            let mut changed = other.clone();
            match field {
                "sha" => changed.sha = "f".repeat(40),
                "tree" => changed.tree = "f".repeat(40),
                _ => changed.fingerprint = "f".repeat(64),
            }
            assert!(!source.same_revision(&changed));
        }
    }
}
