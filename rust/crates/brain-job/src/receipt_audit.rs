use crate::{limits, RawObject, Receipt};
use brain_storage::entity_profile::derivation::game_file_facts::receipt_verify::{
    self as verifier, Format, Policy, ReceiptOutcome, Request, Verification,
};
use sqlx::{PgPool, Row};
use std::collections::BTreeMap;

const TEMPORARY_RAW_BYTES_POLICY_UNTIL_CORE2: usize = 16 * 1024 * 1024;
const SAMPLE_LIMIT: usize = 200;
const PARSER_BYTES: usize = 64 * 1024 * 1024;

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
    sqlx::raw_sql("SET TRANSACTION ISOLATION LEVEL READ COMMITTED; SET LOCAL statement_timeout='100000ms'; SET LOCAL lock_timeout='5000ms'; SELECT pg_advisory_xact_lock(742110026114::bigint)")
        .execute(&mut *tx).await?;
    let run_id = sqlx::query_scalar("INSERT INTO ops.receipt_audit_run(sampled,verified_receipts,rejected_receipts,rejected_objects) VALUES(0,0,0,0) RETURNING id").fetch_one(&mut *tx).await?;
    let rows = sqlx::query("SELECT p.id,p.raw_object_id,p.locator_kind,p.locator,p.value_sha256 FROM serve.published_receipt p LEFT JOIN LATERAL(SELECT a.run_id FROM ops.receipt_audit a WHERE a.receipt_id=p.id ORDER BY a.run_id DESC LIMIT 1) last ON true ORDER BY last.run_id NULLS FIRST,p.id LIMIT 200")
        .fetch_all(&mut *tx).await?;
    let mut report = Report {
        run_id,
        sampled: i32::try_from(rows.len())?,
        verified_receipts: 0,
        rejected_receipts: 0,
        rejected_objects: 0,
    };
    let mut objects: BTreeMap<i64, Vec<(i64, Receipt)>> = BTreeMap::new();
    for row in rows {
        objects
            .entry(row.try_get("raw_object_id")?)
            .or_default()
            .push((
                row.try_get("id")?,
                Receipt {
                    locator_kind: row.try_get("locator_kind")?,
                    locator: row.try_get("locator")?,
                    value_sha256: row
                        .try_get::<Vec<u8>, _>("value_sha256")?
                        .try_into()
                        .map_err(|_| anyhow::anyhow!("Ungültiger Beleg-Hash"))?,
                },
            ));
    }
    for (id, values) in objects {
        let requests: Vec<_> = values
            .iter()
            .map(|(_, receipt)| Request {
                receipt,
                explicitly_unsupported: false,
            })
            .collect();
        let results = if requests
            .iter()
            .any(|r| verifier::supported(Format::Json, r))
        {
            let row = sqlx::query("SELECT left(source,4096) AS source,left(revision,4096) AS revision,left(path,4096) AS path,content_sha256,CASE WHEN octet_length(bytes)<=$2 THEN bytes END AS bytes FROM raw.object WHERE id=$1")
                .bind(id).bind(i32::try_from(TEMPORARY_RAW_BYTES_POLICY_UNTIL_CORE2)?).fetch_one(&mut *tx).await?;
            let bytes: Option<Vec<u8>> = row.try_get("bytes")?;
            match bytes {
                None => requests
                    .iter()
                    .map(|r| {
                        if verifier::supported(Format::Json, r) {
                            Verification::ObjectRejected(verifier::Rejection::RawSizeLimit)
                        } else {
                            Verification::Receipt(ReceiptOutcome::UnsupportedLocatorKind)
                        }
                    })
                    .collect(),
                Some(bytes) => verifier::verify(
                    &RawObject {
                        id,
                        source: row.try_get("source")?,
                        revision: row.try_get("revision")?,
                        path: row.try_get("path")?,
                        bytes,
                        content_sha256: row
                            .try_get::<Vec<u8>, _>("content_sha256")?
                            .try_into()
                            .map_err(|_| anyhow::anyhow!("Ungültiger Rohobjekt-Hash"))?,
                    },
                    Format::Json,
                    &requests,
                    Policy {
                        max_raw_bytes: TEMPORARY_RAW_BYTES_POLICY_UNTIL_CORE2,
                        max_requests: SAMPLE_LIMIT,
                        max_locator_bytes: limits::LOCATOR_BYTES,
                        parser_allocation_bytes: PARSER_BYTES,
                    },
                )
                .map_err(|_| anyhow::anyhow!("Belegprüfung überschreitet die Anfragegrenze"))?,
            }
        } else {
            vec![Verification::Receipt(ReceiptOutcome::UnsupportedLocatorKind); values.len()]
        };
        if let Some(reason) = results.iter().find_map(|v| match v {
            Verification::ObjectRejected(r) => Some(r),
            _ => None,
        }) {
            report.rejected_objects += 1;
            let (code, detail) = rejection(*reason);
            sqlx::query("INSERT INTO ops.receipt_audit_rejection(run_id,raw_object_id,code,detail) VALUES($1,$2,$3,$4)")
                .bind(run_id).bind(id).bind(code).bind(detail).execute(&mut *tx).await?;
        }
        for ((receipt_id, _), result) in values.iter().zip(results) {
            let (outcome, hash) = match result {
                Verification::Receipt(ReceiptOutcome::Match(h)) => {
                    report.verified_receipts += 1;
                    ("match", Some(h))
                }
                Verification::Receipt(ReceiptOutcome::Mismatch(h)) => ("mismatch", Some(h)),
                Verification::Receipt(ReceiptOutcome::LocatorNotFound) => {
                    ("locator_not_found", None)
                }
                Verification::Receipt(ReceiptOutcome::UnsupportedLocatorKind) => {
                    ("unsupported_locator_kind", None)
                }
                Verification::ObjectRejected(_) => ("object_rejected", None),
            };
            if outcome != "match" {
                report.rejected_receipts += 1;
            }
            sqlx::query("INSERT INTO ops.receipt_audit(run_id,receipt_id,outcome,observed_value_sha256) VALUES($1,$2,$3,$4)")
                .bind(run_id).bind(receipt_id).bind(outcome).bind(hash.map(|h| h.to_vec())).execute(&mut *tx).await?;
        }
    }
    sqlx::query("UPDATE ops.receipt_audit_run SET finished=clock_timestamp(),sampled=$2,verified_receipts=$3,rejected_receipts=$4,rejected_objects=$5 WHERE id=$1")
        .bind(run_id).bind(report.sampled).bind(report.verified_receipts).bind(report.rejected_receipts).bind(report.rejected_objects).execute(&mut *tx).await?;
    tx.commit().await?;
    Ok(report)
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
