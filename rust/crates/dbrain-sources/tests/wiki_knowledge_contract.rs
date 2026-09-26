//! Integration through the actual production source crate, with no DB/network.
use dbrain_sources::{
    core::http::HttpClient,
    wiki_capture::CaptureOptions,
    wiki_capture_io::{capture_with_http, WikiCaptureAccess},
    wiki_knowledge::{self, MappingProfile, ProjectionReview},
};
use serde_json::Value;

const CAPTURE: &[u8] =
    include_bytes!("../../../../architecture/migration/s12/fixtures/pilot.capture.json");
const MAPPING: &str =
    include_str!("../../../../architecture/migration/s12/fixtures/completion.mapping.json");

#[test]
fn source_crate_projects_the_same_shared_card_with_exact_source_references() {
    let profile: MappingProfile = serde_json::from_str(MAPPING).unwrap();
    let ir = wiki_knowledge::extract(CAPTURE, &profile).unwrap();
    let release = serde_json::from_str(include_str!(
        "../../../../architecture/migration/s12/fixtures/completion-release.json"
    ))
    .unwrap();
    let review: ProjectionReview = serde_json::from_str(include_str!(
        "../../../../architecture/migration/s12/fixtures/completion-review.json"
    ))
    .unwrap();
    let projected = wiki_knowledge::project_card(&ir, 101, "de", &release, &review).unwrap();
    let golden: Value = serde_json::from_str(include_str!(
        "../../../../architecture/migration/s12/fixtures/completion-card.golden.json"
    ))
    .unwrap();
    assert_eq!(serde_json::to_value(projected).unwrap(), golden);
    assert_eq!(
        ir.sources()
            .iter()
            .filter(|s| s.logical_id.ends_with(":101"))
            .count(),
        2
    );
}

#[test]
fn network_adapter_defaults_to_disabled_and_cannot_enable_itself_from_capture_policy() {
    let profile: MappingProfile = serde_json::from_str(MAPPING).unwrap();
    let ir = wiki_knowledge::extract(CAPTURE, &profile).unwrap();
    let temp = tempfile::tempdir().unwrap();
    let http = HttpClient::new("offline-wiki-contract-test", temp.path()).unwrap();
    let options = CaptureOptions {
        source_key: ir.report().source_key.clone(),
        retrieved_at: ir.report().retrieved_at,
        policy: ir.report().source_policy.clone(),
        scope: dbrain_s12_wiki_probe::model::WikiScope {
            namespace_allowlist: [0].into(),
            page_ids: [101].into(),
            pages: Default::default(),
            categories: Default::default(),
            heroes: Vec::new(),
        },
        max_requests: 1,
        max_pages: 1,
        max_total_bytes: 1024,
        request_interval_ms: 5_000,
        deadline_ms: 10_000,
    };
    let result = capture_with_http(&http, &WikiCaptureAccess::default(), &options);
    assert!(result.unwrap_err().to_string().contains("disabled"));
    assert_eq!(std::fs::read_dir(temp.path()).unwrap().count(), 0);
}
