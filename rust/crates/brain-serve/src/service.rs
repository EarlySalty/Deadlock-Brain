//! Production composition: one hard-bounded LocalPgReader pool is shared by startup,
//! readiness, retrieval/evidence validation and conversation ownership.
use crate::{
    analytics::{AnalyticsRetriever, AnalyticsRuntime},
    config::{ProviderKind, RetrievalKind},
    discord_live::{DiscordLive, DiscordRetriever},
    health::{self, Health},
    log_event, Config, Error, Secrets,
};
use axum::{
    extract::{Request, State},
    middleware::{self, Next},
    response::Response,
};
use brain_contracts::{
    AnswerProviderPort, AuthorizedContext, Evidence, PortError, ProviderAnswer, Query,
};
use brain_kernel::{CachedKernel, Kernel};
use brain_policy::{CredentialRegistry, PolicyEngine};
use brain_providers::{
    CodexSubscriptionProvider, OpenAiCompatibleProvider, PriceCeiling, ProviderConfig,
};
use brain_storage::{LocalPgPoolStats, LocalPgReader};
use dbrain_retrieval::ReleaseRetriever;
use std::{
    future::IntoFuture,
    io::Read,
    path::Path,
    sync::{atomic::Ordering, Arc, Mutex},
    time::{Duration, Instant},
};

fn entity_profile_effective_uid() -> Result<u32, PortError> {
    let mut status = String::new();
    std::fs::File::open("/proc/self/status")
        .and_then(|file| file.take(16 * 1024).read_to_string(&mut status))
        .map_err(|_| {
            PortError::PermissionDenied("Lokale Operatoridentität kann nicht geprüft werden".into())
        })?;
    status
        .lines()
        .find_map(|line| line.strip_prefix("Uid:"))
        .and_then(|uids| uids.split_ascii_whitespace().nth(1))
        .and_then(|uid| uid.parse().ok())
        .ok_or_else(|| PortError::PermissionDenied("Tatsächliche effektive UID fehlt".into()))
}

fn entity_profile_reader(
    reader: LocalPgReader,
    config_path: Option<&Path>,
) -> Result<LocalPgReader, Error> {
    let Some(config_path) = config_path else {
        return Ok(reader);
    };
    let uid = entity_profile_effective_uid().map_err(|_| Error::ReaderConfig)?;
    LocalPgReader::entity_profile_operator(config_path, uid).map_err(|_| Error::ReaderConfig)?;
    let operator_config = config_path.to_owned();
    let blob_config = operator_config.clone();
    Ok(reader.with_entity_profile_access(
        move || {
            LocalPgReader::entity_profile_operator(
                &operator_config,
                entity_profile_effective_uid()?,
            )
        },
        move |record| {
            let (repository_path, commit, registered_origin, repository_url, path) =
                LocalPgReader::entity_evidence_repository(
                    &blob_config,
                    entity_profile_effective_uid()?,
                    record,
                )?;
            let pinned =
                dbrain_sources::git_source::PinnedRepository::open(&repository_path, &commit)
                    .map_err(|_| {
                        PortError::InvalidResponse(
                            "Registriertes Originalrepository kann nicht gelesen werden".into(),
                        )
                    })?;
            pinned.require_origin(&[&registered_origin]).map_err(|_| {
                PortError::InvalidResponse(
                    "Originalrepository widerspricht seiner Registrierung".into(),
                )
            })?;
            let bytes = pinned.read_blob(&path).map_err(|_| {
                PortError::InvalidResponse(
                    "Gepinnter Originalblob kann nicht gelesen werden".into(),
                )
            })?;
            Ok(brain_storage::entity_profile::derivation::GitBlobEvidence {
                source_id: record.descriptor.head.source_id.clone(),
                logical_id: record.descriptor.head.logical_id.clone(),
                store_revision: record.descriptor.head.revision,
                git_commit: commit,
                repository_url,
                bytes,
            })
        },
    ))
}

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
#[derive(Clone)]
enum AnswerProvider {
    OpenAi(OpenAiCompatibleProvider),
    Subscription(CodexSubscriptionProvider),
}

impl AnswerProviderPort for AnswerProvider {
    fn answer(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
    ) -> Result<ProviderAnswer, PortError> {
        match self {
            Self::OpenAi(provider) => provider.answer(query, context, evidence),
            Self::Subscription(provider) => provider.answer(query, context, evidence),
        }
    }
}

pub struct Prepared {
    pub config: Config,
    reader: LocalPgReader,
    provider: AnswerProvider,
    credentials: CredentialRegistry,
    internal_credentials: CredentialRegistry,
    analytics: Option<Arc<AnalyticsRuntime>>,
    discord_live: Option<Arc<DiscordLive>>,
    shutdown: Arc<Shutdown>,
    startup_deadline: Instant,
}

impl Prepared {
    pub fn new(config: Config, secrets: Secrets) -> Result<Self, Error> {
        config.validate()?;
        let deadline = Instant::now() + Duration::from_millis(config.timeouts.startup_ms);
        Self::new_until(config, secrets, deadline)
    }

    pub fn new_until(config: Config, secrets: Secrets, deadline: Instant) -> Result<Self, Error> {
        config.validate()?;
        if Instant::now() >= deadline {
            return Err(Error::StartupTimeout);
        }
        // Ambient PG options would bypass the explicit timeout contract.
        if std::env::var_os("PGOPTIONS").is_some_and(|value| !value.is_empty()) {
            return Err(Error::ConfigInvalid("ambient_postgres_options"));
        }
        let Secrets {
            provider_key,
            postgres_password,
            credentials,
            internal_credentials,
            discord_live_token,
        } = secrets;
        let discord_live = discord_live_token
            .map(DiscordLive::new)
            .transpose()?
            .map(Arc::new);
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
        let reader =
            entity_profile_reader(reader, config.entity_profile_maintenance_config.as_deref())?;
        let reader = reader
            .with_entity_profile_model_consumers(
                config
                    .credentials
                    .iter()
                    .filter(|credential| credential.entity_profile_model_context)
                    .map(|credential| (credential.actor_id.clone(), credential.channel.clone()))
                    .collect(),
            )
            .map_err(|_| Error::ReaderConfig)?;
        let mut provider_config = match config.provider.kind {
            ProviderKind::CodexSubscription => {
                ProviderConfig::codex_subscription(&config.provider.base_url)
            }
            ProviderKind::OpenaiCompatible => ProviderConfig::new(
                provider_key,
                &config.provider.base_url,
                &config.provider.model,
            ),
        };
        provider_config.timeout = Duration::from_millis(t.provider_ms);
        provider_config.retry_attempts = config.provider.retry_attempts;
        provider_config.retry_backoff = Duration::from_millis(config.provider.retry_backoff_ms);
        provider_config.max_response_bytes = config.provider.max_response_bytes;
        provider_config.pricing = config.provider.pricing.map(|price| PriceCeiling {
            input_micros_per_token: price.input_micros_per_token,
            output_micros_per_token: price.output_micros_per_token,
        });
        let provider = match config.provider.kind {
            ProviderKind::OpenaiCompatible => {
                OpenAiCompatibleProvider::new(provider_config).map(AnswerProvider::OpenAi)
            }
            ProviderKind::CodexSubscription => {
                CodexSubscriptionProvider::new(provider_config).map(AnswerProvider::Subscription)
            }
        }
        .map_err(|_| Error::ProviderConfig)?;
        let analytics = config
            .analytics
            .clone()
            .map(|analytics| AnalyticsRuntime::new(analytics, credentials.clone()).map(Arc::new))
            .transpose()?;
        let shutdown = Arc::new(Shutdown {
            deadline: Mutex::new(None),
            budget: Duration::from_millis(t.shutdown_ms),
        });
        if Instant::now() >= deadline {
            return Err(Error::StartupTimeout);
        }
        Ok(Self {
            config,
            reader,
            provider,
            credentials,
            internal_credentials,
            analytics,
            discord_live,
            shutdown,
            startup_deadline: deadline,
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
    let client_releases: Vec<_> = prepared
        .config
        .credentials
        .iter()
        .filter_map(|credential| {
            credential
                .release
                .clone()
                .map(|release| (release, prepared.config.bound_scope(credential)))
        })
        .collect();
    let analytics_patch = prepared
        .config
        .analytics
        .as_ref()
        .map(|value| value.patch.clone());
    let snapshot = tokio::task::spawn_blocking(move || {
        reader.check_core_schema().map_err(|error| match error {
            PortError::InvalidResponse(_) => Error::SchemaIncompatible,
            PortError::PermissionDenied(_) => Error::DatabasePermissions,
            _ => Error::DatabaseUnavailable,
        })?;
        reader.check_permissions().map_err(startup_database_error)?;
        let snapshot = reader
            .read_manifest(&release_id)
            .map_err(|error| match error {
                PortError::InvalidResponse(_) => Error::ReleaseUnavailable,
                other => startup_database_error(other),
            })?;
        if snapshot.release.knowledge_version != knowledge_version {
            return Err(Error::KnowledgeVersion);
        }
        if analytics_patch
            .as_deref()
            .is_some_and(|patch| patch != snapshot.release.patch)
        {
            return Err(Error::ConfigInvalid("analytics_patch"));
        }
        health::validate_manifest(&snapshot)?;
        let mut releases = vec![snapshot.release];
        for (expected, scope) in client_releases {
            let client_snapshot =
                reader
                    .read_manifest(&expected.id)
                    .map_err(|error| match error {
                        PortError::InvalidResponse(_) => Error::ReleaseUnavailable,
                        other => startup_database_error(other),
                    })?;
            if client_snapshot.release.knowledge_version != expected.knowledge_version {
                return Err(Error::KnowledgeVersion);
            }
            health::validate_manifest(&client_snapshot)?;
            if let Some(scope) = scope {
                health::validate_bound_manifest(&client_snapshot, &scope)?;
            }
            if !releases.contains(&client_snapshot.release) {
                releases.push(client_snapshot.release);
            }
        }
        Ok(releases)
    })
    .await
    .map_err(|_| Error::ReaderUnavailable)??;
    Ok(Arc::new(Health::new(
        snapshot,
        Duration::from_millis(prepared.config.timeouts.readiness_ms),
        prepared.reader.clone(),
        prepared
            .config
            .credentials
            .iter()
            .filter_map(|credential| {
                credential.release.as_ref().and_then(|release| {
                    prepared
                        .config
                        .bound_scope(credential)
                        .map(|scope| (release.id.clone(), scope))
                })
            })
            .collect(),
        prepared
            .config
            .internal_operator
            .as_ref()
            .map(|operator| operator.socket.clone()),
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
    if Instant::now() >= prepared.startup_deadline {
        return Err(Error::StartupTimeout);
    }
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
        result = tokio::time::timeout(prepared.startup_deadline.saturating_duration_since(Instant::now()), initialize(prepared)) => {
            result.map_err(|_| Error::StartupTimeout)??
        }
    };
    let retrieval = match prepared.config.retrieval.kind {
        RetrievalKind::ReleaseLexical => {
            ReleaseRetriever::new(prepared.reader.clone(), prepared.config.retrieval.limit)
        }
    };
    let retrieval = AnalyticsRetriever::new(retrieval, prepared.analytics.clone());
    let retrieval = DiscordRetriever::new(retrieval, prepared.discord_live.clone())
        .with_provider(&prepared.config.provider);
    let public_retrieval = DiscordRetriever::new(
        ReleaseRetriever::new(prepared.reader.clone(), prepared.config.retrieval.limit),
        prepared.discord_live.clone(),
    )
    .with_provider(&prepared.config.provider);
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
    )
    .with_retrieval(public_retrieval)
    .with_discord_consumers(
        prepared
            .config
            .trusted_discord_consumers
            .iter()
            .map(|consumer| (consumer.actor_id.clone(), consumer.channel.clone()))
            .collect(),
    )
    .with_release_bindings(
        prepared
            .config
            .credentials
            .iter()
            .map(|credential| {
                (
                    (credential.actor_id.clone(), credential.channel.clone()),
                    credential
                        .release
                        .as_ref()
                        .unwrap_or(&prepared.config.release)
                        .id
                        .clone(),
                )
            })
            .collect(),
    );
    let mut router = brain_api::router(api)
        .layer(middleware::from_fn_with_state(
            health.clone(),
            reject_during_drain,
        ))
        .merge(health::router(health.clone()));
    if let Some(analytics) = &prepared.analytics {
        router = router.merge(
            analytics
                .clone()
                .router()
                .layer(middleware::from_fn_with_state(
                    health.clone(),
                    reject_during_drain,
                )),
        );
    }
    if Instant::now() >= prepared.startup_deadline {
        return Err(Error::StartupTimeout);
    }
    let listener = tokio::time::timeout(
        prepared
            .startup_deadline
            .saturating_duration_since(Instant::now()),
        tokio::net::TcpListener::bind(prepared.config.bind),
    )
    .await
    .map_err(|_| Error::StartupTimeout)?
    .map_err(|_| Error::Bind)?;
    if Instant::now() >= prepared.startup_deadline {
        return Err(Error::StartupTimeout);
    }
    let address = listener.local_addr().map_err(|_| Error::Bind)?;
    let (operator_stop, mut operator_notice) = tokio::sync::watch::channel(false);
    let mut socket_guard = None;
    let operator = if let Some(config) = &prepared.config.internal_operator {
        let (listener, guard) = crate::operator_socket::bind(&config.socket)?;
        socket_guard = Some(guard);
        let service = brain_api::internal::InternalApiService::new(
            PolicyEngine::with_ownership_store(
                prepared.internal_credentials.clone(),
                Arc::new(prepared.reader.clone()),
            ),
            ReleaseRetriever::new(prepared.reader.clone(), prepared.config.retrieval.limit),
            config.release.id.clone(),
            prepared.config.timeouts.request_ms,
            (&prepared.config.budgets).into(),
        );
        let service = match (
            prepared.config.operator_docs_release(),
            prepared.config.operator_docs_scope(),
        ) {
            (Some(release), Some("bot.public")) => {
                service.with_public_bot_release(release.id.clone())
            }
            (Some(release), _) => service.with_public_docs_release(release.id.clone()),
            (None, _) => service,
        };
        let router = brain_api::internal::router(service).layer(middleware::from_fn_with_state(
            health.clone(),
            reject_during_drain,
        ));
        Some((listener, router))
    } else {
        None
    };
    let operator_enabled = operator.is_some();
    let mut operator_task = tokio::spawn(async move {
        if let Some((listener, router)) = operator {
            axum::serve(listener, router)
                .with_graceful_shutdown(async move {
                    let _ = operator_notice.changed().await;
                })
                .await
        } else {
            std::future::pending::<std::io::Result<()>>().await
        }
    });
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
        _ = &mut operator_task => Err(Error::Serve),
        _ = notice => {
            let _ = operator_stop.send(true);
            tokio::time::timeout(prepared.remaining_shutdown(), &mut server).await
                .map_err(|_| Error::ShutdownTimeout).and_then(|result| result.map_err(|_| Error::Serve))
        }
    };
    health.draining.store(true, Ordering::SeqCst);
    prepared.shutdown.begin();
    let _ = operator_stop.send(true);
    let operator_result = if operator_enabled && !operator_task.is_finished() {
        match tokio::time::timeout(prepared.remaining_shutdown(), &mut operator_task).await {
            Ok(Ok(Ok(()))) => Ok(()),
            Ok(_) => Err(Error::Serve),
            Err(_) => {
                operator_task.abort();
                let _ = operator_task.await;
                Err(Error::ShutdownTimeout)
            }
        }
    } else {
        operator_task.abort();
        Ok(())
    };
    drop(socket_guard);
    log_pool_stats(prepared.reader.pool_stats());
    result.and(operator_result)
}

#[cfg(test)]
mod entity_profile_tests {
    use super::*;

    #[tokio::test]
    async fn effective_uid_matches_kernel_peer_credentials() {
        let (stream, _peer) = tokio::net::UnixStream::pair().unwrap();
        assert_eq!(
            entity_profile_effective_uid().unwrap(),
            stream.peer_cred().unwrap().uid()
        );
    }

    #[test]
    fn configured_reader_requires_existing_operator_config_without_new_pool() {
        let dir = tempfile::tempdir().unwrap();
        let reader = LocalPgReader::new(dir.path(), 5432, "brain_core_test", "postgres").unwrap();
        let maximum = reader.pool_stats().max_connections;
        let unchanged = entity_profile_reader(reader.clone(), None).unwrap();
        assert_eq!(unchanged.pool_stats().max_connections, maximum);
        let path = dir.path().join("maintenance.json");
        assert!(entity_profile_reader(reader.clone(), Some(&path)).is_err());
        std::fs::write(
            &path,
            br#"{"internal_doc_scopes":["internal_docs"],"game_sources":[]}"#,
        )
        .unwrap();
        let configured = entity_profile_reader(reader, Some(&path)).unwrap();
        assert_eq!(configured.pool_stats().max_connections, maximum);
        assert_eq!(configured.pool_stats().created_connections, 0);
    }

    #[test]
    fn model_consumer_binding_is_exact_and_does_not_grant_operator_egress() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("maintenance.json");
        std::fs::write(
            &path,
            br#"{"internal_doc_scopes":["internal_docs"],"game_sources":[]}"#,
        )
        .unwrap();
        let reader = LocalPgReader::new(dir.path(), 5432, "brain_core_test", "postgres").unwrap();
        let consumers = std::collections::BTreeSet::from([("dl-bot".into(), "discord".into())]);
        assert!(reader
            .clone()
            .with_entity_profile_model_consumers(consumers.clone())
            .is_err());
        let configured = entity_profile_reader(reader, Some(&path)).unwrap();
        let principal = brain_contracts::Principal {
            actor_id: "dl-bot".into(),
            channel: "discord".into(),
            scopes: std::collections::BTreeSet::from(["bot.public".into()]),
            provider_egress: std::collections::BTreeSet::from(["public".into()]),
        };
        assert!(!configured.permits_entity_profile_model_context(&principal));
        let configured = configured
            .with_entity_profile_model_consumers(consumers)
            .unwrap();
        assert!(configured
            .clone()
            .permits_entity_profile_model_context(&principal));
        for (actor, channel) in [
            ("docs-client", "docs"),
            ("dl-bot", "twitch"),
            ("second-brain", "internal"),
        ] {
            let mut other = principal.clone();
            other.actor_id = actor.into();
            other.channel = channel.into();
            assert!(!configured.permits_entity_profile_model_context(&other));
        }
        let mut widened = principal;
        widened.provider_egress.insert("internal".into());
        assert!(!configured.permits_entity_profile_model_context(&widened));
        assert_eq!(configured.pool_stats().created_connections, 0);
        assert!(configured
            .with_entity_profile_model_consumers(std::collections::BTreeSet::from([(
                "second-brain".into(),
                "internal".into()
            )]))
            .is_err());
    }
}
