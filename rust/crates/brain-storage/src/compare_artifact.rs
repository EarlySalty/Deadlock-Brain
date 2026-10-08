use brain_contracts::{
    store::record_publication_allowed, CorpusRelease, DocumentHead, DocumentRevision, PortError,
    Principal, SnapshotReadPort, SourceRecordV2, SourceVisibility,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

use crate::PgStore;

const MIGRATION: &str =
    include_str!("../../../../scripts/migrations/2026-10-07-brain-compare-artifacts-v1.sql");
const MAX_BYTES: usize = 2_000_000;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompareDependency {
    pub document: DocumentRevision,
    pub record_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompareReleaseBinding {
    pub release_id: String,
    pub knowledge_version: String,
    pub patch: String,
    pub manifest_sha256: String,
}

impl CompareReleaseBinding {
    pub fn from_release(release: &CorpusRelease) -> Result<Self, PortError> {
        Ok(Self {
            release_id: release.release_id.clone(),
            knowledge_version: release.knowledge_version.clone(),
            patch: release.patch.clone(),
            manifest_sha256: compare_fingerprint(release)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompareArtifactBody {
    pub release: CompareReleaseBinding,
    pub calculation: serde_json::Value,
    pub render_model: serde_json::Value,
    pub mechanism_version: String,
    pub dependencies: Vec<CompareDependency>,
    pub html: String,
    pub svg: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompareArtifact {
    id: String,
    body: CompareArtifactBody,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ComparePublication {
    pub artifact_id: String,
    pub html_path: String,
    pub svg_path: String,
}

impl ComparePublication {
    pub fn public_link(&self) -> Result<String, PortError> {
        if !valid_compare_id(&self.artifact_id)
            || self.html_path != format!("/site/compare/{}", self.artifact_id)
            || self.svg_path != format!("/site/compare/{}/chart.svg", self.artifact_id)
        {
            return Err(invalid());
        }
        Ok(format!(
            "https://deutsche-deadlock-community.de/brain{}",
            self.html_path
        ))
    }
}

pub trait CompareCalculationVerifier: Send + Sync {
    fn verify(&self, body: &CompareArtifactBody) -> Result<(), PortError>;
}

pub struct UnconfirmedCompareCalculation;

impl CompareCalculationVerifier for UnconfirmedCompareCalculation {
    fn verify(&self, _: &CompareArtifactBody) -> Result<(), PortError> {
        Err(PortError::Unavailable(
            "Bestätigte Vergleichsrechnung fehlt".into(),
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PublicationReceipt {
    artifact_id: String,
    contract: String,
    dependency_sha256: String,
    release_jsonb_sha256: String,
}

fn invalid() -> PortError {
    PortError::InvalidResponse("Vergleich ist nicht zur Veröffentlichung verfügbar".into())
}

fn unavailable(_: sqlx::Error) -> PortError {
    PortError::Unavailable("Vergleichsspeicher ist nicht verfügbar".into())
}

pub fn compare_sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn json_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, PortError> {
    serde_json::to_vec(value).map_err(|_| invalid())
}

pub fn compare_fingerprint<T: Serialize>(value: &T) -> Result<String, PortError> {
    Ok(compare_sha256(&json_bytes(value)?))
}

pub fn compare_dependency(record: &SourceRecordV2) -> Result<CompareDependency, PortError> {
    record.validate().map_err(|_| invalid())?;
    if record.tombstone || !valid_compare_id(&record.content_hash) {
        return Err(invalid());
    }
    Ok(CompareDependency {
        document: DocumentRevision {
            source_id: record.source_id.clone(),
            logical_id: record.logical_id.clone(),
            revision: record.revision,
            content_hash: record.content_hash.clone(),
        },
        record_sha256: compare_fingerprint(record)?,
    })
}

impl CompareArtifact {
    pub fn pending(body: CompareArtifactBody) -> Result<Self, PortError> {
        let bytes = json_bytes(&body)?;
        if bytes.len() > MAX_BYTES
            || body.dependencies.is_empty()
            || body.dependencies.len() > 32
            || body.mechanism_version.trim().is_empty()
            || body.mechanism_version.len() > 160
            || !valid_compare_id(&body.release.manifest_sha256)
            || !body.calculation.is_object()
            || !body.render_model.is_object()
            || body.html.is_empty()
            || body.svg.is_empty()
        {
            return Err(invalid());
        }
        let mut keys = BTreeSet::new();
        for dependency in &body.dependencies {
            let key = &dependency.document;
            if !keys.insert((&key.source_id, &key.logical_id))
                || !valid_compare_id(&key.content_hash)
                || !valid_compare_id(&dependency.record_sha256)
                || key.source_id.trim().is_empty()
                || key.logical_id.trim().is_empty()
                || key.revision == 0
            {
                return Err(invalid());
            }
        }
        Ok(Self {
            id: compare_sha256(&bytes),
            body,
        })
    }

    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn body(&self) -> &CompareArtifactBody {
        &self.body
    }

    pub fn check_public(
        &self,
        reader: &dyn SnapshotReadPort,
        principal: &Principal,
    ) -> Result<(), PortError> {
        let manifest = reader.read_manifest_until(&self.body.release.release_id, None)?;
        manifest.validate()?;
        check_release(self, &manifest.release)?;
        let keys: Vec<_> = self
            .body
            .dependencies
            .iter()
            .map(|dependency| dependency.document.clone())
            .collect();
        for key in &keys {
            if !manifest.revisions.iter().any(|descriptor| {
                descriptor.head.source_id == key.source_id
                    && descriptor.head.logical_id == key.logical_id
                    && descriptor.head.revision == key.revision
                    && descriptor.content_hash == key.content_hash
            }) {
                return Err(invalid());
            }
        }
        let records = reader.read_documents_until(&self.body.release.release_id, &keys, None)?;
        let heads = reader.read_heads_until(&keys, None)?;
        check_records(&self.body.dependencies, &records, &heads, principal)
    }
}

pub fn valid_compare_id(id: &str) -> bool {
    id.len() == 64
        && id
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn check_records(
    dependencies: &[CompareDependency],
    records: &[SourceRecordV2],
    heads: &[DocumentHead],
    principal: &Principal,
) -> Result<(), PortError> {
    if records.len() != dependencies.len() || heads.len() != dependencies.len() {
        return Err(invalid());
    }
    for dependency in dependencies {
        let key = &dependency.document;
        let record = records
            .iter()
            .find(|record| record.source_id == key.source_id && record.logical_id == key.logical_id)
            .ok_or_else(invalid)?;
        let head = heads
            .iter()
            .find(|head| head.source_id == key.source_id && head.logical_id == key.logical_id)
            .ok_or_else(invalid)?;
        if compare_dependency(record)? != *dependency
            || record.visibility != SourceVisibility::Public
            || head.visibility != SourceVisibility::Public
            || !record.allowed_scopes.is_empty()
            || !head.allowed_scopes.is_empty()
            || !record_publication_allowed(record, head, principal)
        {
            return Err(invalid());
        }
    }
    Ok(())
}

impl PgStore {
    pub async fn migrate_compare_artifacts(&self) -> Result<(), PortError> {
        self.check_core_schema().await?;
        let mut tx = self.pool.begin().await.map_err(unavailable)?;
        sqlx::raw_sql("SET LOCAL lock_timeout='5000ms'; SET LOCAL statement_timeout='60000ms'; SELECT pg_advisory_xact_lock(742110026112::bigint)").execute(&mut *tx).await.map_err(unavailable)?;
        let owner: bool = sqlx::query_scalar("SELECT pg_has_role(current_user,nspowner,'USAGE') FROM pg_namespace WHERE nspname='brain'").fetch_one(&mut *tx).await.map_err(unavailable)?;
        if !owner {
            return Err(invalid());
        }
        let present: bool =
            sqlx::query_scalar("SELECT to_regclass('brain.compare_artifacts_v1') IS NOT NULL")
                .fetch_one(&mut *tx)
                .await
                .map_err(unavailable)?;
        if !present {
            let body = MIGRATION
                .strip_prefix("BEGIN;\n")
                .and_then(|sql| sql.trim_end().strip_suffix("COMMIT;"))
                .ok_or_else(invalid)?;
            sqlx::raw_sql(body)
                .execute(&mut *tx)
                .await
                .map_err(unavailable)?;
        }
        sqlx::query("SELECT artifact_id,body_json,body_text,publication_receipt FROM brain.compare_artifacts_v1 LIMIT 0").fetch_all(&mut *tx).await.map_err(unavailable)?;
        sqlx::query("SELECT brain.read_compare_artifact_v1(NULL)")
            .execute(&mut *tx)
            .await
            .map_err(unavailable)?;
        tx.commit().await.map_err(unavailable)
    }

    pub async fn save_pending_compare(&self, artifact: &CompareArtifact) -> Result<(), PortError> {
        let body = serde_json::to_string(artifact.body()).map_err(|_| invalid())?;
        let mut tx = self.pool.begin().await.map_err(unavailable)?;
        sqlx::query("INSERT INTO brain.compare_artifacts_v1(artifact_id,body_json,body_text) VALUES($1,$2::text::jsonb,$2) ON CONFLICT(artifact_id) DO NOTHING").bind(artifact.id()).bind(&body).execute(&mut *tx).await.map_err(unavailable)?;
        let stored: String = sqlx::query_scalar(
            "SELECT body_text FROM brain.compare_artifacts_v1 WHERE artifact_id=$1 AND body_json=body_text::jsonb",
        )
        .bind(artifact.id())
        .fetch_one(&mut *tx)
        .await
        .map_err(unavailable)?;
        if stored != body {
            return Err(invalid());
        }
        tx.commit().await.map_err(unavailable)
    }

    pub async fn publish_compare_artifact(
        &self,
        artifact: &CompareArtifact,
        reader: &dyn SnapshotReadPort,
        principal: &Principal,
        verifier: &dyn CompareCalculationVerifier,
    ) -> Result<ComparePublication, PortError> {
        verifier.verify(artifact.body())?;
        artifact.check_public(reader, principal)?;
        let mut tx = self.pool.begin().await.map_err(unavailable)?;
        sqlx::query("SET LOCAL statement_timeout='5000ms'")
            .execute(&mut *tx)
            .await
            .map_err(unavailable)?;
        let release_jsonb_sha256 = check_pg_dependencies(&mut tx, artifact, principal).await?;
        let body = serde_json::to_string(artifact.body()).map_err(|_| invalid())?;
        let receipt = PublicationReceipt {
            artifact_id: artifact.id().into(),
            contract: "brain.compare.publication.v1".into(),
            dependency_sha256: compare_fingerprint(&artifact.body.dependencies)?,
            release_jsonb_sha256,
        };
        let changed = sqlx::query("UPDATE brain.compare_artifacts_v1 SET publication_receipt=$3 WHERE artifact_id=$1 AND body_text=$2 AND body_json=body_text::jsonb")
            .bind(artifact.id()).bind(body).bind(serde_json::to_value(receipt).map_err(|_| invalid())?)
            .execute(&mut *tx).await.map_err(unavailable)?;
        if changed.rows_affected() != 1 {
            return Err(invalid());
        }
        tx.commit().await.map_err(unavailable)?;
        Ok(ComparePublication {
            artifact_id: artifact.id().into(),
            html_path: format!("/site/compare/{}", artifact.id()),
            svg_path: format!("/site/compare/{}/chart.svg", artifact.id()),
        })
    }

    pub async fn read_public_compare(
        &self,
        id: &str,
        principal: &Principal,
    ) -> Result<Option<CompareArtifact>, PortError> {
        if !valid_compare_id(id) {
            return Ok(None);
        }
        let mut tx = self.pool.begin().await.map_err(unavailable)?;
        sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ, READ ONLY")
            .execute(&mut *tx)
            .await
            .map_err(unavailable)?;
        sqlx::query("SET LOCAL statement_timeout='5000ms'")
            .execute(&mut *tx)
            .await
            .map_err(unavailable)?;
        let envelope: Option<serde_json::Value> =
            sqlx::query_scalar("SELECT brain.read_compare_artifact_v1($1)")
                .bind(id)
                .fetch_one(&mut *tx)
                .await
                .map_err(unavailable)?;
        let Some(envelope) = envelope else {
            return Ok(None);
        };
        let receipt: PublicationReceipt =
            serde_json::from_value(envelope["receipt"].clone()).map_err(|_| invalid())?;
        let body_text = envelope["body_text"].as_str().ok_or_else(invalid)?;
        if compare_sha256(body_text.as_bytes()) != id {
            return Err(invalid());
        }
        let artifact =
            CompareArtifact::pending(serde_json::from_str(body_text).map_err(|_| invalid())?)?;
        if artifact.id() != id
            || receipt.artifact_id != id
            || receipt.contract != "brain.compare.publication.v1"
            || !valid_compare_id(&receipt.release_jsonb_sha256)
            || receipt.dependency_sha256 != compare_fingerprint(&artifact.body.dependencies)?
        {
            return Err(invalid());
        }
        let release: CompareReleaseBinding =
            serde_json::from_value(envelope["release"].clone()).map_err(|_| invalid())?;
        if release != artifact.body.release {
            return Err(invalid());
        }
        let records: Vec<SourceRecordV2> =
            serde_json::from_value(envelope["records"].clone()).map_err(|_| invalid())?;
        let descriptors: Vec<brain_contracts::DocumentDescriptor> =
            serde_json::from_value(envelope["heads"].clone()).map_err(|_| invalid())?;
        let heads = public_heads(&records, descriptors)?;
        check_records(&artifact.body.dependencies, &records, &heads, principal)?;
        tx.commit().await.map_err(unavailable)?;
        Ok(Some(artifact))
    }
}

async fn check_pg_dependencies(
    tx: &mut sqlx::PgConnection,
    artifact: &CompareArtifact,
    principal: &Principal,
) -> Result<String, PortError> {
    let (release, release_jsonb_sha256): (serde_json::Value, String) = sqlx::query_as(
        "SELECT release_json,encode(sha256(convert_to(release_json::text,'UTF8')),'hex') FROM brain.corpus_releases_v1 WHERE release_id=$1 FOR SHARE",
    )
    .bind(&artifact.body.release.release_id)
    .fetch_one(&mut *tx)
    .await
    .map_err(unavailable)?;
    let release: CorpusRelease = serde_json::from_value(release).map_err(|_| invalid())?;
    check_release(artifact, &release)?;
    let mut records = Vec::new();
    let mut heads = Vec::new();
    for dependency in &artifact.body.dependencies {
        let key = &dependency.document;
        let revision = i64::try_from(key.revision).map_err(|_| invalid())?;
        let record: serde_json::Value = sqlx::query_scalar("SELECT record_json FROM brain.source_record_revisions WHERE source_id=$1 AND logical_id=$2 AND revision=$3 AND content_hash=$4").bind(&key.source_id).bind(&key.logical_id).bind(revision).bind(&key.content_hash).fetch_one(&mut *tx).await.map_err(unavailable)?;
        records.push(serde_json::from_value(record).map_err(|_| invalid())?);
        let head: serde_json::Value = sqlx::query_scalar("SELECT read_header_json FROM brain.source_record_heads WHERE source_id=$1 AND logical_id=$2 AND content_hash=read_header_json->>'content_hash' AND revision=(read_header_json->'head'->>'revision')::bigint FOR SHARE").bind(&key.source_id).bind(&key.logical_id).fetch_one(&mut *tx).await.map_err(unavailable)?;
        heads.push(
            serde_json::from_value::<brain_contracts::DocumentDescriptor>(head)
                .map_err(|_| invalid())?,
        );
    }
    let heads = public_heads(&records, heads)?;
    check_records(&artifact.body.dependencies, &records, &heads, principal)?;
    Ok(release_jsonb_sha256)
}

fn check_release(artifact: &CompareArtifact, release: &CorpusRelease) -> Result<(), PortError> {
    if CompareReleaseBinding::from_release(release)? != artifact.body.release
        || artifact.body.dependencies.iter().any(|dependency| {
            release
                .source_revisions
                .get(&dependency.document.source_id)
                .and_then(|pins| pins.get(&dependency.document.logical_id))
                != Some(&dependency.document.revision)
        })
    {
        return Err(invalid());
    }
    Ok(())
}

fn public_heads(
    records: &[SourceRecordV2],
    descriptors: Vec<brain_contracts::DocumentDescriptor>,
) -> Result<Vec<DocumentHead>, PortError> {
    for descriptor in &descriptors {
        let record = records
            .iter()
            .find(|record| {
                record.source_id == descriptor.head.source_id
                    && record.logical_id == descriptor.head.logical_id
            })
            .ok_or_else(invalid)?;
        if !valid_compare_id(&descriptor.content_hash)
            || (descriptor.head.revision == record.revision
                && descriptor.content_hash != record.content_hash)
        {
            return Err(invalid());
        }
    }
    Ok(descriptors
        .into_iter()
        .map(|descriptor| descriptor.head)
        .collect())
}
