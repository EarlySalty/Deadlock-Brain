use rusqlite::Connection;

pub const YOUTUBE_SCHEMA: &str = r#"
        CREATE TABLE IF NOT EXISTS youtube_feed_sources (
          feed_key TEXT PRIMARY KEY,
          source_type TEXT NOT NULL,
          url TEXT NOT NULL,
          handle TEXT,
          playlist_id TEXT,
          channel_id TEXT,
          title TEXT,
          enabled INTEGER NOT NULL DEFAULT 1,
          metadata_json TEXT NOT NULL DEFAULT '{}',
          created_at INTEGER NOT NULL,
          updated_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS youtube_videos (
          video_id TEXT PRIMARY KEY,
          feed_key TEXT NOT NULL,
          channel_id TEXT,
          channel_title TEXT,
          title TEXT NOT NULL,
          url TEXT NOT NULL,
          published_at TEXT,
          description TEXT,
          metadata_json TEXT NOT NULL DEFAULT '{}',
          transcript_status TEXT NOT NULL DEFAULT 'missing',
          learning_status TEXT NOT NULL DEFAULT 'queued',
          discovered_at INTEGER NOT NULL,
          updated_at INTEGER NOT NULL,
          FOREIGN KEY(feed_key) REFERENCES youtube_feed_sources(feed_key)
        );

        CREATE INDEX IF NOT EXISTS idx_youtube_videos_learning
          ON youtube_videos(learning_status, transcript_status, published_at);

        CREATE TABLE IF NOT EXISTS youtube_transcripts (
          video_id TEXT PRIMARY KEY,
          language TEXT,
          source_kind TEXT NOT NULL,
          transcript_text TEXT NOT NULL,
          content_hash TEXT NOT NULL,
          source_document_id INTEGER,
          imported_at INTEGER NOT NULL,
          updated_at INTEGER NOT NULL,
          FOREIGN KEY(video_id) REFERENCES youtube_videos(video_id),
          FOREIGN KEY(source_document_id) REFERENCES source_documents(id)
        );

        CREATE TABLE IF NOT EXISTS youtube_learning_claims (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          video_id TEXT NOT NULL,
          claim_hash TEXT NOT NULL UNIQUE,
          claim_index INTEGER NOT NULL,
          entity_type TEXT,
          entity_name TEXT,
          claim_type TEXT NOT NULL,
          claim_text TEXT NOT NULL,
          evidence_quote TEXT NOT NULL,
          timestamp_seconds REAL,
          model_confidence REAL NOT NULL,
          verifier_confidence REAL NOT NULL,
          status TEXT NOT NULL,
          model TEXT,
          prompt_version TEXT NOT NULL,
          prompt_text TEXT NOT NULL,
          model_response_text TEXT NOT NULL,
          provider_metadata_json TEXT NOT NULL DEFAULT '{}',
          verifier_json TEXT NOT NULL DEFAULT '{}',
          created_at INTEGER NOT NULL,
          updated_at INTEGER NOT NULL,
          FOREIGN KEY(video_id) REFERENCES youtube_videos(video_id)
        );

        CREATE INDEX IF NOT EXISTS idx_youtube_learning_claims_video
          ON youtube_learning_claims(video_id, status);
"#;

pub fn ensure_youtube_tables(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(YOUTUBE_SCHEMA)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_youtube_schema_in_disposable_sqlite() {
        let conn = Connection::open_in_memory().expect("open sqlite");
        ensure_youtube_tables(&conn).expect("ensure schema");

        let tables: Vec<String> = conn
            .prepare(
                "SELECT name FROM sqlite_master WHERE type='table' AND name LIKE 'youtube_%' ORDER BY name",
            )
            .expect("prepare")
            .query_map([], |row| row.get(0))
            .expect("query")
            .collect::<Result<Vec<String>, _>>()
            .expect("collect");

        assert_eq!(
            tables,
            vec![
                "youtube_feed_sources",
                "youtube_learning_claims",
                "youtube_transcripts",
                "youtube_videos"
            ]
        );

        let learning_status = conn
            .prepare("PRAGMA table_info(youtube_videos)")
            .expect("prepare pragma")
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(1)?,
                    row.get::<_, i64>(3)?,
                    row.get::<_, Option<String>>(4)?,
                ))
            })
            .expect("query pragma")
            .collect::<Result<Vec<_>, _>>()
            .expect("collect pragma")
            .into_iter()
            .find(|(name, _, _)| name == "learning_status")
            .expect("learning_status column exists");

        assert_eq!(learning_status.1, 1);
        assert_eq!(learning_status.2.as_deref(), Some("'queued'"));
    }
}
