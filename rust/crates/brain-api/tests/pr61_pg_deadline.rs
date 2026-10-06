//! PR #61: a stalled local PostgreSQL startup must not pin bounded pool capacity.
//! Uses only an owned temporary Unix socket, never a real database.
#[cfg(unix)]
#[test]
fn postgres_startup_handshake_must_obey_connect_deadline() {
    use brain_contracts::{DocumentRevision, SnapshotReadPort};
    use brain_storage::LocalPgReader;
    use std::{io::Read, os::unix::net::UnixListener, sync::mpsc, thread, time::Duration};

    let directory = tempfile::tempdir().unwrap();
    let listener = UnixListener::bind(directory.path().join(".s.PGSQL.55439")).unwrap();
    let (accepted_tx, accepted_rx) = mpsc::channel();
    let (release_tx, release_rx) = mpsc::channel();
    let server = thread::spawn(move || {
        let (mut socket, _) = listener.accept().unwrap();
        socket.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
        let mut prefix = [0; 8];
        socket.read_exact(&mut prefix).unwrap();
        accepted_tx.send(()).unwrap();
        // Bounded fixture lifetime; drop the socket even if the test fails.
        let _ = release_rx.recv_timeout(Duration::from_secs(2));
    });
    let reader = LocalPgReader::new(directory.path(), 55439, "review", "review").unwrap()
        .with_pool_options(None, Duration::from_millis(50), Duration::from_millis(50),
            Duration::from_millis(25), 1, Duration::from_millis(50)).unwrap();
    let observer = reader.clone();
    let (done_tx, done_rx) = mpsc::channel();
    let worker = thread::spawn(move || {
        let result = reader.read_heads(&[DocumentRevision { source_id: "review".into(),
            logical_id: "review".into(), revision: 1, content_hash: "review".into() }]);
        done_tx.send(result).unwrap();
    });
    accepted_rx.recv_timeout(Duration::from_secs(2)).unwrap();
    let result = done_rx.recv_timeout(Duration::from_millis(300));
    let stats = observer.pool_stats();
    eprintln!("PG_STARTUP_DEADLINE: completed_within_300ms={}, configured_connect_ms=50, connecting={}, max_connections={}",
        result.is_ok(), stats.connecting_connections, stats.max_connections);
    let _ = release_tx.send(());
    server.join().unwrap();
    worker.join().unwrap();
    assert!(result.is_ok(), "a 50ms connection deadline did not cover PostgreSQL startup/authentication; the sole pool slot remains reserved after 300ms");
    assert!(result.unwrap().is_err());
}
