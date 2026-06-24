use std::path::PathBuf;

use rusqlite::Connection;

use crate::schema;

pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("rust crate has repository parent")
        .to_path_buf()
}

pub fn default_db_path() -> PathBuf {
    std::env::var_os("DEADLOCK_BRAIN_DB_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo_root().join("data/deadlock_brain.sqlite3"))
}

pub fn default_feed_config_path() -> PathBuf {
    repo_root().join("config/youtube_feeds.json")
}

pub fn open_db(path: Option<PathBuf>) -> anyhow::Result<Connection> {
    let db_path = path.unwrap_or_else(default_db_path);
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let conn = Connection::open(db_path)?;
    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    conn.busy_timeout(std::time::Duration::from_secs(15))?;
    schema::ensure_youtube_tables(&conn)?;
    Ok(conn)
}

pub fn now_epoch_seconds() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock before unix epoch")
        .as_secs() as i64
}
