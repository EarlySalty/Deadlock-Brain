//! Reviewer regression probes for PR #61. Constructor only; no network requests.
use brain_feeds::build_publish::HttpBuildPublishClient;
use std::time::Duration;

#[test]
fn publish_endpoint_must_not_treat_url_userinfo_as_loopback() {
    let endpoint = "http://127.0.0.1:1234@example.invalid";
    let parsed = reqwest::Url::parse(endpoint).unwrap();
    assert_eq!(parsed.host_str(), Some("example.invalid"));
    assert_eq!(parsed.scheme(), "http");
    const ENV: &str = "PR61_REVIEW_SYNTHETIC_PUBLISH_TOKEN";
    std::env::set_var(ENV, "synthetic-review-token-not-a-secret");
    let accepted = HttpBuildPublishClient::new(endpoint, ENV, Duration::from_secs(1)).is_ok();
    std::env::remove_var(ENV);
    assert!(!accepted, "userinfo prefix accepted as loopback even though the actual host is remote and transport is plaintext");
}
