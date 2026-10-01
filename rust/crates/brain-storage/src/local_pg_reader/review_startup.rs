//! Fake Unix peers never touch a real database. Every worker is joined before assertion.
use super::*;
use std::{
    io::{Read, Write},
    os::unix::net::UnixListener,
    sync::mpsc,
};

#[derive(Clone, Copy, Debug)]
enum Stall {
    Authentication,
    Session,
    Query,
}

fn stalled_peer(stage: Stall, request_budget: bool, cancel: bool) {
    let directory = std::env::temp_dir().join(format!(
        "brain-pg-stall-{}-{stage:?}-{request_budget}-{cancel}",
        std::process::id()
    ));
    // Never reuse or remove a pre-existing directory, including after a PID reuse.
    std::fs::create_dir(&directory).unwrap();
    let listener = UnixListener::bind(directory.join(".s.PGSQL.55439")).unwrap();
    let reader = LocalPgReader::new(&directory, 55439, "fixture", "fixture")
        .unwrap()
        .with_pool_options(
            None,
            if cancel {
                Duration::from_secs(2)
            } else {
                Duration::from_millis(100)
            },
            Duration::from_secs(2),
            Duration::from_millis(100),
            1,
            Duration::from_millis(500),
        )
        .unwrap();
    let (entered_tx, entered_rx) = mpsc::channel();
    let (release_tx, release_rx) = mpsc::channel();
    let peer = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        let mut length = [0; 4];
        stream.read_exact(&mut length).unwrap();
        let length = u32::from_be_bytes(length) as usize;
        assert!((8..4096).contains(&length));
        let mut startup = vec![0; length - 4];
        stream.read_exact(&mut startup).unwrap();
        if !matches!(stage, Stall::Authentication) {
            // AuthenticationOk + ReadyForQuery. Then accept the session-setup query.
            stream
                .write_all(b"R\0\0\0\x08\0\0\0\0Z\0\0\0\x05I")
                .unwrap();
            let mut kind = [0; 1];
            stream.read_exact(&mut kind).unwrap();
            assert_eq!(kind[0], b'Q');
            let mut query_length = [0; 4];
            stream.read_exact(&mut query_length).unwrap();
            let query_length = u32::from_be_bytes(query_length) as usize;
            assert!((4..4096).contains(&query_length));
            let mut query = vec![0; query_length - 4];
            stream.read_exact(&mut query).unwrap();
            if matches!(stage, Stall::Query) {
                stream
                    .write_all(b"C\0\0\0\x08SET\0C\0\0\0\x08SET\0Z\0\0\0\x05I")
                    .unwrap();
                // The query uses extended protocol. Prove it actually reached this
                // peer before expiring/cancelling, then deliberately never answer it.
                stream.read_exact(&mut kind).unwrap();
                assert_eq!(kind[0], b'P');
            }
        }
        entered_tx.send(()).unwrap();
        release_rx.recv_timeout(Duration::from_secs(3)).unwrap();
    });
    let deadline = brain_contracts::RequestDeadline::after(if cancel {
        Duration::from_secs(5)
    } else {
        Duration::from_millis(100)
    });
    let worker_deadline = deadline.clone();
    let worker_reader = reader.clone();
    let (finished_tx, finished_rx) = mpsc::channel();
    let worker = std::thread::spawn(move || {
        let result = if request_budget {
            worker_reader
                .pool
                .acquire_until(Some(&worker_deadline))
                .and_then(|mut client| {
                    let result = client
                        .query_one("SELECT 1", &[])
                        .map(|_| ())
                        .map_err(|_| unavailable("fixture query failed"));
                    request_check(Some(&worker_deadline))?;
                    result
                })
        } else {
            worker_reader.check_permissions()
        };
        finished_tx.send(result).unwrap();
    });
    entered_rx.recv_timeout(Duration::from_secs(2)).unwrap();
    if cancel {
        deadline.cancel();
    }
    let completion = finished_rx.recv_timeout(Duration::from_millis(400));
    let stats = reader.pool_stats();
    release_tx.send(()).unwrap();
    peer.join().unwrap();
    worker.join().unwrap();
    std::fs::remove_dir_all(&directory).unwrap();
    assert!(
        completion.is_ok(),
        "{stage:?} outlived its original budget: {stats:?}"
    );
    let result = completion.unwrap();
    assert!(result.is_err());
    if request_budget {
        assert_eq!(result, Err(PortError::BudgetExceeded));
    }
    if cancel {
        assert!(Instant::now() < deadline.expires_at());
    }
    assert_eq!(stats.connecting_connections, 0);
    assert_eq!(stats.checked_out_connections, 0);
    assert_eq!(
        stats.open_connections, 0,
        "timed-out transport must not enter the idle pool"
    );
}
#[test]
fn stalled_postgres_authentication_releases_reserved_capacity() {
    stalled_peer(Stall::Authentication, false, false);
}
#[test]
fn stalled_session_initialization_obeys_the_request_deadline() {
    stalled_peer(Stall::Session, true, false);
}
#[test]
fn stalled_query_obeys_the_request_deadline() {
    stalled_peer(Stall::Query, true, false);
}
#[test]
fn cancelled_startup_closes_the_transport_and_returns_capacity() {
    stalled_peer(Stall::Authentication, true, true);
}
#[test]
fn cancelled_query_closes_the_transport_and_returns_capacity() {
    stalled_peer(Stall::Query, true, true);
}
