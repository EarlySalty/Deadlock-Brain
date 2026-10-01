use crate::nonnegative;
use std::collections::BTreeSet;

/// Paired binary labels; one-sided 95% Hoeffding bound for differences in [-1,1].
/// The caller must establish independent matches/source families before using this.
/// Weak evidence must not be presented as proof of noninferiority.
pub fn paired_noninferiority(
    baseline: &[bool],
    candidate: &[bool],
    margin: f64,
) -> Result<(f64, f64, bool), String> {
    if baseline.is_empty()
        || baseline.len() != candidate.len()
        || !nonnegative(margin)
        || margin > 1.0
    {
        return Err("invalid paired labels or margin".into());
    }
    let n = baseline.len() as f64;
    let delta = baseline
        .iter()
        .zip(candidate)
        .map(|(a, b)| f64::from(u8::from(*b)) - f64::from(u8::from(*a)))
        .sum::<f64>()
        / n;
    let lower = (delta - (2.0 * 20_f64.ln() / n).sqrt()).max(-1.0);
    Ok((delta, lower, lower >= -margin))
}
#[derive(Clone)]
pub struct TemporalRow {
    pub match_id: String,
    pub split: String,
    pub event_at: i64,
    pub available_at: i64,
}
pub fn temporal_holdout(rows: &[TemporalRow], cutoff: i64) -> Result<(), String> {
    let mut ids = BTreeSet::new();
    let mut splits = BTreeSet::new();
    for row in rows {
        if row.match_id.trim().is_empty()
            || !ids.insert(&row.match_id)
            || row.available_at < row.event_at
        {
            return Err("duplicate match or invalid availability".into());
        }
        splits.insert(row.split.as_str());
        match row.split.as_str() {
            "train" if row.event_at <= cutoff && row.available_at <= cutoff => {}
            "test" if row.event_at > cutoff => {}
            _ => return Err("invalid temporal split or future training information".into()),
        }
    }
    if splits != BTreeSet::from(["train", "test"]) {
        return Err("train and holdout evidence both required".into());
    }
    Ok(())
}
