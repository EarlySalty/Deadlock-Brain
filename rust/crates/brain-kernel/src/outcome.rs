use super::*;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub(super) struct KernelAnswer {
    pub answer: AnswerResponse,
    pub accounting: UsageAccounting,
    pub dependencies: Arc<[Evidence]>,
    pub tool_dependencies: Arc<[ToolEvidenceDependency]>,
    pub build_executions: Arc<[brain_contracts::ToolExecution]>,
    pub trusted_rendering: bool,
}
impl From<AnswerResponse> for KernelAnswer {
    fn from(answer: AnswerResponse) -> Self {
        Self {
            accounting: UsageAccounting::observed(answer.usage.clone()),
            dependencies: answer.citations.clone().into(),
            tool_dependencies: Arc::from([]),
            build_executions: Arc::from([]),
            trusted_rendering: false,
            answer,
        }
    }
}
impl KernelAnswer {
    pub fn with_trusted_rendering(mut self) -> Self {
        self.trusted_rendering = true;
        self
    }

    pub fn enforce_answer_contract(&mut self, query: &Query) {
        if self.trusted_rendering
            || !matches!(
                self.answer.status,
                AnswerStatus::Answered | AnswerStatus::Unverified
            )
        {
            return;
        }
        let mut evidence = self.dependencies.to_vec();
        for item in self
            .tool_dependencies
            .iter()
            .flat_map(|dependency| &dependency.evidence)
        {
            if !evidence.contains(item) {
                evidence.push(item.clone());
            }
        }
        let mut candidate = brain_contracts::ProviderAnswer {
            text: self.answer.text.clone(),
            cited_evidence_ids: self
                .answer
                .citations
                .iter()
                .map(|item| item.evidence_id.clone())
                .collect(),
            usage: self.answer.usage.clone(),
        };
        if brain_contracts::answer_contract::enforce(query, &evidence, &mut candidate) {
            self.answer.text = candidate.text;
            self.answer.citations.clear();
            self.answer.status = AnswerStatus::Unverified;
            eprintln!(
                "{}",
                serde_json::json!({"event":"brain_answer_unverified","request_id":query.request_id,"reason":"output_original_unbound"})
            );
        }
    }

    pub fn with_accounting(mut self, accounting: UsageAccounting) -> Self {
        self.answer.usage = accounting.observed.clone();
        self.accounting = accounting;
        self
    }

    pub fn into_accounted(self) -> Accounted<AnswerResponse> {
        Accounted {
            value: self.answer,
            accounting: self.accounting,
        }
    }

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
        for dependency in self.tool_dependencies.iter() {
            bytes = bytes.saturating_add(std::mem::size_of::<ToolEvidenceDependency>());
            bytes = bytes.saturating_add(json_bytes(dependency.request.arguments()));
            let typed_bytes = serde_json::to_value(dependency.request.subrequest())
                .map(|value| json_bytes(&value))
                .unwrap_or(usize::MAX);
            bytes = bytes.saturating_add(typed_bytes);
            if let Some(pin) = &dependency.game_context {
                bytes = bytes.saturating_add(pin.mechanic_revision.capacity());
            }
            bytes = bytes.saturating_add(
                dependency
                    .evidence
                    .capacity()
                    .saturating_mul(std::mem::size_of::<Evidence>()),
            );
            for evidence in &dependency.evidence {
                bytes = bytes.saturating_add(evidence_bytes(evidence));
            }
        }
        for execution in self.build_executions.iter() {
            bytes = bytes.saturating_add(std::mem::size_of::<brain_contracts::ToolExecution>());
            bytes = bytes.saturating_add(json_bytes(&execution.result.result));
            bytes = bytes.saturating_add(
                execution
                    .result
                    .evidence_ids
                    .capacity()
                    .saturating_mul(std::mem::size_of::<String>()),
            );
            bytes = bytes.saturating_add(execution.result.call_id.capacity());
            bytes = bytes.saturating_add(
                execution
                    .dependencies
                    .capacity()
                    .saturating_mul(std::mem::size_of::<ToolEvidenceDependency>()),
            );
            for value in [&execution.usage.provider, &execution.usage.model]
                .into_iter()
                .flatten()
            {
                bytes = bytes.saturating_add(value.capacity());
            }
            for id in &execution.result.evidence_ids {
                bytes = bytes.saturating_add(id.capacity());
            }
            for dependency in &execution.dependencies {
                bytes = bytes.saturating_add(json_bytes(dependency.request.arguments()));
                let typed_bytes = serde_json::to_value(dependency.request.subrequest())
                    .map(|value| json_bytes(&value))
                    .unwrap_or(usize::MAX);
                bytes = bytes.saturating_add(typed_bytes);
                if let Some(pin) = &dependency.game_context {
                    bytes = bytes.saturating_add(pin.mechanic_revision.capacity());
                }
                bytes = bytes.saturating_add(
                    dependency
                        .evidence
                        .capacity()
                        .saturating_mul(std::mem::size_of::<Evidence>()),
                );
                for evidence in &dependency.evidence {
                    bytes = bytes.saturating_add(evidence_bytes(evidence));
                }
            }
        }
        for value in [
            &self.answer.usage.provider,
            &self.answer.usage.model,
            &self.accounting.observed.provider,
            &self.accounting.observed.model,
            &self.accounting.reserved.provider,
            &self.accounting.reserved.model,
        ]
        .into_iter()
        .flatten()
        {
            bytes = bytes.saturating_add(value.capacity());
        }
        bytes
    }
}
fn json_bytes(value: &serde_json::Value) -> usize {
    let base = std::mem::size_of::<serde_json::Value>();
    match value {
        serde_json::Value::String(text) => base.saturating_add(text.capacity()),
        serde_json::Value::Array(values) => values.iter().fold(
            base.saturating_add(values.capacity().saturating_mul(base)),
            |bytes, value| bytes.saturating_add(json_bytes(value)),
        ),
        serde_json::Value::Object(values) => values.iter().fold(base, |bytes, (key, value)| {
            bytes
                .saturating_add(256)
                .saturating_add(key.capacity())
                .saturating_add(json_bytes(value))
        }),
        _ => base,
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
            accounting: UsageAccounting::default(),
            dependencies: vec![dependency].into(),
            tool_dependencies: Arc::from([]),
            build_executions: Arc::from([]),
            trusted_rendering: false,
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
