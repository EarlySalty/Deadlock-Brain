use super::super::internal_docs::{authorize, internal_search_text, validate_internal_evidence};
use super::*;
use brain_contracts::{
    Budget, Principal, PublicCitation, RequestDeadline, ToolCall, ToolDefinition, ToolName,
};
use std::time::Duration;

const REVISION: &str = "7b0f0e645f026214f28b99dddbb18bd477bc2819";

fn query() -> Query {
    serde_json::from_value(json!({"request_id":"docs","conversation_id":"docs","text":"Baba"}))
        .unwrap()
}

fn context(scope: &str) -> AuthorizedContext {
    AuthorizedContext {
        principal: Principal {
            actor_id: "fixture".into(),
            channel: "docs".into(),
            scopes: BTreeSet::from([scope.into()]),
            provider_egress: BTreeSet::from(["public".into()]),
        },
        conversation_id: "docs".into(),
        knowledge_release: "fixture".into(),
        deadline_ms: 10_000,
        request_deadline: None,
        discord: None,
        budget: Budget::default(),
    }
}

fn internal_context() -> AuthorizedContext {
    let mut context = context("second_brain.internal");
    context.principal.actor_id = "second-brain".into();
    context.principal.channel = "internal".into();
    context.principal.provider_egress.clear();
    context
}

fn request() -> ToolRequest {
    ToolCall {
        id: "docs".into(),
        name: ToolName::ServerKnowledge,
        arguments: json!({"question":"Baba"}),
    }
    .validate(&[ToolDefinition {
        name: ToolName::ServerKnowledge,
        description: "Vorhandener Vertrag ohne Spielversionsbindung".into(),
        input_schema: json!({"type":"object","additionalProperties":false,"required":["question"],"properties":{"question":{"type":"string","minLength":1}}}),
    }])
    .unwrap()
}

fn section_parts(id: i64, text: &str) -> (SectionView, Evidence) {
    let url = format!(
        "https://github.com/EarlySalty/Deadlock-Docs/blob/{REVISION}/public/billy.html#L35-L38"
    );
    let value = SectionView {
        page: "public/billy.html".into(),
        title: "Baba".into(),
        heading_path: vec!["Start".into()],
        excerpt: text.into(),
        excerpt_truncated: false,
        url: Some(url.clone()),
        citation: id.to_string(),
        patch: None,
        source_revision: REVISION.into(),
    };
    let evidence = Evidence {
        evidence_id: id.to_string(),
        source_id: "docs.public/1".into(),
        logical_id: format!("receipt/{id}"),
        revision: 1,
        kind: EvidenceKind::Prose,
        content: String::new(),
        citation: url,
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::from(["docs.public".into()]),
        score: 1.0,
        provenance: None,
        patch: None,
    };
    (value, evidence)
}

fn section_fixture(id: i64, text: &str) -> Section {
    let (value, evidence) = section_parts(id, text);
    Section::new(value, evidence).unwrap()
}

fn internal_section() -> Section {
    let (mut value, mut evidence) = section_parts(1, "private");
    value.page = "internal/fixture.html".into();
    value.url = None;
    evidence.visibility = SourceVisibility::Internal;
    evidence.source_id = "docs.internal/1".into();
    evidence.citation = "receipt/1".into();
    evidence.allowed_scopes = BTreeSet::from(["second_brain.internal".into()]);
    Section::new(value, evidence).unwrap()
}

#[test]
fn search_contract_accepts_only_bounded_typed_arguments() {
    let minimal = SearchDocsRequest::parse(&json!({"query":"Baba"})).unwrap();
    assert_eq!(minimal.entity, None);
    assert_eq!(minimal.limit, None);
    for limit in [1, 5] {
        let search = SearchDocsRequest::parse(
            &json!({"query":"ä".repeat(256),"entity":i64::MAX,"limit":limit}),
        )
        .unwrap();
        assert_eq!(search.query.len(), 512);
        assert_eq!(search.limit, Some(limit));
    }
    for arguments in [
        json!({}),
        json!({"query":""}),
        json!({"query":"  "}),
        json!({"query":"x".repeat(513)}),
        json!({"query":"ä".repeat(257)}),
        json!({"query":"Baba\n"}),
        json!({"query":7}),
        json!({"query":"Baba","entity":0}),
        json!({"query":"Baba","entity":-1}),
        json!({"query":"Baba","entity":i64::MAX as u64+1}),
        json!({"query":"Baba","entity":"1"}),
        json!({"query":"Baba","entity":null}),
        json!({"query":"Baba","limit":0}),
        json!({"query":"Baba","limit":6}),
        json!({"query":"Baba","limit":1.5}),
        json!({"query":"Baba","limit":"5"}),
        json!({"query":"Baba","limit":null}),
    ] {
        assert!(SearchDocsRequest::parse(&arguments).is_err(), "{arguments}");
    }
    for field in [
        "scope",
        "scopes",
        "principal",
        "provider_egress",
        "url",
        "sql",
        "actor_id",
    ] {
        let mut arguments = json!({"query":"Baba"});
        arguments[field] = json!("internal");
        assert!(SearchDocsRequest::parse(&arguments).is_err());
    }
    for code in (0..=0x1f).chain(0x7f..=0x9f) {
        let control = char::from_u32(code).unwrap();
        assert!(SearchDocsRequest::parse(&json!({"query":format!("Baba{control}")})).is_err());
    }
}

#[test]
fn internal_search_normalizes_permitted_controls_and_whitespace() {
    let mut query = query();
    query.text = " \tBaba\r\nprivate-only\u{2003}überleben\u{00a0} ".into();
    query.validate().unwrap();
    assert_eq!(
        internal_search_text(&query).unwrap(),
        "Baba private-only überleben"
    );
    for code in (0..=0x1f).chain(0x7f..=0x9f) {
        let separator = char::from_u32(code).unwrap();
        assert!(separator.is_control());
        query.text = format!("Baba{separator}private-only");
        query.validate().unwrap();
        assert_eq!(internal_search_text(&query).unwrap(), "Baba private-only");
    }
    query.text = "\0\u{0001}\u{007f}\t\r\n \u{2003}".into();
    query.validate().unwrap();
    assert!(internal_search_text(&query).unwrap().is_empty());
}

#[test]
fn internal_search_preserves_complete_long_ascii_and_unicode_inputs() {
    let mut query = query();
    for text in [
        format!("{}private-only", "Baba ".repeat(128)),
        format!("{}private-only", "überleben ".repeat(64)),
    ] {
        assert!(text.len() > 512);
        query.text = text.clone();
        query.validate().unwrap();
        assert_eq!(internal_search_text(&query).unwrap(), text);
    }
    query.text = format!("Baba{}p5suffixnichtvorhanden", " ".repeat(600));
    assert_eq!(
        internal_search_text(&query).unwrap(),
        "Baba p5suffixnichtvorhanden"
    );
    query.text = format!(
        "Baba{}private-only",
        " ".repeat(32_768 - "Baba".len() - "private-only".len())
    );
    assert_eq!(query.text.len(), 32_768);
    query.validate().unwrap();
    assert_eq!(internal_search_text(&query).unwrap(), "Baba private-only");
    query.text = format!(
        "{}private-only",
        "ä".repeat((32_768 - "private-only".len()) / 2)
    );
    assert_eq!(internal_search_text(&query).unwrap(), query.text);
}

#[test]
fn internal_search_validates_original_query_before_normalization() {
    let mut query = query();
    for text in [
        String::new(),
        " \t\n".into(),
        "x".repeat(32_769),
        "ä".repeat(16_385),
        format!("Baba{}private-only", " ".repeat(32_768)),
    ] {
        query.text = text;
        assert!(query.validate().is_err());
        assert!(internal_search_text(&query).is_err());
    }
    query.text = "Baba".into();
    query.request_id = "invalid\nidentifier".into();
    assert!(query.validate().is_err());
    assert!(internal_search_text(&query).is_err());
}

#[test]
fn scope_mapping_uses_only_authorized_principal_without_egress_widening() {
    let mut query = query();
    query
        .requested_scopes
        .insert("second_brain.internal".into());
    for scope in ["docs.public", "bot.public"] {
        assert_eq!(
            scopes(&query, &context(scope)).unwrap(),
            SourceVisibility::Public
        );
        assert!(authorize(&query, &context(scope)).is_err());
    }
    assert_eq!(
        scopes(&query, &internal_context()).unwrap(),
        SourceVisibility::Internal
    );
    authorize(&query, &internal_context()).unwrap();
    for scope in ["docs.internal", "second_brain.internal", "unrelated"] {
        assert!(scopes(&query, &context(scope)).is_err());
    }
    let mut invalid = Vec::new();
    let mut internal = internal_context();
    internal.principal.provider_egress.insert("internal".into());
    invalid.push(internal);
    let mut internal = internal_context();
    internal.principal.provider_egress.insert("public".into());
    invalid.push(internal);
    let mut internal = internal_context();
    internal.principal.actor_id = "docs-client".into();
    invalid.push(internal);
    let mut internal = internal_context();
    internal.principal.channel = "docs".into();
    invalid.push(internal);
    let mut internal = internal_context();
    internal.principal.scopes.insert("docs.public".into());
    invalid.push(internal);
    let mut public = context("docs.public");
    public.conversation_id = "foreign".into();
    invalid.push(public);
    let mut public = context("docs.public");
    public.principal.provider_egress.clear();
    invalid.push(public);
    let mut internal = internal_context();
    internal.request_deadline = Some(RequestDeadline::after(Duration::ZERO));
    invalid.push(internal);
    for context in invalid {
        assert!(scopes(&query, &context).is_err());
        assert!(authorize(&query, &context).is_err());
    }
}

#[test]
fn internal_dependencies_are_never_published_or_sent_to_provider() {
    let section = internal_section();
    assert!(section.value.url.is_none());
    assert!(!section.evidence.content.contains("https://"));
    let public_citation = PublicCitation::from_evidence(&section.evidence, 0);
    assert_ne!(public_citation.label, section.evidence.citation);
    assert!(!public_citation.label.contains("https://"));
    for evidence in [vec![section.evidence.clone()], Vec::new()] {
        for purpose in [
            ToolValidationPurpose::Provider,
            ToolValidationPurpose::Publication,
        ] {
            assert!(
                validate_internal_evidence(&query(), &internal_context(), &evidence, purpose)
                    .is_err()
            );
        }
        validate_internal_evidence(
            &query(),
            &internal_context(),
            &evidence,
            ToolValidationPurpose::Cache,
        )
        .unwrap();
        assert!(validate_internal_evidence(
            &query(),
            &context("docs.public"),
            &evidence,
            ToolValidationPurpose::Cache
        )
        .is_err());
    }
    let mut internal = internal_context();
    internal.conversation_id = "foreign".into();
    assert!(
        validate_internal_evidence(&query(), &internal, &[], ToolValidationPurpose::Cache).is_err()
    );
    let mut unsafe_citation = section.evidence.clone();
    unsafe_citation.citation = format!(
        "https://internal.example/receipt/{}",
        unsafe_citation.evidence_id
    );
    assert!(validate_internal_evidence(
        &query(),
        &internal_context(),
        &[unsafe_citation],
        ToolValidationPurpose::Cache
    )
    .is_err());
    let mut section = internal_section();
    section.evidence.allowed_scopes.insert("docs.public".into());
    assert!(validate_internal_evidence(
        &query(),
        &internal_context(),
        &[section.evidence],
        ToolValidationPurpose::Cache
    )
    .is_err());
}

#[test]
fn public_dependencies_preserve_real_scopes_and_receipt_url() {
    let request = request();
    let execution =
        bounded_execution("docs", &request, vec![section_fixture(7, "Baba")], false).unwrap();
    let call = ToolCall {
        id: "docs".into(),
        name: request.name(),
        arguments: request.arguments().clone(),
    };
    execution.validate_for(&call, &request, None).unwrap();
    assert_eq!(execution.result.evidence_ids, ["7"]);
    let dependency = &execution.dependencies[0];
    assert_eq!(dependency.evidence[0].evidence_id, "7");
    assert_eq!(
        execution.result.result["sections"][0]["url"],
        dependency.evidence[0].citation
    );
    assert_eq!(
        PublicCitation::from_evidence(&dependency.evidence[0], 0).label,
        dependency.evidence[0].citation
    );
    for purpose in [
        ToolValidationPurpose::Provider,
        ToolValidationPurpose::Publication,
        ToolValidationPurpose::Cache,
    ] {
        assert!(validate_evidence_access(
            &query(),
            &context("docs.public"),
            &dependency.evidence,
            purpose
        )
        .is_ok());
        assert!(validate_evidence_access(
            &query(),
            &context("bot.public"),
            &dependency.evidence,
            purpose
        )
        .is_err());
    }
}

#[test]
fn public_evidence_requires_all_scopes_not_any_scope() {
    let mut evidence = section_fixture(1, "Baba").evidence;
    evidence.allowed_scopes.insert("bot.public".into());
    for scope in ["docs.public", "bot.public"] {
        assert!(validate_evidence_access(
            &query(),
            &context(scope),
            &[evidence.clone()],
            ToolValidationPurpose::Cache
        )
        .is_err());
    }
    let mut public = context("docs.public");
    public.principal.scopes.insert("bot.public".into());
    validate_evidence_access(
        &query(),
        &public,
        &[evidence.clone()],
        ToolValidationPurpose::Cache,
    )
    .unwrap();
    evidence.allowed_scopes.clear();
    assert!(validate_evidence_access(
        &query(),
        &public,
        &[evidence.clone()],
        ToolValidationPurpose::Cache
    )
    .is_err());
    evidence.allowed_scopes.insert("unrelated".into());
    public.principal.scopes.insert("unrelated".into());
    assert!(
        validate_evidence_access(&query(), &public, &[evidence], ToolValidationPurpose::Cache)
            .is_err()
    );
    assert!(validate_evidence_access(
        &query(),
        &public,
        &[internal_section().evidence],
        ToolValidationPurpose::Cache
    )
    .is_err());
}

#[test]
fn internal_evidence_bound_accounts_for_the_complete_serialized_list() {
    let make = || {
        (1..=MAX_SECTIONS as i64)
            .map(|id| section_fixture(id, &"\"\\🧠".repeat(375)))
            .collect::<Vec<_>>()
    };
    let complete = make()
        .into_iter()
        .map(|section| section.evidence)
        .collect::<Vec<_>>();
    assert!(serde_json::to_vec(&complete).unwrap().len() > MAX_BYTES);
    let bounded = bounded_evidence(make()).unwrap();
    assert!(!bounded.is_empty());
    assert!(bounded.len() < MAX_SECTIONS);
    assert!(serde_json::to_vec(&bounded).unwrap().len() <= MAX_BYTES);
    assert_eq!(bounded, complete[..bounded.len()]);
    assert_eq!(bounded_evidence(make()).unwrap(), bounded);
    assert!(bounded_evidence(Vec::new()).unwrap().is_empty());
    let mut oversized = section_fixture(1, "Baba");
    oversized.evidence.content = "x".repeat(MAX_BYTES);
    assert!(bounded_evidence(vec![oversized]).unwrap().is_empty());
    assert!(bounded_evidence(vec![section_fixture(1, "a"), section_fixture(1, "b")]).is_err());
    let bounded =
        bounded_evidence((1..=6).map(|id| section_fixture(id, "short")).collect()).unwrap();
    assert_eq!(bounded.len(), MAX_SECTIONS);
}

#[test]
fn whole_result_bound_accounts_for_unicode_escaping_call_id_and_evidence_ids() {
    let request = request();
    let make = || {
        (1..=5)
            .map(|id| section_fixture(id, &"\"\\🧠".repeat(375)))
            .collect()
    };
    let bounded = bounded_execution(&"\"".repeat(512), &request, make(), false).unwrap();
    assert!(serde_json::to_vec(&bounded).unwrap().len() <= MAX_BYTES);
    assert_eq!(bounded.result.result["truncated"], true);
    assert!(!bounded.result.is_error);
    let repeated = bounded_execution("revalidation", &request, make(), false).unwrap();
    assert_eq!(bounded.dependencies, repeated.dependencies);
    assert_eq!(
        bounded.result.evidence_ids.len(),
        bounded.result.result["sections"].as_array().unwrap().len()
    );
    assert_eq!(
        bounded.dependencies[0].evidence.len(),
        bounded.result.evidence_ids.len()
    );
    let bounded = bounded_execution(
        "docs",
        &request,
        (1..=6).map(|id| section_fixture(id, "short")).collect(),
        false,
    )
    .unwrap();
    assert_eq!(bounded.result.evidence_ids.len(), MAX_SECTIONS);
    assert_eq!(bounded.result.result["truncated"], true);
}

#[test]
fn complete_execution_budget_includes_the_dependency_request() {
    let mut request_call = ToolCall {
        id: "docs".into(),
        name: ToolName::ServerKnowledge,
        arguments: json!({"question":"\"".repeat(12_000)}),
    };
    let definition = ToolDefinition {
        name: ToolName::ServerKnowledge,
        description: "Vorhandener Vertrag".into(),
        input_schema: json!({"type":"object","additionalProperties":false,"required":["question"],"properties":{"question":{"type":"string","minLength":1}}}),
    };
    let request = request_call
        .validate(std::slice::from_ref(&definition))
        .unwrap();
    let execution =
        bounded_execution("docs", &request, vec![section_fixture(1, "short")], false).unwrap();
    assert!(execution.result.is_error);
    assert_eq!(execution.result.result["error"], "result_too_large");
    assert!(execution.dependencies.is_empty());
    assert!(execution.result.evidence_ids.is_empty());
    execution
        .validate_for(&request_call, &request, None)
        .unwrap();
    request_call.arguments = json!({"question":"short"});
    let request = request_call.validate(&[definition]).unwrap();
    assert!(
        !bounded_execution("docs", &request, vec![section_fixture(1, "short")], false)
            .unwrap()
            .result
            .is_error
    );
}

#[test]
fn empty_and_oversized_results_cannot_fabricate_evidence() {
    let request = request();
    let mut oversized = section_fixture(1, "Baba");
    oversized.evidence.content = "x".repeat(MAX_BYTES);
    for sections in [Vec::new(), vec![oversized]] {
        let execution = bounded_execution("docs", &request, sections, false).unwrap();
        assert!(execution.result.is_error);
        assert!(execution.result.evidence_ids.is_empty());
        assert!(execution.dependencies.is_empty());
        let call = ToolCall {
            id: "docs".into(),
            name: request.name(),
            arguments: request.arguments().clone(),
        };
        execution.validate_for(&call, &request, None).unwrap();
    }
    for id in [String::new(), "\n".into(), "x".repeat(513)] {
        assert!(bounded_execution(&id, &request, vec![section_fixture(1, "Baba")], false).is_err());
    }
    assert!(bounded_execution(
        "docs",
        &request,
        vec![section_fixture(1, "a"), section_fixture(1, "b")],
        false
    )
    .is_err());
}

#[test]
fn section_views_preserve_unicode_and_reject_mismatched_immutable_citations() {
    let section = section_fixture(1, &"ä".repeat(1501));
    assert_eq!(section.value.excerpt.chars().count(), 1500);
    assert!(section.value.excerpt_truncated);
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&section.evidence.content).unwrap(),
        serde_json::to_value(&section.value).unwrap()
    );
    let (view, evidence) = section_parts(1, "Baba");
    for url in [
        format!(
            "https://github.com/EarlySalty/Deadlock-Docs/blob/{}/public/billy.html#L35-L38",
            "0".repeat(40)
        ),
        format!(
            "https://github.com/EarlySalty/Deadlock-Docs/blob/{REVISION}/public/other.html#L35-L38"
        ),
        format!(
            "https://github.com/EarlySalty/Deadlock-Docs/blob/{REVISION}/public/billy.html#L0-L38"
        ),
    ] {
        let mut value = view.clone();
        let mut item = evidence.clone();
        value.url = Some(url.clone());
        item.citation = url;
        assert!(Section::new(value, item).is_err());
    }
    let mut value = view.clone();
    value.citation = "2".into();
    assert!(Section::new(value, evidence.clone()).is_err());
    let mut value = view.clone();
    value.source_revision = REVISION.to_uppercase();
    assert!(Section::new(value, evidence.clone()).is_err());
    let mut item = evidence.clone();
    item.score = f64::NAN;
    assert!(Section::new(view.clone(), item).is_err());
    let mut value = view;
    let mut item = evidence;
    item.visibility = SourceVisibility::Internal;
    value.page = "internal/fixture.html".into();
    assert!(Section::new(value, item).is_err());
}
