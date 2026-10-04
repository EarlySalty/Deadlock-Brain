use super::{invalid, project_entity_facts};
use crate::{PgStore, Result};
use brain_contracts::{entity_profile::EntityProfileFact, SourceRecordV2};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SemanticProjection {
    pub relative_pointer: String,
    pub predicate: String,
    pub qualifiers: Map<String, Value>,
    pub unit: Option<String>,
}

pub(crate) fn stat_key(value: &str) -> String {
    let mut output = String::new();
    let mut lower = false;
    for character in value.chars() {
        if character.is_ascii_alphanumeric() {
            if character.is_ascii_uppercase() && lower && !output.ends_with('_') {
                output.push('_');
            }
            output.push(character.to_ascii_lowercase());
            lower = character.is_ascii_lowercase() || character.is_ascii_digit();
        } else {
            if !output.ends_with('_') {
                output.push('_');
            }
            lower = false;
        }
    }
    output.trim_matches('_').into()
}

pub fn semantic_projection(
    fact: &EntityProfileFact,
    relative_pointer: &str,
) -> Result<Option<SemanticProjection>> {
    let numeric = fact.value.is_number()
        || (fact
            .qualifiers
            .get("numeric_representation")
            .and_then(Value::as_str)
            == Some("source_numeric_lexeme")
            && fact
                .value
                .as_str()
                .is_some_and(|text| serde_json::from_str::<serde_json::Number>(text).is_ok()));
    if !numeric {
        return Ok(None);
    }
    let pointer = fact
        .qualifiers
        .get("source_pointer")
        .or_else(|| fact.qualifiers.get("json_pointer"))
        .and_then(Value::as_str);
    if relative_pointer.is_empty()
        || !relative_pointer.starts_with('/')
        || !pointer.is_some_and(|path| path.ends_with(relative_pointer))
    {
        return Err(invalid(
            "Semantischer Blattpfad widerspricht dem Originalbeleg",
        ));
    }
    let parts: Vec<_> = relative_pointer
        .split('/')
        .skip(1)
        .map(|part| part.replace("~1", "/").replace("~0", "~"))
        .collect();
    let Some(last) = parts.last() else {
        return Ok(None);
    };
    let value_wrapper = last == "Value" && parts.len() > 1;
    let field = if value_wrapper {
        &parts[parts.len() - 2]
    } else {
        last
    };
    if field.is_empty() || field.bytes().all(|byte| byte.is_ascii_digit()) {
        return Ok(None);
    }
    let predicate = stat_key(field);
    if predicate.is_empty() {
        return Ok(None);
    }
    let mut qualifiers = fact.qualifiers.clone();
    for key in [
        "source_pointer",
        "json_pointer",
        "source_lexeme",
        "gameplay_binding",
    ] {
        qualifiers.remove(key);
    }
    let scope_end = parts.len() - if value_wrapper { 2 } else { 1 };
    if scope_end > 0 {
        qualifiers.insert(
            "semantic_scope".into(),
            Value::String(parts[..scope_end].join("/")),
        );
    }
    Ok(Some(SemanticProjection {
        relative_pointer: relative_pointer.into(),
        predicate,
        qualifiers,
        unit: fact.unit.clone(),
    }))
}

pub fn project_semantic_fact(
    original: &EntityProfileFact,
    projection: &SemanticProjection,
) -> Result<EntityProfileFact> {
    if semantic_projection(original, &projection.relative_pointer)?.as_ref() != Some(projection) {
        return Err(invalid(
            "Semantische Projektion widerspricht dem Originalfakt",
        ));
    }
    let mut projected = original.clone();
    projected.predicate = projection.predicate.clone();
    projected.qualifiers = projection.qualifiers.clone();
    projected.unit = projection.unit.clone();
    Ok(projected)
}

impl PgStore {
    pub async fn stored_git_entity_bindings(
        &self,
        entity_key: &str,
        record: &SourceRecordV2,
    ) -> Result<Vec<super::derivation::StoredGitBinding>> {
        let document: Value = serde_json::from_str(
            record
                .metadata
                .get(crate::source_versions::DOCUMENT_METADATA_KEY)
                .ok_or_else(|| invalid("Originaldokument fehlt"))?,
        )?;
        if document["source_kind"] != "game_file" {
            return Ok(Vec::new());
        }
        let revision =
            i64::try_from(record.revision).map_err(|_| invalid("Revision ist zu groß"))?;
        let rows: Vec<(String,Value,Option<String>,Option<String>,Option<String>,Option<String>)> = sqlx::query_as("SELECT f.fact_json,f.binding_identity_json,s.relative_pointer,s.semantic_predicate,s.semantic_qualifiers_json,s.semantic_unit FROM brain.entity_profile_facts_v1 f LEFT JOIN brain.entity_semantic_projections_v1 s USING(entity_key,source_id,logical_id,revision,fact_id) WHERE f.entity_key=$1 AND f.source_id=$2 AND f.logical_id=$3 AND f.revision=$4 ORDER BY f.fact_id")
            .bind(entity_key).bind(&record.source_id).bind(&record.logical_id).bind(revision).fetch_all(&self.pool).await?;
        let mut bindings = Vec::new();
        for (fact, identity, pointer, predicate, qualifiers, unit) in rows {
            let semantic_projection = match (pointer, predicate, qualifiers) {
                (Some(relative_pointer), Some(predicate), Some(qualifiers)) => {
                    Some(SemanticProjection {
                        relative_pointer,
                        predicate,
                        qualifiers: serde_json::from_str(&qualifiers)?,
                        unit,
                    })
                }
                (None, None, None) => None,
                _ => return Err(invalid("Gespeicherte Semantik ist unvollständig")),
            };
            bindings.push(super::derivation::StoredGitBinding {
                source_id: record.source_id.clone(),
                logical_id: record.logical_id.clone(),
                store_revision: record.revision,
                original_fact: serde_json::from_str(&fact)?,
                binding_identity: serde_json::from_value(identity)?,
                semantic_projection,
            });
        }
        Ok(bindings)
    }

    pub async fn list_bound_entity_keys(
        &self,
        release_id: &str,
        principal: &brain_contracts::Principal,
    ) -> Result<Vec<String>> {
        let snapshot = self
            .snapshot(release_id)
            .await
            .map_err(|_| invalid("Freigegebener Quellenstand fehlt"))?;
        let authorized = snapshot
            .authorized(principal, false)
            .map_err(|_| invalid("Quellenfreigabe fehlt"))?;
        let mut keys = std::collections::BTreeSet::new();
        for source in authorized {
            let revision =
                i64::try_from(source.revision).map_err(|_| invalid("Revision ist zu groß"))?;
            let rows:Vec<String>=sqlx::query_scalar("SELECT DISTINCT entity_key FROM brain.entity_profile_facts_v1 WHERE source_id=$1 AND logical_id=$2 AND revision=$3 AND binding_identity_json IS NOT NULL")
                .bind(&source.source_id).bind(&source.logical_id).bind(revision).fetch_all(&self.pool).await?;
            keys.extend(rows);
        }
        Ok(keys.into_iter().collect())
    }
    pub async fn stored_entity_binding_identity(
        &self,
        entity_key: &str,
        record: &SourceRecordV2,
        fact_id: &str,
    ) -> Result<brain_contracts::entity_profile::EntityIdentity> {
        let revision =
            i64::try_from(record.revision).map_err(|_| invalid("Revision ist zu groß"))?;
        let identity:Value=sqlx::query_scalar("SELECT binding_identity_json FROM brain.entity_profile_facts_v1 WHERE entity_key=$1 AND source_id=$2 AND logical_id=$3 AND revision=$4 AND fact_id=$5")
            .bind(entity_key).bind(&record.source_id).bind(&record.logical_id).bind(revision).bind(fact_id).fetch_one(&self.pool).await?;
        Ok(serde_json::from_value(identity)?)
    }
    pub async fn stored_entity_source(
        &self,
        source_id: &str,
        logical_id: &str,
        revision: u64,
    ) -> Result<SourceRecordV2> {
        let revision = i64::try_from(revision).map_err(|_| invalid("Revision ist zu groß"))?;
        let value: Value = sqlx::query_scalar("SELECT record_json FROM brain.source_record_revisions WHERE source_id=$1 AND logical_id=$2 AND revision=$3")
            .bind(source_id).bind(logical_id).bind(revision).fetch_one(&self.pool).await?;
        let record: SourceRecordV2 = serde_json::from_value(value)?;
        record.validate()?;
        Ok(record)
    }

    pub async fn store_entity_semantic_projection(
        &self,
        entity_key: &str,
        record: &SourceRecordV2,
        fact_id: &str,
        projection: &SemanticProjection,
    ) -> Result<usize> {
        self.store_entity_semantic_projections(
            entity_key,
            record,
            &[(fact_id.into(), projection.clone())],
        )
        .await
    }

    pub async fn store_entity_semantic_projections(
        &self,
        entity_key: &str,
        record: &SourceRecordV2,
        projections: &[(String, SemanticProjection)],
    ) -> Result<usize> {
        if projections.len() > 10_000 {
            return Err(invalid("Zu viele semantische Projektionen"));
        }
        let revision =
            i64::try_from(record.revision).map_err(|_| invalid("Revision ist zu groß"))?;
        let mut tx = self.pool.begin().await?;
        let stored: Value = sqlx::query_scalar("SELECT record_json FROM brain.source_record_revisions WHERE source_id=$1 AND logical_id=$2 AND revision=$3 FOR SHARE")
            .bind(&record.source_id).bind(&record.logical_id).bind(revision).fetch_one(&mut *tx).await?;
        if stored != serde_json::to_value(record)? {
            return Err(invalid(
                "Projektion widerspricht der gespeicherten Quellrevision",
            ));
        }
        let ids: Vec<_> = projections.iter().map(|(id, _)| id.clone()).collect();
        let originals = project_entity_facts(record, &ids)?;
        let mut total = 0;
        for ((fact_id, projection), original) in projections.iter().zip(originals) {
            project_semantic_fact(&original, projection)?;
            let bound: String = sqlx::query_scalar("SELECT fact_json FROM brain.entity_profile_facts_v1 WHERE entity_key=$1 AND source_id=$2 AND logical_id=$3 AND revision=$4 AND fact_id=$5 FOR SHARE")
            .bind(entity_key).bind(&record.source_id).bind(&record.logical_id).bind(revision).bind(fact_id).fetch_one(&mut *tx).await?;
            if serde_json::from_str::<EntityProfileFact>(&bound)? != original {
                return Err(invalid("Originalbindung widerspricht der Projektion"));
            }
            let qualifiers = serde_json::to_string(&projection.qualifiers)?;
            let inserted = sqlx::query("INSERT INTO brain.entity_semantic_projections_v1(entity_key,source_id,logical_id,revision,fact_id,relative_pointer,semantic_predicate,semantic_qualifiers_json,semantic_unit) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9) ON CONFLICT DO NOTHING")
            .bind(entity_key).bind(&record.source_id).bind(&record.logical_id).bind(revision).bind(fact_id)
            .bind(&projection.relative_pointer).bind(&projection.predicate).bind(&qualifiers).bind(&projection.unit)
            .execute(&mut *tx).await?.rows_affected() as usize;
            let stored: (String,String,String,Option<String>) = sqlx::query_as("SELECT relative_pointer,semantic_predicate,semantic_qualifiers_json,semantic_unit FROM brain.entity_semantic_projections_v1 WHERE entity_key=$1 AND source_id=$2 AND logical_id=$3 AND revision=$4 AND fact_id=$5")
            .bind(entity_key).bind(&record.source_id).bind(&record.logical_id).bind(revision).bind(fact_id).fetch_one(&mut *tx).await?;
            if stored
                != (
                    projection.relative_pointer.clone(),
                    projection.predicate.clone(),
                    qualifiers,
                    projection.unit.clone(),
                )
            {
                return Err(invalid(
                    "Gespeicherte semantische Projektion widerspricht dem Beleg",
                ));
            }
            total += inserted;
        }
        tx.commit().await?;
        Ok(total)
    }
}
