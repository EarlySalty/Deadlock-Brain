use std::{fs, path::PathBuf};

use serde::Deserialize;

use crate::{CoreError, Result};

pub const DEFAULT_USER_AGENT: &str = "DeadlockBrain/0.1 contact=admin@earlysalty.com";
pub const DEFAULT_SHEET_ID: &str = "1fj9XMQmVUY0FY4cbozvB18PMBnpbdsaMFTZRLa74VRY";

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
    // Diese Felder bleiben für bestehende Aufrufer leer. Der Abo-Zugang nutzt keine API-Schlüssel.
    pub ai_api_key: Option<String>,
    pub ai_base_url: String,
    pub ai_model: String,
    pub ai_timeout_seconds: u64,
    pub ai_max_completion_tokens: u64,
    pub ai_temperature: f64,
    pub ai_top_p: f64,
    pub ai_use_token_plan: bool,
}

impl std::fmt::Debug for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Settings")
            .field("ai_api_key", &"<redacted>")
            .field("ai_model", &self.ai_model)
            .finish_non_exhaustive()
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiSettings {
    pub provider: String,
    pub cli_path: PathBuf,
    pub model: String,
    pub timeout_seconds: u64,
    pub max_response_bytes: usize,
    pub ai_max_completion_tokens: u64,
    // Kompatfelder des bisherigen Vertrags; die Abo-CLI bietet keine Samplingsteuerung.
    pub ai_temperature: f64,
    pub ai_top_p: f64,
    pub ai_use_token_plan: bool,
    pub ai_reasoning_effort: Option<String>,
}

pub fn load_ai_settings() -> Result<AiSettings> {
    Ok(load_bot_settings(&repo_root())?.ai)
}

fn validate_ai_settings(settings: &AiSettings) -> Result<()> {
    if settings.provider != "openai_chatgpt_subscription"
        || !settings.cli_path.is_absolute()
        || settings.model != "gpt-6-luna"
        || settings.timeout_seconds == 0
        || settings.max_response_bytes == 0
        || !settings.ai_temperature.is_finite()
        || !(0.0..=2.0).contains(&settings.ai_temperature)
        || !settings.ai_top_p.is_finite()
        || !(0.0..=1.0).contains(&settings.ai_top_p)
        || settings.ai_use_token_plan
        || settings
            .ai_reasoning_effort
            .as_deref()
            .is_some_and(|effort| {
                !["none", "minimal", "low", "medium", "high", "xhigh"].contains(&effort)
            })
    {
        return Err(CoreError::ModelSelection(
            "Ungültige Abo-Konfiguration in config/bot.toml".into(),
        ));
    }
    Ok(())
}

#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct DataSettings {
    data_dir: Option<PathBuf>,
    user_agent: Option<String>,
    sheet_id: Option<String>,
    sheet_gid: Option<String>,
    wiki_enabled: Option<bool>,
    wiki_min_delay_seconds: Option<f64>,
    wiki_cache_ttl_seconds: Option<u64>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BotSettings {
    ai: AiSettings,
    infisical: crate::pg::InfisicalConfig,
    settings: DataSettings,
}

fn load_bot_settings(project_root: &std::path::Path) -> Result<BotSettings> {
    let text = fs::read_to_string(project_root.join("config/bot.toml"))?;
    let config: BotSettings = toml::from_str(&text).map_err(|_| {
        CoreError::ModelSelection("Ungültige Konfiguration in config/bot.toml".into())
    })?;
    validate_ai_settings(&config.ai)?;
    if !development_settings_allowed(project_root)
        && (!config
            .settings
            .data_dir
            .as_ref()
            .is_some_and(|path| path.is_absolute())
            || config
                .settings
                .user_agent
                .as_ref()
                .is_none_or(|value| value.trim().is_empty())
            || config
                .settings
                .sheet_id
                .as_ref()
                .is_none_or(|value| value.trim().is_empty())
            || config
                .settings
                .sheet_gid
                .as_ref()
                .is_none_or(|value| value.trim().is_empty())
            || config.settings.wiki_enabled.is_none()
            || config
                .settings
                .wiki_min_delay_seconds
                .is_none_or(|value| !value.is_finite() || value < 0.0)
            || config.settings.wiki_cache_ttl_seconds.is_none())
    {
        return Err(CoreError::ModelSelection(
            "Installierte Lernjobs benötigen vollständige settings und einen absoluten data_dir in config/bot.toml"
                .into(),
        ));
    }
    Ok(config)
}

pub(crate) fn load_infisical_settings(
    path: &std::path::Path,
) -> Result<crate::pg::InfisicalConfig> {
    let root = path
        .parent()
        .and_then(std::path::Path::parent)
        .ok_or_else(|| CoreError::ModelSelection("Ungültiger Bot-TOML-Pfad".into()))?;
    if path != root.join("config/bot.toml") {
        return Err(CoreError::ModelSelection("Ungültiger Bot-TOML-Pfad".into()));
    }
    Ok(load_bot_settings(root)?.infisical)
}

pub fn repo_root() -> PathBuf {
    if let Ok(executable) = std::env::current_exe() {
        if executable.starts_with("/opt/deadlock-brain") {
            if let Some(root) = installed_release_root(&executable) {
                return root;
            }
        }
        for ancestor in executable.ancestors().skip(1) {
            if development_settings_allowed(ancestor) {
                return ancestor.to_path_buf();
            }
        }
    }
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for ancestor in manifest_dir.ancestors() {
        if ancestor.join("data").is_dir() && ancestor.join("src/deadlock_brain").is_dir() {
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

fn installed_release_root(executable: &std::path::Path) -> Option<PathBuf> {
    let parent = executable.parent()?;
    if parent.file_name().is_some_and(|name| name == "bin") {
        parent.parent().map(std::path::Path::to_path_buf)
    } else {
        Some(parent.to_path_buf())
    }
}

pub fn default_data_dir() -> PathBuf {
    repo_root().join("data")
}

fn development_settings_allowed(project_root: &std::path::Path) -> bool {
    !project_root.starts_with("/opt/deadlock-brain")
        && project_root.join(".git").exists()
        && project_root.join("rust/Cargo.toml").is_file()
        && project_root.join("src/deadlock_brain").is_dir()
}

pub fn load_settings() -> Result<Settings> {
    let project_root = repo_root();
    let data_settings = load_bot_settings(&project_root)?.settings;
    let ai = load_ai_settings()?;
    let data_dir = data_settings
        .data_dir
        .unwrap_or_else(|| project_root.join("data"));
    let data_dir = if data_dir.is_absolute() {
        data_dir
    } else {
        project_root.join(data_dir)
    };
    Ok(Settings {
        project_root,
        raw_dir: data_dir.join("raw"),
        cache_dir: data_dir.join("cache"),
        data_dir,
        user_agent: data_settings
            .user_agent
            .unwrap_or_else(|| DEFAULT_USER_AGENT.into()),
        sheet_id: data_settings
            .sheet_id
            .unwrap_or_else(|| DEFAULT_SHEET_ID.into()),
        sheet_gid: data_settings.sheet_gid.unwrap_or_else(|| "0".into()),
        wiki_enabled: data_settings.wiki_enabled.unwrap_or(false),
        wiki_min_delay_seconds: data_settings.wiki_min_delay_seconds.unwrap_or(5.0),
        wiki_cache_ttl_seconds: data_settings.wiki_cache_ttl_seconds.unwrap_or(604_800),
        ai_api_key: None,
        ai_base_url: String::new(),
        ai_model: ai.model,
        ai_timeout_seconds: ai.timeout_seconds,
        ai_max_completion_tokens: ai.ai_max_completion_tokens,
        ai_temperature: ai.ai_temperature,
        ai_top_p: ai.ai_top_p,
        ai_use_token_plan: ai.ai_use_token_plan,
    })
}

// Bestehende Pfadaufrufer behalten ihre Standards, ohne Umgebungsvariablen zu lesen.
pub fn path_env(_name: &'static str, default: PathBuf) -> PathBuf {
    default
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn installed_release_root_never_uses_build_source_or_current_directory() {
        assert_eq!(
            installed_release_root(std::path::Path::new(
                "/opt/deadlock-brain/maintenance-releases/revision/deadlock-brain"
            )),
            Some(PathBuf::from(
                "/opt/deadlock-brain/maintenance-releases/revision"
            ))
        );
        assert_eq!(
            installed_release_root(std::path::Path::new(
                "/opt/deadlock-brain/releases/revision/bin/deadlock-brain"
            )),
            Some(PathBuf::from("/opt/deadlock-brain/releases/revision"))
        );
    }

    #[test]
    fn repo_root_points_at_project() {
        assert!(repo_root().join("rust").is_dir());
    }

    #[test]
    fn missing_job_settings_fail_outside_proven_development_checkout() {
        let root = tempfile::tempdir().unwrap();
        for job in ["sheet-sync", "build-data"] {
            let job_root = root.path().join("legacy").join(job);
            fs::create_dir_all(job_root.join("config")).unwrap();
            assert!(load_bot_settings(&job_root).is_err());
            fs::write(job_root.join("config/bot.toml"), b"invalid-toml").unwrap();
            assert!(load_bot_settings(&job_root).is_err());
            let template = if job == "sheet-sync" {
                include_str!("../../../../ops/luna-abo/sheet-sync.bot.toml")
            } else {
                include_str!("../../../../ops/luna-abo/build-data.bot.toml")
            };
            fs::write(job_root.join("config/bot.toml"), template).unwrap();
            let config = load_bot_settings(&job_root).unwrap();
            let expected = if job == "sheet-sync" {
                "/home/nathanael/repos/Deadlock-Brain/data"
            } else {
                "/home/nathanael/.worktrees/brain-live-main/data"
            };
            assert_eq!(config.settings.data_dir, Some(PathBuf::from(expected)));
            let incomplete = template
                .lines()
                .filter(|line| !line.starts_with("sheet_id ="))
                .collect::<Vec<_>>()
                .join("\n");
            fs::write(job_root.join("config/bot.toml"), incomplete).unwrap();
            assert!(load_infisical_settings(&job_root.join("config/bot.toml")).is_err());
            fs::write(
                job_root.join("config/bot.toml"),
                template.replace(expected, "data"),
            )
            .unwrap();
            assert!(load_bot_settings(&job_root).is_err());
            fs::remove_file(job_root.join("config/bot.toml")).unwrap();
            fs::create_dir(job_root.join("config/bot.toml")).unwrap();
            assert!(load_bot_settings(&job_root).is_err());
        }
        assert!(!development_settings_allowed(std::path::Path::new(
            "/opt/deadlock-brain/maintenance-releases/revision/legacy/sheet-sync"
        )));
        assert!(development_settings_allowed(&repo_root()));
    }

    #[test]
    fn ai_configuration_selects_subscription_luna() {
        let settings = load_ai_settings().unwrap();
        assert_eq!(settings.provider, "openai_chatgpt_subscription");
        assert_eq!(settings.model, "gpt-6-luna");
    }
}
