//! Deterministic anchors for the release-backed, text-only fact path. Typed
//! domain facts are resolved by the domain retriever before this path runs.
use brain_contracts::{
    lexical::{fact_entity_key, fact_names, terms},
    Evidence, EvidenceKind, Query,
};
use std::collections::BTreeSet;

fn words(text: &str) -> Vec<String> {
    terms(text)
}

fn contains_phrase(haystack: &[String], needle: &[String]) -> bool {
    !needle.is_empty() && haystack.windows(needle.len()).any(|part| part == needle)
}

fn identity(evidence: &Evidence) -> Vec<Vec<String>> {
    let Some(provenance) = &evidence.provenance else {
        return Vec::new();
    };
    fact_names(
        &evidence.logical_id,
        &evidence.content,
        &provenance.metadata,
    )
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
            if !matches!(
                key.to_ascii_lowercase().as_str(),
                "aliases" | "alias" | "hero" | "item" | "entity"
            ) {
                keys.push(words(key));
            }
        }
    }
    keys.retain(|key| !key.is_empty());
    keys
}

fn numbers(text: &str) -> BTreeSet<String> {
    words(text)
        .into_iter()
        .filter(|term| {
            term.chars()
                .next()
                .is_some_and(|c| c.is_ascii_digit() || c == '-')
        })
        .collect()
}

fn field_value_numbers(
    evidence: &Evidence,
    query_words: &[String],
    specificity: usize,
) -> Option<BTreeSet<String>> {
    let mut matched = Vec::new();
    let mut values = Vec::new();
    for line in evidence.content.lines() {
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        let key_words = words(key);
        if key_words.len() == specificity && contains_phrase(query_words, &key_words) {
            matched.push(numbers(value));
        }
        if key.eq_ignore_ascii_case("value") {
            values.push(numbers(value));
        }
    }
    if matched.len() == 1 {
        return matched.pop();
    }
    if matched.is_empty() {
        let provenance = evidence.provenance.as_ref()?;
        if let (Some(fact_key), Some(field)) = (
            provenance.metadata.get("fact_key"),
            provenance.metadata.get("field"),
        ) {
            let key_words = words(fact_key);
            if key_words.len() == specificity && contains_phrase(query_words, &key_words) {
                let field_words = words(field);
                let mut bound = evidence
                    .content
                    .lines()
                    .filter_map(|line| line.split_once(':'))
                    .filter(|(key, _)| words(key) == field_words)
                    .map(|(_, value)| numbers(value));
                if let Some(value) = bound.next() {
                    if bound.next().is_none() {
                        return Some(value);
                    }
                }
            }
        }
    }
    if matched.is_empty() && values.len() == 1 {
        return values.pop();
    }
    None
}

/// BM25 orders candidates; it cannot make a fact authoritative. Require an
/// explicit entity/alias and an explicit field in the query. An alias that
/// identifies more than one retrieved entity cannot select either one.
pub(super) fn select<'a>(query: &Query, evidence: &'a [Evidence]) -> Option<&'a Evidence> {
    let query_words = words(&query.text);
    let query_numbers = numbers(&query.text);
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
            item.patch.as_ref() != Some(patch)
                || provenance
                    .metadata
                    .get("patch")
                    .is_some_and(|value| value != patch)
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
        matched_entities.insert(fact_entity_key(
            &item.source_id,
            &item.logical_id,
            &item.content,
            &provenance.metadata,
        ));
        if let Some(specificity) = fields(item)
            .iter()
            .filter(|field| contains_phrase(&query_words, field))
            .map(Vec::len)
            .max()
        {
            candidates.push((item, specificity));
        }
    }
    if matched_entities.len() != 1 {
        return None;
    }
    // Prefer the most specific field explicitly named in the question:
    // "max health" excludes a generic "health" claim from conflict detection.
    let specificity = candidates.iter().map(|(_, length)| *length).max()?;
    candidates.retain(|(_, length)| *length == specificity);
    // A second record asserting the same entity and field is a conflicting
    // fact source until explicitly reconciled. Numbers in the question cannot
    // hide that conflict through the lexical index's numeric prefilter.
    if candidates.len() != 1 {
        return None;
    }
    let fact = candidates.into_iter().next()?.0;
    if query_numbers.is_empty() {
        return Some(fact);
    }
    let values = field_value_numbers(fact, &query_words, specificity)?;
    query_numbers.is_subset(&values).then_some(fact)
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
        assert!(select(&query("Abrams hero"), &evidence).is_none());
        assert!(select(&query("unrelated health"), &evidence).is_none());
        assert!(select(&query("Abrams health 650"), &evidence).is_some());
        assert!(select(&query("Abrams health 999"), &evidence).is_none());
        assert!(select(&query("Abrams health 42"), &evidence).is_none());
        assert!(select(&query("Abrams Gesundheit"), &evidence).is_some());
        assert!(select(&query("Abrams Lebenspunkte"), &evidence).is_some());
        assert!(select(&query("Abrams Schaden"), &evidence).is_some());
    }
    #[test]
    fn metadata_aliases_use_release_tokenizer_and_real_delimiters() {
        let mut geist = fact("Lady Geist", "health: 650", "");
        geist
            .provenance
            .as_mut()
            .unwrap()
            .metadata
            .insert("aliases_de".into(), "Geisterdame; Grüne Lady".into());
        assert!(select(&query("GRUENE LADY Gesundheit"), &[geist.clone()]).is_some());
        assert!(select(&query("Grüne Lady Gesundheit"), &[geist.clone()]).is_some());
        assert!(select(&query("Geisterdame health"), &[geist]).is_some());
    }
    #[test]
    fn ambiguous_alias_does_not_select_highest_bm25_score() {
        let a = fact("Abrams", "health: 650", "Guardian");
        let mut b = fact("Warden", "health: 700", "Guardian");
        b.score = 1.0;
        assert!(select(&query("Guardian health"), &[a, b]).is_none());
    }
    #[test]
    fn same_entity_field_with_two_sources_is_not_selected_by_score() {
        let mut a = fact("Warden", "max health: 770", "");
        a.source_id = "deadlock-assets-heroes".into();
        a.logical_id = "asset/hero/25/starting_stats.max_health.value".into();
        let mut b = fact("Warden", "max health: 700", "");
        b.source_id = a.source_id.clone();
        b.logical_id = "asset/hero/25/legacy_max_health".into();
        b.score = 1.0;
        assert!(select(&query("Warden max health"), &[a.clone(), b.clone()]).is_none());
        assert!(select(&query("Warden max health 770"), &[a, b]).is_none());
    }
    #[test]
    fn specific_field_outranks_generic_field_for_one_entity() {
        let mut specific = fact("Warden", "max health: 770", "");
        specific.source_id = "deadlock-assets-heroes".into();
        specific.logical_id = "asset/hero/25/starting_stats.max_health.value".into();
        let mut generic = fact("Warden", "health: 700", "");
        generic.source_id = specific.source_id.clone();
        generic.logical_id = "asset/hero/25".into();
        generic.score = 20.0;
        let evidence = [generic.clone(), specific.clone()];
        let selected = select(&query("Warden max health"), &evidence).unwrap();
        assert_eq!(selected.logical_id, specific.logical_id);
        assert!(select(&query("Warden max health 770"), &[generic, specific]).is_some());
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
        query.mode = Some("ranked".into());
        a.provenance.as_mut().unwrap().metadata.remove("patch");
        assert!(select(&query, &[a.clone()]).is_some());
        a.provenance
            .as_mut()
            .unwrap()
            .metadata
            .insert("patch".into(), "p0".into());
        assert!(select(&query, &[a.clone()]).is_none());
        a.provenance = None;
        query.patch = None;
        query.mode = None;
        assert!(select(&query, &[a]).is_none());
    }
}
