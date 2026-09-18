use std::{fs, io::{Read, Seek, SeekFrom}, path::{Path, PathBuf}, process::{Command, Stdio}, time::Duration};

use serde::Serialize;
use sha2::{Digest, Sha256};
use sqlx::{postgres::PgPool, Row};
use wait_timeout::ChildExt;

#[path = "caption_evidence.rs"]
mod caption_evidence;

use caption_evidence::{CaptionEvidence, PARSER_VERSION};

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
struct VideoForTranscript { video_id: String, title: String, url: String }

#[derive(Debug, Clone)]
struct CaptionData { source_kind: CaptionSourceKind, language: String, evidence: CaptionEvidence }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CaptionSourceKind { Manual, Auto }

impl CaptionSourceKind {
    fn as_db_value(self) -> &'static str {
        match self { Self::Manual => "youtube_caption_manual", Self::Auto => "youtube_caption_auto" }
    }
    fn priority(self) -> u8 { match self { Self::Manual => 0, Self::Auto => 1 } }
}

#[derive(Debug)]
struct CaptionFile { path: PathBuf, language: String, source_kind: CaptionSourceKind }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SaveOutcome { Inserted, Updated, Unchanged }

pub async fn fetch_transcripts(pool: &PgPool, limit: usize) -> anyhow::Result<FetchTranscriptsSummary> {
    let unavailable_marked = backfill_unavailable_needs_asr(pool).await?;
    let videos = select_videos_missing_transcripts(pool, limit).await?;
    let mut summary = FetchTranscriptsSummary { selected:videos.len(),processed:0,saved:0,unchanged:0,
        unavailable:0,unavailable_marked,failed:0,videos:Vec::new() };
    for video in videos {
        match fetch_caption_for_video(&video) {
            Ok(Some(caption)) => {
                let outcome = save_transcript(pool, &video.video_id, &caption).await?;
                match outcome { SaveOutcome::Inserted | SaveOutcome::Updated => summary.saved += 1,
                    SaveOutcome::Unchanged => summary.unchanged += 1 }
                summary.processed += 1;
                summary.videos.push(FetchVideoSummary {video_id:video.video_id,title:video.title,
                    status:match outcome {SaveOutcome::Inserted=>"inserted",SaveOutcome::Updated=>"updated",SaveOutcome::Unchanged=>"unchanged"}.into(),
                    language:Some(caption.language),source_kind:Some(caption.source_kind.as_db_value().into()),
                    transcript_chars:caption.evidence.text.chars().count(),error:None});
            }
            Ok(None) => {
                mark_transcript_unavailable(pool,&video.video_id).await?;
                summary.processed += 1; summary.unavailable += 1;
                summary.videos.push(FetchVideoSummary {video_id:video.video_id,title:video.title,
                    status:"unavailable".into(),language:None,source_kind:None,transcript_chars:0,error:None});
            }
            Err(error) => {
                summary.processed += 1; summary.failed += 1;
                summary.videos.push(FetchVideoSummary {video_id:video.video_id,title:video.title,
                    status:"failed".into(),language:None,source_kind:None,transcript_chars:0,error:Some(error.to_string())});
            }
        }
    }
    Ok(summary)
}

async fn select_videos_missing_transcripts(pool: &PgPool, limit: usize) -> anyhow::Result<Vec<VideoForTranscript>> {
    let limit = i64::try_from(limit.max(1)).unwrap_or(i64::MAX);
    let rows = sqlx::query("SELECT v.video_id,v.title,v.url FROM brain.youtube_videos v WHERE v.transcript_status='missing' OR (v.transcript_status <> 'unavailable' AND NOT EXISTS (SELECT 1 FROM brain.youtube_transcripts t WHERE t.video_id=v.video_id AND LENGTH(TRIM(t.transcript_text)) > 0)) ORDER BY v.published_at DESC NULLS LAST,v.discovered_at DESC LIMIT $1")
        .bind(limit).fetch_all(pool).await?;
    rows.into_iter().map(|row| Ok(VideoForTranscript {video_id:row.try_get("video_id")?,title:row.try_get("title")?,url:row.try_get("url")?})).collect()
}

fn fetch_caption_for_video(video: &VideoForTranscript) -> anyhow::Result<Option<CaptionData>> {
    anyhow::ensure!(!video.video_id.is_empty() && video.video_id.len() <= 128 && video.video_id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-'), "Ungültige Video-ID.");
    anyhow::ensure!(video.url.starts_with("https://www.youtube.com/watch?") || video.url.starts_with("https://youtube.com/watch?") || video.url.starts_with("https://youtu.be/"), "Keine unterstützte öffentliche YouTube-URL.");
    let temp_dir = tempfile::tempdir()?;
    let output_template = temp_dir.path().join(format!("{}.%(ext)s",video.video_id));
    let mut log = tempfile::tempfile()?;
    let mut child = Command::new(yt_dlp_bin())
        .args(["--ignore-config","--no-update","--no-warnings","--no-progress","--no-playlist",
            "--skip-download","--write-info-json","--write-subs","--write-auto-subs",
            "--sub-langs","en,en-orig,de,de-orig","--sub-format","json3","-o"])
        .arg(&output_template).arg("--").arg(&video.url)
        .stdout(Stdio::from(log.try_clone()?)).stderr(Stdio::from(log.try_clone()?)).spawn()?;
    let timeout = Duration::from_secs(yt_dlp_timeout_seconds());
    let status = match child.wait_timeout(timeout) {
        Ok(Some(status)) => status,
        Ok(None) => { let _=child.kill(); let _=child.wait(); anyhow::bail!("yt-dlp timeout after {}s",timeout.as_secs()); }
        Err(error) => { let _=child.kill(); let _=child.wait(); return Err(error.into()); }
    };
    log.seek(SeekFrom::Start(0))?;
    let mut output = String::new();
    log.take(128 * 1024).read_to_string(&mut output)?;
    let file = select_caption_file(temp_dir.path(), &video.video_id, &output)?;
    if let Some(file) = file {
        anyhow::ensure!(fs::metadata(&file.path)?.len() <= 20_000_000,"Untertiteldatei ist zu groß.");
        return Ok(Some(CaptionData {language:file.language.trim_end_matches("-orig").into(),
            source_kind:file.source_kind,evidence:caption_evidence::parse(&fs::read_to_string(file.path)?)?}));
    }
    if status.success() || looks_like_no_captions(&output) { return Ok(None); }
    anyhow::bail!("yt-dlp failed with status {status}; keine Untertitel gespeichert")
}

fn select_caption_file(dir: &Path, video_id: &str, output: &str) -> anyhow::Result<Option<CaptionFile>> {
    let metadata = fs::read_to_string(dir.join(format!("{video_id}.info.json"))).ok()
        .and_then(|text| serde_json::from_str::<serde_json::Value>(&text).ok());
    let mut candidates = Vec::new();
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        let Some(name)=path.file_name().and_then(|v| v.to_str()) else {continue};
        let Some(language)=caption_language_from_file_name(name,video_id) else {continue};
        if !matches!(language.as_str(),"en"|"en-orig"|"de"|"de-orig") {continue;}
        let source_kind = if let Some(info)=&metadata {
            if info.get("subtitles").and_then(|tracks| tracks.get(&language)).and_then(|tracks| tracks.as_array()).is_some_and(|tracks| !tracks.is_empty()) {
                CaptionSourceKind::Manual
            } else {
                let native=info.get("language").and_then(|v| v.as_str()).map(|v|v.split('-').next().unwrap_or(v));
                let language_base=language.trim_end_matches("-orig");
                if !language.ends_with("-orig") && native != Some(language_base) {continue;}
                let translated=info.get("automatic_captions").and_then(|tracks|tracks.get(&language)).and_then(|v|v.as_array())
                    .is_some_and(|tracks|tracks.iter().any(|track|track.get("url").and_then(|v|v.as_str()).is_some_and(|url|url.contains("&tlang=")||url.contains("?tlang="))));
                if translated {continue;}
                CaptionSourceKind::Auto
            }
        } else {
            if !matches!(language.as_str(),"en"|"en-orig") {continue;}
            infer_source_kind(output,name)
        };
        candidates.push(CaptionFile {path,language,source_kind});
    }
    candidates.sort_by(|a,b| (a.source_kind.priority(),language_priority(&a.language),a.path.as_os_str())
        .cmp(&(b.source_kind.priority(),language_priority(&b.language),b.path.as_os_str())));
    Ok(candidates.into_iter().next())
}

fn caption_language_from_file_name(name: &str, video_id: &str) -> Option<String> {
    let language=name.strip_prefix(&format!("{video_id}."))?.strip_suffix(".json3")?;
    (!language.trim().is_empty()).then(|| language.into())
}

fn infer_source_kind(output: &str, file_name: &str) -> CaptionSourceKind {
    let mut manual=false;
    for line in output.lines().filter(|line|line.contains(file_name)) {
        let lower=line.to_ascii_lowercase();
        if lower.contains("automatic subtitles") {return CaptionSourceKind::Auto;}
        if lower.contains("subtitles") {manual=true;}
    }
    if manual {CaptionSourceKind::Manual} else {CaptionSourceKind::Auto}
}

fn language_priority(language: &str) -> u8 {
    match language {"en"=>0,"en-orig"=>1,"de"=>2,"de-orig"=>3,_=>4}
}

fn looks_like_no_captions(output: &str) -> bool {
    let lower=output.to_ascii_lowercase();
    ["has no subtitles","has no automatic captions","no automatic captions","there are no subtitles","no subtitles for the requested languages"].iter().any(|message|lower.contains(message))
}

pub fn parse_json3_transcript(raw: &str) -> anyhow::Result<String> { Ok(caption_evidence::parse(raw)?.text) }

async fn save_transcript(pool: &PgPool, video_id: &str, caption: &CaptionData) -> anyhow::Result<SaveOutcome> {
    let text=&caption.evidence.text;
    let source_kind=caption.source_kind.as_db_value();
    let content_hash=hex::encode(Sha256::digest(text.as_bytes()));
    let evidence_hash=caption_evidence::evidence_hash(&caption.evidence,&caption.language,source_kind)?;
    let mut tx=pool.begin().await?;
    sqlx::query("SELECT video_id FROM brain.youtube_videos WHERE video_id=$1 FOR UPDATE").bind(video_id).fetch_one(&mut *tx).await?;
    let existing=sqlx::query("SELECT language,source_kind,content_hash FROM brain.youtube_transcripts WHERE video_id=$1")
        .bind(video_id).fetch_optional(&mut *tx).await?;
    let mut outcome=match existing {
        Some(row) if row.try_get::<Option<String>,_>("language")?.as_deref()==Some(caption.language.as_str())
            && row.try_get::<String,_>("source_kind")?==source_kind && row.try_get::<String,_>("content_hash")?==content_hash => SaveOutcome::Unchanged,
        Some(_)=>SaveOutcome::Updated, None=>SaveOutcome::Inserted,
    };
    if outcome != SaveOutcome::Unchanged {
        sqlx::query("INSERT INTO brain.youtube_transcripts(video_id,language,source_kind,transcript_text,content_hash,source_document_id,imported_at,updated_at) VALUES($1,$2,$3,$4,$5,NULL,now(),now()) ON CONFLICT(video_id) DO UPDATE SET language=EXCLUDED.language,source_kind=EXCLUDED.source_kind,transcript_text=EXCLUDED.transcript_text,content_hash=EXCLUDED.content_hash,source_document_id=NULL,updated_at=now()")
            .bind(video_id).bind(&caption.language).bind(source_kind).bind(text).bind(&content_hash).execute(&mut *tx).await?;
    }
    let inserted=sqlx::query("INSERT INTO brain.youtube_transcript_evidence(video_id,evidence_hash,transcript_hash,language,source_kind,raw_caption,segments,timing_status,parser_version) VALUES($1,$2,$3,$4,$5,$6::text::jsonb,$7::text::jsonb,$8,$9) ON CONFLICT(video_id,evidence_hash) DO NOTHING")
        .bind(video_id).bind(&evidence_hash).bind(&content_hash).bind(&caption.language).bind(source_kind)
        .bind(serde_json::to_string(&caption.evidence.raw)?).bind(serde_json::to_string(&caption.evidence.segments)?)
        .bind(caption.evidence.timing_status).bind(PARSER_VERSION).execute(&mut *tx).await?.rows_affected();
    if outcome==SaveOutcome::Unchanged && inserted>0 {outcome=SaveOutcome::Updated;}
    sqlx::query("UPDATE brain.youtube_videos SET transcript_status='ready',metadata=coalesce(metadata,'{}'::jsonb)-'needs_asr',updated_at=now() WHERE video_id=$1")
        .bind(video_id).execute(&mut *tx).await?;
    tx.commit().await?;
    Ok(outcome)
}

async fn mark_transcript_unavailable(pool: &PgPool, video_id: &str) -> anyhow::Result<()> {
    sqlx::query("UPDATE brain.youtube_videos SET transcript_status='unavailable',metadata=jsonb_set(coalesce(metadata,'{}'::jsonb),'{needs_asr}','true'::jsonb),updated_at=now() WHERE video_id=$1")
        .bind(video_id).execute(pool).await?;
    Ok(())
}

async fn backfill_unavailable_needs_asr(pool: &PgPool) -> anyhow::Result<usize> {
    let changed=sqlx::query("UPDATE brain.youtube_videos SET metadata=jsonb_set(coalesce(metadata,'{}'::jsonb),'{needs_asr}','true'::jsonb),updated_at=now() WHERE transcript_status='unavailable' AND coalesce(metadata->>'needs_asr','false') <> 'true'")
        .execute(pool).await?.rows_affected();
    Ok(usize::try_from(changed)?)
}

fn yt_dlp_bin() -> String {std::env::var("YT_DLP_BIN").unwrap_or_else(|_|"yt-dlp".into())}
fn yt_dlp_timeout_seconds() -> u64 {
    std::env::var("YT_DLP_TIMEOUT_SECONDS").ok().and_then(|v|v.parse().ok())
        .unwrap_or(DEFAULT_YT_DLP_TIMEOUT_SECONDS).clamp(1,300)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_json3_into_normalized_flow_text() {
        let parsed=parse_json3_transcript(r#"{"events":[{"segs":[{"utf8":"Hello"},{"utf8":"  world\n"}]},{"segs":[{"utf8":"from"},{"utf8":" captions"}]},{"segs":[{"utf8":"\n"}]}]}"#).unwrap();
        assert_eq!(parsed,"Hello world from captions");
    }
    #[test]
    fn caption_selection_prefers_manual_english_and_rejects_translations() {
        let temp=tempfile::tempdir().unwrap();
        for lang in ["en","en-orig","de"] {fs::write(temp.path().join(format!("abc.{lang}.json3")),"{}").unwrap();}
        let output="[info] Writing video automatic subtitles to: abc.en-orig.json3\n[info] Writing video subtitles to: abc.en.json3";
        let selected=select_caption_file(temp.path(),"abc",output).unwrap().unwrap();
        assert_eq!(selected.language,"en");
        assert_eq!(selected.source_kind,CaptionSourceKind::Manual);
    }
    #[test]
    fn german_native_captions_are_not_mislabeled_as_english() {
        let temp=tempfile::tempdir().unwrap();
        fs::write(temp.path().join("abc.de-orig.json3"),"{}").unwrap();
        fs::write(temp.path().join("abc.en.json3"),"{}").unwrap();
        fs::write(temp.path().join("abc.info.json"),r#"{"language":"de","subtitles":{},"automatic_captions":{"de-orig":[{"url":"https://example.test/?lang=de"}],"en":[{"url":"https://example.test/?lang=de&tlang=en"}]}}"#).unwrap();
        let selected=select_caption_file(temp.path(),"abc","").unwrap().unwrap();
        assert_eq!(selected.language,"de-orig");
        assert_eq!(selected.source_kind,CaptionSourceKind::Auto);
    }
    #[tokio::test]
    #[ignore = "needs scratch Postgres and the 2026-09-18 migration"]
    async fn save_transcript_is_idempotent_and_sets_ready_pg() {
        let Some(pool)=crate::testutil::test_pool().await else {return};
        let suffix=crate::testutil::unique_suffix();
        let feed_key=format!("ztest_feed_{suffix}");
        let video_id=format!("ztest_tr_{suffix}");
        crate::testutil::seed_feed(&pool,&feed_key).await;
        crate::testutil::seed_video(&pool,&video_id,&feed_key,"missing","queued",Some("2026-06-01T00:00:00Z"),r#"{"needs_asr":true}"#).await;
        let caption=CaptionData{source_kind:CaptionSourceKind::Manual,language:"en".into(),
            evidence:caption_evidence::parse(r#"{"events":[{"tStartMs":0,"dDurationMs":1000,"segs":[{"utf8":"one two three"}]}]}"#).unwrap()};
        assert_eq!(save_transcript(&pool,&video_id,&caption).await.unwrap(),SaveOutcome::Inserted);
        assert_eq!(save_transcript(&pool,&video_id,&caption).await.unwrap(),SaveOutcome::Unchanged);
        let status:String=sqlx::query_scalar("SELECT transcript_status FROM brain.youtube_videos WHERE video_id=$1").bind(&video_id).fetch_one(&pool).await.unwrap();
        assert_eq!(status,"ready");
        let present:bool=sqlx::query_scalar("SELECT (metadata->>'needs_asr') IS NOT NULL FROM brain.youtube_videos WHERE video_id=$1").bind(&video_id).fetch_one(&pool).await.unwrap();
        assert!(!present);
        let count:i64=sqlx::query_scalar("SELECT count(*) FROM brain.youtube_transcript_evidence WHERE video_id=$1").bind(&video_id).fetch_one(&pool).await.unwrap();
        assert_eq!(count,1);
        crate::testutil::cleanup(&pool,&[&video_id],&[&feed_key]).await;
    }
}
