//! Shared wire validation for both transports. Never a model or retrieval fallback.
use crate::{ClientError, Result};
use brain_contracts::{PublicAnswerResponse, Query};
use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    StatusCode,
};
use std::{net::IpAddr, time::Duration};

pub const MAX_REQUEST_BYTES: usize = 64 * 1024;
pub const MAX_RESPONSE_BYTES: usize = 512 * 1024;

pub(crate) fn endpoint(
    base_url: &str,
    bearer_token: &str,
    timeout: Duration,
) -> Result<(String, HeaderValue)> {
    let base_url = base_url.trim().trim_end_matches('/');
    let parsed = reqwest::Url::parse(base_url).map_err(|_| ClientError::InvalidBaseUrl)?;
    let loopback = parsed.host_str().is_some_and(|h| {
        h == "localhost"
            || h.trim_matches(['[', ']'])
                .parse::<IpAddr>()
                .is_ok_and(|ip| ip.is_loopback())
    });
    if !(parsed.scheme() == "https" || (parsed.scheme() == "http" && loopback))
        || parsed.host_str().is_none()
        || !parsed.username().is_empty()
        || parsed.password().is_some()
        || parsed.query().is_some()
        || parsed.fragment().is_some()
        || timeout.is_zero()
        || timeout > Duration::from_secs(60)
    {
        return Err(ClientError::InvalidBaseUrl);
    }
    if bearer_token.is_empty()
        || bearer_token.len() > 4096
        || bearer_token.chars().any(char::is_whitespace)
    {
        return Err(ClientError::InvalidToken);
    }
    let mut bearer = HeaderValue::from_str(&format!("Bearer {bearer_token}"))
        .map_err(|_| ClientError::InvalidToken)?;
    bearer.set_sensitive(true);
    Ok((base_url.to_owned(), bearer))
}

pub(crate) fn encode_request(query: &Query) -> Result<Vec<u8>> {
    query
        .validate()
        .map_err(|error| ClientError::InvalidQuery(error.to_string()))?;
    let bytes = serde_json::to_vec(query)?;
    if bytes.len() > MAX_REQUEST_BYTES {
        return Err(ClientError::RequestTooLarge);
    }
    Ok(bytes)
}

pub(crate) fn json_content_type(headers: &HeaderMap) -> bool {
    headers
        .get(CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| {
            v.split(';')
                .next()
                .is_some_and(|m| m.trim().eq_ignore_ascii_case("application/json"))
        })
}

pub(crate) fn decode_response(
    status: StatusCode,
    is_json: bool,
    bytes: &[u8],
    request_id: &str,
) -> Result<PublicAnswerResponse> {
    if bytes.len() > MAX_RESPONSE_BYTES {
        return Err(ClientError::ResponseTooLarge);
    }
    if status != StatusCode::OK {
        // A syntactically valid arbitrary error code could itself contain a secret.
        // Only known protocol codes, never upstream messages, cross this boundary.
        let code = serde_json::from_slice::<brain_contracts::ApiErrorEnvelope>(bytes)
            .ok()
            .map(|e| e.error.code)
            .filter(|code| {
                matches!(
                    code.as_str(),
                    "unauthorized"
                        | "forbidden"
                        | "invalid_query"
                        | "invalid_json"
                        | "invalid_request"
                        | "payload_too_large"
                        | "unsupported_media_type"
                        | "not_found"
                        | "overloaded"
                        | "worker_unavailable"
                        | "deadline_exceeded"
                )
            })
            .unwrap_or_else(|| "http_error".into());
        return Err(ClientError::HttpStatus { status, body: code });
    }
    if !is_json {
        return Err(ClientError::InvalidResponse);
    }
    let answer: PublicAnswerResponse = serde_json::from_slice(bytes)?;
    answer
        .validate(request_id)
        .map_err(|_| ClientError::InvalidResponse)?;
    Ok(answer)
}
