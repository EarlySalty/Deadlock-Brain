use std::collections::HashSet;

use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::{PgPool, Row};

use crate::patch::append_unparsed_patches;
use crate::util::stable_hash_text;
use crate::Result;

const FEED_SOURCE: &str = "deadlock_patchnotes_db";

#[derive(Debug, Deserialize)]
struct FeedPost {
    id: i64,
    title: Option<String>,
    url: Option<String>,
    posted_at: Option<String>,
    raw_content: Option<String>,
}

impl FeedPost {
    fn external_id(&self) -> String {
        self.url
            .as_deref()
            .map(str::trim)
            .filter(|url| !url.is_empty())
            .map_or_else(|| self.id.to_string(), ToString::to_string)
    }

    fn has_content(&self) -> bool {
        self.raw_content
            .as_deref()
            .is_some_and(|content| !content.trim().is_empty())
    }

    fn payload(&self) -> Value {
        json!({
            "id": self.id,
            "title": self.title,
            "url": self.url,
            "posted_at": self.posted_at,
            "raw_content": self.raw_content,
        })
    }
}

pub(crate) async fn sync_patchnote_posts(pool: &PgPool, dry_run: bool) -> Result<Value> {
    let posts = load_feed_posts(pool).await?;
    let known = load_known_post_keys(pool).await?;
    let new_posts = posts
        .iter()
        .filter(|post| post.has_content() && !known.contains(&post_key(&post.external_id())))
        .collect::<Vec<_>>();
    let mut staged = Vec::new();
    for post in &new_posts {
        if dry_run {
            staged.push((post.external_id(), post.payload()));
        } else {
            insert_post_snapshot(pool, post).await?;
        }
    }
    let events = if new_posts.is_empty() {
        json!({"dry_run": dry_run, "patches": [], "events_inserted": 0})
    } else {
        append_unparsed_patches(pool, &staged, dry_run).await?
    };
    Ok(json!({
        "dry_run": dry_run,
        "feed_posts": posts.len(),
        "new_posts": new_posts
            .iter()
            .map(|post| json!([post.id, post.title, post.posted_at]))
            .collect::<Vec<_>>(),
        "events": events,
    }))
}

fn post_key(value: &str) -> String {
    value.trim().trim_end_matches('/').to_lowercase()
}

async fn load_feed_posts(pool: &PgPool) -> Result<Vec<FeedPost>> {
    let has_raw_store: bool = sqlx::query_scalar("SELECT to_regclass('raw.object') IS NOT NULL")
        .fetch_one(pool)
        .await?;
    let rows = if has_raw_store {
        sqlx::query(
            "SELECT DISTINCT ON (path) bytes FROM raw.object WHERE source = $1 ORDER BY path, id DESC",
        )
        .bind(FEED_SOURCE)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query(
            r#"
            SELECT convert_to(json_build_object(
              'id', id, 'title', title, 'url', url, 'posted_at', posted_at, 'raw_content', raw_content
            )::text, 'UTF8') AS bytes
            FROM patchnotes.changelog_posts
            "#,
        )
        .fetch_all(pool)
        .await?
    };
    let mut posts = Vec::with_capacity(rows.len());
    for row in rows {
        let bytes: Vec<u8> = row.try_get("bytes")?;
        posts.push(serde_json::from_slice(&bytes)?);
    }
    Ok(posts)
}

async fn load_known_post_keys(pool: &PgPool) -> Result<HashSet<String>> {
    let rows = sqlx::query(
        "SELECT external_id, payload->>'url' AS url FROM brain.entity_snapshots WHERE entity_type = 'patchnote'",
    )
    .fetch_all(pool)
    .await?;
    let mut keys = HashSet::new();
    for row in rows {
        let external_id: String = row.try_get("external_id")?;
        keys.insert(post_key(&external_id));
        if let Some(url) = row.try_get::<Option<String>, _>("url")? {
            keys.insert(post_key(&url));
        }
    }
    Ok(keys)
}

async fn insert_post_snapshot(pool: &PgPool, post: &FeedPost) -> Result<()> {
    let payload = serde_json::to_string(&post.payload())?;
    sqlx::query(
        r#"
        INSERT INTO brain.entity_snapshots(
          source, entity_type, external_id, canonical_name, payload_hash, payload, fetched_at
        )
        VALUES($1, 'patchnote', $2, $3, $4, $5::text::jsonb, now())
        ON CONFLICT (source, entity_type, external_id, payload_hash) DO NOTHING
        "#,
    )
    .bind(FEED_SOURCE)
    .bind(post.external_id())
    .bind(&post.title)
    .bind(stable_hash_text(&payload))
    .bind(&payload)
    .execute(pool)
    .await?;
    Ok(())
}
