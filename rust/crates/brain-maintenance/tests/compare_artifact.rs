use brain_contracts::{
    CorpusRelease, CorpusSnapshot, DocumentHead, DocumentRevision, PortError, Principal,
    SnapshotReadPort, SourceRecordV2, SourceVisibility,
};
use brain_maintenance::{
    compare_artifact::{prepare_compare_artifact, render_stored_compare},
    hero_compare_render::{
        BoonValue, CompareBinding, CompareMetric, CompareSource, DisplayValue, HeroCompareInput,
        HeroCompareSeries, PublicationStatus, VersionBinding,
    },
};
use brain_storage::compare_artifact::{compare_dependency, compare_sha256, CompareArtifact};
use std::collections::{BTreeMap, BTreeSet};

struct Fixture {
    record: SourceRecordV2,
}
impl SnapshotReadPort for Fixture {
    fn read_snapshot(&self, _: &str) -> Result<CorpusSnapshot, PortError> {
        Ok(CorpusSnapshot {
            release: CorpusRelease {
                release_id: "fixture-release".into(),
                knowledge_version: "fixture-client".into(),
                patch: "fixture".into(),
                created_at_epoch: 1,
                source_revisions: BTreeMap::from([(
                    "fixture".into(),
                    BTreeMap::from([("mechanism".into(), 1)]),
                )]),
            },
            revisions: vec![self.record.clone()],
            heads: vec![self.record.clone()],
        })
    }
    fn read_heads(&self, _: &[DocumentRevision]) -> Result<Vec<DocumentHead>, PortError> {
        Ok(vec![DocumentHead::from(&self.record)])
    }
}

fn fixture() -> (Fixture, HeroCompareInput, Principal) {
    let record = SourceRecordV2 {
        source_id: "fixture".into(),
        logical_id: "mechanism".into(),
        revision: 1,
        content_hash: compare_sha256(b"fixture"),
        content: "fixture".into(),
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::new(),
        tombstone: false,
        valid_from: None,
        valid_to: None,
        metadata: BTreeMap::new(),
    };
    let binding = CompareBinding {
        result_id: "fixture-result".into(),
        version: VersionBinding {
            snapshot_id: "fixture-release".into(),
            client_version: "fixture-client".into(),
        },
        conditions: vec!["Strukturprobe ohne Spielaussage".into()],
    };
    let hero = |id: &str, name: &str, values: [f64; 2]| HeroCompareSeries {
        hero_id: id.into(),
        hero_name: name.into(),
        publication: PublicationStatus::PublicApproved,
        binding: binding.clone(),
        metric: CompareMetric::BaseDps,
        source_ids: vec!["fixture/mechanism".into()],
        values: [0, 1]
            .into_iter()
            .zip(values)
            .map(|(boon, value)| BoonValue {
                boon,
                value: DisplayValue::Quantified(value),
            })
            .collect(),
    };
    let heroes = [
        hero("a", "Testheld A", [10.0, 20.0]),
        hero("b", "Testheld B", [15.0, 25.0]),
    ];
    let input = HeroCompareInput {
        sources: vec![CompareSource {
            source_id: "fixture/mechanism".into(),
            evidence: "Erfundene Strukturprobe".into(),
            version: binding.version.clone(),
            publication: PublicationStatus::PublicApproved,
        }],
        binding,
        publication: PublicationStatus::PublicApproved,
        metric: CompareMetric::BaseDps,
        valid_boon_states: vec![0, 1],
        heroes,
    };
    let principal = Principal {
        actor_id: "fixture".into(),
        channel: "fixture".into(),
        scopes: BTreeSet::new(),
        provider_egress: BTreeSet::new(),
    };
    (Fixture { record }, input, principal)
}

#[test]
fn pending_artifact_replays_exact_html_and_svg_and_rejects_changed_output() {
    let (reader, input, principal) = fixture();
    let document = compare_dependency(&reader.record).unwrap().document;
    let artifact = prepare_compare_artifact(
        &reader,
        &principal,
        &input,
        &[document],
        serde_json::json!({"fixture":true,"scenario":{"boons":[0,1]}}),
        "fixture-mechanism-v1".into(),
    )
    .unwrap();
    let rendered = render_stored_compare(&artifact).unwrap();
    assert_eq!(rendered.html, artifact.body().html);
    assert_eq!(rendered.svg, artifact.body().svg);
    let mut changed = artifact.body().clone();
    changed.svg.push('x');
    assert!(render_stored_compare(&CompareArtifact::pending(changed).unwrap()).is_err());
    let mut changed = artifact.body().clone();
    changed.render_model["heroes"][0]["values"][0] = serde_json::json!(99.0);
    assert!(render_stored_compare(&CompareArtifact::pending(changed).unwrap()).is_err());
}

#[test]
fn renderer_approval_cannot_replace_canonical_public_rights() {
    let (mut reader, input, principal) = fixture();
    let document = compare_dependency(&reader.record).unwrap().document;
    reader.record.visibility = SourceVisibility::Internal;
    assert!(prepare_compare_artifact(
        &reader,
        &principal,
        &input,
        &[document],
        serde_json::json!({"fixture":true}),
        "fixture".into()
    )
    .is_err());
}
