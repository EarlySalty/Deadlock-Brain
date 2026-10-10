use brain_contracts::{
    source::origin_from_record, tools::ToolValidationPurpose, AuthorizedContext, CorpusSnapshot,
    PortError, SourceRecordV2, SourceVisibility,
};
use brain_storage::entity_profile::PinnedMirrorBundle;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

type Originals = BTreeMap<(String, String), SourceRecordV2>;

pub(crate) fn receipt_allowed(
    receipt: &brain_storage::asset_mirror::AssetDocumentReceipt,
    context: &AuthorizedContext,
    purpose: ToolValidationPurpose,
) -> bool {
    receipt.visibility == SourceVisibility::Public
        && receipt.allowed_scopes.is_subset(&context.principal.scopes)
        && match purpose {
            ToolValidationPurpose::Provider | ToolValidationPurpose::Cache => {
                context.principal.provider_egress.contains("public")
                    && receipt.provenance.provider_egress_authorized
            }
            ToolValidationPurpose::Publication => receipt.provenance.publication_authorized,
        }
}

pub(crate) fn originals(
    snapshot: &CorpusSnapshot,
    context: &AuthorizedContext,
    bundle: &PinnedMirrorBundle,
    kinds: &[&str],
    purpose: ToolValidationPurpose,
) -> Result<Option<Originals>, PortError> {
    let canonical = match purpose {
        ToolValidationPurpose::Provider | ToolValidationPurpose::Cache => {
            snapshot.authorized(&context.principal, true)?
        }
        ToolValidationPurpose::Publication => {
            snapshot.authorized_for_publication(&context.principal)?
        }
    };
    let mut originals = BTreeMap::new();
    let mut missing_original = false;
    for (kind, language) in kinds
        .iter()
        .flat_map(|kind| ["english", "german"].map(|language| (kind, language)))
    {
        context.check_deadline()?;
        let asset = bundle.asset(kind, Some(language))?;
        for (receipt, expected_payload) in [
            (&asset.receipt.manifest, None),
            (&asset.receipt.endpoint, Some(&asset.payload)),
        ] {
            if !receipt_allowed(receipt, context, purpose) {
                missing_original = true;
            }
            let matching: Vec<_> = canonical
                .iter()
                .filter(|record| {
                    origin_from_record(record).is_ok_and(|origin| {
                        origin.identity.source_id == receipt.provenance.source
                            && origin.locator == receipt.url
                            && origin.raw_sha256 == receipt.raw_sha256
                            && origin.source_revision == receipt.provenance.source_revision
                            && origin.parser_revision == receipt.provenance.parser_revision
                            && origin.parser_family == receipt.provenance.parser_family
                            && origin.retrieved_at
                                == brain_contracts::value::Observed::known(
                                    brain_contracts::source::SourceTimestamp::UnixSeconds(
                                        receipt.provenance.observed_at,
                                    ),
                                )
                            && origin.origin_artifacts == receipt.provenance.origin_artifacts
                            && origin.derivation_family
                                == brain_contracts::source::observed_option(
                                    receipt.provenance.derivation_family.clone(),
                                )
                            && origin.schema_version == receipt.schema_version
                            && origin.policy.visibility == receipt.visibility
                            && origin.policy.allowed_scopes == receipt.allowed_scopes
                            && origin.policy.license == receipt.license
                    })
                })
                .collect();
            if matching.is_empty() {
                missing_original = true;
                continue;
            }
            if matching.len() != 1 {
                return Err(PortError::PermissionDenied(
                    "Eindeutige kanonische Freigabe des Spiegeloriginals fehlt".into(),
                ));
            }
            let record = matching[0];
            let original: serde_json::Value =
                serde_json::from_str(&record.content).map_err(|_| {
                    PortError::PermissionDenied("Kanonische Originaldaten fehlen".into())
                })?;
            if format!("{:x}", Sha256::digest(record.content.as_bytes())) != receipt.raw_sha256
                || !expected_payload.map_or_else(
                    || {
                        original["client_version"].as_i64()
                            == Some(bundle.game_context().client_version)
                    },
                    |payload| original == *payload,
                )
            {
                return Err(PortError::PermissionDenied(
                    "Spiegeldaten widersprechen dem kanonischen Original".into(),
                ));
            }
            if record.visibility != SourceVisibility::Public {
                return Err(PortError::PermissionDenied(
                    "Spiegeloriginal hat eine eingeschränkte aktuelle Sichtbarkeit".into(),
                ));
            }
            originals.insert(
                (record.source_id.clone(), record.logical_id.clone()),
                record.clone(),
            );
        }
    }
    Ok((!missing_original).then_some(originals))
}
