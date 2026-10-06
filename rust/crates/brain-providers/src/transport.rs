use super::{
    hardening, ChatRequest, ChatResponse, Duration, Instant, OpenAiCompatibleProvider,
    ProviderAnswer, ProviderError, Result, StatusCode, Usage,
};
use brain_contracts::AuthorizedContext;

pub(super) struct Charge {
    input_per_round: u64,
    output_per_round: u64,
    rounds: u32,
}
impl OpenAiCompatibleProvider {
    pub(super) fn send_chat(
        &self,
        payload: ChatRequest,
        context: &AuthorizedContext,
        evidence: &[brain_contracts::Evidence],
    ) -> Result<ProviderAnswer> {
        let mut json = serde_json::to_value(payload).map_err(|_| ProviderError::InvalidConfig)?;
        if self.config.model == "accounts/fireworks/models/deepseek-v4p1-flash" {
            json["reasoning_effort"] = serde_json::json!("none");
        }
        let route = if self.config.subscription {
            "messages"
        } else {
            "chat/completions"
        };
        let (bytes, charge) = self.transport_json(route, &mut json, context, true)?;
        let parsed: ChatResponse = if self.config.subscription {
            subscription_response(&bytes)?
        } else {
            serde_json::from_slice(&bytes)
                .map_err(|_| ProviderError::InvalidResponse("invalid chat schema".into()))?
        };
        if parsed.model.as_deref() != Some(self.config.model.as_str()) || parsed.choices.len() != 1
        {
            return Err(ProviderError::InvalidResponse(
                "model or choice count mismatch".into(),
            ));
        }
        let text = parsed.choices[0].message.content.trim().to_owned();
        if text.is_empty() {
            return Err(ProviderError::InvalidResponse(
                "empty provider answer".into(),
            ));
        }
        let usage = parsed
            .usage
            .ok_or_else(|| ProviderError::InvalidResponse("provider usage missing".into()))?;
        let usage = self.charged_usage(
            context,
            charge,
            usage.prompt_tokens,
            usage.completion_tokens,
        )?;
        #[derive(serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Grounded {
            text: String,
            cited_evidence_ids: Vec<String>,
        }
        let (text, cited_evidence_ids) = if evidence.is_empty() {
            (text, Vec::new())
        } else {
            let grounded: Grounded = serde_json::from_str(&text).map_err(|_| {
                ProviderError::InvalidResponse("grounded answer envelope missing".into())
            })?;
            let unique: std::collections::BTreeSet<_> =
                grounded.cited_evidence_ids.iter().collect();
            let insufficient = grounded.text.is_empty() && unique.is_empty();
            if (!insufficient && (grounded.text.trim().is_empty() || unique.is_empty()))
                || unique.len() != grounded.cited_evidence_ids.len()
                || unique
                    .iter()
                    .any(|id| !evidence.iter().any(|e| &e.evidence_id == *id))
            {
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
    pub(super) fn transport_json(
        &self,
        route: &str,
        payload: &mut serde_json::Value,
        context: &AuthorizedContext,
        chat: bool,
    ) -> Result<(Vec<u8>, Charge)> {
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
        let input = brain_contracts::provider_input::transport_input_ceiling(payload, chat)
            .map_err(|_| ProviderError::InvalidConfig)?;
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
        if self.config.subscription {
            if !chat || route != "messages" {
                return Err(ProviderError::InvalidConfig);
            }
            let messages = payload["messages"]
                .as_array()
                .ok_or(ProviderError::InvalidConfig)?;
            if messages.len() != 2
                || messages[0]["role"] != "system"
                || messages[1]["role"] != "user"
            {
                return Err(ProviderError::InvalidConfig);
            }
            *payload = serde_json::json!({
                "model": self.config.model,
                "system": messages[0]["content"],
                "messages": [messages[1]],
                "max_tokens": output,
                "stream": false,
                "tools": [],
                "tool_choice": {"type": "none"},
                "output_config": {"effort": "low"}
            });
        }
        let reserved_cost = hardening::cost(
            hardening::price(&self.config),
            input * attempts as u64,
            output * attempts as u64,
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
            // Payload encoding and retries spend time from the same original lifetime.
            lifetime
                .check()
                .map_err(|_| ProviderError::BudgetExceeded)?;
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                return Err(ProviderError::BudgetExceeded);
            }
            let result = request.timeout(self.config.timeout.min(remaining)).send();
            lifetime
                .check()
                .map_err(|_| ProviderError::BudgetExceeded)?;
            let mut requested_delay = Duration::ZERO;
            match result {
                Ok(response) if response.status().is_success() => {
                    let bytes = hardening::read_bounded(response, self.config.max_response_bytes)?;
                    if Instant::now() >= deadline || lifetime.check().is_err() {
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
                }
                Ok(response) => {
                    let status = response.status();
                    if status == StatusCode::PRECONDITION_FAILED {
                        let category = hardening::read_bounded(response, 4096)
                            .map(|bytes| {
                                let body = String::from_utf8_lossy(&bytes).to_ascii_lowercase();
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
                                        self.report_category(format!(
                                            "http_412_indicator_{indicator}"
                                        ));
                                    }
                                }
                                precondition_category(&bytes)
                            })
                            .unwrap_or("unknown");
                        self.report_category(format!("http_412_{category}"));
                        return Err(ProviderError::HttpStatus { status });
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
        // Failed calls have unknown usage; charge their FULL reservation, never zero.
        let input = input + charge.input_per_round * (charge.rounds - 1) as u64;
        let output = output + charge.output_per_round * (charge.rounds - 1) as u64;
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

fn subscription_response(bytes: &[u8]) -> Result<ChatResponse> {
    #[derive(serde::Deserialize)]
    struct Response {
        model: String,
        content: Vec<Block>,
        usage: SubscriptionUsage,
    }
    #[derive(serde::Deserialize)]
    struct Block {
        #[serde(rename = "type")]
        kind: String,
        text: Option<String>,
    }
    #[derive(serde::Deserialize)]
    struct SubscriptionUsage {
        input_tokens: u64,
        output_tokens: u64,
    }
    let response: Response = serde_json::from_slice(bytes)
        .map_err(|_| ProviderError::InvalidResponse("invalid chat schema".into()))?;
    let mut text = String::new();
    for block in response.content {
        match block.kind.as_str() {
            "text" => text.push_str(
                &block
                    .text
                    .ok_or_else(|| ProviderError::InvalidResponse("invalid chat schema".into()))?,
            ),
            "thinking" => {}
            _ => return Err(ProviderError::InvalidResponse("invalid chat schema".into())),
        }
    }
    Ok(ChatResponse {
        model: Some(response.model),
        choices: vec![super::Choice {
            message: super::ResponseMessage { content: text },
        }],
        usage: Some(super::ProviderUsage {
            prompt_tokens: response.usage.input_tokens,
            completion_tokens: response.usage.output_tokens,
        }),
    })
}

// Der Fehlertext bleibt im Prozess. Ausgegeben werden ausschließlich feste Klassen.
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
