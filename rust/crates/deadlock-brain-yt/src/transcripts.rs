use std::time::Duration;

use anyhow::{ensure, Context};
use reqwest::blocking::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use sqlx::postgres::PgPool;

const DEFAULT_CAPTION_TIMEOUT_SECONDS: u64 = 30;
const MAX_CAPTION_BYTES: usize = 16 * 1024 * 1024;
const MAX_WATCH_PAGE_BYTES: usize = 8 * 1024 * 1024;
const USER_AGENT: &str = "Mozilla/5.0 DeadlockBrain/0.1";

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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CaptionSegment {
    pub source_event_index: usize,
    pub start_ms: Option<u64>,
    pub duration_ms: Option<u64>,
    pub end_ms: Option<u64>,
    pub text: String,
    pub pieces: Vec<CaptionPiece>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CaptionPiece {
    pub text: String,
    pub offset_ms: Option<u64>,
}

#[derive(Debug, Clone)]
struct CaptionData {
    source_kind: CaptionSourceKind,
    transcript_text: String,
    raw_json: String,
    segments: Vec<CaptionSegment>,
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct CaptionTrack {
    base_url: String,
    language_code: String,
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
    #[serde(rename = "tStartMs")]
    start_ms: Option<u64>,
    #[serde(rename = "dDurationMs")]
    duration_ms: Option<u64>,
    #[serde(default)]
    segs: Vec<Json3Segment>,
}

#[derive(Debug, Deserialize)]
struct Json3Segment {
    utf8: Option<String>,
    #[serde(rename = "tOffsetMs")]
    offset_ms: Option<u64>,
}

pub async fn fetch_transcripts(pool: &PgPool, limit: usize) -> anyhow::Result<FetchTranscriptsSummary> {
    let evidence_table: Option<String> = sqlx::query_scalar(
        "SELECT to_regclass('brain.youtube_transcript_evidence')::text",
    )
    .fetch_one(pool)
    .await?;
    ensure!(
        evidence_table.is_some(),
        "Migration 2026-09-18-patch-evidence.sql fehlt vor dem Caption Import."
    );
    let unavailable_marked = sqlx::query(
        "UPDATE brain.youtube_videos SET metadata=COALESCE(metadata, '{}'::jsonb) || '{\"needs_asr\":true}'::jsonb, updated_at=now() WHERE transcript_status='unavailable' AND COALESCE(metadata->>'needs_asr','false') <> 'true'",
    )
    .execute(pool)
    .await?
    .rows_affected() as usize;
    let limit = i64::try_from(limit.max(1)).unwrap_or(i64::MAX);
    let rows: Vec<(String, String)> = sqlx::query_as(
        "SELECT v.video_id, v.title FROM brain.youtube_videos v WHERE v.transcript_status='missing' OR (v.transcript_status <> 'unavailable' AND NOT EXISTS (SELECT 1 FROM brain.youtube_transcripts t WHERE t.video_id=v.video_id AND LENGTH(TRIM(t.transcript_text)) > 0)) OR (v.transcript_status='ready' AND NOT EXISTS (SELECT 1 FROM brain.youtube_transcript_evidence e WHERE e.video_id=v.video_id AND e.raw_sha256=v.metadata->>'transcript_evidence_hash')) ORDER BY v.published_at DESC NULLS LAST, v.discovered_at DESC LIMIT $1",
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;
    let mut summary = FetchTranscriptsSummary {
        selected: rows.len(),
        processed: 0,
        saved: 0,
        unchanged: 0,
        unavailable: 0,
        unavailable_marked,
        failed: 0,
        videos: Vec::new(),
    };
    for (video_id, title) in rows {
        let video = VideoForTranscript { video_id, title };
        let fetch_video = video.clone();
        let result = tokio::task::spawn_blocking(move || fetch_caption_for_video(&fetch_video))
            .await
            .context("Caption Worker wurde abgebrochen.")?;
        let mut item = FetchVideoSummary {
            video_id: video.video_id.clone(),
            title: video.title,
            status: "failed".to_string(),
            language: None,
            source_kind: None,
            transcript_chars: 0,
            error: None,
        };
        match result {
            Ok(Some(caption)) => match save_transcript(pool, &video.video_id, &caption).await {
                Ok(outcome) => {
                    item.status = match outcome {
                        SaveOutcome::Inserted => "inserted",
                        SaveOutcome::Updated => "updated",
                        SaveOutcome::Unchanged => "unchanged",
                    }
                    .to_string();
                    item.language = Some("en".to_string());
                    item.source_kind = Some(caption.source_kind.as_db_value().to_string());
                    item.transcript_chars = caption.transcript_text.chars().count();
                    if outcome == SaveOutcome::Unchanged {
                        summary.unchanged += 1;
                    } else {
                        summary.saved += 1;
                    }
                }
                Err(_) => {
                    summary.failed += 1;
                    item.error = Some(
                        "Caption Persistenz fehlgeschlagen, die Transaktion wurde zurückgerollt."
                            .to_string(),
                    );
                }
            },
            Ok(None) => {
                sqlx::query("UPDATE brain.youtube_videos SET transcript_status='unavailable', metadata=COALESCE(metadata,'{}'::jsonb) || '{\"needs_asr\":true}'::jsonb, updated_at=now() WHERE video_id=$1")
                    .bind(&video.video_id)
                    .execute(pool)
                    .await?;
                summary.unavailable += 1;
                item.status = "unavailable".to_string();
            }
            Err(error) => {
                summary.failed += 1;
                item.error = Some(error.to_string());
            }
        }
        summary.processed += 1;
        summary.videos.push(item);
    }
    Ok(summary)
}

fn fetch_caption_for_video(video: &VideoForTranscript) -> anyhow::Result<Option<CaptionData>> {
    ensure!(valid_video_id(&video.video_id), "Ungültige YouTube Video ID.");
    let client = Client::builder()
        .user_agent(USER_AGENT)
        .timeout(Duration::from_secs(caption_timeout_seconds()))
        .redirect(reqwest::redirect::Policy::limited(3))
        .build()?;
    let watch_url = format!("https://www.youtube.com/watch?v={}", video.video_id);
    let watch = client
        .get(watch_url)
        .send()
        .context("YouTube Player Seite konnte nicht geladen werden.")?;
    let watch_text = response_text_limited(watch, MAX_WATCH_PAGE_BYTES)?;
    let player = extract_initial_player_response(&watch_text)
        .context("YouTube Player Response fehlt.")?;
    let Some(track) = select_english_caption_track(&player) else {
        return Ok(None);
    };

    let mut caption_url = reqwest::Url::parse(&track.base_url)
        .context("Caption URL ist ungültig.")?;
    ensure!(
        caption_url.scheme() == "https"
            && matches!(caption_url.host_str(), Some("www.youtube.com" | "youtube.com")),
        "Caption URL verweist nicht auf YouTube."
    );
    caption_url.query_pairs_mut().append_pair("fmt", "json3");
    let response = client
        .get(caption_url)
        .send()
        .context("YouTube Caption konnte nicht geladen werden.")?;
    let raw_json = response_text_limited(response, MAX_CAPTION_BYTES)?;
    if raw_json.trim().is_empty() {
        return Ok(None);
    }
    let segments = parse_json3_segments(&raw_json)?;
    let transcript_text = segments
        .iter()
        .map(|part| part.text.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    Ok(Some(CaptionData {
        source_kind: track.source_kind,
        transcript_text,
        raw_json,
        segments,
    }))
}

fn response_text_limited(response: Response, max_bytes: usize) -> anyhow::Result<String> {
    ensure!(response.status().is_success(), "YouTube Anfrage war nicht erfolgreich.");
    if let Some(length) = response.content_length() {
        ensure!(
            length <= max_bytes as u64,
            "YouTube Antwort überschreitet das Größenlimit."
        );
    }
    let bytes = response.bytes()?;
    ensure!(
        bytes.len() <= max_bytes,
        "YouTube Antwort überschreitet das Größenlimit."
    );
    String::from_utf8(bytes.to_vec()).context("YouTube Antwort ist kein UTF-8.")
}

fn extract_initial_player_response(html: &str) -> Option<Value> {
    for marker in [
        "var ytInitialPlayerResponse = ",
        "\"ytInitialPlayerResponse\":",
    ] {
        let marker_start = html.find(marker)? + marker.len();
        if let Some(raw) = extract_json_object(&html[marker_start..]) {
            if let Ok(value) = serde_json::from_str(raw) {
                return Some(value);
            }
        }
    }
    None
}

fn extract_json_object(input: &str) -> Option<&str> {
    let start = input.find('{')?;
    let bytes = input.as_bytes();
    let mut depth = 0_u32;
    let mut in_string = false;
    let mut escaped = false;
    for index in start..bytes.len() {
        let byte = bytes[index];
        if in_string {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            }
            continue;
        }
        match byte {
            b'"' => in_string = true,
            b'{' => depth = depth.saturating_add(1),
            b'}' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return input.get(start..=index);
                }
            }
            _ => {}
        }
    }
    None
}

fn select_english_caption_track(player: &Value) -> Option<CaptionTrack> {
    let tracks = player
        .pointer("/captions/playerCaptionsTracklistRenderer/captionTracks")?
        .as_array()?;
    tracks
        .iter()
        .filter_map(|track| {
            let language_code = track.get("languageCode")?.as_str()?.trim();
            if !language_code.eq_ignore_ascii_case("en")
                && !language_code.to_ascii_lowercase().starts_with("en-")
            {
                return None;
            }
            let base_url = track.get("baseUrl")?.as_str()?.trim();
            if base_url.is_empty() {
                return None;
            }
            let source_kind = if track.get("kind").and_then(Value::as_str) == Some("asr") {
                CaptionSourceKind::Auto
            } else {
                CaptionSourceKind::Manual
            };
            Some(CaptionTrack {
                base_url: base_url.to_string(),
                language_code: language_code.to_string(),
                source_kind,
            })
        })
        .min_by_key(|track| {
            (
                track.source_kind.priority(),
                u8::from(!track.language_code.eq_ignore_ascii_case("en")),
            )
        })
}

fn valid_video_id(id: &str) -> bool {
    id.len() == 11
        && id
            .bytes()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == b'-' || ch == b'_')
}

pub fn parse_json3_segments(raw: &str) -> anyhow::Result<Vec<CaptionSegment>> {
    let root: Json3Root = serde_json::from_str(raw)?;
    let mut result = Vec::new();
    for (source_event_index, event) in root.events.into_iter().enumerate() {
        let pieces: Vec<CaptionPiece> = event
            .segs
            .into_iter()
            .filter_map(|part| {
                part.utf8.map(|text| CaptionPiece {
                    text,
                    offset_ms: part.offset_ms,
                })
            })
            .collect();
        let joined = pieces
            .iter()
            .map(|part| part.text.as_str())
            .collect::<String>();
        let text = joined.split_whitespace().collect::<Vec<_>>().join(" ");
        if text.is_empty() {
            continue;
        }
        ensure!(
            event.start_ms.unwrap_or(0) <= i64::MAX as u64
                && event.duration_ms.unwrap_or(0) <= i64::MAX as u64,
            "Caption Zeit überschreitet den Datenbankbereich."
        );
        let end_ms = match (event.start_ms, event.duration_ms) {
            (Some(start), Some(duration)) => {
                Some(start.checked_add(duration).context("Caption Zeitüberlauf.")?)
            }
            _ => None,
        };
        ensure!(
            end_ms.unwrap_or(0) <= i64::MAX as u64,
            "Caption Ende überschreitet den Datenbankbereich."
        );
        result.push(CaptionSegment {
            source_event_index,
            start_ms: event.start_ms,
            duration_ms: event.duration_ms,
            end_ms,
            text,
            pieces,
        });
    }
    ensure!(!result.is_empty(), "JSON3 Caption enthält keinen Text.");
    Ok(result)
}

async fn save_transcript(
    pool: &PgPool,
    video_id: &str,
    caption: &CaptionData,
) -> anyhow::Result<SaveOutcome> {
    let text_hash = stable_hash_text(&caption.transcript_text);
    let raw_hash = stable_hash_text(&caption.raw_json);
    let source_kind = caption.source_kind.as_db_value();
    let segments = serde_json::to_string(&caption.segments)?;
    let mut tx = pool.begin().await?;
    let previous_raw: Option<String> = sqlx::query_scalar(
        "SELECT metadata->>'transcript_evidence_hash' FROM brain.youtube_videos WHERE video_id=$1 FOR UPDATE",
    )
    .bind(video_id)
    .fetch_one(&mut *tx)
    .await?;
    let existing: Option<(Option<String>, String, String)> = sqlx::query_as(
        "SELECT language, source_kind, content_hash FROM brain.youtube_transcripts WHERE video_id=$1",
    )
    .bind(video_id)
    .fetch_optional(&mut *tx)
    .await?;
    let outcome = match existing.as_ref() {
        None => SaveOutcome::Inserted,
        Some((language, kind, hash))
            if language.as_deref() == Some("en")
                && kind == source_kind
                && hash == &text_hash
                && previous_raw.as_deref() == Some(raw_hash.as_str()) =>
        {
            SaveOutcome::Unchanged
        }
        Some(_) => SaveOutcome::Updated,
    };
    if outcome != SaveOutcome::Unchanged {
        sqlx::query("INSERT INTO brain.youtube_transcripts(video_id,language,source_kind,transcript_text,content_hash,source_document_id,imported_at,updated_at) VALUES($1,'en',$2,$3,$4,NULL,now(),now()) ON CONFLICT(video_id) DO UPDATE SET language=EXCLUDED.language,source_kind=EXCLUDED.source_kind,transcript_text=EXCLUDED.transcript_text,content_hash=EXCLUDED.content_hash,source_document_id=NULL,updated_at=now()")
            .bind(video_id)
            .bind(source_kind)
            .bind(&caption.transcript_text)
            .bind(&text_hash)
            .execute(&mut *tx)
            .await?;
    }
    sqlx::query("INSERT INTO brain.youtube_transcript_evidence(video_id,raw_sha256,text_sha256,language,source_kind,raw_caption_json,segments) VALUES($1,$2,$3,'en',$4,$5,$6::text::jsonb) ON CONFLICT(video_id,source_kind,raw_sha256) DO NOTHING")
        .bind(video_id)
        .bind(&raw_hash)
        .bind(&text_hash)
        .bind(source_kind)
        .bind(&caption.raw_json)
        .bind(&segments)
        .execute(&mut *tx)
        .await?;
    let revalidate = existing.is_some() && outcome != SaveOutcome::Unchanged;
    sqlx::query("UPDATE brain.youtube_videos SET transcript_status='ready', metadata=(COALESCE(metadata,'{}'::jsonb)-'needs_asr') || jsonb_build_object('transcript_evidence_hash',$2::text) || CASE WHEN $3 THEN '{\"needs_claim_revalidation\":true}'::jsonb ELSE '{}'::jsonb END, updated_at=now() WHERE video_id=$1")
        .bind(video_id)
        .bind(&raw_hash)
        .bind(revalidate)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(outcome)
}

fn stable_hash_text(content: &str) -> String {
    hex::encode(Sha256::digest(content.as_bytes()))
}

fn caption_timeout_seconds() -> u64 {
    std::env::var("YOUTUBE_CAPTION_TIMEOUT_SECONDS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(DEFAULT_CAPTION_TIMEOUT_SECONDS)
        .clamp(1, 120)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_timing_and_word_offsets() {
        let rows = parse_json3_segments(
            r#"{"events":[{"tStartMs":1200,"dDurationMs":3500,"segs":[{"utf8":"Unstable "},{"utf8":"Rift","tOffsetMs":200}]}]}"#,
        )
        .unwrap();
        assert_eq!(rows[0].start_ms, Some(1200));
        assert_eq!(rows[0].end_ms, Some(4700));
        assert_eq!(rows[0].pieces[1].offset_ms, Some(200));
        assert_eq!(rows[0].text, "Unstable Rift");
    }

    #[test]
    fn missing_times_remain_unknown() {
        let rows =
            parse_json3_segments(r#"{"events":[{"segs":[{"utf8":"unknown time"}]}]}"#)
                .unwrap();
        assert_eq!(rows[0].start_ms, None);
        assert_eq!(rows[0].end_ms, None);
    }

    #[test]
    fn preserves_repeated_speech_and_source_indices() {
        let rows = parse_json3_segments(
            r#"{"events":[{}, {"tStartMs":0,"segs":[{"utf8":"again"}]},{"tStartMs":5000,"segs":[{"utf8":"again"}]}]}"#,
        )
        .unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].source_event_index, 1);
        assert_eq!(rows[1].start_ms, Some(5000));
    }

    #[test]
    fn concatenates_unicode_fragments_without_inventing_spaces() {
        let rows = parse_json3_segments(
            r#"{"events":[{"segs":[{"utf8":"Än"},{"utf8":"derung  ✓\n"}]}]}"#,
        )
        .unwrap();
        assert_eq!(rows[0].text, "Änderung ✓");
    }

    #[test]
    fn rejects_empty_negative_and_overflow_times() {
        assert!(parse_json3_segments(r#"{"events":[]}"#).is_err());
        assert!(parse_json3_segments(
            r#"{"events":[{"tStartMs":-1,"segs":[{"utf8":"x"}]}]}"#
        )
        .is_err());
        assert!(parse_json3_segments(
            r#"{"events":[{"tStartMs":18446744073709551615,"dDurationMs":1,"segs":[{"utf8":"x"}]}]}"#
        )
        .is_err());
    }

    #[test]
    fn timing_only_changes_have_different_evidence_hashes() {
        let first = r#"{"events":[{"tStartMs":100,"segs":[{"utf8":"same"}]}]}"#;
        let second = first.replace("100", "200");
        assert_ne!(stable_hash_text(first), stable_hash_text(&second));
        assert_eq!(
            parse_json3_segments(first).unwrap()[0].text,
            parse_json3_segments(&second).unwrap()[0].text
        );
    }

    #[test]
    fn player_response_parser_handles_nested_strings() {
        let html = r#"<script>var ytInitialPlayerResponse = {"captions":{"playerCaptionsTracklistRenderer":{"captionTracks":[{"baseUrl":"https://www.youtube.com/api/timedtext?v=abc&x={ok}","languageCode":"en","name":{"simpleText":"English"}},{"baseUrl":"https://www.youtube.com/api/timedtext?v=abc&kind=asr","languageCode":"en","kind":"asr"}]}}};</script>"#;
        let player = extract_initial_player_response(html).unwrap();
        let track = select_english_caption_track(&player).unwrap();
        assert_eq!(track.source_kind, CaptionSourceKind::Manual);
        assert_eq!(track.language_code, "en");
        assert!(track.base_url.contains("timedtext"));
    }

    #[test]
    fn caption_selection_rejects_non_english_and_prefers_manual() {
        let player = serde_json::json!({
            "captions":{"playerCaptionsTracklistRenderer":{"captionTracks":[
                {"baseUrl":"https://www.youtube.com/api/timedtext?v=a","languageCode":"de"},
                {"baseUrl":"https://www.youtube.com/api/timedtext?v=b","languageCode":"en-US","kind":"asr"},
                {"baseUrl":"https://www.youtube.com/api/timedtext?v=c","languageCode":"en"}
            ]}}
        });
        let track = select_english_caption_track(&player).unwrap();
        assert_eq!(track.base_url, "https://www.youtube.com/api/timedtext?v=c");
        assert_eq!(track.source_kind, CaptionSourceKind::Manual);
    }

    #[test]
    fn downloader_uses_validated_id_not_database_url() {
        assert!(valid_video_id("ZWm7ixeWjbQ"));
        for bad in ["../../test", "http://host", "--exec=echo", "abc"] {
            assert!(!valid_video_id(bad));
        }
    }

    #[test]
    fn parses_json3_into_normalized_flow_text() {
        let rows = parse_json3_segments(
            r#"{"events":[{"segs":[{"utf8":"Hello"},{"utf8":"  world\n"}]},{"segs":[{"utf8":"from"},{"utf8":" captions"}]},{"segs":[{"utf8":"\n"}]}]}"#,
        )
        .unwrap();
        let text = rows
            .iter()
            .map(|row| row.text.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        assert_eq!(text, "Hello world from captions");
    }

    #[tokio::test]
    #[ignore = "needs scratch Postgres and 2026-09-18-patch-evidence.sql"]
    async fn save_transcript_is_idempotent_and_sets_ready_pg() {
        let Some(pool) = crate::testutil::test_pool().await else {
            return;
        };
        let suffix = crate::testutil::unique_suffix();
        let feed = format!("ztest_feed_{suffix}");
        let video = format!("ztest_tr_{suffix}");
        crate::testutil::seed_feed(&pool, &feed).await;
        crate::testutil::seed_video(
            &pool,
            &video,
            &feed,
            "missing",
            "queued",
            Some("2026-06-01T00:00:00Z"),
            r#"{"needs_asr":true}"#,
        )
        .await;
        let raw_json = r#"{"events":[{"tStartMs":1234,"dDurationMs":500,"segs":[{"utf8":"one two three"}]}]}"#.to_string();
        let caption = CaptionData {
            segments: parse_json3_segments(&raw_json).unwrap(),
            raw_json,
            source_kind: CaptionSourceKind::Manual,
            transcript_text: "one two three".to_string(),
        };
        assert_eq!(
            save_transcript(&pool, &video, &caption).await.unwrap(),
            SaveOutcome::Inserted
        );
        assert_eq!(
            save_transcript(&pool, &video, &caption).await.unwrap(),
            SaveOutcome::Unchanged
        );
        let (status, needs_asr): (String, bool) = sqlx::query_as(
            "SELECT transcript_status, (metadata->>'needs_asr') IS NOT NULL FROM brain.youtube_videos WHERE video_id=$1",
        )
        .bind(&video)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(status, "ready");
        assert!(!needs_asr);
        let mut retimed = caption.clone();
        retimed.raw_json = retimed.raw_json.replace("1234", "2345");
        retimed.segments = parse_json3_segments(&retimed.raw_json).unwrap();
        assert_eq!(
            save_transcript(&pool, &video, &retimed).await.unwrap(),
            SaveOutcome::Updated
        );
        let count: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM brain.youtube_transcript_evidence WHERE video_id=$1",
        )
        .bind(&video)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(count, 2);
        crate::testutil::cleanup(&pool, &[&video], &[&feed]).await;
    }
}
