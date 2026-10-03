use std::process::Command;

fn rejected_peer(socket: &str, database: &str) {
    let output = Command::new(env!("CARGO_BIN_EXE_dbrain-replay-import"))
        .args([
            "query",
            "unselected-release",
            "test-only",
            "--peer-test",
            socket,
            "55439",
            database,
        ])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .ends_with("InvalidRequest\n")
    );
}

#[test]
fn peer_test_connection_rejects_production_and_relative_locations() {
    rejected_peer("/tmp", "brain");
    rejected_peer("relative-socket", "brain_test");
    rejected_peer("/proc", "brain_test");
    rejected_peer("/tmp/../proc", "brain_test");
}

#[cfg(unix)]
#[test]
fn peer_test_connection_rejects_symlinks_outside_the_test_directory() {
    let dir = tempfile::tempdir().unwrap();
    let socket = dir.path().join("socket");
    std::os::unix::fs::symlink("/proc", &socket).unwrap();
    rejected_peer(socket.to_str().unwrap(), "brain_test");
}
