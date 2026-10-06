use brain_contracts::postgres::{DatabaseAuth, Postgres};
use serde::Deserialize;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use sqlx::{PgConnection, PgPool};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(Deserialize)]
struct ImportRuntime {
    postgres: Postgres,
    infisical_config: PathBuf,
}

impl ImportRuntime {
    fn load(path: &Path) -> Result<Self, String> {
        let file = File::open(path)
            .map_err(|_| "Normale Runtime-Konfiguration kann nicht geöffnet werden")?;
        if !file
            .metadata()
            .map_err(|_| "Runtime-Konfiguration kann nicht geprüft werden")?
            .is_file()
        {
            return Err("Reguläre Runtime-Konfigurationsdatei erforderlich".into());
        }
        let mut bytes = Vec::new();
        file.take(65_537)
            .read_to_end(&mut bytes)
            .map_err(|_| "Runtime-Konfiguration kann nicht gelesen werden")?;
        if bytes.len() > 65_536 {
            return Err("Runtime-Konfiguration überschreitet 64 KiB".into());
        }
        let runtime: Self = serde_json::from_slice(&bytes)
            .map_err(|_| "Normale Runtime-Konfiguration entspricht nicht dem Zielschema")?;
        runtime.validate()?;
        Ok(runtime)
    }

    fn validate(&self) -> Result<(), String> {
        let pg = &self.postgres;
        if !pg.socket_dir.is_absolute()
            || !pg
                .socket_dir
                .to_str()
                .is_some_and(|path| !path.chars().any(char::is_control))
            || pg.port == 0
            || pg.username != "brain_ingest"
            || pg.database != "brain"
            || !(1..=16).contains(&pg.max_connections)
            || pg.auth != DatabaseAuth::Password
            || !pg.password_env.as_deref().is_some_and(|name| {
                !name.is_empty()
                    && name.len() <= 128
                    && name.bytes().enumerate().all(|(index, byte)| {
                        byte == b'_'
                            || byte.is_ascii_uppercase()
                            || (index > 0 && byte.is_ascii_digit())
                    })
            })
            || !self.infisical_config.is_absolute()
        {
            return Err(
                "Runtime-Konfiguration benennt kein dediziertes internes Brain-Importziel".into(),
            );
        }
        Ok(())
    }
}

type DatabaseIdentity = (String, String, String, Option<String>, String, String);

fn identity_matches(pg: &Postgres, identity: &DatabaseIdentity) -> bool {
    identity.0 == pg.username
        && identity.1 == pg.username
        && identity.2 == pg.database
        && identity.3.is_none()
        && identity.4.parse::<u16>() == Ok(pg.port)
        && pg.socket_dir.to_str() == Some(identity.5.as_str())
}

async fn verify_identity(connection: &mut PgConnection, pg: &Postgres) -> Result<(), sqlx::Error> {
    let identity: DatabaseIdentity = sqlx::query_as(
        "SELECT current_user::text, session_user::text, current_database()::text, inet_server_addr()::text, current_setting('port'), current_setting('unix_socket_directories')",
    ).fetch_one(connection).await?;
    if !identity_matches(pg, &identity) {
        return Err(sqlx::Error::Protocol(
            "dedicated Brain target identity mismatch".into(),
        ));
    }
    Ok(())
}

pub(super) async fn connect(
    runtime_path: &Path,
    infisical_override: Option<&Path>,
) -> Result<PgPool, String> {
    let runtime = ImportRuntime::load(runtime_path)?;
    if infisical_override.is_some_and(|path| path != runtime.infisical_config) {
        return Err("Infisical-Pfad widerspricht der normalen Runtime-Konfiguration".into());
    }
    let environment = dbrain_sources::core::pg::infisical_environment(&runtime.infisical_config)
        .await
        .map_err(|_| "Vorhandener Infisical-Snapshot kann nicht geladen werden")?;
    let pg = runtime.postgres;
    let name = pg
        .password_env
        .as_deref()
        .ok_or("Infisical-Passwortverweis fehlt")?;
    let password = environment
        .iter()
        .find(|(key, _)| key == name)
        .map(|(_, value)| value.as_str())
        .filter(|value| !value.is_empty())
        .ok_or("Importzugang fehlt im vorhandenen Infisical-Snapshot")?;
    let options = PgConnectOptions::new_without_pgpass()
        .host(
            pg.socket_dir
                .to_str()
                .ok_or("Datenbank-Socket ist ungültig")?,
        )
        .port(pg.port)
        .username(&pg.username)
        .database(&pg.database)
        .password(password)
        .ssl_mode(PgSslMode::Disable)
        .application_name("brain-knowledge-import")
        .options([("statement_timeout", "120000"), ("lock_timeout", "10000")]);
    drop(environment);
    PgPoolOptions::new().max_connections(pg.max_connections)
        .acquire_timeout(Duration::from_secs(30))
        .after_connect(move |connection, _| {
            let pg = pg.clone();
            Box::pin(async move { verify_identity(connection, &pg).await })
        })
        .connect_with(options).await
        .map_err(|_| "Dediziertes Brain-Ziel oder tatsächliche Datenbankidentität konnte nicht bestätigt werden".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn target() -> Postgres {
        serde_json::from_value(json!({
            "socket_dir": "/run/deadlock-brain-postgresql", "port": 5446,
            "username": "brain_ingest", "database": "brain", "auth": "password",
            "password_env": "BRAIN_PG_INGEST_PASSWORD", "max_connections": 2,
        }))
        .unwrap()
    }

    #[test]
    fn runtime_reuses_exact_existing_target_and_rejects_fallbacks() {
        let mut runtime = ImportRuntime {
            postgres: target(),
            infisical_config: "/etc/deadlock-brain/infisical.json".into(),
        };
        assert!(runtime.validate().is_ok());
        runtime.postgres.database = "postgres".into();
        assert!(runtime.validate().is_err());
        runtime.postgres = target();
        runtime.postgres.auth = DatabaseAuth::Peer;
        assert!(runtime.validate().is_err());
        runtime.postgres = target();
        runtime.postgres.username = "brain_service".into();
        assert!(runtime.validate().is_err());
    }

    #[tokio::test]
    #[ignore = "requires isolated PostgreSQL Unix socket: BRAIN_CORE_TEST_PG_SOCKET"]
    async fn isolated_connection_identity_is_checked_before_import() {
        use sqlx::Connection;
        let socket =
            std::env::var("BRAIN_CORE_TEST_PG_SOCKET").expect("explicit scratch socket required");
        assert!(socket.ends_with("/.core-test-pg"));
        assert!(Path::new(&socket).is_absolute());
        let options = PgConnectOptions::new_without_pgpass()
            .host(&socket)
            .port(55439)
            .username("brain_core_test")
            .database("postgres")
            .password("")
            .ssl_mode(PgSslMode::Disable);
        let mut connection = PgConnection::connect_with(&options).await.unwrap();
        let mut pg = target();
        pg.socket_dir = socket.into();
        pg.port = 55439;
        pg.username = "brain_core_test".into();
        pg.database = "postgres".into();
        verify_identity(&mut connection, &pg).await.unwrap();
        pg.database = "brain".into();
        assert!(verify_identity(&mut connection, &pg).await.is_err());
        pg.database = "postgres".into();
        pg.port = 5446;
        assert!(verify_identity(&mut connection, &pg).await.is_err());
        connection.close().await.unwrap();
    }

    #[test]
    fn connected_identity_checks_role_database_local_transport_port_and_socket() {
        let pg = target();
        let identity = (
            "brain_ingest".into(),
            "brain_ingest".into(),
            "brain".into(),
            None,
            "5446".into(),
            "/run/deadlock-brain-postgresql".into(),
        );
        assert!(identity_matches(&pg, &identity));
        for index in 0..6 {
            let mut wrong = identity.clone();
            match index {
                0 => wrong.0 = "other".into(),
                1 => wrong.1 = "other".into(),
                2 => wrong.2 = "postgres".into(),
                3 => wrong.3 = Some("127.0.0.1".into()),
                4 => wrong.4 = "5432".into(),
                _ => wrong.5 = "/run/postgresql".into(),
            }
            assert!(!identity_matches(&pg, &wrong));
        }
    }
}
