use brain_contracts::{
    external::ExternalSourceIr,
    source::Versioned,
    value::{Observed, UnknownReason},
};
use dbrain_sources::external::{sha256, SourceIr, SourceRevision};
use serde_json::json;

fn source(raw: &[u8], parser: &str) -> SourceIr {
    SourceIr::from_json(
        "fixture",
        "fixture://payload",
        parser,
        SourceRevision::Http {
            body_sha256: sha256(raw),
            etag: Some("original-etag".into()),
            last_modified: None,
        },
        123,
        raw.to_vec(),
    )
    .unwrap()
}
fn restored(ir: &SourceIr) -> Versioned<ExternalSourceIr> {
    serde_json::from_value(ir.metadata()["contract"].clone()).unwrap()
}
#[test]
fn source_parser_persists_the_shared_versioned_contract() {
    let raw = br#"{"a/b~c":null,"zero":0,"future":{"field":5}}"#;
    let mut ir = source(raw, "parser:1");
    ir.add_origin("game-data:commit:artifact").unwrap();
    ir.pin_schema(&"a".repeat(64));
    ir.pin_schema_version("api:0.1.0").unwrap();
    let shared = restored(&ir);
    assert_eq!(shared.data, *ir.contract().data);
    assert_eq!(ir.raw(), raw);
    assert_eq!(shared.data.provenance.raw_sha256, sha256(raw));
    assert_eq!(
        shared.data.field("/a~1b~0c"),
        Observed::unknown(UnknownReason::ExplicitNull)
    );
    assert_eq!(
        shared.data.field("/absent"),
        Observed::unknown(UnknownReason::NotPresent)
    );
    assert_eq!(shared.data.field("/zero"), Observed::known(json!(0)));
    assert!(shared.data.field_provenance.contains_key("/future/field"));
    assert_eq!(
        shared.data.origin_artifact().policy.allowed_scopes,
        shared.data.allowed_scopes
    );
    assert!(!shared.data.origin_artifact().policy.provider_egress_allowed);
}
#[test]
fn reprocessing_separates_parser_schema_and_original_source_revision() {
    let a = source(b"{}", "parser:1");
    let mut b = source(b"{}", "parser:2");
    assert_eq!(
        a.provenance().source_revision,
        b.provenance().source_revision
    );
    assert_ne!(a.document_key("record"), b.document_key("record"));
    let before = b.document_key("record");
    b.pin_schema_version("API:2").unwrap();
    assert_ne!(before, b.document_key("record"));
    assert_eq!(
        a.provenance().source_revision,
        b.provenance().source_revision
    );
    assert!(matches!(
        restored(&b).data.origin_artifact().validity.patch,
        Observed::Unknown { .. }
    ));
    let c = SourceIr::from_json(
        "fixture",
        "fixture://payload",
        "parser:2",
        SourceRevision::Git {
            commit: "b".repeat(40),
        },
        123,
        b"{}".to_vec(),
    )
    .unwrap();
    assert_ne!(
        c.provenance().source_revision,
        b.provenance().source_revision
    );
    assert_ne!(c.document_key("record"), b.document_key("record"));
}
#[test]
fn quarantine_and_literal_null_roundtrip_without_fabricated_values() {
    let null = source(b"null", "parser:1");
    assert_eq!(
        restored(&null).data.payload,
        Observed::known(serde_json::Value::Null)
    );
    for bytes in [b"{broken".as_slice(), b"\xff"] {
        let ir = source(bytes, "parser:1");
        let shared = restored(&ir);
        assert_eq!(ir.raw(), bytes);
        assert!(ir.payload().is_err());
        assert_eq!(
            shared.data.field("/health"),
            Observed::unknown(UnknownReason::Quarantined)
        );
        assert_eq!(shared.data.provenance.raw_sha256, sha256(bytes));
    }
    let mut drifted = source(b"{\"health\":0}", "parser:1");
    let normalized = drifted.metadata()["normalized_sha256"].clone();
    drifted.quarantine("new_schema");
    assert_eq!(drifted.metadata()["normalized_sha256"], normalized);
    assert_eq!(
        restored(&drifted).data.field("/health"),
        Observed::unknown(UnknownReason::Quarantined)
    );
}
