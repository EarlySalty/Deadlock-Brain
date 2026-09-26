//! Bounded, short-lived answer cache. Every hit is reauthorized against canonical storage.
use super::*;
use std::{
    collections::BTreeMap,
    sync::Mutex,
    time::{Duration, Instant},
};
pub struct CachedKernel<R, P> {
    pub(super) inner: Kernel<R, P>,
    pub(super) flights: super::flight::Coordinator,
    entries: Mutex<BTreeMap<String, (Instant, AnswerResponse)>>,
    capacity: usize,
    ttl: Duration,
}
impl<R, P> CachedKernel<R, P> {
    pub fn new(inner: Kernel<R, P>, capacity: usize, ttl: Duration) -> Self {
        Self {
            inner,
            flights: super::flight::Coordinator::default(),
            entries: Mutex::new(BTreeMap::new()),
            capacity: capacity.min(1024),
            ttl: ttl.min(Duration::from_secs(60)),
        }
    }
}
impl<R: RetrievalPort, P: AnswerProviderPort> CachedKernel<R, P> {
    pub(super) fn answer_cached(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> AnswerResponse {
        let started = Instant::now();
        if query.validate().is_err()
            || query.conversation_id != context.conversation_id
            || context.deadline_ms == 0
            || context.deadline_ms > 60000
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
        // Never include request_id: it is rebound on each hit. All authorization/semantic inputs are included.
        let key = match super::flight::cache_key(query, context) {
            Ok(key) => key,
            Err(_) => return self.inner.answer(query, context),
        };
        let hit = self.entries.lock().ok().and_then(|mut entries| {
            entries.retain(|_, (created, _)| created.elapsed() < self.ttl);
            entries.get(&key).map(|(_, answer)| answer.clone())
        });
        if let Some(mut answer) = hit {
            let validation =
                self.inner
                    .retrieval
                    .validate_evidence(query, context, &answer.citations, false);
            if validation.is_ok() && started.elapsed().as_millis() < context.deadline_ms as u128 {
                answer.request_id = query.request_id.clone();
                answer.usage = Usage::default();
                return answer;
            }
            if let Ok(mut entries) = self.entries.lock() {
                entries.remove(&key);
            }
            let recompute_domain =
                query.domain.is_some() && matches!(validation, Err(PortError::PermissionDenied(_)));
            if let (Err(error), false) = (validation, recompute_domain) {
                return response(
                    query,
                    context,
                    super::execution::validation_status(&error),
                    "Cache-Evidenz konnte nicht sicher bestätigt werden.",
                    Vec::new(),
                    Usage::default(),
                );
            }
        }
        let elapsed = started.elapsed().as_millis() as u64;
        let Some(remaining) = context.deadline_ms.checked_sub(elapsed).filter(|v| *v > 0) else {
            return response(
                query,
                context,
                AnswerStatus::BudgetExceeded,
                "Request Deadline erreicht.",
                Vec::new(),
                Usage::default(),
            );
        };
        let mut next = context.clone();
        next.deadline_ms = remaining;
        let answer = self.inner.answer(query, &next);
        if answer.status == AnswerStatus::Answered
            && !answer.citations.is_empty()
            && self.capacity > 0
            && !self.ttl.is_zero()
        {
            if let Ok(mut entries) = self.entries.lock() {
                if entries.len() >= self.capacity {
                    let oldest = entries
                        .iter()
                        .min_by_key(|(_, entry)| entry.0)
                        .map(|(key, _)| key.clone());
                    if let Some(oldest) = oldest {
                        entries.remove(&oldest);
                    }
                }
                entries.insert(key, (Instant::now(), answer.clone()));
            }
        }
        answer
    }
}
