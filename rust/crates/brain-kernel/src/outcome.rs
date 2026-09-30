//! In-process result only. Validation inputs are deliberately NOT part of any wire contract.
use super::*;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub(super) struct KernelAnswer {
    pub answer: AnswerResponse,
    // One immutable pack is shared by cache and flight waiters; cloning a result does not
    // duplicate uncited source bodies. Citations remain a separately selected public subset.
    pub dependencies: Arc<[Evidence]>,
}
impl From<AnswerResponse> for KernelAnswer {
    fn from(answer: AnswerResponse) -> Self {
        Self {
            dependencies: answer.citations.clone().into(),
            answer,
        }
    }
}
impl KernelAnswer {
    /// Conservative retained-payload accounting, not an estimate of total process RSS.
    /// Include string capacities and generous per-tree-entry overhead without serializing
    /// source contents or allocating another buffer. Saturate rather than wrap on overflow.
    pub fn retained_bytes(&self) -> usize {
        let strings = [
            &self.answer.contract_version,
            &self.answer.request_id,
            &self.answer.knowledge_release,
            &self.answer.text,
        ];
        let mut bytes = strings
            .into_iter()
            .fold(std::mem::size_of::<Self>(), |n, s| {
                n.saturating_add(s.capacity())
            });
        for evidence in self.dependencies.iter().chain(&self.answer.citations) {
            bytes = bytes.saturating_add(evidence_bytes(evidence));
        }
        for value in [&self.answer.usage.provider, &self.answer.usage.model]
            .into_iter()
            .flatten()
        {
            bytes = bytes.saturating_add(value.capacity());
        }
        bytes
    }
}
fn evidence_bytes(e: &Evidence) -> usize {
    let mut bytes = std::mem::size_of::<Evidence>();
    for s in [
        &e.evidence_id,
        &e.source_id,
        &e.logical_id,
        &e.content,
        &e.citation,
    ]
    .into_iter()
    .chain(e.patch.iter())
    {
        bytes = bytes.saturating_add(s.capacity());
    }
    for scope in &e.allowed_scopes {
        bytes = bytes.saturating_add(256).saturating_add(scope.capacity());
    }
    if let Some(p) = &e.provenance {
        for s in [
            &p.document.source_id,
            &p.document.logical_id,
            &p.document.content_hash,
            &p.chunker_version,
            &p.source_locator,
            &p.release_id,
            &p.knowledge_version,
        ]
        .into_iter()
        .chain(p.valid_from.iter())
        .chain(p.valid_to.iter())
        {
            bytes = bytes.saturating_add(s.capacity());
        }
        for (key, value) in &p.metadata {
            bytes = bytes
                .saturating_add(256)
                .saturating_add(key.capacity())
                .saturating_add(value.capacity());
        }
    }
    bytes
}
