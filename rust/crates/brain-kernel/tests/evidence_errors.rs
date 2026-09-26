use brain_contracts::*;
use brain_kernel::{AnswerKernelPort, CachedKernel, Kernel};
use brain_storage::{LocalPgReader, MemoryRepository};
use dbrain_retrieval::ReleaseRetriever;
use std::result::Result;
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

#[derive(Clone)]
struct FaultReader {
    inner: MemoryRepository,
    calls: Arc<AtomicUsize>,
    fail_at: usize,
    every: usize,
    error: PortError,
}
impl SnapshotReadPort for FaultReader {
    fn read_snapshot(&self, id: &str) -> Result<CorpusSnapshot, PortError> {
        self.inner.read_snapshot(id)
    }
    fn read_heads(&self, docs: &[DocumentRevision]) -> Result<Vec<DocumentHead>, PortError> {
        let call = self.calls.fetch_add(1, Ordering::SeqCst) + 1;
        if call == self.fail_at || (self.every > 0 && call.is_multiple_of(self.every)) {
            return Err(self.error.clone());
        }
        self.inner.read_heads(docs)
    }
}
struct Provider(Arc<AtomicUsize>);
impl AnswerProviderPort for Provider {
    fn answer(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        evidence: &[Evidence],
    ) -> Result<ProviderAnswer, PortError> {
        self.0.fetch_add(1, Ordering::SeqCst);
        Ok(ProviderAnswer {
            text: "Sensitive grounded answer".into(),
            cited_evidence_ids: vec![evidence[0].evidence_id.clone()],
            usage: Usage {
                input_tokens: 10,
                output_tokens: 5,
                network_rounds: 1,
                ..Usage::default()
            },
        })
    }
}
async fn fixture(fail_at: usize, every: usize, error: PortError) -> FaultReader {
    let inner = MemoryRepository::default();
    inner
        .apply_record(SourceRecordV2 {
            source_id: "fixture".into(),
            logical_id: "a".into(),
            revision: 1,
            content_hash: "fixture-hash".into(),
            content: "Abrams verified evidence".into(),
            visibility: SourceVisibility::Public,
            allowed_scopes: BTreeSet::new(),
            tombstone: false,
            valid_from: None,
            valid_to: None,
            metadata: BTreeMap::new(),
        })
        .unwrap();
    let release = inner.release_from_heads("r1", "v1", "p1").unwrap();
    inner.publish(&release).await.unwrap();
    FaultReader {
        inner,
        calls: Arc::new(AtomicUsize::new(0)),
        fail_at,
        every,
        error,
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
            actor_id: "actor".into(),
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

#[tokio::test]
async fn technical_reader_failures_are_unavailable_at_every_handoff_not_unauthorized() {
    for message in [
        "reader timeout",
        "database unavailable",
        "pool exhausted",
        "temporary reader failure",
    ] {
        // Head reads: retrieval=1, pre-answer=2, pre-provider=3, post-provider=4.
        for fail_at in 1..=4 {
            let reader = fixture(fail_at, 0, PortError::Unavailable(message.into())).await;
            let calls = Arc::new(AtomicUsize::new(0));
            let kernel = Kernel::new(ReleaseRetriever::new(reader, 6), Provider(calls.clone()));
            let answer = kernel.answer(&query(), &context());
            assert_eq!(
                answer.status,
                AnswerStatus::Unavailable,
                "{message}, read {fail_at}"
            );
            assert!(answer.citations.is_empty());
            assert!(!answer.text.contains("Sensitive"));
            assert!(!answer.text.contains(message));
            assert_eq!(calls.load(Ordering::SeqCst), usize::from(fail_at == 4));
            assert_eq!(answer.usage.network_rounds, u32::from(fail_at == 4));
        }
    }
}

#[tokio::test]
async fn permission_budget_and_invalid_reader_responses_are_distinct() {
    for (error, expected) in [
        (
            PortError::PermissionDenied("real denial".into()),
            AnswerStatus::UnauthorizedEvidence,
        ),
        (PortError::BudgetExceeded, AnswerStatus::BudgetExceeded),
        (
            PortError::InvalidResponse("malformed head JSON".into()),
            AnswerStatus::Unavailable,
        ),
        (
            PortError::Unavailable("timeout".into()),
            AnswerStatus::Unavailable,
        ),
    ] {
        for fail_at in 2..=4 {
            let reader = fixture(fail_at, 0, error.clone()).await;
            let calls = Arc::new(AtomicUsize::new(0));
            let answer = Kernel::new(ReleaseRetriever::new(reader, 6), Provider(calls.clone()))
                .answer(&query(), &context());
            assert_eq!(answer.status, expected);
            assert!(answer.citations.is_empty());
            assert_eq!(calls.load(Ordering::SeqCst), usize::from(fail_at == 4));
        }
    }
}

#[tokio::test]
async fn cache_hit_timeout_fails_closed_without_provider_retry_and_does_not_cache_failure() {
    let reader = fixture(
        5,
        0,
        PortError::Unavailable("cache head reader timeout".into()),
    )
    .await;
    let calls = Arc::new(AtomicUsize::new(0));
    let cache = CachedKernel::new(
        Kernel::new(ReleaseRetriever::new(reader, 6), Provider(calls.clone())),
        8,
        Duration::from_secs(30),
    );
    assert_eq!(
        cache.answer(&query(), &context()).status,
        AnswerStatus::Answered
    );
    let failed = cache.answer(&query(), &context());
    assert_eq!(failed.status, AnswerStatus::Unavailable);
    assert!(failed.citations.is_empty());
    assert_eq!(failed.usage.network_rounds, 0);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(
        cache.answer(&query(), &context()).status,
        AnswerStatus::Answered
    );
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn six_hundred_parallel_requests_with_reader_faults_never_report_false_permission_denials() {
    let reader = fixture(
        0,
        37,
        PortError::Unavailable("injected temporary reader outage".into()),
    )
    .await;
    let kernel = Arc::new(Kernel::new(
        ReleaseRetriever::new(reader, 6),
        Provider(Arc::new(AtomicUsize::new(0))),
    ));
    let threads: Vec<_> = (0..8)
        .map(|worker| {
            let kernel = kernel.clone();
            std::thread::spawn(move || {
                let mut answered = 0;
                let mut unavailable = 0;
                for id in (worker..600).step_by(8) {
                    let mut q = query();
                    q.request_id = format!("q-{id}");
                    let answer = kernel.answer(&q, &context());
                    match answer.status {
                        AnswerStatus::Answered => answered += 1,
                        AnswerStatus::Unavailable => {
                            unavailable += 1;
                            assert!(answer.citations.is_empty());
                        }
                        status => {
                            panic!("unexpected status for technical reader fault: {status:?}")
                        }
                    }
                }
                (answered, unavailable)
            })
        })
        .collect();
    let (answered, unavailable) = threads
        .into_iter()
        .map(|t| t.join().unwrap())
        .fold((0, 0), |(a, u), (b, v)| (a + b, u + v));
    assert_eq!(answered + unavailable, 600);
    assert!(answered > 0 && unavailable > 0);
}

#[test]
fn real_postgres_connection_failure_is_unavailable_not_permission_denied() {
    // A unique, non-existent local Unix socket: no production database or TCP connection.
    let missing =
        std::env::temp_dir().join(format!("brain-c2-c3-missing-socket-{}", std::process::id()));
    assert!(!missing.exists());
    let reader =
        LocalPgReader::new(&missing, 55439, "brain_core_test", "pilot_unavailable").unwrap();
    let doc = DocumentRevision {
        source_id: "fixture".into(),
        logical_id: "a".into(),
        revision: 1,
        content_hash: "hash".into(),
    };
    assert!(matches!(
        reader.read_heads(&[doc]),
        Err(PortError::Unavailable(_))
    ));
    assert!(matches!(
        reader.read_snapshot("r1"),
        Err(PortError::Unavailable(_))
    ));
}

#[test]
fn unavailable_roundtrips_through_the_public_response_contract() {
    let value = PublicAnswerResponse {
        contract_version: PUBLIC_API_VERSION.into(),
        request_id: "q1".into(),
        knowledge_release: "r1".into(),
        status: AnswerStatus::Unavailable,
        text: "Evidenz vorübergehend nicht verfügbar.".into(),
        citations: Vec::new(),
    };
    let json = serde_json::to_string(&value).unwrap();
    assert!(json.contains("\"status\":\"unavailable\""));
    let decoded: PublicAnswerResponse = serde_json::from_str(&json).unwrap();
    decoded.validate("q1").unwrap();
    assert_eq!(value, decoded);
}
