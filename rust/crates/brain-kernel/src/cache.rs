use super::*;
use std::{
    collections::BTreeMap,
    sync::Mutex,
    time::{Duration, Instant},
};
const MAX_RETAINED_BYTES: usize = 32 * 1024 * 1024;
pub struct CachedKernel<R, P> {
    pub(super) inner: Kernel<R, P>,
    pub(super) flights: super::flight::Coordinator,
    entries: Mutex<BTreeMap<String, (Instant, KernelAnswer, usize)>>,
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
        purpose: AnswerPurpose,
        session: Option<&ToolSession>,
    ) -> KernelAnswer {
        if context.discord.is_some() {
            return self.inner.answer_prepared(query, context, purpose, session);
        }
        let started = Instant::now();
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
            )
            .into();
        }
        let key = match super::flight::request_key(query, context, purpose, session) {
            Ok(key) => key,
            Err(_) => return self.inner.answer_prepared(query, context, purpose, session),
        };
        let hit = self.entries.lock().ok().and_then(|mut entries| {
            entries.retain(|_, (created, _, _)| created.elapsed() < self.ttl);
            entries.get(&key).map(|(_, answer, _)| answer.clone())
        });
        if let Some(mut answer) = hit {
            let validation = self
                .inner
                .validate_reuse(query, context, &answer, purpose, session);
            if validation.is_ok()
                && context.check_deadline().is_ok()
                && started.elapsed().as_millis() < context.deadline_ms as u128
            {
                answer.answer.request_id = query.request_id.clone();
                answer.answer.usage = Usage::default();
                answer.accounting = UsageAccounting::default();
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
                )
                .into();
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
            )
            .into();
        };
        let mut next = context.clone();
        next.deadline_ms = remaining;
        let answer = self.inner.answer_prepared(query, &next, purpose, session);
        let weight = answer.retained_bytes();
        if answer.answer.status == AnswerStatus::Answered
            && !answer.accounting.unaccounted
            && !answer.answer.citations.is_empty()
            && self.capacity > 0
            && !self.ttl.is_zero()
            && weight <= MAX_RETAINED_BYTES
        {
            if let Ok(mut entries) = self.entries.lock() {
                let mut retained: usize = entries.values().map(|entry| entry.2).sum();
                while entries.len() >= self.capacity
                    || retained.saturating_add(weight) > MAX_RETAINED_BYTES
                {
                    let oldest = entries
                        .iter()
                        .min_by_key(|(_, entry)| entry.0)
                        .map(|(key, _)| key.clone());
                    if let Some(oldest) = oldest {
                        if let Some((_, _, removed)) = entries.remove(&oldest) {
                            retained = retained.saturating_sub(removed);
                        }
                    }
                }
                entries.insert(key, (Instant::now(), answer.clone(), weight));
            }
        }
        answer
    }
}
