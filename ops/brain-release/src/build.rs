use crate::fs_safe::{self, Lock};
use crate::source::{self, Artifact, Manifest, OPERATOR};
use anyhow::{ensure, Context, Result};
#[cfg(test)]
use std::fs;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::path::Path;
use std::process::{Command, Stdio};

fn version(program: &str) -> Result<String> {
    let output = source::command(program).arg("--version").output()?;
    ensure!(
        output.status.success(),
        "Werkzeugversion nicht lesbar (Exit {:?})",
        output.status.code()
    );
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

fn build_dir_override(target: &Path) -> Result<String> {
    let path = target.to_str().context("Targetpfad ist kein UTF-8")?;
    ensure!(
        !path.contains(['"', '\\']) && !path.chars().any(char::is_control),
        "Targetpfad ist für die Cargo-Konfiguration ungeeignet"
    );
    Ok(format!("build.build-dir=\"{path}\""))
}

fn run_cargo(source: &Path, target: &Path) -> Result<()> {
    ensure!(
        fs_safe::uid() == OPERATOR,
        "Cargo darf niemals als root laufen"
    );
    fs_safe::new_dir(target)?;
    let status = source::command("/home/nathanael/.cargo/bin/cargo")
        .current_dir(source.join("rust"))
        .env("CARGO_INCREMENTAL", "0")
        .env(
            "RUSTFLAGS",
            format!(
                "--remap-path-prefix={}=/brain-release-target",
                target.display()
            ),
        )
        .args(source::BUILD_ARGS)
        .arg("--target-dir")
        .arg(target)
        .arg("--config")
        .arg(build_dir_override(target)?)
        .stdout(Stdio::inherit())
        .status()?;
    ensure!(
        status.success(),
        "Releasebau fehlgeschlagen (Exit {:?})",
        status.code()
    );
    Ok(())
}

fn copy_artifacts(target: &Path, bundle: &Path, names: Vec<String>) -> Result<Vec<Artifact>> {
    let mut artifacts = Vec::new();
    for name in names {
        let mut input = fs_safe::cargo_artifact(target, &name, OPERATOR)?;
        let before = input.metadata()?;
        let mut header = [0u8; 4];
        input.read_exact(&mut header)?;
        ensure!(header == *b"\x7fELF", "Build lieferte kein ELF-Binary");
        input.seek(SeekFrom::Start(0))?;
        let destination = bundle.join(&name);
        let mut output = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o500)
            .custom_flags(libc::O_NOFOLLOW)
            .open(&destination)?;
        let copied = std::io::copy(&mut input, &mut output)?;
        output.sync_all()?;
        let after = fs_safe::cargo_artifact(target, &name, OPERATOR)?.metadata()?;
        ensure!(
            copied == before.len()
                && after.dev() == before.dev()
                && after.ino() == before.ino()
                && after.len() == before.len()
                && after.ctime() == before.ctime()
                && after.ctime_nsec() == before.ctime_nsec()
                && after.nlink() == before.nlink(),
            "Cargoartefakt während Übernahme verändert"
        );
        eprintln!(
            "Cargoartefakt {name}: nlink={}, übernommene Bytes={copied}",
            before.nlink()
        );
        artifacts.push(Artifact {
            name,
            sha256: fs_safe::file_hash(&destination, OPERATOR)?,
        });
    }
    Ok(artifacts)
}

pub fn compile(source: &Path, bundle: &Path) -> Result<Manifest> {
    ensure!(
        fs_safe::uid() == OPERATOR,
        "Build nur als Betreiber, niemals als root"
    );
    ensure!(
        std::env::var_os("CARGO_TARGET_DIR").is_none(),
        "Globales CARGO_TARGET_DIR ist nicht erlaubt"
    );
    let before = source::inspect(source)?;
    ensure!(
        source::remote_main()? == before.sha,
        "Quelle entspricht nicht dem aktuellen Remote-main"
    );
    fs_safe::checked_path(
        bundle.parent().context("Bundle-Elternpfad fehlt")?,
        OPERATOR,
        true,
    )?;
    ensure!(
        !bundle.starts_with(source) && bundle.is_absolute(),
        "Bundle muss außerhalb der Quelle liegen"
    );
    let _host = loop {
        let mut available = None;
        for slot in 1..=3 {
            let path = format!("/home/nathanael/Documents/.tasks/2026-10-02-offene-branches/locks/build-slot-{slot}.lock");
            if let Some(lock) = Lock::try_acquire(Path::new(&path), OPERATOR)? {
                available = Some(lock);
                break;
            }
        }
        if let Some(lock) = available {
            break lock;
        }
        std::thread::sleep(std::time::Duration::from_secs(10));
    };
    let _cargo = Lock::acquire(Path::new("/tmp/deadlock-cargo-release.lock"), OPERATOR)?;
    fs_safe::new_dir(bundle)?;
    for (program, args) in [
        ("/usr/bin/free", vec!["-m"]),
        (
            "/usr/bin/df",
            vec!["-h", bundle.to_str().context("Bundle-Pfad ist kein UTF-8")?],
        ),
    ] {
        let output = Command::new(program).args(args).output()?;
        eprintln!("{}", String::from_utf8_lossy(&output.stdout));
        ensure!(
            output.status.success(),
            "Hostprüfung fehlgeschlagen (Exit {:?})",
            output.status.code()
        );
    }
    let cargo_version = version("/home/nathanael/.cargo/bin/cargo")?;
    let rustc_version = version("/home/nathanael/.cargo/bin/rustc")?;
    let names = source::inventory(source)?;
    ensure!(
        source::inspect(source)? == before && source::remote_main()? == before.sha,
        "Quelle oder Remote-main vor Build verändert"
    );
    let target = bundle.join("target");
    run_cargo(source, &target)?;
    ensure!(
        source::inspect(source)? == before && source::remote_main()? == before.sha,
        "Quelle oder Remote-main während Build verändert"
    );
    let artifacts = copy_artifacts(&target, bundle, names)?;
    let manifest = Manifest {
        format: 2,
        source: before,
        origin: source::ORIGIN.into(),
        cargo_version,
        rustc_version,
        build_args: source::BUILD_ARGS.iter().map(|x| x.to_string()).collect(),
        artifacts,
    };
    fs_safe::new_file(
        &bundle.join("manifest.json"),
        &serde_json::to_vec_pretty(&manifest)?,
        0o400,
    )?;
    fs_safe::sync_dir(bundle)?;
    source::verify_unchanged(source, bundle, &manifest)?;
    Ok(manifest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::{symlink, PermissionsExt};

    #[test]
    fn fresh_cargo_release_links_and_forged_bundle_are_checked() {
        let dir = tempfile::tempdir_in(concat!(env!("CARGO_MANIFEST_DIR"), "/target")).unwrap();
        fs::set_permissions(dir.path(), fs::Permissions::from_mode(0o700)).unwrap();
        let source = dir.path().join("source");
        fs_safe::new_dir(&source).unwrap();
        fs_safe::new_dir(&source.join("rust")).unwrap();
        fs_safe::new_dir(&source.join("rust/src")).unwrap();
        let mut cargo = String::from(
            "[package]\nname=\"release-probe\"\nversion=\"0.1.0\"\nedition=\"2021\"\n[workspace]\n",
        );
        for name in source::BINS {
            cargo.push_str(&format!(
                "[[bin]]\nname=\"{name}\"\npath=\"src/{name}.rs\"\n"
            ));
            fs_safe::new_file(
                &source.join(format!("rust/src/{name}.rs")),
                b"fn main() { println!(\"real-cargo-release\"); }",
                0o600,
            )
            .unwrap();
        }
        fs_safe::new_file(&source.join("rust/Cargo.toml"), cargo.as_bytes(), 0o600).unwrap();
        fs_safe::new_file(
            &source.join("rust/Cargo.lock"),
            b"version = 4\n[[package]]\nname = \"release-probe\"\nversion = \"0.1.0\"\n",
            0o600,
        )
        .unwrap();
        let target = dir.path().join("target");
        run_cargo(&source, &target).unwrap();
        let bundle = dir.path().join("bundle");
        fs_safe::new_dir(&bundle).unwrap();
        let artifacts = copy_artifacts(
            &target,
            &bundle,
            source::BINS.iter().map(|name| name.to_string()).collect(),
        )
        .unwrap();
        let name = "brain-serve";
        let actual = fs_safe::cargo_artifact(&target, name, OPERATOR).unwrap();
        assert_eq!(actual.metadata().unwrap().nlink(), 2);
        assert_eq!(fs::metadata(bundle.join(name)).unwrap().nlink(), 1);
        let status = Command::new(bundle.join(name)).status().unwrap();
        assert!(status.success());
        fs::set_permissions(bundle.join(name), fs::Permissions::from_mode(0o700)).unwrap();
        fs::write(bundle.join(name), b"\x7fELFforged").unwrap();
        let mut forged = artifacts.clone();
        forged
            .iter_mut()
            .find(|artifact| artifact.name == name)
            .unwrap()
            .sha256 = fs_safe::file_hash(&bundle.join(name), OPERATOR).unwrap();
        assert!(source::compare_artifacts(&forged, &artifacts).is_err());
        fs::hard_link(
            target.join("release").join(name),
            dir.path().join("foreign-alias"),
        )
        .unwrap();
        assert!(fs_safe::cargo_artifact(&target, name, OPERATOR).is_err());
        fs::remove_file(dir.path().join("foreign-alias")).unwrap();
        let binary = target.join("release").join(name);
        fs::remove_file(&binary).unwrap();
        symlink(bundle.join(name), &binary).unwrap();
        assert!(fs_safe::cargo_artifact(&target, name, OPERATOR).is_err());
    }
}
