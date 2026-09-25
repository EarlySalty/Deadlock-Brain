use crate::release_port::{evidence_for_record, invalid, ReleaseRetriever};
use brain_contracts::{
    AuthorizedContext, DocumentRevision, EmbeddingIdentity, EmbeddingProviderPort, Evidence,
    PortError, Query, RetrievalPort, SnapshotReadPort, Usage,
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone)]
pub struct DenseEntry {
    pub document: DocumentRevision,
    pub vector: Vec<f64>,
}
#[derive(Debug, Clone)]
pub struct DenseIndex {
    pub release_id: String,
    pub identity: EmbeddingIdentity,
    pub entries: Vec<DenseEntry>,
}
impl DenseIndex {
    pub fn validate(&self) -> Result<(), PortError> {
        self.identity.validate()?;
        if self.release_id.trim().is_empty() || self.entries.len() > 10000 {
            return Err(invalid("invalid dense index size or release"));
        }
        let mut keys = BTreeSet::new();
        for entry in &self.entries {
            self.identity.validate_vector(&entry.vector)?;
            if entry.document.revision == 0
                || !keys.insert((
                    &entry.document.source_id,
                    &entry.document.logical_id,
                    entry.document.revision,
                ))
            {
                return Err(invalid("duplicate or invalid dense revision"));
            }
        }
        Ok(())
    }
}
/// Lexical-only remains available by constructing ReleaseRetriever directly.
/// Dense errors are not hidden: unknown remote usage cannot safely be charged as zero.
pub struct HybridRetriever<S, E> {
    lexical: ReleaseRetriever<S>,
    embedding: E,
    index: DenseIndex,
    limit: usize,
}
impl<S: SnapshotReadPort, E: EmbeddingProviderPort> HybridRetriever<S, E> {
    pub fn new(store: S, embedding: E, index: DenseIndex, limit: usize) -> Result<Self, PortError> {
        index.validate()?;
        Ok(Self {
            lexical: ReleaseRetriever::new(store, 100),
            embedding,
            index,
            limit: limit.clamp(1, 100),
        })
    }
    fn retrieve_accounted(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Result<(Vec<Evidence>, Usage), PortError> {
        if self.index.release_id != context.knowledge_release {
            return Err(invalid("dense release mismatch"));
        }
        let lexical = self.lexical.retrieve(query, context)?;
        if matches!(query.profile, brain_contracts::AnswerProfile::Fact)
            || context.budget.max_network_rounds == 0
            || !context.principal.provider_egress.contains("public")
        {
            return Ok((
                lexical.into_iter().take(self.limit).collect(),
                Usage::default(),
            ));
        }
        let records = self.lexical.records(query, context, false)?;
        let selected: Vec<_> = self
            .index
            .entries
            .iter()
            .filter_map(|entry| {
                records
                    .iter()
                    .find(|r| {
                        r.source_id == entry.document.source_id
                            && r.logical_id == entry.document.logical_id
                            && r.revision == entry.document.revision
                    })
                    .map(|record| (entry, record))
            })
            .collect();
        if selected.is_empty() {
            return Ok((
                lexical.into_iter().take(self.limit).collect(),
                Usage::default(),
            ));
        }
        if selected
            .iter()
            .any(|(entry, record)| entry.document.content_hash != record.content_hash)
        {
            return Err(invalid("dense index content hash mismatch"));
        }
        let output = self.embedding.embed(
            &[format!(
                "{}{}",
                self.index.identity.query_prefix, query.text
            )],
            &self.index.identity,
            context,
        )?;
        if output.identity != self.index.identity || output.vectors.len() != 1 {
            return Err(invalid("embedding identity or result count mismatch"));
        }
        self.index.identity.validate_vector(&output.vectors[0])?;
        if output.usage.network_rounds > context.budget.max_network_rounds
            || output.usage.input_tokens > context.budget.max_input_tokens as u64
            || output.usage.cost_micros > context.budget.max_cost_micros
        {
            return Err(PortError::BudgetExceeded);
        }
        let q = &output.vectors[0];
        let qnorm = q.iter().map(|x| x * x).sum::<f64>().sqrt();
        let dense = selected
            .into_iter()
            .map(|(entry, record)| {
                let norm = entry.vector.iter().map(|x| x * x).sum::<f64>().sqrt();
                let cosine =
                    q.iter().zip(&entry.vector).map(|(a, b)| a * b).sum::<f64>() / (qnorm * norm);
                // [0,1] cosine transformation permits shared nonnegative score validation.
                evidence_for_record(record, ((cosine.clamp(-1.0, 1.0)) + 1.0) / 2.0)
            })
            .collect();
        let fused = fuse_ranked(&[lexical, dense], &[1, 1], 60, self.limit)?;
        Ok((fused, output.usage))
    }
}
impl<S: SnapshotReadPort, E: EmbeddingProviderPort> RetrievalPort for HybridRetriever<S, E> {
    fn retrieve(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Result<Vec<Evidence>, PortError> {
        self.retrieve_accounted(query, context).map(|(e, _)| e)
    }
    fn retrieve_with_usage(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Result<(Vec<Evidence>, Usage), PortError> {
        self.retrieve_accounted(query, context)
    }
    fn validate_evidence(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
        provider: bool,
    ) -> Result<(), PortError> {
        self.lexical
            .validate_evidence(query, context, evidence, provider)
    }
}
/// Deterministic weighted reciprocal-rank fusion. No duplicate boosts or score-scale mixing.
pub fn fuse_ranked(
    lists: &[Vec<Evidence>],
    weights: &[u32],
    rank_constant: u32,
    limit: usize,
) -> Result<Vec<Evidence>, PortError> {
    if lists.len() != weights.len()
        || lists.len() > 16
        || rank_constant == 0
        || weights.iter().any(|w| *w > 1000)
    {
        return Err(invalid("invalid fusion configuration"));
    }
    let mut result: BTreeMap<String, (Evidence, f64)> = BTreeMap::new();
    for (list, weight) in lists.iter().zip(weights) {
        if list.len() > 10000 {
            return Err(invalid("fusion list too large"));
        }
        let mut ranked = list.clone();
        ranked.sort_by(|a, b| {
            b.score
                .total_cmp(&a.score)
                .then_with(|| a.evidence_id.cmp(&b.evidence_id))
        });
        let mut seen = BTreeSet::new();
        let mut rank = 0;
        for item in ranked {
            item.validate()
                .map_err(|_| invalid("invalid fusion evidence"))?;
            if let Some((existing, _)) = result.get(&item.evidence_id) {
                let mut equal = item.clone();
                equal.score = existing.score;
                if &equal != existing {
                    return Err(invalid("conflicting evidence for same identity"));
                }
            }
            if !seen.insert(item.evidence_id.clone()) {
                continue;
            }
            rank += 1;
            let contribution = *weight as f64 / (rank_constant as f64 + rank as f64);
            let entry = result
                .entry(item.evidence_id.clone())
                .or_insert((item, 0.0));
            entry.1 += contribution;
        }
    }
    let mut result: Vec<_> = result
        .into_values()
        .filter(|(_, score)| *score > 0.0)
        .map(|(mut item, score)| {
            item.score = score;
            item
        })
        .collect();
    result.sort_by(|a, b| {
        b.score
            .total_cmp(&a.score)
            .then_with(|| a.evidence_id.cmp(&b.evidence_id))
    });
    result.truncate(limit.min(100));
    Ok(result)
}
