//! Native async Brain transport. Dropping the future cancels this HTTP request;
//! it does not create a detached blocking worker, retry, provider call or fallback.
use crate::{transport, ClientError, Result};
use brain_contracts::{PublicAnswerResponse, Query};
use reqwest::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use std::time::Duration;

#[derive(Clone)]
pub struct AsyncBrainClient {
    client: reqwest::Client,
    base_url: String,
    bearer: HeaderValue,
}
impl std::fmt::Debug for AsyncBrainClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncBrainClient")
            .field("endpoint", &"<configured>")
            .field("bearer", &"<redacted>")
            .finish()
    }
}
impl AsyncBrainClient {
    /// For consumers of local community/internal knowledge. HTTPS is not an
    /// implicit authorization to send their requests to an external host.
    pub fn new_local(base_url: &str, bearer_token: &str, timeout: Duration) -> Result<Self> {
        let parsed =
            reqwest::Url::parse(base_url.trim()).map_err(|_| ClientError::InvalidBaseUrl)?;
        let local = parsed.host_str().is_some_and(|host| {
            host == "localhost"
                || host
                    .trim_matches(['[', ']'])
                    .parse::<std::net::IpAddr>()
                    .is_ok_and(|ip| ip.is_loopback())
        });
        if !local {
            return Err(ClientError::InvalidBaseUrl);
        }
        Self::new(base_url, bearer_token, timeout)
    }
    /// Explicit caller configuration only. Nothing is read from production config.
    pub fn new(base_url: &str, bearer_token: &str, timeout: Duration) -> Result<Self> {
        let (base_url, bearer) = transport::endpoint(base_url, bearer_token, timeout)?;
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .connect_timeout(timeout.min(Duration::from_secs(3)))
            .redirect(reqwest::redirect::Policy::none())
            .no_proxy()
            .no_gzip()
            .no_brotli()
            .no_deflate()
            .no_zstd()
            .build()?;
        Ok(Self {
            client,
            base_url,
            bearer,
        })
    }
    pub async fn answer(&self, query: &Query) -> Result<PublicAnswerResponse> {
        let request = transport::encode_request(query)?;
        let mut response = self
            .client
            .post(format!("{}/v1/answer", self.base_url))
            .header(AUTHORIZATION, self.bearer.clone())
            .header(CONTENT_TYPE, "application/json")
            .body(request)
            .send()
            .await?;
        let status = response.status();
        if response
            .content_length()
            .is_some_and(|n| n > transport::MAX_RESPONSE_BYTES as u64)
        {
            return Err(ClientError::ResponseTooLarge);
        }
        let is_json = transport::json_content_type(response.headers());
        let mut bytes = Vec::new();
        while let Some(chunk) = response.chunk().await? {
            if chunk.len() > transport::MAX_RESPONSE_BYTES.saturating_sub(bytes.len()) {
                return Err(ClientError::ResponseTooLarge);
            }
            bytes.extend_from_slice(&chunk);
        }
        transport::decode_response(status, is_json, &bytes, &query.request_id)
    }
}
