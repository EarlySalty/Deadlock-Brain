//! Validation for v2 domain projections. This module grants no source rights.
use brain_contracts::{domain::*, wiki::IrValue, DocumentRevision, PortError, SourceRecordV2};
use std::collections::BTreeSet;

fn invalid(message: &str) -> PortError {
    PortError::InvalidResponse(message.into())
}
fn stable(value: &str) -> bool {
    !value.trim().is_empty() && value.len() <= 512 && !value.chars().any(char::is_control)
}
pub(super) fn reference(record: &SourceRecordV2) -> DocumentRevision {
    DocumentRevision {
        source_id: record.source_id.clone(),
        logical_id: record.logical_id.clone(),
        revision: record.revision,
        content_hash: record.content_hash.clone(),
    }
}
pub(super) fn validity_matches(record: &SourceRecordV2, validity: &Validity) -> bool {
    use brain_contracts::{
        source::{origin_from_record, ORIGIN_METADATA_KEY},
        value::Observed,
    };
    if record.metadata.contains_key(ORIGIN_METADATA_KEY) {
        let Ok(origin) = origin_from_record(record) else {
            return false;
        };
        // An explicit reviewed projection can establish previously unknown game
        // validity, but cannot override a contradictory known source patch/mode.
        if matches!(origin.validity.patch, Observed::Known { value } if value != validity.patch)
            || matches!(origin.validity.mode, Observed::Known { value } if value != validity.mode)
        {
            return false;
        }
        // C4 origin is authoritative. Legacy metadata can contain the JSON
        // spelling "null" for unknown; it is not a literal game-patch pin.
        return true;
    }
    !record
        .metadata
        .get("patch")
        .is_some_and(|p| p != &validity.patch)
        && !record
            .metadata
            .get("mode")
            .is_some_and(|m| m != &validity.mode)
}
pub(super) fn located_valid(value: &LocatedRevision) -> Result<(), PortError> {
    if !stable(&value.locator) || !stable(&value.parser_revision) {
        return Err(invalid("domain locator or parser revision missing"));
    }
    Ok(())
}
/// All raw, alias, effect and synergy sources participate in authorization.
pub(super) fn card_sources(
    card: &DomainKnowledgeCard,
    visible: &[SourceRecordV2],
) -> Result<Option<Vec<DocumentRevision>>, PortError> {
    if !stable(&card.card.hero_id)
        || !stable(&card.review_ref)
        || card.dependencies.len() > 256
        || card.aliases.len() > 256
        || card.fields.len() > 4096
        || card.card.facts.len() > 4096
        || card.effects.len() > 256
        || card.card.rules.len() > 256
        || card.card.synergies.len() > 256
    {
        return Err(invalid("invalid or oversized domain card"));
    }
    let mut refs = Vec::new();
    for located in std::iter::once(&card.source)
        .chain(&card.dependencies)
        .chain(card.aliases.iter().map(|a| &a.provenance))
    {
        located_valid(located)?;
        refs.push(located.source.clone());
    }
    if card
        .aliases
        .iter()
        .any(|a| !stable(&a.text) || !matches!(a.locale.as_str(), "de" | "en"))
    {
        return Err(invalid("invalid localized alias"));
    }
    let mut predicates = BTreeSet::new();
    for fact in &card.card.facts {
        if !stable(&fact.fact_id)
            || !stable(&fact.subject_id)
            || !stable(&fact.key)
            || fact.value.is_null()
            || !predicates.insert((&fact.subject_id, &fact.key))
        {
            return Err(invalid("ambiguous or unknown card fact"));
        }
        // Lossless Wiki IR cannot be flattened into an unconditional or different fact.
        let matching: Vec<_> = card
            .fields
            .iter()
            .filter(|f| {
                f.subject_id == fact.subject_id
                    && f.predicate == fact.key
                    && f.is_unconditional_known()
            })
            .collect();
        if !card.fields.is_empty() && (matching.len() != 1 || !matches_fact(matching[0], fact)) {
            return Err(invalid("card fact disagrees with lossless IR"));
        }
        refs.push(fact.source_revision.clone());
    }
    refs.extend(card.fields.iter().map(|f| f.source_revision.clone()));
    refs.extend(card.card.rules.iter().map(|r| r.source_revision.clone()));
    refs.extend(card.effects.iter().map(|e| e.source_revision.clone()));
    for synergy in &card.card.synergies {
        if !stable(&synergy.synergy_id)
            || synergy.evidence.is_empty()
            || synergy.evidence.len() > 100
        {
            return Err(invalid("synergy needs bounded canonical evidence"));
        }
        for evidence in &synergy.evidence {
            let Some(record) = visible
                .iter()
                .find(|r| r.source_id == evidence.source_id && r.logical_id == evidence.logical_id)
            else {
                return Ok(None);
            };
            if record.revision != evidence.revision || record.content != evidence.content {
                return Err(invalid("synergy evidence does not match canonical source"));
            }
            refs.push(reference(record));
        }
    }
    Ok(Some(refs))
}
fn matches_fact(field: &brain_contracts::wiki::IrField, fact: &brain_contracts::Fact) -> bool {
    if field.source_revision != fact.source_revision
        || !field.unknowns.is_empty()
        || field.condition.is_some()
        || field.variant.is_some()
    {
        return false;
    }
    match &field.value {
        IrValue::Quantity { decimal, unit } => {
            fact.value.as_str() == Some(decimal) && fact.unit.as_deref() == Some(unit.as_str())
        }
        IrValue::Boolean { value } => fact.value.as_bool() == Some(*value) && fact.unit.is_none(),
        IrValue::Text { value } => fact.value.as_str() == Some(value) && fact.unit.is_none(),
        IrValue::Unknown { .. } => false,
    }
}
pub(super) fn catalog_sources(
    catalog: &BuildCatalogRef,
) -> Result<Vec<DocumentRevision>, PortError> {
    if !stable(&catalog.catalog_id)
        || !stable(&catalog.review_ref)
        || !stable(&catalog.active_limit_fact_id)
        || catalog.aliases.len() > 4096
        || catalog.dependencies.len() > 256
        || catalog.unknown_legality_fields.len() > 256
    {
        return Err(invalid("invalid build catalog reference"));
    }
    let mut refs = Vec::new();
    for located in [&catalog.models, &catalog.inventory_rules]
        .into_iter()
        .chain(&catalog.dependencies)
        .chain(catalog.aliases.iter().map(|a| &a.alias.provenance))
    {
        located_valid(located)?;
        refs.push(located.source.clone());
    }
    if catalog.aliases.iter().any(|a| {
        a.item_id <= 0 || !stable(&a.alias.text) || !matches!(a.alias.locale.as_str(), "de" | "en")
    }) {
        return Err(invalid("invalid item alias"));
    }
    Ok(refs)
}
