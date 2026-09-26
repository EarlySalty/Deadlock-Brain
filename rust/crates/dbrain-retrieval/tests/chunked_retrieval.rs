use brain_contracts::{
    provider_input::{grounded_input_ceiling, grounded_messages, transport_input_ceiling},
    *,
};
use brain_storage::MemoryRepository;
use dbrain_retrieval::{DenseEntry, DenseIndex, HybridRetriever, ReleaseRetriever};
use std::result::Result;
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

fn record(id: &str, content: &str) -> SourceRecordV2 {
    SourceRecordV2 {
        source_id: "wiki".into(),
        logical_id: id.into(),
        revision: 7,
        content_hash: format!("hash-{id}"),
        content: content.into(),
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::new(),
        tombstone: false,
        valid_from: Some("2026-09-25".into()),
        valid_to: None,
        metadata: BTreeMap::from([
            ("patch".into(), "p1".into()),
            ("mode".into(), "ranked".into()),
            ("locator".into(), format!("wiki:{id}@42")),
            ("upstream_revision".into(), "42".into()),
            ("parser".into(), "fixture-v1".into()),
        ]),
    }
}
async fn published(records: Vec<SourceRecordV2>) -> MemoryRepository {
    let store = MemoryRepository::default();
    for record in records {
        store.apply_record(record).unwrap();
    }
    let release = store.release_from_heads("r1", "knowledge1", "p1").unwrap();
    store.publish(&release).await.unwrap();
    store
}
fn context() -> AuthorizedContext {
    AuthorizedContext {
        principal: Principal {
            actor_id: "tester".into(),
            channel: "fixture".into(),
            scopes: BTreeSet::new(),
            provider_egress: BTreeSet::from(["public".into()]),
        },
        conversation_id: "c1".into(),
        knowledge_release: "r1".into(),
        deadline_ms: 8000,
        budget: Budget::default(),
    }
}
fn query(text: &str) -> Query {
    Query {
        request_id: "q1".into(),
        conversation_id: "c1".into(),
        text: text.into(),
        requested_scopes: BTreeSet::new(),
        profile: AnswerProfile::Explain,
        patch: Some("p1".into()),
        mode: Some("ranked".into()),
        domain: None,
    }
}

#[tokio::test]
async fn large_hero_default_budget_multiple_chunks_and_exact_provenance() {
    let mut content = String::from("# Abrams\n");
    for _ in 0..350 {
        content.push_str(
            "Abrams äöü 🎯 Siphon Life. Preserve every byte, newline and upstream datum.\n",
        );
    }
    content.push_str("\nAbrams BonusMaxHealthPerHero Max Health\n{\n  \"Key\": \"BonusMaxHealthPerHero\",\n  \"Value\": 650\n}\nTailMarkerImmutable\n");
    assert!(content.len() > 28_000);
    let original = record("abrams.md", &content);
    let store = published(vec![original.clone()]).await;
    let retriever = ReleaseRetriever::new(store.clone(), 6);
    let c = context();
    let q = query("Abrams");
    let hits = retriever.retrieve(&q, &c).unwrap();
    assert!(hits.len() > 1 && hits.len() <= 6);
    assert!(hits.iter().all(|hit| hit.content.len() < content.len()));
    let estimate = grounded_input_ceiling(&q, &hits);
    assert!(estimate <= 12_000);
    let transport = serde_json::json!({"model":"fixture", "messages":grounded_messages(&q, &hits), "max_tokens":2000});
    assert_eq!(estimate, transport_input_ceiling(&transport, true).unwrap());
    for hit in &hits {
        let p = hit.provenance.as_ref().unwrap();
        assert_eq!(p.metadata, original.metadata);
        assert_eq!(p.document.revision, original.revision);
        assert_eq!(p.document.content_hash, original.content_hash);
        assert_eq!(p.source_locator, "wiki:abrams.md@42");
        assert_eq!(p.valid_from, original.valid_from);
        assert_eq!(p.valid_to, original.valid_to);
        assert_eq!(p.release_id, "r1");
        assert_eq!(p.knowledge_version, "knowledge1");
        assert_eq!(hit.allowed_scopes, original.allowed_scopes);
        assert_eq!(hit.visibility, original.visibility);
        assert_eq!(hit.patch.as_deref(), Some("p1"));
        assert_eq!(hit.logical_id, "abrams.md");
        assert_eq!(hit.content, content[p.byte_start..p.byte_end]);
    }
    retriever.validate_evidence(&q, &c, &hits, true).unwrap();
    let exact = retriever
        .retrieve(&query("Abrams BonusMaxHealthPerHero 650"), &c)
        .unwrap();
    assert!(!exact.is_empty());
    assert!(exact[0]
        .content
        .contains("\"Key\": \"BonusMaxHealthPerHero\""));
    assert!(exact[0].content.contains("\"Value\": 650"));
    let tail = retriever
        .retrieve(&query("TailMarkerImmutable"), &c)
        .unwrap();
    assert!(!tail.is_empty());
    assert!(tail
        .iter()
        .any(|hit| hit.content.contains("TailMarkerImmutable")));
    // Rebuilding and cloning yield identical chunk identities, provenance and ranking.
    assert_eq!(
        hits,
        ReleaseRetriever::new(store, 6).retrieve(&q, &c).unwrap()
    );
    assert_eq!(hits, retriever.clone().retrieve(&q, &c).unwrap());
}

#[tokio::test]
async fn aliases_german_english_umlauts_and_metadata_filters() {
    let mut hero = record(
        "lady-geist.md",
        "Lady Geist health 650 damage ability cooldown",
    );
    hero.metadata
        .insert("aliases_de".into(), "Geisterdame; Grüne Lady".into());
    hero.metadata
        .insert("aliases_en".into(), "Lady Geist; Geist".into());
    let mut wrong = record("older.md", "Geisterdame health 900");
    wrong.metadata.insert("patch".into(), "p0".into());
    let retriever = ReleaseRetriever::new(published(vec![hero, wrong]).await, 6);
    let c = context();
    for term in [
        "Geisterdame",
        "Lady Geist",
        "geist",
        "GRUENE LADY",
        "Lebenspunkte",
        "Gesundheit",
        "Schaden",
        "Fähigkeit",
        "Abklingzeit",
    ] {
        let hits = retriever.retrieve(&query(term), &c).unwrap();
        assert!(!hits.is_empty(), "alias {term}");
        assert!(hits.iter().all(|e| e.logical_id == "lady-geist.md"));
    }
    let mut q = query("Geisterdame");
    q.mode = Some("street_brawl".into());
    assert!(retriever.retrieve(&q, &c).unwrap().is_empty());
    q.mode = None;
    q.patch = Some("p0".into());
    assert!(retriever.retrieve(&q, &c).is_err());
    let mut other = c;
    other.knowledge_release = "missing".into();
    assert!(retriever.retrieve(&query("Geisterdame"), &other).is_err());
}

#[tokio::test]
async fn live_revoke_delete_and_historical_acl_never_widen() {
    let mut original = record("restricted.md", "Abrams restricted evidence");
    original.visibility = SourceVisibility::Private;
    original.allowed_scopes.insert("old.scope".into());
    let store = published(vec![original.clone()]).await;
    let retriever = ReleaseRetriever::new(store.clone(), 6);
    let q = query("Abrams");
    let mut c = context();
    assert!(retriever.retrieve(&q, &c).unwrap().is_empty());
    c.principal.scopes.insert("old.scope".into());
    c.principal.provider_egress.insert("private".into());
    let hits = retriever.retrieve(&q, &c).unwrap();
    assert_eq!(hits.len(), 1);
    retriever.validate_evidence(&q, &c, &hits, true).unwrap();
    let mut head = original;
    head.revision += 1;
    head.visibility = SourceVisibility::Public;
    head.allowed_scopes.clear();
    store.apply_record(head.clone()).unwrap();
    assert!(retriever.retrieve(&q, &context()).unwrap().is_empty());
    let still_private = retriever.retrieve(&q, &c).unwrap();
    assert_eq!(still_private[0].visibility, SourceVisibility::Private);
    assert_eq!(
        still_private[0].allowed_scopes,
        BTreeSet::from(["old.scope".into()])
    );
    head.revision += 1;
    head.allowed_scopes.insert("new.scope".into());
    store.apply_record(head.clone()).unwrap();
    assert!(retriever.retrieve(&q, &c).unwrap().is_empty());
    assert!(matches!(
        retriever.validate_evidence(&q, &c, &hits, true),
        Err(PortError::PermissionDenied(_))
    ));
    c.principal.scopes.insert("new.scope".into());
    let restricted = retriever.retrieve(&q, &c).unwrap();
    assert_eq!(
        restricted[0].allowed_scopes,
        BTreeSet::from(["old.scope".into(), "new.scope".into()])
    );
    head.revision += 1;
    head.metadata.insert("egress".into(), "none".into());
    store.apply_record(head.clone()).unwrap();
    retriever
        .validate_evidence(&q, &c, &restricted, false)
        .unwrap();
    assert!(matches!(
        retriever.validate_evidence(&q, &c, &restricted, true),
        Err(PortError::PermissionDenied(_))
    ));
    head.revision += 1;
    head.tombstone = true;
    store.apply_record(head).unwrap();
    assert!(retriever.retrieve(&q, &c).unwrap().is_empty());
    assert!(matches!(
        retriever.validate_evidence(&q, &c, &restricted, false),
        Err(PortError::PermissionDenied(_))
    ));
}

#[tokio::test]
async fn forged_chunk_range_source_provenance_content_and_acl_are_rejected() {
    let retriever = ReleaseRetriever::new(
        published(vec![record("a", &"Abrams text. ".repeat(300))]).await,
        6,
    );
    let q = query("Abrams");
    let c = context();
    let hits = retriever.retrieve(&q, &c).unwrap();
    for variant in 0..7 {
        let mut forged = hits.clone();
        match variant {
            0 => forged[0].provenance.as_mut().unwrap().byte_start += 1,
            1 => forged[0].provenance.as_mut().unwrap().source_locator = "other".into(),
            2 => forged[0]
                .provenance
                .as_mut()
                .unwrap()
                .metadata
                .insert("mode".into(), "other".into())
                .map(|_| ())
                .unwrap(),
            3 => forged[0].content.push_str(" invented"),
            4 => {
                forged[0].allowed_scopes.insert("fake".into());
            }
            5 => forged[0].revision += 1,
            _ => forged[0].provenance.as_mut().unwrap().release_id = "other".into(),
        }
        assert!(matches!(
            retriever.validate_evidence(&q, &c, &forged, false),
            Err(PortError::PermissionDenied(_))
        ));
    }
}

#[derive(Clone)]
struct Counted {
    inner: MemoryRepository,
    snapshots: Arc<AtomicUsize>,
    head_keys: Arc<AtomicUsize>,
    largest_batch: Arc<AtomicUsize>,
}
impl SnapshotReadPort for Counted {
    fn read_snapshot(&self, release: &str) -> Result<CorpusSnapshot, PortError> {
        self.snapshots.fetch_add(1, Ordering::SeqCst);
        self.inner.read_snapshot(release)
    }
    fn read_heads(&self, docs: &[DocumentRevision]) -> Result<Vec<DocumentHead>, PortError> {
        self.head_keys.fetch_add(docs.len(), Ordering::SeqCst);
        self.largest_batch.fetch_max(docs.len(), Ordering::SeqCst);
        self.inner.read_heads(docs)
    }
}
#[tokio::test]
async fn four_thousand_documents_concurrent_queries_build_once_and_read_only_candidate_heads() {
    let mut records: Vec<_> = (0..4096)
        .map(|i| record(&format!("filler-{i}"), "Unrelated background material"))
        .collect();
    records.push(record("target", "UniqueNeedleAbrams"));
    let store = Counted {
        inner: published(records).await,
        snapshots: Arc::new(AtomicUsize::new(0)),
        head_keys: Arc::new(AtomicUsize::new(0)),
        largest_batch: Arc::new(AtomicUsize::new(0)),
    };
    let retriever = Arc::new(ReleaseRetriever::new(store.clone(), 6));
    let workers: Vec<_> = (0..8)
        .map(|_| {
            let retriever = retriever.clone();
            std::thread::spawn(move || {
                for _ in 0..10 {
                    let q = query("UniqueNeedleAbrams");
                    let c = context();
                    let hits = retriever.retrieve(&q, &c).unwrap();
                    assert_eq!(hits.len(), 1);
                    assert_eq!(hits[0].logical_id, "target");
                    retriever.validate_evidence(&q, &c, &hits, false).unwrap();
                    retriever.validate_evidence(&q, &c, &hits, true).unwrap();
                }
            })
        })
        .collect();
    for worker in workers {
        worker.join().unwrap();
    }
    assert_eq!(store.snapshots.load(Ordering::SeqCst), 1);
    assert_eq!(store.head_keys.load(Ordering::SeqCst), 8 * 10 * 3);
    assert_eq!(store.largest_batch.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn stat_identifiers_and_typed_facts_are_not_semantically_reranked() {
    let store = published(vec![
        record("correct", "Abrams BonusMaxHealthPerHero 650"),
        record("decoy", "Abrams unrelated prose"),
    ])
    .await;
    let retriever = HybridRetriever::new(store, NoEmbedding, dense_index(), 6).unwrap();
    let hits = retriever
        .retrieve(&query("Abrams BonusMaxHealthPerHero"), &context())
        .unwrap();
    assert_eq!(hits[0].logical_id, "correct");
    let mut fact = record("fact", "Abrams health 650");
    fact.metadata.insert("kind".into(), "fact".into());
    let store = published(vec![fact, record("decoy", "Abrams unrelated prose")]).await;
    let retriever = HybridRetriever::new(store, NoEmbedding, dense_index(), 6).unwrap();
    let (hits, usage) = retriever
        .retrieve_with_usage(&query("Abrams"), &context())
        .unwrap();
    assert!(hits.iter().any(|hit| hit.kind == EvidenceKind::Fact));
    assert_eq!(usage.network_rounds, 0);
}

#[tokio::test]
async fn weighted_fusion_is_bit_stable_under_list_permutation_and_never_merges_acls() {
    let retriever = ReleaseRetriever::new(
        published(vec![
            record("a", "Abrams alpha"),
            record("b", "Abrams beta"),
        ])
        .await,
        6,
    );
    let hits = retriever.retrieve(&query("Abrams"), &context()).unwrap();
    let mut reverse = hits.clone();
    reverse.reverse();
    reverse[0].score += 1.0;
    let one = vec![hits[0].clone()];
    let forward = dbrain_retrieval::fuse_ranked(
        &[hits.clone(), reverse.clone(), one.clone()],
        &[1, 7, 13],
        60,
        6,
    )
    .unwrap();
    let backward =
        dbrain_retrieval::fuse_ranked(&[one, reverse, hits.clone()], &[13, 7, 1], 60, 6).unwrap();
    assert_eq!(forward, backward);
    let mut conflicting = hits[0].clone();
    conflicting.allowed_scopes.insert("private".into());
    assert!(dbrain_retrieval::fuse_ranked(&[hits, vec![conflicting]], &[1, 1], 60, 6).is_err());
}

struct NoEmbedding;
impl EmbeddingProviderPort for NoEmbedding {
    fn embed(
        &self,
        _: &[String],
        _: &EmbeddingIdentity,
        _: &AuthorizedContext,
    ) -> Result<EmbeddingOutput, PortError> {
        panic!("exact-number or ineligible query must not egress to embedding provider")
    }
}
fn dense_index() -> DenseIndex {
    DenseIndex {
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
        entries: vec![DenseEntry {
            document: DocumentRevision {
                source_id: "wiki".into(),
                logical_id: "decoy".into(),
                revision: 7,
                content_hash: "hash-decoy".into(),
            },
            vector: vec![1.0, 0.0],
        }],
    }
}
#[tokio::test]
async fn exact_numeric_literals_bypass_dense_and_cannot_be_replaced_by_similar_numbers() {
    let store = published(vec![
        record("correct", "Abrams damage 6.5 health 650"),
        record("decoy", "Abrams damage 65 health 650"),
    ])
    .await;
    let retriever = HybridRetriever::new(store, NoEmbedding, dense_index(), 6).unwrap();
    let c = context();
    let hits = retriever.retrieve(&query("Abrams damage 6.5"), &c).unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].logical_id, "correct");
    for exact in ["-6.5", "6,5", "6.50", "6500"] {
        assert!(retriever
            .retrieve(&query(&format!("Abrams {exact}")), &c)
            .unwrap()
            .is_empty());
    }
    let mut wrong_mode = query("Abrams");
    wrong_mode.mode = Some("street_brawl".into());
    assert!(retriever.retrieve(&wrong_mode, &c).unwrap().is_empty());
}

#[tokio::test]
async fn oversized_indivisible_evidence_is_an_explicit_budget_error_not_truncation() {
    let text = format!("{} Abrams", "x".repeat(20_000));
    let mut atomic = record("a", &text);
    atomic.metadata.insert("kind".into(), "rule".into());
    let retriever = ReleaseRetriever::new(published(vec![atomic]).await, 1);
    assert!(matches!(
        retriever.retrieve(&query("Abrams"), &context()),
        Err(PortError::BudgetExceeded)
    ));
}
