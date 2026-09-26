//! C6 release -> domain -> retrieval -> kernel -> public API regression suite.
#[path = "support/domain_fixture.rs"]
mod fixture;
use brain_contracts::{domain::*, value::Observed, *};
use brain_kernel::{AnswerKernelPort, CachedKernel, Kernel};
use brain_storage::MemoryRepository;
use dbrain_retrieval::ReleaseRetriever;
use fixture::*;
use std::result::Result;
use std::{
    collections::BTreeSet,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};
const PATCH: &str = "synthetic-p1";
#[derive(Clone)]
struct NoProvider;
impl AnswerProviderPort for NoProvider {
    fn answer(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: &[Evidence],
    ) -> Result<ProviderAnswer, PortError> {
        panic!("deterministic domain route called a provider")
    }
}
fn context(release: &str) -> AuthorizedContext {
    AuthorizedContext {
        principal: Principal {
            actor_id: "fixture".into(),
            channel: "test".into(),
            scopes: BTreeSet::new(),
            provider_egress: BTreeSet::from(["public".into()]),
        },
        conversation_id: "c6-conversation".into(),
        knowledge_release: release.into(),
        deadline_ms: 5_000,
        budget: Budget::default(),
    }
}
async fn store(records: Vec<SourceRecordV2>) -> MemoryRepository {
    let store = MemoryRepository::default();
    for record in records {
        store.apply_record(record).unwrap();
    }
    let release = store.release_from_heads("r1", "v1", PATCH).unwrap();
    store.publish(&release).await.unwrap();
    store
}
fn answer(store: &MemoryRepository, request: &DomainRequest) -> AnswerResponse {
    Kernel::new(ReleaseRetriever::new(store.clone(), 8), NoProvider)
        .answer(&query(request, PATCH), &context("r1"))
}
fn certificate(answer: &AnswerResponse) -> DomainAnswer {
    assert_eq!(answer.usage, Usage::default());
    assert_eq!(answer.citations.len(), 1);
    assert!(answer.text.contains("[Beleg 1]"));
    serde_json::from_str(&answer.citations[0].content).unwrap()
}
fn fact(hero: &str, locale: &str, predicate: &str) -> DomainRequest {
    DomainRequest::Fact {
        hero: hero.into(),
        locale: locale.into(),
        predicate: predicate.into(),
    }
}

#[tokio::test]
async fn legal_build_reuses_inventory_upgrade_and_cites_exact_inputs_without_llm() {
    let s = store(records("r1", PATCH, 1, "500")).await;
    let a = answer(&s, &build("Fixture Hero", "en", &["101", "102", "103"]));
    assert_eq!(a.status, AnswerStatus::Answered);
    let proof = certificate(&a);
    let result = proof.build_evaluation.unwrap();
    assert_eq!(result.legal, Observed::known(true));
    assert_eq!(result.spent_souls, Observed::known(2400));
    assert_eq!(result.consumed_ids, vec![101]);
    assert!(proof.inputs.iter().any(|i| i.source.logical_id == "models"));
    assert!(proof
        .inputs
        .iter()
        .any(|i| i.source.logical_id == "inventory"));
    assert!(proof
        .inputs
        .iter()
        .any(|i| i.source.logical_id == "fact-active-limit"));
    assert!(proof
        .inputs
        .iter()
        .all(|i| !i.locator.is_empty() && !i.parser_revision.is_empty()));
}
#[tokio::test]
async fn illegal_duplicate_is_rejected_with_evidence_not_a_repaired_or_generated_build() {
    let s = store(records("r1", PATCH, 1, "500")).await;
    let a = answer(&s, &build("Fixture Hero", "en", &["101", "101"]));
    assert_eq!(a.status, AnswerStatus::BuildRejected);
    assert!(a.text.contains("bereits im Inventar"));
    let result = certificate(&a).build_evaluation.unwrap();
    assert_eq!(result.legal, Observed::known(false));
    assert!(matches!(result.spent_souls, Observed::Unknown { .. }));
}
#[tokio::test]
async fn slot_and_active_limits_are_taken_from_the_pinned_rules() {
    let s = store(records("r1", PATCH, 1, "500")).await;
    for (items, reason) in [
        (vec!["101", "103", "201"], "Inventarplätze"),
        (vec!["201", "202"], "höchstens 1 aktive"),
    ] {
        let a = answer(&s, &build("Fixture Hero", "en", &items));
        assert_eq!(a.status, AnswerStatus::BuildRejected);
        assert!(a.text.contains(reason), "{}", a.text);
        certificate(&a);
    }
}
#[tokio::test]
async fn unknown_hero_and_item_never_become_an_illegal_zero_cost_build() {
    let s = store(records("r1", PATCH, 1, "500")).await;
    let hero = answer(&s, &build("Imaginary Hero", "en", &["101"]));
    assert_eq!(hero.status, AnswerStatus::InsufficientEvidence);
    assert!(hero.citations.is_empty());
    let item = answer(&s, &build("Fixture Hero", "en", &["Imaginary Item"]));
    assert_eq!(item.status, AnswerStatus::InsufficientEvidence);
    let result = certificate(&item).build_evaluation.unwrap();
    assert!(matches!(result.legal, Observed::Unknown { .. }));
    assert!(matches!(result.spent_souls, Observed::Unknown { .. }));
}
#[tokio::test]
async fn invalid_or_absent_patch_and_mode_fail_closed() {
    let s = store(records("r1", PATCH, 1, "500")).await;
    let k = Kernel::new(ReleaseRetriever::new(s, 8), NoProvider);
    for (patch, mode) in [
        (Some("p2"), Some("ranked")),
        (Some(PATCH), Some("casual")),
        (None, Some("ranked")),
        (Some(PATCH), None),
    ] {
        let mut q = query(&build("Fixture Hero", "en", &["101"]), PATCH);
        q.patch = patch.map(str::to_owned);
        q.mode = mode.map(str::to_owned);
        let a = k.answer(&q, &context("r1"));
        assert_eq!(a.status, AnswerStatus::InsufficientEvidence);
        assert!(a.citations.is_empty());
    }
}
#[tokio::test]
async fn two_hero_revisions_remain_separately_release_pinned() {
    let s = store(records("r1", PATCH, 1, "500")).await;
    for record in records("r2", PATCH, 2, "600") {
        s.apply_record(record).unwrap();
    }
    let r2 = s.release_from_heads("r2", "v2", PATCH).unwrap();
    s.publish(&r2).await.unwrap();
    let k = Kernel::new(ReleaseRetriever::new(s, 8), NoProvider);
    let q = query(&fact("Fixture Hero", "en", "health"), PATCH);
    for (release, value, revision) in [("r1", "500", 1), ("r2", "600", 2)] {
        let a = k.answer(&q, &context(release));
        assert_eq!(a.status, AnswerStatus::Answered);
        assert!(a.text.contains(value));
        assert_eq!(a.citations[0].revision, revision);
        assert!(certificate(&a)
            .inputs
            .iter()
            .any(|i| i.source.logical_id == "hero" && i.source.revision == revision));
    }
}
#[tokio::test]
async fn en_and_de_aliases_follow_s05_without_casefold_or_fuzzy_fallback() {
    let s = store(records("r1", PATCH, 1, "500")).await;
    for (hero, locale) in [
        ("__FIXTURE__HERO__", "en"),
        ("fixture\u{1c}hero", "en"),
        ("PRÜFHELD", "de"),
        ("STRAßE", "de"),
    ] {
        assert_eq!(
            answer(&s, &fact(hero, locale, "health")).status,
            AnswerStatus::Answered
        );
    }
    for (hero, locale) in [("Strasse", "de"), ("Prüfheld", "en"), ("Fixture", "en")] {
        assert_eq!(
            answer(&s, &fact(hero, locale, "health")).status,
            AnswerStatus::InsufficientEvidence
        );
    }
    assert_eq!(
        answer(&s, &build("Prüfheld", "de", &["__PRÜFOBJEKT__101__"])).status,
        AnswerStatus::Answered
    );
}
#[tokio::test]
async fn static_decimal_fact_and_actual_zero_are_not_llm_calculated() {
    for value in ["500.125", "0"] {
        let s = store(records("r1", PATCH, 1, value)).await;
        let a = answer(&s, &fact("Fixture Hero", "en", "health"));
        assert_eq!(a.status, AnswerStatus::Answered);
        assert!(a.text.contains(&format!("= {value} health")));
        assert_eq!(certificate(&a).route, DomainRoute::Fact);
    }
}
#[tokio::test]
async fn typed_rule_uses_existing_evaluator_and_records_fact_provenance() {
    let s = store(records("r1", PATCH, 1, "500")).await;
    let a = answer(
        &s,
        &DomainRequest::Rule {
            rule_id: "fixture-dps".into(),
        },
    );
    assert_eq!(a.status, AnswerStatus::Answered);
    let proof = certificate(&a);
    let result = proof.rule_evaluation.unwrap();
    assert_eq!(
        result.quantity,
        Quantity {
            value: 8.0,
            unit: "damage/s".into()
        }
    );
    assert_eq!(result.evaluator_version, RULE_EVALUATOR_VERSION);
    assert_eq!(
        result.input_fact_ids,
        BTreeSet::from(["damage".into(), "interval".into()])
    );
    for id in [
        "rule",
        "rule-source",
        "fact-damage",
        "fact-interval",
        "mechanics",
    ] {
        assert!(proof.inputs.iter().any(|i| i.source.logical_id == id));
    }
}
#[tokio::test]
async fn unknown_card_fields_and_unverified_rule_inputs_are_not_zero() {
    let mut data = records("r1", PATCH, 1, "500");
    let index = data
        .iter()
        .position(|r| r.logical_id == "fact-interval")
        .unwrap();
    let mut object: StoredDomainObject = serde_json::from_str(&data[index].content).unwrap();
    if let DomainObject::NumericFact(f) = &mut object.object {
        f.verified = false;
    }
    data[index] = typed("fact-interval", 1, object.object, PATCH);
    let s = store(data).await;
    let a = answer(&s, &fact("Fixture Hero", "en", "mystery"));
    assert_eq!(a.status, AnswerStatus::InsufficientEvidence);
    assert_eq!(certificate(&a).verdict, DomainVerdict::Unknown);
    let a = answer(
        &s,
        &DomainRequest::Rule {
            rule_id: "fixture-dps".into(),
        },
    );
    assert_eq!(a.status, AnswerStatus::InsufficientEvidence);
    assert!(certificate(&a).rule_evaluation.is_none());
}
fn missing_cost() -> Vec<SourceRecordV2> {
    let mut data = records("r1", PATCH, 1, "500");
    let index = data.iter().position(|r| r.logical_id == "models").unwrap();
    let mut models: serde_json::Value = serde_json::from_str(&data[index].content).unwrap();
    models[0].as_object_mut().unwrap().remove("cost");
    data[index] = record("models", 1, &models, PATCH);
    let source = revision(&data[index]);
    let index = data.iter().position(|r| r.logical_id == "catalog").unwrap();
    let mut object: StoredDomainObject = serde_json::from_str(&data[index].content).unwrap();
    if let DomainObject::BuildCatalog(c) = &mut object.object {
        c.models.source = source.clone();
        for alias in &mut c.aliases {
            alias.alias.provenance.source = source.clone();
        }
    }
    data[index] = typed("catalog", 1, object.object, PATCH);
    data
}
#[tokio::test]
async fn missing_model_cost_is_unknown_not_default_zero_or_illegal() {
    let s = store(missing_cost()).await;
    let a = answer(&s, &build("Fixture Hero", "en", &["101"]));
    assert_eq!(a.status, AnswerStatus::InsufficientEvidence);
    let result = certificate(&a).build_evaluation.unwrap();
    assert!(matches!(result.legal, Observed::Unknown { .. }));
    assert!(matches!(result.spent_souls, Observed::Unknown { .. }));
}
#[tokio::test]
async fn source_acl_is_propagated_and_rechecked_on_cached_domain_answers() {
    let data = records("r1", PATCH, 1, "500");
    let mut models = data
        .iter()
        .find(|r| r.logical_id == "models")
        .unwrap()
        .clone();
    let s = store(data).await;
    let q = query(&build("Fixture Hero", "en", &["101"]), PATCH);
    let k = CachedKernel::new(
        Kernel::new(ReleaseRetriever::new(s.clone(), 8), NoProvider),
        8,
        Duration::from_secs(30),
    );
    assert_eq!(k.answer(&q, &context("r1")).status, AnswerStatus::Answered);
    models.revision = 2;
    models.visibility = SourceVisibility::Private;
    models.allowed_scopes.insert("private.models".into());
    s.apply_record(models).unwrap();
    let denied = k.answer(&q, &context("r1"));
    assert_eq!(denied.status, AnswerStatus::InsufficientEvidence);
    assert!(denied.citations.is_empty());
    let mut authorized = context("r1");
    authorized.principal.scopes.insert("private.models".into());
    let a = k.answer(&q, &authorized);
    assert_eq!(a.status, AnswerStatus::Answered);
    assert_eq!(a.citations[0].visibility, SourceVisibility::Private);
    assert!(a.citations[0].allowed_scopes.contains("private.models"));
}
#[tokio::test]
async fn deletion_and_forged_evidence_cannot_reuse_an_old_proof() {
    let data = records("r1", PATCH, 1, "500");
    let mut source = data
        .iter()
        .find(|r| r.logical_id == "hero")
        .unwrap()
        .clone();
    let s = store(data).await;
    let r = ReleaseRetriever::new(s.clone(), 8);
    let q = query(&fact("Fixture Hero", "en", "health"), PATCH);
    let mut evidence = r.retrieve(&q, &context("r1")).unwrap();
    r.validate_evidence(&q, &context("r1"), &evidence, false)
        .unwrap();
    evidence[0].content.push(' ');
    assert!(r
        .validate_evidence(&q, &context("r1"), &evidence, false)
        .is_err());
    source.revision = 2;
    source.tombstone = true;
    s.apply_record(source).unwrap();
    assert_eq!(
        answer(&s, &fact("Fixture Hero", "en", "health")).status,
        AnswerStatus::InsufficientEvidence
    );
}
#[tokio::test]
async fn effects_synergies_and_unknown_fields_are_retrievable_with_full_source_closure() {
    let s = store(records("r1", PATCH, 1, "500")).await;
    let r = ReleaseRetriever::new(s, 8);
    let q = query(
        &DomainRequest::Card {
            hero: "Fixture Hero".into(),
            locale: "en".into(),
        },
        PATCH,
    );
    let pack = r.retrieve(&q, &context("r1")).unwrap();
    r.validate_evidence(&q, &context("r1"), &pack, true)
        .unwrap();
    let proof: DomainAnswer = serde_json::from_str(&pack[0].content).unwrap();
    assert_eq!(proof.route, DomainRoute::Card);
    assert!(proof.text.contains("fixture-effect"));
    assert!(proof.text.contains("fixture-synergy"));
    assert!(proof.text.contains("explicit_null"));
    assert!(proof
        .inputs
        .iter()
        .any(|i| i.source.logical_id == "mechanics"));
}
#[tokio::test]
async fn public_api_rejection_keeps_resolvable_evidence_labels_without_private_locators() {
    use brain_api::ApiService;
    use brain_policy::{AuthGrant, CredentialRegistry, PolicyEngine};
    let s = store(records("r1", PATCH, 1, "500")).await;
    let service = ApiService::new(
        PolicyEngine::new(CredentialRegistry::new(vec![AuthGrant::from_secret(
            "test-only-token",
            "fixture",
            "test",
            BTreeSet::new(),
            BTreeSet::new(),
        )])),
        Kernel::new(ReleaseRetriever::new(s, 8), NoProvider),
        "r1",
        5_000,
        Budget::default(),
    );
    let q = query(&build("Fixture Hero", "en", &["101", "101"]), PATCH);
    let response = service.handle_answer(
        Some("Bearer test-only-token"),
        &serde_json::to_vec(&q).unwrap(),
    );
    assert_eq!(response.status, 200);
    let public: PublicAnswerResponse = serde_json::from_str(&response.body).unwrap();
    public.validate(&q.request_id).unwrap();
    assert_eq!(public.status, AnswerStatus::BuildRejected);
    assert!(public.text.contains(&public.citations[0].label));
    assert!(!response.body.contains(SOURCE));
    assert!(!response.body.contains("parser_revision"));
    assert!(!response.body.contains("allowed_scopes"));
}
#[tokio::test]
async fn ambiguous_item_alias_is_unknown_and_never_chooses_the_first_match() {
    let mut data = records("r1", PATCH, 1, "500");
    let index = data.iter().position(|r| r.logical_id == "catalog").unwrap();
    let mut object: StoredDomainObject = serde_json::from_str(&data[index].content).unwrap();
    if let DomainObject::BuildCatalog(catalog) = &mut object.object {
        let mut alias = catalog.aliases[0].clone();
        alias.item_id = 103;
        catalog.aliases.push(alias);
    }
    data[index] = typed("catalog", 1, object.object, PATCH);
    let s = store(data).await;
    let a = answer(&s, &build("Prüfheld", "de", &["Prüfobjekt 101"]));
    assert_eq!(a.status, AnswerStatus::InsufficientEvidence);
    assert_eq!(certificate(&a).verdict, DomainVerdict::Unknown);
}
#[tokio::test]
async fn upstream_unknown_legality_fields_survive_normalized_model_defaults() {
    let mut data = records("r1", PATCH, 1, "500");
    let index = data.iter().position(|r| r.logical_id == "catalog").unwrap();
    let mut object: StoredDomainObject = serde_json::from_str(&data[index].content).unwrap();
    if let DomainObject::BuildCatalog(catalog) = &mut object.object {
        catalog
            .unknown_legality_fields
            .insert("models/101/is_active".into());
    }
    data[index] = typed("catalog", 1, object.object, PATCH);
    let s = store(data).await;
    let a = answer(&s, &build("Fixture Hero", "en", &["101"]));
    assert_eq!(a.status, AnswerStatus::InsufficientEvidence);
    assert!(matches!(
        certificate(&a).build_evaluation.unwrap().legal,
        Observed::Unknown { .. }
    ));
}

#[tokio::test]
async fn contradictory_source_mode_cannot_be_overridden_by_a_public_catalog() {
    let mut data = records("r1", PATCH, 1, "500");
    data.iter_mut()
        .find(|r| r.logical_id == "models")
        .unwrap()
        .metadata
        .insert("mode".into(), "casual".into());
    let s = store(data).await;
    let a = answer(&s, &build("Fixture Hero", "en", &["101"]));
    assert_eq!(a.status, AnswerStatus::InsufficientEvidence);
    assert!(a.citations.is_empty());
}
#[tokio::test]
async fn deterministic_build_needs_no_provider_input_network_or_cost_budget() {
    let s = store(records("r1", PATCH, 1, "500")).await;
    let mut ctx = context("r1");
    ctx.budget.max_network_rounds = 0;
    ctx.budget.max_input_tokens = 0;
    ctx.budget.max_cost_micros = 0;
    ctx.principal.provider_egress.clear();
    let k = Kernel::new(ReleaseRetriever::new(s, 8), NoProvider);
    let a = k.answer(&query(&build("Fixture Hero", "en", &["101"]), PATCH), &ctx);
    assert_eq!(a.status, AnswerStatus::Answered);
    assert_eq!(a.usage, Usage::default());
}

#[derive(Clone)]
struct Counting(Arc<AtomicUsize>);
impl AnswerProviderPort for Counting {
    fn answer(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        evidence: &[Evidence],
    ) -> Result<ProviderAnswer, PortError> {
        self.0.fetch_add(1, Ordering::SeqCst);
        Ok(ProviderAnswer {
            text: "Kartenzusammenfassung".into(),
            cited_evidence_ids: evidence.iter().map(|e| e.evidence_id.clone()).collect(),
            usage: Usage::default(),
        })
    }
}
#[tokio::test]
async fn card_explanation_remains_generative_but_provider_egress_is_not_widened() {
    let data = records("r1", PATCH, 1, "500");
    let mut source = data
        .iter()
        .find(|r| r.logical_id == "mechanics")
        .unwrap()
        .clone();
    let s = store(data).await;
    let calls = Arc::new(AtomicUsize::new(0));
    let k = Kernel::new(ReleaseRetriever::new(s.clone(), 8), Counting(calls.clone()));
    let q = query(
        &DomainRequest::Card {
            hero: "Fixture Hero".into(),
            locale: "en".into(),
        },
        PATCH,
    );
    assert_eq!(k.answer(&q, &context("r1")).status, AnswerStatus::Answered);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    source.revision = 2;
    source.metadata.insert("egress".into(), "none".into());
    s.apply_record(source).unwrap();
    assert_eq!(
        k.answer(&q, &context("r1")).status,
        AnswerStatus::UnauthorizedEvidence
    );
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}
