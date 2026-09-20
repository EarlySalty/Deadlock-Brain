use super::*;
use std::{fs, process::Command};

const FIXTURE: &str = include_str!("../../../../config/bot.toml");
const ABSOLUTE: &str = "/fixture/config/bot.toml";

fn parse(text: &str) -> Result<BotConfig> {
    BotConfig::parse(text, Path::new(ABSOLUTE))
}

#[test]
fn valid_document_preserves_known_defaults() {
    let c = parse(FIXTURE).unwrap();
    assert_eq!(c.paths().project_root, Path::new("/fixture"));
    assert_eq!(c.paths().data_dir, Path::new("/fixture/data"));
    assert_eq!(c.http().timeout_seconds, 30);
    assert_eq!(c.sheet().gid, "0");
    assert_eq!(c.ai().max_completion_tokens, 16_000);
    assert_eq!(c.builds().min_matches, 500);
    assert!(c.ai().pin.is_none());
}

#[test]
fn unknown_fields_rejected_at_each_depth_without_input() {
    for (needle, replacement) in [
        (
            "schema_version = 1",
            "schema_version = 1\nnever_echo_fixture = 'sentinel-private-fixture'",
        ),
        (
            "[paths]",
            "[paths]\nnever_echo_fixture = 'sentinel-private-fixture'",
        ),
        (
            "[http]",
            "[http]\nnever_echo_fixture = 'sentinel-private-fixture'",
        ),
        (
            "[sheet]",
            "[sheet]\nnever_echo_fixture = 'sentinel-private-fixture'",
        ),
        (
            "[wiki]",
            "[wiki]\nnever_echo_fixture = 'sentinel-private-fixture'",
        ),
        (
            "[ai]",
            "[ai]\nnever_echo_fixture = 'sentinel-private-fixture'",
        ),
        (
            "[ai.selection]",
            "[ai.selection]\nnever_echo_fixture = 'sentinel-private-fixture'",
        ),
        (
            "[builds]",
            "[builds]\nnever_echo_fixture = 'sentinel-private-fixture'",
        ),
    ] {
        let e = parse(&FIXTURE.replacen(needle, replacement, 1)).unwrap_err();
        let diagnostic = format!("{e} {e:?}");
        assert!(!diagnostic.contains("sentinel-private-fixture"));
        assert!(!diagnostic.contains("never_echo_fixture"));
    }
}

#[test]
fn missing_required_fields_and_wrong_types_rejected() {
    for (old, new) in [
        ("schema_version = 1", ""),
        ("schema_version = 1", "schema_version = 2"),
        ("timeout_seconds = 30", ""),
        ("gid = \"0\"", "gid = 0"),
        ("enabled = false", "enabled = \"false\""),
        ("retry_attempts = 3", "retry_attempts = -1"),
        ("min_matches = 500", "min_matches = \"500\""),
        ("provider = \"fireworks\"", "provider = \"unapproved\""),
    ] {
        assert!(
            parse(&FIXTURE.replacen(old, new, 1)).is_err(),
            "case: {old}"
        );
    }
}

#[test]
fn id_bounds_and_control_characters_rejected() {
    for (old, new) in [
        ("gid = \"0\"", "gid = \"\""),
        ("gid = \"0\"", "gid = \"-1\""),
        ("gid = \"0\"", "gid = \"18446744073709551616\""),
        (
            "id = \"1fj9XMQmVUY0FY4cbozvB18PMBnpbdsaMFTZRLa74VRY\"",
            "id = \"not a sheet\"",
        ),
        ("project_root = \"..\"", "project_root = \"\""),
        ("project_root = \"..\"", "project_root = \"bad\\npath\""),
        (
            "user_agent = \"DeadlockBrain/0.1 contact=admin@earlysalty.com\"",
            "user_agent = \"bad\\r\\nHeader\"",
        ),
    ] {
        assert!(parse(&FIXTURE.replacen(old, new, 1)).is_err());
    }
}

#[test]
fn timeouts_counts_and_finite_sampling_have_bounds() {
    for (old, new) in [
        ("timeout_seconds = 30", "timeout_seconds = 0"),
        ("timeout_seconds = 30", "timeout_seconds = 3601"),
        ("retry_attempts = 3", "retry_attempts = 0"),
        (
            "retry_backoff_milliseconds = 600",
            "retry_backoff_milliseconds = 60001",
        ),
        ("min_delay_seconds = 5.0", "min_delay_seconds = nan"),
        ("min_delay_seconds = 5.0", "min_delay_seconds = -1.0"),
        ("temperature = 0.2", "temperature = inf"),
        ("temperature = 0.2", "temperature = 2.1"),
        ("top_p = 0.9", "top_p = 0.0"),
        ("top_p = 0.9", "top_p = 1.1"),
        ("max_completion_tokens = 16000", "max_completion_tokens = 0"),
        ("page_size = 100", "page_size = 201"),
        ("max_pages = 100", "max_pages = 0"),
        (
            "check_interval_seconds = 3600",
            "check_interval_seconds = 0",
        ),
        (
            "last_good_ttl_seconds = 86400",
            "last_good_ttl_seconds = 120",
        ),
        ("probe_budget_seconds = 300", "probe_budget_seconds = 0"),
        ("min_matches = 500", "min_matches = -1"),
        (
            "min_prevalence_builds = 30",
            "min_prevalence_builds = 1000000001",
        ),
    ] {
        assert!(
            parse(&FIXTURE.replacen(old, new, 1)).is_err(),
            "case: {old} -> {new}"
        );
    }
}

#[test]
fn endpoint_cannot_contain_credentials_queries_or_other_hosts() {
    for value in [
        "https://sentinel-private-fixture@api.fireworks.ai/inference/v1",
        "https://api.fireworks.ai/inference/v1?token=sentinel-private-fixture",
        "https://api.fireworks.ai/inference/v1#sentinel-private-fixture",
        "https://api.fireworks.ai.invalid/inference/v1",
        "http://api.fireworks.ai/inference/v1",
        "https://api.fireworks.ai:8443/inference/v1",
    ] {
        let text = FIXTURE.replacen("https://api.fireworks.ai/inference/v1", value, 1);
        let e = parse(&text).unwrap_err();
        assert!(!format!("{e:?} {e}").contains("sentinel-private-fixture"));
    }
}

#[test]
fn explicit_pin_is_validated_and_automatic_is_distinct() {
    for id in [
        "accounts/fireworks/models/deepseek-v4-flash",
        "accounts/fireworks/models/deepseek-v4.1-flash",
        "accounts/fireworks/models/deepseek-v4p1-flash",
        "accounts/fireworks/models/deepseek-v4-flash-20260901",
    ] {
        let text = FIXTURE.replacen("[ai]", &format!("[ai]\npin = {id:?}"), 1);
        assert_eq!(parse(&text).unwrap().ai().pin.as_deref(), Some(id));
    }
    for id in [
        "",
        "deepseek-flash-latest",
        "accounts/fireworks/models/deepseek-v4-pro",
        "accounts/fireworks/models/deepseek-v4-flash-preview",
        "accounts/fireworks/models/deepseek-v4-flash-distill",
        "accounts/fireworks/models/deepseek-v4-flash-vision",
        "accounts/other/models/deepseek-v4-flash",
        "accounts/fireworks/models/deepseek-v4..1-flash",
    ] {
        let text = FIXTURE.replacen("[ai]", &format!("[ai]\npin = {id:?}"), 1);
        assert!(parse(&text).is_err());
    }
}

#[test]
fn missing_relative_directory_and_oversized_files_fail_safely() {
    let temp = tempfile::tempdir().unwrap();
    assert!(BotConfig::load(&temp.path().join("missing.toml")).is_err());
    assert!(BotConfig::load(Path::new("config/bot.toml")).is_err());
    assert!(BotConfig::load(temp.path()).is_err());
    let path = temp.path().join("oversized.toml");
    fs::write(&path, vec![b'#'; MAX_CONFIG_BYTES as usize + 1]).unwrap();
    assert!(BotConfig::load(&path).is_err());
    fs::write(&path, [0xff, 0xfe]).unwrap();
    assert!(BotConfig::load(&path).is_err());
}

#[test]
fn path_resolution_is_independent_of_working_directory_and_does_not_create_dirs() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().join("config");
    fs::create_dir(&base).unwrap();
    let file = base.join("bot.toml");
    fs::write(&file, FIXTURE).unwrap();
    let c = BotConfig::load(&file).unwrap();
    assert_eq!(c.paths().data_dir, temp.path().join("data"));
    assert!(!c.paths().data_dir.exists());
    assert_eq!(fs::read_to_string(file).unwrap(), FIXTURE);
}

#[cfg(unix)]
#[test]
fn relative_paths_follow_real_config_location_not_symlink_location() {
    let temp = tempfile::tempdir().unwrap();
    let real_dir = temp.path().join("real/config");
    fs::create_dir_all(&real_dir).unwrap();
    let real = real_dir.join("bot.toml");
    fs::write(&real, FIXTURE).unwrap();
    let link = temp.path().join("linked.toml");
    std::os::unix::fs::symlink(&real, &link).unwrap();
    assert_eq!(
        BotConfig::load(&link).unwrap().paths().data_dir,
        temp.path().join("real/data")
    );
}

#[test]
fn failed_reload_does_not_replace_or_mutate_running_snapshot() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("bot.toml");
    fs::write(&path, FIXTURE).unwrap();
    let running = BotConfig::load(&path).unwrap();
    let consumer = Arc::clone(&running);
    let before = running.fingerprint().to_string();
    fs::write(&path, "[sentinel-private-fixture\n").unwrap();
    assert!(BotConfig::load(&path).is_err());
    assert!(Arc::ptr_eq(&running, &consumer));
    assert_eq!(running.fingerprint(), before);
    assert_eq!(consumer.http().timeout_seconds, 30);
}

#[test]
fn redacted_status_and_debug_do_not_echo_free_text() {
    let text = FIXTURE.replace(
        "DeadlockBrain/0.1 contact=admin@earlysalty.com",
        "sentinel-private-fixture",
    );
    let c = parse(&text).unwrap();
    assert!(!format!("{c:?} {}", c.redacted_status()).contains("sentinel-private-fixture"));
    assert_eq!(c.redacted_status()["reload"], "restart_required");
}

#[test]
fn formatting_changes_do_not_change_effective_fingerprint() {
    assert_eq!(
        parse(FIXTURE).unwrap().fingerprint(),
        parse(&format!("# Comment\n{FIXTURE}\n"))
            .unwrap()
            .fingerprint()
    );
    assert_ne!(
        parse(FIXTURE).unwrap().fingerprint(),
        parse(&FIXTURE.replacen("timeout_seconds = 30", "timeout_seconds = 31", 1))
            .unwrap()
            .fingerprint()
    );
}

#[test]
fn child_snapshot_ignores_legacy_environment() {
    let c = parse(FIXTURE).unwrap();
    assert_eq!(c.http().timeout_seconds, 30);
    assert_eq!(c.sheet().gid, "0");
    assert_eq!(c.paths().data_dir, Path::new("/fixture/data"));
    assert!(c.ai().pin.is_none());
    assert_eq!(c.ai().timeout_seconds, 300);
    assert_eq!(c.builds().min_matches, 500);
}

#[test]
fn env_has_no_override_effect_in_isolated_child() {
    let result = Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "bot_config::tests::child_snapshot_ignores_legacy_environment",
        ])
        .envs([
            ("DEADLOCK_BRAIN_DATA_DIR", "/wrong"),
            ("DEADLOCK_BRAIN_ROOT", "/wrong"),
            ("DEADLOCK_STATS_SHEET_GID", "999"),
            ("FIREWORKS_MODEL", "unapproved"),
            ("FIREWORK_MODEL", "unapproved"),
            ("FIREWORKS_TIMEOUT_SECONDS", "1"),
            ("DBRAIN_BUILDS_MIN_MATCHES", "1"),
        ])
        .output()
        .unwrap();
    assert!(result.status.success(), "isolated config assertion failed");
    assert!(String::from_utf8_lossy(&result.stdout).contains("1 passed"));
}
