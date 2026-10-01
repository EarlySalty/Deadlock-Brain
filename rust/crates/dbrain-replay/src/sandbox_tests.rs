//! Test-only fault injection, never linked into the production worker.
use super::*;

fn spawn_probe(mode: &str, output: File) -> ChildGuard {
    let child = Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "supervisor::tests::sandbox_probe",
            "--ignored",
            "--nocapture",
        ])
        .env_clear()
        .env("DBRAIN_REPLAY_TEST_PROBE", mode)
        .stdin(Stdio::null())
        .stdout(Stdio::from(output))
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    ChildGuard {
        child,
        reaped: false,
    }
}
#[test]
#[ignore = "subprocess fixture: exercised by the four sandbox/watchdog tests, not a skipped validation"]
fn sandbox_probe() {
    let mode = std::env::var("DBRAIN_REPLAY_TEST_PROBE").expect("test-only invocation");
    if mode == "sleep" {
        std::thread::sleep(Duration::from_secs(60));
        std::process::exit(7);
    }
    let budget = DecodeBudget {
        cpu_seconds: 1,
        memory_bytes: 512 * 1024 * 1024,
        ..Default::default()
    };
    sandbox(&budget).unwrap();
    match mode.as_str() {
        "syscalls" => {
            assert_eq!(
                File::open("/dev/null").unwrap_err().raw_os_error(),
                Some(libc::EPERM)
            );
            assert_eq!(
                std::net::TcpListener::bind("127.0.0.1:0")
                    .unwrap_err()
                    .raw_os_error(),
                Some(libc::EPERM)
            );
            assert!(Command::new("/bin/true").status().is_err());
            std::process::exit(0);
        }
        "memory" => {
            let mut bytes = Vec::<u8>::new();
            // try_reserve returns before committing memory because RLIMIT_AS is smaller.
            assert!(bytes.try_reserve_exact(1024 * 1024 * 1024).is_err());
            std::process::exit(0);
        }
        "cpu" => {
            let mut n = 0u64;
            loop {
                n = n.wrapping_add(1);
                std::hint::black_box(n);
            }
        }
        _ => std::process::exit(8),
    }
}
fn run_probe(mode: &str) -> std::process::ExitStatus {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("out");
    let mut child = spawn_probe(mode, private_file(&path).unwrap());
    wait_for_worker(
        &mut child,
        Instant::now() + Duration::from_secs(10),
        &path,
        1_000_000,
    )
    .unwrap()
}
#[test]
fn sandbox_denies_filesystem_network_and_new_processes() {
    assert!(run_probe("syscalls").success());
}
#[test]
fn address_space_budget_rejects_oversized_allocation() {
    assert!(run_probe("memory").success());
}
#[test]
fn cpu_budget_terminates_a_noncooperative_worker() {
    let started = Instant::now();
    let status = run_probe("cpu");
    assert!(matches!(
        status.signal(),
        Some(libc::SIGKILL | libc::SIGXCPU)
    ));
    assert!(started.elapsed() < Duration::from_secs(10));
}
#[test]
fn wall_watchdog_kills_and_reaps_a_stuck_worker() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("out");
    let mut child = spawn_probe("sleep", private_file(&path).unwrap());
    let pid = child.child.id();
    assert_eq!(
        wait_for_worker(
            &mut child,
            Instant::now() + Duration::from_millis(100),
            &path,
            1_000_000
        ),
        Err(ReplayFailure::WorkerTimeout)
    );
    drop(child);
    assert!(
        !Path::new(&format!("/proc/{pid}")).exists(),
        "worker must be reaped, not left a zombie"
    );
}
