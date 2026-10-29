use super::pg_tools::blocking;
use brain_contracts::{
    canonical_receipt::CanonicalReceipt, tools::BuildPlanRequest, Accounted, AnswerContext,
    AnswerResponse, AnswerStatus, AuthorizedContext, Evidence, PinnedGameContext, PortError,
    PortFailure, Query, ToolCall, ToolDefinition, ToolEvidenceDependency, ToolExecution,
    ToolExecutionPort, ToolRequest, ToolValidationPurpose, Usage, UsageAccounting,
};
use brain_kernel::AnswerKernelPort;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::{postgres::PgConnectOptions, PgConnection, PgPool};
use std::{
    cell::RefCell,
    sync::Arc,
    time::{Duration, Instant},
};

#[derive(Default)]
struct Capture {
    calls: Vec<String>,
    citations: Vec<String>,
}

thread_local! {
    static ACTIVE: RefCell<Option<Capture>> = const { RefCell::new(None) };
}

struct CaptureScope(Option<Capture>);

impl CaptureScope {
    fn begin() -> Self {
        Self(ACTIVE.with(|active| active.replace(Some(Capture::default()))))
    }

    fn take(&self) -> Capture {
        ACTIVE.with(|active| active.borrow_mut().take().unwrap_or_default())
    }
}

impl Drop for CaptureScope {
    fn drop(&mut self) {
        ACTIVE.with(|active| {
            active.replace(self.0.take());
        });
    }
}

fn record_call(request: &ToolRequest) {
    ACTIVE.with(|active| {
        if let Some(capture) = active.borrow_mut().as_mut() {
            capture.calls.push(request.name().as_str().into());
        }
    });
}

fn record_result(execution: &ToolExecution) {
    ACTIVE.with(|active| {
        if let Some(capture) = active.borrow_mut().as_mut() {
            for evidence in execution
                .dependencies
                .iter()
                .flat_map(|dependency| &dependency.evidence)
            {
                if execution
                    .result
                    .evidence_ids
                    .contains(&evidence.evidence_id)
                {
                    record_evidence(capture, evidence);
                }
            }
        }
    });
}

fn record_evidence(capture: &mut Capture, evidence: &Evidence) {
    if let Ok(receipt) = CanonicalReceipt::from_evidence(evidence) {
        let id = receipt.raw_receipt_id.to_string();
        if !capture.citations.contains(&id) {
            capture.citations.push(id);
        }
    }
}

fn receipt_id(id: &str) -> bool {
    id.len() <= 19 && !id.starts_with('0') && id.parse::<i64>().is_ok_and(|value| value > 0)
}

pub(super) struct TracedTools<T>(pub(super) T);

impl<T: ToolExecutionPort> ToolExecutionPort for TracedTools<T> {
    fn required_calls(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        pin: Option<&PinnedGameContext>,
    ) -> Result<Vec<ToolCall>, PortError> {
        self.0.required_calls(query, context, pin)
    }

    fn definitions(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        pin: Option<&PinnedGameContext>,
    ) -> Result<Vec<ToolDefinition>, PortError> {
        self.0.definitions(query, context, pin)
    }

    fn execute(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        pin: Option<&PinnedGameContext>,
        id: &str,
        request: &ToolRequest,
    ) -> Result<ToolExecution, PortError> {
        record_call(request);
        let result = self.0.execute(query, context, pin, id, request);
        if let Ok(execution) = &result {
            record_result(execution);
        }
        result
    }

    fn execute_accounted(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        pin: Option<&PinnedGameContext>,
        id: &str,
        request: &ToolRequest,
    ) -> Result<Accounted<ToolExecution>, PortFailure> {
        record_call(request);
        let result = self.0.execute_accounted(query, context, pin, id, request);
        if let Ok(execution) = &result {
            record_result(&execution.value);
        }
        result
    }

    fn validate_build_plan(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        pin: &PinnedGameContext,
        request: &BuildPlanRequest,
        execution: &ToolExecution,
        purpose: brain_contracts::store::AnswerPurpose,
    ) -> Result<(), PortError> {
        self.0
            .validate_build_plan(query, context, pin, request, execution, purpose)
    }

    fn validate_dependencies(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        pin: Option<&PinnedGameContext>,
        dependencies: &[ToolEvidenceDependency],
        purpose: ToolValidationPurpose,
    ) -> Result<(), PortError> {
        self.0
            .validate_dependencies(query, context, pin, dependencies, purpose)
    }
}

pub(super) struct TraceRecord {
    pub(super) question_sha256: [u8; 32],
    pub(super) tool_calls: Vec<String>,
    pub(super) citation_ids: Vec<String>,
    pub(super) status: &'static str,
    pub(super) tokens: Value,
    pub(super) latency_ms: i32,
    pub(super) source: String,
    pub(super) route: Option<String>,
    pub(super) question: String,
    pub(super) history: Vec<String>,
    pub(super) answer_text: Option<String>,
    pub(super) not_answered_stage: Option<String>,
    pub(super) not_answered_reason: Option<String>,
    pub(super) provider_rounds: i32,
}

const QUESTION_CHARS: usize = 4000;
const HISTORY_TURNS: usize = 4;
const ANSWER_CHARS: usize = 8000;
const SOURCE_CHARS: usize = 128;

fn bounded(text: &str, limit: usize) -> String {
    text.chars().take(limit).collect()
}

fn slug(value: &str) -> Option<String> {
    let slug: String = value
        .chars()
        .map(|c| c.to_ascii_lowercase())
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .take(64)
        .collect();
    slug.starts_with(|c: char| c.is_ascii_lowercase())
        .then_some(slug)
}

fn route(query: &Query) -> Option<String> {
    let purpose = match query.answer_context.as_ref()? {
        AnswerContext::Discord(context) => context.purpose.as_deref().unwrap_or("discord"),
        AnswerContext::Twitch(_) => "twitch",
    };
    let valid = purpose.len() <= 64
        && purpose.starts_with(|c: char| c.is_ascii_lowercase())
        && purpose
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || "_:.-".contains(c));
    valid.then(|| purpose.to_owned())
}

fn status_name(status: AnswerStatus) -> &'static str {
    match status {
        AnswerStatus::Answered => "answered",
        AnswerStatus::Unverified => "unverified",
        AnswerStatus::BuildRejected => "build_rejected",
        AnswerStatus::InsufficientEvidence => "insufficient_evidence",
        AnswerStatus::UnauthorizedEvidence => "unauthorized_evidence",
        AnswerStatus::Unavailable => "unavailable",
        AnswerStatus::ProviderError => "provider_error",
        AnswerStatus::BudgetExceeded => "budget_exceeded",
    }
}

pub(super) trait TraceSink: Send + Sync {
    fn write(&self, record: TraceRecord) -> Result<(), PortError>;
}

pub(super) struct PgTraceWriter {
    pool: PgPool,
    runtime: tokio::runtime::Handle,
    timeout: Duration,
}

async fn assert_writer(connection: &mut PgConnection) -> Result<(), sqlx::Error> {
    let (login, role, readonly): (String, String, String) = sqlx::query_as(
        "SELECT session_user::text,current_user::text,current_setting('transaction_read_only')",
    )
    .fetch_one(connection)
    .await?;
    if login != "brain_service" || role != "brain_service" || readonly != "off" {
        return Err(sqlx::Error::Protocol(
            "Antwortnachweis-Verbindung wurde verunreinigt.".into(),
        ));
    }
    Ok(())
}

impl PgTraceWriter {
    pub(super) fn new(
        options: PgConnectOptions,
        runtime: tokio::runtime::Handle,
        timeout: Duration,
    ) -> Self {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .acquire_timeout(timeout)
            .after_connect(|connection, _| Box::pin(async move { assert_writer(connection).await }))
            .before_acquire(|connection, _| {
                Box::pin(async move {
                    assert_writer(connection).await?;
                    Ok(true)
                })
            })
            .connect_lazy_with(options);
        Self {
            pool,
            runtime,
            timeout,
        }
    }
}

impl TraceSink for PgTraceWriter {
    fn write(&self, record: TraceRecord) -> Result<(), PortError> {
        if record.tool_calls.len() > 8
            || record.citation_ids.len() > 100
            || !record.citation_ids.iter().all(|id| receipt_id(id))
        {
            return Err(PortError::InvalidResponse(
                "Antwortnachweis überschreitet den Speichervertrag.".into(),
            ));
        }
        blocking(&self.runtime, async {
            tokio::time::timeout(self.timeout,sqlx::query(
                "INSERT INTO ops.answer_trace(question_sha256,tool_calls,citation_ids,status,tokens,latency_ms,
                source,route,question,history,answer_text,not_answered_stage,not_answered_reason,provider_rounds)
                VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14)")
                .bind(record.question_sha256.to_vec()).bind(json!(record.tool_calls))
                .bind(json!(record.citation_ids)).bind(record.status).bind(record.tokens)
                .bind(record.latency_ms).bind(record.source).bind(record.route)
                .bind(record.question).bind(json!(record.history)).bind(record.answer_text)
                .bind(record.not_answered_stage).bind(record.not_answered_reason)
                .bind(record.provider_rounds).execute(&self.pool)).await
                .map_err(|_| PortError::Unavailable("Zeitgrenze für Antwortnachweis erreicht.".into()))?
                .map(|_| ()).map_err(|error| {
                    if error.as_database_error().and_then(|error| error.code()).as_deref() == Some("23514") {
                        PortError::InvalidResponse("Antwortnachweis verletzt den Speichervertrag.".into())
                    } else {
                        PortError::Unavailable("Antwortnachweis konnte nicht gespeichert werden.".into())
                    }
                })
        })
    }
}

pub(super) struct TracedKernel<K> {
    inner: K,
    sink: Option<Arc<dyn TraceSink>>,
}

impl<K> TracedKernel<K> {
    pub(super) fn new(inner: K, sink: Option<Arc<dyn TraceSink>>) -> Self {
        Self { inner, sink }
    }

    fn invoke<R>(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        action: impl FnOnce() -> R,
        response: impl FnOnce(&R) -> (&AnswerResponse, Option<&UsageAccounting>),
    ) -> R {
        let Some(sink) = &self.sink else {
            return action();
        };
        let started = Instant::now();
        let scope = CaptureScope::begin();
        let (result, not_answered) = brain_contracts::answer_trace::observe_not_answered(action);
        let mut capture = scope.take();
        let (answer, accounting) = response(&result);
        for evidence in &answer.citations {
            record_evidence(&mut capture, evidence);
        }
        let usage = accounting.map_or(&answer.usage, |value| &value.observed);
        let request = brain_contracts::answer_trace::current_request().unwrap_or_else(|| {
            brain_contracts::answer_trace::TraceRequest {
                question: brain_contracts::discord_task::without_platform_ids(&query.text),
                history: Vec::new(),
            }
        });
        let answered = answer.status == AnswerStatus::Answered;
        let not_answered = not_answered.filter(|_| !answered);
        let record = TraceRecord {
            question_sha256: Sha256::digest(query.text.as_bytes()).into(),
            tool_calls: capture.calls,
            citation_ids: capture.citations,
            status: status_name(answer.status),
            tokens: token_counts(usage, accounting),
            latency_ms: started.elapsed().as_millis().min(i32::MAX as u128) as i32,
            source: bounded(
                &format!(
                    "{}-{}",
                    context.principal.actor_id, context.principal.channel
                ),
                SOURCE_CHARS,
            ),
            route: route(query),
            question: bounded(&request.question, QUESTION_CHARS),
            history: request
                .history
                .iter()
                .skip(request.history.len().saturating_sub(HISTORY_TURNS))
                .map(|text| bounded(text, QUESTION_CHARS))
                .collect(),
            answer_text: answered.then(|| bounded(&answer.text, ANSWER_CHARS)),
            not_answered_stage: not_answered.as_ref().and_then(|note| slug(&note.stage)),
            not_answered_reason: not_answered.as_ref().and_then(|note| slug(&note.reason)),
            provider_rounds: i32::try_from(usage.network_rounds).unwrap_or(i32::MAX),
        };
        let tool_count = record.tool_calls.len();
        let citation_count = record.citation_ids.len();
        if let Err(error) = sink.write(record) {
            let reason = match error {
                PortError::InvalidResponse(_) => "storage_contract",
                _ => "storage_unavailable",
            };
            eprintln!(
                "{}",
                json!({"event":"answer_trace_failed", "reason":reason,
                "tool_count":tool_count, "citation_count":citation_count})
            );
        }
        result
    }
}

fn token_counts(usage: &Usage, accounting: Option<&UsageAccounting>) -> Value {
    let reserved = accounting.map(|value| &value.reserved);
    json!({"input_tokens":usage.input_tokens,"output_tokens":usage.output_tokens,
        "reserved_input_tokens":reserved.map_or(0,|usage| usage.input_tokens),
        "reserved_output_tokens":reserved.map_or(0,|usage| usage.output_tokens),
        "unaccounted":u8::from(accounting.is_some_and(|value| value.unaccounted))})
}

impl<K: AnswerKernelPort> AnswerKernelPort for TracedKernel<K> {
    fn answer(&self, query: &Query, context: &AuthorizedContext) -> AnswerResponse {
        self.invoke(
            query,
            context,
            || self.inner.answer(query, context),
            |answer| (answer, None),
        )
    }

    fn answer_for_publication(&self, query: &Query, context: &AuthorizedContext) -> AnswerResponse {
        self.invoke(
            query,
            context,
            || self.inner.answer_for_publication(query, context),
            |answer| (answer, None),
        )
    }

    fn answer_accounted(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Accounted<AnswerResponse> {
        self.invoke(
            query,
            context,
            || self.inner.answer_accounted(query, context),
            |answer| (&answer.value, Some(&answer.accounting)),
        )
    }

    fn answer_for_publication_accounted(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Accounted<AnswerResponse> {
        self.invoke(
            query,
            context,
            || self.inner.answer_for_publication_accounted(query, context),
            |answer| (&answer.value, Some(&answer.accounting)),
        )
    }
}

#[cfg(test)]
#[path = "answer_trace_tests.rs"]
mod tests;
