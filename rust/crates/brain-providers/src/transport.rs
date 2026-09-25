use super::{
    hardening, thread, ChatRequest, ChatResponse, Duration, Instant, OpenAiCompatibleProvider,
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
        let (bytes, charge) = self.transport_json("chat/completions", &mut json, context, true)?;
        let parsed: ChatResponse = serde_json::from_slice(&bytes)
            .map_err(|_| ProviderError::InvalidResponse("invalid chat schema".into()))?;
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
            if grounded.text.trim().is_empty()
                || unique.is_empty()
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
        let budget = &context.budget;
        if context.deadline_ms == 0
            || context.deadline_ms > 60000
            || budget.max_network_rounds == 0
            || budget.max_input_tokens == 0
        {
            return Err(ProviderError::BudgetExceeded);
        }
        // UTF-8 byte count plus framing is a conservative reservation for configured byte-token models.
        // Production activation must verify this ceiling for the selected model/tokenizer.
        let input = serde_json::to_vec(payload)
            .map_err(|_| ProviderError::InvalidConfig)?
            .len() as u64
            + 64;
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
            input * attempts as u64,
            output * attempts as u64,
        )?;
        if reserved_cost > budget.max_cost_micros {
            return Err(ProviderError::BudgetExceeded);
        }
        let deadline = Instant::now() + Duration::from_millis(context.deadline_ms);
        let url = format!("{}/{}", self.config.base_url.trim_end_matches('/'), route);
        let mut last_status = StatusCode::SERVICE_UNAVAILABLE;
        for attempt in 0..attempts {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                return Err(ProviderError::BudgetExceeded);
            }
            let result = self
                .client
                .post(&url)
                .timeout(self.config.timeout.min(remaining))
                .bearer_auth(&self.config.api_key)
                .json(payload)
                .send();
            let mut requested_delay = Duration::ZERO;
            match result {
                Ok(response) if response.status().is_success() => {
                    let bytes = hardening::read_bounded(response, self.config.max_response_bytes)?;
                    if Instant::now() >= deadline {
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
                    return Err(ProviderError::HttpStatus {
                        status: response.status(),
                    })
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
                thread::sleep(delay);
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
            provider: Some("openai_compatible".into()),
            model: Some(self.config.model.clone()),
            input_tokens: input,
            output_tokens: output,
            network_rounds: charge.rounds,
            cost_micros: cost,
        })
    }
}
