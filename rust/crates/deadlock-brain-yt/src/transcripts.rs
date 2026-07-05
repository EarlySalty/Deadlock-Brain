use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::Duration,
};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::postgres::PgPool;
use wait_timeout::ChildExt;

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

pub async fn fetch_transcripts(
    pool: &PgPool,
    limit: usize,
) -> anyhow::Result<FetchTranscriptsSummary> {
    let unavailable_marked = backfill_unavailable_needs_asr(pool).await?;
    let videos = select_videos_missing_transcripts(pool, limit).await?;
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
                let outcome =
                    save_transcript(pool, &video.video_id, &source_kind, &caption.transcript_text)
                        .await?;
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
                mark_transcript_unavailable(pool, &video.video_id).await?;
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

async fn select_videos_missing_transcripts(
    pool: &PgPool,
    limit: usize,
) -> anyhow::Result<Vec<VideoForTranscript>> {
    let limit = i64::try_from(limit.max(1)).unwrap_or(i64::MAX);
    let rows = sqlx::query!(
        r#"
        SELECT v.video_id, v.title, v.url
        FROM brain.youtube_videos v
        WHERE v.transcript_status='missing'
           OR (
             v.transcript_status <> 'unavailable'
             AND NOT EXISTS (
               SELECT 1 FROM brain.youtube_transcripts t
               WHERE t.video_id=v.video_id
                 AND LENGTH(TRIM(t.transcript_text)) > 0
             )
           )
        ORDER BY v.published_at DESC NULLS LAST, v.discovered_at DESC
        LIMIT $1
        "#,
        limit,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| VideoForTranscript {
            video_id: row.video_id,
            title: row.title,
            url: row.url,
        })
        .collect())
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

async fn save_transcript(
    pool: &PgPool,
    video_id: &str,
    source_kind: &str,
    transcript_text: &str,
) -> anyhow::Result<SaveOutcome> {
    let content_hash = stable_hash_text(transcript_text);
    let existing = sqlx::query!(
        r#"
        SELECT language, source_kind, content_hash
        FROM brain.youtube_transcripts WHERE video_id=$1
        "#,
        video_id,
    )
    .fetch_optional(pool)
    .await?;
    let outcome = match existing {
        Some(row)
            if row.language.as_deref() == Some("en")
                && row.source_kind == source_kind
                && row.content_hash == content_hash =>
        {
            SaveOutcome::Unchanged
        }
        Some(_) => {
            sqlx::query!(
                r#"
                UPDATE brain.youtube_transcripts
                SET language=$1, source_kind=$2, transcript_text=$3, content_hash=$4,
                    source_document_id=NULL, updated_at=now()
                WHERE video_id=$5
                "#,
                "en",
                source_kind,
                transcript_text,
                content_hash,
                video_id,
            )
            .execute(pool)
            .await?;
            SaveOutcome::Updated
        }
        None => {
            sqlx::query!(
                r#"
                INSERT INTO brain.youtube_transcripts(
                  video_id, language, source_kind, transcript_text, content_hash,
                  source_document_id, imported_at, updated_at
                )
                VALUES($1,$2,$3,$4,$5,NULL,now(),now())
                "#,
                video_id,
                "en",
                source_kind,
                transcript_text,
                content_hash,
            )
            .execute(pool)
            .await?;
            SaveOutcome::Inserted
        }
    };
    mark_transcript_ready(pool, video_id).await?;
    Ok(outcome)
}

async fn mark_transcript_ready(pool: &PgPool, video_id: &str) -> anyhow::Result<()> {
    let metadata_json = sqlx::query_scalar!(
        r#"SELECT metadata::text AS "metadata_json!" FROM brain.youtube_videos WHERE video_id=$1"#,
        video_id,
    )
    .fetch_optional(pool)
    .await?;
    let mut metadata = json_object(metadata_json.as_deref());
    let removed_needs_asr = metadata
        .as_object_mut()
        .and_then(|object| object.remove("needs_asr"))
        .is_some();
    if removed_needs_asr {
        let metadata_json = serde_json::to_string(&metadata)?;
        sqlx::query!(
            "UPDATE brain.youtube_videos SET transcript_status='ready', metadata=$1::text::jsonb, updated_at=now() WHERE video_id=$2",
            metadata_json,
            video_id,
        )
        .execute(pool)
        .await?;
    } else {
        sqlx::query!(
            "UPDATE brain.youtube_videos SET transcript_status='ready', updated_at=now() WHERE video_id=$1 AND transcript_status <> 'ready'",
            video_id,
        )
        .execute(pool)
        .await?;
    }
    Ok(())
}

async fn mark_transcript_unavailable(pool: &PgPool, video_id: &str) -> anyhow::Result<()> {
    let metadata_json = sqlx::query_scalar!(
        r#"SELECT metadata::text AS "metadata_json!" FROM brain.youtube_videos WHERE video_id=$1"#,
        video_id,
    )
    .fetch_optional(pool)
    .await?;
    let mut metadata = json_object(metadata_json.as_deref());
    metadata["needs_asr"] = Value::Bool(true);
    let metadata_json = serde_json::to_string(&metadata)?;
    sqlx::query!(
        "UPDATE brain.youtube_videos SET transcript_status='unavailable', metadata=$1::text::jsonb, updated_at=now() WHERE video_id=$2",
        metadata_json,
        video_id,
    )
    .execute(pool)
    .await?;
    Ok(())
}

async fn backfill_unavailable_needs_asr(pool: &PgPool) -> anyhow::Result<usize> {
    let rows = sqlx::query!(
        r#"SELECT video_id, metadata::text AS "metadata_json!" FROM brain.youtube_videos WHERE transcript_status='unavailable'"#,
    )
    .fetch_all(pool)
    .await?;
    let mut marked = 0;
    for row in rows {
        let mut metadata = json_object(Some(&row.metadata_json));
        if metadata
            .get("needs_asr")
            .and_then(Value::as_bool)
            .unwrap_or(false)
        {
            continue;
        }
        metadata["needs_asr"] = Value::Bool(true);
        let metadata_json = serde_json::to_string(&metadata)?;
        sqlx::query!(
            "UPDATE brain.youtube_videos SET metadata=$1::text::jsonb, updated_at=now() WHERE video_id=$2",
            metadata_json,
            row.video_id,
        )
        .execute(pool)
        .await?;
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

    #[tokio::test]
    #[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
    async fn save_transcript_is_idempotent_and_sets_ready_pg() {
        let Some(pool) = crate::testutil::test_pool().await else {
            return;
        };
        let suffix = crate::testutil::unique_suffix();
        let feed_key = format!("ztest_feed_{suffix}");
        let video_id = format!("ztest_tr_{suffix}");
        crate::testutil::seed_feed(&pool, &feed_key).await;
        crate::testutil::seed_video(
            &pool,
            &video_id,
            &feed_key,
            "missing",
            "queued",
            Some("2026-06-01T00:00:00Z"),
            r#"{"needs_asr":true}"#,
        )
        .await;

        let first = save_transcript(&pool, &video_id, "youtube_caption_manual", "one two three")
            .await
            .expect("save transcript");
        let second = save_transcript(&pool, &video_id, "youtube_caption_manual", "one two three")
            .await
            .expect("save transcript again");

        assert_eq!(first, SaveOutcome::Inserted);
        assert_eq!(second, SaveOutcome::Unchanged);

        let status: String = sqlx::query_scalar(
            "SELECT transcript_status FROM brain.youtube_videos WHERE video_id=$1",
        )
        .bind(&video_id)
        .fetch_one(&pool)
        .await
        .expect("query status");
        assert_eq!(status, "ready");

        let needs_asr_present: bool = sqlx::query_scalar(
            "SELECT (metadata->>'needs_asr') IS NOT NULL FROM brain.youtube_videos WHERE video_id=$1",
        )
        .bind(&video_id)
        .fetch_one(&pool)
        .await
        .expect("query needs_asr");
        assert!(!needs_asr_present, "needs_asr should be removed on ready");

        crate::testutil::cleanup(&pool, &[&video_id], &[&feed_key]).await;
    }
}
