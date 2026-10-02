use reqwest::{
    header::{HeaderValue, AUTHORIZATION},
    redirect::Policy,
    Client, Url,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JevTransportConfig {
    pub endpoint: String,
    pub model: String,
    pub timeout_ms: u64,
    pub max_request_bytes: usize,
    pub max_response_bytes: usize,
}

#[derive(Debug, Error)]
pub enum TransportError {
    #[error("Ungültige Jev-Konfiguration")]
    Config,
    #[error("Jev-Eingabe überschreitet die Grenze oder den Modellvertrag")]
    Request,
    #[error("Jev-Transport fehlgeschlagen")]
    Transport,
    #[error("Jev antwortet mit HTTP-Status {0}")]
    Http(u16),
    #[error("Jev-Antwort überschreitet die Grenze")]
    ResponseLimit,
    #[error("Jev-Antwort verletzt den bestehenden JSON-Vertrag")]
    Schema,
}

impl JevTransportConfig {
    pub fn validate(&self) -> Result<(), TransportError> {
        let url = Url::parse(&self.endpoint).map_err(|_| TransportError::Config)?;
        if url.scheme() != "https"
            || url.host_str().is_none()
            || !url.username().is_empty()
            || url.password().is_some()
            || url.query().is_some()
            || url.fragment().is_some()
            || self.model.is_empty()
            || self.model.chars().any(char::is_control)
            || !(1..=120_000).contains(&self.timeout_ms)
            || !(1..=32 * 1024 * 1024).contains(&self.max_request_bytes)
            || !(1..=1024 * 1024).contains(&self.max_response_bytes)
        {
            return Err(TransportError::Config);
        }
        Ok(())
    }
}

pub struct JevClient {
    config: JevTransportConfig,
    client: Client,
}
impl JevClient {
    pub fn new(config: JevTransportConfig) -> Result<Self, TransportError> {
        config.validate()?;
        let client = Client::builder()
            .https_only(true)
            .no_proxy()
            .redirect(Policy::none())
            .timeout(Duration::from_millis(config.timeout_ms))
            .build()
            .map_err(|_| TransportError::Config)?;
        Ok(Self { config, client })
    }
    /// Credential ausschließlich vom autorisierten Aufrufer im RAM übernehmen.
    pub async fn classify(
        &self,
        request_raw: &str,
        credential: &str,
    ) -> Result<Value, TransportError> {
        if request_raw.len() > self.config.max_request_bytes || credential.is_empty() {
            return Err(TransportError::Request);
        }
        let request = crate::parse_strict(request_raw).map_err(|_| TransportError::Request)?;
        if request.get("model").and_then(Value::as_str) != Some(&self.config.model) {
            return Err(TransportError::Request);
        }
        let mut authorization = HeaderValue::from_str(&format!("Bearer {credential}"))
            .map_err(|_| TransportError::Request)?;
        authorization.set_sensitive(true);
        let mut response = self
            .client
            .post(&self.config.endpoint)
            .header(AUTHORIZATION, authorization)
            .json(&request)
            .send()
            .await
            .map_err(|_| TransportError::Transport)?;
        if !response.status().is_success() {
            return Err(TransportError::Http(response.status().as_u16()));
        }
        if response
            .content_length()
            .is_some_and(|size| size > self.config.max_response_bytes as u64)
        {
            return Err(TransportError::ResponseLimit);
        }
        let mut bytes = Vec::new();
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|_| TransportError::Transport)?
        {
            if bytes.len().saturating_add(chunk.len()) > self.config.max_response_bytes {
                return Err(TransportError::ResponseLimit);
            }
            bytes.extend_from_slice(&chunk);
        }
        let raw = std::str::from_utf8(&bytes).map_err(|_| TransportError::Schema)?;
        crate::validate_json(request_raw, raw).map_err(|_| TransportError::Schema)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn config_rejects_cleartext_urls_and_credentials_in_urls() {
        for endpoint in [
            "http://example.com/x",
            "https://secret@example.com/x",
            "https://example.com/x?key=x",
        ] {
            assert!(JevTransportConfig {
                endpoint: endpoint.into(),
                model: "test".into(),
                timeout_ms: 100,
                max_request_bytes: 100,
                max_response_bytes: 100
            }
            .validate()
            .is_err());
        }
    }
}
