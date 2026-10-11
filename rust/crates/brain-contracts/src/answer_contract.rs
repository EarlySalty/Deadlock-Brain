use std::collections::BTreeSet;

use crate::{Evidence, ProviderAnswer};

pub const NO_SUPPORTED_ANSWER: &str = "Dazu habe ich kein passendes Spielobjekt mit gesicherten Angaben gefunden. Namen oder Werte rate ich nicht.";

pub fn is_safe_answer(answer: &ProviderAnswer) -> bool {
    answer.text == NO_SUPPORTED_ANSWER && answer.cited_evidence_ids.is_empty()
}

pub fn has_authorized_citations(evidence: &[Evidence], answer: &ProviderAnswer) -> bool {
    let ids: BTreeSet<_> = answer.cited_evidence_ids.iter().collect();
    !answer.text.trim().is_empty()
        && answer.text.len() <= 64 * 1024
        && !ids.is_empty()
        && ids.len() == answer.cited_evidence_ids.len()
        && ids.iter().all(|id| {
            evidence.iter().filter(|item| &item.evidence_id == *id).count() == 1
        })
}

pub fn enforce(evidence: &[Evidence], answer: &mut ProviderAnswer) -> bool {
    if is_safe_answer(answer) || has_authorized_citations(evidence, answer) {
        return false;
    }
    answer.text = NO_SUPPORTED_ANSWER.into();
    answer.cited_evidence_ids.clear();
    true
}

pub fn safe_answer(usage: crate::Usage) -> ProviderAnswer {
    ProviderAnswer {
        text: NO_SUPPORTED_ANSWER.into(),
        cited_evidence_ids: Vec::new(),
        usage,
    }
}
