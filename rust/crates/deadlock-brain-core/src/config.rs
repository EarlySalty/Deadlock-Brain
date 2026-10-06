use std::{fmt, path::{Path, PathBuf}, sync::Arc};

use crate::{bot_config::{BotConfig, DEFAULT_CONFIG_PATH}, Result};

pub const DEFAULT_USER_AGENT: &str = "DeadlockBrain/0.1 contact=admin@earlysalty.com";
pub const DEFAULT_SHEET_ID: &str = "1fj9XMQmVUY0FY4cbozvB18PMBnpbdsaMFTZRLa74VRY";
pub const DEFAULT_FIREWORKS_MODEL: &str = "accounts/fireworks/models/deepseek-v4-flash";
pub const DEFAULT_FIREWORKS_BASE_URL: &str = "https://api.fireworks.ai/inference/v1";

#[derive(Clone)]
pub struct Settings {
    pub global: Arc<BotConfig>,
    pub project_root: PathBuf,
    pub data_dir: PathBuf,
    pub raw_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub user_agent: String,
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
            .field("project_root", &self.project_root)
            .field("data_dir", &self.data_dir)
            .field("raw_dir", &self.raw_dir)
            .field("cache_dir", &self.cache_dir)
            .field("user_agent", &self.user_agent)
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
            .field(
                "ai_max_completion_tokens",
                &self.ai_max_completion_tokens,
            )
            .field("ai_temperature", &self.ai_temperature)
            .field("ai_top_p", &self.ai_top_p)
            .field("ai_use_token_plan", &self.ai_use_token_plan)
            .finish()
    }
}

/// Installationswurzel nur für Legacy-Werkzeuge ohne eigene Config-Option.
/// Produktive Aufrufer verwenden ausschließlich die Pfade ihrer validierten Momentaufnahme.
pub fn repo_root() -> PathBuf {
    Path::new(DEFAULT_CONFIG_PATH).parent().and_then(Path::parent)
        .expect("absolute install path").to_path_buf()
}

pub fn default_data_dir() -> PathBuf {
    repo_root().join("data")
}

impl Settings {
    /// Reine Übergabe bereits validierter Betriebswerte. Keine ENV-Leser, kein I/O.
    /// Das Secret wird vom vorhandenen Infisical-Bootstrap getrennt übergeben.
    pub fn from_config(config: Arc<BotConfig>, ai_api_key: Option<String>) -> Self {
        let paths = config.paths();
        let ai = config.ai();
        Self {
            global: Arc::clone(&config),
            project_root: paths.project_root.clone(),
            data_dir: paths.data_dir.clone(),
            raw_dir: paths.data_dir.join("raw"),
            cache_dir: paths.data_dir.join("cache"),
            user_agent: config.http().user_agent.clone(),
            sheet_id: config.sheet().id.clone(),
            sheet_gid: config.sheet().gid.clone(),
            wiki_enabled: config.wiki().enabled,
            wiki_min_delay_seconds: config.wiki().min_delay_seconds,
            wiki_cache_ttl_seconds: config.wiki().cache_ttl_seconds,
            ai_api_key,
            ai_base_url: ai.base_url.trim_end_matches('/').to_owned(),
            // Ohne Pin erfolgt kein Rückfall auf einen einkompilierten Alias.
            // Der Auswahlmanager muss ein geprüftes Modell bereitstellen.
            ai_model: ai.pin.clone().unwrap_or_default(),
            ai_timeout_seconds: ai.timeout_seconds,
            ai_max_completion_tokens: ai.max_completion_tokens,
            ai_temperature: ai.temperature,
            ai_top_p: ai.top_p,
            ai_use_token_plan: false,
        }
    }
}

/// Kompatibler Einstieg für Bibliotheksaufrufer. Betriebswerte kommen nur aus der
/// zentralen Datei. Dienste mit --config reichen dieselbe Momentaufnahme direkt weiter.
pub fn load_settings() -> Result<Settings> {
    let config = BotConfig::load(Path::new(DEFAULT_CONFIG_PATH))?;
    Ok(Settings::from_config(config, fireworks_api_key()))
}

/// Ausschließlich Secret-Namen aus dem vorhandenen Infisical-Prozessbootstrap.
/// Keine .env-Datei, keine Modell-/Endpunkt-/Timeout-Variablen und keine Ausgabe.
pub fn fireworks_api_key() -> Option<String> {
    ["FIREWORK_API_KEY", "FIREWORKS_API_KEY"].into_iter().find_map(|name| {
        std::env::var(name).ok().filter(|value| !value.trim().is_empty())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validated_snapshot_reaches_existing_settings_consumers() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("bot.toml");
        let text = include_str!("../../../../config/bot.toml")
            .replacen("timeout_seconds = 300", "timeout_seconds = 347", 1)
            .replacen("max_completion_tokens = 16000", "max_completion_tokens = 4321", 1)
            .replacen("gid = \"0\"", "gid = \"71\"", 1)
            .replacen("cache_ttl_seconds = 604800", "cache_ttl_seconds = 1234", 1);
        std::fs::write(&path, text).unwrap();
        let config = BotConfig::load(&path).unwrap();
        let settings = Settings::from_config(Arc::clone(&config), None);
        let ai = crate::ai::AiConfig::from_settings(&settings);
        assert_eq!(ai.timeout_seconds, 347);
        assert_eq!(ai.max_completion_tokens, 4321);
        assert_eq!(settings.sheet_gid, "71");
        assert_eq!(settings.wiki_cache_ttl_seconds, 1234);
        assert_eq!(settings.project_root, config.paths().project_root);
        assert_eq!(settings.raw_dir, config.paths().data_dir.join("raw"));
        assert!(ai.model.is_empty());
        assert!(!ai.api_key_present());
    }
}
