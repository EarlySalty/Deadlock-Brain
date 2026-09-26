use dbrain_s12_wiki_probe::knowledge::{extract, project_card, MappingProfile, ProjectionReview};
use serde_json::{json, Value};
#[test]
fn an_unbound_alias_cannot_escape_the_release_and_dependency_review() {
    let mut capture: Value = serde_json::from_str(include_str!(
        "../../../../architecture/migration/s12/fixtures/pilot.capture.json"
    ))
    .unwrap();
    for binding in capture["hero_bindings"].as_array_mut().unwrap() {
        if binding["hero_page_id"] == 101 {
            binding["alias_page_ids"] = json!([]);
        }
    }
    let mapping: MappingProfile = serde_json::from_str(include_str!(
        "../../../../architecture/migration/s12/fixtures/completion.mapping.json"
    ))
    .unwrap();
    let ir = extract(&serde_json::to_vec(&capture).unwrap(), &mapping).unwrap();
    assert_eq!(
        ir.aliases().len(),
        1,
        "discovery retains the alias as source evidence"
    );
    let release = serde_json::from_str(include_str!(
        "../../../../architecture/migration/s12/fixtures/completion-release.json"
    ))
    .unwrap();
    let review: ProjectionReview = serde_json::from_str(include_str!(
        "../../../../architecture/migration/s12/fixtures/completion-review.json"
    ))
    .unwrap();
    let projected = project_card(&ir, 101, "de", &release, &review).unwrap();
    assert!(
        projected.aliases.is_empty(),
        "card only contains aliases from its pinned source closure"
    );
    assert!(!projected
        .dependencies
        .iter()
        .any(|r| r.logical_id.ends_with(":901")));
}
