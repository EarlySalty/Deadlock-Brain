//! The only production composition: PgStore + LocalPgReader -> ReleaseRetriever ->
//! domain-aware Kernel/CachedKernel -> ApiService. No migration, legacy HTTP or bridge.
use crate::{
    config::{ProviderKind, RetrievalKind},
    health::{self, Health},
    log_event, Config, Error, Secrets,
};
use axum::{
    extract::{Request, State},
    middleware::{self, Next},
    response::Response,
};
use brain_contracts::SnapshotReadPort;
use brain_kernel::{CachedKernel, Kernel};
use brain_policy::{CredentialRegistry, PolicyEngine};
use brain_providers::{OpenAiCompatibleProvider, PriceCeiling, ProviderConfig};
use brain_storage::{LocalPgReader, PgStore};
use dbrain_retrieval::ReleaseRetriever;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    PgPool,
};
use std::{
    future::IntoFuture,
    sync::{atomic::Ordering, Arc, Mutex},
    time::{Duration, Instant},
};

struct Shutdown {
    deadline: Mutex<Option<Instant>>,
    budget: Duration,
}
impl Shutdown {
    fn begin(&self) -> Instant {
        let mut guard = self.deadline.lock().unwrap_or_else(|p| p.into_inner());
        *guard.get_or_insert_with(|| Instant::now() + self.budget)
    }
    fn remaining(&self) -> Duration {
        self.deadline
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .map(|deadline| deadline.saturating_duration_since(Instant::now()))
            .unwrap_or_default()
    }
}

/// Built before entering Tokio so reqwest's blocking client is created/dropped outside it.
/// The retained provider clone also outlives all API workers during shutdown.
pub struct Prepared {
    pub config: Config,
    reader: LocalPgReader,
    provider: OpenAiCompatibleProvider,
    credentials: CredentialRegistry,
    postgres_password: Option<String>,
    shutdown: Arc<Shutdown>,
}

impl Prepared {
    pub fn new(config: Config, secrets: Secrets) -> Result<Self, Error> {
        config.validate()?;
        // SQLx otherwise inherits arbitrary server startup options outside this config contract.
        if std::env::var_os("PGOPTIONS").is_some_and(|value| !value.is_empty()) {
            return Err(Error::ConfigInvalid("ambient_postgres_options"));
        }
        let Secrets {
            provider_key,
            postgres_password,
            credentials,
        } = secrets;
        let t = &config.timeouts;
        let pg = &config.postgres;
        let reader = LocalPgReader::new(&pg.socket_dir, pg.port, &pg.username, &pg.database)
            .and_then(|reader| {
                reader.with_connection_options(
                    postgres_password.clone(),
                    Duration::from_millis(t.postgres_connect_ms),
                    Duration::from_millis(t.postgres_statement_ms),
                    Duration::from_millis(t.postgres_lock_ms),
                )
            })
            .map_err(|_| Error::ReaderConfig)?;
        let mut provider_config = ProviderConfig::new(
            provider_key,
            &config.provider.base_url,
            &config.provider.model,
        );
        provider_config.timeout = Duration::from_millis(t.provider_ms);
        provider_config.retry_attempts = config.provider.retry_attempts;
        provider_config.retry_backoff = Duration::from_millis(config.provider.retry_backoff_ms);
        provider_config.max_response_bytes = config.provider.max_response_bytes;
        provider_config.pricing = config.provider.pricing.map(|price| PriceCeiling {
            input_micros_per_token: price.input_micros_per_token,
            output_micros_per_token: price.output_micros_per_token,
        });
        let provider = match config.provider.kind {
            ProviderKind::OpenaiCompatible => OpenAiCompatibleProvider::new(provider_config),
        }
        .map_err(|_| Error::ProviderConfig)?;
        let shutdown = Arc::new(Shutdown {
            deadline: Mutex::new(None),
            budget: Duration::from_millis(t.shutdown_ms),
        });
        Ok(Self {
            config,
            reader,
            provider,
            credentials,
            postgres_password,
            shutdown,
        })
    }

    /// A single SIGTERM/SIGINT budget covers HTTP drain, pool close and blocking workers.
    pub fn remaining_shutdown(&self) -> Duration {
        self.shutdown.remaining()
    }
}

async fn initialize(prepared: &Prepared) -> Result<(PgPool, Arc<Health>), Error> {
    let config = &prepared.config;
    let pg = &config.postgres;
    let t = &config.timeouts;
    // Explicit socket, account, database and password override ambient PG*/DSN defaults.
    // An empty password in peer mode intentionally does NOT read PGPASSWORD or .pgpass.
    let options = PgConnectOptions::new_without_pgpass()
        .host(pg.socket_dir.to_str().ok_or(Error::ReaderConfig)?)
        .port(pg.port)
        .username(&pg.username)
        .database(&pg.database)
        .password(prepared.postgres_password.as_deref().unwrap_or(""))
        .ssl_mode(PgSslMode::Disable)
        .application_name("brain-serve");
    let statement_ms = t.postgres_statement_ms;
    let lock_ms = t.postgres_lock_ms;
    let pool = PgPoolOptions::new()
        .max_connections(pg.max_connections)
        .acquire_timeout(Duration::from_millis(t.postgres_connect_ms))
        .after_connect(move |connection, _| Box::pin(async move {
            sqlx::query("SELECT set_config('statement_timeout', $1, false), set_config('lock_timeout', $2, false)")
                .bind(format!("{statement_ms}ms"))
                .bind(format!("{lock_ms}ms"))
                .execute(connection).await?;
            Ok(())
        }))
        .connect_with(options).await.map_err(|_| Error::DatabaseUnavailable)?;
    // Deployment/migrations and release publication are an explicit operator responsibility.
    let store = PgStore::new(pool.clone());
    store
        .check_core_schema()
        .await
        .map_err(|_| Error::SchemaIncompatible)?;
    let snapshot = store
        .snapshot(&config.release.id)
        .await
        .map_err(|_| Error::ReleaseUnavailable)?;
    if snapshot.release.knowledge_version != config.release.knowledge_version {
        return Err(Error::KnowledgeVersion);
    }
    health::validate_snapshot(&snapshot)?;
    health::permissions(&pool).await?;
    let expected = snapshot.release.clone();
    let reader = prepared.reader.clone();
    tokio::task::spawn_blocking(move || {
        let actual = reader
            .read_snapshot(&expected.release_id)
            .map_err(|_| Error::ReaderUnavailable)?;
        health::validate_snapshot(&actual)?;
        if actual.release != expected {
            return Err(Error::ReaderUnavailable);
        }
        Ok(())
    })
    .await
    .map_err(|_| Error::ReaderUnavailable)??;
    let health = Arc::new(Health::new(
        store,
        pool.clone(),
        snapshot.release,
        Duration::from_millis(t.readiness_ms),
        prepared.reader.clone(),
    ));
    Ok((pool, health))
}

async fn reject_during_drain(
    State(health): State<Arc<Health>>,
    request: Request,
    next: Next,
) -> Response {
    if health.draining.load(Ordering::SeqCst) {
        health::unavailable()
    } else {
        next.run(request).await
    }
}

pub async fn run(prepared: &Prepared) -> Result<(), Error> {
    use tokio::signal::unix::{signal, SignalKind};
    let mut term = signal(SignalKind::terminate()).map_err(|_| Error::Signal)?;
    let mut interrupt = signal(SignalKind::interrupt()).map_err(|_| Error::Signal)?;
    let shutdown_state = prepared.shutdown.clone();
    let mut shutdown = Box::pin(async move {
        tokio::select! { _ = term.recv() => {}, _ = interrupt.recv() => {} }
        shutdown_state.begin();
        log_event("shutdown_started");
    });
    // Signal handlers are installed before any database work, and nothing is bound yet.
    log_event("startup_checks");
    let (pool, health) = tokio::select! {
        biased;
        _ = &mut shutdown => return Ok(()),
        result = tokio::time::timeout(Duration::from_millis(prepared.config.timeouts.startup_ms), initialize(prepared)) => {
            result.map_err(|_| Error::StartupTimeout)??
        }
    };
    let retrieval = match prepared.config.retrieval.kind {
        RetrievalKind::ReleaseLexical => {
            ReleaseRetriever::new(prepared.reader.clone(), prepared.config.retrieval.limit)
        }
    };
    // Kernel's existing typed Fact/Rule/Prose evidence semantics are retained. C6's solver/hero
    // wiring is not fabricated here. Persisted ownership prevents cross-restart conversation theft.
    let kernel = CachedKernel::new(
        Kernel::new(retrieval, prepared.provider.clone()),
        prepared.config.kernel.cache_entries,
        Duration::from_millis(prepared.config.kernel.cache_ttl_ms),
    );
    let policy = PolicyEngine::with_ownership_store(
        prepared.credentials.clone(),
        Arc::new(prepared.reader.clone()),
    );
    let api = brain_api::ApiService::new(
        policy,
        kernel,
        &prepared.config.release.id,
        prepared.config.timeouts.request_ms,
        (&prepared.config.budgets).into(),
    );
    let router = brain_api::router(api)
        .layer(middleware::from_fn_with_state(
            health.clone(),
            reject_during_drain,
        ))
        .merge(health::router(health.clone()));
    let listener = tokio::net::TcpListener::bind(prepared.config.bind)
        .await
        .map_err(|_| Error::Bind)?;
    let address = listener.local_addr().map_err(|_| Error::Bind)?;
    eprintln!(
        "{}",
        serde_json::json!({"event": "listening", "address": address.to_string()})
    );
    let (started, notice) = tokio::sync::oneshot::channel();
    let drain_health = health.clone();
    let server = axum::serve(listener, router)
        .with_graceful_shutdown(async move {
            shutdown.await;
            drain_health.draining.store(true, Ordering::SeqCst);
            let _ = started.send(());
        })
        .into_future();
    tokio::pin!(server);
    let result = tokio::select! {
        result = &mut server => result.map_err(|_| Error::Serve),
        _ = notice => {
            tokio::time::timeout(prepared.remaining_shutdown(), &mut server).await
                .map_err(|_| Error::ShutdownTimeout)?.map_err(|_| Error::Serve)
        }
    };
    health.draining.store(true, Ordering::SeqCst);
    prepared.shutdown.begin();
    tokio::time::timeout(prepared.remaining_shutdown(), pool.close())
        .await
        .map_err(|_| Error::ShutdownTimeout)?;
    result
}
