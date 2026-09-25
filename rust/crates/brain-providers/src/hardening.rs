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
        || config.timeout > Duration::from_secs(60)
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
        || context.deadline_ms > 60000
        || !context.principal.provider_egress.contains("public")
    {
        return Err(ProviderError::InvalidResponse(
            "invalid provider context or query egress".into(),
        ));
    }
    if evidence.len() > 100 {
        return Err(ProviderError::InvalidResponse(
            "too many evidence items".into(),
        ));
    }
    for item in evidence {
        item.validate()
            .map_err(|_| ProviderError::InvalidResponse("invalid evidence".into()))?;
        let class = match item.visibility {
            SourceVisibility::Public => "public",
            SourceVisibility::Internal => "internal",
            SourceVisibility::Private => "private",
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
    if response.content_length().is_some_and(|n| n > limit as u64) {
        return Err(ProviderError::ResponseTooLarge);
    }
    let mut bytes = Vec::new();
    response
        .take(limit as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| {
            ProviderError::InvalidResponse("truncated or timed-out provider body".into())
        })?;
    if bytes.len() > limit {
        return Err(ProviderError::ResponseTooLarge);
    }
    Ok(bytes)
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
