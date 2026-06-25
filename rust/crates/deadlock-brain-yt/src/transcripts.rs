use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::Duration,
};

use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use wait_timeout::ChildExt;

use crate::{db, schema};

const DEFAULT_YT_DLP_TIMEOUT_SECONDS: u64 = 120;

#[derive(Debug, Serialize)]
pub struct FetchTranscriptsSummary {
    pub selected: usize,
    pub processed: usize,
    pub saved: usize,
    pub unchanged: usize,
    pub unavailable: usize,
    pub unavailable_marked: usize,
    pub failed: usize,
    pub videos: Vec<FetchVideoSummary>,
}

#[derive(Debug, Serialize)]
pub struct FetchVideoSummary {
    pub video_id: String,
    pub title: String,
    pub status: String,
    pub language: Option<String>,
    pub source_kind: Option<String>,
    pub transcript_chars: usize,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
struct VideoForTranscript {
    video_id: String,
    title: String,
    url: String,
}

#[derive(Debug, Clone)]
struct CaptionData {
    source_kind: CaptionSourceKind,
    transcript_text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CaptionSourceKind {
    Manual,
    Auto,
}

impl CaptionSourceKind {
    fn as_db_value(self) -> &'static str {
        match self {
            Self::Manual => "youtube_caption_manual",
            Self::Auto => "youtube_caption_auto",
        }
    }

    fn priority(self) -> u8 {
        match self {
            Self::Manual => 0,
            Self::Auto => 1,
        }
    }
}

#[derive(Debug)]
struct CaptionFile {
    path: PathBuf,
    language: String,
    source_kind: CaptionSourceKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SaveOutcome {
    Inserted,
    Updated,
    Unchanged,
}

#[derive(Debug, Deserialize)]
struct Json3Root {
    #[serde(default)]
    events: Vec<Json3Event>,
}

#[derive(Debug, Deserialize)]
struct Json3Event {
    #[serde(default)]
    segs: Vec<Json3Segment>,
}

#[derive(Debug, Deserialize)]
struct Json3Segment {
    utf8: Option<String>,
}

pub fn fetch_transcripts(conn: &Connection, limit: usize) -> anyhow::Result<FetchTranscriptsSummary> {
    schema::ensure_youtube_tables(conn)?;
    let unavailable_marked = backfill_unavailable_needs_asr(conn)?;
    let videos = select_videos_missing_transcripts(conn, limit)?;
    let mut summary = FetchTranscriptsSummary {
        selected: videos.len(),
        processed: 0,
        saved: 0,
        unchanged: 0,
        unavailable: 0,
        unavailable_marked,
        failed: 0,
        videos: Vec::new(),
    };

    for video in videos {
        match fetch_caption_for_video(&video) {
            Ok(Some(caption)) => {
                let chars = caption.transcript_text.chars().count();
                let source_kind = caption.source_kind.as_db_value().to_string();
                let outcome = save_transcript(
                    conn,
                    &video.video_id,
                    &source_kind,
                    &caption.transcript_text,
                )?;
                match outcome {
                    SaveOutcome::Inserted | SaveOutcome::Updated => summary.saved += 1,
                    SaveOutcome::Unchanged => summary.unchanged += 1,
                }
                summary.processed += 1;
                summary.videos.push(FetchVideoSummary {
                    video_id: video.video_id,
                    title: video.title,
                    status: match outcome {
                        SaveOutcome::Inserted => "inserted".to_string(),
                        SaveOutcome::Updated => "updated".to_string(),
                        SaveOutcome::Unchanged => "unchanged".to_string(),
                    },
                    language: Some("en".to_string()),
                    source_kind: Some(source_kind),
                    transcript_chars: chars,
                    error: None,
                });
            }
            Ok(None) => {
                mark_transcript_unavailable(conn, &video.video_id)?;
                summary.processed += 1;
                summary.unavailable += 1;
                summary.videos.push(FetchVideoSummary {
                    video_id: video.video_id,
                    title: video.title,
                    status: "unavailable".to_string(),
                    language: None,
                    source_kind: None,
                    transcript_chars: 0,
                    error: None,
                });
            }
            Err(error) => {
                summary.processed += 1;
                summary.failed += 1;
                summary.videos.push(FetchVideoSummary {
                    video_id: video.video_id,
                    title: video.title,
                    status: "failed".to_string(),
                    language: None,
                    source_kind: None,
                    transcript_chars: 0,
                    error: Some(error.to_string()),
                });
            }
        }
    }

    Ok(summary)
}

fn select_videos_missing_transcripts(
    conn: &Connection,
    limit: usize,
) -> rusqlite::Result<Vec<VideoForTranscript>> {
    let mut statement = conn.prepare(
        r#"
        SELECT v.video_id, v.title, v.url
        FROM youtube_videos v
        WHERE v.transcript_status='missing'
           OR (
             v.transcript_status <> 'unavailable'
             AND NOT EXISTS (
               SELECT 1 FROM youtube_transcripts t
               WHERE t.video_id=v.video_id
                 AND LENGTH(TRIM(t.transcript_text)) > 0
             )
           )
        ORDER BY COALESCE(v.published_at, '') DESC, v.discovered_at DESC
        LIMIT ?
        "#,
    )?;
    let rows = statement
        .query_map(params![limit.max(1) as i64], |row| {
            Ok(VideoForTranscript {
                video_id: row.get(0)?,
                title: row.get(1)?,
                url: row.get(2)?,
            })
        })?
        .collect();
    rows
}

fn fetch_caption_for_video(video: &VideoForTranscript) -> anyhow::Result<Option<CaptionData>> {
    let temp_dir = tempfile::tempdir()?;
    let output_template = temp_dir
        .path()
        .join(format!("{}.%(ext)s", video.video_id));
    let mut child = Command::new(yt_dlp_bin())
        .arg("--no-update")
        .arg("--no-warnings")
        .arg("--skip-download")
        .arg("--write-subs")
        .arg("--write-auto-subs")
        .arg("--sub-langs")
        .arg("en.*")
        .arg("--sub-format")
        .arg("json3")
        .arg("-o")
        .arg(&output_template)
        .arg(&video.url)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let timeout = Duration::from_secs(yt_dlp_timeout_seconds());
    let output = match child.wait_timeout(timeout) {
        Ok(Some(_)) => child.wait_with_output()?,
        Ok(None) => {
            let _ = child.kill();
            let _ = child.wait();
            anyhow::bail!("yt-dlp timeout after {}s", timeout.as_secs());
        }
        Err(error) => return Err(error.into()),
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let command_output = format!("{stdout}\n{stderr}");
    let caption_file = select_caption_file(temp_dir.path(), &video.video_id, &command_output)?;
    if let Some(caption_file) = caption_file {
        let raw = fs::read_to_string(&caption_file.path)?;
        let transcript_text = parse_json3_transcript(&raw)?;
        return Ok(Some(CaptionData {
            source_kind: caption_file.source_kind,
            transcript_text,
        }));
    }

    if output.status.success() || looks_like_no_captions(&command_output) {
        return Ok(None);
    }

    anyhow::bail!("yt-dlp failed: {}", command_output.trim())
}

fn select_caption_file(
    dir: &Path,
    video_id: &str,
    command_output: &str,
) -> anyhow::Result<Option<CaptionFile>> {
    let mut candidates = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("json3") {
            continue;
        }
        let Some(file_name) = path.file_name().and_then(|value| value.to_str()) else {
            continue;
        };
        let file_name = file_name.to_string();
        let Some(language) = caption_language_from_file_name(&file_name, video_id) else {
            continue;
        };
        if !matches!(language.as_str(), "en" | "en-orig") {
            continue;
        }
        candidates.push(CaptionFile {
            path,
            language,
            source_kind: infer_source_kind(command_output, &file_name),
        });
    }
    candidates.sort_by(|left, right| {
        (
            left.source_kind.priority(),
            language_priority(&left.language),
            left.path.as_os_str(),
        )
            .cmp(&(
                right.source_kind.priority(),
                language_priority(&right.language),
                right.path.as_os_str(),
            ))
    });
    Ok(candidates.into_iter().next())
}

fn caption_language_from_file_name(file_name: &str, video_id: &str) -> Option<String> {
    let prefix = format!("{video_id}.");
    let without_prefix = file_name.strip_prefix(&prefix)?;
    let language = without_prefix.strip_suffix(".json3")?;
    if language.trim().is_empty() {
        return None;
    }
    Some(language.to_string())
}

fn infer_source_kind(command_output: &str, file_name: &str) -> CaptionSourceKind {
    let mut manual_seen = false;
    for line in command_output.lines() {
        if !line.contains(file_name) {
            continue;
        }
        let lower = line.to_ascii_lowercase();
        if lower.contains("automatic subtitles") {
            return CaptionSourceKind::Auto;
        }
        if lower.contains("subtitles") {
            manual_seen = true;
        }
    }
    if manual_seen {
        CaptionSourceKind::Manual
    } else {
        CaptionSourceKind::Auto
    }
}

fn language_priority(language: &str) -> u8 {
    match language {
        "en" => 0,
        "en-orig" => 1,
        _ => 2,
    }
}

fn looks_like_no_captions(output: &str) -> bool {
    let lower = output.to_ascii_lowercase();
    lower.contains("has no subtitles")
        || lower.contains("has no automatic captions")
        || lower.contains("no automatic captions")
        || lower.contains("there are no subtitles")
        || lower.contains("no subtitles for the requested languages")
}

pub fn parse_json3_transcript(raw: &str) -> anyhow::Result<String> {
    let root: Json3Root = serde_json::from_str(raw)?;
    let mut text = String::new();
    for event in root.events {
        for segment in event.segs {
            if let Some(segment_text) = segment.utf8 {
                text.push_str(&segment_text);
                text.push(' ');
            }
        }
    }
    let normalized = normalize_whitespace(&text);
    if normalized.is_empty() {
        anyhow::bail!("json3 caption has no text");
    }
    Ok(normalized)
}

fn normalize_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn save_transcript(
    conn: &Connection,
    video_id: &str,
    source_kind: &str,
    transcript_text: &str,
) -> anyhow::Result<SaveOutcome> {
    let content_hash = stable_hash_text(transcript_text);
    let existing: Option<(String, String, String)> = conn
        .query_row(
            "SELECT language, source_kind, content_hash FROM youtube_transcripts WHERE video_id=?",
            params![video_id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .optional()?;
    let now = db::now_epoch_seconds();
    let outcome = match existing {
        Some((language, existing_source_kind, existing_hash))
            if language == "en" && existing_source_kind == source_kind && existing_hash == content_hash =>
        {
            SaveOutcome::Unchanged
        }
        Some(_) => {
            conn.execute(
                r#"
                UPDATE youtube_transcripts
                SET language=?, source_kind=?, transcript_text=?, content_hash=?,
                    source_document_id=NULL, updated_at=?
                WHERE video_id=?
                "#,
                params![
                    "en",
                    source_kind,
                    transcript_text,
                    content_hash,
                    now,
                    video_id
                ],
            )?;
            SaveOutcome::Updated
        }
        None => {
            conn.execute(
                r#"
                INSERT INTO youtube_transcripts(
                  video_id, language, source_kind, transcript_text, content_hash,
                  source_document_id, imported_at, updated_at
                )
                VALUES(?,?,?,?,?,?,?,?)
                "#,
                params![
                    video_id,
                    "en",
                    source_kind,
                    transcript_text,
                    content_hash,
                    Option::<i64>::None,
                    now,
                    now
                ],
            )?;
            SaveOutcome::Inserted
        }
    };
    mark_transcript_ready(conn, video_id)?;
    Ok(outcome)
}

fn mark_transcript_ready(conn: &Connection, video_id: &str) -> anyhow::Result<()> {
    let metadata_json: Option<String> = conn
        .query_row(
            "SELECT metadata_json FROM youtube_videos WHERE video_id=?",
            params![video_id],
            |row| row.get(0),
        )
        .optional()?;
    let mut metadata = json_object(metadata_json.as_deref());
    let removed_needs_asr = metadata
        .as_object_mut()
        .and_then(|object| object.remove("needs_asr"))
        .is_some();
    if removed_needs_asr {
        conn.execute(
            "UPDATE youtube_videos SET transcript_status='ready', metadata_json=?, updated_at=? WHERE video_id=?",
            params![serde_json::to_string(&metadata)?, db::now_epoch_seconds(), video_id],
        )?;
    } else {
        conn.execute(
            "UPDATE youtube_videos SET transcript_status='ready', updated_at=? WHERE video_id=? AND transcript_status <> 'ready'",
            params![db::now_epoch_seconds(), video_id],
        )?;
    }
    Ok(())
}

fn mark_transcript_unavailable(conn: &Connection, video_id: &str) -> anyhow::Result<()> {
    let metadata_json: Option<String> = conn
        .query_row(
            "SELECT metadata_json FROM youtube_videos WHERE video_id=?",
            params![video_id],
            |row| row.get(0),
        )
        .optional()?;
    let mut metadata = json_object(metadata_json.as_deref());
    metadata["needs_asr"] = Value::Bool(true);
    conn.execute(
        "UPDATE youtube_videos SET transcript_status='unavailable', metadata_json=?, updated_at=? WHERE video_id=?",
        params![serde_json::to_string(&metadata)?, db::now_epoch_seconds(), video_id],
    )?;
    Ok(())
}

fn backfill_unavailable_needs_asr(conn: &Connection) -> anyhow::Result<usize> {
    let mut statement = conn.prepare(
        "SELECT video_id, metadata_json FROM youtube_videos WHERE transcript_status='unavailable'",
    )?;
    let rows = statement.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;
    let mut marked = 0;
    for row in rows {
        let (video_id, metadata_json) = row?;
        let mut metadata = json_object(Some(&metadata_json));
        if metadata
            .get("needs_asr")
            .and_then(Value::as_bool)
            .unwrap_or(false)
        {
            continue;
        }
        metadata["needs_asr"] = Value::Bool(true);
        conn.execute(
            "UPDATE youtube_videos SET metadata_json=?, updated_at=? WHERE video_id=?",
            params![serde_json::to_string(&metadata)?, db::now_epoch_seconds(), video_id],
        )?;
        marked += 1;
    }
    Ok(marked)
}

fn json_object(raw: Option<&str>) -> Value {
    let mut metadata = raw
        .and_then(|text| serde_json::from_str::<Value>(text).ok())
        .unwrap_or_else(|| json!({}));
    if !metadata.is_object() {
        metadata = json!({});
    }
    metadata
}

fn stable_hash_text(content: &str) -> String {
    hex::encode(Sha256::digest(content.as_bytes()))
}

fn yt_dlp_bin() -> String {
    std::env::var("YT_DLP_BIN").unwrap_or_else(|_| "yt-dlp".to_string())
}

fn yt_dlp_timeout_seconds() -> u64 {
    std::env::var("YT_DLP_TIMEOUT_SECONDS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(DEFAULT_YT_DLP_TIMEOUT_SECONDS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_json3_into_normalized_flow_text() {
        let parsed = parse_json3_transcript(
            r#"{
              "events": [
                {"segs": [{"utf8": "Hello"}, {"utf8": "  world\n"}]},
                {"segs": [{"utf8": "from"}, {"utf8": " captions"}]},
                {"segs": [{"utf8": "\n"}]}
              ]
            }"#,
        )
        .expect("parse json3");

        assert_eq!(parsed, "Hello world from captions");
    }

    #[test]
    fn caption_selection_prefers_manual_english_and_rejects_translations() {
        let temp = tempfile::tempdir().expect("tempdir");
        fs::write(temp.path().join("abc.en.json3"), "{}").expect("write en");
        fs::write(temp.path().join("abc.en-orig.json3"), "{}").expect("write en orig");
        fs::write(temp.path().join("abc.de.json3"), "{}").expect("write de");
        let output = "\
[info] Writing video automatic subtitles to: abc.en-orig.json3
[info] Writing video subtitles to: abc.en.json3";

        let selected = select_caption_file(temp.path(), "abc", output)
            .expect("select caption")
            .expect("caption selected");

        assert_eq!(selected.language, "en");
        assert_eq!(selected.source_kind, CaptionSourceKind::Manual);
    }

    #[test]
    fn save_transcript_is_idempotent_and_sets_ready() {
        let conn = Connection::open_in_memory().expect("open sqlite");
        deadlock_brain_core::schema::ensure_schema(&conn).expect("ensure schema");
        insert_feed(&conn);
        insert_video(&conn, "vid", "missing", "{}");

        let first = save_transcript(&conn, "vid", "youtube_caption_manual", "one two three")
            .expect("save transcript");
        let second = save_transcript(&conn, "vid", "youtube_caption_manual", "one two three")
            .expect("save transcript again");

        assert_eq!(first, SaveOutcome::Inserted);
        assert_eq!(second, SaveOutcome::Unchanged);
        let (rows, status): (i64, String) = conn
            .query_row(
                "SELECT (SELECT COUNT(*) FROM youtube_transcripts), transcript_status FROM youtube_videos WHERE video_id='vid'",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .expect("query saved transcript");
        assert_eq!(rows, 1);
        assert_eq!(status, "ready");
    }

    #[test]
    fn unavailable_marks_needs_asr_and_is_not_selected_again() {
        let conn = Connection::open_in_memory().expect("open sqlite");
        deadlock_brain_core::schema::ensure_schema(&conn).expect("ensure schema");
        insert_feed(&conn);
        insert_video(&conn, "vid", "missing", r#"{"keep": true}"#);

        mark_transcript_unavailable(&conn, "vid").expect("mark unavailable");
        let selected = select_videos_missing_transcripts(&conn, 10).expect("select videos");

        assert!(selected.is_empty());
        let (status, metadata_json): (String, String) = conn
            .query_row(
                "SELECT transcript_status, metadata_json FROM youtube_videos WHERE video_id='vid'",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .expect("query video");
        let metadata: Value = serde_json::from_str(&metadata_json).expect("metadata");
        assert_eq!(status, "unavailable");
        assert_eq!(metadata["keep"], true);
        assert_eq!(metadata["needs_asr"], true);
    }

    #[test]
    fn backfill_marks_existing_unavailable_rows_once() {
        let conn = Connection::open_in_memory().expect("open sqlite");
        deadlock_brain_core::schema::ensure_schema(&conn).expect("ensure schema");
        insert_feed(&conn);
        insert_video(&conn, "vid", "unavailable", r#"{"content_type":"verbal_strategy"}"#);

        let first = backfill_unavailable_needs_asr(&conn).expect("backfill first");
        let second = backfill_unavailable_needs_asr(&conn).expect("backfill second");

        assert_eq!(first, 1);
        assert_eq!(second, 0);
        let metadata_json: String = conn
            .query_row(
                "SELECT metadata_json FROM youtube_videos WHERE video_id='vid'",
                [],
                |row| row.get(0),
            )
            .expect("query metadata");
        let metadata: Value = serde_json::from_str(&metadata_json).expect("metadata");
        assert_eq!(metadata["content_type"], "verbal_strategy");
        assert_eq!(metadata["needs_asr"], true);
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

    fn insert_video(conn: &Connection, video_id: &str, status: &str, metadata_json: &str) {
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
                metadata_json,
                status,
                "queued",
                now,
                now
            ],
        )
        .expect("insert video");
    }
}
