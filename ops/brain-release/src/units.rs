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

const JOB_TEMPLATES: [(&str, &str); 2] = [
    (
        "brain-job@.service",
        include_str!("../../brain-job@.service"),
    ),
    ("brain-job@.timer", include_str!("../../brain-job@.timer")),
];

fn install_templates(mut write: impl FnMut(&Path, &str) -> Result<()>) -> Result<()> {
    for (name, template) in UNITS
        .iter()
        .map(|(name, template, _)| (*name, *template))
        .chain(JOB_TEMPLATES)
    {
        write(&Path::new("/etc/systemd/system").join(name), template)?;
    }
    Ok(())
}

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

fn disabled_or_absent(state: &str, exit: Option<i32>) -> bool {
    matches!(exit, Some(1 | 4))
        && (matches!(state.trim(), "disabled" | "not-found")
            || (state.trim().is_empty() && exit == Some(4)))
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
            disabled_or_absent(&state, output.status.code()),
            "Alter Nutzerdienst ist nicht deaktiviert: {name}"
        );
    }
    replace(
        Path::new("/etc/deadlock-brain/infisical-serve.json"),
        include_str!("../infisical-serve.json"),
    )?;
    install_templates(replace)?;
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
    fn absent_user_units_accept_the_documented_exit_without_accepting_query_failures() {
        assert!(disabled_or_absent("disabled\n", Some(1)));
        assert!(disabled_or_absent("not-found\n", Some(4)));
        assert!(disabled_or_absent("", Some(4)));
        assert!(!disabled_or_absent("", Some(1)));
        assert!(!disabled_or_absent("enabled", Some(0)));
        assert!(!disabled_or_absent("", None));
        assert!(!disabled_or_absent("disabled", None));
        assert!(!disabled_or_absent("not-found", None));
        assert!(!disabled_or_absent("disabled", Some(127)));
    }

    #[test]
    fn installation_writes_both_job_templates_without_adding_active_units() {
        let mut installed = Vec::new();
        install_templates(|path, template| {
            installed.push((path.to_path_buf(), template.to_string()));
            Ok(())
        })
        .unwrap();
        assert_eq!(
            installed
                .iter()
                .map(|(path, _)| path.to_str().unwrap())
                .collect::<Vec<_>>(),
            [
                "/etc/systemd/system/brain-serve.service",
                "/etc/systemd/system/deadlock-brain-site.service",
                "/etc/systemd/system/brain-browser-consent.service",
                "/etc/systemd/system/brain-job@.service",
                "/etc/systemd/system/brain-job@.timer",
            ]
        );
        assert_eq!(
            UNITS.map(|(name, _, _)| name),
            [
                "brain-serve.service",
                "deadlock-brain-site.service",
                "brain-browser-consent.service",
            ]
        );
        assert_eq!(installed[3].1, include_str!("../../brain-job@.service"));
        assert_eq!(installed[4].1, include_str!("../../brain-job@.timer"));
        assert!(installed[3].1.contains("MemoryMax=512M\n"));
        assert!(installed[4].1.contains("Unit=brain-job@%i.service\n"));
        let mut attempted = Vec::new();
        assert!(install_templates(|path, _| {
            attempted.push(path.to_path_buf());
            if path.file_name().unwrap() == "brain-job@.service" {
                anyhow::bail!("Schreibfehler")
            }
            Ok(())
        })
        .is_err());
        assert_eq!(attempted.len(), 4);
    }

    #[test]
    fn job_stop_hook_reports_only_numeric_v1_peak_for_the_unit_cgroup() {
        let hooks: Vec<_> = JOB_TEMPLATES[0]
            .1
            .lines()
            .filter_map(|line| line.strip_prefix("ExecStopPost="))
            .collect();
        assert_eq!(hooks.len(), 1);
        let command: Vec<_> = hooks[0].split_whitespace().collect();
        assert_eq!(
            command,
            [
                "/usr/bin/grep",
                "--with-filename",
                "--line-regexp",
                "--extended-regexp",
                "[0-9]+",
                "/sys/fs/cgroup/memory/system.slice/%n/memory.max_usage_in_bytes",
            ]
        );
        let dir = tempfile::tempdir().unwrap();
        let metric = dir.path().join("memory.max_usage_in_bytes");
        fs::write(&metric, b"123456\nprivate=sentinel\n1234 trailing\n0\n").unwrap();
        let output = source::command(command[0])
            .args(&command[1..5])
            .arg(&metric)
            .output()
            .unwrap();
        assert!(output.status.success());
        assert!(output.stderr.is_empty());
        assert_eq!(
            String::from_utf8(output.stdout).unwrap(),
            format!("{0}:123456\n{0}:0\n", metric.display())
        );
        fs::write(&metric, b"private=sentinel\n").unwrap();
        let output = source::command(command[0])
            .args(&command[1..5])
            .arg(&metric)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(1));
        assert!(output.stdout.is_empty());
    }

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
