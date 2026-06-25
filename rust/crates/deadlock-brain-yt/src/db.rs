use std::path::PathBuf;

use rusqlite::Connection;

pub fn repo_root() -> PathBuf {
    deadlock_brain_core::db::repo_root()
}

pub fn default_feed_config_path() -> PathBuf {
    repo_root().join("config/youtube_feeds.json")
}

pub fn open_db(path: Option<PathBuf>) -> anyhow::Result<Connection> {
    let conn = deadlock_brain_core::db::open_connection(path)?;
    Ok(conn)
}

pub fn now_epoch_seconds() -> i64 {
    deadlock_brain_core::db::now_epoch_seconds().unwrap_or(0)
}
