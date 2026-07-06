//! Test-Hilfen für die Postgres-Integrationstests (nur unter `cfg(test)`).
//!
//! Alle Helfer nutzen die Wegwerf-Postgres aus `DEADLOCK_CENTRAL_DSN` und
//! räumen ihre Seed-Zeilen wieder auf (`cleanup`), damit die geteilte
//! Scratch-DB nicht verschmutzt. Bewusst Laufzeit-Queries (`sqlx::query`),
//! damit Testcode keinen `.sqlx`-Cache braucht.
#![allow(dead_code)]

use sqlx::postgres::{PgPool, PgPoolOptions};

/// Baut einen Pool gegen die Scratch-Postgres oder `None`, wenn kein DSN
/// gesetzt ist (dann überspringt der Test).
pub async fn test_pool() -> Option<PgPool> {
    let dsn = std::env::var("DEADLOCK_CENTRAL_DSN").ok()?;
    PgPoolOptions::new()
        .max_connections(2)
        .connect(&dsn)
        .await
        .ok()
}

pub fn unique_suffix() -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|value| value.as_nanos())
        .unwrap_or(0);
    format!("{}_{}", std::process::id(), nanos)
}

pub async fn seed_feed(pool: &PgPool, feed_key: &str) {
    sqlx::query(
        r#"
        INSERT INTO brain.youtube_feed_sources(
          feed_key, source_type, url, enabled, metadata, created_at, updated_at
        )
        VALUES($1, 'channel', 'https://example.invalid', 1, '{}'::jsonb, now(), now())
        ON CONFLICT(feed_key) DO NOTHING
        "#,
    )
    .bind(feed_key)
    .execute(pool)
    .await
    .expect("seed feed");
}

#[allow(clippy::too_many_arguments)]
pub async fn seed_video(
    pool: &PgPool,
    video_id: &str,
    feed_key: &str,
    transcript_status: &str,
    learning_status: &str,
    published_at: Option<&str>,
    metadata_json: &str,
) {
    sqlx::query(
        r#"
        INSERT INTO brain.youtube_videos(
          video_id, feed_key, channel_title, title, url, published_at, metadata,
          transcript_status, learning_status, discovered_at, updated_at
        )
        VALUES($1, $2, 'Channel', $1, $3, $4::timestamptz, $5::text::jsonb, $6, $7, now(), now())
        ON CONFLICT(video_id) DO UPDATE SET
          transcript_status=excluded.transcript_status,
          learning_status=excluded.learning_status,
          metadata=excluded.metadata,
          updated_at=now()
        "#,
    )
    .bind(video_id)
    .bind(feed_key)
    .bind(format!("https://youtube.com/watch?v={video_id}"))
    .bind(published_at)
    .bind(metadata_json)
    .bind(transcript_status)
    .bind(learning_status)
    .execute(pool)
    .await
    .expect("seed video");
}

pub async fn seed_transcript(pool: &PgPool, video_id: &str, transcript_text: &str) {
    sqlx::query(
        r#"
        INSERT INTO brain.youtube_transcripts(
          video_id, language, source_kind, transcript_text, content_hash,
          source_document_id, imported_at, updated_at
        )
        VALUES($1, 'en', 'youtube_caption_manual', $2, $3, NULL, now(), now())
        ON CONFLICT(video_id) DO UPDATE SET
          transcript_text=excluded.transcript_text,
          content_hash=excluded.content_hash,
          updated_at=now()
        "#,
    )
    .bind(video_id)
    .bind(transcript_text)
    .bind(format!("ztest-hash-{video_id}-{}", transcript_text.len()))
    .execute(pool)
    .await
    .expect("seed transcript");
}

pub async fn insert_claim_marker(
    pool: &PgPool,
    video_id: &str,
    prompt_version: &str,
    model: &str,
    claim_hash: &str,
) {
    sqlx::query(
        r#"
        INSERT INTO brain.youtube_learning_claims(
          video_id, claim_hash, claim_index, claim_type, claim_text, evidence_quote,
          model_confidence, verifier_confidence, status, model, prompt_version, prompt_text,
          model_response_text, provider_metadata, verifier, created_at, updated_at
        )
        VALUES($1, $2, 0, 'build', 'marker', '', 0.5, 0.0, 'accepted', $3, $4, 'prompt',
               '', '{}'::jsonb, '{}'::jsonb, now(), now())
        ON CONFLICT(claim_hash) DO NOTHING
        "#,
    )
    .bind(video_id)
    .bind(claim_hash)
    .bind(model)
    .bind(prompt_version)
    .execute(pool)
    .await
    .expect("seed claim marker");
}

pub async fn cleanup(pool: &PgPool, video_ids: &[&str], feed_keys: &[&str]) {
    for video_id in video_ids {
        let _ = sqlx::query("DELETE FROM brain.youtube_learning_claims WHERE video_id=$1")
            .bind(video_id)
            .execute(pool)
            .await;
        let _ = sqlx::query("DELETE FROM brain.youtube_transcript_claim_attempts WHERE video_id=$1")
            .bind(video_id)
            .execute(pool)
            .await;
        let _ = sqlx::query("DELETE FROM brain.youtube_transcripts WHERE video_id=$1")
            .bind(video_id)
            .execute(pool)
            .await;
        let _ = sqlx::query("DELETE FROM brain.youtube_videos WHERE video_id=$1")
            .bind(video_id)
            .execute(pool)
            .await;
    }
    for feed_key in feed_keys {
        let _ = sqlx::query("DELETE FROM brain.youtube_feed_sources WHERE feed_key=$1")
            .bind(feed_key)
            .execute(pool)
            .await;
    }
}
