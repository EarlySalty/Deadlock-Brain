use std::collections::{BTreeMap, BTreeSet};
use std::io::BufRead;

use brain_contracts::{
    source::{
        GameValidity, OriginArtifact, SourceIdentity, SourcePolicy, SourceRevision, SourceTimestamp,
    },
    value::{Observed, UnknownReason},
    SourceRecordV2, SourceVisibility,
};
use brain_storage::{
    source_versions::{
        StoreRevision, VersionImportError, VersionImportSummary, VersionedSourceRecord,
        DOCUMENT_METADATA_KEY, ORIGINAL_VERSION_KEY,
    },
    PgStore,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::external::SourceIr;
use crate::knowledge_contract::{
    validate_knowledge_jsonl, KnowledgeConflictKind, KnowledgeDocument, KnowledgeSourceKind,
    KnowledgeValidationErrors, ValidatedKnowledgeInput, KNOWLEDGE_CONTRACT_VERSION,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImportGrant {
    pub internal_read_allowed: bool,
    pub raw_retention_allowed: bool,
    pub publication_allowed: bool,
    pub provider_egress_allowed: bool,
    pub authorization_ref: Option<String>,
    pub provenance_evidence_ref: Option<String>,
    pub allowed_scopes: BTreeSet<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImportPolicy {
    pub sources: BTreeMap<String, ImportGrant>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RightsDecision {
    pub source_id: String,
    pub document_id: String,
    pub internal_read_allowed: bool,
    pub raw_retention_allowed: bool,
    pub publication_allowed: bool,
    pub provider_egress_allowed: bool,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct KnowledgeImportSummary {
    pub complete: bool,
    pub storage: VersionImportSummary,
    pub input_documents: usize,
    pub input_facts: usize,
    pub unknown_revisions: usize,
    pub skipped_reasons: BTreeMap<String, usize>,
    pub rights: Vec<RightsDecision>,
}

#[derive(Debug, Error)]
pub enum KnowledgeImportError {
    #[error(transparent)]
    Validation(#[from] KnowledgeValidationErrors),
    #[error(transparent)]
    Storage(#[from] VersionImportError),
    #[error("Ungültiger Wissensimport: {0}")]
    Invalid(String),
}

#[derive(Debug)]
pub struct PreparedKnowledgeImport {
    records: Vec<VersionedSourceRecord>,
    input_documents: usize,
    input_facts: usize,
    unknown_revisions: usize,
    skipped_reasons: BTreeMap<String, usize>,
    rights: Vec<RightsDecision>,
}

impl PreparedKnowledgeImport {
    pub fn records(&self) -> &[VersionedSourceRecord] {
        &self.records
    }

    pub fn rights(&self) -> &[RightsDecision] {
        &self.rights
    }

    pub fn skipped_reasons(&self) -> &BTreeMap<String, usize> {
        &self.skipped_reasons
    }
}

pub fn prepare_knowledge_jsonl(
    reader: impl BufRead,
    policy: &ImportPolicy,
    parser_revision: &str,
) -> Result<PreparedKnowledgeImport, KnowledgeImportError> {
    let input = validate_knowledge_jsonl(reader)?;
    prepare_validated_knowledge(&input, policy, parser_revision)
}

pub fn prepare_validated_knowledge(
    input: &ValidatedKnowledgeInput,
    policy: &ImportPolicy,
    parser_revision: &str,
) -> Result<PreparedKnowledgeImport, KnowledgeImportError> {
    if !valid_ref(parser_revision) {
        return Err(KnowledgeImportError::Invalid("Parserstand fehlt".into()));
    }
    if input.documents().len() > 10_000 {
        return Err(KnowledgeImportError::Invalid(
            "Atomarer Import ist auf 10.000 Dokumentversionen begrenzt".into(),
        ));
    }
    if input
        .conflicts()
        .iter()
        .any(|conflict| conflict.kind == KnowledgeConflictKind::SourceIdentityMismatch)
    {
        return Err(KnowledgeImportError::Invalid(
            "Logische Dokument-ID besitzt widersprüchliche Quellen".into(),
        ));
    }
    let mut prepared = PreparedKnowledgeImport {
        records: Vec::new(),
        input_documents: input.documents().len(),
        input_facts: input
            .documents()
            .iter()
            .map(|record| record.document.facts.len())
            .sum(),
        unknown_revisions: input.unknown_revision_count(),
        skipped_reasons: BTreeMap::new(),
        rights: Vec::new(),
    };
    for located in input.documents() {
        located.document.validate(located.line).map_err(|error| {
            KnowledgeImportError::Validation(KnowledgeValidationErrors {
                errors: vec![error],
            })
        })?;
        let document = &located.document;
        let grant = policy.sources.get(&document.source_id);
        let permitted = grant.is_some_and(|grant| {
            grant.internal_read_allowed
                && grant.raw_retention_allowed
                && grant.authorization_ref.as_deref().is_some_and(valid_ref)
                && grant
                    .provenance_evidence_ref
                    .as_deref()
                    .is_some_and(valid_ref)
                && grant.allowed_scopes.iter().all(|scope| valid_ref(scope))
        });
        let redistribution = document.license.publication_permitted_by_declaration();
        let publication =
            permitted && redistribution && grant.is_some_and(|grant| grant.publication_allowed);
        let egress =
            permitted && redistribution && grant.is_some_and(|grant| grant.provider_egress_allowed);
        prepared.rights.push(RightsDecision {
            source_id: document.source_id.clone(),
            document_id: document.document_id.clone(),
            internal_read_allowed: permitted,
            raw_retention_allowed: permitted,
            publication_allowed: publication,
            provider_egress_allowed: egress,
            reason: if !permitted {
                "operator_internal_read_raw_retention_or_provenance_missing"
            } else if !redistribution {
                "internal_only_license_disallows_redistribution"
            } else {
                "operator_grant_intersected_with_license"
            }
            .into(),
        });
        if !permitted {
            *prepared
                .skipped_reasons
                .entry("operator_internal_read_raw_retention_or_provenance_missing".into())
                .or_default() += 1;
            continue;
        }
        let grant =
            grant.ok_or_else(|| KnowledgeImportError::Invalid("Operatorfreigabe fehlt".into()))?;
        prepared.records.push(prepare_record(
            document,
            grant,
            parser_revision,
            publication,
            egress,
        )?);
    }
    Ok(prepared)
}

pub async fn import_prepared_knowledge(
    store: &PgStore,
    prepared: PreparedKnowledgeImport,
) -> Result<KnowledgeImportSummary, KnowledgeImportError> {
    let storage = store.import_source_versions(&prepared.records).await?;
    Ok(KnowledgeImportSummary {
        complete: storage.committed
            && storage.conflicts.is_empty()
            && prepared.skipped_reasons.is_empty(),
        storage,
        input_documents: prepared.input_documents,
        input_facts: prepared.input_facts,
        unknown_revisions: prepared.unknown_revisions,
        skipped_reasons: prepared.skipped_reasons,
        rights: prepared.rights,
    })
}

fn prepare_record(
    document: &KnowledgeDocument,
    grant: &ImportGrant,
    parser_revision: &str,
    publication: bool,
    egress: bool,
) -> Result<VersionedSourceRecord, KnowledgeImportError> {
    if document.source_kind == KnowledgeSourceKind::Wiki {
        document
            .document_id
            .strip_prefix(&format!("wiki:{}:page:", document.source_id))
            .map(|page| page.parse::<i64>())
            .transpose()
            .map_err(|_| {
                KnowledgeImportError::Invalid(
                    "Wiki-Seiten-ID überschreitet den unterstützten Wertebereich".into(),
                )
            })?;
    }
    let wiki_revision = if document.source_kind == KnowledgeSourceKind::Wiki
        && document.revision.bytes().all(|byte| byte.is_ascii_digit())
    {
        let revision = document.revision.parse::<i64>().map_err(|_| {
            KnowledgeImportError::Invalid(
                "Wiki-Revision überschreitet den unterstützten Wertebereich".into(),
            )
        })?;
        if revision <= 0 {
            return Err(KnowledgeImportError::Invalid(
                "Wiki-Revision muss positiv sein".into(),
            ));
        }
        Some(revision)
    } else {
        None
    };
    let source_revision = SourceRevision::Api {
        api_version: KNOWLEDGE_CONTRACT_VERSION.into(),
        original_revision: Some(document.revision.clone()),
    };
    let revision = wiki_revision.map_or(StoreRevision::LocalMonotonic, |revision| {
        StoreRevision::OriginalWiki(revision as u64)
    });
    let observed = chrono::DateTime::parse_from_rfc3339(&document.observed_at)
        .map_err(|_| KnowledgeImportError::Invalid("Beobachtungszeit ist ungültig".into()))?
        .timestamp();
    let mut ir = SourceIr::from_text(
        &document.source_id,
        &document.source_locator,
        parser_revision,
        source_revision.clone(),
        observed,
        document.content.as_bytes().to_vec(),
    )
    .map_err(|_| {
        KnowledgeImportError::Invalid("Quelltext konnte nicht als SourceIr erhalten werden".into())
    })?;
    ir.pin_schema_version(KNOWLEDGE_CONTRACT_VERSION)
        .map_err(|_| KnowledgeImportError::Invalid("SourceIr-Vertragsstand ist ungültig".into()))?;
    if ir.is_quarantined() || ir.provenance().raw_sha256 != document.content_sha256 {
        return Err(KnowledgeImportError::Invalid(
            "Quelltext ist quarantänisiert oder sein Hash weicht ab".into(),
        ));
    }
    let authorization = grant
        .authorization_ref
        .clone()
        .ok_or_else(|| KnowledgeImportError::Invalid("Operatorfreigabe fehlt".into()))?;
    let mut scopes = grant.allowed_scopes.clone();
    if !publication {
        scopes.insert(format!("source.review:{}", document.source_id));
    }
    let visibility = if publication {
        SourceVisibility::Public
    } else {
        SourceVisibility::Internal
    };
    let origin = OriginArtifact {
        identity: SourceIdentity {
            source_id: document.source_id.clone(),
            logical_id: document.document_id.clone(),
        },
        source_revision,
        raw_sha256: ir.provenance().raw_sha256.clone(),
        locator: document.source_locator.clone(),
        parser_revision: parser_revision.into(),
        parser_family: "dbrain-sources/wiki-spielwissen".into(),
        schema_version: Observed::known(KNOWLEDGE_CONTRACT_VERSION.into()),
        schema_sha256: Observed::unknown(UnknownReason::NotPresent),
        retrieved_at: Observed::known(SourceTimestamp::UnixSeconds(observed)),
        source_time: Observed::unknown(UnknownReason::NotPresent),
        language: if document.language == "und" {
            Observed::unknown(UnknownReason::NotPresent)
        } else {
            Observed::known(document.language.clone())
        },
        origin_artifacts: BTreeSet::from([format!(
            "{}:{}:{}",
            document.source_id, document.document_id, document.revision
        )]),
        derivation_family: Observed::known("wiki-spielwissen-v1".into()),
        policy: SourcePolicy {
            visibility,
            allowed_scopes: scopes.clone(),
            authorization_ref: Observed::known(authorization),
            license: if document
                .license
                .name
                .trim()
                .eq_ignore_ascii_case("unverified")
            {
                Observed::unknown(UnknownReason::NotPresent)
            } else {
                Observed::known(document.license.name.clone())
            },
            publication_allowed: publication,
            provider_egress_allowed: egress,
            raw_retention_allowed: true,
        },
        validity: GameValidity::unknown(),
    };
    let mut canonical = document.clone();
    canonical.facts.sort_by(|a, b| a.fact_id.cmp(&b.fact_id));
    let mut metadata = BTreeMap::from([
        (
            DOCUMENT_METADATA_KEY.into(),
            serde_json::to_string(&canonical).map_err(|_| {
                KnowledgeImportError::Invalid("Dokumentmetadaten sind ungültig".into())
            })?,
        ),
        (ORIGINAL_VERSION_KEY.into(), document.revision.clone()),
        (
            "wiki-spielwissen.revision_kind".into(),
            if wiki_revision.is_some() {
                "original_wiki"
            } else {
                "local_monotonic"
            }
            .into(),
        ),
    ]);
    metadata.insert(
        "wiki-spielwissen.provenance_evidence_ref".into(),
        grant
            .provenance_evidence_ref
            .clone()
            .ok_or_else(|| KnowledgeImportError::Invalid("Herkunftsnachweis fehlt".into()))?,
    );
    let mut record = SourceRecordV2 {
        source_id: document.source_id.clone(),
        logical_id: document.document_id.clone(),
        revision: 1,
        content_hash: document.content_sha256.clone(),
        content: document.content.clone(),
        visibility,
        allowed_scopes: scopes,
        tombstone: false,
        valid_from: None,
        valid_to: None,
        metadata,
    };
    origin
        .bind_record(&mut record)
        .map_err(KnowledgeImportError::Invalid)?;
    Ok(VersionedSourceRecord {
        record,
        original_revision: document.revision.clone(),
        revision,
        fact_count: document.facts.len(),
    })
}

fn valid_ref(value: &str) -> bool {
    !value.trim().is_empty() && !value.chars().any(char::is_control)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::knowledge_contract::{sha256_content, validate_knowledge_jsonl_str};
    use serde_json::json;

    fn input(revision: &str, license: &str, redistribution: bool) -> ValidatedKnowledgeInput {
        let content = "Quellenbeleg";
        let document = json!({
            "contract_version": KNOWLEDGE_CONTRACT_VERSION,
            "source_kind": "wiki",
            "source_id": "fixture",
            "document_id": "wiki:fixture:page:123",
            "source_locator": "https://example.org/Page",
            "title": "Page",
            "language": "und",
            "revision": revision,
            "observed_at": "2026-10-03T12:00:00Z",
            "content_sha256": sha256_content(content),
            "content": content,
            "evidence_status": "source_statement",
            "license": {"name": license, "url": null, "attribution": "Fixture", "redistribution_allowed": redistribution},
            "metadata": {},
            "facts": [{"fact_id": "cooldown", "subject": "hero:test", "predicate": "ability.cooldown", "value": 12, "unit": "seconds", "evidence_status": "hypothesis", "source_span": "Page:cooldown", "qualifiers": {"mode": "unknown"}}]
        });
        validate_knowledge_jsonl_str(&document.to_string()).unwrap()
    }

    fn policy() -> ImportPolicy {
        ImportPolicy {
            sources: BTreeMap::from([(
                "fixture".into(),
                ImportGrant {
                    internal_read_allowed: true,
                    raw_retention_allowed: true,
                    publication_allowed: true,
                    provider_egress_allowed: true,
                    authorization_ref: Some("operator:fixture".into()),
                    provenance_evidence_ref: Some("evidence:fixture".into()),
                    allowed_scopes: BTreeSet::new(),
                },
            )]),
        }
    }

    #[test]
    fn defaults_do_not_import_or_publish() {
        let prepared = prepare_validated_knowledge(
            &input("456", "CC-BY-SA", true),
            &ImportPolicy::default(),
            "parser-v1",
        )
        .unwrap();
        assert!(prepared.records().is_empty());
        assert_eq!(prepared.skipped_reasons().values().sum::<usize>(), 1);
        assert!(!prepared.rights()[0].publication_allowed);
    }

    #[test]
    fn unverified_or_nonredistributable_remains_internal() {
        for (license, redistribution) in [("unverified", true), ("CC-BY-SA", false)] {
            let prepared = prepare_validated_knowledge(
                &input("456", license, redistribution),
                &policy(),
                "parser-v1",
            )
            .unwrap();
            let record = &prepared.records()[0].record;
            let origin = brain_contracts::source::origin_from_record(record).unwrap();
            assert_eq!(record.visibility, SourceVisibility::Internal);
            assert!(!origin.policy.publication_allowed);
            assert!(!origin.policy.provider_egress_allowed);
            assert!(origin.policy.raw_retention_allowed);
            assert!(record.valid_from.is_none());
            assert_eq!(origin.validity, GameValidity::unknown());
            let document: serde_json::Value =
                serde_json::from_str(&record.metadata[DOCUMENT_METADATA_KEY]).unwrap();
            assert_eq!(document["facts"][0]["evidence_status"], "hypothesis");
            assert_eq!(document["facts"][0]["unit"], "seconds");
            assert_eq!(document["facts"][0]["qualifiers"]["mode"], "unknown");
        }
    }

    #[test]
    fn numeric_wiki_uses_actual_page_and_revision() {
        let prepared =
            prepare_validated_knowledge(&input("456", "CC-BY-SA", true), &policy(), "parser-v1")
                .unwrap();
        let version = &prepared.records()[0];
        assert_eq!(version.revision, StoreRevision::OriginalWiki(456));
        assert_eq!(version.record.revision, 1);
        assert_eq!(version.record.logical_id, "wiki:fixture:page:123");
        assert_eq!(version.original_revision, "456");
        assert_eq!(
            brain_contracts::source::origin_from_record(&version.record)
                .unwrap()
                .source_revision,
            SourceRevision::Api {
                api_version: KNOWLEDGE_CONTRACT_VERSION.into(),
                original_revision: Some("456".into())
            }
        );
    }

    #[test]
    fn string_version_is_preserved_and_never_guessed_as_number() {
        let revision = "abcdef0123456789abcdef0123456789abcdef01";
        let prepared = prepare_validated_knowledge(
            &input(revision, "unverified", false),
            &policy(),
            "parser-v1",
        )
        .unwrap();
        let version = &prepared.records()[0];
        assert_eq!(version.revision, StoreRevision::LocalMonotonic);
        assert_eq!(version.original_revision, revision);
        assert_eq!(
            brain_contracts::source::origin_from_record(&version.record)
                .unwrap()
                .source_revision,
            SourceRevision::Api {
                api_version: KNOWLEDGE_CONTRACT_VERSION.into(),
                original_revision: Some(revision.into())
            }
        );
    }

    fn url_input(revision: &str, content: &str, observed_at: &str) -> ValidatedKnowledgeInput {
        let mut document = input(revision, "unverified", false).documents()[0]
            .document
            .clone();
        document.document_id = format!(
            "wiki:fixture:url:{}",
            sha256_content(&document.source_locator)
        );
        document.content = content.into();
        document.content_sha256 = sha256_content(content);
        document.observed_at = observed_at.into();
        validate_knowledge_jsonl_str(&serde_json::to_string(&document).unwrap()).unwrap()
    }

    #[test]
    fn numeric_url_wiki_preserves_order_without_inventing_page_identity() {
        for (revision, content) in [
            ("456", "Aktueller Inhalt"),
            ("123", "Älterer Inhalt"),
            ("456", "Aktueller Inhalt"),
        ] {
            let prepared = prepare_validated_knowledge(
                &url_input(revision, content, "2026-10-03T12:00:00Z"),
                &policy(),
                "parser-v1",
            )
            .unwrap();
            let version = &prepared.records()[0];
            assert_eq!(
                version.revision,
                StoreRevision::OriginalWiki(revision.parse().unwrap())
            );
            assert_eq!(version.record.revision, 1);
            assert_eq!(
                version.record.metadata["wiki-spielwissen.revision_kind"],
                "original_wiki"
            );
            let origin = brain_contracts::source::origin_from_record(&version.record).unwrap();
            assert_eq!(
                origin.source_revision,
                SourceRevision::Api {
                    api_version: KNOWLEDGE_CONTRACT_VERSION.into(),
                    original_revision: Some(revision.into()),
                }
            );
            assert_eq!(origin.identity.logical_id, version.record.logical_id);
            let document: serde_json::Value =
                serde_json::from_str(&version.record.metadata[DOCUMENT_METADATA_KEY]).unwrap();
            assert!(document["metadata"].get("page_id").is_none());
            assert_eq!(document["revision"], revision);
            assert_eq!(document["content"], content);
        }
    }

    #[test]
    fn numeric_wiki_aliases_keep_original_text_but_share_numeric_identity() {
        for revision in ["7", "07", "0007"] {
            for url_identity in [false, true] {
                let input = if url_identity {
                    url_input(revision, "Quellenbeleg", "2026-10-03T12:00:00Z")
                } else {
                    input(revision, "unverified", false)
                };
                let prepared = prepare_validated_knowledge(&input, &policy(), "parser-v1").unwrap();
                let version = &prepared.records()[0];
                assert_eq!(version.revision, StoreRevision::OriginalWiki(7));
                assert_eq!(version.original_revision, revision);
                assert_eq!(version.record.metadata[ORIGINAL_VERSION_KEY], revision);
                let document: serde_json::Value =
                    serde_json::from_str(&version.record.metadata[DOCUMENT_METADATA_KEY]).unwrap();
                assert_eq!(document["revision"], revision);
            }
        }
    }

    #[test]
    fn url_wiki_git_and_unknown_versions_remain_locally_monotonic() {
        for revision in [
            "abcdef0123456789abcdef0123456789abcdef01".to_string(),
            format!("unknown:{}", sha256_content("Quellenbeleg")),
        ] {
            let prepared = prepare_validated_knowledge(
                &url_input(&revision, "Quellenbeleg", "2026-10-03T12:00:00Z"),
                &policy(),
                "parser-v1",
            )
            .unwrap();
            let version = &prepared.records()[0];
            assert_eq!(version.revision, StoreRevision::LocalMonotonic);
            assert_eq!(version.record.revision, 1);
            assert_eq!(version.original_revision, revision);
            assert_eq!(
                brain_contracts::source::origin_from_record(&version.record)
                    .unwrap()
                    .source_revision,
                SourceRevision::Api {
                    api_version: KNOWLEDGE_CONTRACT_VERSION.into(),
                    original_revision: Some(revision)
                }
            );
        }
    }

    #[test]
    fn numeric_game_file_version_remains_locally_monotonic() {
        let mut document = input("456", "unverified", false).documents()[0]
            .document
            .clone();
        document.source_kind = KnowledgeSourceKind::GameFile;
        document.document_id = "game:1422450:scripts/abilities.txt".into();
        document.source_locator = "scripts/abilities.txt".into();
        let input =
            validate_knowledge_jsonl_str(&serde_json::to_string(&document).unwrap()).unwrap();
        let prepared = prepare_validated_knowledge(&input, &policy(), "parser-v1").unwrap();
        let version = &prepared.records()[0];
        assert_eq!(version.revision, StoreRevision::LocalMonotonic);
        assert_eq!(version.record.revision, 1);
        assert_eq!(version.original_revision, "456");
    }

    #[test]
    fn numeric_wiki_bounds_apply_to_page_and_url_identities() {
        let maximum = i64::MAX.to_string();
        for url in [false, true] {
            let make_input = |revision: &str| {
                if url {
                    url_input(revision, "Quellenbeleg", "2026-10-03T12:00:00Z")
                } else {
                    input(revision, "unverified", false)
                }
            };
            let prepared =
                prepare_validated_knowledge(&make_input(&maximum), &policy(), "parser-v1").unwrap();
            assert_eq!(
                prepared.records()[0].revision,
                StoreRevision::OriginalWiki(i64::MAX as u64)
            );
            for revision in ["0", "9223372036854775808", "18446744073709551615"] {
                assert!(
                    prepare_validated_knowledge(&make_input(revision), &policy(), "parser-v1")
                        .is_err()
                );
            }
        }
    }

    #[test]
    fn missing_provenance_evidence_denies_internal_storage() {
        let mut policy = policy();
        policy
            .sources
            .get_mut("fixture")
            .unwrap()
            .provenance_evidence_ref = None;
        let prepared =
            prepare_validated_knowledge(&input("456", "CC-BY-SA", true), &policy, "parser-v1")
                .unwrap();
        assert!(prepared.records().is_empty());
    }
}
