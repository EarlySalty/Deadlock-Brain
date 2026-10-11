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
            || context.deadline_ms > 600_000
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
                answer.enforce_answer_contract(query);
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

#[cfg(test)]
mod original_bound_tests {
    use super::*;
    use brain_contracts::{DocumentStorePort, SourceRecordV2, SourceVisibility};
    use brain_storage::MemoryRepository;
    use dbrain_retrieval::ReleaseRetriever;
    use std::collections::BTreeSet;

    struct NoGeneration;
    impl AnswerProviderPort for NoGeneration {
        fn answer(
            &self,
            _: &Query,
            _: &AuthorizedContext,
            _: &[Evidence],
        ) -> Result<brain_contracts::ProviderAnswer, PortError> {
            panic!("Ein gültiger Cachetreffer benötigt keine Modellrunde")
        }
    }

    async fn fixture() -> (
        MemoryRepository,
        SourceRecordV2,
        Query,
        AuthorizedContext,
        Vec<Evidence>,
    ) {
        let record = SourceRecordV2 {
            source_id: "original-localization".into(),
            logical_id: "german.txt".into(),
            revision: 1,
            content_hash: dbrain_sources::external::sha256(b"Seelenurne"),
            content: "Seelenurne".into(),
            visibility: SourceVisibility::Public,
            allowed_scopes: BTreeSet::new(),
            tombstone: false,
            valid_from: None,
            valid_to: None,
            metadata: BTreeMap::new(),
        };
        let store = MemoryRepository::default();
        store.apply_record(record.clone()).unwrap();
        let release = store.release_from_heads("r1", "k1", "p1").unwrap();
        store.publish(&release).await.unwrap();
        let query: Query = serde_json::from_value(serde_json::json!({"request_id":"cache-name-proof", "conversation_id":"c1", "text":"Seelenurne"})).unwrap();
        let context: AuthorizedContext = serde_json::from_value(serde_json::json!({
            "principal":{"actor_id":"test-operator","channel":"test","scopes":[],"provider_egress":["public"]},
            "conversation_id":"c1","knowledge_release":"r1","deadline_ms":30000,
            "budget":{"max_network_rounds":4,"max_input_tokens":100000,"max_output_tokens":2000,"max_cost_micros":50000}
        })).unwrap();
        let evidence = ReleaseRetriever::new(store.clone(), 6)
            .retrieve(&query, &context)
            .unwrap();
        assert!(!evidence.is_empty());
        (store, record, query, context, evidence)
    }

    #[tokio::test]
    async fn stale_answered_and_unverified_model_cache_entries_fail_closed() {
        let (store, _, query, context, evidence) = fixture().await;
        for status in [AnswerStatus::Answered, AnswerStatus::Unverified] {
            let cache = CachedKernel::new(
                Kernel::new(ReleaseRetriever::new(store.clone(), 6), NoGeneration),
                4,
                Duration::from_secs(60),
            );
            let stale = KernelAnswer::from(response(
                &query,
                &context,
                status,
                "Riftwalker",
                evidence.clone(),
                Usage::default(),
            ));
            let key = super::super::flight::request_key(
                &query,
                &context,
                AnswerPurpose::InternalRead,
                None,
            )
            .unwrap();
            let weight = stale.retained_bytes();
            cache
                .entries
                .lock()
                .unwrap()
                .insert(key, (Instant::now(), stale, weight));
            let answer = cache.answer(&query, &context);
            assert_eq!(answer.status, AnswerStatus::Unverified);
            assert_eq!(
                answer.text,
                brain_contracts::answer_contract::NO_SUPPORTED_ANSWER
            );
            assert!(answer.citations.is_empty());
        }
    }

    #[tokio::test]
    async fn cached_original_excerpt_still_rechecks_current_publication_rights() {
        let (store, mut record, query, context, evidence) = fixture().await;
        let cache = CachedKernel::new(
            Kernel::new(ReleaseRetriever::new(store.clone(), 6), NoGeneration),
            4,
            Duration::from_secs(60),
        );
        let stale = KernelAnswer::from(response(
            &query,
            &context,
            AnswerStatus::Answered,
            "Seelenurne",
            evidence,
            Usage::default(),
        ));
        let key = super::super::flight::request_key(
            &query,
            &context,
            AnswerPurpose::ExternalPublication,
            None,
        )
        .unwrap();
        let weight = stale.retained_bytes();
        cache
            .entries
            .lock()
            .unwrap()
            .insert(key, (Instant::now(), stale, weight));
        record.revision = 2;
        record.tombstone = true;
        store.apply_record(record).unwrap();
        let answer = cache.answer_for_publication(&query, &context);
        assert_eq!(answer.status, AnswerStatus::UnauthorizedEvidence);
        assert!(answer.citations.is_empty());
        assert!(!answer.text.contains("Seelenurne"));
    }
}
