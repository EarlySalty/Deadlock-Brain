use super::*;

const NOW: u64 = 1_800_000_000;
const OLD: &str = "accounts/fireworks/models/deepseek-v4-flash";
const NEW: &str = "accounts/fireworks/models/deepseek-v4p1-flash";

pub(crate) fn policy(pin: Option<&str>) -> Policy {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("bot.toml");
    let mut text = include_str!("../../../../config/bot.toml").to_owned();
    if let Some(pin) = pin { text = text.replacen("[ai]", &format!("[ai]\npin = {pin:?}"), 1); }
    std::fs::write(&file, text).unwrap();
    let config = BotConfig::load(&file).unwrap();
    Policy::new(config, None, "synthetic-credential-marker").unwrap()
}

#[derive(Default)]
struct MemoryStore { state: Option<State>, saves: usize, busy: bool, fail_save: bool }
impl Store for MemoryStore {
    fn load(&mut self, _: &Policy) -> Result<Option<State>> { Ok(self.state.clone()) }
    fn try_lock(&mut self, _: &Policy) -> Result<bool> { Ok(!self.busy) }
    fn save(&mut self, _: &Policy, state: &State) -> Result<()> {
        if self.fail_save { return Err(SelectionError::StateUnavailable); }
        self.state = Some(state.clone()); self.saves += 1; Ok(())
    }
}

struct FakeNetwork { candidates: Vec<Candidate>, catalog_error: Option<SelectionError>, probe_error: Option<SelectionError>, catalogs: usize, probes: Vec<String> }
impl FakeNetwork {
    fn new(ids: &[&str]) -> Self {
        Self { candidates: ids.iter().map(|id| Candidate { model:(*id).to_owned(), version:numeric_version(id).unwrap(), created_at:NOW-10_000 }).collect(), catalog_error:None,probe_error:None,catalogs:0,probes:vec![] }
    }
}
impl Network for FakeNetwork {
    fn catalog(&mut self, _: &Policy, _: u64) -> Result<Vec<Candidate>> {
        self.catalogs+=1;
        match self.catalog_error { Some(e)=>Err(e),None=>Ok(self.candidates.clone()) }
    }
    fn probe(&mut self, _: &Policy, model: &str) -> Result<()> {
        self.probes.push(model.to_owned()); match self.probe_error { Some(e)=>Err(e),None=>Ok(()) }
    }
}

fn previous(policy: &Policy, model: &str, checked_at: u64) -> State {
    State { schema_version:1,last_good:Some(VerifiedModel { model:model.to_owned(),version:numeric_version(model).unwrap(), policy_fingerprint:policy.fingerprint.clone(),checked_at,expires_at:checked_at+policy.config.ai().selection.last_good_ttl_seconds,catalog_created_at:Some(checked_at-10_000),probe_revision:PROBE_REVISION.to_owned() }),last_check_at:checked_at,next_check_at:checked_at+policy.config.ai().selection.check_interval_seconds,reason:SelectionReason::InitialCatalog,last_error:None }
}

#[test]
fn numeric_versions_not_lexical_or_snapshot_dates() {
    // Synthetic version identifiers, not assertions about existing provider models.
    assert!(numeric_version("accounts/fireworks/models/deepseek-v10-flash") > numeric_version("accounts/fireworks/models/deepseek-v9-flash"));
    assert!(numeric_version("accounts/fireworks/models/deepseek-v4p10-flash") > numeric_version("accounts/fireworks/models/deepseek-v4p9-flash"));
    assert_eq!(numeric_version(OLD),numeric_version("accounts/fireworks/models/deepseek-v4.0-flash-999999"));
    assert_eq!(numeric_version(NEW),numeric_version("accounts/fireworks/models/deepseek-v4.1.0-flash"));
    for id in ["deepseek-latest","accounts/fireworks/models/deepseek-v4-pro","accounts/fireworks/models/deepseek-v4-flash-vision","accounts/fireworks/models/deepseek-v4-flash-preview","accounts/fireworks/models/deepseek-v4-flash-distill","accounts/other/models/deepseek-v4-flash"] {
        assert!(numeric_version(id).is_none());
    }
}

#[test]
fn explicit_pin_skips_catalog_and_rejects_conflicting_request() {
    let p=policy(Some(OLD)); let mut s=MemoryStore::default();let mut n=FakeNetwork::new(&[NEW]);
    n.catalog_error=Some(SelectionError::Unauthorized);
    let result=select(&mut s,&mut n,&p,NOW).unwrap();
    assert_eq!(result.verified.model,OLD);assert_eq!(result.reason,SelectionReason::Pin);
    assert_eq!(n.catalogs,0);assert_eq!(n.probes,vec![OLD]);
    assert!(matches!(Policy::new(Arc::clone(&p.config),Some(NEW),"synthetic-credential-marker"),Err(SelectionError::PinConflict)));
}

#[test]
fn newest_family_wins_not_newest_upload() {
    let p=policy(None);let mut s=MemoryStore::default();let mut n=FakeNetwork::new(&[OLD,NEW]);
    n.candidates[0].created_at=NOW-1;
    assert_eq!(select(&mut s,&mut n,&p,NOW).unwrap().verified.model,NEW);
    assert_eq!(s.saves,1);assert_eq!(n.probes,vec![NEW]);
}

#[test]
fn new_copy_of_same_version_never_replaces_checked_model() {
    let p=policy(None);let mut s=MemoryStore { state:Some(previous(&p,OLD,NOW-4000)),..Default::default() };
    let mut n=FakeNetwork::new(&["accounts/fireworks/models/deepseek-v4-flash-999999"]);
    let result=select(&mut s,&mut n,&p,NOW).unwrap();
    assert_eq!(result.verified.model,OLD);assert_eq!(result.reason,SelectionReason::LastGood);
    assert_eq!(result.last_error,Some(SelectionError::NoCandidate));assert!(n.probes.is_empty());
}

#[test]
fn checked_model_is_reverified_not_silently_downgraded() {
    let p=policy(None);let mut s=MemoryStore { state:Some(previous(&p,NEW,NOW-4000)),..Default::default() };
    let mut n=FakeNetwork::new(&[OLD,NEW]);
    let r=select(&mut s,&mut n,&p,NOW).unwrap();assert_eq!(r.verified.model,NEW);assert_eq!(r.reason,SelectionReason::Reverified);
    let mut n=FakeNetwork::new(&[OLD]);
    let mut s=MemoryStore { state:Some(previous(&p,NEW,NOW-100_000)),..Default::default() };
    assert!(matches!(select(&mut s,&mut n,&p,NOW),Err(SelectionError::NoCandidate)));
}

#[test]
fn successful_probe_is_required_before_atomic_publication() {
    let p=policy(None);let old=previous(&p,OLD,NOW-4000);
    let mut s=MemoryStore { state:Some(old.clone()),..Default::default() };let mut n=FakeNetwork::new(&[NEW]);
    n.probe_error=Some(SelectionError::ProbeFailed);
    let r=select(&mut s,&mut n,&p,NOW).unwrap();assert_eq!(r.verified.model,OLD);
    assert_eq!(s.state.as_ref().unwrap().last_good,old.last_good);
    let mut s=MemoryStore { state:Some(old.clone()),fail_save:true,..Default::default() };let mut n=FakeNetwork::new(&[NEW]);
    assert!(matches!(select(&mut s,&mut n,&p,NOW),Err(SelectionError::StateUnavailable)));
    assert_eq!(s.state.as_ref().unwrap().last_good,old.last_good);
}

#[test]
fn all_transport_failures_keep_only_policy_valid_unexpired_state() {
    let p=policy(None);
    for error in [SelectionError::Timeout,SelectionError::Unauthorized,SelectionError::Forbidden,SelectionError::RateLimited,SelectionError::ProviderUnavailable,SelectionError::CatalogInvalid] {
        let old=previous(&p,OLD,NOW-4000);
        let mut s=MemoryStore { state:Some(old.clone()),..Default::default() };let mut n=FakeNetwork::new(&[]);n.catalog_error=Some(error);
        let r=select(&mut s,&mut n,&p,NOW).unwrap();assert_eq!(r.last_error,Some(error));assert_eq!(r.verified,old.last_good.unwrap());
        let mut s=MemoryStore { state:Some(previous(&p,OLD,NOW-100_000)),..Default::default() };
        assert_eq!(select(&mut s,&mut n,&p,NOW).unwrap_err(),error);
    }
}

#[test]
fn first_start_empty_catalog_or_failed_probe_has_no_successful_model() {
    let p=policy(None);let mut s=MemoryStore::default();let mut n=FakeNetwork::new(&[]);
    assert!(matches!(select(&mut s,&mut n,&p,NOW),Err(SelectionError::NoCandidate)));
    assert!(s.state.as_ref().unwrap().last_good.is_none());
    let mut n=FakeNetwork::new(&[NEW]);n.probe_error=Some(SelectionError::ProbeFailed);
    assert!(matches!(select(&mut s,&mut n,&p,NOW),Err(SelectionError::ProbeFailed)));
    assert!(s.state.as_ref().unwrap().last_good.is_none());
}

#[test]
fn interval_cache_does_not_reprobe_or_extend_expiry() {
    let p=policy(None);let old=previous(&p,OLD,NOW-1);
    let mut s=MemoryStore { state:Some(old.clone()),..Default::default() };let mut n=FakeNetwork::new(&[NEW]);
    let r=select(&mut s,&mut n,&p,NOW).unwrap();assert_eq!(r.reason,SelectionReason::IntervalCache);
    assert_eq!(r.verified,old.last_good.unwrap());assert_eq!(n.catalogs,0);assert!(n.probes.is_empty());assert_eq!(s.saves,0);
}

#[test]
fn changed_policy_or_credential_cannot_reuse_old_cache() {
    let p=policy(None);let old=previous(&p,OLD,NOW-1);
    let changed=Policy::new(Arc::clone(&p.config),None,"rotated-synthetic-credential").unwrap();
    assert_ne!(p.fingerprint,changed.fingerprint);
    assert!(!format!("{changed:?}").contains("rotated-synthetic-credential"));
    let mut s=MemoryStore { state:Some(old.clone()),..Default::default() };let mut n=FakeNetwork::new(&[]);
    assert!(select(&mut s,&mut n,&changed,NOW).is_err());
    assert!(s.state.unwrap().last_good.is_none());
    let changed=Policy::new(Arc::clone(&p.config),Some(NEW),"synthetic-credential-marker").unwrap();
    let mut s=MemoryStore { state:Some(old),..Default::default() };let mut n=FakeNetwork::new(&[]);n.probe_error=Some(SelectionError::ProbeFailed);
    assert!(select(&mut s,&mut n,&changed,NOW).is_err());
}

#[test]
fn future_or_extended_cache_timestamps_are_not_trusted() {
    let p=policy(None);
    for mut state in [previous(&p,OLD,NOW+1),previous(&p,OLD,NOW-1)] {
        state.last_good.as_mut().unwrap().expires_at=NOW+10_000_000;
        let mut s=MemoryStore { state:Some(state),..Default::default() };let mut n=FakeNetwork::new(&[]);
        assert!(select(&mut s,&mut n,&p,NOW).is_err());
    }
}

#[test]
fn parallel_refresh_returns_old_snapshot_without_second_probe() {
    let p=policy(None);let old=previous(&p,OLD,NOW-4000);
    let mut s=MemoryStore { state:Some(old.clone()),busy:true,..Default::default() };let mut n=FakeNetwork::new(&[NEW]);
    let r=select(&mut s,&mut n,&p,NOW).unwrap();assert_eq!(r.verified,old.last_good.unwrap());assert_eq!(n.catalogs,0);assert!(n.probes.is_empty());
    let mut s=MemoryStore { busy:true,..Default::default() };
    assert!(matches!(select(&mut s,&mut n,&p,NOW),Err(SelectionError::Busy)));
}
