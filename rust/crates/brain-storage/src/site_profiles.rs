use brain_contracts::{
    entity_profile::{EntityIdentity, EntityProfile, EntityProfileFact, ProfileSourceKind},
    source::{origin_from_record, SourceRevision},
    store::record_publication_allowed,
    DocumentDescriptor, Principal, SourceRecordV2, SourceVisibility,
};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};

use crate::{entity_profile, PgStore, StorageError};

fn invalid() -> StorageError {
    StorageError::Json(<serde_json::Error as serde::de::Error>::custom(
        "Öffentlicher Steckbrief ist nicht verfügbar",
    ))
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Envelope {
    documents: Vec<Document>,
    bindings: Vec<Binding>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Document {
    record: SourceRecordV2,
    head: DocumentDescriptor,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Binding {
    source_id: String,
    logical_id: String,
    revision: u64,
    fact: EntityProfileFact,
    identity: EntityIdentity,
    semantic: Option<Projection>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Projection {
    relative_pointer: String,
    predicate: String,
    qualifiers: serde_json::Map<String, serde_json::Value>,
    unit: Option<String>,
}

fn profiles(envelope: Envelope) -> crate::Result<Vec<EntityProfile>> {
    let mut documents = BTreeMap::new();
    for document in &envelope.documents {
        let record = &document.record;
        if documents
            .insert(
                (&record.source_id, &record.logical_id, record.revision),
                (
                    document,
                    entity_profile::EntityDocumentContext::new(record)?,
                ),
            )
            .is_some()
        {
            return Err(invalid());
        }
    }
    let public = Principal {
        actor_id: "public-site".into(),
        channel: "public-site".into(),
        scopes: BTreeSet::new(),
        provider_egress: BTreeSet::new(),
    };
    let mut grouped: BTreeMap<String, (EntityIdentity, Vec<EntityProfileFact>)> = BTreeMap::new();
    for binding in envelope.bindings {
        let (document, context) = documents
            .get(&(&binding.source_id, &binding.logical_id, binding.revision))
            .ok_or_else(invalid)?;
        let record = &document.record;
        document.head.head.validate().map_err(|_| invalid())?;
        let original = origin_from_record(record).map_err(|_| invalid())?;
        if !record_publication_allowed(record, &document.head.head, &public)
            || record.visibility != SourceVisibility::Public
            || !record.allowed_scopes.is_empty()
            || binding.fact.provenance.source_kind != ProfileSourceKind::GameFile
            || !matches!(original.source_revision, SourceRevision::Git { .. })
        {
            continue;
        }
        let identity = binding.identity;
        if identity.entity_key.is_empty()
            || identity.entity_key.len() > 100
            || identity.name.trim().is_empty()
            || identity.identity_evidence.is_empty()
        {
            return Err(invalid());
        }
        let verified = context.project(std::slice::from_ref(&binding.fact.fact_id))?;
        if verified != [binding.fact.clone()] {
            return Err(invalid());
        }
        let fact = match binding.semantic {
            Some(projection) => entity_profile::semantic::project_semantic_fact_with_context(
                &binding.fact,
                &entity_profile::semantic::SemanticProjection {
                    relative_pointer: projection.relative_pointer,
                    predicate: projection.predicate,
                    qualifiers: projection.qualifiers,
                    unit: projection.unit,
                },
                context,
                &identity,
            )?,
            None => binding.fact,
        };
        let entry = grouped
            .entry(identity.entity_key.clone())
            .or_insert_with(|| (identity.clone(), Vec::new()));
        if entry.0.kind != identity.kind || entry.0.name != identity.name {
            return Err(invalid());
        }
        entry.1.push(fact);
    }
    Ok(grouped
        .into_values()
        .map(|(identity, facts)| {
            entity_profile::assemble_profile(
                entity_profile::consumer_entity_identity(&identity),
                None,
                facts,
                Vec::new(),
            )
        })
        .collect())
}

impl PgStore {
    pub async fn migrate_site_comments(&self) -> crate::Result<()> {
        let exists: bool = sqlx::query_scalar(
            "SELECT to_regprocedure('brain.append_site_comment_v1(text,text,text)') IS NOT NULL",
        )
        .fetch_one(&self.pool)
        .await?;
        if !exists {
            sqlx::raw_sql(include_str!(
                "../../../../scripts/migrations/2026-10-08-brain-site-comments-v1.sql"
            ))
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }

    pub async fn migrate_site_profiles(&self) -> crate::Result<()> {
        let exists: bool = sqlx::query_scalar(
            "SELECT to_regprocedure('brain.read_site_profile_bindings_v1(text,text)') IS NOT NULL",
        )
        .fetch_one(&self.pool)
        .await?;
        if !exists {
            sqlx::raw_sql(include_str!(
                "../../../../scripts/migrations/2026-10-08-brain-site-profiles-v1.sql"
            ))
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }

    pub async fn read_site_profiles(
        &self,
        release: &str,
        key: Option<&str>,
    ) -> crate::Result<Vec<EntityProfile>> {
        if release.trim().is_empty()
            || release.len() > 160
            || key.is_some_and(|key| key.is_empty() || key.len() > 100)
        {
            return Err(invalid());
        }
        let mut tx = self.pool.begin().await?;
        sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ, READ ONLY")
            .execute(&mut *tx)
            .await?;
        sqlx::query("SET LOCAL statement_timeout='5000ms'")
            .execute(&mut *tx)
            .await?;
        let value: serde_json::Value =
            sqlx::query_scalar("SELECT brain.read_site_profile_bindings_v1($1,$2)")
                .bind(release)
                .bind(key)
                .fetch_one(&mut *tx)
                .await?;
        let envelope: Envelope = serde_json::from_value(value)?;
        if envelope.bindings.len() > 20000 || envelope.documents.len() > 10000 {
            return Err(invalid());
        }
        let result = profiles(envelope)?;
        if key.is_some_and(|key| {
            result
                .iter()
                .any(|profile| profile.entity.entity_key != key)
        }) {
            return Err(invalid());
        }
        tx.commit().await?;
        Ok(result)
    }
}
