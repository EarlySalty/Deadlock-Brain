//! Offline contract regressions. All HTTP fixtures listen on loopback only.
use brain_client::{AsyncBrainClient, ClientError};
use brain_contracts::{AnswerProfile, Query};
use std::{collections::BTreeSet, time::Duration};

fn query() -> Query {
    Query {
        request_id: "security-1".into(),
        conversation_id: "fixture-1".into(),
        text: "Abrams".into(),
        requested_scopes: BTreeSet::new(),
        profile: AnswerProfile::Explain,
        patch: None,
        mode: None,
    }
}

#[test]
fn rejects_unsafe_endpoints_before_network_access() {
    for endpoint in [
        "http://example.invalid",
        "http://localhost.example.invalid",
        "https://user:secret@example.invalid",
        "https://example.invalid?token=secret",
        "https://example.invalid#secret",
        "file:///etc/passwd",
    ] {
        assert!(matches!(
            AsyncBrainClient::new(endpoint, "fixture-token", Duration::from_secs(1)),
            Err(ClientError::InvalidBaseUrl)
        ));
    }
}

#[test]
fn local_consumers_do_not_treat_https_as_egress_permission() {
    for endpoint in [
        "https://example.invalid",
        "https://localhost.example.invalid",
        "https://127.0.0.1.example.invalid",
    ] {
        assert!(
            AsyncBrainClient::new_local(endpoint, "fixture-token", Duration::from_secs(1)).is_err()
        );
    }
    for endpoint in [
        "http://127.0.0.1:1",
        "https://localhost:1",
        "http://[::1]:1",
    ] {
        assert!(
            AsyncBrainClient::new_local(endpoint, "fixture-token", Duration::from_secs(1)).is_ok()
        );
    }
}

#[test]
fn rejects_invalid_credentials_without_logging_them() {
    for token in ["", "two words", "line\nbreak", "tab\ttoken"] {
        assert!(matches!(
            AsyncBrainClient::new("http://127.0.0.1:1", token, Duration::from_secs(1)),
            Err(ClientError::InvalidToken)
        ));
    }
    let client = AsyncBrainClient::new(
        "http://127.0.0.1:1",
        "secret-fixture-value",
        Duration::from_secs(1),
    )
    .unwrap();
    assert!(!format!("{client:?}").contains("secret-fixture-value"));
}

#[tokio::test]
async fn invalid_request_is_rejected_without_contacting_an_endpoint() {
    let client = AsyncBrainClient::new(
        "http://127.0.0.1:1",
        "fixture-token",
        Duration::from_secs(1),
    )
    .unwrap();
    let mut q = query();
    q.text.clear();
    assert!(matches!(
        client.answer(&q).await,
        Err(ClientError::InvalidQuery(_))
    ));
}
