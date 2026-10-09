use brain_contracts::postgres::{DatabaseAuth, Postgres};
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use sqlx::{PgConnection, PgPool};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
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

#[derive(Clone, Debug, Serialize)]
struct ConnectionDiagnostic {
    stage: &'static str,
    category: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    sqlstate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_field: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    io_kind: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    os_code: Option<i32>,
}

fn safe_sqlstate(code: &str) -> Option<String> {
    (code.len() == 5
        && code
            .bytes()
            .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit()))
    .then(|| code.to_owned())
}

impl ConnectionDiagnostic {
    fn from_sqlx(stage: &'static str, error: &sqlx::Error) -> Self {
        let category = match error {
            sqlx::Error::Configuration(_) => "configuration",
            sqlx::Error::InvalidArgument(_) => "invalid_argument",
            sqlx::Error::Database(_) => "database",
            sqlx::Error::Io(_) => "io",
            sqlx::Error::Tls(_) => "tls",
            sqlx::Error::Protocol(_) => "protocol",
            sqlx::Error::RowNotFound => "row_not_found",
            sqlx::Error::ColumnDecode { .. } | sqlx::Error::Decode(_) => "decode",
            sqlx::Error::ColumnNotFound(_) | sqlx::Error::ColumnIndexOutOfBounds { .. } => "column",
            sqlx::Error::TypeNotFound { .. } => "type_not_found",
            sqlx::Error::Encode(_) => "encode",
            sqlx::Error::PoolTimedOut => "pool_timeout",
            sqlx::Error::PoolClosed => "pool_closed",
            sqlx::Error::WorkerCrashed => "worker_crashed",
            _ => "other",
        };
        let sqlstate = error
            .as_database_error()
            .and_then(|error| error.code())
            .and_then(|code| safe_sqlstate(&code));
        let (io_kind, os_code) = if let sqlx::Error::Io(error) = error {
            let kind = match error.kind() {
                std::io::ErrorKind::NotFound => "not_found",
                std::io::ErrorKind::PermissionDenied => "permission_denied",
                std::io::ErrorKind::ConnectionRefused => "connection_refused",
                std::io::ErrorKind::ConnectionReset => "connection_reset",
                std::io::ErrorKind::ConnectionAborted => "connection_aborted",
                std::io::ErrorKind::NotConnected => "not_connected",
                std::io::ErrorKind::TimedOut => "timed_out",
                std::io::ErrorKind::UnexpectedEof => "unexpected_eof",
                std::io::ErrorKind::Interrupted => "interrupted",
                _ => "other",
            };
            (Some(kind), error.raw_os_error())
        } else {
            (None, None)
        };
        Self {
            stage,
            category,
            sqlstate,
            identity_field: None,
            io_kind,
            os_code,
        }
    }

    fn identity_mismatch(field: &'static str) -> Self {
        Self {
            stage: "identity_validation",
            category: "identity_mismatch",
            sqlstate: None,
            identity_field: Some(field),
            io_kind: None,
            os_code: None,
        }
    }
}

fn connection_failure(
    error: &sqlx::Error,
    identity_error: Option<&ConnectionDiagnostic>,
) -> String {
    let diagnostic = serde_json::json!({
        "connect": ConnectionDiagnostic::from_sqlx("connect", error),
        "after_connect": identity_error,
    });
    format!("Dediziertes Brain-Ziel oder tatsächliche Datenbankidentität konnte nicht bestätigt werden: {diagnostic}")
}

type DatabaseIdentity = (String, String, String, Option<String>, String);

fn identity_mismatch_field(pg: &Postgres, identity: &DatabaseIdentity) -> Option<&'static str> {
    [
        (identity.0 == pg.username, "current_user"),
        (identity.1 == pg.username, "session_user"),
        (identity.2 == pg.database, "current_database"),
        (identity.3.is_none(), "inet_server_addr"),
        (identity.4.parse::<u16>() == Ok(pg.port), "port"),
    ]
    .into_iter()
    .find_map(|(matches, field)| (!matches).then_some(field))
}

async fn verify_identity(
    connection: &mut PgConnection,
    pg: &Postgres,
) -> Result<(), ConnectionDiagnostic> {
    let identity: DatabaseIdentity = sqlx::query_as(
        "SELECT current_user::text, session_user::text, current_database()::text, inet_server_addr()::text, current_setting('port')",
    ).fetch_one(connection).await
        .map_err(|error| ConnectionDiagnostic::from_sqlx("identity_query", &error))?;
    if let Some(field) = identity_mismatch_field(pg, &identity) {
        return Err(ConnectionDiagnostic::identity_mismatch(field));
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
    let identity_error = Arc::new(Mutex::new(None));
    let callback_error = Arc::clone(&identity_error);
    PgPoolOptions::new()
        .max_connections(pg.max_connections)
        .acquire_timeout(Duration::from_secs(30))
        .after_connect(move |connection, _| {
            let pg = pg.clone();
            let callback_error = Arc::clone(&callback_error);
            Box::pin(async move {
                if let Err(diagnostic) = verify_identity(connection, &pg).await {
                    let safe_error = serde_json::json!({"brain_import_diagnostic": &diagnostic});
                    *callback_error
                        .lock()
                        .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(diagnostic);
                    return Err(sqlx::Error::Protocol(safe_error.to_string()));
                }
                Ok(())
            })
        })
        .connect_with(options)
        .await
        .map_err(|error| {
            let identity_error = identity_error
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            connection_failure(&error, identity_error.as_ref())
        })
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

    #[test]
    fn sqlstate_diagnostics_accept_only_five_uppercase_ascii_alphanumeric_bytes() {
        for code in ["28000", "28P01", "42501", "3D000"] {
            assert_eq!(safe_sqlstate(code).as_deref(), Some(code));
        }
        for code in [
            "",
            "4250",
            "425010",
            "28p01",
            "28P0\n",
            "28PÖ1",
            "28P01 private-error-marker",
        ] {
            assert_eq!(safe_sqlstate(code), None);
        }
    }

    #[test]
    fn connection_diagnostics_never_include_raw_error_payloads() {
        let marker = "private-error-marker";
        for (category, error) in [
            ("protocol", sqlx::Error::Protocol(marker.into())),
            (
                "invalid_argument",
                sqlx::Error::InvalidArgument(marker.into()),
            ),
            (
                "configuration",
                sqlx::Error::Configuration(std::io::Error::other(marker).into()),
            ),
            (
                "tls",
                sqlx::Error::Tls(std::io::Error::other(marker).into()),
            ),
            (
                "decode",
                sqlx::Error::ColumnDecode {
                    index: marker.into(),
                    source: std::io::Error::other(marker).into(),
                },
            ),
            (
                "io",
                sqlx::Error::Io(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    marker,
                )),
            ),
        ] {
            let diagnostic = ConnectionDiagnostic::from_sqlx("identity_query", &error);
            assert_eq!(diagnostic.category, category);
            assert_eq!(diagnostic.sqlstate, None);
            assert!(!serde_json::to_string(&diagnostic).unwrap().contains(marker));
            assert!(!connection_failure(&error, Some(&diagnostic)).contains(marker));
        }
        let io = ConnectionDiagnostic::from_sqlx(
            "connect",
            &sqlx::Error::Io(std::io::Error::from_raw_os_error(13)),
        );
        assert_eq!(io.io_kind, Some("permission_denied"));
        assert_eq!(io.os_code, Some(13));
    }

    #[test]
    fn pool_timeout_preserves_the_safe_identity_failure() {
        let diagnostic = ConnectionDiagnostic::identity_mismatch("current_database");
        let error = connection_failure(&sqlx::Error::PoolTimedOut, Some(&diagnostic));
        assert!(error.contains("pool_timeout"));
        assert!(error.contains("identity_validation"));
        assert!(error.contains("current_database"));
        let query_error =
            ConnectionDiagnostic::from_sqlx("identity_query", &sqlx::Error::RowNotFound);
        let error = connection_failure(&sqlx::Error::PoolTimedOut, Some(&query_error));
        assert!(error.contains("identity_query"));
        assert!(error.contains("row_not_found"));
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
    fn connected_identity_checks_role_database_local_transport_and_port() {
        let pg = target();
        let identity = (
            "brain_ingest".into(),
            "brain_ingest".into(),
            "brain".into(),
            None,
            "5446".into(),
        );
        assert_eq!(identity_mismatch_field(&pg, &identity), None);
        for (index, field) in [
            "current_user",
            "session_user",
            "current_database",
            "inet_server_addr",
            "port",
        ]
        .into_iter()
        .enumerate()
        {
            let mut wrong = identity.clone();
            match index {
                0 => wrong.0 = "other".into(),
                1 => wrong.1 = "other".into(),
                2 => wrong.2 = "postgres".into(),
                3 => wrong.3 = Some("127.0.0.1".into()),
                _ => wrong.4 = "5432".into(),
            }
            assert_eq!(identity_mismatch_field(&pg, &wrong), Some(field));
        }
        let mut wrong = identity;
        wrong.4 = "invalid-port".into();
        assert_eq!(identity_mismatch_field(&pg, &wrong), Some("port"));
    }
}
