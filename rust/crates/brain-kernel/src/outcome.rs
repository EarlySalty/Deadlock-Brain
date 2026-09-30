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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn uncited_dependency_pack_is_shared_accounted_and_absent_from_wire_response() {
        let dependency = Evidence {
            evidence_id: "uncited".into(),
            source_id: "fixture".into(),
            logical_id: "uncited".into(),
            revision: 1,
            kind: brain_contracts::EvidenceKind::Prose,
            content: "x".repeat(256 * 1024),
            citation: "fixture".into(),
            visibility: brain_contracts::SourceVisibility::Public,
            allowed_scopes: Default::default(),
            score: 1.0,
            provenance: None,
            patch: None,
        };
        let bytes = dependency.content.capacity();
        let outcome = KernelAnswer {
            answer: AnswerResponse {
                contract_version: CONTRACT_VERSION.into(),
                request_id: "test".into(),
                knowledge_release: "release".into(),
                status: AnswerStatus::Answered,
                text: "answer".into(),
                citations: Vec::new(),
                usage: Usage::default(),
            },
            dependencies: vec![dependency].into(),
        };
        assert!(outcome.retained_bytes() >= bytes);
        let reused = outcome.clone();
        assert!(Arc::ptr_eq(&outcome.dependencies, &reused.dependencies));
        assert_eq!(Arc::strong_count(&outcome.dependencies), 2);
        let public = serde_json::to_value(&outcome.answer).unwrap();
        assert!(public.get("dependencies").is_none());
        assert!(!public.to_string().contains("uncited"));
    }
}
