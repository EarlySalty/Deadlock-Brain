use crate::gold::{Expect, GoldCase};
use crate::matching::{contains_term, normalize};
use brain_client::{AnswerStatus, PublicAnswerResponse};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct FactResult {
    pub any_of: Vec<String>,
    pub matched: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct CaseResult {
    pub id: String,
    pub category: String,
    pub lang: String,
    pub question: String,
    pub expect: Expect,
    pub status: Option<AnswerStatus>,
    pub error: Option<String>,
    pub outcome_ok: bool,
    pub correct: bool,
    pub facts_matched: usize,
    pub facts_total: usize,
    pub facts: Vec<FactResult>,
    pub citation_count: usize,
    pub knowledge_release: Option<String>,
    pub latency_ms: u128,
    pub text: Option<String>,
}

pub fn score_response(
    case: &GoldCase,
    response: &PublicAnswerResponse,
    latency_ms: u128,
) -> CaseResult {
    let facts = match_facts(case, &response.text);
    let facts_matched = facts.iter().filter(|f| f.matched).count();
    let outcome_ok = outcome_matches(case.expect, response.status);
    let correct = match case.expect {
        Expect::Answered => outcome_ok && facts_matched == facts.len(),
        Expect::InsufficientEvidence => outcome_ok,
    };
    CaseResult {
        status: Some(response.status),
        error: None,
        outcome_ok,
        correct,
        facts_matched,
        facts_total: facts.len(),
        facts,
        citation_count: response.citations.len(),
        knowledge_release: Some(response.knowledge_release.clone()),
        latency_ms,
        text: Some(response.text.clone()),
        ..base(case)
    }
}

pub fn score_error(case: &GoldCase, error: String, latency_ms: u128) -> CaseResult {
    CaseResult {
        error: Some(error),
        latency_ms,
        ..base(case)
    }
}

fn base(case: &GoldCase) -> CaseResult {
    CaseResult {
        id: case.id.clone(),
        category: case.category.clone(),
        lang: case.lang.clone(),
        question: case.question.clone(),
        expect: case.expect,
        status: None,
        error: None,
        outcome_ok: false,
        correct: false,
        facts_matched: 0,
        facts_total: case.facts.len(),
        facts: Vec::new(),
        citation_count: 0,
        knowledge_release: None,
        latency_ms: 0,
        text: None,
    }
}

fn outcome_matches(expect: Expect, status: AnswerStatus) -> bool {
    match expect {
        Expect::Answered => status == AnswerStatus::Answered,
        Expect::InsufficientEvidence => status == AnswerStatus::InsufficientEvidence,
    }
}

fn match_facts(case: &GoldCase, text: &str) -> Vec<FactResult> {
    let normalized = normalize(text);
    case.facts
        .iter()
        .map(|fact| FactResult {
            any_of: fact.any_of.clone(),
            matched: fact.any_of.iter().any(|t| contains_term(&normalized, t)),
        })
        .collect()
}
