use std::{
    env, fmt,
    path::PathBuf,
};

use crate::{CoreError, Result};

pub const DEFAULT_USER_AGENT: &str = "DeadlockBrain/0.1 contact=admin@earlysalty.com";
pub const DEFAULT_SHEET_ID: &str = "1fj9XMQmVUY0FY4cbozvB18PMBnpbdsaMFTZRLa74VRY";
pub const DEFAULT_FIREWORKS_MODEL: &str = "accounts/fireworks/models/deepseek-v4-flash";
pub const DEFAULT_FIREWORKS_BASE_URL: &str = "https://api.fireworks.ai/inference/v1";

#[derive(Clone)]
pub struct Settings {
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

pub fn repo_root() -> PathBuf {
    // Laufzeitvorgabe hat Vorrang: ein aus einem Feature-Worktree gebautes und
    // spaeter bereinigtes Release wuerde sonst ueber das eingebrannte
    // CARGO_MANIFEST_DIR auf den entfernten Worktree zeigen. Der Dienst setzt
    // DEADLOCK_BRAIN_ROOT auf das installierte Repo.
    repo_root_resolve(
        std::env::var_os("DEADLOCK_BRAIN_ROOT").map(PathBuf::from),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")),
    )
}

fn repo_root_resolve(env_root: Option<PathBuf>, manifest_dir: PathBuf) -> PathBuf {
    if let Some(root) = env_root {
        if root.is_dir() {
            return root;
        }
        eprintln!(
            "DEADLOCK_BRAIN_ROOT is set but not a usable directory; falling back to the build-time repo path."
        );
    }

    for ancestor in manifest_dir.ancestors() {
        if ancestor.join("rust/Cargo.toml").is_file() && ancestor.join("config").is_dir() {
            return ancestor.to_path_buf();
        }
    }

    let mut fallback = manifest_dir.as_path();
    for _ in 0..3 {
        if let Some(parent) = fallback.parent() {
            fallback = parent;
        }
    }
    fallback.to_path_buf()
}

pub fn default_data_dir() -> PathBuf {
    repo_root().join("data")
}

pub fn load_settings() -> Result<Settings> {
    let project_root = repo_root();
    let data_dir = path_env("DEADLOCK_BRAIN_DATA_DIR", project_root.join("data"));
    let fireworks_api_key = setting("FIREWORK_API_KEY")
        .or_else(|| setting("FIREWORKS_API_KEY"))
        .filter(|value| !value.trim().is_empty());
    let fireworks_base_url = setting("FIREWORK_BASE_URL")
        .or_else(|| setting("FIREWORKS_BASE_URL"))
        .unwrap_or_else(|| DEFAULT_FIREWORKS_BASE_URL.to_string());
    let fireworks_model = setting("FIREWORK_MODEL")
        .or_else(|| setting("FIREWORKS_MODEL"))
        .unwrap_or_else(|| DEFAULT_FIREWORKS_MODEL.to_string());

    Ok(Settings {
        project_root,
        raw_dir: data_dir.join("raw"),
        cache_dir: data_dir.join("cache"),
        data_dir,
        user_agent: string_setting("DEADLOCK_BRAIN_USER_AGENT", DEFAULT_USER_AGENT),
        sheet_id: string_setting("DEADLOCK_STATS_SHEET_ID", DEFAULT_SHEET_ID),
        sheet_gid: string_setting("DEADLOCK_STATS_SHEET_GID", "0"),
        wiki_enabled: bool_setting("DEADLOCK_BRAIN_WIKI_ENABLED", false),
        wiki_min_delay_seconds: f64_setting("DEADLOCK_BRAIN_WIKI_MIN_DELAY_SECONDS", 5.0)?,
        wiki_cache_ttl_seconds: u64_setting(
            "DEADLOCK_BRAIN_WIKI_CACHE_TTL_SECONDS",
            604_800,
        )?,
        ai_api_key: fireworks_api_key,
        ai_base_url: fireworks_base_url.trim_end_matches('/').to_string(),
        ai_model: fireworks_model,
        ai_timeout_seconds: u64_setting("FIREWORKS_TIMEOUT_SECONDS", 300)?,
        ai_max_completion_tokens: u64_setting("FIREWORKS_MAX_TOKENS", 16_000)?,
        ai_temperature: f64_setting("FIREWORKS_TEMPERATURE", 0.2)?,
        ai_top_p: f64_setting("FIREWORKS_TOP_P", 0.9)?,
        ai_use_token_plan: false,
    })
}

pub fn path_env(name: &'static str, default: PathBuf) -> PathBuf {
    env::var_os(name)
        .map(PathBuf::from)
        .filter(|value| !value.as_os_str().is_empty())
        .unwrap_or(default)
}

fn string_setting(name: &'static str, default: &str) -> String {
    setting(name).unwrap_or_else(|| default.to_string())
}

fn bool_setting(name: &'static str, default: bool) -> bool {
    setting(name)
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(default)
}

fn u64_setting(name: &'static str, default: u64) -> Result<u64> {
    match setting(name) {
        Some(value) => value
            .parse::<u64>()
            .map_err(|error| CoreError::invalid_integer_env(name, value, error)),
        None => Ok(default),
    }
}

fn f64_setting(name: &'static str, default: f64) -> Result<f64> {
    match setting(name) {
        Some(value) => value
            .parse::<f64>()
            .map_err(|error| CoreError::invalid_float_env(name, value, error)),
        None => Ok(default),
    }
}

fn setting(name: &'static str) -> Option<String> {
    env::var(name)
        .ok()
        .filter(|value| !value.trim().is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repo_root_points_at_project() {
        let root = repo_root();
        assert!(root.join("rust").is_dir());
    }

    #[test]
    fn repo_root_prefers_runtime_root_env() {
        let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        // Ein gesetztes, existierendes DEADLOCK_BRAIN_ROOT gewinnt gegen den
        // eingebrannten Build-Pfad (Relokation nach Worktree-Bereinigung).
        let tmp = std::env::temp_dir();
        assert_eq!(repo_root_resolve(Some(tmp.clone()), manifest.clone()), tmp);
        // Ein ungueltiger Root faellt auf die Build-Zeit-Aufloesung zurueck.
        let bogus = tmp.join("deadlock-brain-nonexistent-root-xyz");
        let resolved = repo_root_resolve(Some(bogus), manifest.clone());
        assert!(resolved.join("rust").is_dir());
    }
}
