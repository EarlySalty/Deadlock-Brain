//! Production composition: one hard-bounded LocalPgReader pool is shared by startup,
//! readiness, retrieval/evidence validation and conversation ownership.
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
use brain_contracts::{PortError, SnapshotReadPort};
use brain_kernel::{CachedKernel, Kernel};
use brain_policy::{CredentialRegistry, PolicyEngine};
use brain_providers::{OpenAiCompatibleProvider, PriceCeiling, ProviderConfig};
use brain_storage::{LocalPgPoolStats, LocalPgReader};
use dbrain_retrieval::ReleaseRetriever;
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
    shutdown: Arc<Shutdown>,
}

impl Prepared {
    pub fn new(config: Config, secrets: Secrets) -> Result<Self, Error> {
        config.validate()?;
        // Ambient PG options would bypass the explicit timeout contract.
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
                reader.with_pool_options(
                    postgres_password,
                    Duration::from_millis(t.postgres_connect_ms),
                    Duration::from_millis(t.postgres_statement_ms),
                    Duration::from_millis(t.postgres_lock_ms),
                    pg.max_connections,
                    Duration::from_millis(t.postgres_pool_wait_ms),
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
            shutdown,
        })
    }

    pub fn remaining_shutdown(&self) -> Duration {
        self.shutdown.remaining()
    }
}

fn startup_database_error(error: PortError) -> Error {
    match error {
        PortError::PermissionDenied(_) => Error::DatabasePermissions,
        _ => Error::DatabaseUnavailable,
    }
}

async fn initialize(prepared: &Prepared) -> Result<Arc<Health>, Error> {
    let reader = prepared.reader.clone();
    let release_id = prepared.config.release.id.clone();
    let knowledge_version = prepared.config.release.knowledge_version.clone();
    let snapshot = tokio::task::spawn_blocking(move || {
        reader.check_core_schema().map_err(|error| match error {
            PortError::InvalidResponse(_) => Error::SchemaIncompatible,
            PortError::PermissionDenied(_) => Error::DatabasePermissions,
            _ => Error::DatabaseUnavailable,
        })?;
        reader.check_permissions().map_err(startup_database_error)?;
        let snapshot = reader
            .read_snapshot(&release_id)
            .map_err(|error| match error {
                PortError::InvalidResponse(_) => Error::ReleaseUnavailable,
                other => startup_database_error(other),
            })?;
        if snapshot.release.knowledge_version != knowledge_version {
            return Err(Error::KnowledgeVersion);
        }
        health::validate_snapshot(&snapshot)?;
        Ok(snapshot)
    })
    .await
    .map_err(|_| Error::ReaderUnavailable)??;
    Ok(Arc::new(Health::new(
        snapshot.release,
        Duration::from_millis(prepared.config.timeouts.readiness_ms),
        prepared.reader.clone(),
    )))
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

fn log_pool_stats(stats: LocalPgPoolStats) {
    eprintln!(
        "{}",
        serde_json::json!({
            "event": "postgres_pool_stats",
            "max_connections": stats.max_connections,
            "open_connections": stats.open_connections,
            "connecting_connections": stats.connecting_connections,
            "idle_connections": stats.idle_connections,
            "checked_out_connections": stats.checked_out_connections,
            "peak_connections": stats.peak_connections,
            "created_connections": stats.created_connections,
            "reused_checkouts": stats.reused_checkouts,
            "wait_count": stats.wait_count,
            "wait_timeout_count": stats.wait_timeout_count,
            "wait_total_micros": stats.wait_total_micros,
            "wait_max_micros": stats.wait_max_micros,
        })
    );
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
    log_event("startup_checks");
    let health = tokio::select! {
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
    log_pool_stats(prepared.reader.pool_stats());
    result
}
