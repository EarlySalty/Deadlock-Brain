use std::path::Path;

use serde_json::{json, Map, Value};
use sqlx::Row;

use crate::{
    store::{
        complete_run, json_bytes, open_pool, EntitySnapshotInput, SourceDocumentInput, SourceStore,
    },
    Result,
};

pub const SOURCE: &str = "deadlock_patchnotes_db";

/// Optionen fuer den Patchnotes-Import. Die frueher noetige SQLite-Pfadangabe
/// entfaellt: die Changelog-Quelle liegt jetzt in der zentralen Postgres
/// (`patchnotes.changelog_posts`), die der Import ueber `pg_pool()` selbst
/// oeffnet.
#[derive(Debug, Clone, Default)]
pub struct PullPatchnotesOptions;

pub fn classify_source_kind(url: Option<&str>) -> &'static str {
    let lower = url.unwrap_or_default().to_lowercase();
    if lower.contains("steamcommunity.com")
        || lower.contains("steampowered.com")
        || lower.contains("steamstore-a.akamaihd.net")
    {
        "steam"
    } else if lower.contains("forums.playdeadlock.com") {
        "forum"
    } else {
        "other"
    }
}

pub async fn pull_patchnotes(
    raw_dir: &Path,
    _options: PullPatchnotesOptions,
) -> Result<Value> {
    let pool = open_pool().await?;
    let store = SourceStore::new(&pool, raw_dir)?;
    let run_id = store.begin_run("patchnotes").await?;
    let outcome = pull_patchnotes_inner(&store).await;
    complete_run(&store, run_id, outcome).await
}

async fn pull_patchnotes_inner(store: &SourceStore<'_>) -> Result<Value> {
    // Laufzeit-Query (bewusst kein compile-geprueftes `query!`): die Quelle liegt
    // im `patchnotes`-Schema des Patchnotes-Bots, das nicht Teil des
    // brain-Validierungs-Schemas ist. Keine Werte werden interpoliert.
    let rows = sqlx::query(
        "SELECT id, title, url, posted_at::text AS posted_at, raw_content, translated_content \
         FROM patchnotes.changelog_posts \
         WHERE raw_content IS NOT NULL AND raw_content <> '' \
         ORDER BY id ASC",
    )
    .fetch_all(store.pool())
    .await?;

    let mut imported = 0usize;
    let mut source_kinds = Map::new();
    for row in &rows {
        let id: i64 = row.try_get("id")?;
        let title: Option<String> = row.try_get("title")?;
        let url: Option<String> = row.try_get("url")?;
        let posted_at_raw: Option<String> = row.try_get("posted_at")?;
        let posted_at = posted_at_or_title_date(posted_at_raw.as_deref(), title.as_deref());
        let raw_content: String = row.try_get("raw_content")?;
        let translated_content: Option<String> = row.try_get("translated_content")?;

        let external_id = url
            .as_deref()
            .filter(|value| !value.is_empty())
            .map(ToString::to_string)
            .unwrap_or_else(|| id.to_string());
        let source_kind = classify_source_kind(url.as_deref());
        increment_counter(&mut source_kinds, source_kind);
        let payload = json!({
            "id": id,
            "title": title.clone(),
            "url": url.clone(),
            "posted_at": posted_at.clone(),
            "raw_content": raw_content,
            "translated_content": translated_content.clone(),
        });
        let raw = json_bytes(&payload)?;
        let raw_external_id = id.to_string();
        let raw_path = store.write_raw(SOURCE, &raw_external_id, &raw, "json")?;
        let metadata = json!({
            "changelog_post_id": id,
            "source_kind": source_kind,
            "source_table": "patchnotes.changelog_posts",
        });
        let document_id = store
            .upsert_source_document(SourceDocumentInput {
                source: SOURCE,
                external_id: &external_id,
                title: title.as_deref(),
                url: url.as_deref(),
                content_type: "application/json",
                raw_path: &raw_path,
                content: &raw,
                metadata: &metadata,
            })
            .await?;
        store
            .upsert_entity_snapshot(
                &EntitySnapshotInput {
                    source: SOURCE.to_string(),
                    entity_type: "patchnote".to_string(),
                    external_id,
                    canonical_name: title,
                    payload,
                },
                Some(document_id),
            )
            .await?;
        imported += 1;
    }

    Ok(json!({
        "source_table": "patchnotes.changelog_posts",
        "patchnotes": imported,
        "source_kinds": Value::Object(source_kinds),
    }))
}

fn increment_counter(counters: &mut Map<String, Value>, key: &str) {
    let next = counters.get(key).and_then(Value::as_i64).unwrap_or(0) + 1;
    counters.insert(key.to_string(), json!(next));
}

fn posted_at_or_title_date(posted_at: Option<&str>, title: Option<&str>) -> Option<String> {
    posted_at
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
        .or_else(|| title_date_rfc3339(title))
}

fn title_date_rfc3339(title: Option<&str>) -> Option<String> {
    title
        .and_then(extract_mm_dd_yyyy)
        .map(|(month, day, year)| format!("{year:04}-{month:02}-{day:02}T00:00:00+00:00"))
}

fn extract_mm_dd_yyyy(value: &str) -> Option<(u32, u32, u32)> {
    for bytes in value.as_bytes().windows(10) {
        if bytes[2] == b'-'
            && bytes[5] == b'-'
            && bytes[..2].iter().all(u8::is_ascii_digit)
            && bytes[3..5].iter().all(u8::is_ascii_digit)
            && bytes[6..].iter().all(u8::is_ascii_digit)
        {
            let month = std::str::from_utf8(&bytes[..2]).ok()?.parse().ok()?;
            let day = std::str::from_utf8(&bytes[3..5]).ok()?.parse().ok()?;
            let year = std::str::from_utf8(&bytes[6..]).ok()?.parse().ok()?;
            if (1..=12).contains(&month) && (1..=31).contains(&day) {
                return Some((month, day, year));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_source_kind_maps_known_hosts() {
        assert_eq!(
            classify_source_kind(Some("https://steamcommunity.com/games/x")),
            "steam"
        );
        assert_eq!(
            classify_source_kind(Some("https://forums.playdeadlock.com/threads/2")),
            "forum"
        );
        assert_eq!(classify_source_kind(Some("https://example.com")), "other");
        assert_eq!(classify_source_kind(None), "other");
    }

    #[test]
    fn posted_at_falls_back_to_patch_title_date() {
        assert_eq!(
            posted_at_or_title_date(None, Some("07-09-2026 Update")).as_deref(),
            Some("2026-07-09T00:00:00+00:00")
        );
        assert_eq!(
            posted_at_or_title_date(
                Some("2026-07-09 19:42:11+00"),
                Some("07-09-2026 Update")
            )
            .as_deref(),
            Some("2026-07-09 19:42:11+00")
        );
    }

    /// PG-Integration nur, wenn die Changelog-Quelle im `patchnotes`-Schema
    /// existiert. Auf der brain-only Scratch-PG fehlt sie -> Test-Skip ohne
    /// destruktiven Schreibpfad (Zaehler bleiben unveraendert).
    #[tokio::test]
    #[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN with patchnotes.changelog_posts"]
    async fn patchnotes_source_is_readable_when_present() {
        use sqlx::postgres::PgPoolOptions;
        let Ok(dsn) = std::env::var("DEADLOCK_CENTRAL_DSN") else {
            return;
        };
        let Ok(pool) = PgPoolOptions::new().max_connections(1).connect(&dsn).await else {
            return;
        };
        let present: bool =
            sqlx::query_scalar("SELECT to_regclass('patchnotes.changelog_posts') IS NOT NULL")
                .fetch_one(&pool)
                .await
                .unwrap_or(false);
        if !present {
            return;
        }
        // Reiner Lesbarkeits-Check, kein Schreibpfad (echte Zaehler unangetastet).
        let readable = sqlx::query("SELECT id FROM patchnotes.changelog_posts LIMIT 1")
            .fetch_optional(&pool)
            .await;
        assert!(readable.is_ok());
    }
}
