use super::{
    hardening, provider_contract_error, Duration, Instant, OpenAiCompatibleProvider,
    ProviderAnswer, ProviderError, Result, StatusCode, Usage,
};
use brain_contracts::{
    provider_input::parse_unique_json, AuthorizedContext, Evidence, ModelBlock,
    ProviderFinishReason, ProviderTurn, ToolCall, ToolConversation, ToolDefinition, ToolMessage,
    ToolName, UsageAccounting,
};
use serde::Deserialize;
use serde_json::Value;

pub(super) struct ChatRequest<'a> {
    pub query: &'a brain_contracts::Query,
    pub evidence: &'a [Evidence],
    pub tools: &'a [ToolDefinition],
    pub conversation: &'a ToolConversation,
}

pub(super) struct Charge {
    input_per_round: u64,
    output_per_round: u64,
    rounds: u32,
}
impl OpenAiCompatibleProvider {
    pub(super) fn send_chat(
        &self,
        mut payload: Value,
        context: &AuthorizedContext,
        request: ChatRequest<'_>,
        accounting: &mut UsageAccounting,
        transport_failure: &mut Option<bool>,
    ) -> Result<ProviderTurn> {
        let ChatRequest {
            evidence,
            tools,
            conversation,
            ..
        } = &request;
        let route = if self.config.subscription {
            "messages"
        } else {
            "chat/completions"
        };
        let (bytes, charge) = self.transport_json_with_accounting(
            route,
            &mut payload,
            context,
            Some(&request),
            accounting,
            transport_failure,
        )?;
        let mut failed_check = super::audit::check(
            "chat_schema",
            "response",
            "valid JSON with model, usage and one complete assistant response",
        );
        let mut deviations = Vec::new();
        let result = (|| {
            let mut parsed = if self.config.subscription {
                subscription_response(&bytes, &mut failed_check)?
            } else {
                compatible_response(&bytes, &mut failed_check)?
            };
            failed_check = super::audit::check("model_identity", "model", "the configured model");
            if parsed.model != self.config.model {
                return Err(ProviderError::InvalidResponse(
                    "model or choice count mismatch".into(),
                ));
            }
            failed_check = super::audit::check(
                "usage_budget",
                "usage",
                "reported usage within the request budget",
            );
            let usage =
                self.charged_usage(context, charge, parsed.input_tokens, parsed.output_tokens)?;
            let charged = accounting.charged();
            if accounting.unaccounted
                || charged.input_tokens > u64::from(context.budget.max_input_tokens)
                || charged.output_tokens > u64::from(context.budget.max_output_tokens)
                || charged.network_rounds > context.budget.max_network_rounds
                || charged.cost_micros > context.budget.max_cost_micros
            {
                return Err(ProviderError::BudgetExceeded);
            }
            failed_check = super::audit::check(
                "finish_reason",
                "finish_reason",
                "complete end_turn or valid tool_use",
            );
            let unverified_finish = !self.config.quality_filters
                && parsed.finish_reason == ProviderFinishReason::MaxTokens
                && !parsed
                    .blocks
                    .iter()
                    .any(|block| matches!(block, ModelBlock::ToolUse { .. }));
            if unverified_finish {
                deviations.push(failed_check.clone());
                self.report_category(
                    Some(&request.query.request_id),
                    "answer_unverified_truncated".into(),
                );
                parsed.finish_reason = ProviderFinishReason::EndTurn;
            } else if matches!(
                parsed.finish_reason,
                ProviderFinishReason::MaxTokens | ProviderFinishReason::Refusal
            ) {
                return Err(incomplete_turn(parsed.finish_reason));
            }
            let has_calls = parsed
                .blocks
                .iter()
                .any(|block| matches!(block, ModelBlock::ToolUse { .. }));
            let turn = if has_calls {
                let seen: std::collections::BTreeSet<_> = conversation
                    .messages
                    .iter()
                    .filter_map(|message| match message {
                        ToolMessage::Assistant { blocks } => Some(blocks),
                        ToolMessage::ToolResults { .. } => None,
                    })
                    .flatten()
                    .filter_map(|block| match block {
                        ModelBlock::ToolUse { call } => Some(call.id.as_str()),
                        ModelBlock::Text { .. } => None,
                    })
                    .collect();
                failed_check = super::audit::check(
                    "tool_call_history",
                    "tool_calls.id",
                    "tool call IDs not previously used in this conversation",
                );
                if parsed.blocks.iter().any(|block| {
                    matches!(block, ModelBlock::ToolUse { call } if seen.contains(call.id.as_str()))
                }) {
                    return Err(ProviderError::InvalidResponse(
                        "duplicate tool call ID across turns".into(),
                    ));
                }
                ProviderTurn::ToolCalls {
                    blocks: parsed.blocks,
                    finish_reason: parsed.finish_reason,
                    usage,
                }
            } else {
                if parsed.finish_reason != ProviderFinishReason::EndTurn {
                    return Err(incomplete_turn(parsed.finish_reason));
                }
                let text: String = parsed
                    .blocks
                    .into_iter()
                    .filter_map(|block| match block {
                        ModelBlock::Text { text } => Some(text),
                        ModelBlock::ToolUse { .. } => None,
                    })
                    .collect();
                failed_check = super::audit::check("grounded_envelope", "message.content", "object with text and cited_evidence_ids; nonempty text requires known unique evidence IDs");
                let answer = grounded_answer(
                    text.clone(),
                    usage.clone(),
                    evidence,
                    !tools.is_empty()
                        || !conversation.messages.is_empty()
                        || !self.config.quality_filters,
                    &mut failed_check,
                );
                if matches!(&answer, Err(ProviderError::InvalidResponse(message)) if message == "grounded answer envelope missing")
                {
                    let shape = if text.trim().starts_with("```") {
                        "fenced"
                    } else {
                        match parse_unique_json(text.as_bytes()) {
                            Ok(Value::Object(_)) => "object_schema",
                            Ok(_) => "non_object_json",
                            Err(error) if error.is_eof() => "incomplete_json",
                            Err(error) if error.is_data() => "duplicate_json_keys",
                            Err(_) => "non_json",
                        }
                    };
                    self.report_category(
                        Some(&request.query.request_id),
                        format!("grounded_envelope_{shape}"),
                    );
                }
                let answer = match answer {
                    Ok(_) if unverified_finish => {
                        failed_check = super::audit::check(
                            "unverified_answer",
                            "message.content",
                            "nonempty display-safe answer text",
                        );
                        unverified_answer(&text, usage, evidence)?
                    }
                    Ok(answer) => answer,
                    Err(error @ ProviderError::InvalidResponse(_))
                        if !self.config.quality_filters =>
                    {
                        deviations.push(failed_check.clone());
                        self.report_failure(Some(&request.query.request_id), &error);
                        self.report_category(
                            Some(&request.query.request_id),
                            "answer_unverified".into(),
                        );
                        failed_check = super::audit::check(
                            "unverified_answer",
                            "message.content",
                            "nonempty display-safe answer text",
                        );
                        unverified_answer(&text, usage, evidence)?
                    }
                    Err(error) => return Err(error),
                };
                ProviderTurn::Final {
                    answer,
                    finish_reason: parsed.finish_reason,
                }
            };
            failed_check = super::audit::check(
                "tool_contract",
                "assistant.blocks",
                "valid complete turn and calls matching the supplied tool definitions",
            );
            turn.validate(tools).map_err(|error| {
                failed_check.expected = format!("valid tool contract: {error}")
                    .chars()
                    .take(256)
                    .collect();
                provider_contract_error(error)
            })?;
            failed_check = super::audit::check(
                "request_deadline",
                "response",
                "response completed within the original request deadline",
            );
            context
                .check_deadline()
                .map_err(|_| ProviderError::BudgetExceeded)?;
            Ok(turn)
        })();
        if result.is_err() {
            deviations.push(failed_check);
        }
        let disposition = if result.is_ok() {
            brain_contracts::response_audit::ResponseDisposition::UncheckedReturned
        } else {
            brain_contracts::response_audit::ResponseDisposition::Rejected
        };
        let outputs: Vec<_> = deviations
            .into_iter()
            .map(|check| super::OutputDeviation {
                check,
                raw_output: &bytes,
                complete: true,
                disposition,
            })
            .collect();
        self.record_response_deviations(request.query, context, evidence, &outputs)?;
        result
    }
    pub(super) fn transport_json(
        &self,
        route: &str,
        payload: &mut serde_json::Value,
        context: &AuthorizedContext,
        transport_failure: &mut Option<bool>,
    ) -> Result<(Vec<u8>, Charge)> {
        self.transport_json_with_accounting(
            route,
            payload,
            context,
            None,
            &mut UsageAccounting::default(),
            transport_failure,
        )
    }

    fn transport_json_with_accounting(
        &self,
        route: &str,
        payload: &mut serde_json::Value,
        context: &AuthorizedContext,
        audit_request: Option<&ChatRequest<'_>>,
        accounting: &mut UsageAccounting,
        transport_failure: &mut Option<bool>,
    ) -> Result<(Vec<u8>, Charge)> {
        let chat = audit_request.is_some();
        let request_id = audit_request.map(|request| request.query.request_id.as_str());
        let bound = context.with_request_deadline();
        let context = &bound;
        let lifetime = context
            .request_deadline
            .as_ref()
            .expect("bound request deadline");
        let deadline = Instant::now()
            .checked_add(
                context
                    .remaining_time()
                    .map_err(|_| ProviderError::BudgetExceeded)?,
            )
            .ok_or(ProviderError::BudgetExceeded)?
            .min(lifetime.expires_at());
        let budget = &context.budget;
        if context.deadline_ms == 0
            || context.deadline_ms > 60000
            || budget.max_network_rounds == 0
            || budget.max_input_tokens == 0
        {
            return Err(ProviderError::BudgetExceeded);
        }
        if self.config.subscription && (!chat || route != "messages") {
            return Err(ProviderError::InvalidConfig);
        }
        let input = brain_contracts::provider_input::transport_input_ceiling(payload, chat)
            .map_err(provider_contract_error)?;
        let mut attempts = self
            .config
            .retry_attempts
            .min(budget.max_network_rounds as usize)
            .min((budget.max_input_tokens as u64 / input) as usize);
        if chat {
            attempts = attempts.min(budget.max_output_tokens as usize);
        }
        if attempts == 0 {
            return Err(ProviderError::BudgetExceeded);
        }
        let output = if chat {
            budget.max_output_tokens as u64 / attempts as u64
        } else {
            0
        };
        if chat {
            payload["max_tokens"] = serde_json::json!(output);
        }
        let reserved_cost = hardening::cost(
            hardening::price(&self.config),
            input
                .checked_mul(attempts as u64)
                .ok_or(ProviderError::BudgetExceeded)?,
            output
                .checked_mul(attempts as u64)
                .ok_or(ProviderError::BudgetExceeded)?,
        )?;
        if reserved_cost > budget.max_cost_micros {
            return Err(ProviderError::BudgetExceeded);
        }
        let url = format!("{}/{}", self.config.base_url.trim_end_matches('/'), route);
        let mut last_status = StatusCode::SERVICE_UNAVAILABLE;
        for attempt in 0..attempts {
            lifetime
                .check()
                .map_err(|_| ProviderError::BudgetExceeded)?;
            let request = self.client.post(&url);
            let request = if self.config.subscription {
                request.header("anthropic-version", "2023-06-01")
            } else {
                request.bearer_auth(&self.config.api_key)
            }
            .json(payload);
            lifetime
                .check()
                .map_err(|_| ProviderError::BudgetExceeded)?;
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                return Err(ProviderError::BudgetExceeded);
            }
            let reserved = Usage {
                input_tokens: input,
                output_tokens: output,
                cost_micros: hardening::cost(hardening::price(&self.config), input, output)?,
                ..Usage::default()
            };
            accounting.accumulate(&UsageAccounting {
                observed: Usage {
                    provider: Some(
                        if self.config.subscription {
                            "codex_subscription"
                        } else {
                            "openai_compatible"
                        }
                        .into(),
                    ),
                    model: Some(self.config.model.clone()),
                    network_rounds: 1,
                    ..Usage::default()
                },
                reserved: reserved.clone(),
                unaccounted: false,
            });
            let result = request.timeout(self.config.timeout.min(remaining)).send();
            *transport_failure = match &result {
                Ok(response) => {
                    let failed = response.status().is_server_error();
                    if failed {
                        self.report_event(
                            "provider_transport_failure",
                            request_id,
                            format!("http_{}", response.status().as_u16()),
                        );
                    }
                    Some(failed)
                }
                Err(error) if error.is_builder() || error.is_decode() => None,
                Err(error) => {
                    self.report_event(
                        "provider_transport_failure",
                        request_id,
                        if error.is_timeout() {
                            "transport_timeout"
                        } else if error.is_connect() {
                            "transport_connect"
                        } else {
                            "transport"
                        }
                        .into(),
                    );
                    Some(true)
                }
            };
            lifetime
                .check()
                .map_err(|_| ProviderError::BudgetExceeded)?;
            let mut requested_delay = Duration::ZERO;
            match result {
                Ok(response) if response.status().is_success() => {
                    let (read, bytes) =
                        hardening::read_bounded_observed(response, self.config.max_response_bytes);
                    if let Err(error) = read {
                        if matches!(&error, ProviderError::BodyTransport(_)) {
                            *transport_failure = Some(true);
                        }
                        if let Some(request) = audit_request {
                            self.record_response_deviation(request.query, context, request.evidence, super::OutputDeviation {
                                check: super::audit::check(if matches!(&error, ProviderError::ResponseTooLarge) { "response_size" } else { "response_body_transport" }, "response", "complete response within the configured byte limit"),
                                raw_output: &bytes,
                                complete: false,
                                disposition: brain_contracts::response_audit::ResponseDisposition::Rejected,
                            })?;
                        }
                        return Err(error);
                    }
                    if chat {
                        if let Err(error) = self.observe_usage(&bytes, &reserved, accounting) {
                            if let Some(request) = audit_request {
                                self.record_response_deviation(request.query, context, request.evidence, super::OutputDeviation {
                                    check: super::audit::check("usage_budget", "usage", "reported usage within the reserved request budget"),
                                    raw_output: &bytes,
                    complete: true,
                                    disposition: brain_contracts::response_audit::ResponseDisposition::Rejected,
                                })?;
                            }
                            return Err(error);
                        }
                    }
                    if Instant::now() >= deadline || lifetime.check().is_err() {
                        if let Some(request) = audit_request {
                            self.record_response_deviation(
                                request.query,
                                context,
                                request.evidence,
                                super::OutputDeviation {
                                    check: super::audit::check(
                                        "request_deadline",
                                        "response",
                                        "response completed within the original request deadline",
                                    ),
                                    raw_output: &bytes,
                                    complete: true,
                                    disposition: brain_contracts::response_audit::ResponseDisposition::Rejected,
                                },
                            )?;
                        }
                        return Err(ProviderError::BudgetExceeded);
                    }
                    return Ok((
                        bytes,
                        Charge {
                            input_per_round: input,
                            output_per_round: output,
                            rounds: (attempt + 1) as u32,
                        },
                    ));
                }
                Ok(response)
                    if response.status() == StatusCode::TOO_MANY_REQUESTS
                        || response.status().is_server_error() =>
                {
                    last_status = response.status();
                    requested_delay = hardening::retry_after(&response)?.unwrap_or_default();
                    if chat {
                        match hardening::read_bounded(response, self.config.max_response_bytes) {
                            Ok(bytes) => self.observe_usage(&bytes, &reserved, accounting)?,
                            Err(error) => self.report_failure(request_id, &error),
                        }
                    }
                }
                Ok(response) => {
                    let status = response.status();
                    let bytes = hardening::read_bounded(
                        response,
                        if status == StatusCode::PRECONDITION_FAILED {
                            4096
                        } else {
                            self.config.max_response_bytes
                        },
                    );
                    if let Ok(bytes) = &bytes {
                        if chat {
                            self.observe_usage(bytes, &reserved, accounting)?;
                        }
                        if status == StatusCode::PRECONDITION_FAILED {
                            let body = String::from_utf8_lossy(bytes).to_ascii_lowercase();
                            for indicator in [
                                "credit",
                                "balance",
                                "funds",
                                "billing",
                                "payment",
                                "limit",
                                "quota",
                                "budget",
                                "exceed",
                                "reached",
                                "zero",
                                "billing-account",
                                "pay-as-you-go",
                                "organization",
                                "disabled",
                                "suspended",
                                "inactive",
                            ] {
                                if body.contains(indicator) {
                                    self.report_category(
                                        request_id,
                                        format!("http_412_indicator_{indicator}"),
                                    );
                                }
                            }
                        }
                    }
                    if status == StatusCode::PRECONDITION_FAILED {
                        let category = bytes
                            .as_ref()
                            .map(|bytes| precondition_category(bytes))
                            .unwrap_or("unknown");
                        self.report_category(request_id, format!("http_412_{category}"));
                    }
                    return Err(ProviderError::HttpStatus { status });
                }
                Err(error) => {
                    if !error.is_connect() && !error.is_timeout() {
                        return Err(ProviderError::Http(error));
                    }
                    if attempt + 1 == attempts {
                        return Err(ProviderError::Http(error));
                    }
                }
            }
            if attempt + 1 < attempts {
                let base = self
                    .config
                    .retry_backoff
                    .saturating_mul(1u32 << attempt.min(7));
                let spread = (base.as_nanos() / 4).min(100_000_000) as u64;
                let jitter = std::hash::BuildHasher::hash_one(
                    &std::collections::hash_map::RandomState::new(),
                    (attempt, &url),
                ) % (spread + 1);
                let delay = base
                    .saturating_add(Duration::from_nanos(jitter))
                    .max(requested_delay);
                if delay >= deadline.saturating_duration_since(Instant::now()) {
                    return Err(ProviderError::BudgetExceeded);
                }
                lifetime
                    .wait(delay)
                    .map_err(|_| ProviderError::BudgetExceeded)?;
            }
        }
        Err(ProviderError::HttpStatus {
            status: last_status,
        })
    }
    fn observe_usage(
        &self,
        bytes: &[u8],
        reservation: &Usage,
        accounting: &mut UsageAccounting,
    ) -> Result<()> {
        let Ok(value) = parse_unique_json(bytes) else {
            return Ok(());
        };
        let Some(usage) = value.get("usage") else {
            return Ok(());
        };
        let (input, output, overflow) = if self.config.subscription {
            let Ok(usage) = serde_json::from_value::<SubscriptionUsage>(usage.clone()) else {
                return Ok(());
            };
            let input = usage
                .input_tokens
                .checked_add(usage.cache_creation_input_tokens)
                .and_then(|value| value.checked_add(usage.cache_read_input_tokens));
            (
                input.unwrap_or(u64::MAX),
                usage.output_tokens,
                input.is_none(),
            )
        } else {
            let Ok(usage) = serde_json::from_value::<ProviderUsage>(usage.clone()) else {
                return Ok(());
            };
            (usage.prompt_tokens, usage.completion_tokens, false)
        };
        accounting.reserved.input_tokens -= reservation.input_tokens;
        accounting.reserved.output_tokens -= reservation.output_tokens;
        accounting.reserved.cost_micros -= reservation.cost_micros;
        let cost = hardening::cost(
            hardening::price(&self.config),
            input.max(reservation.input_tokens),
            output,
        );
        accounting.accumulate(&UsageAccounting {
            observed: Usage {
                input_tokens: input,
                output_tokens: output,
                ..Usage::default()
            },
            reserved: Usage {
                input_tokens: reservation.input_tokens.saturating_sub(input),
                cost_micros: cost.as_ref().copied().unwrap_or(u64::MAX),
                ..Usage::default()
            },
            unaccounted: overflow || cost.is_err(),
        });
        if overflow
            || input > reservation.input_tokens
            || output > reservation.output_tokens
            || accounting.unaccounted
        {
            return Err(ProviderError::BudgetExceeded);
        }
        Ok(())
    }

    pub(super) fn charged_usage(
        &self,
        context: &AuthorizedContext,
        charge: Charge,
        input: u64,
        output: u64,
    ) -> Result<Usage> {
        if input > charge.input_per_round || output > charge.output_per_round {
            return Err(ProviderError::BudgetExceeded);
        }
        let failed_rounds = charge
            .rounds
            .checked_sub(1)
            .ok_or(ProviderError::BudgetExceeded)? as u64;
        let input = charge
            .input_per_round
            .checked_mul(failed_rounds)
            .and_then(|reserved| input.checked_add(reserved))
            .ok_or(ProviderError::BudgetExceeded)?;
        let output = charge
            .output_per_round
            .checked_mul(failed_rounds)
            .and_then(|reserved| output.checked_add(reserved))
            .ok_or(ProviderError::BudgetExceeded)?;
        let cost = hardening::cost(hardening::price(&self.config), input, output)?;
        if input > context.budget.max_input_tokens as u64
            || output > context.budget.max_output_tokens as u64
            || cost > context.budget.max_cost_micros
        {
            return Err(ProviderError::BudgetExceeded);
        }
        Ok(Usage {
            provider: Some(
                if self.config.subscription {
                    "codex_subscription"
                } else {
                    "openai_compatible"
                }
                .into(),
            ),
            model: Some(self.config.model.clone()),
            input_tokens: input,
            output_tokens: output,
            network_rounds: charge.rounds,
            cost_micros: cost,
        })
    }
}

#[derive(Deserialize)]
struct SubscriptionUsage {
    input_tokens: u64,
    output_tokens: u64,
    #[serde(default)]
    cache_creation_input_tokens: u64,
    #[serde(default)]
    cache_read_input_tokens: u64,
}

#[derive(Deserialize)]
struct ProviderUsage {
    prompt_tokens: u64,
    completion_tokens: u64,
}

struct ParsedTurn {
    model: String,
    blocks: Vec<ModelBlock>,
    finish_reason: ProviderFinishReason,
    input_tokens: u64,
    output_tokens: u64,
}

fn invalid_schema() -> ProviderError {
    ProviderError::InvalidResponse("invalid chat schema".into())
}

fn incomplete_turn(reason: ProviderFinishReason) -> ProviderError {
    ProviderError::InvalidResponse(
        match reason {
            ProviderFinishReason::MaxTokens => "provider output truncated",
            ProviderFinishReason::Refusal => "provider refused answer",
            _ => "finish reason does not match provider turn",
        }
        .into(),
    )
}

fn finish_reason(reason: &str, native: bool) -> Result<ProviderFinishReason> {
    match (native, reason) {
        (true, "end_turn") | (false, "stop") => Ok(ProviderFinishReason::EndTurn),
        (true, "tool_use") | (false, "tool_calls") => Ok(ProviderFinishReason::ToolUse),
        (true, "max_tokens") | (false, "length") => Ok(ProviderFinishReason::MaxTokens),
        (true, "refusal") | (false, "content_filter") => Ok(ProviderFinishReason::Refusal),
        _ => Err(ProviderError::InvalidResponse(
            "unknown provider finish reason".into(),
        )),
    }
}

fn checked_finish_reason(
    reason: &str,
    native: bool,
    check: &mut brain_contracts::response_audit::ResponseCheck,
) -> Result<ProviderFinishReason> {
    finish_reason(reason, native).inspect_err(|_| {
        *check = super::audit::check(
            "finish_reason",
            if native {
                "stop_reason"
            } else {
                "choices[0].finish_reason"
            },
            "known end_turn, tool_use, truncation or refusal reason",
        );
    })
}

fn checked_response<T: serde::de::DeserializeOwned>(
    bytes: &[u8],
    check: &mut brain_contracts::response_audit::ResponseCheck,
) -> Result<T> {
    let value = parse_unique_json(bytes).map_err(|error| {
        *check = super::audit::check(
            "chat_json",
            "response",
            &format!("unique valid JSON: {error}")
                .chars()
                .take(256)
                .collect::<String>(),
        );
        invalid_schema()
    })?;
    serde_path_to_error::deserialize(value).map_err(|error| {
        *check = super::audit::check(
            "chat_schema",
            &error.path().to_string(),
            &error
                .inner()
                .to_string()
                .chars()
                .take(256)
                .collect::<String>(),
        );
        invalid_schema()
    })
}

fn subscription_response(
    bytes: &[u8],
    check: &mut brain_contracts::response_audit::ResponseCheck,
) -> Result<ParsedTurn> {
    #[derive(Deserialize)]
    struct Response {
        model: String,
        content: Vec<Block>,
        stop_reason: String,
        usage: SubscriptionUsage,
    }
    #[derive(Deserialize)]
    #[serde(tag = "type", deny_unknown_fields)]
    enum Block {
        #[serde(rename = "text")]
        Text { text: String },
        #[serde(rename = "tool_use")]
        ToolUse {
            id: String,
            name: ToolName,
            input: Value,
        },
        #[serde(rename = "thinking")]
        Thinking {
            thinking: String,
            signature: Option<String>,
        },
    }
    let response: Response = checked_response(bytes, check)?;
    let mut blocks = Vec::new();
    for block in response.content {
        match block {
            Block::Text { text } => blocks.push(ModelBlock::Text { text }),
            Block::ToolUse { id, name, input } => blocks.push(ModelBlock::ToolUse {
                call: ToolCall {
                    id,
                    name,
                    arguments: input,
                },
            }),
            Block::Thinking {
                thinking,
                signature,
            } => {
                let _ = (thinking, signature);
            }
        }
    }
    Ok(ParsedTurn {
        model: response.model,
        blocks,
        finish_reason: checked_finish_reason(&response.stop_reason, true, check)?,
        input_tokens: response
            .usage
            .input_tokens
            .checked_add(response.usage.cache_creation_input_tokens)
            .and_then(|input| input.checked_add(response.usage.cache_read_input_tokens))
            .ok_or(ProviderError::BudgetExceeded)?,
        output_tokens: response.usage.output_tokens,
    })
}

fn compatible_response(
    bytes: &[u8],
    check: &mut brain_contracts::response_audit::ResponseCheck,
) -> Result<ParsedTurn> {
    #[derive(Deserialize)]
    struct Response {
        model: String,
        choices: Vec<Choice>,
        usage: ProviderUsage,
    }
    #[derive(Deserialize)]
    struct Choice {
        message: Message,
        finish_reason: String,
    }
    #[derive(Deserialize)]
    struct Message {
        role: Option<String>,
        content: Content,
        #[serde(default)]
        tool_calls: Vec<Call>,
        refusal: Option<String>,
    }
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Content {
        Text(String),
        Null(()),
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Call {
        id: String,
        #[serde(rename = "type")]
        kind: String,
        function: Function,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Function {
        name: ToolName,
        arguments: String,
    }
    let response: Response = checked_response(bytes, check)?;
    if response.choices.len() != 1 {
        *check = super::audit::check("choice_count", "choices", "exactly one assistant choice");
        return Err(ProviderError::InvalidResponse(
            "model or choice count mismatch".into(),
        ));
    }
    let choice = response
        .choices
        .into_iter()
        .next()
        .ok_or_else(invalid_schema)?;
    if choice
        .message
        .role
        .as_deref()
        .is_some_and(|role| role != "assistant")
    {
        *check = super::audit::check("assistant_role", "choices[0].message.role", "assistant");
        return Err(invalid_schema());
    }
    if choice
        .message
        .refusal
        .is_some_and(|refusal| !refusal.is_empty())
    {
        *check = super::audit::check("refusal", "choices[0].message.refusal", "no model refusal");
        return Err(incomplete_turn(ProviderFinishReason::Refusal));
    }
    let mut blocks = Vec::new();
    if let Content::Text(text) = choice.message.content {
        blocks.push(ModelBlock::Text { text });
    }
    for call in choice.message.tool_calls {
        if call.kind != "function" {
            *check = super::audit::check("tool_call_type", "tool_calls.type", "function");
            return Err(invalid_schema());
        }
        let arguments = parse_unique_json(call.function.arguments.as_bytes()).map_err(|error| {
            *check = super::audit::check(
                "tool_arguments",
                "tool_calls.function.arguments",
                &format!("unique valid JSON: {error}")
                    .chars()
                    .take(256)
                    .collect::<String>(),
            );
            invalid_schema()
        })?;
        blocks.push(ModelBlock::ToolUse {
            call: ToolCall {
                id: call.id,
                name: call.function.name,
                arguments,
            },
        });
    }
    Ok(ParsedTurn {
        model: response.model,
        blocks,
        finish_reason: checked_finish_reason(&choice.finish_reason, false, check)?,
        input_tokens: response.usage.prompt_tokens,
        output_tokens: response.usage.completion_tokens,
    })
}

fn unverified_answer(raw: &str, usage: Usage, evidence: &[Evidence]) -> Result<ProviderAnswer> {
    let raw = raw.trim();
    let body = raw
        .strip_prefix("```")
        .and_then(|fenced| fenced.split_once('\n').map(|(_, body)| body))
        .map(|body| body.strip_suffix("```").unwrap_or(body).trim())
        .unwrap_or(raw);
    let decoded = parse_unique_json(body.as_bytes()).ok();
    let extracted = decoded.as_ref().and_then(|value| {
        value
            .as_str()
            .or_else(|| value.get("text").and_then(Value::as_str))
    });
    let partial = if extracted.is_none() && body.starts_with('{') {
        body.find("\"text\"").and_then(|start| {
            let rest = &body[start + "\"text\"".len()..];
            let rest = rest.trim_start().strip_prefix(':')?.trim_start();
            match serde_json::Deserializer::from_str(rest)
                .into_iter::<String>()
                .next()?
            {
                Ok(text) => Some(text),
                Err(error) if error.is_eof() && rest.starts_with('"') => {
                    let mut completed = rest.to_owned();
                    if completed
                        .as_bytes()
                        .iter()
                        .rev()
                        .take_while(|byte| **byte == b'\\')
                        .count()
                        % 2
                        == 1
                    {
                        completed.pop();
                    }
                    completed.push('"');
                    serde_json::from_str(&completed).ok()
                }
                Err(_) => None,
            }
        })
    } else {
        None
    };
    let answer_text = extracted.or(partial.as_deref());
    if answer_text.is_none() && body.starts_with(['{', '[']) {
        return Err(ProviderError::InvalidResponse(
            "structured answer text missing".into(),
        ));
    }
    let text = answer_text.unwrap_or(body);
    let mut text = brain_contracts::provider_input::discord_display_text(text);
    text = text
        .split("[[ev-")
        .enumerate()
        .filter_map(|(index, part)| {
            if index == 0 {
                Some(part)
            } else {
                part.split_once("]]").map(|(_, rest)| rest)
            }
        })
        .collect();
    for item in evidence {
        text = text.replace(&item.evidence_id, "");
    }
    text = text.replace("[[]]", "");
    if text.trim().is_empty() || text.len() > 64 * 1024 {
        return Err(ProviderError::InvalidResponse(
            "empty or oversized unverified answer".into(),
        ));
    }
    Ok(ProviderAnswer {
        text,
        cited_evidence_ids: Vec::new(),
        usage,
    })
}

fn grounded_answer(
    text: String,
    usage: Usage,
    evidence: &[Evidence],
    require_grounded: bool,
    check: &mut brain_contracts::response_audit::ResponseCheck,
) -> Result<ProviderAnswer> {
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Grounded {
        text: String,
        cited_evidence_ids: Vec<String>,
    }
    let text = text.trim().to_owned();
    if text.is_empty() {
        return Err(ProviderError::InvalidResponse(
            "empty provider answer".into(),
        ));
    }
    let (text, cited_evidence_ids) = if evidence.is_empty() && !require_grounded {
        if parse_unique_json(text.as_bytes()).is_err_and(|error| error.is_data()) {
            return Err(ProviderError::InvalidResponse(
                "grounded answer envelope missing".into(),
            ));
        }
        (text, Vec::new())
    } else {
        let envelope_error =
            || ProviderError::InvalidResponse("grounded answer envelope missing".into());
        let grounded: Grounded = checked_response(text.as_bytes(), check).map_err(|_| {
            check.check = "grounded_envelope".into();
            check.field = format!("message.content.{}", check.field);
            envelope_error()
        })?;
        let unique: std::collections::BTreeSet<_> = grounded.cited_evidence_ids.iter().collect();
        let insufficient = grounded.text.is_empty() && unique.is_empty();
        if (!insufficient && (grounded.text.trim().is_empty() || unique.is_empty()))
            || unique.len() != grounded.cited_evidence_ids.len()
            || unique
                .iter()
                .any(|id| !evidence.iter().any(|item| &item.evidence_id == *id))
        {
            *check = super::audit::check(
                "citation_invalid",
                "cited_evidence_ids",
                "nonempty unique IDs from the supplied evidence",
            );
            return Err(ProviderError::InvalidResponse(
                "unknown, duplicate or missing citation".into(),
            ));
        }
        (grounded.text, grounded.cited_evidence_ids)
    };
    Ok(ProviderAnswer {
        text,
        cited_evidence_ids,
        usage,
    })
}

fn precondition_category(bytes: &[u8]) -> &'static str {
    let body = String::from_utf8_lossy(bytes).to_ascii_lowercase();
    if [
        "account disabled",
        "account suspended",
        "account is disabled",
        "account is suspended",
    ]
    .iter()
    .any(|word| body.contains(word))
    {
        "account_disabled"
    } else if (["credit", "balance", "funds"]
        .iter()
        .any(|word| body.contains(word)))
        && [
            "insufficient",
            "negative",
            "exhausted",
            "depleted",
            "not enough",
            "too low",
            "out of credits",
            "no remaining",
        ]
        .iter()
        .any(|word| body.contains(word))
    {
        "credit_insufficient"
    } else if ["payment required", "payment method", "billing not enabled"]
        .iter()
        .any(|word| body.contains(word))
    {
        "payment_required"
    } else if [
        "billing",
        "payment",
        "credit",
        "balance",
        "account disabled",
        "account suspended",
    ]
    .iter()
    .any(|word| body.contains(word))
    {
        "account_or_billing"
    } else if ["api key", "api_key", "permission", "unauthorized"]
        .iter()
        .any(|word| body.contains(word))
    {
        "api_key_policy"
    } else if ["parameter", "reasoning", "max_tokens", "token limit"]
        .iter()
        .any(|word| body.contains(word))
    {
        "request_parameter"
    } else if ["moderation", "policy", "firewall"]
        .iter()
        .any(|word| body.contains(word))
    {
        "provider_policy"
    } else if body.contains("deploy") {
        "deployment_required"
    } else if ["model", "license", "terms"]
        .iter()
        .any(|word| body.contains(word))
    {
        "model_disabled_or_access"
    } else {
        "unknown"
    }
}

#[cfg(test)]
mod precondition_tests {
    #[test]
    fn specific_rejections_take_precedence_over_model_mentions() {
        assert_eq!(
            super::precondition_category(b"model deployment does not support reasoning parameter"),
            "request_parameter"
        );
        assert_eq!(
            super::precondition_category(b"model blocked by policy"),
            "provider_policy"
        );
        assert_eq!(
            super::precondition_category(b"Account has insufficient credit balance"),
            "credit_insufficient"
        );
        assert_eq!(
            super::precondition_category(b"accounts/fireworks/models/example request rejected"),
            "model_disabled_or_access"
        );
    }
}
