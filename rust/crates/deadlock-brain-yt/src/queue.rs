use std::{fmt, path::Path, thread, time::Duration};

use regex::Regex;
use reqwest::{blocking::Client, StatusCode};
use roxmltree::Node;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::postgres::PgPool;

use crate::{claims, db};

#[derive(Debug, Clone, Serialize)]
pub struct Video {
    pub video_id: String,
    pub title: String,
    pub url: String,
    pub channel_title: Option<String>,
    pub published_at: Option<String>,
    pub learning_status: String,
}

#[derive(Debug, Serialize)]
pub struct DiscoverSummary {
    pub feeds: usize,
    pub videos_seen: usize,
    pub inserted: usize,
    pub updated: usize,
    pub errors: Vec<DiscoverError>,
}

#[derive(Debug, Serialize)]
pub struct DiscoverError {
    pub feed: String,
    pub error: String,
}

#[derive(Debug, Clone)]
struct Feed {
    source_type: String,
    url: String,
    handle: Option<String>,
    playlist_id: Option<String>,
    channel_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FeedConfig {
    feeds: Vec<Value>,
}

#[derive(Debug)]
struct AtomVideo {
    video_id: String,
    feed_key: String,
    channel_id: Option<String>,
    channel_title: Option<String>,
    title: String,
    url: String,
    published_at: Option<String>,
    description: Option<String>,
}

const FEED_FETCH_ATTEMPTS: usize = 3;
const FEED_FETCH_BACKOFF_MS: u64 = 600;

#[derive(Debug)]
struct FetchFailure {
    url: String,
    attempts: usize,
    status: Option<StatusCode>,
    message: String,
}

impl FetchFailure {
    fn is_server_error(&self) -> bool {
        self.status
            .map(|status| status.is_server_error())
            .unwrap_or(false)
    }
}

impl fmt::Display for FetchFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.status {
            Some(status) => write!(
                formatter,
                "fetch {} failed after {} attempts: HTTP {} ({})",
                self.url, self.attempts, status, self.message
            ),
            None => write!(
                formatter,
                "fetch {} failed after {} attempts: {}",
                self.url, self.attempts, self.message
            ),
        }
    }
}

impl std::error::Error for FetchFailure {}

pub async fn discover_youtube_videos(
    pool: &PgPool,
    config_path: &Path,
    max_videos_per_feed: usize,
) -> anyhow::Result<DiscoverSummary> {
    let feeds = load_feed_config(config_path)?;
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .user_agent("deadlock-brain-yt/0.1")
        .build()?;
    let mut summary = DiscoverSummary {
        feeds: feeds.len(),
        videos_seen: 0,
        inserted: 0,
        updated: 0,
        errors: Vec::new(),
    };

    for feed in feeds {
        let feed_key = feed_key(&feed);
        if let Err(error) = discover_feed(
            pool,
            &client,
            &feed,
            &feed_key,
            max_videos_per_feed,
            &mut summary,
        )
        .await
        {
            summary.errors.push(DiscoverError {
                feed: feed_key,
                error: error.to_string(),
            });
        }
    }

    Ok(summary)
}

async fn discover_feed(
    pool: &PgPool,
    client: &Client,
    feed: &Feed,
    feed_key: &str,
    max_videos_per_feed: usize,
    summary: &mut DiscoverSummary,
) -> anyhow::Result<()> {
    upsert_feed_source(pool, feed_key, feed).await?;
    let (rss_url, xml) = if feed.source_type == "playlist" {
        let playlist_id = feed
            .playlist_id
            .as_deref()
            .filter(|value| !value.is_empty())
            .ok_or_else(|| anyhow::anyhow!("missing playlist_id"))?;
        let rss_url = playlist_rss_url(playlist_id);
        let xml = fetch_text_with_retries(client, &rss_url)?;
        (rss_url, xml)
    } else {
        let stored = match &feed.channel_id {
            Some(id) => Some(id.clone()),
            None => stored_channel_id(pool, feed_key).await.ok().flatten(),
        };
        let mut channel_id = match stored {
            Some(id) => id,
            None => resolve_channel_id(client, feed)?,
        };
        set_feed_channel_id(pool, feed_key, &channel_id).await?;
        let mut rss_url = channel_rss_url(&channel_id);
        match fetch_text_with_retries(client, &rss_url) {
            Ok(xml) => (rss_url, xml),
            Err(error) if error.is_server_error() => match resolve_channel_id(client, feed) {
                Ok(refreshed_channel_id) if refreshed_channel_id != channel_id => {
                    channel_id = refreshed_channel_id;
                    set_feed_channel_id(pool, feed_key, &channel_id).await?;
                    rss_url = channel_rss_url(&channel_id);
                    let xml = fetch_text_with_retries(client, &rss_url)?;
                    (rss_url, xml)
                }
                Ok(_) => return Err(error.into()),
                Err(resolve_error) => {
                    anyhow::bail!("{error}; channel_id refresh failed: {resolve_error}")
                }
            },
            Err(error) => return Err(error.into()),
        }
    };

    let videos = parse_atom_videos(&xml, feed_key)?;
    for video in videos.into_iter().take(max_videos_per_feed.max(1)) {
        match upsert_video(pool, &video, &rss_url).await? {
            UpsertAction::Inserted => summary.inserted += 1,
            UpsertAction::Updated => summary.updated += 1,
        }
        summary.videos_seen += 1;
    }
    Ok(())
}

fn playlist_rss_url(playlist_id: &str) -> String {
    format!(
        "https://www.youtube.com/feeds/videos.xml?playlist_id={}",
        urlencoding::encode(playlist_id)
    )
}

fn channel_rss_url(channel_id: &str) -> String {
    format!(
        "https://www.youtube.com/feeds/videos.xml?channel_id={}",
        urlencoding::encode(channel_id)
    )
}

fn fetch_text_with_retries(client: &Client, url: &str) -> Result<String, FetchFailure> {
    let mut attempts_used = 0;
    let mut last_status = None;
    let mut last_message = String::new();

    for attempt in 1..=FEED_FETCH_ATTEMPTS {
        attempts_used = attempt;
        let should_retry = match client.get(url).send() {
            Ok(response) => {
                let status = response.status();
                last_status = Some(status);
                if status.is_success() {
                    match response.text() {
                        Ok(text) => return Ok(text),
                        Err(error) => {
                            last_message = error.to_string();
                            true
                        }
                    }
                } else {
                    last_message = format!("HTTP {status}");
                    status.is_server_error()
                }
            }
            Err(error) => {
                last_status = None;
                last_message = error.to_string();
                true
            }
        };

        if !should_retry || attempt == FEED_FETCH_ATTEMPTS {
            break;
        }
        thread::sleep(Duration::from_millis(FEED_FETCH_BACKOFF_MS * attempt as u64));
    }

    Err(FetchFailure {
        url: url.to_string(),
        attempts: attempts_used,
        status: last_status,
        message: last_message,
    })
}

fn load_feed_config(path: &Path) -> anyhow::Result<Vec<Feed>> {
    let raw = std::fs::read_to_string(path)?;
    let value: Value = serde_json::from_str(&raw)?;
    let items = if let Ok(config) = serde_json::from_value::<FeedConfig>(value.clone()) {
        config.feeds
    } else if let Some(list) = value.as_array() {
        list.clone()
    } else {
        anyhow::bail!("invalid feed config");
    };

    let mut feeds = Vec::new();
    for item in items {
        if let Some(feed) = normalize_feed(&item) {
            feeds.push(feed);
        }
    }
    Ok(dedupe_feeds(feeds))
}

fn normalize_feed(item: &Value) -> Option<Feed> {
    let url = item.get("url")?.as_str()?.trim().to_string();
    let handle = item
        .get("handle")
        .and_then(Value::as_str)
        .map(|value| value.trim().trim_start_matches('@').to_string())
        .filter(|value| !value.is_empty())
        .or_else(|| handle_from_url(&url));
    let playlist_id = item
        .get("playlist_id")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .or_else(|| query_param(&url, "list"));
    let source_type = item
        .get("type")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(if playlist_id.is_some() {
            "playlist"
        } else {
            "channel"
        })
        .to_string();
    let channel_id = item
        .get("channel_id")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned);
    Some(Feed {
        source_type,
        url,
        handle,
        playlist_id,
        channel_id,
    })
}

fn dedupe_feeds(feeds: Vec<Feed>) -> Vec<Feed> {
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();
    for feed in feeds {
        if seen.insert(feed_key(&feed)) {
            result.push(feed);
        }
    }
    result
}

fn feed_key(feed: &Feed) -> String {
    if feed.source_type == "playlist" {
        return format!(
            "playlist:{}",
            feed.playlist_id.as_deref().unwrap_or_default()
        );
    }
    format!(
        "channel:{}",
        feed.handle
            .as_deref()
            .or(feed.channel_id.as_deref())
            .unwrap_or(&feed.url)
            .to_lowercase()
    )
}

fn handle_from_url(url: &str) -> Option<String> {
    url.split("/@")
        .nth(1)
        .and_then(|tail| tail.split('/').next())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn query_param(url: &str, name: &str) -> Option<String> {
    let query = url.split_once('?')?.1;
    for pair in query.split('&') {
        let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
        if key == name && !value.is_empty() {
            return Some(value.to_string());
        }
    }
    None
}

async fn upsert_feed_source(pool: &PgPool, feed_key: &str, feed: &Feed) -> anyhow::Result<()> {
    let metadata_json = serde_json::to_string(&json!({
        "type": feed.source_type,
        "url": feed.url,
        "handle": feed.handle,
        "playlist_id": feed.playlist_id,
        "channel_id": feed.channel_id,
    }))
    .unwrap_or_else(|_| "{}".to_string());
    sqlx::query!(
        r#"
        INSERT INTO brain.youtube_feed_sources(
          feed_key, source_type, url, handle, playlist_id, channel_id, enabled, metadata, created_at, updated_at
        )
        VALUES($1,$2,$3,$4,$5,$6,$7,$8::text::jsonb,now(),now())
        ON CONFLICT(feed_key) DO UPDATE SET
          source_type=excluded.source_type,
          url=excluded.url,
          handle=excluded.handle,
          playlist_id=excluded.playlist_id,
          channel_id=COALESCE(youtube_feed_sources.channel_id, excluded.channel_id),
          metadata=excluded.metadata,
          updated_at=now()
        "#,
        feed_key,
        feed.source_type,
        feed.url,
        feed.handle.as_deref(),
        feed.playlist_id.as_deref(),
        feed.channel_id.as_deref(),
        1_i64,
        metadata_json,
    )
    .execute(pool)
    .await?;
    Ok(())
}

async fn stored_channel_id(pool: &PgPool, feed_key: &str) -> anyhow::Result<Option<String>> {
    let row = sqlx::query_scalar!(
        "SELECT channel_id FROM brain.youtube_feed_sources WHERE feed_key=$1",
        feed_key,
    )
    .fetch_optional(pool)
    .await?;
    Ok(row.flatten())
}

async fn set_feed_channel_id(
    pool: &PgPool,
    feed_key: &str,
    channel_id: &str,
) -> anyhow::Result<()> {
    sqlx::query!(
        "UPDATE brain.youtube_feed_sources SET channel_id=$1, updated_at=now() WHERE feed_key=$2",
        channel_id,
        feed_key,
    )
    .execute(pool)
    .await?;
    Ok(())
}

fn resolve_channel_id(client: &Client, feed: &Feed) -> anyhow::Result<String> {
    let url = if feed.url.is_empty() {
        format!(
            "https://www.youtube.com/@{}",
            feed.handle.as_deref().unwrap_or("")
        )
    } else {
        feed.url.clone()
    };
    let text = client.get(url).send()?.error_for_status()?.text()?;
    for pattern in [
        r#""channelId"\s*:\s*"(UC[^"]+)""#,
        r#""browseId"\s*:\s*"(UC[^"]+)""#,
        r#"<meta itemprop="channelId" content="(UC[^"]+)">"#,
    ] {
        if let Some(captures) = Regex::new(pattern)?.captures(&text) {
            if let Some(channel_id) = captures.get(1) {
                return Ok(channel_id.as_str().to_string());
            }
        }
    }
    anyhow::bail!("channel_id not found")
}

fn parse_atom_videos(xml: &str, feed_key: &str) -> anyhow::Result<Vec<AtomVideo>> {
    let document = roxmltree::Document::parse(xml)?;
    let mut videos = Vec::new();
    for entry in document
        .descendants()
        .filter(|node| node.is_element() && node.tag_name().name() == "entry")
    {
        let Some(video_id) = child_text(entry, "videoId") else {
            continue;
        };
        let title = child_text(entry, "title").unwrap_or_else(|| format!("YouTube {video_id}"));
        let url = entry
            .children()
            .find(|node| node.is_element() && node.tag_name().name() == "link")
            .and_then(|node| node.attribute("href"))
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| format!("https://www.youtube.com/watch?v={video_id}"));
        videos.push(AtomVideo {
            video_id,
            feed_key: feed_key.to_string(),
            channel_id: child_text(entry, "channelId"),
            channel_title: child_text(entry, "name"),
            title,
            url,
            published_at: child_text(entry, "published"),
            description: child_text(entry, "description"),
        });
    }
    Ok(videos)
}

fn child_text(node: Node<'_, '_>, tag_name: &str) -> Option<String> {
    node.descendants()
        .find(|child| child.is_element() && child.tag_name().name() == tag_name)
        .and_then(|child| child.text())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

#[derive(Debug, PartialEq, Eq)]
enum UpsertAction {
    Inserted,
    Updated,
}

async fn upsert_video(
    pool: &PgPool,
    video: &AtomVideo,
    rss_url: &str,
) -> anyhow::Result<UpsertAction> {
    let existed = sqlx::query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM brain.youtube_videos WHERE video_id=$1) AS "exists!""#,
        video.video_id,
    )
    .fetch_one(pool)
    .await?;
    let metadata_json = serde_json::to_string(&json!({ "rss_url": rss_url }))?;
    sqlx::query!(
        r#"
        INSERT INTO brain.youtube_videos(
          video_id, feed_key, channel_id, channel_title, title, url, published_at, description,
          metadata, transcript_status, learning_status, discovered_at, updated_at
        )
        VALUES($1,$2,$3,$4,$5,$6,$7::text::timestamptz,$8,$9::text::jsonb,$10,$11,now(),now())
        ON CONFLICT(video_id) DO UPDATE SET
          feed_key=excluded.feed_key,
          channel_id=COALESCE(excluded.channel_id, youtube_videos.channel_id),
          channel_title=COALESCE(excluded.channel_title, youtube_videos.channel_title),
          title=excluded.title,
          url=excluded.url,
          published_at=COALESCE(excluded.published_at, youtube_videos.published_at),
          description=COALESCE(excluded.description, youtube_videos.description),
          metadata=excluded.metadata,
          updated_at=now()
        "#,
        video.video_id,
        video.feed_key,
        video.channel_id.as_deref(),
        video.channel_title.as_deref(),
        video.title,
        video.url,
        video.published_at.as_deref(),
        video.description.as_deref(),
        metadata_json,
        "missing",
        "queued",
    )
    .execute(pool)
    .await?;
    Ok(if existed {
        UpsertAction::Updated
    } else {
        UpsertAction::Inserted
    })
}

pub async fn select_next_videos(pool: &PgPool, limit: usize) -> anyhow::Result<Vec<Video>> {
    let limit = i64::try_from(limit.max(1)).unwrap_or(i64::MAX);
    let rows = sqlx::query!(
        r#"
        SELECT video_id, title, url, channel_title,
               published_at::text AS "published_at?", learning_status
        FROM brain.youtube_videos
        WHERE learning_status IN ('queued', 'failed', 'missing_transcript')
          AND video_id NOT IN (
            SELECT video_id FROM brain.youtube_learning_claims
            WHERE prompt_version=$1 AND COALESCE(model, '')=COALESCE($2, '')
          )
        ORDER BY published_at DESC NULLS LAST, discovered_at DESC
        LIMIT $3
        "#,
        claims::PROMPT_VERSION,
        claims::MODEL,
        limit,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| Video {
            video_id: row.video_id,
            title: row.title,
            url: row.url,
            channel_title: row.channel_title,
            published_at: row.published_at,
            learning_status: row.learning_status,
        })
        .collect())
}

pub async fn mark_failed(
    pool: &PgPool,
    video_id: &str,
    kind: &str,
    message: &str,
) -> anyhow::Result<()> {
    update_status_with_metadata(pool, video_id, "failed", kind, message).await
}

pub async fn mark_success(pool: &PgPool, video_id: &str, status: &str) -> anyhow::Result<()> {
    update_status_with_metadata(pool, video_id, status, "ok", "").await
}

async fn update_status_with_metadata(
    pool: &PgPool,
    video_id: &str,
    status: &str,
    kind: &str,
    message: &str,
) -> anyhow::Result<()> {
    let current = sqlx::query_scalar!(
        r#"SELECT metadata::text AS "metadata_json!" FROM brain.youtube_videos WHERE video_id=$1"#,
        video_id,
    )
    .fetch_optional(pool)
    .await?;
    let mut metadata: Value = current
        .as_deref()
        .and_then(|raw| serde_json::from_str(raw).ok())
        .unwrap_or_else(|| json!({}));
    if !metadata.is_object() {
        metadata = json!({});
    }
    metadata["gemini_ingest"] = json!({
        "status": status,
        "kind": kind,
        "message": message,
        "updated_at": db::now_epoch_seconds(),
        "model": claims::MODEL,
        "prompt_version": claims::PROMPT_VERSION,
    });
    let metadata_json = serde_json::to_string(&metadata)?;
    sqlx::query!(
        "UPDATE brain.youtube_videos SET learning_status=$1, metadata=$2::text::jsonb, updated_at=now() WHERE video_id=$3",
        status,
        metadata_json,
        video_id,
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
    async fn select_next_videos_and_mark_status_round_trip_pg() {
        let Some(pool) = crate::testutil::test_pool().await else {
            return;
        };
        let suffix = crate::testutil::unique_suffix();
        let feed_key = format!("ztest_feed_{suffix}");
        let success_id = format!("ztest_ok_{suffix}");
        let failed_id = format!("ztest_fail_{suffix}");
        crate::testutil::seed_feed(&pool, &feed_key).await;
        crate::testutil::seed_video(
            &pool,
            &success_id,
            &feed_key,
            "missing",
            "queued",
            Some("2999-01-01T00:00:00Z"),
            "{}",
        )
        .await;
        crate::testutil::seed_video(
            &pool,
            &failed_id,
            &feed_key,
            "missing",
            "queued",
            Some("2999-01-02T00:00:00Z"),
            "{}",
        )
        .await;

        let selected = select_next_videos(&pool, 500_000).await.expect("select");
        assert!(selected.iter().any(|video| video.video_id == success_id));
        assert!(selected.iter().any(|video| video.video_id == failed_id));

        mark_success(&pool, &success_id, "claims_ready")
            .await
            .expect("mark success");
        mark_failed(&pool, &failed_id, "timeout", "response timeout")
            .await
            .expect("mark failed");

        let after = select_next_videos(&pool, 500_000).await.expect("select 2");
        assert!(
            !after.iter().any(|video| video.video_id == success_id),
            "claims_ready video must not be selectable"
        );
        assert!(
            after.iter().any(|video| video.video_id == failed_id),
            "failed video must remain selectable"
        );

        let (status, metadata_text): (String, String) = {
            let row = sqlx::query(
                "SELECT learning_status, metadata::text AS metadata_text FROM brain.youtube_videos WHERE video_id=$1",
            )
            .bind(&failed_id)
            .fetch_one(&pool)
            .await
            .expect("failed row");
            use sqlx::Row;
            (row.get("learning_status"), row.get("metadata_text"))
        };
        let metadata: serde_json::Value =
            serde_json::from_str(&metadata_text).expect("metadata json");
        assert_eq!(status, "failed");
        assert_eq!(metadata["gemini_ingest"]["status"], "failed");
        assert_eq!(metadata["gemini_ingest"]["kind"], "timeout");
        assert_eq!(metadata["gemini_ingest"]["model"], claims::MODEL);

        crate::testutil::insert_claim_marker(
            &pool,
            &failed_id,
            claims::PROMPT_VERSION,
            claims::MODEL,
            &format!("ztest-hash-{suffix}"),
        )
        .await;
        let after_claim = select_next_videos(&pool, 500_000)
            .await
            .expect("select 3");
        assert!(
            !after_claim.iter().any(|video| video.video_id == failed_id),
            "video with matching claim must be excluded"
        );

        crate::testutil::cleanup(&pool, &[&success_id, &failed_id], &[&feed_key]).await;
    }
}
