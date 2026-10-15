#[path = "answer_trace.rs"]
mod answer_trace;
#[path = "pg_tools.rs"]
mod pg_tools;

use crate::{
    analytics::AnalyticsRuntime,
    config::ProviderKind,
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
    Accounted, AnswerProviderPort, AuthorizedContext, Evidence, PortError, PortFailure,
    ProviderAnswer, ProviderTurn, Query, ToolConversation, ToolDefinition,
};
use brain_kernel::{CachedKernel, Kernel};
use brain_policy::{CredentialRegistry, PolicyEngine};
use brain_providers::{
    CodexSubscriptionProvider, GenerationEndpoint, OpenAiCompatibleProvider, PriceCeiling,
    ProviderConfig, TextGenerationProvider, TwitchTitleGenerator,
};
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

#[derive(Clone)]
enum AnswerProvider {
    OpenAi(OpenAiCompatibleProvider),
    Subscription(CodexSubscriptionProvider),
}

impl AnswerProviderPort for AnswerProvider {
    fn answer_accounted(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
    ) -> Result<Accounted<ProviderAnswer>, PortFailure> {
        match self {
            Self::OpenAi(provider) => provider.answer_accounted(query, context, evidence),
            Self::Subscription(provider) => provider.answer_accounted(query, context, evidence),
        }
    }

    fn answer_turn_accounted(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
        tools: &[ToolDefinition],
        conversation: &ToolConversation,
    ) -> Result<Accounted<ProviderTurn>, PortFailure> {
        match self {
            Self::OpenAi(provider) => {
                provider.answer_turn_accounted(query, context, evidence, tools, conversation)
            }
            Self::Subscription(provider) => {
                provider.answer_turn_accounted(query, context, evidence, tools, conversation)
            }
        }
    }

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

    fn answer_turn(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
        tools: &[ToolDefinition],
        conversation: &ToolConversation,
    ) -> Result<ProviderTurn, PortError> {
        match self {
            Self::OpenAi(provider) => {
                provider.answer_turn(query, context, evidence, tools, conversation)
            }
            Self::Subscription(provider) => {
                provider.answer_turn(query, context, evidence, tools, conversation)
            }
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
    mirror_password: Option<String>,
    trace_options: Option<sqlx::postgres::PgConnectOptions>,
    twitch_title: Option<Arc<TwitchTitleGenerator>>,
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
        if std::env::var_os("PGOPTIONS").is_some_and(|value| !value.is_empty()) {
            return Err(Error::ConfigInvalid("ambient_postgres_options"));
        }
        let Secrets {
            provider_key,
            postgres_password,
            credentials,
            internal_credentials,
            discord_live_token,
            mirror_password,
            twitch_title_key,
        } = secrets;
        let twitch_title = twitch_title_generator(&config, twitch_title_key)?;
        let discord_live = discord_live_token
            .map(DiscordLive::new)
            .transpose()?
            .map(Arc::new);
        let t = &config.timeouts;
        let pg = &config.postgres;
        let trace_options = config.deadlock_api.as_ref().map(|_| {
            let options = sqlx::postgres::PgConnectOptions::new_without_pgpass()
                .host(&pg.socket_dir.to_string_lossy())
                .port(pg.port)
                .username(&pg.username)
                .database(&pg.database);
            match postgres_password.as_deref() {
                Some(password) => options.password(password),
                None => options,
            }
        });
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
        provider_config.quality_filters = true;
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
                OpenAiCompatibleProvider::new(provider_config).map(|provider| {
                    AnswerProvider::OpenAi(provider.with_response_audit(Arc::new(reader.clone())))
                })
            }
            ProviderKind::CodexSubscription => {
                CodexSubscriptionProvider::new(provider_config).map(|provider| {
                    AnswerProvider::Subscription(
                        provider.with_response_audit(Arc::new(reader.clone())),
                    )
                })
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
            mirror_password,
            trace_options,
            twitch_title,
            shutdown,
            startup_deadline: deadline,
        })
    }

    pub fn remaining_shutdown(&self) -> Duration {
        self.shutdown.remaining()
    }
}

fn twitch_title_generator(
    config: &Config,
    key: Option<String>,
) -> Result<Option<Arc<TwitchTitleGenerator>>, Error> {
    let Some(title) = config
        .task_providers
        .as_ref()
        .and_then(|providers| providers.twitch_title.as_ref())
    else {
        return Ok(None);
    };
    let Some(key) = key else {
        log_event("twitch_title_key_missing");
        return Ok(None);
    };
    let provider = TextGenerationProvider::new(GenerationEndpoint::new(
        key,
        &title.base_url,
        Duration::from_millis(title.timeout_ms),
        title.retry_attempts_on_429,
        title.max_response_bytes,
    ))
    .map_err(|_| Error::ProviderConfig)?;
    Ok(Some(Arc::new(TwitchTitleGenerator::new(
        provider,
        title.model_selection_path.clone(),
        title.max_output_tokens,
    ))))
}

fn startup_database_error(error: PortError) -> Error {
    match error {
        PortError::PermissionDenied(_) => Error::DatabasePermissions,
        _ => Error::DatabaseUnavailable,
    }
}

async fn initialize(prepared: &Prepared) -> Result<Arc<Health>, Error> {
    let reader = prepared.reader.clone();
    tokio::task::spawn_blocking(move || {
        reader
            .check_core_schema()
            .map_err(|_| Error::SchemaIncompatible)?;
        reader
            .check_tool_serving()
            .map_err(startup_database_error)?;
        reader.check_permissions().map_err(startup_database_error)?;
        reader
            .check_response_audit()
            .map_err(startup_database_error)
    })
    .await
    .map_err(|_| Error::ReaderUnavailable)??;
    Ok(Arc::new(Health::tools_only(
        Duration::from_millis(prepared.config.timeouts.readiness_ms),
        prepared.reader.clone(),
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
        if let Some(route) = match request.uri().path() {
            "/v1/answer" => Some("/v1/answer"),
            "/v1/retrieve" => Some("/v1/retrieve"),
            "/v1/operator/query" => Some("/v1/operator/query"),
            _ => None,
        } {
            eprintln!(
                "{}",
                serde_json::json!({
                    "event": "request_failure",
                    "route": route,
                    "request_id": null,
                    "error_class": "draining",
                    "http_status": 503,
                })
            );
        }
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
    let public_retrieval =
        DiscordRetriever::new(crate::tool_base::ToolBase, prepared.discord_live.clone())
            .with_provider(&prepared.config.provider);
    let mut answer_artifacts = None;
    let mut answer_kernel = Kernel::new(crate::tool_base::ToolBase, prepared.provider.clone())
        .with_tools(
            crate::tool_base::ToolBase,
            crate::tool_base::ToolBase,
            "brain.tools-only.v2:disabled",
        )
        .with_tools_only_limits(
            prepared.config.kernel.max_tool_calls,
            prepared.config.kernel.max_tool_result_bytes,
        );
    if let Some(config) = &prepared.config.deadlock_api {
        let pg = &prepared.config.postgres;
        let options = sqlx::postgres::PgConnectOptions::new_without_pgpass()
            .host(&pg.socket_dir.to_string_lossy())
            .port(pg.port)
            .username("brain_readonly")
            .database(&pg.database)
            .password(
                prepared
                    .mirror_password
                    .as_deref()
                    .ok_or(Error::SecretMissing("mirror"))?,
            )
            .options([
                ("default_transaction_read_only", "on".to_owned()),
                (
                    "statement_timeout",
                    config
                        .request_timeout_ms
                        .min(prepared.config.timeouts.request_ms)
                        .to_string(),
                ),
            ]);
        let gamefacts_pool = pg_tools::tool_pool(
            options.clone(),
            Duration::from_millis(prepared.config.timeouts.postgres_pool_wait_ms),
        );
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .acquire_timeout(Duration::from_millis(
                prepared.config.timeouts.postgres_connect_ms,
            ))
            .connect_lazy_with(options);
        answer_artifacts = Some(crate::answer_artifacts::CompareArtifacts::new(
            pool.clone(),
            tokio::runtime::Handle::current(),
        ));
        answer_kernel = answer_kernel
            .with_tools(
                answer_trace::TracedTools(pg_tools::PgTools::new(
                    crate::tool_base::ToolBase,
                    gamefacts_pool,
                    tokio::runtime::Handle::current(),
                )),
                crate::tool_base::ToolBase,
                format!(
                    "brain.tools-only.v2:{}:{}",
                    prepared.config.provider.base_url, prepared.config.provider.model
                ),
            )
            .with_tools_only_limits(
                prepared.config.kernel.max_tool_calls,
                prepared.config.kernel.max_tool_result_bytes,
            );
    }
    let kernel = CachedKernel::new(
        answer_kernel,
        prepared.config.kernel.cache_entries,
        Duration::from_millis(prepared.config.kernel.cache_ttl_ms),
    );
    let kernel = answer_trace::TracedKernel::new(
        kernel,
        prepared.trace_options.clone().map(|options| {
            Arc::new(answer_trace::PgTraceWriter::new(
                options,
                tokio::runtime::Handle::current(),
                Duration::from_millis(prepared.config.timeouts.postgres_pool_wait_ms),
            )) as Arc<dyn answer_trace::TraceSink>
        }),
    );
    let policy = PolicyEngine::with_ownership_store(
        prepared.credentials.clone(),
        Arc::new(prepared.reader.clone()),
    );
    let api = brain_api::ApiService::new(
        policy,
        kernel,
        "postgres-tools-v2",
        prepared.config.timeouts.request_ms,
        (&prepared.config.budgets).into(),
    )
    .with_retrieval(public_retrieval)
    .with_discord_context_resolver(crate::tool_base::ToolBase)
    .with_discord_consumers(
        prepared
            .config
            .trusted_discord_consumers
            .iter()
            .map(|consumer| (consumer.actor_id.clone(), consumer.channel.clone()))
            .collect(),
    )
    .with_bot_task_consumers(
        prepared
            .config
            .bot_task_consumers
            .iter()
            .map(|consumer| {
                (
                    (consumer.actor_id.clone(), consumer.channel.clone()),
                    consumer.capabilities.clone(),
                )
            })
            .collect(),
    );
    let mut api = match &prepared.twitch_title {
        Some(generator) => api.with_twitch_title(generator.clone()),
        None => api,
    };
    if let Some(port) = answer_artifacts {
        api = api.with_answer_artifacts(
            port,
            prepared
                .config
                .answer_artifact_consumers
                .iter()
                .map(|consumer| (consumer.actor_id.clone(), consumer.channel.clone()))
                .collect(),
        );
    }
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
#[path = "answer_provider_tests.rs"]
mod answer_provider_tests;
