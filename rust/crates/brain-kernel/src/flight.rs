#[cfg(test)]
mod c3_shared_validation;
#[cfg(test)]
mod review_dependencies;
#[cfg(test)]
mod review_publication;
use super::*;
use std::{
    collections::BTreeMap,
    sync::{Arc, Condvar, Mutex},
    time::{Duration, Instant},
};
const MAX_SHARED_RESULT_BYTES: usize = 1024 * 1024;
#[derive(Default)]
pub(super) struct Coordinator {
    flights: Mutex<BTreeMap<String, Arc<Flight>>>,
}
#[derive(Default)]
struct Flight {
    state: Mutex<FlightState>,
    ready: Condvar,
}
#[derive(Default)]
struct FlightState {
    done: bool,
    result: Option<KernelAnswer>,
    waiters: usize,
}
fn unavailable() -> PortError {
    PortError::Unavailable("single-flight state unavailable".into())
}
struct Cleanup<'a> {
    coordinator: &'a Coordinator,
    key: String,
    flight: Arc<Flight>,
}
impl Drop for Cleanup<'_> {
    fn drop(&mut self) {
        if let Ok(mut state) = self.flight.state.lock() {
            state.done = true;
        }
        self.flight.ready.notify_all();
        if let Ok(mut flights) = self.coordinator.flights.lock() {
            if flights
                .get(&self.key)
                .is_some_and(|f| Arc::ptr_eq(f, &self.flight))
            {
                flights.remove(&self.key);
            }
        }
    }
}
impl Coordinator {
    #[cfg(test)]
    pub(super) fn wait_for_waiter(&self, key: &str) {
        let flight = self.flights.lock().unwrap().get(key).unwrap().clone();
        let mut state = flight.state.lock().unwrap();
        while state.waiters == 0 {
            state = flight.ready.wait(state).unwrap();
        }
    }

    fn run(
        &self,
        key: String,
        timeout: Duration,
        deadline: Option<&brain_contracts::RequestDeadline>,
        operation: impl FnOnce() -> KernelAnswer,
    ) -> Result<(KernelAnswer, bool), PortError> {
        let started = Instant::now();
        if let Some(deadline) = deadline {
            deadline.check()?;
        }
        let (flight, leader) = {
            let mut flights = self.flights.lock().map_err(|_| unavailable())?;
            if let Some(flight) = flights.get(&key) {
                (flight.clone(), false)
            } else {
                if flights.len() >= 128 {
                    return Err(unavailable());
                }
                let flight = Arc::new(Flight::default());
                flights.insert(key.clone(), flight.clone());
                (flight, true)
            }
        };
        if leader {
            let _cleanup = Cleanup {
                coordinator: self,
                key,
                flight: flight.clone(),
            };
            if started.elapsed() >= timeout || deadline.is_some_and(|d| d.check().is_err()) {
                return Err(PortError::BudgetExceeded);
            }
            let result = operation();
            {
                let mut state = flight.state.lock().map_err(|_| unavailable())?;
                state.result = Some(if result.retained_bytes() <= MAX_SHARED_RESULT_BYTES {
                    result.clone()
                } else {
                    AnswerResponse {
                        contract_version: result.answer.contract_version.clone(),
                        request_id: result.answer.request_id.clone(),
                        knowledge_release: result.answer.knowledge_release.clone(),
                        status: AnswerStatus::Unavailable,
                        text: "Antwort ist für die sichere Wiederverwendung zu groß.".into(),
                        citations: Vec::new(),
                        usage: Usage::default(),
                    }
                    .into()
                });
                state.done = true;
            }
            flight.ready.notify_all();
            Ok((result, false))
        } else {
            let mut state = flight.state.lock().map_err(|_| unavailable())?;
            state.waiters += 1;
            #[cfg(test)]
            flight.ready.notify_all();
            loop {
                let mut remaining = timeout.saturating_sub(started.elapsed());
                if let Some(deadline) = deadline {
                    remaining = remaining.min(deadline.remaining().unwrap_or(Duration::ZERO));
                }
                if remaining.is_zero() {
                    state.waiters -= 1;
                    return Err(PortError::BudgetExceeded);
                }
                if state.done {
                    break;
                }
                if deadline.is_some() {
                    remaining = remaining.min(Duration::from_millis(10));
                }
                let (next, _) = flight
                    .ready
                    .wait_timeout(state, remaining)
                    .map_err(|_| unavailable())?;
                state = next;
            }
            state.waiters -= 1;
            state
                .result
                .clone()
                .map(|r| (r, true))
                .ok_or_else(unavailable)
        }
    }
}
#[cfg(test)]
pub(super) fn cache_key(
    query: &Query,
    context: &AuthorizedContext,
) -> Result<String, serde_json::Error> {
    cache_key_for_purpose(query, context, AnswerPurpose::InternalRead)
}
#[cfg(test)]
pub(super) fn cache_key_for_purpose(
    query: &Query,
    context: &AuthorizedContext,
    purpose: AnswerPurpose,
) -> Result<String, serde_json::Error> {
    request_key(query, context, purpose, None)
}
pub(super) fn request_key(
    query: &Query,
    context: &AuthorizedContext,
    purpose: AnswerPurpose,
    session: Option<&ToolSession>,
) -> Result<String, serde_json::Error> {
    serde_json::to_string(&(
        "brain.policy.v3",
        purpose,
        &context.principal,
        &context.conversation_id,
        &context.knowledge_release,
        &context.budget,
        &query.text,
        &query.domain,
        &query.profile,
        &query.patch,
        &query.mode,
        &query.requested_scopes,
        session.map(|session| {
            (
                &session.game_context,
                &session.definitions,
                &session.provider_identity,
            )
        }),
    ))
}
impl<R: RetrievalPort, P: AnswerProviderPort> AnswerKernelPort for CachedKernel<R, P> {
    fn answer_accounted(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Accounted<AnswerResponse> {
        self.answer_with_purpose(query, context, AnswerPurpose::InternalRead)
            .into_accounted()
    }
    fn answer_for_publication_accounted(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Accounted<AnswerResponse> {
        self.answer_with_purpose(query, context, AnswerPurpose::ExternalPublication)
            .into_accounted()
    }
    fn answer(&self, query: &Query, context: &AuthorizedContext) -> AnswerResponse {
        self.answer_with_purpose(query, context, AnswerPurpose::InternalRead)
            .answer
    }
    fn answer_for_publication(&self, query: &Query, context: &AuthorizedContext) -> AnswerResponse {
        self.answer_with_purpose(query, context, AnswerPurpose::ExternalPublication)
            .answer
    }
}
impl<R: RetrievalPort, P: AnswerProviderPort> CachedKernel<R, P> {
    fn answer_with_purpose(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        purpose: AnswerPurpose,
    ) -> KernelAnswer {
        let bound = context.with_request_deadline();
        let context = &bound;
        let start = Instant::now();
        if query.validate().is_err()
            || query.conversation_id != context.conversation_id
            || !(1..=60000).contains(&context.deadline_ms)
            || context.knowledge_release.trim().is_empty()
            || !query.requested_scopes.is_subset(&context.principal.scopes)
        {
            return response(
                query,
                context,
                AnswerStatus::UnauthorizedEvidence,
                "Ungültiger Anfragekontext.",
                Vec::new(),
                Usage::default(),
            )
            .into();
        }
        if context.check_deadline().is_err() {
            return response(
                query,
                context,
                AnswerStatus::BudgetExceeded,
                "Request Deadline erreicht.",
                Vec::new(),
                Usage::default(),
            )
            .into();
        }
        let session = match self.inner.prepare_tools(query, context) {
            Ok(session) => session,
            Err(error) => {
                return response(
                    query,
                    context,
                    super::execution::validation_status(&error),
                    "Werkzeugkontext konnte nicht sicher gebunden werden.",
                    Vec::new(),
                    Usage::default(),
                )
                .into()
            }
        };
        if context.discord.is_some()
            || (query.profile == brain_contracts::AnswerProfile::Fact
                && !session
                    .as_ref()
                    .is_some_and(|session| !session.definitions.is_empty()))
        {
            return self
                .inner
                .answer_prepared(query, context, purpose, session.as_ref());
        }
        let key = match request_key(query, context, purpose, session.as_ref()) {
            Ok(key) => key,
            Err(_) => {
                return self
                    .inner
                    .answer_prepared(query, context, purpose, session.as_ref())
            }
        };
        match self.flights.run(
            key,
            Duration::from_millis(context.deadline_ms),
            context.request_deadline.as_ref(),
            || self.answer_cached(query, context, purpose, session.as_ref()),
        ) {
            Ok((mut answer, shared)) => {
                if shared {
                    answer.answer.request_id = query.request_id.clone();
                    answer.answer.usage = Usage::default();
                    answer.accounting = UsageAccounting::default();
                }
                if context.check_deadline().is_err() {
                    return KernelAnswer::from(response(
                        query,
                        context,
                        AnswerStatus::BudgetExceeded,
                        "Request Deadline erreicht.",
                        Vec::new(),
                        answer.accounting.observed.clone(),
                    ))
                    .with_accounting(answer.accounting);
                }
                if shared {
                    if let Err(error) = self.inner.validate_reuse(
                        query,
                        context,
                        &answer,
                        purpose,
                        session.as_ref(),
                    ) {
                        return response(
                            query,
                            context,
                            super::execution::validation_status(&error),
                            "Geteilte Evidenz konnte nicht sicher bestätigt werden.",
                            Vec::new(),
                            Usage::default(),
                        )
                        .into();
                    }
                }
                if context.check_deadline().is_err()
                    || start.elapsed().as_millis() >= context.deadline_ms as u128
                {
                    return KernelAnswer::from(response(
                        query,
                        context,
                        AnswerStatus::BudgetExceeded,
                        "Request Deadline erreicht.",
                        Vec::new(),
                        answer.accounting.observed.clone(),
                    ))
                    .with_accounting(answer.accounting);
                }
                answer
            }
            Err(PortError::BudgetExceeded) => response(
                query,
                context,
                AnswerStatus::BudgetExceeded,
                "Wartebudget ausgeschöpft.",
                Vec::new(),
                Usage::default(),
            )
            .into(),
            Err(_) => response(
                query,
                context,
                AnswerStatus::Unavailable,
                "Request-Koordination nicht verfügbar.",
                Vec::new(),
                Usage::default(),
            )
            .into(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc,
    };
    #[test]
    fn concurrent_failures_are_shared_but_not_retained() {
        let c = Arc::new(Coordinator::default());
        let calls = Arc::new(AtomicUsize::new(0));
        let (ready_tx, ready_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        let leader_c = c.clone();
        let leader_calls = calls.clone();
        let leader = std::thread::spawn(move || {
            leader_c
                .run("same".into(), Duration::from_secs(3), None, || {
                    leader_calls.fetch_add(1, Ordering::SeqCst);
                    ready_tx.send(()).unwrap();
                    release_rx.recv().unwrap();
                    AnswerResponse {
                        contract_version: CONTRACT_VERSION.into(),
                        request_id: "leader".into(),
                        knowledge_release: "r1".into(),
                        status: AnswerStatus::ProviderError,
                        text: "fixture failure".into(),
                        citations: Vec::new(),
                        usage: Usage::default(),
                    }
                    .into()
                })
                .unwrap()
        });
        ready_rx.recv().unwrap();
        let follower_c = c.clone();
        let follower = std::thread::spawn(move || {
            follower_c
                .run("same".into(), Duration::from_secs(3), None, || {
                    panic!("must share leader")
                })
                .unwrap()
        });
        let started = Instant::now();
        loop {
            let flight = c.flights.lock().unwrap().get("same").unwrap().clone();
            if flight.state.lock().unwrap().waiters > 0 {
                break;
            }
            assert!(started.elapsed() < Duration::from_secs(2));
            std::thread::yield_now();
        }
        release_tx.send(()).unwrap();
        let (a, leader_shared) = leader.join().unwrap();
        let (b, follower_shared) = follower.join().unwrap();
        assert!(!leader_shared);
        assert!(follower_shared);
        assert_eq!(a, b);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert!(c.flights.lock().unwrap().is_empty());
    }
}

#[cfg(test)]
mod retained_pack_tests {
    use super::*;
    use std::sync::mpsc;

    #[test]
    fn oversized_uncited_pack_is_not_retained_for_single_flight_followers() {
        let coordinator = Arc::new(Coordinator::default());
        let (ready_tx, ready_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        let leader_coordinator = coordinator.clone();
        let leader = std::thread::spawn(move || {
            leader_coordinator
                .run("large".into(), Duration::from_secs(3), None, || {
                    ready_tx.send(()).unwrap();
                    release_rx.recv_timeout(Duration::from_secs(3)).unwrap();
                    KernelAnswer {
                        answer: AnswerResponse {
                            contract_version: CONTRACT_VERSION.into(),
                            request_id: "leader".into(),
                            knowledge_release: "release".into(),
                            status: AnswerStatus::Answered,
                            text: "leader-only text".into(),
                            citations: Vec::new(),
                            usage: Usage::default(),
                        },
                        dependencies: vec![Evidence {
                            evidence_id: "uncited".into(),
                            source_id: "fixture".into(),
                            logical_id: "uncited".into(),
                            revision: 1,
                            kind: brain_contracts::EvidenceKind::Prose,
                            content: String::with_capacity(1024 * 1024 + 1),
                            citation: "fixture".into(),
                            visibility: brain_contracts::SourceVisibility::Public,
                            allowed_scopes: Default::default(),
                            score: 1.0,
                            provenance: None,
                            patch: None,
                        }]
                        .into(),
                        accounting: UsageAccounting::default(),
                        tool_dependencies: Arc::from([]),
                    }
                })
                .unwrap()
        });
        ready_rx.recv_timeout(Duration::from_secs(2)).unwrap();
        let follower_coordinator = coordinator.clone();
        let follower = std::thread::spawn(move || {
            follower_coordinator
                .run("large".into(), Duration::from_secs(3), None, || {
                    panic!("follower must not start another operation")
                })
                .unwrap()
        });
        let started = Instant::now();
        loop {
            let flight = coordinator
                .flights
                .lock()
                .unwrap()
                .get("large")
                .unwrap()
                .clone();
            if flight.state.lock().unwrap().waiters == 1 {
                break;
            }
            assert!(started.elapsed() < Duration::from_secs(2));
            std::thread::yield_now();
        }
        release_tx.send(()).unwrap();
        let (leader_answer, leader_shared) = leader.join().unwrap();
        let (follower_answer, follower_shared) = follower.join().unwrap();
        assert!(!leader_shared);
        assert_eq!(leader_answer.answer.status, AnswerStatus::Answered);
        assert!(follower_shared);
        assert_eq!(follower_answer.answer.status, AnswerStatus::Unavailable);
        assert!(follower_answer.dependencies.is_empty());
        assert!(follower_answer.answer.citations.is_empty());
        assert!(!follower_answer.answer.text.contains("leader-only"));
    }
}
