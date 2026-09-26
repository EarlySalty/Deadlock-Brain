use brain_contracts::{
    source::{origin_from_record, Versioned},
    value::Observed,
    wiki,
};
use dbrain_s12_wiki_probe::knowledge::{extract, MappingProfile};

const CAPTURE: &[u8] =
    include_bytes!("../../../../architecture/migration/s12/fixtures/pilot.capture.json");
const MAPPING: &str =
    include_str!("../../../../architecture/migration/s12/fixtures/completion.mapping.json");

#[test]
fn actual_extractor_exports_shared_ir_with_exact_sources_and_policy() {
    let mapping: MappingProfile = serde_json::from_str(MAPPING).unwrap();
    let ir = extract(CAPTURE, &mapping).unwrap();
    let decoded: Versioned<wiki::WikiIr> =
        serde_json::from_slice(&serde_json::to_vec(ir.contract()).unwrap()).unwrap();
    assert_eq!(&decoded, ir.contract());
    assert_eq!(decoded.data.sources, ir.sources());
    assert_eq!(decoded.data.fields, ir.fields());
    assert!(!decoded.data.sources.is_empty());
    assert!(!decoded.data.dependencies.is_empty());
    for source in &decoded.data.sources {
        let origin = origin_from_record(source).unwrap();
        assert_eq!(origin.policy.visibility, source.visibility);
        assert_eq!(origin.policy.allowed_scopes, source.allowed_scopes);
        assert_eq!(
            origin.policy.publication_allowed,
            ir.report().source_policy.publication_allowed
        );
        assert_eq!(
            origin.policy.provider_egress_allowed,
            ir.report().source_policy.provider_egress_allowed
        );
        assert_eq!(origin.parser_revision, ir.report().parser_version);
        assert!(matches!(origin.validity.patch, Observed::Unknown { .. }));
        assert!(matches!(origin.validity.mode, Observed::Unknown { .. }));
        assert!(decoded.data.artifacts.contains(&origin));
        // Offline review is not an explicit grant for indefinite raw retention.
        assert!(!origin.policy.raw_retention_allowed);
    }
}

#[test]
fn actual_extractor_keeps_conditional_fields_out_of_bare_fact_semantics() {
    let mapping: MappingProfile = serde_json::from_str(MAPPING).unwrap();
    let ir = extract(CAPTURE, &mapping).unwrap();
    let conditional: Vec<_> = ir
        .fields()
        .iter()
        .filter(|f| f.condition.is_some() || f.variant.is_some())
        .collect();
    assert!(!conditional.is_empty());
    for field in conditional {
        assert!(!field.is_unconditional_known());
        let restored: wiki::IrField =
            serde_json::from_slice(&serde_json::to_vec(field).unwrap()).unwrap();
        assert_eq!(&restored, field);
        assert!(!restored.locators.is_empty());
        assert_eq!(
            restored.locators[0].revision_id as u64,
            restored.source_revision.revision
        );
    }
}
