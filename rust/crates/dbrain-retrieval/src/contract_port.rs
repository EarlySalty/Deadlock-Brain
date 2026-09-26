use std::collections::BTreeSet;

use brain_contracts::{
    AuthorizedContext, Evidence, EvidenceKind, PortError, Query, RetrievalPort, SourceRecordV2,
    SourceVisibility,
};

#[derive(Debug, Clone)]
pub struct LexicalRetriever {
    records: Vec<SourceRecordV2>,
    limit: usize,
}

impl LexicalRetriever {
    pub fn new(records: Vec<SourceRecordV2>, limit: usize) -> Self {
        Self {
            records,
            limit: limit.max(1),
        }
    }
}

impl RetrievalPort for LexicalRetriever {
    fn retrieve(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Result<Vec<Evidence>, PortError> {
        let query_tokens = tokens(&query.text);
        if query_tokens.is_empty() {
            return Ok(Vec::new());
        }

        let mut scored = self
            .records
            .iter()
            .filter(|record| !record.tombstone)
            .filter(|record| record_allowed(record, &context.principal.scopes))
            .filter(|record| patch_allowed(record, query.patch.as_deref()))
            .filter_map(|record| {
                let content_tokens = tokens(&record.content);
                let logical_tokens = tokens(&record.logical_id);
                let overlap = query_tokens
                    .iter()
                    .filter(|token| {
                        content_tokens.contains(*token) || logical_tokens.contains(*token)
                    })
                    .count();
                if overlap == 0 {
                    return None;
                }

                let coverage = overlap as f64 / query_tokens.len() as f64;
                let phrase_bonus = if record
                    .content
                    .to_lowercase()
                    .contains(&query.text.to_lowercase())
                {
                    0.5
                } else {
                    0.0
                };
                let score = coverage + phrase_bonus;
                let evidence = Evidence {
                    evidence_id: format!(
                        "{}:{}:{}",
                        record.source_id, record.logical_id, record.revision
                    ),
                    source_id: record.source_id.clone(),
                    logical_id: record.logical_id.clone(),
                    revision: record.revision,
                    kind: evidence_kind(record),
                    content: record.content.clone(),
                    citation: format!(
                        "{}:{}@{}",
                        record.source_id, record.logical_id, record.revision
                    ),
                    visibility: record.visibility,
                    allowed_scopes: record.allowed_scopes.clone(),
                    score,
                    patch: record.metadata.get("patch").cloned(),
                };
                Some(evidence)
            })
            .collect::<Vec<_>>();

        scored.sort_by(|left, right| {
            right
                .score
                .total_cmp(&left.score)
                .then_with(|| left.source_id.cmp(&right.source_id))
                .then_with(|| left.logical_id.cmp(&right.logical_id))
                .then_with(|| right.revision.cmp(&left.revision))
        });
        scored.truncate(self.limit);
        Ok(scored)
    }
}

fn record_allowed(record: &SourceRecordV2, scopes: &BTreeSet<String>) -> bool {
    (record.visibility == SourceVisibility::Public || !record.allowed_scopes.is_empty())
        && record.allowed_scopes.is_subset(scopes)
}

fn patch_allowed(record: &SourceRecordV2, requested_patch: Option<&str>) -> bool {
    match (
        requested_patch,
        record.metadata.get("patch").map(String::as_str),
    ) {
        (Some(requested), Some(record_patch)) => requested == record_patch,
        _ => true,
    }
}

fn evidence_kind(record: &SourceRecordV2) -> EvidenceKind {
    match record.metadata.get("kind").map(String::as_str) {
        Some("fact") => EvidenceKind::Fact,
        Some("rule") => EvidenceKind::Rule,
        Some("mechanic") => EvidenceKind::Mechanic,
        Some("population") => EvidenceKind::Population,
        Some("replay") => EvidenceKind::Replay,
        _ => EvidenceKind::Prose,
    }
}

fn tokens(value: &str) -> BTreeSet<String> {
    value
        .split(|ch: char| !ch.is_alphanumeric())
        .filter(|token| token.chars().count() >= 2)
        .map(str::to_lowercase)
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use brain_contracts::{
        AnswerProfile, Budget, Principal, Query, SourceRecordV2, SourceVisibility,
    };

    use super::*;

    fn record(
        logical_id: &str,
        content: &str,
        visibility: SourceVisibility,
        scopes: &[&str],
        patch: Option<&str>,
    ) -> SourceRecordV2 {
        SourceRecordV2 {
            source_id: "fixture".into(),
            logical_id: logical_id.into(),
            revision: 1,
            content_hash: logical_id.into(),
            content: content.into(),
            visibility,
            allowed_scopes: scopes.iter().map(|scope| (*scope).to_string()).collect(),
            tombstone: false,
            valid_from: None,
            valid_to: None,
            metadata: patch
                .map(|value| BTreeMap::from([("patch".into(), value.into())]))
                .unwrap_or_default(),
        }
    }

    fn context(scopes: &[&str]) -> AuthorizedContext {
        AuthorizedContext {
            principal: Principal {
                actor_id: "actor".into(),
                channel: "test".into(),
                scopes: scopes.iter().map(|scope| (*scope).to_string()).collect(),
                provider_egress: BTreeSet::new(),
            },
            conversation_id: "c1".into(),
            knowledge_release: "k1".into(),
            deadline_ms: 1000,
            budget: Budget::default(),
        }
    }

    fn query(text: &str, patch: Option<&str>) -> Query {
        Query {
            domain: None,
            request_id: "r1".into(),
            conversation_id: "c1".into(),
            text: text.into(),
            requested_scopes: BTreeSet::new(),
            profile: AnswerProfile::Explain,
            patch: patch.map(str::to_string),
            mode: None,
        }
    }

    #[test]
    fn restrictive_scope_is_filtered_before_ranking() {
        let retriever = LexicalRetriever::new(
            vec![
                record(
                    "public",
                    "Abrams shoulder charge",
                    SourceVisibility::Public,
                    &[],
                    None,
                ),
                record(
                    "private",
                    "Abrams private scrim note",
                    SourceVisibility::Private,
                    &["scrim.private"],
                    None,
                ),
            ],
            10,
        );

        let public = retriever
            .retrieve(&query("Abrams", None), &context(&[]))
            .unwrap();
        assert_eq!(public.len(), 1);
        assert_eq!(public[0].logical_id, "public");

        let private = retriever
            .retrieve(&query("Abrams", None), &context(&["scrim.private"]))
            .unwrap();
        assert_eq!(private.len(), 2);
    }

    #[test]
    fn internal_record_without_scope_is_fail_closed() {
        let retriever = LexicalRetriever::new(
            vec![record(
                "internal",
                "Abrams internal note",
                SourceVisibility::Internal,
                &[],
                None,
            )],
            10,
        );

        let hits = retriever
            .retrieve(&query("Abrams", None), &context(&["docs.internal"]))
            .unwrap();
        assert!(hits.is_empty());
    }

    #[test]
    fn patch_filter_and_stable_sort_are_deterministic() {
        let retriever = LexicalRetriever::new(
            vec![
                record(
                    "b",
                    "Geist Malice",
                    SourceVisibility::Public,
                    &[],
                    Some("p2"),
                ),
                record(
                    "a",
                    "Geist Malice",
                    SourceVisibility::Public,
                    &[],
                    Some("p1"),
                ),
            ],
            10,
        );

        let hits = retriever
            .retrieve(&query("Geist Malice", Some("p1")), &context(&[]))
            .unwrap();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].logical_id, "a");
    }

    #[test]
    fn rare_terms_work_without_dense_index() {
        let retriever = LexicalRetriever::new(
            vec![record(
                "hero/viscous",
                "Viscous Goo Ball",
                SourceVisibility::Public,
                &[],
                None,
            )],
            3,
        );
        let hits = retriever
            .retrieve(&query("Goo Ball", None), &context(&[]))
            .unwrap();
        assert_eq!(hits.len(), 1);
        assert!(hits[0].score > 0.0);
    }
}
