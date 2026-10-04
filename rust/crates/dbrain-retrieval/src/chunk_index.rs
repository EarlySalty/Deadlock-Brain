use brain_contracts::{
    lexical::{fact_names, terms},
    store::record_allowed,
    AnswerProfile, AuthorizedContext, ChunkProvenance, CorpusRelease, DocumentHead,
    DocumentRevision, Evidence, EvidenceKind, PortError, Query, SourceRecordV2,
};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

pub(crate) const CHUNKER_VERSION: &str = "utf8-window-v1-1024-overlap192";
const HTML_CHUNKER_VERSION: &str = "html-semantic-v2+utf8-window-v1-1024-overlap192";
const TARGET_BYTES: usize = 1024;
const OVERLAP_BYTES: usize = 192;
const K1: f64 = 1.2;
const B: f64 = 0.75;

#[derive(Debug)]
enum IndexedText {
    Raw(String),
    Html(String),
    Knowledge(crate::knowledge_projection::KnowledgeProjection),
}

impl IndexedText {
    fn text(&self) -> &str {
        match self {
            Self::Raw(text) | Self::Html(text) => text,
            Self::Knowledge(projection) => &projection.text,
        }
    }
    fn chunker_version(&self) -> &str {
        match self {
            Self::Raw(_) => CHUNKER_VERSION,
            Self::Html(_) => HTML_CHUNKER_VERSION,
            Self::Knowledge(_) => crate::knowledge_projection::KNOWLEDGE_CHUNKER_VERSION,
        }
    }
    fn ranges(&self, atomic: bool) -> Vec<(usize, usize)> {
        match self {
            Self::Knowledge(projection) => {
                let mut result = ranges(&projection.text[..projection.raw_byte_end], false);
                result.extend(
                    projection
                        .facts
                        .iter()
                        .map(|fact| (fact.byte_start, fact.byte_end)),
                );
                result
            }
            _ => ranges(self.text(), atomic),
        }
    }
}

fn collect_indexed_texts(
    texts: impl Iterator<Item = Result<IndexedText, PortError>>,
    max_bytes: usize,
) -> Result<Vec<IndexedText>, PortError> {
    let mut result = Vec::new();
    let mut total = 0usize;
    for text in texts {
        let text = text?;
        total = total
            .checked_add(text.text().len())
            .filter(|total| *total <= max_bytes)
            .ok_or(PortError::BudgetExceeded)?;
        result.push(text);
    }
    Ok(result)
}

#[derive(Debug)]
pub(crate) struct IndexedChunk {
    pub document: usize,
    pub start: usize,
    pub end: usize,
    pub ordinal: u32,
    pub id: String,
    numbers: BTreeSet<String>,
}
#[derive(Debug)]
pub(crate) struct ChunkIndex {
    pub release: CorpusRelease,
    pub records: Vec<SourceRecordV2>,
    texts: Vec<IndexedText>,
    pub chunks: Vec<IndexedChunk>,
    pub by_id: BTreeMap<String, usize>,
    pub by_document: BTreeMap<(String, String, u64), Vec<usize>>,
    postings: BTreeMap<String, Vec<(usize, u32)>>,
    lengths: Vec<usize>,
    average_length: f64,
    fact_name_owners: BTreeMap<Vec<String>, BTreeSet<usize>>,
}

pub(crate) fn numeric_terms(text: &str) -> BTreeSet<String> {
    terms(text)
        .into_iter()
        .filter(|s| {
            s.chars()
                .next()
                .is_some_and(|c| c.is_ascii_digit() || c == '-')
        })
        .collect()
}

fn ranges(text: &str, atomic: bool) -> Vec<(usize, usize)> {
    if text.is_empty() {
        return Vec::new();
    }
    if atomic {
        return vec![(0, text.len())];
    }
    let mut result = Vec::new();
    let mut start = 0;
    while start < text.len() {
        let mut end = (start + TARGET_BYTES).min(text.len());
        while !text.is_char_boundary(end) {
            end -= 1;
        }
        if end < text.len() {
            let window = &text[start..end];
            let boundary = window
                .char_indices()
                .filter(|(i, c)| *i >= TARGET_BYTES / 2 && c.is_whitespace())
                .map(|(i, c)| start + i + c.len_utf8())
                .next_back();
            end = boundary.unwrap_or_else(|| {
                text[end..]
                    .char_indices()
                    .find(|(_, c)| c.is_whitespace())
                    .map_or(text.len(), |(i, c)| end + i + c.len_utf8())
            });
        }
        result.push((start, end));
        if end == text.len() {
            break;
        }
        let mut next = end.saturating_sub(OVERLAP_BYTES).max(start + 1);
        while !text.is_char_boundary(next) {
            next += 1;
        }
        next = text[next..end]
            .char_indices()
            .find(|(_, c)| c.is_whitespace())
            .map_or(end, |(i, c)| next + i + c.len_utf8());
        start = next;
    }
    result
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ReleaseIndexProof {
    pub documents: usize,
    pub indexed_documents: usize,
    pub projected_bytes: usize,
    pub chunks: usize,
}

pub fn preflight_release_index(
    release: CorpusRelease,
    records: Vec<SourceRecordV2>,
) -> Result<ReleaseIndexProof, PortError> {
    brain_storage::validate_release(&release)?;
    let documents = release
        .source_revisions
        .values()
        .map(BTreeMap::len)
        .sum::<usize>();
    let mut seen = BTreeSet::new();
    for record in &records {
        record
            .validate()
            .map_err(|_| PortError::InvalidResponse("invalid preflight revision".into()))?;
        if release
            .source_revisions
            .get(&record.source_id)
            .and_then(|pins| pins.get(&record.logical_id))
            != Some(&record.revision)
            || !seen.insert((&record.source_id, &record.logical_id))
        {
            return Err(PortError::InvalidResponse(
                "preflight records differ from exact release pins".into(),
            ));
        }
    }
    if seen.len() != documents {
        return Err(PortError::InvalidResponse(
            "preflight is missing pinned revisions".into(),
        ));
    }
    drop(seen);
    let prose = records
        .into_iter()
        .filter(|record| {
            !record.metadata.contains_key("domain_contract")
                && record.metadata.get("kind").map(String::as_str) != Some("domain_input")
        })
        .collect();
    let index = ChunkIndex::build(release, prose)?;
    Ok(ReleaseIndexProof {
        documents,
        indexed_documents: index.records.len(),
        projected_bytes: index.texts.iter().map(|text| text.text().len()).sum(),
        chunks: index.chunks.len(),
    })
}

impl ChunkIndex {
    pub fn build(
        release: CorpusRelease,
        mut records: Vec<SourceRecordV2>,
    ) -> Result<Self, PortError> {
        records.retain(|record| {
            record.source_id != "git-game-facts-derived"
                && !record
                    .metadata
                    .contains_key("brain.entity_projection.contract")
        });
        records.sort_by(|a, b| {
            (&a.source_id, &a.logical_id, a.revision).cmp(&(
                &b.source_id,
                &b.logical_id,
                b.revision,
            ))
        });
        if records
            .iter()
            .try_fold(0usize, |total, record| {
                total
                    .checked_add(record.content.len())
                    .filter(|total| *total <= crate::knowledge_projection::MAX_INDEX_BYTES)
            })
            .is_none()
        {
            return Err(PortError::BudgetExceeded);
        }
        let mut index = Self {
            release,
            texts: collect_indexed_texts(
                records.iter().map(|record| {
                    if let Some(projection) =
                        crate::knowledge_projection::project_knowledge(record)?
                    {
                        return Ok(IndexedText::Knowledge(projection));
                    }
                    if record.metadata.get("content_format").map(String::as_str) != Some("html") {
                        return Ok(IndexedText::Raw(record.content.clone()));
                    }
                    let projection = crate::html_projection::project_html(&record.content)?;
                    let legacy = [
                        "html_projection_version",
                        "html_raw_sha256",
                        "html_semantic_sha256",
                    ]
                    .iter()
                    .all(|key| !record.metadata.contains_key(*key));
                    if record.content_hash != projection.raw_sha256
                        || (!legacy
                            && (record
                                .metadata
                                .get("html_projection_version")
                                .map(String::as_str)
                                != Some(crate::html_projection::HTML_PROJECTION_VERSION)
                                || record.metadata.get("html_raw_sha256")
                                    != Some(&projection.raw_sha256)
                                || record.metadata.get("html_semantic_sha256")
                                    != Some(&projection.semantic_sha256)))
                    {
                        return Err(PortError::InvalidResponse("html_projection_binding".into()));
                    }
                    Ok(IndexedText::Html(projection.text))
                }),
                crate::knowledge_projection::MAX_INDEX_BYTES,
            )?,
            records,
            chunks: Vec::new(),
            by_id: BTreeMap::new(),
            by_document: BTreeMap::new(),
            postings: BTreeMap::new(),
            lengths: Vec::new(),
            average_length: 1.0,
            fact_name_owners: BTreeMap::new(),
        };
        for (document, record) in index.records.iter().enumerate() {
            if record.tombstone {
                continue;
            }
            if record.metadata.get("kind").map(String::as_str) == Some("fact") {
                for name in fact_names(&record.logical_id, &record.content, &record.metadata) {
                    index
                        .fact_name_owners
                        .entry(name)
                        .or_default()
                        .insert(document);
                }
            }
            if let IndexedText::Knowledge(projection) = &index.texts[document] {
                for fact in &projection.facts {
                    let mut names = fact_names("", &fact.subject, &BTreeMap::new());
                    names.push(terms(&fact.subject));
                    for name in names.into_iter().filter(|name| !name.is_empty()) {
                        index
                            .fact_name_owners
                            .entry(name)
                            .or_default()
                            .insert(document);
                    }
                }
            }
            let atomic = record.metadata.contains_key("domain_contract")
                || matches!(
                    record.metadata.get("kind").map(String::as_str),
                    Some("fact" | "rule")
                );
            let indexed_text = &index.texts[document];
            let text = indexed_text.text();
            let chunker_version = indexed_text.chunker_version();
            for (ordinal, (start, end)) in indexed_text.ranges(atomic).into_iter().enumerate() {
                if index.chunks.len() >= 500_000 {
                    return Err(PortError::BudgetExceeded);
                }
                let identity = serde_json::to_vec(&(
                    &index.release.release_id,
                    &record.source_id,
                    &record.logical_id,
                    record.revision,
                    &record.content_hash,
                    chunker_version,
                    start,
                    end,
                ))
                .expect("identity serializes");
                let id = if let IndexedText::Knowledge(projection) = indexed_text {
                    let bound = serde_json::to_vec(&(&identity, &projection.semantic_sha256))
                        .expect("projection identity serializes");
                    format!("ev-{:x}", Sha256::digest(bound))
                } else {
                    format!("ev-{:x}", Sha256::digest(identity))
                };
                let chunk_id = index.chunks.len();
                let text = &text[start..end];
                let mut words = terms(text);
                words.extend(
                    terms(&record.logical_id)
                        .into_iter()
                        .filter(|t| numeric_terms(t).is_empty()),
                );
                for key in [
                    "title",
                    "name",
                    "canonical_name",
                    "aliases",
                    "aliases_de",
                    "aliases_en",
                ] {
                    if let Some(value) = record.metadata.get(key) {
                        words.extend(
                            terms(value)
                                .into_iter()
                                .filter(|t| numeric_terms(t).is_empty()),
                        );
                    }
                }
                index.lengths.push(words.len().max(1));
                let mut frequencies = BTreeMap::<String, u32>::new();
                for word in words {
                    *frequencies.entry(word).or_default() += 1;
                }
                for (term, frequency) in frequencies {
                    index
                        .postings
                        .entry(term)
                        .or_default()
                        .push((chunk_id, frequency));
                }
                index.by_id.insert(id.clone(), chunk_id);
                index
                    .by_document
                    .entry((
                        record.source_id.clone(),
                        record.logical_id.clone(),
                        record.revision,
                    ))
                    .or_default()
                    .push(chunk_id);
                index.chunks.push(IndexedChunk {
                    document,
                    start,
                    end,
                    ordinal: ordinal as u32,
                    id,
                    numbers: numeric_terms(text),
                });
            }
        }
        if !index.lengths.is_empty() {
            index.average_length =
                index.lengths.iter().sum::<usize>() as f64 / index.lengths.len() as f64;
        }
        Ok(index)
    }
    pub fn eligible(
        &self,
        record: &SourceRecordV2,
        query: &Query,
        context: &AuthorizedContext,
    ) -> bool {
        !record.tombstone
            && record_allowed(record, &context.principal, false)
            && brain_contracts::source::patch_validity_for(&record.metadata, query.patch.as_deref())
                .is_ok()
            && query
                .mode
                .as_ref()
                .is_none_or(|m| record.metadata.get("mode") == Some(m))
    }
    pub fn rank(&self, query: &Query, context: &AuthorizedContext) -> Vec<(usize, f64)> {
        let query_words = terms(&query.text);
        let mut query_terms: BTreeSet<_> = query_words
            .iter()
            .filter(|term| {
                query.profile != AnswerProfile::Explain
                    || !matches!(
                        term.as_str(),
                        "welche"
                            | "welcher"
                            | "welches"
                            | "wie"
                            | "was"
                            | "wo"
                            | "gibt"
                            | "es"
                            | "auf"
                            | "dem"
                            | "der"
                            | "die"
                            | "das"
                            | "den"
                            | "ein"
                            | "eine"
                            | "und"
                            | "oder"
                            | "ist"
                            | "sind"
                    )
            })
            .cloned()
            .collect();
        if query.profile == AnswerProfile::Explain {
            let singulars: Vec<_> = query_terms
                .iter()
                .filter(|term| {
                    term.len() > 4 && !term.ends_with("ss") && term.chars().all(char::is_alphabetic)
                })
                .filter_map(|term| term.strip_suffix('s'))
                .filter(|term| self.postings.contains_key(*term))
                .map(str::to_owned)
                .collect();
            query_terms.extend(singulars);
        }
        let fact_documents: Option<BTreeSet<usize>> =
            (query.profile == AnswerProfile::Fact).then(|| {
                self.fact_name_owners
                    .iter()
                    .filter(|(name, _)| {
                        !name.is_empty()
                            && query_words.windows(name.len()).any(|part| part == *name)
                    })
                    .flat_map(|(_, owners)| owners.iter().copied())
                    .filter(|document| self.eligible(&self.records[*document], query, context))
                    .collect()
            });
        let numbers = numeric_terms(&query.text);
        let mut scores = BTreeMap::<usize, f64>::new();
        for term in &query_terms {
            let Some(postings) = self.postings.get(term) else {
                continue;
            };
            let df = postings.len() as f64;
            let idf = (1.0 + (self.chunks.len() as f64 - df + 0.5) / (df + 0.5)).ln();
            for &(chunk, frequency) in postings {
                let entry = &self.chunks[chunk];
                if fact_documents
                    .as_ref()
                    .is_some_and(|documents| !documents.contains(&entry.document))
                    || (query.profile != AnswerProfile::Fact && !numbers.is_subset(&entry.numbers))
                    || !self.eligible(&self.records[entry.document], query, context)
                {
                    continue;
                }
                let tf = frequency as f64;
                let norm = K1 * (1.0 - B + B * self.lengths[chunk] as f64 / self.average_length);
                let record = &self.records[entry.document];
                let title = record
                    .metadata
                    .get("title")
                    .map(String::as_str)
                    .unwrap_or_else(|| {
                        self.texts[entry.document]
                            .text()
                            .lines()
                            .next()
                            .unwrap_or("")
                    });
                let title_weight =
                    if query.profile == AnswerProfile::Explain && terms(title).contains(term) {
                        2.0
                    } else {
                        1.0
                    };
                *scores.entry(chunk).or_default() +=
                    title_weight * idf * tf * (K1 + 1.0) / (tf + norm);
            }
        }
        let mut ranked: Vec<_> = scores.into_iter().collect();
        ranked.sort_by(|(a, sa), (b, sb)| {
            sb.total_cmp(sa)
                .then_with(|| self.chunks[*a].id.cmp(&self.chunks[*b].id))
        });
        ranked
    }
    pub fn matching_fact_owners<'a>(
        &'a self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Vec<Vec<&'a SourceRecordV2>> {
        let query_words = terms(&query.text);
        self.fact_name_owners
            .iter()
            .filter(|(name, _)| {
                !name.is_empty() && query_words.windows(name.len()).any(|part| part == *name)
            })
            .map(|(_, owners)| {
                owners
                    .iter()
                    .map(|document| &self.records[*document])
                    .filter(|record| self.eligible(record, query, context))
                    .collect::<Vec<_>>()
            })
            .filter(|owners| owners.len() > 1)
            .collect()
    }
    pub fn document(&self, chunk: usize) -> DocumentRevision {
        let record = &self.records[self.chunks[chunk].document];
        DocumentRevision {
            source_id: record.source_id.clone(),
            logical_id: record.logical_id.clone(),
            revision: record.revision,
            content_hash: record.content_hash.clone(),
        }
    }
    pub fn evidence(&self, chunk: usize, effective: &DocumentHead, score: f64) -> Evidence {
        let entry = &self.chunks[chunk];
        let original = &self.records[entry.document];
        let public_maintenance = original.source_id.starts_with("maintenance-docs:")
            && effective.visibility == brain_contracts::SourceVisibility::Public;
        let locator = if let IndexedText::Knowledge(projection) = &self.texts[entry.document] {
            projection.source_locator.clone()
        } else if public_maintenance {
            original.logical_id.clone()
        } else {
            original
                .metadata
                .get("locator")
                .or_else(|| original.metadata.get("path"))
                .or_else(|| original.metadata.get("relative_path"))
                .cloned()
                .unwrap_or_else(|| original.logical_id.clone())
        };
        let public_metadata = original
            .metadata
            .iter()
            .filter(|(key, _)| {
                matches!(
                    key.as_str(),
                    "content_format"
                        | "language"
                        | "kind"
                        | "patch"
                        | "mode"
                        | "evidence_status"
                        | "currentness"
                        | "source_date"
                )
            })
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();
        Evidence {
            evidence_id: entry.id.clone(),
            source_id: original.source_id.clone(),
            logical_id: original.logical_id.clone(),
            revision: original.revision,
            kind: if matches!(&self.texts[entry.document], IndexedText::Knowledge(_)) {
                EvidenceKind::Prose
            } else {
                match original.metadata.get("kind").map(String::as_str) {
                    Some("fact") => EvidenceKind::Fact,
                    Some("rule") => EvidenceKind::Rule,
                    Some("mechanic") => EvidenceKind::Mechanic,
                    Some("population") => EvidenceKind::Population,
                    Some("replay") => EvidenceKind::Replay,
                    _ => EvidenceKind::Prose,
                }
            },
            content: if original.source_id == "playdeadlock_forum" {
                format!("Forumbericht, unbestätigt. Aktuelle Gültigkeit und Behebung unbekannt. Beitragsdatum: {}. Quelle: {}.\n\n{}", original.metadata.get("source_date").map(String::as_str).unwrap_or("unknown"), locator, &self.texts[entry.document].text()[entry.start..entry.end])
            } else {
                self.texts[entry.document].text()[entry.start..entry.end].into()
            },
            citation: if matches!(&self.texts[entry.document], IndexedText::Knowledge(_)) {
                format!(
                    "brain:{}@{}#basis={}&bytes={}-{}",
                    entry.id,
                    original.revision,
                    crate::knowledge_projection::KNOWLEDGE_BYTE_BASIS,
                    entry.start,
                    entry.end
                )
            } else {
                format!(
                    "brain:{}@{}#bytes={}-{}",
                    entry.id, original.revision, entry.start, entry.end
                )
            },
            visibility: effective.visibility,
            allowed_scopes: effective.allowed_scopes.clone(),
            score,
            patch: match brain_contracts::source::patch_validity_for(&original.metadata, None) {
                Ok(brain_contracts::value::Observed::Known { value }) => Some(value),
                _ => None,
            },
            provenance: Some(ChunkProvenance {
                document: self.document(chunk),
                chunker_version: self.texts[entry.document].chunker_version().into(),
                ordinal: entry.ordinal,
                byte_start: entry.start,
                byte_end: entry.end,
                source_locator: locator,
                release_id: self.release.release_id.clone(),
                knowledge_version: self.release.knowledge_version.clone(),
                valid_from: original.valid_from.clone(),
                valid_to: original.valid_to.clone(),
                metadata: if let IndexedText::Knowledge(projection) = &self.texts[entry.document] {
                    projection.provenance_metadata(original, entry.start)
                } else if public_maintenance {
                    public_metadata
                } else {
                    original.metadata.clone()
                },
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn release_preflight_uses_exact_pins_and_the_same_index_counts() {
        let record = SourceRecordV2 {
            source_id: "legacy".into(),
            logical_id: "document".into(),
            revision: 2,
            content_hash: format!("{:x}", Sha256::digest("Überliefert".as_bytes())),
            content: "Überliefert".into(),
            visibility: brain_contracts::SourceVisibility::Internal,
            allowed_scopes: BTreeSet::new(),
            tombstone: false,
            valid_from: None,
            valid_to: None,
            metadata: BTreeMap::new(),
        };
        let release = CorpusRelease {
            release_id: "preflight".into(),
            knowledge_version: "knowledge".into(),
            patch: "patch".into(),
            created_at_epoch: 1,
            source_revisions: BTreeMap::from([(
                "legacy".into(),
                BTreeMap::from([("document".into(), 2)]),
            )]),
        };
        let index = ChunkIndex::build(release.clone(), vec![record.clone()]).unwrap();
        let proof = preflight_release_index(release.clone(), vec![record.clone()]).unwrap();
        assert_eq!(proof.documents, 1);
        assert_eq!(proof.indexed_documents, index.records.len());
        assert_eq!(proof.projected_bytes, index.texts[0].text().len());
        assert_eq!(proof.chunks, index.chunks.len());
        assert!(preflight_release_index(release.clone(), Vec::new()).is_err());
        assert!(
            preflight_release_index(release.clone(), vec![record.clone(), record.clone()]).is_err()
        );
        let mut newer = record.clone();
        newer.revision = 3;
        assert!(preflight_release_index(release.clone(), vec![newer]).is_err());
        let mut excessive = release.clone();
        excessive.source_revisions.insert(
            "many".into(),
            (0..10_000).map(|id| (id.to_string(), 1)).collect(),
        );
        assert!(preflight_release_index(excessive, vec![record.clone()]).is_err());
        let mut domain = record;
        domain.metadata.insert("kind".into(), "domain_input".into());
        let proof = preflight_release_index(release, vec![domain]).unwrap();
        assert_eq!(proof.documents, 1);
        assert_eq!(proof.indexed_documents, 0);
        assert_eq!(proof.projected_bytes, 0);
        assert_eq!(proof.chunks, 0);
    }

    #[test]
    fn projected_budget_stops_before_requesting_later_documents() {
        let mut projected = 0;
        let texts = (0..3).map(|_| {
            projected += 1;
            assert!(projected <= 2);
            Ok(IndexedText::Knowledge(
                crate::knowledge_projection::KnowledgeProjection {
                    text: "Faktenbeleg".repeat(2),
                    raw_sha256: String::new(),
                    semantic_sha256: String::new(),
                    raw_byte_end: 1,
                    source_locator: String::new(),
                    document_evidence_status: String::new(),
                    facts: Vec::new(),
                },
            ))
        });
        assert!(matches!(
            collect_indexed_texts(texts, 30),
            Err(PortError::BudgetExceeded)
        ));
        assert_eq!(projected, 2);
    }

    #[test]
    fn projected_budget_accepts_exact_total_and_counts_utf8_bytes() {
        let texts = vec![
            Ok(IndexedText::Raw("ä".into())),
            Ok(IndexedText::Html("ö".into())),
        ];
        assert_eq!(
            collect_indexed_texts(texts.into_iter(), 4).unwrap().len(),
            2
        );
        let texts = vec![
            Ok(IndexedText::Raw("ä".into())),
            Ok(IndexedText::Html("ö".into())),
        ];
        assert!(matches!(
            collect_indexed_texts(texts.into_iter(), 3),
            Err(PortError::BudgetExceeded)
        ));
    }

    #[test]
    fn chunk_ranges_are_deterministic_utf8_and_cover_every_byte() {
        let text = "Abschnitt äöü 🎯. Exact -123.45%\n\"Key\": \"BonusMaxHealthPerHero\",\n\"Value\": 650\n".repeat(300);
        let chunks = ranges(&text, false);
        assert_eq!(chunks, ranges(&text, false));
        assert!(chunks.len() > 2);
        let mut covered = vec![false; text.len()];
        for (start, end) in chunks {
            assert!(text.is_char_boundary(start) && text.is_char_boundary(end));
            assert!(end - start <= TARGET_BYTES);
            covered[start..end].fill(true);
        }
        assert!(covered.into_iter().all(|b| b));
    }
    #[test]
    fn long_atomic_tokens_are_not_truncated_or_split_into_fake_numbers() {
        let text = format!("{} rest", "7".repeat(2048));
        let chunks = ranges(&text, false);
        assert!(chunks[0].1 >= 2048);
        assert_eq!(
            numeric_terms("6.5 65 -6.5 6,5"),
            BTreeSet::from(["6.5".into(), "65".into(), "-6.5".into(), "6,5".into()])
        );
    }
}
