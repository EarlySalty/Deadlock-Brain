use serde::Serialize;
use sqlx::postgres::PgPool;

use crate::{claims, db, gemini, queue};

#[derive(Debug, Serialize)]
pub struct IngestSummary {
    pub discover: queue::DiscoverSummary,
    pub selected: usize,
    pub processed: usize,
    pub claims_saved: usize,
    pub videos: Vec<VideoRunSummary>,
}

#[derive(Debug, Serialize)]
pub struct VideoRunSummary {
    pub video_id: String,
    pub title: String,
    pub status: String,
    pub claims_saved: usize,
    pub error_kind: Option<String>,
}

pub async fn run_ingest(pool: &PgPool, limit: usize) -> anyhow::Result<IngestSummary> {
    let discover =
        queue::discover_youtube_videos(pool, &db::default_feed_config_path(), 50).await?;
    run_ingest_after_discover(pool, limit, discover).await
}

pub async fn run_ingest_after_discover(
    pool: &PgPool,
    limit: usize,
    discover: queue::DiscoverSummary,
) -> anyhow::Result<IngestSummary> {
    let videos = queue::select_next_videos(pool, limit).await?;
    let mut summary = IngestSummary {
        discover,
        selected: videos.len(),
        processed: 0,
        claims_saved: 0,
        videos: Vec::new(),
    };

    for video in videos {
        let prompt = claims::build_prompt(&video.url);
        match gemini::analyze_url(&video.url, &prompt) {
            Ok(response_text) => {
                let parsed_claims = claims::parse_model_claims(&response_text);
                let saved =
                    claims::save_claims(pool, &video, &parsed_claims, &prompt, &response_text)
                        .await?;
                let status = if saved > 0 {
                    "claims_ready"
                } else {
                    "no_claims"
                };
                queue::mark_success(pool, &video.video_id, status).await?;
                summary.processed += 1;
                summary.claims_saved += saved;
                summary.videos.push(VideoRunSummary {
                    video_id: video.video_id,
                    title: video.title,
                    status: status.to_string(),
                    claims_saved: saved,
                    error_kind: None,
                });
            }
            Err(error) if error.kind.pauses_run() => {
                gemini::send_pause_alert(error.kind, &error.message);
                anyhow::bail!("gemini ingest paused on {}: {}", video.video_id, error);
            }
            Err(error) => {
                queue::mark_failed(
                    pool,
                    &video.video_id,
                    &error.kind.to_string(),
                    &error.message,
                )
                .await?;
                summary.processed += 1;
                summary.videos.push(VideoRunSummary {
                    video_id: video.video_id,
                    title: video.title,
                    status: "failed".to_string(),
                    claims_saved: 0,
                    error_kind: Some(error.kind.to_string()),
                });
            }
        }
    }

    Ok(summary)
}
