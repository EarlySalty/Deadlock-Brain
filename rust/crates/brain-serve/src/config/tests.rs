use super::*;
use serde_json::{json, Value};
const EXAMPLE: &[u8] = include_bytes!("../../../../../config/brain-serve.example.toml");
#[path = "../../../../test-support/bot_toml.rs"]
mod fixture_toml;
fn example() -> Value {
    serde_json::to_value(crate::bot_toml::document(EXAMPLE).unwrap()["brain"]["serve"].clone())
        .unwrap()
}
fn config_bytes(value: &Value) -> Vec<u8> {
    let mut serve = value.clone();
    let operator = serve.as_object_mut().unwrap().remove("internal_operator");
    let mut root = json!({"brain":{"serve":serve}});
    if let Some(operator) = operator {
        root["brain"]["operator"] = operator;
    }
    fixture_toml::section(&root, &[])
}
fn parse(value: &Value) -> Result<Config, Error> {
    Config::parse(&config_bytes(value))
}

#[test]
fn example_parses_without_secret_values_and_preserves_pilot_defaults() {
    let config = Config::parse(EXAMPLE).unwrap();
    assert_eq!(config.bind.to_string(), "127.0.0.1:8787");
    assert_eq!(config.release.id, "pilot-r1");
    assert_eq!(config.retrieval.limit, 6);
    assert_eq!(config.budgets.max_input_tokens, 12000);
    assert_eq!(Budget::from(&config.budgets), Budget::default());
    assert_eq!(config.provider.api_key_env, "BRAIN_SERVE_PROVIDER_API_KEY");
}

#[test]
fn c9_releasebindung_ist_explizit_und_widersprueche_scheitern() {
    let mut value = example();
    assert!(parse(&value).unwrap().credentials[0].release.is_none());
    value["credentials"][0]["release"] = json!({"id":"docs-r1","knowledge_version":"docs-v1"});
    assert_eq!(
        parse(&value).unwrap().credentials[0]
            .release
            .as_ref()
            .unwrap()
            .id,
        "docs-r1"
    );
    for invalid in [
        json!({"id":"current","knowledge_version":"docs-v1"}),
        json!({"id":"docs-r1"}),
        json!({"id":"docs-r1","knowledge_version":"latest"}),
    ] {
        let mut changed = value.clone();
        changed["credentials"][0]["release"] = invalid;
        assert!(parse(&changed).is_err());
    }
    let mut second = value["credentials"][0].clone();
    second["token_env"] = json!("SECOND_CLIENT_TOKEN");
    second["release"]["id"] = json!("other-r1");
    value["credentials"].as_array_mut().unwrap().push(second);
    assert!(parse(&value).is_err());
}

#[test]
fn every_required_section_and_field_is_fail_closed() {
    let original = example();
    for key in original.as_object().unwrap().keys() {
        let mut value = original.clone();
        value.as_object_mut().unwrap().remove(key);
        assert!(parse(&value).is_err(), "missing section: {key}");
    }
    for section in [
        "postgres",
        "release",
        "provider",
        "budgets",
        "timeouts",
        "retrieval",
        "kernel",
    ] {
        for key in original[section].as_object().unwrap().keys() {
            let mut value = original.clone();
            value[section].as_object_mut().unwrap().remove(key);
            assert!(parse(&value).is_err(), "missing field: {section}.{key}");
        }
    }
    for key in original["credentials"][0].as_object().unwrap().keys() {
        let mut value = original.clone();
        value["credentials"][0].as_object_mut().unwrap().remove(key);
        assert!(parse(&value).is_err(), "missing credential field: {key}");
    }
}

#[test]
fn unknown_and_inline_secret_fields_are_rejected_without_echoing_values() {
    for section in [
        "postgres",
        "release",
        "provider",
        "budgets",
        "timeouts",
        "retrieval",
        "kernel",
    ] {
        for field in ["password", "api_key", "typo"] {
            let mut value = example();
            value[section][field] = json!("DO-NOT-LOG-synthetic-sensitive-value");
            let error = parse(&value).unwrap_err();
            assert!(!format!("{error:?} {error}").contains("DO-NOT-LOG"));
        }
    }
    let mut value = example();
    value["credentials"][0]["token"] = json!("DO-NOT-LOG-synthetic-sensitive-value");
    assert!(parse(&value).is_err());
    value = example();
    value["legacy_url"] = json!("http://127.0.0.1:9999");
    assert!(parse(&value).is_err());
}

#[test]
fn numeric_limits_and_pins_are_not_silently_clamped_or_defaulted() {
    for (section, field, invalid) in [
        ("postgres", "port", json!(0)),
        ("postgres", "socket_dir", json!("127.0.0.1")),
        ("postgres", "max_connections", json!(0)),
        ("release", "id", json!("current")),
        ("release", "id", json!("")),
        ("release", "knowledge_version", json!("latest")),
        ("provider", "model", json!("")),
        ("provider", "retry_attempts", json!(0)),
        ("provider", "retry_attempts", json!(9)),
        ("provider", "api_key_env", json!("bad-name")),
        ("provider", "max_response_bytes", json!(127)),
        ("provider", "kind", json!("legacy")),
        ("retrieval", "kind", json!("python")),
        ("retrieval", "limit", json!(0)),
        ("retrieval", "limit", json!(101)),
        ("kernel", "cache_entries", json!(1025)),
        ("kernel", "cache_ttl_ms", json!(60001)),
        ("budgets", "max_input_tokens", json!(0)),
        ("budgets", "max_output_tokens", json!(0)),
        ("budgets", "max_network_rounds", json!(0)),
        ("budgets", "max_cost_micros", json!(0)),
        ("timeouts", "startup_ms", json!(0)),
        ("timeouts", "request_ms", json!(60001)),
        ("timeouts", "provider_ms", json!(9000)),
        ("timeouts", "postgres_pool_wait_ms", json!(0)),
        ("timeouts", "postgres_pool_wait_ms", json!(8001)),
        ("timeouts", "postgres_lock_ms", json!(2001)),
        ("timeouts", "shutdown_ms", json!(7999)),
    ] {
        let mut value = example();
        value[section][field] = invalid;
        assert!(parse(&value).is_err(), "{section}.{field}");
    }
}

#[test]
fn credentials_require_explicit_valid_scopes_and_unambiguous_secret_references() {
    let mut value = example();
    value["credentials"] = json!([]);
    assert!(parse(&value).is_err());
    value = example();
    value["credentials"][0]["provider_egress"] = json!(["public", "everything"]);
    assert!(parse(&value).is_err());
    value = example();
    value["credentials"][0]["token_env"] = value["provider"]["api_key_env"].clone();
    assert!(parse(&value).is_err());
    value = example();
    value["postgres"]["auth"] = json!("password");
    assert!(parse(&value).is_err());
    value["postgres"]["password_env"] = json!("BRAIN_SERVE_PG_PASSWORD");
    assert!(parse(&value).is_ok());
    value["postgres"]["auth"] = json!("peer");
    assert!(parse(&value).is_err());
}

#[test]
fn provider_endpoint_is_explicit_https_or_loopback_and_priced() {
    for endpoint in [
        "http://provider.invalid/v1",
        "https://user:password@provider.invalid",
        "https://provider.invalid?token=synthetic",
        "https://provider.invalid/#synthetic",
        "not-a-url",
    ] {
        let mut value = example();
        value["provider"]["base_url"] = json!(endpoint);
        assert!(parse(&value).is_err());
    }
    let mut value = example();
    value["provider"]["base_url"] = json!("https://provider.invalid/v1");
    assert!(parse(&value).is_err());
    value["provider"]["pricing"] =
        json!({"input_micros_per_token": 1, "output_micros_per_token": 2});
    assert!(parse(&value).is_ok());
    value["provider"]["pricing"]["input_micros_per_token"] = json!(u64::MAX);
    assert!(parse(&value).is_err());
}

#[test]
fn analytics_window_and_schema_are_explicit_when_runtime_is_configured() {
    let mut value = example();
    value["analytics"] = json!({
        "patch": "2026-09-24",
        "min_unix_timestamp": 1790000000,
        "max_unix_timestamp": 1790086400,
        "max_rows": 8,
        "request_timeout_ms": 2000,
        "schema_sha256": dbrain_sources::schema_watch::OpenApiSnapshot::pinned().unwrap().schema_sha256,
    });
    assert!(parse(&value).is_ok());
    value["analytics"]["schema_sha256"] = json!("unreviewed");
    assert!(parse(&value).is_err());
    value["analytics"]["schema_sha256"] = json!(
        dbrain_sources::schema_watch::OpenApiSnapshot::pinned()
            .unwrap()
            .schema_sha256
    );
    value["analytics"]["max_unix_timestamp"] = json!(1790000000 + 32 * 86400);
    assert!(parse(&value).is_err());
    value["analytics"]["max_unix_timestamp"] = json!(1790086400);
    value["analytics"]["request_timeout_ms"] = json!(5000);
    assert!(parse(&value).is_err());
    value["analytics"]["request_timeout_ms"] = json!(2000);
    value["analytics"]["min_unix_timestamp"] = json!(3601);
    value["analytics"]["max_unix_timestamp"] = json!(7199);
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("analytics-empty-window.toml");
    std::fs::write(&path, config_bytes(&value)).unwrap();
    assert!(matches!(
        Config::load(&path),
        Err(Error::ConfigInvalid("analytics"))
    ));
}

#[test]
fn missing_malformed_and_oversized_config_are_sanitized() {
    let dir = tempfile::tempdir().unwrap();
    assert_eq!(
        Config::load(&dir.path().join("missing")).unwrap_err(),
        Error::ConfigMissing
    );
    for bytes in [b"DO-NOT-LOG".to_vec(), vec![b' '; 64 * 1024 + 1]] {
        let error = Config::parse(&bytes).unwrap_err();
        assert!(!format!("{error:?} {error}").contains("DO-NOT-LOG"));
    }
    assert_eq!(
        format!("{:?}", Config::parse(EXAMPLE).unwrap()),
        "Config { .. }"
    );
}

fn c9_config() -> serde_json::Value {
    let mut value = example();
    value["internal_operator"] = json!({"socket":"/run/user/1000/brain-operator/operator.sock", "release":{"id":"fixture-internal-release","knowledge_version":"fixture-internal-version"}});
    value["credentials"].as_array_mut().unwrap().push(
        json!({"token_env":"BRAIN_SERVE_DOCS_PUBLIC_TOKEN","actor_id":"docs-client","channel":"docs","scopes":["docs.public"],"provider_egress":["public"],"release":{"id":"fixture-docs-release","knowledge_version":"fixture-docs-version"}}));
    value["internal_operator"]["credential"] = json!({"token_env":"BRAIN_SERVE_SECOND_BRAIN_TOKEN","actor_id":"second-brain","channel":"internal","scopes":["second_brain.internal"],"provider_egress":[],"release":{"id":"fixture-internal-release","knowledge_version":"fixture-internal-version"}});
    value
}

#[test]
fn c9_grants_sind_exakt_und_releasebindungen_werden_beim_neustart_geprueft() {
    let value = c9_config();
    assert!(parse(&value).is_ok());
    for (index, field, bad) in [
        (1, "actor_id", json!("second-brain")),
        (1, "channel", json!("twitch")),
        (1, "scopes", json!(["docs.public", "bot.public"])),
        (1, "release", json!(null)),
        (2, "actor_id", json!("docs-client")),
        (2, "channel", json!("docs")),
        (2, "scopes", json!(["second_brain.internal", "docs.public"])),
        (2, "provider_egress", json!(["public"])),
        (
            2,
            "release",
            json!({"id":"wrong-release","knowledge_version":"fixture-internal-version"}),
        ),
    ] {
        let mut changed = value.clone();
        if index == 2 {
            changed["internal_operator"]["credential"][field] = bad;
        } else {
            changed["credentials"][index][field] = bad;
        }
        assert!(parse(&changed).is_err());
    }
    let mut changed = value.clone();
    changed["internal_operator"]["credential"] = json!(null);
    assert!(parse(&changed).is_err());
    let mut changed = value;
    changed["internal_operator"]["release"]["knowledge_version"] = json!("wrong-version");
    assert!(parse(&changed).is_err());
}

#[test]
fn second_brain_credential_ist_in_der_oeffentlichen_registry_unbekannt() {
    let config = parse(&c9_config()).unwrap();
    let secrets = crate::Secrets::load(&config, |name| Some(format!("synthetic-{name}"))).unwrap();
    let token = "synthetic-BRAIN_SERVE_SECOND_BRAIN_TOKEN";
    assert!(secrets.credentials.authenticate(token).is_err());
    let principal = secrets.internal_credentials.authenticate(token).unwrap();
    assert_eq!(principal.actor_id, "second-brain");
    assert_eq!(principal.channel, "internal");
    assert!(secrets
        .internal_credentials
        .authenticate("synthetic-BRAIN_SERVE_DOCS_PUBLIC_TOKEN")
        .is_err());
    assert!(secrets
        .credentials
        .authenticate("synthetic-BRAIN_SERVE_DOCS_PUBLIC_TOKEN")
        .is_ok());
}
