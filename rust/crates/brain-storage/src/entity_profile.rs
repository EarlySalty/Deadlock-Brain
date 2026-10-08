use crate::{PgStore, StorageError};
use brain_contracts::{
    entity_profile::{
        EntityIdentity, EntityProfile, EntityProfileFact, PatchStoryChange, PatchStoryProvenance,
        PatchValidity, ProfileConflict, ProfileProvenance, ProfileSourceKind, RestrictedPatchLine,
        ENTITY_PROFILE_VERSION,
    },
    source::origin_from_record,
    SourceRecordV2,
};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

#[path = "entity_semantic.rs"]
pub mod semantic;

#[path = "entity_compact.rs"]
pub mod compact;

#[path = "entity_derivation.rs"]
pub mod derivation;

fn invalid(message: &str) -> StorageError {
    StorageError::Json(<serde_json::Error as serde::de::Error>::custom(message))
}

pub fn project_entity_facts(
    record: &SourceRecordV2,
    fact_ids: &[String],
) -> crate::Result<Vec<EntityProfileFact>> {
    EntityDocumentContext::new(record)?.project(fact_ids)
}

/// Dokumentlokaler Originalbeleg. Die private Bindung verhindert den Austausch der Quelle.
pub struct EntityDocumentContext<'a> {
    record: &'a SourceRecordV2,
    document: Value,
    by_id: BTreeMap<String, usize>,
    origin: brain_contracts::source::OriginArtifact,
    source_kind: ProfileSourceKind,
}

impl<'a> EntityDocumentContext<'a> {
    pub fn new(record: &'a SourceRecordV2) -> crate::Result<Self> {
        record.validate()?;
        let origin = origin_from_record(record).map_err(|_| invalid("Quellherkunft fehlt"))?;
        let document: Value = serde_json::from_str(
            record
                .metadata
                .get(crate::source_versions::DOCUMENT_METADATA_KEY)
                .ok_or_else(|| invalid("Originaldokument fehlt"))?,
        )?;
        if document["document_id"] != record.logical_id
            || document["source_id"] != record.source_id
            || document["content_sha256"] != record.content_hash
            || document["content"] != record.content
        {
            return Err(invalid("Dokument und Quelle stimmen nicht überein"));
        }
        let source_kind = match document["source_kind"].as_str() {
            Some("wiki") => ProfileSourceKind::Wiki,
            Some("game_file") => ProfileSourceKind::GameFile,
            _ => return Err(invalid("Quellenart fehlt")),
        };
        let facts = document["facts"]
            .as_array()
            .ok_or_else(|| invalid("Fakten fehlen"))?;
        let mut by_id = BTreeMap::new();
        for (position, fact) in facts.iter().enumerate() {
            let id = fact["fact_id"]
                .as_str()
                .ok_or_else(|| invalid("Fakten-ID fehlt"))?;
            if by_id.insert(id.to_owned(), position).is_some() {
                return Err(invalid("Faktenzuordnung ist nicht eindeutig"));
            }
        }
        Ok(Self {
            record,
            document,
            by_id,
            origin,
            source_kind,
        })
    }

    pub fn record(&self) -> &'a SourceRecordV2 {
        self.record
    }
    pub fn document(&self) -> &Value {
        &self.document
    }
    pub(crate) fn fact_index(&self) -> &BTreeMap<String, usize> {
        &self.by_id
    }

    pub fn project(&self, fact_ids: &[String]) -> crate::Result<Vec<EntityProfileFact>> {
        let document = &self.document;
        let facts = document["facts"]
            .as_array()
            .ok_or_else(|| invalid("Fakten fehlen"))?;
        let mut seen = BTreeSet::new();
        let mut result = Vec::new();
        for id in fact_ids {
            if !seen.insert(id) {
                return Err(invalid("Faktenzuordnung ist nicht eindeutig"));
            }
            let fact = self
                .by_id
                .get(id)
                .and_then(|position| facts.get(*position))
                .ok_or_else(|| invalid("Faktenzuordnung ist nicht eindeutig"))?;
            let text = |key: &str| {
                fact[key]
                    .as_str()
                    .map(str::to_owned)
                    .ok_or_else(|| invalid("Faktenfeld fehlt"))
            };
            result.push(EntityProfileFact {
                fact_id: id.clone(),
                subject: text("subject")?,
                predicate: text("predicate")?,
                value: fact["value"].clone(),
                unit: serde_json::from_value(fact["unit"].clone())?,
                qualifiers: serde_json::from_value(fact["qualifiers"].clone())?,
                evidence_status: text("evidence_status")?,
                validity: PatchValidity::Unknown {
                    reason: "Quelle belegt keine Patchgrenzen".into(),
                },
                provenance: ProfileProvenance {
                    source_kind: self.source_kind,
                    origin: self.origin.clone(),
                    original_revision: document["revision"]
                        .as_str()
                        .ok_or_else(|| invalid("Originalrevision fehlt"))?
                        .into(),
                    observed_at: document["observed_at"]
                        .as_str()
                        .ok_or_else(|| invalid("Quellenzeit fehlt"))?
                        .into(),
                    source_span: serde_json::from_value(fact["source_span"].clone())?,
                    license: document["license"].clone(),
                    document_metadata: serde_json::from_value(document["metadata"].clone())?,
                },
            });
        }
        Ok(result)
    }
}

pub fn validity_contains(validity: &PatchValidity, patch: &str) -> bool {
    match validity {
        PatchValidity::Unknown { .. } => false,
        PatchValidity::Known {
            from_patch,
            to_patch_exclusive,
            through_patch_inclusive,
            ..
        } => {
            from_patch.as_str() <= patch
                && to_patch_exclusive.as_deref().is_none_or(|end| patch < end)
                && through_patch_inclusive
                    .as_deref()
                    .is_none_or(|end| patch <= end)
        }
    }
}

pub fn assemble_profile(
    entity: EntityIdentity,
    patch: Option<String>,
    facts: Vec<EntityProfileFact>,
    patch_story: Vec<PatchStoryChange>,
) -> EntityProfile {
    let mut grouped = BTreeMap::new();
    let mut unknowns = Vec::new();
    for fact in &facts {
        let mut comparison_qualifiers = fact.qualifiers.clone();
        if fact.provenance.source_kind == ProfileSourceKind::GameFile
            && fact.predicate == "file.kv_value"
        {
            let pointer = (|| {
                let path = fact
                    .provenance
                    .document_metadata
                    .get("relative_path")?
                    .as_str()?;
                if fact.subject.strip_prefix("game_file:")? != path
                    || fact
                        .provenance
                        .document_metadata
                        .get("parse_status")?
                        .as_str()?
                        != "kv1_lossless_entries"
                {
                    return None;
                }
                let pointer = fact
                    .provenance
                    .source_span
                    .as_deref()?
                    .strip_prefix(path)?
                    .strip_prefix(':')?;
                if fact.fact_id.strip_prefix("kv:")? != pointer || !pointer.starts_with('/') {
                    return None;
                }
                let key = fact
                    .qualifiers
                    .get("source_key")?
                    .as_str()?
                    .replace('~', "~0")
                    .replace('/', "~1");
                let occurrence = fact.qualifiers.get("occurrence")?.as_u64()?;
                if !pointer.ends_with(&format!("/{key}/{occurrence}"))
                    || fact
                        .qualifiers
                        .get("source_pointer")
                        .is_some_and(|value| value.as_str() != Some(pointer))
                {
                    return None;
                }
                Some(pointer)
            })();
            let Some(pointer) = pointer else {
                unknowns.push(format!(
                    "KV1-Feldpfad ist nicht vollständig belegt: {}",
                    fact_reference(fact)
                ));
                continue;
            };
            comparison_qualifiers.insert("source_pointer".into(), Value::String(pointer.into()));
        }
        for key in ["source_lexeme", "numeric_representation"] {
            comparison_qualifiers.remove(key);
        }
        grouped
            .entry((
                fact.predicate.clone(),
                matches!(
                    fact.predicate.as_str(),
                    "file.kv_value" | "file.json_value" | "file.kv3_value" | "wiki.data.value"
                )
                .then(|| {
                    (
                        fact.subject.clone(),
                        fact.provenance.origin.identity.logical_id.clone(),
                    )
                }),
                serde_json::to_string(&comparison_qualifiers).unwrap_or_default(),
                fact.unit.clone(),
            ))
            .or_insert_with(Vec::new)
            .push(fact);
    }
    let mut conflicts = Vec::new();
    for ((predicate, _, _, _), group) in grouped {
        if group.iter().any(|f| f.value != group[0].value) {
            let numeric = group.iter().all(|f| {
                f.value.is_number()
                    || f.qualifiers
                        .get("numeric_representation")
                        .and_then(Value::as_str)
                        == Some("source_numeric_lexeme")
            });
            let mut preferred: Vec<_> = group
                .iter()
                .filter(|f| {
                    f.provenance.source_kind
                        == if numeric {
                            ProfileSourceKind::GameFile
                        } else {
                            ProfileSourceKind::Wiki
                        }
                })
                .collect();
            preferred.sort_by_key(|fact| fact_reference(fact));
            let consistent = preferred
                .first()
                .is_some_and(|first| preferred.iter().all(|fact| fact.value == first.value));
            conflicts.push(ProfileConflict {
                predicate,
                preferred_fact_id: if consistent {
                    preferred.first().map(|fact| fact_reference(fact))
                } else {
                    None
                },
                fact_ids: group.iter().map(|f| fact_reference(f)).collect(),
                reason: if numeric {
                    "Git-Spieldaten haben Vorrang bei Zahlen; abweichende Belege bleiben erhalten"
                } else {
                    "Wiki hat Vorrang bei Beschreibungen; abweichende Belege bleiben erhalten"
                }
                .into(),
            });
        }
    }
    let mut source_state: Vec<_> = facts
        .iter()
        .map(|f| {
            format!(
                "{}:{}:{}",
                f.provenance.origin.identity.source_id,
                f.provenance.origin.identity.logical_id,
                f.provenance.original_revision
            )
        })
        .collect();
    source_state.sort();
    source_state.dedup();
    if facts
        .iter()
        .any(|f| matches!(f.validity, PatchValidity::Unknown { .. }))
    {
        unknowns.push("Patchgültigkeit ist für einen Teil der Fakten unbekannt".into());
    }
    let (context, facts) = facts
        .into_iter()
        .partition(|f| f.evidence_status == "source_statement" && f.value.is_string());
    EntityProfile {
        contract_version: ENTITY_PROFILE_VERSION.into(),
        entity,
        patch,
        source_state,
        facts,
        context,
        conflicts,
        patch_story,
        unknowns,
    }
}

fn fact_reference(fact: &EntityProfileFact) -> String {
    format!(
        "{}:{}:{}:{}",
        fact.provenance.origin.identity.source_id,
        fact.provenance.origin.identity.logical_id,
        fact.provenance.original_revision,
        fact.fact_id
    )
}

pub fn consumer_entity_identity(identity: &EntityIdentity) -> EntityIdentity {
    EntityIdentity {
        entity_key: identity.entity_key.clone(),
        kind: identity.kind,
        name: identity.name.clone(),
        aliases: Vec::new(),
        identity_evidence: Vec::new(),
    }
}

pub struct PinnedMirrorBundle {
    game_context: brain_contracts::PinnedGameContext,
    assets: BTreeMap<String, crate::asset_mirror::MirroredAssets>,
}

impl PinnedMirrorBundle {
    pub fn game_context(&self) -> &brain_contracts::PinnedGameContext {
        &self.game_context
    }

    pub fn asset(
        &self,
        kind: &str,
        language: Option<&str>,
    ) -> std::result::Result<&crate::asset_mirror::MirroredAssets, brain_contracts::PortError> {
        let key = crate::asset_mirror::mirrored_asset_key(kind, language)
            .map_err(|_| mirror_error("Ungültiger Spiegelendpunkt"))?;
        self.assets
            .get(&key)
            .ok_or_else(|| mirror_error("Endpunkt gehört nicht zum gebundenen Spiegel"))
    }
}

#[derive(Clone)]
pub struct MirroredGameContextReader {
    pool: sqlx::PgPool,
    runtime: tokio::runtime::Handle,
    language: brain_contracts::tools::ToolLanguage,
}

fn mirror_error(message: &str) -> brain_contracts::PortError {
    brain_contracts::PortError::Unavailable(message.to_owned())
}

impl MirroredGameContextReader {
    pub fn new(
        pool: sqlx::PgPool,
        runtime: tokio::runtime::Handle,
        language: brain_contracts::tools::ToolLanguage,
    ) -> std::result::Result<Self, brain_contracts::PortError> {
        if runtime.runtime_flavor() != tokio::runtime::RuntimeFlavor::MultiThread {
            return Err(mirror_error(
                "Spiegelleser benötigt die vorhandene Mehrthread-Runtime",
            ));
        }
        Ok(Self {
            pool,
            runtime,
            language,
        })
    }

    pub fn read_latest(
        &self,
        context: &brain_contracts::AuthorizedContext,
    ) -> std::result::Result<PinnedMirrorBundle, brain_contracts::PortError> {
        self.drive(context, async {
            let version = crate::asset_mirror::latest_mirrored_client_version(&self.pool).await?;
            let anchor = crate::asset_mirror::load_mirrored_assets_with_receipt(
                &self.pool,
                version,
                "heroes_all",
                Some("english"),
            )
            .await?;
            self.read_run(version, anchor.receipt.source_run_id).await
        })
    }

    pub fn read_pinned(
        &self,
        context: &brain_contracts::AuthorizedContext,
        pin: &brain_contracts::PinnedGameContext,
    ) -> std::result::Result<PinnedMirrorBundle, brain_contracts::PortError> {
        pin.validate()?;
        if pin.language != self.language {
            return Err(mirror_error(
                "Spiegelsprache weicht von der Serverbindung ab",
            ));
        }
        let mut parts = pin.mechanic_revision.split(':');
        let run = match (parts.next(), parts.next(), parts.next(), parts.next()) {
            (Some("mirror.v1"), Some(run), Some(hash), None)
                if hash.len() == 64 && hash.bytes().all(|byte| byte.is_ascii_hexdigit()) =>
            {
                run.parse::<i64>().ok().filter(|id| *id > 0)
            }
            _ => None,
        }
        .ok_or_else(|| mirror_error("Gebundener Spiegellauf fehlt"))?;
        let bundle = self.drive(context, self.read_run(pin.client_version, run))?;
        if bundle.game_context() != pin {
            return Err(mirror_error(
                "Spiegelbelege oder Payload haben sich geändert",
            ));
        }
        Ok(bundle)
    }

    fn drive<T>(
        &self,
        context: &brain_contracts::AuthorizedContext,
        read: impl std::future::Future<Output = anyhow::Result<T>>,
    ) -> std::result::Result<T, brain_contracts::PortError> {
        let deadline = context
            .request_deadline
            .as_ref()
            .ok_or_else(|| mirror_error("Ursprüngliche Anfragefrist fehlt"))?;
        deadline.check()?;
        if tokio::runtime::Handle::try_current().is_ok_and(|handle| {
            handle.runtime_flavor() == tokio::runtime::RuntimeFlavor::CurrentThread
        }) {
            return Err(mirror_error(
                "Synchroner Spiegelleser benötigt den blockierenden Servicepfad",
            ));
        }
        tokio::task::block_in_place(|| {
            self.runtime.block_on(async {
            tokio::pin!(read);
            loop {
                let remaining = deadline.remaining()?;
                tokio::select! {
                    biased;
                    result = &mut read => {
                        deadline.check()?;
                        return result.map_err(|_| mirror_error("Lokaler Spiegel konnte nicht bestätigt werden"));
                    }
                    _ = tokio::time::sleep(remaining.min(std::time::Duration::from_millis(5))) => {}
                }
            }
        })
        })
    }

    async fn read_run(&self, version: i64, run: i64) -> anyhow::Result<PinnedMirrorBundle> {
        use sha2::{Digest, Sha256};
        let mut assets = BTreeMap::new();
        let mut anchor: Option<crate::asset_mirror::AssetMirrorReceipt> = None;
        for kind in crate::asset_mirror::MIRRORED_ASSET_KINDS {
            for language in crate::asset_mirror::mirrored_asset_languages(kind)
                .ok_or_else(|| anyhow::anyhow!("Unbekannte Spiegelart"))?
            {
                let language = (!language.is_empty()).then_some(*language);
                let asset = crate::asset_mirror::load_mirrored_assets_for_run(
                    &self.pool, run, version, kind, language,
                )
                .await?;
                let receipt = &asset.receipt;
                anyhow::ensure!(
                    receipt.source_run_id == run && receipt.client_version == version,
                    "Spiegellauf weicht ab"
                );
                if let Some(anchor) = &anchor {
                    anyhow::ensure!(
                        receipt.parser_revision == anchor.parser_revision
                            && receipt.run_started_at == anchor.run_started_at
                            && receipt.run_finished_at == anchor.run_finished_at
                            && receipt.mirrored_at == anchor.mirrored_at
                            && receipt.checked_at == anchor.checked_at
                            && serde_json::to_value(&receipt.manifest)?
                                == serde_json::to_value(&anchor.manifest)?,
                        "Spiegelendpunkte gehören nicht zu demselben vollständigen Lauf"
                    );
                } else {
                    anchor = Some(receipt.clone());
                }
                let key = crate::asset_mirror::mirrored_asset_key(kind, language)?;
                anyhow::ensure!(
                    assets.insert(key, asset).is_none(),
                    "Doppelter Spiegelendpunkt"
                );
            }
        }
        anyhow::ensure!(anchor.is_some(), "Spiegelbelege fehlen");
        let digest = format!("{:x}", Sha256::digest(serde_json::to_vec(&assets)?));
        let game_context = brain_contracts::PinnedGameContext {
            client_version: version,
            language: self.language,
            mechanic_revision: format!("mirror.v1:{run}:{digest}"),
        };
        game_context
            .validate()
            .map_err(|_| anyhow::anyhow!("Ungültige Spiegelbindung"))?;
        Ok(PinnedMirrorBundle {
            game_context,
            assets,
        })
    }
}

impl brain_contracts::tools::GameContextResolver for MirroredGameContextReader {
    fn resolve(
        &self,
        _query: &brain_contracts::Query,
        context: &brain_contracts::AuthorizedContext,
    ) -> std::result::Result<Option<brain_contracts::PinnedGameContext>, brain_contracts::PortError>
    {
        Ok(Some(self.read_latest(context)?.game_context))
    }

    fn validate(
        &self,
        _query: &brain_contracts::Query,
        context: &brain_contracts::AuthorizedContext,
        game_context: Option<&brain_contracts::PinnedGameContext>,
    ) -> std::result::Result<(), brain_contracts::PortError> {
        let pin = game_context.ok_or_else(|| mirror_error("Gebundener Spiegel fehlt"))?;
        pin.validate()?;
        if self.read_latest(context)?.game_context() != pin {
            return Err(mirror_error(
                "Serverseitiger Spiegelstand hat sich geändert",
            ));
        }
        Ok(())
    }
}

impl PgStore {
    pub async fn store_entity_fact_bindings(
        &self,
        entity: &EntityIdentity,
        record: &SourceRecordV2,
        fact_ids: &[String],
    ) -> crate::Result<usize> {
        let context = EntityDocumentContext::new(record)?;
        self.store_entity_fact_bindings_with_context(entity, &context, fact_ids)
            .await
    }

    pub async fn store_entity_fact_bindings_with_context(
        &self,
        entity: &EntityIdentity,
        context: &EntityDocumentContext<'_>,
        fact_ids: &[String],
    ) -> crate::Result<usize> {
        let record = context.record();
        if entity.entity_key.trim().is_empty()
            || entity.name.trim().is_empty()
            || entity.identity_evidence.is_empty()
            || fact_ids.len() > 10_000
        {
            return Err(invalid("Belegte Entitätszuordnung fehlt oder ist zu groß"));
        }
        let identity = serde_json::to_value(entity)?;
        let mut tx = self.pool.begin().await?;
        crate::pg_jobs::lock_source(&mut tx, &record.source_id).await?;
        let revision =
            i64::try_from(record.revision).map_err(|_| invalid("Revision ist zu groß"))?;
        let stored: Value = sqlx::query_scalar("SELECT record_json FROM brain.source_record_revisions WHERE source_id=$1 AND logical_id=$2 AND revision=$3")
            .bind(&record.source_id).bind(&record.logical_id).bind(revision).fetch_one(&mut *tx).await?;
        let original: SourceRecordV2 = serde_json::from_value(stored)?;
        if serde_json::to_value(&original)? != serde_json::to_value(record)? {
            return Err(invalid(
                "Faktenbindung widerspricht der gespeicherten Quellrevision",
            ));
        }
        let facts = context.project(fact_ids)?;
        sqlx::query("INSERT INTO brain.entity_profile_entities_v1(entity_key,identity_json) VALUES($1,$2) ON CONFLICT(entity_key) DO NOTHING").bind(&entity.entity_key).bind(&identity).execute(&mut *tx).await?;
        let stored_identity: Value = sqlx::query_scalar("SELECT identity_json FROM brain.entity_profile_entities_v1 WHERE entity_key=$1 FOR UPDATE")
            .bind(&entity.entity_key).fetch_one(&mut *tx).await?;
        let stored_entity: EntityIdentity = serde_json::from_value(stored_identity)?;
        if stored_entity.kind != entity.kind {
            return Err(invalid("Entitätsart widerspricht vorhandener Zuordnung"));
        }
        let semantic_ready: bool = sqlx::query_scalar(
            "SELECT to_regclass('brain.entity_semantic_projections_v1') IS NOT NULL",
        )
        .fetch_one(&mut *tx)
        .await?;
        let mut inserted = 0;
        for fact in facts {
            let encoded = serde_json::to_string(&fact)?;
            inserted += sqlx::query("INSERT INTO brain.entity_profile_facts_v1(entity_key,source_id,logical_id,revision,fact_id,fact_json,binding_identity_json) VALUES($1,$2,$3,$4,$5,$6,$7) ON CONFLICT(entity_key,source_id,logical_id,revision,fact_id) DO UPDATE SET binding_identity_json=EXCLUDED.binding_identity_json WHERE brain.entity_profile_facts_v1.binding_identity_json IS NULL")
                .bind(&entity.entity_key).bind(&record.source_id).bind(&record.logical_id).bind(revision).bind(&fact.fact_id).bind(&encoded).bind(&identity).execute(&mut *tx).await?.rows_affected() as usize;
            let stored: String = sqlx::query_scalar("SELECT fact_json FROM brain.entity_profile_facts_v1 WHERE entity_key=$1 AND source_id=$2 AND logical_id=$3 AND revision=$4 AND fact_id=$5")
                .bind(&entity.entity_key).bind(&record.source_id).bind(&record.logical_id).bind(revision).bind(&fact.fact_id).fetch_one(&mut *tx).await?;
            let stored_identity: Value = sqlx::query_scalar("SELECT binding_identity_json FROM brain.entity_profile_facts_v1 WHERE entity_key=$1 AND source_id=$2 AND logical_id=$3 AND revision=$4 AND fact_id=$5")
                .bind(&entity.entity_key).bind(&record.source_id).bind(&record.logical_id).bind(revision).bind(&fact.fact_id).fetch_one(&mut *tx).await?;
            if stored != encoded {
                return Err(invalid("Faktenbeleg widerspricht vorhandenem Import"));
            }
            if stored_identity != identity {
                let previous: EntityIdentity = serde_json::from_value(stored_identity)?;
                let mut expected = entity.clone();
                expected.aliases = previous.aliases.clone();
                expected.identity_evidence = previous.identity_evidence.clone();
                if previous != expected
                    || previous
                        .identity_evidence
                        .iter()
                        .any(|evidence| !entity.identity_evidence.contains(evidence))
                    || previous
                        .aliases
                        .iter()
                        .any(|alias| !entity.aliases.contains(alias))
                {
                    return Err(invalid("Faktenbeleg widerspricht vorhandenem Import"));
                }
                let projection: Option<(String, String, String, Option<String>)> = if semantic_ready
                {
                    sqlx::query_as("SELECT relative_pointer,semantic_predicate,semantic_qualifiers_json,semantic_unit FROM brain.entity_semantic_projections_v1 WHERE entity_key=$1 AND source_id=$2 AND logical_id=$3 AND revision=$4 AND fact_id=$5")
                        .bind(&entity.entity_key).bind(&record.source_id).bind(&record.logical_id).bind(revision).bind(&fact.fact_id).fetch_optional(&mut *tx).await?
                } else {
                    None
                };
                if let Some((relative_pointer, predicate, qualifiers, unit)) = projection {
                    let projection = semantic::SemanticProjection {
                        relative_pointer,
                        predicate,
                        qualifiers: serde_json::from_str(&qualifiers)?,
                        unit,
                    };
                    semantic::project_semantic_fact_with_context(
                        &fact,
                        &projection,
                        context,
                        &previous,
                    )?;
                    semantic::project_semantic_fact_with_context(
                        &fact,
                        &projection,
                        context,
                        entity,
                    )?;
                }
                sqlx::query("UPDATE brain.entity_profile_facts_v1 SET binding_identity_json=$6 WHERE entity_key=$1 AND source_id=$2 AND logical_id=$3 AND revision=$4 AND fact_id=$5")
                    .bind(&entity.entity_key).bind(&record.source_id).bind(&record.logical_id).bind(revision).bind(&fact.fact_id).bind(&identity).execute(&mut *tx).await?;
            }
        }
        tx.commit().await?;
        Ok(inserted)
    }

    pub async fn read_entity_profile(
        &self,
        entity_key: &str,
        release_id: &str,
        principal: &brain_contracts::Principal,
        patch: Option<&str>,
    ) -> crate::Result<Option<EntityProfile>> {
        let snapshot = self
            .snapshot(release_id)
            .await
            .map_err(|_| invalid("Freigegebener Quellenstand fehlt"))?;
        let visible = snapshot
            .authorized(principal, false)
            .map_err(|_| invalid("Quellenfreigabe fehlt"))?;
        let mut entity: Option<EntityIdentity> = None;
        let mut facts = Vec::new();
        let mut historical_unknown = false;
        let semantic_ready: bool = sqlx::query_scalar(
            "SELECT to_regclass('brain.entity_semantic_projections_v1') IS NOT NULL",
        )
        .fetch_one(&self.pool)
        .await?;
        for source in visible {
            let original = snapshot
                .revisions
                .iter()
                .find(|original| {
                    original.source_id == source.source_id
                        && original.logical_id == source.logical_id
                        && original.revision == source.revision
                })
                .ok_or_else(|| invalid("Gespeicherte Quellrevision fehlt"))?;
            let values: Vec<(String, Option<Value>)> = sqlx::query_as("SELECT fact_json,binding_identity_json FROM brain.entity_profile_facts_v1 WHERE entity_key=$1 AND source_id=$2 AND logical_id=$3 AND revision=$4 ORDER BY fact_id")
                .bind(entity_key).bind(&source.source_id).bind(&source.logical_id).bind(source.revision as i64).fetch_all(&self.pool).await?;
            for (value, binding_identity) in values {
                let fact: EntityProfileFact = serde_json::from_str(&value)?;
                if project_entity_facts(original, std::slice::from_ref(&fact.fact_id))?[0] != fact {
                    return Err(invalid("Fakt und freigegebene Quelle widersprechen sich"));
                }
                let binding: EntityIdentity =
                    serde_json::from_value(binding_identity.ok_or_else(|| {
                        invalid("Belegte Bindungsidentität fehlt; erneute Zuordnung erforderlich")
                    })?)?;
                if binding.entity_key != entity_key
                    || binding.name.trim().is_empty()
                    || binding.identity_evidence.is_empty()
                {
                    return Err(invalid("Gespeicherte Bindungsidentität ist ungültig"));
                }
                let semantic_identity = binding.clone();
                if let Some(identity) = &mut entity {
                    if identity.kind != binding.kind {
                        return Err(invalid("Autorisierte Entitätsbelege widersprechen sich"));
                    }
                    for name in std::iter::once(binding.name).chain(binding.aliases) {
                        if identity.name != name && !identity.aliases.contains(&name) {
                            identity.aliases.push(name);
                        }
                    }
                    for evidence in binding.identity_evidence {
                        if !identity.identity_evidence.contains(&evidence) {
                            identity.identity_evidence.push(evidence);
                        }
                    }
                } else {
                    entity = Some(binding);
                }
                let projection: Option<(String, String, String, Option<String>)> = if semantic_ready
                {
                    sqlx::query_as("SELECT relative_pointer,semantic_predicate,semantic_qualifiers_json,semantic_unit FROM brain.entity_semantic_projections_v1 WHERE entity_key=$1 AND source_id=$2 AND logical_id=$3 AND revision=$4 AND fact_id=$5")
                    .bind(entity_key).bind(&source.source_id).bind(&source.logical_id).bind(source.revision as i64).bind(&fact.fact_id).fetch_optional(&self.pool).await?
                } else {
                    None
                };
                let projected = if let Some((relative_pointer, predicate, qualifiers, unit)) =
                    projection
                {
                    let semantic = semantic::SemanticProjection {
                        relative_pointer,
                        predicate,
                        qualifiers: serde_json::from_str(&qualifiers)?,
                        unit,
                    };
                    semantic::project_semantic_fact(&fact, &semantic, original, &semantic_identity)?
                } else {
                    fact
                };
                if let Some(patch) = patch {
                    historical_unknown |=
                        matches!(projected.validity, PatchValidity::Unknown { .. });
                    if !validity_contains(&projected.validity, patch) {
                        continue;
                    }
                }
                facts.push(projected);
            }
        }
        let Some(entity) = entity else {
            return Ok(None);
        };
        let mut story = self.entity_patch_story(&entity).await?;
        if let Some(patch) = patch {
            story.retain(|change| change.patch_date.as_str() <= patch);
        }
        let story = derivation::consumer_patch_story(&entity, &story)?;
        let mut profile = assemble_profile(
            consumer_entity_identity(&entity),
            patch.map(str::to_owned),
            facts,
            story,
        );
        if historical_unknown {
            profile.unknowns.push(
                "Historische Fakten ohne belegte Patchgrenzen wurden nicht als gültig ausgegeben"
                    .into(),
            );
        }
        Ok(Some(profile))
    }

    pub async fn entity_patch_story(
        &self,
        entity: &EntityIdentity,
    ) -> crate::Result<Vec<PatchStoryChange>> {
        let names: Vec<_> = std::iter::once(entity.name.clone())
            .chain(entity.aliases.clone())
            .collect();
        let kind = match entity.kind {
            brain_contracts::entity_profile::EntityKind::Hero => "hero",
            brain_contracts::entity_profile::EntityKind::Ability => "ability",
            brain_contracts::entity_profile::EntityKind::Item => "item",
        };
        let mut tx = self.pool.begin().await?;
        sqlx::raw_sql("SET TRANSACTION READ ONLY; SET LOCAL statement_timeout='5000ms'")
            .execute(&mut *tx)
            .await?;
        let names: Vec<String> = sqlx::query_scalar(NORMALIZE_PATCH_STORY_NAMES_SQL)
            .bind(names)
            .fetch_one(&mut *tx)
            .await?;
        // Der konkrete Entitätstyp muss vor der View-Aufbereitung feststehen.
        let story: Vec<Value> = sqlx::query_scalar(PATCH_STORY_SQL)
            .persistent(false)
            .bind(names)
            .bind(kind)
            .fetch_all(&mut *tx)
            .await?;
        tx.commit().await?;
        story.into_iter().map(decode_patch_change).collect()
    }
}

pub(crate) const NORMALIZE_PATCH_STORY_NAMES_SQL: &str =
    "SELECT ARRAY(SELECT lower(n) FROM unnest($1::text[]) n)";
pub(crate) const PATCH_STORY_SQL: &str = "SELECT to_jsonb(c) FROM brain.patch_changes c WHERE (c.entity_type=$2 AND lower(c.entity_name)=ANY($1::text[])) OR ($2='ability' AND c.entity_type='hero' AND lower(c.ability_name)=ANY($1::text[])) ORDER BY c.patch_date,c.stat_name";

pub(crate) fn decode_patch_change(value: Value) -> crate::Result<PatchStoryChange> {
    let mut fields = value
        .as_object()
        .cloned()
        .ok_or_else(|| invalid("Patchzeile ist kein Objekt"))?;
    let mut text = |key: &str| -> crate::Result<Option<String>> {
        serde_json::from_value(fields.remove(key).unwrap_or(Value::Null))
            .map_err(StorageError::from)
    };
    let patch_date = text("patch_date")?.ok_or_else(|| invalid("Patchdatum fehlt"))?;
    let patch_title = text("patch_title")?;
    let entity_type = text("entity_type")?;
    let entity_name = text("entity_name")?;
    let ability_name = text("ability_name")?;
    let stat_name = text("stat_name")?;
    let change_type = text("change_type")?;
    let numeric_direction = text("numeric_direction")?;
    let source_url = text("patch_url")?;
    let raw_line = text("raw_line")?;
    Ok(PatchStoryChange {
        provenance: PatchStoryProvenance {
            relation: "brain.patch_changes".into(),
            source_url,
            evidence_ref: format!(
                "brain.patch_changes:{}:{}:{}:{}",
                patch_date,
                entity_name.as_deref().unwrap_or(""),
                ability_name.as_deref().unwrap_or(""),
                stat_name.as_deref().unwrap_or("")
            ),
        },
        patch_date,
        patch_title,
        entity_type,
        entity_name,
        ability_name,
        stat_name,
        old_value: fields.remove("old_value").unwrap_or(Value::Null),
        new_value: fields.remove("new_value").unwrap_or(Value::Null),
        change_type,
        numeric_direction,
        confidence: fields.remove("confidence").unwrap_or(Value::Null),
        original_line: RestrictedPatchLine {
            text: raw_line,
            redistribution_allowed: false,
        },
        additional_fields: fields,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn patch_story_retains_structured_values_and_restricts_original_line() {
        let row = serde_json::json!({"patch_date":"2026-09-16","patch_title":"Fixture","entity_name":"Test","ability_name":null,"stat_name":"health","old_value":"100.00000000000000001","new_value":123,"raw_line":"Originalzeile","patch_url":"https://example.org/patch","confidence":0.8,"retained_extra":"Quelle"});
        let change = decode_patch_change(row).unwrap();
        assert_eq!(change.old_value, "100.00000000000000001");
        assert_eq!(change.new_value, 123);
        assert_eq!(change.provenance.relation, "brain.patch_changes");
        assert_eq!(change.original_line.text.as_deref(), Some("Originalzeile"));
        assert!(!change.original_line.redistribution_allowed);
        assert_eq!(change.additional_fields["retained_extra"], "Quelle");
    }
}
