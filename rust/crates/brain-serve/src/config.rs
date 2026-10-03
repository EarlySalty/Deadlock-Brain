//! Strict, non-secret configuration. Unknown fields and incomplete sections are errors.
use crate::Error;
use brain_contracts::Budget;
use serde::Deserialize;
use std::{collections::BTreeSet, net::SocketAddr, path::Path};

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub bind: SocketAddr,
    pub postgres: Postgres,
    pub release: Release,
    pub provider: Provider,
    pub budgets: Budgets,
    pub timeouts: Timeouts,
    pub retrieval: Retrieval,
    pub kernel: Kernel,
    pub analytics: Option<Analytics>,
    pub credentials: Vec<Credential>,
    pub internal_operator: Option<InternalOperator>,
}

impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Even invalid input can contain a mistakenly pasted secret in a non-secret field.
        f.debug_struct("Config").finish_non_exhaustive()
    }
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Postgres {
    pub socket_dir: std::path::PathBuf,
    pub port: u16,
    pub username: String,
    pub database: String,
    pub auth: DatabaseAuth,
    /// Bestehender Feldname für den Infisical-Schlüsselnamen, ohne ENV-Zugriff.
    pub password_env: Option<String>,
    pub max_connections: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseAuth {
    Peer,
    Password,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Release {
    pub id: String,
    pub knowledge_version: String,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Provider {
    pub kind: ProviderKind,
    pub base_url: String,
    pub model: String,
    /// Bestehender Feldname für den Infisical-Schlüsselnamen, ohne ENV-Zugriff.
    pub api_key_env: String,
    pub retry_attempts: usize,
    pub retry_backoff_ms: u64,
    pub max_response_bytes: usize,
    pub pricing: Option<Pricing>,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderKind {
    OpenaiCompatible,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pricing {
    pub input_micros_per_token: u64,
    pub output_micros_per_token: u64,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Budgets {
    pub max_network_rounds: u32,
    pub max_input_tokens: u32,
    pub max_output_tokens: u32,
    pub max_cost_micros: u64,
}

impl From<&Budgets> for Budget {
    fn from(value: &Budgets) -> Self {
        Self {
            max_network_rounds: value.max_network_rounds,
            max_input_tokens: value.max_input_tokens,
            max_output_tokens: value.max_output_tokens,
            max_cost_micros: value.max_cost_micros,
        }
    }
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Timeouts {
    pub startup_ms: u64,
    pub request_ms: u64,
    pub postgres_connect_ms: u64,
    pub postgres_pool_wait_ms: u64,
    pub postgres_statement_ms: u64,
    pub postgres_lock_ms: u64,
    pub provider_ms: u64,
    pub readiness_ms: u64,
    pub shutdown_ms: u64,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Retrieval {
    pub kind: RetrievalKind,
    pub limit: usize,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalKind {
    ReleaseLexical,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Kernel {
    pub cache_entries: usize,
    pub cache_ttl_ms: u64,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Analytics {
    pub patch: String,
    pub min_unix_timestamp: i64,
    pub max_unix_timestamp: i64,
    pub max_rows: usize,
    pub request_timeout_ms: u64,
    pub schema_sha256: String,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Credential {
    /// Bestehender Feldname für den Infisical-Schlüsselnamen, ohne ENV-Zugriff.
    pub token_env: String,
    pub actor_id: String,
    pub channel: String,
    pub scopes: BTreeSet<String>,
    pub provider_egress: BTreeSet<String>,
    /// Optionaler fester Wissensstand; ohne Override bleibt das bisherige Release.
    pub release: Option<Release>,
}

impl Credential {
    /// Quellenbindung der bestätigten C9-Identitäten, unabhängig von anderen Scope-Trägern.
    pub(crate) fn c9_bound_scope(&self) -> Option<&'static str> {
        match (self.actor_id.as_str(), self.channel.as_str()) {
            ("docs-client", "docs") => Some("docs.public"),
            ("second-brain", "internal") => Some("second_brain.internal"),
            _ => None,
        }
    }
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InternalOperator {
    pub socket: std::path::PathBuf,
    pub release: Release,
    pub credential: Credential,
}

fn identifier(value: &str, maximum: usize) -> bool {
    !value.is_empty()
        && value.len() <= maximum
        && value
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"._:/-".contains(&c))
}

fn secret_name(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .enumerate()
            .all(|(i, c)| c == b'_' || c.is_ascii_uppercase() || (i > 0 && c.is_ascii_digit()))
}

fn require(condition: bool, section: &'static str) -> Result<(), Error> {
    if condition {
        Ok(())
    } else {
        Err(Error::ConfigInvalid(section))
    }
}

impl Config {
    pub fn load(path: &Path) -> Result<Self, Error> {
        Self::parse(&crate::bot_toml::read(path)?)
    }

    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        let root = crate::bot_toml::document(bytes)?;
        let serve = crate::bot_toml::value(&root, &["brain", "serve"])?;
        require(serve.get("internal_operator").is_none(), "operator_table")?;
        let mut config: Self = serve.try_into().map_err(|_| Error::ConfigSyntax)?;
        config.internal_operator = root
            .get("brain")
            .and_then(|brain| brain.get("operator"))
            .map(|operator| operator.clone().try_into().map_err(|_| Error::ConfigSyntax))
            .transpose()?;
        config.validate()?;
        Ok(config)
    }

    pub fn all_credentials(&self) -> impl Iterator<Item = &Credential> {
        self.credentials.iter().chain(
            self.internal_operator
                .iter()
                .map(|operator| &operator.credential),
        )
    }

    pub fn validate(&self) -> Result<(), Error> {
        let pg = &self.postgres;
        require(
            pg.socket_dir.is_absolute()
                && pg
                    .socket_dir
                    .to_str()
                    .is_some_and(|v| !v.chars().any(char::is_control))
                && pg.port > 0
                && identifier(&pg.username, 63)
                && identifier(&pg.database, 63)
                && (1..=16).contains(&pg.max_connections),
            "postgres",
        )?;
        require(
            match pg.auth {
                DatabaseAuth::Peer => pg.password_env.is_none(),
                DatabaseAuth::Password => pg.password_env.as_deref().is_some_and(secret_name),
            },
            "postgres_auth",
        )?;
        require(
            identifier(&self.release.id, 512)
                && !["current", "latest"].contains(&self.release.id.as_str())
                && identifier(&self.release.knowledge_version, 512)
                && !["current", "latest"].contains(&self.release.knowledge_version.as_str()),
            "release",
        )?;
        let p = &self.provider;
        let endpoint =
            reqwest::Url::parse(&p.base_url).map_err(|_| Error::ConfigInvalid("provider"))?;
        let loopback = endpoint.host_str().is_some_and(|host| {
            host == "localhost"
                || host
                    .trim_matches(['[', ']'])
                    .parse::<std::net::IpAddr>()
                    .is_ok_and(|ip| ip.is_loopback())
        });
        require(
            endpoint.has_host()
                && endpoint.username().is_empty()
                && endpoint.password().is_none()
                && endpoint.query().is_none()
                && endpoint.fragment().is_none()
                && (endpoint.scheme() == "https" || (endpoint.scheme() == "http" && loopback))
                && (loopback || p.pricing.is_some())
                && identifier(&p.model, 512)
                && secret_name(&p.api_key_env)
                && (1..=8).contains(&p.retry_attempts)
                && p.retry_backoff_ms <= 10_000
                && (128..=8 * 1024 * 1024).contains(&p.max_response_bytes),
            "provider",
        )?;
        let b = &self.budgets;
        require(
            (1..=8).contains(&b.max_network_rounds)
                && b.max_input_tokens > 0
                && b.max_output_tokens > 0
                && b.max_cost_micros > 0,
            "budgets",
        )?;
        if let Some(price) = p.pricing {
            require(
                (loopback
                    || (price.input_micros_per_token > 0 && price.output_micros_per_token > 0))
                    && u64::from(b.max_input_tokens)
                        .checked_mul(price.input_micros_per_token)
                        .and_then(|input| {
                            u64::from(b.max_output_tokens)
                                .checked_mul(price.output_micros_per_token)
                                .and_then(|output| input.checked_add(output))
                        })
                        .is_some(),
                "pricing",
            )?;
        }
        let t = &self.timeouts;
        require(
            (1..=120_000).contains(&t.startup_ms)
                && (1..=60_000).contains(&t.request_ms)
                && (1..=t.startup_ms.min(t.request_ms)).contains(&t.postgres_connect_ms)
                && (1..=t.request_ms).contains(&t.postgres_pool_wait_ms)
                && (1..=t.request_ms).contains(&t.postgres_statement_ms)
                && (1..=t.postgres_statement_ms).contains(&t.postgres_lock_ms)
                && (1..=t.request_ms).contains(&t.provider_ms)
                && (1..=t.startup_ms).contains(&t.readiness_ms)
                && (t.request_ms..=120_000).contains(&t.shutdown_ms),
            "timeouts",
        )?;
        require((1..=100).contains(&self.retrieval.limit), "retrieval")?;
        require(
            self.kernel.cache_entries <= 1024 && self.kernel.cache_ttl_ms <= 60_000,
            "kernel",
        )?;
        if let Some(analytics) = &self.analytics {
            let pinned = dbrain_sources::schema_watch::OpenApiSnapshot::pinned()
                .map_err(|_| Error::ConfigInvalid("analytics_schema"))?;
            let request = dbrain_sources::AnalyticsLookupRequest {
                kind: dbrain_sources::AnalyticsKind::Meta,
                hero_id: 1,
                item_id: None,
                patch: analytics.patch.clone(),
                min_unix_timestamp: analytics.min_unix_timestamp,
                max_unix_timestamp: analytics.max_unix_timestamp,
                max_rows: analytics.max_rows,
            };
            require(
                request.validate().is_ok()
                    && (1..=5_000).contains(&analytics.request_timeout_ms)
                    && analytics.request_timeout_ms.saturating_mul(2) <= t.request_ms
                    && analytics.schema_sha256 == pinned.schema_sha256,
                "analytics",
            )?;
        }
        require(
            !self.credentials.is_empty() && self.credentials.len() <= 128,
            "credentials",
        )?;
        let mut names = BTreeSet::from([p.api_key_env.as_str()]);
        if let Some(name) = pg.password_env.as_deref() {
            require(names.insert(name), "secret_references")?;
        }
        let mut bindings = std::collections::BTreeMap::new();
        let mut internal_count = 0;
        require(
            self.credentials.iter().all(|grant| {
                grant.token_env != "BRAIN_SERVE_SECOND_BRAIN_TOKEN"
                    && grant.actor_id != "second-brain"
                    && grant.channel != "internal"
                    && !grant.scopes.contains("second_brain.internal")
            }),
            "public_registry_internal_grant",
        )?;
        for grant in self.all_credentials() {
            let release = grant.release.as_ref().unwrap_or(&self.release);
            require(
                identifier(&release.id, 512)
                    && !["current", "latest"].contains(&release.id.as_str())
                    && identifier(&release.knowledge_version, 512)
                    && !["current", "latest"].contains(&release.knowledge_version.as_str()),
                "credential_release",
            )?;
            let key = (&grant.actor_id, &grant.channel);
            let value = (&release.id, &release.knowledge_version);
            if let Some(previous) = bindings.insert(key, value) {
                require(previous == value, "credential_release_conflict")?;
            }
            if grant.actor_id == "docs-client"
                || grant.channel == "docs"
                || grant.token_env == "BRAIN_SERVE_DOCS_PUBLIC_TOKEN"
            {
                require(
                    grant.actor_id == "docs-client"
                        && grant.channel == "docs"
                        && grant.scopes == BTreeSet::from(["docs.public".into()])
                        && grant.provider_egress == BTreeSet::from(["public".into()])
                        && grant.release.is_some(),
                    "docs_client_grant",
                )?;
            }
            if grant.actor_id == "second-brain"
                || grant.channel == "internal"
                || grant.scopes.contains("second_brain.internal")
            {
                internal_count += 1;
                let internal = self
                    .internal_operator
                    .as_ref()
                    .ok_or(Error::ConfigInvalid("internal_operator"))?;
                require(
                    grant.actor_id == "second-brain"
                        && grant.channel == "internal"
                        && grant.scopes == BTreeSet::from(["second_brain.internal".into()])
                        && grant.provider_egress.is_empty()
                        && grant.release.is_some()
                        && release.id == internal.release.id
                        && release.knowledge_version == internal.release.knowledge_version,
                    "internal_operator_grant",
                )?;
            }
            require(
                secret_name(&grant.token_env)
                    && names.insert(&grant.token_env)
                    && identifier(&grant.actor_id, 128)
                    && identifier(&grant.channel, 128)
                    && grant.scopes.len() <= 128
                    && grant.scopes.iter().all(|s| identifier(s, 128))
                    && grant
                        .provider_egress
                        .iter()
                        .all(|s| ["public", "internal", "private"].contains(&s.as_str()))
                    && (grant.provider_egress.is_empty()
                        || grant.provider_egress.contains("public")),
                "credentials",
            )?;
        }
        if let Some(internal) = &self.internal_operator {
            require(
                internal_count == 1
                    && internal.credential.actor_id == "second-brain"
                    && internal.credential.channel == "internal"
                    && internal.socket.is_absolute()
                    && internal.socket.as_os_str().len() <= 107
                    && internal.socket.components().all(|component| {
                        matches!(
                            component,
                            std::path::Component::RootDir | std::path::Component::Normal(_)
                        )
                    }),
                "internal_operator",
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
