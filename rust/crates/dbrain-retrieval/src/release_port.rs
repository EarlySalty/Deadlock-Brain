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
        let manifest = self.store.read_manifest_until(
            &context.knowledge_release,
            context.request_deadline.as_ref(),
        )?;
        check_release(&manifest.release, query, context)?;
        let documents = manifest.authorized(&context.principal, false, false)?;
        // Aktuelle Rechte und der Principal gehören zum Cachevertrag. Neue Freigaben
        // bauen den Index neu auf, Sperren werden zusätzlich vor jeder Übergabe geprüft.
        let mut index_principal = context.principal.clone();
        let declared_scopes: BTreeSet<_> = manifest
            .revisions
            .iter()
            .map(|record| &record.head)
            .chain(&manifest.heads)
            .flat_map(|record| record.allowed_scopes.iter())
            .collect();
        index_principal
            .scopes
            .retain(|scope| declared_scopes.contains(scope));
        let cache_key = serde_json::to_string(&(
            &context.knowledge_release,
            &index_principal,
            &manifest.heads,
        ))
        .map_err(|_| invalid("Indexrechte sind ungültig"))?;
        if let Some(index) = indexes.get(&cache_key) {
            check_release(&index.release, query, context)?;
            return Ok(index.clone());
        }
        let prose: Vec<_> = self
            .store
            .read_documents_until(
                &context.knowledge_release,
                &documents,
                context.request_deadline.as_ref(),
            )?
            .into_iter()
            .filter(|r| {
                !r.metadata.contains_key("domain_contract")
                    && r.metadata.get("kind").map(String::as_str) != Some("domain_input")
            })
            .collect();
        let index = Arc::new(ChunkIndex::build(manifest.release, prose)?);
        context.check_deadline()?;
        if indexes.len() >= 4 {
            if let Some(key) = indexes.keys().next().cloned() {
                indexes.remove(&key);
            }
        }
        indexes.insert(cache_key, index.clone());
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
impl<S: SnapshotReadPort> brain_contracts::discord_task::DiscordContextResolver
    for ReleaseRetriever<S>
{
    fn resolve(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        user_questions: &[String],
    ) -> Result<brain_contracts::discord_task::DiscordReference, PortError> {
        use brain_contracts::discord_task::{needs_reference, DiscordReference};
        let bound = context.with_request_deadline();
        let context = &bound;
        check_context(query, context)?;
        if !needs_reference(&query.text) {
            return Ok(DiscordReference::Independent);
        }
        if user_questions.len() > 4
            || user_questions
                .iter()
                .map(|q| q.chars().count())
                .sum::<usize>()
                > 4000
        {
            return Err(invalid("invalid private context bounds"));
        }
        let index = self.index(query, context)?;
        let current = brain_contracts::lexical::terms(&query.text);
        let history: Vec<_> = user_questions
            .iter()
            .map(|text| brain_contracts::lexical::terms(text))
            .collect();
        let mut matches: BTreeMap<String, Vec<DocumentRevision>> = BTreeMap::new();
        for record in &index.records {
            context.check_deadline()?;
            if record.visibility != SourceVisibility::Public
                || !record_allowed(record, &context.principal, true)
                || record.tombstone
            {
                continue;
            }
            for (subject, names) in public_context_names(record) {
                if names.iter().any(|name| {
                    contains_terms(&current, name)
                        || history
                            .iter()
                            .any(|question| contains_terms(question, name))
                }) {
                    matches.entry(subject).or_default().push(DocumentRevision {
                        source_id: record.source_id.clone(),
                        logical_id: record.logical_id.clone(),
                        revision: record.revision,
                        content_hash: record.content_hash.clone(),
                    });
                }
            }
        }
        let documents: BTreeMap<_, _> = matches
            .values()
            .flatten()
            .map(|doc| ((doc.source_id.clone(), doc.logical_id.clone()), doc.clone()))
            .collect();
        if documents.len() > 256 {
            return Ok(DiscordReference::Clarification);
        }
        let heads = self.heads(&documents.into_values().collect::<Vec<_>>(), context)?;
        let mut vocabulary: BTreeMap<String, Vec<Vec<String>>> = BTreeMap::new();
        for record in &index.records {
            if record.visibility != SourceVisibility::Public {
                continue;
            }
            let key = (record.source_id.clone(), record.logical_id.clone());
            if let Some(head) = heads.get(&key) {
                if brain_contracts::store::record_publication_allowed(
                    record,
                    head,
                    &context.principal,
                ) && effective_head(record, Some(head), query, context, true)?
                    .is_some_and(|head| head.visibility == SourceVisibility::Public)
                {
                    for (subject, names) in public_context_names(record) {
                        if matches.contains_key(&subject) {
                            vocabulary.entry(subject).or_default().extend(names);
                        }
                    }
                }
            }
        }
        let subjects = |words: &[String]| -> BTreeSet<String> {
            vocabulary
                .iter()
                .filter(|(_, names)| names.iter().any(|name| contains_terms(words, name)))
                .map(|(subject, _)| subject.clone())
                .collect()
        };
        if !subjects(&current).is_empty() {
            return Ok(DiscordReference::Independent);
        }
        if !reference_only(&query.text) {
            return Ok(DiscordReference::Clarification);
        }
        for (text, words) in user_questions.iter().zip(&history).rev() {
            let mut found = subjects(words);
            if found.len() == 1 {
                return Ok(DiscordReference::Subject(
                    found.pop_first().expect("Ein öffentlicher Bezug"),
                ));
            }
            if !found.is_empty() || !reference_only(text) {
                break;
            }
        }
        Ok(DiscordReference::Clarification)
    }
}

fn reference_only(text: &str) -> bool {
    use brain_contracts::discord_task::needs_reference;
    let words = brain_contracts::lexical::terms(text);
    needs_reference(text)
        && words.iter().enumerate().all(|(index, word)| {
            (word == "ult"
                && index > 0
                && matches!(
                    words[index - 1].as_str(),
                    "seine" | "ihre" | "dessen" | "deren"
                ))
                || matches!(
                    word.as_str(),
                    "wie"
                        | "wo"
                        | "was"
                        | "warum"
                        | "wieso"
                        | "und"
                        | "auch"
                        | "ich"
                        | "mich"
                        | "mir"
                        | "man"
                        | "wir"
                        | "das"
                        | "es"
                        | "sie"
                        | "der"
                        | "die"
                        | "den"
                        | "welche"
                        | "dafuer"
                        | "dort"
                        | "dabei"
                        | "damit"
                        | "darueber"
                        | "dazu"
                        | "davon"
                        | "dahin"
                        | "daraus"
                        | "dieser"
                        | "diese"
                        | "dieses"
                        | "diesem"
                        | "diesen"
                        | "dessen"
                        | "er"
                        | "ihn"
                        | "ihm"
                        | "ihnen"
                        | "ihre"
                        | "ihren"
                        | "ihrem"
                        | "seine"
                        | "seinen"
                        | "seiner"
                        | "deren"
                        | "ist"
                        | "sind"
                        | "kann"
                        | "koennen"
                        | "mache"
                        | "macht"
                        | "machen"
                        | "bekomme"
                        | "bekommen"
                        | "finde"
                        | "finden"
                        | "nutze"
                        | "nutzen"
                        | "starten"
                        | "einstellen"
                        | "melde"
                        | "anmelden"
                        | "an"
                        | "mit"
                        | "fuer"
                        | "denn"
                        | "geht"
                        | "funktioniert"
                        | "kostet"
                        | "spiele"
                        | "spielt"
                        | "spielen"
                        | "verbessern"
                        | "erzaehl"
                        | "erzaehle"
                        | "erklaer"
                        | "erklaere"
                        | "erklaeren"
                        | "mehr"
                        | "weiter"
                        | "genau"
                        | "genauer"
                        | "nochmal"
                )
        })
}

fn contains_terms(words: &[String], name: &[String]) -> bool {
    !name.is_empty() && words.windows(name.len()).any(|window| window == name)
}

fn public_context_names(record: &SourceRecordV2) -> Vec<(String, Vec<Vec<String>>)> {
    use brain_contracts::lexical::{fact_names, terms};
    let mut result = Vec::new();
    let words = terms(&record.content);
    for (subject, aliases) in [
        ("Paten", &["Pate", "Paten", "Patenschaft"][..]),
        ("Coaching", &["Coaching", "Coach", "Coaches"][..]),
        (
            "Sprachkanäle",
            &["Sprachkanal", "Sprachkanäle", "Voice", "Lanes"][..],
        ),
        ("Mitspieler", &["Mitspieler", "LFG"][..]),
        ("Rollen", &["Rolle", "Rollen"][..]),
        ("Datenschutz", &["Datenschutz"][..]),
        ("FAQ", &["FAQ"][..]),
    ] {
        let names: Vec<_> = aliases.iter().map(|name| terms(name)).collect();
        if names.iter().any(|name| contains_terms(&words, name)) {
            result.push((subject.to_owned(), names));
        }
    }
    let hero = (record.source_id == "deadlock-assets-heroes"
        && record.logical_id.starts_with("asset/hero/"))
        || (record.source_id == "legacy-entities" && record.logical_id.starts_with("entity/hero/"));
    if hero {
        let canonical = record
            .metadata
            .get("name")
            .or_else(|| record.metadata.get("canonical_name"));
        if let Some(name) = canonical.filter(|name| {
            !name.is_empty()
                && name.chars().count() <= 80
                && name
                    .chars()
                    .all(|c| c.is_alphabetic() || matches!(c, ' ' | '-' | '\''))
        }) {
            result.push((
                name.clone(),
                fact_names("", &record.content, &record.metadata)
                    .into_iter()
                    .filter(|words| {
                        words.len() <= 5
                            && words
                                .iter()
                                .all(|word| word.chars().all(char::is_alphabetic))
                    })
                    .collect(),
            ));
        }
    }
    result
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

#[cfg(test)]
mod cache_scope_tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[derive(Clone)]
    struct CountingStore {
        snapshot: CorpusSnapshot,
        documents: Arc<AtomicUsize>,
    }
    impl SnapshotReadPort for CountingStore {
        fn read_snapshot(&self, _: &str) -> Result<CorpusSnapshot, PortError> {
            Ok(self.snapshot.clone())
        }
        fn read_documents_until(
            &self,
            _: &str,
            _: &[DocumentRevision],
            _: Option<&brain_contracts::RequestDeadline>,
        ) -> Result<Vec<SourceRecordV2>, PortError> {
            self.documents.fetch_add(1, Ordering::SeqCst);
            Ok(self.snapshot.revisions.clone())
        }
    }

    #[test]
    fn consecutive_discord_request_scopes_reuse_the_corpus_index() {
        let record = SourceRecordV2 {
            source_id: "fixture".into(),
            logical_id: "regel".into(),
            revision: 1,
            content_hash: "a".repeat(64),
            content: "Eine öffentliche Spielregel".into(),
            visibility: SourceVisibility::Public,
            allowed_scopes: BTreeSet::from(["bot.public".into()]),
            tombstone: false,
            valid_from: None,
            valid_to: None,
            metadata: BTreeMap::new(),
        };
        let snapshot = CorpusSnapshot {
            release: brain_contracts::CorpusRelease {
                release_id: "release".into(),
                knowledge_version: "wissen".into(),
                patch: "patch".into(),
                created_at_epoch: 1,
                source_revisions: BTreeMap::from([(
                    "fixture".into(),
                    BTreeMap::from([("regel".into(), 1)]),
                )]),
            },
            revisions: vec![record.clone()],
            heads: vec![record],
        };
        let documents = Arc::new(AtomicUsize::new(0));
        let retriever = ReleaseRetriever::new(
            CountingStore {
                snapshot,
                documents: documents.clone(),
            },
            10,
        );
        let query: Query = serde_json::from_value(serde_json::json!({"request_id":"eins","conversation_id":"gespräch","text":"Spielregel","requested_scopes":["bot.public"]})).unwrap();
        let mut context = AuthorizedContext {
            principal: brain_contracts::Principal {
                actor_id: "tester".into(),
                channel: "discord".into(),
                scopes: BTreeSet::from(["bot.public".into(), "discord.request:eins".into()]),
                provider_egress: BTreeSet::from(["public".into()]),
            },
            discord: None,
            conversation_id: query.conversation_id.clone(),
            knowledge_release: "release".into(),
            deadline_ms: 1000,
            budget: Default::default(),
            request_deadline: None,
        };
        let first = retriever.index(&query, &context).unwrap();
        context.principal.scopes.remove("discord.request:eins");
        context
            .principal
            .scopes
            .insert("discord.request:zwei".into());
        let second = retriever.index(&query, &context).unwrap();
        assert!(Arc::ptr_eq(&first, &second));
        assert_eq!(documents.load(Ordering::SeqCst), 1);
    }
}
