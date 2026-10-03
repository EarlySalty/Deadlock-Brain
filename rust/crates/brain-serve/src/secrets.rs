//! One explicit Infisical snapshot supplies the configured secret references.
use crate::{
    config::{DatabaseAuth, ProviderKind},
    Config, Error,
};
use brain_policy::{AuthGrant, CredentialRegistry};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
    sync::mpsc,
    time::Instant,
};

pub struct Secrets {
    pub(crate) provider_key: String,
    pub(crate) postgres_password: Option<String>,
    pub(crate) credentials: CredentialRegistry,
}

impl std::fmt::Debug for Secrets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Secrets(<redacted>)")
    }
}

fn required(
    lookup: &impl Fn(&str) -> Option<String>,
    name: &str,
    role: &'static str,
    bearer: bool,
) -> Result<String, Error> {
    let value = lookup(name).ok_or(Error::SecretMissing(role))?;
    if value.trim().is_empty()
        || value.len() > 4096
        || value.chars().any(char::is_control)
        || (bearer && !value.bytes().all(|c| c.is_ascii_graphic()))
    {
        return Err(Error::SecretInvalid(role));
    }
    Ok(value)
}

impl Secrets {
    /// The shared loader can synchronously poll a private pipe. Isolate startup
    /// loading from the caller's absolute deadline without a runtime shutdown join.
    pub fn load_until(config: &Config, path: &Path, deadline: Instant) -> Result<Self, Error> {
        config.validate()?;
        if Instant::now() >= deadline {
            return Err(Error::StartupTimeout);
        }
        let config = config.clone();
        let path = path.to_owned();
        let (sender, receiver) = mpsc::sync_channel(1);
        std::thread::Builder::new()
            .name("brain-secret-startup".into())
            .spawn(move || {
                let result = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .map_err(|_| Error::Runtime)
                    .and_then(|runtime| runtime.block_on(Self::from_infisical(&config, &path)));
                // An expired caller closes the receiver; dropping this result then
                // disposes of the snapshot instead of logging or retaining it.
                let _ = sender.send(result);
            })
            .map_err(|_| Error::Runtime)?;
        let result = receiver
            .recv_timeout(deadline.saturating_duration_since(Instant::now()))
            .map_err(|error| match error {
                mpsc::RecvTimeoutError::Timeout => Error::StartupTimeout,
                mpsc::RecvTimeoutError::Disconnected => Error::SecretSource,
            })?;
        if Instant::now() >= deadline {
            return Err(Error::StartupTimeout);
        }
        result
    }

    pub async fn from_infisical(config: &Config, path: &Path) -> Result<Self, Error> {
        config.validate()?;
        let snapshot: BTreeMap<_, _> = dl_token_secrets::values(path)
            .await
            .map_err(|_| Error::SecretSource)?
            .into_iter()
            .collect();
        Self::load(config, |name| {
            snapshot.get(name).map(|value| value.as_str().to_owned())
        })
    }

    /// Injectable lookup uses the same validation for runtime snapshots and synthetic tests.
    pub fn load(config: &Config, lookup: impl Fn(&str) -> Option<String>) -> Result<Self, Error> {
        config.validate()?;
        let provider_key = match config.provider.kind {
            ProviderKind::CodexCli => String::new(),
            ProviderKind::OpenaiCompatible => {
                required(&lookup, &config.provider.api_key_env, "provider", true)?
            }
        };
        let postgres_password = match config.postgres.auth {
            DatabaseAuth::Peer => None,
            DatabaseAuth::Password => Some(required(
                &lookup,
                config
                    .postgres
                    .password_env
                    .as_deref()
                    .ok_or(Error::ConfigInvalid("postgres_auth"))?,
                "postgres",
                false,
            )?),
        };
        let mut tokens = BTreeSet::from([provider_key.clone()]);
        if let Some(password) = &postgres_password {
            if !tokens.insert(password.clone()) {
                return Err(Error::SecretInvalid("duplicate"));
            }
        }
        let mut grants = Vec::with_capacity(config.credentials.len());
        for grant in &config.credentials {
            let token = required(&lookup, &grant.token_env, "api", true)?;
            if !tokens.insert(token.clone()) {
                return Err(Error::SecretInvalid("duplicate"));
            }
            grants.push(AuthGrant::from_secret(
                &token,
                &grant.actor_id,
                &grant.channel,
                grant.scopes.clone(),
                grant.provider_egress.clone(),
            ));
        }
        Ok(Self {
            provider_key,
            postgres_password,
            credentials: CredentialRegistry::new(grants),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const CONFIG: &[u8] = include_bytes!("../../../../config/brain-serve.example.json");

    fn fixture(name: &str) -> Option<String> {
        match name {
            "BRAIN_SERVE_PROVIDER_API_KEY" => Some("synthetic-provider-credential".into()),
            "BRAIN_SERVE_API_TOKEN" => Some("synthetic-client-credential".into()),
            "BRAIN_SERVE_PG_PASSWORD" => Some("synthetic-database-credential".into()),
            _ => None,
        }
    }

    #[test]
    fn absent_empty_and_invalid_bearer_secrets_fail_closed() {
        let config = Config::parse(CONFIG).unwrap();
        for key in [
            &config.provider.api_key_env,
            &config.credentials[0].token_env,
        ] {
            for value in [
                None,
                Some(String::new()),
                Some(" ".into()),
                Some("bad\nheader".into()),
                Some("has space".into()),
            ] {
                assert!(Secrets::load(&config, |name| if name == key {
                    value.clone()
                } else {
                    fixture(name)
                })
                .is_err());
            }
        }
    }

    #[test]
    fn password_is_required_only_for_explicit_password_authentication() {
        let mut config = Config::parse(CONFIG).unwrap();
        assert!(Secrets::load(&config, fixture).is_ok());
        config.postgres.auth = DatabaseAuth::Password;
        config.postgres.password_env = Some("BRAIN_SERVE_PG_PASSWORD".into());
        assert!(matches!(
            Secrets::load(&config, |name| if name == "BRAIN_SERVE_PG_PASSWORD" {
                None
            } else {
                fixture(name)
            }),
            Err(Error::SecretMissing("postgres"))
        ));
        assert!(Secrets::load(&config, fixture).is_ok());
    }

    #[test]
    fn duplicate_credentials_are_not_ambiguous_and_debug_is_redacted() {
        let mut config = Config::parse(CONFIG).unwrap();
        let secret = Secrets::load(&config, fixture).unwrap();
        let debug = format!("{secret:?}");
        assert!(debug.contains("redacted"));
        for name in ["BRAIN_SERVE_PROVIDER_API_KEY", "BRAIN_SERVE_API_TOKEN"] {
            assert!(!debug.contains(&fixture(name).unwrap()));
        }
        let mut second = config.credentials[0].clone();
        second.token_env = "SECOND_CLIENT_TOKEN".into();
        second.actor_id = "different-actor".into();
        config.credentials.push(second);
        assert!(matches!(
            Secrets::load(&config, |name| if name == "SECOND_CLIENT_TOKEN" {
                fixture("BRAIN_SERVE_API_TOKEN")
            } else {
                fixture(name)
            }),
            Err(Error::SecretInvalid("duplicate"))
        ));
    }
}
