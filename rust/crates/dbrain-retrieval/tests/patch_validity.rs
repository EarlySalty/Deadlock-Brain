//! Game validity is source evidence, not the release/index version.
use brain_contracts::{domain::*, source::*, value::*, *};
use brain_storage::{DomainReader, MemoryRepository};
use dbrain_retrieval::ReleaseRetriever;
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    result::Result,
};

fn record(patch: Option<&str>, legacy: Option<&str>) -> SourceRecordV2 {
    let content = "hero: Abrams\nhealth: 650";
    let mut record = SourceRecordV2 {
        source_id: "wiki".into(),
        logical_id: "entity/hero/Abrams".into(),
        revision: 1,
        content_hash: format!("{:x}", Sha256::digest(content)),
        content: content.into(),
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::new(),
        tombstone: false,
        valid_from: None,
        valid_to: None,
        metadata: BTreeMap::from([
            ("kind".into(), "fact".into()),
            ("mode".into(), "ranked".into()),
        ]),
    };
    if let Some(patch) = legacy {
        record.metadata.insert("patch".into(), patch.into());
    }
    bind(&mut record, patch);
    record
}
fn bind(record: &mut SourceRecordV2, patch: Option<&str>) {
    OriginArtifact {
        identity: SourceIdentity {
            source_id: record.source_id.clone(),
            logical_id: record.logical_id.clone(),
        },
        source_revision: SourceRevision::Wiki {
            page_id: 1,
            revision_id: record.revision as i64,
        },
        raw_sha256: record.content_hash.clone(),
        locator: "wiki:1".into(),
        parser_revision: "test-v1".into(),
        parser_family: "fixture".into(),
        schema_version: observed_option(None),
        schema_sha256: observed_option(None),
        retrieved_at: observed_option(None),
        source_time: observed_option(None),
        language: Observed::known("en".into()),
        origin_artifacts: BTreeSet::new(),
        derivation_family: observed_option(None),
        policy: SourcePolicy {
            visibility: record.visibility,
            allowed_scopes: record.allowed_scopes.clone(),
            authorization_ref: observed_option(None),
            license: observed_option(None),
            publication_allowed: true,
            provider_egress_allowed: true,
            raw_retention_allowed: true,
        },
        validity: GameValidity {
            patch: observed_option(patch.map(str::to_owned)),
            mode: Observed::known("ranked".into()),
            ..GameValidity::unknown()
        },
    }
    .bind_record(record)
    .unwrap();
}
fn query() -> Query {
    Query {
        request_id: "patch-q".into(),
        conversation_id: "patch-c".into(),
        text: "Abrams health".into(),
        requested_scopes: BTreeSet::new(),
        profile: AnswerProfile::Fact,
        patch: Some("p1".into()),
        mode: Some("ranked".into()),
        domain: None,
    }
}
fn context() -> AuthorizedContext {
    AuthorizedContext {
        discord: None,
        request_deadline: None,
        principal: Principal {
            actor_id: "patch-tester".into(),
            channel: "test".into(),
            scopes: BTreeSet::new(),
            provider_egress: BTreeSet::from(["public".into()]),
        },
        conversation_id: "patch-c".into(),
        knowledge_release: "r1".into(),
        deadline_ms: 8000,
        budget: Budget::default(),
    }
}
async fn published(source: SourceRecordV2) -> MemoryRepository {
    let store = MemoryRepository::default();
    store.apply_record(source).unwrap();
    let release = store
        .release_from_heads("r1", "index-version-not-a-game-patch", "p1")
        .unwrap();
    store.publish(&release).await.unwrap();
    store
}

#[tokio::test]
async fn canonical_old_patch_cannot_be_relabelled_by_new_release() {
    let retriever = ReleaseRetriever::new(published(record(Some("p0"), None)).await, 1);
    for profile in [AnswerProfile::Fact, AnswerProfile::Explain] {
        let mut q = query();
        q.profile = profile.clone();
        assert!(
            retriever.retrieve(&q, &context()).unwrap().is_empty(),
            "old source for {profile:?}"
        );
        assert!(retriever.records(&q, &context(), false).unwrap().is_empty());
    }
}

#[tokio::test]
async fn unpinned_query_preserves_source_patch_not_release_patch() {
    let retriever = ReleaseRetriever::new(published(record(Some("p0"), None)).await, 1);
    let mut q = query();
    q.patch = None;
    let hits = retriever.retrieve(&q, &context()).unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].patch.as_deref(), Some("p0"));
}

#[tokio::test]
async fn current_head_legacy_conflict_is_not_hidden_by_canonical_origin() {
    let source = record(Some("p1"), None);
    let store = published(source.clone()).await;
    let retriever = ReleaseRetriever::new(store.clone(), 1);
    let hits = retriever.retrieve(&query(), &context()).unwrap();
    let mut head = source;
    head.revision += 1;
    bind(&mut head, Some("p1"));
    head.metadata.insert("patch".into(), "p0".into());
    store.apply_record(head).unwrap();
    assert!(retriever.retrieve(&query(), &context()).unwrap().is_empty());
    assert!(retriever
        .validate_evidence(&query(), &context(), &hits, false)
        .is_err());
}

#[test]
fn static_lexical_adapter_uses_the_same_patch_contract() {
    for (patch, legacy, allowed) in [
        (Some("p0"), None, false),
        (Some("p1"), Some("p0"), false),
        (None, None, true),
        (Some("p1"), Some("null"), true),
    ] {
        let hits = dbrain_retrieval::LexicalRetriever::new(vec![record(patch, legacy)], 1)
            .retrieve(&query(), &context())
            .unwrap();
        assert_eq!(!hits.is_empty(), allowed);
        if let Some(hit) = hits.first() {
            assert_eq!(hit.patch.as_deref(), patch);
        }
    }
}

struct NoEmbedding;
impl EmbeddingProviderPort for NoEmbedding {
    fn embed(
        &self,
        _: &[String],
        _: &EmbeddingIdentity,
        _: &AuthorizedContext,
    ) -> Result<EmbeddingOutput, PortError> {
        panic!("patch-ineligible source must not trigger query embedding");
    }
}
#[tokio::test]
async fn dense_selection_rejects_old_patch_before_embedding() {
    for head_only in [false, true] {
        let source = record(Some(if head_only { "p1" } else { "p0" }), None);
        let store = published(source.clone()).await;
        if head_only {
            let mut head = source.clone();
            head.revision += 1;
            bind(&mut head, Some("p0"));
            store.apply_record(head).unwrap();
        }
        let index = dbrain_retrieval::DenseIndex {
            release_id: "r1".into(),
            identity: EmbeddingIdentity {
                provider: "fixture".into(),
                model: "fixture".into(),
                revision: "v1".into(),
                dimension: 2,
                pooling: "mean".into(),
                query_prefix: "q: ".into(),
                document_prefix: "d: ".into(),
                normalized: true,
            },
            entries: vec![dbrain_retrieval::DenseEntry {
                document: DocumentRevision {
                    source_id: source.source_id.clone(),
                    logical_id: source.logical_id.clone(),
                    revision: source.revision,
                    content_hash: source.content_hash.clone(),
                },
                vector: vec![1.0, 0.0],
            }],
        };
        let retriever =
            dbrain_retrieval::HybridRetriever::new(store, NoEmbedding, index, 1).unwrap();
        let mut q = query();
        q.profile = AnswerProfile::Explain;
        q.text = "survivability strategy".into();
        assert!(retriever.retrieve(&q, &context()).unwrap().is_empty());
    }
}

#[tokio::test]
async fn matching_known_patch_remains_usable() {
    let retriever = ReleaseRetriever::new(published(record(Some("p1"), None)).await, 1);
    let hits = retriever.retrieve(&query(), &context()).unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].patch.as_deref(), Some("p1"));
    retriever
        .validate_evidence(&query(), &context(), &hits, true)
        .unwrap();
}

#[tokio::test]
async fn unknown_patch_is_retrievable_but_never_invented_from_release() {
    for legacy in [None, Some("null"), Some("p1")] {
        let retriever = ReleaseRetriever::new(published(record(None, legacy)).await, 1);
        let hits = retriever.retrieve(&query(), &context()).unwrap();
        assert_eq!(
            hits.len(),
            1,
            "unknown source remains available, {legacy:?}"
        );
        assert_eq!(hits[0].patch, None, "unknown is not p1");
        retriever
            .validate_evidence(&query(), &context(), &hits, false)
            .unwrap();
        let records = retriever.records(&query(), &context(), false).unwrap();
        assert_eq!(records[0].metadata.get("patch").map(String::as_str), legacy);
    }
}

#[tokio::test]
async fn known_legacy_and_canonical_contradictions_fail_closed() {
    for (canonical, legacy) in [(Some("p0"), "p1"), (Some("p1"), "p0"), (None, "p0")] {
        let retriever = ReleaseRetriever::new(published(record(canonical, Some(legacy))).await, 1);
        assert!(
            retriever.retrieve(&query(), &context()).unwrap().is_empty(),
            "{canonical:?} / {legacy}"
        );
    }
    let retriever = ReleaseRetriever::new(published(record(Some("p1"), Some("null"))).await, 1);
    assert_eq!(
        retriever.retrieve(&query(), &context()).unwrap().len(),
        1,
        "legacy null is not a conflicting pin"
    );
}

#[tokio::test]
async fn legacy_only_patch_is_respected_without_inventing_unknown() {
    for (legacy, expected) in [
        (Some("p0"), 0),
        (Some("p1"), 1),
        (None, 1),
        (Some("null"), 1),
    ] {
        let mut source = record(None, legacy);
        source.metadata.remove(ORIGIN_METADATA_KEY);
        let retriever = ReleaseRetriever::new(published(source).await, 1);
        let hits = retriever.retrieve(&query(), &context()).unwrap();
        assert_eq!(hits.len(), expected);
        if let Some(hit) = hits.first() {
            assert_eq!(hit.patch.as_deref(), legacy.filter(|p| *p != "null"));
        }
    }
}

#[tokio::test]
async fn fresh_head_validity_revokes_retrieval_and_existing_evidence() {
    let source = record(Some("p1"), None);
    let store = published(source.clone()).await;
    let retriever = ReleaseRetriever::new(store.clone(), 1);
    let hits = retriever.retrieve(&query(), &context()).unwrap();
    assert_eq!(hits.len(), 1);
    let mut head = source;
    head.revision += 1;
    bind(&mut head, Some("p0"));
    store.apply_record(head.clone()).unwrap();
    assert!(retriever.retrieve(&query(), &context()).unwrap().is_empty());
    assert!(retriever
        .records(&query(), &context(), false)
        .unwrap()
        .is_empty());
    for provider in [false, true] {
        assert!(retriever
            .validate_evidence(&query(), &context(), &hits, provider)
            .is_err());
    }
    head.revision += 1;
    bind(&mut head, Some("p1"));
    store.apply_record(head).unwrap();
    assert_eq!(
        retriever.retrieve(&query(), &context()).unwrap(),
        hits,
        "unchanged historical pin is restored"
    );
}

#[tokio::test]
async fn new_head_cannot_rewrite_historical_pin_validity() {
    for historical in [Some("p0"), None] {
        let source = record(historical, None);
        let store = published(source.clone()).await;
        let mut head = source;
        head.revision += 1;
        bind(&mut head, Some("p1"));
        store.apply_record(head).unwrap();
        let hits = ReleaseRetriever::new(store, 1)
            .retrieve(&query(), &context())
            .unwrap();
        if historical.is_some() {
            assert!(hits.is_empty());
        } else {
            assert_eq!(hits.len(), 1);
            assert_eq!(hits[0].patch, None);
        }
    }
}

async fn with_reviewed_fact(source: SourceRecordV2) -> MemoryRepository {
    let store = MemoryRepository::default();
    let proof = DocumentRevision {
        source_id: source.source_id.clone(),
        logical_id: source.logical_id.clone(),
        revision: source.revision,
        content_hash: source.content_hash.clone(),
    };
    store.apply_record(source).unwrap();
    let content = serde_json::to_string(&StoredDomainObject {
        contract_version: DOMAIN_CONTRACT_VERSION.into(),
        object: DomainObject::NumericFact(NumericFact {
            fact_id: "health".into(),
            subject_id: "Abrams".into(),
            predicate: "health".into(),
            quantity: Quantity {
                value: 650.0,
                unit: "hp".into(),
            },
            validity: Validity {
                patch: "p1".into(),
                mode: "ranked".into(),
            },
            source: proof,
            verified: true,
        }),
    })
    .unwrap();
    let mut derived = record(Some("p1"), None);
    derived.logical_id = "reviewed/health".into();
    derived.content_hash = format!("{:x}", Sha256::digest(&content));
    derived.content = content;
    derived
        .metadata
        .insert("domain_contract".into(), DOMAIN_CONTRACT_VERSION.into());
    bind(&mut derived, Some("p1"));
    store.apply_record(derived).unwrap();
    let release = store.release_from_heads("r1", "knowledge1", "p1").unwrap();
    store.publish(&release).await.unwrap();
    store
}

#[tokio::test]
async fn typed_and_generic_paths_agree_on_source_validity() {
    for (patch, legacy, allowed) in [
        (Some("p0"), None, false),
        (Some("p1"), None, true),
        (None, None, true),
        (Some("p0"), Some("p1"), false),
        (Some("p1"), Some("p0"), false),
        (Some("p1"), Some("null"), true),
    ] {
        let store = with_reviewed_fact(record(patch, legacy)).await;
        let generic = ReleaseRetriever::new(store.clone(), 1)
            .retrieve(&query(), &context())
            .unwrap();
        let typed = DomainReader::new(store)
            .read_domain(
                &context(),
                &Validity {
                    patch: "p1".into(),
                    mode: "ranked".into(),
                },
            )
            .unwrap();
        assert_eq!(!generic.is_empty(), allowed, "generic {patch:?}/{legacy:?}");
        assert_eq!(
            !typed.facts.is_empty(),
            allowed,
            "typed {patch:?}/{legacy:?}"
        );
        if patch.is_none() && allowed {
            assert_eq!(
                generic[0].patch, None,
                "only the reviewed projection establishes typed validity"
            );
        }
    }
}

#[tokio::test]
async fn typed_and_generic_paths_both_recheck_changed_head_validity() {
    let source = record(Some("p1"), None);
    let store = with_reviewed_fact(source.clone()).await;
    let retriever = ReleaseRetriever::new(store.clone(), 1);
    assert_eq!(retriever.retrieve(&query(), &context()).unwrap().len(), 1);
    let validity = Validity {
        patch: "p1".into(),
        mode: "ranked".into(),
    };
    assert_eq!(
        DomainReader::new(store.clone())
            .read_domain(&context(), &validity)
            .unwrap()
            .facts
            .len(),
        1
    );
    let mut head = source;
    head.revision += 1;
    bind(&mut head, Some("p0"));
    store.apply_record(head).unwrap();
    assert!(retriever.retrieve(&query(), &context()).unwrap().is_empty());
    assert!(DomainReader::new(store)
        .read_domain(&context(), &validity)
        .unwrap()
        .facts
        .is_empty());
}
