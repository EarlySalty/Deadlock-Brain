use anyhow::{ensure, Context, Result};
use brain_contracts::{DocumentStorePort, SourceVisibility};
use brain_feeds::{
    patchnotes::{prepare_batch, SOURCE},
    patchnotes_http::{endpoint, fetch_feed, http_client, validated_feed, DEFAULT_URL},
    FeedPolicy,
};
use brain_ingestion::document_set::current_pins;
use brain_storage::PgStore;
use serde_json::json;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};
use std::{str::FromStr, time::Duration};

const MAX_POSTS: i64 = 10_000;

#[derive(sqlx::FromRow)]
struct ChangelogPost {
    id: i64,
    title: String,
    url: String,
    posted_at: Option<String>,
    raw_content: Option<String>,
    translated_content: Option<String>,
}

pub async fn mirror_changelog_posts(
    source_dsn: &str,
    target: &PgPool,
) -> Result<serde_json::Value> {
    let posts = read_source_posts(source_dsn).await?;
    let mut inserted = 0usize;
    let mut updated = 0usize;
    let mut tx = target.begin().await?;
    for post in &posts {
        let outcome: Option<bool> = sqlx::query_scalar(
            "INSERT INTO patchnotes.changelog_posts(id,title,url,posted_at,raw_content,translated_content) \
             VALUES($1,$2,$3,$4::timestamptz,$5,$6) \
             ON CONFLICT (id) DO UPDATE SET title=EXCLUDED.title,url=EXCLUDED.url,posted_at=EXCLUDED.posted_at,\
             raw_content=EXCLUDED.raw_content,translated_content=EXCLUDED.translated_content \
             WHERE (changelog_posts.title,changelog_posts.url,changelog_posts.posted_at,changelog_posts.raw_content,changelog_posts.translated_content) \
             IS DISTINCT FROM (EXCLUDED.title,EXCLUDED.url,EXCLUDED.posted_at,EXCLUDED.raw_content,EXCLUDED.translated_content) \
             RETURNING (xmax = 0)",
        )
        .bind(post.id)
        .bind(&post.title)
        .bind(&post.url)
        .bind(&post.posted_at)
        .bind(&post.raw_content)
        .bind(&post.translated_content)
        .fetch_optional(&mut *tx)
        .await?;
        match outcome {
            Some(true) => inserted += 1,
            Some(false) => updated += 1,
            None => {}
        }
    }
    tx.commit().await?;
    Ok(json!({
        "source_posts": posts.len(),
        "newest_post_id": posts.iter().map(|post| post.id).max(),
        "inserted": inserted,
        "updated": updated,
    }))
}

pub async fn ingest_canonical_feed(
    store: &PgStore,
    key: &str,
    owner: &str,
) -> Result<serde_json::Value> {
    let client = http_client().map_err(anyhow::Error::msg)?;
    let url = endpoint(DEFAULT_URL).map_err(anyhow::Error::msg)?;
    let bytes = fetch_feed(&client, url, key)
        .await
        .map_err(anyhow::Error::msg)?;
    let feed = validated_feed(&bytes).map_err(anyhow::Error::msg)?;
    let policy = FeedPolicy {
        visibility: SourceVisibility::Public,
        allowed_scopes: std::collections::BTreeSet::from(["bot.public".into()]),
        provider_egress_allowed: true,
        publication_allowed: true,
        raw_retention_allowed: true,
    };
    prepare_batch(&feed, &policy, None)?;
    let lease = store.claim(SOURCE, owner, 60_000).await?;
    let previous = store.checkpoint(SOURCE).await?;
    let batch = prepare_batch(&feed, &policy, previous.as_ref())?;
    let receipt = store.commit(&batch, &lease).await?;
    Ok(json!({
        "source_id": SOURCE,
        "status": if batch.records.is_empty() { "up_to_date" } else { "source_committed" },
        "export_revision": feed.export_revision,
        "generation": receipt.generation,
        "replayed": receipt.replayed,
        "records": batch.records.len(),
        "tombstones": batch.records.iter().filter(|record| record.tombstone).count(),
        "live_documents": current_pins(&batch.checkpoint)?.len(),
        "publication_allowed": true,
        "provider_egress_allowed": true,
        "activation_performed": false,
    }))
}

async fn read_source_posts(source_dsn: &str) -> Result<Vec<ChangelogPost>> {
    let options = PgConnectOptions::from_str(source_dsn)
        .map_err(|_| anyhow::anyhow!("patch_feed_dsn_invalid"))?
        .options([
            ("default_transaction_read_only", "on"),
            ("statement_timeout", "30000"),
            ("TimeZone", "UTC"),
        ]);
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(15))
        .connect_with(options)
        .await
        .map_err(|_| anyhow::anyhow!("patch_feed_unavailable"))?;
    let posts: Vec<ChangelogPost> = sqlx::query_as(
        "SELECT id,title,url,posted_at::text AS posted_at,raw_content,translated_content \
         FROM patchnotes.changelog_posts ORDER BY id LIMIT $1",
    )
    .bind(MAX_POSTS + 1)
    .fetch_all(&pool)
    .await
    .context("patch_feed_query")?;
    pool.close().await;
    ensure!(
        !posts.is_empty() && posts.len() as i64 <= MAX_POSTS,
        "patch_feed_size"
    );
    Ok(posts)
}
