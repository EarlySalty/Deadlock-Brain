use super::{invalid, EntityDocumentContext};
use crate::{PgStore, Result};
use brain_contracts::{
    entity_profile::{EntityIdentity, EntityProfileFact, ProfileSourceKind},
    SourceRecordV2,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::BTreeMap;

type StoredGitBindingRow = (
    String,
    Value,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
);

type DocumentBindingRow = (
    String,
    String,
    String,
    Option<Value>,
    Option<Value>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SemanticProjection {
    pub relative_pointer: String,
    pub predicate: String,
    pub qualifiers: Map<String, Value>,
    pub unit: Option<String>,
}

fn bound_relative_pointer_from_facts(
    fact: &EntityProfileFact,
    identity: &EntityIdentity,
    source_id: &str,
    logical_id: &str,
    facts: &[Value],
    relative_pointer: &str,
    by_id: Option<&BTreeMap<String, usize>>,
) -> Result<String> {
    let pointer = fact
        .qualifiers
        .get("source_pointer")
        .or_else(|| fact.qualifiers.get("json_pointer"))
        .and_then(Value::as_str)
        .ok_or_else(|| invalid("Vollständiger Originalpfad fehlt"))?;
    let evidence_prefix = format!(
        "{}:{}:{}:",
        source_id, logical_id, fact.provenance.original_revision
    );
    let matches_identity = |identifier: &str| {
        std::iter::once(&identity.entity_key)
            .chain(std::iter::once(&identity.name))
            .chain(&identity.aliases)
            .any(|name| name.to_lowercase() == identifier.to_lowercase())
    };
    let mut scopes = Vec::new();
    for evidence in &identity.identity_evidence {
        let Some(reference) = evidence.strip_prefix(&evidence_prefix) else {
            continue;
        };
        if let Some(root) = reference.strip_prefix('/') {
            if !root.is_empty()
                && !root.contains('/')
                && !root.bytes().all(|byte| byte.is_ascii_digit())
                && matches_identity(&root.replace("~1", "/").replace("~0", "~"))
            {
                scopes.push(reference.to_owned());
            }
        }
        let identity_fact = match by_id {
            Some(index) => index.get(reference).and_then(|index| facts.get(*index)),
            None => facts
                .iter()
                .find(|fact| fact["fact_id"].as_str() == Some(reference)),
        };
        if let Some(identity_fact) = identity_fact {
            let identity_pointer = identity_fact["qualifiers"]["source_pointer"]
                .as_str()
                .or_else(|| identity_fact["qualifiers"]["json_pointer"].as_str());
            if let Some((parent, field)) =
                identity_pointer.and_then(|pointer| pointer.rsplit_once('/'))
            {
                if matches!(
                    field.to_lowercase().as_str(),
                    "class_name" | "classname" | "key" | "external_id"
                ) && identity_fact["value"]
                    .as_str()
                    .is_some_and(matches_identity)
                {
                    scopes.push(parent.to_owned());
                }
            }
        }
    }
    let scope = scopes
        .iter()
        .filter(|scope| {
            pointer
                .strip_prefix(scope.as_str())
                .is_some_and(|rest| rest.starts_with('/'))
        })
        .find(|scope| &pointer[scope.len()..] == relative_pointer)
        .ok_or_else(|| {
            invalid("Konkrete Entitätsbindung besitzt keinen passenden Originalbeleg")
        })?;
    Ok(pointer[scope.len()..].into())
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

fn numeric_original(fact: &EntityProfileFact) -> bool {
    fact.value.is_number()
        || (fact
            .qualifiers
            .get("numeric_representation")
            .and_then(Value::as_str)
            == Some("source_numeric_lexeme")
            && fact
                .value
                .as_str()
                .is_some_and(|text| serde_json::from_str::<serde_json::Number>(text).is_ok()))
}

pub fn semantic_projection(
    fact: &EntityProfileFact,
    relative_pointer: &str,
    record: &SourceRecordV2,
    binding_identity: &EntityIdentity,
) -> Result<Option<SemanticProjection>> {
    if !numeric_original(fact) {
        return Ok(None);
    }
    let context = EntityDocumentContext::new(record)?;
    semantic_projection_with_context(fact, relative_pointer, &context, binding_identity)
}

pub fn semantic_projection_with_context(
    fact: &EntityProfileFact,
    relative_pointer: &str,
    context: &EntityDocumentContext<'_>,
    binding_identity: &EntityIdentity,
) -> Result<Option<SemanticProjection>> {
    if !numeric_original(fact) {
        return Ok(None);
    }
    let original = context.project(std::slice::from_ref(&fact.fact_id))?;
    if original[0] != *fact || binding_identity.entity_key.is_empty() {
        return Err(invalid(
            "Semantischer Originalbeleg widerspricht der Bindung",
        ));
    }
    let record = context.record();
    let relative = bound_relative_pointer_from_facts(
        fact,
        binding_identity,
        &record.source_id,
        &record.logical_id,
        context.document()["facts"]
            .as_array()
            .ok_or_else(|| invalid("Originalfakten fehlen"))?,
        relative_pointer,
        Some(context.fact_index()),
    )?;
    semantic_projection_verified(fact, relative_pointer, &relative)
}

pub(crate) fn semantic_projection_from_verified_original(
    fact: &EntityProfileFact,
    relative_pointer: &str,
    source_id: &str,
    logical_id: &str,
    identity: &EntityIdentity,
    facts: &[Value],
    by_id: &BTreeMap<String, usize>,
) -> Result<Option<SemanticProjection>> {
    if !numeric_original(fact) {
        return Ok(None);
    }
    let relative = bound_relative_pointer_from_facts(
        fact,
        identity,
        source_id,
        logical_id,
        facts,
        relative_pointer,
        Some(by_id),
    )?;
    semantic_projection_verified(fact, relative_pointer, &relative)
}

fn semantic_projection_verified(
    fact: &EntityProfileFact,
    relative_pointer: &str,
    proven_relative_pointer: &str,
) -> Result<Option<SemanticProjection>> {
    if relative_pointer.is_empty()
        || !relative_pointer.starts_with('/')
        || proven_relative_pointer != relative_pointer
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
    record: &SourceRecordV2,
    binding_identity: &EntityIdentity,
) -> Result<EntityProfileFact> {
    let context = EntityDocumentContext::new(record)?;
    project_semantic_fact_with_context(original, projection, &context, binding_identity)
}

pub fn project_semantic_fact_with_context(
    original: &EntityProfileFact,
    projection: &SemanticProjection,
    context: &EntityDocumentContext<'_>,
    binding_identity: &EntityIdentity,
) -> Result<EntityProfileFact> {
    if semantic_projection_with_context(
        original,
        &projection.relative_pointer,
        context,
        binding_identity,
    )?
    .as_ref()
        != Some(projection)
    {
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
    /// Vollständige gespeicherte Bindungen einer unveränderten Originalrevision.
    pub async fn stored_document_bindings_with_context(
        &self,
        context: &EntityDocumentContext<'_>,
    ) -> Result<Vec<(String, super::derivation::StoredGitBinding)>> {
        let record = context.record();
        let revision =
            i64::try_from(record.revision).map_err(|_| invalid("Revision ist zu groß"))?;
        let mut tx = self.pool.begin().await?;
        sqlx::raw_sql("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ, READ ONLY")
            .execute(&mut *tx)
            .await?;
        crate::pg_jobs::lock_source(&mut tx, &record.source_id).await?;
        let stored: Value = sqlx::query_scalar("SELECT record_json FROM brain.source_record_revisions WHERE source_id=$1 AND logical_id=$2 AND revision=$3")
            .bind(&record.source_id).bind(&record.logical_id).bind(revision).fetch_one(&mut *tx).await?;
        if stored != serde_json::to_value(record)? {
            return Err(invalid(
                "Bindungsprüfung widerspricht der gespeicherten Quellrevision",
            ));
        }
        drop(stored);
        let rows: Vec<DocumentBindingRow> = sqlx::query_as("SELECT f.entity_key,f.fact_id,f.fact_json,f.binding_identity_json,e.identity_json,s.relative_pointer,s.semantic_predicate,s.semantic_qualifiers_json,s.semantic_unit FROM brain.entity_profile_facts_v1 f LEFT JOIN brain.entity_profile_entities_v1 e USING(entity_key) LEFT JOIN brain.entity_semantic_projections_v1 s USING(entity_key,source_id,logical_id,revision,fact_id) WHERE f.source_id=$1 AND f.logical_id=$2 AND f.revision=$3 ORDER BY f.entity_key,f.fact_id")
            .bind(&record.source_id).bind(&record.logical_id).bind(revision).fetch_all(&mut *tx).await?;
        let mut bindings = Vec::new();
        for (key, fact_id, fact, identity, entity, pointer, predicate, qualifiers, unit) in rows {
            let original_fact: EntityProfileFact = serde_json::from_str(&fact)?;
            if original_fact.fact_id != fact_id {
                return Err(invalid(
                    "Gespeicherter Faktenschlüssel widerspricht dem Originalbeleg",
                ));
            }
            let identity: EntityIdentity = serde_json::from_value(
                identity.ok_or_else(|| invalid("Gespeicherte Bindungsidentität fehlt"))?,
            )?;
            let entity: EntityIdentity = serde_json::from_value(
                entity.ok_or_else(|| invalid("Gespeicherte Entität fehlt"))?,
            )?;
            if identity.entity_key != key
                || entity.entity_key != key
                || identity.kind != entity.kind
            {
                return Err(invalid(
                    "Gespeicherte Entität widerspricht der Faktenbindung",
                ));
            }
            let projection = match (pointer, predicate, qualifiers) {
                (Some(relative_pointer), Some(predicate), Some(qualifiers)) => {
                    Some(SemanticProjection {
                        relative_pointer,
                        predicate,
                        qualifiers: serde_json::from_str(&qualifiers)?,
                        unit,
                    })
                }
                (None, None, None) if unit.is_none() => None,
                _ => return Err(invalid("Gespeicherte Semantik ist unvollständig")),
            };
            bindings.push((
                key,
                super::derivation::StoredGitBinding {
                    source_id: record.source_id.clone(),
                    logical_id: record.logical_id.clone(),
                    store_revision: record.revision,
                    original_fact,
                    binding_identity: identity,
                    semantic_projection: projection,
                },
            ));
        }
        tx.commit().await?;
        Ok(bindings)
    }
    pub async fn stored_git_entity_bindings(
        &self,
        entity_key: &str,
        record: &SourceRecordV2,
    ) -> Result<Vec<super::derivation::StoredGitBinding>> {
        self.stored_git_entity_bindings_inner(entity_key, record, None)
            .await
    }

    pub async fn stored_git_entity_bindings_with_context(
        &self,
        entity_key: &str,
        context: &EntityDocumentContext<'_>,
    ) -> Result<Vec<super::derivation::StoredGitBinding>> {
        self.stored_git_entity_bindings_inner(entity_key, context.record(), Some(context))
            .await
    }

    async fn stored_git_entity_bindings_inner(
        &self,
        entity_key: &str,
        record: &SourceRecordV2,
        context: Option<&EntityDocumentContext<'_>>,
    ) -> Result<Vec<super::derivation::StoredGitBinding>> {
        let revision =
            i64::try_from(record.revision).map_err(|_| invalid("Revision ist zu groß"))?;
        let rows: Vec<StoredGitBindingRow> = sqlx::query_as("SELECT f.fact_json,f.binding_identity_json,s.relative_pointer,s.semantic_predicate,s.semantic_qualifiers_json,s.semantic_unit FROM brain.entity_profile_facts_v1 f LEFT JOIN brain.entity_semantic_projections_v1 s USING(entity_key,source_id,logical_id,revision,fact_id) WHERE f.entity_key=$1 AND f.source_id=$2 AND f.logical_id=$3 AND f.revision=$4 ORDER BY f.fact_id")
            .bind(entity_key).bind(&record.source_id).bind(&record.logical_id).bind(revision).fetch_all(&self.pool).await?;
        let mut bindings = Vec::new();
        for (fact, identity, pointer, predicate, qualifiers, unit) in rows {
            let original_fact: EntityProfileFact = serde_json::from_str(&fact)?;
            if original_fact.provenance.source_kind != ProfileSourceKind::GameFile {
                continue;
            }
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
                original_fact,
                binding_identity: serde_json::from_value(identity)?,
                semantic_projection,
            });
        }
        if bindings.is_empty() {
            return Ok(Vec::new());
        }
        let parsed;
        let document = if let Some(context) = context {
            context.document()
        } else {
            parsed = serde_json::from_str::<Value>(
                record
                    .metadata
                    .get(crate::source_versions::DOCUMENT_METADATA_KEY)
                    .ok_or_else(|| invalid("Originaldokument fehlt"))?,
            )?;
            &parsed
        };
        if document["source_kind"] != "game_file" {
            return Ok(Vec::new());
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
        let context = EntityDocumentContext::new(record)?;
        self.store_entity_semantic_projections_with_context(entity_key, &context, projections)
            .await
    }

    pub async fn store_entity_semantic_projections_with_context(
        &self,
        entity_key: &str,
        context: &EntityDocumentContext<'_>,
        projections: &[(String, SemanticProjection)],
    ) -> Result<usize> {
        let record = context.record();
        if projections.len() > 10_000 {
            return Err(invalid("Zu viele semantische Projektionen"));
        }
        let revision =
            i64::try_from(record.revision).map_err(|_| invalid("Revision ist zu groß"))?;
        let mut tx = self.pool.begin().await?;
        crate::pg_jobs::lock_source(&mut tx, &record.source_id).await?;
        let stored: Value = sqlx::query_scalar("SELECT record_json FROM brain.source_record_revisions WHERE source_id=$1 AND logical_id=$2 AND revision=$3")
            .bind(&record.source_id).bind(&record.logical_id).bind(revision).fetch_one(&mut *tx).await?;
        if stored != serde_json::to_value(record)? {
            return Err(invalid(
                "Projektion widerspricht der gespeicherten Quellrevision",
            ));
        }
        let ids: Vec<_> = projections.iter().map(|(id, _)| id.clone()).collect();
        let originals = context.project(&ids)?;
        let mut total = 0;
        for ((fact_id, projection), original) in projections.iter().zip(originals) {
            let (bound, identity): (String, Value) = sqlx::query_as("SELECT fact_json,binding_identity_json FROM brain.entity_profile_facts_v1 WHERE entity_key=$1 AND source_id=$2 AND logical_id=$3 AND revision=$4 AND fact_id=$5 FOR SHARE")
            .bind(entity_key).bind(&record.source_id).bind(&record.logical_id).bind(revision).bind(fact_id).fetch_one(&mut *tx).await?;
            let identity: EntityIdentity = serde_json::from_value(identity)?;
            if identity.entity_key != entity_key {
                return Err(invalid(
                    "Gespeicherte Entitätsbindung widerspricht der Projektion",
                ));
            }
            project_semantic_fact_with_context(&original, projection, context, &identity)?;
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
