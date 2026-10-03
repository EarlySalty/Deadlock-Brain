mod build;
mod fs_safe;
mod install;
mod source;

use anyhow::{bail, ensure, Context, Result};
use fs_safe::uid;
use source::{Manifest, OPERATOR};
use std::fs;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};

const TRUSTED_HELPER: &str = "/usr/local/libexec/brain-release";

fn print_manifest(manifest: &Manifest) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(manifest)?);
    Ok(())
}

fn privileged() -> Result<()> {
    ensure!(
        uid() == 0,
        "Installation benötigt den separat eingerichteten, eng begrenzten Root-Helfer"
    );
    let exe = std::env::current_exe()?;
    ensure!(
        exe == Path::new(TRUSTED_HELPER),
        "Root-Ausführung nur aus dem fest installierten Helfer"
    );
    fs_safe::checked_path(&exe, 0, false)?;
    let file = fs_safe::regular(&exe, 0)?;
    use std::os::unix::fs::MetadataExt;
    ensure!(
        file.metadata()?.mode() & 0o6000 == 0,
        "Kein setuid/setgid-Helfer erlaubt"
    );
    Ok(())
}

fn operator_verify(source: &Path, bundle: &Path) -> Result<Manifest> {
    privileged()?;
    let mut command = source::command(TRUSTED_HELPER);
    command.args(["verify"]).arg(source).arg(bundle);
    command.stderr(std::process::Stdio::inherit());
    unsafe {
        command.pre_exec(|| {
            if libc::setgroups(0, std::ptr::null()) != 0
                || libc::setgid(OPERATOR) != 0
                || libc::setuid(OPERATOR) != 0
            {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    let output = command.output()?;
    ensure!(
        output.status.success() && output.stdout.len() <= 65536,
        "Unprivilegierte Herkunftsprüfung fehlgeschlagen (Exit {:?})",
        output.status.code()
    );
    let manifest: Manifest = serde_json::from_slice(&output.stdout)?;
    source::validate_manifest(&manifest)?;
    Ok(manifest)
}

fn main_result() -> Result<()> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    let verb = args.first().and_then(|s| s.to_str()).unwrap_or("");
    match (verb, args.len()) {
        ("plan", 2) => {
            ensure!(uid() == OPERATOR, "Plan nur unprivilegiert ausführen");
            let source = PathBuf::from(&args[1]);
            let remote = source::remote_main()?;
            println!("Remote-main: {remote}");
            for name in ["current", "maintenance-current"] {
                let path = Path::new(install::ROOT).join(name);
                let meta = fs::symlink_metadata(&path)?;
                use std::os::unix::fs::MetadataExt;
                println!("{name}: {} (UID {}, Symlink {})", fs::read_link(&path)?.display(), meta.uid(), meta.file_type().is_symlink());
            }
            let state = source::inspect(&source)?;
            ensure!(state.sha == remote, "Quelle liegt nicht auf Remote-main");
            println!("{}", serde_json::to_string_pretty(&state)?);
            println!("Buildargv: /home/nathanael/.cargo/bin/cargo {} --target-dir <neues Bundle>/target", source::BUILD_ARGS.join(" "));
            Ok(())
        }
        ("build", 3) => print_manifest(&build::compile(Path::new(&args[1]), Path::new(&args[2]))?),
        ("verify", 3) => print_manifest(&source::verify(Path::new(&args[1]), Path::new(&args[2]))?),
        ("install", 3) => {
            privileged()?;
            let source = Path::new(&args[1]);
            let bundle = Path::new(&args[2]);
            let installer = install::Installer::open(Path::new(install::ROOT), 0)?;
            let manifest = operator_verify(source, bundle)?;
            installer.stage(bundle, &manifest)?;
            installer.activate_checked(&manifest.source.sha, || {
                ensure!(operator_verify(source, bundle)? == manifest, "Herkunft unmittelbar vor Zeigerwechsel verändert");
                Ok(())
            })?;
            print_manifest(&installer.installed(&manifest.source.sha)?)
        }
        ("rollback", 2) => {
            privileged()?;
            let sha = args[1].to_str().context("Ungültiger SHA")?;
            let installer = install::Installer::open(Path::new(install::ROOT), 0)?;
            installer.activate(sha)?;
            print_manifest(&installer.installed(sha)?)
        }
        ("recover", 1) => {
            privileged()?;
            let installer = install::Installer::open(Path::new(install::ROOT), 0)?;
            installer.recover()?;
            println!("Abgebrochene Installation zurückgenommen. Dienste wurden nicht neu gestartet.");
            Ok(())
        }
        _ => bail!("Aufrufe: plan QUELLE | build QUELLE NEUES_BUNDLE | verify QUELLE BUNDLE | install QUELLE BUNDLE | rollback SHA | recover"),
    }
}

fn main() {
    if let Err(error) = main_result() {
        eprintln!("STOPP: {error:#}");
        std::process::exit(1);
    }
}
