use serde::Serialize;
use sqlx::postgres::PgPool;

use crate::{claims, db, queue, transcripts};

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
    let transcript_limit = limit.max(1).saturating_mul(4).max(20);
    let _ = transcripts::fetch_transcripts(pool, transcript_limit).await?;
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
        let url = video.url.clone();
        let transcript = video.transcript_text.clone();
        let analysis = tokio::task::spawn_blocking(move || {
            claims::analyze_transcript(&url, &transcript)
        })
        .await;

        match analysis {
            Ok(Ok((prompt, response_text))) => {
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
            Ok(Err(error)) => {
                queue::mark_failed(pool, &video.video_id, "analysis", &error.to_string()).await?;
                summary.processed += 1;
                summary.videos.push(VideoRunSummary {
                    video_id: video.video_id,
                    title: video.title,
                    status: "failed".to_string(),
                    claims_saved: 0,
                    error_kind: Some("analysis".to_string()),
                });
            }
            Err(_) => {
                queue::mark_failed(
                    pool,
                    &video.video_id,
                    "worker_join",
                    "Transcript Analyse Worker wurde abgebrochen.",
                )
                .await?;
                summary.processed += 1;
                summary.videos.push(VideoRunSummary {
                    video_id: video.video_id,
                    title: video.title,
                    status: "failed".to_string(),
                    claims_saved: 0,
                    error_kind: Some("worker_join".to_string()),
                });
            }
        }
    }

    Ok(summary)
}
