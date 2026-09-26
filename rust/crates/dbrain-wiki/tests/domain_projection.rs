//! The real C4 extractor/projector reaches the C6 answer path, not a hand-built card.
use brain_contracts::{domain::*, *};
use brain_kernel::{AnswerKernelPort, Kernel};
use brain_storage::MemoryRepository;
use dbrain_retrieval::ReleaseRetriever;
use dbrain_s12_wiki_probe::knowledge::{
    extract, project_domain_card, MappingProfile, ProjectionReview, WikiIr,
};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::result::Result;
fn fixture() -> (WikiIr, CorpusRelease, ProjectionReview) {
    let mapping: MappingProfile = serde_json::from_str(include_str!(
        "../../../../architecture/migration/s12/fixtures/completion.mapping.json"
    ))
    .unwrap();
    let ir = extract(
        include_bytes!("../../../../architecture/migration/s12/fixtures/pilot.capture.json"),
        &mapping,
    )
    .unwrap();
    let mut pins: BTreeMap<String, BTreeMap<String, u64>> = BTreeMap::new();
    let mut revisions = BTreeMap::new();
    for source in ir.sources() {
        pins.entry(source.source_id.clone())
            .or_default()
            .insert(source.logical_id.clone(), source.revision);
        revisions.insert(
            source.logical_id.clone(),
            DocumentRevision {
                source_id: source.source_id.clone(),
                logical_id: source.logical_id.clone(),
                revision: source.revision,
                content_hash: source.content_hash.clone(),
            },
        );
    }
    let release = CorpusRelease {
        release_id: "wiki-domain-fixture".into(),
        knowledge_version: "fixture-v1".into(),
        patch: "fixture-p1".into(),
        created_at_epoch: ir.report().retrieved_at,
        source_revisions: pins,
    };
    let review = ProjectionReview {
        decision_ref: "synthetic-domain-game-validity-review".into(),
        source_revisions: revisions,
        approved_fields: ir
            .fields()
            .iter()
            .map(|f| (f.id.clone(), f.source_revision.clone()))
            .collect(),
    };
    (ir, release, review)
}
struct NoProvider;
impl AnswerProviderPort for NoProvider {
    fn answer(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: &[Evidence],
    ) -> Result<ProviderAnswer, PortError> {
        panic!("wiki fact called LLM")
    }
}
#[tokio::test]
async fn reviewed_wiki_card_is_retrievable_as_an_exact_source_authorized_fact() {
    let (ir, mut release, review) = fixture();
    let validity = Validity {
        patch: release.patch.clone(),
        mode: "ranked".into(),
    };
    let card = project_domain_card(&ir, 101, "de", &release, &validity, &review).unwrap();
    let fact = card
        .card
        .facts
        .iter()
        .find(|f| f.subject_id == card.card.hero_id)
        .unwrap()
        .clone();
    let hero_id = card.card.hero_id.clone();
    assert!(!card.fields.is_empty());
    assert!(card
        .fields
        .iter()
        .any(|f| f.condition.is_some() || f.variant.is_some() || !f.unknowns.is_empty()));
    let content = serde_json::to_string(&StoredDomainObject {
        contract_version: DOMAIN_CONTRACT_VERSION.into(),
        object: DomainObject::HeroCard(Box::new(card)),
    })
    .unwrap();
    let record = SourceRecordV2 {
        source_id: "wiki-domain-projection".into(),
        logical_id: "card".into(),
        revision: 1,
        content_hash: format!("{:x}", Sha256::digest(&content)),
        content,
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::new(),
        tombstone: false,
        valid_from: None,
        valid_to: None,
        metadata: BTreeMap::from([("domain_contract".into(), DOMAIN_CONTRACT_VERSION.into())]),
    };
    release
        .source_revisions
        .entry(record.source_id.clone())
        .or_default()
        .insert(record.logical_id.clone(), 1);
    let store = MemoryRepository::default();
    for source in ir.sources() {
        store.apply_record(source.clone()).unwrap();
    }
    store.apply_record(record).unwrap();
    store.publish(&release).await.unwrap();
    let context = AuthorizedContext {
        principal: Principal {
            actor_id: "reviewer".into(),
            channel: "test".into(),
            scopes: BTreeSet::from(["wiki.review".into()]),
            provider_egress: BTreeSet::new(),
        },
        conversation_id: "wiki-domain".into(),
        knowledge_release: release.release_id,
        deadline_ms: 5_000,
        budget: Budget::default(),
    };
    let q = Query {
        request_id: "wiki-domain".into(),
        conversation_id: context.conversation_id.clone(),
        text: "Exakter geprüfter Wikiwert".into(),
        domain: Some(DomainRequest::Fact {
            hero: hero_id,
            locale: "de".into(),
            predicate: fact.key.clone(),
        }),
        requested_scopes: BTreeSet::new(),
        profile: AnswerProfile::Fact,
        patch: Some(validity.patch),
        mode: Some(validity.mode),
    };
    let view = brain_storage::DomainReader::new(store.clone())
        .read_domain(
            &context,
            &Validity {
                patch: q.patch.clone().unwrap(),
                mode: q.mode.clone().unwrap(),
            },
        )
        .unwrap();
    assert_eq!(view.cards.len(), 1);
    let kernel = Kernel::new(ReleaseRetriever::new(store, 8), NoProvider);
    let result = kernel.answer(&q, &context);
    assert_eq!(result.status, AnswerStatus::Answered, "{}", result.text);
    assert!(result.text.contains(fact.value.as_str().unwrap()));
    assert_eq!(result.citations.len(), 1);
    assert_eq!(result.citations[0].visibility, SourceVisibility::Internal);
    assert!(result.citations[0].allowed_scopes.contains("wiki.review"));
    let proof: DomainAnswer = serde_json::from_str(&result.citations[0].content).unwrap();
    assert!(proof
        .inputs
        .iter()
        .any(|r| r.source == fact.source_revision));
    assert_eq!(result.usage, Usage::default());
}
#[test]
fn wiki_timestamps_are_not_accepted_as_implicit_patch_or_mode() {
    let (ir, release, review) = fixture();
    for (patch, mode) in [
        ("unknown", "ranked"),
        ("fixture-p1", "unknown"),
        ("fixture-p2", "ranked"),
    ] {
        assert!(project_domain_card(
            &ir,
            101,
            "de",
            &release,
            &Validity {
                patch: patch.into(),
                mode: mode.into()
            },
            &review
        )
        .is_err());
    }
}
