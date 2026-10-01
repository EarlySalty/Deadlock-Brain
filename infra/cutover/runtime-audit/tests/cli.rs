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

#[test]
fn runtime_command_is_documented_as_point_in_time_only() {
    let output = Command::new(env!("CARGO_BIN_EXE_brain-runtime-audit"))
        .arg("--help")
        .env_clear()
        .output()
        .unwrap();
    let help = String::from_utf8_lossy(&output.stdout);
    assert!(help.contains("runtime"));
    assert!(help.contains("point-in-time"));
}

#[test]
fn runtime_without_systemd_session_fails_without_a_partial_report() {
    let output = Command::new(env!("CARGO_BIN_EXE_brain-runtime-audit"))
        .arg("runtime")
        .env_clear()
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(error.contains("cutover_authorized=false"));
    assert!(!error.contains("Failed to connect"));
}
