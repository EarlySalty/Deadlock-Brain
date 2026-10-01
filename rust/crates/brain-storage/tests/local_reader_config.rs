use brain_storage::LocalPgReader;
use std::time::Duration;

#[test]
fn explicit_reader_options_retain_socket_boundary_and_redact_passwords() {
    let reader = LocalPgReader::new("/tmp/c1-test-socket", 55439, "fixture", "fixture")
        .unwrap()
        .with_connection_options(
            Some("synthetic-private-reader-password".into()),
            Duration::from_millis(500),
            Duration::from_millis(700),
            Duration::from_millis(200),
        )
        .unwrap();
    let debug = format!("{reader:?}");
    assert!(!debug.contains("synthetic-private"));
    assert!(debug.contains("local Unix socket"));
    assert!(LocalPgReader::new("127.0.0.1", 5432, "fixture", "fixture").is_err());
}

#[test]
fn invalid_reader_timeouts_fail_before_connecting() {
    for (connect, statement, lock) in [(0, 100, 100), (60001, 100, 100), (100, 100, 101)] {
        assert!(
            LocalPgReader::new("/tmp/c1-test-socket", 55439, "fixture", "fixture")
                .unwrap()
                .with_connection_options(
                    None,
                    Duration::from_millis(connect),
                    Duration::from_millis(statement),
                    Duration::from_millis(lock)
                )
                .is_err()
        );
    }
}
