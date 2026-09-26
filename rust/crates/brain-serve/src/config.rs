//! Strict, non-secret configuration. Unknown fields and incomplete sections are errors.
use crate::Error;
use brain_contracts::Budget;
use serde::Deserialize;
use std::{collections::BTreeSet, fs::File, io::Read, net::SocketAddr, path::Path};

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
    pub credentials: Vec<Credential>,
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
pub struct Credential {
    pub token_env: String,
    pub actor_id: String,
    pub channel: String,
    pub scopes: BTreeSet<String>,
    pub provider_egress: BTreeSet<String>,
}

fn identifier(value: &str, maximum: usize) -> bool {
    !value.is_empty()
        && value.len() <= maximum
        && value
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"._:/-".contains(&c))
}

fn env_name(value: &str) -> bool {
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
        let metadata = std::fs::metadata(path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                Error::ConfigMissing
            } else {
                Error::ConfigIo
            }
        })?;
        require(metadata.is_file(), "file")?;
        let file = File::open(path).map_err(|_| Error::ConfigIo)?;
        require(
            file.metadata().map_err(|_| Error::ConfigIo)?.is_file(),
            "file",
        )?;
        let mut bytes = Vec::new();
        file.take(64 * 1024 + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| Error::ConfigIo)?;
        Self::parse(&bytes)
    }

    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        require(bytes.len() <= 64 * 1024, "size")?;
        // Never expose serde's error: it can include input values.
        let config: Self = serde_json::from_slice(bytes).map_err(|_| Error::ConfigSyntax)?;
        config.validate()?;
        Ok(config)
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
                DatabaseAuth::Password => pg.password_env.as_deref().is_some_and(env_name),
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
                && env_name(&p.api_key_env)
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
        require(
            !self.credentials.is_empty() && self.credentials.len() <= 128,
            "credentials",
        )?;
        let mut names = BTreeSet::from([p.api_key_env.as_str()]);
        if let Some(name) = pg.password_env.as_deref() {
            require(names.insert(name), "secret_references")?;
        }
        for grant in &self.credentials {
            require(
                env_name(&grant.token_env)
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
        Ok(())
    }
}

#[cfg(test)]
mod tests;
