use crate::git::Source;
use anyhow::{ensure, Context, Result};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

#[path = "json_document.rs"]
mod json_document;
#[path = "../../brain-storage/src/game_file_facts/json_text.rs"]
mod json_text;
#[path = "../../brain-storage/src/game_file_facts/kv.rs"]
mod kv;
#[path = "../../brain-storage/src/game_file_facts/kv3.rs"]
mod kv3;
#[path = "kv_document.rs"]
mod kv_document;

mod budget {
    pub struct Budget {
        remaining: usize,
    }
    impl Budget {
        pub fn new() -> Self {
            Self {
                remaining: 32 * 1024 * 1024,
            }
        }
        pub fn charge(&mut self, size: usize) -> Result<(), &'static str> {
            self.remaining = self
                .remaining
                .checked_sub(size)
                .ok_or("extraction_budget")?;
            Ok(())
        }
        pub fn expanded(&mut self, lengths: &[usize], overhead: usize) -> Result<(), &'static str> {
            self.charge(
                lengths
                    .iter()
                    .try_fold(overhead, |sum, length| {
                        sum.checked_add(length.checked_mul(8)?)
                    })
                    .ok_or("extraction_budget")?,
            )
        }
    }
    pub fn escaped_len(key: &str) -> usize {
        key.len() + key.bytes().filter(|b| matches!(b, b'~' | b'/')).count()
    }
}

pub const EXTRACTOR: &str = "hero-ability-atoms";
pub const VERSION: &str = "p1-v1";

#[derive(Debug)]
pub struct Assertion {
    pub entity: String,
    pub kind: &'static str,
    pub locator_kind: &'static str,
    pub locator: String,
    pub hash: Vec<u8>,
    pub value: Atom,
    pub condition: Value,
    pub alias: Option<(String, &'static str)>,
    pub relation: Option<(String, i16)>,
    pub stat: String,
    pub section: &'static str,
    pub unit: Option<&'static str>,
    pub order: i32,
}

#[derive(Debug)]
pub enum Atom {
    Number(String),
    Text(String),
}

struct Leaf {
    pointer: String,
    text: String,
    lexeme: String,
    numeric: bool,
    flags: Vec<String>,
}

pub fn supported(source: Source, path: &str) -> bool {
    match source.key {
        "deadlock-data" => matches!(
            path,
            "data/json/hero-data.json"
                | "data/json/ability-data.json"
                | "data/localizations/english.json"
                | "data/localizations/german.json"
        ),
        "gametracking" => matches!(
            path,
            "game/citadel/pak01_dir/scripts/heroes.vdata"
                | "game/citadel/pak01_dir/scripts/abilities.vdata"
        ),
        _ => false,
    }
}

fn leaves(path: &str, bytes: &[u8]) -> Result<Vec<Leaf>> {
    ensure!(bytes.len() <= 16 * 1024 * 1024, "extraction_input_limit");
    let content = std::str::from_utf8(bytes)
        .context("extraction_utf8")?
        .trim_start_matches('\u{feff}');
    if path.ends_with(".json") {
        let parsed = json_document::parse(content).map_err(|_| anyhow::anyhow!("json_parse"))?;
        Ok(parsed
            .into_iter()
            .map(|leaf| Leaf {
                pointer: leaf.pointer,
                text: leaf
                    .value
                    .as_str()
                    .map(str::to_owned)
                    .unwrap_or_else(|| leaf.value.to_string()),
                lexeme: leaf.lexeme,
                numeric: leaf.numeric_representation.is_some(),
                flags: leaf.flags,
            })
            .collect())
    } else if content.trim_start().starts_with("<!--") || content.contains(" = ") {
        let parsed = kv_document::parse(content.trim_start())
            .map_err(anyhow::Error::msg)
            .context("kv3_parse")?;
        Ok(parsed
            .into_iter()
            .map(|leaf| Leaf {
                pointer: leaf.pointer,
                text: leaf
                    .value
                    .as_str()
                    .map(str::to_owned)
                    .unwrap_or_else(|| leaf.value.to_string()),
                lexeme: leaf.lexeme,
                numeric: leaf.numeric_representation.is_some(),
                flags: leaf.flags,
            })
            .collect())
    } else {
        let parsed = kv::parse(content).map_err(|_| anyhow::anyhow!("kv_parse"))?;
        for entry in &parsed {
            ensure!(entry.conditions.is_empty(), "kv_conditions_unsupported");
            let suffix = format!(
                "/{}/{}",
                entry.key.replace('~', "~0").replace('/', "~1"),
                entry.occurrence
            );
            ensure!(entry.pointer.ends_with(&suffix), "kv_locator_metadata");
        }
        Ok(parsed
            .into_iter()
            .map(|leaf| {
                let numeric = json_text::number(&leaf.value).is_ok();
                Leaf {
                    pointer: leaf.pointer,
                    text: leaf.value.clone(),
                    lexeme: leaf.value,
                    numeric,
                    flags: leaf.conditions,
                }
            })
            .collect())
    }
}

fn supported_ability_type(value: &str) -> bool {
    matches!(
        value,
        "EAbilityType_Signature"
            | "EAbilityType_Ultimate"
            | "EAbilityType_Weapon"
            | "EAbilityType_Melee"
            | "EAbilityType_Innate"
            | "EAbilityType_Cosmetic"
            | "EAbilityType_Held"
    )
}

fn entity_kind(
    path: &str,
    key: &str,
    index: &BTreeMap<&str, &Leaf>,
    identities: &BTreeMap<String, &'static str>,
) -> Option<&'static str> {
    match path {
        "data/json/hero-data.json" | "game/citadel/pak01_dir/scripts/heroes.vdata" => Some("hero"),
        "data/json/ability-data.json" => Some("ability"),
        "game/citadel/pak01_dir/scripts/abilities.vdata" => {
            let ability_type = index.get(format!("/{key}/0/m_eAbilityType/0").as_str())?;
            supported_ability_type(&ability_type.text).then_some("ability")
        }
        "data/localizations/english.json" | "data/localizations/german.json" => {
            identities.get(key).copied()
        }
        _ => None,
    }
}

fn valid_key(key: &str) -> bool {
    key.len() <= 128 && key.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
}

fn stat(section: &str, field: &str) -> (String, Option<&'static str>, i32) {
    match field {
        "MaxHealth" if section == "stats" => ("max_health".into(), Some("health"), 0),
        "MaxMoveSpeed" if section == "stats" => ("move_speed".into(), Some("m/s"), 1),
        "Stamina" if section == "stats" => ("stamina".into(), Some("charges"), 2),
        "status" => ("status".into(), None, 0),
        "name" => ("name".into(), None, 1),
        "AbilityCooldown" | "AbilityDuration" | "DebuffDuration" | "ReloadTime" => {
            (field.into(), Some("s"), 3)
        }
        _ => (field.into(), None, 10),
    }
}

fn assertion(
    key: &str,
    kind: &'static str,
    leaf: &Leaf,
    json_source: bool,
    section: &'static str,
    field: &str,
    mut condition: Value,
) -> Assertion {
    if !leaf.flags.is_empty() {
        condition["source_flags"] = Value::String(leaf.flags.join(" "));
    }
    let (field, unit, order) = stat(section, field);
    Assertion {
        entity: key.into(),
        kind,
        locator_kind: if json_source {
            "json_pointer"
        } else {
            "kv_pointer"
        },
        locator: leaf.pointer.clone(),
        hash: Sha256::digest(leaf.lexeme.as_bytes()).to_vec(),
        value: if leaf.numeric {
            Atom::Number(leaf.lexeme.clone())
        } else {
            Atom::Text(leaf.text.clone())
        },
        condition,
        alias: None,
        relation: None,
        stat: format!("{section}.{field}"),
        section,
        unit,
        order,
    }
}

fn gametracking_assertion(
    key: &str,
    kind: &'static str,
    leaf: &Leaf,
    parts: &[&str],
    index: &BTreeMap<&str, &Leaf>,
) -> Result<Option<Assertion>> {
    if kind == "hero"
        && parts.len() == 4
        && parts[1] == "0"
        && parts[2] == "m_eHeroDevelopmentState"
        && parts[3] == "0"
    {
        let Some(status) = release_status(&leaf.text) else {
            return Ok(None);
        };
        let mut fact = assertion(key, kind, leaf, false, "identity", "status", json!({}));
        fact.value = Atom::Text(status.into());
        return Ok(Some(fact));
    }
    if kind == "hero"
        && parts.len() == 6
        && parts[1] == "0"
        && parts[2] == "m_mapStartingStats"
        && parts[3] == "0"
        && parts[5] == "0"
        && leaf.numeric
    {
        if let Some(field) = parts[4].strip_prefix('E') {
            return Ok(Some(assertion(
                key,
                kind,
                leaf,
                false,
                "stats",
                field,
                json!({}),
            )));
        }
    }
    if kind != "ability" {
        return Ok(None);
    }
    if parts.len() == 8
        && parts[1] == "0"
        && parts[2] == "m_mapAbilityProperties"
        && parts[3] == "0"
        && parts[5] == "0"
        && parts[6] == "m_strValue"
        && parts[7] == "0"
    {
        if json_text::number(&leaf.text).is_err() {
            return Ok(None);
        }
        let prefix = format!(
            "/{key}/0/m_mapAbilityProperties/0/{}/0/m_subclassScaleFunction/0",
            parts[4]
        );
        let mut condition = json!({});
        for (field, label) in [
            ("_class", "scale_class"),
            ("m_eSpecificStatScaleType", "scale_type"),
            ("m_flStatScale", "scale_value"),
        ] {
            if let Some(scale) = index.get(format!("{prefix}/{field}/0").as_str()) {
                condition[label] = Value::String(scale.text.clone());
            }
        }
        for i in 0..4 {
            if let Some(scale) = index.get(format!("{prefix}/m_vecScalingStats/0/[{i}]").as_str()) {
                let label = format!("scale_stat_{i}");
                condition[&label] = Value::String(scale.text.clone());
            }
        }
        if condition
            .as_object()
            .is_some_and(|fields| !fields.is_empty())
        {
            condition["scale_locator"] = Value::String(prefix.clone());
        }
        ensure!(
            !index.contains_key(format!("{prefix}/m_vecScalingStats/0/[4]").as_str()),
            "scale_stats_limit"
        );
        let mut fact = assertion(key, kind, leaf, false, "ability", parts[4], condition);
        fact.value = Atom::Number(leaf.text.clone());
        return Ok(Some(fact));
    }
    if parts.len() == 10
        && parts[1] == "0"
        && parts[2] == "m_vecAbilityUpgrades"
        && parts[3] == "0"
        && parts[5] == "m_vecPropertyUpgrades"
        && parts[6] == "0"
        && parts[8] == "m_strBonus"
        && parts[9] == "0"
    {
        if json_text::number(&leaf.text).is_err() {
            return Ok(None);
        }
        let Some(tier) = parts[4]
            .strip_prefix('[')
            .and_then(|v| v.strip_suffix(']'))
            .and_then(|v| v.parse::<u32>().ok())
        else {
            return Ok(None);
        };
        ensure!(tier < 3, "upgrade_tier");
        let prefix = leaf
            .pointer
            .rsplit_once('/')
            .context("upgrade_pointer")?
            .0
            .rsplit_once('/')
            .context("upgrade_pointer")?
            .0;
        let property = match (
            index.get(format!("{prefix}/m_strPropertyName/0").as_str()),
            index.get(format!("{prefix}/m_StrPropertyNAme/0").as_str()),
        ) {
            (Some(property), None) | (None, Some(property)) => property,
            _ => anyhow::bail!("upgrade_property"),
        };
        ensure!(valid_key(&property.text), "upgrade_property");
        let condition = json!({"upgrade_tier":tier+1,"operation":"delta","upgrade_property":property.text,"upgrade_property_locator":property.pointer});
        let mut fact = assertion(key, kind, leaf, false, "ability", &property.text, condition);
        fact.value = Atom::Number(leaf.text.clone());
        return Ok(Some(fact));
    }
    Ok(None)
}

fn release_status(value: &str) -> Option<&'static str> {
    match value {
        "EHeroDevState_Release" => Some("released"),
        "EHeroDevState_HeroLabs" => Some("hero_labs"),
        "EHeroDevState_InDevelopment" => Some("in_development"),
        _ => None,
    }
}

fn push_assertion(
    output: &mut Vec<Assertion>,
    remaining: &mut usize,
    fact: Assertion,
) -> Result<()> {
    let condition = fact.condition.to_string();
    ensure!(
        output.len() < 30000
            && fact.locator.len() <= 2048
            && fact.stat.len() <= 128
            && condition.len() <= 1024,
        "assertion_size"
    );
    let value = match &fact.value {
        Atom::Number(value) => {
            ensure!(value.len() <= 64, "numeric_size");
            value
        }
        Atom::Text(value) => {
            ensure!(value.len() <= 512, "text_size");
            value
        }
    };
    let lengths = fact.entity.len()
        + fact.locator.len()
        + fact.stat.len()
        + value.len()
        + condition.len()
        + fact.alias.as_ref().map_or(0, |a| a.0.len())
        + fact.relation.as_ref().map_or(0, |r| r.0.len());
    let fields = fact.condition.as_object().map_or(0, |fields| fields.len());
    let charge = lengths
        .checked_mul(2)
        .and_then(|n| n.checked_add(1024 + fields * 128))
        .context("assertion_budget")?;
    *remaining = remaining.checked_sub(charge).context("assertion_budget")?;
    output.push(fact);
    Ok(())
}

pub fn extract(source: Source, path: &str, bytes: &[u8]) -> Result<Vec<Assertion>> {
    extract_with_identities(source, path, bytes, &BTreeMap::new())
}

pub fn extract_with_identities(
    source: Source,
    path: &str,
    bytes: &[u8],
    identities: &BTreeMap<String, &'static str>,
) -> Result<Vec<Assertion>> {
    if !supported(source, path) {
        return Ok(Vec::new());
    }
    let parsed = leaves(path, bytes)?;
    let index: BTreeMap<_, _> = parsed
        .iter()
        .map(|leaf| (leaf.pointer.as_str(), leaf))
        .collect();
    let mut output = Vec::new();
    let mut remaining = 32 * 1024 * 1024;
    let localization = path.starts_with("data/localizations/");
    let json_source = path.ends_with(".json");
    for leaf in &parsed {
        ensure!(output.len() < 30000, "assertion_limit");
        let parts: Vec<_> = leaf.pointer.split('/').skip(1).collect();
        let Some(key) = parts.first().copied() else {
            continue;
        };
        let Some(kind) = entity_kind(path, key, &index, identities) else {
            continue;
        };
        if !valid_key(key) {
            continue;
        }
        if localization {
            if parts.len() == 1
                && leaf.lexeme.starts_with('"')
                && leaf.text.len() <= 256
                && !leaf.text.contains(['<', '>'])
            {
                let lang = if path.ends_with("german.json") {
                    "de"
                } else {
                    "en"
                };
                let mut fact = assertion(
                    key,
                    kind,
                    leaf,
                    true,
                    "identity",
                    "name",
                    json!({"lang":lang}),
                );
                fact.alias = Some((leaf.text.clone(), lang));
                push_assertion(&mut output, &mut remaining, fact)?;
            }
            continue;
        }
        if !json_source {
            if let Some(fact) = gametracking_assertion(key, kind, leaf, &parts, &index)? {
                push_assertion(&mut output, &mut remaining, fact)?;
            }
            continue;
        }
        if parts.len() == 2 && parts[1] == "Name" {
            if !leaf.lexeme.starts_with('"') || leaf.text.len() > 256 {
                continue;
            }
            let mut fact = assertion(
                key,
                kind,
                leaf,
                true,
                "identity",
                "name",
                json!({"lang":"en"}),
            );
            fact.alias = Some((leaf.text.clone(), "en"));
            push_assertion(&mut output, &mut remaining, fact)?;
            continue;
        }
        if kind == "hero" && parts.len() == 4 && parts[1] == "BoundAbilities" && parts[3] == "Key" {
            let slot: i16 = parts[2].parse().context("ability_slot")?;
            ensure!(
                (1..=4).contains(&slot)
                    && leaf.lexeme.starts_with('"')
                    && !leaf.text.is_empty()
                    && valid_key(&leaf.text),
                "ability_link"
            );
            let mut fact = assertion(key, kind, leaf, true, "identity", "ability", json!({}));
            fact.relation = Some((leaf.text.clone(), slot));
            push_assertion(&mut output, &mut remaining, fact)?;
            continue;
        }
        if !leaf.numeric {
            continue;
        }
        ensure!(leaf.lexeme.len() <= 64, "numeric_size");
        if kind == "hero" {
            let (section, field, condition) = if parts.len() == 2
                && matches!(
                    parts[1],
                    "MaxHealth"
                        | "MaxMoveSpeed"
                        | "Stamina"
                        | "BaseHealthRegen"
                        | "SprintSpeed"
                        | "LightMeleeDamage"
                        | "HeavyMeleeDamage"
                        | "TechResist"
                        | "StaminaCooldown"
                        | "StaminaRegenPerSecond"
                        | "CritDamageBonusPercent"
                        | "CrouchSpeed"
                ) {
                ("stats", parts[1], json!({}))
            } else if parts.len() == 3 && parts[1] == "Weapon" {
                ("weapon", parts[2], json!({}))
            } else if parts.len() == 3 && matches!(parts[1], "LevelScaling" | "SpiritScaling") {
                ("stats", parts[2], json!({"scale":parts[1]}))
            } else {
                continue;
            };
            push_assertion(
                &mut output,
                &mut remaining,
                assertion(key, kind, leaf, true, section, field, condition),
            )?;
        } else {
            let (field, mut condition) = if parts.len() == 2 {
                (parts[1], json!({}))
            } else if parts.len() == 3 && parts[2] == "Value" {
                let prefix = format!("/{key}/{}", parts[1]);
                let mut condition = json!({});
                if let Some(scale) = index.get(format!("{prefix}/Scale/Value").as_str()) {
                    condition["scale_value"] = Value::String(scale.lexeme.clone());
                    condition["scale_value_locator"] = Value::String(scale.pointer.clone());
                }
                if let Some(scale) = index.get(format!("{prefix}/Scale/Type").as_str()) {
                    condition["scale_type"] = Value::String(scale.text.clone());
                    condition["scale_type_locator"] = Value::String(scale.pointer.clone());
                }
                (parts[1], condition)
            } else if parts.len() == 4 && parts[1] == "Upgrades" {
                let tier: u32 = parts[2].parse().context("upgrade_tier")?;
                ensure!(tier < 3, "upgrade_tier");
                (
                    parts[3],
                    json!({"upgrade_tier":tier + 1,"operation":"delta"}),
                )
            } else {
                continue;
            };
            if field.len() > 96 {
                continue;
            }
            if !condition.is_object() {
                condition = json!({});
            }
            push_assertion(
                &mut output,
                &mut remaining,
                assertion(key, kind, leaf, true, "ability", field, condition),
            )?;
        }
        ensure!(output.len() <= 30000, "assertion_limit");
    }
    for fact in &output {
        ensure!(
            fact.locator.len() <= 2048
                && fact.stat.len() <= 128
                && fact.condition.to_string().len() <= 1024,
            "assertion_size"
        );
        match &fact.value {
            Atom::Number(value) => ensure!(value.len() <= 64, "numeric_size"),
            Atom::Text(value) => ensure!(value.len() <= 512, "text_size"),
        }
    }
    Ok(output)
}

fn prove_scale(assertion: &Assertion, index: &BTreeMap<&str, &Leaf>) -> Result<()> {
    let Some(prefix) = assertion
        .condition
        .get("scale_locator")
        .and_then(Value::as_str)
    else {
        return Ok(());
    };
    let parent = assertion
        .locator
        .strip_suffix("/m_strValue/0")
        .context("proof_scale_scope")?;
    ensure!(
        prefix == format!("{parent}/m_subclassScaleFunction/0"),
        "proof_scale_scope"
    );
    for (field, label) in [
        ("_class", "scale_class"),
        ("m_eSpecificStatScaleType", "scale_type"),
        ("m_flStatScale", "scale_value"),
    ] {
        let expected = index
            .get(format!("{prefix}/{field}/0").as_str())
            .map(|leaf| leaf.text.as_str());
        ensure!(
            assertion.condition.get(label).and_then(Value::as_str) == expected,
            "proof_scale_value"
        );
    }
    for i in 0..4 {
        let expected = index
            .get(format!("{prefix}/m_vecScalingStats/0/[{i}]").as_str())
            .map(|leaf| leaf.text.as_str());
        ensure!(
            assertion
                .condition
                .get(format!("scale_stat_{i}"))
                .and_then(Value::as_str)
                == expected,
            "proof_scale_stat"
        );
    }
    Ok(())
}

pub fn prove(source: Source, path: &str, bytes: &[u8], assertions: &[Assertion]) -> Result<()> {
    ensure!(
        supported(source, path) || assertions.is_empty(),
        "proof_source"
    );
    if assertions.is_empty() {
        return Ok(());
    }
    let parsed = leaves(path, bytes)?;
    let index: BTreeMap<_, _> = parsed
        .iter()
        .map(|leaf| (leaf.pointer.as_str(), leaf))
        .collect();
    for assertion in assertions {
        let source = index
            .get(assertion.locator.as_str())
            .context("proof_locator_missing")?;
        ensure!(
            Sha256::digest(source.lexeme.as_bytes()).as_slice() == assertion.hash,
            "proof_value_hash"
        );
        ensure!(
            source.pointer.split('/').nth(1) == Some(assertion.entity.as_str()),
            "proof_entity"
        );
        match &assertion.value {
            Atom::Number(number) => {
                let actual = if source.numeric {
                    &source.lexeme
                } else {
                    &source.text
                };
                ensure!(
                    json_text::number(actual).is_ok() && actual == number,
                    "proof_numeric"
                );
            }
            Atom::Text(text) => {
                ensure!(
                    text == &source.text
                        || (assertion.stat == "identity.status"
                            && release_status(&source.text) == Some(text.as_str())),
                    "proof_text"
                );
            }
        }
        if let Some((text, _)) = &assertion.alias {
            ensure!(text == &source.text, "proof_alias");
        }
        if let Some((ability, slot)) = &assertion.relation {
            ensure!(
                source
                    .pointer
                    .split('/')
                    .nth(3)
                    .and_then(|v| v.parse::<i16>().ok())
                    == Some(*slot),
                "proof_relation_slot"
            );
            ensure!(
                &source.text == ability && source.pointer.contains("/BoundAbilities/"),
                "proof_relation"
            );
        }
        prove_scale(assertion, &index)?;
        let qualifiers = assertion.condition.as_object().context("proof_condition")?;
        for (name, value) in qualifiers {
            if name == "scale_locator" || !name.ends_with("_locator") {
                continue;
            }
            if let Some(locator) = value.as_str() {
                let qualifier = index.get(locator).context("proof_qualifier_missing")?;
                let expected = assertion
                    .condition
                    .get(name.trim_end_matches("_locator"))
                    .and_then(Value::as_str)
                    .context("proof_qualifier_value")?;
                ensure!(
                    expected == qualifier.text || expected == qualifier.lexeme,
                    "proof_qualifier_mismatch"
                );
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_lexemes_upgrade_conditions_and_links_are_proven() {
        let bytes = br#"{"hero_baba":{"Name":"Baba","MaxHealth":780.0,"BoundAbilities":{"1":{"Key":"ability_baba_bubbling_brew"}}}}"#;
        let facts = extract(Source::ALL[0], "data/json/hero-data.json", bytes).unwrap();
        assert!(facts
            .iter()
            .any(|fact| matches!(&fact.value, Atom::Number(value) if value == "780.0")));
        assert_eq!(
            facts.iter().filter(|fact| fact.relation.is_some()).count(),
            1
        );
        prove(Source::ALL[0], "data/json/hero-data.json", bytes, &facts).unwrap();
        assert!(prove(
            Source::ALL[0],
            "data/json/hero-data.json",
            br#"{"hero_baba":{"Name":"Baba","MaxHealth":781.0}}"#,
            &facts
        )
        .is_err());
        let bytes = br#"{"ability_baba_bubbling_brew":{"AbilityCooldown":{"Value":26.0,"Scale":{"Value":1,"Type":"cooldown"}},"Upgrades":[{"AbilityCooldown":-12}]}}"#;
        let facts = extract(Source::ALL[0], "data/json/ability-data.json", bytes).unwrap();
        assert_eq!(facts.len(), 2);
        assert_eq!(facts[0].condition["scale_type"], "cooldown");
        assert_eq!(facts[1].condition["upgrade_tier"], 1);
        prove(Source::ALL[0], "data/json/ability-data.json", bytes, &facts).unwrap();
    }

    #[test]
    fn document_identity_and_ability_type_do_not_depend_on_key_prefixes() {
        let key = "viscous_goo_grenade";
        let hero = br#"{"hero_viscous":{"Name":"Viscous","BoundAbilities":{"1":{"Key":"viscous_goo_grenade"}}}}"#;
        let facts = extract(Source::ALL[0], "data/json/hero-data.json", hero).unwrap();
        assert_eq!(facts[1].relation, Some((key.into(), 1)));
        prove(Source::ALL[0], "data/json/hero-data.json", hero, &facts).unwrap();
        let ability =
            br#"{"viscous_goo_grenade":{"Name":"Splatter","AbilityCooldown":{"Value":26.0}}}"#;
        let facts = extract(Source::ALL[0], "data/json/ability-data.json", ability).unwrap();
        assert_eq!(facts.len(), 2);
        assert!(facts
            .iter()
            .all(|fact| fact.entity == key && fact.kind == "ability"));
        prove(
            Source::ALL[0],
            "data/json/ability-data.json",
            ability,
            &facts,
        )
        .unwrap();
        for bytes in [
            br#"<!-- kv3 encoding:text --> { viscous_goo_grenade = { m_eAbilityType = "EAbilityType_Signature" m_mapAbilityProperties = { AbilityCooldown = { m_strValue = 26.0 } } } ability_misleading = { m_eAbilityType = "EAbilityType_Item" m_mapAbilityProperties = { AbilityCooldown = { m_strValue = 60.0 } } } }"#.as_slice(),
            br#"viscous_goo_grenade { m_eAbilityType "EAbilityType_Signature" m_mapAbilityProperties { AbilityCooldown { m_strValue "26.0" } } } ability_misleading { m_eAbilityType "EAbilityType_Item" m_mapAbilityProperties { AbilityCooldown { m_strValue "60.0" } } }"#.as_slice(),
        ] {
            let path = "game/citadel/pak01_dir/scripts/abilities.vdata";
            let facts = extract(Source::ALL[1], path, bytes).unwrap();
            assert_eq!(facts.len(), 1);
            assert_eq!(facts[0].entity, key);
            assert_eq!(facts[0].locator, "/viscous_goo_grenade/0/m_mapAbilityProperties/0/AbilityCooldown/0/m_strValue/0");
            assert!(matches!(&facts[0].value, Atom::Number(value) if value == "26.0"));
            prove(Source::ALL[1], path, bytes, &facts).unwrap();
        }
        let path = "data/localizations/german.json";
        let bytes = br#"{"viscous_goo_grenade":"Splatter","ability_misleading":"Item","hero_unknown":"Unknown"}"#;
        assert!(extract(Source::ALL[0], path, bytes).unwrap().is_empty());
        let identities = BTreeMap::from([(key.into(), "ability")]);
        let facts = extract_with_identities(Source::ALL[0], path, bytes, &identities).unwrap();
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].alias, Some(("Splatter".into(), "de")));
        prove(Source::ALL[0], path, bytes, &facts).unwrap();
    }

    #[test]
    fn verified_upgrade_property_spelling_keeps_its_original_locator() {
        let path = "game/citadel/pak01_dir/scripts/abilities.vdata";
        let bytes = br#"<!-- kv3 encoding:text --> { citadel_ability_infinity_slash = { m_eAbilityType = "EAbilityType_Signature" m_vecAbilityUpgrades = [ { m_vecPropertyUpgrades = [ { m_StrPropertyNAme = "AbilityCooldown" m_strBonus = "-20" } ] } ] } }"#;
        let facts = extract(Source::ALL[1], path, bytes).unwrap();
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].stat, "ability.AbilityCooldown");
        assert_eq!(facts[0].condition["upgrade_property_locator"], "/citadel_ability_infinity_slash/0/m_vecAbilityUpgrades/0/[0]/m_vecPropertyUpgrades/0/[0]/m_StrPropertyNAme/0");
        prove(Source::ALL[1], path, bytes, &facts).unwrap();
    }

    #[test]
    fn json_upgrade_tiers_keep_three_tier_conditions_and_original_value_hashes() {
        let path = "data/json/ability-data.json";
        for upgrades in [
            r#"[{"AbilityCooldown":-12.0},{"AbilityCooldown":1e2},{"AbilityCooldown":0.50}]"#,
            r#"{"0":{"AbilityCooldown":-12.0},"1":{"AbilityCooldown":1e2},"2":{"AbilityCooldown":0.50}}"#,
        ] {
            let bytes = format!(r#"{{"ability_test":{{"Upgrades":{upgrades}}}}}"#);
            let mut facts = extract(Source::ALL[0], path, bytes.as_bytes()).unwrap();
            assert_eq!(facts.len(), 3);
            for (index, (fact, lexeme)) in facts.iter().zip(["-12.0", "1e2", "0.50"]).enumerate() {
                assert_eq!(
                    fact.condition,
                    json!({"upgrade_tier":index+1,"operation":"delta"})
                );
                assert_eq!(
                    fact.locator,
                    format!("/ability_test/Upgrades/{index}/AbilityCooldown")
                );
                assert_eq!(fact.hash, Sha256::digest(lexeme.as_bytes()).to_vec());
                assert!(matches!(&fact.value, Atom::Number(value) if value == lexeme));
            }
            prove(Source::ALL[0], path, bytes.as_bytes(), &facts).unwrap();
            facts[2].hash[0] ^= 1;
            assert!(prove(Source::ALL[0], path, bytes.as_bytes(), &facts).is_err());
        }
        for tier in ["3", "4294967294", "4294967295", "4294967296", "-1"] {
            let bytes = format!(
                r#"{{"ability_test":{{"Upgrades":{{"{tier}":{{"AbilityCooldown":1}}}}}}}}"#
            );
            assert!(extract(Source::ALL[0], path, bytes.as_bytes()).is_err());
        }
        let fourth = br#"{"ability_test":{"Upgrades":[{},{},{},{"AbilityCooldown":1}]}}"#;
        assert!(extract(Source::ALL[0], path, fourth).is_err());
    }

    #[test]
    fn absent_identity_names_do_not_become_text_or_numeric_facts() {
        let path = "data/json/ability-data.json";
        let bytes = br#"{"unnamed":{"Name":null,"AbilityCooldown":26.0},"invalid":{"Name":7}}"#;
        let facts = extract(Source::ALL[0], path, bytes).unwrap();
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].stat, "ability.AbilityCooldown");
        prove(Source::ALL[0], path, bytes, &facts).unwrap();
        let path = "data/localizations/german.json";
        let bytes = br#"{"unnamed":null,"invalid":7}"#;
        let identities =
            BTreeMap::from([("unnamed".into(), "ability"), ("invalid".into(), "ability")]);
        assert!(
            extract_with_identities(Source::ALL[0], path, bytes, &identities)
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn release_requires_supported_occurrence_aware_source() {
        assert!(extract(
            Source::ALL[0],
            "data/json/hero-data.json",
            br#"{"hero_baba":{"InDevelopment":false}}"#
        )
        .unwrap()
        .is_empty());
        let bytes = br#"<!-- kv3 encoding:text --> { hero_baba = { m_eHeroDevelopmentState = "EHeroDevState_Release" } }"#;
        let facts = extract(
            Source::ALL[1],
            "game/citadel/pak01_dir/scripts/heroes.vdata",
            bytes,
        )
        .unwrap();
        assert_eq!(facts[0].locator, "/hero_baba/0/m_eHeroDevelopmentState/0");
        assert!(matches!(&facts[0].value, Atom::Text(status) if status == "released"));
        prove(
            Source::ALL[1],
            "game/citadel/pak01_dir/scripts/heroes.vdata",
            bytes,
            &facts,
        )
        .unwrap();
    }
}
