use dbrain_s12_wiki_probe::{
    analyze,
    capture::{collect, CaptureOptions, Params},
    model::CapturePolicy,
};
use serde_json::{json, Value};
use std::collections::{BTreeSet, VecDeque};
fn options() -> CaptureOptions {
    CaptureOptions {
        source_key: "synthetic_wiki".into(),
        retrieved_at: 1790280000,
        policy: CapturePolicy {
            offline_review_allowed: true,
            decision_ref: Some("synthetic-only".into()),
            source_license: Some("synthetic fixture".into()),
            publication_allowed: false,
            provider_egress_allowed: false,
            media_download_allowed: false,
        },
        page_ids: BTreeSet::from([1]),
        max_requests: 20,
        max_pages: 10,
        max_total_bytes: 1_000_000,
    }
}
fn rev(id: i64) -> Value {
    json!({"revid":id,"timestamp":"2026-09-20T00:00:00Z","slots":{"main":{"contentmodel":"json","content":"{\"value\":\"1\"}"}}})
}
fn replies() -> Vec<Value> {
    vec![
        json!({"query":{"general":{"lang":"en"},"namespaces":[{"id":0,"name":""},{"id":3000,"name":"Data"}],"rightsinfo":{"text":"synthetic"}}}),
        json!({"query":{"allpages":[{"pageid":1,"ns":0,"title":"Hero"}]},"continue":{"continue":"-||","apcontinue":"Other"}}),
        json!({"query":{"allpages":[{"pageid":2,"ns":0,"title":"Other"}]}}),
        json!({"query":{"allpages":[{"pageid":3,"ns":3000,"title":"Data:Stats"}]}}),
        json!({"query":{"pages":[{"pageid":1,"ns":0,"title":"Hero","lastrevid":10,"pagelanguage":"en","revisions":[rev(10),rev(9)]}]},"continue":{"continue":"||","rvcontinue":"1|older"}}),
        json!({"query":{"pages":[{"pageid":1,"ns":0,"title":"Hero","lastrevid":10,"templates":[{"ns":3000,"title":"Data:Stats"}]}]},"continue":{"continue":"||","tlcontinue":"1|tail","plcontinue":"1|next"}}),
        json!({"query":{"pages":[{"pageid":1,"ns":0,"title":"Hero","lastrevid":10,"links":[{"ns":0,"title":"Other"}]}]}}),
    ]
}
#[test]
fn dynamic_discovery_preserves_all_tokens_and_merges_property_continuation() {
    let mut queue = VecDeque::from(replies());
    let mut requests = Vec::<Params>::new();
    let capture = collect(&options(), |p| {
        requests.push(p.clone());
        Ok(queue.pop_front().expect("unexpected extra request"))
    })
    .unwrap();
    assert!(queue.is_empty());
    assert_eq!(requests[2]["continue"], "-||");
    assert_eq!(requests[2]["apcontinue"], "Other");
    assert_eq!(requests[4]["rvlimit"], "2");
    assert_eq!(requests[6]["tlcontinue"], "1|tail");
    assert_eq!(requests[6]["plcontinue"], "1|next");
    assert!(capture.pages[0].dependencies_complete);
    assert!(
        capture.pages[0].response["query"]["pages"][0]["_capture_older_history_continue"]
            .is_object()
    );
    let report = analyze(&serde_json::to_vec(&capture).unwrap()).unwrap();
    assert!(report.discovery_complete);
    let page = report.pages.iter().find(|p| p.page_id == 1).unwrap();
    assert_eq!(page.revisions.len(), 2);
    assert_eq!(page.dependency_ids, BTreeSet::from([2, 3]));
    assert!(page.dependencies_complete);
}
#[test]
fn metadata_only_selection_never_fetches_any_body() {
    let mut opts = options();
    opts.page_ids.clear();
    let mut q = VecDeque::from(replies()[..4].to_vec());
    let c = collect(&opts, |p| {
        assert!(!p.contains_key("prop"));
        Ok(q.pop_front().unwrap())
    })
    .unwrap();
    assert!(c.pages.is_empty());
    assert!(q.is_empty());
}
#[test]
fn network_denial_is_not_retried_or_replaced_by_another_endpoint() {
    let mut calls = 0;
    let result = collect(&options(), |_| {
        calls += 1;
        Err("HTTP 403".into())
    });
    assert!(result.is_err());
    assert_eq!(calls, 1);
}
#[test]
fn invalid_policy_makes_no_transport_calls() {
    let mut opts = options();
    opts.policy.offline_review_allowed = false;
    assert!(collect(&opts, |_| panic!("unauthorized request")).is_err());
}
#[test]
fn request_and_page_budgets_cannot_return_false_completeness() {
    let mut opts = options();
    opts.max_requests = 2;
    let mut q = VecDeque::from(replies());
    let mut calls = 0;
    assert!(collect(&opts, |_| {
        calls += 1;
        Ok(q.pop_front().unwrap())
    })
    .is_err());
    assert_eq!(calls, 2);
    let mut opts = options();
    opts.max_pages = 1;
    let mut q = VecDeque::from(replies());
    assert!(collect(&opts, |_| Ok(q.pop_front().unwrap())).is_err());
}
#[test]
fn continuation_cannot_override_action_identity_or_execution_mode() {
    for key in ["action", "pageids", "prop", "titles"] {
        let mut responses = replies();
        responses[1]["continue"][key] = json!("unexpected");
        let mut q = VecDeque::from(responses);
        assert!(collect(&options(), |_| Ok(q.pop_front().unwrap())).is_err());
    }
}
#[test]
fn changing_revision_during_dependency_capture_is_not_accepted() {
    let mut responses = replies();
    responses[6]["query"]["pages"][0]["lastrevid"] = json!(11);
    let mut q = VecDeque::from(responses);
    assert!(collect(&options(), |_| Ok(q.pop_front().unwrap())).is_err());
}
#[test]
fn repeated_dependency_token_and_api_warnings_stop_capture() {
    let mut responses = replies();
    responses[6]["continue"] = responses[5]["continue"].clone();
    let mut q = VecDeque::from(responses);
    assert!(collect(&options(), |_| Ok(q.pop_front().unwrap())).is_err());
    let mut responses = replies();
    responses[5]["warnings"] = json!({"templates":"partial"});
    let mut q = VecDeque::from(responses);
    assert!(collect(&options(), |_| Ok(q.pop_front().unwrap())).is_err());
}
#[test]
fn aggregate_size_limit_is_enforced_before_a_capture_is_returned() {
    let mut opts = options();
    opts.max_total_bytes = 20;
    assert!(collect(&opts, |_| Ok(replies()[0].clone())).is_err());
}
