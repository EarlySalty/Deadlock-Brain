//! Adapter from authorized frozen models to the EXISTING inventory algorithm.
//! No network, DB writes, AI, inferred prices, or parallel stats/legality engine.
use crate::{
    inventory::{Inventory, InventoryRules},
    ItemModel,
};
use brain_contracts::{
    domain::*,
    value::{Observed, UnknownReason},
};
use dbrain_builds::spec::{CandidateItem, CandidateSet};
use dbrain_normalize::normalize_alias;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub const BUILD_EVALUATOR_VERSION: &str = "dbrain.inventory.v2";

pub fn resolve_hero<'a>(
    cards: &'a [DomainKnowledgeCard],
    text: &str,
    locale: &str,
) -> Option<&'a DomainKnowledgeCard> {
    if !matches!(locale, "de" | "en") {
        return None;
    }
    let key = normalize_alias(text);
    if key.is_empty() {
        return None;
    }
    let mut found = cards.iter().filter(|c| {
        c.card.hero_id == text
            || c.aliases
                .iter()
                .any(|a| a.locale == locale && normalize_alias(&a.text) == key)
    });
    let card = found.next()?;
    found.next().is_none().then_some(card)
}

pub fn source_value(snapshot: &DomainSnapshot, located: &LocatedRevision) -> Option<Value> {
    let source = snapshot.records.iter().find(|r| {
        r.source_id == located.source.source_id
            && r.logical_id == located.source.logical_id
            && r.revision == located.source.revision
            && r.content_hash == located.source.content_hash
    })?;
    if source.content.len() > 4 * 1024 * 1024 {
        return None;
    }
    let value: Value = serde_json::from_str(&source.content).ok()?;
    let pointer = located.locator.strip_prefix("json:")?;
    value.pointer(pointer).cloned()
}

fn unknown(reason: &str) -> BuildEvaluation {
    BuildEvaluation {
        legal: Observed::unknown(UnknownReason::NotPresent),
        reason: reason.into(),
        item_ids: Vec::new(),
        spent_souls: Observed::unknown(UnknownReason::NotPresent),
        consumed_ids: Vec::new(),
        evaluator_version: BUILD_EVALUATOR_VERSION.into(),
    }
}

pub fn evaluate_build(
    snapshot: &DomainSnapshot,
    catalog: &BuildCatalogRef,
    names: &[String],
    locale: &str,
) -> BuildEvaluation {
    if !catalog.unknown_legality_fields.is_empty() {
        return unknown(
            "Mindestens ein für die Legalität notwendiges Quellfeld ist ausdrücklich unbekannt.",
        );
    }
    if names.is_empty() || names.len() > 64 || !matches!(locale, "de" | "en") {
        return unknown(
            "Build benötigt eine begrenzte Kaufreihenfolge und die Sprache de oder en.",
        );
    }
    let Some(Value::Array(raw)) = source_value(snapshot, &catalog.models) else {
        return unknown("Eingefrorene Itemmodelle fehlen.");
    };
    if raw.is_empty() || raw.len() > 4096 {
        return unknown("Itemkatalog ist leer oder überschreitet das Limit.");
    }
    // Required legality fields may not be supplied by serde defaults. Null, missing,
    // and unknown values remain unknown even when an older model has defaults.
    for value in &raw {
        if ![
            "item_id",
            "name",
            "cost",
            "is_active",
            "shopable",
            "disabled",
            "class_name",
            "component_items",
        ]
        .iter()
        .all(|key| value.get(key).is_some_and(|v| !v.is_null()))
        {
            return unknown("Für die Legalität notwendige Itemfelder sind unbekannt.");
        }
    }
    let Ok(models) = serde_json::from_value::<Vec<ItemModel>>(Value::Array(raw)) else {
        return unknown("Itemmodelle sind nicht vollständig lesbar.");
    };
    if models
        .iter()
        .any(|m| m.shopable && !m.disabled && m.cost <= 0)
    {
        return unknown("Für ein kaufbares Item fehlt ein positiv belegter Preis; ein möglicher Normalisierungs-Nullwert wird nicht verwendet.");
    }
    let Some(raw_rules) = source_value(snapshot, &catalog.inventory_rules) else {
        return unknown("Inventarregeln fehlen.");
    };
    let Ok(rules) = serde_json::from_value::<InventoryRules>(raw_rules) else {
        return unknown("Inventarregeln sind unbekannt oder ungültig.");
    };
    if !(1..=64).contains(&rules.max_slots)
        || !rules.resale_fraction.is_finite()
        || !(0.0..=1.0).contains(&rules.resale_fraction)
    {
        return unknown("Inventarregeln sind nicht auswertbar.");
    }
    let Some(active_limit) = snapshot.facts.iter().find(|f| {
        f.fact_id == catalog.active_limit_fact_id && f.verified && f.validity == snapshot.validity
    }) else {
        return unknown("Verifizierte Regel für aktive Inventarplätze fehlt.");
    };
    let limit = active_limit.quantity.value;
    if active_limit.quantity.unit != "1"
        || !limit.is_finite()
        || !(0.0..=64.0).contains(&limit)
        || limit.fract() != 0.0
    {
        return unknown("Regel für aktive Inventarplätze hat keine gültige Einheit oder Ganzzahl.");
    }
    let mut by_id = BTreeMap::new();
    let mut by_class = BTreeMap::new();
    for model in &models {
        if model.item_id <= 0
            || model.name.trim().is_empty()
            || model.class_name.trim().is_empty()
            || by_id.insert(model.item_id, model).is_some()
            || by_class.insert(&model.class_name, model.item_id).is_some()
        {
            return unknown("Itemidentitäten im Snapshot sind ungültig oder mehrdeutig.");
        }
    }
    // Reuse the producer's component graph, but reject disagreement between the
    // frozen models and frozen rules instead of assuming missing components = none.
    for model in &models {
        let Some(expected) = model
            .component_items
            .iter()
            .map(|name| by_class.get(name).copied())
            .collect::<Option<BTreeSet<_>>>()
        else {
            return unknown("Eine Upgrade-Abhängigkeit fehlt im Snapshot.");
        };
        let components = rules
            .upgrade_components
            .get(&model.item_id)
            .cloned()
            .unwrap_or_default();
        if components.iter().copied().collect::<BTreeSet<_>>() != expected
            || components.len() != expected.len()
            || expected.contains(&model.item_id)
        {
            return unknown("Upgrade-Regeln widersprechen dem eingefrorenen Itemmodell.");
        }
    }
    if rules
        .upgrade_components
        .keys()
        .any(|id| !by_id.contains_key(id))
    {
        return unknown("Upgrade-Regel verweist auf ein unbekanntes Item.");
    }
    // CandidateSet is the existing dbrain-builds candidate contract, with an ADR
    // S05 collision-checked name map. No prefix/fuzzy fallback or silent drop.
    let mut candidates = CandidateSet {
        items: Vec::new(),
        name_to_id: BTreeMap::new(),
    };
    let mut collisions = BTreeSet::new();
    let mut insert = |name: &str, id: i64| {
        let key = normalize_alias(name);
        if candidates
            .name_to_id
            .insert(key.clone(), id)
            .is_some_and(|old| old != id)
        {
            collisions.insert(key);
        }
    };
    for model in &models {
        // Stable item IDs are locale-independent; names are explicitly EN.
        insert(&model.item_id.to_string(), model.item_id);
        if locale == "en" {
            insert(&model.name, model.item_id);
        }
        candidates.items.push(CandidateItem {
            id: model.item_id,
            name: model.name.clone(),
            slot: format!("{:?}", model.slot),
            tier: model.tier,
            signal: catalog.review_ref.clone(),
        });
    }
    for alias in catalog.aliases.iter().filter(|a| a.alias.locale == locale) {
        if !by_id.contains_key(&alias.item_id) {
            return unknown("Alias verweist auf ein unbekanntes Item.");
        }
        insert(&alias.alias.text, alias.item_id);
    }
    let keys: Vec<_> = names.iter().map(|name| normalize_alias(name)).collect();
    if keys
        .iter()
        .any(|key| key.is_empty() || collisions.contains(key))
    {
        return unknown("Itemalias ist leer oder mehrdeutig.");
    }
    let Ok(ids) = candidates.resolve_required_keys(&keys) else {
        return unknown("Mindestens ein Item ist für diese Sprache und diesen Snapshot unbekannt.");
    };
    let mut inventory = Inventory::default();
    let mut consumed = Vec::new();
    for id in &ids {
        let item = by_id[id];
        match inventory.preview_purchase_with_active_limit(
            item,
            &models,
            &rules,
            &[],
            limit as usize,
        ) {
            Ok(transition) => {
                consumed.extend(&transition.consumed_ids);
                if inventory.apply_transition(&transition).is_err() {
                    return unknown("Inventarzustand konnte nicht bestätigt werden.");
                }
            }
            Err(error) => {
                return BuildEvaluation {
                    legal: Observed::known(false),
                    reason: error.to_string(),
                    item_ids: ids,
                    spent_souls: Observed::unknown(UnknownReason::Unsupported),
                    consumed_ids: consumed,
                    evaluator_version: BUILD_EVALUATOR_VERSION.into(),
                }
            }
        }
    }
    BuildEvaluation {
        legal: Observed::known(true),
        reason: "Alle Käufe erfüllen Kaufbarkeit, Duplikat-, Upgrade-, Slot- und Aktivitemregeln."
            .into(),
        item_ids: ids,
        spent_souls: Observed::known(inventory.spent_souls),
        consumed_ids: consumed,
        evaluator_version: BUILD_EVALUATOR_VERSION.into(),
    }
}
