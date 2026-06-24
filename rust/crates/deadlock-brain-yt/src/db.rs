use std::path::PathBuf;

use rusqlite::Connection;

pub fn repo_root() -> PathBuf {
    deadlock_brain_core::db::repo_root()
}

pub fn default_db_path() -> PathBuf {
    deadlock_brain_core::db::default_db_path()
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
    deadlock_brain_core::db::apply_pragmas(&conn)?;
    deadlock_brain_core::schema::ensure_schema(&conn)?;
    Ok(conn)
}

pub fn now_epoch_seconds() -> i64 {
    deadlock_brain_core::db::now_epoch_seconds().unwrap_or(0)
}
