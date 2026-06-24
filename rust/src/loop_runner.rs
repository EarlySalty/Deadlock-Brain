use rusqlite::Connection;
use serde::Serialize;

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

pub fn run_ingest(conn: &Connection, limit: usize) -> anyhow::Result<IngestSummary> {
    let discover = queue::discover_youtube_videos(conn, &db::default_feed_config_path(), 50)?;
    let videos = queue::select_next_videos(conn, limit)?;
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
                    claims::save_claims(conn, &video, &parsed_claims, &prompt, &response_text)?;
                let status = if saved > 0 {
                    "claims_ready"
                } else {
                    "no_claims"
                };
                queue::mark_success(conn, &video.video_id, status)?;
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
                    conn,
                    &video.video_id,
                    &error.kind.to_string(),
                    &error.message,
                )?;
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
