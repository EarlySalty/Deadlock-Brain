//! Frische Producerbelege ohne Übertragung archivierter Dokumentbäume.
use crate::{
    entity_profile::{
        derivation::{OriginalEvidenceHeader, StoredGitBinding, VerifiedOriginalEvidence},
        semantic::SemanticProjection,
    },
    PgStore, Result,
};
use brain_contracts::{
    entity_profile::{EntityProfileFact, ProfileSourceKind},
    CorpusRelease, Principal, ReleaseReadManifest,
};
use serde_json::Value;
use std::collections::BTreeSet;

fn invalid(message: &str) -> crate::StorageError {
    crate::StorageError::Json(<serde_json::Error as serde::de::Error>::custom(message))
}

type BindingRow = (
    String,
    String,
    i64,
    String,
    Option<Value>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
);

impl PgStore {
    pub async fn entity_derivation_inputs(
        &self,
        release_id: &str,
        operator: &Principal,
        entity_key: &str,
    ) -> Result<(VerifiedOriginalEvidence, Vec<StoredGitBinding>)> {
        let mut tx = self.pool.begin().await?;
        sqlx::raw_sql("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ, READ ONLY")
            .execute(&mut *tx)
            .await?;
        let release: Value = sqlx::query_scalar(
            "SELECT release_json FROM brain.corpus_releases_v1 WHERE release_id=$1",
        )
        .bind(release_id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| invalid("Originalrelease fehlt"))?;
        let release: CorpusRelease = serde_json::from_value(release)?;
        crate::validate_release(&release).map_err(|_| invalid("Originalrelease ist ungültig"))?;
        if release.release_id != release_id {
            return Err(invalid("Originalrelease widerspricht dem Schlüssel"));
        }
        let pins = serde_json::to_value(&release.source_revisions)?;
        let rows: Vec<(Value,Value)> = sqlx::query_as("SELECT r.read_header_json,h.read_header_json->'head' FROM jsonb_each($1::jsonb) s CROSS JOIN LATERAL jsonb_each_text(s.value) d JOIN brain.source_record_revisions r ON r.source_id=s.key AND r.logical_id=d.key AND r.revision=d.value::bigint JOIN brain.source_record_heads h USING(source_id,logical_id) ORDER BY r.source_id,r.logical_id")
            .bind(&pins).fetch_all(&mut *tx).await?;
        let mut manifest = ReleaseReadManifest {
            release,
            revisions: Vec::new(),
            heads: Vec::new(),
        };
        for (revision, head) in rows {
            manifest.revisions.push(serde_json::from_value(revision)?);
            manifest.heads.push(serde_json::from_value(head)?);
        }
        let authorized: BTreeSet<_> = manifest
            .authorized(operator, false, false)
            .map_err(|_| invalid("Frische Originalautorisierung fehlt"))?
            .into_iter()
            .map(|record| (record.source_id, record.logical_id, record.revision))
            .collect();
        let rows: Vec<BindingRow> = sqlx::query_as("SELECT f.source_id,f.logical_id,f.revision,f.fact_json,f.binding_identity_json,s.relative_pointer,s.semantic_predicate,s.semantic_qualifiers_json,s.semantic_unit FROM brain.entity_profile_facts_v1 f LEFT JOIN brain.entity_semantic_projections_v1 s USING(entity_key,source_id,logical_id,revision,fact_id) WHERE f.entity_key=$1 AND ($2::jsonb->f.source_id->>f.logical_id)::bigint=f.revision ORDER BY f.source_id,f.logical_id,f.revision,f.fact_id")
            .bind(entity_key).bind(&pins).fetch_all(&mut *tx).await?;
        let mut bindings = Vec::new();
        let mut needed = BTreeSet::new();
        for (
            source_id,
            logical_id,
            revision,
            fact,
            identity,
            pointer,
            predicate,
            qualifiers,
            unit,
        ) in rows
        {
            let revision =
                u64::try_from(revision).map_err(|_| invalid("Originalrevision ist ungültig"))?;
            if !authorized.contains(&(source_id.clone(), logical_id.clone(), revision)) {
                continue;
            }
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
            needed.insert((source_id.clone(), logical_id.clone(), revision));
            bindings.push(StoredGitBinding {
                source_id,
                logical_id,
                store_revision: revision,
                original_fact,
                binding_identity: serde_json::from_value(
                    identity.ok_or_else(|| invalid("Belegte Bindungsidentität fehlt"))?,
                )?,
                semantic_projection,
            });
        }
        let mut originals = Vec::new();
        for (source, logical, revision) in needed {
            let descriptor = manifest
                .revisions
                .iter()
                .find(|record| {
                    record.head.source_id == source
                        && record.head.logical_id == logical
                        && record.head.revision == revision
                })
                .ok_or_else(|| invalid("Originalheader ist nicht gepinnt"))?
                .clone();
            let header: Value = sqlx::query_scalar("SELECT original_header_json FROM brain.source_record_revisions WHERE source_id=$1 AND logical_id=$2 AND revision=$3")
                .bind(&source).bind(&logical).bind(i64::try_from(revision).map_err(|_| invalid("Originalrevision ist zu groß"))?).fetch_one(&mut *tx).await?;
            originals.push(OriginalEvidenceHeader {
                descriptor,
                document_header: header,
            });
        }
        tx.commit().await?;
        Ok((
            VerifiedOriginalEvidence {
                manifest,
                originals,
            },
            bindings,
        ))
    }
}
