use dbrain_sources::{
    assets_api::prepare_assets,
    deadlock_api::prepare_match_response,
    external::{group_provenance, sha256},
    schema_watch::{
        asset_dependencies, Compatibility, OpenApiSnapshot, SchemaDependency, OPENAPI_URL,
    },
};
use deadlock_brain_core::http::{HttpClient, SourceHttpOptions, SourceHttpResponse};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

fn response(raw: &[u8]) -> SourceHttpResponse {
    SourceHttpResponse {
        url: "https://fixture.invalid/owned-contract".into(),
        status: 200,
        content: raw.into(),
        headers: BTreeMap::from([("content-type".into(), "application/json".into())]),
        observed_at: 1,
        attempts: 1,
    }
}
fn spec(version: &str, schema: Value) -> OpenApiSnapshot {
    OpenApiSnapshot::from_raw(json!({"openapi":"3.1.0","info":{"version":version},"paths":{"/items":{"get":{"responses":{"200":{"content":{"application/json":{"schema":schema}}}}}}}}).to_string().into_bytes()).unwrap()
}
fn dependency() -> Vec<SchemaDependency> {
    vec![SchemaDependency {
        id: "items".into(),
        path: "/items".into(),
        method: "get".into(),
    }]
}

#[test]
fn match_api_rejects_malformed_missing_ambiguous_and_duplicate_without_losing_raw() {
    for raw in [
        b"{broken".as_slice(),
        b"{}",
        b"[null]",
        b"[{\"match_id\":false}]",
        b"[{\"match_id\":false,\"matchId\":1}]",
        b"[{\"match_id\":1,\"matchId\":2}]",
        b"[{\"match_id\":1},{\"match_id\":1}]",
        b"[{\"match_id\":1,\"players\":{}}]",
    ] {
        let ir = prepare_match_response(response(raw), false).unwrap();
        assert!(ir.is_quarantined(), "{raw:?}");
        assert_eq!(ir.raw(), raw);
        assert!(ir.payload().is_err());
        assert_eq!(ir.provenance().raw_sha256, sha256(raw));
    }
    let raw = b" [ {\"match_id\":0, \"hero_id\":1, \"extra\":true} ]\n";
    let ir = prepare_match_response(response(raw), true).unwrap();
    assert!(!ir.is_quarantined());
    assert_eq!(ir.raw(), raw);
    assert!(
        prepare_match_response(response(b"[{\"match_id\":1}]"), true)
            .unwrap()
            .is_quarantined()
    );
}

#[test]
fn status_type_encoding_and_schema_version_gate_preserve_quarantine() {
    let raw = b"[{\"id\":1}]";
    let valid = prepare_assets("items", response(raw), None).unwrap();
    let mut bad = response(raw);
    bad.status = 429;
    let bad = prepare_assets("items", bad, None).unwrap();
    assert!(bad.is_quarantined());
    assert_ne!(valid.document_key("items"), bad.document_key("items"));
    for (header, value) in [("content-type", "text/html"), ("content-encoding", "gzip")] {
        let mut r = response(raw);
        r.headers.insert(header.into(), value.into());
        assert!(prepare_assets("items", r, None).unwrap().is_quarantined());
    }
    let schema = json!({"type":"array","items":{"type":"object","required":["id"],"properties":{"id":{"type":"integer"}}}});
    let report = spec("1", schema.clone())
        .compare(&spec("2", schema), &dependency())
        .unwrap();
    let ir = prepare_assets("items", response(raw), Some(&report)).unwrap();
    assert!(ir.is_quarantined());
    assert_eq!(ir.raw(), raw);
    assert_eq!(
        report.quarantined_dependencies,
        BTreeSet::from(["items".into()])
    );
    assert_eq!(
        ir.contract().data.schema_version,
        brain_contracts::value::Observed::known("2".into())
    );
    assert_eq!(
        ir.provenance().source_revision,
        dbrain_sources::external::SourceRevision::Http {
            body_sha256: sha256(raw),
            etag: None,
            last_modified: None
        }
    );
    assert_ne!(ir.provenance().parser_revision, "2");
}

#[test]
fn removing_enum_is_breaking_and_missing_route_fails_closed() {
    let schema = json!({"type":"string","enum":["a","b"]});
    let a = spec("1", schema);
    let widened = spec("1", json!({"type":"string"}));
    assert_eq!(
        a.compare(&widened, &dependency()).unwrap().classification,
        Compatibility::Breaking
    );
    let removed = OpenApiSnapshot::from_raw(
        br#"{"openapi":"3.1.0","info":{"version":"1"},"paths":{}}"#.to_vec(),
    )
    .unwrap();
    assert!(a
        .compare(&removed, &dependency())
        .unwrap()
        .quarantined_dependencies
        .contains("items"));
}

#[test]
fn same_derivation_merges_new_origin_evidence_without_an_extra_vote() {
    let mut a = prepare_assets("items", response(b"[{\"id\":1}]"), None).unwrap();
    a.add_origin("origin:a").unwrap();
    let mut again = a.clone();
    again.add_origin("origin:b").unwrap();
    let mut mirror = prepare_assets("items", response(b"[{\"id\":2}]"), None).unwrap();
    mirror.add_origin("origin:b").unwrap();
    // Remove family correlation to make the assertion depend on the new origin.
    a.set_derivation_family("");
    again.set_derivation_family("");
    mirror.set_derivation_family("");
    let groups = group_provenance(&[
        a.provenance().clone(),
        again.provenance().clone(),
        mirror.provenance().clone(),
    ]);
    assert_eq!(groups.correlated_groups.len(), 1);
    assert_eq!(groups.correlated_groups[0].len(), 2);
}

#[test]
fn fixture_manifest_is_exact_and_current_paths_are_contract_covered() {
    let manifest: Value =
        serde_json::from_slice(include_bytes!("fixtures/external/manifest.json")).unwrap();
    let snapshot = OpenApiSnapshot::pinned().unwrap();
    assert_eq!(manifest["schema"]["raw_sha256"], snapshot.raw_sha256);
    assert_eq!(manifest["schema"]["bytes"], snapshot.raw().len());
    let report = snapshot.compare(&snapshot, &asset_dependencies()).unwrap();
    assert_eq!(report.classification, Compatibility::Unchanged);
    assert!(report.quarantined_dependencies.is_empty());
}

/// Explicit opt-in, TWO GETs maximum and no retries: schema <=1 MiB and
/// colors <=64 KiB. No DB, players, matches, repositories, exports or jobs.
#[test]
#[ignore = "explicit small public contract test; never part of deterministic CI"]
fn live_small_current_assets_contract() {
    assert_eq!(
        std::env::var("DBRAIN_EXTERNAL_LIVE_CONTRACT").as_deref(),
        Ok("1")
    );
    let temp = tempfile::tempdir().unwrap();
    let http = HttpClient::new("Deadlock-Brain-Contract/1", temp.path()).unwrap();
    let schema_response = http
        .get_bounded(
            OPENAPI_URL,
            SourceHttpOptions {
                max_bytes: 1024 * 1024,
                attempts: 1,
                ..Default::default()
            },
        )
        .unwrap();
    schema_response.ensure_success().unwrap();
    let schema = OpenApiSnapshot::from_raw(schema_response.content).unwrap();
    let dependencies = vec![SchemaDependency {
        id: "colors".into(),
        path: "/v1/assets/colors".into(),
        method: "get".into(),
    }];
    let report = OpenApiSnapshot::pinned()
        .unwrap()
        .compare(&schema, &dependencies)
        .unwrap();
    let response = http
        .get_bounded(
            "https://api.deadlock-api.com/v1/assets/colors",
            SourceHttpOptions {
                max_bytes: 64 * 1024,
                attempts: 1,
                ..Default::default()
            },
        )
        .unwrap();
    let ir = prepare_assets("colors", response, Some(&report)).unwrap();
    println!("schema_bytes={} schema_raw_sha256={} schema_version={} api_version={} colors_bytes={} colors_raw_sha256={} validation={:?}",schema.raw().len(),schema.raw_sha256,schema.openapi_version,schema.api_version,ir.raw().len(),ir.provenance().raw_sha256,ir.validation());
    assert!(
        !ir.is_quarantined(),
        "small public contract failed: {:?}",
        ir.validation()
    );
    assert!(ir.payload().unwrap().is_object());
}

#[test]
fn duplicate_json_keys_never_become_last_key_wins_evidence() {
    let raw = br#"[{"id":1,"id":2}]"#;
    let ir = prepare_assets("items", response(raw), None).unwrap();
    assert!(ir.is_quarantined());
    assert_eq!(ir.raw(), raw);
    let raw = br#"{"openapi":"3.1.0","info":{"version":"1","version":"2"},"paths":{}}"#;
    assert!(OpenApiSnapshot::from_raw(raw.to_vec()).is_err());
}
