//! Authoritative retrieval over an immutable release plus CURRENT restrictions.
use crate::contract_port::LexicalRetriever;
use brain_contracts::{
    AuthorizedContext, CorpusSnapshot, Evidence, PortError, Query, RetrievalPort, SnapshotReadPort,
    SourceRecordV2,
};
use sha2::{Digest, Sha256};

#[derive(Clone)]
pub struct ReleaseRetriever<S> {
    store: S,
    limit: usize,
}
impl<S: SnapshotReadPort> ReleaseRetriever<S> {
    pub fn new(store: S, limit: usize) -> Self {
        Self {
            store,
            limit: limit.clamp(1, 100),
        }
    }
    pub fn snapshot(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Result<CorpusSnapshot, PortError> {
        query.validate().map_err(|_| invalid("invalid query"))?;
        if context.deadline_ms == 0 || query.conversation_id != context.conversation_id {
            return Err(PortError::BudgetExceeded);
        }
        let snapshot = self.store.read_snapshot(&context.knowledge_release)?;
        if snapshot.release.release_id != context.knowledge_release
            || snapshot.release.release_id == "current"
            || query
                .patch
                .as_ref()
                .is_some_and(|p| p != &snapshot.release.patch)
        {
            return Err(invalid("release or patch mismatch"));
        }
        Ok(snapshot)
    }
    pub fn records(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        provider: bool,
    ) -> Result<Vec<SourceRecordV2>, PortError> {
        let snapshot = self.snapshot(query, context)?;
        let mut records = snapshot.authorized(&context.principal, provider)?;
        records.retain(|r| {
            r.metadata
                .get("patch")
                .is_none_or(|p| p == &snapshot.release.patch)
                && query
                    .mode
                    .as_ref()
                    .is_none_or(|m| r.metadata.get("mode") == Some(m))
        });
        for record in &mut records {
            record
                .metadata
                .entry("patch".into())
                .or_insert_with(|| snapshot.release.patch.clone());
        }
        Ok(records)
    }
}
impl<S: SnapshotReadPort> RetrievalPort for ReleaseRetriever<S> {
    fn retrieve(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Result<Vec<Evidence>, PortError> {
        let records = self.records(query, context, false)?;
        let hits = LexicalRetriever::new(records, self.limit).retrieve(query, context)?;
        Ok(hits.into_iter().map(canonical_id).collect())
    }
    fn validate_evidence(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
        for_provider: bool,
    ) -> Result<(), PortError> {
        if evidence.is_empty() || evidence.len() > 100 {
            return Err(invalid("invalid evidence pack size"));
        }
        let records = self.records(query, context, for_provider)?;
        let mut seen = std::collections::BTreeSet::new();
        for item in evidence {
            item.validate().map_err(|_| invalid("invalid evidence"))?;
            if !seen.insert(&item.evidence_id) {
                return Err(invalid("duplicate evidence"));
            }
            let record = records
                .iter()
                .find(|r| {
                    r.source_id == item.source_id
                        && r.logical_id == item.logical_id
                        && r.revision == item.revision
                })
                .ok_or_else(|| invalid("evidence no longer authorized in release"))?;
            let canonical = evidence_for_record(record, item.score);
            if &canonical != item {
                return Err(invalid("evidence content or provenance mismatch"));
            }
        }
        Ok(())
    }
}
pub(crate) fn invalid(message: &str) -> PortError {
    PortError::InvalidResponse(message.into())
}
pub(crate) fn canonical_id(mut item: Evidence) -> Evidence {
    let identity = serde_json::to_vec(&(&item.source_id, &item.logical_id, item.revision))
        .expect("string tuple serializes");
    item.evidence_id = format!("ev-{:x}", Sha256::digest(identity));
    item.citation = format!("brain:{}@{}", item.evidence_id, item.revision);
    item
}
pub(crate) fn evidence_for_record(record: &SourceRecordV2, score: f64) -> Evidence {
    use brain_contracts::EvidenceKind;
    canonical_id(Evidence {
        evidence_id: String::new(),
        source_id: record.source_id.clone(),
        logical_id: record.logical_id.clone(),
        revision: record.revision,
        kind: match record.metadata.get("kind").map(String::as_str) {
            Some("fact") => EvidenceKind::Fact,
            Some("rule") => EvidenceKind::Rule,
            Some("mechanic") => EvidenceKind::Mechanic,
            Some("population") => EvidenceKind::Population,
            Some("replay") => EvidenceKind::Replay,
            _ => EvidenceKind::Prose,
        },
        content: record.content.clone(),
        citation: String::new(),
        visibility: record.visibility,
        allowed_scopes: record.allowed_scopes.clone(),
        score,
        patch: record.metadata.get("patch").cloned(),
    })
}
