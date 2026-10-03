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
            !self.root.join(".deploy-pending.json").try_exists()?,
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
        self.available()?;
        validate_manifest(manifest)?;
        let sha = &manifest.source.sha;
        let cli = self.root.join("releases").join(sha);
        let maintenance = self.root.join("maintenance-releases").join(sha);
        let exists = |path: &Path| -> Result<bool> {
            match fs::symlink_metadata(path) {
                Ok(_) => Ok(true),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
                Err(error) => Err(error.into()),
            }
        };
        let cli_exists = exists(&cli)?;
        let maintenance_exists = exists(&maintenance)?;
        if cli_exists || maintenance_exists {
            ensure!(
                cli_exists && maintenance_exists,
                "Nur ein Release-Layout vorhanden"
            );
            ensure!(
                self.installed(sha)? == *manifest,
                "Vorhandenes Release weicht ab"
            );
            return Ok(());
        }
        let stage = self.root.join(format!(".stage-{}", std::process::id()));
        fs_safe::new_dir(&stage)?;
        let a = stage.join("release");
        let b = stage.join("maintenance");
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
            }
        }
        let data = serde_json::to_vec_pretty(manifest)?;
        fs_safe::new_file(&a.join("manifest.json"), &data, 0o444)?;
        fs_safe::new_file(&b.join("manifest.json"), &data, 0o444)?;
        for dir in [&a.join("bin"), &a, &b] {
            fs::set_permissions(dir, fs::Permissions::from_mode(0o755))?;
            fs_safe::sync_dir(dir)?;
        }
        fs::rename(&a, &cli)?;
        fs_safe::sync_dir(&self.root.join("releases"))?;
        fs::rename(&b, &maintenance)?;
        fs_safe::sync_dir(&self.root.join("maintenance-releases"))?;
        fs::remove_dir(stage)?;
        ensure!(
            self.installed(sha)? == *manifest,
            "Installationsnachweis weicht ab"
        );
        Ok(())
    }

    pub fn installed(&self, sha: &str) -> Result<Manifest> {
        fs_safe::sha(sha)?;
        let a = self.root.join("releases").join(sha);
        let b = self.root.join("maintenance-releases").join(sha);
        fs_safe::checked_path(&a, self.owner, true)?;
        fs_safe::checked_path(&b, self.owner, true)?;
        let left = fs_safe::bytes(&a.join("manifest.json"), self.owner, 65536)?;
        let right = fs_safe::bytes(&b.join("manifest.json"), self.owner, 65536)?;
        ensure!(left == right, "Release-Manifeste unterscheiden sich");
        let manifest: Manifest = serde_json::from_slice(&left)?;
        validate_manifest(&manifest)?;
        ensure!(manifest.source.sha == sha, "Release-SHA stimmt nicht");
        for artifact in &manifest.artifacts {
            ensure!(
                fs_safe::file_hash(&a.join("bin").join(&artifact.name), self.owner)?
                    == artifact.sha256,
                "Installiertes Binary verändert"
            );
            ensure!(
                fs_safe::file_hash(&b.join(&artifact.name), self.owner)? == artifact.sha256,
                "Maintenance-Binary verändert"
            );
        }
        Ok(manifest)
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

    pub fn activate(&self, sha: &str) -> Result<()> {
        self.available()?;
        self.installed(sha)?;
        let pending = Pending {
            sha: sha.to_string(),
            current: self.link_target("current", "releases")?,
            maintenance: self.link_target("maintenance-current", "maintenance-releases")?,
        };
        fs_safe::new_file(
            &self.root.join(".deploy-pending.json"),
            &serde_json::to_vec(&pending)?,
            0o600,
        )?;
        fs_safe::sync_dir(&self.root)?;
        let result = (|| {
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
        fs::remove_file(self.root.join(".deploy-pending.json"))?;
        fs_safe::sync_dir(&self.root)
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
            format: 1,
            source: Source {
                sha: "a".repeat(40),
                tree: "b".repeat(40),
                fingerprint: "c".repeat(64),
                path: "/not-executed".into(),
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
