//! Release-local inverted BM25 index. Query work traverses postings, not corpus documents.
use brain_contracts::{
    store::record_allowed, AuthorizedContext, ChunkProvenance, CorpusRelease, DocumentHead,
    DocumentRevision, Evidence, EvidenceKind, PortError, Query, SourceRecordV2,
};
use regex::Regex;
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::OnceLock,
};

pub(crate) const CHUNKER_VERSION: &str = "utf8-window-v1-1024-overlap192";
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
    pub chunks: Vec<IndexedChunk>,
    pub by_id: BTreeMap<String, usize>,
    pub by_document: BTreeMap<(String, String, u64), Vec<usize>>,
    postings: BTreeMap<String, Vec<(usize, u32)>>,
    lengths: Vec<usize>,
    average_length: f64,
}

/// No normalization of numeric literals: 6.5, 65, -6.5 and 6,5 remain distinct.
pub(crate) fn terms(text: &str) -> Vec<String> {
    static TOKEN: OnceLock<Regex> = OnceLock::new();
    let regex = TOKEN.get_or_init(|| {
        Regex::new(r"-?\d+(?:[.,]\d+)*%?|[\p{L}_][\p{L}\p{N}_]*").expect("constant tokenizer")
    });
    regex
        .find_iter(text)
        .map(|m| {
            let word = m
                .as_str()
                .to_lowercase()
                .replace('ä', "ae")
                .replace('ö', "oe")
                .replace('ü', "ue")
                .replace('ß', "ss");
            // Small explicit DE/EN vocabulary, versioned with the index. Source-provided aliases
            // are indexed separately; no provider or speculative entity resolution is involved.
            match word.as_str() {
                "lebenspunkte" | "gesundheit" => "health".into(),
                "schaden" => "damage".into(),
                "abklingzeit" => "cooldown".into(),
                "reichweite" => "range".into(),
                "faehigkeit" => "ability".into(),
                _ => word,
            }
        })
        .collect()
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
            records,
            chunks: Vec::new(),
            by_id: BTreeMap::new(),
            by_document: BTreeMap::new(),
            postings: BTreeMap::new(),
            lengths: Vec::new(),
            average_length: 1.0,
        };
        for (document, record) in index.records.iter().enumerate() {
            if record.tombstone {
                continue;
            }
            // Typed facts/rules are indivisible objects, not prose. Oversized objects fail
            // explicitly at packing rather than turning fragments into authoritative facts.
            let atomic = record.metadata.contains_key("domain_contract")
                || matches!(
                    record.metadata.get("kind").map(String::as_str),
                    Some("fact" | "rule")
                );
            for (ordinal, (start, end)) in ranges(&record.content, atomic).into_iter().enumerate() {
                if index.chunks.len() >= 500_000 {
                    return Err(PortError::BudgetExceeded);
                }
                let identity = serde_json::to_vec(&(
                    &index.release.release_id,
                    &record.source_id,
                    &record.logical_id,
                    record.revision,
                    &record.content_hash,
                    CHUNKER_VERSION,
                    start,
                    end,
                ))
                .expect("identity serializes");
                let id = format!("ev-{:x}", Sha256::digest(identity));
                let chunk_id = index.chunks.len();
                let text = &record.content[start..end];
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
            && record
                .metadata
                .get("patch")
                .is_none_or(|p| p == &self.release.patch)
            && query
                .mode
                .as_ref()
                .is_none_or(|m| record.metadata.get("mode") == Some(m))
    }
    pub fn rank(&self, query: &Query, context: &AuthorizedContext) -> Vec<(usize, f64)> {
        let query_terms: BTreeSet<_> = terms(&query.text).into_iter().collect();
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
                if !numbers.is_subset(&entry.numbers)
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
        let locator = original
            .metadata
            .get("locator")
            .or_else(|| original.metadata.get("path"))
            .or_else(|| original.metadata.get("relative_path"))
            .cloned()
            .unwrap_or_else(|| original.logical_id.clone());
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
            content: original.content[entry.start..entry.end].into(),
            citation: format!(
                "brain:{}@{}#bytes={}-{}",
                entry.id, original.revision, entry.start, entry.end
            ),
            visibility: effective.visibility,
            allowed_scopes: effective.allowed_scopes.clone(),
            score,
            patch: Some(self.release.patch.clone()),
            provenance: Some(ChunkProvenance {
                document: self.document(chunk),
                chunker_version: CHUNKER_VERSION.into(),
                ordinal: entry.ordinal,
                byte_start: entry.start,
                byte_end: entry.end,
                source_locator: locator,
                release_id: self.release.release_id.clone(),
                knowledge_version: self.release.knowledge_version.clone(),
                valid_from: original.valid_from.clone(),
                valid_to: original.valid_to.clone(),
                metadata: original.metadata.clone(),
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
