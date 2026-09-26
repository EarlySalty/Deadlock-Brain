//! Environment is the sole secret port, compatible with deadlock-brain-secret-exec.
use crate::{config::DatabaseAuth, Config, Error};
use brain_policy::{AuthGrant, CredentialRegistry};
use std::collections::BTreeSet;

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
    pub fn from_environment(config: &Config) -> Result<Self, Error> {
        Self::load(config, |name| std::env::var(name).ok())
    }

    /// Injectable lookup avoids mutating process-global environment in parallel tests.
    pub fn load(config: &Config, lookup: impl Fn(&str) -> Option<String>) -> Result<Self, Error> {
        config.validate()?;
        let provider_key = required(&lookup, &config.provider.api_key_env, "provider", true)?;
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
