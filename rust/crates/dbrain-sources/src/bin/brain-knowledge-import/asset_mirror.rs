use brain_contracts::{
    external::{ExternalSourceIr, SourceRevision, Validation},
    source::Versioned,
    value::Observed,
    SourceRecordV2, SourceVisibility,
};
use brain_storage::{ApplyOutcome, PgStore};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::Read;
use std::path::Path;

const SOURCE: &str = "deadlock_assets_api";
const MAX_RAW_BYTES: usize = 64 * 1024 * 1024;
const SLOTS: [&str; 5] = [
    "manifest",
    "heroes_all/english",
    "heroes_all/german",
    "items/english",
    "items/german",
];

pub(super) struct MirrorDocument {
    pub(super) slot: String,
    pub(super) id: i64,
    pub(super) url: Option<String>,
    pub(super) content_hash: String,
    pub(super) client_version: i64,
    pub(super) contract: Value,
    pub(super) provenance: Value,
    pub(super) validation: Value,
}

type DocumentRow = (
    String,
    i64,
    Option<String>,
    String,
    String,
    String,
    String,
    String,
    String,
);

async fn latest_documents(pool: &sqlx::PgPool) -> Result<Vec<(MirrorDocument, String)>, String> {
    let rows: Vec<DocumentRow> = sqlx::query_as(
        "WITH docs AS ( \
            SELECT id, url, content_hash, raw_path, metadata, \
                metadata #>> '{adapter,client_version}' AS version, \
                CASE WHEN metadata #>> '{adapter,role}'='client_manifest' THEN 'manifest' \
                    ELSE (metadata #>> '{adapter,kind}') || '/' || (metadata #>> '{adapter,language}') END AS slot \
            FROM brain.source_documents \
            WHERE source=$1 AND metadata #>> '{validation,state}'='validated' \
                AND metadata #>> '{adapter,client_version}' ~ '^[1-9][0-9]{0,17}$' \
        ), wanted AS (SELECT unnest($2::text[]) AS slot), \
        complete AS ( \
            SELECT version FROM docs JOIN wanted USING (slot) GROUP BY version \
            HAVING count(DISTINCT slot)=(SELECT count(*) FROM wanted) \
            ORDER BY version::bigint DESC LIMIT 1 \
        ) \
        SELECT DISTINCT ON (slot) slot, id, url, content_hash, raw_path, version, \
            jsonb_set(metadata->'contract', '{data,field_provenance}', '{}'::jsonb)::text, \
            (metadata->'provenance')::text, (metadata->'validation')::text \
        FROM docs JOIN wanted USING (slot) JOIN complete USING (version) \
        ORDER BY slot, id DESC",
    )
    .bind(SOURCE)
    .bind(SLOTS.map(str::to_owned).to_vec())
    .fetch_all(pool)
    .await
    .map_err(|_| "Spiegeloriginale können nicht gelesen werden")?;
    if rows.len() != SLOTS.len() {
        return Err("Kein vollständiger validierter Assets-Spiegel mit Manifest vorhanden".into());
    }
    rows.into_iter()
        .map(
            |(slot, id, url, content_hash, raw_path, version, contract, provenance, validation)| {
                let parse = |text: &str| {
                    serde_json::from_str::<Value>(text)
                        .map_err(|_| "Spiegelmetadaten sind ungültig".to_owned())
                };
                Ok((
                    MirrorDocument {
                        slot,
                        id,
                        url,
                        content_hash,
                        client_version: version
                            .parse()
                            .map_err(|_| "Ungültige Clientversion im Spiegel")?,
                        contract: parse(&contract)?,
                        provenance: parse(&provenance)?,
                        validation: parse(&validation)?,
                    },
                    raw_path,
                ))
            },
        )
        .collect()
}

fn read_raw(path: &str) -> Result<Vec<u8>, String> {
    let path = Path::new(path);
    if !path.is_absolute() {
        return Err("Originalpfad des Spiegels ist nicht absolut".into());
    }
    let file = File::open(path).map_err(|_| "Spiegeloriginal kann nicht geöffnet werden")?;
    if !file
        .metadata()
        .map_err(|_| "Spiegeloriginal kann nicht geprüft werden")?
        .is_file()
    {
        return Err("Spiegeloriginal ist keine reguläre Datei".into());
    }
    let mut bytes = Vec::new();
    file.take(MAX_RAW_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "Spiegeloriginal kann nicht gelesen werden")?;
    if bytes.len() > MAX_RAW_BYTES {
        return Err("Spiegeloriginal überschreitet 64 MiB".into());
    }
    Ok(bytes)
}

fn authorization_ref(document: &MirrorDocument) -> String {
    format!(
        "{SOURCE}:source_document:{}:sha256:{}",
        document.id, document.content_hash
    )
}

pub(super) fn mirror_record(
    document: &MirrorDocument,
    raw: Vec<u8>,
) -> Result<SourceRecordV2, String> {
    let content_hash = format!("{:x}", Sha256::digest(&raw));
    if content_hash != document.content_hash {
        return Err("Spiegeloriginal widerspricht seinem gespeicherten Hash".into());
    }
    let content = String::from_utf8(raw).map_err(|_| "Spiegeloriginal ist kein UTF-8")?;
    let ir = serde_json::from_value::<Versioned<ExternalSourceIr>>(document.contract.clone())
        .map_err(|_| "Spiegelvertrag entspricht nicht dem Schema")?
        .data;
    let provenance = &ir.provenance;
    let url = document
        .url
        .as_deref()
        .ok_or("Originaladresse des Spiegels fehlt")?;
    let original: Value =
        serde_json::from_str(&content).map_err(|_| "Spiegeloriginal ist kein JSON")?;
    let payload_matches = match (&ir.payload, document.slot.as_str()) {
        (_, "manifest") => original["client_version"].as_i64() == Some(document.client_version),
        (Observed::Known { value }, _) => *value == original,
        _ => false,
    };
    if provenance.source != SOURCE
        || provenance.locator != url
        || provenance.raw_sha256 != document.content_hash
        || !matches!(&provenance.source_revision, SourceRevision::Http { body_sha256, .. } if *body_sha256 == document.content_hash)
        || !matches!(ir.validation, Validation::Validated { .. })
        || serde_json::to_value(provenance).ok().as_ref() != Some(&document.provenance)
        || serde_json::to_value(&ir.validation).ok().as_ref() != Some(&document.validation)
        || !payload_matches
    {
        return Err("Spiegeloriginal und Vertrag stimmen nicht überein".into());
    }
    if ir.visibility != SourceVisibility::Public
        || !provenance.publication_authorized
        || !provenance.provider_egress_authorized
    {
        return Err("Spiegelvertrag gewährt keine öffentliche Freigabe".into());
    }
    let mut origin = ir.origin_artifact();
    origin.policy.authorization_ref = Observed::known(authorization_ref(document));
    let mut record = SourceRecordV2 {
        source_id: origin.identity.source_id.clone(),
        logical_id: origin.identity.logical_id.clone(),
        revision: 1,
        content_hash,
        content,
        visibility: ir.visibility,
        allowed_scopes: ir.allowed_scopes,
        tombstone: false,
        valid_from: None,
        valid_to: None,
        metadata: BTreeMap::new(),
    };
    origin
        .bind_record(&mut record)
        .map_err(|_| "Spiegelherkunft kann nicht gebunden werden")?;
    Ok(record)
}

fn next_revision(record: &SourceRecordV2, head: Option<&SourceRecordV2>) -> Result<u64, String> {
    let Some(head) = head else {
        return Ok(1);
    };
    let mut same = record.clone();
    same.revision = head.revision;
    if same == *head {
        return Ok(head.revision);
    }
    head.revision
        .checked_add(1)
        .ok_or_else(|| "Speicherrevision ist zu groß".into())
}

async fn current_head(
    pool: &sqlx::PgPool,
    record: &SourceRecordV2,
) -> Result<Option<SourceRecordV2>, String> {
    let row: Option<String> = sqlx::query_scalar(
        "SELECT record_json::text FROM brain.source_record_heads WHERE source_id=$1 AND logical_id=$2",
    )
    .bind(&record.source_id)
    .bind(&record.logical_id)
    .fetch_optional(pool)
    .await
    .map_err(|_| "Quellkopf kann nicht gelesen werden")?;
    row.map(|json| {
        serde_json::from_str(&json).map_err(|_| "Gespeicherter Quellkopf ist ungültig".to_owned())
    })
    .transpose()
}

async fn live_heads(pool: &sqlx::PgPool) -> Result<Vec<SourceRecordV2>, String> {
    let rows: Vec<String> = sqlx::query_scalar(
        "SELECT record_json::text FROM brain.source_record_heads \
         WHERE source_id=$1 AND NOT tombstone ORDER BY logical_id",
    )
    .bind(SOURCE)
    .fetch_all(pool)
    .await
    .map_err(|_| "Quellköpfe des Spiegels können nicht gelesen werden")?;
    rows.iter()
        .map(|json| {
            serde_json::from_str(json)
                .map_err(|_| "Gespeicherter Quellkopf ist ungültig".to_owned())
        })
        .collect()
}

fn withdrawn(
    head: &SourceRecordV2,
    current: &BTreeSet<String>,
) -> Result<Option<SourceRecordV2>, String> {
    if head.tombstone || current.contains(&head.logical_id) {
        return Ok(None);
    }
    let mut record = head.clone();
    record.revision = head
        .revision
        .checked_add(1)
        .ok_or("Speicherrevision ist zu groß")?;
    record.tombstone = true;
    Ok(Some(record))
}

async fn retire_stale(
    pool: &sqlx::PgPool,
    store: &PgStore,
    current: &BTreeSet<String>,
) -> Result<Vec<Value>, String> {
    let mut retired = Vec::new();
    for head in live_heads(pool).await? {
        let Some(record) = withdrawn(&head, current)? else {
            continue;
        };
        match store.apply(&record).await {
            Ok(ApplyOutcome::Tombstoned) => {}
            _ => return Err("Veralteter Spiegelkopf konnte nicht zurückgezogen werden".into()),
        }
        retired.push(json!({
            "logical_id": record.logical_id,
            "previous_revision": head.revision,
            "revision": record.revision,
            "outcome": "tombstoned",
        }));
    }
    Ok(retired)
}

pub(super) async fn register(pool: &sqlx::PgPool, store: &PgStore) -> Result<Value, String> {
    let mut prepared = Vec::new();
    for (document, raw_path) in latest_documents(pool).await? {
        let record = mirror_record(&document, read_raw(&raw_path)?)?;
        prepared.push((document, record));
    }
    let mut records = Vec::new();
    for (document, mut record) in prepared {
        let head = current_head(pool, &record).await?;
        record.revision = next_revision(&record, head.as_ref())?;
        let outcome = store
            .apply(&record)
            .await
            .map_err(|_| "Spiegeloriginal konnte nicht als Quellkopf übernommen werden")?;
        let outcome = match outcome {
            ApplyOutcome::Inserted => "inserted",
            ApplyOutcome::Updated => "updated",
            ApplyOutcome::Unchanged => "unchanged",
            ApplyOutcome::Tombstoned | ApplyOutcome::IgnoredStale => {
                return Err("Unerwartetes Ergebnis beim Übernehmen des Spiegeloriginals".into())
            }
        };
        records.push(json!({
            "slot": document.slot,
            "source_document_id": document.id,
            "client_version": document.client_version,
            "logical_id": record.logical_id,
            "raw_sha256": record.content_hash,
            "content_bytes": record.content.len(),
            "previous_revision": head.map(|head| head.revision),
            "revision": record.revision,
            "outcome": outcome,
        }));
    }
    let current = records
        .iter()
        .filter_map(|record| record["logical_id"].as_str().map(str::to_owned))
        .collect::<BTreeSet<_>>();
    let tombstoned = retire_stale(pool, store, &current).await?;
    Ok(json!({
        "complete": true,
        "source": SOURCE,
        "records": records,
        "tombstoned": tombstoned,
        "published": false,
        "activated": false,
        "next_step": "publish --source deadlock_assets_api",
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use brain_contracts::{
        external::Provenance,
        source::{observed_option, origin_from_record, SourceTimestamp},
        value::UnknownReason,
    };
    use std::collections::BTreeSet;

    fn fixture(raw: &[u8], egress: bool) -> MirrorDocument {
        let hash = format!("{:x}", Sha256::digest(raw));
        let url = "https://assets.deadlock-api.com/v2/items?client_version=6100&language=german";
        let provenance = Provenance {
            source: SOURCE.into(),
            locator: url.into(),
            source_revision: SourceRevision::Http {
                body_sha256: hash.clone(),
                etag: None,
                last_modified: None,
            },
            parser_revision: "assets/7".into(),
            parser_family: "deadlock-assets".into(),
            raw_sha256: hash.clone(),
            schema_sha256: None,
            observed_at: 1_760_000_000,
            origin_artifacts: BTreeSet::from(["deadlock-assets:items".into()]),
            derivation_family: Some("deadlock-assets".into()),
            publication_authorized: true,
            provider_egress_authorized: egress,
        };
        let ir = ExternalSourceIr {
            payload: Observed::known(serde_json::from_slice(raw).unwrap_or(Value::Null)),
            provenance: provenance.clone(),
            validation: Validation::Validated {
                extra_fields: Vec::new(),
            },
            transport: json!({}),
            schema_version: Observed::unknown(UnknownReason::NotPresent),
            field_provenance: BTreeMap::new(),
            visibility: SourceVisibility::Public,
            allowed_scopes: BTreeSet::from(["bot.public".into()]),
            license: Observed::known("public-assets".into()),
        };
        MirrorDocument {
            slot: "items/german".into(),
            id: 42,
            url: Some(url.into()),
            content_hash: hash,
            client_version: 6100,
            contract: serde_json::to_value(Versioned::new(ir)).unwrap_or(Value::Null),
            provenance: serde_json::to_value(provenance).unwrap_or(Value::Null),
            validation: json!({"state": "validated", "extra_fields": []}),
        }
    }

    #[test]
    fn mirror_record_origin_matches_the_receipt_fields_and_requires_grants() {
        let raw = br#"[{"id":1,"name":"Mystischer Ausbruch"}]"#;
        let document = fixture(raw, true);
        let record = mirror_record(&document, raw.to_vec()).unwrap();
        let receipt: Versioned<ExternalSourceIr> =
            serde_json::from_value(document.contract.clone()).unwrap();
        let receipt = receipt.data;
        let origin = origin_from_record(&record).unwrap();
        assert_eq!(origin.identity.source_id, receipt.provenance.source);
        assert_eq!(origin.locator, document.url.clone().unwrap());
        assert_eq!(origin.raw_sha256, document.content_hash);
        assert_eq!(origin.source_revision, receipt.provenance.source_revision);
        assert_eq!(origin.parser_revision, receipt.provenance.parser_revision);
        assert_eq!(origin.parser_family, receipt.provenance.parser_family);
        assert_eq!(
            origin.retrieved_at,
            Observed::known(SourceTimestamp::UnixSeconds(receipt.provenance.observed_at))
        );
        assert_eq!(origin.origin_artifacts, receipt.provenance.origin_artifacts);
        assert_eq!(
            origin.derivation_family,
            observed_option(receipt.provenance.derivation_family.clone())
        );
        assert_eq!(origin.schema_version, receipt.schema_version);
        assert_eq!(origin.policy.visibility, receipt.visibility);
        assert_eq!(origin.policy.allowed_scopes, receipt.allowed_scopes);
        assert_eq!(origin.policy.license, receipt.license);
        assert_eq!(
            origin.policy.authorization_ref,
            Observed::known(format!(
                "deadlock_assets_api:source_document:42:sha256:{}",
                document.content_hash
            ))
        );
        assert!(origin.policy.publication_allowed && origin.policy.provider_egress_allowed);
        assert_eq!(
            format!("{:x}", Sha256::digest(record.content.as_bytes())),
            document.content_hash
        );
        assert_eq!(record.content.as_bytes(), raw);
        assert_eq!(next_revision(&record, None).unwrap(), 1);
        assert_eq!(next_revision(&record, Some(&record)).unwrap(), 1);
        assert!(mirror_record(&fixture(raw, false), raw.to_vec()).is_err());
        assert!(mirror_record(&document, br#"[{"id":2}]"#.to_vec()).is_err());
    }

    #[test]
    fn heads_of_older_client_versions_are_withdrawn() {
        let raw = br#"[{"id":1}]"#;
        let mut stale = mirror_record(&fixture(raw, true), raw.to_vec()).unwrap();
        stale.revision = 3;
        let current = BTreeSet::from([format!("{}-new", stale.logical_id)]);
        let withdrawn_head = withdrawn(&stale, &current).unwrap().unwrap();
        assert!(withdrawn_head.tombstone);
        assert_eq!(withdrawn_head.revision, 4);
        assert_eq!(withdrawn_head.logical_id, stale.logical_id);
        assert!(withdrawn_head.validate().is_ok());
        let kept = BTreeSet::from([stale.logical_id.clone()]);
        assert!(withdrawn(&stale, &kept).unwrap().is_none());
        assert!(withdrawn(&withdrawn_head, &current).unwrap().is_none());
    }
}
