//! Exercise the real pool queue with its only slot already reserved. No DB is contacted.
use super::*;
use brain_contracts::RequestDeadline;

fn saturated_reader(socket: &Path) -> LocalPgReader {
    let reader = LocalPgReader::new(socket, 5446, "fixture", "fixture")
        .unwrap()
        .with_pool_options(
            None,
            Duration::from_secs(1),
            Duration::from_secs(1),
            Duration::from_millis(100),
            1,
            Duration::from_millis(700),
        )
        .unwrap();
    // An in-progress connection occupies the hard limit; acquire must wait, not connect.
    reader.pool.state.lock().unwrap().connecting = 1;
    reader
}

#[test]
fn pool_wait_is_paid_from_the_original_request_deadline() {
    let root = std::env::temp_dir().join("brain-pool-capacity-fixture-no-server");
    let reader = saturated_reader(&root);
    let deadline = RequestDeadline::after(Duration::from_millis(100));
    let started = Instant::now();
    let result = reader.claim_conversation_until("conversation", "actor", &deadline);
    assert_eq!(result, Err(PortError::BudgetExceeded));
    assert!(
        started.elapsed() < Duration::from_millis(400),
        "pool restarted its own 700ms wait: {:?}",
        started.elapsed()
    );
    let state = reader.pool.state.lock().unwrap();
    assert!(
        state.waiting.may_acquire(None),
        "expired waiter must leave the FIFO queue"
    );
    assert_eq!(state.created, 0);
    assert_eq!(state.connecting, 1);
}

#[test]
fn cancelling_a_queued_pool_request_does_not_wait_for_pool_timeout() {
    let root = std::env::temp_dir().join("brain-pool-capacity-fixture-no-server");
    let reader = saturated_reader(&root);
    let deadline = RequestDeadline::after(Duration::from_secs(5));
    let worker_reader = reader.clone();
    let worker_deadline = deadline.clone();
    let worker = std::thread::spawn(move || {
        worker_reader.claim_conversation_until("conversation", "actor", &worker_deadline)
    });
    let started = Instant::now();
    while reader.pool_stats().wait_count == 0 {
        assert!(started.elapsed() < Duration::from_secs(1));
        std::thread::yield_now();
    }
    let cancelled = Instant::now();
    deadline.cancel();
    assert_eq!(worker.join().unwrap(), Err(PortError::BudgetExceeded));
    assert!(
        cancelled.elapsed() < Duration::from_millis(300),
        "cancelled waiter remained queued"
    );
    let state = reader.pool.state.lock().unwrap();
    assert!(state.waiting.may_acquire(None));
    assert_eq!(state.created, 0);
    assert_eq!(state.connecting, 1);
}
