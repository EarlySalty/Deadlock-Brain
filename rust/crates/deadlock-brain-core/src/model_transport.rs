//! Bounded provider transport. Errors contain no response bodies, URLs or credentials.
use std::{collections::HashSet, io::Read, thread, time::{Duration, Instant}};

use reqwest::{blocking::Client, Method, StatusCode};
use serde_json::Value;

use super::{catalog, probes, Candidate, Network, Policy, Result, SelectionError};

const MAX_RESPONSE_BYTES: u64 = 4 * 1024 * 1024;

pub(super) struct FireworksNetwork {
    client: Client,
    credential: String,
    base_url: String,
    catalog_url: String,
    attempts: usize,
    backoff: Duration,
    request_timeout: Duration,
}

impl FireworksNetwork {
    pub fn new(policy: &Policy, credential: &str) -> Result<Self> {
        let selection = &policy.config.ai().selection;
        let client = Client::builder().no_proxy()
            .redirect(reqwest::redirect::Policy::none())
            .user_agent("DeadlockBrain/model-verification")
            .timeout(Duration::from_secs(selection.request_timeout_seconds))
            .build().map_err(|_| SelectionError::Transport)?;
        Ok(Self { client, credential: credential.to_owned(),
            base_url: policy.config.ai().base_url.trim_end_matches('/').to_owned(),
            catalog_url: selection.catalog_url.clone(),
            attempts: selection.retry_attempts,
            backoff: Duration::from_millis(selection.retry_backoff_milliseconds),
            request_timeout: Duration::from_secs(selection.request_timeout_seconds),
        })
    }

    fn request(&self, method: Method, url: &str, query: &[(String,String)], payload: Option<&Value>, deadline: Instant) -> Result<Value> {
        let mut last_error = SelectionError::Transport;
        for attempt in 0..self.attempts {
            let remaining = deadline.checked_duration_since(Instant::now()).ok_or(SelectionError::Timeout)?;
            let mut request = self.client.request(method.clone(), url)
                .bearer_auth(&self.credential).query(query).timeout(self.request_timeout.min(remaining));
            if let Some(body) = payload { request = request.json(body); }
            let response = request.send();
            let mut wait = self.backoff.saturating_mul((attempt + 1) as u32);
            match response {
                Ok(response) => {
                    let status = response.status();
                    if status.is_success() {
                        let mut bytes = Vec::new();
                        response.take(MAX_RESPONSE_BYTES + 1).read_to_end(&mut bytes)
                            .map_err(|_| if Instant::now() >= deadline { SelectionError::Timeout } else { SelectionError::Transport })?;
                        if Instant::now() >= deadline { return Err(SelectionError::Timeout); }
                        if bytes.len() as u64 > MAX_RESPONSE_BYTES { return Err(SelectionError::CatalogInvalid); }
                        return serde_json::from_slice(&bytes).map_err(|_| SelectionError::CatalogInvalid);
                    }
                    last_error = classify(status);
                    if status != StatusCode::TOO_MANY_REQUESTS && !status.is_server_error() {
                        return Err(last_error);
                    }
                    if let Some(seconds) = response.headers().get(reqwest::header::RETRY_AFTER)
                        .and_then(|s| s.to_str().ok()).and_then(|s| s.parse::<u64>().ok())
                    {
                        wait = wait.max(Duration::from_secs(seconds.min(60)));
                    }
                }
                Err(error) => {
                    last_error = if error.is_timeout() { SelectionError::Timeout } else { SelectionError::Transport };
                }
            }
            if attempt + 1 < self.attempts {
                let remaining = deadline.checked_duration_since(Instant::now()).ok_or(SelectionError::Timeout)?;
                if wait >= remaining { return Err(SelectionError::Timeout); }
                thread::sleep(wait);
            }
        }
        Err(last_error)
    }
}

impl Network for FireworksNetwork {
    fn catalog(&mut self, policy: &Policy, now: u64) -> Result<Vec<Candidate>> {
        let selection = &policy.config.ai().selection;
        let deadline = Instant::now() + Duration::from_secs(selection.catalog_budget_seconds);
        let mut page_token: Option<String> = None;
        let mut seen_tokens = HashSet::new();
        let mut seen_models = HashSet::new();
        let mut candidates = Vec::new();
        for _ in 0..selection.max_pages {
            // No filter/order mutation across pages; API max page size is 200.
            let mut query = vec![("pageSize".to_owned(), selection.page_size.to_string())];
            if let Some(token) = &page_token { query.push(("pageToken".to_owned(), token.clone())); }
            let body = self.request(Method::GET, &self.catalog_url, &query, None, deadline)?;
            let (entries, next) = catalog::parse_page(&body, policy, now)?;
            for entry in entries {
                if !seen_models.insert(entry.model.clone()) { return Err(SelectionError::CatalogInvalid); }
                candidates.push(entry);
            }
            match next {
                None => return Ok(candidates),
                Some(token) if seen_tokens.insert(token.clone()) => page_token = Some(token),
                Some(_) => return Err(SelectionError::CatalogInvalid),
            }
        }
        // Never activate a candidate from an incomplete catalog traversal.
        Err(SelectionError::CatalogInvalid)
    }

    fn probe(&mut self, policy: &Policy, model: &str) -> Result<()> {
        let deadline = Instant::now() + Duration::from_secs(policy.config.ai().selection.probe_budget_seconds);
        let url = format!("{}/chat/completions", self.base_url);
        probes::run(model, policy.config.ai().max_completion_tokens, |payload| {
            self.request(Method::POST, &url, &[], Some(payload), deadline)
        })?;
        if Instant::now() >= deadline { return Err(SelectionError::Timeout); }
        Ok(())
    }
}

fn classify(status: StatusCode) -> SelectionError {
    match status {
        StatusCode::UNAUTHORIZED => SelectionError::Unauthorized,
        StatusCode::FORBIDDEN => SelectionError::Forbidden,
        StatusCode::TOO_MANY_REQUESTS => SelectionError::RateLimited,
        status if status.is_server_error() => SelectionError::ProviderUnavailable,
        _ => SelectionError::Transport,
    }
}

#[cfg(test)]
#[path = "model_transport_tests.rs"]
mod tests;
