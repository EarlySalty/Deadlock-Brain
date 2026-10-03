//! Überführt archivierte öffentliche Beiträge in den bestehenden Kernelkorpus.
use crate::{forum::SOURCE, Result, SourcesError};
use brain_contracts::{
    source::{
        observed_option, GameValidity, OriginArtifact, SourceIdentity, SourcePolicy,
        SourceRevision, SourceTimestamp,
    },
    store::{DocumentStorePort, SourceBatch, SourceCheckpoint},
    value::{Observed, UnknownReason},
    CorpusRelease, SourceRecordV2, SourceVisibility,
};
use brain_storage::PgStore;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::{PgPool, Row};
use std::collections::{BTreeMap, BTreeSet};

const CONFIGURATION: &str = "forum-corpus/1";
fn invalid(message: impl Into<String>) -> SourcesError {
    SourcesError::invalid_input(message)
}

pub(crate) fn record(payload: &Value, revision: u64, observed: i64) -> Result<SourceRecordV2> {
    let post = payload["post_id"]
        .as_u64()
        .ok_or_else(|| invalid("Beitrags-ID fehlt."))?;
    let url = payload["thread_url"]
        .as_str()
        .ok_or_else(|| invalid("Threadquelle fehlt."))?;
    if !url.starts_with("https://forums.playdeadlock.com/threads/") {
        return Err(invalid("Ungültige Forumquelle."));
    }
    let locator = format!("{url}#post-{post}");
    let text = payload["text"]
        .as_str()
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| invalid("Leerer Forumbeitrag."))?;
    // Quellenstatus und Zeit stehen auch im Text, damit einzelne Suchausschnitte
    // einen Spielerbericht nicht als bestätigte aktuelle Spielregel darstellen.
    let content = format!("Forumbericht, unbestätigt. Aktuelle Gültigkeit und Behebung unbekannt.\nThema: {}\nKategorie: {}\nBeitragsdatum: {}\nAutor: {}\nQuelle: {}\n\n{}", payload["thread_title"].as_str().unwrap_or("Unbekannt"), payload["category"].as_str().unwrap_or("Unbekannt"), payload["datetime"].as_str().unwrap_or("Unbekannt"), payload["author"].as_str().unwrap_or("Unbekannt"), locator, text);
    let hash = format!("{:x}", Sha256::digest(content.as_bytes()));
    let mut source = SourceRecordV2 {
        source_id: SOURCE.into(),
        logical_id: format!("post:{post}"),
        revision,
        content_hash: hash.clone(),
        content,
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::new(),
        tombstone: false,
        valid_from: None,
        valid_to: None,
        metadata: BTreeMap::from([
            ("kind".into(), "prose".into()),
            (
                "thread_id".into(),
                payload["thread_id"]
                    .as_u64()
                    .map(|id| id.to_string())
                    .unwrap_or_default(),
            ),
            (
                "title".into(),
                payload["thread_title"]
                    .as_str()
                    .unwrap_or("Forumbeitrag")
                    .into(),
            ),
            ("locator".into(), locator.clone()),
            ("evidence_status".into(), "reported_unverified".into()),
            ("currentness".into(), "unknown".into()),
            (
                "source_date".into(),
                payload["datetime"].as_str().unwrap_or("unknown").into(),
            ),
        ]),
    };
    let unknown = || Observed::unknown(UnknownReason::NotPresent);
    OriginArtifact {
        identity: SourceIdentity {
            source_id: SOURCE.into(),
            logical_id: source.logical_id.clone(),
        },
        source_revision: SourceRevision::Api {
            api_version: CONFIGURATION.into(),
            original_revision: Some(hash.clone()),
        },
        raw_sha256: hash,
        locator,
        parser_revision: CONFIGURATION.into(),
        parser_family: "xenforo-public-post".into(),
        schema_version: unknown(),
        schema_sha256: unknown(),
        retrieved_at: Observed::known(SourceTimestamp::UnixSeconds(observed)),
        source_time: observed_option(
            payload["timestamp"]
                .as_i64()
                .map(SourceTimestamp::UnixSeconds),
        ),
        language: unknown(),
        origin_artifacts: BTreeSet::from([format!("forum-post:{post}")]),
        derivation_family: Observed::known("forum-report".into()),
        policy: SourcePolicy {
            visibility: SourceVisibility::Public,
            allowed_scopes: BTreeSet::new(),
            authorization_ref: unknown(),
            license: unknown(),
            publication_allowed: true,
            provider_egress_allowed: true,
            raw_retention_allowed: true,
        },
        validity: GameValidity::unknown(),
    }
    .bind_record(&mut source)
    .map_err(invalid)?;
    Ok(source)
}

/// Behält die Pins anderer Quellen und unbekannter Forumthreads des Basisstands. Teilarchive werden als
/// Teilarchive veröffentlicht; eine erfolgreiche Veröffentlichung behauptet keine
/// vollständige Forumabdeckung. Ein bestehender Release wird niemals verändert.
pub async fn publish_archive(
    archive_pool: &PgPool,
    pool: &PgPool,
    base_id: &str,
    release_id: &str,
) -> Result<Value> {
    if pool.options().get_max_connections() < 2 {
        return Err(invalid(
            "Kernel-Veröffentlichung benötigt mindestens zwei Datenbankverbindungen.",
        ));
    }
    if base_id == release_id || release_id.trim().is_empty() {
        return Err(invalid("Neuer Wissensstand benötigt eine eigene ID."));
    }
    let store = PgStore::new(pool.clone());
    let value: Value =
        sqlx::query_scalar("SELECT release_json FROM brain.corpus_releases_v1 WHERE release_id=$1")
            .bind(base_id)
            .fetch_one(pool)
            .await?;
    let mut release: CorpusRelease = serde_json::from_value(value)?;
    release.release_id = release_id.into();
    release.created_at_epoch = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs() as i64;
    let mut guard = pool.begin().await?;
    sqlx::query("SELECT pg_advisory_xact_lock(hashtext($1)::bigint)")
        .bind("forum-corpus-publish")
        .execute(&mut *guard)
        .await?;
    let mut archive_snapshot = archive_pool.begin().await?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ, READ ONLY")
        .execute(&mut *archive_snapshot)
        .await?;
    let rows = sqlx::query("WITH latest_threads AS (SELECT DISTINCT ON (t.external_id) t.payload FROM brain.entity_snapshots t JOIN brain.source_documents d ON d.id=t.source_document_id WHERE t.source=$1 AND t.entity_type='forum_thread' AND d.metadata->>'complete'='true' ORDER BY t.external_id,t.fetched_at DESC,t.id DESC) SELECT DISTINCT ON (s.external_id) s.id,s.payload,extract(epoch FROM s.fetched_at)::bigint AS observed FROM brain.entity_snapshots s JOIN brain.source_documents d ON d.id=s.source_document_id JOIN latest_threads t ON t.payload->>'thread_id'=s.payload->>'thread_id' AND t.payload->'post_ids' @> jsonb_build_array((s.payload->>'post_id')::bigint) WHERE s.source=$1 AND s.entity_type='forum_post' AND d.metadata->>'complete'='true' ORDER BY s.external_id,s.fetched_at DESC,s.id DESC").bind(SOURCE).fetch_all(&mut *archive_snapshot).await?;
    if rows.is_empty() {
        return Err(invalid(
            "Kein vollständig archivierter Thread für den Kernel vorhanden.",
        ));
    }
    // Nur vollständig beobachtete Threads liefern eine Löschungsentscheidung.
    // Basisbelege unbekannter Threads bleiben bei Teilarchiven gepinnt.
    let observed_threads: Vec<Value> = sqlx::query_scalar("SELECT DISTINCT ON (t.external_id) t.payload FROM brain.entity_snapshots t JOIN brain.source_documents d ON d.id=t.source_document_id WHERE t.source=$1 AND t.entity_type='forum_thread' AND d.metadata->>'complete'='true' ORDER BY t.external_id,t.fetched_at DESC,t.id DESC").bind(SOURCE).fetch_all(&mut *archive_snapshot).await?;
    archive_snapshot.commit().await?;
    let membership: BTreeMap<String, BTreeSet<String>> = observed_threads
        .into_iter()
        .filter_map(|payload| {
            Some((
                payload["thread_id"].as_u64()?.to_string(),
                payload["post_ids"]
                    .as_array()?
                    .iter()
                    .filter_map(Value::as_u64)
                    .map(|id| format!("post:{id}"))
                    .collect(),
            ))
        })
        .collect();
    if let Some(pins) = release.source_revisions.get_mut(SOURCE) {
        let mut remove = Vec::new();
        for (logical, revision) in pins.iter() {
            let value: Value = sqlx::query_scalar("SELECT record_json FROM brain.source_record_revisions WHERE source_id=$1 AND logical_id=$2 AND revision=$3").bind(SOURCE).bind(logical).bind(*revision as i64).fetch_one(pool).await?;
            let previous: SourceRecordV2 = serde_json::from_value(value)?;
            let thread_id = previous
                .metadata
                .get("thread_id")
                .cloned()
                .filter(|id| !id.is_empty())
                .or_else(|| {
                    previous
                        .metadata
                        .get("locator")
                        .and_then(|url| url.split("/threads/").nth(1))
                        .and_then(|slug| slug.split('/').next())
                        .and_then(|slug| slug.rsplit('.').next())
                        .filter(|id| id.parse::<u64>().is_ok())
                        .map(str::to_string)
                });
            if thread_id
                .as_ref()
                .and_then(|id| membership.get(id))
                .is_some_and(|members| !members.contains(logical))
            {
                remove.push(logical.clone());
            }
        }
        for logical in remove {
            pins.remove(&logical);
        }
    }
    let mut committed = 0usize;
    let mut unchanged = 0usize;
    for chunk in rows.chunks(200) {
        let checkpoint = store
            .checkpoint(SOURCE)
            .await
            .map_err(|e| invalid(format!("Forumcheckpoint: {e:?}")))?;
        if checkpoint
            .as_ref()
            .is_some_and(|c| c.configuration != CONFIGURATION)
        {
            return Err(invalid("Fremder Forumcheckpoint."));
        }
        let generation = checkpoint.map_or(0, |c| c.generation);
        let mut records = Vec::new();
        for row in chunk {
            let payload: Value = row.try_get("payload")?;
            let mut incoming = record(
                &payload,
                row.try_get::<i64, _>("id")? as u64,
                row.try_get("observed")?,
            )?;
            let previous: Option<Value> = sqlx::query_scalar("SELECT record_json FROM brain.source_record_heads WHERE source_id=$1 AND logical_id=$2").bind(SOURCE).bind(&incoming.logical_id).fetch_optional(pool).await?;
            if let Some(previous) = previous {
                let previous: SourceRecordV2 = serde_json::from_value(previous)?;
                if previous.tombstone
                    || previous.visibility != incoming.visibility
                    || previous.allowed_scopes != incoming.allowed_scopes
                    || brain_contracts::source::origin_from_record(&previous)
                        .map_err(invalid)?
                        .policy
                        != brain_contracts::source::origin_from_record(&incoming)
                            .map_err(invalid)?
                            .policy
                {
                    return Err(invalid("Widerrufene oder geänderte Forumrechte benötigen eine ausdrückliche Abklärung."));
                }
                if previous.content_hash == incoming.content_hash {
                    incoming = previous;
                    unchanged += 1;
                } else {
                    incoming.revision = previous
                        .revision
                        .checked_add(1)
                        .ok_or_else(|| invalid("Forumrevision erschöpft."))?;
                    records.push(incoming.clone());
                }
            } else {
                records.push(incoming.clone());
            }
            release
                .source_revisions
                .entry(SOURCE.into())
                .or_default()
                .insert(incoming.logical_id, incoming.revision);
        }
        if !records.is_empty() {
            let count = records.len();
            let batch = SourceBatch {
                expected_generation: generation,
                checkpoint: SourceCheckpoint {
                    source_id: SOURCE.into(),
                    configuration: CONFIGURATION.into(),
                    generation: generation + 1,
                    state: json!({"last_archive_snapshot": chunk.last().map(|r| r.get::<i64,_>("id"))}),
                },
                records,
            };
            let lease = store
                .claim(SOURCE, "forum-corpus", 60_000)
                .await
                .map_err(|e| invalid(format!("Forumlease: {e:?}")))?;
            store
                .commit(&batch, &lease)
                .await
                .map_err(|e| invalid(format!("Forumimport: {e:?}")))?;
            committed += count;
        }
    }
    store
        .publish_release(&release)
        .await
        .map_err(|e| invalid(format!("Forumrelease: {e:?}")))?;
    guard.commit().await?;
    Ok(
        json!({"base_release": base_id, "knowledge_release": release_id, "posts": rows.len(), "committed": committed, "unchanged": unchanged, "other_source_pins_preserved": true, "unknown_forum_thread_pins_preserved": true, "evidence_status": "reported_unverified"}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn forum_report_does_not_establish_patch_validity() {
        let source = record(&json!({"post_id": 9, "thread_url": "https://forums.playdeadlock.com/threads/bug.1/", "text": "Item verursacht Fehler", "timestamp": 100}), 3, 200).unwrap();
        let origin = brain_contracts::source::origin_from_record(&source).unwrap();
        assert_eq!(origin.validity, GameValidity::unknown());
        assert_eq!(source.metadata["evidence_status"], "reported_unverified");
        assert!(source.valid_from.is_none());
    }
}
