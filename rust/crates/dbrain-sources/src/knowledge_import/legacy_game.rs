use crate::knowledge_contract::{
    sha256_content, KnowledgeDocument, KnowledgeEvidenceStatus, KnowledgeLicense,
    KnowledgeSourceKind, KNOWLEDGE_CONTRACT_VERSION,
};
use brain_contracts::SourceRecordV2;
use brain_storage::source_versions::DOCUMENT_METADATA_KEY;
use serde::Serialize;
use serde_json::{json, Value};
use sqlx::{PgConnection, PgPool, Row};
use std::{collections::BTreeMap, io::Write};

pub const SNAPSHOT_KEY: &str = "legacy_game_snapshot";
pub const SNAPSHOT_VERSION: &str = "legacy-game-snapshot-v1";

fn allowed(source: &str, kind: &str, external: &str, target: &str) -> bool {
    match target {
        "legacy-entities" => {
            matches!(source, "deadlock_data" | "deadlock_assets_api")
                && (matches!(
                    kind,
                    "hero"
                        | "hero_card"
                        | "ability"
                        | "ability_card"
                        | "item"
                        | "item_card"
                        | "npc_unit"
                ) || (source == "deadlock_data"
                    && kind == "localization"
                    && matches!(external, "english" | "german")))
        }
        "legacy-patchnotes" => {
            matches!(source, "deadlock_data" | "deadlock_patchnotes_db") && kind == "patchnote"
        }
        _ => false,
    }
}

fn official_patch_url(url: &str) -> bool {
    url.starts_with("https://forums.playdeadlock.com/threads/")
        || url.starts_with("https://store.steampowered.com/news/app/1422450/")
        || url.starts_with("https://steamcommunity.com/games/1422450/announcements/detail/")
}

fn original_admission(
    upstream: &str,
    source_document_id: Option<i64>,
    document_source: Option<&str>,
) -> Result<Option<&'static str>, String> {
    match (source_document_id, document_source) {
        (None, None) => Ok(Some("original_document_binding_missing")),
        (Some(id), None) if id > 0 => Ok(Some("original_document_missing")),
        (Some(id), Some(source)) if id > 0 && source == upstream => Ok(None),
        _ => Err("Snapshot besitzt eine widersprüchliche Originalquellenbindung".into()),
    }
}

#[derive(Debug, Serialize)]
pub struct LegacyGameExclusion {
    pub snapshot_id: i64,
    pub reason: String,
}

#[derive(Debug, Default, Serialize)]
pub struct LegacyGameExport {
    pub documents: usize,
    pub considered: usize,
    pub excluded: usize,
    pub exclusion_reasons: BTreeMap<String, usize>,
    pub excluded_snapshots: Vec<LegacyGameExclusion>,
}

impl LegacyGameExport {
    fn exclude(&mut self, id: i64, reason: String) {
        self.excluded += 1;
        *self.exclusion_reasons.entry(reason.clone()).or_default() += 1;
        self.excluded_snapshots.push(LegacyGameExclusion {
            snapshot_id: id,
            reason,
        });
    }
}

pub async fn export_legacy_game(
    pool: &PgPool,
    source: &str,
    output: &mut impl Write,
) -> Result<LegacyGameExport, String> {
    let (upstreams, kinds) = match source {
        "legacy-entities" => (
            vec!["deadlock_data", "deadlock_assets_api"],
            vec![
                "hero",
                "hero_card",
                "ability",
                "ability_card",
                "item",
                "item_card",
                "npc_unit",
                "localization",
            ],
        ),
        "legacy-patchnotes" => (
            vec!["deadlock_data", "deadlock_patchnotes_db"],
            vec!["patchnote"],
        ),
        _ => {
            return Err(
                "Legacy-Export ist auf öffentliche Spielentitäten und Patchnotes begrenzt".into(),
            )
        }
    };
    let mut tx = pool
        .begin()
        .await
        .map_err(|_| "Legacy-Lesetransaktion kann nicht starten")?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ, READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(|_| "Konsistenter privater Snapshot kann nicht geöffnet werden")?;
    let rows = sqlx::query("SELECT DISTINCT ON (e.source,e.entity_type,e.external_id) e.id,e.source,e.entity_type,e.external_id,e.source_document_id,d.source AS document_source,d.url FROM brain.entity_snapshots e LEFT JOIN brain.source_documents d ON d.id=e.source_document_id WHERE e.source=ANY($1) AND e.entity_type=ANY($2) AND (e.entity_type<>'localization' OR (e.source='deadlock_data' AND e.external_id IN ('english','german'))) ORDER BY e.source,e.entity_type,e.external_id,e.fetched_at DESC,e.id DESC LIMIT 10001")
        .bind(&upstreams).bind(&kinds).fetch_all(&mut *tx).await.map_err(|_| "Spiel-Snapshot-Metadaten können nicht gelesen werden")?;
    if rows.len() > 10_000 {
        return Err("Legacy-Spielbestand überschreitet 10.000 Dokumente".into());
    }
    let mut report = LegacyGameExport {
        considered: rows.len(),
        ..Default::default()
    };
    let excluded = sqlx::query("SELECT entity_type,count(DISTINCT (source,external_id)) AS documents FROM brain.entity_snapshots WHERE source=ANY($1) AND (entity_type<>ALL($2) OR (entity_type='localization' AND NOT (source='deadlock_data' AND external_id IN ('english','german')))) GROUP BY entity_type")
        .bind(&upstreams).bind(&kinds).fetch_all(&mut *tx).await.map_err(|_| "Ausgeschlossene Kategorien können nicht gezählt werden")?;
    for category in excluded {
        let kind: String = category
            .try_get("entity_type")
            .map_err(|_| "Ausgeschlossene Kategorie fehlt")?;
        let count: i64 = category
            .try_get("documents")
            .map_err(|_| "Kategoriezahl fehlt")?;
        let count = usize::try_from(count).map_err(|_| "Kategoriezahl ist ungültig")?;
        report.considered = report
            .considered
            .checked_add(count)
            .ok_or("Kategoriezahl ist zu groß")?;
        report.excluded = report
            .excluded
            .checked_add(count)
            .ok_or("Kategoriezahl ist zu groß")?;
        report
            .exclusion_reasons
            .insert(format!("game_category_not_admitted:{kind}"), count);
    }
    for row in rows {
        let id: i64 = row.try_get("id").map_err(|_| "Snapshot-ID fehlt")?;
        let upstream: String = row.try_get("source").map_err(|_| "Snapshot-Quelle fehlt")?;
        let kind: String = row
            .try_get("entity_type")
            .map_err(|_| "Snapshot-Typ fehlt")?;
        let external: String = row
            .try_get("external_id")
            .map_err(|_| "Snapshot-Identität fehlt")?;
        let document_source: Option<String> = row
            .try_get("document_source")
            .map_err(|_| "Originalquelle fehlt")?;
        if !allowed(&upstream, &kind, &external, source) {
            return Err("Snapshot hat eine fremde Spielherkunft".into());
        }
        let original_document_id: Option<i64> = row
            .try_get("source_document_id")
            .map_err(|_| "Originalbindung ist ungültig")?;
        if let Some(reason) =
            original_admission(&upstream, original_document_id, document_source.as_deref())?
        {
            report.exclude(id, reason.into());
            continue;
        }
        let url: Option<String> = row.try_get("url").map_err(|_| "Originaladresse fehlt")?;
        if source == "legacy-patchnotes" && !url.as_deref().is_some_and(official_patch_url) {
            report.exclude(id, "official_public_patch_url_missing".into());
            continue;
        }
        let payload_row = sqlx::query("SELECT e.canonical_name,e.payload_hash,e.payload,e.fetched_at,e.source_document_id,d.content_hash AS document_hash,d.metadata FROM brain.entity_snapshots e JOIN brain.source_documents d ON d.id=e.source_document_id WHERE e.id=$1")
            .bind(id).fetch_one(&mut *tx).await.map_err(|_| "Ausgewählter Spiel-Snapshot kann nicht gelesen werden")?;
        let payload: Value = payload_row
            .try_get("payload")
            .map_err(|_| "Snapshot-Nutzlast fehlt")?;
        let payload_hash: String = payload_row
            .try_get("payload_hash")
            .map_err(|_| "Snapshot-Hash fehlt")?;
        let content = crate::store::json_string(&payload)
            .map_err(|_| "Snapshot kann nicht originaltreu serialisiert werden")?;
        if sha256_content(&content) != payload_hash {
            return Err("Snapshot-Hash widerspricht seiner Nutzlast".into());
        }
        let observed: chrono::DateTime<chrono::Utc> = payload_row
            .try_get("fetched_at")
            .map_err(|_| "Snapshot-Beobachtungszeit fehlt")?;
        let source_document_id: i64 = payload_row
            .try_get("source_document_id")
            .map_err(|_| "Originalbindung fehlt")?;
        let document_hash: String = payload_row
            .try_get("document_hash")
            .map_err(|_| "Originalhash fehlt")?;
        let source_metadata: Value = payload_row
            .try_get("metadata")
            .map_err(|_| "Originalmetadaten fehlen")?;
        if id <= 0
            || source_document_id <= 0
            || Some(source_document_id) != original_document_id
            || !source_metadata.is_object()
            || document_hash.len() != 64
            || !document_hash
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err("Spiel-Snapshot besitzt keine gültige Originalbindung".into());
        }
        let path = format!("legacy/{upstream}/{kind}/{}", sha256_content(&external));
        let (facts, parse_status) =
            brain_storage::entity_profile::derivation::game_file_facts::extract_facts(
                &format!("{path}.json"),
                &content,
            );
        if facts.is_empty() {
            report.exclude(
                id,
                format!("public_game_facts_empty_or_unparsed:{parse_status}"),
            );
            continue;
        }
        let facts = serde_json::from_value(Value::Array(facts))
            .map_err(|_| "Legacy-Fakten verletzen den Wissensvertrag")?;
        let document = KnowledgeDocument {
            contract_version: KNOWLEDGE_CONTRACT_VERSION.into(),
            source_kind: KnowledgeSourceKind::GameFile,
            source_id: source.into(),
            document_id: format!("game:1422450:{path}"),
            source_locator: format!("brain.entity_snapshots/{id}"),
            title: payload_row.try_get::<Option<String>, _>("canonical_name").map_err(|_| "Snapshot-Name ist ungültig")?
                .filter(|value| !value.trim().is_empty()).unwrap_or_else(|| format!("{kind}:{external}")),
            language: if kind == "localization" { if external == "german" { "de" } else { "en" } } else { "und" }.into(),
            revision: format!("legacy-snapshot:{id}:{payload_hash}"),
            observed_at: observed.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            content_sha256: payload_hash.clone(),
            content,
            evidence_status: KnowledgeEvidenceStatus::ExtractedValue,
            license: KnowledgeLicense {
                name: "unverified".into(), url: None,
                attribution: format!("{upstream}; Valve game data; original rights declaration retained in source metadata"),
                redistribution_allowed: false,
            },
            metadata: serde_json::from_value(json!({"app_id": 1422450, "relative_path": path,
                "original_source_document_metadata": source_metadata,
                (SNAPSHOT_KEY): {"contract_version": SNAPSHOT_VERSION, "snapshot_id": id,
                    "upstream_source": upstream, "entity_type": kind, "external_id": external,
                    "payload_hash": payload_hash, "source_document_id": source_document_id,
                    "original_document_sha256": document_hash, "original_url": url}}))
                .map_err(|_| "Snapshot-Herkunft ist ungültig")?,
            facts,
        };
        document
            .validate(1)
            .map_err(|_| "Legacy-Dokument verletzt den Wissensvertrag")?;
        serde_json::to_writer(&mut *output, &document)
            .map_err(|_| "Legacy-Dokument kann nicht serialisiert werden")?;
        output
            .write_all(b"\n")
            .map_err(|_| "Legacy-Dokument kann nicht geschrieben werden")?;
        report.documents += 1;
    }
    tx.commit()
        .await
        .map_err(|_| "Legacy-Lesesnapshot wurde nicht vollständig bestätigt")?;
    Ok(report)
}

pub fn validate_snapshot_record(
    record: &SourceRecordV2,
    document: &KnowledgeDocument,
) -> Result<(), String> {
    let binding = document
        .metadata
        .get(SNAPSHOT_KEY)
        .ok_or("Spiel-Snapshot-Bindung fehlt")?;
    let source = binding["upstream_source"]
        .as_str()
        .ok_or("Snapshot-Quelle fehlt")?;
    let kind = binding["entity_type"]
        .as_str()
        .ok_or("Snapshot-Typ fehlt")?;
    let external = binding["external_id"]
        .as_str()
        .ok_or("Snapshot-Identität fehlt")?;
    let id = binding["snapshot_id"]
        .as_i64()
        .filter(|value| *value > 0)
        .ok_or("Snapshot-ID ist ungültig")?;
    let path = format!("legacy/{source}/{kind}/{}", sha256_content(external));
    if binding["contract_version"] != SNAPSHOT_VERSION
        || !allowed(source, kind, external, &record.source_id)
        || document.source_kind != KnowledgeSourceKind::GameFile
        || document.source_id != record.source_id
        || document.document_id != record.logical_id
        || record.logical_id != format!("game:1422450:{path}")
        || document.content != record.content
        || document.content_sha256 != record.content_hash
        || binding["payload_hash"].as_str() != Some(record.content_hash.as_str())
        || document.revision != format!("legacy-snapshot:{id}:{}", record.content_hash)
        || document.source_locator != format!("brain.entity_snapshots/{id}")
        || document.metadata.get("app_id") != Some(&json!(1422450))
        || (record.source_id == "legacy-patchnotes"
            && !binding["original_url"]
                .as_str()
                .is_some_and(official_patch_url))
    {
        return Err("Snapshot-Bindung belegt keine öffentliche Spielquelle".into());
    }
    if !document.facts.is_empty() {
        let (facts, _) = brain_storage::entity_profile::derivation::game_file_facts::extract_facts(
            &format!("{path}.json"),
            &record.content,
        );
        if serde_json::to_value(&document.facts).map_err(|_| "Legacy-Fakten sind ungültig")?
            != Value::Array(facts)
        {
            return Err("Legacy-Fakten widersprechen dem gebundenen Spieloriginal".into());
        }
    }
    Ok(())
}

pub async fn verify_snapshot_record(
    connection: &mut PgConnection,
    record: &SourceRecordV2,
) -> Result<(), String> {
    let Some(encoded) = record.metadata.get(DOCUMENT_METADATA_KEY) else {
        return Ok(());
    };
    let document: KnowledgeDocument =
        serde_json::from_str(encoded).map_err(|_| "Originaldokument ist ungültig")?;
    let Some(binding) = document.metadata.get(SNAPSHOT_KEY) else {
        return Ok(());
    };
    validate_snapshot_record(record, &document)?;
    let row = sqlx::query("SELECT e.source,e.entity_type,e.external_id,e.payload_hash,e.payload,e.source_document_id,d.source AS document_source,d.content_hash,d.url,d.metadata FROM brain.entity_snapshots e JOIN brain.source_documents d ON d.id=e.source_document_id WHERE e.id=$1")
        .bind(binding["snapshot_id"].as_i64()).fetch_optional(connection).await.map_err(|_| "Original-Snapshot kann nicht geprüft werden")?.ok_or("Original-Snapshot fehlt")?;
    let payload: Value = row
        .try_get("payload")
        .map_err(|_| "Original-Nutzlast fehlt")?;
    let source: String = row.try_get("source").map_err(|_| "Originalquelle fehlt")?;
    let kind: String = row
        .try_get("entity_type")
        .map_err(|_| "Originaltyp fehlt")?;
    let external: String = row
        .try_get("external_id")
        .map_err(|_| "Originalidentität fehlt")?;
    let hash: String = row
        .try_get("payload_hash")
        .map_err(|_| "Originalhash fehlt")?;
    let doc_source: String = row
        .try_get("document_source")
        .map_err(|_| "Originalquelle fehlt")?;
    let doc_id: i64 = row
        .try_get("source_document_id")
        .map_err(|_| "Originalbindung fehlt")?;
    let doc_hash: String = row
        .try_get("content_hash")
        .map_err(|_| "Originalhash fehlt")?;
    let url: Option<String> = row.try_get("url").map_err(|_| "Originaladresse fehlt")?;
    let metadata: Value = row
        .try_get("metadata")
        .map_err(|_| "Originalmetadaten fehlen")?;
    if binding["upstream_source"] != source
        || binding["entity_type"] != kind
        || binding["external_id"] != external
        || binding["payload_hash"] != hash
        || binding["source_document_id"] != doc_id
        || binding["original_document_sha256"] != doc_hash
        || binding["original_url"] != json!(url)
        || source != doc_source
        || document.metadata.get("original_source_document_metadata") != Some(&metadata)
        || crate::store::json_string(&payload)
            .map_err(|_| "Originalnutzlast kann nicht geprüft werden")?
            != record.content
    {
        return Err("Kanonisches Dokument weicht vom gespeicherten Spieloriginal ab".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata_admission_excludes_missing_originals_without_source_aliases() {
        let mut report = LegacyGameExport {
            considered: 2,
            ..Default::default()
        };
        for (id, binding, reason) in [
            (1, None, "original_document_binding_missing"),
            (2, Some(17), "original_document_missing"),
        ] {
            let excluded = original_admission("deadlock_data", binding, None)
                .unwrap()
                .unwrap();
            assert_eq!(excluded, reason);
            report.exclude(id, excluded.into());
        }
        assert_eq!(report.documents, 0);
        assert_eq!(report.excluded, 2);
        assert_eq!(report.excluded_snapshots.len(), 2);
        for source in ["deadlock_data", "deadlock_patchnotes_db"] {
            assert_eq!(
                original_admission(source, Some(17), Some(source)).unwrap(),
                None
            );
        }
        for (binding, source) in [
            (Some(17), Some("deadlock_patchnotes_db")),
            (Some(17), Some("member_data")),
            (None, Some("deadlock_data")),
            (Some(0), None),
            (Some(-1), Some("deadlock_data")),
        ] {
            assert!(original_admission("deadlock_data", binding, source).is_err());
        }
    }
}
