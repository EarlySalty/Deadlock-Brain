use crate::{InputObject, InputObjects};
use anyhow::{anyhow, ensure, Context};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::{postgres::PgConnectOptions, ConnectOptions, Connection, PgConnection, Row};
use std::{
    future::Future,
    io::Read,
    path::{Path, PathBuf},
    str::FromStr,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    thread::JoinHandle,
    time::Duration,
};

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    infisical_config: PathBuf,
    max_posts: u32,
    max_row_bytes: u32,
    timeout_seconds: u32,
}

impl Config {
    fn load(path: &Path) -> anyhow::Result<Self> {
        ensure!(
            absolute(path),
            "Absoluter Patchnotes-Konfigurationspfad fehlt"
        );
        ensure!(
            std::fs::metadata(path)
                .is_ok_and(|metadata| metadata.is_file() && metadata.len() <= 8192),
            "Patchnotes-Konfiguration muss eine begrenzte reguläre Datei sein"
        );
        let mut bytes = Vec::new();
        std::fs::File::open(path)
            .map_err(|_| anyhow!("Patchnotes-Konfiguration ist nicht lesbar"))?
            .take(8193)
            .read_to_end(&mut bytes)
            .map_err(|_| anyhow!("Patchnotes-Konfiguration ist nicht lesbar"))?;
        ensure!(bytes.len() <= 8192, "Patchnotes-Konfiguration ist zu groß");
        let config: Self = serde_json::from_slice(&bytes)
            .map_err(|_| anyhow!("Patchnotes-Konfiguration ist ungültig"))?;
        ensure!(
            absolute(&config.infisical_config)
                && (1..=5000).contains(&config.max_posts)
                && (crate::limits::RAW_BYTES as u32 + 1024..=8 * 1024 * 1024)
                    .contains(&config.max_row_bytes)
                && (1..=60).contains(&config.timeout_seconds),
            "Patchnotes-Konfiguration benötigt gültige Grenzen und einen absoluten Infisical-Pfad"
        );
        Ok(config)
    }

    fn timeout(&self) -> Duration {
        Duration::from_secs(u64::from(self.timeout_seconds))
    }
}

fn absolute(path: &Path) -> bool {
    path.is_absolute()
        && path
            .to_str()
            .is_some_and(|text| text.len() <= 4096 && !text.chars().any(char::is_control))
}

async fn options(config: &Config) -> anyhow::Result<PgConnectOptions> {
    ensure!(
        [
            "INFISICAL_TOKEN_FILE",
            "PGOPTIONS",
            "PGPASSWORD",
            "PGPASSFILE",
            "PGHOST",
            "PGHOSTADDR",
            "PGPORT",
            "PGUSER",
            "PGDATABASE",
            "PGSSLROOTCERT",
            "PGSSLCERT",
            "PGSSLKEY",
            "PGSSLMODE",
            "PGAPPNAME",
        ]
        .iter()
        .all(|name| std::env::var_os(name).is_none()),
        "Umgebungsbasierte Zugänge und Datenbankoptionen sind unzulässig"
    );
    ensure!(
        std::fs::metadata(&config.infisical_config)
            .is_ok_and(|metadata| metadata.is_file() && metadata.len() <= 8192),
        "Infisical-Konfiguration muss eine begrenzte reguläre Datei sein"
    );
    let values = deadlock_brain_core::pg::infisical_environment(&config.infisical_config)
        .await
        .map_err(|_| anyhow!("Patchnotes-Zugang aus Infisical konnte nicht geladen werden"))?;
    let dsn = values
        .iter()
        .find(|(name, _)| name == "PATCHNOTES_FEED_READONLY_DSN")
        .map(|(_, value)| value.as_str())
        .context("Patchnotes-Lesezugang fehlt in Infisical")?;
    parse_options(dsn)
}

fn parse_options(dsn: &str) -> anyhow::Result<PgConnectOptions> {
    let (base, query) = dsn.split_once('?').unwrap_or((dsn, ""));
    let credentials = base
        .split_once("://")
        .and_then(|(_, rest)| rest.split('/').next()?.rsplit_once('@'));
    ensure!(
        !dsn.is_empty()
            && dsn.len() <= 8192
            && !dsn.chars().any(char::is_control)
            && !dsn.contains('#')
            && (base.starts_with("postgres://") || base.starts_with("postgresql://"))
            && credentials.is_some_and(|(userinfo, _)| {
                userinfo
                    .split_once(':')
                    .is_some_and(|(user, password)| !user.is_empty() && !password.is_empty())
            })
            && query
                .split('&')
                .filter(|part| !part.is_empty())
                .all(|part| {
                    part.split_once('=').is_some_and(|(name, _)| {
                        matches!(name, "sslmode" | "ssl-mode" | "host" | "port" | "dbname")
                    })
                }),
        "Patchnotes-Lesezugang muss vollständig und ohne zusätzliche Optionen angegeben sein"
    );
    PgConnectOptions::from_str(dsn)
        .map(|options| {
            options
                .application_name("brain-job-patchnotes")
                .options([("default_transaction_read_only", "on")])
                .disable_statement_logging()
        })
        .map_err(|_| anyhow!("Patchnotes-Lesezugang ist ungültig"))
}

pub fn load(path: &Path, _fetch: bool) -> InputObjects {
    match Config::load(path) {
        Ok(config) => {
            let credential_config = config.clone();
            spawn(config, async move { options(&credential_config).await })
        }
        Err(error) => Box::new(std::iter::once(Err(error))),
    }
}

enum Message {
    Object(anyhow::Result<InputObject>),
    Finished,
}

struct Stream {
    receiver: Option<Receiver<Message>>,
    request: tokio::sync::watch::Sender<()>,
    cancel: tokio::sync::watch::Sender<bool>,
    worker: Option<JoinHandle<()>>,
}

impl Iterator for Stream {
    type Item = anyhow::Result<InputObject>;

    fn next(&mut self) -> Option<Self::Item> {
        let receiver = self.receiver.as_ref()?;
        let _ = self.request.send(());
        match receiver.recv() {
            Ok(Message::Object(object)) => Some(object),
            Ok(Message::Finished) => {
                self.receiver.take();
                None
            }
            Err(_) => {
                self.receiver.take();
                Some(Err(anyhow!("Patchnotes-Lesethread wurde unterbrochen")))
            }
        }
    }
}

impl Drop for Stream {
    fn drop(&mut self) {
        self.receiver.take();
        let _ = self.cancel.send(true);
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

fn spawn<F>(config: Config, options: F) -> InputObjects
where
    F: Future<Output = anyhow::Result<PgConnectOptions>> + Send + 'static,
{
    let (sender, receiver) = sync_channel(0);
    let (request, mut requested) = tokio::sync::watch::channel(());
    let (cancel, mut cancelled) = tokio::sync::watch::channel(false);
    let worker = std::thread::Builder::new()
        .name("patchnotes-source".into())
        .spawn(move || {
            let runtime = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(runtime) => runtime,
                Err(_) => {
                    let _ = sender.send(Message::Object(Err(anyhow!(
                        "Patchnotes-Laufzeit konnte nicht gestartet werden"
                    ))));
                    let _ = sender.send(Message::Finished);
                    return;
                }
            };
            runtime.block_on(async {
                tokio::select! {
                    biased;
                    _ = cancelled.changed() => {},
                    result = read(&config, options, &sender, &mut requested) => {
                        if let Err(error) = result {
                            let _ = sender.send(Message::Object(Err(error)));
                        }
                        let _ = sender.send(Message::Finished);
                    }
                }
            });
            runtime.shutdown_timeout(Duration::from_secs(1));
        });
    match worker {
        Ok(worker) => Box::new(Stream {
            receiver: Some(receiver),
            request,
            cancel,
            worker: Some(worker),
        }),
        Err(_) => Box::new(std::iter::once(Err(anyhow!(
            "Patchnotes-Lesethread konnte nicht gestartet werden"
        )))),
    }
}

async fn bounded<T, E>(
    duration: Duration,
    future: impl Future<Output = Result<T, E>>,
) -> anyhow::Result<T> {
    tokio::time::timeout(duration, future)
        .await
        .map_err(|_| anyhow!("Patchnotes-Lesezugriff hat das Zeitlimit überschritten"))?
        .map_err(|_| anyhow!("Patchnotes-Lesezugriff fehlgeschlagen"))
}

async fn read(
    config: &Config,
    options: impl Future<Output = anyhow::Result<PgConnectOptions>>,
    sender: &SyncSender<Message>,
    requested: &mut tokio::sync::watch::Receiver<()>,
) -> anyhow::Result<()> {
    let timeout = config.timeout();
    let statement_timeout = format!("{}ms", timeout.as_millis());
    let options = bounded(timeout, options)
        .await?
        .options([("statement_timeout", statement_timeout.as_str())]);
    let mut connection = bounded(timeout, PgConnection::connect_with(&options)).await?;
    let mut transaction = bounded(timeout, connection.begin()).await?;
    bounded(
        timeout,
        sqlx::query("SET TRANSACTION READ ONLY").execute(&mut *transaction),
    )
    .await?;
    let role = bounded(
        timeout,
        sqlx::query(
            "SELECT current_user = 'patchnotes_feed_reader'
                    AND has_table_privilege(current_user, 'patchnotes.changelog_posts', 'SELECT')
                    AND NOT has_table_privilege(current_user, 'patchnotes.changelog_posts',
                        'INSERT, UPDATE, DELETE, TRUNCATE, REFERENCES, TRIGGER')
                    AND NOT has_schema_privilege(current_user, 'patchnotes', 'CREATE')
                    AND NOT (rolsuper OR rolcreatedb OR rolcreaterole) AS valid
             FROM pg_roles WHERE rolname = current_user",
        )
        .fetch_one(&mut *transaction),
    )
    .await?;
    ensure!(
        role.try_get::<bool, _>("valid").unwrap_or(false),
        "Patchnotes-Quelle benötigt die ausschließlich lesende Feed-Rolle"
    );
    bounded(
        timeout,
        sqlx::query(
            "DECLARE brain_patch_source NO SCROLL CURSOR FOR
             WITH parameters AS (SELECT $1::bigint AS max_row_bytes, $2::bigint AS max_posts),
             sized AS (
                 SELECT id, title, url, posted_at, raw_content,
                        COALESCE(octet_length(title)::bigint, 0)
                        + COALESCE(octet_length(url)::bigint, 0)
                        + COALESCE(octet_length(raw_content)::bigint, 0) AS byte_size
                 FROM patchnotes.changelog_posts
                 ORDER BY id LIMIT (SELECT max_posts + 1 FROM parameters)
             ),
             checked AS (
                 SELECT *, byte_size <= (SELECT max_row_bytes FROM parameters) AS bounded
                 FROM sized
             )
             SELECT id, byte_size, bounded,
                    CASE WHEN bounded THEN title END AS title,
                    CASE WHEN bounded THEN url END AS url,
                    CASE WHEN isfinite(posted_at)
                              AND EXTRACT(YEAR FROM posted_at AT TIME ZONE 'UTC') BETWEEN 1 AND 9999
                         THEN to_char(posted_at AT TIME ZONE 'UTC',
                              'YYYY-MM-DD\"T\"HH24:MI:SS.US\"Z\"')
                         ELSE posted_at::text END AS posted_at,
                    CASE WHEN bounded THEN raw_content END AS raw_content
             FROM checked ORDER BY id",
        )
        .bind(i64::from(config.max_row_bytes))
        .bind(i64::from(config.max_posts))
        .execute(&mut *transaction),
    )
    .await?;
    let mut count = 0;
    let mut skipped = 0;
    'posts: loop {
        requested
            .changed()
            .await
            .map_err(|_| anyhow!("Patchnotes-Empfänger wurde beendet"))?;
        loop {
            let row = bounded(
                timeout,
                sqlx::query("FETCH FORWARD 1 FROM brain_patch_source")
                    .fetch_optional(&mut *transaction),
            )
            .await?;
            let Some(row) = row else { break 'posts };
            count += 1;
            ensure!(
                count <= config.max_posts,
                "Patchnotes-Anzahl überschreitet die konfigurierte Grenze"
            );
            let within_limit: bool = row
                .try_get("bounded")
                .map_err(|_| anyhow!("Patchnotes-Zeile enthält ungültige Felder"))?;
            if !within_limit {
                let post_id: i64 = row
                    .try_get("id")
                    .map_err(|_| anyhow!("Patchnotes-Zeile enthält ungültige Felder"))?;
                let byte_size: i64 = row
                    .try_get("byte_size")
                    .map_err(|_| anyhow!("Patchnotes-Zeile enthält ungültige Felder"))?;
                skipped += 1;
                eprintln!(
                    "{}",
                    serde_json::json!({
                        "event": "patchnotes_source_skip",
                        "source": super::SOURCE,
                        "post_id": post_id,
                        "reason": "transport_row_too_large",
                        "byte_size": byte_size,
                        "skipped_oversized_posts": skipped
                    })
                );
                continue;
            }
            let object =
                envelope(&row).map_err(|_| anyhow!("Patchnotes-Zeile enthält ungültige Felder"))?;
            drop(row);
            if sender.send(Message::Object(Ok(object))).is_err() {
                return Ok(());
            }
            break;
        }
    }
    bounded(timeout, transaction.commit()).await?;
    eprintln!(
        "{}",
        serde_json::json!({
            "event": "patchnotes_source_summary",
            "source": super::SOURCE,
            "scanned_posts": count,
            "emitted_posts": count - skipped,
            "skipped_oversized_posts": skipped
        })
    );
    Ok(())
}

#[derive(Serialize)]
struct Envelope {
    id: i64,
    title: Option<String>,
    url: Option<String>,
    posted_at: Option<String>,
    raw_content: Option<String>,
}

fn envelope(row: &sqlx::postgres::PgRow) -> anyhow::Result<InputObject> {
    let post = Envelope {
        id: row.try_get("id")?,
        title: row.try_get("title")?,
        url: row.try_get("url")?,
        posted_at: row.try_get("posted_at")?,
        raw_content: row.try_get("raw_content")?,
    };
    let mut bytes = serde_json::to_vec(&post)?;
    bytes.shrink_to_fit();
    Ok(InputObject {
        revision: format!("sha256:{:x}", Sha256::digest(&bytes)),
        path: post.id.to_string(),
        bytes,
    })
}

#[cfg(test)]
#[path = "../tests/support/patch_source_pg.rs"]
mod tests;
