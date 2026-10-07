use std::collections::{BTreeMap, BTreeSet};

use brain_contracts::entity_profile::{EntityIdentity, EntityKind};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{PgPool, Row};

use crate::knowledge_contract::{KnowledgeDocument, KnowledgeFact, KnowledgeSourceKind};

#[path = "entity_derivation.rs"]
pub mod derivation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogEntity {
    pub identity: EntityIdentity,
    pub identifiers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FactBinding {
    pub entity_key: String,
    pub source_id: String,
    pub document_id: String,
    pub revision: String,
    pub source_kind: KnowledgeSourceKind,
    pub fact: KnowledgeFact,
    pub identity_evidence: Vec<String>,
    pub relative_pointer: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BindingCoverage {
    pub bindings: Vec<FactBinding>,
    pub unresolved_fact_ids: Vec<String>,
    pub ambiguous_fact_ids: Vec<String>,
}

pub async fn load_entity_catalog(pool: &PgPool) -> Result<Vec<CatalogEntity>, sqlx::Error> {
    let mut tx = pool.begin().await?;
    sqlx::raw_sql("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY; SET LOCAL statement_timeout='5000ms'")
        .execute(&mut *tx).await?;
    let rows = sqlx::query("SELECT id,entity_type,canonical_name,primary_external_id FROM brain.entities WHERE entity_type IN ('hero','ability','item') ORDER BY id")
        .fetch_all(&mut *tx).await?;
    let aliases = sqlx::query(
        "SELECT entity_id,alias,alias_kind,id FROM brain.entity_aliases ORDER BY entity_id,id",
    )
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    let mut result = Vec::new();
    for row in rows {
        let id: i64 = row.try_get("id")?;
        let kind = match row.try_get::<String, _>("entity_type")?.as_str() {
            "hero" => EntityKind::Hero,
            "ability" => EntityKind::Ability,
            _ => EntityKind::Item,
        };
        let mut identifiers = Vec::new();
        let mut names = Vec::new();
        let mut evidence = vec![format!("brain.entities:{id}")];
        if let Some(external) = row.try_get::<Option<String>, _>("primary_external_id")? {
            identifiers.push(external);
        }
        for alias in &aliases {
            if alias.try_get::<i64, _>("entity_id")? != id {
                continue;
            }
            let value: String = alias.try_get("alias")?;
            let alias_kind: String = alias.try_get("alias_kind")?;
            if matches!(alias_kind.as_str(), "class_name" | "external_id") {
                identifiers.push(value.clone());
            }
            names.push(value);
            evidence.push(format!(
                "brain.entity_aliases:{}",
                alias.try_get::<i64, _>("id")?
            ));
        }
        identifiers.sort();
        identifiers.dedup();
        names.sort();
        names.dedup();
        result.push(CatalogEntity {
            identity: EntityIdentity {
                entity_key: format!("brain.entities:{id}"),
                kind,
                name: row.try_get("canonical_name")?,
                aliases: names,
                identity_evidence: evidence,
            },
            identifiers,
        });
    }
    Ok(result)
}

fn pointer(fact: &KnowledgeFact) -> Option<&str> {
    fact.qualifiers
        .get("source_pointer")
        .or_else(|| fact.qualifiers.get("json_pointer"))
        .and_then(Value::as_str)
}

fn unescape(token: &str) -> String {
    token.replace("~1", "/").replace("~0", "~")
}

pub fn bind_document(document: &KnowledgeDocument, catalog: &[CatalogEntity]) -> BindingCoverage {
    let mut index: BTreeMap<String, BTreeSet<usize>> = BTreeMap::new();
    for (position, entity) in catalog.iter().enumerate() {
        let names = if document.source_kind == KnowledgeSourceKind::Wiki {
            entity.identity.aliases.as_slice()
        } else {
            &[]
        };
        for id in entity.identifiers.iter().chain(names) {
            if !id.is_empty() {
                index.entry(id.to_lowercase()).or_default().insert(position);
            }
        }
    }
    let mut scopes: BTreeMap<String, BTreeMap<usize, BTreeSet<String>>> = BTreeMap::new();
    for fact in &document.facts {
        let Some(path) = pointer(fact) else { continue };
        let segments: Vec<_> = path.split('/').skip(1).collect();
        for (offset, segment) in segments.iter().take(1).enumerate() {
            if segment.is_empty() || segment.bytes().all(|byte| byte.is_ascii_digit()) {
                continue;
            }
            if let Some(candidates) = index.get(&unescape(segment).to_lowercase()) {
                let scope = format!("/{}", segments[..=offset].join("/"));
                for candidate in candidates {
                    scopes
                        .entry(scope.clone())
                        .or_default()
                        .entry(*candidate)
                        .or_default()
                        .insert(format!(
                            "{}:{}:{}:{}",
                            document.source_id, document.document_id, document.revision, scope
                        ));
                }
            }
        }
        if let Some((parent, field)) = path.rsplit_once('/') {
            if matches!(
                field.to_lowercase().as_str(),
                "class_name" | "classname" | "key" | "external_id"
            ) {
                if let Some(value) = fact.value.as_str() {
                    if let Some(candidates) = index.get(&value.to_lowercase()) {
                        for candidate in candidates {
                            scopes
                                .entry(parent.into())
                                .or_default()
                                .entry(*candidate)
                                .or_default()
                                .insert(format!(
                                    "{}:{}:{}:{}",
                                    document.source_id,
                                    document.document_id,
                                    document.revision,
                                    fact.fact_id
                                ));
                        }
                    }
                }
            }
        }
    }
    let mut result = BindingCoverage::default();
    let mut seen = BTreeSet::new();
    for fact in &document.facts {
        let Some(path) = pointer(fact) else {
            result.unresolved_fact_ids.push(fact.fact_id.clone());
            continue;
        };
        let scope = scopes
            .iter()
            .filter(|(prefix, _)| {
                path == prefix.as_str()
                    || path
                        .strip_prefix(prefix.as_str())
                        .is_some_and(|rest| rest.starts_with('/'))
            })
            .max_by_key(|(prefix, _)| prefix.len());
        let Some((prefix, candidates)) = scope else {
            result.unresolved_fact_ids.push(fact.fact_id.clone());
            continue;
        };
        if candidates.len() != 1 {
            result.ambiguous_fact_ids.push(fact.fact_id.clone());
            continue;
        }
        let (&position, evidence) = candidates
            .iter()
            .next()
            .expect("Nichtleere Kandidatenmenge");
        if seen.insert((position, fact.fact_id.clone())) {
            let mut identity_evidence = catalog[position].identity.identity_evidence.clone();
            identity_evidence.extend(evidence.iter().cloned());
            identity_evidence.sort();
            identity_evidence.dedup();
            result.bindings.push(FactBinding {
                entity_key: catalog[position].identity.entity_key.clone(),
                source_id: document.source_id.clone(),
                document_id: document.document_id.clone(),
                revision: document.revision.clone(),
                source_kind: document.source_kind,
                fact: fact.clone(),
                identity_evidence,
                relative_pointer: path[prefix.len()..].into(),
            });
        }
    }
    result
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BindingConflict {
    pub entity_key: String,
    pub relative_pointer: String,
    pub fact_references: Vec<String>,
    pub preferred_fact_reference: Option<String>,
}

pub fn binding_conflicts(bindings: &[FactBinding]) -> Vec<BindingConflict> {
    let mut groups = BTreeMap::new();
    for binding in bindings {
        let mut qualifiers = binding.fact.qualifiers.clone();
        for key in [
            "source_pointer",
            "json_pointer",
            "source_lexeme",
            "numeric_representation",
            "gameplay_binding",
            "references_resolved",
            "type_flags",
            "unit_status",
            "unit_inferred",
        ] {
            qualifiers.remove(key);
        }
        groups
            .entry((
                binding.entity_key.clone(),
                binding.relative_pointer.clone(),
                serde_json::to_string(&qualifiers).expect("JSON-Qualifier"),
                binding.fact.unit.clone(),
            ))
            .or_insert_with(Vec::new)
            .push(binding);
    }
    groups
        .into_iter()
        .filter_map(|((entity_key, relative_pointer, _, _), group)| {
            if group
                .iter()
                .all(|binding| binding.fact.value == group[0].fact.value)
            {
                return None;
            }
            let numeric = group.iter().all(|binding| numeric_fact(&binding.fact));
            let preferred_kind = if numeric {
                KnowledgeSourceKind::GameFile
            } else {
                KnowledgeSourceKind::Wiki
            };
            let mut preferred: Vec<_> = group
                .iter()
                .filter(|binding| binding.source_kind == preferred_kind)
                .collect();
            preferred.sort_by_key(|binding| reference(binding));
            let consistent = preferred.first().is_some_and(|first| {
                preferred
                    .iter()
                    .all(|binding| binding.fact.value == first.fact.value)
            });
            let mut fact_references: Vec<_> =
                group.iter().map(|binding| reference(binding)).collect();
            fact_references.sort();
            fact_references.dedup();
            Some(BindingConflict {
                entity_key,
                relative_pointer,
                fact_references,
                preferred_fact_reference: if consistent {
                    preferred.first().map(|binding| reference(binding))
                } else {
                    None
                },
            })
        })
        .collect()
}

fn reference(binding: &FactBinding) -> String {
    format!(
        "{}:{}:{}:{}",
        binding.source_id, binding.document_id, binding.revision, binding.fact.fact_id
    )
}

fn numeric_fact(fact: &KnowledgeFact) -> bool {
    fact.value.is_number()
        || (fact
            .qualifiers
            .get("numeric_representation")
            .and_then(Value::as_str)
            == Some("source_numeric_lexeme")
            && fact.value.as_str().is_some_and(|text| {
                serde_json::from_str::<Value>(text).is_ok_and(|value| value.is_number())
            }))
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StoredBindingSummary {
    pub bound_facts: usize,
    pub inserted_bindings: usize,
    pub inserted_projections: usize,
    pub entity_keys: Vec<String>,
    pub unresolved_fact_ids: Vec<String>,
    pub ambiguous_fact_ids: Vec<String>,
}

pub async fn bind_stored_document(
    store: &brain_storage::PgStore,
    source_id: &str,
    logical_id: &str,
    revision: u64,
    catalog: &[CatalogEntity],
) -> brain_storage::Result<StoredBindingSummary> {
    use brain_storage::entity_profile::{
        semantic::{project_semantic_fact_with_context, semantic_projection_with_context},
        EntityDocumentContext,
    };
    let record = store
        .stored_entity_source(source_id, logical_id, revision)
        .await?;
    let context = EntityDocumentContext::new(&record)?;
    let document = KnowledgeDocument::deserialize(context.document())?;
    let coverage = bind_document(&document, catalog);
    let original_prefix = format!(
        "{}:{}:{}:",
        document.source_id, document.document_id, document.revision
    );
    drop(document);
    let mut result = StoredBindingSummary {
        bound_facts: coverage.bindings.len(),
        unresolved_fact_ids: coverage.unresolved_fact_ids,
        ambiguous_fact_ids: coverage.ambiguous_fact_ids,
        ..Default::default()
    };
    let mut grouped: BTreeMap<String, Vec<FactBinding>> = BTreeMap::new();
    for binding in coverage.bindings {
        grouped
            .entry(binding.entity_key.clone())
            .or_default()
            .push(binding);
    }
    for (key, bindings) in grouped {
        let entry = catalog
            .iter()
            .find(|entry| entry.identity.entity_key == key)
            .ok_or_else(|| {
                brain_storage::StorageError::Json(<serde_json::Error as serde::de::Error>::custom(
                    "Katalogbeleg fehlt",
                ))
            })?;
        let mut identity = entry.identity.clone();
        identity.aliases.extend(entry.identifiers.clone());
        identity.aliases.sort();
        identity.aliases.dedup();
        identity.identity_evidence = bindings
            .iter()
            .flat_map(|binding| binding.identity_evidence.clone())
            .collect();
        identity.identity_evidence.sort();
        identity.identity_evidence.dedup();
        let previous: BTreeMap<_, _> = store
            .stored_git_entity_bindings_with_context(&key, &context)
            .await?
            .into_iter()
            .map(|binding| (binding.original_fact.fact_id.clone(), binding))
            .collect();
        identity
            .identity_evidence
            .extend(previous.values().flat_map(|binding| {
                binding
                    .binding_identity
                    .identity_evidence
                    .iter()
                    .filter(|evidence| evidence.starts_with(&original_prefix))
                    .cloned()
            }));
        identity.identity_evidence.sort();
        identity.identity_evidence.dedup();
        for chunk in bindings.chunks(10_000) {
            let ids: Vec<_> = chunk
                .iter()
                .map(|binding| binding.fact.fact_id.clone())
                .collect();
            let mut projections = Vec::new();
            for (binding, original) in chunk.iter().zip(context.project(&ids)?) {
                if let Some(previous) = previous.get(&original.fact_id) {
                    if let Some(projection) = &previous.semantic_projection {
                        project_semantic_fact_with_context(
                            &original,
                            projection,
                            &context,
                            &previous.binding_identity,
                        )?;
                        project_semantic_fact_with_context(
                            &original, projection, &context, &identity,
                        )?;
                        projections.push((original.fact_id, projection.clone()));
                        continue;
                    }
                }
                if let Some(projection) = semantic_projection_with_context(
                    &original,
                    &binding.relative_pointer,
                    &context,
                    &identity,
                )? {
                    projections.push((original.fact_id, projection));
                }
            }
            result.inserted_bindings += store
                .store_entity_fact_bindings_with_context(&identity, &context, &ids)
                .await?;
            result.inserted_projections += store
                .store_entity_semantic_projections_with_context(&key, &context, &projections)
                .await?;
        }
        result.entity_keys.push(key);
    }
    Ok(result)
}
