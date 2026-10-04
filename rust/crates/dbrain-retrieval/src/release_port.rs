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
    /// Bulk canonical view for diagnostics and deterministic domain revalidation.
    /// Lexical/dense queries and their validation only load a full snapshot for the
    /// first index build; up to four immutable release indexes are kept. Domain
    /// proofs also read current snapshots when revalidating publication on cache hits.
    pub fn snapshot(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Result<CorpusSnapshot, PortError> {
        let bound = context.with_request_deadline();
        let context = &bound;
        check_context(query, context)?;
        let snapshot = self.store.read_snapshot_until(
            &context.knowledge_release,
            context.request_deadline.as_ref(),
        )?;
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
        context.check_deadline()?;
        if let Some(index) = indexes.get(&context.knowledge_release) {
            check_release(&index.release, query, context)?;
            return Ok(index.clone());
        }
        let snapshot = self.snapshot(query, context)?;
        // Validate ALL pins/revisions/heads; indexing is not scoped to the first caller.
        // The result is deliberately discarded. No mutable head/authorization is cached.
        snapshot.authorized(&context.principal, false)?;
        let prose: Vec<_> = snapshot
            .revisions
            .into_iter()
            .filter(|r| {
                !r.metadata.contains_key("domain_contract")
                    && r.metadata.get("kind").map(String::as_str) != Some("domain_input")
            })
            .collect();
        let index = Arc::new(ChunkIndex::build(snapshot.release, prose)?);
        context.check_deadline()?;
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
        context: &AuthorizedContext,
    ) -> Result<BTreeMap<(String, String), DocumentHead>, PortError> {
        context.check_deadline()?;
        let requested: BTreeSet<_> = documents
            .iter()
            .map(|d| (d.source_id.clone(), d.logical_id.clone()))
            .collect();
        let mut result = BTreeMap::new();
        for head in self
            .store
            .read_heads_until(documents, context.request_deadline.as_ref())?
        {
            head.validate()?;
            let key = (head.source_id.clone(), head.logical_id.clone());
            if !requested.contains(&key) || result.insert(key, head).is_some() {
                return Err(invalid("unexpected or duplicate current head"));
            }
        }
        Ok(result)
    }
    fn ambiguous_fact_owner(
        &self,
        index: &ChunkIndex,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Result<bool, PortError> {
        let groups = index.matching_fact_owners(query, context);
        if groups.is_empty() {
            return Ok(false);
        }
        let documents: BTreeMap<_, _> = groups
            .iter()
            .flatten()
            .map(|record| {
                (
                    (record.source_id.clone(), record.logical_id.clone()),
                    DocumentRevision {
                        source_id: record.source_id.clone(),
                        logical_id: record.logical_id.clone(),
                        revision: record.revision,
                        content_hash: record.content_hash.clone(),
                    },
                )
            })
            .collect();
        // The production reader caps one head snapshot at 256 keys. An excess
        // of possible owners is ambiguous rather than a reader error or an
        // inconsistent sequence of ACL snapshots.
        if documents.len() > 256 {
            return Ok(true);
        }
        let heads = self.heads(&documents.into_values().collect::<Vec<_>>(), context)?;
        for owners in groups {
            let mut visible = BTreeSet::new();
            for record in owners {
                let head = heads.get(&(record.source_id.clone(), record.logical_id.clone()));
                if effective_head(record, head, query, context, false)?.is_some() {
                    visible.insert(brain_contracts::lexical::fact_entity_key(
                        &record.source_id,
                        &record.logical_id,
                        &record.content,
                        &record.metadata,
                    ));
                    if visible.len() > 1 {
                        return Ok(true);
                    }
                }
            }
        }
        Ok(false)
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
            let heads = self.heads(&documents.into_values().collect::<Vec<_>>(), context)?;
            for &(chunk, score) in batch {
                let record = &index.records[index.chunks[chunk].document];
                if !index.eligible(record, query, context) {
                    continue;
                }
                let head = heads.get(&(record.source_id.clone(), record.logical_id.clone()));
                if let Some(effective) = effective_head(record, head, query, context, provider)? {
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
        let bound = context.with_request_deadline();
        let context = &bound;
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
            let heads = self.heads(&docs, context)?;
            for record in batch {
                if !index.eligible(record, query, context) {
                    continue;
                }
                if let Some(effective) = effective_head(
                    record,
                    heads.get(&(record.source_id.clone(), record.logical_id.clone())),
                    query,
                    context,
                    provider,
                )? {
                    let mut record = record.clone();
                    record.visibility = effective.visibility;
                    record.allowed_scopes = effective.allowed_scopes;
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
        let bound = context.with_request_deadline();
        let context = &bound;
        check_context(query, context)?;
        if crate::domain_port::handles(query) {
            return crate::domain_port::retrieve(&self.store, query, context, false);
        }
        let profiles = crate::entity_profile_port::retrieve(
            &self.store,
            query,
            context,
            false,
            brain_contracts::store::AnswerPurpose::InternalRead,
        )?;
        if let Some(profiles) = profiles {
            return Ok(profiles);
        }
        if query.text.to_lowercase().contains("patch")
            && crate::entity_profile_port::patch_date(&query.text)?.is_some()
        {
            return Ok(Vec::new());
        }
        let index = self.index(query, context)?;
        if query.profile == brain_contracts::AnswerProfile::Fact
            && self.ambiguous_fact_owner(&index, query, context)?
        {
            return Ok(Vec::new());
        }
        let ranked = index.rank(query, context);
        // A caller limit of one must not hide a second assertion of the same
        // fact. The kernel receives the full bounded lexical candidate pack.
        // More than 100 ranked candidates cannot be checked exhaustively.
        if query.profile == brain_contracts::AnswerProfile::Fact && ranked.len() > 100 {
            return Ok(Vec::new());
        }
        let limit = if query.profile == brain_contracts::AnswerProfile::Fact {
            100
        } else {
            self.limit
        };
        let hits = self.selected(&index, &ranked, query, context, limit, false)?;
        pack(query, context, hits)
    }
    fn validate_evidence(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
        for_provider: bool,
    ) -> Result<(), PortError> {
        self.validate_for_purpose(
            query,
            context,
            evidence,
            for_provider,
            brain_contracts::store::AnswerPurpose::InternalRead,
        )
    }
    fn validate_publication(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
    ) -> Result<(), PortError> {
        self.validate_for_purpose(
            query,
            context,
            evidence,
            false,
            brain_contracts::store::AnswerPurpose::ExternalPublication,
        )
    }
}
impl<S: SnapshotReadPort> ReleaseRetriever<S> {
    fn validate_for_purpose(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
        for_provider: bool,
        purpose: brain_contracts::store::AnswerPurpose,
    ) -> Result<(), PortError> {
        let bound = context.with_request_deadline();
        let context = &bound;
        check_context(query, context)?;
        if evidence.is_empty() || evidence.len() > 100 {
            return Err(denied("invalid evidence pack size"));
        }
        if evidence
            .iter()
            .any(|item| item.evidence_id.starts_with("entity-profile:"))
        {
            let canonical = crate::entity_profile_port::retrieve(
                &self.store,
                query,
                context,
                for_provider,
                purpose,
            )?;
            if canonical.as_deref().unwrap_or_default() != evidence {
                return Err(denied(
                    "Entitätsbelege haben sich geändert oder sind nicht freigegeben",
                ));
            }
            return Ok(());
        }
        if crate::domain_port::handles(query) {
            let canonical =
                crate::domain_port::retrieve(&self.store, query, context, for_provider)?;
            if canonical != evidence {
                return Err(denied("domain evidence changed or no longer authorized"));
            }
            if purpose == brain_contracts::store::AnswerPurpose::ExternalPublication {
                // Only parse dependency identities after recomputing the complete
                // canonical proof. A model/source cannot invent its own grants.
                let snapshot = self.snapshot(query, context)?;
                let publishable = snapshot.authorized_for_publication(&context.principal)?;
                for item in &canonical {
                    let proof: brain_contracts::domain::DomainAnswer =
                        serde_json::from_str(&item.content)
                            .map_err(|_| denied("invalid canonical domain proof"))?;
                    if proof.inputs.is_empty()
                        || proof.inputs.iter().any(|input| {
                            !publishable.iter().any(|record| {
                                record.source_id == input.source.source_id
                                    && record.logical_id == input.source.logical_id
                                    && record.revision == input.source.revision
                                    && record.content_hash == input.source.content_hash
                            })
                        })
                    {
                        return Err(denied("domain dependency publication denied"));
                    }
                }
            }
            return Ok(());
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
        let heads = self.heads(&documents.into_values().collect::<Vec<_>>(), context)?;
        for (item, chunk) in canonical {
            let record = &index.records[index.chunks[chunk].document];
            if !index.eligible(record, query, context) {
                return Err(denied("evidence metadata or scope denied"));
            }
            let effective = effective_head(
                record,
                heads.get(&(record.source_id.clone(), record.logical_id.clone())),
                query,
                context,
                for_provider,
            )?
            .ok_or_else(|| denied("evidence permission revoked or document deleted"))?;
            if purpose == brain_contracts::store::AnswerPurpose::ExternalPublication
                && !heads
                    .get(&(record.source_id.clone(), record.logical_id.clone()))
                    .is_some_and(|head| {
                        brain_contracts::store::record_publication_allowed(
                            record,
                            head,
                            &context.principal,
                        )
                    })
            {
                return Err(denied("evidence publication permission denied or revoked"));
            }
            if index.evidence(chunk, &effective, item.score) != *item {
                return Err(denied("evidence content, ACL or provenance mismatch"));
            }
        }
        Ok(())
    }
}

fn check_context(query: &Query, context: &AuthorizedContext) -> Result<(), PortError> {
    query.validate().map_err(|_| invalid("invalid query"))?;
    context.check_deadline()?;
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
    query: &Query,
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
        || (provider
            && record
                .metadata
                .contains_key(brain_contracts::source::ORIGIN_METADATA_KEY)
            && !head
                .metadata
                .contains_key(brain_contracts::source::ORIGIN_METADATA_KEY))
        || brain_contracts::source::patch_validity_for(&head.metadata, query.patch.as_deref())
            .is_err()
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
pub fn pack(
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
