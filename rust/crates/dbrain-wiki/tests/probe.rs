use dbrain_s12_wiki_probe::{analyze, compare, model::*, parse_json};
use serde_json::{json, Value};
use std::collections::BTreeSet;

fn fixture() -> Value {
    serde_json::from_str(include_str!(
        "../../../../architecture/migration/s12/fixtures/pilot.capture.json"
    ))
    .unwrap()
}
fn inspect(value: &Value) -> Report {
    analyze(&serde_json::to_vec(value).unwrap()).unwrap()
}
fn fails(value: &Value) {
    assert!(analyze(&serde_json::to_vec(value).unwrap()).is_err());
}
fn capture_mut(value: &mut Value, id: i64) -> &mut Value {
    value["pages"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|p| p["page_id"] == id)
        .unwrap()
}
fn page_mut(value: &mut Value, id: i64) -> &mut Value {
    &mut capture_mut(value, id)["response"]["query"]["pages"][0]
}
fn revision_mut(value: &mut Value, id: i64) -> &mut Value {
    page_mut(value, id)["revisions"]
        .as_array_mut()
        .unwrap()
        .last_mut()
        .unwrap()
}
fn slot_mut(value: &mut Value, id: i64) -> &mut Value {
    &mut revision_mut(value, id)["slots"]["main"]
}
fn page(report: &Report, id: i64) -> &PageProbe {
    report.pages.iter().find(|p| p.page_id == id).unwrap()
}
fn ids(values: &[i64]) -> BTreeSet<i64> {
    values.iter().copied().collect()
}
fn content_change(value: &mut Value, id: i64) {
    let head = page_mut(value, id)["lastrevid"].as_i64().unwrap() + 100_000;
    page_mut(value, id)["lastrevid"] = json!(head);
    revision_mut(value, id)["revid"] = json!(head);
    slot_mut(value, id)["content"] = json!("{\"per_tick\":\"0.75\",\"unit\":\"percent\"}");
}
fn omit_page(value: &mut Value, id: i64) {
    value["pages"]
        .as_array_mut()
        .unwrap()
        .retain(|p| p["page_id"] != id);
    value["classification"]
        .as_array_mut()
        .unwrap()
        .retain(|p| p["page_id"] != id);
    for batch in value["discovery"].as_array_mut().unwrap() {
        batch["response"]["query"]["allpages"]
            .as_array_mut()
            .unwrap()
            .retain(|p| p["pageid"] != id);
    }
}

#[test]
fn pilot_covers_dynamic_namespaces_categories_and_revision_history() {
    let r = inspect(&fixture());
    assert!(r.discovery_complete);
    assert!(r.missing_namespaces.is_empty());
    assert_eq!(r.namespace_names[&3000], "Data");
    assert_eq!(r.pages.len(), 19); // fixture count only, never a live roster assertion
    assert_eq!(page(&r, 101).revisions.len(), 2);
    for kind in [
        PageKind::Hero,
        PageKind::Ability,
        PageKind::Item,
        PageKind::Mechanic,
        PageKind::Rule,
        PageKind::PatchHistory,
        PageKind::Data,
        PageKind::Template,
        PageKind::Module,
        PageKind::Lore,
        PageKind::Guide,
        PageKind::Localization,
        PageKind::Redirect,
        PageKind::MediaMetadata,
        PageKind::Other,
    ] {
        assert!(r.coverage[&kind].discovered > 0);
    }
}

#[test]
fn syntax_success_never_becomes_canonical_validation_or_publication() {
    let mut v = fixture();
    v["policy"]["publication_allowed"] = json!(true);
    let r = inspect(&v);
    assert!(!r.production_publishable && !r.integration_verified);
    assert_eq!(r.mode, "prepare_only");
    assert!(r
        .coverage
        .values()
        .all(|c| c.normalized == 0 && c.validated == 0 && c.published == 0));
    assert!(r.blockers.contains("G1_contracts_not_integrated"));
    assert!(r.card_previews.iter().all(|c| !c.publishable));
}

#[test]
fn preview_is_rebuildable_reference_only_and_has_no_invented_versions() {
    let a = inspect(&fixture());
    let b = inspect(&fixture());
    assert_eq!(a.card_previews, b.card_previews);
    let card = &a.card_previews[0];
    assert_eq!(card.locale, "de");
    assert!(
        card.patch.is_none() && card.mode.is_none() && card.canonical_knowledge_release.is_none()
    );
    assert!(card.unknowns.contains("build_legality_not_evaluated"));
    let expected = [
        ("profile", vec![101]),
        ("abilities", vec![201, 202]),
        ("mechanics", vec![401, 402]),
        ("items", vec![301, 302]),
        ("build_rules", vec![501]),
        ("aliases", vec![901]),
        (
            "dependency_evidence",
            vec![101, 201, 202, 301, 302, 401, 402, 501, 601, 701, 901],
        ),
    ];
    for (section, expected) in expected {
        assert_eq!(
            card.sections[section]
                .iter()
                .map(|r| r.source_id.clone())
                .collect::<Vec<_>>(),
            expected
                .iter()
                .map(|id| format!("synthetic_wiki:page:{id}"))
                .collect::<Vec<_>>()
        );
    }
    let wire = serde_json::to_value(card).unwrap();
    assert!(wire.get("stats").is_none() && wire.get("facts").is_none());
}

#[test]
fn all_preview_locators_and_hashes_are_grounded_in_exact_source_revisions() {
    let r = inspect(&fixture());
    for card in &r.card_previews {
        for reference in card.sections.values().flatten() {
            let p = r
                .pages
                .iter()
                .find(|p| p.source_id == reference.source_id)
                .unwrap();
            let rev = p
                .revisions
                .iter()
                .find(|v| v.revision_id == reference.revision_id)
                .unwrap();
            assert_eq!(Some(&reference.raw_sha256), rev.raw_sha256.as_ref());
            for locator in &reference.candidate_locators {
                assert!(rev.candidates.iter().any(|c| &c.locator == locator));
            }
        }
    }
}

#[test]
fn locale_never_creates_another_numeric_source() {
    let r = inspect(&fixture());
    assert_eq!(r.card_previews[0].sections, r.card_previews[1].sections);
    assert_eq!(
        r.card_previews[0].hero_source_id,
        r.card_previews[1].hero_source_id
    );
}

#[test]
fn null_missing_percent_percentage_point_and_multiplier_are_not_conflated() {
    let r = inspect(&fixture());
    let c = &page(&r, 101).latest().unwrap().candidates;
    for (locator, expected) in [
        ("json:/stats/1/unit", json!("percent")),
        ("json:/stats/2/unit", json!("percentage_point")),
        ("json:/stats/3/unit", json!("multiplier")),
        ("json:/optional_value", Value::Null),
        ("json:/stats/0/value", json!("1000.0")),
    ] {
        assert_eq!(
            c.iter().find(|c| c.locator == locator).unwrap().value,
            expected
        );
    }
    assert!(!c.iter().any(|c| c.locator == "json:/missing_stat"));
}

#[test]
fn condition_variant_and_mechanic_identity_survive_as_uninterpreted_candidates() {
    let r = inspect(&fixture());
    let c = &page(&r, 201).latest().unwrap().candidates;
    assert!(c
        .iter()
        .any(|c| c.locator == "json:/condition" && c.value == "while_airborne"));
    assert!(c
        .iter()
        .any(|c| c.locator == "json:/variant" && c.value == "charged"));
    assert_ne!(page(&r, 401).source_id, page(&r, 402).source_id);
}

#[test]
fn raw_json_pointers_escape_slashes_and_tildes_and_keep_false() {
    let mut v = fixture();
    slot_mut(&mut v, 101)["content"] = json!("{\"a/b\":{\"~key\":null},\"flag\":false}");
    let r = inspect(&v);
    let c = &page(&r, 101).latest().unwrap().candidates;
    assert!(c
        .iter()
        .any(|c| c.locator == "json:/a~1b/~0key" && c.value.is_null()));
    assert!(c
        .iter()
        .any(|c| c.locator == "json:/flag" && c.value == false));
}

#[test]
fn missing_dynamic_namespace_prevents_complete_inventory() {
    let mut v = fixture();
    v["siteinfo"]["query"]["namespaces"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":9876,"canonical":"Additional","name":"Additional"}));
    let r = inspect(&v);
    assert!(!r.discovery_complete);
    assert_eq!(r.missing_namespaces, ids(&[9876]));
}

#[test]
fn incomplete_continuation_is_never_a_complete_corpus() {
    let mut v = fixture();
    v["discovery"][1]["response"]["continue"] = json!({"apcontinue":"still-more","continue":"-||"});
    let r = inspect(&v);
    assert!(!r.discovery_complete);
    assert!(r.continuation_pending.contains_key(&0));
}

#[test]
fn continuation_must_preserve_the_entire_token() {
    let mut v = fixture();
    v["discovery"][1]["request_continue"]
        .as_object_mut()
        .unwrap()
        .remove("continue");
    fails(&v);
}

#[test]
fn repeated_continuation_is_rejected() {
    let mut v = fixture();
    v["discovery"][1]["response"]["continue"] = v["discovery"][0]["response"]["continue"].clone();
    fails(&v);
}

#[test]
fn empty_batch_with_continuation_is_supported() {
    let mut v = fixture();
    v["discovery"][0]["request_continue"] = json!({"apcontinue":"empty-first","continue":"-||"});
    v["discovery"].as_array_mut().unwrap().insert(0,json!({"namespace":0,"request_continue":{},
        "response":{"query":{"allpages":[]},"continue":{"apcontinue":"empty-first","continue":"-||"}}}));
    assert!(inspect(&v).discovery_complete);
}

#[test]
fn duplicate_discovered_pageids_are_not_silently_deduplicated() {
    let mut v = fixture();
    let duplicate = v["discovery"][0]["response"]["query"]["allpages"][0].clone();
    v["discovery"][0]["response"]["query"]["allpages"]
        .as_array_mut()
        .unwrap()
        .push(duplicate);
    fails(&v);
}

#[test]
fn discovery_api_errors_do_not_look_like_zero_pages() {
    let mut v = fixture();
    v["pages"] = json!([]);
    v["classification"] = json!([]);
    v["hero_bindings"] = json!([]);
    v["discovery"] =
        json!([{"namespace":0,"request_continue":{},"response":{"error":{"code":"maxlag"}}}]);
    let r = inspect(&v);
    assert!(!r.discovery_complete && !r.diagnostics.is_empty());
}

#[test]
fn siteinfo_api_error_is_not_a_namespace_manifest() {
    let mut v = fixture();
    v["siteinfo"]["error"] = json!({"code":"permissiondenied"});
    fails(&v);
}

#[test]
fn legacy_mediawiki_page_map_and_main_slot_star_are_supported() {
    let mut v = fixture();
    let raw = page_mut(&mut v, 101).clone();
    capture_mut(&mut v, 101)["response"]["query"]["pages"] = json!({"101":raw});
    let revisions = capture_mut(&mut v, 101)["response"]["query"]["pages"]["101"]["revisions"]
        .as_array_mut()
        .unwrap();
    for rev in revisions {
        let content = rev["slots"]["main"]
            .as_object_mut()
            .unwrap()
            .remove("content")
            .unwrap();
        rev["slots"]["main"]["*"] = content;
    }
    assert!(page(&inspect(&v), 101).previewable());
}

#[test]
fn wrong_fetched_page_identity_is_rejected() {
    let mut v = fixture();
    page_mut(&mut v, 101)["pageid"] = json!(9999);
    fails(&v);
}

#[test]
fn same_revision_is_idempotent() {
    let mut v = fixture();
    let duplicate = capture_mut(&mut v, 101).clone();
    v["pages"].as_array_mut().unwrap().push(duplicate);
    assert_eq!(page(&inspect(&v), 101).revisions.len(), 2);
}

#[test]
fn same_revision_with_different_content_is_quarantined() {
    let mut v = fixture();
    let mut duplicate = capture_mut(&mut v, 101).clone();
    duplicate["response"]["query"]["pages"][0]["revisions"][1]["slots"]["main"]["content"] =
        json!("{\"health\":\"1\"}");
    v["pages"].as_array_mut().unwrap().push(duplicate);
    assert_eq!(page(&inspect(&v), 101).stage, Stage::Quarantined);
}

#[test]
fn out_of_order_revision_array_preserves_head_and_history() {
    let mut v = fixture();
    page_mut(&mut v, 101)["revisions"]
        .as_array_mut()
        .unwrap()
        .reverse();
    let r = inspect(&v);
    assert_eq!(page(&r, 101).latest_revision, Some(10101));
    assert_eq!(page(&r, 101).revisions[0].revision_id, 9101);
}

#[test]
fn suppressed_latest_revision_never_falls_back_to_old_stats() {
    let mut v = fixture();
    revision_mut(&mut v, 101)["texthidden"] = json!(true);
    let r = inspect(&v);
    let latest = page(&r, 101).latest().unwrap();
    assert!(latest.raw_sha256.is_none() && latest.candidates.is_empty());
    assert_eq!(page(&r, 101).stage, Stage::Quarantined);
    assert!(r.card_previews[0].sections["profile"].is_empty());
}

#[test]
fn future_source_revision_is_quarantined_not_treated_as_current_patch() {
    let mut v = fixture();
    revision_mut(&mut v, 101)["timestamp"] = json!("2099-01-01T00:00:00Z");
    let r = inspect(&v);
    assert_eq!(page(&r, 101).stage, Stage::Quarantined);
    assert!(
        r.card_previews[0].patch.is_none() && r.card_previews[0].sections["profile"].is_empty()
    );
}

#[test]
fn invalid_source_time_does_not_get_replaced_by_fetch_time() {
    let mut v = fixture();
    revision_mut(&mut v, 101)["timestamp"] = json!("not-a-time");
    let r = inspect(&v);
    assert!(page(&r, 101).latest().unwrap().source_time.is_none());
    assert!(!page(&r, 101).previewable());
}

#[test]
fn missing_content_is_not_an_empty_success() {
    let mut v = fixture();
    slot_mut(&mut v, 101)
        .as_object_mut()
        .unwrap()
        .remove("content");
    assert_eq!(page(&inspect(&v), 101).stage, Stage::Quarantined);
}

#[test]
fn absent_advertised_head_is_not_invented_from_returned_revisions() {
    let mut v = fixture();
    page_mut(&mut v, 101)
        .as_object_mut()
        .unwrap()
        .remove("lastrevid");
    let r = inspect(&v);
    assert!(page(&r, 101).latest_revision.is_none());
    assert_eq!(page(&r, 101).stage, Stage::Quarantined);
}

#[test]
fn fetch_failure_and_missing_page_remain_visible() {
    for response in [
        json!({"error":{"code":"permissiondenied"}}),
        json!({"query":{"pages":{"-1":{"missing":"","title":"Fixture Ranger"}}}}),
    ] {
        let mut v = fixture();
        capture_mut(&mut v, 101)["response"] = response;
        assert_eq!(page(&inspect(&v), 101).stage, Stage::Unavailable);
    }
}

#[test]
fn duplicate_json_keys_rejected_at_capture_and_content_depth() {
    assert!(parse_json(br#"{"nested":{"access":true,"access":false}}"#).is_err());
    let mut v = fixture();
    slot_mut(&mut v, 101)["content"] = json!("{\"damage\":1,\"damage\":2}");
    assert_eq!(page(&inspect(&v), 101).stage, Stage::Quarantined);
}

#[test]
fn nonfinite_numbers_are_not_valid_json_candidates() {
    let mut v = fixture();
    slot_mut(&mut v, 101)["content"] = json!("{\"value\":NaN}");
    assert_eq!(page(&inspect(&v), 101).stage, Stage::Quarantined);
}

#[test]
fn lua_and_html_have_no_execution_or_silent_fallback() {
    let r = inspect(&fixture());
    assert_eq!(page(&r, 702).stage, Stage::Quarantined);
    assert!(page(&r, 702).latest().unwrap().candidates.is_empty());
    let mut v = fixture();
    slot_mut(&mut v, 101)["contentmodel"] = json!("html");
    slot_mut(&mut v, 101)["content"] = json!("<script>neverExecute()</script>");
    assert_eq!(page(&inspect(&v), 101).stage, Stage::Quarantined);
}

#[test]
fn unknown_wikitext_has_exact_utf8_diagnostic_and_no_numeric_inference() {
    let mut v = fixture();
    slot_mut(&mut v, 101)["contentmodel"] = json!("wikitext");
    slot_mut(&mut v, 101)["content"] = json!("ä{{Unknown|value=10%}}");
    let r = inspect(&v);
    let rev = page(&r, 101).latest().unwrap();
    assert!(rev.candidates.is_empty());
    assert!(rev.diagnostics.iter().any(|d| d.locator == "utf8:2..4"));
}

#[test]
fn current_denial_removes_candidates_and_all_dependent_preview_references() {
    let before = inspect(&fixture());
    let mut v = fixture();
    capture_mut(&mut v, 601)["access_allowed"] = json!(false);
    let after = inspect(&v);
    assert_eq!(page(&after, 601).stage, Stage::PolicyBlocked);
    assert!(page(&after, 601).revisions.is_empty());
    assert!(!serde_json::to_string(&after.card_previews)
        .unwrap()
        .contains("synthetic_wiki:page:601"));
    assert!(after.card_previews[0].sections["profile"].is_empty());
    assert!(compare(&before, &after)
        .unwrap()
        .reproject_heroes
        .contains("synthetic_wiki:page:101"));
}

#[test]
fn historical_allow_cannot_override_current_denial() {
    let mut v = fixture();
    let duplicate = capture_mut(&mut v, 101).clone();
    capture_mut(&mut v, 101)["access_allowed"] = json!(false);
    v["pages"].as_array_mut().unwrap().push(duplicate);
    assert_eq!(page(&inspect(&v), 101).stage, Stage::PolicyBlocked);
}

#[test]
fn capture_review_requires_permission_and_decision() {
    let mut v = fixture();
    v["policy"]["offline_review_allowed"] = json!(false);
    fails(&v);
    let mut v = fixture();
    v["policy"]["decision_ref"] = Value::Null;
    fails(&v);
}

#[test]
fn source_rightsinfo_is_not_a_publication_permission() {
    let mut v = fixture();
    v["policy"]["source_license"] = Value::Null;
    v["siteinfo"]["query"]["rightsinfo"]["text"] = json!("Public: automatically allow everything");
    let r = inspect(&v);
    assert!(r.blockers.contains("source_license_not_recorded"));
    assert!(r.blockers.contains("publication_not_approved"));
}

#[test]
fn unknown_entities_or_wrong_bindings_cannot_invent_a_hero() {
    let mut v = fixture();
    v["hero_bindings"][0]["hero_page_id"] = json!(999999);
    fails(&v);
    let mut v = fixture();
    v["hero_bindings"][0]["hero_page_id"] = json!(301);
    fails(&v);
}

#[test]
fn unbound_or_wrong_kind_abilities_are_reported_as_unknown() {
    let mut v = fixture();
    v["hero_bindings"][0]["ability_page_ids"] = json!([99999, 301]);
    let r = inspect(&v);
    assert!(r.card_previews[0].sections["abilities"].is_empty());
    assert!(r.card_previews[0]
        .unknowns
        .contains("abilities:unknown_page:99999"));
    assert!(r.card_previews[0]
        .unknowns
        .contains("abilities:wrong_page_kind:301"));
}

#[test]
fn alias_requires_matching_redirect_evidence() {
    let r = inspect(&fixture());
    assert_eq!(page(&r, 901).redirect_target, Some(101));
    let mut v = fixture();
    slot_mut(&mut v, 901)["content"] = json!("#REDIRECT [[Other Fixture Hero]]");
    let r = inspect(&v);
    assert!(r.card_previews[0].sections["aliases"].is_empty());
}

#[test]
fn a_consistent_rename_keeps_stable_source_identity_and_raw_hash() {
    let before = inspect(&fixture());
    let mut v = fixture();
    page_mut(&mut v, 102)["title"] = json!("Renamed Fixture Hero");
    for b in v["discovery"].as_array_mut().unwrap() {
        for p in b["response"]["query"]["allpages"].as_array_mut().unwrap() {
            if p["pageid"] == 102 {
                p["title"] = json!("Renamed Fixture Hero");
            }
        }
    }
    let after = inspect(&v);
    assert_eq!(page(&before, 102).source_id, page(&after, 102).source_id);
    let delta = compare(&before, &after).unwrap();
    assert_eq!(delta.changed_pages, ids(&[102]));
    assert!(delta.raw_content_changed.is_empty());
}

#[test]
fn a_move_during_capture_is_quarantined_until_reconcile() {
    let mut v = fixture();
    page_mut(&mut v, 101)["title"] = json!("Different title");
    assert_eq!(page(&inspect(&v), 101).stage, Stage::Quarantined);
}

#[test]
fn unchanged_content_and_new_fetch_time_do_not_require_reparsing() {
    let before = inspect(&fixture());
    let mut v = fixture();
    v["retrieved_at"] = json!(v["retrieved_at"].as_i64().unwrap() + 3600);
    let delta = compare(&before, &inspect(&v)).unwrap();
    assert!(delta.changed_pages.is_empty() && delta.reparse_pages.is_empty());
    assert!(delta.raw_content_changed.is_empty() && delta.reproject_heroes.is_empty());
    assert_eq!(delta.embedding_jobs_scheduled, 0);
}

#[test]
fn template_delta_selectively_invalidates_transitive_dependants() {
    let before = inspect(&fixture());
    let mut v = fixture();
    content_change(&mut v, 701);
    let after = inspect(&v);
    let delta = compare(&before, &after).unwrap();
    assert_eq!(delta.changed_pages, ids(&[701]));
    assert_eq!(delta.raw_content_changed, ids(&[701]));
    assert_eq!(delta.reparse_pages, ids(&[101, 201, 601, 701, 901]));
    assert_eq!(
        delta.reproject_heroes,
        BTreeSet::from(["synthetic_wiki:page:101".into()])
    );
    assert_ne!(
        before.card_previews[0].preview_sha256,
        after.card_previews[0].preview_sha256
    );
    assert!(!delta.full_reconcile_required && !delta.publication_performed);
}

#[test]
fn dependency_cycles_terminate_without_losing_transitive_invalidation() {
    let mut v = fixture();
    page_mut(&mut v, 701)["links"] = json!([{"title":"Data:Fixture Scaling"}]);
    let before = inspect(&v);
    content_change(&mut v, 701);
    let delta = compare(&before, &inspect(&v)).unwrap();
    assert_eq!(delta.reparse_pages, ids(&[101, 201, 601, 701, 901]));
}

#[test]
fn incomplete_dependencies_force_reconcile_not_optimistic_selective_delta() {
    let before = inspect(&fixture());
    let mut v = fixture();
    capture_mut(&mut v, 201)["dependencies_complete"] = json!(false);
    let after = inspect(&v);
    let delta = compare(&before, &after).unwrap();
    assert!(delta.full_reconcile_required);
    assert_eq!(delta.reparse_pages.len(), before.pages.len());
    assert!(after.card_previews[0].sections["profile"].is_empty());
}

#[test]
fn dependency_continuation_or_absent_arrays_cannot_be_declared_complete() {
    let mut v = fixture();
    capture_mut(&mut v, 201)["response"]["continue"] = json!({"tlcontinue":"more"});
    assert!(!page(&inspect(&v), 201).dependencies_complete);
    let mut v = fixture();
    page_mut(&mut v, 201)
        .as_object_mut()
        .unwrap()
        .remove("templates");
    assert!(!page(&inspect(&v), 201).dependencies_complete);
}

#[test]
fn unresolved_dependency_remains_a_visible_blocker() {
    let mut v = fixture();
    page_mut(&mut v, 201)["links"]
        .as_array_mut()
        .unwrap()
        .push(json!({"title":"Undiscovered:Dependency"}));
    let r = inspect(&v);
    assert!(!page(&r, 201).dependencies_complete);
    assert!(page(&r, 201)
        .unresolved_dependencies
        .contains("Undiscovered:Dependency"));
}

#[test]
fn deletion_is_only_inferred_from_complete_snapshots() {
    let before = inspect(&fixture());
    let mut v = fixture();
    omit_page(&mut v, 1001);
    let after = inspect(&v);
    assert_eq!(
        compare(&before, &after).unwrap().removed_pages,
        ids(&[1001])
    );
    v["discovery"]
        .as_array_mut()
        .unwrap()
        .retain(|b| b["namespace"] != 6);
    let delta = compare(&before, &inspect(&v)).unwrap();
    assert!(delta.removed_pages.is_empty() && delta.full_reconcile_required);
}

#[test]
fn removing_a_dependency_invalidates_its_users() {
    let before = inspect(&fixture());
    let mut v = fixture();
    omit_page(&mut v, 701);
    let after = inspect(&v);
    let delta = compare(&before, &after).unwrap();
    assert!(delta.removed_pages.contains(&701) && delta.reparse_pages.contains(&101));
    assert!(after.card_previews[0].sections["profile"].is_empty());
}

#[test]
fn stale_capture_and_stale_page_revision_are_rejected() {
    let before = inspect(&fixture());
    let mut after = before.clone();
    after.retrieved_at -= 1;
    assert!(compare(&before, &after).is_err());
    let mut v = fixture();
    let p = page_mut(&mut v, 101);
    p["revisions"].as_array_mut().unwrap().pop();
    p["lastrevid"] = json!(9101);
    assert!(compare(&before, &inspect(&v)).is_err());
}

#[test]
fn parser_change_reparses_without_changing_patch_or_raw_embedding_inputs() {
    let before = inspect(&fixture());
    let mut after = before.clone();
    after.parser_version = "next-test-parser".into();
    let delta = compare(&before, &after).unwrap();
    assert_eq!(delta.reparse_pages.len(), before.pages.len());
    assert!(delta.raw_content_changed.is_empty());
    assert!(after.card_previews.iter().all(|c| c.patch.is_none()));
}

#[test]
fn exclusions_are_explicit_and_cannot_hide_build_critical_pages() {
    let mut v = fixture();
    v["classification"][0]["approved_exclusion"] = json!("decision-1");
    fails(&v);
    let mut v = fixture();
    let c = v["classification"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|p| p["page_id"] == 1001)
        .unwrap();
    c["approved_exclusion"] = json!("media-metadata-only-scope-decision");
    let r = inspect(&v);
    assert_eq!(page(&r, 1001).stage, Stage::ApprovedExclusion);
    assert_eq!(r.coverage[&PageKind::MediaMetadata].approved_exclusion, 1);
}

#[test]
fn unclassified_pages_are_counted_not_silently_dropped() {
    let mut v = fixture();
    v["classification"]
        .as_array_mut()
        .unwrap()
        .retain(|p| p["page_id"] != 1002);
    let r = inspect(&v);
    assert_eq!(r.coverage[&PageKind::Unclassified].discovered, 1);
    assert!(r.blockers.contains("unclassified_pages"));
}

#[test]
fn input_content_and_candidate_budgets_fail_closed() {
    assert!(analyze(&vec![b' '; MAX_INPUT_BYTES + 1]).is_err());
    let mut v = fixture();
    slot_mut(&mut v, 101)["content"] = json!("x".repeat(MAX_CONTENT_BYTES + 1));
    assert_eq!(page(&inspect(&v), 101).stage, Stage::Quarantined);
    let mut v = fixture();
    slot_mut(&mut v, 101)["content"] =
        json!(serde_json::to_string(&vec![1; MAX_CANDIDATES + 1]).unwrap());
    let r = inspect(&v);
    assert!(page(&r, 101).latest().unwrap().candidates.is_empty());
    assert_eq!(page(&r, 101).stage, Stage::Quarantined);
}

#[test]
fn excessive_nesting_and_unknown_capture_fields_are_rejected() {
    let deep = format!("{}null{}", "[".repeat(200), "]".repeat(200));
    assert!(parse_json(deep.as_bytes()).is_err());
    let mut v = fixture();
    v["allow_live_import"] = json!(true);
    fails(&v);
}

#[test]
fn same_revision_cannot_change_raw_content_between_captures() {
    let before = inspect(&fixture());
    let mut v = fixture();
    slot_mut(&mut v, 701)["content"] = json!("{\"per_tick\":\"999\"}");
    assert!(compare(&before, &inspect(&v)).is_err());
}

#[test]
fn same_revision_cannot_change_source_time_between_captures() {
    let before = inspect(&fixture());
    let mut v = fixture();
    revision_mut(&mut v, 701)["timestamp"] = json!("2026-09-24T11:00:00Z");
    assert!(compare(&before, &inspect(&v)).is_err());
}

#[test]
fn report_format_mismatch_cannot_be_compared_as_compatible() {
    let before = inspect(&fixture());
    let mut after = before.clone();
    after.report_version = "unsupported-next-report".into();
    assert!(compare(&before, &after).is_err());
}

#[test]
fn a_new_revision_with_unchanged_raw_bytes_does_not_change_embedding_input() {
    let before = inspect(&fixture());
    let mut v = fixture();
    page_mut(&mut v, 701)["lastrevid"] = json!(110701);
    revision_mut(&mut v, 701)["revid"] = json!(110701);
    let delta = compare(&before, &inspect(&v)).unwrap();
    assert!(delta.changed_pages.contains(&701));
    assert!(delta.raw_content_changed.is_empty());
}

#[test]
fn hero_preview_matches_reviewed_golden_fixture() {
    let expected: Value = serde_json::from_str(include_str!(
        "../../../../architecture/migration/s12/fixtures/hero-preview.golden.json"
    ))
    .unwrap();
    let report = inspect(&fixture());
    assert_eq!(
        serde_json::to_value(&report.card_previews[0]).unwrap(),
        expected
    );
}

#[test]
fn source_language_policy_and_url_are_preserved_not_derived_from_preview_locale() {
    let r = inspect(&fixture());
    assert_eq!(page(&r, 101).source_language.as_deref(), Some("en"));
    assert_eq!(page(&r, 804).source_language.as_deref(), Some("de"));
    assert_eq!(
        page(&r, 101).upstream_url.as_deref(),
        Some("https://example.invalid/wiki/fixture/101")
    );
    assert_eq!(r.site_default_language.as_deref(), Some("en"));
    assert!(!r.source_policy.provider_egress_allowed && !r.source_policy.media_download_allowed);
    assert!(r.source_policy.decision_ref.is_some());
}

#[test]
fn same_revision_cannot_change_content_model_between_captures() {
    let before = inspect(&fixture());
    let mut v = fixture();
    slot_mut(&mut v, 701)["contentmodel"] = json!("wikitext");
    assert!(compare(&before, &inspect(&v)).is_err());
}

#[test]
fn source_key_cannot_be_a_path_or_url() {
    let mut v = fixture();
    v["source_key"] = json!("../other-wiki");
    fails(&v);
}

#[test]
fn preview_fanout_is_bounded_without_silent_truncation() {
    let mut v = fixture();
    let template = v["hero_bindings"][0].clone();
    v["hero_bindings"] = json!((0..32)
        .map(|i| {
            let mut b = template.clone();
            b["locale"] = json!(format!("fixture-{i}"));
            b
        })
        .collect::<Vec<_>>());
    slot_mut(&mut v, 101)["content"] = json!(serde_json::to_string(&vec![1; 2000]).unwrap());
    fails(&v);
}

#[test]
fn raw_hash_uses_the_standard_sha256_bytes_not_parsed_json() {
    assert_eq!(
        dbrain_s12_wiki_probe::sha256(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    assert_ne!(
        dbrain_s12_wiki_probe::sha256(b"{\"a\":1}"),
        dbrain_s12_wiki_probe::sha256(b"{ \"a\": 1 }")
    );
}
