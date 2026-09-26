//! Shared lexical terms for release retrieval and deterministic fact anchors.
use regex::Regex;
use std::collections::BTreeMap;
use std::sync::OnceLock;

/// Numeric literals retain punctuation; German game-stat vocabulary follows
/// the release index's fixed, versioned normalization.
pub fn terms(text: &str) -> Vec<String> {
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

/// Canonical names and aliases carried by a fact record. Keep this identical
/// for release-wide ambiguity checks and the kernel's final evidence gate.
pub fn fact_names(
    logical_id: &str,
    content: &str,
    metadata: &BTreeMap<String, String>,
) -> Vec<Vec<String>> {
    let mut names = Vec::new();
    for key in [
        "name",
        "canonical_name",
        "title",
        "aliases",
        "aliases_de",
        "aliases_en",
    ] {
        if let Some(value) = metadata.get(key) {
            names.extend(value.split([',', ';']).map(terms));
        }
    }
    if let Some((_, name)) = logical_id.rsplit_once('/') {
        names.push(terms(name));
    }
    for line in content.lines() {
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        if matches!(
            key.trim().to_ascii_lowercase().as_str(),
            "aliases" | "alias"
        ) {
            names.extend(value.split([',', ';']).map(terms));
        } else if matches!(
            key.trim().to_ascii_lowercase().as_str(),
            "hero" | "item" | "entity"
        ) {
            names.push(terms(value));
        }
    }
    names.retain(|name| !name.is_empty());
    names.sort();
    names.dedup();
    names
}
