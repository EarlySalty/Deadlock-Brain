use std::{
    fmt,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{
    bot_config::{BotConfig, DEFAULT_CONFIG_PATH},
    CoreError, Result,
};

pub const DEFAULT_USER_AGENT: &str = "DeadlockBrain/0.1 contact=admin@earlysalty.com";
pub const DEFAULT_SHEET_ID: &str = "1fj9XMQmVUY0FY4cbozvB18PMBnpbdsaMFTZRLa74VRY";
pub const DEFAULT_FIREWORKS_MODEL: &str = "accounts/fireworks/models/deepseek-v4p1-flash";
pub const DEFAULT_FIREWORKS_BASE_URL: &str = "https://api.fireworks.ai/inference/v1";

#[derive(Clone)]
pub struct Settings {
    pub global: Arc<BotConfig>,
    pub project_root: PathBuf,
    pub data_dir: PathBuf,
    pub raw_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub user_agent: String,
    pub http_timeout_seconds: u64,
    pub http_retry_attempts: usize,
    pub http_retry_backoff_milliseconds: u64,
    pub sheet_id: String,
    pub sheet_gid: String,
    pub wiki_enabled: bool,
    pub wiki_min_delay_seconds: f64,
    pub wiki_cache_ttl_seconds: u64,
    pub ai_api_key: Option<String>,
    pub ai_base_url: String,
    pub ai_model: String,
    pub ai_timeout_seconds: u64,
    pub ai_max_completion_tokens: u64,
    pub ai_temperature: f64,
    pub ai_top_p: f64,
    pub ai_use_token_plan: bool,
}

impl fmt::Debug for Settings {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Settings")
            .field("global", &self.global)
            .field("project_root", &self.project_root)
            .field("data_dir", &self.data_dir)
            .field("raw_dir", &self.raw_dir)
            .field("cache_dir", &self.cache_dir)
            .field("user_agent", &self.user_agent)
            .field("http_timeout_seconds", &self.http_timeout_seconds)
            .field("http_retry_attempts", &self.http_retry_attempts)
            .field(
                "http_retry_backoff_milliseconds",
                &self.http_retry_backoff_milliseconds,
            )
            .field("sheet_id", &self.sheet_id)
            .field("sheet_gid", &self.sheet_gid)
            .field("wiki_enabled", &self.wiki_enabled)
            .field("wiki_min_delay_seconds", &self.wiki_min_delay_seconds)
            .field("wiki_cache_ttl_seconds", &self.wiki_cache_ttl_seconds)
            .field(
                "ai_api_key",
                &self.ai_api_key.as_ref().map(|_| "<redacted>"),
            )
            .field("ai_base_url", &self.ai_base_url)
            .field("ai_model", &self.ai_model)
            .field("ai_timeout_seconds", &self.ai_timeout_seconds)
            .field("ai_max_completion_tokens", &self.ai_max_completion_tokens)
            .field("ai_temperature", &self.ai_temperature)
            .field("ai_top_p", &self.ai_top_p)
            .field("ai_use_token_plan", &self.ai_use_token_plan)
            .finish()
    }
}

pub fn repo_root() -> PathBuf {
    Path::new(DEFAULT_CONFIG_PATH)
        .parent()
        .and_then(Path::parent)
        .expect("the default Brain config path is absolute")
        .to_path_buf()
}

pub fn default_data_dir() -> PathBuf {
    repo_root().join("data")
}

/// Loads the installation's default TOML file for library callers.
pub fn load_settings() -> Result<Settings> {
    load_settings_from(Path::new(DEFAULT_CONFIG_PATH))
}

/// Loads and validates the complete TOML snapshot before reading the existing secret source.
pub fn load_settings_from(path: &Path) -> Result<Settings> {
    let global = BotConfig::load(path).map_err(CoreError::from)?;
    Ok(Settings::from_config(global, fireworks_api_key()))
}

impl Settings {
    /// Converts one validated snapshot into the existing consumer settings.
    /// The credential is passed separately by the existing Infisical bootstrap.
    pub fn from_config(global: Arc<BotConfig>, ai_api_key: Option<String>) -> Self {
        let paths = global.paths();
        let http = global.http();
        let sheet = global.sheet();
        let wiki = global.wiki();
        let ai = global.ai();
        Self {
            project_root: paths.project_root.clone(),
            data_dir: paths.data_dir.clone(),
            raw_dir: paths.data_dir.join("raw"),
            cache_dir: paths.data_dir.join("cache"),
            user_agent: http.user_agent.clone(),
            http_timeout_seconds: http.timeout_seconds,
            http_retry_attempts: http.retry_attempts,
            http_retry_backoff_milliseconds: http.retry_backoff_milliseconds,
            sheet_id: sheet.id.clone(),
            sheet_gid: sheet.gid.clone(),
            wiki_enabled: wiki.enabled,
            wiki_min_delay_seconds: wiki.min_delay_seconds,
            wiki_cache_ttl_seconds: wiki.cache_ttl_seconds,
            ai_api_key,
            ai_base_url: ai.base_url.trim_end_matches('/').to_owned(),
            ai_model: DEFAULT_FIREWORKS_MODEL.to_owned(),
            ai_timeout_seconds: ai.timeout_seconds,
            ai_max_completion_tokens: ai.max_completion_tokens,
            ai_temperature: ai.temperature,
            ai_top_p: ai.top_p,
            ai_use_token_plan: false,
            global,
        }
    }
}

/// Reads only the credential names supplied by the existing Infisical process bootstrap.
/// Non-secret operating values are loaded from TOML and never from the environment.
pub fn fireworks_api_key() -> Option<String> {
    ["FIREWORK_API_KEY", "FIREWORKS_API_KEY"]
        .into_iter()
        .find_map(|name| {
            std::env::var(name)
                .ok()
                .filter(|value| !value.trim().is_empty())
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const FIXTURE: &str = include_str!("../../../../config/bot.toml");

    #[test]
    fn repo_root_points_at_project() {
        let root = repo_root();
        assert!(root.join("rust").is_dir());
    }

    #[test]
    fn settings_are_mapped_from_the_validated_snapshot() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("bot.toml");
        fs::write(&path, FIXTURE).unwrap();
        let global = BotConfig::load(&path).unwrap();
        let settings = Settings::from_config(Arc::clone(&global), None);

        assert!(Arc::ptr_eq(&settings.global, &global));
        assert_eq!(settings.project_root, global.paths().project_root);
        assert_eq!(settings.data_dir, global.paths().data_dir);
        assert_eq!(settings.raw_dir, global.paths().data_dir.join("raw"));
        assert_eq!(settings.cache_dir, global.paths().data_dir.join("cache"));
        assert_eq!(settings.user_agent, global.http().user_agent);
        assert_eq!(settings.http_timeout_seconds, global.http().timeout_seconds);
        assert_eq!(settings.http_retry_attempts, global.http().retry_attempts);
        assert_eq!(
            settings.http_retry_backoff_milliseconds,
            global.http().retry_backoff_milliseconds
        );
        assert_eq!(settings.sheet_id, global.sheet().id);
        assert_eq!(settings.sheet_gid, global.sheet().gid);
        assert_eq!(settings.wiki_enabled, global.wiki().enabled);
        assert_eq!(
            settings.wiki_cache_ttl_seconds,
            global.wiki().cache_ttl_seconds
        );
        assert_eq!(
            settings.ai_base_url,
            global.ai().base_url.trim_end_matches('/')
        );
        assert_eq!(settings.ai_model, DEFAULT_FIREWORKS_MODEL);
        assert_eq!(settings.ai_timeout_seconds, global.ai().timeout_seconds);
        assert_eq!(
            settings.ai_max_completion_tokens,
            global.ai().max_completion_tokens
        );
        assert_eq!(settings.ai_temperature, global.ai().temperature);
        assert_eq!(settings.ai_top_p, global.ai().top_p);
        assert!(settings.ai_api_key.is_none());
    }

    #[test]
    fn config_path_must_be_absolute() {
        let error = load_settings_from(Path::new("relative/bot.toml")).unwrap_err();
        assert!(!error.to_string().contains("relative/bot.toml"));
    }
}
