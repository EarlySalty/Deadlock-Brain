use crate::{
    invalid, pg::LegacyRead, prepare_batch, sha256_hex, ImportContext, LegacySource, Result,
    SourcePolicyConfig, ENTITIES_SOURCE, PATCHNOTES_SOURCE,
};
use brain_contracts::{SourceRecordV2, SourceVisibility};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CutoverBinding {
    pub approval_ref: String,
    pub database_oid: i64,
    pub archive_schema_oid: i64,
    pub core_schema_oid: i64,
    pub schema_sha256: String,
    pub snapshot_sha256: String,
    pub policy_sha256: String,
    pub table_counts: BTreeMap<String, i64>,
    pub active_ids: BTreeMap<String, BTreeSet<String>>,
    pub revoked_ids: BTreeMap<String, BTreeSet<String>>,
    pub tombstone_ids: BTreeMap<String, BTreeSet<String>>,
}

fn valid_hash(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn exact_sources<T>(values: &BTreeMap<String, T>) -> bool {
    values
        .keys()
        .map(String::as_str)
        .eq([ENTITIES_SOURCE, PATCHNOTES_SOURCE])
}

pub fn snapshot_sha256(
    sources: &[LegacySource; 2],
    read: &LegacyRead,
    label: &str,
    epoch: i64,
) -> Result<String> {
    Ok(sha256_hex(&serde_json::to_vec(&(
        label,
        epoch,
        &read.schema_sha256,
        &read.table_counts,
        &read.entities,
        &read.patch_lines,
        sources,
    ))?))
}

impl CutoverBinding {
    pub fn policy_sha256(&self, policies: &BTreeMap<String, SourcePolicyConfig>) -> Result<String> {
        Ok(sha256_hex(&serde_json::to_vec(&(
            &self.approval_ref,
            policies,
            &self.active_ids,
            &self.revoked_ids,
            &self.tombstone_ids,
        ))?))
    }

    pub fn verify_sources(
        &self,
        sources: &mut [LegacySource; 2],
        read: &LegacyRead,
        label: &str,
        epoch: i64,
        policies: &BTreeMap<String, SourcePolicyConfig>,
    ) -> Result<()> {
        if !self.approval_ref.starts_with("sha256:")
            || !valid_hash(&self.approval_ref[7..])
            || self.database_oid <= 0
            || self.archive_schema_oid <= 0
            || self.core_schema_oid <= 0
            || self.archive_schema_oid == self.core_schema_oid
            || !valid_hash(&self.schema_sha256)
            || !valid_hash(&self.snapshot_sha256)
            || !valid_hash(&self.policy_sha256)
            || !exact_sources(policies)
            || !exact_sources(&self.active_ids)
            || !exact_sources(&self.revoked_ids)
            || !exact_sources(&self.tombstone_ids)
            || label.trim().is_empty()
            || epoch <= 0
        {
            return Err(invalid(
                "cutover approval, snapshot or source inventory missing",
            ));
        }
        let patch_policy = &policies[PATCHNOTES_SOURCE];
        if patch_policy.visibility != SourceVisibility::Private
            || patch_policy.allowed_scopes != BTreeSet::from(["brain.legacy.review".to_string()])
        {
            return Err(invalid(
                "patchnotes must remain private to their review scope",
            ));
        }
        for policy in policies.values() {
            policy.validate()?;
            if policy.provider_egress_allowed
                || policy.publication_allowed
                || policy.authorization_ref.as_deref() != Some(self.approval_ref.as_str())
            {
                return Err(invalid(
                    "cutover source policy lacks approved private boundaries",
                ));
            }
        }
        if self.schema_sha256 != read.schema_sha256
            || self.table_counts != read.table_counts
            || self.snapshot_sha256 != snapshot_sha256(sources, read, label, epoch)?
            || self.policy_sha256 != self.policy_sha256(policies)?
        {
            return Err(invalid("cutover snapshot or policy baseline changed"));
        }
        for source in sources {
            let active = &self.active_ids[source.source_id];
            let revoked = &self.revoked_ids[source.source_id];
            let tombstones = &self.tombstone_ids[source.source_id];
            if active.is_empty()
                || !active.is_disjoint(revoked)
                || !active.is_disjoint(tombstones)
                || !revoked.is_disjoint(tombstones)
                || active
                    .iter()
                    .chain(revoked)
                    .chain(tombstones)
                    .any(|id| id.trim().is_empty() || id.chars().any(char::is_control))
            {
                return Err(invalid(
                    "cutover inventory contains conflicting document states",
                ));
            }
            let observed: BTreeSet<_> = source
                .documents
                .iter()
                .map(|d| d.logical_id.clone())
                .collect();
            let represented: BTreeSet<_> = active.union(revoked).cloned().collect();
            let represented: BTreeSet<_> = represented.union(tombstones).cloned().collect();
            if observed.len() != source.documents.len()
                || !active.is_subset(&observed)
                || !revoked.is_subset(&observed)
                || observed
                    != represented
                        .intersection(&observed)
                        .cloned()
                        .collect::<BTreeSet<_>>()
            {
                return Err(invalid(
                    "cutover inventory does not cover archive documents",
                ));
            }
            source
                .documents
                .retain(|document| active.contains(&document.logical_id));
        }
        Ok(())
    }

    pub fn verify_head(
        &self,
        record: &SourceRecordV2,
        sources: &[LegacySource; 2],
        policies: &BTreeMap<String, SourcePolicyConfig>,
        context: &ImportContext,
    ) -> Result<()> {
        let source = sources
            .iter()
            .find(|source| source.source_id == record.source_id)
            .ok_or_else(|| invalid("unexpected cutover source head"))?;
        if self.revoked_ids[source.source_id].contains(&record.logical_id)
            || self.tombstone_ids[source.source_id].contains(&record.logical_id)
        {
            return if record.tombstone {
                Ok(())
            } else {
                Err(invalid("cutover would retain a revoked or deleted head"))
            };
        }
        let document = source
            .documents
            .iter()
            .find(|document| document.logical_id == record.logical_id)
            .ok_or_else(|| invalid("unapproved cutover target head"))?;
        let policy = &policies[source.source_id];
        let expected_source = LegacySource {
            source_id: source.source_id,
            documents: vec![document.clone()],
        };
        let batch = prepare_batch(&expected_source, policy, context, None)?;
        let mut expected =
            batch.records.into_iter().next().ok_or_else(|| {
                invalid("approved cutover document did not produce a target record")
            })?;
        expected.revision = record.revision;
        if record != &expected {
            return Err(invalid(
                "cutover target head differs from approved snapshot or rights",
            ));
        }
        Ok(())
    }
}
