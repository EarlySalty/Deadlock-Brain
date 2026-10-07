use anyhow::{anyhow, ensure, Context, Result};
use serde_json::Value;
use sqlx::PgPool;

pub async fn latest_mirrored_client_version(pool: &PgPool) -> Result<i64> {
    let version = sqlx::query_scalar::<_, String>(
        "SELECT sr.summary->>'client_version' FROM brain.source_runs sr \
         WHERE sr.source='assets' AND sr.status='ok' AND sr.summary->>'mirror_complete'='true' \
           AND NOT EXISTS ( \
             SELECT 1 FROM (VALUES ('items/english'), ('items/german'), ('heroes/english'), \
               ('heroes/german'), ('heroes_all/english'), ('heroes_all/german')) AS required(key) \
             LEFT JOIN brain.source_documents sd \
               ON sd.id=(sr.summary->'endpoints'->required.key->>'source_document_id')::bigint \
               AND sd.source='deadlock_assets_api' \
             WHERE NOT COALESCE( \
               jsonb_typeof(sd.metadata #> '{contract,data,payload,value}')='array' \
               AND sd.metadata #> '{contract,data,payload,value}' <> '[]'::jsonb, false)) \
         ORDER BY (sr.summary->>'client_version')::bigint DESC, sr.id DESC LIMIT 1",
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| anyhow!("Kein vollständiger lokaler Assets-Spiegel vorhanden"))?;
    let version: i64 = version
        .parse()
        .context("Ungültige gespiegelte Clientversion")?;
    ensure!(version > 0, "Ungültige gespiegelte Clientversion");
    Ok(version)
}

pub async fn load_mirrored_assets(
    pool: &PgPool,
    client_version: i64,
    kind: &str,
    language: &str,
) -> Result<Value> {
    ensure!(client_version > 0, "Ungültige Clientversion");
    ensure!(
        matches!(kind, "items" | "heroes" | "heroes_all"),
        "Unbekannte Assets-Art"
    );
    ensure!(
        matches!(language, "english" | "german"),
        "Unbekannte Assets-Sprache"
    );
    let key = format!("{kind}/{language}");
    let metadata = sqlx::query_scalar::<_, String>(
        "SELECT sd.metadata::text FROM brain.source_runs sr \
         JOIN brain.source_documents sd ON sd.id=(sr.summary->'endpoints'->$2->>'source_document_id')::bigint \
         WHERE sr.source='assets' AND sr.status='ok' AND sr.summary->>'mirror_complete'='true' \
           AND sr.summary->>'client_version'=$1 AND sd.source='deadlock_assets_api' \
         ORDER BY sr.id DESC LIMIT 1",
    )
    .bind(client_version.to_string())
    .bind(&key)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| anyhow!("Lokale Assets fehlen: {client_version}/{key}"))?;
    mirrored_payload(
        &serde_json::from_str(&metadata)?,
        client_version,
        kind,
        language,
    )
}

fn mirrored_payload(metadata: &Value, version: i64, kind: &str, language: &str) -> Result<Value> {
    ensure!(
        metadata["adapter"]["client_version"].as_i64() == Some(version)
            && metadata["adapter"]["kind"].as_str() == Some(kind)
            && metadata["adapter"]["language"].as_str() == Some(language),
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
        payload
            .as_array()
            .is_some_and(|entries| !entries.is_empty()),
        "Lokale Pflicht-Assets sind leer oder keine Liste"
    );
    Ok(payload.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn mirror_read_rejects_empty_or_wrong_containers_for_every_required_endpoint() {
        for kind in ["items", "heroes", "heroes_all"] {
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
}
