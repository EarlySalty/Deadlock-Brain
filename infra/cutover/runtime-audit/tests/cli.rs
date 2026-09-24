use std::process::Command;

#[test]
fn missing_command_is_usage_error() {
    let output = Command::new(env!("CARGO_BIN_EXE_brain-runtime-audit"))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn mutating_or_extra_arguments_are_rejected_without_echoing_input() {
    for args in [
        vec!["restart", "NEVER_PRINT_THIS"],
        vec!["snapshot", "--start"],
        vec!["--help", "snapshot"],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_brain-runtime-audit"))
            .args(args)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2));
        assert!(!String::from_utf8_lossy(&output.stderr).contains("NEVER_PRINT_THIS"));
        assert!(output.stdout.is_empty());
    }
}

#[test]
fn help_requires_no_systemd_session() {
    let output = Command::new(env!("CARGO_BIN_EXE_brain-runtime-audit"))
        .arg("--help")
        .env_clear()
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("read-only"));
}
