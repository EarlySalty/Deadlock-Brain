use crate::{config, Error};
use axum::{
    extract::{DefaultBodyLimit, State},
    http::{header, HeaderMap, StatusCode},
    routing::post,
    Json, Router,
};
use brain_contracts::{
    domain::DomainRequest, AnswerProfile, AuthorizedContext, Evidence, EvidenceKind, PortError,
    Query, RetrievalPort, SnapshotReadPort, SourceVisibility, Usage,
};
use brain_policy::CredentialRegistry;
use dbrain_reasoner::{PopulationItem, PopulationPrior};
use dbrain_retrieval::ReleaseRetriever;
use dbrain_sources::{
    AnalyticsKind, AnalyticsLookupRequest, AnalyticsObservation, DeadlockAnalyticsClient,
    PatchMembership,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tokio::sync::Semaphore;

pub(crate) struct AnalyticsRuntime {
    client: DeadlockAnalyticsClient,
    config: config::Analytics,
    credentials: CredentialRegistry,
    slots: Arc<Semaphore>,
    _scratch: tempfile::TempDir,
}

impl AnalyticsRuntime {
    pub(crate) fn new(
        config: config::Analytics,
        credentials: CredentialRegistry,
    ) -> Result<Self, Error> {
        let scratch = tempfile::tempdir().map_err(|_| Error::ConfigInvalid("analytics_scratch"))?;
        let http =
            dbrain_sources::core::http::HttpClient::new("brain-serve-analytics/1", scratch.path())
                .map_err(|_| Error::ConfigInvalid("analytics_http"))?;
        let client =
            DeadlockAnalyticsClient::new(http, Duration::from_millis(config.request_timeout_ms))
                .map_err(|_| Error::ConfigInvalid("analytics_http"))?;
        Ok(Self {
            client,
            config,
            credentials,
            slots: Arc::new(Semaphore::new(4)),
            _scratch: scratch,
        })
    }

    fn authorize(&self, headers: &HeaderMap, request: &AnalyticsLookupRequest) -> StatusCode {
        let Some(token) = headers
            .get(header::AUTHORIZATION)
            .and_then(|header| header.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "))
        else {
            return StatusCode::UNAUTHORIZED;
        };
        let Ok(principal) = self.credentials.authenticate(token) else {
            return StatusCode::UNAUTHORIZED;
        };
        if !principal.scopes.contains("analytics.internal") {
            return StatusCode::FORBIDDEN;
        }
        if request.patch != self.config.patch
            || request.min_unix_timestamp < self.config.min_unix_timestamp
            || request.max_unix_timestamp > self.config.max_unix_timestamp
            || request.max_rows > self.config.max_rows
            || request.validate().is_err()
        {
            return StatusCode::BAD_REQUEST;
        }
        StatusCode::OK
    }

    pub(crate) fn router(self: Arc<Self>) -> Router {
        Router::new()
            .route("/v1/analytics/observation", post(lookup))
            .layer(DefaultBodyLimit::max(1024))
            .with_state(self)
    }
}

const META_PREDICATE: &str = "analytics.hero_matches";
const POPULATION_PREDICATE: &str = "analytics.item_matches.";
const EVIDENCE_LIFETIME: Duration = Duration::from_secs(60);

#[derive(Clone, Copy)]
enum AnalyticsTarget {
    Meta { hero_id: u32 },
    Population { hero_id: u32, item_id: u32 },
}

fn analytics_target(query: &Query) -> Option<Option<AnalyticsTarget>> {
    let DomainRequest::Fact {
        hero,
        locale,
        predicate,
    } = query.domain.as_ref()?
    else {
        return None;
    };
    if !predicate.starts_with("analytics.") {
        return None;
    }
    if query.profile != AnswerProfile::Fact
        || query.patch.is_some()
        || query.mode.is_some()
        || locale != "de"
    {
        return Some(None);
    }
    let Some(hero_id) = hero.parse::<u32>().ok().filter(|id| *id > 0) else {
        return Some(None);
    };
    if predicate == META_PREDICATE {
        Some(Some(AnalyticsTarget::Meta { hero_id }))
    } else {
        let item_id = predicate
            .strip_prefix(POPULATION_PREDICATE)
            .and_then(|value| value.parse::<u32>().ok())
            .filter(|id| *id > 0);
        Some(item_id.map(|item_id| AnalyticsTarget::Population { hero_id, item_id }))
    }
}

fn analytics_key(query: &Query, context: &AuthorizedContext) -> Result<String, PortError> {
    let bytes = serde_json::to_vec(&(
        &query.conversation_id,
        &query.text,
        &query.domain,
        &query.requested_scopes,
        &query.profile,
        &query.patch,
        &query.mode,
        &context.principal,
        &context.knowledge_release,
    ))
    .map_err(|_| PortError::InvalidResponse("analytics query identity".into()))?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

fn hero_matches(observation: &AnalyticsObservation) -> Option<(u64, u64, u64)> {
    if !matches!(
        (observation.kind, observation.item_id),
        (AnalyticsKind::Meta, None) | (AnalyticsKind::Population, Some(1..))
    ) || observation.provenance.patch_membership != PatchMembership::Unverified
        || observation.rows.len() != 1
    {
        return None;
    }
    let row = &observation.rows[0];
    if row.get("hero_id")?.as_u64()? != u64::from(observation.hero_id)
        || row.get("bucket")?.as_i64()? != 0
    {
        return None;
    }
    let wins = row.get("wins")?.as_u64()?;
    let losses = row.get("losses")?.as_u64()?;
    let matches = row.get("matches")?.as_u64()?;
    (wins.checked_add(losses)? == matches).then_some((wins, losses, matches))
}

fn analytics_content(
    target: &AnalyticsTarget,
    meta: &AnalyticsObservation,
    population: Option<&AnalyticsObservation>,
) -> Option<String> {
    let hero_id = match target {
        AnalyticsTarget::Meta { hero_id } | AnalyticsTarget::Population { hero_id, .. } => hero_id,
    };
    if meta.kind != AnalyticsKind::Meta || meta.hero_id != *hero_id {
        return None;
    }
    let (wins, losses, matches) = hero_matches(meta)?;
    let from = meta.provenance.min_unix_timestamp;
    let to = meta.provenance.max_unix_timestamp;
    match target {
        AnalyticsTarget::Meta { hero_id } => Some(format!(
            "Held {hero_id}: {wins} Siege und {losses} Niederlagen in {matches} erfassten Partien mit Startzeit im Unix-Zeitraum [{from}, {to}]. Diese Beobachtung ist keinem Spielpatch zugeordnet."
        )),
        AnalyticsTarget::Population { hero_id, item_id } => {
            let population = population?;
            if population.kind != AnalyticsKind::Population
                || population.hero_id != *hero_id
                || population.item_id != Some(*item_id)
                || population.provenance.min_unix_timestamp != from
                || population.provenance.max_unix_timestamp != to
            {
                return None;
            }
            let (_, _, item_matches) = hero_matches(population)?;
            if matches == 0 || item_matches > matches {
                return None;
            }
            let prior = PopulationPrior::from_items([PopulationItem {
                item_id: i64::from(*item_id),
                prevalence: item_matches as f64 / matches as f64,
                median_position: None,
                is_staple: false,
            }]);
            let share = prior.prevalence(i64::from(*item_id)) * 100.0;
            Some(format!(
                "Held {hero_id}: Gegenstand {item_id} erscheint in {item_matches} von {matches} erfassten Partien mit Startzeit im Unix-Zeitraum [{from}, {to}] ({share:.1} %). Das ist eine beobachtete Matchhäufigkeit, keine Anzahl veröffentlichter Builds. Diese Beobachtung ist keinem Spielpatch zugeordnet."
            ))
        }
    }
}

pub(crate) struct AnalyticsRetriever<S> {
    release: ReleaseRetriever<S>,
    analytics: Option<Arc<AnalyticsRuntime>>,
    evidence: Mutex<BTreeMap<String, (Instant, String, Evidence)>>,
}

impl<S: SnapshotReadPort> AnalyticsRetriever<S> {
    pub(crate) fn new(
        release: ReleaseRetriever<S>,
        analytics: Option<Arc<AnalyticsRuntime>>,
    ) -> Self {
        Self {
            release,
            analytics,
            evidence: Mutex::new(BTreeMap::new()),
        }
    }

    fn retrieve_analytics(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        target: AnalyticsTarget,
    ) -> Result<(Vec<Evidence>, Usage), PortError> {
        self.retrieve_analytics_with_clock(query, context, target, Instant::now)
    }

    fn retrieve_analytics_with_clock(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        target: AnalyticsTarget,
        now: impl Fn() -> Instant,
    ) -> Result<(Vec<Evidence>, Usage), PortError> {
        let bound = context.with_request_deadline();
        let context: &AuthorizedContext = bound.as_ref();
        context.check_deadline()?;
        let Some(runtime) = self.analytics.as_ref() else {
            return Ok((Vec::new(), Usage::default()));
        };
        let deadline = now()
            + Duration::from_millis(
                context
                    .deadline_ms
                    .min(runtime.config.request_timeout_ms.saturating_mul(2)),
            );
        let deadline = deadline.min(
            context
                .request_deadline
                .as_ref()
                .expect("bound request deadline")
                .expires_at(),
        );
        if !query.requested_scopes.contains("analytics.internal")
            || !context.principal.scopes.contains("analytics.internal")
        {
            return Err(PortError::PermissionDenied(
                "analytics scope required".into(),
            ));
        }
        let release = self.release.snapshot(query, context)?;
        if now() >= deadline || context.check_deadline().is_err() {
            return Err(PortError::BudgetExceeded);
        }
        if release.release.patch != runtime.config.patch {
            return Ok((Vec::new(), Usage::default()));
        }
        let rounds = match target {
            AnalyticsTarget::Meta { .. } => 2,
            AnalyticsTarget::Population { .. } => 4,
        };
        if context.budget.max_network_rounds < rounds {
            return Err(PortError::BudgetExceeded);
        }
        let _permit = runtime
            .slots
            .clone()
            .try_acquire_owned()
            .map_err(|_| PortError::Unavailable("analytics capacity unavailable".into()))?;
        if now() >= deadline || context.check_deadline().is_err() {
            return Err(PortError::BudgetExceeded);
        }
        let hero_id = match target {
            AnalyticsTarget::Meta { hero_id } | AnalyticsTarget::Population { hero_id, .. } => {
                hero_id
            }
        };
        let request = |kind, item_id| AnalyticsLookupRequest {
            kind,
            hero_id,
            item_id,
            patch: runtime.config.patch.clone(),
            min_unix_timestamp: runtime.config.min_unix_timestamp,
            max_unix_timestamp: runtime.config.max_unix_timestamp,
            max_rows: runtime.config.max_rows,
        };
        let meta = runtime
            .client
            .lookup_with_request_deadline(
                &request(AnalyticsKind::Meta, None),
                deadline,
                context
                    .request_deadline
                    .as_ref()
                    .expect("bound request deadline"),
            )
            .map_err(|_| {
                context.check_deadline().err().unwrap_or_else(|| {
                    PortError::Unavailable("analytics meta lookup failed".into())
                })
            })?;
        if now() >= deadline || context.check_deadline().is_err() {
            return Err(PortError::BudgetExceeded);
        }
        let population = if let AnalyticsTarget::Population { item_id, .. } = target {
            Some(
                runtime
                    .client
                    .lookup_with_request_deadline(
                        &request(AnalyticsKind::Population, Some(item_id)),
                        deadline,
                        context
                            .request_deadline
                            .as_ref()
                            .expect("bound request deadline"),
                    )
                    .map_err(|_| {
                        context.check_deadline().err().unwrap_or_else(|| {
                            PortError::Unavailable("analytics population lookup failed".into())
                        })
                    })?,
            )
        } else {
            None
        };
        if now() >= deadline || context.check_deadline().is_err() {
            return Err(PortError::BudgetExceeded);
        }
        let usage = Usage {
            network_rounds: (meta.provenance.attempts
                + population
                    .as_ref()
                    .map_or(0, |value| value.provenance.attempts))
                as u32,
            ..Usage::default()
        };
        let Some(content) = analytics_content(&target, &meta, population.as_ref()) else {
            return Ok((Vec::new(), usage));
        };
        let key = analytics_key(query, context)?;
        let digest = format!(
            "{:x}",
            Sha256::digest(
                serde_json::to_vec(&(
                    &key,
                    &content,
                    &meta.provenance.raw_sha256,
                    population
                        .as_ref()
                        .map(|value| &value.provenance.raw_sha256),
                ))
                .map_err(|_| PortError::InvalidResponse("analytics evidence identity".into()))?
            )
        );
        let citation = format!(
            "{}#sha256={}{}",
            meta.provenance.locator,
            meta.provenance.raw_sha256,
            population
                .as_ref()
                .map_or_else(String::new, |value| format!(
                    ";{}#sha256={}",
                    value.provenance.locator, value.provenance.raw_sha256
                ))
        );
        let evidence = Evidence {
            evidence_id: format!("analytics-{digest}"),
            source_id: dbrain_sources::analytics_runtime::SOURCE.into(),
            logical_id: match target {
                AnalyticsTarget::Meta { .. } => format!("hero:{hero_id}:meta"),
                AnalyticsTarget::Population { item_id, .. } => {
                    format!("hero:{hero_id}:item:{item_id}:population")
                }
            },
            revision: 1,
            kind: match target {
                AnalyticsTarget::Meta { .. } => EvidenceKind::Fact,
                AnalyticsTarget::Population { .. } => EvidenceKind::Population,
            },
            content,
            citation,
            visibility: SourceVisibility::Internal,
            allowed_scopes: BTreeSet::from(["analytics.internal".into()]),
            score: 100.0,
            provenance: None,
            patch: None,
        };
        let mut cache = self
            .evidence
            .lock()
            .map_err(|_| PortError::Unavailable("analytics evidence lock unavailable".into()))?;
        context.check_deadline()?;
        cache.retain(|_, (observed, _, _)| observed.elapsed() < EVIDENCE_LIFETIME);
        if cache.len() >= 128 {
            if let Some(oldest) = cache
                .iter()
                .min_by_key(|(_, (observed, _, _))| observed)
                .map(|(id, _)| id.clone())
            {
                cache.remove(&oldest);
            }
        }
        cache.insert(
            evidence.evidence_id.clone(),
            (Instant::now(), key, evidence.clone()),
        );
        Ok((vec![evidence], usage))
    }
}

impl<S: SnapshotReadPort> RetrievalPort for AnalyticsRetriever<S> {
    fn retrieve(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Result<Vec<Evidence>, PortError> {
        self.retrieve_with_usage(query, context)
            .map(|(items, _)| items)
    }

    fn retrieve_with_usage(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Result<(Vec<Evidence>, Usage), PortError> {
        match analytics_target(query) {
            None => self.release.retrieve_with_usage(query, context),
            Some(None) => Ok((Vec::new(), Usage::default())),
            Some(Some(target)) => self.retrieve_analytics(query, context, target),
        }
    }

    fn validate_publication(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
    ) -> Result<(), PortError> {
        if analytics_target(query).is_none() {
            return self.release.validate_publication(query, context, evidence);
        }
        // Live analytics has its own immutable, scoped observation contract, not
        // an OriginArtifact release record. Preserve that existing authorization;
        // never let this wrapper downgrade release-source publication to reading.
        self.validate_evidence(query, context, evidence, false)
    }
    fn validate_evidence(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
        for_provider: bool,
    ) -> Result<(), PortError> {
        let Some(target) = analytics_target(query) else {
            return self
                .release
                .validate_evidence(query, context, evidence, for_provider);
        };
        if target.is_none()
            || for_provider
            || evidence.len() != 1
            || !query.requested_scopes.contains("analytics.internal")
            || !context.principal.scopes.contains("analytics.internal")
        {
            return Err(PortError::PermissionDenied(
                "analytics evidence not permitted".into(),
            ));
        }
        let release = self.release.snapshot(query, context)?;
        if self
            .analytics
            .as_ref()
            .is_none_or(|runtime| release.release.patch != runtime.config.patch)
        {
            return Err(PortError::PermissionDenied(
                "analytics release changed".into(),
            ));
        }
        let key = analytics_key(query, context)?;
        let cache = self
            .evidence
            .lock()
            .map_err(|_| PortError::Unavailable("analytics evidence lock unavailable".into()))?;
        match cache.get(&evidence[0].evidence_id) {
            Some((created, cached_key, cached))
                if created.elapsed() < EVIDENCE_LIFETIME
                    && cached_key == &key
                    && cached == &evidence[0]
                    && cached.patch.is_none() =>
            {
                Ok(())
            }
            _ => Err(PortError::PermissionDenied(
                "analytics evidence expired".into(),
            )),
        }
    }
}

async fn lookup(
    State(runtime): State<Arc<AnalyticsRuntime>>,
    headers: HeaderMap,
    Json(request): Json<AnalyticsLookupRequest>,
) -> (StatusCode, Json<Value>) {
    let deadline = tokio::time::Instant::now()
        + Duration::from_millis(runtime.config.request_timeout_ms.saturating_mul(2));
    let authorization = runtime.authorize(&headers, &request);
    if authorization != StatusCode::OK {
        return (
            authorization,
            Json(json!({"error": "analytics_request_rejected"})),
        );
    }
    if tokio::time::Instant::now() >= deadline {
        eprintln!(
            "{}",
            json!({"event": "analytics_lookup", "class": "deadline"})
        );
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"error": "analytics_unavailable"})),
        );
    }
    let Ok(Ok(permit)) =
        tokio::time::timeout_at(deadline, runtime.slots.clone().acquire_owned()).await
    else {
        eprintln!(
            "{}",
            json!({"event": "analytics_lookup", "class": "slot_timeout"})
        );
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"error": "analytics_busy"})),
        );
    };
    if tokio::time::Instant::now() >= deadline {
        eprintln!(
            "{}",
            json!({"event": "analytics_lookup", "class": "deadline"})
        );
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"error": "analytics_unavailable"})),
        );
    }
    let client = runtime.client.clone();
    let task = tokio::task::spawn_blocking(move || {
        let _permit = permit;
        client.lookup_with_deadline(&request, deadline.into_std())
    });
    match tokio::time::timeout_at(deadline, task).await {
        Ok(Ok(Ok(observation))) if tokio::time::Instant::now() < deadline => {
            (StatusCode::OK, Json(json!(observation)))
        }
        Ok(Ok(Ok(_))) => {
            eprintln!(
                "{}",
                json!({"event": "analytics_lookup", "class": "deadline"})
            );
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({"error": "analytics_unavailable"})),
            )
        }
        result => {
            let class = match result {
                Err(_) => "deadline",
                Ok(Err(_)) => "worker_join",
                Ok(Ok(Err(_))) => "source_error",
                Ok(Ok(Ok(_))) => unreachable!(),
            };
            eprintln!("{}", json!({"event": "analytics_lookup", "class": class}));
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({"error": "analytics_unavailable"})),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use brain_contracts::{
        AnswerProviderPort, AnswerStatus, Budget, CorpusRelease, CorpusSnapshot, Principal,
        ProviderAnswer, PublicAnswerResponse,
    };
    use brain_kernel::{AnswerKernelPort, CachedKernel, Kernel};
    use brain_policy::{AuthGrant, PolicyEngine};
    use dbrain_sources::AnalyticsKind;
    use std::{
        collections::BTreeSet,
        io::{Read, Write},
        net::TcpListener,
        sync::atomic::{AtomicU64, AtomicUsize, Ordering},
        thread,
    };

    fn runtime() -> AnalyticsRuntime {
        AnalyticsRuntime::new(
            config::Analytics {
                patch: "2026-09-24".into(),
                min_unix_timestamp: 1_790_000_000,
                max_unix_timestamp: 1_790_086_400,
                max_rows: 8,
                request_timeout_ms: 100,
                schema_sha256: dbrain_sources::schema_watch::OpenApiSnapshot::pinned()
                    .unwrap()
                    .schema_sha256,
            },
            CredentialRegistry::new(vec![AuthGrant::from_secret(
                "fixture-secret",
                "fixture-actor",
                "fixture-channel",
                BTreeSet::from(["analytics.internal".into()]),
                BTreeSet::new(),
            )]),
        )
        .unwrap()
    }

    fn fixture_request_headers(reader: &mut impl Read) -> std::io::Result<Vec<u8>> {
        let mut headers = Vec::new();
        while !headers.ends_with(b"\r\n\r\n") {
            if headers.len() == 8192 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "fixture request headers exceed limit",
                ));
            }
            let mut byte = [0];
            reader.read_exact(&mut byte)?;
            headers.push(byte[0]);
        }
        Ok(headers)
    }

    #[test]
    fn fixture_waits_for_complete_fragmented_request_headers() {
        struct Fragmented(std::io::Cursor<Vec<u8>>);
        impl Read for Fragmented {
            fn read(&mut self, bytes: &mut [u8]) -> std::io::Result<usize> {
                let count = bytes.len().min(3);
                self.0.read(&mut bytes[..count])
            }
        }
        let headers = b"GET /v1/analytics/hero-stats?hero=18 HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let mut request = Fragmented(std::io::Cursor::new(headers.to_vec()));
        assert_eq!(fixture_request_headers(&mut request).unwrap(), headers);
    }

    #[test]
    fn fixture_rejects_truncated_or_unbounded_request_headers() {
        let mut truncated = std::io::Cursor::new(b"GET / HTTP/1.1\r\nHost: localhost\r\n".to_vec());
        assert_eq!(
            fixture_request_headers(&mut truncated).unwrap_err().kind(),
            std::io::ErrorKind::UnexpectedEof
        );
        let mut oversized = std::io::Cursor::new(vec![b'x'; 8193]);
        assert_eq!(
            fixture_request_headers(&mut oversized).unwrap_err().kind(),
            std::io::ErrorKind::InvalidData
        );
    }

    struct FixedSnapshot;

    impl SnapshotReadPort for FixedSnapshot {
        fn read_snapshot(&self, release_id: &str) -> Result<CorpusSnapshot, PortError> {
            Ok(CorpusSnapshot {
                release: CorpusRelease {
                    release_id: release_id.into(),
                    knowledge_version: "fixture-v1".into(),
                    patch: "2026-09-24".into(),
                    created_at_epoch: 1_790_000_000,
                    source_revisions: BTreeMap::new(),
                },
                revisions: Vec::new(),
                heads: Vec::new(),
            })
        }
    }

    struct AdvancingSnapshot {
        elapsed_ms: Arc<AtomicU64>,
        advance_ms: u64,
        reads: Arc<AtomicUsize>,
    }

    impl SnapshotReadPort for AdvancingSnapshot {
        fn read_snapshot(&self, release_id: &str) -> Result<CorpusSnapshot, PortError> {
            self.reads.fetch_add(1, Ordering::SeqCst);
            self.elapsed_ms.store(self.advance_ms, Ordering::SeqCst);
            FixedSnapshot.read_snapshot(release_id)
        }
    }

    struct SecondSnapshotAdvancesClock {
        elapsed_ms: Arc<AtomicU64>,
        advance_ms: u64,
        reads: Arc<AtomicUsize>,
    }

    impl SnapshotReadPort for SecondSnapshotAdvancesClock {
        fn read_snapshot(&self, release_id: &str) -> Result<CorpusSnapshot, PortError> {
            match self.reads.fetch_add(1, Ordering::SeqCst) {
                0 => assert_eq!(self.elapsed_ms.load(Ordering::SeqCst), 0),
                1 => self.elapsed_ms.store(self.advance_ms, Ordering::SeqCst),
                read => panic!("unexpected snapshot read: {read}"),
            }
            FixedSnapshot.read_snapshot(release_id)
        }
    }

    struct DeniedProvider(Arc<AtomicUsize>);

    impl AnswerProviderPort for DeniedProvider {
        fn answer(
            &self,
            _query: &Query,
            _context: &AuthorizedContext,
            _evidence: &[Evidence],
        ) -> Result<ProviderAnswer, PortError> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Err(PortError::PermissionDenied(
                "fixture provider disabled".into(),
            ))
        }
    }

    fn answer_query(predicate: &str) -> Query {
        Query {
            request_id: "fixture-request".into(),
            conversation_id: "fixture-conversation".into(),
            text: "Held 18: erfasste Partien".into(),
            domain: Some(DomainRequest::Fact {
                hero: "18".into(),
                locale: "de".into(),
                predicate: predicate.into(),
            }),
            requested_scopes: BTreeSet::from(["analytics.internal".into()]),
            profile: AnswerProfile::Fact,
            patch: None,
            mode: None,
        }
    }

    #[test]
    fn cancelled_analytics_request_cannot_start_an_http_retry() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let deadline = brain_contracts::RequestDeadline::after(Duration::from_secs(3));
        let cancellation = deadline.clone();
        let calls = Arc::new(AtomicUsize::new(0));
        let seen = calls.clone();
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut bytes = [0; 4096];
            let count = stream.read(&mut bytes).unwrap();
            assert!(count > 0, "fixture must receive the first HTTP request");
            seen.fetch_add(1, Ordering::SeqCst);
            // Cancellation is synchronized with an actual first upstream request,
            // before the retryable response is delivered.
            cancellation.cancel();
            stream
                .write_all(
                    b"HTTP/1.1 503 Unavailable\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                )
                .unwrap();
            drop(stream);
            listener.set_nonblocking(true).unwrap();
            let end = Instant::now() + Duration::from_millis(500);
            while Instant::now() < end {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        let count = stream.read(&mut bytes).unwrap();
                        assert!(count > 0, "fixture must receive the retried HTTP request");
                        seen.fetch_add(1, Ordering::SeqCst);
                        stream.write_all(b"HTTP/1.1 503 Unavailable\r\nContent-Length: 0\r\nConnection: close\r\n\r\n").unwrap();
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::park_timeout(Duration::from_millis(2))
                    }
                    Err(error) => panic!("fixture listener: {error}"),
                }
            }
        });
        let mut runtime = runtime();
        let http = dbrain_sources::core::http::HttpClient::new(
            "cancelled-analytics-fixture",
            runtime._scratch.path(),
        )
        .unwrap();
        runtime.client = DeadlockAnalyticsClient::with_base_url(
            http,
            format!("http://{address}"),
            Duration::from_millis(1000),
        )
        .unwrap();
        runtime.config.request_timeout_ms = 1000;
        let retrieval = AnalyticsRetriever::new(
            ReleaseRetriever::new(FixedSnapshot, 8),
            Some(Arc::new(runtime)),
        );
        let context = AuthorizedContext {
            principal: Principal {
                actor_id: "fixture-actor".into(),
                channel: "fixture-channel".into(),
                scopes: BTreeSet::from(["analytics.internal".into()]),
                provider_egress: BTreeSet::new(),
            },
            conversation_id: "fixture-conversation".into(),
            knowledge_release: "fixture-release".into(),
            deadline_ms: 3000,
            request_deadline: Some(deadline),
            budget: Budget::default(),
        };
        let result = retrieval.retrieve_analytics(
            &answer_query(META_PREDICATE),
            &context,
            AnalyticsTarget::Meta { hero_id: 18 },
        );
        server.join().unwrap();
        assert_eq!(
            calls.load(Ordering::SeqCst),
            1,
            "a cancelled request started a second external HTTP operation"
        );
        assert!(matches!(result, Err(PortError::BudgetExceeded)));
    }

    #[test]
    fn snapshot_exhaustion_never_starts_meta_or_population_http() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let mut runtime = runtime();
        let budget_ms = runtime.config.request_timeout_ms.saturating_mul(2);
        let http = dbrain_sources::core::http::HttpClient::new(
            "brain-serve-snapshot-deadline-fixture",
            runtime._scratch.path(),
        )
        .unwrap();
        runtime.client = DeadlockAnalyticsClient::with_base_url(
            http,
            format!("http://{}", listener.local_addr().unwrap()),
            Duration::from_millis(runtime.config.request_timeout_ms),
        )
        .unwrap();
        let analytics = Arc::new(runtime);
        let elapsed_ms = Arc::new(AtomicU64::new(0));
        let reads = Arc::new(AtomicUsize::new(0));
        let context = AuthorizedContext {
            principal: Principal {
                actor_id: "fixture-actor".into(),
                channel: "fixture-channel".into(),
                scopes: BTreeSet::from(["analytics.internal".into()]),
                provider_egress: BTreeSet::new(),
            },
            conversation_id: "fixture-conversation".into(),
            knowledge_release: "fixture-release".into(),
            deadline_ms: budget_ms,
            request_deadline: None,
            budget: Budget::default(),
        };
        for target in [
            AnalyticsTarget::Meta { hero_id: 18 },
            AnalyticsTarget::Population {
                hero_id: 18,
                item_id: 42,
            },
        ] {
            elapsed_ms.store(0, Ordering::SeqCst);
            let retrieval = AnalyticsRetriever::new(
                ReleaseRetriever::new(
                    AdvancingSnapshot {
                        elapsed_ms: elapsed_ms.clone(),
                        advance_ms: budget_ms + 1,
                        reads: reads.clone(),
                    },
                    8,
                ),
                Some(analytics.clone()),
            );
            let started = Instant::now();
            let result = retrieval.retrieve_analytics_with_clock(
                &answer_query(META_PREDICATE),
                &context,
                target,
                || started + Duration::from_millis(elapsed_ms.load(Ordering::SeqCst)),
            );
            assert!(matches!(result, Err(PortError::BudgetExceeded)));
            assert_eq!(analytics.slots.available_permits(), 4);
            assert!(matches!(
                listener.accept(),
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock
            ));
        }
        assert_eq!(reads.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn ordinary_brain_answer_uses_meta_and_match_population_without_patch_claim() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let seen = Arc::new(Mutex::new(Vec::new()));
        let served = seen.clone();
        let server = thread::spawn(move || {
            for item_filter in [false, false, true] {
                let (mut stream, _) = listener.accept().unwrap();
                stream
                    .set_read_timeout(Some(Duration::from_secs(2)))
                    .unwrap();
                let bytes = fixture_request_headers(&mut stream).unwrap();
                let request = String::from_utf8_lossy(&bytes);
                assert!(request.starts_with("GET /v1/analytics/hero-stats?"));
                assert_eq!(request.contains("include_item_ids=42"), item_filter);
                assert!(request.contains("min_match_id=0"));
                assert!(request.contains("min_unix_timestamp=1790002800"));
                assert!(request.contains("max_unix_timestamp=1790082000"));
                served.lock().unwrap().push(request.to_string());
                let (wins, losses, matches) = if item_filter {
                    (6, 4, 10)
                } else {
                    (10, 10, 20)
                };
                let body = json!([{
                    "hero_id":18,"bucket":0,"wins":wins,"losses":losses,"matches":matches,
                    "matches_per_bucket":matches,"total_kills":100,"total_deaths":50,
                    "total_assists":200,"total_net_worth":300000,"total_last_hits":1000,
                    "total_denies":10,"total_player_damage":500000,
                    "total_player_damage_taken":400000,"total_boss_damage":10000,
                    "total_creep_damage":200000,"total_neutral_damage":50000,
                    "total_max_health":30000,"total_shots_hit":1000,"total_shots_missed":500
                }]);
                let bytes = serde_json::to_vec(&body).unwrap();
                write!(
                    stream,
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    bytes.len()
                )
                .unwrap();
                stream.write_all(&bytes).unwrap();
            }
        });
        let mut runtime = runtime();
        let http = dbrain_sources::core::http::HttpClient::new(
            "brain-serve-analytics-fixture",
            runtime._scratch.path(),
        )
        .unwrap();
        runtime.client = DeadlockAnalyticsClient::with_base_url(
            http,
            format!("http://{address}"),
            Duration::from_millis(500),
        )
        .unwrap();
        runtime.config.request_timeout_ms = 500;
        let retrieval = AnalyticsRetriever::new(
            ReleaseRetriever::new(FixedSnapshot, 8),
            Some(Arc::new(runtime)),
        );
        let calls = Arc::new(AtomicUsize::new(0));
        let kernel = CachedKernel::new(
            Kernel::new(retrieval, DeniedProvider(calls.clone())),
            4,
            Duration::from_secs(30),
        );
        let api = brain_api::ApiService::new(
            PolicyEngine::new(CredentialRegistry::new(vec![AuthGrant::from_secret(
                "fixture-secret",
                "fixture-actor",
                "fixture-channel",
                BTreeSet::from(["analytics.internal".into()]),
                BTreeSet::new(),
            )])),
            kernel,
            "fixture-release",
            2_000,
            Budget::default(),
        );
        let invoke = |query: &Query| {
            let response = api.handle_answer(
                Some("Bearer fixture-secret"),
                &serde_json::to_vec(query).unwrap(),
            );
            assert_eq!(response.status, 200, "{}", response.body);
            serde_json::from_str::<PublicAnswerResponse>(&response.body).unwrap()
        };
        let meta = invoke(&answer_query(META_PREDICATE));
        assert_eq!(meta.status, AnswerStatus::Answered);
        assert!(meta.text.contains("10 Siege"));
        assert!(meta.text.contains("keinem Spielpatch"));
        assert_eq!(meta.citations.len(), 1);
        let pop = invoke(&answer_query(&format!("{POPULATION_PREDICATE}42")));
        assert_eq!(pop.status, AnswerStatus::Answered);
        assert!(pop.text.contains("10 von 20"));
        assert!(pop.text.contains("50.0 %"));
        assert!(pop.text.contains("keinem Spielpatch"));
        assert_eq!(pop.citations.len(), 1);
        let mut patch = answer_query(META_PREDICATE);
        patch.patch = Some("2026-09-24".into());
        let denied = invoke(&patch);
        assert_eq!(denied.status, AnswerStatus::InsufficientEvidence);
        assert!(denied.citations.is_empty());
        let mut no_scope = answer_query(META_PREDICATE);
        no_scope.requested_scopes.clear();
        let denied = invoke(&no_scope);
        assert_eq!(denied.status, AnswerStatus::UnauthorizedEvidence);
        assert!(denied.citations.is_empty());
        server.join().unwrap();
        assert_eq!(seen.lock().unwrap().len(), 3);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        drop(api);
    }

    #[test]
    fn second_snapshot_exhaustion_rejects_direct_kernel_service_and_http_answers() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let requests = Arc::new(AtomicUsize::new(0));
        let observed = requests.clone();
        let server = thread::spawn(move || {
            for _ in 0..3 {
                let (mut stream, _) = listener.accept().unwrap();
                stream
                    .set_read_timeout(Some(Duration::from_secs(2)))
                    .unwrap();
                let bytes = fixture_request_headers(&mut stream).unwrap();
                assert!(bytes.starts_with(b"GET /v1/analytics/hero-stats?"));
                observed.fetch_add(1, Ordering::SeqCst);
                let body = json!([{
                    "hero_id":18,"bucket":0,"wins":10,"losses":10,"matches":20,
                    "matches_per_bucket":20,"total_kills":100,"total_deaths":50,
                    "total_assists":200,"total_net_worth":300000,"total_last_hits":1000,
                    "total_denies":10,"total_player_damage":500000,
                    "total_player_damage_taken":400000,"total_boss_damage":10000,
                    "total_creep_damage":200000,"total_neutral_damage":50000,
                    "total_max_health":30000,"total_shots_hit":1000,"total_shots_missed":500
                }]);
                let bytes = serde_json::to_vec(&body).unwrap();
                write!(
                    stream,
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    bytes.len()
                )
                .unwrap();
                stream.write_all(&bytes).unwrap();
            }
        });
        let mut runtime = runtime();
        runtime.config.request_timeout_ms = 500;
        let http = dbrain_sources::core::http::HttpClient::new(
            "brain-serve-second-snapshot-fixture",
            runtime._scratch.path(),
        )
        .unwrap();
        runtime.client = DeadlockAnalyticsClient::with_base_url(
            http,
            format!("http://{address}"),
            Duration::from_millis(500),
        )
        .unwrap();
        let analytics = Arc::new(runtime);
        let provider_calls = Arc::new(AtomicUsize::new(0));
        let make_kernel = || {
            let elapsed_ms = Arc::new(AtomicU64::new(0));
            let reads = Arc::new(AtomicUsize::new(0));
            let origin = Instant::now();
            let clock_elapsed = elapsed_ms.clone();
            let retrieval = AnalyticsRetriever::new(
                ReleaseRetriever::new(
                    SecondSnapshotAdvancesClock {
                        elapsed_ms,
                        advance_ms: 2_001,
                        reads: reads.clone(),
                    },
                    8,
                ),
                Some(analytics.clone()),
            );
            let kernel = Kernel::new(retrieval, DeniedProvider(provider_calls.clone())).with_clock(
                move || origin + Duration::from_millis(clock_elapsed.load(Ordering::SeqCst)),
            );
            (kernel, reads)
        };
        let request = answer_query(META_PREDICATE);
        let context = AuthorizedContext {
            principal: Principal {
                actor_id: "fixture-actor".into(),
                channel: "fixture-channel".into(),
                scopes: BTreeSet::from(["analytics.internal".into()]),
                provider_egress: BTreeSet::new(),
            },
            conversation_id: "fixture-conversation".into(),
            knowledge_release: "fixture-release".into(),
            deadline_ms: 2_000,
            request_deadline: None,
            budget: Budget::default(),
        };
        let (kernel, reads) = make_kernel();
        let direct = kernel.answer(&request, &context);
        assert_eq!(direct.status, AnswerStatus::BudgetExceeded);
        assert!(direct.citations.is_empty());
        assert_eq!(reads.load(Ordering::SeqCst), 2);

        let make_service = || {
            let (kernel, reads) = make_kernel();
            let cached = CachedKernel::new(kernel, 4, Duration::from_secs(30));
            let policy = PolicyEngine::new(CredentialRegistry::new(vec![AuthGrant::from_secret(
                "fixture-secret",
                "fixture-actor",
                "fixture-channel",
                BTreeSet::from(["analytics.internal".into()]),
                BTreeSet::new(),
            )]));
            (
                brain_api::ApiService::new(
                    policy,
                    cached,
                    "fixture-release",
                    2_000,
                    Budget::default(),
                ),
                reads,
            )
        };
        let (service, reads) = make_service();
        let handled = service.handle_answer(
            Some("Bearer fixture-secret"),
            &serde_json::to_vec(&request).unwrap(),
        );
        assert_eq!(handled.status, 200);
        let public: PublicAnswerResponse = serde_json::from_str(&handled.body).unwrap();
        assert_eq!(public.status, AnswerStatus::BudgetExceeded);
        assert!(public.citations.is_empty());
        assert_eq!(reads.load(Ordering::SeqCst), 2);

        let (service, reads) = make_service();
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            let address = listener.local_addr().unwrap();
            let server = tokio::spawn(async move {
                axum::serve(listener, brain_api::router(service))
                    .await
                    .unwrap();
            });
            let response = reqwest::Client::new()
                .post(format!("http://{address}/v1/answer"))
                .bearer_auth("fixture-secret")
                .json(&request)
                .send()
                .await
                .unwrap();
            assert_eq!(response.status(), reqwest::StatusCode::OK);
            let public: PublicAnswerResponse = response.json().await.unwrap();
            assert_eq!(public.status, AnswerStatus::BudgetExceeded);
            assert!(public.citations.is_empty());
            server.abort();
            server.await.unwrap_err();
        });
        assert_eq!(reads.load(Ordering::SeqCst), 2);
        server.join().unwrap();
        assert_eq!(requests.load(Ordering::SeqCst), 3);
        assert_eq!(provider_calls.load(Ordering::SeqCst), 0);
        assert_eq!(analytics.slots.available_permits(), 4);
    }

    #[test]
    fn incongruent_population_cannot_become_a_match_prior() {
        let meta = AnalyticsObservation {
            kind: AnalyticsKind::Meta,
            hero_id: 18,
            item_id: None,
            rows: vec![json!({"hero_id":18,"bucket":0,"wins":10,"losses":10,"matches":20})],
            provenance: dbrain_sources::AnalyticsProvenance {
                source: "fixture".into(),
                locator: "fixture".into(),
                observed_at: 1_790_086_401,
                raw_sha256: "fixture".into(),
                schema_sha256: "fixture".into(),
                api_version: "fixture".into(),
                parser_revision: "fixture".into(),
                patch_membership: PatchMembership::Unverified,
                min_unix_timestamp: 1_790_000_000,
                max_unix_timestamp: 1_790_086_400,
                attempts: 1,
            },
        };
        let mut population = meta.clone();
        population.kind = AnalyticsKind::Population;
        population.item_id = Some(42);
        population.rows = vec![json!({"hero_id":18,"bucket":0,"wins":11,"losses":10,"matches":21})];
        assert!(analytics_content(
            &AnalyticsTarget::Population {
                hero_id: 18,
                item_id: 42
            },
            &meta,
            Some(&population),
        )
        .is_none());
        population.rows[0]["wins"] = json!(6);
        population.rows[0]["losses"] = json!(4);
        population.rows[0]["matches"] = json!(10);
        population.provenance.max_unix_timestamp += 1;
        assert!(analytics_content(
            &AnalyticsTarget::Population {
                hero_id: 18,
                item_id: 42
            },
            &meta,
            Some(&population),
        )
        .is_none());
        population.provenance.max_unix_timestamp = meta.provenance.max_unix_timestamp;
        population.rows[0] = json!({"hero_id":18,"bucket":0,"wins":0,"losses":0,"matches":0});
        let content = analytics_content(
            &AnalyticsTarget::Population {
                hero_id: 18,
                item_id: 42,
            },
            &meta,
            Some(&population),
        )
        .unwrap();
        assert!(content.contains("0 von 20"));
        assert!(content.contains("0.0 %"));
        let mut empty_meta = meta.clone();
        empty_meta.rows[0] = json!({"hero_id":18,"bucket":0,"wins":0,"losses":0,"matches":0});
        assert!(analytics_content(
            &AnalyticsTarget::Population {
                hero_id: 18,
                item_id: 42
            },
            &empty_meta,
            Some(&population),
        )
        .is_none());
    }

    #[test]
    fn delayed_fifth_lookup_keeps_original_deadline_after_slot_release() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let (arrived_tx, arrived_rx) = tokio::sync::oneshot::channel();
        let (release_tx, release_rx) = std::sync::mpsc::channel();
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut bytes = [0u8; 4096];
            let count = stream.read(&mut bytes).unwrap();
            let request = String::from_utf8_lossy(&bytes[..count]);
            assert!(request.starts_with("GET /v1/analytics/hero-stats?"));
            arrived_tx.send(()).unwrap();
            release_rx.recv().unwrap();
            stream
                .write_all(b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 2\r\nConnection: close\r\n\r\n[]")
                .unwrap();
        });
        let mut runtime = runtime();
        runtime.config.request_timeout_ms = 500;
        let http = dbrain_sources::core::http::HttpClient::new(
            "brain-serve-deadline-fixture",
            runtime._scratch.path(),
        )
        .unwrap();
        runtime.client = DeadlockAnalyticsClient::with_base_url(
            http,
            format!("http://{address}"),
            Duration::from_millis(500),
        )
        .unwrap();
        let analytics = Arc::new(runtime);
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            tokio::time::pause();
            let mut permits: Vec<_> = (0..4)
                .map(|_| analytics.slots.clone().try_acquire_owned().unwrap())
                .collect();
            let mut headers = HeaderMap::new();
            headers.insert(
                header::AUTHORIZATION,
                "Bearer fixture-secret".parse().unwrap(),
            );
            let request = AnalyticsLookupRequest {
                kind: AnalyticsKind::Meta,
                hero_id: 18,
                item_id: None,
                patch: "2026-09-24".into(),
                min_unix_timestamp: 1_790_000_000,
                max_unix_timestamp: 1_790_086_400,
                max_rows: 8,
            };
            let active = analytics.clone();
            let fifth = tokio::spawn(lookup(State(active), headers, Json(request)));
            tokio::task::yield_now().await;
            tokio::time::advance(Duration::from_millis(750)).await;
            drop(permits.pop());
            arrived_rx.await.unwrap();
            tokio::time::advance(Duration::from_millis(251)).await;
            let (status, body) = fifth.await.unwrap();
            assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
            assert_eq!(body.0["error"], "analytics_unavailable");
            release_tx.send(()).unwrap();
            drop(permits);
        });
        server.join().unwrap();
        drop(analytics);
        drop(rt);
    }

    #[test]
    fn fifth_lookup_expires_on_one_deadline_while_four_slots_are_occupied() {
        let analytics = Arc::new(runtime());
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            tokio::time::pause();
            let permits: Vec<_> = (0..4)
                .map(|_| analytics.slots.clone().try_acquire_owned().unwrap())
                .collect();
            let mut headers = HeaderMap::new();
            headers.insert(
                header::AUTHORIZATION,
                "Bearer fixture-secret".parse().unwrap(),
            );
            let request = AnalyticsLookupRequest {
                kind: AnalyticsKind::Meta,
                hero_id: 18,
                item_id: None,
                patch: "2026-09-24".into(),
                min_unix_timestamp: 1_790_000_000,
                max_unix_timestamp: 1_790_086_400,
                max_rows: 8,
            };
            let active = analytics.clone();
            let fifth = tokio::spawn(lookup(State(active), headers, Json(request)));
            tokio::task::yield_now().await;
            tokio::time::advance(Duration::from_millis(199)).await;
            assert!(!fifth.is_finished());
            tokio::time::advance(Duration::from_millis(2)).await;
            let (status, body) = fifth.await.unwrap();
            assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
            assert_eq!(body.0["error"], "analytics_busy");
            drop(permits);
            assert_eq!(analytics.slots.available_permits(), 4);
        });
        drop(analytics);
        drop(rt);
    }

    #[test]
    fn runtime_rejects_missing_grant_wrong_patch_and_unapproved_window() {
        let runtime = runtime();
        let mut request = AnalyticsLookupRequest {
            kind: AnalyticsKind::Population,
            hero_id: 18,
            item_id: Some(42),
            patch: "2026-09-24".into(),
            min_unix_timestamp: 1_790_000_000,
            max_unix_timestamp: 1_790_086_400,
            max_rows: 8,
        };
        assert_eq!(
            runtime.authorize(&HeaderMap::new(), &request),
            StatusCode::UNAUTHORIZED
        );
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            "Bearer fixture-secret".parse().unwrap(),
        );
        assert_eq!(runtime.authorize(&headers, &request), StatusCode::OK);
        request.patch = "wrong-patch".into();
        assert_eq!(
            runtime.authorize(&headers, &request),
            StatusCode::BAD_REQUEST
        );
        request.patch = "2026-09-24".into();
        request.min_unix_timestamp -= 1;
        assert_eq!(
            runtime.authorize(&headers, &request),
            StatusCode::BAD_REQUEST
        );
        request.min_unix_timestamp += 1;
        request.max_rows = 9;
        assert_eq!(
            runtime.authorize(&headers, &request),
            StatusCode::BAD_REQUEST
        );
    }
}
