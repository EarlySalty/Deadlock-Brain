use rusqlite::Connection;

pub const YOUTUBE_SCHEMA: &str = deadlock_brain_core::schema::YOUTUBE_SCHEMA;

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
                "youtube_transcript_claim_attempts",
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
