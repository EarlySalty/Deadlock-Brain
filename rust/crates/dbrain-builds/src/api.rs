use std::time::Duration;

use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::de::DeserializeOwned;

const ASSETS_BASE_URL: &str = "https://assets.deadlock-api.com/v2";
const ANALYTICS_BASE_URL: &str = "https://api.deadlock-api.com/v1/analytics";

#[derive(Debug, Clone)]
pub(crate) struct DeadlockApiClient {
    client: Client,
}

impl DeadlockApiClient {
    pub(crate) fn new(user_agent: &str) -> Result<Self> {
        let client = Client::builder()
            .user_agent(user_agent)
            .timeout(Duration::from_secs(15))
            .build()
            .context("Deadlock API HTTP-Client konnte nicht erstellt werden")?;
        Ok(Self { client })
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
            .with_context(|| format!("GET {url} lieferte kein gueltiges JSON"))
    }
}
