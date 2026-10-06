//! Policy-bound model selection. No ENV, implicit aliases, or provider changes.
//! Catalog upload/update time is metadata, never a model-version ordering key.

use std::{sync::{Arc, Mutex, OnceLock}, time::{SystemTime, UNIX_EPOCH}};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::bot_config::BotConfig;

#[path = "model_catalog.rs"]
mod catalog;
#[path = "model_probes.rs"]
mod probes;
#[path = "model_store.rs"]
mod store;
#[path = "model_transport.rs"]
mod transport;

const PROBE_REVISION: &str = "brain-config-probes-v1-text-json-tools-reasoning-domain";
static REFRESH_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, thiserror::Error)]
#[serde(rename_all = "snake_case")]
pub enum SelectionError {
    #[error("Modellprüfung: Zeitbudget überschritten")]
    Timeout,
    #[error("Modellprüfung: Anbieter-Zugriff nicht authentifiziert (401)")]
    Unauthorized,
    #[error("Modellprüfung: Anbieter-Zugriff nicht freigegeben (403)")]
    Forbidden,
    #[error("Modellprüfung: Anbieter-Limit (429)")]
    RateLimited,
    #[error("Modellprüfung: Anbieter vorübergehend nicht verfügbar")]
    ProviderUnavailable,
    #[error("Modellprüfung: Transportfehler; Antwortdetails werden nicht ausgegeben")]
    Transport,
    #[error("Modellprüfung: ungültige oder unvollständige Katalogdaten")]
    CatalogInvalid,
    #[error("Modellprüfung: kein freigegebener verfügbarer Katalogkandidat")]
    NoCandidate,
    #[error("Modellprüfung: Funktionsprobe nicht bestanden")]
    ProbeFailed,
    #[error("Modellprüfung: Anfrage widerspricht dem zentralen Pin")]
    PinConflict,
    #[error("Modellprüfung: Prüfung läuft bereits; kein gültiger Rückfall verfügbar")]
    Busy,
    #[error("Modellprüfung: PostgreSQL-Laufzeitstatus nicht verfügbar")]
    StateUnavailable,
    #[error("Modellprüfung: ungültige Uhrzeit")]
    Clock,
}

type Result<T> = std::result::Result<T, SelectionError>;

#[derive(Clone)]
pub struct Policy {
    pub(crate) config: Arc<BotConfig>,
    pub(crate) fingerprint: String,
    pub(crate) pin: Option<String>,
}

impl std::fmt::Debug for Policy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ModelPolicy")
            .field("fingerprint", &self.fingerprint)
            .field("pinned", &self.pin.is_some())
            .finish()
    }
}

impl Policy {
    pub fn new(config: Arc<BotConfig>, requested_pin: Option<&str>, credential: &str) -> Result<Self> {
        let requested_pin = requested_pin.filter(|s| !s.is_empty());
        if let (Some(global), Some(requested)) = (&config.ai().pin, requested_pin) {
            if global != requested {
                return Err(SelectionError::PinConflict);
            }
        }
        let pin = config.ai().pin.as_deref().or(requested_pin).map(str::to_owned);
        if pin.as_ref().is_some_and(|id| !crate::bot_config::valid_flash_pin(id)) {
            return Err(SelectionError::PinConflict);
        }
        // A rotated/different credential invalidates cached account availability.
        // Only this composite digest is persisted; neither credentials nor a separate
        // credential digest are serialized or logged.
        let mut hasher = Sha256::new();
        hasher.update(PROBE_REVISION.as_bytes());
        hasher.update(include_bytes!("model_probes.rs"));
        hasher.update(include_bytes!("model_catalog.rs"));
        hasher.update([0]);
        hasher.update(config.fingerprint().as_bytes());
        hasher.update([0]);
        hasher.update(pin.as_deref().unwrap_or_default().as_bytes());
        hasher.update([0]);
        hasher.update(credential.as_bytes());
        let fingerprint = hex::encode(hasher.finalize());
        Ok(Self { config, fingerprint, pin })
    }

    pub fn fingerprint(&self) -> &str { &self.fingerprint }

    fn accepts(&self, model: &VerifiedModel, now: u64) -> bool {
        let Some(version) = numeric_version(&model.model) else { return false; };
        model.policy_fingerprint == self.fingerprint
            && version == model.version
            && model.checked_at <= now
            && model.expires_at > now
            && model.expires_at <= model.checked_at.saturating_add(self.config.ai().selection.last_good_ttl_seconds)
            && self.pin.as_ref().is_none_or(|pin| pin == &model.model)
            && model.probe_revision == PROBE_REVISION
    }
}

/// Numeric family version, normalizing trailing zeros. A date after '-flash'
/// is a snapshot identifier and is intentionally absent from this version.
pub fn numeric_version(model: &str) -> Option<Vec<u32>> {
    if !crate::bot_config::valid_flash_pin(model) { return None; }
    let text = model.strip_prefix("accounts/fireworks/models/deepseek-v")?
        .split_once("-flash")?.0;
    let mut version = text.split(['.', 'p']).map(str::parse::<u32>).collect::<std::result::Result<Vec<_>, _>>().ok()?;
    if version.first().copied().unwrap_or(0) == 0 { return None; }
    while version.len() > 1 && version.last() == Some(&0) { version.pop(); }
    Some(version)
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct VerifiedModel {
    pub model: String,
    pub version: Vec<u32>,
    pub policy_fingerprint: String,
    pub checked_at: u64,
    pub expires_at: u64,
    pub catalog_created_at: Option<u64>,
    pub probe_revision: String,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SelectionReason { Pin, NumericUpgrade, InitialCatalog, Reverified, LastGood, IntervalCache }

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub schema_version: u32,
    pub last_good: Option<VerifiedModel>,
    pub last_check_at: u64,
    pub next_check_at: u64,
    pub reason: SelectionReason,
    pub last_error: Option<SelectionError>,
}

#[derive(Clone, Debug)]
pub struct Resolution {
    pub verified: VerifiedModel,
    pub reason: SelectionReason,
    pub last_error: Option<SelectionError>,
}

#[derive(Clone, Debug)]
pub(crate) struct Candidate {
    model: String,
    version: Vec<u32>,
    created_at: u64,
}

pub(crate) trait Network {
    fn catalog(&mut self, policy: &Policy, now: u64) -> Result<Vec<Candidate>>;
    fn probe(&mut self, policy: &Policy, model: &str) -> Result<()>;
}

pub(crate) trait Store {
    fn load(&mut self, policy: &Policy) -> Result<Option<State>>;
    /// An advisory transaction lock spans the complete check and atomic write.
    fn try_lock(&mut self, policy: &Policy) -> Result<bool>;
    fn save(&mut self, policy: &Policy, state: &State) -> Result<()>;
}

fn cached(state: &State, policy: &Policy, now: u64, reason: SelectionReason) -> Option<Resolution> {
    if state.schema_version != 1 || state.last_check_at > now { return None; }
    state.last_good.as_ref().filter(|m| policy.accepts(m, now)).map(|verified| Resolution {
        verified: verified.clone(), reason, last_error: state.last_error,
    })
}

pub(crate) fn select(store: &mut impl Store, network: &mut impl Network, policy: &Policy, now: u64) -> Result<Resolution> {
    let previous = store.load(policy)?;
    if let Some(state) = &previous {
        if state.next_check_at <= state.last_check_at.saturating_add(policy.config.ai().selection.check_interval_seconds)
            && now < state.next_check_at
        {
            if let Some(hit) = cached(state, policy, now, SelectionReason::IntervalCache) { return Ok(hit); }
        }
    }
    if !store.try_lock(policy)? {
        return previous.as_ref().and_then(|s| cached(s, policy, now, SelectionReason::LastGood)).ok_or(SelectionError::Busy);
    }
    // Another process may have finished between the first read and the lock.
    let previous = store.load(policy)?.or(previous);
    if let Some(state) = &previous {
        if state.next_check_at <= state.last_check_at.saturating_add(policy.config.ai().selection.check_interval_seconds)
            && now < state.next_check_at
        {
            if let Some(hit) = cached(state, policy, now, SelectionReason::IntervalCache) { return Ok(hit); }
        }
    }
    let old_model = previous.as_ref().filter(|s| s.schema_version == 1)
        .and_then(|s| s.last_good.as_ref())
        .filter(|m| m.policy_fingerprint == policy.fingerprint && m.probe_revision == PROBE_REVISION
            && numeric_version(&m.model).as_ref() == Some(&m.version) && m.checked_at <= now);
    let check = (|| {
        let (candidate, reason) = if let Some(pin) = &policy.pin {
            (Candidate { model: pin.clone(), version: numeric_version(pin).ok_or(SelectionError::PinConflict)?, created_at: 0 }, SelectionReason::Pin)
        } else {
            let mut candidates = network.catalog(policy, now)?;
            candidates.retain(|c| old_model.is_none_or(|old| c.version > old.version || c.model == old.model));
            candidates.sort_by(|a, b| b.version.cmp(&a.version)
                .then_with(|| a.created_at.cmp(&b.created_at))
                .then_with(|| a.model.cmp(&b.model)));
            let candidate = candidates.into_iter().next().ok_or(SelectionError::NoCandidate)?;
            let reason = match old_model {
                Some(old) if candidate.version > old.version => SelectionReason::NumericUpgrade,
                Some(_) => SelectionReason::Reverified,
                None => SelectionReason::InitialCatalog,
            };
            (candidate, reason)
        };
        network.probe(policy, &candidate.model)?;
        Ok((VerifiedModel {
            model: candidate.model,
            version: candidate.version,
            policy_fingerprint: policy.fingerprint.clone(),
            checked_at: now,
            expires_at: now.checked_add(policy.config.ai().selection.last_good_ttl_seconds).ok_or(SelectionError::Clock)?,
            catalog_created_at: (candidate.created_at > 0).then_some(candidate.created_at),
            probe_revision: PROBE_REVISION.to_owned(),
        }, reason))
    })();
    let next_check_at = now.checked_add(policy.config.ai().selection.check_interval_seconds).ok_or(SelectionError::Clock)?;
    match check {
        Ok((verified, reason)) => {
            store.save(policy, &State { schema_version: 1, last_good: Some(verified.clone()), last_check_at: now, next_check_at, reason, last_error: None })?;
            Ok(Resolution { verified, reason, last_error: None })
        }
        Err(error) => {
            let fallback = previous.as_ref().and_then(|s| cached(s, policy, now, SelectionReason::LastGood));
            let last_good = old_model.cloned();
            store.save(policy, &State { schema_version: 1, last_good, last_check_at: now, next_check_at, reason: SelectionReason::LastGood, last_error: Some(error) })?;
            fallback.map(|mut result| { result.last_error = Some(error); result }).ok_or(error)
        }
    }
}

/// Called on a dedicated request-construction thread, never inside an async runtime.
/// Config was fully validated before this function accesses PostgreSQL or Fireworks.
pub fn resolve(policy: Policy, credential: &str) -> Result<Resolution> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|_| SelectionError::Clock)?.as_secs();
    let mut store = store::PgStore::new()?;
    let _guard = REFRESH_LOCK.get_or_init(|| Mutex::new(())).try_lock();
    if _guard.is_err() {
        return store.load(&policy)?.as_ref().and_then(|s| cached(s, &policy, now, SelectionReason::LastGood)).ok_or(SelectionError::Busy);
    }
    let mut network = transport::FireworksNetwork::new(&policy, credential)?;
    select(&mut store, &mut network, &policy, now)
}

#[cfg(test)]
#[path = "model_resolver_tests.rs"]
mod tests;
