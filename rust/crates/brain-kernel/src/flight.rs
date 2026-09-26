//! Coalesce identical in-flight requests, including failures, without caching failures.
#[cfg(test)]
mod c3_shared_validation;
use super::*;
use std::{
    collections::BTreeMap,
    sync::{Arc, Condvar, Mutex},
    time::{Duration, Instant},
};
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
    result: Option<AnswerResponse>,
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
    fn run(
        &self,
        key: String,
        timeout: Duration,
        operation: impl FnOnce() -> AnswerResponse,
    ) -> Result<(AnswerResponse, bool), PortError> {
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
            let result = operation();
            {
                let mut state = flight.state.lock().map_err(|_| unavailable())?;
                state.result = Some(result.clone());
                state.done = true;
            }
            flight.ready.notify_all();
            Ok((result, false))
        } else {
            let mut state = flight.state.lock().map_err(|_| unavailable())?;
            state.waiters += 1;
            let (mut state, _) = flight
                .ready
                .wait_timeout_while(state, timeout, |s| !s.done)
                .map_err(|_| unavailable())?;
            state.waiters -= 1;
            if !state.done {
                return Err(PortError::BudgetExceeded);
            }
            state
                .result
                .clone()
                .map(|r| (r, true))
                .ok_or_else(unavailable)
        }
    }
}
pub(super) fn cache_key(
    query: &Query,
    context: &AuthorizedContext,
) -> Result<String, serde_json::Error> {
    serde_json::to_string(&(
        "brain.policy.v2",
        &context.principal,
        &context.conversation_id,
        &context.knowledge_release,
        &context.budget,
        &query.text,
        &query.profile,
        &query.patch,
        &query.mode,
        &query.requested_scopes,
    ))
}
impl<R: RetrievalPort, P: AnswerProviderPort> AnswerKernelPort for CachedKernel<R, P> {
    fn answer(&self, query: &Query, context: &AuthorizedContext) -> AnswerResponse {
        let start = Instant::now();
        if query.validate().is_err()
            || query.conversation_id != context.conversation_id
            || !(1..=60000).contains(&context.deadline_ms)
            || !query.requested_scopes.is_subset(&context.principal.scopes)
        {
            return response(
                query,
                context,
                AnswerStatus::UnauthorizedEvidence,
                "Ungültiger Anfragekontext.",
                Vec::new(),
                Usage::default(),
            );
        }
        let key = match cache_key(query, context) {
            Ok(key) => key,
            Err(_) => return self.inner.answer(query, context),
        };
        match self
            .flights
            .run(key, Duration::from_millis(context.deadline_ms), || {
                self.answer_cached(query, context)
            }) {
            Ok((mut answer, shared)) => {
                if shared {
                    if answer.status == AnswerStatus::Answered {
                        if let Err(error) = self.inner.retrieval.validate_evidence(
                            query,
                            context,
                            &answer.citations,
                            false,
                        ) {
                            return response(
                                query,
                                context,
                                super::execution::validation_status(&error),
                                "Geteilte Evidenz konnte nicht sicher bestätigt werden.",
                                Vec::new(),
                                Usage::default(),
                            );
                        }
                    }
                    answer.request_id = query.request_id.clone();
                    answer.usage = Usage::default();
                }
                if start.elapsed().as_millis() >= context.deadline_ms as u128 {
                    return response(
                        query,
                        context,
                        AnswerStatus::BudgetExceeded,
                        "Request Deadline erreicht.",
                        Vec::new(),
                        Usage::default(),
                    );
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
            ),
            Err(_) => response(
                query,
                context,
                AnswerStatus::Unavailable,
                "Request-Koordination nicht verfügbar.",
                Vec::new(),
                Usage::default(),
            ),
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
                .run("same".into(), Duration::from_secs(3), || {
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
                })
                .unwrap()
        });
        ready_rx.recv().unwrap();
        let follower_c = c.clone();
        let follower = std::thread::spawn(move || {
            follower_c
                .run("same".into(), Duration::from_secs(3), || {
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
