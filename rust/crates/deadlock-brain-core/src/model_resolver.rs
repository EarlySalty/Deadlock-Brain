use std::sync::{Mutex, OnceLock};

use reqwest::blocking::Client;
use serde_json::Value;

use crate::config::DEFAULT_FIREWORKS_MODEL;

static RESOLVED_MODEL: OnceLock<Mutex<Option<String>>> = OnceLock::new();

fn cache() -> &'static Mutex<Option<String>> {
    RESOLVED_MODEL.get_or_init(|| Mutex::new(None))
}

pub fn resolved_model() -> Option<String> {
    cache()
        .lock()
        .ok()
        .and_then(|model| model.clone().filter(|value| !value.is_empty()))
}

pub fn explicit_model() -> Option<String> {
    ["FIREWORKS_MODEL", "FIREWORK_MODEL"]
        .into_iter()
        .find_map(|name| {
            std::env::var(name)
                .ok()
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty())
        })
}

pub fn model_for_request(configured: &str) -> String {
    explicit_model().or_else(resolved_model).unwrap_or_else(|| {
        if configured.trim().is_empty() {
            DEFAULT_FIREWORKS_MODEL.to_string()
        } else {
            configured.to_string()
        }
    })
}

pub fn resolve_after_not_found(base_url: &str, api_key: &str) -> Option<String> {
    let mut guard = cache().lock().ok()?;
    if let Some(model) = guard.as_ref().filter(|model| !model.is_empty()) {
        return Some(model.clone());
    }
    let model = Client::builder()
        .build()
        .ok()
        .and_then(|client| {
            client
                .get(format!("{}/models", base_url.trim_end_matches('/')))
                .bearer_auth(api_key)
                .send()
                .ok()
        })
        .filter(|response| response.status().is_success())
        .and_then(|response| response.json::<Value>().ok())
        .and_then(|body| {
            body.get("data")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(|entry| entry.get("id").and_then(Value::as_str))
                .filter(|id| is_family_model(id))
                .max()
                .map(str::to_string)
        });
    *guard = Some(model.clone().unwrap_or_default());
    model
}

fn is_family_model(model: &str) -> bool {
    let Some(suffix) = model.strip_prefix(DEFAULT_FIREWORKS_MODEL) else {
        return false;
    };
    suffix.is_empty()
        || suffix
            .strip_prefix('-')
            .is_some_and(|value| !value.is_empty() && value.chars().all(|c| c.is_ascii_digit()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selects_latest_numeric_family_revision() {
        assert_eq!(
            [
                "accounts/fireworks/models/deepseek-v4-flash-0901",
                "accounts/fireworks/models/deepseek-v4-flash-1015",
                "accounts/fireworks/models/deepseek-v4-flash-lite"
            ]
            .into_iter()
            .filter(|model| is_family_model(model))
            .max(),
            Some("accounts/fireworks/models/deepseek-v4-flash-1015")
        );
    }

    #[test]
    fn ignores_non_family_variants() {
        assert!(!is_family_model(
            "accounts/fireworks/models/deepseek-v4-flash-preview"
        ));
    }
}
