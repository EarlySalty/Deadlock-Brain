mod common;
use common::{config, credentials, Service};
use serde_json::json;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
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
    let help = String::from_utf8(output.stdout).unwrap();
    assert!(help.contains("--config"));
    assert!(help.contains("[brain.infisical]"));
    assert!(!help.contains("BRAIN_SERVE_CONFIG"));
}

#[test]
fn ambient_config_is_not_a_startup_source() {
    let output = Command::new(env!("CARGO_BIN_EXE_brain-serve"))
        .env_clear()
        .env("BRAIN_SERVE_CONFIG", "/ambient/DO-NOT-LOG-config.json")
        .output()
        .unwrap();
    assert!(!output.status.success());
    let log = String::from_utf8(output.stderr).unwrap();
    assert!(log.contains("config_missing"));
    assert!(!log.contains("DO-NOT-LOG"));
}

#[test]
fn unavailable_explicit_infisical_source_is_redacted_and_never_reaches_startup() {
    let directory = tempfile::tempdir().unwrap();
    let config_path = directory.path().join("bot.toml");
    std::fs::write(
        &config_path,
        toml::to_string(&json!({"brain":{"serve":config()}})).unwrap(),
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_brain-serve"))
        .env_clear()
        .args(["--config", config_path.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let log = String::from_utf8(output.stderr).unwrap();
    assert!(log.contains("secret_source_unavailable"));
    assert!(!log.contains("DO-NOT-LOG"));
    assert!(!log.contains("startup_checks"));
}

fn short_startup_config(budget_ms: u64) -> serde_json::Value {
    let mut value = config();
    value["timeouts"]["startup_ms"] = json!(budget_ms);
    value["timeouts"]["postgres_connect_ms"] = json!(budget_ms);
    value["timeouts"]["readiness_ms"] = json!(budget_ms);
    value
}

#[test]
fn private_snapshot_without_eof_cannot_hold_main_past_startup_deadline() {
    let value = short_startup_config(200);
    let started = Instant::now();
    let mut child =
        Service::spawn_with_snapshot_delay(&value, &credentials(), Duration::ZERO, true);
    assert!(!child.wait(Duration::from_secs(1)).success());
    assert!(started.elapsed() < Duration::from_secs(1));
    let log = child.log();
    assert!(log.contains("startup_timeout"), "{log}");
    assert!(!log.contains("startup_checks"));
    assert!(!log.contains("listening"));
    assert!(!log.contains(common::API_TOKEN));
}

#[test]
fn secret_delay_and_database_startup_share_one_absolute_budget() {
    use std::os::unix::net::UnixListener;
    let directory = tempfile::tempdir().unwrap();
    let mut value = short_startup_config(800);
    value["postgres"]["socket_dir"] = json!(directory.path());
    let port = value["postgres"]["port"].as_u64().unwrap();
    let listener = UnixListener::bind(directory.path().join(format!(".s.PGSQL.{port}"))).unwrap();
    listener.set_nonblocking(true).unwrap();
    let started = Instant::now();
    let mut child = Service::spawn_with_snapshot_delay(
        &value,
        &credentials(),
        Duration::from_millis(400),
        false,
    );
    let connection = loop {
        match listener.accept() {
            Ok((connection, _)) => break connection,
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                assert!(
                    started.elapsed() < Duration::from_secs(1),
                    "database startup never began"
                );
                std::thread::sleep(Duration::from_millis(5));
            }
            Err(error) => panic!("private startup socket failed: {error}"),
        }
    };
    // The private peer remains silent while the original startup budget expires.
    assert!(!child.wait(Duration::from_millis(700)).success());
    assert!(started.elapsed() < Duration::from_millis(1100));
    let log = child.log();
    assert!(log.contains("startup_checks"), "{log}");
    assert!(log.contains("startup_timeout"), "{log}");
    assert!(!log.contains("listening"));
    drop(connection);
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

#[tokio::test]
#[ignore = "scripts/test_brain_serve.sh: private scratch PostgreSQL SCRAM check"]
async fn private_scratch_scram_accepts_runtime_password_and_rejects_wrong_password() {
    let socket = std::env::var("BRAIN_CORE_TEST_PG_SOCKET").expect("scratch socket required");
    assert!(socket.ends_with("/.core-test-pg") && std::path::Path::new(&socket).is_absolute());
    assert!(std::env::var_os("PGPASSWORD").is_none());
    assert!(std::env::var_os("BRAIN_SERVE_PG_PASSWORD").is_none());
    let options = PgConnectOptions::new_without_pgpass()
        .host(&socket)
        .port(55439)
        .username("brain_core_test")
        .database("brain_scram_test");
    let admin = PgPoolOptions::new()
        .max_connections(1)
        .connect_with(options.clone())
        .await
        .unwrap();
    let secret_scratch = tempfile::tempdir().unwrap();
    let password = format!(
        "synthetic-c1-{}",
        secret_scratch.path().file_name().unwrap().to_string_lossy()
    );
    let escaped_password = password.replace('\'', "''");
    let create_role =
        format!("CREATE ROLE brain_scram_fixture LOGIN PASSWORD '{escaped_password}'");
    sqlx::raw_sql(&create_role).execute(&admin).await.unwrap();
    let stored_password: String =
        sqlx::query_scalar("SELECT rolpassword FROM pg_authid WHERE rolname='brain_scram_fixture'")
            .fetch_one(&admin)
            .await
            .unwrap();
    assert!(stored_password.starts_with("SCRAM-SHA-256$"));

    let authenticated = PgPoolOptions::new()
        .max_connections(1)
        .connect_with(
            options
                .clone()
                .username("brain_scram_fixture")
                .password(&password),
        )
        .await
        .unwrap();
    let current_user: String = sqlx::query_scalar("SELECT current_user")
        .fetch_one(&authenticated)
        .await
        .unwrap();
    assert_eq!(current_user, "brain_scram_fixture");
    authenticated.close().await;

    let wrong_password = format!("{password}-wrong");
    let rejected = PgPoolOptions::new()
        .max_connections(1)
        .connect_with(
            options
                .username("brain_scram_fixture")
                .password(&wrong_password),
        )
        .await;
    assert!(matches!(
        rejected,
        Err(sqlx::Error::Database(error)) if error.code().as_deref() == Some("28P01")
    ));
    sqlx::query("DROP ROLE brain_scram_fixture")
        .execute(&admin)
        .await
        .unwrap();
    admin.close().await;
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
