use crate::{PgStore, StorageError};
use brain_contracts::{
    entity_profile::{
        EntityIdentity, EntityKind, EntityProfile, EntityProfileFact, PatchStoryChange,
        PatchStoryProvenance, PatchValidity, ProfileConflict, ProfileProvenance, ProfileSourceKind,
        RestrictedPatchLine, ENTITY_PROFILE_VERSION,
    },
    source::origin_from_record,
    SourceRecordV2,
};
use serde_json::Value;
use std::collections::BTreeMap;

fn invalid(message: &str) -> StorageError {
    StorageError::Json(<serde_json::Error as serde::de::Error>::custom(message))
}

pub fn project_entity_facts(
    record: &SourceRecordV2,
    fact_ids: &[String],
) -> crate::Result<Vec<EntityProfileFact>> {
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
    let mut result = Vec::new();
    for id in fact_ids {
        let matches: Vec<_> = facts
            .iter()
            .filter(|f| f["fact_id"].as_str() == Some(id))
            .collect();
        if matches.len() != 1 || result.iter().any(|f: &EntityProfileFact| f.fact_id == *id) {
            return Err(invalid("Faktenzuordnung ist nicht eindeutig"));
        }
        let fact = matches[0];
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
                source_kind,
                origin: origin.clone(),
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

pub fn validity_contains(validity: &PatchValidity, patch: &str) -> bool {
    match validity {
        PatchValidity::Unknown { .. } => false,
        PatchValidity::Known {
            from_patch,
            to_patch_exclusive,
            ..
        } => {
            from_patch.as_str() <= patch
                && to_patch_exclusive.as_deref().is_none_or(|end| patch < end)
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
    for fact in &facts {
        let mut comparison_qualifiers = fact.qualifiers.clone();
        for key in ["source_lexeme", "numeric_representation"] {
            comparison_qualifiers.remove(key);
        }
        grouped
            .entry((
                fact.predicate.clone(),
                serde_json::to_string(&comparison_qualifiers).unwrap_or_default(),
                fact.unit.clone(),
            ))
            .or_insert_with(Vec::new)
            .push(fact);
    }
    let mut conflicts = Vec::new();
    for ((predicate, _, _), group) in grouped {
        if group.iter().any(|f| f.value != group[0].value) {
            let numeric = group.iter().all(|f| {
                f.value.is_number()
                    || f.qualifiers
                        .get("numeric_representation")
                        .and_then(Value::as_str)
                        == Some("source_numeric_lexeme")
            });
            let preferred = group.iter().find(|f| {
                f.provenance.source_kind
                    == if numeric {
                        ProfileSourceKind::GameFile
                    } else {
                        ProfileSourceKind::Wiki
                    }
            });
            conflicts.push(ProfileConflict {
                predicate,
                preferred_fact_id: preferred.map(|f| fact_reference(f)),
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
    let unknowns = if facts
        .iter()
        .any(|f| matches!(f.validity, PatchValidity::Unknown { .. }))
    {
        vec!["Patchgültigkeit ist für einen Teil der Fakten unbekannt".into()]
    } else {
        Vec::new()
    };
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

impl PgStore {
    pub async fn store_entity_fact_bindings(
        &self,
        entity: &EntityIdentity,
        record: &SourceRecordV2,
        fact_ids: &[String],
    ) -> crate::Result<usize> {
        if entity.entity_key.trim().is_empty()
            || entity.name.trim().is_empty()
            || entity.identity_evidence.is_empty()
            || fact_ids.len() > 10_000
        {
            return Err(invalid("Belegte Entitätszuordnung fehlt oder ist zu groß"));
        }
        let identity = serde_json::to_value(entity)?;
        let mut tx = self.pool.begin().await?;
        let revision =
            i64::try_from(record.revision).map_err(|_| invalid("Revision ist zu groß"))?;
        let stored: Value = sqlx::query_scalar("SELECT record_json FROM brain.source_record_revisions WHERE source_id=$1 AND logical_id=$2 AND revision=$3 FOR SHARE")
            .bind(&record.source_id).bind(&record.logical_id).bind(revision).fetch_one(&mut *tx).await?;
        let original: SourceRecordV2 = serde_json::from_value(stored)?;
        if serde_json::to_value(&original)? != serde_json::to_value(record)? {
            return Err(invalid(
                "Faktenbindung widerspricht der gespeicherten Quellrevision",
            ));
        }
        let facts = project_entity_facts(&original, fact_ids)?;
        sqlx::query("INSERT INTO brain.entity_profile_entities_v1(entity_key,identity_json) VALUES($1,$2) ON CONFLICT(entity_key) DO NOTHING").bind(&entity.entity_key).bind(&identity).execute(&mut *tx).await?;
        let stored: Value = sqlx::query_scalar("SELECT identity_json FROM brain.entity_profile_entities_v1 WHERE entity_key=$1 FOR UPDATE").bind(&entity.entity_key).fetch_one(&mut *tx).await?;
        if stored != identity {
            return Err(invalid("Entitätsidentität widerspricht vorhandenem Beleg"));
        }
        let mut inserted = 0;
        for fact in facts {
            let encoded = serde_json::to_string(&fact)?;
            inserted += sqlx::query("INSERT INTO brain.entity_profile_facts_v1(entity_key,source_id,logical_id,revision,fact_id,fact_json) VALUES($1,$2,$3,$4,$5,$6) ON CONFLICT DO NOTHING")
                .bind(&entity.entity_key).bind(&record.source_id).bind(&record.logical_id).bind(revision).bind(&fact.fact_id).bind(&encoded).execute(&mut *tx).await?.rows_affected() as usize;
            let stored: String = sqlx::query_scalar("SELECT fact_json FROM brain.entity_profile_facts_v1 WHERE entity_key=$1 AND source_id=$2 AND logical_id=$3 AND revision=$4 AND fact_id=$5")
                .bind(&entity.entity_key).bind(&record.source_id).bind(&record.logical_id).bind(revision).bind(&fact.fact_id).fetch_one(&mut *tx).await?;
            if stored != encoded {
                return Err(invalid("Faktenbeleg widerspricht vorhandenem Import"));
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
            let values: Vec<String> = sqlx::query_scalar("SELECT fact_json FROM brain.entity_profile_facts_v1 WHERE entity_key=$1 AND source_id=$2 AND logical_id=$3 AND revision=$4 ORDER BY fact_id")
                .bind(entity_key).bind(&source.source_id).bind(&source.logical_id).bind(source.revision as i64).fetch_all(&self.pool).await?;
            for value in values {
                let fact: EntityProfileFact = serde_json::from_str(&value)?;
                if project_entity_facts(original, std::slice::from_ref(&fact.fact_id))?[0] != fact {
                    return Err(invalid("Fakt und freigegebene Quelle widersprechen sich"));
                }
                let kind = [
                    ("hero", EntityKind::Hero),
                    ("ability", EntityKind::Ability),
                    ("item", EntityKind::Item),
                ]
                .into_iter()
                .find_map(|(prefix, kind)| {
                    (fact.subject == format!("{prefix}:{entity_key}")).then_some(kind)
                });
                if let Some(kind) = kind {
                    let document: Value = serde_json::from_str(
                        &original.metadata[crate::source_versions::DOCUMENT_METADATA_KEY],
                    )?;
                    let name = document["title"]
                        .as_str()
                        .filter(|name| !name.trim().is_empty())
                        .ok_or_else(|| invalid("Belegter Entitätsname fehlt"))?;
                    let evidence = format!("{}:title", fact_reference(&fact));
                    let identity = entity.get_or_insert_with(|| EntityIdentity {
                        entity_key: entity_key.into(),
                        kind,
                        name: name.into(),
                        aliases: Vec::new(),
                        identity_evidence: Vec::new(),
                    });
                    if identity.kind != kind {
                        return Err(invalid("Autorisierte Entitätsbelege widersprechen sich"));
                    }
                    if identity.name != name && !identity.aliases.iter().any(|alias| alias == name)
                    {
                        identity.aliases.push(name.into());
                    }
                    if !identity.identity_evidence.contains(&evidence) {
                        identity.identity_evidence.push(evidence);
                    }
                }
                if let Some(patch) = patch {
                    historical_unknown |= matches!(fact.validity, PatchValidity::Unknown { .. });
                    if !validity_contains(&fact.validity, patch) {
                        continue;
                    }
                }
                facts.push(fact);
            }
        }
        let Some(entity) = entity else {
            return Ok(None);
        };
        let mut story = self.entity_patch_story(&entity).await?;
        if let Some(patch) = patch {
            story.retain(|change| change.patch_date.as_str() <= patch);
        }
        let mut profile = assemble_profile(entity, patch.map(str::to_owned), facts, story);
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
        let mut tx = self.pool.begin().await?;
        sqlx::raw_sql("SET TRANSACTION READ ONLY; SET LOCAL statement_timeout='5000ms'")
            .execute(&mut *tx)
            .await?;
        let story: Vec<Value> = sqlx::query_scalar("SELECT to_jsonb(c) FROM brain.patch_changes c WHERE lower(c.entity_name)=ANY(SELECT lower(n) FROM unnest($1::text[]) n) OR lower(c.ability_name)=ANY(SELECT lower(n) FROM unnest($1::text[]) n) ORDER BY c.patch_date,c.stat_name").bind(names).fetch_all(&mut *tx).await?;
        tx.commit().await?;
        story.into_iter().map(decode_patch_change).collect()
    }
}

fn decode_patch_change(value: Value) -> crate::Result<PatchStoryChange> {
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
