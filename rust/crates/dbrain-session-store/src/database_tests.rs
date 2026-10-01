use super::*;
use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    process::{Command, Output},
};

const FIXTURE_OWNER: &str = "brain_session_fixture";
const FIXTURE_PORT: u16 = 55449;

struct PrivatePostgres {
    scratch: Option<tempfile::TempDir>,
    bin: PathBuf,
    data: PathBuf,
    socket: PathBuf,
}

impl PrivatePostgres {
    fn new() -> Self {
        let scratch = tempfile::Builder::new()
            .prefix("brain-session-pg-")
            .tempdir_in("/tmp")
            .unwrap();
        fs::set_permissions(scratch.path(), fs::Permissions::from_mode(0o700)).unwrap();
        let output = Command::new("/usr/lib/postgresql/16/bin/pg_config")
            .env_clear()
            .arg("--bindir")
            .output()
            .unwrap();
        assert!(output.status.success());
        let bin = PathBuf::from(String::from_utf8(output.stdout).unwrap().trim());
        assert!(bin.is_absolute());
        let data = scratch.path().join("data");
        let socket = scratch.path().join("socket");
        fs::create_dir(&socket).unwrap();
        fs::set_permissions(&socket, fs::Permissions::from_mode(0o700)).unwrap();
        let fixture = Self {
            scratch: Some(scratch),
            bin,
            data,
            socket,
        };
        let version = fixture
            .command("postgres")
            .arg("--version")
            .output()
            .unwrap();
        assert!(version.status.success());
        assert!(String::from_utf8(version.stdout)
            .unwrap()
            .starts_with("postgres (PostgreSQL) 16."));
        let initialized = fixture
            .command("initdb")
            .arg("-D")
            .arg(&fixture.data)
            .args([
                "--username=brain_session_fixture",
                "--auth-local=peer",
                "--auth-host=reject",
                "--no-locale",
                "--encoding=UTF8",
            ])
            .output()
            .unwrap();
        Self::assert_success(initialized, "initdb");
        let user = Command::new("/usr/bin/id")
            .env_clear()
            .arg("-un")
            .output()
            .unwrap();
        assert!(user.status.success());
        let user = String::from_utf8(user.stdout).unwrap();
        let user = user.trim();
        assert!(!user.is_empty());
        assert!(user
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"_.-".contains(&byte)));
        fs::write(
            fixture.data.join("pg_ident.conf"),
            format!("session_fixture {user} {FIXTURE_OWNER}\n"),
        )
        .unwrap();
        fs::write(
            fixture.data.join("pg_hba.conf"),
            "local all all peer map=session_fixture\nhost all all all reject\n",
        )
        .unwrap();
        let started = fixture
            .command("pg_ctl")
            .arg("-D")
            .arg(&fixture.data)
            .arg("-l")
            .arg(fixture.data.join("server.log"))
            .args(["-w", "-t", "10", "-o"])
            .arg(format!(
                "-c listen_addresses='' -k '{}' -p {FIXTURE_PORT} -c unix_socket_permissions=0700 -c max_connections=12 -c shared_buffers=16MB",
                fixture.socket.display()
            ))
            .arg("start")
            .output()
            .unwrap();
        Self::assert_success(started, "pg_ctl start");
        fixture
    }

    fn command(&self, binary: &str) -> Command {
        let mut command = Command::new(self.bin.join(binary));
        command.env_clear();
        command
    }

    fn assert_success(output: Output, operation: &str) {
        assert!(
            output.status.success(),
            "{operation} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    fn options(&self) -> postgres::Config {
        let mut options = postgres::Config::new();
        options
            .host_path(&self.socket)
            .port(FIXTURE_PORT)
            .user(FIXTURE_OWNER)
            .dbname("postgres")
            .connect_timeout(std::time::Duration::from_secs(10));
        options
    }

    fn stop(&self) -> bool {
        if !self.data.join("postmaster.pid").exists() {
            return true;
        }
        ["fast", "immediate"].into_iter().any(|mode| {
            self.command("pg_ctl")
                .arg("-D")
                .arg(&self.data)
                .args(["-m", mode, "-w", "-t", "10", "stop"])
                .output()
                .is_ok_and(|output| output.status.success())
        })
    }
}

impl Drop for PrivatePostgres {
    fn drop(&mut self) {
        if !self.stop() {
            // Do not unlink a possibly live cluster or panic during unwinding.
            // Retain only our private fixture so the failure can be investigated.
            if let Some(scratch) = self.scratch.take() {
                eprintln!(
                    "private PostgreSQL cleanup failed: {}",
                    scratch.keep().display()
                );
            }
        }
    }
}

#[test]
fn encrypted_browser_state_roundtrip_concurrency_revocation_and_account_isolation() {
    let fixture = PrivatePostgres::new();
    let options = fixture.options();
    let mut admin = options.connect(NoTls).unwrap();
    // The real migration grants its runtime role access; create that role only
    // inside this disposable cluster, without login privileges or credentials.
    admin.batch_execute("CREATE ROLE deadlock NOLOGIN").unwrap();
    let database = format!("token_db_browser_{}", std::process::id());
    admin
        .batch_execute(&format!("CREATE DATABASE {database}"))
        .unwrap();
    let mut local = options.clone();
    local.dbname(&database);
    let mut client = local.connect(NoTls).unwrap();
    client.batch_execute("CREATE SCHEMA core").unwrap();
    client.batch_execute(include_str!("../../../../../Deadlock-Bots/rust/crates/dl-central-db/migrations/20260930220300_browser_credentials.sql")).unwrap();
    let cipher = FieldCipher::from_hex_key(&"11".repeat(32), "v1").unwrap();
    let state =
        json!({"cookies":[{"name":"synthetic_cookie","value":"synthetic_secret"}],"origins":[]});
    assert_eq!(
        load(&mut client, &cipher, "account-a").unwrap()["revision"],
        -1
    );
    save(&mut client, &cipher, "account-a", -1, &state).unwrap();
    let first = load(&mut client, &cipher, "account-a").unwrap();
    assert_eq!(first["revision"], 0);
    assert_eq!(first["state"], state);
    assert_eq!(
        load(&mut client, &cipher, "account-b").unwrap()["revision"],
        -1
    );
    assert!(save(&mut client, &cipher, "account-a", -1, &state).is_err());
    // Replay the actual helper get/put codec at the exact stored-state limit.
    let mut boundary = json!({"cookies":[],"origins":[],"padding":""});
    let overhead = serde_json::to_vec(&boundary).unwrap().len();
    boundary["padding"] = json!("x".repeat(LIMIT - overhead));
    assert_eq!(serde_json::to_vec(&boundary).unwrap().len(), LIMIT);
    save(&mut client, &cipher, "boundary-account", -1, &boundary).unwrap();
    let envelope = load(&mut client, &cipher, "boundary-account").unwrap();
    let encoded = encode_envelope(&envelope).unwrap();
    assert!(encoded.len() > LIMIT);
    let (revision, replay) = decode_envelope(encoded.as_slice()).unwrap();
    save(&mut client, &cipher, "boundary-account", revision, &replay).unwrap();
    assert_eq!(
        load(&mut client, &cipher, "boundary-account").unwrap()["revision"],
        1
    );
    let mut oversized = boundary.clone();
    oversized["padding"] = json!("x".repeat(LIMIT - overhead + 1));
    assert!(decode_envelope(
        encode_envelope(&json!({"revision":1,"state":oversized}))
            .unwrap()
            .as_slice()
    )
    .is_err());
    assert!(save(&mut client, &cipher, "boundary-account", 1, &oversized).is_err());
    assert_eq!(
        load(&mut client, &cipher, "boundary-account").unwrap()["revision"],
        1
    );
    let blob: Vec<u8> = client
        .query_one(
            "SELECT state_enc FROM core.browser_credentials WHERE account_id='account-a'",
            &[],
        )
        .unwrap()
        .get(0);
    assert!(!blob
        .windows(b"synthetic_secret".len())
        .any(|v| v == b"synthetic_secret"));
    assert!(cipher.decrypt_field(&blob, &aad("account-b")).is_err());
    save(&mut client, &cipher, "account-a", 0, &state).unwrap();
    assert!(save(&mut client, &cipher, "account-a", 0, &state).is_err());
    assert_eq!(
        load(&mut client, &cipher, "account-a").unwrap()["revision"],
        1
    );
    let wrong = FieldCipher::from_hex_key(&"22".repeat(32), "v1").unwrap();
    assert!(load(&mut client, &wrong, "account-a").is_err());
    client
        .execute(
            "UPDATE core.browser_credentials SET revoked_at=now() WHERE account_id='account-a'",
            &[],
        )
        .unwrap();
    assert!(load(&mut client, &cipher, "account-a").is_err());
    assert!(save(&mut client, &cipher, "account-a", 1, &state).is_err());
    assert!(save(&mut client, &cipher, "account-a", -1, &state).is_err());
    drop(client);
    admin
        .batch_execute(&format!("DROP DATABASE {database}"))
        .unwrap();
    drop(admin);
    assert!(fixture.stop(), "private PostgreSQL cleanup failed");
}
