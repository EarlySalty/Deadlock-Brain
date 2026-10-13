use super::pg_tools::blocking;
use brain_contracts::{
    tools::BuildPlanRequest, Accounted, AnswerResponse, AnswerStatus, AuthorizedContext,
    PinnedGameContext, PortError, PortFailure, Query, ToolCall, ToolDefinition,
    ToolEvidenceDependency, ToolExecution, ToolExecutionPort, ToolRequest, ToolValidationPurpose,
    Usage, UsageAccounting,
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
            for id in &execution.result.evidence_ids {
                if receipt_id(id) && !capture.citations.contains(id) {
                    capture.citations.push(id.clone());
                }
            }
        }
    });
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
                "INSERT INTO ops.answer_trace(question_sha256,tool_calls,citation_ids,status,tokens,latency_ms)
                VALUES ($1,$2,$3,$4,$5,$6)")
                .bind(record.question_sha256.to_vec()).bind(json!(record.tool_calls))
                .bind(json!(record.citation_ids)).bind(record.status).bind(record.tokens)
                .bind(record.latency_ms).execute(&self.pool)).await
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
        action: impl FnOnce() -> R,
        response: impl FnOnce(&R) -> (&AnswerResponse, Option<&UsageAccounting>),
    ) -> R {
        let Some(sink) = &self.sink else {
            return action();
        };
        let started = Instant::now();
        let scope = CaptureScope::begin();
        let result = action();
        let mut capture = scope.take();
        let (answer, accounting) = response(&result);
        for evidence in &answer.citations {
            if receipt_id(&evidence.evidence_id)
                && !capture.citations.contains(&evidence.evidence_id)
            {
                capture.citations.push(evidence.evidence_id.clone());
            }
        }
        let usage = accounting.map_or(&answer.usage, |value| &value.observed);
        let record = TraceRecord {
            question_sha256: Sha256::digest(query.text.as_bytes()).into(),
            tool_calls: capture.calls,
            citation_ids: capture.citations,
            status: match answer.status {
                AnswerStatus::Answered => "answered",
                AnswerStatus::InsufficientEvidence => "insufficient_evidence",
                _ => "failed",
            },
            tokens: token_counts(usage, accounting),
            latency_ms: started.elapsed().as_millis().min(i32::MAX as u128) as i32,
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
            || self.inner.answer(query, context),
            |answer| (answer, None),
        )
    }

    fn answer_for_publication(&self, query: &Query, context: &AuthorizedContext) -> AnswerResponse {
        self.invoke(
            query,
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
            || self.inner.answer_for_publication_accounted(query, context),
            |answer| (&answer.value, Some(&answer.accounting)),
        )
    }
}

#[cfg(test)]
#[path = "answer_trace_tests.rs"]
mod tests;
