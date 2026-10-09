//! One explicit Infisical snapshot supplies the configured secret references.
use crate::{config::DatabaseAuth, Config, Error};
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
    pub(crate) internal_credentials: CredentialRegistry,
    pub(crate) discord_live_token: Option<String>,
    pub(crate) mirror_password: Option<String>,
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
        let discord_live_token = config
            .discord_live
            .as_ref()
            .map(|live| {
                if live.token_secret != "DISCORD_PUBLIC_FACTS_TOKEN" {
                    return Err(Error::ConfigInvalid("discord_live"));
                }
                required(&lookup, &live.token_secret, "discord_live", true)
            })
            .transpose()?;
        let provider_key = match config.provider.kind {
            crate::config::ProviderKind::OpenaiCompatible => {
                required(&lookup, &config.provider.api_key_env, "provider", true)?
            }
            crate::config::ProviderKind::CodexSubscription => String::new(),
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
        let mirror_password = config
            .deadlock_api
            .as_ref()
            .map(|api| required(&lookup, &api.mirror_password_secret, "mirror", false))
            .transpose()?;
        let mut tokens = BTreeSet::new();
        if !provider_key.is_empty() {
            tokens.insert(provider_key.clone());
        }
        if let Some(password) = &postgres_password {
            if !tokens.insert(password.clone()) {
                return Err(Error::SecretInvalid("duplicate"));
            }
        }
        if let Some(password) = &mirror_password {
            if !tokens.insert(password.clone()) {
                return Err(Error::SecretInvalid("duplicate"));
            }
        }
        let mut grants = Vec::with_capacity(config.credentials.len());
        let mut internal_grants = Vec::new();
        for grant in &config.credentials {
            let token = required(&lookup, &grant.token_env, "api", true)?;
            if !tokens.insert(token.clone()) {
                return Err(Error::SecretInvalid("duplicate"));
            }
            let auth = AuthGrant::from_secret(
                &token,
                &grant.actor_id,
                &grant.channel,
                grant.scopes.clone(),
                grant.provider_egress.clone(),
            );
            if grant.actor_id == "second-brain" && grant.channel == "internal" {
                internal_grants.push(auth);
            } else {
                grants.push(auth);
            }
        }
        Ok(Self {
            provider_key,
            postgres_password,
            credentials: CredentialRegistry::new(grants),
            internal_credentials: CredentialRegistry::new(internal_grants),
            discord_live_token,
            mirror_password,
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
    #[ignore = "explicit service snapshot probe with its existing private credential on FD5"]
    fn live_service_snapshot_supplies_the_readonly_mirror_reference() {
        let mut config = Config::parse(
            &std::fs::read("/home/nathanael/.config/deadlock-brain/brain-serve.json").unwrap(),
        )
        .unwrap();
        config.deadlock_api = Some(crate::config::DeadlockApi::default());
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let secrets = runtime
            .block_on(Secrets::from_infisical(
                &config,
                Path::new("/etc/deadlock-brain/infisical.json"),
            ))
            .unwrap();
        assert!(secrets.mirror_password.is_some());
        println!("mirror_reference_available=true");
    }

    #[test]
    fn mirror_secret_is_explicit_required_and_not_a_duplicate_client_credential() {
        let mut config = Config::parse(CONFIG).unwrap();
        config.deadlock_api = Some(crate::config::DeadlockApi::default());
        assert!(matches!(
            Secrets::load(&config, fixture),
            Err(Error::SecretMissing("mirror"))
        ));
        let secrets = Secrets::load(&config, |name| {
            if name == "BRAIN_PG_READONLY_PASSWORD" {
                Some("synthetic-mirror-password".into())
            } else {
                fixture(name)
            }
        })
        .unwrap();
        assert!(secrets.mirror_password.is_some());
        assert!(!format!("{secrets:?}").contains("synthetic-mirror-password"));
        assert!(matches!(
            Secrets::load(&config, |name| if name == "BRAIN_PG_READONLY_PASSWORD" {
                fixture("BRAIN_SERVE_API_TOKEN")
            } else {
                fixture(name)
            }),
            Err(Error::SecretInvalid("duplicate"))
        ));
    }

    #[test]
    fn subscription_does_not_load_a_provider_secret() {
        let mut value: serde_json::Value = serde_json::from_slice(CONFIG).unwrap();
        value["provider"] = serde_json::from_slice(include_bytes!(
            "../../../../config/codex-subscription-provider.example.json"
        ))
        .unwrap();
        let config = Config::parse(&serde_json::to_vec(&value).unwrap()).unwrap();
        let secrets = Secrets::load(&config, |name| {
            assert_ne!(name, "BRAIN_SERVE_PROVIDER_API_KEY");
            fixture(name)
        })
        .unwrap();
        assert!(secrets.provider_key.is_empty());
        for (field, invalid) in [
            ("base_url", serde_json::json!("https://example.com/v1")),
            ("model", serde_json::json!("gpt-6-sol")),
            ("api_key_env", serde_json::json!("FIREWORK_API_KEY")),
            ("retry_attempts", serde_json::json!(2)),
        ] {
            let mut invalid_config = value.clone();
            invalid_config["provider"][field] = invalid;
            assert!(Config::parse(&serde_json::to_vec(&invalid_config).unwrap()).is_err());
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
    fn vorhandene_interne_referenz_wird_validiert_und_nicht_ausgegeben() {
        let mut config = Config::parse(CONFIG).unwrap();
        config.discord_live = Some(crate::config::DiscordLiveConfig {
            token_secret: "DISCORD_PUBLIC_FACTS_TOKEN".into(),
        });
        let secrets = Secrets::load(&config, |name| {
            if name == "DISCORD_PUBLIC_FACTS_TOKEN" {
                Some("synthetische-interne-referenz".into())
            } else {
                fixture(name)
            }
        })
        .unwrap();
        assert!(secrets.discord_live_token.is_some());
        assert!(!format!("{secrets:?}").contains("synthetische-interne-referenz"));
        assert!(Secrets::load(&config, |name| {
            if name == "DISCORD_PUBLIC_FACTS_TOKEN" {
                Some("ungueltig\n".into())
            } else {
                fixture(name)
            }
        })
        .is_err());
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
        second.actor_id = "twitch-bot".into();
        second.channel = "twitch".into();
        second.scopes = BTreeSet::from(["bot.public".into()]);
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
