use super::{PriceCeiling, ProviderConfig, ProviderError, Result};
use brain_contracts::{AuthorizedContext, Evidence, Query, SourceVisibility};
use std::{io::Read, net::IpAddr, time::Duration};

pub(super) fn validate_endpoint(config: &ProviderConfig) -> Result<()> {
    let url = reqwest::Url::parse(&config.base_url).map_err(|_| ProviderError::InvalidConfig)?;
    let loopback = url.host_str().is_some_and(|host| {
        host == "localhost"
            || host
                .trim_matches(['[', ']'])
                .parse::<IpAddr>()
                .is_ok_and(|ip| ip.is_loopback())
    });
    if !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || !(url.scheme() == "https" || (url.scheme() == "http" && loopback))
        || (!loopback && config.pricing.is_none())
        || config.timeout.is_zero()
        || config.timeout > Duration::from_secs(600)
        || !(1..=8).contains(&config.retry_attempts)
        || !(128..=8 * 1024 * 1024).contains(&config.max_response_bytes)
    {
        return Err(ProviderError::InvalidConfig);
    }
    Ok(())
}
pub(super) fn authorize(
    query: &Query,
    context: &AuthorizedContext,
    evidence: &[Evidence],
) -> Result<()> {
    if query.validate().is_err()
        || query.conversation_id != context.conversation_id
        || context.knowledge_release.trim().is_empty()
        || context.deadline_ms == 0
        || context.deadline_ms > 600_000
        || !context.principal.provider_egress.contains("public")
    {
        return Err(ProviderError::InvalidResponse(
            "invalid provider context or query egress".into(),
        ));
    }
    if brain_contracts::invite::requested(query)
        || evidence
            .iter()
            .any(|item| item.source_id == brain_contracts::invite::SOURCE)
    {
        brain_contracts::invite::validate_projection(query, context, evidence, true)
            .map_err(super::provider_contract_error)?;
    }
    if evidence.len() > 100 {
        return Err(ProviderError::InvalidResponse(
            "too many evidence items".into(),
        ));
    }
    for item in evidence {
        if item.visibility == SourceVisibility::RequestScoped
            && !context.discord.as_ref().is_some_and(|request| {
                request.request_id == query.request_id
                    && item.allowed_scopes
                        == std::collections::BTreeSet::from([request.scope.clone()])
            })
        {
            return Err(ProviderError::InvalidResponse(
                "request evidence denied".into(),
            ));
        }
        item.validate()
            .map_err(|_| ProviderError::InvalidResponse("invalid evidence".into()))?;
        let class = match item.visibility {
            SourceVisibility::Public => "public",
            SourceVisibility::Internal => "internal",
            SourceVisibility::Private => "private",
            SourceVisibility::RequestScoped => "discord_request",
        };
        if (item.visibility != SourceVisibility::Public && item.allowed_scopes.is_empty())
            || !item.allowed_scopes.is_subset(&context.principal.scopes)
            || !context.principal.provider_egress.contains(class)
        {
            return Err(ProviderError::InvalidResponse(
                "evidence egress denied".into(),
            ));
        }
    }
    Ok(())
}
pub(super) fn authorize_turn(
    query: &Query,
    context: &AuthorizedContext,
    evidence: &[Evidence],
    tools: &[brain_contracts::ToolDefinition],
    conversation: &brain_contracts::ToolConversation,
) -> Result<()> {
    authorize(query, context, evidence)?;
    if brain_contracts::invite::requested(query)
        && (!tools.is_empty() || !conversation.messages.is_empty())
    {
        return Err(ProviderError::InvalidResponse(
            "invite tool conversation denied".into(),
        ));
    }
    conversation
        .validate(tools)
        .map_err(super::provider_contract_error)?;
    let ids: std::collections::BTreeSet<_> = evidence
        .iter()
        .map(|item| item.evidence_id.as_str())
        .collect();
    if ids.len() != evidence.len() || conversation.messages.iter().any(|message| {
        matches!(message, brain_contracts::ToolMessage::ToolResults { results } if results.iter().any(|result| result.evidence_ids.iter().any(|id| !ids.contains(id.as_str()))))
    }) {
        return Err(ProviderError::InvalidResponse("tool evidence egress denied".into()));
    }
    Ok(())
}

pub(super) fn price(config: &ProviderConfig) -> PriceCeiling {
    config.pricing.unwrap_or(PriceCeiling {
        input_micros_per_token: 0,
        output_micros_per_token: 0,
    })
}
pub(super) fn cost(price: PriceCeiling, input: u64, output: u64) -> Result<u64> {
    input
        .checked_mul(price.input_micros_per_token)
        .and_then(|i| {
            output
                .checked_mul(price.output_micros_per_token)
                .and_then(|o| i.checked_add(o))
        })
        .ok_or(ProviderError::BudgetExceeded)
}
pub(super) fn read_bounded(response: reqwest::blocking::Response, limit: usize) -> Result<Vec<u8>> {
    let (result, bytes) = read_bounded_observed(response, limit);
    result.map(|()| bytes)
}
pub(super) fn read_bounded_observed(
    response: reqwest::blocking::Response,
    limit: usize,
) -> (Result<()>, Vec<u8>) {
    let announced_overflow = response.content_length().is_some_and(|n| n > limit as u64);
    let mut bytes = Vec::new();
    let result = response
        .take(limit as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(ProviderError::BodyTransport);
    let result = result.and_then(|_| {
        if announced_overflow || bytes.len() > limit {
            Err(ProviderError::ResponseTooLarge)
        } else {
            Ok(())
        }
    });
    (result, bytes)
}
pub(super) fn retry_after(response: &reqwest::blocking::Response) -> Result<Option<Duration>> {
    let Some(header) = response.headers().get(reqwest::header::RETRY_AFTER) else {
        return Ok(None);
    };
    let text = header
        .to_str()
        .map_err(|_| ProviderError::InvalidResponse("invalid Retry-After".into()))?;
    if let Ok(seconds) = text.parse::<u64>() {
        return Ok(Some(Duration::from_secs(seconds)));
    }
    let date = httpdate::parse_http_date(text)
        .map_err(|_| ProviderError::InvalidResponse("invalid Retry-After date".into()))?;
    Ok(Some(
        date.duration_since(std::time::SystemTime::now())
            .unwrap_or_default(),
    ))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn endpoint_and_price_are_explicit() {
        for endpoint in [
            "http://example.com/v1",
            "https://token@example.com/v1",
            "https://example.com/v1?key=bad",
            "https://example.com/v1#secret",
        ] {
            let mut c = ProviderConfig::new("fixture", endpoint, "fixture");
            c.pricing = Some(PriceCeiling {
                input_micros_per_token: 1,
                output_micros_per_token: 2,
            });
            assert!(validate_endpoint(&c).is_err());
        }
        let c = ProviderConfig::new("fixture", "https://example.com/v1", "fixture");
        assert!(validate_endpoint(&c).is_err());
        assert!(validate_endpoint(&ProviderConfig::new(
            "fixture",
            "http://127.0.0.1:1234",
            "fixture"
        ))
        .is_ok());
    }
    #[test]
    fn statusbelege_duerfen_weder_gemischt_noch_fuer_allgemeine_fragen_ausgehen() {
        use brain_contracts::{invite, Budget, DiscordRequestContext, Principal};
        use std::collections::BTreeSet;
        let mut query: Query = serde_json::from_value(serde_json::json!({
            "request_id":"fixture-request", "conversation_id":"fixture-conversation",
            "text":invite::QUESTION, "requested_scopes":["bot.public"],
        }))
        .unwrap();
        let context = AuthorizedContext {
            discord: Some(DiscordRequestContext {
                user_id: Some(42),
                request_id: query.request_id.clone(),
                scope: "discord.request:fixture".into(),
                allow_discord_reads: false,
            }),
            principal: Principal {
                actor_id: "fixture".into(),
                channel: "discord".into(),
                scopes: BTreeSet::from(["bot.public".into(), "discord.request:fixture".into()]),
                provider_egress: BTreeSet::from(["public".into(), "discord_request".into()]),
            },
            conversation_id: query.conversation_id.clone(),
            knowledge_release: "fixture".into(),
            deadline_ms: 1000,
            budget: Budget::default(),
            request_deadline: None,
        };
        let item = invite::status_evidence(
            &invite::SelfInviteStatus {
                status: invite::InviteStatus::Unknown,
                at: None,
            },
            "discord.request:fixture".into(),
        )
        .unwrap();
        assert!(authorize(&query, &context, std::slice::from_ref(&item)).is_ok());
        assert!(authorize(&query, &context, &[item.clone(), item.clone()]).is_err());
        let mut forged = item.clone();
        forged.content = "{\"status\":\"sent\",\"at\":null,\"user_id\":99}".into();
        assert!(authorize(&query, &context, &[forged]).is_err());
        query.text = "Wie funktioniert der Invite-Bot?".into();
        assert!(authorize(&query, &context, &[item]).is_err());
    }

    #[test]
    fn ten_minute_provider_timeout_has_an_explicit_upper_bound() {
        let mut config = ProviderConfig::new("fixture", "http://127.0.0.1:1234", "fixture");
        config.timeout = Duration::from_secs(600);
        assert!(validate_endpoint(&config).is_ok());
        config.timeout = Duration::from_millis(600_001);
        assert!(validate_endpoint(&config).is_err());
    }

    #[test]
    fn cost_overflow_fails_closed() {
        assert!(cost(
            PriceCeiling {
                input_micros_per_token: u64::MAX,
                output_micros_per_token: 1
            },
            2,
            0
        )
        .is_err());
    }
}
