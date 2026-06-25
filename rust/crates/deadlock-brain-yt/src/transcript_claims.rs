use std::{
    collections::{BTreeMap, HashMap, HashSet},
    ffi::OsString,
    fmt,
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;
use clap::ValueEnum;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

use crate::db;

pub const PROMPT_VERSION: &str = "youtube_claims_de_transcript_v1";
pub const PROMPT_TEXT: &str =
    "Transcript-basierte Claim-Extraktion (Claude) + DB-Verifikation gegen deadlock_data/patch_events.";

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
}

impl fmt::Display for PrepareOrder {
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

#[derive(Debug)]
pub struct IngestOptions {
    pub write: bool,
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
    pub errors: Vec<String>,
    pub status_breakdown: BTreeMap<String, usize>,
    pub total_claims_in_table_after: usize,
}

pub fn prepare(
    conn: &Connection,
    limit: usize,
    order: PrepareOrder,
) -> anyhow::Result<PrepareSummary> {
    let order_sql = match order {
        PrepareOrder::Recent => "COALESCE(v.published_at,'') DESC, v.video_id ASC",
        PrepareOrder::Shortest => "length(t.transcript_text) ASC, v.video_id ASC",
    };
    let sql = format!(
        r#"
        SELECT v.video_id, v.title, v.url, v.channel_title, t.transcript_text
        FROM youtube_videos v JOIN youtube_transcripts t ON t.video_id=v.video_id
        WHERE json_extract(v.metadata_json,'$.content_type')='verbal_strategy'
          AND NOT EXISTS (
            SELECT 1 FROM youtube_learning_claims c
            WHERE c.video_id=v.video_id AND c.prompt_version=?
          )
        ORDER BY {order_sql}
        LIMIT ?
        "#
    );
    let limit = i64::try_from(limit).context("prepare limit is too large")?;
    let mut statement = conn.prepare(&sql)?;
    let videos = statement
        .query_map(params![PROMPT_VERSION, limit], |row| {
            let transcript_text: String = row.get(4)?;
            Ok(PrepareVideo {
                video_id: row.get(0)?,
                title: row.get(1)?,
                url: row.get(2)?,
                channel_title: row.get(3)?,
                char_len: transcript_text.chars().count(),
                transcript_text,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    Ok(PrepareSummary {
        generated_at: db::now_epoch_seconds(),
        prompt_version: PROMPT_VERSION.to_string(),
        order: order.as_str().to_string(),
        count: videos.len(),
        videos,
    })
}

pub fn ingest(
    conn: &mut Connection,
    input_path: &Path,
    options: IngestOptions,
) -> anyhow::Result<IngestSummary> {
    let raw_input = fs::read_to_string(input_path)
        .with_context(|| format!("read transcript claims input {}", input_path.display()))?;
    let videos: Vec<RawVideoInput> = serde_json::from_str(&raw_input)
        .with_context(|| format!("parse transcript claims input {}", input_path.display()))?;

    let backup_path = if options.write && !options.no_backup {
        Some(create_backup(conn)?)
    } else {
        None
    };

    let tx = conn.transaction()?;
    let now = db::now_epoch_seconds();
    let mut summary = IngestSummary {
        dry_run: !options.write,
        backup_path,
        inserted: 0,
        skipped_existing: 0,
        errors: Vec::new(),
        status_breakdown: initial_status_breakdown(),
        total_claims_in_table_after: 0,
    };
    let mut next_claim_index_by_video: HashMap<String, i64> = HashMap::new();
    let mut new_hashes_in_batch = HashSet::new();

    for video in &videos {
        for (claim_position, raw_claim) in video.claims().iter().enumerate() {
            let Some(claim) =
                validate_claim_record(&video.video_id, claim_position, raw_claim, &mut summary.errors)
            else {
                continue;
            };
            let claim_hash = transcript_claim_hash(
                &video.video_id,
                &claim.claim_text,
                &claim.evidence_quote,
            );
            if claim_hash_exists(&tx, &claim_hash)? || new_hashes_in_batch.contains(&claim_hash) {
                summary.skipped_existing += 1;
                continue;
            }

            let next_claim_index = next_claim_index_by_video
                .entry(video.video_id.clone())
                .or_insert(0);
            let claim_index = *next_claim_index;
            if options.write {
                let inserted = insert_claim(
                    &tx,
                    &video.video_id,
                    &claim_hash,
                    claim_index,
                    &claim,
                    &options,
                    now,
                )?;
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
    }

    summary.total_claims_in_table_after = count_claims(&tx)?;
    if options.write {
        tx.commit()?;
    } else {
        tx.rollback()?;
    }
    Ok(summary)
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

fn claim_hash_exists(conn: &Connection, claim_hash: &str) -> rusqlite::Result<bool> {
    conn.query_row(
        "SELECT 1 FROM youtube_learning_claims WHERE claim_hash=? LIMIT 1",
        params![claim_hash],
        |_| Ok(()),
    )
    .optional()
    .map(|row| row.is_some())
}

fn insert_claim(
    conn: &Connection,
    video_id: &str,
    claim_hash: &str,
    claim_index: i64,
    claim: &ValidatedClaim,
    options: &IngestOptions,
    now: i64,
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
    let inserted = conn.execute(
        r#"
        INSERT INTO youtube_learning_claims(
          video_id, claim_hash, claim_index, entity_type, entity_name, claim_type,
          claim_text, evidence_quote, timestamp_seconds, model_confidence,
          verifier_confidence, status, model, prompt_version, prompt_text,
          model_response_text, provider_metadata_json, verifier_json, created_at, updated_at
        )
        VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)
        ON CONFLICT(claim_hash) DO NOTHING
        "#,
        params![
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
            "{}",
            verifier_json,
            now,
            now
        ],
    )?;
    Ok(inserted)
}

fn count_claims(conn: &Connection) -> anyhow::Result<usize> {
    let count = conn.query_row("SELECT COUNT(*) FROM youtube_learning_claims", [], |row| {
        row.get::<_, i64>(0)
    })?;
    usize::try_from(count).context("claim count is negative or too large")
}

fn create_backup(conn: &Connection) -> anyhow::Result<String> {
    let db_path = main_database_path(conn)?;
    let _ = conn.execute_batch("PRAGMA wal_checkpoint(TRUNCATE);");
    let now = db::now_epoch_seconds();
    let (year, month, day, hour, minute, second) = unix_datetime_utc(now);
    let base_suffix = format!(".bak-{year:04}-{month:02}-{day:02}-transcript-claims-scale");
    let mut candidate = append_suffix(&db_path, &base_suffix);
    if candidate.exists() {
        let timed_suffix = format!("{base_suffix}-{hour:02}{minute:02}{second:02}");
        candidate = append_suffix(&db_path, &timed_suffix);
        let mut counter = 2;
        while candidate.exists() {
            candidate = append_suffix(&db_path, &format!("{timed_suffix}-{counter}"));
            counter += 1;
        }
    }
    fs::copy(&db_path, &candidate).with_context(|| {
        format!(
            "copy sqlite backup from {} to {}",
            db_path.display(),
            candidate.display()
        )
    })?;

    let wal_path = append_suffix(&db_path, "-wal");
    match fs::metadata(&wal_path) {
        Ok(metadata) if metadata.len() > 0 => {
            let backup_wal_path = append_suffix(&candidate, "-wal");
            fs::copy(&wal_path, &backup_wal_path).with_context(|| {
                format!(
                    "copy sqlite backup sidecar from {} to {}",
                    wal_path.display(),
                    backup_wal_path.display()
                )
            })?;

            let shm_path = append_suffix(&db_path, "-shm");
            match fs::metadata(&shm_path) {
                Ok(_) => {
                    let backup_shm_path = append_suffix(&candidate, "-shm");
                    fs::copy(&shm_path, &backup_shm_path).with_context(|| {
                        format!(
                            "copy sqlite backup sidecar from {} to {}",
                            shm_path.display(),
                            backup_shm_path.display()
                        )
                    })?;
                }
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => {
                    return Err(error).with_context(|| {
                        format!("stat sqlite backup sidecar {}", shm_path.display())
                    });
                }
            }
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(error)
                .with_context(|| format!("stat sqlite WAL sidecar {}", wal_path.display()));
        }
    }
    Ok(candidate.to_string_lossy().into_owned())
}

fn main_database_path(conn: &Connection) -> anyhow::Result<PathBuf> {
    let mut statement = conn.prepare("PRAGMA database_list")?;
    let rows = statement.query_map([], |row| {
        Ok((row.get::<_, String>(1)?, row.get::<_, String>(2)?))
    })?;
    for row in rows {
        let (name, file) = row?;
        if name == "main" && !file.is_empty() {
            return Ok(PathBuf::from(file));
        }
    }
    anyhow::bail!("cannot determine sqlite main database path for backup")
}

fn append_suffix(path: &Path, suffix: &str) -> PathBuf {
    let mut value: OsString = path.as_os_str().to_os_string();
    value.push(suffix);
    PathBuf::from(value)
}

fn unix_datetime_utc(timestamp: i64) -> (i32, u32, u32, u32, u32, u32) {
    let days = timestamp.div_euclid(86_400);
    let seconds_of_day = timestamp.rem_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    let hour = (seconds_of_day / 3_600) as u32;
    let minute = ((seconds_of_day % 3_600) / 60) as u32;
    let second = (seconds_of_day % 60) as u32;
    (year, month, day, hour, minute, second)
}

fn civil_from_days(days_since_epoch: i64) -> (i32, u32, u32) {
    let days = days_since_epoch + 719_468;
    let era = if days >= 0 { days } else { days - 146_096 } / 146_097;
    let day_of_era = days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    let year = year + if month <= 2 { 1 } else { 0 };
    (year as i32, month as u32, day as u32)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use rusqlite::{params, Connection};
    use serde_json::{json, Value};

    use super::*;

    const TEST_SCHEMA: &str = r#"
CREATE TABLE youtube_videos (video_id TEXT PRIMARY KEY, feed_key TEXT NOT NULL, channel_id TEXT, channel_title TEXT, title TEXT NOT NULL, url TEXT NOT NULL, published_at TEXT, description TEXT, metadata_json TEXT NOT NULL DEFAULT '{}', transcript_status TEXT NOT NULL DEFAULT 'missing', learning_status TEXT NOT NULL DEFAULT 'queued', discovered_at INTEGER NOT NULL, updated_at INTEGER NOT NULL);
CREATE TABLE youtube_transcripts (video_id TEXT PRIMARY KEY, language TEXT, source_kind TEXT NOT NULL, transcript_text TEXT NOT NULL, content_hash TEXT NOT NULL, source_document_id INTEGER, imported_at INTEGER NOT NULL, updated_at INTEGER NOT NULL);
CREATE TABLE youtube_learning_claims (id INTEGER PRIMARY KEY AUTOINCREMENT, video_id TEXT NOT NULL, claim_hash TEXT NOT NULL UNIQUE, claim_index INTEGER NOT NULL, entity_type TEXT, entity_name TEXT, claim_type TEXT NOT NULL, claim_text TEXT NOT NULL, evidence_quote TEXT NOT NULL, timestamp_seconds REAL, model_confidence REAL NOT NULL, verifier_confidence REAL NOT NULL, status TEXT NOT NULL, model TEXT, prompt_version TEXT NOT NULL, prompt_text TEXT NOT NULL, model_response_text TEXT NOT NULL, provider_metadata_json TEXT NOT NULL DEFAULT '{}', verifier_json TEXT NOT NULL DEFAULT '{}', created_at INTEGER NOT NULL, updated_at INTEGER NOT NULL);
"#;

    #[test]
    fn ingest_maps_verdicts_and_is_idempotent() {
        let mut conn = test_conn();
        let input = temp_input(&json!([
            {
                "video_id": "vid-ingest",
                "claims": [
                    claim_json("supported", "build", "Claim supported.", "Quote supported.", 0.91, Some(0.82), Some("supported reason")),
                    claim_json("uncertain", "mechanic", "Claim uncertain.", "Quote uncertain.", 0.52, None, None),
                    claim_json("no_trusted_data", "macro", "Claim unverified.", "Quote unverified.", 0.43, None, Some("no trusted data")),
                    claim_json("contradicted", "counterplay", "Claim rejected.", "Quote rejected.", 0.77, Some(0.66), Some("contradicted reason"))
                ]
            }
        ]));

        let first = ingest(
            &mut conn,
            input.path(),
            IngestOptions {
                write: true,
                no_backup: true,
                model: "claude-test".to_string(),
                prompt_version: PROMPT_VERSION.to_string(),
            },
        )
        .expect("first ingest");
        let second = ingest(
            &mut conn,
            input.path(),
            IngestOptions {
                write: true,
                no_backup: true,
                model: "claude-test".to_string(),
                prompt_version: PROMPT_VERSION.to_string(),
            },
        )
        .expect("second ingest");

        assert_eq!(first.inserted, 4);
        assert_eq!(first.skipped_existing, 0);
        assert_eq!(first.status_breakdown["accepted"], 1);
        assert_eq!(first.status_breakdown["needs_review"], 1);
        assert_eq!(first.status_breakdown["unverified"], 1);
        assert_eq!(first.status_breakdown["rejected"], 1);
        assert_eq!(first.total_claims_in_table_after, 4);
        assert_eq!(second.inserted, 0);
        assert_eq!(second.skipped_existing, 4);
        assert_eq!(second.total_claims_in_table_after, 4);

        let statuses = query_statuses(&conn);
        assert_eq!(
            statuses,
            vec![
                (0, "accepted".to_string()),
                (1, "needs_review".to_string()),
                (2, "unverified".to_string()),
                (3, "rejected".to_string()),
            ]
        );

        let null_or_empty_count: i64 = conn
            .query_row(
                r#"
                SELECT COUNT(*)
                FROM youtube_learning_claims
                WHERE claim_hash IS NULL
                   OR claim_type IS NULL OR claim_type=''
                   OR claim_text IS NULL OR claim_text=''
                   OR evidence_quote IS NULL
                   OR model_confidence IS NULL
                   OR verifier_confidence IS NULL
                   OR status IS NULL OR status=''
                   OR prompt_version IS NULL OR prompt_version=''
                   OR prompt_text IS NULL OR prompt_text=''
                   OR model_response_text IS NULL
                   OR provider_metadata_json IS NULL OR provider_metadata_json=''
                   OR verifier_json IS NULL OR verifier_json=''
                   OR created_at IS NULL
                   OR updated_at IS NULL
                "#,
                [],
                |row| row.get(0),
            )
            .expect("query nulls");
        assert_eq!(null_or_empty_count, 0);

        let (prompt_text, provider_metadata_json, verifier_json): (String, String, String) = conn
            .query_row(
                "SELECT prompt_text, provider_metadata_json, verifier_json FROM youtube_learning_claims WHERE claim_index=0",
                [],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .expect("query verifier");
        let verifier: Value = serde_json::from_str(&verifier_json).expect("parse verifier");
        assert_eq!(prompt_text, PROMPT_TEXT);
        assert_eq!(provider_metadata_json, "{}");
        assert_eq!(verifier["verifier"], "claude_db_crosscheck_v1");
        assert_eq!(verifier["status"], "accepted");
    }

    #[test]
    fn ingest_dry_run_does_not_insert_and_accepts_verified_alias() {
        let mut conn = test_conn();
        let input = temp_input(&json!([
            {
                "video_id": "vid-dry",
                "verified": [
                    claim_json("supported", "combo", "Claim dry.", "Quote dry.", 0.8, None, None)
                ]
            }
        ]));

        let summary = ingest(
            &mut conn,
            input.path(),
            IngestOptions {
                write: false,
                no_backup: false,
                model: "claude-test".to_string(),
                prompt_version: PROMPT_VERSION.to_string(),
            },
        )
        .expect("dry ingest");

        assert!(summary.dry_run);
        assert_eq!(summary.backup_path, None);
        assert_eq!(summary.inserted, 1);
        assert_eq!(summary.skipped_existing, 0);
        assert_eq!(count_claims(&conn), 0);
        assert_eq!(summary.total_claims_in_table_after, 0);
    }

    #[test]
    fn ingest_backup_checkpoint_captures_committed_wal_rows() {
        let dir = tempfile::tempdir().expect("temp dir");
        let db_path = dir.path().join("db.sqlite3");
        let mut conn = Connection::open(&db_path).expect("open file sqlite");
        conn.execute_batch("PRAGMA journal_mode=WAL;")
            .expect("enable WAL");
        conn.execute_batch(TEST_SCHEMA).expect("create schema");
        conn.execute_batch("PRAGMA wal_checkpoint(TRUNCATE);")
            .expect("checkpoint schema");
        seed_prepare_video(
            &conn,
            "vid-wal-first",
            "First WAL video",
            Some("2026-06-01T00:00:00Z"),
            r#"{"content_type":"verbal_strategy"}"#,
            "first transcript",
        );
        seed_prepare_video(
            &conn,
            "vid-wal-second",
            "Second WAL video",
            Some("2026-06-02T00:00:00Z"),
            r#"{"content_type":"verbal_strategy"}"#,
            "second transcript",
        );

        let first_input = temp_input(&json!([
            {
                "video_id": "vid-wal-first",
                "claims": [
                    claim_json("supported", "build", "First WAL claim.", "First WAL quote.", 0.91, None, Some("first"))
                ]
            }
        ]));
        let first = ingest(
            &mut conn,
            first_input.path(),
            IngestOptions {
                write: true,
                no_backup: true,
                model: "claude-test".to_string(),
                prompt_version: PROMPT_VERSION.to_string(),
            },
        )
        .expect("first WAL ingest");
        assert_eq!(first.inserted, 1);

        let wal_path = append_suffix(&db_path, "-wal");
        assert!(
            fs::metadata(&wal_path).expect("WAL metadata").len() > 0,
            "first committed row should still be in the WAL before backup"
        );

        let second_input = temp_input(&json!([
            {
                "video_id": "vid-wal-second",
                "claims": [
                    claim_json("supported", "build", "Second WAL claim.", "Second WAL quote.", 0.91, None, Some("second"))
                ]
            }
        ]));
        let second = ingest(
            &mut conn,
            second_input.path(),
            IngestOptions {
                write: true,
                no_backup: false,
                model: "claude-test".to_string(),
                prompt_version: PROMPT_VERSION.to_string(),
            },
        )
        .expect("second WAL ingest");

        let backup_path = std::path::PathBuf::from(
            second
                .backup_path
                .as_ref()
                .expect("second ingest backup path"),
        );
        assert!(backup_path.exists());

        let backup_conn = Connection::open(&backup_path).expect("open backup sqlite");
        let first_count: i64 = backup_conn
            .query_row(
                "SELECT COUNT(*) FROM youtube_learning_claims WHERE claim_text=?",
                params!["First WAL claim."],
                |row| row.get(0),
            )
            .expect("count first WAL claim in backup");
        let second_count: i64 = backup_conn
            .query_row(
                "SELECT COUNT(*) FROM youtube_learning_claims WHERE claim_text=?",
                params!["Second WAL claim."],
                |row| row.get(0),
            )
            .expect("count second WAL claim in backup");
        assert_eq!(first_count, 1);
        assert_eq!(second_count, 0);
    }

    #[test]
    fn ingest_unknown_verdict_is_error_and_not_inserted() {
        let mut conn = test_conn();
        let input = temp_input(&json!([
            {
                "video_id": "vid-error",
                "claims": [
                    claim_json("opinion", "meta", "Claim unknown.", "Quote unknown.", 0.7, None, Some("bad verdict"))
                ]
            }
        ]));

        let summary = ingest(
            &mut conn,
            input.path(),
            IngestOptions {
                write: true,
                no_backup: true,
                model: "claude-test".to_string(),
                prompt_version: PROMPT_VERSION.to_string(),
            },
        )
        .expect("ingest unknown verdict");

        assert_eq!(summary.inserted, 0);
        assert_eq!(summary.skipped_existing, 0);
        assert_eq!(summary.errors.len(), 1);
        assert!(summary.errors[0].contains("unknown verdict"));
        assert_eq!(count_claims(&conn), 0);
    }

    #[test]
    fn prepare_selects_only_untreated_verbal_videos_and_orders_them() {
        let conn = test_conn();
        seed_prepare_video(
            &conn,
            "verbal_recent",
            "Recent verbal",
            Some("2026-06-02T00:00:00Z"),
            r#"{"content_type":"verbal_strategy"}"#,
            "a longer transcript",
        );
        seed_prepare_video(
            &conn,
            "verbal_short",
            "Short verbal",
            Some("2026-06-01T00:00:00Z"),
            r#"{"content_type":"verbal_strategy"}"#,
            "short",
        );
        seed_prepare_video(
            &conn,
            "visual",
            "Visual video",
            Some("2026-06-03T00:00:00Z"),
            r#"{"content_type":"visual_tech"}"#,
            "visual transcript",
        );
        seed_prepare_video(
            &conn,
            "verbal_done",
            "Done verbal",
            Some("2026-06-04T00:00:00Z"),
            r#"{"content_type":"verbal_strategy"}"#,
            "done transcript",
        );
        insert_existing_claim(&conn, "verbal_done", PROMPT_VERSION, "done-hash");

        let recent = prepare(&conn, 20, PrepareOrder::Recent).expect("prepare recent");
        let shortest = prepare(&conn, 20, PrepareOrder::Shortest).expect("prepare shortest");

        assert_eq!(recent.prompt_version, PROMPT_VERSION);
        assert_eq!(recent.order, "recent");
        assert_eq!(recent.count, 2);
        assert_eq!(
            recent
                .videos
                .iter()
                .map(|video| video.video_id.as_str())
                .collect::<Vec<_>>(),
            vec!["verbal_recent", "verbal_short"]
        );
        assert_eq!(recent.videos[0].char_len, "a longer transcript".chars().count());
        assert_eq!(recent.videos[0].transcript_text, "a longer transcript");

        assert_eq!(
            shortest
                .videos
                .iter()
                .map(|video| video.video_id.as_str())
                .collect::<Vec<_>>(),
            vec!["verbal_short", "verbal_recent"]
        );
        assert_eq!(shortest.videos[0].char_len, "short".chars().count());
    }

    fn test_conn() -> Connection {
        let conn = Connection::open_in_memory().expect("open sqlite");
        conn.execute_batch(TEST_SCHEMA).expect("create schema");
        conn
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

    fn claim_json(
        verdict: &str,
        claim_type: &str,
        claim_text: &str,
        evidence_quote: &str,
        confidence: f64,
        model_confidence: Option<f64>,
        reasoning: Option<&str>,
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
            "model_confidence": model_confidence,
            "db_evidence": {"source": "test"},
            "db_value": {"value": 1},
            "reasoning": reasoning
        })
    }

    fn query_statuses(conn: &Connection) -> Vec<(i64, String)> {
        let mut statement = conn
            .prepare(
                "SELECT claim_index, status FROM youtube_learning_claims ORDER BY claim_index",
            )
            .expect("prepare status query");
        statement
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .expect("query statuses")
            .collect::<rusqlite::Result<Vec<_>>>()
            .expect("collect statuses")
    }

    fn count_claims(conn: &Connection) -> usize {
        conn.query_row("SELECT COUNT(*) FROM youtube_learning_claims", [], |row| {
            row.get::<_, i64>(0)
        })
        .expect("count claims") as usize
    }

    fn seed_prepare_video(
        conn: &Connection,
        video_id: &str,
        title: &str,
        published_at: Option<&str>,
        metadata_json: &str,
        transcript_text: &str,
    ) {
        let now = 1_700_000_000_i64;
        conn.execute(
            r#"
            INSERT INTO youtube_videos(
              video_id, feed_key, channel_id, channel_title, title, url,
              published_at, description, metadata_json, transcript_status,
              learning_status, discovered_at, updated_at
            )
            VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?)
            "#,
            params![
                video_id,
                "feed",
                Option::<String>::None,
                "Channel",
                title,
                format!("https://youtube.com/watch?v={video_id}"),
                published_at,
                Option::<String>::None,
                metadata_json,
                "ready",
                "queued",
                now,
                now
            ],
        )
        .expect("insert video");
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
                "youtube_caption_manual",
                transcript_text,
                format!("hash-{video_id}"),
                Option::<i64>::None,
                now,
                now
            ],
        )
        .expect("insert transcript");
    }

    fn insert_existing_claim(
        conn: &Connection,
        video_id: &str,
        prompt_version: &str,
        claim_hash: &str,
    ) {
        let now = 1_700_000_000_i64;
        conn.execute(
            r#"
            INSERT INTO youtube_learning_claims(
              video_id, claim_hash, claim_index, entity_type, entity_name,
              claim_type, claim_text, evidence_quote, timestamp_seconds,
              model_confidence, verifier_confidence, status, model,
              prompt_version, prompt_text, model_response_text,
              provider_metadata_json, verifier_json, created_at, updated_at
            )
            VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)
            "#,
            params![
                video_id,
                claim_hash,
                0_i64,
                Option::<String>::None,
                Option::<String>::None,
                "build",
                "Existing claim.",
                "Existing quote.",
                Option::<f64>::None,
                1.0_f64,
                1.0_f64,
                "accepted",
                "claude-test",
                prompt_version,
                PROMPT_TEXT,
                "",
                "{}",
                r#"{"verifier":"test"}"#,
                now,
                now
            ],
        )
        .expect("insert existing claim");
    }
}
