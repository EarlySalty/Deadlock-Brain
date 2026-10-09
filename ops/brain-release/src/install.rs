use crate::fs_safe::{self, Lock};
use crate::source::{validate_manifest, Manifest, OPERATOR};
use anyhow::{bail, ensure, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::fs::{symlink, MetadataExt, OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};

pub const ROOT: &str = "/opt/deadlock-brain";

pub struct Installer {
    root: PathBuf,
    owner: u32,
    _lock: Lock,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Pending {
    sha: String,
    current: Option<PathBuf>,
    maintenance: Option<PathBuf>,
}

fn exists(path: &Path) -> Result<bool> {
    match fs::symlink_metadata(path) {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error.into()),
    }
}

fn layout(path: &Path, binary_dir: &str, sha: &str, owner: u32) -> Result<Manifest> {
    fs_safe::checked_path(path, owner, true)?;
    let bytes = fs_safe::bytes(&path.join("manifest.json"), owner, 65536)?;
    let manifest: Manifest = serde_json::from_slice(&bytes)?;
    validate_manifest(&manifest)?;
    ensure!(manifest.source.sha == sha, "Release-SHA stimmt nicht");
    for artifact in &manifest.artifacts {
        ensure!(
            fs_safe::file_hash(&path.join(binary_dir).join(&artifact.name), owner)?
                == artifact.sha256,
            "Installiertes Binary verändert"
        );
    }
    Ok(manifest)
}

pub fn staged(root: &Path, sha: &str, owner: u32) -> Result<Manifest> {
    fs_safe::sha(sha)?;
    let a = root.join("releases").join(sha);
    let b = root.join("maintenance-releases").join(sha);
    let left = layout(&a, "bin", sha, owner)?;
    let right = layout(&b, "", sha, owner)?;
    ensure!(
        left == right
            && fs_safe::bytes(&a.join("manifest.json"), owner, 65536)?
                == fs_safe::bytes(&b.join("manifest.json"), owner, 65536)?,
        "Release-Manifeste unterscheiden sich"
    );
    Ok(left)
}

pub fn build_proof(root: &Path, sha: &str, owner: u32) -> Result<Option<(Manifest, PathBuf)>> {
    fs_safe::sha(sha)?;
    let mut proof: Option<(Manifest, PathBuf)> = None;
    for (parent, binary_dir) in [("releases", "bin"), ("maintenance-releases", "")] {
        let path = root.join(parent).join(sha);
        if !exists(&path)? {
            continue;
        }
        let manifest = layout(&path, binary_dir, sha, owner)?;
        if let Some((previous, _)) = &proof {
            ensure!(*previous == manifest, "Buildbelege unterscheiden sich");
        } else {
            proof = Some((manifest, path.join(binary_dir)));
        }
    }
    Ok(proof)
}

impl Installer {
    pub fn open(root: &Path, owner: u32) -> Result<Self> {
        fs_safe::checked_path(root, owner, true)?;
        fs_safe::checked_path(&root.join("releases"), owner, true)?;
        fs_safe::checked_path(&root.join("maintenance-releases"), owner, true)?;
        let lock = Lock::acquire(&root.join(".deploy.lock"), owner)?;
        Ok(Self {
            root: root.to_path_buf(),
            owner,
            _lock: lock,
        })
    }

    fn available(&self) -> Result<()> {
        ensure!(
            !exists(&self.root.join(".deploy-pending.json"))?,
            "Unvollständiger Zeigerwechsel: zuerst recover ausführen"
        );
        Ok(())
    }

    fn link_target(&self, name: &str, parent: &str) -> Result<Option<PathBuf>> {
        let path = self.root.join(name);
        let meta = match fs::symlink_metadata(&path) {
            Ok(meta) => meta,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(e.into()),
        };
        ensure!(
            meta.file_type().is_symlink() && meta.uid() == self.owner,
            "Ungültiger Releasezeiger"
        );
        let target = fs::read_link(path)?;
        self.check_target(&target, parent)?;
        Ok(Some(target))
    }

    fn check_target(&self, target: &Path, parent: &str) -> Result<()> {
        ensure!(
            target.parent() == Some(self.root.join(parent).as_path()),
            "Releasezeiger außerhalb des Layouts"
        );
        fs_safe::sha(
            target
                .file_name()
                .and_then(|s| s.to_str())
                .context("Release-SHA fehlt")?,
        )?;
        fs_safe::checked_path(target, self.owner, true)?;
        Ok(())
    }

    pub fn stage(&self, bundle: &Path, manifest: &Manifest) -> Result<()> {
        self.stage_checked(bundle, manifest, |_| Ok(()))
    }

    fn stage_checked(
        &self,
        bundle: &Path,
        manifest: &Manifest,
        mut checkpoint: impl FnMut(&str) -> Result<()>,
    ) -> Result<()> {
        self.available()?;
        validate_manifest(manifest)?;
        let sha = &manifest.source.sha;
        let cli = self.root.join("releases").join(sha);
        let maintenance = self.root.join("maintenance-releases").join(sha);
        let cli_exists = exists(&cli)?;
        let maintenance_exists = exists(&maintenance)?;
        for (path, binary_dir, present) in [
            (&cli, "bin", cli_exists),
            (&maintenance, "", maintenance_exists),
        ] {
            if present {
                ensure!(
                    self.layout(path, binary_dir, sha)? == *manifest,
                    "Vorhandenes Release weicht ab"
                );
            }
        }
        if cli_exists && maintenance_exists {
            return Ok(());
        }
        let stage = tempfile::Builder::new()
            .prefix(".stage-")
            .tempdir_in(&self.root)?;
        let a = stage.path().join("release");
        let b = stage.path().join("maintenance");
        fs_safe::new_dir(&a)?;
        fs_safe::new_dir(&a.join("bin"))?;
        fs_safe::new_dir(&b)?;
        for artifact in &manifest.artifacts {
            let mut input = fs_safe::regular(&bundle.join(&artifact.name), OPERATOR)?;
            ensure!(
                input.metadata()?.permissions().mode() & 0o111 != 0,
                "Nicht ausführbares Artefakt"
            );
            let mut header = [0u8; 4];
            input.read_exact(&mut header)?;
            ensure!(header == *b"\x7fELF", "Kein ELF-Binary");
            use std::io::{Seek, SeekFrom};
            input.seek(SeekFrom::Start(0))?;
            let output_path = a.join("bin").join(&artifact.name);
            let mut output = OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o500)
                .custom_flags(libc::O_NOFOLLOW)
                .open(&output_path)?;
            std::io::copy(&mut input, &mut output)?;
            output.flush()?;
            output.sync_all()?;
            ensure!(
                fs_safe::file_hash(&output_path, self.owner)? == artifact.sha256,
                "Artefakt während Installation geändert"
            );
            fs::set_permissions(&output_path, fs::Permissions::from_mode(0o555))?;
            output.sync_all()?;
            {
                let target = b.join(&artifact.name);
                let mut from = fs_safe::regular(&output_path, self.owner)?;
                let mut to = OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .mode(0o500)
                    .custom_flags(libc::O_NOFOLLOW)
                    .open(&target)?;
                std::io::copy(&mut from, &mut to)?;
                to.sync_all()?;
                fs::set_permissions(&target, fs::Permissions::from_mode(0o555))?;
                to.sync_all()?;
            }
        }
        let data = serde_json::to_vec_pretty(manifest)?;
        fs_safe::new_file(&a.join("manifest.json"), &data, 0o444)?;
        fs_safe::new_file(&b.join("manifest.json"), &data, 0o444)?;
        for dir in [&a.join("bin"), &a, &b] {
            fs::set_permissions(dir, fs::Permissions::from_mode(0o755))?;
            fs_safe::sync_dir(dir)?;
        }
        fs_safe::sync_dir(stage.path())?;
        ensure!(
            self.layout(&a, "bin", sha)? == *manifest && self.layout(&b, "", sha)? == *manifest,
            "Stagingnachweis weicht ab"
        );
        checkpoint("before-layouts")?;
        if !cli_exists {
            ensure!(!exists(&cli)?, "Release während Staging fremd angelegt");
            fs::rename(&a, &cli)?;
            fs_safe::sync_dir(&self.root.join("releases"))?;
        }
        checkpoint("after-cli-layout")?;
        if !maintenance_exists {
            ensure!(
                !exists(&maintenance)?,
                "Maintenance-Release während Staging fremd angelegt"
            );
            fs::rename(&b, &maintenance)?;
            fs_safe::sync_dir(&self.root.join("maintenance-releases"))?;
        }
        checkpoint("after-maintenance-layout")?;
        ensure!(
            self.installed(sha)? == *manifest,
            "Installationsnachweis weicht ab"
        );
        Ok(())
    }

    fn layout(&self, path: &Path, binary_dir: &str, sha: &str) -> Result<Manifest> {
        layout(path, binary_dir, sha, self.owner)
    }

    pub fn installed(&self, sha: &str) -> Result<Manifest> {
        staged(&self.root, sha, self.owner)
    }

    fn atomic_link(&self, name: &str, target: Option<&Path>) -> Result<()> {
        let path = self.root.join(name);
        if let Some(target) = target {
            let temporary = self.root.join(format!(".next-{name}"));
            symlink(target, &temporary)?;
            fs::rename(temporary, &path)?;
        } else if fs::symlink_metadata(&path).is_ok() {
            ensure!(
                fs::symlink_metadata(&path)?.file_type().is_symlink(),
                "Releasezeiger ausgetauscht"
            );
            fs::remove_file(path)?;
        }
        fs_safe::sync_dir(&self.root)
    }

    fn clear_temporary_journal(&self) -> Result<()> {
        let path = self.root.join(".deploy-pending.json.tmp");
        if exists(&path)? {
            let file = fs_safe::regular(&path, self.owner)?;
            ensure!(
                file.metadata()?.mode() & 0o077 == 0,
                "Unsicheres Zwischenjournal"
            );
            fs::remove_file(path)?;
            fs_safe::sync_dir(&self.root)?;
        }
        Ok(())
    }

    fn publish_pending(
        &self,
        pending: &Pending,
        mut checkpoint: impl FnMut(&str) -> Result<()>,
    ) -> Result<()> {
        self.available()?;
        self.clear_temporary_journal()?;
        let temporary = self.root.join(".deploy-pending.json.tmp");
        fs_safe::new_file(&temporary, &serde_json::to_vec(pending)?, 0o600)?;
        checkpoint("after-journal-write")?;
        fs::rename(&temporary, self.root.join(".deploy-pending.json"))?;
        fs_safe::sync_dir(&self.root)?;
        checkpoint("after-journal-publish")?;
        Ok(())
    }

    pub fn activate(&self, sha: &str) -> Result<()> {
        self.activate_checked(sha, || Ok(()))
    }

    pub fn activate_checked(
        &self,
        sha: &str,
        preflight: impl FnOnce() -> Result<()>,
    ) -> Result<()> {
        self.available()?;
        self.installed(sha)?;
        let pending = Pending {
            sha: sha.to_string(),
            current: self.link_target("current", "releases")?,
            maintenance: self.link_target("maintenance-current", "maintenance-releases")?,
        };
        self.publish_pending(&pending, |_| Ok(()))?;
        let result = (|| {
            preflight()?;
            self.atomic_link("current", Some(&self.root.join("releases").join(sha)))?;
            self.atomic_link(
                "maintenance-current",
                Some(&self.root.join("maintenance-releases").join(sha)),
            )?;
            ensure!(
                self.link_target("current", "releases")?
                    == Some(self.root.join("releases").join(sha))
                    && self.link_target("maintenance-current", "maintenance-releases")?
                        == Some(self.root.join("maintenance-releases").join(sha)),
                "Zeigerprüfung fehlgeschlagen"
            );
            self.installed(sha)?;
            Ok::<_, anyhow::Error>(())
        })();
        if let Err(error) = result {
            match self.restore(&pending) {
                Ok(()) => bail!("Zeigerwechsel fehlgeschlagen; alter Stand wiederhergestellt: {error}"),
                Err(rollback) => bail!("STOPP: Zeigerwechsel und Rückweg fehlgeschlagen; Journal erhalten: {error}; {rollback}"),
            }
        }
        self.clear_pending()?;
        Ok(())
    }

    fn clear_pending(&self) -> Result<()> {
        fs_safe::regular(&self.root.join(".deploy-pending.json"), self.owner)?;
        fs::remove_file(self.root.join(".deploy-pending.json"))?;
        fs_safe::sync_dir(&self.root)?;
        self.clear_temporary_journal()
    }

    fn restore(&self, pending: &Pending) -> Result<()> {
        fs_safe::sha(&pending.sha)?;
        for (target, parent) in [
            (&pending.current, "releases"),
            (&pending.maintenance, "maintenance-releases"),
        ] {
            if let Some(target) = target {
                self.check_target(target, parent)?;
            }
        }
        for (name, parent, previous) in [
            ("current", "releases", &pending.current),
            (
                "maintenance-current",
                "maintenance-releases",
                &pending.maintenance,
            ),
        ] {
            let temporary = self.root.join(format!(".next-{name}"));
            match fs::symlink_metadata(&temporary) {
                Ok(meta) => {
                    ensure!(
                        meta.file_type().is_symlink() && meta.uid() == self.owner,
                        "Fremdes Zwischenartefakt verhindert Rückweg"
                    );
                    let target = fs::read_link(&temporary)?;
                    self.check_target(&target, parent)?;
                    ensure!(
                        Some(target.clone()) == *previous
                            || target == self.root.join(parent).join(&pending.sha),
                        "Fremdes Symlinkziel verhindert Rückweg"
                    );
                    fs::remove_file(temporary)?;
                }
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => (),
                Err(error) => return Err(error.into()),
            }
            self.atomic_link(name, previous.as_deref())?;
        }
        ensure!(
            self.link_target("current", "releases")? == pending.current
                && self.link_target("maintenance-current", "maintenance-releases")?
                    == pending.maintenance,
            "Rückweg nicht vollständig"
        );
        self.clear_pending()
    }

    pub fn recover(&self) -> Result<()> {
        if !exists(&self.root.join(".deploy-pending.json"))? {
            return self.clear_temporary_journal();
        }
        let pending: Pending = serde_json::from_slice(&fs_safe::bytes(
            &self.root.join(".deploy-pending.json"),
            self.owner,
            65536,
        )?)?;
        for (name, parent, previous) in [
            ("current", "releases", &pending.current),
            (
                "maintenance-current",
                "maintenance-releases",
                &pending.maintenance,
            ),
        ] {
            let current = self.link_target(name, parent)?;
            ensure!(
                current == *previous || current == Some(self.root.join(parent).join(&pending.sha)),
                "Zeiger seit Abbruch fremd verändert"
            );
        }
        self.restore(&pending)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::{Artifact, Source, BINS, BUILD_ARGS, ORIGIN};
    use tempfile::{tempdir_in, TempDir};

    fn fixture() -> (TempDir, PathBuf, PathBuf, Manifest) {
        let dir = tempdir_in("/home/nathanael/.cache").unwrap();
        fs::set_permissions(dir.path(), fs::Permissions::from_mode(0o700)).unwrap();
        let root = dir.path().join("root");
        let bundle = dir.path().join("bundle");
        fs_safe::new_dir(&root).unwrap();
        fs_safe::new_dir(&root.join("releases")).unwrap();
        fs_safe::new_dir(&root.join("maintenance-releases")).unwrap();
        fs_safe::new_dir(&bundle).unwrap();
        let mut artifacts = Vec::new();
        for name in BINS {
            let data = [b"\x7fELF".as_slice(), name.as_bytes()].concat();
            fs_safe::new_file(&bundle.join(name), &data, 0o700).unwrap();
            artifacts.push(Artifact {
                name: name.into(),
                sha256: fs_safe::hash(&data),
            });
        }
        let manifest = Manifest {
            format: 2,
            source: Source {
                sha: "a".repeat(40),
                tree: "b".repeat(40),
                fingerprint: "c".repeat(64),
                path: "/not-executed".into(),
                worktree_dev: 1,
                worktree_ino: 1,
                git_pointer_sha256: "e".repeat(64),
            },
            origin: ORIGIN.into(),
            cargo_version: "test".into(),
            rustc_version: "test".into(),
            build_args: BUILD_ARGS.iter().map(|x| x.to_string()).collect(),
            artifacts,
        };
        (dir, root, bundle, manifest)
    }

    #[test]
    fn installs_both_layouts_and_preserves_releases() {
        let (_dir, root, bundle, manifest) = fixture();
        let install = Installer::open(&root, fs_safe::uid()).unwrap();
        install.stage(&bundle, &manifest).unwrap();
        install.activate(&manifest.source.sha).unwrap();
        let mut next = manifest.clone();
        next.source.sha = "d".repeat(40);
        install.stage(&bundle, &next).unwrap();
        install.activate(&next.source.sha).unwrap();
        install.activate(&manifest.source.sha).unwrap();
        assert_eq!(install.installed(&next.source.sha).unwrap(), next);
        assert_eq!(
            fs::read_link(root.join("maintenance-current")).unwrap(),
            root.join("maintenance-releases").join(&manifest.source.sha)
        );
    }

    #[test]
    fn one_published_layout_preserves_build_proof_without_recompilation() {
        let (_dir, root, bundle, mut manifest) = fixture();
        manifest.format = 3;
        let installer = Installer::open(&root, fs_safe::uid()).unwrap();
        assert!(build_proof(&root, &manifest.source.sha, fs_safe::uid())
            .unwrap()
            .is_none());
        installer.stage(&bundle, &manifest).unwrap();
        fs::rename(
            root.join("releases").join(&manifest.source.sha),
            root.join("retained-release"),
        )
        .unwrap();
        let (proof, binaries) = build_proof(&root, &manifest.source.sha, fs_safe::uid())
            .unwrap()
            .unwrap();
        assert_eq!(proof, manifest);
        assert_eq!(
            binaries,
            root.join("maintenance-releases").join(&manifest.source.sha)
        );
        assert!(staged(&root, &manifest.source.sha, fs_safe::uid()).is_err());
        installer.stage(&bundle, &proof).unwrap();
        assert_eq!(
            staged(&root, &manifest.source.sha, fs_safe::uid()).unwrap(),
            manifest
        );
    }

    #[test]
    fn forged_bundle_and_manifest_do_not_match_staged_provenance() {
        let (_dir, root, bundle, mut manifest) = fixture();
        manifest.format = 3;
        let installer = Installer::open(&root, fs_safe::uid()).unwrap();
        installer.stage(&bundle, &manifest).unwrap();
        let trusted = staged(&root, &manifest.source.sha, fs_safe::uid()).unwrap();
        assert_eq!(trusted, manifest);
        fs::write(bundle.join("brain-serve"), b"\x7fELFforged").unwrap();
        let artifact = manifest
            .artifacts
            .iter_mut()
            .find(|a| a.name == "brain-serve")
            .unwrap();
        artifact.sha256 = fs_safe::file_hash(&bundle.join("brain-serve"), OPERATOR).unwrap();
        assert!(crate::source::compare_artifacts(&manifest.artifacts, &trusted.artifacts).is_err());
        assert!(installer.stage(&bundle, &manifest).is_err());
        assert!(fs::symlink_metadata(root.join("current")).is_err());
        fs::set_permissions(
            root.join("releases")
                .join(&trusted.source.sha)
                .join("bin/brain-serve"),
            fs::Permissions::from_mode(0o700),
        )
        .unwrap();
        fs::write(
            root.join("releases")
                .join(&trusted.source.sha)
                .join("bin/brain-serve"),
            b"\x7fELFchanged",
        )
        .unwrap();
        assert!(staged(&root, &trusted.source.sha, fs_safe::uid()).is_err());
    }

    #[test]
    fn drift_and_missing_binary_never_change_pointers() {
        let (_dir, root, bundle, mut manifest) = fixture();
        let install = Installer::open(&root, fs_safe::uid()).unwrap();
        fs::set_permissions(
            bundle.join("brain-serve"),
            fs::Permissions::from_mode(0o700),
        )
        .unwrap();
        fs::write(bundle.join("brain-serve"), b"\x7fELFchanged").unwrap();
        assert!(install.stage(&bundle, &manifest).is_err());
        assert!(fs::symlink_metadata(root.join("current")).is_err());
        manifest.artifacts.pop();
        assert!(install.stage(&bundle, &manifest).is_err());
    }

    #[test]
    fn failed_second_switch_rolls_back_first_and_reports_error() {
        let (_dir, root, bundle, manifest) = fixture();
        let install = Installer::open(&root, fs_safe::uid()).unwrap();
        install.stage(&bundle, &manifest).unwrap();
        install.activate(&manifest.source.sha).unwrap();
        let mut next = manifest.clone();
        next.source.sha = "d".repeat(40);
        install.stage(&bundle, &next).unwrap();
        fs_safe::new_file(&root.join(".next-maintenance-current"), b"collision", 0o600).unwrap();
        assert!(install.activate(&next.source.sha).is_err());
        assert_eq!(
            fs::read_link(root.join("current")).unwrap(),
            root.join("releases").join(&manifest.source.sha)
        );
        assert!(root.join(".deploy-pending.json").exists());
        fs::remove_file(root.join(".next-maintenance-current")).unwrap();
        install.recover().unwrap();
        assert!(!root.join(".deploy-pending.json").exists());
        assert_eq!(
            fs::read_link(root.join("maintenance-current")).unwrap(),
            root.join("maintenance-releases").join(&manifest.source.sha)
        );
    }

    #[test]
    fn symlink_artifact_and_release_are_rejected() {
        let (_dir, root, bundle, manifest) = fixture();
        let install = Installer::open(&root, fs_safe::uid()).unwrap();
        fs::remove_file(bundle.join("brain-serve")).unwrap();
        symlink(bundle.join("deadlock-brain"), bundle.join("brain-serve")).unwrap();
        assert!(install.stage(&bundle, &manifest).is_err());
        symlink(&bundle, root.join("releases").join(&manifest.source.sha)).unwrap();
        assert!(install.installed(&manifest.source.sha).is_err());
    }

    #[test]
    fn recovers_crash_between_pointer_updates_with_stale_symlink() {
        let (_dir, root, bundle, manifest) = fixture();
        let install = Installer::open(&root, fs_safe::uid()).unwrap();
        install.stage(&bundle, &manifest).unwrap();
        install.activate(&manifest.source.sha).unwrap();
        let mut next = manifest.clone();
        next.source.sha = "d".repeat(40);
        install.stage(&bundle, &next).unwrap();
        let pending = Pending {
            sha: next.source.sha.clone(),
            current: Some(root.join("releases").join(&manifest.source.sha)),
            maintenance: Some(root.join("maintenance-releases").join(&manifest.source.sha)),
        };
        fs_safe::new_file(
            &root.join(".deploy-pending.json"),
            &serde_json::to_vec(&pending).unwrap(),
            0o600,
        )
        .unwrap();
        install
            .atomic_link(
                "current",
                Some(&root.join("releases").join(&next.source.sha)),
            )
            .unwrap();
        symlink(
            root.join("maintenance-releases").join(&next.source.sha),
            root.join(".next-maintenance-current"),
        )
        .unwrap();
        assert!(install.activate(&next.source.sha).is_err());
        install.recover().unwrap();
        assert_eq!(
            install.link_target("current", "releases").unwrap(),
            pending.current
        );
        assert_eq!(
            install
                .link_target("maintenance-current", "maintenance-releases")
                .unwrap(),
            pending.maintenance
        );
        assert!(!root.join(".deploy-pending.json").exists());
    }

    #[test]
    fn rejects_missing_writer_path_traversal_and_hardlinks() {
        let (_dir, root, bundle, manifest) = fixture();
        let install = Installer::open(&root, fs_safe::uid()).unwrap();
        let mut missing = manifest.clone();
        missing
            .artifacts
            .retain(|artifact| artifact.name != "brain-patchnotes-ingest");
        assert!(install.stage(&bundle, &missing).is_err());
        let mut traversal = manifest.clone();
        traversal.artifacts.push(Artifact {
            name: "../outside".into(),
            sha256: "a".repeat(64),
        });
        assert!(install.stage(&bundle, &traversal).is_err());
        fs::hard_link(bundle.join("brain-serve"), bundle.join("alias")).unwrap();
        assert!(install.stage(&bundle, &manifest).is_err());
        assert!(fs::symlink_metadata(root.join("current")).is_err());
    }

    #[test]
    fn dangling_release_symlink_is_never_overwritten() {
        let (_dir, root, bundle, manifest) = fixture();
        let install = Installer::open(&root, fs_safe::uid()).unwrap();
        let destination = root.join("releases").join(&manifest.source.sha);
        symlink(root.join("missing"), &destination).unwrap();
        assert!(install.stage(&bundle, &manifest).is_err());
        assert!(fs::symlink_metadata(destination)
            .unwrap()
            .file_type()
            .is_symlink());
    }

    #[test]
    fn preflight_failure_preserves_both_pointers() {
        let (_dir, root, bundle, manifest) = fixture();
        let install = Installer::open(&root, fs_safe::uid()).unwrap();
        install.stage(&bundle, &manifest).unwrap();
        install.activate(&manifest.source.sha).unwrap();
        let mut next = manifest.clone();
        next.source.sha = "d".repeat(40);
        install.stage(&bundle, &next).unwrap();
        assert!(install
            .activate_checked(&next.source.sha, || Err(anyhow::anyhow!(
                "Herkunftsprüfung fehlgeschlagen"
            )))
            .is_err());
        assert_eq!(
            install.link_target("current", "releases").unwrap(),
            Some(root.join("releases").join(&manifest.source.sha))
        );
        assert_eq!(
            install
                .link_target("maintenance-current", "maintenance-releases")
                .unwrap(),
            Some(root.join("maintenance-releases").join(&manifest.source.sha))
        );
        assert!(!root.join(".deploy-pending.json").exists());
    }

    #[test]
    fn crash_worker() {
        let Some(root) = std::env::var_os("BRAIN_RELEASE_TEST_ROOT") else {
            return;
        };
        let root = PathBuf::from(root);
        let bundle = root.parent().unwrap().join("bundle");
        let manifest: Manifest = serde_json::from_slice(
            &fs_safe::bytes(&bundle.join("manifest.json"), OPERATOR, 65536).unwrap(),
        )
        .unwrap();
        let point = std::env::var("BRAIN_RELEASE_TEST_CRASH").unwrap();
        let installer = Installer::open(&root, fs_safe::uid()).unwrap();
        let checkpoint = |position: &str| -> Result<()> {
            if position == point {
                std::process::exit(73);
            }
            Ok(())
        };
        if point.starts_with("after-journal-") {
            let pending = Pending {
                sha: manifest.source.sha,
                current: installer.link_target("current", "releases").unwrap(),
                maintenance: installer
                    .link_target("maintenance-current", "maintenance-releases")
                    .unwrap(),
            };
            installer.publish_pending(&pending, checkpoint).unwrap();
        } else {
            installer
                .stage_checked(&bundle, &manifest, checkpoint)
                .unwrap();
        }
        panic!("Abbruchpunkt wurde nicht erreicht");
    }

    fn terminate_at(root: &Path, bundle: &Path, manifest: &Manifest, point: &str) {
        fs_safe::new_file(
            &bundle.join("manifest.json"),
            &serde_json::to_vec(manifest).unwrap(),
            0o600,
        )
        .unwrap();
        let status = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "install::tests::crash_worker",
                "--nocapture",
                "--test-threads=1",
            ])
            .env("BRAIN_RELEASE_TEST_ROOT", root)
            .env("BRAIN_RELEASE_TEST_CRASH", point)
            .status()
            .unwrap();
        assert_eq!(status.code(), Some(73));
    }

    #[test]
    fn process_exit_during_layout_publication_can_be_retried() {
        for point in [
            "before-layouts",
            "after-cli-layout",
            "after-maintenance-layout",
        ] {
            let (_dir, root, bundle, manifest) = fixture();
            let installer = Installer::open(&root, fs_safe::uid()).unwrap();
            installer.stage(&bundle, &manifest).unwrap();
            installer.activate(&manifest.source.sha).unwrap();
            let old_inode = fs::metadata(root.join("releases").join(&manifest.source.sha))
                .unwrap()
                .ino();
            drop(installer);
            let mut next = manifest.clone();
            next.source.sha = "d".repeat(40);
            terminate_at(&root, &bundle, &next, point);
            let installer = Installer::open(&root, fs_safe::uid()).unwrap();
            assert_eq!(
                installer.link_target("current", "releases").unwrap(),
                Some(root.join("releases").join(&manifest.source.sha))
            );
            let published_inode = fs::metadata(root.join("releases").join(&next.source.sha))
                .ok()
                .map(|metadata| metadata.ino());
            installer.recover().unwrap();
            installer.stage(&bundle, &next).unwrap();
            assert_eq!(installer.installed(&next.source.sha).unwrap(), next);
            assert_eq!(installer.installed(&manifest.source.sha).unwrap(), manifest);
            assert_eq!(
                fs::metadata(root.join("releases").join(&manifest.source.sha))
                    .unwrap()
                    .ino(),
                old_inode
            );
            if let Some(ino) = published_inode {
                assert_eq!(
                    fs::metadata(root.join("releases").join(&next.source.sha))
                        .unwrap()
                        .ino(),
                    ino
                );
            }
            installer.activate(&next.source.sha).unwrap();
        }
    }

    #[test]
    fn process_exit_during_journal_publication_is_recoverable() {
        for point in ["after-journal-write", "after-journal-publish"] {
            let (_dir, root, bundle, manifest) = fixture();
            let installer = Installer::open(&root, fs_safe::uid()).unwrap();
            installer.stage(&bundle, &manifest).unwrap();
            installer.activate(&manifest.source.sha).unwrap();
            let mut next = manifest.clone();
            next.source.sha = "d".repeat(40);
            installer.stage(&bundle, &next).unwrap();
            drop(installer);
            terminate_at(&root, &bundle, &next, point);
            let installer = Installer::open(&root, fs_safe::uid()).unwrap();
            installer.recover().unwrap();
            assert_eq!(
                installer.link_target("current", "releases").unwrap(),
                Some(root.join("releases").join(&manifest.source.sha))
            );
            assert_eq!(
                installer
                    .link_target("maintenance-current", "maintenance-releases")
                    .unwrap(),
                Some(root.join("maintenance-releases").join(&manifest.source.sha))
            );
            assert!(!exists(&root.join(".deploy-pending.json")).unwrap());
            assert!(!exists(&root.join(".deploy-pending.json.tmp")).unwrap());
            installer.activate(&next.source.sha).unwrap();
        }
    }

    #[test]
    fn partial_journal_and_missing_cli_layout_can_be_retried() {
        let (_dir, root, bundle, manifest) = fixture();
        let installer = Installer::open(&root, fs_safe::uid()).unwrap();
        installer.stage(&bundle, &manifest).unwrap();
        let maintenance = root.join("maintenance-releases").join(&manifest.source.sha);
        let ino = fs::metadata(&maintenance).unwrap().ino();
        fs::rename(
            root.join("releases").join(&manifest.source.sha),
            root.join("retained-layout"),
        )
        .unwrap();
        fs_safe::new_file(&root.join(".deploy-pending.json.tmp"), b"{\"sha\":", 0o600).unwrap();
        installer.stage(&bundle, &manifest).unwrap();
        assert_eq!(fs::metadata(maintenance).unwrap().ino(), ino);
        installer.activate(&manifest.source.sha).unwrap();
        assert_eq!(installer.installed(&manifest.source.sha).unwrap(), manifest);
        assert!(!exists(&root.join(".deploy-pending.json.tmp")).unwrap());
    }

    #[test]
    fn mismatched_partial_layout_and_foreign_journal_are_preserved() {
        let (_dir, root, bundle, manifest) = fixture();
        let installer = Installer::open(&root, fs_safe::uid()).unwrap();
        installer.stage(&bundle, &manifest).unwrap();
        fs::rename(
            root.join("maintenance-releases").join(&manifest.source.sha),
            root.join("retained-maintenance"),
        )
        .unwrap();
        let mut forged = manifest.clone();
        forged.artifacts[0].sha256 = "f".repeat(64);
        assert!(installer.stage(&bundle, &forged).is_err());
        assert_eq!(
            installer
                .layout(
                    &root.join("releases").join(&manifest.source.sha),
                    "bin",
                    &manifest.source.sha
                )
                .unwrap(),
            manifest
        );
        installer.stage(&bundle, &manifest).unwrap();
        symlink(root.join("missing"), root.join(".deploy-pending.json")).unwrap();
        assert!(installer.activate(&manifest.source.sha).is_err());
        assert!(installer.recover().is_err());
        assert!(fs::symlink_metadata(root.join(".deploy-pending.json"))
            .unwrap()
            .file_type()
            .is_symlink());
    }

    #[test]
    fn lock_blocks_an_independent_process() {
        let (_dir, root, _, _) = fixture();
        let _install = Installer::open(&root, fs_safe::uid()).unwrap();
        let status = std::process::Command::new("/usr/bin/flock")
            .args(["--nonblock", "--exclusive"])
            .arg(root.join(".deploy.lock"))
            .arg("/usr/bin/true")
            .status()
            .unwrap();
        assert_eq!(status.code(), Some(1));
    }
}
