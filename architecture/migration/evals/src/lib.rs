#![forbid(unsafe_code)]
//! Offline S10 evidence assessment, not a production API or release gate.
mod assessment;
pub mod model;
mod quality;
mod statistics;
pub use assessment::assess;
pub use model::*;
pub use quality::{paired_noninferiority, temporal_holdout, TemporalRow};
use sha2::{Digest, Sha256};
pub use statistics::{distribution, summarize};
use std::collections::BTreeSet;

pub fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
pub fn valid_hash(value: &str, length: usize) -> bool {
    value.len() == length
        && value
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
}
fn present(value: &Option<String>) -> bool {
    value.as_ref().is_some_and(|s| !s.trim().is_empty())
}
fn evidence(value: &Option<String>) -> bool {
    value.as_ref().is_some_and(|s| valid_hash(s, 64))
}
fn nonnegative(value: f64) -> bool {
    value.is_finite() && value >= 0.0
}
fn identifier(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || c == b'_' || c == b'-')
}
fn require(condition: bool, issues: &mut Vec<String>, message: &str) {
    if !condition {
        issues.push(message.into());
    }
}

pub fn validate_catalog(catalog: &Catalog) -> Result<(), String> {
    if catalog.schema_version != 1 || catalog.cases.len() < 40 {
        return Err("catalogue version or mandatory coverage invalid".into());
    }
    let canonical: Catalog = serde_json::from_str(include_str!("../cases.json"))
        .map_err(|_| "compiled catalogue invalid")?;
    let mut seen = BTreeSet::new();
    for case in &catalog.cases {
        let checks: BTreeSet<_> = case.checks.iter().collect();
        if !identifier(&case.id)
            || !seen.insert(&case.id)
            || case.scenario.trim().is_empty()
            || case.checks.is_empty()
            || checks.len() != case.checks.len()
            || !case.checks.iter().all(|v| identifier(v))
            || !["answer", "abstain", "deny", "degraded", "quarantine"]
                .contains(&case.expected.as_str())
        {
            return Err("invalid or duplicate case/assertion".into());
        }
    }
    // Required set is compiled and versioned, never inferred from available run results.
    for expected in canonical.cases {
        if !catalog.cases.iter().any(|c| c == &expected) {
            return Err("mandatory case changed or absent".into());
        }
    }
    Ok(())
}
fn accepted(case: &Case, sample: &Sample) -> bool {
    sample.outcome == case.expected
        && case.checks.iter().all(|id| {
            sample
                .checks
                .iter()
                .any(|c| &c.id == id && c.passed == Some(true) && evidence(&c.evidence_sha256))
        })
}
