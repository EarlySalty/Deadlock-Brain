use std::{
    thread,
    time::Duration,
};

use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::de::DeserializeOwned;

// assets.deadlock-api.com ist seit September 2026 NXDOMAIN; die Assets liegen
// jetzt unter der Haupt-API mit derselben Feldstruktur.
const ASSETS_BASE_URL: &str = "https://api.deadlock-api.com/v1/assets";
const ANALYTICS_BASE_URL: &str = "https://api.deadlock-api.com/v1/analytics";
const DEFAULT_RETRY_ATTEMPTS: u32 = 5;
const DEFAULT_RETRY_BASE: Duration = Duration::from_secs(2);

#[derive(Debug, Clone)]
pub(crate) struct DeadlockApiClient {
    client: Client,
    retry_attempts: u32,
    retry_base: Duration,
}

impl DeadlockApiClient {
    pub(crate) fn new(user_agent: &str) -> Result<Self> {
        Self::with_retry(user_agent, DEFAULT_RETRY_ATTEMPTS, DEFAULT_RETRY_BASE)
    }

    fn with_retry(user_agent: &str, retry_attempts: u32, retry_base: Duration) -> Result<Self> {
        let client = Client::builder()
            .user_agent(user_agent)
            .timeout(Duration::from_secs(15))
            .build()
            .context("Deadlock API HTTP-Client konnte nicht erstellt werden")?;
        Ok(Self {
            client,
            retry_attempts: retry_attempts.max(1),
            retry_base,
        })
    }

    pub(crate) fn items(&self) -> Result<serde_json::Value> {
        self.get_json(&format!("{ASSETS_BASE_URL}/items?language=english"))
    }

    pub(crate) fn heroes(&self) -> Result<serde_json::Value> {
        self.get_json(&format!("{ASSETS_BASE_URL}/heroes?only_active=true"))
    }

    pub(crate) fn build_item_stats(&self, hero_id: i64) -> Result<serde_json::Value> {
        self.get_json(&format!(
            "{ANALYTICS_BASE_URL}/build-item-stats?hero_id={hero_id}"
        ))
    }

    pub(crate) fn item_stats(
        &self,
        hero_id: i64,
        min_average_badge: i64,
    ) -> Result<serde_json::Value> {
        self.get_json(&format!(
            "{ANALYTICS_BASE_URL}/item-stats?hero_id={hero_id}&min_average_badge={min_average_badge}"
        ))
    }

    pub(crate) fn hero_stats(
        &self,
        hero_id: i64,
        min_average_badge: i64,
    ) -> Result<serde_json::Value> {
        self.get_json(&format!(
            "{ANALYTICS_BASE_URL}/hero-stats?hero_ids={hero_id}&min_average_badge={min_average_badge}"
        ))
    }

    pub(crate) fn hero_stats_with_item(
        &self,
        hero_id: i64,
        item_id: i64,
        min_average_badge: i64,
    ) -> Result<serde_json::Value> {
        self.get_json(&format!(
            "{ANALYTICS_BASE_URL}/hero-stats?hero_ids={hero_id}&min_average_badge={min_average_badge}&include_item_ids={item_id}"
        ))
    }

    pub(crate) fn ability_order_stats(
        &self,
        hero_id: i64,
        min_average_badge: i64,
        min_matches: i64,
    ) -> Result<serde_json::Value> {
        self.get_json(&format!(
            "{ANALYTICS_BASE_URL}/ability-order-stats?hero_id={hero_id}&min_average_badge={min_average_badge}&min_matches={min_matches}"
        ))
    }

    pub(crate) fn item_permutation_stats(&self, hero_id: i64) -> Result<serde_json::Value> {
        self.get_json(&format!(
            "{ANALYTICS_BASE_URL}/item-permutation-stats?hero_id={hero_id}"
        ))
    }

    fn get_json<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        let mut last_error = None;
        for attempt in 1..=self.retry_attempts {
            match self.get_json_once(url) {
                Ok(value) => return Ok(value),
                Err(error) if attempt < self.retry_attempts && error.retryable => {
                    last_error = Some(error.source);
                    thread::sleep(self.retry_base.saturating_mul(attempt));
                }
                Err(error) => return Err(error.source),
            }
        }
        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("GET {url} fehlgeschlagen")))
    }

    fn get_json_once<T: DeserializeOwned>(&self, url: &str) -> std::result::Result<T, FetchError> {
        let response = self.client.get(url).send().map_err(|error| FetchError {
            retryable: is_retryable_transport_error(&error, false),
            source: anyhow::Error::new(error).context(format!("GET {url} fehlgeschlagen")),
        })?;
        let status = response.status();
        let retryable_status =
            status.as_u16() == 408 || status.as_u16() == 429 || status.is_server_error();
        let body = response.text().map_err(|error| FetchError {
            retryable: is_retryable_transport_error(&error, true),
            source: anyhow::Error::new(error)
                .context(format!("GET {url} Body konnte nicht gelesen werden")),
        })?;
        if !status.is_success() {
            let truncated: String = body.chars().take(1000).collect();
            return Err(FetchError {
                retryable: retryable_status,
                source: anyhow::anyhow!("GET {url} lieferte HTTP {status}: {truncated}"),
            });
        }
        serde_json::from_str(&body).map_err(|error| FetchError {
            retryable: false,
            source: anyhow::Error::new(error)
                .context(format!("GET {url} lieferte kein gueltiges JSON")),
        })
    }
}

fn is_retryable_transport_error(error: &reqwest::Error, reading_body: bool) -> bool {
    error.is_timeout() || error.is_connect() || (reading_body && error.is_body())
}

struct FetchError {
    retryable: bool,
    source: anyhow::Error,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io::{Read, Write},
        net::TcpListener,
        sync::{
            atomic::{AtomicU32, Ordering},
            Arc,
        },
    };

    fn spawn_flaky_json_server(
        failures_before_ok: u32,
        fail_status: &str,
        max_requests: u32,
    ) -> (String, Arc<AtomicU32>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/items", listener.local_addr().unwrap());
        let hits = Arc::new(AtomicU32::new(0));
        let hits_for_thread = hits.clone();
        let fail_status = fail_status.to_string();
        thread::spawn(move || {
            for hit in 1..=max_requests {
                let (mut stream, _) = listener.accept().unwrap();
                let mut request = [0_u8; 1024];
                let _ = stream.read(&mut request);
                hits_for_thread.fetch_add(1, Ordering::SeqCst);
                if hit <= failures_before_ok {
                    let body = b"temporarily unavailable";
                    write!(
                        stream,
                        "HTTP/1.1 {fail_status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                        body.len()
                    )
                    .unwrap();
                    stream.write_all(body).unwrap();
                } else {
                    let body = br#"{"ok":true}"#;
                    write!(
                        stream,
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                        body.len()
                    )
                    .unwrap();
                    stream.write_all(body).unwrap();
                }
            }
        });
        (url, hits)
    }

    #[test]
    fn retries_connection_errors() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        drop(listener);
        let error = Client::new()
            .get(format!("http://{address}/items"))
            .send()
            .unwrap_err();
        assert!(is_retryable_transport_error(&error, false));
    }

    #[test]
    fn does_not_retry_invalid_request_errors() {
        let error = Client::new().get("not a URL").send().unwrap_err();
        assert!(!is_retryable_transport_error(&error, false));
    }

    #[test]
    fn retries_server_errors_then_returns_json() {
        let (url, hits) = spawn_flaky_json_server(2, "503 Service Unavailable", 3);
        let client = DeadlockApiClient::with_retry("deadlock-brain-test", 4, Duration::ZERO)
                .unwrap();
        let value: serde_json::Value = client.get_json(&url).unwrap();
        assert_eq!(value["ok"], true);
        assert_eq!(hits.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn retries_request_timeout_then_returns_json() {
        let (url, hits) = spawn_flaky_json_server(1, "408 Request Timeout", 2);
        let client = DeadlockApiClient::with_retry("deadlock-brain-test", 3, Duration::ZERO)
            .unwrap();
        let value: serde_json::Value = client.get_json(&url).unwrap();
        assert_eq!(value["ok"], true);
        assert_eq!(hits.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn does_not_retry_client_errors() {
        let (url, hits) = spawn_flaky_json_server(3, "404 Not Found", 1);
        let client = DeadlockApiClient::with_retry("deadlock-brain-test", 4, Duration::ZERO)
            .unwrap();
        let error = client.get_json::<serde_json::Value>(&url).unwrap_err();
        assert!(error.to_string().contains("HTTP 404"));
        assert_eq!(hits.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn stops_after_retry_attempt_limit() {
        let (url, hits) = spawn_flaky_json_server(3, "503 Service Unavailable", 3);
        let client = DeadlockApiClient::with_retry("deadlock-brain-test", 3, Duration::ZERO)
            .unwrap();
        let error = client.get_json::<serde_json::Value>(&url).unwrap_err();
        assert!(error.to_string().contains("HTTP 503"));
        assert_eq!(hits.load(Ordering::SeqCst), 3);
    }
}
