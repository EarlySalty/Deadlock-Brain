use brain_contracts::*;
use brain_storage::MemoryRepository;
use dbrain_retrieval::{fuse_ranked, DenseEntry, DenseIndex, HybridRetriever, ReleaseRetriever};
use std::collections::{BTreeMap, BTreeSet};
fn record(id: &str, revision: u64) -> SourceRecordV2 {
    SourceRecordV2 {
        source_id: "fixture".into(),
        logical_id: id.into(),
        revision,
        content_hash: format!("{id}-{revision}"),
        content: format!("Abrams {id} revision {revision}"),
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::new(),
        tombstone: false,
        valid_from: None,
        valid_to: None,
        metadata: BTreeMap::new(),
    }
}
fn query() -> Query {
    Query {
        request_id: "q1".into(),
        conversation_id: "c1".into(),
        text: "Abrams".into(),
        requested_scopes: BTreeSet::new(),
        profile: AnswerProfile::Explain,
        patch: None,
        mode: None,
    }
}
fn context() -> AuthorizedContext {
    AuthorizedContext {
        principal: Principal {
            actor_id: "fixture".into(),
            channel: "test".into(),
            scopes: BTreeSet::new(),
            provider_egress: BTreeSet::from(["public".into()]),
        },
        conversation_id: "c1".into(),
        knowledge_release: "r1".into(),
        deadline_ms: 1000,
        budget: Budget::default(),
    }
}
async fn store() -> MemoryRepository {
    let store = MemoryRepository::default();
    store.apply_record(record("a", 1)).unwrap();
    store.apply_record(record("b", 1)).unwrap();
    let r = store.release_from_heads("r1", "v1", "p1").unwrap();
    store.publish(&r).await.unwrap();
    store
}
#[tokio::test]
async fn release_pinning_acl_tombstone_and_forged_evidence() {
    let store = store().await;
    let retriever = ReleaseRetriever::new(store.clone(), 10);
    let q = query();
    let c = context();
    let hits = retriever.retrieve(&q, &c).unwrap();
    assert_eq!(hits.len(), 2);
    assert!(retriever.validate_evidence(&q, &c, &hits, false).is_ok());
    let mut forged = hits.clone();
    forged[0].content = "made up".into();
    assert!(retriever.validate_evidence(&q, &c, &forged, false).is_err());
    store.apply_record(record("a", 2)).unwrap();
    assert_eq!(retriever.retrieve(&q, &c).unwrap(), hits);
    let mut restricted = record("a", 3);
    restricted.allowed_scopes.insert("private".into());
    store.apply_record(restricted).unwrap();
    assert!(retriever.validate_evidence(&q, &c, &hits, false).is_err());
    assert_eq!(retriever.retrieve(&q, &c).unwrap().len(), 1);
    let mut deleted = record("b", 2);
    deleted.tombstone = true;
    store.apply_record(deleted).unwrap();
    assert!(retriever.retrieve(&q, &c).unwrap().is_empty());
}
#[tokio::test]
async fn wrong_release_patch_mode_and_egress_are_closed() {
    let store = store().await;
    let retriever = ReleaseRetriever::new(store.clone(), 10);
    let mut c = context();
    c.knowledge_release = "unknown".into();
    assert!(retriever.retrieve(&query(), &c).is_err());
    c = context();
    let mut q = query();
    q.patch = Some("p2".into());
    assert!(retriever.retrieve(&q, &c).is_err());
    q = query();
    q.mode = Some("ranked".into());
    assert!(retriever.retrieve(&q, &c).unwrap().is_empty());
    let hits = retriever.retrieve(&query(), &c).unwrap();
    let mut no_egress = record("a", 2);
    no_egress.metadata.insert("egress".into(), "none".into());
    store.apply_record(no_egress).unwrap();
    assert!(retriever
        .validate_evidence(&query(), &c, &hits, false)
        .is_ok());
    assert!(retriever
        .validate_evidence(&query(), &c, &hits, true)
        .is_err());
}
#[tokio::test]
async fn fusion_is_stable_deduplicated_and_rejects_conflicts() {
    let retriever = ReleaseRetriever::new(store().await, 10);
    let hits = retriever.retrieve(&query(), &context()).unwrap();
    let mut reversed = hits.clone();
    reversed.reverse();
    let a = fuse_ranked(&[hits.clone(), reversed.clone()], &[1, 1], 60, 10).unwrap();
    let b = fuse_ranked(&[reversed, hits.clone()], &[1, 1], 60, 10).unwrap();
    assert_eq!(a, b);
    let dup = vec![hits[0].clone(), hits[0].clone()];
    let single = vec![hits[0].clone()];
    assert_eq!(
        fuse_ranked(&[dup], &[1], 60, 10).unwrap(),
        fuse_ranked(&[single], &[1], 60, 10).unwrap()
    );
    let mut bad = hits[0].clone();
    bad.content = "forged".into();
    assert!(fuse_ranked(&[hits, vec![bad]], &[1, 1], 60, 10).is_err());
}
fn identity() -> EmbeddingIdentity {
    EmbeddingIdentity {
        provider: "fixture".into(),
        model: "fixture".into(),
        revision: "1".into(),
        dimension: 2,
        pooling: "mean".into(),
        query_prefix: "q: ".into(),
        document_prefix: "d: ".into(),
        normalized: true,
    }
}
struct FixtureEmbedding {
    wrong: bool,
}
impl EmbeddingProviderPort for FixtureEmbedding {
    fn embed(
        &self,
        _inputs: &[String],
        model: &EmbeddingIdentity,
        _c: &AuthorizedContext,
    ) -> std::result::Result<EmbeddingOutput, PortError> {
        let mut identity = model.clone();
        if self.wrong {
            identity.revision = "other".into();
        }
        Ok(EmbeddingOutput {
            identity,
            vectors: vec![vec![1.0, 0.0]],
            usage: Usage {
                network_rounds: 1,
                input_tokens: 10,
                ..Usage::default()
            },
        })
    }
}
fn index() -> DenseIndex {
    DenseIndex {
        release_id: "r1".into(),
        identity: identity(),
        entries: vec![DenseEntry {
            document: DocumentRevision {
                source_id: "fixture".into(),
                logical_id: "b".into(),
                revision: 1,
                content_hash: "b-1".into(),
            },
            vector: vec![1.0, 0.0],
        }],
    }
}
#[tokio::test]
async fn dense_accounts_usage_and_rejects_model_mismatch() {
    let store = store().await;
    let hybrid = HybridRetriever::new(
        store.clone(),
        FixtureEmbedding { wrong: false },
        index(),
        10,
    )
    .unwrap();
    let (hits, usage) = hybrid.retrieve_with_usage(&query(), &context()).unwrap();
    assert_eq!(hits[0].logical_id, "b");
    assert_eq!(usage.network_rounds, 1);
    hybrid
        .validate_evidence(&query(), &context(), &hits, false)
        .unwrap();
    let bad = HybridRetriever::new(store, FixtureEmbedding { wrong: true }, index(), 10).unwrap();
    assert!(bad.retrieve(&query(), &context()).is_err());
    let mut c = context();
    c.budget.max_network_rounds = 0;
    let (lexical, usage) = bad.retrieve_with_usage(&query(), &c).unwrap();
    assert_eq!(lexical.len(), 2);
    assert_eq!(usage.network_rounds, 0);
}
