use std::collections::{BTreeMap, BTreeSet};

use brain_contracts::{
    source::origin_from_record, value::Observed, SourceRecordV2, SourceVisibility,
};
use brain_storage::source_versions::{DOCUMENT_METADATA_KEY, ORIGINAL_VERSION_KEY};
use serde::Serialize;
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::knowledge_contract::{sha256_content, KnowledgeDocument, KnowledgeSourceKind};

pub const AUTHORIZATION_KEY: &str = "brain.public_game_authorization";
pub const AUTHORIZATION_VERSION: &str = "public-game-facts-v1";
pub const PUBLIC_SCOPE: &str = "bot.public";
pub const SOURCES: [&str; 4] = [
    "deadlock-wiki-deadlock-data",
    "steamtracking-gametracking-deadlock",
    "legacy-entities",
    "legacy-patchnotes",
];

pub fn validate_sources(sources: &[String]) -> Result<(), String> {
    if sources.is_empty()
        || sources.len() > SOURCES.len()
        || sources.iter().collect::<BTreeSet<_>>().len() != sources.len()
        || sources
            .iter()
            .any(|source| !SOURCES.contains(&source.as_str()))
    {
        return Err("Nur ausdrücklich benannte öffentliche Spielquellen sind zulässig".into());
    }
    Ok(())
}

fn valid_ref(value: &str) -> bool {
    !value.trim().is_empty() && value.len() <= 2048 && !value.chars().any(char::is_control)
}

pub fn validate_game_origin(record: &SourceRecordV2) -> Result<(), String> {
    if !SOURCES.contains(&record.source_id.as_str()) || record.tombstone {
        return Err("Quelle ist nicht freigegeben oder wurde zurückgezogen".into());
    }
    record.validate().map_err(|_| "Quellvertrag ist ungültig")?;
    let origin = origin_from_record(record)?;
    if !origin.policy.raw_retention_allowed
        || sha256_content(&record.content) != record.content_hash
        || !matches!(&origin.policy.authorization_ref, Observed::Known { value } if valid_ref(value))
    {
        return Err("Erhaltungsrecht, Herkunftsfreigabe oder Inhaltsintegrität fehlt".into());
    }
    if let Some(document) = record.metadata.get(DOCUMENT_METADATA_KEY) {
        let document: KnowledgeDocument =
            serde_json::from_str(document).map_err(|_| "Originaldokument ist ungültig")?;
        if document
            .metadata
            .contains_key(super::legacy_game::SNAPSHOT_KEY)
        {
            document
                .validate(1)
                .map_err(|_| "Legacy-Dokument verletzt den Wissensvertrag")?;
            if record.metadata.get(ORIGINAL_VERSION_KEY) != Some(&document.revision)
                || origin.locator != document.source_locator
            {
                return Err("Legacy-Originalbindung ist ungültig".into());
            }
            return super::legacy_game::validate_snapshot_record(record, &document);
        }
    }
    if matches!(
        record.source_id.as_str(),
        "legacy-entities" | "legacy-patchnotes"
    ) {
        let family = if record.source_id == "legacy-entities" {
            "brain_legacy.entities+entity_aliases"
        } else {
            "brain_legacy.patch_events+patch_event_enrichments"
        };
        let kind_allowed = if record.source_id == "legacy-entities" {
            ["hero", "ability", "item", "npc"]
                .iter()
                .any(|kind| record.logical_id.starts_with(&format!("entity/{kind}/")))
                && record.metadata.get("entity_source").is_some_and(|source| {
                    matches!(source.as_str(), "deadlock_data" | "deadlock_assets_api")
                })
                && record
                    .metadata
                    .get("entity_source")
                    .zip(record.metadata.get("entity_external_id"))
                    .is_some_and(|(source, id)| {
                        origin.origin_artifacts == BTreeSet::from([format!("{source}:{id}")])
                    })
                && origin.locator.starts_with("brain_legacy.entities#")
        } else {
            record.logical_id.starts_with("patch/")
                && origin.origin_artifacts.iter().all(|url| {
                    url.starts_with("https://forums.playdeadlock.com/threads/")
                        || url.starts_with("https://store.steampowered.com/news/app/1422450/")
                })
                && !origin.origin_artifacts.is_empty()
        };
        if origin.parser_family != "brain_legacy"
            || origin.derivation_family != Observed::known(family.into())
            || !kind_allowed
            || !matches!(&origin.source_revision, brain_contracts::source::SourceRevision::Api { api_version, .. } if api_version == "brain_legacy.archive.v1")
        {
            return Err("Legacy-Dokument besitzt keinen passenden Spielherkunftsbeleg".into());
        }
        return Ok(());
    }
    let document: KnowledgeDocument = serde_json::from_str(
        record
            .metadata
            .get(DOCUMENT_METADATA_KEY)
            .ok_or("Originaldokument fehlt")?,
    )
    .map_err(|_| "Originaldokument ist ungültig")?;
    document
        .validate(1)
        .map_err(|_| "Originaldokument verletzt den Wissensvertrag")?;
    let metadata = &document.metadata;
    let repository = if record.source_id == "deadlock-wiki-deadlock-data" {
        "https://github.com/deadlock-wiki/deadlock-data"
    } else {
        "https://github.com/SteamTracking/GameTracking-Deadlock"
    };
    let path = metadata
        .get("relative_path")
        .and_then(Value::as_str)
        .ok_or("Spielpfad fehlt")?;
    let original_path = metadata
        .get("original_relative_path")
        .and_then(Value::as_str)
        .ok_or("Originalpfad fehlt")?;
    let path_allowed = if record.source_id == "deadlock-wiki-deadlock-data" {
        (original_path.starts_with("data/json/") && original_path.ends_with(".json"))
            || matches!(
                original_path,
                "data/localizations/english.json" | "data/localizations/german.json"
            )
    } else {
        original_path.starts_with("game/citadel/pak01_dir/scripts/")
            || ((original_path.starts_with("game/citadel/resource/localization/")
                || original_path.starts_with("game/citadel/pak01_dir/resource/localization/"))
                && (original_path.ends_with("_english.txt")
                    || original_path.ends_with("_german.txt")))
    };
    let safe_path = |path: &str| {
        !path.contains(['\\', ':'])
            && std::path::Path::new(path)
                .components()
                .all(|component| matches!(component, std::path::Component::Normal(_)))
    };
    let revision_matches = metadata.get("source_revision").and_then(Value::as_str)
        == Some(document.revision.as_str())
        && metadata
            .get("provenance")
            .and_then(|value| value.get("git_commit"))
            .and_then(Value::as_str)
            == Some(document.revision.as_str())
        && crate::git_source::validate_commit(&document.revision).is_ok();
    if document.source_kind != KnowledgeSourceKind::GameFile
        || document.source_id != record.source_id
        || document.document_id != record.logical_id
        || document.content != record.content
        || document.content_sha256 != record.content_hash
        || record.metadata.get(ORIGINAL_VERSION_KEY) != Some(&document.revision)
        || origin.locator != document.source_locator
        || metadata.get("app_id") != Some(&json!(1422450))
        || metadata
            .get("provenance")
            .and_then(|value| value.get("repository_url"))
            .and_then(Value::as_str)
            != Some(repository)
        || document.document_id != format!("game:1422450:{path}")
        || !path_allowed
        || !safe_path(path)
        || !safe_path(original_path)
        || !revision_matches
        || metadata
            .get("extraction")
            .and_then(|value| value.get("version"))
            .and_then(Value::as_str)
            != Some(crate::game_files::EXTRACTOR_VERSION)
        || !record
            .metadata
            .get("wiki-spielwissen.provenance_evidence_ref")
            .is_some_and(|value| valid_ref(value))
    {
        return Err("Originaldokument belegt keine freigegebene, gepinnte Spielquelle".into());
    }
    Ok(())
}

fn bind_game_authorization(
    record: &SourceRecordV2,
    authorization_ref: &str,
    provider_egress_ref: Option<&str>,
    next_revision: u64,
) -> Result<SourceRecordV2, String> {
    validate_game_origin(record)?;
    if !valid_ref(authorization_ref)
        || provider_egress_ref.is_some_and(|value| !valid_ref(value))
        || next_revision <= record.revision
        || next_revision > i64::MAX as u64
    {
        return Err("Expliziter Freigabenachweis oder monotone Folgerevision fehlt".into());
    }
    let mut next = record.clone();
    let mut origin = origin_from_record(record)?;
    let original_policy = origin.policy.clone();
    next.revision = next_revision;
    next.visibility = SourceVisibility::Public;
    next.allowed_scopes = BTreeSet::from([PUBLIC_SCOPE.into()]);
    origin.policy.visibility = next.visibility;
    origin.policy.allowed_scopes = next.allowed_scopes.clone();
    origin.policy.publication_allowed = true;
    origin.policy.provider_egress_allowed = provider_egress_ref.is_some();
    origin.policy.authorization_ref = Observed::known(authorization_ref.into());
    next.metadata.insert(
        AUTHORIZATION_KEY.into(),
        json!({
            "contract_version": AUTHORIZATION_VERSION,
            "authorization_ref": authorization_ref,
            "provider_egress_ref": provider_egress_ref,
            "publication_basis": "operator_public_factual_game_data",
            "raw_asset_redistribution_authorized": false,
            "previous_store_revision": record.revision,
            "previous_policy": original_policy,
            "original_license_declaration_retained": true,
        })
        .to_string(),
    );
    origin.bind_record(&mut next)?;
    Ok(next)
}

pub fn authorize_game_record(
    record: &SourceRecordV2,
    authorization_ref: &str,
    provider_egress_ref: Option<&str>,
    next_revision: u64,
) -> Result<SourceRecordV2, String> {
    let next = bind_game_authorization(
        record,
        authorization_ref,
        provider_egress_ref,
        next_revision,
    )?;
    dbrain_retrieval::knowledge_projection::project_knowledge(&next)
        .map_err(|error| {
            format!(
                "Faktenfreigabe verletzt den Projektionsvertrag für {}:{}: {error}",
                record.source_id, record.logical_id
            )
        })?
        .ok_or("Faktenprojektion fehlt; keine öffentliche Freigabe")?;
    Ok(next)
}

#[derive(Debug, Clone, Serialize)]
pub struct GameExclusion {
    pub source_id: String,
    pub logical_id: String,
    pub reason: String,
    pub tombstone: bool,
}

#[derive(Debug, Default)]
pub struct GameSelection {
    pub selected: BTreeSet<(String, String)>,
    pub excluded: Vec<GameExclusion>,
}

impl GameSelection {
    pub fn exclusion_reasons(&self) -> BTreeMap<String, usize> {
        let mut reasons = BTreeMap::new();
        for excluded in &self.excluded {
            *reasons.entry(excluded.reason.clone()).or_default() += 1;
        }
        reasons
    }

    pub fn report(&self) -> Value {
        let tombstones = self
            .excluded
            .iter()
            .filter(|record| record.tombstone)
            .count();
        json!({"selected": self.selected.len(), "excluded": self.excluded.len(),
            "excluded_live": self.excluded.len() - tombstones, "excluded_tombstones": tombstones,
            "exclusion_reasons": self.exclusion_reasons(), "excluded_documents": self.excluded})
    }

    fn exclude(&mut self, record: &SourceRecordV2, reason: String) -> Result<(), String> {
        let origin = origin_from_record(record)?;
        if !record.tombstone
            && (record.visibility == SourceVisibility::Public || origin.policy.publication_allowed)
        {
            return Err("Nicht zulässiges Spielmaterial ist bereits öffentlich freigegeben; keine automatische Rechteänderung".into());
        }
        self.excluded.push(GameExclusion {
            source_id: record.source_id.clone(),
            logical_id: record.logical_id.clone(),
            reason,
            tombstone: record.tombstone,
        });
        Ok(())
    }
}

pub fn select_game_heads(
    records: &[SourceRecordV2],
    authorization_ref: &str,
    provider_egress_ref: Option<&str>,
) -> Result<GameSelection, String> {
    if !valid_ref(authorization_ref) || provider_egress_ref.is_some_and(|value| !valid_ref(value)) {
        return Err("Expliziter Freigabenachweis fehlt".into());
    }
    let mut selection = GameSelection::default();
    let mut seen = BTreeSet::new();
    for record in records {
        record.validate().map_err(|_| "Quellvertrag ist ungültig")?;
        origin_from_record(record)?;
        if !SOURCES.contains(&record.source_id.as_str())
            || !seen.insert((record.source_id.clone(), record.logical_id.clone()))
        {
            return Err("Fremde oder doppelte Spielquellköpfe".into());
        }
        if sha256_content(&record.content) != record.content_hash {
            return Err("Spieloriginal hat eine widersprüchliche Inhaltsintegrität".into());
        }
        if record.tombstone {
            selection.exclude(record, "withdrawn".into())?;
            continue;
        }
        if let Err(error) = validate_game_origin(record) {
            selection.exclude(record, format!("game_origin_unconfirmed:{error}"))?;
            continue;
        }
        let next_revision = record
            .revision
            .checked_add(1)
            .ok_or("Store-Revision ist ausgeschöpft")?;
        let next = bind_game_authorization(
            record,
            authorization_ref,
            provider_egress_ref,
            next_revision,
        )?;
        match dbrain_retrieval::knowledge_projection::project_knowledge(&next) {
            Ok(Some(projection))
                if projection.raw_byte_end == 0 && !projection.facts.is_empty() =>
            {
                selection
                    .selected
                    .insert((record.source_id.clone(), record.logical_id.clone()));
            }
            Err(brain_contracts::PortError::InvalidResponse(reason))
                if reason.starts_with("public_game_facts_empty_or_unparsed:") =>
            {
                if let Some(encoded) = record.metadata.get(DOCUMENT_METADATA_KEY) {
                    let document: KnowledgeDocument = serde_json::from_str(encoded)
                        .map_err(|_| "Originaldokument ist ungültig")?;
                    if !document.facts.is_empty() {
                        return Err(
                            "Deklarierte Spielfakten besitzen keine auswertbare Originalgrundlage"
                                .into(),
                        );
                    }
                }
                selection.exclude(record, reason)?;
            }
            Err(brain_contracts::PortError::InvalidResponse(reason))
                if reason == "knowledge_projection_binding"
                    && record.source_id.starts_with("legacy-")
                    && !record.metadata.contains_key(DOCUMENT_METADATA_KEY) =>
            {
                selection.exclude(record, "legacy_text_facts_not_projectable".into())?;
            }
            Err(error) => {
                return Err(format!(
                    "Spielprojektion verletzt ihren Vertrag für {}:{}: {error}",
                    record.source_id, record.logical_id
                ))
            }
            _ => return Err("Spielprojektion hat keine gebundenen Fakten geliefert".into()),
        }
    }
    Ok(selection)
}

pub fn select_game_publication(records: &[SourceRecordV2]) -> Result<GameSelection, String> {
    let mut selection = select_game_heads(records, "publish:dry-game-admission", None)?;
    for record in records {
        let key = (record.source_id.clone(), record.logical_id.clone());
        if !selection.selected.contains(&key) {
            continue;
        }
        let origin = origin_from_record(record)?;
        if record.visibility != SourceVisibility::Public
            || record.allowed_scopes != BTreeSet::from([PUBLIC_SCOPE.into()])
            || !origin.policy.publication_allowed
            || !record.metadata.contains_key(AUTHORIZATION_KEY)
        {
            selection.selected.remove(&key);
            selection.exclude(record, "public_factual_authorization_missing".into())?;
            continue;
        }
        let projection = dbrain_retrieval::knowledge_projection::project_knowledge(record)
            .map_err(|_| "Gespeicherte öffentliche Faktenfreigabe ist ungültig")?
            .ok_or("Gespeicherte öffentliche Faktenprojektion fehlt")?;
        if projection.raw_byte_end != 0 || projection.facts.is_empty() {
            return Err("Gespeicherte Freigabe ist keine reine Faktenprojektion".into());
        }
    }
    Ok(selection)
}

pub fn is_game_reauthorization_transition(old: &SourceRecordV2, next: &SourceRecordV2) -> bool {
    if validate_game_origin(old).is_err() || validate_game_origin(next).is_err() {
        return false;
    }
    let Ok(old_origin) = origin_from_record(old) else {
        return false;
    };
    let Some(grant) = next
        .metadata
        .get(AUTHORIZATION_KEY)
        .and_then(|encoded| serde_json::from_str::<Value>(encoded).ok())
    else {
        return false;
    };
    if grant["previous_store_revision"].as_u64() != Some(old.revision)
        || serde_json::from_value::<brain_contracts::source::SourcePolicy>(
            grant["previous_policy"].clone(),
        )
        .ok()
            != Some(old_origin.policy)
    {
        return false;
    }
    let Some(authorization_ref) = grant["authorization_ref"].as_str() else {
        return false;
    };
    let egress = match &grant["provider_egress_ref"] {
        Value::Null => None,
        Value::String(value) => Some(value.as_str()),
        _ => return false,
    };
    authorize_game_record(old, authorization_ref, egress, next.revision)
        .is_ok_and(|expected| expected == *next)
}

pub async fn read_game_heads(
    pool: &PgPool,
    sources: &[String],
) -> Result<Vec<SourceRecordV2>, String> {
    validate_sources(sources)?;
    let rows: Vec<Value> = sqlx::query_scalar("SELECT record_json FROM brain.source_record_heads WHERE source_id=ANY($1) ORDER BY source_id,logical_id LIMIT 10001")
        .bind(sources).fetch_all(pool).await.map_err(|_| "Spielquellköpfe können nicht gelesen werden")?;
    if rows.len() > 10_000 {
        return Err("Spielquellmenge überschreitet 10.000 Dokumente".into());
    }
    let records = rows
        .into_iter()
        .map(|value| {
            serde_json::from_value(value).map_err(|_| "Spielquellkopf ist ungültig".to_owned())
        })
        .collect::<Result<Vec<SourceRecordV2>, _>>()?;
    Ok(records)
}

pub async fn reauthorize_game_heads(
    pool: &PgPool,
    sources: &[String],
    expected: &[SourceRecordV2],
    authorization_ref: &str,
    provider_egress_ref: Option<&str>,
) -> Result<Value, String> {
    validate_sources(sources)?;
    if !valid_ref(authorization_ref) || provider_egress_ref.is_some_and(|value| !valid_ref(value)) {
        return Err("Expliziter Freigabenachweis fehlt".into());
    }
    if expected.is_empty() || expected.len() > 10_000 {
        return Err("Erwartete Spielquellköpfe fehlen oder sind zu zahlreich".into());
    }
    let selection = select_game_heads(expected, authorization_ref, provider_egress_ref)?;
    let mut expected_map = BTreeMap::new();
    for record in expected {
        if !sources.contains(&record.source_id)
            || expected_map
                .insert(
                    (record.source_id.clone(), record.logical_id.clone()),
                    record,
                )
                .is_some()
        {
            return Err("Fremde oder doppelte erwartete Dokumente".into());
        }
    }
    if sources
        .iter()
        .any(|source| !expected.iter().any(|record| &record.source_id == source))
    {
        return Err("Erwartete Köpfe decken die ausgewählten Quellen nicht ab".into());
    }
    let mut tx = pool
        .begin()
        .await
        .map_err(|_| "Freigabetransaktion kann nicht starten")?;
    let mut ordered = sources.to_vec();
    ordered.sort();
    for source in ordered {
        sqlx::query("SELECT pg_advisory_xact_lock(hashtext($1)::bigint)")
            .bind(format!("core-source:{source}"))
            .execute(&mut *tx)
            .await
            .map_err(|_| "Quellensperre fehlgeschlagen")?;
    }
    let inconsistent: bool = sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM brain.source_record_heads h LEFT JOIN brain.source_record_revisions r ON r.source_id=h.source_id AND r.logical_id=h.logical_id AND r.revision=h.revision WHERE h.source_id=ANY($1) AND (r.record_json IS DISTINCT FROM h.record_json OR r.content_hash IS DISTINCT FROM h.content_hash OR r.tombstone IS DISTINCT FROM h.tombstone OR h.record_json->>'source_id' IS DISTINCT FROM h.source_id OR h.record_json->>'logical_id' IS DISTINCT FROM h.logical_id OR h.record_json->>'revision' IS DISTINCT FROM h.revision::text OR h.record_json->>'content_hash' IS DISTINCT FROM h.content_hash OR h.record_json->>'tombstone' IS DISTINCT FROM h.tombstone::text))")
        .bind(sources).fetch_one(&mut *tx).await.map_err(|_| "Kopfspalten können nicht geprüft werden")?;
    if inconsistent {
        return Err("Kopfspalten oder Originalrevision widersprechen sich".into());
    }
    let actual: Vec<Value> = sqlx::query_scalar("SELECT record_json FROM brain.source_record_heads WHERE source_id=ANY($1) ORDER BY source_id,logical_id FOR UPDATE")
        .bind(sources).fetch_all(&mut *tx).await.map_err(|_| "Aktuelle Quellköpfe können nicht geprüft werden")?;
    let actual: Vec<SourceRecordV2> = actual
        .into_iter()
        .map(|value| {
            serde_json::from_value(value).map_err(|_| "Aktueller Kopf ist ungültig".to_owned())
        })
        .collect::<Result<_, _>>()?;
    if actual.len() != expected.len()
        || actual.iter().any(|record| {
            expected_map
                .get(&(record.source_id.clone(), record.logical_id.clone()))
                .copied()
                != Some(record)
        })
    {
        return Err(
            "Kopfstand oder Rechte wurden zwischenzeitlich geändert; keine Freigabe".into(),
        );
    }
    let mut updated = 0usize;
    let mut unchanged = 0usize;
    for record in actual {
        if !selection
            .selected
            .contains(&(record.source_id.clone(), record.logical_id.clone()))
        {
            continue;
        }
        super::legacy_game::verify_snapshot_record(&mut tx, &record).await?;
        let historical: Option<Value> = sqlx::query_scalar("SELECT record_json FROM brain.source_record_revisions WHERE source_id=$1 AND logical_id=$2 AND revision=$3")
            .bind(&record.source_id).bind(&record.logical_id).bind(record.revision as i64).fetch_optional(&mut *tx).await.map_err(|_| "Originalrevision kann nicht geprüft werden")?;
        if historical
            != Some(serde_json::to_value(&record).map_err(|_| "Kopf kann nicht geprüft werden")?)
        {
            return Err("Kopf und Originalrevision widersprechen sich".into());
        }
        let origin = origin_from_record(&record)?;
        let retry = record
            .metadata
            .get(AUTHORIZATION_KEY)
            .and_then(|value| serde_json::from_str::<Value>(value).ok());
        if retry.as_ref().is_some_and(|value| {
            value["contract_version"] == AUTHORIZATION_VERSION
                && value["authorization_ref"] == authorization_ref
                && value["provider_egress_ref"] == json!(provider_egress_ref)
        }) && record.visibility == SourceVisibility::Public
            && record.allowed_scopes == BTreeSet::from([PUBLIC_SCOPE.into()])
            && origin.policy.publication_allowed
            && origin.policy.provider_egress_allowed == provider_egress_ref.is_some()
        {
            dbrain_retrieval::knowledge_projection::project_knowledge(&record)
                .map_err(|_| "Bestehende Faktenfreigabe verletzt den Projektionsvertrag")?
                .ok_or("Bestehende Faktenprojektion fehlt")?;
            unchanged += 1;
            continue;
        }
        let maximum: i64 = sqlx::query_scalar("SELECT max(revision) FROM brain.source_record_revisions WHERE source_id=$1 AND logical_id=$2")
            .bind(&record.source_id).bind(&record.logical_id).fetch_one(&mut *tx).await.map_err(|_| "Revisionsmaximum kann nicht geprüft werden")?;
        let revision = maximum
            .checked_add(1)
            .filter(|value| *value > 0)
            .ok_or("Store-Revision ist ausgeschöpft")? as u64;
        let next =
            authorize_game_record(&record, authorization_ref, provider_egress_ref, revision)?;
        let value = serde_json::to_value(&next)
            .map_err(|_| "Folgerevision kann nicht serialisiert werden")?;
        sqlx::query("INSERT INTO brain.source_record_revisions(source_id,logical_id,revision,content_hash,tombstone,record_json) VALUES($1,$2,$3,$4,false,$5)")
            .bind(&next.source_id).bind(&next.logical_id).bind(revision as i64).bind(&next.content_hash).bind(&value).execute(&mut *tx).await.map_err(|_| "Neue Freigaberevision kann nicht gespeichert werden")?;
        let changed = sqlx::query("UPDATE brain.source_record_heads SET revision=$3,record_json=$4,updated_at=now() WHERE source_id=$1 AND logical_id=$2 AND revision=$5 AND record_json=$6 AND content_hash=$7 AND tombstone=false")
            .bind(&next.source_id).bind(&next.logical_id).bind(revision as i64).bind(&value).bind(record.revision as i64).bind(serde_json::to_value(&record).map_err(|_| "Erwarteter Kopf ist ungültig")?).bind(&record.content_hash)
            .execute(&mut *tx).await.map_err(|_| "Optimistische Kopfänderung fehlgeschlagen")?;
        if changed.rows_affected() != 1 {
            return Err("Optimistische Kopfprüfung fehlgeschlagen; keine Freigabe".into());
        }
        updated += 1;
    }
    tx.commit()
        .await
        .map_err(|_| "Atomare Freigabe wurde nicht bestätigt")?;
    Ok(
        json!({"reauthorized": !selection.selected.is_empty(), "updated": updated, "unchanged": unchanged,
        "selection": selection.report(), "checked_heads": expected.len(),
        "sources": sources, "authorization_ref": authorization_ref,
        "provider_egress_ref": provider_egress_ref, "scope": PUBLIC_SCOPE,
        "publication_basis": "operator_public_factual_game_data", "raw_asset_redistribution_authorized": false,
        "published": false, "activated": false}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        game_files::{extract_game_files, GameFileOptions},
        knowledge_contract::validate_knowledge_jsonl,
        knowledge_import::{prepare_validated_knowledge, ImportPolicy},
    };

    fn fixture() -> SourceRecordV2 {
        fixture_content("{\"Urn\":12}")
    }

    fn fixture_content(content: &str) -> SourceRecordV2 {
        fixture_path(content, "data/json/generic-data.json")
    }

    fn fixture_path(content: &str, path: &str) -> SourceRecordV2 {
        let directory = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(directory.path().join("data/json")).unwrap();
        std::fs::write(directory.path().join(path), content).unwrap();
        let commit = "a".repeat(40);
        let options = GameFileOptions {
            root: directory.path().into(),
            app_id: 1422450,
            source_id: SOURCES[0].into(),
            observed_at: "2026-10-09T12:00:00Z".into(),
            build_id: None,
            manifest_id: None,
            source_revision: Some(commit.clone()),
            depot_id: None,
            language: "und".into(),
            attribution: "Valve game data".into(),
            license_name: "MIT (repository); Valve asset rights not verified".into(),
            license_url: None,
            provenance: json!({"repository_url": "https://github.com/deadlock-wiki/deadlock-data", "git_commit": commit}),
            max_file_bytes: 8388608,
        };
        let mut bytes = Vec::new();
        extract_game_files(&options, &mut bytes).unwrap();
        let input = validate_knowledge_jsonl(std::io::Cursor::new(bytes)).unwrap();
        let policy: ImportPolicy = serde_json::from_value(json!({"sources": {(SOURCES[0]): {
            "internal_read_allowed": true, "raw_retention_allowed": true, "publication_allowed": false,
            "provider_egress_allowed": false, "authorization_ref": "operator:original", "provenance_evidence_ref": "evidence:original", "allowed_scopes": ["internal_docs"]
        }}})).unwrap();
        prepare_validated_knowledge(&input, &policy, "game-files-v2")
            .unwrap()
            .records()[0]
            .record
            .clone()
    }

    #[test]
    fn admission_preserves_unconfirmed_empty_and_withdrawn_originals() {
        let valid = fixture();
        let empty = fixture_path("{}", "data/json/empty.json");
        let mut withdrawn = fixture_path("{\"Urn\":12}", "data/json/withdrawn.json");
        withdrawn.tombstone = true;
        let mut unconfirmed = fixture_path("{\"Urn\":12}", "data/json/unconfirmed.json");
        let mut document: Value =
            serde_json::from_str(&unconfirmed.metadata[DOCUMENT_METADATA_KEY]).unwrap();
        document["metadata"]["provenance"]["repository_url"] = json!("https://example.invalid");
        unconfirmed
            .metadata
            .insert(DOCUMENT_METADATA_KEY.into(), document.to_string());
        let records = vec![valid.clone(), empty.clone(), withdrawn.clone(), unconfirmed];
        let before = records.clone();
        let selection = select_game_heads(&records, "operator:public", None).unwrap();
        assert_eq!(
            selection.selected,
            BTreeSet::from([(valid.source_id.clone(), valid.logical_id.clone())])
        );
        assert_eq!(selection.report()["excluded_live"], 2);
        assert_eq!(selection.report()["excluded_tombstones"], 1);
        assert!(selection.excluded.iter().any(|excluded| excluded
            .reason
            .starts_with("public_game_facts_empty_or_unparsed:")));
        assert_eq!(records, before);
        let private = select_game_publication(std::slice::from_ref(&valid)).unwrap();
        assert!(private.selected.is_empty());
        assert_eq!(
            private.excluded[0].reason,
            "public_factual_authorization_missing"
        );
        let granted = authorize_game_record(&valid, "operator:public", None, 2).unwrap();
        let publication = select_game_publication(&[granted.clone(), empty, withdrawn]).unwrap();
        assert_eq!(publication.selected, selection.selected);
        let mut invalid_public = granted;
        let mut origin = origin_from_record(&invalid_public).unwrap();
        origin.policy.raw_retention_allowed = false;
        origin.bind_record(&mut invalid_public).unwrap();
        assert!(select_game_heads(&[invalid_public], "operator:public", None).is_err());
    }

    #[test]
    fn admission_does_not_hide_corrupt_originals_or_fabricated_facts() {
        let mut corrupt = fixture();
        corrupt.content.push(' ');
        assert!(select_game_heads(&[corrupt], "operator:public", None).is_err());
        let mut fabricated = fixture();
        let mut document: Value =
            serde_json::from_str(&fabricated.metadata[DOCUMENT_METADATA_KEY]).unwrap();
        document["facts"][0]["value"] = json!(999);
        fabricated
            .metadata
            .insert(DOCUMENT_METADATA_KEY.into(), document.to_string());
        assert!(select_game_heads(&[fabricated], "operator:public", None)
            .unwrap_err()
            .contains("public_game_facts_disagree_with_original"));
        let mut internal = fixture();
        let mut origin = origin_from_record(&internal).unwrap();
        internal.source_id = "legacy-entities".into();
        internal.logical_id = "entity/ability_internal/private".into();
        internal.metadata.remove(DOCUMENT_METADATA_KEY);
        internal.metadata.remove(ORIGINAL_VERSION_KEY);
        origin.identity.source_id = internal.source_id.clone();
        origin.identity.logical_id = internal.logical_id.clone();
        origin.bind_record(&mut internal).unwrap();
        let selection = select_game_heads(&[internal], "operator:public", None).unwrap();
        assert!(selection.selected.is_empty());
        assert!(selection.excluded[0]
            .reason
            .starts_with("game_origin_unconfirmed:"));
    }

    #[test]
    fn activation_exception_accepts_only_an_exact_original_preserving_game_transition() {
        let original = fixture();
        let authorized =
            authorize_game_record(&original, "operator:public", Some("operator:provider"), 2)
                .unwrap();
        assert!(is_game_reauthorization_transition(&original, &authorized));
        let mut foreign = authorized.clone();
        foreign.metadata.insert("foreign".into(), "changed".into());
        assert!(!is_game_reauthorization_transition(&original, &foreign));
        let mut foreign = authorized.clone();
        let mut document: Value =
            serde_json::from_str(&foreign.metadata[DOCUMENT_METADATA_KEY]).unwrap();
        document["title"] = json!("Fremdes Dokument");
        foreign
            .metadata
            .insert(DOCUMENT_METADATA_KEY.into(), document.to_string());
        assert!(!is_game_reauthorization_transition(&original, &foreign));
        let mut foreign = authorized.clone();
        let mut grant: Value = serde_json::from_str(&foreign.metadata[AUTHORIZATION_KEY]).unwrap();
        grant["previous_store_revision"] = json!(2);
        foreign
            .metadata
            .insert(AUTHORIZATION_KEY.into(), grant.to_string());
        assert!(!is_game_reauthorization_transition(&original, &foreign));
    }

    #[test]
    fn factual_authorization_preserves_originals_and_does_not_invent_a_license() {
        let record = fixture();
        let next =
            authorize_game_record(&record, "operator:explicit-public-facts", None, 2).unwrap();
        assert_eq!(next.content, record.content);
        assert_eq!(next.content_hash, record.content_hash);
        assert_eq!(
            next.metadata[DOCUMENT_METADATA_KEY],
            record.metadata[DOCUMENT_METADATA_KEY]
        );
        assert_eq!(
            next.metadata[ORIGINAL_VERSION_KEY],
            record.metadata[ORIGINAL_VERSION_KEY]
        );
        let original = origin_from_record(&record).unwrap();
        let authorized = origin_from_record(&next).unwrap();
        assert_eq!(authorized.policy.license, original.policy.license);
        assert_eq!(authorized.source_revision, original.source_revision);
        assert_eq!(authorized.validity, original.validity);
        assert!(authorized.policy.publication_allowed);
        assert!(!authorized.policy.provider_egress_allowed);
        assert_eq!(next.allowed_scopes, BTreeSet::from([PUBLIC_SCOPE.into()]));
        let projection = dbrain_retrieval::knowledge_projection::project_knowledge(&next)
            .unwrap()
            .unwrap();
        assert_eq!(projection.raw_byte_end, 0);
        assert_eq!(projection.raw_sha256, record.content_hash);
        assert!(!projection.text.contains(&record.content));
        assert_eq!(projection.facts.len(), 1);
        let release = brain_contracts::CorpusRelease {
            release_id: "game-factual".into(),
            knowledge_version: "game-factual".into(),
            patch: "fixture".into(),
            created_at_epoch: 1,
            source_revisions: BTreeMap::from([(
                next.source_id.clone(),
                BTreeMap::from([(next.logical_id.clone(), next.revision)]),
            )]),
        };
        let indexed = dbrain_retrieval::preflight_release_index(release, vec![next]).unwrap();
        assert_eq!(indexed.chunks, projection.facts.len());
        assert!(authorize_game_record(&record, "", None, 2).is_err());
        assert!(authorize_game_record(&record, "operator:grant", None, 1).is_err());
    }

    #[test]
    fn factual_projection_rejects_empty_unparsed_and_fabricated_game_facts() {
        for content in ["{}", "Spielcode ohne semantischen Parser"] {
            let record = fixture_content(content);
            let error = authorize_game_record(&record, "operator:public", None, 2).unwrap_err();
            assert!(error.contains("public_game_facts_empty_or_unparsed"));
        }
        let mut record = fixture();
        let mut document: Value =
            serde_json::from_str(&record.metadata[DOCUMENT_METADATA_KEY]).unwrap();
        document["facts"][0]["value"] = json!(999);
        record
            .metadata
            .insert(DOCUMENT_METADATA_KEY.into(), document.to_string());
        assert!(authorize_game_record(&record, "operator:public", None, 2)
            .unwrap_err()
            .contains("public_game_facts_disagree_with_original"));
    }

    #[test]
    fn legacy_text_originals_publish_only_bound_facts_and_source_statements() {
        for patch in [false, true] {
            let mut record = fixture();
            let mut origin = origin_from_record(&record).unwrap();
            record.metadata.remove(DOCUMENT_METADATA_KEY);
            record.metadata.remove(ORIGINAL_VERSION_KEY);
            if patch {
                record.source_id = "legacy-patchnotes".into();
                record.logical_id = "patch/test".into();
                record.content = "Patch: Prüfpatch\nURL: https://forums.playdeadlock.com/threads/test/\nPosted: 2026-10-09\nSource kind: forum\n\n- Prüfheld: Cooldown 12 -> 10\n".into();
                origin.locator = "https://forums.playdeadlock.com/threads/test/".into();
                origin.origin_artifacts = BTreeSet::from([origin.locator.clone()]);
                origin.derivation_family =
                    Observed::known("brain_legacy.patch_events+patch_event_enrichments".into());
                record.metadata.insert("kind".into(), "prose".into());
            } else {
                record.source_id = "legacy-entities".into();
                record.logical_id = "entity/hero/Prüfheld".into();
                record.content = "hero: Prüfheld\nExternal ID: 7\nSource: deadlock_assets_api\nhealth: 700\nabilities: {\"cooldown\":12}\n".into();
                record
                    .metadata
                    .insert("entity_source".into(), "deadlock_assets_api".into());
                record
                    .metadata
                    .insert("entity_external_id".into(), "7".into());
                record.metadata.insert("kind".into(), "fact".into());
                origin.locator = "brain_legacy.entities#7".into();
                origin.origin_artifacts = BTreeSet::from(["deadlock_assets_api:7".into()]);
                origin.derivation_family =
                    Observed::known("brain_legacy.entities+entity_aliases".into());
            }
            record.content_hash = sha256_content(&record.content);
            origin.identity.source_id = record.source_id.clone();
            origin.identity.logical_id = record.logical_id.clone();
            origin.raw_sha256 = record.content_hash.clone();
            origin.source_revision = brain_contracts::source::SourceRevision::Api {
                api_version: "brain_legacy.archive.v1".into(),
                original_revision: Some("legacy:test".into()),
            };
            origin.parser_family = "brain_legacy".into();
            origin.policy.license =
                Observed::unknown(brain_contracts::value::UnknownReason::NotPresent);
            origin.bind_record(&mut record).unwrap();
            let granted = authorize_game_record(&record, "operator:public", None, 2).unwrap();
            assert_eq!(granted.content, record.content);
            let projection = dbrain_retrieval::knowledge_projection::project_knowledge(&granted)
                .unwrap()
                .unwrap();
            assert_eq!(projection.raw_byte_end, 0);
            assert!(!projection.text.contains(&record.content));
            assert!(!projection.facts.is_empty());
            assert_eq!(projection.raw_sha256, record.content_hash);
            if patch {
                assert!(projection
                    .facts
                    .iter()
                    .all(|fact| fact.evidence_status == "source_statement"));
            } else {
                assert!(projection.text.contains("700"));
                assert!(projection.text.contains("cooldown"));
            }
            let release = brain_contracts::CorpusRelease {
                release_id: "legacy-factual".into(),
                knowledge_version: "legacy-factual".into(),
                patch: "fixture".into(),
                created_at_epoch: 1,
                source_revisions: BTreeMap::from([(
                    granted.source_id.clone(),
                    BTreeMap::from([(granted.logical_id.clone(), granted.revision)]),
                )]),
            };
            let indexed =
                dbrain_retrieval::preflight_release_index(release, vec![granted]).unwrap();
            assert_eq!(indexed.chunks, projection.facts.len());
            let mut foreign = record.clone();
            foreign.content.push_str("Ungeparstes Fremdmaterial\n");
            foreign.content_hash = sha256_content(&foreign.content);
            let mut foreign_origin = origin_from_record(&record).unwrap();
            foreign_origin.raw_sha256 = foreign.content_hash.clone();
            foreign_origin.bind_record(&mut foreign).unwrap();
            assert!(authorize_game_record(&foreign, "operator:public", None, 2).is_err());
        }
    }

    #[tokio::test]
    async fn postgres_factual_reauthorization_is_atomic_optimistic_and_retryable() {
        struct ScratchPg(tempfile::TempDir);
        impl Drop for ScratchPg {
            fn drop(&mut self) {
                let _ = std::process::Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
                    .arg("-D")
                    .arg(self.0.path().join("data"))
                    .args(["-m", "immediate", "-w", "stop"])
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .status();
            }
        }
        let pg = ScratchPg(tempfile::tempdir().unwrap());
        let socket = pg.0.path().join("socket");
        std::fs::create_dir(&socket).unwrap();
        assert!(
            std::process::Command::new("/usr/lib/postgresql/16/bin/initdb")
                .arg("-D")
                .arg(pg.0.path().join("data"))
                .args(["-A", "trust", "-U", "brain_game_test", "--no-locale"])
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status()
                .unwrap()
                .success()
        );
        assert!(
            std::process::Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
                .arg("-D")
                .arg(pg.0.path().join("data"))
                .arg("-l")
                .arg(pg.0.path().join("postgres.log"))
                .args([
                    "-o",
                    &format!("-k {} -p 55445 -c listen_addresses=''", socket.display()),
                    "-w",
                    "start"
                ])
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status()
                .unwrap()
                .success()
        );
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(2)
            .connect_with(
                sqlx::postgres::PgConnectOptions::new_without_pgpass()
                    .host(socket.to_str().unwrap())
                    .port(55445)
                    .username("brain_game_test")
                    .database("postgres")
                    .password(""),
            )
            .await
            .unwrap();
        let store = brain_storage::PgStore::new(pool.clone());
        store.migrate_core().await.unwrap();
        let record = fixture();
        let mut second = record.clone();
        let mut doc: Value = serde_json::from_str(&second.metadata[DOCUMENT_METADATA_KEY]).unwrap();
        second.logical_id = "game:1422450:data/json/convars.json".into();
        doc["document_id"] = json!(second.logical_id);
        doc["source_locator"] = json!("data/json/convars.json");
        doc["title"] = json!("convars.json");
        doc["metadata"]["relative_path"] = json!("data/json/convars.json");
        doc["metadata"]["original_relative_path"] = json!("data/json/convars.json");
        doc["facts"] = json!(
            brain_storage::entity_profile::derivation::game_file_facts::extract_facts(
                "data/json/convars.json",
                &second.content
            )
            .0
        );
        second
            .metadata
            .insert(DOCUMENT_METADATA_KEY.into(), doc.to_string());
        assert!(origin_from_record(&second).is_err());
        let mut source_origin = origin_from_record(&record).unwrap();
        source_origin.identity.logical_id = second.logical_id.clone();
        source_origin.locator = "data/json/convars.json".into();
        source_origin.origin_artifacts = BTreeSet::from([format!(
            "{}:{}:{}",
            second.source_id, second.logical_id, second.metadata[ORIGINAL_VERSION_KEY]
        )]);
        source_origin.bind_record(&mut second).unwrap();
        store.apply(&record).await.unwrap();
        store.apply(&second).await.unwrap();
        let sources = vec![record.source_id.clone()];
        let expected = read_game_heads(&pool, &sources).await.unwrap();
        let mut stale = expected.clone();
        stale[0].revision += 1;
        assert!(
            reauthorize_game_heads(&pool, &sources, &stale, "operator:public", None)
                .await
                .is_err()
        );
        assert_eq!(read_game_heads(&pool, &sources).await.unwrap(), expected);
        let report = reauthorize_game_heads(
            &pool,
            &sources,
            &expected,
            "operator:public",
            Some("operator:public-provider"),
        )
        .await
        .unwrap();
        assert_eq!(report["updated"], 2);
        let promoted = read_game_heads(&pool, &sources).await.unwrap();
        for (old, new) in expected.iter().zip(&promoted) {
            assert_eq!(new.revision, 2);
            assert_eq!(new.content, old.content);
            assert_eq!(
                new.metadata[DOCUMENT_METADATA_KEY],
                old.metadata[DOCUMENT_METADATA_KEY]
            );
            assert!(
                dbrain_retrieval::knowledge_projection::project_knowledge(new)
                    .unwrap()
                    .is_some()
            );
        }
        let report = reauthorize_game_heads(
            &pool,
            &sources,
            &promoted,
            "operator:public",
            Some("operator:public-provider"),
        )
        .await
        .unwrap();
        assert_eq!(report["unchanged"], 2);
        assert_eq!(read_game_heads(&pool, &sources).await.unwrap(), promoted);
        let policy: ImportPolicy = serde_json::from_value(json!({"sources": {(SOURCES[0]): {
            "internal_read_allowed": true, "raw_retention_allowed": true, "publication_allowed": false,
            "provider_egress_allowed": false, "authorization_ref": "operator:original", "provenance_evidence_ref": "evidence:original", "allowed_scopes": ["internal_docs"]
        }}})).unwrap();
        let mut repeated_documents: Vec<KnowledgeDocument> = expected
            .iter()
            .map(|record| serde_json::from_str(&record.metadata[DOCUMENT_METADATA_KEY]).unwrap())
            .collect();
        for document in &mut repeated_documents {
            document.observed_at = "2026-10-10T12:00:00Z".into();
        }
        let encode = |documents: &[KnowledgeDocument]| {
            documents
                .iter()
                .map(|document| serde_json::to_string(document).unwrap())
                .collect::<Vec<_>>()
                .join("\n")
        };
        let repeated =
            crate::knowledge_contract::validate_knowledge_jsonl_str(&encode(&repeated_documents))
                .unwrap();
        let imported = crate::knowledge_import::import_prepared_knowledge(
            &store,
            prepare_validated_knowledge(&repeated, &policy, "game-files-v2").unwrap(),
        )
        .await
        .unwrap();
        assert!(imported.complete);
        assert_eq!(imported.storage.unchanged, 2);
        assert_eq!(imported.storage.inserted + imported.storage.updated, 0);
        assert_eq!(read_game_heads(&pool, &sources).await.unwrap(), promoted);
        let mut localization = repeated_documents[0].clone();
        localization.document_id = "game:1422450:data/localizations/german.json".into();
        localization.source_locator = "data/localizations/german.json".into();
        localization.title = "german.json".into();
        localization.language = "de".into();
        localization.metadata.insert(
            "relative_path".into(),
            json!("data/localizations/german.json"),
        );
        localization.metadata.insert(
            "original_relative_path".into(),
            json!("data/localizations/german.json"),
        );
        localization.facts = serde_json::from_value(json!(
            brain_storage::entity_profile::derivation::game_file_facts::extract_facts(
                "data/localizations/german.json",
                &localization.content
            )
            .0
        ))
        .unwrap();
        repeated_documents.push(localization.clone());
        let with_localization =
            crate::knowledge_contract::validate_knowledge_jsonl_str(&encode(&repeated_documents))
                .unwrap();
        let imported = crate::knowledge_import::import_prepared_knowledge(
            &store,
            prepare_validated_knowledge(&with_localization, &policy, "game-files-v2").unwrap(),
        )
        .await
        .unwrap();
        assert!(imported.complete);
        assert_eq!(imported.storage.unchanged, 2);
        assert_eq!(imported.storage.inserted, 1);
        let with_localization_heads = read_game_heads(&pool, &sources).await.unwrap();
        assert_eq!(with_localization_heads.len(), 3);
        for record in &promoted {
            assert!(with_localization_heads.contains(record));
        }
        assert_eq!(
            with_localization_heads
                .iter()
                .find(|record| record.logical_id == localization.document_id)
                .unwrap()
                .metadata[ORIGINAL_VERSION_KEY],
            expected[0].metadata[ORIGINAL_VERSION_KEY]
        );
        assert!(
            reauthorize_game_heads(&pool, &sources, &expected, "operator:public", None)
                .await
                .is_err()
        );
        let empty = fixture_path("{}", "data/json/empty.json");
        let mut withdrawn = fixture_path("{\"Urn\":12}", "data/json/withdrawn.json");
        withdrawn.tombstone = true;
        store.apply(&empty).await.unwrap();
        store.apply(&withdrawn).await.unwrap();
        let mixed = read_game_heads(&pool, &sources).await.unwrap();
        let mut stale_exclusion = mixed.clone();
        stale_exclusion
            .iter_mut()
            .find(|record| record.logical_id == withdrawn.logical_id)
            .unwrap()
            .revision += 1;
        assert!(reauthorize_game_heads(
            &pool,
            &sources,
            &stale_exclusion,
            "operator:public",
            Some("operator:public-provider")
        )
        .await
        .is_err());
        let report = reauthorize_game_heads(
            &pool,
            &sources,
            &mixed,
            "operator:public",
            Some("operator:public-provider"),
        )
        .await
        .unwrap();
        assert_eq!(report["updated"], 1);
        assert_eq!(report["unchanged"], 2);
        assert_eq!(report["selection"]["excluded_live"], 1);
        assert_eq!(report["selection"]["excluded_tombstones"], 1);
        let admitted = read_game_heads(&pool, &sources).await.unwrap();
        assert!(admitted.contains(&empty));
        assert!(admitted.contains(&withdrawn));
        let base_release = brain_contracts::CorpusRelease {
            release_id: "selection-base".into(),
            knowledge_version: "selection-base".into(),
            patch: "fixture".into(),
            created_at_epoch: 1,
            source_revisions: BTreeMap::new(),
        };
        store.publish_release(&base_release).await.unwrap();
        let base = store.snapshot(&base_release.release_id).await.unwrap();
        let selection = select_game_publication(&admitted).unwrap();
        let (candidate, expected_all) = brain_storage::PgStore::prepare_selected_imported_release(
            &base,
            &sources,
            admitted.clone(),
            &selection.selected,
            "selection-candidate",
            "selection-candidate",
            2,
        )
        .unwrap();
        assert_eq!(expected_all.len(), 5);
        assert_eq!(candidate.source_revisions[&sources[0]].len(), 3);
        let mut changed_exclusion = empty.clone();
        changed_exclusion.revision += 1;
        store.apply(&changed_exclusion).await.unwrap();
        assert!(store
            .publish_selected_imported_heads_checked(
                &base.release.release_id,
                &sources,
                &candidate,
                &expected_all,
                &selection.selected
            )
            .await
            .is_err());
        let fresh = read_game_heads(&pool, &sources).await.unwrap();
        let (candidate, expected_all) = brain_storage::PgStore::prepare_selected_imported_release(
            &base,
            &sources,
            fresh,
            &selection.selected,
            "selection-candidate",
            "selection-candidate",
            2,
        )
        .unwrap();
        assert_eq!(
            store
                .publish_selected_imported_heads_checked(
                    &base.release.release_id,
                    &sources,
                    &candidate,
                    &expected_all,
                    &selection.selected
                )
                .await
                .unwrap(),
            3
        );
        let published = store.snapshot(&candidate.release_id).await.unwrap();
        assert_eq!(published.revisions.len(), 3);
        assert!(!published.heads.contains(&changed_exclusion));
        assert!(!published.heads.contains(&withdrawn));
        let mut changed = promoted[1].clone();
        changed.revision += 1;
        let mut origin = origin_from_record(&changed).unwrap();
        origin.policy.raw_retention_allowed = false;
        origin.bind_record(&mut changed).unwrap();
        store.apply(&changed).await.unwrap();
        let revoked = read_game_heads(&pool, &sources).await.unwrap();
        assert!(
            reauthorize_game_heads(&pool, &sources, &revoked, "operator:new", None)
                .await
                .is_err()
        );
        assert_eq!(read_game_heads(&pool, &sources).await.unwrap(), revoked);
        sqlx::raw_sql("CREATE TABLE brain.source_documents(id bigint PRIMARY KEY,source text NOT NULL,content_hash text NOT NULL,url text,metadata jsonb NOT NULL); CREATE TABLE brain.entity_snapshots(id bigint PRIMARY KEY,source text NOT NULL,entity_type text NOT NULL,external_id text NOT NULL,canonical_name text,payload_hash text NOT NULL,payload jsonb NOT NULL,fetched_at timestamptz NOT NULL DEFAULT now(),source_document_id bigint)").execute(&pool).await.unwrap();
        let payload = json!({"name": "Prüfheld", "health": 700});
        let content = crate::store::json_string(&payload).unwrap();
        sqlx::query("INSERT INTO brain.source_documents VALUES(1,'deadlock_data',$1,NULL,'{}')")
            .bind("a".repeat(64))
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO brain.entity_snapshots(id,source,entity_type,external_id,canonical_name,payload_hash,payload,source_document_id) VALUES(1,'deadlock_data','hero','test','Prüfheld',$1,$2,1),(2,'deadlock_data','member','excluded','Nicht übernehmen',$1,$2,1)").bind(sha256_content(&content)).bind(&payload).execute(&pool).await.unwrap();
        let mut output = Vec::new();
        let exported =
            super::super::legacy_game::export_legacy_game(&pool, "legacy-entities", &mut output)
                .await
                .unwrap();
        assert_eq!(exported.documents, 1);
        assert_eq!(exported.excluded, 1);
        let validated =
            crate::knowledge_contract::validate_knowledge_jsonl(std::io::Cursor::new(output))
                .unwrap();
        assert_eq!(validated.documents()[0].document.content, content);
        sqlx::query("INSERT INTO brain.source_documents VALUES(2,'deadlock_data',$1,'https://example.invalid/private','{}'),(3,'deadlock_data',$1,'https://steamcommunity.com/games/1422450/announcements/detail/123','{}')")
            .bind("b".repeat(64)).execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO brain.entity_snapshots(id,source,entity_type,external_id,payload_hash,payload,source_document_id) VALUES(3,'deadlock_data','patchnote','private','invalid','{\"private\":true}',2),(4,'deadlock_data','patchnote','official',$1,$2,3),(5,'deadlock_data','hero','empty',$3,'{}',1)")
            .bind(sha256_content(&content)).bind(&payload).bind(sha256_content("{}"))
            .execute(&pool).await.unwrap();
        let mut patch_output = Vec::new();
        let patches = super::super::legacy_game::export_legacy_game(
            &pool,
            "legacy-patchnotes",
            &mut patch_output,
        )
        .await
        .unwrap();
        assert_eq!(patches.documents, 1);
        assert_eq!(
            patches.exclusion_reasons["official_public_patch_url_missing"],
            1
        );
        let patch_documents = crate::knowledge_contract::validate_knowledge_jsonl(
            std::io::Cursor::new(&patch_output),
        )
        .unwrap();
        assert_eq!(
            patch_documents.documents()[0].document.source_locator,
            "brain.entity_snapshots/4"
        );
        assert!(!String::from_utf8(patch_output)
            .unwrap()
            .contains("example.invalid/private"));
        let mut repeated_output = Vec::new();
        let entities = super::super::legacy_game::export_legacy_game(
            &pool,
            "legacy-entities",
            &mut repeated_output,
        )
        .await
        .unwrap();
        assert_eq!(entities.documents, 1);
        assert_eq!(
            entities.exclusion_reasons["public_game_facts_empty_or_unparsed:json"],
            1
        );
        let mut empty_buffer = [];
        let mut broken_writer = std::io::Cursor::new(empty_buffer.as_mut_slice());
        assert!(super::super::legacy_game::export_legacy_game(
            &pool,
            "legacy-entities",
            &mut broken_writer
        )
        .await
        .is_err());
        let policy: ImportPolicy = serde_json::from_value(json!({"sources": {"legacy-entities": {
            "internal_read_allowed": true, "raw_retention_allowed": true, "publication_allowed": false,
            "provider_egress_allowed": false, "authorization_ref": "operator:legacy-internal", "provenance_evidence_ref": "evidence:legacy-snapshot", "allowed_scopes": ["internal_docs"]
        }}})).unwrap();
        let imported = crate::knowledge_import::import_prepared_knowledge(
            &store,
            prepare_validated_knowledge(&validated, &policy, "legacy-game-snapshot-v1").unwrap(),
        )
        .await
        .unwrap();
        assert!(imported.complete);
        let legacy_sources = vec!["legacy-entities".to_owned()];
        let legacy_heads = read_game_heads(&pool, &legacy_sources).await.unwrap();
        reauthorize_game_heads(
            &pool,
            &legacy_sources,
            &legacy_heads,
            "operator:public",
            Some("operator:provider"),
        )
        .await
        .unwrap();
        let granted = read_game_heads(&pool, &legacy_sources).await.unwrap();
        assert!(
            dbrain_retrieval::knowledge_projection::project_knowledge(&granted[0])
                .unwrap()
                .is_some()
        );
        sqlx::query("UPDATE brain.entity_snapshots SET payload='{}' WHERE id=1")
            .execute(&pool)
            .await
            .unwrap();
        assert!(
            reauthorize_game_heads(&pool, &legacy_sources, &granted, "operator:new", None)
                .await
                .is_err()
        );
        assert_eq!(
            read_game_heads(&pool, &legacy_sources).await.unwrap(),
            granted
        );
        pool.close().await;
    }

    #[test]
    fn unverified_license_requires_bound_operator_grant_and_separate_egress_evidence() {
        let mut record = fixture();
        let mut document: Value =
            serde_json::from_str(&record.metadata[DOCUMENT_METADATA_KEY]).unwrap();
        document["license"]["name"] = json!("unverified");
        record
            .metadata
            .insert(DOCUMENT_METADATA_KEY.into(), document.to_string());
        let mut origin = origin_from_record(&record).unwrap();
        origin.policy.license =
            Observed::unknown(brain_contracts::value::UnknownReason::NotPresent);
        origin.bind_record(&mut record).unwrap();
        let next =
            authorize_game_record(&record, "operator:public", Some("operator:egress"), 2).unwrap();
        assert!(
            dbrain_retrieval::knowledge_projection::project_knowledge(&next)
                .unwrap()
                .is_some()
        );
        assert_eq!(
            origin_from_record(&next).unwrap().policy.license,
            origin.policy.license
        );
        let mut missing = next.clone();
        missing.metadata.remove(AUTHORIZATION_KEY);
        assert!(dbrain_retrieval::knowledge_projection::project_knowledge(&missing).is_err());
        for field in [
            "authorization_ref",
            "provider_egress_ref",
            "publication_basis",
        ] {
            let mut forged = next.clone();
            let mut grant: Value =
                serde_json::from_str(&forged.metadata[AUTHORIZATION_KEY]).unwrap();
            grant[field] = Value::Null;
            forged
                .metadata
                .insert(AUTHORIZATION_KEY.into(), grant.to_string());
            assert!(dbrain_retrieval::knowledge_projection::project_knowledge(&forged).is_err());
        }
        let mut foreign = next.clone();
        let mut document: Value =
            serde_json::from_str(&foreign.metadata[DOCUMENT_METADATA_KEY]).unwrap();
        document["source_kind"] = json!("wiki");
        foreign
            .metadata
            .insert(DOCUMENT_METADATA_KEY.into(), document.to_string());
        assert!(dbrain_retrieval::knowledge_projection::project_knowledge(&foreign).is_err());
    }

    #[test]
    fn source_name_alone_cannot_authorize_foreign_data() {
        let record = fixture();
        for field in [
            "app_id",
            "provenance",
            "original_relative_path",
            "source_revision",
        ] {
            let mut foreign = record.clone();
            let mut doc: Value =
                serde_json::from_str(&foreign.metadata[DOCUMENT_METADATA_KEY]).unwrap();
            doc["metadata"][field] = json!("foreign");
            foreign
                .metadata
                .insert(DOCUMENT_METADATA_KEY.into(), doc.to_string());
            assert!(authorize_game_record(&foreign, "operator:grant", None, 2).is_err());
        }
        let mut deleted = record;
        deleted.tombstone = true;
        assert!(authorize_game_record(&deleted, "operator:grant", None, 2).is_err());
        assert!(validate_sources(&["member-data".into()]).is_err());
    }
}
