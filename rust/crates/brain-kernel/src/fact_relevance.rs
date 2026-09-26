//! Deterministic anchors for the release-backed, text-only fact path. Typed
//! domain facts are resolved by the domain retriever before this path runs.
use brain_contracts::{Evidence, EvidenceKind, Query};
use std::collections::BTreeSet;

fn words(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric() && c != '_')
        .filter(|word| !word.is_empty())
        .map(str::to_owned)
        .collect()
}

fn contains_phrase(haystack: &[String], needle: &[String]) -> bool {
    !needle.is_empty() && haystack.windows(needle.len()).any(|part| part == needle)
}

fn identity(evidence: &Evidence) -> Vec<Vec<String>> {
    let Some(provenance) = &evidence.provenance else {
        return Vec::new();
    };
    let mut names = Vec::new();
    for key in [
        "name",
        "canonical_name",
        "title",
        "aliases",
        "aliases_de",
        "aliases_en",
    ] {
        if let Some(value) = provenance.metadata.get(key) {
            for name in value.split(',') {
                names.push(words(name));
            }
        }
    }
    if let Some((_, name)) = evidence.logical_id.rsplit_once('/') {
        names.push(words(name));
    }
    for line in evidence.content.lines() {
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        if matches!(
            key.trim().to_ascii_lowercase().as_str(),
            "aliases" | "alias"
        ) {
            names.extend(value.split(',').map(words));
        } else if matches!(
            key.trim().to_ascii_lowercase().as_str(),
            "hero" | "item" | "entity"
        ) {
            names.push(words(value));
        }
    }
    names.retain(|name| !name.is_empty());
    names.sort();
    names.dedup();
    names
}

fn fields(evidence: &Evidence) -> Vec<Vec<String>> {
    let Some(provenance) = &evidence.provenance else {
        return Vec::new();
    };
    let mut keys: Vec<Vec<String>> = ["fact_key", "field", "key"]
        .into_iter()
        .filter_map(|key| provenance.metadata.get(key))
        .map(|value| words(value))
        .collect();
    for line in evidence.content.lines() {
        if let Some((key, _)) = line.split_once(':') {
            let key = key.trim();
            if !matches!(key.to_ascii_lowercase().as_str(), "aliases" | "alias") {
                keys.push(words(key));
            }
        }
    }
    keys.retain(|key| !key.is_empty());
    keys
}

/// BM25 orders candidates; it cannot make a fact authoritative. Require an
/// explicit entity/alias and an explicit field in the query. An alias that
/// identifies more than one retrieved entity cannot select either one.
pub(super) fn select<'a>(query: &Query, evidence: &'a [Evidence]) -> Option<&'a Evidence> {
    let query_words = words(&query.text);
    let mut matched_entities = BTreeSet::new();
    let mut candidates = Vec::new();
    for item in evidence
        .iter()
        .filter(|item| item.kind == EvidenceKind::Fact)
    {
        let Some(provenance) = &item.provenance else {
            continue;
        };
        if query.patch.as_ref().is_some_and(|patch| {
            provenance.metadata.get("patch") != Some(patch) || item.patch.as_ref() != Some(patch)
        }) || query
            .mode
            .as_ref()
            .is_some_and(|mode| provenance.metadata.get("mode") != Some(mode))
        {
            continue;
        }
        if !identity(item)
            .iter()
            .any(|name| contains_phrase(&query_words, name))
        {
            continue;
        }
        matched_entities.insert((&item.source_id, &item.logical_id));
        if fields(item)
            .iter()
            .any(|field| contains_phrase(&query_words, field))
        {
            candidates.push(item);
        }
    }
    if matched_entities.len() != 1 {
        return None;
    }
    candidates.into_iter().next()
}

#[cfg(test)]
mod tests {
    use super::*;
    use brain_contracts::{AnswerProfile, ChunkProvenance, DocumentRevision, SourceVisibility};
    use std::collections::BTreeMap;

    fn fact(entity: &str, fields: &str, aliases: &str) -> Evidence {
        let logical_id = format!("entity/hero/{entity}");
        let content = format!("hero: {entity}\nAliases: {aliases}\n{fields}");
        Evidence {
            evidence_id: logical_id.clone(),
            source_id: "fixture".into(),
            logical_id: logical_id.clone(),
            revision: 1,
            kind: EvidenceKind::Fact,
            content: content.clone(),
            citation: "fixture".into(),
            visibility: SourceVisibility::Public,
            allowed_scopes: BTreeSet::new(),
            score: 9.0,
            provenance: Some(ChunkProvenance {
                document: DocumentRevision {
                    source_id: "fixture".into(),
                    logical_id,
                    revision: 1,
                    content_hash: "fixture".into(),
                },
                chunker_version: "fixture".into(),
                ordinal: 0,
                byte_start: 0,
                byte_end: content.len(),
                source_locator: "fixture".into(),
                release_id: "r1".into(),
                knowledge_version: "v1".into(),
                valid_from: None,
                valid_to: None,
                metadata: BTreeMap::new(),
            }),
            patch: Some("p1".into()),
        }
    }
    fn query(text: &str) -> Query {
        Query {
            domain: None,
            request_id: "q1".into(),
            conversation_id: "c1".into(),
            text: text.into(),
            requested_scopes: BTreeSet::new(),
            profile: AnswerProfile::Fact,
            patch: None,
            mode: None,
        }
    }
    #[test]
    fn entity_field_alias_and_numeric_anchors() {
        let abrams = fact("Abrams", "health: 650\ndamage: 42", "The Cop");
        let evidence = [abrams];
        assert!(select(&query("Abrams health"), &evidence).is_some());
        assert!(select(&query("The Cop health"), &evidence).is_some());
        assert!(select(&query("Abrams armor"), &evidence).is_none());
        assert!(select(&query("Seven health"), &evidence).is_none());
        assert!(select(&query("Abrams"), &evidence).is_none());
        assert!(select(&query("unrelated health"), &evidence).is_none());
        assert!(select(&query("Abrams health 650"), &evidence).is_some());
    }
    #[test]
    fn ambiguous_alias_does_not_select_highest_bm25_score() {
        let a = fact("Abrams", "health: 650", "Guardian");
        let mut b = fact("Warden", "health: 700", "Guardian");
        b.score = 1.0;
        assert!(select(&query("Guardian health"), &[a, b]).is_none());
    }
    #[test]
    fn patch_mode_and_provenance_are_required_when_requested() {
        let mut a = fact("Abrams", "health: 650", "The Cop");
        let mut query = query("Abrams health");
        query.patch = Some("p1".into());
        query.mode = Some("ranked".into());
        assert!(select(&query, &[a.clone()]).is_none());
        let metadata = &mut a.provenance.as_mut().unwrap().metadata;
        metadata.insert("patch".into(), "p1".into());
        metadata.insert("mode".into(), "ranked".into());
        assert!(select(&query, &[a.clone()]).is_some());
        query.patch = Some("p2".into());
        assert!(select(&query, &[a.clone()]).is_none());
        query.patch = Some("p1".into());
        query.mode = Some("casual".into());
        assert!(select(&query, &[a.clone()]).is_none());
        a.provenance = None;
        query.patch = None;
        query.mode = None;
        assert!(select(&query, &[a]).is_none());
    }
}
