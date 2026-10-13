use crate::{limits, RawObject, Receipt};
use brain_storage::entity_profile::derivation::game_file_facts::receipt_verify::{
    self as verifier, Format, Policy, ReceiptOutcome, Request, Verification,
};
use sqlx::{postgres::PgRow, PgConnection, PgPool, Row};
use std::collections::BTreeMap;

const SAMPLE_LIMIT: usize = 200;
const PARSER_BYTES: usize = 64 * 1024 * 1024;
const SAMPLE_SQL: &str =
    "SELECT id,raw_object_id,locator_kind,locator,value_sha256 FROM ops.sample_receipt_audit()";

type Sample = BTreeMap<i64, Vec<(i64, Receipt)>>;

#[derive(Debug)]
pub struct Report {
    pub run_id: i64,
    pub sampled: i32,
    pub verified_receipts: i32,
    pub rejected_receipts: i32,
    pub rejected_objects: i32,
}

pub async fn run(pool: &PgPool) -> anyhow::Result<Report> {
    let mut tx = pool.begin().await?;
    let run_id = start_run(&mut tx).await?;
    let (sampled, objects) = sample(&mut tx).await?;
    let mut report = Report {
        run_id,
        sampled,
        verified_receipts: 0,
        rejected_receipts: 0,
        rejected_objects: 0,
    };
    for (id, values) in objects {
        let results = verify_object(&mut tx, id, &values).await?;
        record_rejection(&mut tx, &mut report, id, &results).await?;
        record_outcomes(&mut tx, &mut report, &values, results).await?;
    }
    finish_run(&mut tx, &report).await?;
    tx.commit().await?;
    Ok(report)
}

async fn start_run(connection: &mut PgConnection) -> anyhow::Result<i64> {
    sqlx::raw_sql("SET TRANSACTION ISOLATION LEVEL READ COMMITTED; SET LOCAL statement_timeout='100000ms'; SET LOCAL lock_timeout='5000ms'; SELECT pg_advisory_xact_lock(742110026114::bigint)")
        .execute(&mut *connection).await?;
    Ok(sqlx::query_scalar("INSERT INTO ops.receipt_audit_run(sampled,verified_receipts,rejected_receipts,rejected_objects) VALUES(0,0,0,0) RETURNING id").fetch_one(connection).await?)
}

async fn sample(connection: &mut PgConnection) -> anyhow::Result<(i32, Sample)> {
    let rows = sqlx::query(SAMPLE_SQL).fetch_all(connection).await?;
    let mut objects = Sample::new();
    for row in &rows {
        let (object, receipt) = sampled_receipt(row)?;
        objects.entry(object).or_default().push(receipt);
    }
    Ok((i32::try_from(rows.len())?, objects))
}

fn sampled_receipt(row: &PgRow) -> anyhow::Result<(i64, (i64, Receipt))> {
    let receipt = Receipt {
        locator_kind: row.try_get("locator_kind")?,
        locator: row.try_get("locator")?,
        value_sha256: row
            .try_get::<Vec<u8>, _>("value_sha256")?
            .try_into()
            .map_err(|_| anyhow::anyhow!("Ungültiger Beleg-Hash"))?,
    };
    Ok((row.try_get("raw_object_id")?, (row.try_get("id")?, receipt)))
}

async fn verify_object(
    connection: &mut PgConnection,
    id: i64,
    values: &[(i64, Receipt)],
) -> anyhow::Result<Vec<Verification>> {
    let requests: Vec<_> = values
        .iter()
        .map(|(_, receipt)| Request {
            receipt,
            explicitly_unsupported: false,
        })
        .collect();
    if !requests
        .iter()
        .any(|r| verifier::supported(Format::Json, r))
    {
        return Ok(vec![
            Verification::Receipt(
                ReceiptOutcome::UnsupportedLocatorKind
            );
            values.len()
        ]);
    }
    match guarded_object(connection, id).await? {
        Some(raw) => verifier::verify(&raw, Format::Json, &requests, policy())
            .map_err(|_| anyhow::anyhow!("Belegprüfung überschreitet die Anfragegrenze")),
        None => Ok(oversized(&requests)),
    }
}

fn policy() -> Policy {
    Policy {
        max_raw_bytes: limits::RAW_BYTES,
        max_requests: SAMPLE_LIMIT,
        max_locator_bytes: limits::LOCATOR_BYTES,
        parser_allocation_bytes: PARSER_BYTES,
    }
}

fn oversized(requests: &[Request<'_>]) -> Vec<Verification> {
    requests
        .iter()
        .map(|r| {
            if verifier::supported(Format::Json, r) {
                Verification::ObjectRejected(verifier::Rejection::RawSizeLimit)
            } else {
                Verification::Receipt(ReceiptOutcome::UnsupportedLocatorKind)
            }
        })
        .collect()
}

async fn guarded_object(
    connection: &mut PgConnection,
    id: i64,
) -> anyhow::Result<Option<RawObject>> {
    let row = sqlx::query("SELECT left(source,4096) AS source,left(revision,4096) AS revision,left(path,4096) AS path,content_sha256,CASE WHEN octet_length(bytes)<=$2 THEN bytes END AS bytes FROM raw.object WHERE id=$1")
        .bind(id).bind(i32::try_from(limits::RAW_BYTES)?).fetch_one(connection).await?;
    let Some(bytes) = row.try_get::<Option<Vec<u8>>, _>("bytes")? else {
        return Ok(None);
    };
    Ok(Some(RawObject {
        id,
        source: row.try_get("source")?,
        revision: row.try_get("revision")?,
        path: row.try_get("path")?,
        bytes,
        content_sha256: row
            .try_get::<Vec<u8>, _>("content_sha256")?
            .try_into()
            .map_err(|_| anyhow::anyhow!("Ungültiger Rohobjekt-Hash"))?,
    }))
}

async fn record_rejection(
    connection: &mut PgConnection,
    report: &mut Report,
    id: i64,
    results: &[Verification],
) -> anyhow::Result<()> {
    let Some(reason) = results.iter().find_map(|v| match v {
        Verification::ObjectRejected(r) => Some(*r),
        Verification::Receipt(_) => None,
    }) else {
        return Ok(());
    };
    report.rejected_objects += 1;
    let (code, detail) = rejection(reason);
    sqlx::query("INSERT INTO ops.receipt_audit_rejection(run_id,raw_object_id,code,detail) VALUES($1,$2,$3,$4)")
        .bind(report.run_id).bind(id).bind(code).bind(detail).execute(connection).await?;
    Ok(())
}

async fn record_outcomes(
    connection: &mut PgConnection,
    report: &mut Report,
    values: &[(i64, Receipt)],
    results: Vec<Verification>,
) -> anyhow::Result<()> {
    for ((receipt_id, _), result) in values.iter().zip(results) {
        let (outcome, hash) = outcome(result);
        if outcome == "match" {
            report.verified_receipts += 1;
        } else {
            report.rejected_receipts += 1;
        }
        sqlx::query("INSERT INTO ops.receipt_audit(run_id,receipt_id,outcome,observed_value_sha256) VALUES($1,$2,$3,$4)")
            .bind(report.run_id).bind(receipt_id).bind(outcome).bind(hash.map(|h| h.to_vec())).execute(&mut *connection).await?;
    }
    Ok(())
}

fn outcome(result: Verification) -> (&'static str, Option<[u8; 32]>) {
    match result {
        Verification::Receipt(ReceiptOutcome::Match(h)) => ("match", Some(h)),
        Verification::Receipt(ReceiptOutcome::Mismatch(h)) => ("mismatch", Some(h)),
        Verification::Receipt(ReceiptOutcome::LocatorNotFound) => ("locator_not_found", None),
        Verification::Receipt(ReceiptOutcome::UnsupportedLocatorKind) => {
            ("unsupported_locator_kind", None)
        }
        Verification::ObjectRejected(_) => ("object_rejected", None),
    }
}

async fn finish_run(connection: &mut PgConnection, report: &Report) -> anyhow::Result<()> {
    sqlx::query("UPDATE ops.receipt_audit_run SET finished=clock_timestamp(),sampled=$2,verified_receipts=$3,rejected_receipts=$4,rejected_objects=$5 WHERE id=$1")
        .bind(report.run_id).bind(report.sampled).bind(report.verified_receipts).bind(report.rejected_receipts).bind(report.rejected_objects).execute(connection).await?;
    Ok(())
}

fn rejection(reason: verifier::Rejection) -> (&'static str, &'static str) {
    match reason {
        verifier::Rejection::RawSizeLimit => {
            ("raw_size_limit", "Rohobjekt überschreitet die Byte-Grenze")
        }
        verifier::Rejection::LocatorSizeLimit => (
            "locator_size_limit",
            "Locator überschreitet die Byte-Grenze",
        ),
        verifier::Rejection::LocatorCollision => (
            "locator_collision",
            "Rohobjekt enthält kollidierende Locator-Schlüssel",
        ),
        verifier::Rejection::ContentHashMismatch => (
            "content_hash_mismatch",
            "Rohobjekt stimmt nicht mit seinem Inhalts-Hash überein",
        ),
        verifier::Rejection::InvalidUtf8 => ("invalid_utf8", "Rohobjekt enthält ungültiges UTF-8"),
        verifier::Rejection::RequestLimit => {
            ("request_limit", "Belegprüfung enthält zu viele Anfragen")
        }
        verifier::Rejection::Parser(_) => (
            "parser_rejection",
            "Rohobjekt ist ungültig oder überschreitet das Parserbudget",
        ),
    }
}
