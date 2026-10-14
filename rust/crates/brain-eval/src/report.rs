use crate::gold::Expect;
use crate::score::CaseResult;
use brain_client::AnswerStatus;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Default, Serialize)]
pub struct Tally {
    pub cases: usize,
    pub answerable: usize,
    pub answered: usize,
    pub correct: usize,
    pub answered_with_citation: usize,
    pub unanswerable: usize,
    pub unanswerable_insufficient: usize,
    pub unanswerable_answered: usize,
    pub errors: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct Rates {
    pub answered_rate: f64,
    pub correctness_rate: f64,
    pub answered_precision: f64,
    pub citation_rate: f64,
    pub unanswerable_compliance: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Summary {
    pub totals: Tally,
    pub rates: Rates,
    pub by_category: BTreeMap<String, Tally>,
    pub by_lang: BTreeMap<String, Tally>,
    pub statuses: BTreeMap<String, usize>,
    pub knowledge_releases: BTreeSet<String>,
}

pub fn summarize(results: &[CaseResult]) -> Summary {
    let mut totals = Tally::default();
    let mut by_category: BTreeMap<String, Tally> = BTreeMap::new();
    let mut by_lang: BTreeMap<String, Tally> = BTreeMap::new();
    for result in results {
        add(&mut totals, result);
        add(
            by_category.entry(result.category.clone()).or_default(),
            result,
        );
        add(by_lang.entry(result.lang.clone()).or_default(), result);
    }
    Summary {
        rates: rates(&totals),
        totals,
        by_category,
        by_lang,
        statuses: statuses(results),
        knowledge_releases: results
            .iter()
            .filter_map(|r| r.knowledge_release.clone())
            .collect(),
    }
}

fn add(tally: &mut Tally, result: &CaseResult) {
    let answered = result.status == Some(AnswerStatus::Answered);
    tally.cases += 1;
    tally.errors += usize::from(result.error.is_some());
    match result.expect {
        Expect::Answered => {
            tally.answerable += 1;
            tally.answered += usize::from(answered);
            tally.correct += usize::from(result.correct);
            tally.answered_with_citation += usize::from(answered && result.citation_count > 0);
        }
        Expect::InsufficientEvidence => {
            tally.unanswerable += 1;
            tally.unanswerable_insufficient += usize::from(result.outcome_ok);
            tally.unanswerable_answered += usize::from(answered);
        }
    }
}

fn rates(t: &Tally) -> Rates {
    Rates {
        answered_rate: ratio(t.answered, t.answerable),
        correctness_rate: ratio(t.correct, t.answerable),
        answered_precision: ratio(t.correct, t.answered),
        citation_rate: ratio(t.answered_with_citation, t.answered),
        unanswerable_compliance: ratio(t.unanswerable_insufficient, t.unanswerable),
    }
}

fn ratio(part: usize, whole: usize) -> f64 {
    if whole == 0 {
        return 0.0;
    }
    let value = part as f64 / whole as f64;
    (value * 10_000.0).round() / 10_000.0
}

fn statuses(results: &[CaseResult]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for result in results {
        *counts.entry(status_label(result)).or_insert(0) += 1;
    }
    counts
}

fn status_label(result: &CaseResult) -> String {
    match result.status {
        Some(status) => serde_json::to_value(status)
            .ok()
            .and_then(|v| v.as_str().map(str::to_owned))
            .unwrap_or_else(|| format!("{status:?}")),
        None => "transport_error".to_owned(),
    }
}

pub fn render_text(summary: &Summary) -> String {
    let t = &summary.totals;
    let r = &summary.rates;
    let mut lines = vec![
        format!(
            "cases {} (answerable {}, non-answerable {}, errors {})",
            t.cases, t.answerable, t.unanswerable, t.errors
        ),
        format!(
            "answered rate {:.1}% ({}/{})",
            pct(r.answered_rate),
            t.answered,
            t.answerable
        ),
        format!(
            "correctness rate {:.1}% ({}/{})",
            pct(r.correctness_rate),
            t.correct,
            t.answerable
        ),
        format!(
            "answered precision {:.1}% ({}/{})",
            pct(r.answered_precision),
            t.correct,
            t.answered
        ),
        format!(
            "citation rate {:.1}% ({}/{})",
            pct(r.citation_rate),
            t.answered_with_citation,
            t.answered
        ),
        format!(
            "non-answerable compliance {:.1}% ({}/{}, answered {})",
            pct(r.unanswerable_compliance),
            t.unanswerable_insufficient,
            t.unanswerable,
            t.unanswerable_answered
        ),
    ];
    lines.push(format!("statuses {}", join_counts(&summary.statuses)));
    for (category, c) in &summary.by_category {
        lines.push(format!(
            "  {category}: cases {} answered {}/{} correct {} non-answerable ok {}/{}",
            c.cases,
            c.answered,
            c.answerable,
            c.correct,
            c.unanswerable_insufficient,
            c.unanswerable
        ));
    }
    lines.join("\n")
}

fn pct(value: f64) -> f64 {
    value * 100.0
}

fn join_counts(counts: &BTreeMap<String, usize>) -> String {
    counts
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join(" ")
}
