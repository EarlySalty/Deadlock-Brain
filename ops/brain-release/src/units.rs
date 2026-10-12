use crate::{fs_safe, source};
use anyhow::{ensure, Context, Result};
use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

const UNITS: [(&str, &str, u64); 3] = [
    (
        "brain-serve.service",
        include_str!("../systemd/brain-serve.service"),
        512 * 1024 * 1024,
    ),
    (
        "deadlock-brain-site.service",
        include_str!("../systemd/deadlock-brain-site.service"),
        256 * 1024 * 1024,
    ),
    (
        "brain-browser-consent.service",
        include_str!("../systemd/brain-browser-consent.service"),
        64 * 1024 * 1024,
    ),
];

fn systemctl() -> Command {
    let mut command = source::command("/usr/bin/sudo");
    command.args(["-n", "--", "/usr/bin/systemctl", "--system"]);
    command
}

fn user_systemctl() -> Command {
    let mut command = source::command("/usr/bin/sudo");
    command.args([
        "-n",
        "-u",
        "nathanael",
        "--",
        "/usr/bin/env",
        "XDG_RUNTIME_DIR=/run/user/1000",
        "DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/1000/bus",
        "/usr/bin/systemctl",
        "--user",
    ]);
    command
}

fn checked_status(mut command: Command) -> Result<()> {
    let operation = command
        .get_args()
        .map(|arg| arg.to_string_lossy())
        .collect::<Vec<_>>()
        .join(" ");
    let status = command.status().with_context(|| operation.clone())?;
    ensure!(
        status.success(),
        "Dienstinstallation fehlgeschlagen: {operation} (Exit {:?})",
        status.code()
    );
    Ok(())
}

fn replace(path: &Path, contents: &str) -> Result<()> {
    let parent = path.parent().context("Installationsverzeichnis fehlt")?;
    fs_safe::checked_path(parent, 0, true)?;
    match fs::symlink_metadata(path) {
        Ok(_) => {
            fs_safe::regular(path, 0)?;
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => (),
        Err(error) => return Err(error.into()),
    }
    let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
    temporary.write_all(contents.as_bytes())?;
    temporary
        .as_file()
        .set_permissions(fs::Permissions::from_mode(0o644))?;
    temporary.as_file().sync_all()?;
    temporary.persist(path)?;
    fs_safe::sync_dir(parent)
}

pub fn install() -> Result<()> {
    ensure!(fs_safe::uid() == 0, "Dienstinstallation benötigt root");
    let mut old = Vec::new();
    for (name, _, _) in UNITS {
        let path = Path::new("/home/nathanael/.config/systemd/user").join(name);
        match fs::symlink_metadata(&path) {
            Ok(_) => {
                fs_safe::regular(&path, source::OPERATOR)?;
                old.push(name);
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => (),
            Err(error) => return Err(error.into()),
        }
    }
    if !old.is_empty() {
        let mut command = user_systemctl();
        command.args(["disable", "--now"]).args(old);
        checked_status(command)?;
    }
    for (name, _, _) in UNITS {
        let output = user_systemctl()
            .args(["is-active", "--quiet", name])
            .output()?;
        ensure!(
            matches!(output.status.code(), Some(3 | 4)),
            "Alter Nutzerdienst läuft oder ist nicht prüfbar: {name}"
        );
        let output = user_systemctl().args(["is-enabled", name]).output()?;
        let state = String::from_utf8(output.stdout)?;
        ensure!(
            matches!(state.trim(), "disabled" | "not-found"),
            "Alter Nutzerdienst ist nicht deaktiviert: {name}"
        );
    }
    replace(
        Path::new("/etc/deadlock-brain/infisical-serve.json"),
        include_str!("../infisical-serve.json"),
    )?;
    for (name, template, _) in UNITS {
        replace(&Path::new("/etc/systemd/system").join(name), template)?;
    }
    let mut reload = systemctl();
    reload.arg("daemon-reload");
    checked_status(reload)?;
    let names: Vec<_> = UNITS.iter().map(|(name, _, _)| *name).collect();
    let mut enable = systemctl();
    enable.arg("enable").args(&names);
    checked_status(enable)?;
    let mut restart = systemctl();
    restart.arg("restart").args(&names);
    checked_status(restart)?;
    for (name, _, cap) in UNITS {
        let mut active = systemctl();
        active.args(["is-active", "--quiet", name]);
        checked_status(active)?;
        let cgroup = Path::new("/sys/fs/cgroup/memory/system.slice").join(name);
        let limit: u64 = fs::read_to_string(cgroup.join("memory.limit_in_bytes"))?
            .trim()
            .parse()?;
        let peak: u64 = fs::read_to_string(cgroup.join("memory.max_usage_in_bytes"))?
            .trim()
            .parse()?;
        ensure!(
            limit == cap,
            "Speichergrenze wird nicht durchgesetzt: {name}"
        );
        eprintln!("{name}: memory.limit_in_bytes={limit} memory.max_usage_in_bytes={peak}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_templates_preserve_operator_paths_and_enforced_caps() {
        for (_, template, cap) in UNITS {
            assert!(template.contains("User=nathanael\n"));
            assert!(template.contains("WantedBy=multi-user.target\n"));
            assert!(template.contains("MemoryAccounting=yes\n"));
            assert!(template.contains(&format!("MemoryMax={}M\n", cap / 1024 / 1024)));
            assert!(template.contains("LoadCredential=infisical-token:/home/nathanael/"));
            assert!(template.contains("ExecStart=/opt/deadlock-brain/current/bin/"));
            assert!(!template.contains("%h"));
            assert!(!template.contains("ProtectHome=yes"));
        }
        let config: serde_json::Value =
            serde_json::from_str(include_str!("../infisical-serve.json")).unwrap();
        let path = config["credential_path"].as_str().unwrap();
        assert!(UNITS[0].1.contains(&format!(
            "ExecStartPost=/usr/bin/test {path} = ${{CREDENTIALS_DIRECTORY}}/infisical-token"
        )));
    }
}
