use anyhow::{anyhow, ensure, Context, Result};
use brain_contracts::{
    external::{ExternalSourceIr, Provenance, SourceRevision, Validation},
    source::Versioned,
    value::Observed,
    SourceVisibility,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;
use std::collections::BTreeSet;

pub const REQUIRED_MIRROR_KINDS: &[&str] = &["items", "heroes", "heroes_all"];
pub const MIRRORED_ASSET_KINDS: &[&str] = &[
    "items",
    "heroes",
    "heroes_all",
    "generic_data",
    "npc_units",
    "misc_entities",
    "modifiers",
];

pub fn mirrored_asset_languages(kind: &str) -> Option<&'static [&'static str]> {
    match kind {
        "items" | "heroes" | "heroes_all" | "generic_data" | "npc_units" | "misc_entities" => {
            Some(&["english", "german"])
        }
        "modifiers" => Some(&[""]),
        _ => None,
    }
}

pub fn mirrored_asset_key(kind: &str, language: Option<&str>) -> Result<String> {
    let languages =
        mirrored_asset_languages(kind).ok_or_else(|| anyhow!("Unbekannte Assets-Art"))?;
    ensure!(
        kind != "modifiers" || language.is_none(),
        "Modifiers haben keine Assets-Sprache"
    );
    let language = language.unwrap_or("");
    ensure!(languages.contains(&language), "Unbekannte Assets-Sprache");
    Ok(if language.is_empty() {
        kind.to_owned()
    } else {
        format!("{kind}/{language}")
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetDocumentReceipt {
    pub source_document_id: i64,
    pub url: String,
    pub raw_sha256: String,
    pub fetched_at: String,
    pub provenance: Provenance,
    pub validation: Validation,
    pub visibility: SourceVisibility,
    pub allowed_scopes: BTreeSet<String>,
    pub license: Observed<String>,
    pub schema_version: Observed<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetMirrorReceipt {
    pub source_run_id: i64,
    pub client_version: i64,
    pub kind: String,
    pub language: Option<String>,
    pub parser_revision: String,
    pub run_started_at: String,
    pub run_finished_at: String,
    pub mirrored_at: i64,
    pub checked_at: i64,
    pub manifest: AssetDocumentReceipt,
    pub endpoint: AssetDocumentReceipt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirroredAssets {
    pub payload: Value,
    pub receipt: AssetMirrorReceipt,
}

const COMPLETE_MIRROR_FILTER: &str = "sr.source='assets' AND sr.status='ok' \
    AND sr.finished_at IS NOT NULL AND sr.finished_at >= sr.started_at \
    AND sr.summary->>'mirror_complete'='true' \
    AND COALESCE(sr.summary->>'reused_local_mirror', 'false') <> 'true' \
    AND NOT EXISTS ( \
        SELECT 1 FROM (VALUES ('items','english'), ('items','german'), ('heroes','english'), \
            ('heroes','german'), ('heroes_all','english'), ('heroes_all','german')) \
            AS required(kind, language) \
        LEFT JOIN brain.source_documents required_sd \
            ON required_sd.id=(sr.summary->'endpoints'->(required.kind || '/' || required.language)->>'source_document_id')::bigint \
            AND required_sd.source='deadlock_assets_api' \
        WHERE NOT COALESCE( \
            jsonb_typeof(required_sd.metadata #> '{contract,data,payload,value}')='array' \
            AND required_sd.metadata #> '{contract,data,payload,value}' <> '[]'::jsonb \
            AND required_sd.metadata #>> '{adapter,client_version}'=sr.summary->>'client_version' \
            AND required_sd.metadata #>> '{adapter,kind}'=required.kind \
            AND required_sd.metadata #>> '{adapter,language}'=required.language \
            AND required_sd.metadata #>> '{validation,state}'='validated', false))";

pub async fn latest_mirrored_client_version(pool: &PgPool) -> Result<i64> {
    let sql = format!(
        "SELECT sr.summary->>'client_version' FROM brain.source_runs sr \
        WHERE {COMPLETE_MIRROR_FILTER} \
        ORDER BY (sr.summary->>'client_version')::bigint DESC, sr.id DESC LIMIT 1"
    );
    let version = sqlx::query_scalar::<_, String>(&sql)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| anyhow!("Kein vollständiger lokaler Assets-Spiegel vorhanden"))?;
    let version: i64 = version
        .parse()
        .context("Ungültige gespiegelte Clientversion")?;
    ensure!(version > 0, "Ungültige gespiegelte Clientversion");
    Ok(version)
}

#[derive(sqlx::FromRow)]
struct MirrorRow {
    source_run_id: i64,
    run_summary: String,
    run_started_at: String,
    run_finished_at: String,
    endpoint_id: i64,
    endpoint_url: Option<String>,
    endpoint_hash: String,
    endpoint_fetched_at: String,
    endpoint_metadata: String,
    manifest_id: Option<i64>,
    manifest_url: Option<String>,
    manifest_hash: Option<String>,
    manifest_fetched_at: Option<String>,
    manifest_metadata: Option<String>,
}

async fn load_mirror_row(
    pool: &PgPool,
    client_version: i64,
    kind: &str,
    language: Option<&str>,
    source_run_id: Option<i64>,
) -> Result<MirrorRow> {
    ensure!(client_version > 0, "Ungültige Clientversion");
    ensure!(
        source_run_id.is_none_or(|id| id > 0),
        "Ungültiger Assets-Lauf"
    );
    let key = mirrored_asset_key(kind, language)?;
    let sql = format!("SELECT sr.id AS source_run_id, sr.summary::text AS run_summary, \
        sr.started_at::text AS run_started_at, sr.finished_at::text AS run_finished_at, \
        sd.id AS endpoint_id, sd.url AS endpoint_url, sd.content_hash AS endpoint_hash, \
        sd.fetched_at::text AS endpoint_fetched_at, sd.metadata::text AS endpoint_metadata, \
        md.id AS manifest_id, md.url AS manifest_url, md.content_hash AS manifest_hash, \
        md.fetched_at::text AS manifest_fetched_at, md.metadata::text AS manifest_metadata \
        FROM brain.source_runs sr \
        JOIN brain.source_documents sd ON sd.id=(sr.summary->'endpoints'->$2->>'source_document_id')::bigint \
            AND sd.source='deadlock_assets_api' \
        LEFT JOIN brain.source_documents md ON md.id=(sr.summary->>'manifest_document_id')::bigint \
            AND md.source='deadlock_assets_api' \
        WHERE {COMPLETE_MIRROR_FILTER} AND sr.summary->>'client_version'=$1 \
            AND ($3::bigint IS NULL OR sr.id=$3) ORDER BY sr.id DESC LIMIT 1");
    sqlx::query_as::<_, MirrorRow>(&sql)
        .bind(client_version.to_string())
        .bind(&key)
        .bind(source_run_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| anyhow!("Lokale Assets fehlen: {client_version}/{key}"))
}

pub async fn load_mirrored_assets(
    pool: &PgPool,
    client_version: i64,
    kind: &str,
    language: &str,
) -> Result<Value> {
    let row = load_mirror_row(
        pool,
        client_version,
        kind,
        (!language.is_empty()).then_some(language),
        None,
    )
    .await?;
    mirrored_payload(
        &serde_json::from_str(&row.endpoint_metadata)?,
        client_version,
        kind,
        language,
    )
}

pub async fn load_mirrored_assets_with_receipt(
    pool: &PgPool,
    client_version: i64,
    kind: &str,
    language: Option<&str>,
) -> Result<MirroredAssets> {
    let row = load_mirror_row(pool, client_version, kind, language, None).await?;
    bound_mirror(row, client_version, kind, language)
}

pub async fn load_mirrored_assets_for_run(
    pool: &PgPool,
    source_run_id: i64,
    client_version: i64,
    kind: &str,
    language: Option<&str>,
) -> Result<MirroredAssets> {
    let row = load_mirror_row(pool, client_version, kind, language, Some(source_run_id)).await?;
    bound_mirror(row, client_version, kind, language)
}

fn bound_mirror(
    row: MirrorRow,
    version: i64,
    kind: &str,
    language: Option<&str>,
) -> Result<MirroredAssets> {
    let summary: Value = serde_json::from_str(&row.run_summary)?;
    let metadata: Value = serde_json::from_str(&row.endpoint_metadata)?;
    let payload = mirrored_payload(&metadata, version, kind, language.unwrap_or(""))?;
    let parser_revision = summary["parser_revision"]
        .as_str()
        .filter(|revision| !revision.trim().is_empty())
        .ok_or_else(|| anyhow!("Assets-Lauf enthält keine Parserrevision"))?;
    let key = mirrored_asset_key(kind, language)?;
    let endpoint = document_receipt(
        row.endpoint_id,
        row.endpoint_url.as_deref(),
        &row.endpoint_hash,
        &row.endpoint_fetched_at,
        &metadata,
        parser_revision,
    )?;
    ensure!(
        summary["endpoints"][&key]["raw_sha256"].as_str() == Some(endpoint.raw_sha256.as_str()),
        "Assets-Lauf und Originalhash stimmen nicht überein"
    );
    let manifest_metadata: Value = serde_json::from_str(
        row.manifest_metadata
            .as_deref()
            .ok_or_else(|| anyhow!("Assets-Lauf enthält kein Originalmanifest"))?,
    )?;
    let manifest = document_receipt(
        row.manifest_id
            .ok_or_else(|| anyhow!("Assets-Manifest fehlt"))?,
        row.manifest_url.as_deref(),
        row.manifest_hash
            .as_deref()
            .ok_or_else(|| anyhow!("Assets-Manifesthash fehlt"))?,
        row.manifest_fetched_at
            .as_deref()
            .ok_or_else(|| anyhow!("Assets-Manifestzeit fehlt"))?,
        &manifest_metadata,
        parser_revision,
    )?;
    ensure!(
        manifest_metadata["adapter"]["role"].as_str() == Some("client_manifest")
            && manifest_metadata["adapter"]["client_version"].as_i64() == Some(version)
            && manifest_metadata
                .pointer("/contract/data/payload/value/client_version")
                .and_then(Value::as_i64)
                == Some(version),
        "Assets-Manifest und Clientversion stimmen nicht überein"
    );
    if let Some(hash) = summary.get("manifest_raw_sha256") {
        ensure!(
            hash.as_str() == Some(manifest.raw_sha256.as_str()),
            "Assets-Manifesthash stimmt nicht überein"
        );
    }
    let mirrored_at = summary["mirrored_at"]
        .as_i64()
        .filter(|at| *at >= 0)
        .ok_or_else(|| anyhow!("Assets-Lauf enthält keine Spiegelzeit"))?;
    let checked_at = summary["checked_at"]
        .as_i64()
        .filter(|at| *at >= mirrored_at)
        .ok_or_else(|| anyhow!("Assets-Lauf enthält keine gültige Prüfzeit"))?;
    Ok(MirroredAssets {
        payload,
        receipt: AssetMirrorReceipt {
            source_run_id: row.source_run_id,
            client_version: version,
            kind: kind.to_owned(),
            language: language.map(str::to_owned),
            parser_revision: parser_revision.to_owned(),
            run_started_at: row.run_started_at,
            run_finished_at: row.run_finished_at,
            mirrored_at,
            checked_at,
            manifest,
            endpoint,
        },
    })
}

fn document_receipt(
    id: i64,
    url: Option<&str>,
    hash: &str,
    fetched_at: &str,
    metadata: &Value,
    parser_revision: &str,
) -> Result<AssetDocumentReceipt> {
    ensure!(
        id > 0 && metadata["source_ir_version"].as_u64() == Some(1),
        "Ungültiges Assets-Originaldokument"
    );
    let contract: Versioned<ExternalSourceIr> =
        serde_json::from_value(metadata["contract"].clone())
            .context("Ungültiger Assets-Originalvertrag")?;
    let ir = contract.data;
    ir.origin_artifact()
        .validate()
        .map_err(|error| anyhow!("Ungültige Assets-Herkunft: {error}"))?;
    let url = url.ok_or_else(|| anyhow!("Assets-Originaladresse fehlt"))?;
    ensure!(
        ir.provenance.source == "deadlock_assets_api"
            && ir.provenance.locator == url
            && ir.provenance.raw_sha256 == hash
            && ir.provenance.parser_revision == parser_revision
            && matches!(&ir.provenance.source_revision, SourceRevision::Http { body_sha256, .. } if body_sha256 == hash)
            && matches!(ir.validation, Validation::Validated { .. })
            && metadata.get("provenance") == metadata.pointer("/contract/data/provenance")
            && metadata.get("validation") == metadata.pointer("/contract/data/validation"),
        "Assets-Originaldokument und Herkunft stimmen nicht überein"
    );
    Ok(AssetDocumentReceipt {
        source_document_id: id,
        url: url.to_owned(),
        raw_sha256: hash.to_owned(),
        fetched_at: fetched_at.to_owned(),
        provenance: ir.provenance,
        validation: ir.validation,
        visibility: ir.visibility,
        allowed_scopes: ir.allowed_scopes,
        license: ir.license,
        schema_version: ir.schema_version,
    })
}

fn mirrored_payload(metadata: &Value, version: i64, kind: &str, language: &str) -> Result<Value> {
    mirrored_asset_key(kind, (!language.is_empty()).then_some(language))?;
    ensure!(
        metadata["adapter"]["client_version"].as_i64() == Some(version)
            && metadata["adapter"]["kind"].as_str() == Some(kind)
            && if language.is_empty() {
                metadata["adapter"].get("language") == Some(&Value::Null)
            } else {
                metadata["adapter"]["language"].as_str() == Some(language)
            },
        "Assets-Version, Art oder Sprache stimmen nicht überein"
    );
    ensure!(
        metadata["validation"]["state"].as_str() == Some("validated"),
        "Assets sind nicht validiert"
    );
    let payload = metadata
        .pointer("/contract/data/payload/value")
        .ok_or_else(|| anyhow!("Lokaler Assets-Spiegel enthält keine Originaldaten"))?;
    ensure!(
        if kind == "generic_data" {
            payload
                .as_object()
                .is_some_and(|entries| !entries.is_empty())
        } else {
            payload
                .as_array()
                .is_some_and(|entries| !entries.is_empty())
        },
        "Lokale Assets sind leer oder haben den falschen Datentyp"
    );
    Ok(payload.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn mirror_read_rejects_empty_or_wrong_containers_for_every_required_endpoint() {
        for kind in REQUIRED_MIRROR_KINDS {
            for language in ["english", "german"] {
                for payload in [json!([]), json!({}), Value::Null] {
                    let metadata = json!({"adapter":{"client_version":6759,"kind":kind,"language":language},
                        "validation":{"state":"validated"},"contract":{"data":{"payload":{"value":payload}}}});
                    assert!(mirrored_payload(&metadata, 6759, kind, language).is_err());
                }
            }
        }
    }

    #[test]
    fn mirror_read_preserves_values_and_checks_binding() {
        let payload = json!([{"id":1998374645,"name":"Mystischer Ausbruch","properties":{"Damage":"40","Radius":"16m"}}]);
        let mut metadata = json!({"adapter":{"client_version":6759,"kind":"items","language":"german"},
            "validation":{"state":"validated"},"contract":{"data":{"payload":{"value":payload}}}});
        assert_eq!(
            mirrored_payload(&metadata, 6759, "items", "german").unwrap(),
            payload
        );
        assert!(mirrored_payload(&metadata, 6757, "items", "german").is_err());
        assert!(mirrored_payload(&metadata, 6759, "items", "english").is_err());
        metadata["validation"]["state"] = json!("quarantined");
        assert!(mirrored_payload(&metadata, 6759, "items", "german").is_err());
    }

    #[test]
    fn globals_are_versioned_and_modifiers_have_no_language() {
        assert_eq!(mirrored_asset_key("modifiers", None).unwrap(), "modifiers");
        assert!(mirrored_asset_key("modifiers", Some("english")).is_err());
        for kind in ["generic_data", "npc_units", "misc_entities"] {
            assert!(mirrored_asset_key(kind, None).is_err());
            for language in ["english", "german"] {
                assert_eq!(
                    mirrored_asset_key(kind, Some(language)).unwrap(),
                    format!("{kind}/{language}")
                );
            }
        }
        let payload = json!({"item_price_per_tier":[800,1600,3200,6400]});
        let metadata = json!({"adapter":{"client_version":6759,"kind":"generic_data","language":"german"},
            "validation":{"state":"validated"},"contract":{"data":{"payload":{"value":payload}}}});
        assert_eq!(
            mirrored_payload(&metadata, 6759, "generic_data", "german").unwrap(),
            payload
        );
    }
}
