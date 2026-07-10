use std::{fmt, path::Path, thread, time::Duration};

use regex::Regex;
use reqwest::{blocking::Client, StatusCode};
use roxmltree::Node;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{claims, db, schema};

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

pub fn discover_youtube_videos(
    conn: &Connection,
    config_path: &Path,
    max_videos_per_feed: usize,
) -> anyhow::Result<DiscoverSummary> {
    schema::ensure_youtube_tables(conn)?;
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
            conn,
            &client,
            &feed,
            &feed_key,
            max_videos_per_feed,
            &mut summary,
        ) {
            summary.errors.push(DiscoverError {
                feed: feed_key,
                error: error.to_string(),
            });
        }
    }

    Ok(summary)
}

fn discover_feed(
    conn: &Connection,
    client: &Client,
    feed: &Feed,
    feed_key: &str,
    max_videos_per_feed: usize,
    summary: &mut DiscoverSummary,
) -> anyhow::Result<()> {
    upsert_feed_source(conn, feed_key, feed)?;
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
        let mut channel_id = feed
            .channel_id
            .clone()
            .or_else(|| stored_channel_id(conn, feed_key).ok().flatten())
            .map(Ok)
            .unwrap_or_else(|| resolve_channel_id(client, feed))?;
        set_feed_channel_id(conn, feed_key, &channel_id)?;
        let mut rss_url = channel_rss_url(&channel_id);
        match fetch_text_with_retries(client, &rss_url) {
            Ok(xml) => (rss_url, xml),
            Err(error) if error.is_server_error() => match resolve_channel_id(client, feed) {
                Ok(refreshed_channel_id) if refreshed_channel_id != channel_id => {
                    channel_id = refreshed_channel_id;
                    set_feed_channel_id(conn, feed_key, &channel_id)?;
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
        match upsert_video(conn, &video, &rss_url)? {
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
        thread::sleep(Duration::from_millis(
            FEED_FETCH_BACKOFF_MS * attempt as u64,
        ));
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

fn upsert_feed_source(conn: &Connection, feed_key: &str, feed: &Feed) -> rusqlite::Result<()> {
    let now = db::now_epoch_seconds();
    conn.execute(
        r#"
        INSERT INTO youtube_feed_sources(
          feed_key, source_type, url, handle, playlist_id, channel_id, enabled, metadata_json, created_at, updated_at
        )
        VALUES(?,?,?,?,?,?,?,?,?,?)
        ON CONFLICT(feed_key) DO UPDATE SET
          source_type=excluded.source_type,
          url=excluded.url,
          handle=excluded.handle,
          playlist_id=excluded.playlist_id,
          channel_id=COALESCE(youtube_feed_sources.channel_id, excluded.channel_id),
          metadata_json=excluded.metadata_json,
          updated_at=excluded.updated_at
        "#,
        params![
            feed_key,
            feed.source_type,
            feed.url,
            feed.handle,
            feed.playlist_id,
            feed.channel_id,
            1_i64,
            serde_json::to_string(&json!({
                "type": feed.source_type,
                "url": feed.url,
                "handle": feed.handle,
                "playlist_id": feed.playlist_id,
                "channel_id": feed.channel_id,
            }))
            .unwrap_or_else(|_| "{}".to_string()),
            now,
            now
        ],
    )?;
    Ok(())
}

fn stored_channel_id(conn: &Connection, feed_key: &str) -> rusqlite::Result<Option<String>> {
    conn.query_row(
        "SELECT channel_id FROM youtube_feed_sources WHERE feed_key=?",
        params![feed_key],
        |row| row.get(0),
    )
    .optional()
}

fn set_feed_channel_id(
    conn: &Connection,
    feed_key: &str,
    channel_id: &str,
) -> rusqlite::Result<()> {
    conn.execute(
        "UPDATE youtube_feed_sources SET channel_id=?, updated_at=? WHERE feed_key=?",
        params![channel_id, db::now_epoch_seconds(), feed_key],
    )?;
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

fn upsert_video(
    conn: &Connection,
    video: &AtomVideo,
    rss_url: &str,
) -> anyhow::Result<UpsertAction> {
    let existed = conn
        .query_row(
            "SELECT 1 FROM youtube_videos WHERE video_id=?",
            params![video.video_id],
            |_| Ok(()),
        )
        .optional()?
        .is_some();
    let now = db::now_epoch_seconds();
    conn.execute(
        r#"
        INSERT INTO youtube_videos(
          video_id, feed_key, channel_id, channel_title, title, url, published_at, description,
          metadata_json, transcript_status, learning_status, discovered_at, updated_at
        )
        VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?)
        ON CONFLICT(video_id) DO UPDATE SET
          feed_key=excluded.feed_key,
          channel_id=COALESCE(excluded.channel_id, youtube_videos.channel_id),
          channel_title=COALESCE(excluded.channel_title, youtube_videos.channel_title),
          title=excluded.title,
          url=excluded.url,
          published_at=COALESCE(excluded.published_at, youtube_videos.published_at),
          description=COALESCE(excluded.description, youtube_videos.description),
          metadata_json=excluded.metadata_json,
          updated_at=excluded.updated_at
        "#,
        params![
            video.video_id,
            video.feed_key,
            video.channel_id,
            video.channel_title,
            video.title,
            video.url,
            video.published_at,
            video.description,
            serde_json::to_string(&json!({ "rss_url": rss_url }))?,
            "missing",
            "queued",
            now,
            now
        ],
    )?;
    Ok(if existed {
        UpsertAction::Updated
    } else {
        UpsertAction::Inserted
    })
}

pub fn select_next_videos(conn: &Connection, limit: usize) -> rusqlite::Result<Vec<Video>> {
    let mut statement = conn.prepare(
        r#"
        SELECT video_id, title, url, channel_title, published_at, learning_status
        FROM youtube_videos
        WHERE learning_status IN ('queued', 'failed', 'missing_transcript')
          AND video_id NOT IN (
            SELECT video_id FROM youtube_learning_claims
            WHERE prompt_version=? AND COALESCE(model, '')=COALESCE(?, '')
          )
        ORDER BY COALESCE(published_at, '') DESC, discovered_at DESC
        LIMIT ?
        "#,
    )?;
    let videos = statement
        .query_map(
            params![claims::PROMPT_VERSION, claims::MODEL, limit.max(1) as i64],
            |row| {
                Ok(Video {
                    video_id: row.get(0)?,
                    title: row.get(1)?,
                    url: row.get(2)?,
                    channel_title: row.get(3)?,
                    published_at: row.get(4)?,
                    learning_status: row.get(5)?,
                })
            },
        )?
        .collect();
    videos
}

pub fn mark_failed(
    conn: &Connection,
    video_id: &str,
    kind: &str,
    message: &str,
) -> anyhow::Result<()> {
    update_status_with_metadata(conn, video_id, "failed", kind, message)
}

pub fn mark_success(conn: &Connection, video_id: &str, status: &str) -> anyhow::Result<()> {
    update_status_with_metadata(conn, video_id, status, "ok", "")
}

fn update_status_with_metadata(
    conn: &Connection,
    video_id: &str,
    status: &str,
    kind: &str,
    message: &str,
) -> anyhow::Result<()> {
    let current: Option<String> = conn
        .query_row(
            "SELECT metadata_json FROM youtube_videos WHERE video_id=?",
            params![video_id],
            |row| row.get(0),
        )
        .optional()?;
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
    conn.execute(
        "UPDATE youtube_videos SET learning_status=?, metadata_json=?, updated_at=? WHERE video_id=?",
        params![
            status,
            serde_json::to_string(&metadata)?,
            db::now_epoch_seconds(),
            video_id
        ],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_status_transitions_and_dedupe_work() {
        let conn = Connection::open_in_memory().expect("open sqlite");
        schema::ensure_youtube_tables(&conn).expect("ensure schema");
        insert_feed(&conn);
        insert_video(&conn, "queued-video", "queued", 10);
        insert_video(&conn, "failed-video", "failed", 20);
        insert_video(&conn, "missing-video", "missing_transcript", 30);
        insert_video(&conn, "done-video", "claims_ready", 40);
        insert_claim_marker(&conn, "queued-video");

        let selected = select_next_videos(&conn, 10).expect("select");
        let ids: Vec<_> = selected
            .iter()
            .map(|video| video.video_id.as_str())
            .collect();
        assert_eq!(ids, vec!["missing-video", "failed-video"]);

        mark_success(&conn, "failed-video", "no_claims").expect("status");
        let selected = select_next_videos(&conn, 10).expect("select");
        let ids: Vec<_> = selected
            .iter()
            .map(|video| video.video_id.as_str())
            .collect();
        assert_eq!(ids, vec!["missing-video"]);

        mark_failed(&conn, "done-video", "timeout", "response timeout").expect("failed status");
        let (status, metadata_json): (String, String) = conn
            .query_row(
                "SELECT learning_status, metadata_json FROM youtube_videos WHERE video_id='done-video'",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .expect("status metadata");
        let metadata: Value = serde_json::from_str(&metadata_json).expect("metadata json");
        assert_eq!(status, "failed");
        assert_eq!(metadata["gemini_ingest"]["status"], "failed");
        assert_eq!(metadata["gemini_ingest"]["kind"], "timeout");
        assert_eq!(metadata["gemini_ingest"]["model"], claims::MODEL);
    }

    fn insert_feed(conn: &Connection) {
        let now = db::now_epoch_seconds();
        conn.execute(
            r#"
            INSERT INTO youtube_feed_sources(feed_key, source_type, url, enabled, metadata_json, created_at, updated_at)
            VALUES('feed', 'channel', 'https://example.invalid', 1, '{}', ?, ?)
            "#,
            params![now, now],
        )
        .expect("insert feed");
    }

    fn insert_video(conn: &Connection, video_id: &str, status: &str, discovered_at: i64) {
        let now = db::now_epoch_seconds();
        conn.execute(
            r#"
            INSERT INTO youtube_videos(
              video_id, feed_key, title, url, metadata_json, transcript_status, learning_status, discovered_at, updated_at
            )
            VALUES(?,?,?,?,?,?,?,?,?)
            "#,
            params![
                video_id,
                "feed",
                video_id,
                format!("https://youtube.com/watch?v={video_id}"),
                "{}",
                "missing",
                status,
                discovered_at,
                now
            ],
        )
        .expect("insert video");
    }

    fn insert_claim_marker(conn: &Connection, video_id: &str) {
        let now = db::now_epoch_seconds();
        conn.execute(
            r#"
            INSERT INTO youtube_learning_claims(
              video_id, claim_hash, claim_index, claim_type, claim_text, evidence_quote,
              model_confidence, verifier_confidence, status, model, prompt_version, prompt_text,
              model_response_text, provider_metadata_json, verifier_json, created_at, updated_at
            )
            VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)
            "#,
            params![
                video_id,
                format!("hash-{video_id}"),
                0_i64,
                "build",
                "claim",
                "",
                0.5_f64,
                0.0_f64,
                "accepted",
                claims::MODEL,
                claims::PROMPT_VERSION,
                "prompt",
                "{}",
                "{}",
                "{}",
                now,
                now
            ],
        )
        .expect("insert claim");
    }
}
