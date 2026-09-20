//! Beschreibungen stammen aus derselben Quellrevision wie die jeweilige Karte.
use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use sqlx::{PgPool, Row};

use super::SnapshotRow;
use crate::Result;

struct Localization {
    id: i64,
    language: String,
    revision: String,
    values: Value,
    fetched_at: Option<DateTime<Utc>>,
    url: Option<String>,
}

pub(super) async fn enrich(pool: &PgPool, snapshots: &mut [SnapshotRow]) -> Result<()> {
    // An die bereits ausgewählten Karten binden, auch wenn währenddessen ein
    // weiterer Import fertig wird.
    let revisions: Vec<String> = snapshots
        .iter()
        .filter_map(|snapshot| {
            snapshot
                .payload
                .pointer("/_deadlock_data/commit_sha")?
                .as_str()
        })
        .map(str::to_owned)
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();
    let rows = sqlx::query(
        r#"
        SELECT DISTINCT ON (es.external_id, es.payload->'_deadlock_data'->>'commit_sha')
            es.id, es.external_id, es.payload, es.fetched_at, sd.url
        FROM brain.entity_snapshots es
        LEFT JOIN brain.source_documents sd ON sd.id = es.source_document_id
        WHERE es.source = 'deadlock_data' AND es.entity_type = 'localization'
          AND es.external_id IN ('german', 'english')
          AND es.payload->'_deadlock_data'->>'commit_sha' = ANY($1)
        ORDER BY es.external_id DESC, es.payload->'_deadlock_data'->>'commit_sha',
                 es.fetched_at DESC, es.id DESC
    "#,
    )
    .bind(&revisions)
    .fetch_all(pool)
    .await?;
    let mut translations = Vec::with_capacity(rows.len());
    for row in rows {
        let payload: Value = row.try_get("payload")?;
        translations.push(Localization {
            id: row.try_get("id")?,
            language: row.try_get("external_id")?,
            revision: payload
                .pointer("/_deadlock_data/commit_sha")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_owned(),
            values: payload["values"].clone(),
            fetched_at: row.try_get("fetched_at")?,
            url: row.try_get("url")?,
        });
    }
    for snapshot in snapshots {
        attach(&mut snapshot.payload, &translations);
    }
    Ok(())
}

fn attach(payload: &mut Value, translations: &[Localization]) {
    if let Some(key) = payload["DescKey"].as_str().map(str::to_owned) {
        attach_text(
            payload,
            translations,
            &key,
            "Description",
            "_wiki_description_source",
        );
    }
    if let Some(name) = payload["Name"].as_str() {
        // Exakte Namenskonvention: nur Leerraum entfernen, keine Ähnlichkeitssuche.
        let compact: String = name
            .chars()
            .filter(|character| !character.is_whitespace())
            .collect();
        if !compact.is_empty() {
            let key = format!("StatDesc_{compact}Desc");
            attach_text(
                payload,
                translations,
                &key,
                "MechanicsDescription",
                "_wiki_mechanics_source",
            );
        }
    }
}

fn attach_text(
    payload: &mut Value,
    translations: &[Localization],
    key: &str,
    field: &str,
    provenance_field: &str,
) {
    if payload[field]
        .as_str()
        .is_some_and(|value| !value.trim().is_empty())
    {
        return;
    }
    let Some(revision) = payload
        .pointer("/_deadlock_data/commit_sha")
        .and_then(Value::as_str)
    else {
        return;
    };
    let Some((source, text)) = translations
        .iter()
        .filter(|source| !source.revision.is_empty() && source.revision == revision)
        .find_map(|source| {
            source.values[key]
                .as_str()
                .filter(|value| !value.trim().is_empty())
                .map(|text| (source, text))
        })
    else {
        return;
    };
    // Platzhalter bleiben unverändert: keine Zahlen- oder Ablaufinterpretation.
    let provenance = json!({"key": key, "language": source.language,
        "snapshot_id": source.id, "source_revision": source.revision,
        "fetched_at": source.fetched_at.map(|date| date.to_rfc3339()),
        "source_url": source.url});
    payload[field] = json!(text);
    payload[provenance_field] = provenance;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mechanics_use_only_exact_normalized_stat_key() {
        let source = Localization {
            id: 1,
            language: "english".into(),
            revision: "same".into(),
            values: json!({"StatDesc_VitalReturnDesc":"Recovery from matching damage."}),
            fetched_at: None,
            url: None,
        };
        let mut payload =
            json!({"Name":"Vital Return", "Cost":100, "_deadlock_data":{"commit_sha":"same"}});
        attach(&mut payload, std::slice::from_ref(&source));
        assert_eq!(
            payload["MechanicsDescription"],
            "Recovery from matching damage."
        );
        assert_eq!(payload["Cost"], 100);
        assert_eq!(
            payload["_wiki_mechanics_source"]["key"],
            "StatDesc_VitalReturnDesc"
        );
        let mut similar = json!({"Name":"Vital Returns", "_deadlock_data":{"commit_sha":"same"}});
        attach(&mut similar, &[source]);
        assert!(similar["MechanicsDescription"].is_null());
    }

    #[test]
    fn descriptions_require_matching_revision_and_existing_key() {
        let source = Localization {
            id: 1,
            language: "german".into(),
            revision: "revision-a".into(),
            values: json!({"generic_desc":"Erst nach {s:Delay} tritt die Wirkung ein."}),
            fetched_at: None,
            url: None,
        };
        let mut payload =
            json!({"DescKey":"generic_desc", "_deadlock_data":{"commit_sha":"revision-b"}});
        attach(&mut payload, std::slice::from_ref(&source));
        assert!(payload["Description"].is_null());
        payload["_deadlock_data"]["commit_sha"] = json!("revision-a");
        attach(&mut payload, std::slice::from_ref(&source));
        assert_eq!(
            payload["Description"],
            "Erst nach {s:Delay} tritt die Wirkung ein."
        );
        assert_eq!(
            payload["_wiki_description_source"]["source_revision"],
            "revision-a"
        );
        payload["Description"] = Value::Null;
        payload["DescKey"] = json!("missing_desc");
        attach(&mut payload, std::slice::from_ref(&source));
        assert!(payload["Description"].is_null());
    }
}
