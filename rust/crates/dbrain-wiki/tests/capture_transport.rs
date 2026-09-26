#[path = "support/c5.rs"]
mod fixture;
use dbrain_s12_wiki_probe::{
    analyze,
    capture::{collect, collect_bytes, Params},
    knowledge::{self, IrValue},
    model::WikiScope,
};
use serde_json::{json, Value};
use std::{
    collections::{BTreeSet, VecDeque},
    time::{Duration, Instant},
};

#[test]
fn explicit_union_preserves_two_revisions_locators_dependencies_and_namespace_filter() {
    let mut seen = Vec::new();
    let capture = collect(&fixture::options(), |p| {
        seen.push(p.clone());
        Ok(fixture::reply(p, 101, 201))
    })
    .unwrap();
    assert_eq!(capture.pages.len(), 4);
    assert!(seen
        .iter()
        .all(|p| p.get("list").map(String::as_str) != Some("allpages")));
    let bytes = serde_json::to_vec(&capture).unwrap();
    let report = analyze(&bytes).unwrap();
    assert!(report.discovery_complete);
    assert_eq!(
        report
            .discovery_scope
            .unwrap()
            .selection
            .namespace_allowlist,
        BTreeSet::from([0, 10])
    );
    assert!(report.missing_namespaces.is_empty()); // File(6) is not required!
    let page = report.pages.iter().find(|p| p.page_id == 1).unwrap();
    assert_eq!(page.revisions.len(), 2);
    assert_eq!(page.dependency_ids, BTreeSet::from([2]));
    let ir = knowledge::extract(&bytes, &fixture::mapping()).unwrap();
    assert_eq!(ir.resolve_alias("pilot_alias", "en"), Some("hero:pilot"));
    assert!(ir
        .fields()
        .iter()
        .any(|f| matches!(f.value, IrValue::Unknown { .. })));
    assert!(ir.fields().iter().any(
        |f| f.condition.as_deref() == Some("night") && f.variant.as_deref() == Some("boosted")
    ));
    assert!(ir
        .contract()
        .data
        .artifacts
        .iter()
        .all(|a| a.locator.contains("oldid=") && !a.policy.provider_egress_allowed));
    assert!(ir
        .sources()
        .iter()
        .all(|s| s.allowed_scopes.contains("wiki.review")));
}
#[test]
fn empty_scope_or_allowlist_is_rejected_before_transport() {
    let mut opts = fixture::options();
    opts.scope.namespace_allowlist.clear();
    assert!(collect(&opts, |_| panic!("must not request")).is_err());
    opts = fixture::options();
    opts.scope = WikiScope {
        namespace_allowlist: [0].into(),
        page_ids: BTreeSet::new(),
        pages: BTreeSet::new(),
        categories: BTreeSet::new(),
        heroes: vec![],
    };
    assert!(collect(&opts, |_| panic!(
        "namespace alone is not a crawl instruction"
    ))
    .is_err());
    opts.scope.page_ids.insert(1);
    opts.scope.namespace_allowlist.insert(-1);
    assert!(collect(&opts, |_| panic!("virtual namespace")).is_err());
}
#[test]
fn outside_namespace_and_unknown_namespace_are_rejected() {
    let mut opts = fixture::options();
    opts.scope.namespace_allowlist = [0].into();
    assert!(collect(&opts, |p| Ok(fixture::reply(p, 101, 201)))
        .unwrap_err()
        .contains("allowlist"));
    opts.scope.namespace_allowlist = [0, 9999].into();
    let mut calls = 0;
    assert!(collect(&opts, |_| {
        calls += 1;
        Ok(fixture::siteinfo())
    })
    .is_err());
    assert_eq!(calls, 1);
}
#[test]
fn max_pages_is_not_silent_truncation_and_no_bodies_are_requested() {
    let mut opts = fixture::options();
    opts.max_pages = 1;
    let result = collect(&opts, |p| {
        assert_ne!(p.get("prop").map(String::as_str), Some("info|revisions"));
        Ok(fixture::reply(p, 101, 201))
    });
    assert!(result.unwrap_err().contains("page budget"));
}
#[test]
fn max_requests_counts_siteinfo_identity_history_and_dependencies() {
    for budget in [1, 3, 7, 10] {
        let mut opts = fixture::options();
        opts.max_requests = budget;
        let mut calls = 0;
        let result = collect(&opts, |p| {
            calls += 1;
            Ok(fixture::reply(p, 101, 201))
        });
        assert!(result.unwrap_err().contains("request budget"));
        assert_eq!(calls, budget);
    }
}
#[test]
fn bytes_are_counted_before_json_whitespace_normalization() {
    let mut opts = fixture::options();
    opts.max_total_bytes = 1024;
    let mut bytes = serde_json::to_vec(&fixture::siteinfo()).unwrap();
    bytes.extend(vec![b' '; 1024]);
    let mut calls = 0;
    assert!(collect_bytes(&opts, |_, limit| {
        calls += 1;
        assert_eq!(limit.max_bytes, 1024);
        Ok(bytes.clone())
    })
    .unwrap_err()
    .contains("byte budget"));
    assert_eq!(calls, 1);
}
#[test]
fn deadline_includes_transport_and_pacing() {
    let mut opts = fixture::options();
    opts.deadline_ms = 5;
    let mut calls = 0;
    assert!(collect(&opts, |p| {
        calls += 1;
        std::thread::sleep(Duration::from_millis(10));
        Ok(fixture::reply(p, 101, 201))
    })
    .unwrap_err()
    .contains("deadline"));
    assert_eq!(calls, 1);
    opts.request_interval_ms = 50;
    opts.deadline_ms = 20;
    calls = 0;
    assert!(collect(&opts, |p| {
        calls += 1;
        Ok(fixture::reply(p, 101, 201))
    })
    .unwrap_err()
    .contains("deadline"));
    assert_eq!(calls, 1);
}
#[test]
fn request_interval_is_enforced() {
    let mut opts = fixture::options();
    opts.request_interval_ms = 3;
    let mut last: Option<Instant> = None;
    collect(&opts, |p| {
        if let Some(at) = last {
            assert!(at.elapsed() >= Duration::from_millis(3));
        }
        last = Some(Instant::now());
        Ok(fixture::reply(p, 101, 201))
    })
    .unwrap();
}
#[test]
fn errors_warnings_and_network_denials_are_not_retried() {
    for response in [
        Err("HTTP 403".into()),
        Ok(json!({"warnings":{"query":"partial"}})),
        Ok(json!({"error":{"code":"maxlag"}})),
    ] {
        let mut calls = 0;
        assert!(collect(&fixture::options(), |_| {
            calls += 1;
            response.clone()
        })
        .is_err());
        assert_eq!(calls, 1);
    }
    let mut opts = fixture::options();
    opts.policy.offline_review_allowed = false;
    assert!(collect(&opts, |_| panic!("unauthorized")).is_err());
}
fn category_options() -> dbrain_s12_wiki_probe::capture::CaptureOptions {
    let mut o = fixture::options();
    o.scope.page_ids.clear();
    o.scope.heroes.clear();
    o.scope.namespace_allowlist = [0].into();
    o.scope.categories.insert("Category:Pilot".into());
    o
}
#[test]
fn category_continuation_handles_empty_batches_and_all_tokens_without_recursion() {
    let mut pages = VecDeque::from([
        json!({"query":{"categorymembers":[]},"continue":{"continue":"-||","cmcontinue":"A"}}),
        json!({"query":{"categorymembers":[{"pageid":4,"ns":0,"title":"Other Hero"}]}}),
    ]);
    let mut requests = Vec::<Params>::new();
    let capture = collect(&category_options(), |p| {
        requests.push(p.clone());
        if p.get("list").is_some() {
            assert_eq!(p["list"], "categorymembers");
            assert_eq!(p["cmtype"], "page");
            Ok(pages.pop_front().unwrap())
        } else {
            Ok(fixture::reply(p, 101, 201))
        }
    })
    .unwrap();
    assert_eq!(capture.pages.len(), 1);
    assert!(pages.is_empty());
    assert_eq!(requests[2]["continue"], "-||");
    assert_eq!(requests[2]["cmcontinue"], "A");
}
#[test]
fn continuation_cannot_override_identity_action_namespace_or_execution() {
    for key in ["action", "pageids", "prop", "titles", "cmnamespace"] {
        let mut calls = 0;
        let result = collect(&category_options(), |p| {
            calls += 1;
            if p.get("meta").is_some() {
                return Ok(fixture::siteinfo());
            }
            let mut response = json!({"query":{"categorymembers":[]},"continue":{"continue":"-||","cmcontinue":"A"}});
            response["continue"][key] = json!("unexpected");
            Ok(response)
        });
        assert!(result.is_err());
        assert_eq!(calls, 2);
    }
    let mut calls = 0;
    assert!(collect(&category_options(),|p|{calls+=1;if p.get("meta").is_some(){Ok(fixture::siteinfo())}else{Ok(json!({"query":{"categorymembers":[]},"continue":{"continue":"-||","cmcontinue":"A"}}))}}).unwrap_err().contains("repeated"));
    assert_eq!(calls, 3);
}
#[test]
fn revision_continuation_fetches_exactly_two_not_all_history() {
    let mut opts = fixture::options();
    opts.scope.heroes.clear();
    opts.scope.page_ids = [4].into();
    let mut history_calls = 0;
    let capture=collect(&opts,|p|{
        let mut response=fixture::reply(p,101,201);
        if p.get("prop").map(String::as_str)==Some("info|revisions") {
            history_calls+=1;let revisions=&mut response["query"]["pages"][0]["revisions"];
            let chosen=revisions[if history_calls==1 {0}else{1}].clone();*revisions=json!([chosen]);
            if history_calls==2{assert_eq!(p["rvcontinue"],"4|older");assert_eq!(p["rvlimit"],"1");}
            response["continue"]=json!({"continue":"||","rvcontinue":if history_calls==1 {"4|older"}else{"4|ancient"}});
        }Ok(response)
    }).unwrap();
    assert_eq!(history_calls, 2);
    assert_eq!(
        capture.pages[0].response["query"]["pages"][0]["revisions"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert_eq!(
        capture.pages[0].response["query"]["pages"][0]["_capture_older_history_continue"]
            ["rvcontinue"],
        "4|ancient"
    );
}
#[test]
fn dependency_continuation_merges_both_tokens_and_rejects_head_race() {
    let mut opts = fixture::options();
    opts.scope.heroes.clear();
    opts.scope.page_ids = [4].into();
    let mut deps = 0;
    collect(&opts, |p| {
        let mut r = fixture::reply(p, 101, 201);
        if p.get("prop").map(String::as_str) == Some("info|templates|links") {
            deps += 1;
            if deps == 1 {
                r["continue"] = json!({"continue":"||","tlcontinue":"4|t","plcontinue":"4|p"});
            } else {
                assert_eq!(p["tlcontinue"], "4|t");
                assert_eq!(p["plcontinue"], "4|p");
            }
        }
        Ok(r)
    })
    .unwrap();
    assert_eq!(deps, 2);
    assert!(collect(&opts, |p| {
        let mut r = fixture::reply(p, 101, 201);
        if p.get("prop").map(String::as_str) == Some("info|templates|links") {
            r["query"]["pages"][0]["lastrevid"] = json!(999);
        }
        Ok(r)
    })
    .is_err());
}
#[test]
fn unchanged_changed_delta_and_selective_template_invalidation() {
    let mapping = fixture::mapping();
    let before = knowledge::extract(&fixture::bytes(101, 201), &mapping).unwrap();
    let same = knowledge::extract(&fixture::bytes(101, 201), &mapping).unwrap();
    let unchanged = knowledge::compare_ir(&before, &same).unwrap();
    assert!(unchanged.reparse_pages.is_empty());
    let after = knowledge::extract(&fixture::bytes(101, 202), &mapping).unwrap();
    let impact = knowledge::compare_ir(&before, &after).unwrap();
    assert_eq!(impact.changed_pages, BTreeSet::from([2]));
    assert_eq!(impact.reparse_pages, BTreeSet::from([1, 2, 3]));
    assert!(!impact.full_reconcile_required);
    let after = knowledge::extract(&fixture::bytes(102, 201), &mapping).unwrap();
    let impact = knowledge::compare_ir(&before, &after).unwrap();
    assert_eq!(impact.raw_content_changed, BTreeSet::from([1]));
    assert_eq!(impact.reparse_pages, BTreeSet::from([1, 3]));
}
#[test]
fn changed_scope_is_not_deletion_and_missing_dependencies_remain_unknown() {
    let before = analyze(&fixture::bytes(101, 201)).unwrap();
    let mut opts = fixture::options();
    opts.scope.page_ids.remove(&4);
    let cap = collect(&opts, |p| Ok(fixture::reply(p, 101, 201))).unwrap();
    let after = analyze(&serde_json::to_vec(&cap).unwrap()).unwrap();
    assert!(dbrain_s12_wiki_probe::compare(&before, &after)
        .unwrap_err()
        .contains("scope"));
    opts = fixture::options();
    opts.scope.page_ids.remove(&2);
    let cap = collect(&opts, |p| Ok(fixture::reply(p, 101, 201))).unwrap();
    let after = analyze(&serde_json::to_vec(&cap).unwrap()).unwrap();
    assert!(after
        .pages
        .iter()
        .find(|p| p.page_id == 1)
        .unwrap()
        .unresolved_dependencies
        .contains("Template:Stats"));
}
#[test]
fn scoped_manifest_cannot_be_relabelled_as_full_inventory() {
    let mut capture = serde_json::to_value(fixture::capture(101, 201)).unwrap();
    capture["format"] = json!("s12-capture-v1");
    assert!(analyze(&serde_json::to_vec(&capture).unwrap()).is_err());
    capture["format"] = json!("wiki-scoped-capture-v1");
    capture["discovery_scope"] = Value::Null;
    assert!(analyze(&serde_json::to_vec(&capture).unwrap()).is_err());
}
