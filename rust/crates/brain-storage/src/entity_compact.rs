use brain_contracts::entity_profile::{EntityProfile, EntityProfileFact};
use std::collections::BTreeMap;

pub fn compact_document(profile: &EntityProfile) -> crate::Result<String> {
    let withheld: Vec<_> = profile
        .facts
        .iter()
        .chain(&profile.context)
        .filter(|fact| {
            !super::derivation::public_statement_qualifiers(&fact.qualifiers)
                || fact.unit.as_ref().is_some_and(|unit| {
                    super::derivation::public_qualifier_text(&serde_json::Value::String(
                        unit.clone(),
                    ))
                    .is_none()
                })
        })
        .map(super::fact_reference)
        .collect();
    let mut unknowns = profile.unknowns.clone();
    if !withheld.is_empty() {
        unknowns.push("Eine Aussage wird zurückgehalten: Eine belegte öffentliche Beschreibung der nötigen Bedingung oder Variante fehlt".into());
    }
    let mut source_refs = BTreeMap::new();
    let mut sources = Vec::new();
    let mut compact_fact = |fact: &EntityProfileFact| {
        let provenance = &fact.provenance;
        let identity = &provenance.origin.identity;
        let key = (
            identity.source_id.clone(),
            provenance.original_revision.clone(),
        );
        let source_ref = *source_refs.entry(key).or_insert_with(|| {
            sources.push(serde_json::json!({
                "source_id": identity.source_id,
                "original_revision": provenance.original_revision,
            }));
            sources.len() - 1
        });
        serde_json::json!({
            "fact_id": fact.fact_id,
            "subject": fact.subject,
            "predicate": fact.predicate,
            "value": fact.value,
            "unit": fact.unit,
            "qualifiers": fact.qualifiers,
            "evidence_status": fact.evidence_status,
            "validity": fact.validity,
            "source_ref": source_ref,
            "source_kind": provenance.source_kind,
            "logical_id": identity.logical_id,
            "locator": provenance.origin.locator,
            "source_span": provenance.source_span,
            "observed_at": provenance.observed_at,
            "policy": provenance.origin.policy,
            "license": provenance.license,
        })
    };
    let facts: Vec<_> = profile
        .facts
        .iter()
        .filter(|fact| !withheld.contains(&super::fact_reference(fact)))
        .map(&mut compact_fact)
        .collect();
    let context: Vec<_> = profile
        .context
        .iter()
        .filter(|fact| !withheld.contains(&super::fact_reference(fact)))
        .map(&mut compact_fact)
        .collect();
    let conflicts: Vec<_> = profile
        .conflicts
        .iter()
        .filter(|conflict| !conflict.fact_ids.iter().any(|id| withheld.contains(id)))
        .collect();
    let story: Vec<_> = profile
        .patch_story
        .iter()
        .map(|change| {
            let change = super::derivation::consumer_patch_statement(change)?;
            Ok(serde_json::json!({
                "patch_date": change.patch_date,
                "patch_title": change.patch_title,
                "entity_type": change.entity_type,
                "entity_name": change.entity_name,
                "ability_name": change.ability_name,
                "stat_name": change.stat_name,
                "old_value": change.old_value,
                "new_value": change.new_value,
                "change_type": change.change_type,
                "numeric_direction": change.numeric_direction,
                "confidence": change.confidence,
                "conditions": change.additional_fields.iter().filter(|(key,_)| matches!(key.as_str(),"unit" | "level" | "variant" | "condition" | "ability_name")).collect::<BTreeMap<_,_>>(),
                "provenance": change.provenance,
            }))
        })
        .collect::<crate::Result<_>>()?;
    Ok(serde_json::to_string(&serde_json::json!({
        "contract_version": profile.contract_version,
        "entity": profile.entity,
        "patch": profile.patch,
        "source_state": profile.source_state,
        "facts": facts,
        "context": context,
        "conflicts": conflicts,
        "patch_story": story,
        "unknowns": unknowns,
        "sources": sources,
    }))?)
}
