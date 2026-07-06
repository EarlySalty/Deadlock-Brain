use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fmt, fs,
    path::{Path, PathBuf},
};

use anyhow::Context;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::{postgres::PgPool, Row};

use crate::db;

pub const PROMPT_VERSION: &str = "youtube_claims_de_transcript_v1";
pub const PROMPT_TEXT: &str =
    "Transcript-basierte Claim-Extraktion (Claude) + DB-Verifikation gegen deadlock_data/patch_events.";
pub const MONSTER_CHAR_THRESHOLD: usize = 150_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum PrepareOrder {
    Recent,
    Shortest,
}

impl PrepareOrder {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Recent => "recent",
            Self::Shortest => "shortest",
        }
    }

    /// Konstante, hartkodierte `ORDER BY`-Klausel (keine Nutzer-Eingabe, daher
    /// interpolierbar).
    fn order_sql(self) -> &'static str {
        match self {
            Self::Recent => "v.published_at DESC NULLS LAST, v.video_id ASC",
            Self::Shortest => "length(t.transcript_text) ASC, v.video_id ASC",
        }
    }
}

impl fmt::Display for PrepareOrder {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum PrepareMode {
    Normal,
    Monster,
}

impl PrepareMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Monster => "monster",
        }
    }
}

impl fmt::Display for PrepareMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, Serialize)]
pub struct PrepareSummary {
    pub generated_at: i64,
    pub prompt_version: String,
    pub order: String,
    pub count: usize,
    pub videos: Vec<PrepareVideo>,
}

#[derive(Debug, Serialize)]
pub struct PrepareVideo {
    pub video_id: String,
    pub title: String,
    pub url: String,
    pub channel_title: Option<String>,
    pub char_len: usize,
    pub transcript_text: String,
}

#[derive(Debug, Serialize)]
pub struct MonsterPrepareSummary {
    pub generated_at: i64,
    pub prompt_version: String,
    pub mode: String,
    pub chunk_chars: usize,
    pub overlap_chars: usize,
    pub min_chars: usize,
    pub count: usize,
    pub videos: Vec<MonsterPrepareVideo>,
}

#[derive(Debug, Serialize)]
pub struct MonsterPrepareVideo {
    pub video_id: String,
    pub title: String,
    pub url: String,
    pub channel_title: Option<String>,
    pub char_len: usize,
    pub chunk_count: usize,
    pub chunks: Vec<TranscriptChunk>,
}

#[derive(Debug, Serialize)]
pub struct TranscriptChunk {
    pub chunk_index: usize,
    pub char_start: usize,
    pub char_end: usize,
    pub char_len: usize,
    pub text: String,
}

#[derive(Debug)]
pub struct IngestOptions {
    pub write: bool,
    /// Nach dem PG-Cutover wirkungslos: das SQLite-Datei-Backup entfällt, die
    /// zentrale Postgres wird zentral gesichert. Flag bleibt für CLI-Stabilität.
    #[allow(dead_code)]
    pub no_backup: bool,
    pub model: String,
    pub prompt_version: String,
}

#[derive(Debug, Serialize)]
pub struct IngestSummary {
    pub dry_run: bool,
    pub backup_path: Option<String>,
    pub inserted: usize,
    pub skipped_existing: usize,
    pub attempts_recorded: usize,
    pub errors: Vec<String>,
    pub status_breakdown: BTreeMap<String, usize>,
    pub total_claims_in_table_after: usize,
}

#[derive(Debug)]
pub struct BackfillAttemptsOptions {
    pub write: bool,
    /// Wirkungslos nach PG-Cutover (siehe [`IngestOptions::no_backup`]).
    #[allow(dead_code)]
    pub no_backup: bool,
    pub prompt_version: String,
    pub max_chars: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct BackfillAttemptsSummary {
    pub dry_run: bool,
    pub would_mark: usize,
    pub marked: usize,
    pub backup_path: Option<String>,
}

#[derive(Debug)]
pub struct MarkOfftopicOptions {
    pub write: bool,
    /// Wirkungslos nach PG-Cutover (siehe [`IngestOptions::no_backup`]).
    #[allow(dead_code)]
    pub no_backup: bool,
    pub prompt_version: String,
    pub title_contains: Vec<String>,
    pub video_ids_path: Option<PathBuf>,
}

#[derive(Debug, Serialize)]
pub struct MarkOfftopicSummary {
    pub dry_run: bool,
    pub would_mark: usize,
    pub marked: usize,
    pub matched_titles: Vec<MarkOfftopicTitleMatch>,
    pub backup_path: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MarkOfftopicTitleMatch {
    pub video_id: String,
    pub title: String,
    pub char_len: usize,
    pub matched_by_title_contains: Vec<String>,
    pub matched_by_video_ids: bool,
}

pub async fn prepare(
    pool: &PgPool,
    limit: usize,
    order: PrepareOrder,
    max_chars: Option<usize>,
) -> anyhow::Result<PrepareSummary> {
    let max_chars = max_chars
        .map(|value| usize_to_i64(value, "prepare max chars"))
        .transpose()?;
    let limit = usize_to_i64(limit, "prepare limit")?;
    let sql = format!(
        r#"
        SELECT v.video_id, v.title, v.url, v.channel_title, t.transcript_text
        FROM brain.youtube_videos v JOIN brain.youtube_transcripts t ON t.video_id=v.video_id
        WHERE v.metadata->>'content_type'='verbal_strategy'
          AND NOT EXISTS (
            SELECT 1 FROM brain.youtube_learning_claims c
            WHERE c.video_id=v.video_id AND c.prompt_version=$1
          )
          AND NOT EXISTS (
            SELECT 1 FROM brain.youtube_transcript_claim_attempts a
            WHERE a.video_id=v.video_id AND a.prompt_version=$1
          )
          AND ($2::bigint IS NULL OR length(t.transcript_text) <= $2)
        ORDER BY {order_sql}
        LIMIT $3
        "#,
        order_sql = order.order_sql()
    );
    let rows = sqlx::query(&sql)
        .bind(PROMPT_VERSION)
        .bind(max_chars)
        .bind(limit)
        .fetch_all(pool)
        .await?;

    let videos = rows
        .iter()
        .map(|row| {
            let transcript_text: String = row.try_get("transcript_text")?;
            Ok(PrepareVideo {
                video_id: row.try_get("video_id")?,
                title: row.try_get("title")?,
                url: row.try_get("url")?,
                channel_title: row.try_get("channel_title")?,
                char_len: transcript_text.chars().count(),
                transcript_text,
            })
        })
        .collect::<Result<Vec<_>, sqlx::Error>>()?;

    Ok(PrepareSummary {
        generated_at: db::now_epoch_seconds(),
        prompt_version: PROMPT_VERSION.to_string(),
        order: order.as_str().to_string(),
        count: videos.len(),
        videos,
    })
}

pub async fn prepare_monster(
    pool: &PgPool,
    limit: usize,
    order: PrepareOrder,
    min_chars: usize,
    chunk_chars: usize,
    overlap_chars: usize,
) -> anyhow::Result<MonsterPrepareSummary> {
    validate_chunk_options(chunk_chars, overlap_chars)?;

    let limit = usize_to_i64(limit, "monster prepare limit")?;
    let min_chars_i64 = if min_chars == 0 {
        None
    } else {
        Some(usize_to_i64(min_chars, "monster prepare min chars")?)
    };

    let sql = format!(
        r#"
        SELECT v.video_id, v.title, v.url, v.channel_title, t.transcript_text
        FROM brain.youtube_videos v JOIN brain.youtube_transcripts t ON t.video_id=v.video_id
        WHERE v.metadata->>'content_type'='verbal_strategy'
          AND length(t.transcript_text) > 0
          AND NOT EXISTS (
            SELECT 1 FROM brain.youtube_learning_claims c
            WHERE c.video_id=v.video_id AND c.prompt_version=$1
          )
          AND NOT EXISTS (
            SELECT 1 FROM brain.youtube_transcript_claim_attempts a
            WHERE a.video_id=v.video_id AND a.prompt_version=$1
          )
          AND ($2::bigint IS NULL OR length(t.transcript_text) > $2)
        ORDER BY {order_sql}
        LIMIT $3
        "#,
        order_sql = order.order_sql()
    );
    let rows = sqlx::query(&sql)
        .bind(PROMPT_VERSION)
        .bind(min_chars_i64)
        .bind(limit)
        .fetch_all(pool)
        .await?;

    let mut videos = Vec::with_capacity(rows.len());
    for row in &rows {
        let transcript_text: String = row.try_get("transcript_text")?;
        let char_len = transcript_text.chars().count();
        let chunks = chunk_transcript_text(&transcript_text, chunk_chars, overlap_chars)?;
        videos.push(MonsterPrepareVideo {
            video_id: row.try_get("video_id")?,
            title: row.try_get("title")?,
            url: row.try_get("url")?,
            channel_title: row.try_get("channel_title")?,
            char_len,
            chunk_count: chunks.len(),
            chunks,
        });
    }

    Ok(MonsterPrepareSummary {
        generated_at: db::now_epoch_seconds(),
        prompt_version: PROMPT_VERSION.to_string(),
        mode: PrepareMode::Monster.as_str().to_string(),
        chunk_chars,
        overlap_chars,
        min_chars,
        count: videos.len(),
        videos,
    })
}

fn validate_chunk_options(chunk_chars: usize, overlap_chars: usize) -> anyhow::Result<()> {
    if overlap_chars >= chunk_chars {
        anyhow::bail!(
            "overlap_chars ({overlap_chars}) must be smaller than chunk_chars ({chunk_chars})"
        );
    }
    Ok(())
}

fn chunk_transcript_text(
    text: &str,
    chunk_chars: usize,
    overlap_chars: usize,
) -> anyhow::Result<Vec<TranscriptChunk>> {
    validate_chunk_options(chunk_chars, overlap_chars)?;

    let mut char_byte_offsets = Vec::with_capacity(text.len().saturating_add(1).min(1_000_000));
    for (byte_offset, _) in text.char_indices() {
        char_byte_offsets.push(byte_offset);
    }
    char_byte_offsets.push(text.len());
    let char_len = char_byte_offsets.len().saturating_sub(1);
    if char_len == 0 {
        return Ok(Vec::new());
    }

    let step = chunk_chars - overlap_chars;
    let mut chunks = Vec::new();
    let mut char_start = 0_usize;
    while char_start < char_len {
        let char_end = char_start.saturating_add(chunk_chars).min(char_len);
        if char_end <= char_start {
            break;
        }
        let byte_start = char_byte_offsets[char_start];
        let byte_end = char_byte_offsets[char_end];
        chunks.push(TranscriptChunk {
            chunk_index: chunks.len(),
            char_start,
            char_end,
            char_len: char_end - char_start,
            text: text[byte_start..byte_end].to_string(),
        });
        if char_end == char_len {
            break;
        }
        char_start = char_start.saturating_add(step);
    }
    Ok(chunks)
}

pub async fn ingest(
    pool: &PgPool,
    input_path: &Path,
    options: IngestOptions,
) -> anyhow::Result<IngestSummary> {
    let raw_input = fs::read_to_string(input_path)
        .with_context(|| format!("read transcript claims input {}", input_path.display()))?;
    let videos: Vec<RawVideoInput> = serde_json::from_str(&raw_input)
        .with_context(|| format!("parse transcript claims input {}", input_path.display()))?;

    let mut tx = pool.begin().await?;
    let mut summary = IngestSummary {
        dry_run: !options.write,
        // SQLite-Datei-Backup nach PG-Cutover entfernt (zentrale Sicherung).
        backup_path: None,
        inserted: 0,
        skipped_existing: 0,
        attempts_recorded: 0,
        errors: Vec::new(),
        status_breakdown: initial_status_breakdown(),
        total_claims_in_table_after: 0,
    };
    let mut next_claim_index_by_video: HashMap<String, i64> = HashMap::new();
    let mut new_hashes_in_batch = HashSet::new();

    for video in &videos {
        let mut valid_claim_count = 0_usize;
        for (claim_position, raw_claim) in video.claims().iter().enumerate() {
            let Some(claim) = validate_claim_record(
                &video.video_id,
                claim_position,
                raw_claim,
                &mut summary.errors,
            ) else {
                continue;
            };
            valid_claim_count += 1;
            let claim_hash =
                transcript_claim_hash(&video.video_id, &claim.claim_text, &claim.evidence_quote);
            if claim_hash_exists(&mut *tx, &claim_hash).await?
                || new_hashes_in_batch.contains(&claim_hash)
            {
                summary.skipped_existing += 1;
                continue;
            }

            let next_claim_index = next_claim_index_by_video
                .entry(video.video_id.clone())
                .or_insert(0);
            let claim_index = *next_claim_index;
            if options.write {
                let inserted =
                    insert_claim(&mut *tx, &video.video_id, &claim_hash, claim_index, &claim, &options)
                        .await?;
                if inserted == 0 {
                    summary.skipped_existing += 1;
                    continue;
                }
            }

            *next_claim_index += 1;
            new_hashes_in_batch.insert(claim_hash);
            summary.inserted += 1;
            increment_status(&mut summary.status_breakdown, &claim.status);
        }

        if options.write {
            let char_len = transcript_char_len(&mut *tx, &video.video_id).await?;
            summary.attempts_recorded += upsert_transcript_claim_attempt(
                &mut *tx,
                &video.video_id,
                &options.prompt_version,
                valid_claim_count,
                char_len,
            )
            .await?;
        }
    }

    summary.total_claims_in_table_after = count_claims(&mut *tx).await?;
    if options.write {
        tx.commit().await?;
    } else {
        tx.rollback().await?;
    }
    Ok(summary)
}

pub async fn backfill_attempts(
    pool: &PgPool,
    options: BackfillAttemptsOptions,
) -> anyhow::Result<BackfillAttemptsSummary> {
    let max_chars = options
        .max_chars
        .map(|value| usize_to_i64(value, "backfill max chars"))
        .transpose()?;
    let would_mark =
        count_backfill_attempt_candidates(pool, &options.prompt_version, max_chars).await?;
    let mut summary = BackfillAttemptsSummary {
        dry_run: !options.write,
        would_mark,
        marked: 0,
        backup_path: None,
    };

    if options.write && would_mark > 0 {
        let mut tx = pool.begin().await?;
        summary.marked =
            insert_backfill_attempts(&mut *tx, &options.prompt_version, max_chars).await?;
        tx.commit().await?;
    }

    Ok(summary)
}

pub async fn mark_offtopic(
    pool: &PgPool,
    options: MarkOfftopicOptions,
) -> anyhow::Result<MarkOfftopicSummary> {
    let title_terms = normalize_title_terms(&options.title_contains);
    let video_ids = load_video_ids(options.video_ids_path.as_deref())?;
    if title_terms.is_empty() && video_ids.is_empty() {
        anyhow::bail!("mark-offtopic requires --title-contains or --video-ids");
    }

    let candidates = off_topic_candidates(pool, &options.prompt_version).await?;
    let matched_candidates = candidates
        .into_iter()
        .filter_map(|candidate| match_offtopic_candidate(candidate, &title_terms, &video_ids))
        .collect::<Vec<_>>();

    let mut summary = MarkOfftopicSummary {
        dry_run: !options.write,
        would_mark: matched_candidates.len(),
        marked: 0,
        matched_titles: matched_candidates
            .iter()
            .map(|candidate| MarkOfftopicTitleMatch {
                video_id: candidate.video_id.clone(),
                title: candidate.title.clone(),
                char_len: candidate.char_len,
                matched_by_title_contains: candidate.matched_by_title_contains.clone(),
                matched_by_video_ids: candidate.matched_by_video_ids,
            })
            .collect(),
        backup_path: None,
    };

    if options.write && !matched_candidates.is_empty() {
        let mut tx = pool.begin().await?;
        let mut marked = 0_usize;
        for candidate in &matched_candidates {
            marked += insert_offtopic_attempt(
                &mut *tx,
                &candidate.video_id,
                &options.prompt_version,
                candidate.char_len,
            )
            .await?;
        }
        tx.commit().await?;
        summary.marked = marked;
    }

    Ok(summary)
}

#[derive(Debug)]
struct OfftopicCandidate {
    video_id: String,
    title: String,
    char_len: usize,
}

#[derive(Debug)]
struct MatchedOfftopicCandidate {
    video_id: String,
    title: String,
    char_len: usize,
    matched_by_title_contains: Vec<String>,
    matched_by_video_ids: bool,
}

fn normalize_title_terms(values: &[String]) -> Vec<String> {
    values
        .iter()
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .map(str::to_lowercase)
        .collect()
}

fn load_video_ids(path: Option<&Path>) -> anyhow::Result<HashSet<String>> {
    let Some(path) = path else {
        return Ok(HashSet::new());
    };
    let content = fs::read_to_string(path)
        .with_context(|| format!("read mark-offtopic video ids {}", path.display()))?;
    Ok(content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned)
        .collect())
}

async fn off_topic_candidates(
    pool: &PgPool,
    prompt_version: &str,
) -> anyhow::Result<Vec<OfftopicCandidate>> {
    let threshold = usize_to_i64(MONSTER_CHAR_THRESHOLD, "monster char threshold")?;
    let rows = sqlx::query!(
        r#"
        SELECT v.video_id, v.title, length(t.transcript_text) AS "char_len!"
        FROM brain.youtube_videos v JOIN brain.youtube_transcripts t ON t.video_id=v.video_id
        WHERE v.metadata->>'content_type'='verbal_strategy'
          AND length(t.transcript_text) > 0
          AND length(t.transcript_text) > $1::bigint
          AND NOT EXISTS (
            SELECT 1 FROM brain.youtube_learning_claims c
            WHERE c.video_id=v.video_id AND c.prompt_version=$2
          )
          AND NOT EXISTS (
            SELECT 1 FROM brain.youtube_transcript_claim_attempts a
            WHERE a.video_id=v.video_id AND a.prompt_version=$2
          )
        ORDER BY v.published_at DESC NULLS LAST, v.video_id ASC
        "#,
        threshold,
        prompt_version,
    )
    .fetch_all(pool)
    .await?;

    rows.into_iter()
        .map(|row| {
            Ok(OfftopicCandidate {
                video_id: row.video_id,
                title: row.title,
                char_len: usize::try_from(row.char_len)
                    .context("transcript char length is negative or too large")?,
            })
        })
        .collect()
}

fn match_offtopic_candidate(
    candidate: OfftopicCandidate,
    title_terms: &[String],
    video_ids: &HashSet<String>,
) -> Option<MatchedOfftopicCandidate> {
    let title_lower = candidate.title.to_lowercase();
    let matched_by_title_contains = title_terms
        .iter()
        .filter(|term| title_lower.contains(term.as_str()))
        .cloned()
        .collect::<Vec<_>>();
    let matched_by_video_ids = video_ids.contains(&candidate.video_id);
    if matched_by_title_contains.is_empty() && !matched_by_video_ids {
        return None;
    }
    Some(MatchedOfftopicCandidate {
        video_id: candidate.video_id,
        title: candidate.title,
        char_len: candidate.char_len,
        matched_by_title_contains,
        matched_by_video_ids,
    })
}

async fn insert_offtopic_attempt(
    executor: impl sqlx::PgExecutor<'_>,
    video_id: &str,
    prompt_version: &str,
    char_len: usize,
) -> anyhow::Result<usize> {
    let char_len = usize_to_i64(char_len, "offtopic transcript char length")?;
    let result = sqlx::query!(
        r#"
        INSERT INTO brain.youtube_transcript_claim_attempts(
          video_id, prompt_version, mode, status, claim_count, char_len,
          note, attempted_at, updated_at
        )
        VALUES($1, $2, 'monster', 'offtopic', 0, $3, $4, now(), now())
        ON CONFLICT(video_id, prompt_version) DO NOTHING
        "#,
        video_id,
        prompt_version,
        char_len,
        "marked off-topic before monster workflow",
    )
    .execute(executor)
    .await?;
    Ok(result.rows_affected() as usize)
}

#[derive(Debug, Deserialize)]
struct RawVideoInput {
    video_id: String,
    claims: Option<Vec<RawClaimInput>>,
    verified: Option<Vec<RawClaimInput>>,
}

impl RawVideoInput {
    fn claims(&self) -> &[RawClaimInput] {
        if let Some(claims) = &self.claims {
            claims
        } else if let Some(verified) = &self.verified {
            verified
        } else {
            &[]
        }
    }
}

#[derive(Debug, Deserialize)]
struct RawClaimInput {
    entity_type: Option<String>,
    entity_name: Option<String>,
    claim_type: Option<String>,
    claim_text: Option<String>,
    evidence_quote: Option<String>,
    timestamp_seconds: Option<f64>,
    verdict: Option<String>,
    confidence: Option<f64>,
    model_confidence: Option<f64>,
    db_evidence: Option<Value>,
    db_value: Option<Value>,
    reasoning: Option<String>,
}

#[derive(Debug)]
struct ValidatedClaim {
    entity_type: Option<String>,
    entity_name: Option<String>,
    claim_type: String,
    claim_text: String,
    evidence_quote: String,
    timestamp_seconds: Option<f64>,
    verdict: String,
    confidence: f64,
    model_confidence: f64,
    status: String,
    db_evidence: Option<Value>,
    db_value: Option<Value>,
    reasoning: Option<String>,
}

fn validate_claim_record(
    video_id: &str,
    claim_position: usize,
    raw: &RawClaimInput,
    errors: &mut Vec<String>,
) -> Option<ValidatedClaim> {
    let record = format!("video {video_id} claim {claim_position}");
    let claim_type = required_string(&raw.claim_type, &record, "claim_type", errors)?;
    let claim_text = required_string(&raw.claim_text, &record, "claim_text", errors)?;
    let verdict = required_string(&raw.verdict, &record, "verdict", errors)?;
    let Some(status) = verdict_to_status(&verdict) else {
        errors.push(format!("{record}: unknown verdict '{verdict}'"));
        return None;
    };
    let Some(confidence) = raw.confidence else {
        errors.push(format!("{record}: missing confidence"));
        return None;
    };

    Some(ValidatedClaim {
        entity_type: raw.entity_type.clone(),
        entity_name: raw.entity_name.clone(),
        claim_type,
        claim_text,
        evidence_quote: raw.evidence_quote.clone().unwrap_or_default(),
        timestamp_seconds: raw.timestamp_seconds,
        verdict,
        confidence,
        model_confidence: raw.model_confidence.unwrap_or(confidence),
        status: status.to_string(),
        db_evidence: raw.db_evidence.clone(),
        db_value: raw.db_value.clone(),
        reasoning: raw.reasoning.clone(),
    })
}

fn required_string(
    value: &Option<String>,
    record: &str,
    field: &str,
    errors: &mut Vec<String>,
) -> Option<String> {
    match value {
        Some(value) if !value.is_empty() => Some(value.clone()),
        _ => {
            errors.push(format!("{record}: missing {field}"));
            None
        }
    }
}

fn verdict_to_status(verdict: &str) -> Option<&'static str> {
    match verdict {
        "supported" => Some("accepted"),
        "uncertain" => Some("needs_review"),
        "no_trusted_data" => Some("unverified"),
        "contradicted" => Some("rejected"),
        _ => None,
    }
}

fn initial_status_breakdown() -> BTreeMap<String, usize> {
    let mut breakdown = BTreeMap::new();
    for status in ["accepted", "needs_review", "unverified", "rejected"] {
        breakdown.insert(status.to_string(), 0);
    }
    breakdown
}

fn increment_status(breakdown: &mut BTreeMap<String, usize>, status: &str) {
    if let Some(count) = breakdown.get_mut(status) {
        *count += 1;
    }
}

fn transcript_claim_hash(video_id: &str, claim_text: &str, evidence_quote: &str) -> String {
    let content = format!("{video_id}|{claim_text}|{evidence_quote}");
    hex::encode(Sha256::digest(content.as_bytes()))
}

async fn claim_hash_exists(
    executor: impl sqlx::PgExecutor<'_>,
    claim_hash: &str,
) -> anyhow::Result<bool> {
    let exists = sqlx::query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM brain.youtube_learning_claims WHERE claim_hash=$1) AS "exists!""#,
        claim_hash,
    )
    .fetch_one(executor)
    .await?;
    Ok(exists)
}

async fn insert_claim(
    executor: impl sqlx::PgExecutor<'_>,
    video_id: &str,
    claim_hash: &str,
    claim_index: i64,
    claim: &ValidatedClaim,
    options: &IngestOptions,
) -> anyhow::Result<usize> {
    let verifier_json = serde_json::to_string(&json!({
        "verdict": &claim.verdict,
        "status": &claim.status,
        "confidence": claim.confidence,
        "verifier": "claude_db_crosscheck_v1",
        "db_evidence": claim.db_evidence.clone().unwrap_or(Value::Null),
        "db_value": claim.db_value.clone().unwrap_or(Value::Null),
        "reasoning": &claim.reasoning,
    }))?;
    let result = sqlx::query!(
        r#"
        INSERT INTO brain.youtube_learning_claims(
          video_id, claim_hash, claim_index, entity_type, entity_name, claim_type,
          claim_text, evidence_quote, timestamp_seconds, model_confidence,
          verifier_confidence, status, model, prompt_version, prompt_text,
          model_response_text, provider_metadata, verifier, created_at, updated_at
        )
        VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,'{}'::jsonb,$17::text::jsonb,now(),now())
        ON CONFLICT(claim_hash) DO NOTHING
        "#,
        video_id,
        claim_hash,
        claim_index,
        claim.entity_type.as_deref(),
        claim.entity_name.as_deref(),
        claim.claim_type.as_str(),
        claim.claim_text.as_str(),
        claim.evidence_quote.as_str(),
        claim.timestamp_seconds,
        claim.model_confidence,
        claim.confidence,
        claim.status.as_str(),
        options.model.as_str(),
        options.prompt_version.as_str(),
        PROMPT_TEXT,
        claim.reasoning.as_deref().unwrap_or(""),
        verifier_json,
    )
    .execute(executor)
    .await?;
    Ok(result.rows_affected() as usize)
}

async fn upsert_transcript_claim_attempt(
    executor: impl sqlx::PgExecutor<'_>,
    video_id: &str,
    prompt_version: &str,
    claim_count: usize,
    char_len: usize,
) -> anyhow::Result<usize> {
    let status = if claim_count == 0 { "zero_yield" } else { "ok" };
    let mode = if char_len > MONSTER_CHAR_THRESHOLD {
        PrepareMode::Monster.as_str()
    } else {
        PrepareMode::Normal.as_str()
    };
    let claim_count = usize_to_i64(claim_count, "attempt claim count")?;
    let char_len = usize_to_i64(char_len, "attempt transcript char length")?;
    let result = sqlx::query!(
        r#"
        INSERT INTO brain.youtube_transcript_claim_attempts(
          video_id, prompt_version, mode, status, claim_count, char_len,
          note, attempted_at, updated_at
        )
        VALUES($1, $2, $3, $4, $5, $6, NULL, now(), now())
        ON CONFLICT(video_id, prompt_version) DO UPDATE SET
          status=excluded.status,
          claim_count=excluded.claim_count,
          mode=excluded.mode,
          char_len=excluded.char_len,
          updated_at=now()
        "#,
        video_id,
        prompt_version,
        mode,
        status,
        claim_count,
        char_len,
    )
    .execute(executor)
    .await?;
    Ok(result.rows_affected() as usize)
}

async fn transcript_char_len(
    executor: impl sqlx::PgExecutor<'_>,
    video_id: &str,
) -> anyhow::Result<usize> {
    let char_len = sqlx::query_scalar!(
        r#"SELECT length(transcript_text) AS "len!" FROM brain.youtube_transcripts WHERE video_id=$1 LIMIT 1"#,
        video_id,
    )
    .fetch_optional(executor)
    .await?;
    match char_len {
        Some(char_len) => {
            usize::try_from(char_len).context("transcript char length is negative or too large")
        }
        None => Ok(0),
    }
}

async fn count_backfill_attempt_candidates(
    pool: &PgPool,
    prompt_version: &str,
    max_chars: Option<i64>,
) -> anyhow::Result<usize> {
    let count = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) AS "cnt!"
        FROM brain.youtube_videos v JOIN brain.youtube_transcripts t ON t.video_id=v.video_id
        WHERE v.metadata->>'content_type'='verbal_strategy'
          AND length(t.transcript_text) > 0
          AND NOT EXISTS (
            SELECT 1 FROM brain.youtube_learning_claims c
            WHERE c.video_id=v.video_id AND c.prompt_version=$1
          )
          AND NOT EXISTS (
            SELECT 1 FROM brain.youtube_transcript_claim_attempts a
            WHERE a.video_id=v.video_id AND a.prompt_version=$1
          )
          AND ($2::bigint IS NULL OR length(t.transcript_text) <= $2)
        "#,
        prompt_version,
        max_chars,
    )
    .fetch_one(pool)
    .await?;
    usize::try_from(count).context("backfill candidate count is negative or too large")
}

async fn insert_backfill_attempts(
    executor: impl sqlx::PgExecutor<'_>,
    prompt_version: &str,
    max_chars: Option<i64>,
) -> anyhow::Result<usize> {
    let result = sqlx::query!(
        r#"
        INSERT INTO brain.youtube_transcript_claim_attempts(
          video_id, prompt_version, mode, status, claim_count, char_len,
          note, attempted_at, updated_at
        )
        SELECT v.video_id, $1, 'normal', 'zero_yield', 0, length(t.transcript_text),
               NULL, now(), now()
        FROM brain.youtube_videos v JOIN brain.youtube_transcripts t ON t.video_id=v.video_id
        WHERE v.metadata->>'content_type'='verbal_strategy'
          AND length(t.transcript_text) > 0
          AND NOT EXISTS (
            SELECT 1 FROM brain.youtube_learning_claims c
            WHERE c.video_id=v.video_id AND c.prompt_version=$1
          )
          AND NOT EXISTS (
            SELECT 1 FROM brain.youtube_transcript_claim_attempts a
            WHERE a.video_id=v.video_id AND a.prompt_version=$1
          )
          AND ($2::bigint IS NULL OR length(t.transcript_text) <= $2)
        "#,
        prompt_version,
        max_chars,
    )
    .execute(executor)
    .await?;
    Ok(result.rows_affected() as usize)
}

fn usize_to_i64(value: usize, name: &str) -> anyhow::Result<i64> {
    i64::try_from(value).with_context(|| format!("{name} is too large"))
}

async fn count_claims(executor: impl sqlx::PgExecutor<'_>) -> anyhow::Result<usize> {
    let count = sqlx::query_scalar!(
        r#"SELECT COUNT(*) AS "cnt!" FROM brain.youtube_learning_claims"#
    )
    .fetch_one(executor)
    .await?;
    usize::try_from(count).context("claim count is negative or too large")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunking_is_char_based_overlapping_and_multibyte_safe() {
        let transcript = "x".repeat(250_000);
        let chunks = chunk_transcript_text(&transcript, 60_000, 4_000).expect("chunk transcript");

        assert_eq!(chunks.len(), 5);
        assert_eq!(chunks.last().expect("last chunk").char_end, 250_000);
        for chunk in &chunks {
            assert_eq!(chunk.char_len, chunk.char_end - chunk.char_start);
            assert_eq!(chunk.text.chars().count(), chunk.char_len);
            assert!(!chunk.text.is_empty());
        }
        for pair in chunks.windows(2) {
            assert_eq!(pair[1].char_start, pair[0].char_end - 4_000);
        }
        let mut covered = vec![false; 250_000];
        for chunk in &chunks {
            for covered_slot in &mut covered[chunk.char_start..chunk.char_end] {
                *covered_slot = true;
            }
        }
        assert!(covered.into_iter().all(|covered_slot| covered_slot));

        let multibyte = "aä🙂漢".repeat(5);
        let multibyte_chunks =
            chunk_transcript_text(&multibyte, 7, 2).expect("chunk multibyte transcript");
        assert_eq!(multibyte.chars().count(), 20);
        assert_eq!(
            multibyte_chunks.last().expect("last multibyte").char_end,
            20
        );
        for chunk in &multibyte_chunks {
            let expected_text = multibyte
                .chars()
                .skip(chunk.char_start)
                .take(chunk.char_len)
                .collect::<String>();
            assert_eq!(chunk.text, expected_text);
        }
    }

    #[test]
    fn chunking_rejects_overlap_greater_or_equal_to_chunk_size() {
        let error = chunk_transcript_text("abc", 4, 4).expect_err("invalid overlap");
        assert!(error
            .to_string()
            .contains("overlap_chars (4) must be smaller than chunk_chars (4)"));
    }

    #[test]
    fn verdict_mapping_covers_known_and_unknown_values() {
        assert_eq!(verdict_to_status("supported"), Some("accepted"));
        assert_eq!(verdict_to_status("uncertain"), Some("needs_review"));
        assert_eq!(verdict_to_status("no_trusted_data"), Some("unverified"));
        assert_eq!(verdict_to_status("contradicted"), Some("rejected"));
        assert_eq!(verdict_to_status("opinion"), None);
    }

    fn claim_json(
        verdict: &str,
        claim_type: &str,
        claim_text: &str,
        evidence_quote: &str,
        confidence: f64,
    ) -> Value {
        json!({
            "entity_type": "hero",
            "entity_name": "Lash",
            "claim_type": claim_type,
            "claim_text": claim_text,
            "evidence_quote": evidence_quote,
            "timestamp_seconds": 12.5,
            "verdict": verdict,
            "confidence": confidence,
            "db_evidence": {"source": "test"},
            "db_value": {"value": 1},
            "reasoning": "reason"
        })
    }

    fn temp_input(value: &Value) -> tempfile::NamedTempFile {
        let file = tempfile::NamedTempFile::new().expect("temp input");
        fs::write(
            file.path(),
            serde_json::to_string(value).expect("serialize input"),
        )
        .expect("write input");
        file
    }

    #[tokio::test]
    #[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
    async fn ingest_inserts_maps_verdicts_and_is_idempotent_pg() {
        let Some(pool) = crate::testutil::test_pool().await else {
            return;
        };
        let suffix = crate::testutil::unique_suffix();
        let feed_key = format!("ztest_feed_{suffix}");
        let video_id = format!("ztest_ing_{suffix}");
        crate::testutil::seed_feed(&pool, &feed_key).await;
        crate::testutil::seed_video(
            &pool,
            &video_id,
            &feed_key,
            "ready",
            "queued",
            Some("2026-06-01T00:00:00Z"),
            r#"{"content_type":"verbal_strategy"}"#,
        )
        .await;
        crate::testutil::seed_transcript(&pool, &video_id, "ingest transcript body").await;

        let baseline = count_claims(&pool).await.expect("baseline count");

        let input = temp_input(&json!([
            {
                "video_id": video_id,
                "claims": [
                    claim_json("supported", "build", "Claim supported.", "Quote supported.", 0.91),
                    claim_json("uncertain", "mechanic", "Claim uncertain.", "Quote uncertain.", 0.52),
                    claim_json("no_trusted_data", "macro", "Claim unverified.", "Quote unverified.", 0.43),
                    claim_json("contradicted", "counterplay", "Claim rejected.", "Quote rejected.", 0.77)
                ]
            }
        ]));

        let first = ingest(
            &pool,
            input.path(),
            IngestOptions {
                write: true,
                no_backup: true,
                model: "claude-test".to_string(),
                prompt_version: PROMPT_VERSION.to_string(),
            },
        )
        .await
        .expect("first ingest");
        let second = ingest(
            &pool,
            input.path(),
            IngestOptions {
                write: true,
                no_backup: true,
                model: "claude-test".to_string(),
                prompt_version: PROMPT_VERSION.to_string(),
            },
        )
        .await
        .expect("second ingest");

        assert_eq!(first.inserted, 4);
        assert_eq!(first.skipped_existing, 0);
        assert_eq!(first.attempts_recorded, 1);
        assert_eq!(first.status_breakdown["accepted"], 1);
        assert_eq!(first.status_breakdown["needs_review"], 1);
        assert_eq!(first.status_breakdown["unverified"], 1);
        assert_eq!(first.status_breakdown["rejected"], 1);
        assert_eq!(first.total_claims_in_table_after, baseline + 4);
        assert_eq!(second.inserted, 0);
        assert_eq!(second.skipped_existing, 4);
        assert_eq!(second.total_claims_in_table_after, baseline + 4);

        let claims_for_video: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM brain.youtube_learning_claims WHERE video_id=$1",
        )
        .bind(&video_id)
        .fetch_one(&pool)
        .await
        .expect("count claims for video");
        assert_eq!(claims_for_video, 4);

        crate::testutil::cleanup(&pool, &[&video_id], &[&feed_key]).await;
    }

    #[tokio::test]
    #[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
    async fn prepare_selects_untreated_verbal_video_then_excludes_it_pg() {
        let Some(pool) = crate::testutil::test_pool().await else {
            return;
        };
        let suffix = crate::testutil::unique_suffix();
        let feed_key = format!("ztest_feed_{suffix}");
        let video_id = format!("ztest_prep_{suffix}");
        crate::testutil::seed_feed(&pool, &feed_key).await;
        crate::testutil::seed_video(
            &pool,
            &video_id,
            &feed_key,
            "ready",
            "queued",
            Some("2999-01-01T00:00:00Z"),
            r#"{"content_type":"verbal_strategy"}"#,
        )
        .await;
        crate::testutil::seed_transcript(&pool, &video_id, "prepare transcript body").await;

        let selected = prepare(&pool, 100_000, PrepareOrder::Recent, Some(150_000))
            .await
            .expect("prepare");
        assert!(
            selected.videos.iter().any(|v| v.video_id == video_id),
            "seeded untreated verbal video must be selectable"
        );

        // Ein Attempt-Ledger-Eintrag muss das Video von der Auswahl ausschließen.
        let mut tx = pool.begin().await.expect("begin");
        upsert_transcript_claim_attempt(&mut *tx, &video_id, PROMPT_VERSION, 0, 20)
            .await
            .expect("attempt");
        tx.commit().await.expect("commit");

        let after = prepare(&pool, 100_000, PrepareOrder::Recent, Some(150_000))
            .await
            .expect("prepare after");
        assert!(
            !after.videos.iter().any(|v| v.video_id == video_id),
            "video with attempt ledger row must be excluded"
        );

        crate::testutil::cleanup(&pool, &[&video_id], &[&feed_key]).await;
    }
}
