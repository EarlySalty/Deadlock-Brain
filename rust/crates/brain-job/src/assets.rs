pub mod extraction;
pub mod storage;

use crate::{config::Config, store::Counts};
use anyhow::{ensure, Result};
use deadlock_brain_core::http::{HttpClient, SourceHttpOptions};
use serde::Deserialize;
use std::{path::PathBuf, sync::Arc};

pub const EXTRACTOR: &str = "brain-assets";
pub const VERSION: &str = "p2-v1";
pub const MAX_BYTES: usize = 8 * 1024 * 1024;
pub const BASE_URL: &str = "https://api.deadlock-api.com";

#[derive(Deserialize)]
struct Manifest {
    client_version: u64,
}

async fn client(state: PathBuf) -> Result<Arc<HttpClient>> {
    tokio::task::spawn_blocking(move || {
        Ok(Arc::new(HttpClient::new("Deadlock-Brain-Assets/1", state)?))
    })
    .await?
}

async fn fetch(http: Arc<HttpClient>, url: String, max_bytes: usize) -> Result<Vec<u8>> {
    tokio::task::spawn_blocking(move || {
        let response = http.get_bounded(
            &url,
            SourceHttpOptions {
                max_bytes,
                ..Default::default()
            },
        )?;
        response.ensure_success()?;
        ensure!(
            response.content_type().split(';').next() == Some("application/json")
                && response
                    .headers
                    .get("content-encoding")
                    .is_none_or(|encoding| encoding == "identity"),
            "assets_media_type"
        );
        Ok(response.content)
    })
    .await?
}

pub async fn ingest(config: &Config) -> Result<Counts> {
    let store = storage::Store::start(config.pool().await?, config.lease_seconds).await?;
    let mut counts = Counts::default();
    let mut unmapped = 0;
    let result = async {
        let http = store.during(client(config.state()?)).await?;
        let manifest = store
            .during(fetch(
                http.clone(),
                format!("{BASE_URL}/v1/assets/steam-info"),
                64 * 1024,
            ))
            .await?;
        let manifest: Manifest = serde_json::from_slice(&manifest)?;
        ensure!(manifest.client_version > 0, "assets_client_version");
        for (language, code) in [("english", "en"), ("german", "de")] {
            let locator = format!("{BASE_URL}/v1/assets/items?language={language}");
            let url = format!("{locator}&client_version={}", manifest.client_version);
            let bytes = store.during(fetch(http.clone(), url, MAX_BYTES)).await?;
            unmapped += store.object(&locator, code, &bytes, &mut counts).await?;
        }
        Ok::<(), anyhow::Error>(())
    }
    .await;
    store.finish(&counts, unmapped, result.is_err()).await?;
    result?;
    Ok(counts)
}
