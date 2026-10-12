use crate::{
    config::Config,
    extract::{raw_locator_hashes, RawLocatorHash},
    store::Counts,
};
use anyhow::{ensure, Context, Result};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::{Connection, PgConnection, PgPool, Postgres, Row};
use std::{collections::BTreeMap, time::Duration};

const SAMPLE_SIZE: i64 = 200;
const JOB_LOCK: i64 = 742110026119;
const VERSION: &str = "p9-v1";

#[derive(Default)]
pub struct AuditCounts {
    pub sampled: u64,
    pub objects: u64,
    pub matched: u64,
    pub mismatched: u64,
    pub locator_not_found: u64,
    pub unsupported_locator_kind: u64,
}

impl AuditCounts {
    pub fn value(&self) -> Value {
        json!({"sampled":self.sampled,"objects":self.objects,"match":self.matched,
            "mismatch":self.mismatched,"locator_not_found":self.locator_not_found,
            "unsupported_locator_kind":self.unsupported_locator_kind})
    }
}

struct Receipt {
    id: i64,
    kind: String,
    locator: String,
    hash: Vec<u8>,
}

struct AuditResult {
    receipt: i64,
    outcome: &'static str,
    observed: Option<Vec<u8>>,
}

pub async fn run(config: &Config) -> Result<Counts> {
    let pool = config.pool().await?;
    let counts = audit_pool(&pool, config.max_blob_bytes).await;
    pool.close().await;
    let counts = counts?;
    ensure!(counts.unsupported_locator_kind == 0, "audit_incomplete");
    Ok(Counts {
        objects_completed: counts.objects,
        ..Counts::default()
    })
}

pub async fn audit_pool(pool: &PgPool, max_blob_bytes: usize) -> Result<AuditCounts> {
    ensure!(
        (1024..=32 * 1024 * 1024).contains(&max_blob_bytes),
        "audit_blob_limit"
    );
    let mut connection = pool.acquire().await?.detach();
    sqlx::query("SET statement_timeout='10000ms'")
        .execute(&mut connection)
        .await?;
    sqlx::query("SET lock_timeout='5000ms'")
        .execute(&mut connection)
        .await?;
    let acquired: bool = sqlx::query_scalar("SELECT pg_try_advisory_lock($1)")
        .bind(JOB_LOCK)
        .fetch_one(&mut connection)
        .await?;
    ensure!(acquired, "audit_busy");
    sqlx::query("SELECT job_run_id,receipt_id,outcome FROM ops.receipt_audit LIMIT 0")
        .execute(&mut connection)
        .await?;
    let job: i64 = sqlx::query_scalar("INSERT INTO ops.job_run(job,extractor,extractor_version) VALUES ('audit-receipts','receipt-audit',$1) RETURNING id")
        .bind(VERSION).fetch_one(&mut connection).await?;
    let mut counts = AuditCounts::default();
    let result = tokio::time::timeout(
        Duration::from_secs(90),
        sample(
            &mut connection,
            job,
            max_blob_bytes.min(16 * 1024 * 1024),
            &mut counts,
        ),
    )
    .await;
    let failed = !matches!(&result, Ok(Ok(())));
    let status = if failed {
        "failed"
    } else if counts.unsupported_locator_kind > 0 {
        "partial"
    } else {
        "succeeded"
    };
    sqlx::query("UPDATE ops.job_run SET finished_at=clock_timestamp(),status=$2,counts=$3,error_code=$4 WHERE id=$1")
        .bind(job).bind(status).bind(counts.value()).bind(failed.then_some("audit_failed"))
        .execute(&mut connection).await?;
    connection.close().await?;
    result.context("audit_timeout")??;
    Ok(counts)
}

async fn sample(
    connection: &mut PgConnection,
    job: i64,
    max_bytes: usize,
    counts: &mut AuditCounts,
) -> Result<()> {
    let rows = sqlx::query("SELECT r.id,r.raw_object_id FROM canon.receipt r
        JOIN canon.source_head h ON h.source_id=r.source_id AND h.published_revision_id>=r.revision_id
        WHERE NOT EXISTS (SELECT 1 FROM canon.receipt_quarantine q WHERE q.receipt_id=r.id)
        ORDER BY random() LIMIT $1")
        .bind(SAMPLE_SIZE).fetch_all(&mut *connection).await?;
    let mut objects: BTreeMap<i64, Vec<i64>> = BTreeMap::new();
    for row in rows {
        objects
            .entry(row.try_get("raw_object_id")?)
            .or_default()
            .push(row.try_get("id")?);
    }
    for (object, receipts) in objects {
        let results = audit_object(connection, job, object, &receipts, max_bytes).await?;
        counts.objects += 1;
        counts.sampled += receipts.len() as u64;
        for result in results {
            match result.outcome {
                "match" => counts.matched += 1,
                "mismatch" => counts.mismatched += 1,
                "locator_not_found" => counts.locator_not_found += 1,
                "unsupported_locator_kind" => counts.unsupported_locator_kind += 1,
                _ => anyhow::bail!("audit_outcome_invalid"),
            }
        }
    }
    Ok(())
}

async fn audit_object(
    connection: &mut PgConnection,
    job: i64,
    object: i64,
    receipts: &[i64],
    max_bytes: usize,
) -> Result<Vec<AuditResult>> {
    let mut tx = connection.begin().await?;
    let raw = sqlx::query(
        "SELECT content_sha256,
        CASE WHEN octet_length(body)<=$2 THEN body ELSE NULL END AS body
        FROM raw.object WHERE id=$1",
    )
    .bind(object)
    .bind(i64::try_from(max_bytes)?)
    .fetch_one(&mut *tx)
    .await?;
    let bytes: Vec<u8> = raw
        .try_get::<Option<Vec<u8>>, _>("body")?
        .context("audit_raw_limit")?;
    let hash: Vec<u8> = raw.try_get("content_sha256")?;
    let raw_mismatch = Sha256::digest(&bytes).as_slice() != hash;
    let rows = sqlx::query(
        "SELECT id,locator_kind,locator,value_sha256
        FROM canon.receipt WHERE raw_object_id=$1 AND id=ANY($2::bigint[]) ORDER BY id LIMIT 200",
    )
    .bind(object)
    .bind(receipts)
    .fetch_all(&mut *tx)
    .await?;
    ensure!(rows.len() == receipts.len(), "audit_receipt_missing");
    let rows = rows
        .into_iter()
        .map(|row| {
            Ok(Receipt {
                id: row.try_get("id")?,
                kind: row.try_get("locator_kind")?,
                locator: row.try_get("locator")?,
                hash: row.try_get("value_sha256")?,
            })
        })
        .collect::<Result<Vec<_>, sqlx::Error>>()?;
    let results = tokio::task::spawn_blocking(move || verify(&bytes, &rows, raw_mismatch))
        .await
        .context("audit_locator_task")??;
    let mut insert = sqlx::QueryBuilder::<Postgres>::new(
        "INSERT INTO ops.receipt_audit(job_run_id,receipt_id,outcome,observed_value_sha256) ",
    );
    insert.push_values(&results, |mut row, result| {
        row.push_bind(job)
            .push_bind(result.receipt)
            .push_bind(result.outcome)
            .push_bind(&result.observed);
    });
    sqlx::query("SELECT pg_advisory_xact_lock(742110026114::bigint)")
        .execute(&mut *tx)
        .await?;
    insert.build().execute(&mut *tx).await?;
    tx.commit().await?;
    Ok(results)
}

fn verify(bytes: &[u8], receipts: &[Receipt], raw_mismatch: bool) -> Result<Vec<AuditResult>> {
    let mut groups: BTreeMap<&str, Vec<&Receipt>> = BTreeMap::new();
    for receipt in receipts {
        groups.entry(&receipt.kind).or_default().push(receipt);
    }
    let mut results = Vec::with_capacity(receipts.len());
    for (kind, group) in groups {
        let locators: Vec<_> = group
            .iter()
            .map(|receipt| receipt.locator.as_str())
            .collect();
        let hashes = if raw_mismatch && matches!(kind, "json_pointer" | "kv_pointer") {
            group
                .iter()
                .map(|receipt| AuditResult {
                    receipt: receipt.id,
                    outcome: "mismatch",
                    observed: None,
                })
                .collect::<Vec<_>>()
        } else {
            raw_locator_hashes(bytes, kind, &locators)?
                .into_iter()
                .zip(group)
                .map(|(hash, receipt)| {
                    let (outcome, observed) = match hash {
                        RawLocatorHash::Found(hash) => (
                            if hash == receipt.hash {
                                "match"
                            } else {
                                "mismatch"
                            },
                            Some(hash),
                        ),
                        RawLocatorHash::LocatorNotFound => ("locator_not_found", None),
                        RawLocatorHash::UnsupportedLocatorKind => {
                            ("unsupported_locator_kind", None)
                        }
                    };
                    AuditResult {
                        receipt: receipt.id,
                        outcome,
                        observed,
                    }
                })
                .collect()
        };
        results.extend(hashes);
    }
    Ok(results)
}
