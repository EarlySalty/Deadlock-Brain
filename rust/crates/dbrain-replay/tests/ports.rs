mod common;
use common::*;
use dbrain_replay::*;

// Test double ONLY. Production uses the existing S03 store/transaction, never a second database.
#[derive(Default)]
struct Store {
    rows: Vec<(ReplayDedupKeys, ReplayReport)>,
    quarantined: Vec<ReplayFailure>,
}
impl ObservationStore for Store {
    fn commit_replay(
        &mut self,
        keys: &ReplayDedupKeys,
        report: &ReplayReport,
    ) -> Result<ObservationCommit, ReplayFailure> {
        for (old_keys, old_report) in &mut self.rows {
            let decision = classify_replay_duplicate(keys, old_keys);
            match decision {
                ObservationCommit::Inserted => continue,
                ObservationCommit::Reparsed => {
                    *old_keys = keys.clone();
                    *old_report = report.clone();
                    return Ok(decision);
                }
                _ => return Ok(decision),
            }
        }
        self.rows.push((keys.clone(), report.clone()));
        Ok(ObservationCommit::Inserted)
    }
    fn quarantine_replay(
        &mut self,
        _: &ReplaySource,
        reason: ReplayFailure,
    ) -> Result<(), ReplayFailure> {
        self.quarantined.push(reason);
        Ok(())
    }
}
#[test]
fn actual_decoder_reaches_shared_observation_port_idempotently() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("raw");
    std::fs::write(&path, minimal()).unwrap();
    let mut store = Store::default();
    let decoder = worker();
    let r = request();
    assert_eq!(
        decode_into_store(&decoder, &mut store, &path, &r).unwrap(),
        ObservationCommit::Inserted
    );
    assert_eq!(
        decode_into_store(&decoder, &mut store, &path, &r).unwrap(),
        ObservationCommit::DuplicateGeneration
    );
    assert_eq!(store.rows.len(), 1);
    assert!(store.quarantined.is_empty());
    let mut changed = r.clone();
    changed.selection.sample_every_ticks += 1;
    assert_eq!(
        decode_into_store(&decoder, &mut store, &path, &changed).unwrap(),
        ObservationCommit::Reparsed
    );
    assert_eq!(store.rows.len(), 1);
    assert_eq!(store.rows[0].1.observations.len(), 1);
}
#[test]
fn damaged_replay_is_quarantined_not_committed_as_partial_observations() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("raw");
    std::fs::write(&path, b"PBDEMS2\0").unwrap();
    let mut store = Store::default();
    assert!(decode_into_store(&worker(), &mut store, &path, &request()).is_err());
    assert!(store.rows.is_empty());
    assert_eq!(store.quarantined, vec![ReplayFailure::DamagedReplay]);
}
#[test]
fn same_match_different_raw_does_not_double_the_population_or_cross_scopes() {
    let mut r = request();
    r.source.match_reference = Some(MatchReference {
        match_id: "42".into(),
        evidence_ref: "authorized-match-manifest".into(),
    });
    let a = ReplayDedupKeys::from_report(&decode(&minimal(), &r).unwrap());
    let mut b = ReplayDedupKeys::from_report(&decode(&container(&[], true), &r).unwrap());
    assert_eq!(
        classify_replay_duplicate(&b, &a),
        ObservationCommit::DuplicateMatch
    );
    b.parser_revision.push_str("-new");
    assert_eq!(
        classify_replay_duplicate(&b, &a),
        ObservationCommit::Reparsed
    );
    b.scope = "another-private-owner".into();
    assert_eq!(
        classify_replay_duplicate(&b, &a),
        ObservationCommit::Inserted
    );
}
#[test]
fn no_match_identity_never_deduplicates_two_unrelated_unknown_matches() {
    let a = ReplayDedupKeys::from_report(&decode(&minimal(), &request()).unwrap());
    let b = ReplayDedupKeys::from_report(&decode(&container(&[], true), &request()).unwrap());
    assert_eq!(
        classify_replay_duplicate(&b, &a),
        ObservationCommit::Inserted
    );
}
#[derive(Clone)]
struct ForgedDecoder(ReplayReport);
impl ReplayDecoder for ForgedDecoder {
    fn decode(
        &self,
        _: &std::path::Path,
        _: &ReplayRequest,
    ) -> Result<ReplayReport, ReplayFailure> {
        Ok(self.0.clone())
    }
}
#[test]
fn shared_port_rejects_fabricated_coaching_or_real_validation_claims() {
    let mut report = decode(&minimal(), &request()).unwrap();
    report.coaching_eligible = true;
    let mut store = Store::default();
    assert_eq!(
        decode_into_store(
            &ForgedDecoder(report),
            &mut store,
            std::path::Path::new("unused"),
            &request()
        ),
        Err(ReplayFailure::InvalidWorkerOutput)
    );
    assert!(store.rows.is_empty());
    assert_eq!(store.quarantined.len(), 1);
}
