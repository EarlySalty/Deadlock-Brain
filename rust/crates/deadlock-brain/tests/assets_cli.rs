use std::{fs, process::Command};

#[test]
fn assets_cli_exposes_the_durable_default() {
    let output = Command::new(env!("CARGO_BIN_EXE_deadlock-brain"))
        .env_clear()
        .args(["pull", "assets", "--help"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(String::from_utf8(output.stdout)
        .unwrap()
        .contains("/home/nathanael/.local/share/deadlock-brain"));
}

#[test]
fn assets_cli_uses_configured_directories_and_keeps_explicit_cli_priority() {
    for explicit in [false, true] {
        let directory = tempfile::tempdir().unwrap();
        let configured = directory.path().join("configured");
        let cli_path = directory.path().join("explicit");
        let mut command = Command::new(env!("CARGO_BIN_EXE_deadlock-brain"));
        command
            .env_clear()
            .env("DEADLOCK_BRAIN_DATA_DIR", &configured)
            .current_dir(directory.path())
            .args(["pull", "assets"]);
        if explicit {
            command.arg("--data-dir").arg(&cli_path);
        }
        let output = command.output().unwrap();
        assert_eq!(output.status.code(), Some(1));
        assert!(String::from_utf8_lossy(&output.stderr).contains("DEADLOCK_CENTRAL_DSN"));
        assert!(output.stdout.is_empty());
        let selected = if explicit { &cli_path } else { &configured };
        assert!(selected.join("raw").is_dir());
        assert!(selected.join("cache").is_dir());
        assert_eq!(configured.exists(), !explicit);
        assert_eq!(cli_path.exists(), explicit);
    }
}

#[test]
fn assets_cli_rejects_relative_and_empty_directories_without_writes() {
    for (data_dir, exit_code) in [("relative-assets", 1), ("", 2)] {
        let directory = tempfile::tempdir().unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_deadlock-brain"))
            .env_clear()
            .current_dir(directory.path())
            .args(["pull", "assets", "--data-dir", data_dir])
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(exit_code));
        assert!(output.stdout.is_empty());
        assert_eq!(fs::read_dir(directory.path()).unwrap().count(), 0);
    }
}
