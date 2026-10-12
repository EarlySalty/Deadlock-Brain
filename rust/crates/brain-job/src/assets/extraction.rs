use crate::extract::{
    assets_parser::{self as parser, Leaf},
    Assertion, Atom,
};
use anyhow::{ensure, Context, Result};
use serde::Deserialize;
use serde_json::{json, value::RawValue, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Deserialize)]
struct Identity {
    id: u64,
    class_name: String,
    #[serde(rename = "type")]
    kind: String,
}

pub struct ItemEdge {
    pub proof: Assertion,
    pub from: String,
    pub to: String,
    pub kind: &'static str,
}

pub struct Extraction {
    pub facts: Vec<Assertion>,
    pub edges: Vec<ItemEdge>,
    pub unmapped: u64,
}

fn valid_key(key: &str) -> bool {
    !key.is_empty()
        && key.len() <= 96
        && key.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
}

fn kind(value: &str) -> Option<&'static str> {
    match value {
        "ability" => Some("ability"),
        "upgrade" => Some("item"),
        _ => None,
    }
}

fn records(bytes: &[u8]) -> Result<Vec<&RawValue>> {
    ensure!(bytes.len() <= super::MAX_BYTES, "assets_payload_limit");
    let records: Vec<&RawValue> = serde_json::from_slice(bytes).context("assets_array")?;
    ensure!(
        !records.is_empty() && records.len() <= 4096,
        "assets_record_count"
    );
    Ok(records)
}

fn text<'a>(index: &BTreeMap<&str, &'a Leaf>, pointer: &str) -> Option<&'a str> {
    index
        .get(pointer)
        .and_then(|leaf| leaf.value.as_str())
        .filter(|_| !index[pointer].numeric)
}

fn assertion(
    identity: &Identity,
    leaf: &Leaf,
    prefix: &str,
    stat: String,
    section: &'static str,
    condition: Value,
) -> Assertion {
    let numeric = if leaf.numeric {
        Some(leaf.lexeme.as_str())
    } else if section != "identity" {
        leaf.value.as_str().filter(|value| parser::number(value))
    } else {
        None
    };
    Assertion {
        entity: identity.class_name.clone(),
        kind: kind(&identity.kind).unwrap_or("ability"),
        locator_kind: "json_pointer",
        locator: format!("{prefix}{}", leaf.pointer),
        hash: Sha256::digest(leaf.lexeme.as_bytes()).to_vec(),
        value: match numeric {
            Some(value) => Atom::Number(value.into()),
            None => Atom::Text(leaf.value.as_str().unwrap_or("").into()),
        },
        condition,
        alias: None,
        relation: None,
        stat,
        section,
        unit: None,
        order: 10,
    }
}

fn push(output: &mut Extraction, fact: Assertion, remaining: &mut usize) -> Result<()> {
    let value = match &fact.value {
        Atom::Number(value) | Atom::Text(value) => value,
    };
    if value.len() > 512 {
        output.unmapped += 1;
        return Ok(());
    }
    let condition = fact.condition.to_string();
    ensure!(
        fact.locator.len() <= 2048 && fact.stat.len() <= 128 && condition.len() <= 1024,
        "assets_assertion_size"
    );
    let charge = 2048 + 4 * (value.len() + fact.locator.len() + condition.len());
    *remaining = remaining
        .checked_sub(charge)
        .context("assets_assertion_budget")?;
    ensure!(output.facts.len() < 30000, "assets_fact_limit");
    output.facts.push(fact);
    Ok(())
}

pub fn extract(bytes: &[u8], language: &'static str) -> Result<Extraction> {
    ensure!(matches!(language, "en" | "de"), "assets_language");
    let records = records(bytes)?;
    let mut ids = BTreeSet::new();
    let mut catalog = BTreeMap::new();
    for record in &records {
        let identity: Identity = serde_json::from_str(record.get()).context("assets_identity")?;
        ensure!(
            (kind(&identity.kind).is_none() || valid_key(&identity.class_name))
                && ids.insert(identity.id)
                && catalog.insert(identity.class_name, identity.kind).is_none(),
            "assets_duplicate_identity"
        );
    }
    let mut output = Extraction {
        facts: Vec::new(),
        edges: Vec::new(),
        unmapped: 0,
    };
    let mut remaining = 64 * 1024 * 1024usize;
    for (position, record) in records.iter().enumerate() {
        let identity: Identity = serde_json::from_str(record.get())?;
        let leaves = parser::parse(record.get())?;
        let Some(entity_kind) = kind(&identity.kind) else {
            output.unmapped += 1;
            continue;
        };
        let index: BTreeMap<_, _> = leaves
            .iter()
            .map(|leaf| (leaf.pointer.as_str(), leaf))
            .collect();
        let prefix = format!("/{position}");
        for leaf in leaves
            .iter()
            .filter(|leaf| leaf.pointer == "/class_name")
            .chain(leaves.iter().filter(|leaf| leaf.pointer != "/class_name"))
        {
            let parts: Vec<_> = leaf.pointer.split('/').skip(1).collect();
            let (stat, section, mut condition) = match parts.as_slice() {
                ["id"] if leaf.numeric => ("identity.game_id".into(), "identity", json!({})),
                ["class_name"] => ("identity.key".into(), "identity", json!({})),
                ["name"] if leaf.value.is_string() && !leaf.numeric => {
                    ("identity.name".into(), "identity", json!({"lang":language}))
                }
                ["image"] | ["image_webp"] if leaf.value.is_string() && !leaf.numeric => {
                    (format!("identity.{}", parts[0]), "identity", json!({}))
                }
                ["cost"] | ["item_tier"] if entity_kind == "item" && leaf.numeric => (
                    format!(
                        "item.{}",
                        if parts[0] == "item_tier" {
                            "tier"
                        } else {
                            "cost"
                        }
                    ),
                    "stats",
                    json!({}),
                ),
                ["item_slot_type"] | ["activation"]
                    if entity_kind == "item" && leaf.value.is_string() && !leaf.numeric =>
                {
                    (
                        format!(
                            "item.{}",
                            if parts[0] == "item_slot_type" {
                                "slot"
                            } else {
                                "activation"
                            }
                        ),
                        "stats",
                        json!({}),
                    )
                }
                ["description", field]
                    if leaf.value.is_string() && !leaf.numeric && valid_key(field) =>
                {
                    (
                        format!("{}.description_{field}", entity_kind),
                        "identity",
                        json!({"lang":language}),
                    )
                }
                ["properties", field, "value"] if valid_key(field) => {
                    if !(leaf.numeric || leaf.value.as_str().is_some_and(parser::number)) {
                        output.unmapped += 1;
                        continue;
                    }
                    let mut condition = json!({});
                    let scale = format!("/properties/{field}/scale_function");
                    for (source, target) in [
                        ("class_name", "scale_class"),
                        ("specific_stat_scale_type", "scale_type"),
                        ("stat_scale", "scale_value"),
                    ] {
                        if let Some(value) = index.get(format!("{scale}/{source}").as_str()) {
                            condition[target] = Value::String(if value.numeric {
                                value.lexeme.clone()
                            } else {
                                value.value.as_str().context("assets_scale_type")?.into()
                            });
                            condition[format!("{target}_locator")] =
                                Value::String(format!("{prefix}{}", value.pointer));
                        }
                    }
                    for i in 0..4 {
                        if let Some(value) =
                            index.get(format!("{scale}/scaling_stats/{i}").as_str())
                        {
                            condition[format!("scale_stat_{i}")] = Value::String(
                                value.value.as_str().context("assets_scale_stat")?.into(),
                            );
                            condition[format!("scale_stat_{i}_locator")] =
                                Value::String(format!("{prefix}{}", value.pointer));
                        }
                    }
                    ensure!(
                        !index.contains_key(format!("{scale}/scaling_stats/4").as_str()),
                        "assets_scale_limit"
                    );
                    (
                        format!("{}.{field}", entity_kind),
                        if entity_kind == "ability" {
                            "ability"
                        } else {
                            "stats"
                        },
                        condition,
                    )
                }
                ["upgrades", tier, "property_upgrades", _, "bonus"] if entity_kind == "ability" => {
                    let tier: u32 = tier.parse().context("assets_upgrade_tier")?;
                    ensure!(tier < 3, "assets_upgrade_tier");
                    let parent = leaf
                        .pointer
                        .strip_suffix("/bonus")
                        .context("assets_upgrade_pointer")?;
                    let name_pointer = format!("{parent}/name");
                    let name = text(&index, &name_pointer).context("assets_upgrade_name")?;
                    ensure!(valid_key(name), "assets_upgrade_name");
                    if !(leaf.numeric || leaf.value.as_str().is_some_and(parser::number)) {
                        output.unmapped += 1;
                        continue;
                    }
                    let operation = text(&index, &format!("{parent}/upgrade_type"));
                    let scale_filter = text(&index, &format!("{parent}/scale_stat_filter"));
                    if operation.is_some() || scale_filter.is_some() {
                        output.unmapped += 1;
                        continue;
                    }
                    let mut condition = json!({"upgrade_tier":tier+1,"operation":"delta"});
                    let scope = format!("/upgrades/{tier}/property_upgrades/");
                    let duplicates = index
                        .iter()
                        .filter(|(pointer, value)| {
                            pointer.starts_with(&scope)
                                && pointer.ends_with("/name")
                                && value.value.as_str() == Some(name)
                        })
                        .count();
                    if duplicates > 1 {
                        condition["upgrade_entry"] =
                            json!(parts[3].parse::<u32>().context("assets_upgrade_entry")?);
                    }
                    (format!("ability.{name}"), "ability", condition)
                }
                ["component_items", _] if entity_kind == "item" => {
                    let component = leaf.value.as_str().context("assets_component_type")?;
                    if catalog.get(component).and_then(|value| kind(value)) != Some("item") {
                        output.unmapped += 1;
                        continue;
                    }
                    ensure!(component != identity.class_name, "assets_component_cycle");
                    for edge_kind in ["component_of", "upgrades_to"] {
                        output.edges.push(ItemEdge {
                            proof: assertion(
                                &identity,
                                leaf,
                                &prefix,
                                edge_kind.into(),
                                "identity",
                                json!({}),
                            ),
                            from: component.into(),
                            to: identity.class_name.clone(),
                            kind: edge_kind,
                        });
                    }
                    ensure!(output.edges.len() <= 8192, "assets_edge_limit");
                    continue;
                }
                _ => continue,
            };
            if parts.first() == Some(&"properties") {
                let property = format!("/properties/{}", parts[1]);
                for i in 0..4 {
                    if let Some(flag) = text(&index, &format!("{property}/usage_flags/{i}")) {
                        condition[format!("usage_flag_{i}")] = Value::String(flag.into());
                        condition[format!("usage_flag_{i}_locator")] =
                            Value::String(format!("{prefix}{property}/usage_flags/{i}"));
                    }
                }
                ensure!(
                    !index.contains_key(format!("{property}/usage_flags/4").as_str()),
                    "assets_usage_flags_limit"
                );
            }
            let mut fact = assertion(&identity, leaf, &prefix, stat, section, condition);
            if matches!(parts.as_slice(), ["name"]) && !leaf.value.as_str().unwrap_or("").is_empty()
            {
                fact.alias = Some((leaf.value.as_str().context("assets_name")?.into(), language));
            }
            if matches!(parts.as_slice(), ["class_name"]) {
                fact.alias = Some((identity.class_name.clone(), "und"));
            }
            if matches!(parts.as_slice(), ["cost"]) {
                fact.unit = Some("souls");
                fact.order = 0;
            }
            if matches!(parts.as_slice(), ["item_tier"]) {
                fact.order = 1;
            }
            if fact.stat == "ability.AbilityCooldown"
                || fact.stat == "ability.AbilityDuration"
                || fact.stat == "ability.DebuffDuration"
                || fact.stat == "ability.ReloadTime"
            {
                fact.unit = Some("s");
                fact.order = 3;
            }
            push(&mut output, fact, &mut remaining)?;
        }
    }
    Ok(output)
}

pub fn prove(bytes: &[u8], extraction: &Extraction) -> Result<()> {
    let records = records(bytes)?;
    let mut verified = 0;
    for (position, record) in records.iter().enumerate() {
        let identity: Identity = serde_json::from_str(record.get())?;
        let prefix = format!("/{position}/");
        let leaves = parser::parse(record.get())?;
        let index: BTreeMap<_, _> = leaves
            .iter()
            .map(|leaf| (format!("/{position}{}", leaf.pointer), leaf))
            .collect();
        for fact in extraction
            .facts
            .iter()
            .chain(extraction.edges.iter().map(|edge| &edge.proof))
            .filter(|fact| fact.locator.starts_with(&prefix))
        {
            ensure!(
                fact.entity == identity.class_name
                    && Some(fact.kind) == kind(&identity.kind)
                    && fact.locator_kind == "json_pointer",
                "assets_proof_identity"
            );
            let leaf = index.get(&fact.locator).context("assets_proof_locator")?;
            ensure!(
                fact.hash == Sha256::digest(leaf.lexeme.as_bytes()).as_slice(),
                "assets_proof_hash"
            );
            match &fact.value {
                Atom::Number(value) => ensure!(
                    parser::number(value)
                        && value
                            == if leaf.numeric {
                                &leaf.lexeme
                            } else {
                                leaf.value.as_str().context("assets_proof_number")?
                            },
                    "assets_proof_number"
                ),
                Atom::Text(value) => ensure!(
                    Some(value.as_str()) == leaf.value.as_str() && !leaf.numeric,
                    "assets_proof_text"
                ),
            }
            if let Some((alias, _)) = &fact.alias {
                ensure!(
                    Some(alias.as_str()) == leaf.value.as_str(),
                    "assets_proof_alias"
                );
            }
            for (key, value) in fact
                .condition
                .as_object()
                .context("assets_proof_condition")?
            {
                if let Some(label) = key.strip_suffix("_locator") {
                    let qualifier = index
                        .get(value.as_str().context("assets_proof_qualifier")?)
                        .context("assets_proof_qualifier")?;
                    ensure!(
                        fact.condition[label].as_str()
                            == if qualifier.numeric {
                                Some(qualifier.lexeme.as_str())
                            } else {
                                qualifier.value.as_str()
                            },
                        "assets_proof_qualifier"
                    );
                }
            }
            if let Some(tier) = fact.condition.get("upgrade_tier") {
                let parts: Vec<_> = leaf.pointer.split('/').skip(1).collect();
                ensure!(
                    parts.len() == 5
                        && parts[0] == "upgrades"
                        && parts[1].parse::<u64>().ok().map(|value| value + 1) == tier.as_u64()
                        && fact.condition["operation"] == "delta",
                    "assets_proof_upgrade"
                );
                if let Some(entry) = fact.condition.get("upgrade_entry") {
                    ensure!(
                        parts[3].parse::<u64>().ok() == entry.as_u64(),
                        "assets_proof_upgrade_entry"
                    );
                }
                let parent = leaf
                    .pointer
                    .strip_suffix("/bonus")
                    .context("assets_proof_upgrade")?;
                let property = index
                    .get(&format!("/{position}{parent}/name"))
                    .context("assets_proof_upgrade")?;
                ensure!(
                    fact.stat
                        == format!(
                            "ability.{}",
                            property.value.as_str().context("assets_proof_upgrade")?
                        ),
                    "assets_proof_upgrade"
                );
                ensure!(
                    !index.contains_key(&format!("/{position}{parent}/upgrade_type"))
                        && !index.contains_key(&format!("/{position}{parent}/scale_stat_filter")),
                    "assets_proof_upgrade_operation"
                );
            }
            verified += 1;
        }
        for edge in extraction
            .edges
            .iter()
            .filter(|edge| edge.proof.locator.starts_with(&prefix))
        {
            let leaf = index
                .get(&edge.proof.locator)
                .context("assets_proof_edge")?;
            ensure!(
                edge.proof
                    .locator
                    .starts_with(&format!("/{position}/component_items/"))
                    && Some(edge.from.as_str()) == leaf.value.as_str()
                    && edge.to == identity.class_name
                    && matches!(edge.kind, "component_of" | "upgrades_to"),
                "assets_proof_edge"
            );
        }
    }
    ensure!(
        verified == extraction.facts.len() + extraction.edges.len(),
        "assets_proof_record"
    );
    Ok(())
}
