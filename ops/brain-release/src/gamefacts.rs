use crate::{fs_safe, install, source};
use anyhow::{ensure, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::fs::{FileTypeExt, MetadataExt, OpenOptionsExt, PermissionsExt};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const DBA: &str = "deadlock-brain-pg";
const CONFIG_NAME: &str = "gamefacts-migrate.json";

#[derive(Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Config {
    socket: PathBuf,
    port: u16,
    database: String,
    user: String,
}

fn fixed_config() -> Config {
    Config {
        socket: "/run/deadlock-brain-postgresql".into(),
        port: 5446,
        database: "brain".into(),
        user: DBA.into(),
    }
}

#[derive(Clone, Copy)]
struct Identity {
    uid: u32,
    gid: u32,
}

fn account_record(kind: &str) -> Result<String> {
    let program = Path::new("/usr/bin/getent");
    fs_safe::checked_path(program, 0, false)?;
    fs_safe::regular(program, 0)?;
    let output = source::command("/usr/bin/getent")
        .current_dir("/")
        .args([kind, DBA])
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()?;
    ensure!(
        output.status.success() && output.stdout.len() <= 4096,
        "Lokale PostgreSQL-Identität konnte nicht geprüft werden"
    );
    let text = String::from_utf8(output.stdout)?;
    ensure!(
        text.lines().count() == 1,
        "Mehrdeutige PostgreSQL-Identität"
    );
    Ok(text.trim_end().to_string())
}

fn identity() -> Result<Identity> {
    let passwd = account_record("passwd")?;
    let user: Vec<_> = passwd.split(':').collect();
    let group = account_record("group")?;
    let group: Vec<_> = group.split(':').collect();
    ensure!(
        user.len() == 7 && user[0] == DBA && group.len() == 4 && group[0] == DBA,
        "Unerwartete PostgreSQL-Identität"
    );
    let uid: u32 = user[2].parse().context("Ungültige PostgreSQL-UID")?;
    let gid: u32 = user[3].parse().context("Ungültige PostgreSQL-GID")?;
    ensure!(
        uid != 0
            && uid != source::OPERATOR
            && gid != 0
            && gid != source::OPERATOR
            && group[2].parse::<u32>()? == gid,
        "PostgreSQL-Identität ist nicht ausreichend getrennt"
    );
    Ok(Identity { uid, gid })
}

fn check_socket(directory: &Path, identity: Identity) -> Result<()> {
    fs_safe::checked_path(directory, identity.uid, true)?;
    let socket = directory.join(format!(".s.PGSQL.{}", fixed_config().port));
    let meta = fs::symlink_metadata(socket)?;
    ensure!(
        meta.file_type().is_socket()
            && meta.uid() == identity.uid
            && meta.nlink() == 1
            && meta.mode() & 0o002 == 0,
        "Unsicherer lokaler PostgreSQL-Socket"
    );
    Ok(())
}

fn migration_config(root: &Path, owner: u32) -> Result<PathBuf> {
    fs_safe::checked_path(root, owner, true)?;
    let path = root.join(CONFIG_NAME);
    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o444)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
        .open(&path)
    {
        Ok(mut file) => {
            file.write_all(&serde_json::to_vec(&fixed_config())?)?;
            file.set_permissions(fs::Permissions::from_mode(0o444))?;
            file.sync_all()?;
            fs_safe::sync_dir(root)?;
        }
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => (),
        Err(error) => {
            return Err(error).context("Migrationskonfiguration konnte nicht angelegt werden")
        }
    }
    let file = fs_safe::regular(&path, owner)?;
    ensure!(
        file.metadata()?.mode() & 0o7777 == 0o444,
        "Migrationskonfiguration benötigt den Dateimodus 0444"
    );
    let config: Config = serde_json::from_slice(&fs_safe::bytes(&path, owner, 4096)?)
        .map_err(|_| anyhow::anyhow!("Ungültige feste Migrationskonfiguration"))?;
    ensure!(
        config == fixed_config(),
        "Migrationskonfiguration weicht vom festen lokalen Ziel ab"
    );
    Ok(path)
}

fn migration_binary(root: &Path, owner: u32, manifest: &source::Manifest) -> Result<PathBuf> {
    ensure!(
        manifest.format == 3,
        "Spielwertmigration benötigt einen bestätigten Einmal-Build"
    );
    ensure!(
        install::staged(root, &manifest.source.sha, owner)? == *manifest,
        "Migrationsrelease weicht vom bestätigten Build ab"
    );
    let artifact = manifest
        .artifacts
        .iter()
        .find(|artifact| artifact.name == "brain-migrate")
        .context("Bestätigtes Release enthält kein brain-migrate")?;
    let path = root
        .join("releases")
        .join(&manifest.source.sha)
        .join("bin/brain-migrate");
    let mut file = fs_safe::regular(&path, owner)?;
    ensure!(
        file.metadata()?.mode() & 0o111 != 0 && file.metadata()?.mode() & 0o6000 == 0,
        "Unsicherer oder nicht ausführbarer Migrator"
    );
    let mut header = [0u8; 4];
    file.read_exact(&mut header)?;
    ensure!(header == *b"\x7fELF", "Migrator ist kein ELF-Binary");
    ensure!(
        fs_safe::file_hash(&path, owner)? == artifact.sha256,
        "Bestätigter Migrator wurde verändert"
    );
    Ok(path)
}

fn migration_command(program: &Path, config: &Path) -> Command {
    let mut command = Command::new(program);
    command
        .env_clear()
        .env("HOME", "/")
        .env("PATH", "/usr/bin:/bin")
        .env("LANG", "C.UTF-8")
        .current_dir("/")
        .arg("up-gamefacts")
        .arg("--config")
        .arg(config)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    command
}

fn migrate(program: &Path, config: &Path, identity: Identity) -> Result<()> {
    let mut command = migration_command(program, config);
    if fs_safe::uid() == 0 {
        unsafe {
            command.pre_exec(move || {
                if libc::setgroups(0, std::ptr::null()) != 0
                    || libc::setgid(identity.gid) != 0
                    || libc::setuid(identity.uid) != 0
                    || libc::geteuid() != identity.uid
                    || libc::getegid() != identity.gid
                {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            });
        }
    } else {
        ensure!(
            fs_safe::uid() == identity.uid && unsafe { libc::getegid() } == identity.gid,
            "Migrator darf nur mit der geprüften PostgreSQL-Identität laufen"
        );
    }
    let status = command
        .status()
        .context("Spielwertmigration konnte nicht gestartet werden")?;
    ensure!(
        status.success(),
        "Spielwertmigration fehlgeschlagen (Exit {:?}); Release wird nicht aktiviert",
        status.code()
    );
    Ok(())
}

fn activate_at(
    installer: &install::Installer,
    root: &Path,
    owner: u32,
    manifest: &source::Manifest,
    identity: Identity,
    mut verify: impl FnMut() -> Result<()>,
) -> Result<()> {
    installer.activate_checked(&manifest.source.sha, || {
        verify()?;
        let program = migration_binary(root, owner, manifest)?;
        let config = migration_config(root, owner)?;
        migrate(&program, &config, identity)?;
        ensure!(
            installer.installed(&manifest.source.sha)? == *manifest,
            "Bestätigtes Release während Migration verändert"
        );
        verify()
    })
}

pub fn activate(
    installer: &install::Installer,
    manifest: &source::Manifest,
    verify: impl FnMut() -> Result<()>,
) -> Result<()> {
    ensure!(
        fs_safe::uid() == 0,
        "Migrationsintegration benötigt den festen Root-Helfer"
    );
    let identity = identity()?;
    check_socket(&fixed_config().socket, identity)?;
    activate_at(
        installer,
        Path::new(install::ROOT),
        0,
        manifest,
        identity,
        verify,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::{Artifact, Manifest, Source, BINS, BUILD_ARGS, ORIGIN};
    use std::cell::Cell;
    use std::os::unix::fs::symlink;
    use tempfile::{tempdir_in, TempDir};

    fn fixture(binary: &[u8]) -> (TempDir, PathBuf, PathBuf, Manifest) {
        let dir = tempdir_in("/home/nathanael/.cache").unwrap();
        let root = dir.path().join("root");
        let bundle = dir.path().join("bundle");
        fs_safe::new_dir(&root).unwrap();
        fs_safe::new_dir(&root.join("releases")).unwrap();
        fs_safe::new_dir(&root.join("maintenance-releases")).unwrap();
        fs_safe::new_dir(&bundle).unwrap();
        let mut artifacts = Vec::new();
        for name in BINS.into_iter().chain(["brain-migrate"]) {
            fs_safe::new_file(&bundle.join(name), binary, 0o700).unwrap();
            artifacts.push(Artifact {
                name: name.into(),
                sha256: fs_safe::hash(binary),
            });
        }
        let manifest = Manifest {
            format: 3,
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
            build_args: BUILD_ARGS.iter().map(|value| value.to_string()).collect(),
            artifacts,
        };
        (dir, root, bundle, manifest)
    }

    fn test_identity() -> Identity {
        Identity {
            uid: fs_safe::uid(),
            gid: unsafe { libc::getegid() },
        }
    }

    fn assert_active_release(root: &Path, sha: &str) {
        let release = root.join("releases").join(sha);
        assert_eq!(fs::read_link(root.join("current")).unwrap(), release);
        assert_eq!(
            fs::read_link(root.join("maintenance-current")).unwrap(),
            root.join("current/bin")
        );
        for name in BINS.into_iter().chain(["brain-migrate"]) {
            let expected = release.join("bin").join(name);
            for prefix in ["current/bin", "maintenance-current"] {
                assert_eq!(
                    fs::canonicalize(root.join(prefix).join(name)).unwrap(),
                    expected
                );
            }
        }
    }

    fn probe() -> (TempDir, Vec<u8>) {
        let dir = tempdir_in("/home/nathanael/.cache").unwrap();
        let source = dir.path().join("probe.rs");
        let binary = dir.path().join("probe");
        fs::write(&source, r#"
use std::{fs, io::Write, path::Path, process::{Command, ExitCode, Stdio}};
fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    assert_eq!(args.len(), 3);
    assert_eq!(args[0], "up-gamefacts");
    assert_eq!(args[1], "--config");
    let config = Path::new(&args[2]);
    assert_eq!(config.file_name().unwrap(), "gamefacts-migrate.json");
    let root = config.parent().unwrap();
    let active = root.join("releases").join("a".repeat(40));
    assert_eq!(fs::read_link(root.join("current")).unwrap(), active);
    assert_eq!(fs::read_link(root.join("maintenance-current")).unwrap(), root.join("current/bin"));
    for prefix in ["current/bin", "maintenance-current"] {
        assert_eq!(fs::canonicalize(root.join(prefix)).unwrap(), active.join("bin"));
        for binary in fs::read_dir(active.join("bin")).unwrap() {
            let binary = binary.unwrap();
            assert_eq!(fs::canonicalize(root.join(prefix).join(binary.file_name())).unwrap(), binary.path());
        }
    }
    assert_eq!(std::env::current_exe().unwrap(), root.join("releases").join("d".repeat(40)).join("bin/brain-migrate"));
    assert!(root.join(".deploy-pending.json").is_file());
    assert_eq!(std::env::current_dir().unwrap(), Path::new("/"));
    let mut env: Vec<_> = std::env::vars().collect();
    env.sort();
    assert_eq!(env, vec![("HOME".into(), "/".into()), ("LANG".into(), "C.UTF-8".into()), ("PATH".into(), "/usr/bin:/bin".into())]);
    let status = Command::new("/usr/bin/flock").env_clear()
        .args(["--nonblock", "--exclusive"]).arg(root.join(".deploy.lock"))
        .arg("/usr/bin/true").stdin(Stdio::null()).status().unwrap();
    assert_eq!(status.code(), Some(1));
    fs::OpenOptions::new().write(true).create_new(true).open(root.join("migration-ran")).unwrap()
        .write_all(b"before-pointer-change").unwrap();
    if root.join("migration-fail").exists() { ExitCode::from(73) } else { ExitCode::SUCCESS }
}
"#).unwrap();
        let status = source::command("/home/nathanael/.cargo/bin/rustc")
            .args(["--edition=2021", "-O", "--crate-name", "migration_probe"])
            .arg(&source)
            .arg("-o")
            .arg(&binary)
            .status()
            .unwrap();
        assert!(status.success());
        let bytes = fs::read(binary).unwrap();
        (dir, bytes)
    }

    #[test]
    fn migration_process_runs_under_lock_before_both_pointer_changes() {
        let (_probe, binary) = probe();
        for fail in [false, true] {
            let (_dir, root, bundle, manifest) = fixture(&binary);
            let installer = install::Installer::open(&root, fs_safe::uid()).unwrap();
            installer.stage(&bundle, &manifest).unwrap();
            installer.activate(&manifest.source.sha).unwrap();
            let mut next = manifest.clone();
            next.source.sha = "d".repeat(40);
            installer.stage(&bundle, &next).unwrap();
            if fail {
                fs_safe::new_file(&root.join("migration-fail"), b"", 0o600).unwrap();
            }
            let checks = Cell::new(0);
            let result = activate_at(
                &installer,
                &root,
                fs_safe::uid(),
                &next,
                test_identity(),
                || {
                    assert_active_release(&root, &manifest.source.sha);
                    assert_eq!(root.join("migration-ran").exists(), checks.get() == 1);
                    checks.set(checks.get() + 1);
                    Ok(())
                },
            );
            assert_eq!(result.is_err(), fail);
            if let Err(error) = result {
                assert!(error.to_string().contains("Exit Some(73)"));
            }
            assert_eq!(checks.get(), if fail { 1 } else { 2 });
            assert_eq!(
                fs::read(root.join("migration-ran")).unwrap(),
                b"before-pointer-change"
            );
            let active = if fail {
                &manifest.source.sha
            } else {
                &next.source.sha
            };
            assert_active_release(&root, active);
            assert!(!root.join(".deploy-pending.json").exists());
        }
    }

    #[test]
    fn socket_type_owner_and_symlinks_are_checked() {
        let (_dir, root, _, _) = fixture(b"\x7fELFtest");
        let path = root.join(".s.PGSQL.5446");
        let listener = std::os::unix::net::UnixListener::bind(&path).unwrap();
        assert!(check_socket(&root, test_identity()).is_ok());
        let mut foreign = test_identity();
        foreign.uid += 1;
        assert!(check_socket(&root, foreign).is_err());
        drop(listener);
        fs::remove_file(&path).unwrap();
        fs_safe::new_file(&path, b"not-a-socket", 0o600).unwrap();
        assert!(check_socket(&root, test_identity()).is_err());
        fs::remove_file(&path).unwrap();
        symlink(root.join("missing"), &path).unwrap();
        assert!(check_socket(&root, test_identity()).is_err());
        assert!(fs::symlink_metadata(path).unwrap().file_type().is_symlink());
    }

    #[test]
    fn config_is_fixed_and_existing_foreign_content_is_never_overwritten() {
        let (_dir, root, _, _) = fixture(b"\x7fELFtest");
        let path = migration_config(&root, fs_safe::uid()).unwrap();
        assert_eq!(migration_config(&root, fs_safe::uid()).unwrap(), path);
        assert_eq!(fs::metadata(&path).unwrap().mode() & 0o7777, 0o444);
        for config in [
            br#"{"socket":"/other","port":5446,"database":"brain","user":"deadlock-brain-pg"}"#.as_slice(),
            br#"{"socket":"/run/deadlock-brain-postgresql","port":5446,"database":"other","user":"deadlock-brain-pg"}"#,
            br#"{"socket":"/run/deadlock-brain-postgresql","port":5446,"database":"brain","user":"brain_migrate"}"#,
            br#"{"socket":"/run/deadlock-brain-postgresql","port":5446,"database":"brain","user":"deadlock-brain-pg","command":"up"}"#,
            br#"{"socket":"/run/deadlock-brain-postgresql","port":"redaction-probe","database":"brain","user":"deadlock-brain-pg"}"#,
        ] {
            fs::remove_file(&path).unwrap();
            fs_safe::new_file(&path, config, 0o444).unwrap();
            let error = migration_config(&root, fs_safe::uid()).unwrap_err();
            assert!(!format!("{error:#}").contains("redaction-probe"));
            assert_eq!(fs::read(&path).unwrap(), config);
        }
    }

    #[test]
    fn config_symlinks_hardlinks_and_writable_modes_are_rejected() {
        let (_dir, root, _, _) = fixture(b"\x7fELFtest");
        let foreign = root.join("foreign.json");
        let content = serde_json::to_vec(&fixed_config()).unwrap();
        fs_safe::new_file(&foreign, &content, 0o444).unwrap();
        let path = root.join(CONFIG_NAME);
        symlink(&foreign, &path).unwrap();
        assert!(migration_config(&root, fs_safe::uid()).is_err());
        assert!(fs::symlink_metadata(&path)
            .unwrap()
            .file_type()
            .is_symlink());
        assert_eq!(fs::read(&foreign).unwrap(), content);
        fs::remove_file(&path).unwrap();
        fs::hard_link(&foreign, &path).unwrap();
        assert!(migration_config(&root, fs_safe::uid()).is_err());
        fs::remove_file(&path).unwrap();
        fs_safe::new_file(&path, &content, 0o644).unwrap();
        assert!(migration_config(&root, fs_safe::uid()).is_err());
        assert_eq!(fs::metadata(path).unwrap().mode() & 0o7777, 0o644);
    }

    #[test]
    fn missing_unattested_and_tampered_migrators_never_activate() {
        let binary = fs::read("/usr/bin/true").unwrap();
        let (_dir, root, bundle, manifest) = fixture(&binary);
        let installer = install::Installer::open(&root, fs_safe::uid()).unwrap();
        installer.stage(&bundle, &manifest).unwrap();
        installer.activate(&manifest.source.sha).unwrap();
        let mut next = manifest.clone();
        next.source.sha = "d".repeat(40);
        next.artifacts
            .retain(|artifact| artifact.name != "brain-migrate");
        installer.stage(&bundle, &next).unwrap();
        assert!(activate_at(
            &installer,
            &root,
            fs_safe::uid(),
            &next,
            test_identity(),
            || Ok(())
        )
        .is_err());
        next.source.sha = "e".repeat(40);
        next.artifacts = manifest.artifacts.clone();
        next.format = 2;
        installer.stage(&bundle, &next).unwrap();
        assert!(activate_at(
            &installer,
            &root,
            fs_safe::uid(),
            &next,
            test_identity(),
            || Ok(())
        )
        .is_err());
        next.source.sha = "f".repeat(40);
        next.format = 3;
        installer.stage(&bundle, &next).unwrap();
        let path = root
            .join("releases")
            .join(&next.source.sha)
            .join("bin/brain-migrate");
        fs::set_permissions(&path, fs::Permissions::from_mode(0o700)).unwrap();
        fs::write(path, b"\x7fELFchanged").unwrap();
        assert!(activate_at(
            &installer,
            &root,
            fs_safe::uid(),
            &next,
            test_identity(),
            || Ok(())
        )
        .is_err());
        assert_eq!(
            fs::read_link(root.join("current")).unwrap(),
            root.join("releases").join(&manifest.source.sha)
        );
        assert!(!root.join(CONFIG_NAME).exists());
        assert!(!root.join(".deploy-pending.json").exists());
    }

    #[test]
    fn source_recheck_failure_after_migration_preserves_old_pointers() {
        let (_probe, binary) = probe();
        let (_dir, root, bundle, manifest) = fixture(&binary);
        let installer = install::Installer::open(&root, fs_safe::uid()).unwrap();
        installer.stage(&bundle, &manifest).unwrap();
        installer.activate(&manifest.source.sha).unwrap();
        let mut next = manifest.clone();
        next.source.sha = "d".repeat(40);
        installer.stage(&bundle, &next).unwrap();
        let checks = Cell::new(0);
        let result = activate_at(
            &installer,
            &root,
            fs_safe::uid(),
            &next,
            test_identity(),
            || {
                assert_active_release(&root, &manifest.source.sha);
                assert_eq!(root.join("migration-ran").exists(), checks.get() == 1);
                checks.set(checks.get() + 1);
                ensure!(checks.get() == 1, "Geänderte Herkunft");
                Ok(())
            },
        );
        assert!(result.is_err());
        assert_eq!(checks.get(), 2);
        assert_eq!(
            fs::read(root.join("migration-ran")).unwrap(),
            b"before-pointer-change"
        );
        assert_active_release(&root, &manifest.source.sha);
        assert!(!root.join(".deploy-pending.json").exists());
    }
}
