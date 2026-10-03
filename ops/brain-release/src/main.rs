mod fs_safe;
mod install;
mod source;

use anyhow::{bail, ensure, Context, Result};
use fs_safe::{uid, Lock};
use source::{Artifact, Manifest, OPERATOR};
use std::fs;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;

const TRUSTED_HELPER: &str = "/usr/local/libexec/brain-release";

fn compiler_busy() -> Result<bool> {
    let mut busy = false;
    for entry in fs::read_dir("/proc")? {
        let entry = entry?;
        if entry.file_name().to_string_lossy().parse::<u32>().is_err() {
            continue;
        }
        let stat = match fs::read_to_string(entry.path().join("stat")) {
            Ok(value) => value,
            Err(_) => continue,
        };
        if stat
            .rsplit_once(") ")
            .is_some_and(|(_, rest)| rest.starts_with('Z'))
        {
            continue;
        }
        let comm = match fs::read_to_string(entry.path().join("comm")) {
            Ok(value) => value,
            Err(_) => continue,
        };
        if ["cargo", "rustc", "clippy-driver", "rustdoc"].contains(&comm.trim()) {
            if comm.trim() == "cargo" {
                if let Ok(args) = fs::read(entry.path().join("cmdline")) {
                    let parts = args
                        .split(|b| *b == 0)
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<_>>();
                    if parts.len() == 7
                        && parts[1..6]
                            == [
                                b"metadata".as_slice(),
                                b"--format-version",
                                b"1",
                                b"--no-deps",
                                b"--manifest-path",
                            ]
                        && std::str::from_utf8(parts[6]).ok().is_some_and(|p| {
                            Path::new(p).is_absolute()
                                && Path::new(p).file_name().is_some_and(|x| x == "Cargo.toml")
                        })
                        && fs::read_to_string(entry.path().join("stat"))
                            .ok()
                            .as_deref()
                            == Some(stat.as_str())
                    {
                        continue;
                    }
                }
            }
            eprintln!(
                "Compilerprobe: PID {}, {} blockiert",
                entry.file_name().to_string_lossy(),
                comm.trim()
            );
            busy = true;
        }
    }
    Ok(busy)
}

fn version(program: &str) -> Result<String> {
    let output = source::command(program).arg("--version").output()?;
    ensure!(output.status.success(), "Werkzeugversion nicht lesbar");
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

fn build(source: &Path, bundle: &Path) -> Result<()> {
    ensure!(
        uid() == OPERATOR,
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
    fs_safe::new_dir(bundle)?;
    let _host = Lock::acquire(
        Path::new(
            "/home/nathanael/Documents/.tasks/2026-10-02-offene-branches/locks/host-checks.lock",
        ),
        OPERATOR,
    )?;
    let _cargo = Lock::acquire(Path::new("/tmp/deadlock-cargo-release.lock"), OPERATOR)?;
    while compiler_busy()? {
        std::thread::sleep(std::time::Duration::from_secs(30));
    }
    for (program, args) in [
        ("/usr/bin/free", vec!["-m"]),
        (
            "/usr/bin/df",
            vec!["-h", bundle.to_str().context("Bundle-Pfad ist kein UTF-8")?],
        ),
    ] {
        ensure!(
            Command::new(program).args(args).status()?.success(),
            "Hostprüfung fehlgeschlagen"
        );
    }
    let cargo_version = version("/home/nathanael/.cargo/bin/cargo")?;
    let rustc_version = version("/home/nathanael/.cargo/bin/rustc")?;
    ensure!(
        source::inspect(source)? == before && source::remote_main()? == before.sha,
        "Quelle oder Remote-main vor Build verändert"
    );
    while compiler_busy()? {
        std::thread::sleep(std::time::Duration::from_secs(30));
    }
    let target = bundle.join("target");
    let status = source::command("/home/nathanael/.cargo/bin/cargo")
        .current_dir(source.join("rust"))
        .args(source::BUILD_ARGS)
        .arg("--target-dir")
        .arg(&target)
        .status()?;
    ensure!(
        status.success(),
        "Releasebau fehlgeschlagen (Exit {:?})",
        status.code()
    );
    ensure!(
        source::inspect(source)? == before && source::remote_main()? == before.sha,
        "Quelle oder Remote-main während Build verändert"
    );
    let binary_names = source::inventory(source)?;
    let mut artifacts = Vec::new();
    for name in binary_names {
        let input = target.join("release").join(&name);
        let data = fs_safe::bytes(&input, OPERATOR, 2 * 1024 * 1024 * 1024)?;
        ensure!(
            data.starts_with(b"\x7fELF"),
            "Build lieferte kein ELF-Binary"
        );
        fs_safe::new_file(&bundle.join(&name), &data, 0o500)?;
        artifacts.push(Artifact {
            name,
            sha256: fs_safe::hash(&data),
        });
    }
    let manifest = Manifest {
        format: 1,
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
    print_manifest(&source::verify(source, bundle)?)
}

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
        ("build", 3) => build(Path::new(&args[1]), Path::new(&args[2])),
        ("verify", 3) => print_manifest(&source::verify(Path::new(&args[1]), Path::new(&args[2]))?),
        ("install", 3) => {
            privileged()?;
            let source = Path::new(&args[1]);
            let bundle = Path::new(&args[2]);
            let installer = install::Installer::open(Path::new(install::ROOT), 0)?;
            let manifest = operator_verify(source, bundle)?;
            installer.stage(bundle, &manifest)?;
            ensure!(operator_verify(source, bundle)? == manifest, "Herkunft vor Zeigerwechsel verändert");
            installer.activate(&manifest.source.sha)?;
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
