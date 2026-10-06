use serde_json::{json, Value};
use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::Path,
    process::{Command, Output},
};

fn git(repo: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .args(["-c", "core.hooksPath=/dev/null"])
        .args(args)
        .current_dir(repo)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_AUTHOR_NAME", "Fixture")
        .env("GIT_AUTHOR_EMAIL", "fixture@example.invalid")
        .env("GIT_COMMITTER_NAME", "Fixture")
        .env("GIT_COMMITTER_EMAIL", "fixture@example.invalid")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap().trim_end().into()
}

fn fixture() -> (tempfile::TempDir, Value) {
    let dir = tempfile::tempdir().unwrap();
    let repo = dir.path().join("repo");
    fs::create_dir(&repo).unwrap();
    git(&repo, &["init", "--quiet"]);
    git(
        &repo,
        &[
            "remote",
            "add",
            "origin",
            "https://github.com/deadlock-wiki/deadlock-data.git",
        ],
    );
    fs::create_dir(repo.join("data")).unwrap();
    fs::write(repo.join("data/version.txt"), "ClientVersion=123\n").unwrap();
    git(&repo, &["add", "data"]);
    git(&repo, &["commit", "--quiet", "-m", "fixture"]);
    let pin = json!({"schema_version":1,"deadlock_data":{
        "commit":git(&repo, &["rev-parse", "HEAD"]),
        "parser_revision":dbrain_sources::deadlock_data::PARSER_REVISION,
        "schema_version":1,"data_version":"123"
    }});
    (dir, pin)
}

fn cli() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_deadlock-brain"));
    command.env_clear().env("PATH", "/usr/bin:/bin");
    command
}

fn wiki_config(dir: &Path, pins: &Value) -> Value {
    json!({"infisical_config":dir.join("never-read-credentials.json"),
        "source_repository":dir.join("repo"),"raw_directory":dir.join("raw"),
        "publication_root":dir.join("published"),"source_pins":pins})
}

fn wiki_check(dir: &Path, config: &Value) -> Output {
    let path = dir.join("wiki.json");
    fs::write(&path, serde_json::to_vec(config).unwrap()).unwrap();
    cli()
        .args(["wiki", "refresh", "--config"])
        .arg(path)
        .arg("--check-config")
        .output()
        .unwrap()
}

#[test]
fn wiki_startup_checks_pins_before_credentials_or_writes() {
    let (dir, pin) = fixture();
    let config = wiki_config(dir.path(), &pin);
    let out = wiki_check(dir.path(), &config);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(
        serde_json::from_slice::<Value>(&out.stdout).unwrap()["writes"],
        false
    );
    assert!(!dir.path().join("raw").exists());
    assert!(!dir.path().join("published").exists());
    assert!(!dir.path().join("never-read-credentials.json").exists());
    for field in ["commit", "parser_revision"] {
        let mut bad = config.clone();
        bad["source_pins"]["deadlock_data"]
            .as_object_mut()
            .unwrap()
            .remove(field);
        let out = wiki_check(dir.path(), &bad);
        assert!(!out.status.success());
        assert!(String::from_utf8_lossy(&out.stderr).contains(field));
    }
    let mut bad = config.clone();
    bad["source_pins"]["deadlock_data"]["commit"] = json!("f".repeat(40));
    let out = wiki_check(dir.path(), &bad);
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("unavailable locally"));
    let mut bad = config;
    bad["wiki"] = json!({"enabled":true});
    let out = wiki_check(dir.path(), &bad);
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("wiki.source_pin"));
}

#[test]
fn pull_uses_explicit_config_without_hidden_environment_overrides() {
    let (dir, pin) = fixture();
    let file = dir.path().join("pins.json");
    fs::write(&file, serde_json::to_vec(&pin).unwrap()).unwrap();
    let mut command = cli();
    command
        .args(["pull", "deadlock-data", "--source-config"])
        .arg(&file)
        .arg("--repo-dir")
        .arg(dir.path().join("repo"))
        .arg("--check-config");
    let out = command.output().unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(
        serde_json::from_slice::<Value>(&out.stdout).unwrap()["source_pin"],
        pin["deadlock_data"]
    );
    let out = command
        .env("DBRAIN_DEADLOCK_DATA_COMMIT", "f".repeat(40))
        .output()
        .unwrap();
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("pin overrides"));
    let out = cli()
        .args(["pull", "deadlock-data", "--check-config"])
        .output()
        .unwrap();
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("never implicit HEAD"));
}

#[tokio::test]
#[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
async fn enrichment_cli_exits_on_failed_batches_and_keeps_their_summary() {
    let dsn = std::env::var("DEADLOCK_CENTRAL_DSN").expect("scratch Postgres required");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(&dsn)
        .await
        .expect("scratch Postgres");
    let dir = tempfile::tempdir().unwrap();
    let run = |args: &[&str]| {
        cli()
            .env("DEADLOCK_CENTRAL_DSN", &dsn)
            .env("DEADLOCK_BRAIN_DATA_DIR", dir.path())
            .env("FIREWORKS_BASE_URL", "http://127.0.0.1:1")
            .args(args)
            .output()
            .unwrap()
    };
    let args = ["enrich", "patch-impact", "--limit", "1"];
    let empty = run(&args);
    assert_eq!(empty.status.code(), Some(0));
    let empty: Value = serde_json::from_slice(&empty.stdout).unwrap();
    assert_eq!(empty["processed"], 0);
    assert_eq!(empty["failed"], 0);

    let event_id: i64 = sqlx::query_scalar(
        "INSERT INTO brain.patch_events (legacy_patch_snapshot_id, patch_external_id, source_kind, line_index, entity_type, entity_name, change_type, raw_line, normalized_line, confidence, event_hash, created_at) VALUES (0, 'fixture-cli', 'patch_notes', 1, 'hero', 'FixtureCliBatch', 'buff', 'Cooldown increased from 10s to 12s', 'Cooldown increased from 10s to 12s', 1.0, 'fixture-cli-batch', now()) RETURNING id",
    )
    .fetch_one(&pool)
    .await
    .expect("fixture event");
    let failed = run(&args);
    assert_eq!(failed.status.code(), Some(1));
    let summary: Value = serde_json::from_slice(&failed.stdout).unwrap();
    assert_eq!(summary["processed"], 1);
    assert_eq!(summary["success"], 0);
    assert_eq!(summary["failed"], 1);
    let (status, result_text): (String, String) = sqlx::query_as(
        "SELECT status, result_text FROM brain.patch_impact_notes WHERE entity_name = 'FixtureCliBatch'",
    )
    .fetch_one(&pool)
    .await
    .expect("saved diagnostic");
    assert_eq!(status, "analysis_failed");
    assert!(!result_text.is_empty());
    let retry = run(&args);
    assert_eq!(retry.status.code(), Some(1));
    let retry: Value = serde_json::from_slice(&retry.stdout).unwrap();
    assert_eq!(retry["processed"], 1);
    assert_eq!(retry["failed"], 1);

    let meta = run(&["enrich", "meta-trends"]);
    assert_eq!(meta.status.code(), Some(1));
    let meta: Value = serde_json::from_slice(&meta.stdout).unwrap();
    assert_eq!(meta["processed"], 2);
    assert_eq!(meta["success"], 0);
    assert_eq!(meta["failed"], 2);
    sqlx::query("DELETE FROM brain.patch_impact_notes WHERE entity_name = 'FixtureCliBatch'")
        .execute(&pool)
        .await
        .expect("cleanup note");
    sqlx::query("DELETE FROM brain.patch_events WHERE id = $1")
        .bind(event_id)
        .execute(&pool)
        .await
        .expect("cleanup event");
}

fn executable(path: &Path, content: &str) {
    fs::write(path, content).unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
}

#[test]
fn completion_runner_is_nonmutating_and_propagates_failure() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap();
    let script = root.join("architecture/migration/s12/check-completion.sh");
    let script_text = fs::read_to_string(&script).unwrap();
    assert!(!script_text.contains("python3"));
    let before = git(&root, &["status", "--short"]);
    for fail in [false, true] {
        let dir = tempfile::tempdir().unwrap();
        let tools = dir.path().join("tools");
        let target = dir.path().join("target");
        fs::create_dir(&tools).unwrap();
        fs::create_dir_all(target.join("release")).unwrap();
        executable(
            &tools.join("cargo"),
            if fail {
                "#!/bin/bash\n[[ $1 == clippy ]] && exit 19\nexit 0\n"
            } else {
                "#!/bin/bash\nexit 0\n"
            },
        );
        for tool in ["rustc", "rustfmt"] {
            executable(&tools.join(tool), "#!/bin/bash\nexit 0\n");
        }
        executable(
            &target.join("release/dbrain-s12-wiki-probe"),
            "#!/bin/bash\n[[ $* == *--require-production-ready* ]] && exit 3\nexit 0\n",
        );
        let out = Command::new("bash")
            .arg(&script)
            .current_dir(&root)
            .env_clear()
            .env("PATH", format!("{}:/usr/bin:/bin", tools.display()))
            .env("HOME", dir.path())
            .env("CARGO_TARGET_DIR", &target)
            .output()
            .unwrap();
        assert_eq!(
            out.status.success(),
            !fail,
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        let stdout = String::from_utf8(out.stdout).unwrap();
        let reports = Path::new(
            stdout
                .lines()
                .find_map(|line| line.strip_prefix("Wiki completion artifacts: "))
                .unwrap(),
        );
        assert!(reports.starts_with(root.join("rust/target")));
        let evidence = fs::read_to_string(reports.join("completion-results.tsv")).unwrap();
        assert_eq!(evidence.lines().count(), 10);
        if fail {
            assert!(evidence.contains("contracts-source-clippy\t19\t"));
        }
        assert!(fs::read_to_string(reports.join("status.txt"))
            .unwrap()
            .contains("complete=true"));
        assert_eq!(git(&root, &["status", "--short"]), before);
    }
}
