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
    if !logical_id.starts_with("asset/") || logical_id.split('/').count() <= 3 {
        if let Some((_, name)) = logical_id.rsplit_once('/') {
            names.push(terms(name));
        }
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

/// Several field records from one asset share the upstream entity ID. The
/// reviewed legacy Assets mirror may join that identity only with its explicit
/// source label and external ID; names alone never merge entities.
pub fn fact_entity_key(
    source_id: &str,
    logical_id: &str,
    _content: &str,
    metadata: &BTreeMap<String, String>,
) -> (String, String) {
    let mut parts = logical_id.split('/');
    if parts.next() == Some("asset") {
        if let (Some(kind), Some(id)) = (parts.next(), parts.next()) {
            if !kind.is_empty() && !id.is_empty() {
                if source_id == "deadlock-assets-heroes"
                    && metadata.get("connector").map(String::as_str) == Some("deadlock-assets")
                    && kind == "hero"
                {
                    if let (Ok(number), Some(name)) = (id.parse::<u64>(), metadata.get("name")) {
                        let canonical = terms(name).join("_");
                        if !canonical.is_empty() {
                            return (
                                "deadlock_assets_api".into(),
                                format!("hero/{number}/{canonical}"),
                            );
                        }
                    }
                }
                return (source_id.to_owned(), format!("asset/{kind}/{id}"));
            }
        }
    }
    if source_id == "legacy-entities"
        && metadata.get("connector").map(String::as_str) == Some("brain_legacy")
        && logical_id.starts_with("entity/hero/")
        && metadata.get("entity_source").map(String::as_str) == Some("deadlock_assets_api")
    {
        if let Some(id) = metadata.get("entity_external_id").map(String::as_str) {
            let id = id.strip_prefix("hero_").unwrap_or(id);
            if let Ok(number) = id.parse::<u64>() {
                let canonical = terms(logical_id.trim_start_matches("entity/hero/")).join("_");
                if !canonical.is_empty() {
                    return (
                        "deadlock_assets_api".into(),
                        format!("hero/{number}/{canonical}"),
                    );
                }
            }
        }
    }
    (source_id.to_owned(), logical_id.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_source_join_requires_matching_id_and_canonical_name() {
        let assets = BTreeMap::from([
            ("connector".into(), "deadlock-assets".into()),
            ("name".into(), "Warden".into()),
        ]);
        let legacy = BTreeMap::from([
            ("connector".into(), "brain_legacy".into()),
            ("entity_source".into(), "deadlock_assets_api".into()),
            ("entity_external_id".into(), "hero_25".into()),
        ]);
        let asset_key = fact_entity_key(
            "deadlock-assets-heroes",
            "asset/hero/25/starting_stats.max_health.value",
            "Hero: Warden\nmax health: 770",
            &assets,
        );
        let legacy_key = fact_entity_key(
            "legacy-entities",
            "entity/hero/Warden",
            "hero: Warden\nExternal ID: hero_25\nSource: deadlock_assets_api\n",
            &legacy,
        );
        assert_eq!(asset_key, legacy_key);
        assert_ne!(
            asset_key,
            fact_entity_key(
                "legacy-entities",
                "entity/hero/Other",
                "hero: Other\nExternal ID: hero_25\nSource: deadlock_assets_api\n",
                &legacy,
            )
        );
        assert_eq!(
            asset_key,
            fact_entity_key(
                "legacy-entities",
                "entity/hero/Warden",
                "hero: Warden\nExternal ID: hero_25\nSource: deadlock_assets_api\nExternal ID: hero_999\n",
                &legacy,
            )
        );
        assert_eq!(
            asset_key,
            fact_entity_key(
                "legacy-entities",
                "entity/hero/Warden",
                "hero: Warden\nExternal ID: hero_25\nSource: deadlock_assets_api\nSource: other\n",
                &legacy,
            )
        );
        let untrusted = BTreeMap::from([("connector".into(), "brain_legacy".into())]);
        assert_ne!(
            asset_key,
            fact_entity_key(
                "legacy-entities",
                "entity/hero/Warden",
                "hero: Warden\nExternal ID: hero_25\nSource: deadlock_assets_api\n",
                &untrusted,
            )
        );
    }
}
