use brain_contracts::{
    provider_input::{
        grounded_input_ceiling, grounded_turn_payload, transport_input_ceiling, ToolWireFormat,
    },
    *,
};
use brain_storage::MemoryRepository;
use dbrain_retrieval::{DenseEntry, DenseIndex, HybridRetriever, ReleaseRetriever};
use sha2::{Digest, Sha256};
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
        content_hash: format!("{:x}", Sha256::digest(content.as_bytes())),
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
        discord: None,
        request_deadline: None,
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
        answer_context: None,
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
async fn versions_mehrdeutige_daten_und_allgemeine_patchfragen_bleiben_suchbar() {
    let document = record(
        "patchnotes/2026-09-16",
        "Patchnotes: Version 0.5.1 und Patch 6.0.1. Zwischen 01.09. und 16.09. gab es Änderungen. Im Patch vom 16.09.2026 kam eine Kartenänderung.",
    );
    let store = published(vec![document.clone()]).await;
    let retriever = ReleaseRetriever::new(store, 6);
    for text in [
        "Was kam in Version 0.5.1?",
        "Was kam in Patch 6.0.1?",
        "Was änderte sich zwischen 01.09. und 16.09.?",
        "Was kam im Patch vom 16.09.2026?",
    ] {
        let mut question = query(text);
        question.patch = None;
        let evidence = retriever.retrieve(&question, &context()).unwrap();
        assert!(!evidence.is_empty(), "{text}");
        assert!(evidence
            .iter()
            .all(|item| item.logical_id == document.logical_id));
        retriever
            .validate_evidence(&question, &context(), &evidence, false)
            .unwrap();
    }
}

#[tokio::test]
async fn public_maintenance_citations_hide_private_origin_metadata() {
    let mut document = record(
        "public/hilfe.html",
        "<html><body><h1>Steam verbinden</h1><p>Öffentliche Hilfe öffnen.</p></body></html>",
    );
    let projection = dbrain_retrieval::html_projection::project_html(&document.content).unwrap();
    document.content_hash = projection.raw_sha256.clone();
    projection.bind_metadata(&mut document.metadata);
    document.source_id = "maintenance-docs:owned-target".into();
    document
        .metadata
        .insert("locator".into(), "/private/code/implementation.rs".into());
    document
        .metadata
        .insert("private_origin".into(), "interner Mechanismus".into());
    let store = published(vec![document]).await;
    let retriever = ReleaseRetriever::new(store, 6);
    let evidence = retriever
        .retrieve(&query("Steam verbinden"), &context())
        .unwrap();
    assert!(!evidence.is_empty());
    retriever
        .validate_evidence(&query("Steam verbinden"), &context(), &evidence, false)
        .unwrap();
    retriever
        .validate_evidence(&query("Steam verbinden"), &context(), &evidence, true)
        .unwrap();
    for hit in evidence {
        let provenance = hit.provenance.unwrap();
        assert_eq!(provenance.source_locator, "public/hilfe.html");
        assert!(!provenance.metadata.contains_key("private_origin"));
        assert!(!provenance.metadata.contains_key("locator"));
        assert!(!provenance.metadata.contains_key("html_raw_sha256"));
        assert!(!provenance.metadata.contains_key("html_semantic_sha256"));
    }
}

#[tokio::test]
async fn html_evidence_binds_semantic_ranges_to_unchanged_reviewed_artifact() {
    let html = "<html><body><h1 id='einrichtung'>Steam verbinden</h1><p>Nutze den Steam-Knopf.</p><img alt='Steam-Verbindung' src='https://invalid.test/no-fetch'><figcaption>Verbindung öffnen</figcaption><script>FalscheAdminBehauptung</script></body></html>";
    let projection = dbrain_retrieval::html_projection::project_html(html).unwrap();
    let mut original = record("steam.html", html);
    original.content_hash = projection.raw_sha256.clone();
    projection.bind_metadata(&mut original.metadata);
    let store = published(vec![original.clone()]).await;
    let retriever = ReleaseRetriever::new(store, 6);
    let q = query("Steam verbinden");
    let c = context();
    let hits = retriever.retrieve(&q, &c).unwrap();
    assert!(!hits.is_empty());
    for hit in &hits {
        let p = hit.provenance.as_ref().unwrap();
        assert_eq!(p.document.content_hash, projection.raw_sha256);
        assert_eq!(
            p.metadata["html_semantic_sha256"],
            projection.semantic_sha256
        );
        assert!(p.chunker_version.starts_with("html-semantic-v2+"));
        assert_eq!(hit.content, projection.text[p.byte_start..p.byte_end]);
        assert!(!hit.content.contains("<"));
        assert!(!hit.content.contains("FalscheAdminBehauptung"));
    }
    retriever.validate_evidence(&q, &c, &hits, true).unwrap();
    let mut forged = hits.clone();
    forged[0]
        .provenance
        .as_mut()
        .unwrap()
        .metadata
        .insert("html_semantic_sha256".into(), "0".repeat(64));
    assert!(retriever.validate_evidence(&q, &c, &forged, true).is_err());
    let mut wrong = original;
    wrong
        .metadata
        .insert("html_raw_sha256".into(), "0".repeat(64));
    let retriever = ReleaseRetriever::new(published(vec![wrong]).await, 6);
    assert!(retriever.retrieve(&q, &c).is_err());
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
    let transport_estimate = [ToolWireFormat::Native, ToolWireFormat::OpenAiCompatible]
        .into_iter()
        .map(|format| {
            let payload =
                grounded_turn_payload(&q, &hits, &[], &ToolConversation::default(), format)
                    .unwrap();
            transport_input_ceiling(&payload, true).unwrap()
        })
        .max()
        .unwrap();
    assert_eq!(estimate, transport_estimate);
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
async fn fact_alias_ambiguity_is_checked_across_the_entire_release() {
    let mut records = Vec::new();
    for i in 0..110 {
        let mut other = record(&format!("entity/hero/Other{i}"), "hero: Other\nhealth: 500");
        other.metadata.insert("kind".into(), "fact".into());
        records.push(other);
    }
    let mut first = record("entity/hero/Abrams", "hero: Abrams\nhealth: 650");
    first.metadata.insert("kind".into(), "fact".into());
    first
        .metadata
        .insert("aliases_de".into(), "Guardian; Wächter".into());
    records.push(first);
    let mut second = record("entity/hero/Warden", "hero: Warden\nhealth: 700");
    second.metadata.insert("kind".into(), "fact".into());
    second
        .metadata
        .insert("aliases_en".into(), "Guardian".into());
    records.push(second);
    let retriever = ReleaseRetriever::new(published(records).await, 1);
    let mut request = query("Guardian health");
    request.profile = AnswerProfile::Fact;
    assert!(retriever.retrieve(&request, &context()).unwrap().is_empty());
    request.text = "Wächter Gesundheit".into();
    assert_eq!(retriever.retrieve(&request, &context()).unwrap().len(), 1);
}

#[tokio::test]
async fn ineligible_alias_owners_do_not_hide_authorized_fact() {
    let mut first = record("entity/hero/Abrams", "hero: Abrams\nhealth: 650");
    first.metadata.insert("kind".into(), "fact".into());
    first
        .metadata
        .insert("aliases_en".into(), "Guardian".into());
    let mut second = record("entity/hero/Warden", "hero: Warden\nhealth: 700");
    second.metadata.insert("kind".into(), "fact".into());
    second
        .metadata
        .insert("aliases_en".into(), "Guardian".into());
    let mut request = query("Guardian health");
    request.profile = AnswerProfile::Fact;
    for variant in ["wrong_patch", "wrong_mode", "unauthorized"] {
        let mut other = second.clone();
        match variant {
            "wrong_patch" => {
                other.metadata.insert("patch".into(), "p0".into());
            }
            "wrong_mode" => {
                other.metadata.insert("mode".into(), "casual".into());
            }
            "unauthorized" => {
                other.visibility = SourceVisibility::Private;
                other.allowed_scopes.insert("private".into());
            }
            _ => unreachable!(),
        }
        let retriever = ReleaseRetriever::new(published(vec![first.clone(), other]).await, 1);
        let hits = retriever.retrieve(&request, &context()).unwrap();
        assert_eq!(hits.len(), 1, "{variant}");
        assert_eq!(hits[0].logical_id, first.logical_id, "{variant}");
    }
    let store = published(vec![first.clone(), second.clone()]).await;
    let retriever = ReleaseRetriever::new(store.clone(), 1);
    assert!(retriever.retrieve(&request, &context()).unwrap().is_empty());
    second.revision += 1;
    second.visibility = SourceVisibility::Private;
    second.allowed_scopes.insert("private".into());
    store.apply_record(second).unwrap();
    let hits = retriever.retrieve(&request, &context()).unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].logical_id, first.logical_id);
}

#[tokio::test]
async fn two_unambiguous_entities_in_one_question_are_not_alias_ambiguous() {
    let mut abrams = record("entity/hero/Abrams", "hero: Abrams\nhealth: 650");
    abrams.metadata.insert("kind".into(), "fact".into());
    let mut warden = record("entity/hero/Warden", "hero: Warden\nhealth: 700");
    warden.metadata.insert("kind".into(), "fact".into());
    let retriever = ReleaseRetriever::new(published(vec![abrams, warden]).await, 10);
    let mut request = query("Abrams Warden health");
    request.profile = AnswerProfile::Fact;
    let hits = retriever.retrieve(&request, &context()).unwrap();
    assert_eq!(hits.len(), 2);
}

#[tokio::test]
async fn repeated_owner_across_ambiguous_names_is_deduplicated_before_head_read() {
    let mut abrams = record("entity/hero/Abrams", "hero: Abrams\nhealth: 650");
    abrams.metadata.insert("kind".into(), "fact".into());
    abrams
        .metadata
        .insert("aliases_en".into(), "Abraham".into());
    let mut same_canonical = record("entity/hero/Other", "hero: Other\nhealth: 700");
    same_canonical.metadata.insert("kind".into(), "fact".into());
    same_canonical
        .metadata
        .insert("aliases_en".into(), "Abrams".into());
    let mut same_alias = record("entity/hero/Third", "hero: Third\nhealth: 800");
    same_alias.metadata.insert("kind".into(), "fact".into());
    same_alias
        .metadata
        .insert("aliases_en".into(), "Abraham".into());
    let retriever = ReleaseRetriever::new(
        published(vec![abrams, same_canonical, same_alias]).await,
        10,
    );
    let mut request = query("Abrams Abraham health");
    request.profile = AnswerProfile::Fact;
    assert!(retriever.retrieve(&request, &context()).unwrap().is_empty());
}

#[tokio::test]
async fn hundreds_of_alias_owners_fail_closed_without_exceeding_head_batch_limit() {
    let records = (0..300)
        .map(|number| {
            let mut owner = record(
                &format!("entity/hero/Hero{number}"),
                &format!("hero: Hero{number}\nhealth: 650"),
            );
            owner.metadata.insert("kind".into(), "fact".into());
            owner.metadata.insert("aliases_en".into(), "Shared".into());
            owner
        })
        .collect();
    let retriever = ReleaseRetriever::new(published(records).await, 1);
    let mut request = query("Shared health");
    request.profile = AnswerProfile::Fact;
    assert!(retriever.retrieve(&request, &context()).unwrap().is_empty());
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
    manifests: Arc<AtomicUsize>,
    document_batches: Arc<AtomicUsize>,
    document_keys: Arc<AtomicUsize>,
    head_keys: Arc<AtomicUsize>,
    largest_batch: Arc<AtomicUsize>,
}
impl SnapshotReadPort for Counted {
    fn read_manifest_until(
        &self,
        release: &str,
        deadline: Option<&RequestDeadline>,
    ) -> Result<ReleaseReadManifest, PortError> {
        self.manifests.fetch_add(1, Ordering::SeqCst);
        self.inner.read_manifest_until(release, deadline)
    }
    fn read_documents_until(
        &self,
        release: &str,
        documents: &[DocumentRevision],
        deadline: Option<&RequestDeadline>,
    ) -> Result<Vec<SourceRecordV2>, PortError> {
        self.document_batches.fetch_add(1, Ordering::SeqCst);
        self.document_keys
            .fetch_add(documents.len(), Ordering::SeqCst);
        self.inner
            .read_documents_until(release, documents, deadline)
    }
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
        manifests: Arc::new(AtomicUsize::new(0)),
        document_batches: Arc::new(AtomicUsize::new(0)),
        document_keys: Arc::new(AtomicUsize::new(0)),
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
    assert_eq!(store.snapshots.load(Ordering::SeqCst), 0);
    assert_eq!(store.manifests.load(Ordering::SeqCst), 8 * 10 * 3);
    assert_eq!(store.document_batches.load(Ordering::SeqCst), 1);
    assert_eq!(store.document_keys.load(Ordering::SeqCst), 4097);
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

#[tokio::test]
async fn normal_server_lanes_question_packs_matching_public_documentation() {
    let documents = [
        ("public/discord-server/tempvoice-guide.html", "TempVoice — die Anleitung\nÜberblick. TempVoice gibt jeder Gruppe automatisch ihren eigenen Sprachkanal. Diese Anleitung zeigt, wie eine Lane entsteht, was du als Owner steuerst und wie eine Lane wieder endet.\nEine Lane erstellen\nGeh in den sichtbaren Voice-Router. Ohne gespeicherten Standard erstellt der Bot zunächst eine Casual-Lane, zieht dich hinein und macht dich zum Owner. In der einmaligen Willkommens-DM kannst du anschließend Casual, Ranked oder Street Brawl und deine Voreinstellungen wählen. Mit Fertig übernimmst du die Wahl für deine bestehende Lane; es entsteht dadurch keine zweite Lane.\nErreicht dich die Willkommens-DM nicht, nutze die sichtbare Moduswahl im Router-Panel. Der gewählte Modus wird als Standard gespeichert; spätere Router-Beitritte verwenden ihn direkt. Die erste Casual-Lane allein speichert noch keinen Standard.\nDas Lane-Panel\nDeine Lane steuerst du komplett selbst über das sichtbare Lane-Panel. Als Owner kannst du die Lane umbenennen, ein Teilnehmerlimit setzen sowie "),
        ("public/discord-server/module/voice-lanes.html", "Voice-Lanes (TempVoice)\nÜberblick. Über den Voice-Router bekommst du eine eigene, selbst verwaltete Lane. Diese Seite erklärt, wie sie entsteht, was du als Owner tun kannst und wie Ranked-Lanes funktionieren.\nEigene Lane über den Router\nGeh in den sichtbaren Voice-Router. Ohne gespeicherten Standard erstellt der Bot zunächst eine Casual-Lane, zieht dich hinein und macht dich zum Owner. In der einmaligen Willkommens-DM kannst du anschließend Casual, Ranked oder Street Brawl und deine Voreinstellungen wählen. Mit Fertig übernimmst du die Wahl für deine bestehende Lane; es entsteht dadurch keine zweite Lane.\nErreicht dich die Willkommens-DM nicht, nutze die sichtbare Moduswahl im Router-Panel. Der gewählte Modus wird als Standard gespeichert; spätere Router-Beitritte verwenden ihn direkt. Die erste Casual-Lane allein speichert noch keinen Standard.\nOwner-Funktionen\nÜber das sichtbare Lane-Panel steuerst du deine Lane: umbenennen, ein Teilnehmerlimit setzen sowie Mitglieder kicken, bannen und wieder "),
        ("public/discord-server/voice-features.html", "Voice-Features\nÜberblick. Die Sprach-Lanes der Community: Wie du über den Router in eine Lane kommst, was du als Owner steuerst, wie Ranked-Lanes funktionieren, was ein Lane-Name anzeigt und wie du optional Feedback gibst.\nVoice-Router und Lanes\nGeh in den sichtbaren Voice-Router. Ohne gespeicherten Standard erstellt der Bot zunächst eine Casual-Lane, zieht dich hinein und macht dich zum Owner. In der einmaligen Willkommens-DM kannst du anschließend Casual, Ranked oder Street Brawl und deine Voreinstellungen wählen. Mit Fertig übernimmst du die Wahl für deine bestehende Lane; es entsteht dadurch keine zweite Lane.\nErreicht dich die Willkommens-DM nicht, nutze die sichtbare Moduswahl im Router-Panel. Der gewählte Modus wird als Standard gespeichert; spätere Router-Beitritte verwenden ihn direkt. Die erste Casual-Lane allein speichert noch keinen Standard.\nDeine Lane steuern\nAls Owner steuerst du deine Lane über das sichtbare Lane-Panel. Du kannst die Lane umbenennen, ein Teilnehmerlimit setzen sowie "),
        ("public/discord-server/ueber-bot-und-server.html", "Über den Bot und den Server\nKurz: Der Community-Bot unterstützt den Server bei Rollen, Voice-Lanes, Onboarding, Coaching und dem Willkommens-Hub. Für dein Anliegen führt jeweils ein sichtbarer Mitgliedsweg weiter.\nWas der Bot übernimmt\nIm Team-Abschnitt des Willkommens-Hubs ist der Bot als Server-Management sichtbar: Er kümmert sich unter anderem um Rollen, Voice-Lanes, das Onboarding, Coaching-Wege und den Hub selbst. Er wird von der Community laufend weiterentwickelt.\nEin paar Community-Fakten\nDie Deutsche Deadlock Community gibt es seit September 2024.\nDen Server finanziell unterstützen\nWer den Server freiwillig finanziell unterstützen möchte, hat zwei sichtbare Wege: den Discord-Server-Boost direkt in Discord und den sichtbaren Ko-fi-Weg der Community.\nDie Supporter-Rolle auf dem Server bekommst du über einen sichtbaren Ablauf. Die sichtbaren Boost-, Ko-fi- und Supporter-Wege haben jeweils ihre aktuellen sichtbaren Bedingungen und Vorteile; maßgeblich ist der jeweils aktuelle sichtbare "),
        ("public/discord-server/bots-und-dienste.html", "Bots und Dienste der Deadlock Community\nKurz: Auf dem Server gibt es diese Bots und Dienste: den Community-Bot im Discord, den Steam-Bot, den Twitch-Bot, den Patchnotes-Bot, das Turnierportal und die Website-Portale. Diese Übersicht nennt den Zweck und den sichtbaren Mitgliedsweg jedes Angebots, ohne wechselnde Namen oder nicht sichtbare Funktionen fest zu versprechen. Einen aktuellen Betriebszustand bestätigt sie nicht; bei einem Ausfall führt der sichtbare Menschen-Support weiter.\nCommunity-Bot und Discord-Server\nWas kann die Community alles für dich erledigen? Der Community-Bot unterstützt die sichtbaren Wege für Onboarding und Rollen, Voice-Lanes, Mitspielersuche, Coaching, Scrims und weitere Community-Werkzeuge. Beginne in Willkommen und Kanäle & Rollen; dort findest du die aktuell sichtbaren Bereiche und Einstiege. Von dieser Übersicht gelangst du außerdem zu Steam-Bot, Twitch-Bot, Patchnotes-Bot, Turnierportal und Website-Portalen. Einen kompakten Einstieg bietet Über den Bot und den Server.\n"),
        ("public/discord-server/module/mitspielersuche-lfg.html", "Benachrichtigungen für passende Gesuche sind optional und selbst wählbar.\nKlassische Textsuche als Alternative\nIst statt des Panels nur ein Textkanal für die Suche sichtbar, nutzt du die klassische Textsuche: Schreib dort eine klare Mitspieler-Suche mit Modus und gewünschter Gruppengröße und warte auf Antworten anderer Mitglieder. Automatische Bot-Vorschläge gibt es nur, wenn diese Suche ausdrücklich aktiviert ist.\nStrukturiertes Ranked-LFG\nBeim strukturierten Ranked-LFG wird ein verifizierter Rang verlangt. Das betrifft nur diesen strukturierten Weg — normale Ranked-Lanes im Voice sind davon getrennt und grundsätzlich offen.\nPraktischer Ablauf: Mitspieler finden · Übersicht: Community-Werkzeuge"),
        ("public/discord-server/haeufige-probleme.html", "kannst\nBefehle wie /faq müssen auf dem Community-Server ausgeführt werden, nicht in einer DM.\nGibt es schon einen offenen Fragechat, nutze ihn weiter oder beende ihn und starte neu.\nLane-Funktionen setzen voraus, dass du in einer Lane bist beziehungsweise ihr Owner bist.\nMeldet ein angebundener Dienst kurz einen Fehler, versuche die Aktion nach ein paar Sekunden erneut.\nSichtbarer Zustand und sicherer nächster Schritt\n| Was du siehst | Sicherer nächster Schritt\n| /faq wurde außerhalb des Servers benutzt | Den Befehl auf dem Community-Server erneut ausführen.\n| Es gibt bereits einen aktiven Fragechat | Den verlinkten Chat weiterverwenden oder dort Chat beenden nutzen und bei Bedarf neu starten.\n| Der Fragechat ließ sich mit einem gewöhnlichen Fehler nicht erstellen | Später erneut versuchen; bleibt es dabei, den sichtbaren Supportweg nutzen.\n| Die Chat-Anlage wurde ausdrücklich als technisch unsicher gemeldet | Keinen zweiten Chat starten, den sichtbaren Zustand prüfen und ein Ticket öffnen.\n| Der "),
        ("public/discord-server/custom-games.html", "Custom Games\nÜberblick. Normale Custom Games — also eigene Lobbys ohne Turnier- oder Scrim-Charakter — organisierst du im sichtbaren Custom-Games-Bereich. Zum Treffen dient der zugehörige Voice-Sammelpunkt.\nEin Custom Game organisieren\nEin normales Custom Game organisierst du im sichtbaren Custom-Games-Chat: Dort verabredest du Zeit, Modus und Teilnehmende. Der Bereich ist für spontane eigene Lobbys gedacht; für angemeldete Übungsspiele mit organisierter Lobby nutzt du stattdessen Scrims.\nVoice-Sammelpunkt\nZum gemeinsamen Treffen gibt es einen zugehörigen Voice-Sammelpunkt. Dort findet ihr euch für die eigene Lobby zusammen, bevor es losgeht. Wie Sprach-Lanes allgemein funktionieren, steht in den Voice-Features."),
        ("public/discord-server/community-tools.html", "einsenden\nGameplay-Clips reichst du über das sichtbare Clip-Panel ein. Ein Klick öffnet zuerst eine Einverständnis-Bestätigung: Du bestätigst, dass du selbst der Ersteller bist oder die Erlaubnis des Erstellers hast. Danach trägst du den vollständigen Web-Link zum Clip, einen Credit beziehungsweise Anzeigenamen und optional etwas Kontext ein; mindestens 1080p sind dabei Voraussetzung, nicht bloß eine Empfehlung. Nach dem Einsenden erlaubst du die freie Verwendung deines Clips; dabei wird dein Credit genannt. Nach dem Absenden bekommst du eine sichtbare Absende-Bestätigung, dass du die Einsendung abgeschickt hast; sie ist aber weder eine Zusage, dass der Clip zugestellt, noch dass er veröffentlicht wird.\nWeitere Gruppen-Werkzeuge\nFür organisierte Spiele gibt es eigene Seiten: Scrims für angemeldete Übungsspiele, Custom Games für eigene Lobbys, Coaching für Trainingsanfragen und Voice-Features für Sprach-Lanes."),
    ];
    let records = documents
        .iter()
        .map(|(id, content)| {
            let mut document = record(id, content);
            document.source_id = "maintenance-docs:public-fixture".into();
            let escape = |text: &str| {
                text.replace('&', "&amp;")
                    .replace('<', "&lt;")
                    .replace('>', "&gt;")
            };
            let (title, body) = content.split_once('\n').unwrap();
            document.content = format!(
                "<main><h1>{}</h1><p>{}</p></main>",
                escape(title),
                escape(body)
            );
            let projection =
                dbrain_retrieval::html_projection::project_html(&document.content).unwrap();
            document.content_hash = projection.raw_sha256.clone();
            projection.bind_metadata(&mut document.metadata);
            document
        })
        .collect();
    let store = published(records).await;
    let retriever = ReleaseRetriever::new(store.clone(), 6);
    let q = query("Welche Lanes gibt es auf dem Discord-Server?");
    let mut c = context();
    c.principal.provider_egress.clear();
    let hits = retriever.retrieve(&q, &c).unwrap();
    let matches = |hit: &Evidence| hit.content.contains("Casual, Ranked oder Street Brawl");
    assert!(
        matches(&hits[0]),
        "{:?}",
        hits.iter().map(|hit| &hit.logical_id).collect::<Vec<_>>()
    );
    for (id, _) in &documents[..3] {
        assert!(
            hits.iter().any(|hit| hit.logical_id == *id && matches(hit)),
            "{id}"
        );
    }
    retriever.validate_evidence(&q, &c, &hits, false).unwrap();
    let first = ReleaseRetriever::new(store, 1).retrieve(&q, &c).unwrap();
    assert!(matches(&first[0]));
    c.budget.max_input_tokens = grounded_input_ceiling(&q, &first) as u32;
    let packed = retriever.retrieve(&q, &c).unwrap();
    assert!(matches(&packed[0]));
    assert!(grounded_input_ceiling(&q, &packed) <= c.budget.max_input_tokens as u64);
    assert!(retriever
        .retrieve(&query("UnbekanntesQuantenportal"), &c)
        .unwrap()
        .is_empty());
}
