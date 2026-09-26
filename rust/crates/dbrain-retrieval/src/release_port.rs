//! Immutable indexed release data plus fresh CURRENT ACL/delete checks on every handoff.
use crate::chunk_index::ChunkIndex;
use brain_contracts::{
    provider_input::grounded_input_ceiling, store::record_allowed, AuthorizedContext,
    CorpusSnapshot, DocumentHead, DocumentRevision, Evidence, PortError, Query, RetrievalPort,
    SnapshotReadPort, SourceRecordV2, SourceVisibility,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{Arc, Mutex},
};

type IndexCache = Arc<Mutex<BTreeMap<String, Arc<ChunkIndex>>>>;
#[derive(Clone)]
pub struct ReleaseRetriever<S> {
    store: S,
    limit: usize,
    indexes: IndexCache,
}
impl<S: SnapshotReadPort> ReleaseRetriever<S> {
    pub fn new(store: S, limit: usize) -> Self {
        Self {
            store,
            limit: limit.clamp(1, 100),
            indexes: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }
    /// Explicit bulk diagnostic read. The query and evidence-validation paths do not call this
    /// after the first index build for a release. Up to four immutable release indexes are kept.
    pub fn snapshot(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Result<CorpusSnapshot, PortError> {
        check_context(query, context)?;
        let snapshot = self.store.read_snapshot(&context.knowledge_release)?;
        check_release(&snapshot.release, query, context)?;
        Ok(snapshot)
    }
    fn index(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Result<Arc<ChunkIndex>, PortError> {
        check_context(query, context)?;
        // Hold build lock: concurrent cold requests must not load/build the same release twice.
        let mut indexes = self
            .indexes
            .lock()
            .map_err(|_| PortError::Unavailable("retrieval index lock unavailable".into()))?;
        if let Some(index) = indexes.get(&context.knowledge_release) {
            check_release(&index.release, query, context)?;
            return Ok(index.clone());
        }
        let snapshot = self.snapshot(query, context)?;
        // Validate ALL pins/revisions/heads; indexing is not scoped to the first caller.
        // The result is deliberately discarded. No mutable head/authorization is cached.
        snapshot.authorized(&context.principal, false)?;
        let index = Arc::new(ChunkIndex::build(snapshot.release, snapshot.revisions)?);
        if indexes.len() >= 4 {
            if let Some(key) = indexes.keys().next().cloned() {
                indexes.remove(&key);
            }
        }
        indexes.insert(context.knowledge_release.clone(), index.clone());
        Ok(index)
    }
    fn heads(
        &self,
        documents: &[DocumentRevision],
    ) -> Result<BTreeMap<(String, String), DocumentHead>, PortError> {
        let requested: BTreeSet<_> = documents
            .iter()
            .map(|d| (d.source_id.clone(), d.logical_id.clone()))
            .collect();
        let mut result = BTreeMap::new();
        for head in self.store.read_heads(documents)? {
            head.validate()?;
            let key = (head.source_id.clone(), head.logical_id.clone());
            if !requested.contains(&key) || result.insert(key, head).is_some() {
                return Err(invalid("unexpected or duplicate current head"));
            }
        }
        Ok(result)
    }
    fn selected(
        &self,
        index: &ChunkIndex,
        ranked: &[(usize, f64)],
        query: &Query,
        context: &AuthorizedContext,
        limit: usize,
        provider: bool,
    ) -> Result<Vec<Evidence>, PortError> {
        let mut result = Vec::new();
        for batch in ranked.chunks(128) {
            let mut documents = BTreeMap::new();
            for &(chunk, _) in batch {
                let doc = index.document(chunk);
                documents.insert((doc.source_id.clone(), doc.logical_id.clone()), doc);
            }
            let heads = self.heads(&documents.into_values().collect::<Vec<_>>())?;
            for &(chunk, score) in batch {
                let record = &index.records[index.chunks[chunk].document];
                if !index.eligible(record, query, context) {
                    continue;
                }
                let head = heads.get(&(record.source_id.clone(), record.logical_id.clone()));
                if let Some(effective) = effective_head(record, head, context, provider)? {
                    result.push(index.evidence(chunk, &effective, score));
                    if result.len() == limit {
                        return Ok(result);
                    }
                }
            }
        }
        Ok(result)
    }
    /// Compatibility bulk view for explicit callers; not used by lexical/dense queries.
    pub fn records(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        provider: bool,
    ) -> Result<Vec<SourceRecordV2>, PortError> {
        let index = self.index(query, context)?;
        let mut records = Vec::new();
        for batch in index.records.chunks(128) {
            let docs: Vec<_> = batch
                .iter()
                .map(|r| DocumentRevision {
                    source_id: r.source_id.clone(),
                    logical_id: r.logical_id.clone(),
                    revision: r.revision,
                    content_hash: r.content_hash.clone(),
                })
                .collect();
            let heads = self.heads(&docs)?;
            for record in batch {
                if !index.eligible(record, query, context) {
                    continue;
                }
                if let Some(effective) = effective_head(
                    record,
                    heads.get(&(record.source_id.clone(), record.logical_id.clone())),
                    context,
                    provider,
                )? {
                    let mut record = record.clone();
                    record.visibility = effective.visibility;
                    record.allowed_scopes = effective.allowed_scopes;
                    record
                        .metadata
                        .entry("patch".into())
                        .or_insert_with(|| index.release.patch.clone());
                    records.push(record);
                }
            }
        }
        Ok(records)
    }
    /// Dense vectors remain document-revision keyed. Expand only selected documents to the
    /// SAME canonical chunks as lexical retrieval; no whole-dossier provider payloads.
    pub(crate) fn dense_evidence(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        documents: &[(DocumentRevision, f64)],
    ) -> Result<Vec<Evidence>, PortError> {
        let index = self.index(query, context)?;
        let mut ranked = Vec::new();
        for (doc, score) in documents {
            if let Some(chunks) = index.by_document.get(&(
                doc.source_id.clone(),
                doc.logical_id.clone(),
                doc.revision,
            )) {
                for &chunk in chunks {
                    let record = &index.records[index.chunks[chunk].document];
                    if !index.eligible(record, query, context) {
                        continue;
                    }
                    if doc.content_hash != record.content_hash {
                        return Err(invalid("dense index content hash mismatch"));
                    }
                    ranked.push((chunk, *score));
                }
            }
        }
        ranked.sort_by(|(a, sa), (b, sb)| {
            sb.total_cmp(sa)
                .then_with(|| index.chunks[*a].id.cmp(&index.chunks[*b].id))
        });
        self.selected(&index, &ranked, query, context, 100, false)
    }
}
impl<S: SnapshotReadPort> RetrievalPort for ReleaseRetriever<S> {
    fn retrieve(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Result<Vec<Evidence>, PortError> {
        let index = self.index(query, context)?;
        let ranked = index.rank(query, context);
        let hits = self.selected(&index, &ranked, query, context, self.limit, false)?;
        pack(query, context, hits)
    }
    fn validate_evidence(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
        for_provider: bool,
    ) -> Result<(), PortError> {
        if evidence.is_empty() || evidence.len() > 100 {
            return Err(denied("invalid evidence pack size"));
        }
        let index = self.index(query, context)?;
        let mut seen = BTreeSet::new();
        let mut documents = BTreeMap::new();
        let mut canonical = Vec::new();
        for item in evidence {
            item.validate().map_err(|_| denied("invalid evidence"))?;
            if !seen.insert(&item.evidence_id) {
                return Err(denied("duplicate evidence"));
            }
            let chunk = *index
                .by_id
                .get(&item.evidence_id)
                .ok_or_else(|| denied("evidence not pinned in release"))?;
            let doc = index.document(chunk);
            documents.insert((doc.source_id.clone(), doc.logical_id.clone()), doc);
            canonical.push((item, chunk));
        }
        // Only bounded current heads, never a full release scan or historical document fetch.
        let heads = self.heads(&documents.into_values().collect::<Vec<_>>())?;
        for (item, chunk) in canonical {
            let record = &index.records[index.chunks[chunk].document];
            if !index.eligible(record, query, context) {
                return Err(denied("evidence metadata or scope denied"));
            }
            let effective = effective_head(
                record,
                heads.get(&(record.source_id.clone(), record.logical_id.clone())),
                context,
                for_provider,
            )?
            .ok_or_else(|| denied("evidence permission revoked or document deleted"))?;
            if index.evidence(chunk, &effective, item.score) != *item {
                return Err(denied("evidence content, ACL or provenance mismatch"));
            }
        }
        Ok(())
    }
}

fn check_context(query: &Query, context: &AuthorizedContext) -> Result<(), PortError> {
    query.validate().map_err(|_| invalid("invalid query"))?;
    if context.deadline_ms == 0 {
        return Err(PortError::BudgetExceeded);
    }
    if query.conversation_id != context.conversation_id
        || !query.requested_scopes.is_subset(&context.principal.scopes)
    {
        return Err(denied("invalid retrieval context"));
    }
    Ok(())
}
fn check_release(
    release: &brain_contracts::CorpusRelease,
    query: &Query,
    context: &AuthorizedContext,
) -> Result<(), PortError> {
    if release.release_id != context.knowledge_release
        || release.release_id == "current"
        || query.patch.as_ref().is_some_and(|p| p != &release.patch)
    {
        return Err(invalid("release or patch mismatch"));
    }
    Ok(())
}
fn effective_head(
    record: &SourceRecordV2,
    head: Option<&DocumentHead>,
    context: &AuthorizedContext,
    provider: bool,
) -> Result<Option<DocumentHead>, PortError> {
    let Some(head) = head else {
        return Ok(None);
    };
    if head.revision < record.revision {
        return Err(invalid("head predates pinned revision"));
    }
    if record.tombstone
        || !record_allowed(record, &context.principal, provider)
        || !head.allowed(&context.principal, provider)
    {
        return Ok(None);
    }
    let mut effective = head.clone();
    effective
        .allowed_scopes
        .extend(record.allowed_scopes.iter().cloned());
    effective.visibility = match (record.visibility, head.visibility) {
        (SourceVisibility::Private, _) | (_, SourceVisibility::Private) => {
            SourceVisibility::Private
        }
        (SourceVisibility::Internal, _) | (_, SourceVisibility::Internal) => {
            SourceVisibility::Internal
        }
        _ => SourceVisibility::Public,
    };
    Ok(Some(effective))
}
pub(crate) fn pack(
    query: &Query,
    context: &AuthorizedContext,
    hits: Vec<Evidence>,
) -> Result<Vec<Evidence>, PortError> {
    if matches!(query.profile, brain_contracts::AnswerProfile::Fact) {
        return Ok(hits);
    }
    let had_hits = !hits.is_empty();
    let mut selected = Vec::new();
    for hit in hits {
        selected.push(hit);
        if grounded_input_ceiling(query, &selected) > context.budget.max_input_tokens as u64 {
            selected.pop();
        }
    }
    if had_hits && selected.is_empty() {
        return Err(PortError::BudgetExceeded);
    }
    Ok(selected)
}
pub(crate) fn invalid(message: &str) -> PortError {
    PortError::InvalidResponse(message.into())
}
fn denied(message: &str) -> PortError {
    PortError::PermissionDenied(message.into())
}
