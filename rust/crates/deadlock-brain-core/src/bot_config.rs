//! Globaler, unveränderlicher Konfigurationsvertrag. Keine Zugangsdaten und keine ENV-Leser.
//! Alle internen relativen Pfade beziehen sich auf das Verzeichnis der realen TOML-Datei.
//! Änderungen erfordern einen Neustart; dieser Lader verändert weder Dateien noch Clients.

use std::{
    fmt,
    fs::File,
    io::Read,
    path::{Component, Path, PathBuf},
    sync::Arc,
};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

pub const DEFAULT_CONFIG_PATH: &str = "/home/nathanael/repos/Deadlock-Brain/config/bot.toml";
const MAX_CONFIG_BYTES: u64 = 128 * 1024;

/// Nur feste Feldnamen und feste Fehlertexte. Auch Debug darf niemals Parserinput zeigen.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("Brain-Konfiguration: {field}: {message}")]
pub struct ConfigError {
    pub field: &'static str,
    pub message: &'static str,
}

type Result<T> = std::result::Result<T, ConfigError>;

fn invalid(field: &'static str, message: &'static str) -> ConfigError {
    ConfigError { field, message }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Document {
    schema_version: u32,
    paths: Paths,
    http: Http,
    sheet: Sheet,
    wiki: Wiki,
    ai: Ai,
    builds: Builds,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Paths {
    pub project_root: PathBuf,
    pub data_dir: PathBuf,
    pub game_wiki_dir: PathBuf,
    pub reasoner_seed_dir: PathBuf,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Http {
    pub user_agent: String,
    pub timeout_seconds: u64,
    pub retry_attempts: usize,
    pub retry_backoff_milliseconds: u64,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Sheet {
    pub id: String,
    pub gid: String,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Wiki {
    pub enabled: bool,
    pub min_delay_seconds: f64,
    pub cache_ttl_seconds: u64,
}

#[derive(Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Provider {
    Fireworks,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Ai {
    pub provider: Provider,
    pub base_url: String,
    /// Bewusster Pin. Kein Pin bedeutet automatische Auswahl, niemals einen Kompilat-Alias.
    pub pin: Option<String>,
    pub timeout_seconds: u64,
    pub max_completion_tokens: u64,
    pub temperature: f64,
    pub top_p: f64,
    pub selection: Selection,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Selection {
    pub catalog_url: String,
    pub check_interval_seconds: u64,
    pub last_good_ttl_seconds: u64,
    pub request_timeout_seconds: u64,
    pub retry_attempts: usize,
    pub retry_backoff_milliseconds: u64,
    pub page_size: u16,
    pub max_pages: u16,
    pub probe_budget_seconds: u64,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Builds {
    pub min_matches: i64,
    pub min_prevalence_builds: i64,
}

/// Zugriff nur über unveränderliche Referenzen. Arc-Klone teilen dieselbe validierte Momentaufnahme.
#[derive(Clone)]
pub struct BotConfig {
    document: Document,
    path: PathBuf,
    fingerprint: String,
}

impl fmt::Debug for BotConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BotConfig")
            .field("schema_version", &self.document.schema_version)
            .field("fingerprint", &self.fingerprint)
            .finish_non_exhaustive()
    }
}

impl BotConfig {
    /// Keine Verzeichniserstellung, keine Netzwerkverbindung und kein Secret-Zugriff.
    pub fn load(path: &Path) -> Result<Arc<Self>> {
        if !path.is_absolute() {
            return Err(invalid(
                "config",
                "ein absoluter Dateipfad ist erforderlich",
            ));
        }
        let path = path.canonicalize().map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                invalid("config", "Datei fehlt")
            } else {
                invalid("config", "Datei ist nicht lesbar")
            }
        })?;
        let file = File::open(&path).map_err(|_| invalid("config", "Datei ist nicht lesbar"))?;
        if !file
            .metadata()
            .map_err(|_| invalid("config", "Datei ist nicht lesbar"))?
            .is_file()
        {
            return Err(invalid("config", "eine reguläre Datei ist erforderlich"));
        }
        let mut bytes = Vec::new();
        file.take(MAX_CONFIG_BYTES + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| invalid("config", "Datei ist nicht lesbar"))?;
        if bytes.len() as u64 > MAX_CONFIG_BYTES {
            return Err(invalid("config", "Datei überschreitet 128 KiB"));
        }
        let text =
            std::str::from_utf8(&bytes).map_err(|_| invalid("config", "UTF-8 erforderlich"))?;
        Self::parse(text, &path).map(Arc::new)
    }

    fn parse(text: &str, path: &Path) -> Result<Self> {
        // toml::de::Error enthält den Originaltext. Nie als source, Display oder Debug weiterreichen.
        let mut document: Document = toml::from_str(text).map_err(|_| {
            invalid(
                "config",
                "TOML-Syntax, Feldname, Pflichtfeld oder Feldtyp ungültig",
            )
        })?;
        validate(&document)?;
        let base = path
            .parent()
            .ok_or_else(|| invalid("config", "Dateipfad ungültig"))?;
        document.paths.project_root = resolve(base, &document.paths.project_root)?;
        document.paths.data_dir = resolve(base, &document.paths.data_dir)?;
        document.paths.game_wiki_dir = resolve(base, &document.paths.game_wiki_dir)?;
        document.paths.reasoner_seed_dir = resolve(base, &document.paths.reasoner_seed_dir)?;
        let canonical = serde_json::to_vec(&document)
            .map_err(|_| invalid("config", "Konfiguration kann nicht normalisiert werden"))?;
        let fingerprint = hex::encode(Sha256::digest(canonical));
        Ok(Self {
            document,
            path: path.to_path_buf(),
            fingerprint,
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
    pub fn fingerprint(&self) -> &str {
        &self.fingerprint
    }
    pub fn paths(&self) -> &Paths {
        &self.document.paths
    }
    pub fn http(&self) -> &Http {
        &self.document.http
    }
    pub fn sheet(&self) -> &Sheet {
        &self.document.sheet
    }
    pub fn wiki(&self) -> &Wiki {
        &self.document.wiki
    }
    pub fn ai(&self) -> &Ai {
        &self.document.ai
    }
    pub fn builds(&self) -> &Builds {
        &self.document.builds
    }

    /// Keine Rohwerte frei beschreibbarer Textfelder, keine Zugangsdaten, keine DSNs.
    pub fn redacted_status(&self) -> Value {
        json!({
            "schema_version": self.document.schema_version,
            "fingerprint": self.fingerprint,
            "reload": "restart_required",
            "paths": {
                "project_root_sha256": digest_path(&self.paths().project_root),
                "data_dir_sha256": digest_path(&self.paths().data_dir),
                "game_wiki_dir_sha256": digest_path(&self.paths().game_wiki_dir),
                "reasoner_seed_dir_sha256": digest_path(&self.paths().reasoner_seed_dir)
            },
            "http": {
                "user_agent_sha256": hex::encode(Sha256::digest(self.http().user_agent.as_bytes())),
                "timeout_seconds": self.http().timeout_seconds,
                "retry_attempts": self.http().retry_attempts,
                "retry_backoff_milliseconds": self.http().retry_backoff_milliseconds
            },
            "sheet": {
                "id_sha256": hex::encode(Sha256::digest(self.sheet().id.as_bytes())),
                "gid_sha256": hex::encode(Sha256::digest(self.sheet().gid.as_bytes()))
            },
            "wiki": {
                "enabled": self.wiki().enabled,
                "min_delay_seconds": self.wiki().min_delay_seconds,
                "cache_ttl_seconds": self.wiki().cache_ttl_seconds
            },
            "ai": {
                "provider": "fireworks",
                "pinned": self.ai().pin.is_some(),
                "pin_sha256": self.ai().pin.as_ref().map(|v| hex::encode(Sha256::digest(v.as_bytes()))),
                "timeout_seconds": self.ai().timeout_seconds,
                "max_completion_tokens": self.ai().max_completion_tokens,
                "temperature": self.ai().temperature,
                "top_p": self.ai().top_p,
                "selection": self.ai().selection
            },
            "builds": {
                "min_matches": self.builds().min_matches,
                "min_prevalence_builds": self.builds().min_prevalence_builds
            }
        })
    }
}

fn digest_path(path: &Path) -> String {
    hex::encode(Sha256::digest(path.as_os_str().as_encoded_bytes()))
}

fn resolve(base: &Path, value: &Path) -> Result<PathBuf> {
    if value.as_os_str().is_empty() {
        return Err(invalid("paths", "leere Pfade sind unzulässig"));
    }
    let joined = if value.is_absolute() {
        value.to_path_buf()
    } else {
        base.join(value)
    };
    let mut normalized = PathBuf::new();
    for part in joined.components() {
        match part {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            component => normalized.push(component.as_os_str()),
        }
    }
    Ok(normalized)
}

fn range(value: u64, min: u64, max: u64, field: &'static str) -> Result<()> {
    if !(min..=max).contains(&value) {
        return Err(invalid(field, "Wert außerhalb des zulässigen Bereichs"));
    }
    Ok(())
}

fn endpoint(value: &str, expected: &str, field: &'static str) -> Result<()> {
    let url = reqwest::Url::parse(value).map_err(|_| invalid(field, "URL ungültig"))?;
    if !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || value.trim_end_matches('/') != expected
    {
        return Err(invalid(
            field,
            "nur der freigegebene Endpunkt ohne Zugangsdaten ist zulässig",
        ));
    }
    Ok(())
}

fn validate(d: &Document) -> Result<()> {
    if d.schema_version != 1 {
        return Err(invalid("schema_version", "nur Version 1 wird unterstützt"));
    }
    for p in [
        &d.paths.project_root,
        &d.paths.data_dir,
        &d.paths.game_wiki_dir,
        &d.paths.reasoner_seed_dir,
    ] {
        if p.as_os_str().is_empty() || p.to_string_lossy().contains(['\0', '\n', '\r']) {
            return Err(invalid(
                "paths",
                "leere Pfade und Steuerzeichen sind unzulässig",
            ));
        }
    }
    if d.http.user_agent.is_empty()
        || d.http.user_agent.len() > 256
        || !d.http.user_agent.bytes().all(|c| (32..=126).contains(&c))
    {
        return Err(invalid(
            "http.user_agent",
            "1 bis 256 druckbare ASCII-Zeichen erforderlich",
        ));
    }
    range(d.http.timeout_seconds, 1, 3600, "http.timeout_seconds")?;
    range(d.http.retry_attempts as u64, 1, 10, "http.retry_attempts")?;
    range(
        d.http.retry_backoff_milliseconds,
        1,
        60_000,
        "http.retry_backoff_milliseconds",
    )?;
    if d.sheet.id.is_empty()
        || d.sheet.id.len() > 128
        || !d
            .sheet
            .id
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || c == b'_' || c == b'-')
    {
        return Err(invalid("sheet.id", "Google-Sheet-ID ungültig"));
    }
    if d.sheet.gid.is_empty()
        || !d.sheet.gid.bytes().all(|c| c.is_ascii_digit())
        || d.sheet.gid.parse::<u64>().is_err()
    {
        return Err(invalid("sheet.gid", "numerische Tabellen-ID erforderlich"));
    }
    if !d.wiki.min_delay_seconds.is_finite() || !(0.1..=3600.0).contains(&d.wiki.min_delay_seconds)
    {
        return Err(invalid(
            "wiki.min_delay_seconds",
            "0,1 bis 3600 Sekunden erforderlich",
        ));
    }
    range(
        d.wiki.cache_ttl_seconds,
        1,
        31_536_000,
        "wiki.cache_ttl_seconds",
    )?;
    endpoint(
        &d.ai.base_url,
        "https://api.fireworks.ai/inference/v1",
        "ai.base_url",
    )?;
    if let Some(pin) = &d.ai.pin {
        if !valid_flash_pin(pin) {
            return Err(invalid(
                "ai.pin",
                "explizite stabile DeepSeek-Flash-Modell-ID erforderlich",
            ));
        }
    }
    range(d.ai.timeout_seconds, 1, 3600, "ai.timeout_seconds")?;
    range(
        d.ai.max_completion_tokens,
        1,
        131_072,
        "ai.max_completion_tokens",
    )?;
    if !d.ai.temperature.is_finite() || !(0.0..=2.0).contains(&d.ai.temperature) {
        return Err(invalid("ai.temperature", "0 bis 2 erforderlich"));
    }
    if !d.ai.top_p.is_finite() || !(0.0..=1.0).contains(&d.ai.top_p) || d.ai.top_p == 0.0 {
        return Err(invalid("ai.top_p", "größer 0 bis 1 erforderlich"));
    }
    let s = &d.ai.selection;
    endpoint(
        &s.catalog_url,
        "https://api.fireworks.ai/v1/accounts/fireworks/models",
        "ai.selection.catalog_url",
    )?;
    range(
        s.check_interval_seconds,
        60,
        604_800,
        "ai.selection.check_interval_seconds",
    )?;
    range(
        s.last_good_ttl_seconds,
        60,
        2_592_000,
        "ai.selection.last_good_ttl_seconds",
    )?;
    if s.last_good_ttl_seconds < s.check_interval_seconds {
        return Err(invalid(
            "ai.selection.last_good_ttl_seconds",
            "darf das Prüfintervall nicht unterschreiten",
        ));
    }
    range(
        s.request_timeout_seconds,
        1,
        300,
        "ai.selection.request_timeout_seconds",
    )?;
    range(s.retry_attempts as u64, 1, 5, "ai.selection.retry_attempts")?;
    range(
        s.retry_backoff_milliseconds,
        1,
        60_000,
        "ai.selection.retry_backoff_milliseconds",
    )?;
    range(s.page_size.into(), 1, 200, "ai.selection.page_size")?;
    range(s.max_pages.into(), 1, 1000, "ai.selection.max_pages")?;
    range(
        s.probe_budget_seconds,
        1,
        3600,
        "ai.selection.probe_budget_seconds",
    )?;
    for (value, field) in [
        (d.builds.min_matches, "builds.min_matches"),
        (
            d.builds.min_prevalence_builds,
            "builds.min_prevalence_builds",
        ),
    ] {
        if !(0..=1_000_000_000).contains(&value) {
            return Err(invalid(field, "0 bis 1000000000 erforderlich"));
        }
    }
    Ok(())
}

/// Syntaxprüfung für bewusste Pins. Keine Erfindung oder Konstruktion von Modell-IDs.
/// Datierte Snapshots sind als Pin zulässig, aber ihr Datum ist keine Familienversion.
pub(crate) fn valid_flash_pin(id: &str) -> bool {
    let Some(rest) = id.strip_prefix("accounts/fireworks/models/deepseek-v") else {
        return false;
    };
    let Some((version, suffix)) = rest.split_once("-flash") else {
        return false;
    };
    let version_ok = !version.is_empty()
        && version.split(['.', 'p']).all(|part| {
            !part.is_empty()
                && part.bytes().all(|c| c.is_ascii_digit())
                && part.parse::<u32>().is_ok()
        });
    version_ok
        && (suffix.is_empty()
            || suffix.strip_prefix('-').is_some_and(|s| {
                !s.is_empty() && s.len() <= 14 && s.bytes().all(|c| c.is_ascii_digit())
            }))
}

#[cfg(test)]
#[path = "bot_config_tests.rs"]
mod tests;
