use crate::gold::GoldCase;
use crate::score::{self, Answer, CaseResult};
use anyhow::{bail, Context, Result};
use brain_client::AnswerStatus;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Deserialize)]
pub struct StoredReport {
    pub schema: String,
    pub endpoint: String,
    pub scopes: Vec<String>,
    pub started_unix: u64,
    pub duration_ms: u128,
    pub cases: Vec<StoredCase>,
}

#[derive(Deserialize)]
pub struct StoredCase {
    id: String,
    status: Option<AnswerStatus>,
    error: Option<String>,
    citation_count: usize,
    knowledge_release: Option<String>,
    latency_ms: u128,
    text: Option<String>,
}

pub fn load(path: &Path) -> Result<StoredReport> {
    let bytes = std::fs::read(path).with_context(|| format!("read {}", path.display()))?;
    let report: StoredReport = serde_json::from_slice(&bytes).context("parse report")?;
    if report.schema != crate::REPORT_SCHEMA {
        bail!("unexpected report schema {}", report.schema);
    }
    Ok(report)
}

pub fn rescore(gold: &[GoldCase], stored: Vec<StoredCase>) -> Result<Vec<CaseResult>> {
    let by_id: BTreeMap<&str, &GoldCase> = gold.iter().map(|c| (c.id.as_str(), c)).collect();
    stored
        .into_iter()
        .map(|case| {
            let gold_case = by_id
                .get(case.id.as_str())
                .with_context(|| format!("case {} is not in the gold file", case.id))?;
            rescore_case(gold_case, case)
        })
        .collect()
}

fn rescore_case(gold_case: &GoldCase, stored: StoredCase) -> Result<CaseResult> {
    let StoredCase {
        id,
        status,
        error,
        citation_count,
        knowledge_release,
        latency_ms,
        text,
    } = stored;
    match (status, text, error) {
        (Some(status), Some(text), None) => Ok(score::score_answer(
            gold_case,
            Answer {
                status,
                text,
                citation_count,
                knowledge_release,
                latency_ms,
            },
        )),
        (None, None, Some(error)) => Ok(score::score_error(gold_case, error, latency_ms)),
        _ => bail!("case {id} has neither a complete answer nor an error"),
    }
}
