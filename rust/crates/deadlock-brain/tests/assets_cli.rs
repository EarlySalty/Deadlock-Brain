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
