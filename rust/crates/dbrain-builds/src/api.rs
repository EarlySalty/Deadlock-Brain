use std::time::Duration;

use anyhow::{ensure, Context, Result};
use reqwest::blocking::Client;
use serde::de::DeserializeOwned;

const ANALYTICS_BASE_URL: &str = "https://api.deadlock-api.com/v1/analytics";

#[derive(Debug, Clone)]
pub(crate) struct DeadlockApiClient {
    client: Client,
    min_unix_timestamp: i64,
}

impl DeadlockApiClient {
    pub(crate) fn new(user_agent: &str, min_unix_timestamp: i64) -> Result<Self> {
        ensure!(
            min_unix_timestamp > 0,
            "Analytics brauchen einen belegten Patchbeginn"
        );
        let client = Client::builder()
            .user_agent(user_agent)
            .timeout(Duration::from_secs(15))
            .build()
            .context("Deadlock API HTTP-Client konnte nicht erstellt werden")?;
        Ok(Self {
            client,
            min_unix_timestamp,
        })
    }

    pub(crate) fn item_stats(
        &self,
        hero_id: i64,
        min_average_badge: i64,
    ) -> Result<serde_json::Value> {
        self.get_json(&analytics_url(
            "item-stats",
            self.min_unix_timestamp,
            &[
                ("hero_id", hero_id),
                ("min_average_badge", min_average_badge),
            ],
        ))
    }

    pub(crate) fn hero_stats(&self, min_average_badge: i64) -> Result<serde_json::Value> {
        self.get_json(&analytics_url(
            "hero-stats",
            self.min_unix_timestamp,
            &[("min_average_badge", min_average_badge)],
        ))
    }

    pub(crate) fn hero_stats_with_item(
        &self,
        item_id: i64,
        min_average_badge: i64,
    ) -> Result<serde_json::Value> {
        self.get_json(&analytics_url(
            "hero-stats",
            self.min_unix_timestamp,
            &[
                ("include_item_ids", item_id),
                ("min_average_badge", min_average_badge),
            ],
        ))
    }

    pub(crate) fn ability_order_stats(
        &self,
        hero_id: i64,
        min_average_badge: i64,
        min_matches: i64,
    ) -> Result<serde_json::Value> {
        self.get_json(&analytics_url(
            "ability-order-stats",
            self.min_unix_timestamp,
            &[
                ("hero_id", hero_id),
                ("min_average_badge", min_average_badge),
                ("min_matches", min_matches),
            ],
        ))
    }

    pub(crate) fn item_permutation_stats(&self, hero_id: i64) -> Result<serde_json::Value> {
        self.get_json(&analytics_url(
            "item-permutation-stats",
            self.min_unix_timestamp,
            &[("hero_id", hero_id)],
        ))
    }

    fn get_json<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        let response = self
            .client
            .get(url)
            .send()
            .with_context(|| format!("GET {url} fehlgeschlagen"))?;
        let status = response.status();
        let body = response
            .text()
            .with_context(|| format!("GET {url} Body konnte nicht gelesen werden"))?;
        if !status.is_success() {
            let truncated: String = body.chars().take(1000).collect();
            anyhow::bail!("GET {url} lieferte HTTP {status}: {truncated}");
        }
        serde_json::from_str(&body)
            .with_context(|| format!("GET {url} lieferte kein gültiges JSON"))
    }
}

pub fn analytics_url(endpoint: &str, start: i64, params: &[(&str, i64)]) -> String {
    let cutoff = if endpoint == "build-item-stats" {
        "min_last_updated_unix_timestamp"
    } else {
        "min_unix_timestamp"
    };
    let mut url = format!("{ANALYTICS_BASE_URL}/{endpoint}?{cutoff}={start}");
    if endpoint != "build-item-stats" {
        url.push_str("&game_mode=normal&match_mode=ranked");
    }
    for (key, value) in params {
        url.push_str(&format!("&{key}={value}"));
    }
    url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_statistical_path_has_the_correct_time_filter() {
        for endpoint in [
            "item-stats",
            "hero-stats",
            "ability-order-stats",
            "item-permutation-stats",
        ] {
            let url = analytics_url(endpoint, 1791241532, &[("min_average_badge", 80)]);
            assert!(url.contains("min_unix_timestamp=1791241532"));
            assert!(url.contains("game_mode=normal&match_mode=ranked"));
            assert!(!url.contains("hero_ids="));
        }
        let build = analytics_url("build-item-stats", 1791241532, &[("hero_id", 25)]);
        assert!(build.contains("min_last_updated_unix_timestamp=1791241532"));
        assert!(!build.contains("?min_unix_timestamp="));
        assert!(!build.contains("match_mode="));
    }
}
