use crate::{
    config::Config,
    patch_store::{Counts, PatchStore},
};
use anyhow::{ensure, Context, Result};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, PgPool, Postgres, Row, Transaction,
};
use std::{str::FromStr, time::Duration};

pub const SOURCE_LOCATOR: &str = "postgres://patchnotes-feed/public.patchnotes";
pub const MAX_ENVELOPE_BYTES: usize = 1024 * 1024;
pub const MAX_POSTS: u64 = 10_000;

pub struct Post {
    pub id: i64,
    pub bytes: Option<Vec<u8>>,
}

pub struct Feed {
    tx: Transaction<'static, Postgres>,
    upper_id: i64,
    max_bytes: i64,
    completed_id: i64,
    pending_id: Option<i64>,
    read: u64,
    exhausted: bool,
}

pub async fn ingest(config: &Config) -> Result<Counts> {
    let store = PatchStore::start(config.pool().await?, config.lease_seconds).await?;
    let mut counts = Counts::default();
    let result = async {
        let source = store.during(source_pool(config)).await?;
        let result = scan(
            &store,
            &source,
            config.max_blob_bytes.min(MAX_ENVELOPE_BYTES),
            &mut counts,
        )
        .await;
        source.close().await;
        result
    }
    .await;
    store.finish(&counts, result.is_err()).await?;
    result?;
    ensure!(
        !counts.partial(),
        "Patchimport wurde mit ausgewiesenen Datenlücken abgeschlossen."
    );
    Ok(counts)
}

pub async fn scan(
    store: &PatchStore,
    source: &PgPool,
    max_bytes: usize,
    counts: &mut Counts,
) -> Result<()> {
    let mut feed = store.during(Feed::begin(source, max_bytes)).await?;
    while let Some(post) = store.during(feed.next_post()).await? {
        store.object(&post, counts).await?;
        feed.confirm(post.id)?;
    }
    store.during(feed.finish()).await?;
    counts.sources_completed += 1;
    Ok(())
}

pub async fn source_pool(config: &Config) -> Result<PgPool> {
    reject_ambient()?;
    let values = deadlock_brain_core::pg::infisical_environment(&config.infisical_config)
        .await
        .map_err(|_| {
            anyhow::anyhow!("Patchquelle konnte nicht aus dem Secret-Manager geladen werden.")
        })?;
    let dsn = values
        .iter()
        .find(|(name, _)| name == "PATCHNOTES_FEED_READONLY_DSN")
        .map(|(_, value)| value.as_str())
        .context("Lesender Zugang zur Patchquelle fehlt.")?;
    connect_source(dsn).await
}

pub async fn connect_source(dsn: &str) -> Result<PgPool> {
    let options = source_options(dsn)?;
    PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(15))
        .connect_with(options)
        .await
        .map_err(|_| anyhow::anyhow!("Lesende Patchquelle ist nicht erreichbar."))
}

fn reject_ambient() -> Result<()> {
    ensure!(
        !std::env::vars_os().any(|(name, _)| {
            let name = name.to_string_lossy();
            name.starts_with("PG")
                || matches!(
                    name.as_ref(),
                    "DATABASE_URL" | "DEADLOCK_CENTRAL_DSN" | "INFISICAL_TOKEN_FILE"
                )
        }),
        "Umgebungszugangsdaten sind für den Patchimport nicht zulässig."
    );
    Ok(())
}

fn source_options(dsn: &str) -> Result<PgConnectOptions> {
    reject_ambient()?;
    ensure!(
        dsn.len() <= 16 * 1024 && !dsn.chars().any(char::is_control),
        "Zugang zur Patchquelle ist ungültig."
    );
    let rest = dsn
        .strip_prefix("postgres://")
        .or_else(|| dsn.strip_prefix("postgresql://"))
        .context("Zugang zur Patchquelle ist ungültig.")?;
    let authority = rest
        .split(['/', '?', '#'])
        .next()
        .context("Zugang zur Patchquelle ist ungültig.")?;
    let login = authority
        .rsplit_once('@')
        .map(|(login, _)| login)
        .context("Patchquelle benötigt ausdrückliche Zugangsdaten.")?;
    ensure!(
        login.split_once(':').is_some_and(|(user, password)| {
            user == "patchnotes_feed_reader" && !password.is_empty()
        }) && !dsn.contains('#'),
        "Patchquelle benötigt den ausdrücklich lesenden Zugang."
    );
    if let Some((_, query)) = dsn.split_once('?') {
        for parameter in query.split('&') {
            let key = parameter.split_once('=').map(|(key, _)| key);
            ensure!(
                matches!(key, Some("host" | "port" | "sslmode" | "ssl-mode")),
                "Nicht zulässiger Verbindungsparameter der Patchquelle."
            );
        }
    }
    let options = PgConnectOptions::from_str(dsn)
        .map_err(|_| anyhow::anyhow!("Zugang zur Patchquelle ist ungültig."))?
        .application_name("brain-job:ingest-patchnotes")
        .options([
            ("default_transaction_read_only", "on"),
            ("statement_timeout", "30000"),
            ("TimeZone", "UTC"),
        ])
        .disable_statement_logging();
    ensure!(
        options.get_username() == "patchnotes_feed_reader",
        "Patchquelle benötigt den ausdrücklich lesenden Zugang."
    );
    Ok(options)
}

impl Feed {
    pub async fn begin(pool: &PgPool, max_bytes: usize) -> Result<Self> {
        ensure!(
            (1024..=MAX_ENVELOPE_BYTES).contains(&max_bytes),
            "Ungültige Größenbegrenzung der Patchquelle."
        );
        let mut tx = pool
            .begin()
            .await
            .map_err(|_| anyhow::anyhow!("Lesender Patchquellenlauf konnte nicht beginnen."))?;
        sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ, READ ONLY")
            .execute(&mut *tx)
            .await
            .map_err(|_| {
                anyhow::anyhow!(
                    "Patchquelle konnte nicht auf ausschließliches Lesen begrenzt werden."
                )
            })?;
        sqlx::query("SET LOCAL TIME ZONE 'UTC'")
            .execute(&mut *tx)
            .await
            .map_err(|_| {
                anyhow::anyhow!("Patchquellenzeit konnte nicht vereinheitlicht werden.")
            })?;
        let safe: bool = sqlx::query_scalar(
            "SELECT current_user='patchnotes_feed_reader' AND session_user='patchnotes_feed_reader'
            AND current_setting('transaction_read_only')='on'
            AND current_setting('TimeZone')='UTC'
            AND NOT EXISTS(SELECT 1 FROM pg_roles WHERE rolname=current_user
                AND (rolsuper OR rolcreatedb OR rolcreaterole OR rolreplication OR rolbypassrls))
            AND has_table_privilege(current_user,'patchnotes.changelog_posts','SELECT')
            AND NOT has_table_privilege(current_user,'patchnotes.changelog_posts','INSERT,UPDATE,DELETE,TRUNCATE,TRIGGER')
            AND NOT has_any_column_privilege(current_user,'patchnotes.changelog_posts','INSERT,UPDATE')
            AND NOT has_schema_privilege(current_user,'patchnotes','CREATE')
            AND NOT has_schema_privilege(current_user,'public','CREATE')
            AND NOT has_database_privilege(current_user,current_database(),'CREATE')",
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| anyhow::anyhow!("Leserechte der Patchquelle konnten nicht überprüft werden."))?;
        ensure!(
            safe,
            "Patchquelle besitzt nicht die vorgeschriebenen reinen Leserechte."
        );
        let (upper_id, valid): (i64, bool) = sqlx::query_as(
            "SELECT coalesce(max(id),0),coalesce(min(id)>0,true) FROM patchnotes.changelog_posts",
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| anyhow::anyhow!("Patchquellenumfang konnte nicht gelesen werden."))?;
        ensure!(valid, "Patchquelle enthält ungültige Beitragskennungen.");
        Ok(Self {
            tx,
            upper_id,
            max_bytes: i64::try_from(max_bytes)
                .context("Ungültige Größenbegrenzung der Patchquelle.")?,
            completed_id: 0,
            pending_id: None,
            read: 0,
            exhausted: false,
        })
    }

    pub async fn next_post(&mut self) -> Result<Option<Post>> {
        ensure!(
            self.pending_id.is_none(),
            "Vorheriger Patchbeitrag wurde noch nicht bestätigt."
        );
        let row = sqlx::query(
            "WITH page AS MATERIALIZED (
                SELECT id,title,url,posted_at,raw_content,
                    512::bigint+6*(octet_length(title)::bigint+octet_length(url)::bigint
                        +coalesce(octet_length(raw_content)::bigint,0)) AS bound
                FROM patchnotes.changelog_posts WHERE id>$1 AND id<=$2 ORDER BY id LIMIT $3
            ) SELECT id,CASE WHEN bound<=$4 THEN
                json_build_object('id',id,'title',title,'url',url,'posted_at',posted_at,'raw_content',raw_content)::text
                ELSE NULL END AS envelope FROM page",
        )
        .bind(self.completed_id)
        .bind(self.upper_id)
        .bind(1_i64)
        .bind(self.max_bytes)
        .fetch_optional(&mut *self.tx)
        .await
        .map_err(|_| anyhow::anyhow!("Patchbeitrag konnte nicht gelesen werden."))?;
        let Some(row) = row else {
            self.exhausted = true;
            return Ok(None);
        };
        ensure!(
            self.read < MAX_POSTS,
            "Patchquelle überschreitet die Beitragsbegrenzung."
        );
        let id: i64 = row.get("id");
        let envelope: Option<String> = row.get("envelope");
        ensure!(
            id > self.completed_id
                && envelope.as_ref().is_none_or(|value| value.len() <= usize::try_from(self.max_bytes).unwrap_or(0)),
            "Patchquellenbegrenzung wurde überschritten."
        );
        self.pending_id = Some(id);
        self.read += 1;
        Ok(Some(Post {
            id,
            bytes: envelope.map(String::into_bytes),
        }))
    }

    pub fn confirm(&mut self, id: i64) -> Result<()> {
        ensure!(
            self.pending_id == Some(id),
            "Patchbeitragsbestätigung passt nicht zum gelesenen Beitrag."
        );
        self.completed_id = id;
        self.pending_id = None;
        Ok(())
    }

    pub async fn finish(self) -> Result<()> {
        ensure!(
            self.pending_id.is_none() && self.exhausted,
            "Unvollständig bestätigte Patchquelle verhindert den Quellenabschluss."
        );
        self.tx.commit().await.map_err(|_| {
            anyhow::anyhow!("Lesender Patchquellenlauf konnte nicht abgeschlossen werden.")
        })
    }
}
