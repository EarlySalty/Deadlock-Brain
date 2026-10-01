//! Publication rights are separate from internal reading and provider egress.
use brain_contracts::{
    source::{GameValidity, OriginArtifact, SourceIdentity, SourcePolicy, SourceRevision},
    value::{Observed, UnknownReason},
    AnswerProfile, AnswerProviderPort, AnswerStatus, AuthorizedContext, Budget, DocumentStorePort,
    Evidence, PortError, Principal, ProviderAnswer, PublicAnswerResponse, Query, SourceRecordV2,
    SourceVisibility, Usage,
};
use brain_kernel::{AnswerKernelPort, CachedKernel, Kernel};
use brain_policy::{AuthGrant, CredentialRegistry, PolicyEngine};
use brain_storage::MemoryRepository;
use dbrain_retrieval::ReleaseRetriever;
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

fn scopes() -> BTreeSet<String> {
    BTreeSet::from(["docs.internal".into()])
}
fn record(id: &str, revision: u64, publication: bool) -> SourceRecordV2 {
    let mut record = SourceRecordV2 {
        source_id: format!("publication-{id}"),
        logical_id: "entity/hero/Abrams".into(),
        revision,
        content_hash: id.repeat(64),
        content: "Abrams health: 700".into(),
        visibility: SourceVisibility::Internal,
        allowed_scopes: scopes(),
        tombstone: false,
        valid_from: None,
        valid_to: None,
        metadata: BTreeMap::from([("kind".into(), "fact".into())]),
    };
    OriginArtifact {
        identity: SourceIdentity {
            source_id: record.source_id.clone(),
            logical_id: record.logical_id.clone(),
        },
        source_revision: SourceRevision::Api {
            api_version: "fixture-v1".into(),
            original_revision: Some(revision.to_string()),
        },
        raw_sha256: record.content_hash.clone(),
        locator: format!("fixture:{id}"),
        parser_revision: "fixture-v1".into(),
        parser_family: "fixture".into(),
        schema_version: Observed::unknown(UnknownReason::NotPresent),
        schema_sha256: Observed::unknown(UnknownReason::NotPresent),
        retrieved_at: Observed::unknown(UnknownReason::NotPresent),
        source_time: Observed::unknown(UnknownReason::NotPresent),
        language: Observed::known("en".into()),
        origin_artifacts: BTreeSet::new(),
        derivation_family: Observed::unknown(UnknownReason::NotPresent),
        policy: SourcePolicy {
            visibility: record.visibility,
            allowed_scopes: scopes(),
            authorization_ref: Observed::known("fixture-only".into()),
            license: Observed::unknown(UnknownReason::NotPresent),
            publication_allowed: publication,
            provider_egress_allowed: true,
            raw_retention_allowed: false,
        },
        validity: GameValidity::unknown(),
    }
    .bind_record(&mut record)
    .unwrap();
    record
}
fn query(profile: AnswerProfile) -> Query {
    Query {
        request_id: "publication-request".into(),
        conversation_id: "publication-conversation".into(),
        text: "Abrams health".into(),
        requested_scopes: scopes(),
        profile,
        patch: None,
        mode: None,
        domain: None,
    }
}
fn context() -> AuthorizedContext {
    AuthorizedContext {
        principal: Principal {
            actor_id: "fixture-actor".into(),
            channel: "test".into(),
            scopes: scopes(),
            provider_egress: BTreeSet::from(["internal".into()]),
        },
        conversation_id: "publication-conversation".into(),
        knowledge_release: "r1".into(),
        deadline_ms: 5000,
        budget: Budget::default(),
        request_deadline: None,
    }
}
async fn store(two: bool, publication: bool) -> MemoryRepository {
    let store = MemoryRepository::default();
    store
        .apply_record(record("a", 1, if two { true } else { publication }))
        .unwrap();
    if two {
        store.apply_record(record("b", 1, publication)).unwrap();
    }
    store
        .publish(&store.release_from_heads("r1", "v1", "p1").unwrap())
        .await
        .unwrap();
    store
}
struct Provider {
    calls: Arc<AtomicUsize>,
    revoke: Option<MemoryRepository>,
}
impl AnswerProviderPort for Provider {
    fn answer(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        evidence: &[Evidence],
    ) -> Result<ProviderAnswer, PortError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        assert_eq!(
            evidence.len(),
            2,
            "provider must receive A and B, not just cited A"
        );
        let a = evidence
            .iter()
            .find(|e| e.source_id == "publication-a")
            .unwrap();
        assert!(evidence.iter().any(|e| e.source_id == "publication-b"));
        if let Some(store) = &self.revoke {
            store.apply_record(record("b", 2, false)).unwrap();
        }
        Ok(ProviderAnswer {
            text: "derived from uncited B: 700".into(),
            cited_evidence_ids: vec![a.evidence_id.clone()],
            usage: Usage {
                network_rounds: 1,
                ..Usage::default()
            },
        })
    }
}
fn service<K: AnswerKernelPort>(kernel: K) -> brain_api::ApiService<K> {
    brain_api::ApiService::new(
        PolicyEngine::new(CredentialRegistry::new(vec![AuthGrant::from_secret(
            "fixture-token",
            "fixture-actor",
            "test",
            scopes(),
            BTreeSet::from(["internal".into()]),
        )])),
        kernel,
        "r1",
        5000,
        Budget::default(),
    )
}
fn answer<K: AnswerKernelPort>(
    service: &brain_api::ApiService<K>,
    query: &Query,
) -> PublicAnswerResponse {
    let result = service.handle_answer(
        Some("Bearer fixture-token"),
        &serde_json::to_vec(query).unwrap(),
    );
    assert_eq!(result.status, 200, "{}", result.body);
    let wire: serde_json::Value = serde_json::from_str(&result.body).unwrap();
    assert!(wire.get("dependencies").is_none());
    assert!(wire.get("purpose").is_none());
    assert!(wire.get("usage").is_none());
    serde_json::from_str(&result.body).unwrap()
}
fn denied(answer: &PublicAnswerResponse) {
    assert_eq!(
        answer.status,
        AnswerStatus::UnauthorizedEvidence,
        "{answer:?}"
    );
    assert!(answer.citations.is_empty());
    assert!(!answer.text.contains("700"));
    assert!(!answer.text.contains("uncited B"));
}

#[tokio::test]
async fn publication_internal_fact_remains_readable_but_external_api_denies_text_and_citations() {
    for publication in [false, true] {
        let store = store(false, publication).await;
        let calls = Arc::new(AtomicUsize::new(0));
        let kernel = Kernel::new(
            ReleaseRetriever::new(store, 10),
            Provider {
                calls: calls.clone(),
                revoke: None,
            },
        );
        let query = query(AnswerProfile::Fact);
        let internal = kernel.answer(&query, &context());
        assert_eq!(internal.status, AnswerStatus::Answered);
        assert!(internal.text.contains("700"));
        let external = answer(&service(kernel), &query);
        if publication {
            assert_eq!(external.status, AnswerStatus::Answered);
            assert!(external.text.contains("700"));
        } else {
            denied(&external);
        }
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }
}

#[tokio::test]
async fn publication_uncited_provider_dependency_blocks_external_output() {
    for publication in [false, true] {
        let calls = Arc::new(AtomicUsize::new(0));
        let kernel = Kernel::new(
            ReleaseRetriever::new(store(true, publication).await, 10),
            Provider {
                calls: calls.clone(),
                revoke: None,
            },
        );
        let result = answer(&service(kernel), &query(AnswerProfile::Explain));
        if publication {
            assert_eq!(result.status, AnswerStatus::Answered);
            assert_eq!(result.citations.len(), 1);
        } else {
            denied(&result);
        }
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
}

#[tokio::test]
async fn publication_current_head_revocation_during_provider_call_blocks_output() {
    let store = store(true, true).await;
    let kernel = Kernel::new(
        ReleaseRetriever::new(store.clone(), 10),
        Provider {
            calls: Arc::new(AtomicUsize::new(0)),
            revoke: Some(store),
        },
    );
    denied(&answer(&service(kernel), &query(AnswerProfile::Explain)));
}

#[tokio::test]
async fn publication_cache_revalidates_uncited_dependency_current_head() {
    let store = store(true, true).await;
    let calls = Arc::new(AtomicUsize::new(0));
    let cache = CachedKernel::new(
        Kernel::new(
            ReleaseRetriever::new(store.clone(), 10),
            Provider {
                calls: calls.clone(),
                revoke: None,
            },
        ),
        8,
        Duration::from_secs(30),
    );
    let service = service(cache);
    let query = query(AnswerProfile::Explain);
    assert_eq!(answer(&service, &query).status, AnswerStatus::Answered);
    assert_eq!(answer(&service, &query).status, AnswerStatus::Answered);
    assert_eq!(
        calls.load(Ordering::SeqCst),
        1,
        "second answer must be a cache hit"
    );
    store.apply_record(record("b", 2, false)).unwrap();
    denied(&answer(&service, &query));
    assert_eq!(
        calls.load(Ordering::SeqCst),
        1,
        "revoked hit must not retry the provider"
    );
}

#[tokio::test]
async fn publication_purpose_cannot_be_supplied_by_client() {
    let calls = Arc::new(AtomicUsize::new(0));
    let service = service(Kernel::new(
        ReleaseRetriever::new(store(false, false).await, 10),
        Provider {
            calls: calls.clone(),
            revoke: None,
        },
    ));
    for field in ["purpose", "output_purpose", "publication_allowed"] {
        let mut wire = serde_json::to_value(query(AnswerProfile::Fact)).unwrap();
        wire[field] = serde_json::json!("internal_read");
        let response = service.handle_answer(
            Some("Bearer fixture-token"),
            &serde_json::to_vec(&wire).unwrap(),
        );
        assert_eq!(response.status, 400);
    }
    assert_eq!(calls.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn publication_internal_only_kernel_adapter_fails_closed_at_api_boundary() {
    struct InternalOnly;
    impl AnswerKernelPort for InternalOnly {
        fn answer(&self, _: &Query, _: &AuthorizedContext) -> brain_contracts::AnswerResponse {
            panic!("an external API must not fall back to an internal-only kernel")
        }
    }
    let result = answer(&service(InternalOnly), &query(AnswerProfile::Fact));
    assert_eq!(result.status, AnswerStatus::Unavailable);
    assert!(result.citations.is_empty());
}

#[path = "support/domain_fixture.rs"]
mod domain_fixture;

#[tokio::test]
async fn publication_derived_domain_output_checks_nonprimary_current_head() {
    use brain_contracts::source::origin_from_record;
    let records = domain_fixture::records("r1", "synthetic-p1", 1, "500");
    let mut dependency = records
        .iter()
        .find(|r| r.logical_id == "models")
        .unwrap()
        .clone();
    let store = MemoryRepository::default();
    for record in records {
        store.apply_record(record).unwrap();
    }
    store
        .publish(
            &store
                .release_from_heads("r1", "v1", "synthetic-p1")
                .unwrap(),
        )
        .await
        .unwrap();
    let request = domain_fixture::query(
        &domain_fixture::build("Fixture Hero", "en", &["101", "102", "103"]),
        "synthetic-p1",
    );
    let mut c = context();
    c.conversation_id = request.conversation_id.clone();
    let calls = Arc::new(AtomicUsize::new(0));
    let kernel = Kernel::new(
        ReleaseRetriever::new(store.clone(), 10),
        Provider {
            calls: calls.clone(),
            revoke: None,
        },
    );
    let internal = kernel.answer(&request, &c);
    assert_eq!(internal.status, AnswerStatus::Answered);
    assert_ne!(internal.citations[0].logical_id, "models");
    assert_eq!(
        kernel
            .answer_for_purpose(
                &request,
                &c,
                brain_kernel::AnswerPurpose::ExternalPublication
            )
            .status,
        AnswerStatus::Answered
    );
    let mut origin = origin_from_record(&record("a", 1, false)).unwrap();
    dependency.revision += 1;
    dependency.content_hash = "e".repeat(64);
    origin.identity.source_id = dependency.source_id.clone();
    origin.identity.logical_id = dependency.logical_id.clone();
    origin.raw_sha256 = dependency.content_hash.clone();
    origin.policy.visibility = dependency.visibility;
    origin.policy.allowed_scopes = dependency.allowed_scopes.clone();
    origin.bind_record(&mut dependency).unwrap();
    store.apply_record(dependency).unwrap();
    assert_eq!(kernel.answer(&request, &c).status, AnswerStatus::Answered);
    denied(&answer(&service(kernel), &request));
    assert_eq!(calls.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn publication_hybrid_fact_output_uses_canonical_grants_without_embedding() {
    struct NoEmbedding;
    impl brain_contracts::EmbeddingProviderPort for NoEmbedding {
        fn embed(
            &self,
            _: &[String],
            _: &brain_contracts::EmbeddingIdentity,
            _: &AuthorizedContext,
        ) -> Result<brain_contracts::EmbeddingOutput, PortError> {
            panic!("fact output must not use embedding egress")
        }
    }
    for allowed in [false, true] {
        let index = dbrain_retrieval::DenseIndex {
            release_id: "r1".into(),
            entries: Vec::new(),
            identity: brain_contracts::EmbeddingIdentity {
                provider: "fixture".into(),
                model: "fixture".into(),
                revision: "v1".into(),
                dimension: 2,
                pooling: "mean".into(),
                query_prefix: String::new(),
                document_prefix: String::new(),
                normalized: true,
            },
        };
        let hybrid = dbrain_retrieval::HybridRetriever::new(
            store(false, allowed).await,
            NoEmbedding,
            index,
            10,
        )
        .unwrap();
        let result = answer(
            &service(Kernel::new(
                hybrid,
                Provider {
                    calls: Arc::new(AtomicUsize::new(0)),
                    revoke: None,
                },
            )),
            &query(AnswerProfile::Fact),
        );
        if allowed {
            assert_eq!(result.status, AnswerStatus::Answered);
        } else {
            denied(&result);
        }
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn publication_http_route_does_not_publish_internally_readable_fact() {
    let kernel = Kernel::new(
        ReleaseRetriever::new(store(false, false).await, 10),
        Provider {
            calls: Arc::new(AtomicUsize::new(0)),
            revoke: None,
        },
    );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let (stop, stopped) = tokio::sync::oneshot::channel();
    let server = tokio::spawn(async move {
        axum::serve(listener, brain_api::router(service(kernel)))
            .with_graceful_shutdown(async {
                let _ = stopped.await;
            })
            .await
            .unwrap();
    });
    let client = brain_client::AsyncBrainClient::new(
        &format!("http://{address}"),
        "fixture-token",
        Duration::from_secs(5),
    )
    .unwrap();
    let result = client.answer(&query(AnswerProfile::Fact)).await;
    stop.send(()).unwrap();
    server.await.unwrap();
    denied(&result.unwrap());
}
