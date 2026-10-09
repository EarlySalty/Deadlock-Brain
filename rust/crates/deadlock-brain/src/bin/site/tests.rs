#[path = "compare_tests.rs"]
mod compare_tests;
#[path = "profile_tests.rs"]
mod profile_tests;

use super::*;
use reqwest::Client;
use serde_json::{json, Value};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use std::{fs, path::PathBuf, process::Command};
use tempfile::TempDir;

struct TestStore {
    pool: PgPool,
}

impl TestStore {
    fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn migrate_core(&self) -> Result<()> {
        brain_storage::PgStore::new(self.pool.clone())
            .migrate_core()
            .await?;
        Ok(())
    }

    async fn check_site_comments_schema(&self) -> Result<()> {
        sqlx::query("SELECT id,group_key,text,ts FROM brain.site_comments_v1 LIMIT 0")
            .fetch_all(&self.pool)
            .await?;
        Ok(())
    }

    async fn migrate_site_comments(&self) -> Result<()> {
        brain_storage::PgStore::new(self.pool.clone())
            .migrate_site_comments()
            .await?;
        self.check_site_comments_schema().await
    }
}

struct Postgres {
    directory: TempDir,
    port: u16,
}

impl Postgres {
    fn new() -> Self {
        let directory = tempfile::tempdir().unwrap();
        let port = std::net::TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port();
        let data = directory.path().join("data");
        let result = Command::new("/usr/lib/postgresql/16/bin/initdb")
            .args([
                "--no-locale",
                "--encoding=UTF8",
                "--auth=trust",
                "--username=owner",
                "-D",
            ])
            .arg(&data)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        let options = format!("-k {} -p {port} -h '' -F", directory.path().display());
        let result = Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
            .arg("-D")
            .arg(data)
            .arg("-l")
            .arg(directory.path().join("postgres.log"))
            .args(["-o", &options, "-w", "start"])
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        Self { directory, port }
    }

    async fn pool(&self, user: &str) -> PgPool {
        PgPoolOptions::new()
            .max_connections(if user == "brain_migrate" { 3 } else { 4 })
            .connect_with(
                PgConnectOptions::new_without_pgpass()
                    .host(&self.directory.path().to_string_lossy())
                    .port(self.port)
                    .database("brain")
                    .username(user)
                    .password("")
                    .ssl_mode(PgSslMode::Disable),
            )
            .await
            .unwrap()
    }

    fn roles(&self) {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let result = Command::new("/usr/lib/postgresql/16/bin/psql")
            .args(["-X", "-v", "ON_ERROR_STOP=1", "-h"])
            .arg(self.directory.path())
            .args([
                "-p",
                &self.port.to_string(),
                "-U",
                "owner",
                "-d",
                "postgres",
                "-f",
            ])
            .arg(root.join("ops/brain-postgres/roles.sql"))
            .args([
                "-c",
                "CREATE ROLE brain_site LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOREPLICATION NOBYPASSRLS NOINHERIT CONNECTION LIMIT 4; GRANT CONNECT ON DATABASE brain TO brain_site;",
            ])
            .env_remove("PGOPTIONS")
            .env_remove("PGPASSWORD")
            .env("PGPASSFILE", "/dev/null")
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
    }

    fn grants(&self) {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let result = Command::new("/usr/lib/postgresql/16/bin/psql")
            .args(["-X", "-v", "ON_ERROR_STOP=1", "-v", "db=brain", "-h"])
            .arg(self.directory.path())
            .args([
                "-p",
                &self.port.to_string(),
                "-U",
                "owner",
                "-d",
                "brain",
                "-f",
            ])
            .arg(root.join("ops/brain-postgres/grants.sql"))
            .args([
                "-c",
                "REVOKE ALL ON ALL TABLES IN SCHEMA brain FROM brain_site; REVOKE ALL ON ALL SEQUENCES IN SCHEMA brain FROM brain_site; GRANT USAGE ON SCHEMA brain TO brain_site; GRANT SELECT ON brain.site_comments_v1 TO brain_site; GRANT EXECUTE ON FUNCTION brain.append_site_comment_v1(text,text,text) TO brain_site; REVOKE ALL ON brain.site_comments_v1 FROM brain_readonly;",
            ])
            .env_remove("PGOPTIONS")
            .env_remove("PGPASSWORD")
            .env("PGPASSFILE", "/dev/null")
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
}

impl Drop for Postgres {
    fn drop(&mut self) {
        let result = Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
            .arg("-D")
            .arg(self.directory.path().join("data"))
            .args(["-m", "immediate", "-w", "stop"])
            .output()
            .unwrap();
        assert!(result.status.success());
    }
}

async fn server(root: &Path, pool: PgPool) -> (String, tokio::task::JoinHandle<()>) {
    let router = router(root, pool).await.unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let handle = tokio::spawn(async move { axum::serve(listener, router).await.unwrap() });
    (url, handle)
}

async fn raw_get(url: &str, path: &str) -> String {
    let address = url.strip_prefix("http://").unwrap().to_owned();
    let path = path.to_owned();
    tokio::task::spawn_blocking(move || {
        use std::io::{Read, Write};
        let mut stream = std::net::TcpStream::connect(&address).unwrap();
        stream
            .write_all(
                format!("GET {path} HTTP/1.1\r\nHost: {address}\r\nConnection: close\r\n\r\n")
                    .as_bytes(),
            )
            .unwrap();
        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();
        response
    })
    .await
    .unwrap()
}

fn fixtures() -> TempDir {
    let root = tempfile::tempdir().unwrap();
    fs::create_dir_all(root.path().join("site/vendor")).unwrap();
    fs::create_dir_all(root.path().join("understanding")).unwrap();
    fs::write(root.path().join("builds_registry.json"), b"{\"Warden\":[]}").unwrap();
    fs::write(
        root.path().join("site/index.html"),
        b"<!doctype html><title>Site</title>",
    )
    .unwrap();
    root
}

#[test]
fn assets_are_explicit_and_symlinks_never_followed() {
    let root = fixtures();
    let assets = assets::Assets::new(root.path()).unwrap();
    fs::write(root.path().join("understanding/Warden.md"), b"Dossier").unwrap();
    assert_eq!(
        assets.read("/understanding/Warden.md").unwrap().1,
        b"Dossier"
    );
    for path in [
        "/site/server.py",
        "/comments.json",
        "/builds_registry.md",
        "/understanding/Private.md",
        "/site/entities/hero/xyz.html",
        "/site/entities/hero/abc.html",
        "/site/entities/wiki/abcd.html",
        "/site/entities/hero/abcd.json",
        "/site/entities/hero/abcd.html/private",
        "/site/entities/",
        "/site/../index.html",
        "/site/%2e%2e/index.html",
        "/site/%252e%252e/index.html",
        "/site%5cindex.html",
        "/site/%00index.html",
        "/site//index.html",
    ] {
        assert!(assets.read(path).is_err(), "{path}");
    }
    fs::write(
        root.path().join("builds_registry.json"),
        b"{\"Warden\":[],\"Abrams\":[]}",
    )
    .unwrap();
    fs::write(root.path().join("understanding/Abrams.md"), b"Aktualisiert").unwrap();
    assert_eq!(
        assets.read("/understanding/Abrams.md").unwrap().1,
        b"Aktualisiert"
    );
    let outside = tempfile::tempdir().unwrap();
    fs::write(outside.path().join("private"), b"Private").unwrap();
    std::os::unix::fs::symlink(
        outside.path().join("private"),
        root.path().join("site/app.js"),
    )
    .unwrap();
    assert!(assets.read("/site/app.js").is_err());
    fs::create_dir(outside.path().join("hero")).unwrap();
    fs::write(outside.path().join("hero/abcd.html"), b"Private").unwrap();
    std::os::unix::fs::symlink(outside.path(), root.path().join("site/entities")).unwrap();
    assert!(assets.read("/site/entities/hero/abcd.html").is_err());
    assert!(Command::new("/usr/bin/mkfifo")
        .arg(root.path().join("site/style.css"))
        .status()
        .unwrap()
        .success());
    assert!(assets.read("/site/style.css").is_err());
}

#[tokio::test]
async fn postgres_http_comments_survive_restart_and_roles_are_isolated() {
    let pg = Postgres::new();
    pg.roles();
    let owner = pg.pool("brain_migrate").await;
    let store = TestStore::new(owner.clone());
    store.migrate_core().await.unwrap();
    assert!(store.check_site_comments_schema().await.is_err());
    store.migrate_site_comments().await.unwrap();
    store.migrate_site_comments().await.unwrap();
    store.check_site_comments_schema().await.unwrap();
    pg.grants();
    let pool = pg.pool("brain_site").await;
    let root = fixtures();
    assert!(router(root.path(), owner.clone()).await.is_err());
    let (url, handle) = server(root.path(), pool.clone()).await;
    let client = Client::new();
    assert_eq!(
        client
            .get(format!("{url}/api/comments"))
            .send()
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap(),
        json!({})
    );
    let first = json!({"action":"add", "text":"Grüße <script>bleibt Text</script>", "ts":"06.10.2026, 12:34"});
    let reply = client
        .post(format!("{url}/api/comments"))
        .json(&first)
        .send()
        .await
        .unwrap();
    assert_eq!(reply.status(), StatusCode::OK);
    assert_eq!(
        reply.json::<Value>().await.unwrap(),
        json!({"ok":true,"comments":[{"text":first["text"],"ts":first["ts"]}]})
    );
    let second =
        json!({"action":"add", "key":"Warden", "text":"ä".repeat(4001), "ts":"ß".repeat(41)});
    let reply = client
        .post(format!("{url}/api/comments"))
        .json(&second)
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();
    assert_eq!(
        reply["comments"][0]["text"]
            .as_str()
            .unwrap()
            .chars()
            .count(),
        4000
    );
    assert_eq!(
        reply["comments"][0]["ts"].as_str().unwrap().chars().count(),
        40
    );
    for body in [
        json!({"action":"add","text":" ","key":"general"}),
        json!({"action":"delete","key":"general"}),
    ] {
        let reply = client
            .post(format!("{url}/api/comments"))
            .json(&body)
            .send()
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap();
        assert_eq!(reply["comments"].as_array().unwrap().len(), 1);
    }
    for body in [
        json!({"action":"add","text":"test","key":"ä".repeat(41)}),
        json!({"action":"add","text":"test","ts":"<img src=x onerror=alert(1)>"}),
        json!({"action":"add","text":"test\0"}),
    ] {
        assert_eq!(
            client
                .post(format!("{url}/api/comments"))
                .json(&body)
                .send()
                .await
                .unwrap()
                .status(),
            StatusCode::BAD_REQUEST
        );
    }
    assert_eq!(
        client
            .post(format!("{url}/api/comments"))
            .header("Content-Type", "application/json")
            .body("{")
            .send()
            .await
            .unwrap()
            .status(),
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        client
            .post(format!("{url}/api/comments"))
            .header("Content-Type", "application/json")
            .body("a".repeat(65537))
            .send()
            .await
            .unwrap()
            .status(),
        StatusCode::PAYLOAD_TOO_LARGE
    );
    let mut tasks = Vec::new();
    for index in 0..16 {
        let client = client.clone();
        let url = url.clone();
        tasks.push(tokio::spawn(async move {
            let response = client
                .post(format!("{url}/api/comments"))
                .json(&json!({"action":"add","key":"parallel","text":index.to_string()}))
                .send()
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            response.json::<Value>().await.unwrap()["comments"]
                .as_array()
                .unwrap()
                .len()
        }));
    }
    let mut lengths = Vec::new();
    for task in tasks {
        lengths.push(task.await.unwrap());
    }
    lengths.sort();
    assert_eq!(lengths, (1..=16).collect::<Vec<_>>());
    let saved = client
        .get(format!("{url}/api/comments"))
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();
    assert_eq!(saved["parallel"].as_array().unwrap().len(), 16);
    handle.abort();
    let _ = handle.await;
    pool.close().await;
    let pool = pg.pool("brain_site").await;
    let (url, handle) = server(root.path(), pool.clone()).await;
    assert_eq!(
        client
            .get(format!("{url}/api/comments?unused=1"))
            .send()
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap(),
        saved
    );
    for statement in [
        "INSERT INTO brain.site_comments_v1(group_key,text,ts) VALUES ('test','test','')",
        "SELECT nextval('brain.site_comments_v1_id_seq')",
        "DELETE FROM brain.site_comments_v1",
        "UPDATE brain.site_comments_v1 SET text='mutated'",
        "SELECT * FROM brain.source_record_heads",
        "CREATE TABLE brain.site_forbidden(id int)",
        "SELECT setval('brain.site_comments_v1_id_seq',1)",
    ] {
        let error = sqlx::query(statement).execute(&pool).await.unwrap_err();
        assert_eq!(
            error
                .as_database_error()
                .unwrap_or_else(|| panic!("{statement}: {error:?}"))
                .code()
                .as_deref(),
            Some("42501"),
            "{statement}"
        );
    }
    sqlx::query("ALTER TABLE brain.site_comments_v1 RENAME TO site_comments_test_unavailable")
        .execute(&owner)
        .await
        .unwrap();
    assert_eq!(
        client
            .get(format!("{url}/api/comments"))
            .send()
            .await
            .unwrap()
            .status(),
        StatusCode::SERVICE_UNAVAILABLE
    );
    assert_eq!(
        client
            .post(format!("{url}/api/comments"))
            .json(&first)
            .send()
            .await
            .unwrap()
            .status(),
        StatusCode::SERVICE_UNAVAILABLE
    );
    sqlx::query("ALTER TABLE brain.site_comments_test_unavailable RENAME TO site_comments_v1")
        .execute(&owner)
        .await
        .unwrap();
    assert_eq!(
        client
            .get(format!("{url}/api/comments"))
            .send()
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap(),
        saved
    );
    let before: i64 = sqlx::query_scalar("SELECT count(*) FROM brain.site_comments_v1")
        .fetch_one(&owner)
        .await
        .unwrap();
    store.migrate_site_comments().await.unwrap();
    let after: i64 = sqlx::query_scalar("SELECT count(*) FROM brain.site_comments_v1")
        .fetch_one(&owner)
        .await
        .unwrap();
    assert_eq!(before, after);
    assert!(
        sqlx::query("SELECT * FROM brain.append_site_comment_v1('test',$1,'')")
            .bind("x".repeat(4001))
            .execute(&pool)
            .await
            .is_err()
    );
    handle.abort();
    let _ = handle.await;
    pool.close().await;
    owner.close().await;
    println!("HTTP-Kommentarvertrag: Speicherung, 16 parallele POSTs, Neustart, Grenzen und fünf Rechteverbote geprüft.");
}

#[tokio::test]
async fn current_public_corpus_is_served_byte_for_byte_without_private_files() {
    let pg = Postgres::new();
    pg.roles();
    let owner = pg.pool("brain_migrate").await;
    let store = TestStore::new(owner.clone());
    store.migrate_core().await.unwrap();
    store.migrate_site_comments().await.unwrap();
    pg.grants();
    let pool = pg.pool("brain_site").await;
    let fixture = fixtures();
    for file in [
        "site/app.js",
        "site/style.css",
        "site/vendor/marked.min.js",
        "understanding/Warden.md",
        "HERO_STRENGTH.md",
        "ITEM_STRENGTH.md",
        "MASTER_METHODOLOGY.md",
    ] {
        fs::write(fixture.path().join(file), b"Fixture").unwrap();
    }
    for file in ["hero_meta_compact.json", "item_meta_compact.json"] {
        fs::write(fixture.path().join(file), b"[]").unwrap();
    }
    let corpus = std::env::var_os("BRAIN_SITE_TEST_CORPUS_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| fixture.path().to_owned());
    let (url, handle) = server(&corpus, pool.clone()).await;
    let client = Client::new();
    let registry: Value =
        serde_json::from_slice(&fs::read(corpus.join("builds_registry.json")).unwrap()).unwrap();
    let mut paths = vec![
        "site/index.html".to_string(),
        "site/app.js".into(),
        "site/style.css".into(),
        "site/vendor/marked.min.js".into(),
        "builds_registry.json".into(),
        "hero_meta_compact.json".into(),
        "item_meta_compact.json".into(),
        "HERO_STRENGTH.md".into(),
        "ITEM_STRENGTH.md".into(),
        "MASTER_METHODOLOGY.md".into(),
    ];
    paths.extend(
        registry
            .as_object()
            .unwrap()
            .keys()
            .map(|hero| format!("understanding/{hero}.md")),
    );
    for path in &paths {
        let response = client.get(format!("{url}/{path}")).send().await.unwrap();
        assert_eq!(response.headers()["x-content-type-options"], "nosniff");
        assert_eq!(response.headers()["cache-control"], "no-cache");
        let csp = response.headers()["content-security-policy"]
            .to_str()
            .unwrap();
        for directive in [
            "default-src 'self'",
            "script-src 'self'",
            "object-src 'none'",
            "base-uri 'self'",
            "frame-ancestors 'none'",
        ] {
            assert!(csp.split(';').any(|part| part.trim() == directive));
        }
        let expected = fs::read(corpus.join(path));
        match expected {
            Ok(expected) => {
                assert_eq!(response.status(), StatusCode::OK, "{path}");
                assert_eq!(response.bytes().await.unwrap().as_ref(), expected, "{path}");
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                assert_eq!(response.status(), StatusCode::NOT_FOUND)
            }
            Err(error) => panic!("{error}"),
        }
    }
    assert_eq!(
        client
            .get(format!("{url}/site/"))
            .send()
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap()
            .as_ref(),
        fs::read(corpus.join("site/index.html")).unwrap()
    );
    let response = client.head(format!("{url}/site/")).send().await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()["x-content-type-options"], "nosniff");
    assert_eq!(
        response.headers()["content-length"]
            .to_str()
            .unwrap()
            .parse::<usize>()
            .unwrap(),
        fs::read(corpus.join("site/index.html")).unwrap().len()
    );
    assert!(response.bytes().await.unwrap().is_empty());
    for path in [
        "site/server.py",
        "comments.json",
        "understanding/",
        "site/entities/",
        "patches.json",
        "meta/",
        "raw/",
        "site/entities/hero/private.md",
        "site/entities/hero/xyz.html",
    ] {
        let response = client.get(format!("{url}/{path}")).send().await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND, "{path}");
        assert!(!response.text().await.unwrap().contains("/home/"));
    }
    assert_eq!(
        client
            .head(format!("{url}/site/server.py"))
            .send()
            .await
            .unwrap()
            .status(),
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        client
            .post(format!("{url}/site/"))
            .send()
            .await
            .unwrap()
            .status(),
        StatusCode::METHOD_NOT_ALLOWED
    );
    assert_eq!(
        client
            .get(format!("{url}/brain"))
            .send()
            .await
            .unwrap()
            .status(),
        StatusCode::NOT_FOUND
    );
    let temp = fixtures();
    for kind in ["hero", "ability", "item"] {
        fs::create_dir_all(temp.path().join(format!("site/entities/{kind}"))).unwrap();
        fs::write(
            temp.path()
                .join(format!("site/entities/{kind}/77617264656e.html")),
            b"<!doctype html><h1>Steckbrief</h1>",
        )
        .unwrap();
    }
    let (fixture_url, fixture_handle) = server(temp.path(), pool.clone()).await;
    for kind in ["hero", "ability", "item"] {
        assert_eq!(
            client
                .get(format!(
                    "{fixture_url}/site/entities/{kind}/77617264656e.html"
                ))
                .send()
                .await
                .unwrap()
                .status(),
            StatusCode::OK
        );
    }
    let redirect = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap()
        .get(format!("{fixture_url}/site"))
        .send()
        .await
        .unwrap();
    assert_eq!(redirect.status(), StatusCode::MOVED_PERMANENTLY);
    assert_eq!(redirect.headers()["location"], "site/");
    fs::write(temp.path().join("private"), b"private-fixture").unwrap();
    std::os::unix::fs::symlink(temp.path().join("private"), temp.path().join("site/app.js"))
        .unwrap();
    fs::remove_dir(temp.path().join("site/vendor")).unwrap();
    let outside = tempfile::tempdir().unwrap();
    fs::write(outside.path().join("marked.min.js"), b"private-fixture").unwrap();
    std::os::unix::fs::symlink(outside.path(), temp.path().join("site/vendor")).unwrap();
    for path in [
        "/site/app.js",
        "/site/vendor/marked.min.js",
        "/site/../private",
        "/site/%2e%2e/private",
        "/site/%252e%252e/private",
        "/site%5cindex.html",
        "/site/%00index.html",
        "/site//index.html",
    ] {
        let response = raw_get(&fixture_url, path).await;
        assert!(response.starts_with("HTTP/1.1 404"), "{path}: {response}");
        assert!(!response.contains("private-fixture"));
        assert!(!response.contains("/home/"));
    }
    fixture_handle.abort();
    let _ = fixture_handle.await;
    handle.abort();
    let _ = handle.await;
    pool.close().await;
    owner.close().await;
    println!("HTTP-Dateivertrag: {} Dateien verglichen, drei Steckbriefpfade geprüft; synthetisches HTML ist kein Profil-Fertigbeleg.", paths.len());
}
