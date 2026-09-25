//! Reproducible SYNTHETIC test artifact generator. Never approves live input.
//! Inputs are compiled-in repository fixtures; no external files/network accepted.
use brain_contracts::{CorpusRelease, DocumentRevision};
use dbrain_s12_wiki_probe::knowledge::{extract, project_card, MappingProfile, ProjectionReview};
use std::collections::BTreeMap;
fn main() {
    let mapping: MappingProfile =
        serde_json::from_str(include_str!("../fixtures/completion.mapping.json")).unwrap();
    let ir = extract(include_bytes!("../fixtures/pilot.capture.json"), &mapping).unwrap();
    let release = CorpusRelease {
        release_id: "fixture-release-one".into(),
        knowledge_version: "fixture-knowledge-v1".into(),
        patch: "unknown".into(),
        created_at_epoch: ir.report().retrieved_at,
        source_revisions: BTreeMap::from([(
            ir.report().source_key.clone(),
            ir.report()
                .pages
                .iter()
                .filter_map(|p| Some((p.source_id.clone(), p.latest_revision? as u64)))
                .collect(),
        )]),
    };
    let review = ProjectionReview {
        decision_ref: "synthetic-test-review-not-production".into(),
        approved_fields: ir
            .fields()
            .iter()
            .map(|f| (f.id.clone(), f.source_revision.clone()))
            .collect(),
        source_revisions: ir
            .report()
            .pages
            .iter()
            .filter_map(|p| {
                let r = p.latest()?;
                Some((
                    p.source_id.clone(),
                    DocumentRevision {
                        logical_id: p.source_id.clone(),
                        source_id: ir.report().source_key.clone(),
                        revision: r.revision_id as u64,
                        content_hash: r.raw_sha256.clone()?,
                    },
                ))
            })
            .collect(),
    };
    let projection = project_card(&ir, 101, "de", &release, &review).unwrap();
    println!(
        "{}",
        serde_json::to_string_pretty(
            &serde_json::json!({"release":release,"review":review,"projection":projection})
        )
        .unwrap()
    );
}
