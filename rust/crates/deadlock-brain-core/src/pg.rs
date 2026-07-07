//! sqlx `PgPool`-Fundament fuer das zentrale Postgres-Brain.

use anyhow::{anyhow, Result};
use sqlx::postgres::{PgPool, PgPoolOptions};

/// Env-Variable mit dem zentralen Postgres-DSN. Der Wert wird nie geloggt oder
/// ausgegeben (Secret-Hygiene).
pub const DSN_ENV: &str = "DEADLOCK_CENTRAL_DSN";

/// Postgres-Schema mit den Brain-Tabellen.
pub const SCHEMA: &str = "brain";

/// Baut einen kleinen async Verbindungspool gegen die zentrale Postgres.
///
/// Das DSN kommt aus [`DSN_ENV`]. Weder das DSN noch die konkreten
/// Verbindungsdetails werden im Fehlerfall ausgegeben.
pub async fn pg_pool() -> Result<PgPool> {
    let dsn = std::env::var(DSN_ENV)
        .map_err(|_| anyhow!("{DSN_ENV} ist nicht gesetzt; DSN wird nicht ausgegeben."))?;
    PgPoolOptions::new()
        .max_connections(4)
        .connect(&dsn)
        .await
        .map_err(|_| {
            anyhow!("Verbindung zur zentralen Postgres fehlgeschlagen (DSN aus {DSN_ENV}).")
        })
}
