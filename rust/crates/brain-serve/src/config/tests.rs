use super::*;
use serde_json::{json, Value};
const EXAMPLE: &[u8] = include_bytes!("../../../../../config/brain-serve.example.json");
fn example() -> Value {
    serde_json::from_slice(EXAMPLE).unwrap()
}
fn parse(value: &Value) -> Result<Config, Error> {
    Config::parse(&serde_json::to_vec(value).unwrap())
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
