mod common;
use common::{config, credentials, Service};
use serde_json::json;
use std::{
    process::Command,
    time::{Duration, Instant},
};

#[test]
fn missing_config_and_bad_arguments_exit_without_echoing_input() {
    for args in [
        vec![],
        vec!["--config", "/missing/DO-NOT-LOG-config"],
        vec!["--unexpected-DO-NOT-LOG"],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_brain-serve"))
            .env_clear()
            .args(&args)
            .output()
            .unwrap();
        assert!(!output.status.success());
        let log = String::from_utf8(output.stderr).unwrap();
        assert!(!log.contains("DO-NOT-LOG"));
        assert!(!log.contains("listening"));
    }
}

#[test]
fn help_does_not_require_configuration_or_credentials() {
    let output = Command::new(env!("CARGO_BIN_EXE_brain-serve"))
        .env_clear()
        .arg("--help")
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(String::from_utf8(output.stdout)
        .unwrap()
        .contains("BRAIN_SERVE_CONFIG"));
}

#[test]
fn incomplete_configuration_and_inline_secrets_never_reach_startup() {
    // Build the synthetic secret at runtime so secret scanners do not mistake a
    // committed test sentinel for a real credential. The test still verifies
    // that rejected inline secret values never appear in service logs.
    let inline_secret = ["redaction", "sentinel", "value"].join("-");
    let inline_config = json!({"password": inline_secret.clone()});

    for value in [json!({}), inline_config] {
        let mut child = Service::spawn(&value, &credentials());
        assert!(!child.wait(Duration::from_secs(3)).success());
        assert!(child.log().contains("config_schema_invalid"));
        assert!(!child.log().contains(&inline_secret));
        assert!(!child.log().contains("startup_checks"));
    }
}

#[test]
fn each_required_secret_is_checked_before_database_or_bind() {
    let mut value = config();
    value["postgres"]["auth"] = json!("password");
    value["postgres"]["password_env"] = json!("BRAIN_SERVE_PG_PASSWORD");
    let mut env = credentials();
    env.push(("BRAIN_SERVE_PG_PASSWORD", common::PG_PASSWORD));
    for missing in [
        "BRAIN_SERVE_API_TOKEN",
        "BRAIN_SERVE_PROVIDER_API_KEY",
        "BRAIN_SERVE_PG_PASSWORD",
    ] {
        let remaining: Vec<_> = env
            .iter()
            .copied()
            .filter(|(key, _)| *key != missing)
            .collect();
        let mut child = Service::spawn(&value, &remaining);
        assert!(!child.wait(Duration::from_secs(3)).success());
        assert!(child.log().contains("required_secret_missing"));
        assert!(!child.log().contains("startup_checks"));
        for (_, secret) in &env {
            assert!(!child.log().contains(secret));
        }
    }
}

#[test]
fn unavailable_database_is_bounded_and_ignores_legacy_and_ambient_dsns() {
    let dir = tempfile::tempdir().unwrap();
    let trap = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    trap.set_nonblocking(true).unwrap();
    let legacy = format!("http://{}", trap.local_addr().unwrap());
    let mut value = config();
    value["postgres"]["socket_dir"] = json!(dir.path().join("nonexistent"));
    value["postgres"]["auth"] = json!("password");
    value["postgres"]["password_env"] = json!("BRAIN_SERVE_PG_PASSWORD");
    value["timeouts"]["postgres_connect_ms"] = json!(300);
    value["timeouts"]["startup_ms"] = json!(1500);
    value["timeouts"]["readiness_ms"] = json!(500);
    let mut env = credentials();
    env.extend([
        ("BRAIN_SERVE_PG_PASSWORD", common::PG_PASSWORD),
        ("DATABASE_URL", "postgres://DO-NOT-LOG@127.0.0.1:1/unwanted"),
        ("PGHOST", "127.0.0.1"),
        ("PGPASSWORD", "DO-NOT-LOG-ambient-password"),
        ("BRAIN_LEGACY_URL", legacy.as_str()),
    ]);
    let started = Instant::now();
    let mut child = Service::spawn(&value, &env);
    assert!(!child.wait(Duration::from_secs(5)).success());
    assert!(started.elapsed() < Duration::from_secs(5));
    assert!(child.log().contains("database_unavailable"));
    assert!(!child.log().contains("listening"));
    for (_, secret) in &env {
        assert!(!child.log().contains(secret));
    }
    assert!(
        trap.accept().is_err(),
        "no legacy or ambient TCP connection allowed"
    );
}

#[test]
fn sigterm_during_database_startup_exits_cleanly_without_binding() {
    let dir = tempfile::tempdir().unwrap();
    let _unresponsive =
        std::os::unix::net::UnixListener::bind(dir.path().join(".s.PGSQL.55439")).unwrap();
    let mut value = config();
    value["postgres"]["socket_dir"] = json!(dir.path());
    value["postgres"]["port"] = json!(55439);
    let mut child = Service::spawn(&value, &credentials());
    let deadline = Instant::now() + Duration::from_secs(5);
    while !child.log().contains("startup_checks") {
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(10));
    }
    child.signal("TERM");
    assert!(
        child.wait(Duration::from_secs(3)).success(),
        "{}",
        child.log()
    );
    assert!(child.log().contains("stopped"));
    assert!(!child.log().contains("listening"));
}
