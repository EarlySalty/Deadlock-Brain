use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use rusqlite::Connection;

use crate::{config, schema, Result};

pub fn repo_root() -> PathBuf {
    config::repo_root()
}

pub fn default_db_path() -> PathBuf {
    config::default_db_path()
}

pub fn open_default_connection() -> Result<Connection> {
    open_connection(None)
}

pub fn open_connection(path: Option<PathBuf>) -> Result<Connection> {
    let db_path = path.unwrap_or_else(default_db_path);
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let conn = Connection::open(db_path)?;
    apply_pragmas(&conn)?;
    schema::ensure_schema(&conn)?;
    Ok(conn)
}

pub fn apply_pragmas(conn: &Connection) -> rusqlite::Result<()> {
    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    conn.busy_timeout(std::time::Duration::from_secs(15))?;
    Ok(())
}

pub fn now_epoch_seconds() -> Result<i64> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_connection_creates_regular_schema_without_vector_tables() {
        let temp = tempfile::tempdir().expect("tempdir");
        let path = temp.path().join("brain.sqlite3");
        let conn = open_connection(Some(path)).expect("open connection");

        for table in schema::REGULAR_TABLES {
            let exists: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?1",
                    [table],
                    |row| row.get(0),
                )
                .expect("table lookup");
            assert_eq!(exists, 1, "missing table {table}");
        }

        for table in schema::VECTOR_TABLES_OMITTED {
            let exists: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?1",
                    [table],
                    |row| row.get(0),
                )
                .expect("table lookup");
            assert_eq!(exists, 0, "vector table should be omitted: {table}");
        }
    }
}
