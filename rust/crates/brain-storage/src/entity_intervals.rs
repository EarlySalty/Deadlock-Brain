use super::{
    invalid,
    semantic::{project_semantic_fact, SemanticProjection},
};
use crate::{PgStore, Result};
use brain_contracts::entity_profile::{
    EntityIdentity, EntityKind, EntityProfileFact, PatchStoryChange, PatchValidity,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

impl PgStore {
    pub async fn store_entity_patch_intervals(
        &self,
        entity_key: &str,
        record: &brain_contracts::SourceRecordV2,
        anchor_fact_id: &str,
        current_patch_fact_id: &str,
    ) -> Result<usize> {
        let original = self
            .stored_entity_source(&record.source_id, &record.logical_id, record.revision)
            .await?;
        if serde_json::to_value(&original)? != serde_json::to_value(record)? {
            return Err(invalid(
                "Intervallanker widerspricht der gespeicherten Originalrevision",
            ));
        }
        let revision =
            i64::try_from(record.revision).map_err(|_| invalid("Revision ist zu groß"))?;
        let (identity,relative_pointer,predicate,qualifiers,unit):(Value,String,String,String,Option<String>)=sqlx::query_as("SELECT f.binding_identity_json,s.relative_pointer,s.semantic_predicate,s.semantic_qualifiers_json,s.semantic_unit FROM brain.entity_profile_facts_v1 f JOIN brain.entity_semantic_projections_v1 s USING(entity_key,source_id,logical_id,revision,fact_id) WHERE entity_key=$1 AND source_id=$2 AND logical_id=$3 AND revision=$4 AND fact_id=$5")
            .bind(entity_key).bind(&record.source_id).bind(&record.logical_id).bind(revision).bind(anchor_fact_id).fetch_one(&self.pool).await?;
        let entity: EntityIdentity = serde_json::from_value(identity)?;
        let semantic = SemanticProjection {
            relative_pointer,
            predicate,
            qualifiers: serde_json::from_str(&qualifiers)?,
            unit,
        };
        let facts = super::project_entity_facts(
            &original,
            &[anchor_fact_id.into(), current_patch_fact_id.into()],
        )?;
        let story = self.entity_patch_story(&entity).await?;
        let intervals = derive_patch_intervals(&entity, &facts[0], &facts[1], &semantic, &story)?;
        let encoded = serde_json::to_string(&intervals)?;
        let count=sqlx::query("INSERT INTO brain.entity_patch_intervals_v1(entity_key,source_id,logical_id,revision,fact_id,current_patch_fact_id,interval_json) VALUES($1,$2,$3,$4,$5,$6,$7) ON CONFLICT(entity_key,source_id,logical_id,revision,fact_id) DO UPDATE SET current_patch_fact_id=EXCLUDED.current_patch_fact_id,interval_json=EXCLUDED.interval_json WHERE brain.entity_patch_intervals_v1.interval_json<>EXCLUDED.interval_json")
            .bind(entity_key).bind(&record.source_id).bind(&record.logical_id).bind(revision).bind(anchor_fact_id).bind(current_patch_fact_id).bind(encoded).execute(&self.pool).await?.rows_affected() as usize;
        Ok(count)
    }
}

pub fn project_interval_fact(
    entity: &EntityIdentity,
    original: &EntityProfileFact,
    current_patch_fact: &EntityProfileFact,
    semantic: &SemanticProjection,
    stored: &IntervalProjection,
    story: &[PatchStoryChange],
    patch: &str,
) -> Result<Option<EntityProfileFact>> {
    let expected = derive_patch_intervals(entity, original, current_patch_fact, semantic, story)?;
    if expected != *stored {
        return Err(invalid(
            "Gespeicherte Intervalle widersprechen den aktuellen Patchbelegen",
        ));
    }
    let Some(interval) = stored
        .intervals
        .iter()
        .find(|interval| super::validity_contains(&interval.validity, patch))
    else {
        return Ok(None);
    };
    let mut fact = project_semantic_fact(original, semantic)?;
    fact.value = interval.value.clone();
    fact.validity = interval.validity.clone();
    fact.provenance.document_metadata.insert(
        "derived_interval_evidence".into(),
        serde_json::to_value(interval)?,
    );
    fact.provenance.document_metadata.insert(
        "current_patch_fact_id".into(),
        Value::String(stored.current_patch_fact_id.clone()),
    );
    Ok(Some(fact))
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DerivedInterval {
    pub value: Value,
    pub validity: PatchValidity,
    pub patch_evidence: Vec<PatchStoryChange>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntervalProjection {
    pub anchor_fact_id: String,
    pub current_patch_fact_id: String,
    pub current_patch: String,
    pub intervals: Vec<DerivedInterval>,
}

fn numeric(value: &Value) -> Option<(bool, String, i64)> {
    let text = match value {
        Value::Number(number) => number.to_string(),
        Value::String(text) => text.clone(),
        _ => return None,
    };
    serde_json::from_str::<serde_json::Number>(&text).ok()?;
    let negative = text.starts_with('-');
    let text = text.strip_prefix('-').unwrap_or(&text);
    let (mantissa, exponent) = match text.split_once(['e', 'E']) {
        Some((mantissa, exponent)) => (mantissa, exponent.parse::<i64>().ok()?),
        None => (text, 0),
    };
    let decimals = mantissa
        .split_once('.')
        .map_or(0, |(_, tail)| tail.len() as i64);
    let mut exponent = exponent.checked_sub(decimals)?;
    let mut digits = mantissa.replace('.', "").trim_start_matches('0').to_owned();
    if digits.is_empty() {
        return Some((false, "0".into(), 0));
    }
    while digits.ends_with('0') {
        digits.pop();
        exponent = exponent.checked_add(1)?;
    }
    Some((negative, digits, exponent))
}

fn same_number(left: &Value, right: &Value) -> bool {
    numeric(left).is_some_and(|left| Some(left) == numeric(right))
}

fn date(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 10
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || !bytes
            .iter()
            .enumerate()
            .all(|(i, b)| i == 4 || i == 7 || b.is_ascii_digit())
    {
        return false;
    }
    let year = value[..4].parse::<u32>().unwrap_or(0);
    let month = value[5..7].parse::<u32>().unwrap_or(0);
    let day = value[8..].parse::<u32>().unwrap_or(0);
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                29
            } else {
                28
            }
        }
        _ => 0,
    };
    year > 0 && day > 0 && day <= days
}

fn matching_entity(entity: &EntityIdentity, change: &PatchStoryChange) -> bool {
    let matches = |name: Option<&str>| {
        name.is_some_and(|name| {
            std::iter::once(&entity.name)
                .chain(&entity.aliases)
                .any(|candidate| candidate.eq_ignore_ascii_case(name))
        })
    };
    match entity.kind {
        EntityKind::Hero => {
            change.entity_type.as_deref() == Some("hero")
                && change.ability_name.is_none()
                && matches(change.entity_name.as_deref())
        }
        EntityKind::Item => {
            change.entity_type.as_deref() == Some("item") && matches(change.entity_name.as_deref())
        }
        EntityKind::Ability => {
            (change.entity_type.as_deref() == Some("ability")
                && matches(change.entity_name.as_deref()))
                || (change.entity_type.as_deref() == Some("hero")
                    && matches(change.ability_name.as_deref()))
        }
    }
}

pub fn derive_patch_intervals(
    entity: &EntityIdentity,
    anchor: &EntityProfileFact,
    current_patch_fact: &EntityProfileFact,
    semantic: &SemanticProjection,
    story: &[PatchStoryChange],
) -> Result<IntervalProjection> {
    let anchor = project_semantic_fact(anchor, semantic)?;
    let current_patch = current_patch_fact
        .value
        .as_str()
        .filter(|date_text| date(date_text))
        .ok_or_else(|| invalid("Konkreter aktueller Patchanker fehlt"))?;
    if current_patch_fact.predicate != "current_patch"
        || current_patch_fact.provenance.origin != anchor.provenance.origin
        || current_patch_fact.provenance.original_revision != anchor.provenance.original_revision
    {
        return Err(invalid(
            "Aktueller Patchanker ist nicht durch dieselbe Originalrevision belegt",
        ));
    }
    let mut changes: Vec<_> = story
        .iter()
        .filter(|change| {
            matching_entity(entity, change)
                && change
                    .stat_name
                    .as_deref()
                    .is_some_and(|stat| super::semantic::stat_key(stat) == semantic.predicate)
        })
        .cloned()
        .collect();
    if changes.is_empty() {
        return Err(invalid("Belegte Patchkette fehlt"));
    }
    changes.sort_by(|left, right| left.patch_date.cmp(&right.patch_date));
    for change in &changes {
        if !date(&change.patch_date)
            || change.patch_date.as_str() > current_patch
            || change.provenance.relation != "brain.patch_changes"
            || change.provenance.evidence_ref.is_empty()
            || change.additional_fields.get("unit").and_then(Value::as_str)
                != semantic.unit.as_deref()
        {
            return Err(invalid(
                "Patchdatum, Einheit oder Patchbeleg passt nicht zum Anker",
            ));
        }
        for (key, value) in &semantic.qualifiers {
            if !matches!(
                key.as_str(),
                "numeric_representation"
                    | "references_resolved"
                    | "type_flags"
                    | "unit_status"
                    | "unit_inferred"
            ) && change.additional_fields.get(key) != Some(value)
            {
                return Err(invalid(
                    "Patchvariante oder Bedingung passt nicht zum Originalfakt",
                ));
            }
        }
    }
    if changes
        .windows(2)
        .any(|pair| pair[0].patch_date == pair[1].patch_date)
    {
        return Err(invalid("Mehrdeutige Patchkette am selben Datum"));
    }
    let mut expected = anchor.value.clone();
    for change in changes.iter().rev() {
        if !same_number(&expected, &change.new_value) || numeric(&change.old_value).is_none() {
            return Err(invalid(
                "Patchkette widerspricht dem aktuellen Originalwert",
            ));
        }
        expected = change.old_value.clone();
    }
    let mut intervals = Vec::new();
    for (index, change) in changes.iter().enumerate() {
        let value = if index + 1 == changes.len() {
            anchor.value.clone()
        } else {
            change.new_value.clone()
        };
        intervals.push(DerivedInterval {
            value,
            validity: PatchValidity::Known {
                from_patch: change.patch_date.clone(),
                to_patch_exclusive: changes.get(index + 1).map(|next| next.patch_date.clone()),
                evidence_ref: change.provenance.evidence_ref.clone(),
            },
            patch_evidence: changes[index..].to_vec(),
        });
    }
    Ok(IntervalProjection {
        anchor_fact_id: anchor.fact_id,
        current_patch_fact_id: current_patch_fact.fact_id.clone(),
        current_patch: current_patch.into(),
        intervals,
    })
}
