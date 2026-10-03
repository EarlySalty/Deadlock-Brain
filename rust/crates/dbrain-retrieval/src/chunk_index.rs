//! Release-local inverted BM25 index. Query work traverses postings, not corpus documents.
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
    texts: Vec<String>,
    pub chunks: Vec<IndexedChunk>,
    pub by_id: BTreeMap<String, usize>,
    pub by_document: BTreeMap<(String, String, u64), Vec<usize>>,
    postings: BTreeMap<String, Vec<(usize, u32)>>,
    lengths: Vec<usize>,
    average_length: f64,
    fact_name_owners: BTreeMap<Vec<String>, BTreeSet<usize>>,
}

/// No normalization of numeric literals: 6.5, 65, -6.5 and 6,5 remain distinct.
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

/// Exact overlapping UTF-8 slices, covering every byte, including whitespace. Overlap keeps
/// adjacent key/value lines together in a candidate even when a window cuts a code block.
/// An indivisible token larger than the target is retained whole, never silently truncated.
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
        // Start only at token boundaries, so numbers are never manufactured by slicing.
        next = text[next..end]
            .char_indices()
            .find(|(_, c)| c.is_whitespace())
            .map_or(end, |(i, c)| next + i + c.len_utf8());
        start = next;
    }
    result
}

impl ChunkIndex {
    pub fn build(
        release: CorpusRelease,
        mut records: Vec<SourceRecordV2>,
    ) -> Result<Self, PortError> {
        records.sort_by(|a, b| {
            (&a.source_id, &a.logical_id, a.revision).cmp(&(
                &b.source_id,
                &b.logical_id,
                b.revision,
            ))
        });
        if records.iter().map(|r| r.content.len()).sum::<usize>() > 256 * 1024 * 1024 {
            return Err(PortError::BudgetExceeded);
        }
        let mut index = Self {
            release,
            texts: records
                .iter()
                .map(|record| {
                    if record.metadata.get("content_format").map(String::as_str) != Some("html") {
                        return Ok(record.content.clone());
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
                    Ok(projection.text)
                })
                .collect::<Result<Vec<_>, PortError>>()?,
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
            // Typed facts/rules are indivisible objects, not prose. Oversized objects fail
            // explicitly at packing rather than turning fragments into authoritative facts.
            let atomic = record.metadata.contains_key("domain_contract")
                || matches!(
                    record.metadata.get("kind").map(String::as_str),
                    Some("fact" | "rule")
                );
            let text = &index.texts[document];
            let chunker_version =
                if record.metadata.get("content_format").map(String::as_str) == Some("html") {
                    HTML_CHUNKER_VERSION
                } else {
                    CHUNKER_VERSION
                };
            for (ordinal, (start, end)) in ranges(text, atomic).into_iter().enumerate() {
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
                let id = format!("ev-{:x}", Sha256::digest(identity));
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
        let query_terms: BTreeSet<_> = query_words.iter().cloned().collect();
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
                *scores.entry(chunk).or_default() += idf * tf * (K1 + 1.0) / (tf + norm);
            }
        }
        let mut ranked: Vec<_> = scores.into_iter().collect();
        ranked.sort_by(|(a, sa), (b, sb)| {
            sb.total_cmp(sa)
                .then_with(|| self.chunks[*a].id.cmp(&self.chunks[*b].id))
        });
        ranked
    }
    /// Group owners by the specific matched name. Different unambiguous names
    /// in one question are not themselves an alias collision.
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
        let locator = if public_maintenance {
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
            kind: match original.metadata.get("kind").map(String::as_str) {
                Some("fact") => EvidenceKind::Fact,
                Some("rule") => EvidenceKind::Rule,
                Some("mechanic") => EvidenceKind::Mechanic,
                Some("population") => EvidenceKind::Population,
                Some("replay") => EvidenceKind::Replay,
                _ => EvidenceKind::Prose,
            },
            content: if original.source_id == "playdeadlock_forum" {
                format!("Forumbericht, unbestätigt. Aktuelle Gültigkeit und Behebung unbekannt. Beitragsdatum: {}. Quelle: {}.\n\n{}", original.metadata.get("source_date").map(String::as_str).unwrap_or("unknown"), locator, &self.texts[entry.document][entry.start..entry.end])
            } else {
                self.texts[entry.document][entry.start..entry.end].into()
            },
            citation: format!(
                "brain:{}@{}#bytes={}-{}",
                entry.id, original.revision, entry.start, entry.end
            ),
            visibility: effective.visibility,
            allowed_scopes: effective.allowed_scopes.clone(),
            score,
            patch: match brain_contracts::source::patch_validity_for(&original.metadata, None) {
                Ok(brain_contracts::value::Observed::Known { value }) => Some(value),
                _ => None,
            },
            provenance: Some(ChunkProvenance {
                document: self.document(chunk),
                chunker_version: if original.metadata.get("content_format").map(String::as_str)
                    == Some("html")
                {
                    HTML_CHUNKER_VERSION
                } else {
                    CHUNKER_VERSION
                }
                .into(),
                ordinal: entry.ordinal,
                byte_start: entry.start,
                byte_end: entry.end,
                source_locator: locator,
                release_id: self.release.release_id.clone(),
                knowledge_version: self.release.knowledge_version.clone(),
                valid_from: original.valid_from.clone(),
                valid_to: original.valid_to.clone(),
                metadata: if public_maintenance {
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
