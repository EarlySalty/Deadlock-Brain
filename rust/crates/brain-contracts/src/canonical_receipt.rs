use crate::{Evidence, EvidenceKind, PortError, SourceVisibility};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

pub const RECEIPT_ID_PREFIX: &str = "gamefacts.receipt.v1:sha256:";
const HASH_DOMAIN: &[u8] = b"brain.gamefacts.receipt.v1\0";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalReceipt {
    pub raw_receipt_id: i64,
    pub raw_object_id: i64,
    pub ingest_run_id: i64,
    pub source: String,
    pub source_revision: String,
    pub path: String,
    pub locator_kind: String,
    pub locator: String,
    pub value_sha256: String,
    pub extractor: String,
    pub patch: Option<String>,
}

fn invalid() -> PortError {
    PortError::InvalidResponse("Kanonischer Spielbeleg ist ungültig.".into())
}

pub fn format_receipt(receipt: &CanonicalReceipt) -> Result<Evidence, PortError> {
    if receipt.raw_receipt_id <= 0
        || receipt.raw_object_id <= 0
        || receipt.ingest_run_id <= 0
        || receipt.patch.is_some()
        || receipt.source.len() + "canon.source/".len() > 512
        || receipt.source.chars().any(char::is_control)
        || [
            &receipt.source,
            &receipt.source_revision,
            &receipt.path,
            &receipt.locator_kind,
            &receipt.extractor,
        ]
        .into_iter()
        .any(|text| text.trim().is_empty() || text.contains('\0'))
        || receipt.locator.contains('\0')
        || receipt.value_sha256.len() != 64
        || !receipt
            .value_sha256
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        return Err(invalid());
    }
    let content = serde_json::to_string(receipt).map_err(|_| invalid())?;
    let mut hash = Sha256::new();
    hash.update(HASH_DOMAIN);
    hash.update(content.as_bytes());
    let evidence_id = format!("{RECEIPT_ID_PREFIX}{:x}", hash.finalize());
    Ok(Evidence {
        citation: evidence_id.clone(),
        evidence_id,
        source_id: format!("canon.source/{}", receipt.source),
        logical_id: format!("receipt/{}", receipt.raw_receipt_id),
        revision: receipt.ingest_run_id as u64,
        kind: EvidenceKind::Fact,
        content,
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::new(),
        score: 1.0,
        provenance: None,
        patch: None,
    })
}

impl CanonicalReceipt {
    pub fn from_evidence(evidence: &Evidence) -> Result<Self, PortError> {
        if !evidence.evidence_id.starts_with(RECEIPT_ID_PREFIX) {
            return Err(invalid());
        }
        let receipt: Self = serde_json::from_str(&evidence.content).map_err(|_| invalid())?;
        if format_receipt(&receipt)? != *evidence {
            return Err(invalid());
        }
        Ok(receipt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn receipt() -> CanonicalReceipt {
        CanonicalReceipt {
            raw_receipt_id: 42,
            raw_object_id: 7,
            ingest_run_id: 3,
            source: "fixture".into(),
            source_revision: "r1".into(),
            path: "hero.json".into(),
            locator_kind: "json_pointer".into(),
            locator: "/name".into(),
            value_sha256: "ab".repeat(32),
            extractor: "fixture-v1".into(),
            patch: None,
        }
    }

    #[test]
    fn golden_receipt_is_byte_stable_and_tool_independent() {
        let input = receipt();
        let evidence = format_receipt(&input).unwrap();
        assert_eq!(evidence.content, concat!(
            "{\"raw_receipt_id\":42,\"raw_object_id\":7,\"ingest_run_id\":3,",
            "\"source\":\"fixture\",\"source_revision\":\"r1\",\"path\":\"hero.json\",",
            "\"locator_kind\":\"json_pointer\",\"locator\":\"/name\",",
            "\"value_sha256\":\"abababababababababababababababababababababababababababababababab\",",
            "\"extractor\":\"fixture-v1\",\"patch\":null}"
        ));
        assert_eq!(evidence.evidence_id, "gamefacts.receipt.v1:sha256:60501a49e7739dbdc631ecccc54668862c87e1eaf0872a0ff9d1f1e1bbb0e239");
        assert_eq!(evidence.citation, evidence.evidence_id);
        assert_eq!(evidence.source_id, "canon.source/fixture");
        assert_eq!(evidence.logical_id, "receipt/42");
        assert_eq!(evidence.revision, 3);
        assert!(evidence.evidence_id.len() <= 512);
        assert_eq!(format_receipt(&input).unwrap(), evidence);
        let decoded = CanonicalReceipt::from_evidence(&evidence).unwrap();
        assert_eq!(decoded, input);
        assert_eq!(format_receipt(&decoded).unwrap(), evidence);
        assert_eq!(
            serde_json::to_vec(&format_receipt(&decoded).unwrap()).unwrap(),
            serde_json::to_vec(&evidence).unwrap()
        );
    }

    #[test]
    fn every_authoritative_field_changes_the_content_address() {
        let original = format_receipt(&receipt()).unwrap();
        for field in [
            "raw_receipt_id",
            "raw_object_id",
            "ingest_run_id",
            "source",
            "source_revision",
            "path",
            "locator_kind",
            "locator",
            "value_sha256",
            "extractor",
        ] {
            let mut value = serde_json::to_value(receipt()).unwrap();
            value[field] = match field {
                "raw_receipt_id" | "raw_object_id" | "ingest_run_id" => serde_json::json!(99),
                "value_sha256" => serde_json::json!("cd".repeat(32)),
                _ => serde_json::json!("changed"),
            };
            let changed: CanonicalReceipt = serde_json::from_value(value).unwrap();
            let evidence = format_receipt(&changed).unwrap();
            assert_ne!(original.content, evidence.content, "{field}");
            assert_ne!(original.evidence_id, evidence.evidence_id, "{field}");
        }
    }

    #[test]
    fn metadata_resolution_rejects_noncanonical_evidence_and_official_patch_claims() {
        let input = receipt();
        let evidence = format_receipt(&input).unwrap();
        for field in [
            "evidence_id",
            "source_id",
            "logical_id",
            "content",
            "citation",
        ] {
            let mut value = serde_json::to_value(&evidence).unwrap();
            value[field] = serde_json::json!("changed");
            let changed: Evidence = serde_json::from_value(value).unwrap();
            assert!(CanonicalReceipt::from_evidence(&changed).is_err());
        }
        let mut changed = evidence.clone();
        changed.score = 0.5;
        assert!(CanonicalReceipt::from_evidence(&changed).is_err());
        let mut changed = input.clone();
        changed.patch = Some("official".into());
        assert!(format_receipt(&changed).is_err());
        let mut changed = input.clone();
        changed.value_sha256 = "AB".repeat(32);
        assert!(format_receipt(&changed).is_err());
        let mut changed = input;
        changed.raw_receipt_id = 0;
        assert!(format_receipt(&changed).is_err());
    }
}
