use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use anyhow::{anyhow, Result};
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde_json::Value;

pub const API_BASE: &str = "https://api.deadlock-api.com";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36";
const API_KEY_ENV: &str = "DEADLOCK_API_KEY";
const API_KEY_HEADER: &str = "X-API-KEY";

const MATCHES_BUCKET: &str = "matches";
const ASSETS_BUCKET: &str = "assets";
const MATCHES_RPM_ANON: f64 = 9.0;
const MATCHES_RPM_KEYED: f64 = 50.0;
const ASSETS_RPM: f64 = 30.0;
const MAX_RETRIES: usize = 5;

pub struct ApiClient {
    client: Client,
    has_key: bool,
    limiter: Mutex<HashMap<String, Instant>>,
}

impl ApiClient {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .user_agent(USER_AGENT)
            .gzip(true)
            .timeout(Duration::from_secs(300))
            .build()
            .map_err(|error| anyhow!("HTTP-Client konnte nicht gebaut werden: {error}"))?;
        let has_key = std::env::var(API_KEY_ENV)
            .map(|value| !value.trim().is_empty())
            .unwrap_or(false);
        Ok(Self {
            client,
            has_key,
            limiter: Mutex::new(HashMap::new()),
        })
    }

    pub fn fetch_metadata_page(
        &self,
        cursor: Option<i64>,
        limit: usize,
        min_unix_timestamp: i64,
        hero_ids: &[i64],
    ) -> Result<Vec<Value>> {
        let params = metadata_params(cursor, limit, min_unix_timestamp, hero_ids);
        let value = self.get_json(MATCHES_BUCKET, "/v1/matches/metadata", &params)?;
        Ok(value.as_array().cloned().unwrap_or_default())
    }

    pub fn fetch_assets_items(&self) -> Result<Value> {
        self.get_json(ASSETS_BUCKET, "/v1/assets/items", &[])
    }

    pub fn fetch_assets_heroes(&self) -> Result<Value> {
        self.get_json(ASSETS_BUCKET, "/v1/assets/heroes", &[])
    }

    fn requests_per_minute(&self, bucket: &str) -> f64 {
        match bucket {
            MATCHES_BUCKET => {
                if self.has_key {
                    MATCHES_RPM_KEYED
                } else {
                    MATCHES_RPM_ANON
                }
            }
            _ => ASSETS_RPM,
        }
    }

    fn pace(&self, bucket: &str) {
        let interval = Duration::from_secs_f64(60.0 / self.requests_per_minute(bucket));
        let wait = {
            let mut guard = self.limiter.lock().unwrap();
            let now = Instant::now();
            let wait = match guard.get(bucket) {
                Some(last) => interval.checked_sub(now.duration_since(*last)),
                None => None,
            };
            guard.insert(bucket.to_string(), now + wait.unwrap_or(Duration::ZERO));
            wait
        };
        if let Some(wait) = wait {
            std::thread::sleep(wait);
        }
    }

    fn get_json(&self, bucket: &str, path: &str, params: &[(String, String)]) -> Result<Value> {
        let url = format!("{API_BASE}{path}");
        let mut attempt = 0usize;
        loop {
            self.pace(bucket);
            let mut request = self
                .client
                .get(&url)
                .query(params)
                .header("Accept", "application/json");
            if self.has_key {
                if let Ok(key) = std::env::var(API_KEY_ENV) {
                    request = request.header(API_KEY_HEADER, key);
                }
            }
            let response = match request.send() {
                Ok(response) => response,
                Err(error) => {
                    attempt += 1;
                    if attempt >= MAX_RETRIES {
                        return Err(anyhow!("Anfrage an {path} scheiterte am Netzwerk."));
                    }
                    std::thread::sleep(backoff(attempt));
                    let _ = error;
                    continue;
                }
            };
            let status = response.status();
            if status == StatusCode::TOO_MANY_REQUESTS {
                attempt += 1;
                let header_hint = retry_after(&response);
                let body_hint = response.text().ok().and_then(|body| next_request_in(&body));
                if attempt >= MAX_RETRIES {
                    return Err(anyhow!("{path} bleibt nach mehreren Versuchen gedrosselt."));
                }
                let wait = header_hint
                    .or(body_hint)
                    .unwrap_or_else(|| backoff(attempt));
                std::thread::sleep(wait);
                continue;
            }
            if status == StatusCode::UNAUTHORIZED || status == StatusCode::FORBIDDEN {
                return Err(anyhow!(
                    "{path} lehnt den Zugriff ab ({}); User-Agent oder Schlüssel prüfen.",
                    status.as_u16()
                ));
            }
            if !status.is_success() {
                attempt += 1;
                if attempt >= MAX_RETRIES {
                    return Err(anyhow!("{path} antwortete mit Status {}.", status.as_u16()));
                }
                std::thread::sleep(backoff(attempt));
                continue;
            }
            let body = response
                .text()
                .map_err(|_| anyhow!("Antwort von {path} war nicht lesbar."))?;
            return serde_json::from_str(&body)
                .map_err(|_| anyhow!("Antwort von {path} war kein gültiges JSON."));
        }
    }
}

fn metadata_params(
    cursor: Option<i64>,
    limit: usize,
    min_unix_timestamp: i64,
    hero_ids: &[i64],
) -> Vec<(String, String)> {
    let mut params: Vec<(String, String)> = vec![
        ("match_mode".into(), "Ranked".into()),
        ("game_mode".into(), "normal".into()),
        ("include_player_items".into(), "true".into()),
        ("include_player_stats".into(), "true".into()),
        ("include_player_info".into(), "true".into()),
        ("include_objectives".into(), "true".into()),
        ("include_mid_boss".into(), "true".into()),
        ("order_by".into(), "match_id".into()),
        ("order_direction".into(), "desc".into()),
        ("limit".into(), limit.to_string()),
        ("min_unix_timestamp".into(), min_unix_timestamp.to_string()),
    ];
    if let Some(cursor) = cursor {
        params.push(("max_match_id".into(), cursor.to_string()));
    }
    if !hero_ids.is_empty() {
        let joined = hero_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        params.push(("hero_ids".into(), joined));
    }
    params
}

fn backoff(attempt: usize) -> Duration {
    let seconds = (2f64.powi(attempt as i32)).min(60.0);
    Duration::from_secs_f64(seconds)
}

fn retry_after(response: &reqwest::blocking::Response) -> Option<Duration> {
    response
        .headers()
        .get("Retry-After")
        .and_then(|value| value.to_str().ok())
        .and_then(|text| text.parse::<f64>().ok())
        .map(|seconds| Duration::from_secs_f64(seconds + 1.0))
}

fn next_request_in(body: &str) -> Option<Duration> {
    let value: Value = serde_json::from_str(body).ok()?;
    let seconds = value
        .get("error")
        .and_then(|error| error.get("quota"))
        .and_then(|quota| quota.get("next_request_in"))
        .and_then(Value::as_f64)?;
    Some(Duration::from_secs_f64(seconds + 1.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn value_of<'a>(params: &'a [(String, String)], key: &str) -> Option<&'a str> {
        params
            .iter()
            .find(|(name, _)| name == key)
            .map(|(_, value)| value.as_str())
    }

    #[test]
    fn hero_filter_reaches_the_request_as_hero_ids() {
        let params = metadata_params(Some(100), 200, 42, &[8, 15]);
        assert_eq!(value_of(&params, "hero_ids"), Some("8,15"));
        assert_eq!(value_of(&params, "max_match_id"), Some("100"));
        assert_eq!(value_of(&params, "match_mode"), Some("Ranked"));

        let without = metadata_params(None, 200, 42, &[]);
        assert_eq!(value_of(&without, "hero_ids"), None);
        assert_eq!(value_of(&without, "max_match_id"), None);
    }
}
