use dbrain_s10_evals::*;
use serde_json::json;

fn catalog() -> Catalog {
    serde_json::from_str(include_str!("../cases.json")).unwrap()
}
fn setup() -> (Profile, Catalog, Run) {
    let c = catalog();
    let mut p: Profile = serde_json::from_str(include_str!("../profile.json")).unwrap();
    p.approved = true;
    p.approval_reference = Some("test-only-independent-approval".into());
    p.hardware_fingerprint = Some("fixture-hardware".into());
    p.versions = Versions {
        baseline_commit: Some("b".repeat(40)),
        candidate_commit: Some("a".repeat(40)),
        contract: Some("fixture-contract".into()),
        db_schema: Some("fixture-schema".into()),
        knowledge: Some("fixture-knowledge".into()),
        corpus_sha256: Some("c".repeat(64)),
        answer_model: Some("fixture-answer".into()),
        embedding: Some("fixture-embedding".into()),
        search: Some("fixture-search".into()),
        jev: Some("disabled".into()),
    };
    p.dataset_sha256 = "d".repeat(64);
    p.workload = Workload {
        seed: 42,
        repetitions: 3,
        cache_states: vec!["cold".into(), "warm".into()],
        target_arrivals_per_second: Some(2.0),
        target_concurrency: Some(8),
        duration_seconds: Some(120),
        background_load: Some("mixed".into()),
    };
    p.acceptance = Acceptance {
        p95_ms: Some(100.0),
        p99_ms: Some(100.0),
        error_rate_max: Some(0.0),
        ram_mib_max: Some(100.0),
        cost_per_accepted_answer_max: Some(1.0),
        ingest_lag_seconds_max: Some(5.0),
        recall_noninferiority_margin: Some(0.02),
        answer_quality_noninferiority_margin: Some(0.02),
        rpo_seconds: Some(0.0),
        rto_seconds: Some(30.0),
    };
    p.g0_evidence_sha256 = Some("e".repeat(64));
    p.g1_evidence_sha256 = p.g0_evidence_sha256.clone();
    p.g2_evidence_sha256 = p.g0_evidence_sha256.clone();
    p.g3_evidence_sha256 = p.g0_evidence_sha256.clone();
    let mut samples = vec![];
    for case in &c.cases {
        for repetition in 0..3 {
            for cache in ["cold", "warm"] {
                samples.push(Sample {
                    id: format!("{}-{repetition}-{cache}", case.id),
                    case_id: case.id.clone(),
                    repetition,
                    cache_state: cache.into(),
                    scheduled_us: 0,
                    started_us: 1000,
                    finished_us: 2000,
                    outcome: case.expected.clone(),
                    checks: case
                        .checks
                        .iter()
                        .map(|id| Check {
                            id: id.clone(),
                            passed: Some(true),
                            evidence_sha256: Some("e".repeat(64)),
                        })
                        .collect(),
                    stages: vec![Stage {
                        name: "retrieval".into(),
                        elapsed_us: 500,
                    }],
                    tokens: Some(2),
                    costs: Costs {
                        answer: Some(0.1),
                        embedding: Some(0.0),
                        jev: Some(0.0),
                        shadow: Some(0.0),
                        retries: Some(0.0),
                    },
                });
            }
        }
    }
    let run = Run {
        schema_version: 1,
        kind: "real_e2e".into(),
        variant: "B".into(),
        versions: p.versions.clone(),
        hardware_fingerprint: "fixture-hardware".into(),
        profile_sha256: "f".repeat(64),
        dataset_sha256: "d".repeat(64),
        evaluator: "independent_human".into(),
        label_revision: Some("fixture-labels".into()),
        workload: p.workload.clone(),
        measurement_started_us: 0,
        measurement_finished_us: 120_000_000,
        offered_requests: samples.len() as u64,
        raw_artifact_sha256: Some("e".repeat(64)),
        resources: Resources {
            peak_ram_mib: Some(8.0),
            cpu_seconds: Some(1.0),
            io_read_bytes: Some(0),
            io_write_bytes: Some(0),
            ingest_lag_seconds: Some(0.0),
            rebuild_seconds: Some(1.0),
            max_queue_depth: Some(0),
            recovered_after_overload: Some(true),
        },
        samples,
    };
    (p, c, run)
}
fn verdict(p: &Profile, c: &Catalog, r: &Run) -> Assessment {
    assess(p, c, r, &"a".repeat(40), &"f".repeat(64), &"d".repeat(64))
}

#[test]
fn catalog_is_nonempty_and_complete() {
    validate_catalog(&catalog()).unwrap();
}
#[test]
fn empty_catalog_is_rejected() {
    let mut c = catalog();
    c.cases.clear();
    assert!(validate_catalog(&c).is_err());
}
#[test]
fn missing_mandatory_case_is_rejected() {
    let mut c = catalog();
    c.cases.pop();
    assert!(validate_catalog(&c).is_err());
}
#[test]
fn duplicate_case_is_rejected() {
    let mut c = catalog();
    c.cases.push(c.cases[0].clone());
    assert!(validate_catalog(&c).is_err());
}
#[test]
fn empty_assertion_set_is_rejected() {
    let mut c = catalog();
    c.cases[0].checks.clear();
    assert!(validate_catalog(&c).is_err());
}
#[test]
fn unknown_json_field_is_rejected() {
    let mut v = serde_json::to_value(catalog()).unwrap();
    v["approved"] = json!(true);
    assert!(serde_json::from_value::<Catalog>(v).is_err());
}
#[test]
fn duplicate_json_struct_key_is_rejected() {
    assert!(serde_json::from_str::<Catalog>(
        r#"{"schema_version":1,"schema_version":1,"cases":[]}"#
    )
    .is_err());
}
#[test]
fn hash_is_sha256() {
    assert_eq!(
        digest(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}
#[test]
fn nearest_rank_percentiles_are_correct() {
    let d = distribution(&(1..=100).map(f64::from).collect::<Vec<_>>()).unwrap();
    assert_eq!((d.p50, d.p95, d.p99), (Some(50.0), Some(95.0), Some(99.0)));
}
#[test]
fn empty_metrics_are_null_not_zero() {
    let d = distribution(&[]).unwrap();
    assert_eq!(d.p95, None);
    assert_eq!(d.mean, None);
}
#[test]
fn nonfinite_and_negative_latency_rejected() {
    for v in [f64::NAN, f64::INFINITY, -1.0] {
        assert!(distribution(&[v]).is_err());
    }
}
#[test]
fn standard_deviation_is_reported() {
    let d = distribution(&[1.0, 3.0]).unwrap();
    assert_eq!(d.mean, Some(2.0));
    assert_eq!(d.stddev, Some(1.0));
}
#[test]
fn queue_time_is_part_of_end_to_end_latency() {
    let (_, c, r) = setup();
    let s = summarize(&c, &r).unwrap();
    assert_eq!(s.end_to_end_ms.p95, Some(2.0));
    assert_eq!(s.queue_ms.p95, Some(1.0));
}
#[test]
fn fully_labelled_fixture_only_proposes_pass() {
    let (p, c, r) = setup();
    let a = verdict(&p, &c, &r);
    assert_eq!(a.status, "pass", "{:?} {:?}", a.blockers, a.failures);
    assert!(!a.release_approved);
}
#[test]
fn default_profile_cannot_pass() {
    let (_, c, r) = setup();
    let p = serde_json::from_str(include_str!("../profile.json")).unwrap();
    assert_ne!(verdict(&p, &c, &r).status, "pass");
}
macro_rules! blocked {
    ($name:ident, $p:ident, $r:ident, $change:block) => {
        #[test]
        fn $name() {
            let (mut $p, c, mut $r) = setup();
            let _ = (&mut $p, &mut $r);
            $change;
            assert_ne!(verdict(&$p, &c, &$r).status, "pass");
        }
    };
}
blocked!(unapproved_profile_blocks, p, r, {
    p.approved = false;
});
blocked!(missing_approval_blocks, p, r, {
    p.approval_reference = None;
});
blocked!(missing_slo_blocks, p, r, {
    p.acceptance.p95_ms = None;
});
blocked!(missing_g3_blocks, p, r, {
    p.g3_evidence_sha256 = None;
});
blocked!(unknown_schema_blocks, p, r, {
    r.schema_version = 99;
});
blocked!(old_commit_cannot_reuse_results, p, r, {
    r.versions.candidate_commit = Some("0".repeat(40));
});
blocked!(missing_contract_blocks, p, r, {
    p.versions.contract = None;
    r.versions.contract = None;
});
blocked!(mixed_knowledge_blocks, p, r, {
    r.versions.knowledge = Some("other".into());
});
blocked!(wrong_dataset_hash_blocks, p, r, {
    r.dataset_sha256 = "0".repeat(64);
});
blocked!(wrong_profile_hash_blocks, p, r, {
    r.profile_sha256 = "0".repeat(64);
});
blocked!(mock_never_becomes_e2e, p, r, {
    r.kind = "mock".into();
});
blocked!(stage_fixture_never_becomes_e2e, p, r, {
    r.kind = "stage_fixture".into();
});
blocked!(same_model_is_not_independent_label, p, r, {
    r.evaluator = "answer_model".into();
});
blocked!(missing_raw_artifact_blocks, p, r, {
    r.raw_artifact_sha256 = None;
});
blocked!(missing_label_revision_blocks, p, r, {
    r.label_revision = None;
});
blocked!(missing_samples_block, p, r, {
    r.samples.clear();
});
blocked!(missing_case_sample_blocks, p, r, {
    r.samples.pop();
    r.offered_requests -= 1;
});
blocked!(duplicate_samples_block, p, r, {
    r.samples.push(r.samples[0].clone());
    r.offered_requests += 1;
});
blocked!(offered_requests_cannot_be_dropped, p, r, {
    r.offered_requests += 1;
});
blocked!(missing_check_blocks, p, r, {
    r.samples[0].checks.pop();
});
blocked!(failed_check_blocks, p, r, {
    r.samples[0].checks[0].passed = Some(false);
});
blocked!(unmeasured_check_blocks, p, r, {
    r.samples[0].checks[0].passed = None;
});
blocked!(missing_check_evidence_blocks, p, r, {
    r.samples[0].checks[0].evidence_sha256 = None;
});
blocked!(timeout_is_not_an_abstention, p, r, {
    r.samples[0].outcome = "timeout".into();
});
blocked!(unknown_case_cannot_count_as_coverage, p, r, {
    r.samples[0].case_id = "invented".into();
});
blocked!(missing_cost_is_not_free, p, r, {
    r.samples[0].costs.embedding = None;
});
blocked!(negative_cost_blocks, p, r, {
    r.samples[0].costs.answer = Some(-1.0);
});
blocked!(missing_tokens_block, p, r, {
    r.samples[0].tokens = None;
});
blocked!(missing_ram_blocks, p, r, {
    r.resources.peak_ram_mib = None;
});
blocked!(queue_failure_blocks, p, r, {
    r.resources.recovered_after_overload = Some(false);
});
blocked!(over_slo_blocks, p, r, {
    p.acceptance.p95_ms = Some(0.1);
});
blocked!(bad_timestamp_blocks, p, r, {
    r.samples[0].finished_us = 0;
});
blocked!(out_of_window_timestamp_blocks, p, r, {
    r.samples[0].finished_us = r.measurement_finished_us + 1;
});
blocked!(background_profile_mismatch_blocks, p, r, {
    r.workload.background_load = Some("idle".into());
});
blocked!(zero_repetitions_cannot_skip_coverage, p, r, {
    p.workload.repetitions = 0;
    r.workload.repetitions = 0;
});
blocked!(missing_cache_state_blocks, p, r, {
    p.workload.cache_states.clear();
    r.workload.cache_states.clear();
});
blocked!(invalid_margin_blocks, p, r, {
    p.acceptance.answer_quality_noninferiority_margin = Some(2.0);
});
blocked!(unapproved_hardware_blocks, p, r, {
    r.hardware_fingerprint = "different".into();
});
blocked!(unknown_variant_blocks, p, r, {
    r.variant = "fastest".into();
});
blocked!(insufficient_offered_load_blocks, p, r, {
    p.workload.target_arrivals_per_second = Some(1000.0);
    r.workload = p.workload.clone();
});
blocked!(short_measurement_cannot_claim_full_duration, p, r, {
    r.measurement_finished_us = 60_000_000;
});
blocked!(incoherent_latency_limits_block, p, r, {
    p.acceptance.p95_ms = Some(200.0);
    p.acceptance.p99_ms = Some(100.0);
});
blocked!(overflowing_offered_load_blocks, p, r, {
    p.workload.target_arrivals_per_second = Some(f64::MAX);
    r.workload = p.workload.clone();
});

#[test]
fn incomplete_total_cost_stays_null() {
    let (_, c, mut r) = setup();
    r.samples[0].costs.retries = None;
    assert_eq!(summarize(&c, &r).unwrap().cost_per_accepted_answer, None);
}
#[test]
fn failed_requests_remain_in_denominator_and_cost() {
    let (_, c, mut r) = setup();
    let n = r.samples.len();
    r.samples[0].outcome = "error".into();
    let s = summarize(&c, &r).unwrap();
    assert_eq!(s.requests, n);
    assert_eq!(s.accepted, n - 1);
    assert!((s.error_rate.unwrap() - 1.0 / n as f64).abs() < 1e-9);
    assert!(s.cost_per_accepted_answer.unwrap() > 0.1);
}
#[test]
fn statistically_weak_sample_does_not_prove_noninferiority() {
    let (_, _, passed) = paired_noninferiority(&[true; 10], &[true; 10], 0.02).unwrap();
    assert!(!passed);
}
#[test]
fn paired_quality_regression_is_rejected() {
    let (_, _, passed) = paired_noninferiority(&[true; 1000], &[false; 1000], 0.02).unwrap();
    assert!(!passed);
}
#[test]
fn sufficient_identical_pairs_can_pass() {
    let (_, _, passed) = paired_noninferiority(&[true; 20000], &[true; 20000], 0.02).unwrap();
    assert!(passed);
}
#[test]
fn unequal_and_empty_quality_pairs_rejected() {
    assert!(paired_noninferiority(&[], &[], 0.02).is_err());
    assert!(paired_noninferiority(&[true], &[], 0.02).is_err());
}
fn row(id: &str, split: &str, event: i64, available: i64) -> TemporalRow {
    TemporalRow {
        match_id: id.into(),
        split: split.into(),
        event_at: event,
        available_at: available,
    }
}
#[test]
fn valid_availability_holdout_passes() {
    temporal_holdout(&[row("a", "train", 1, 2), row("b", "test", 11, 12)], 10).unwrap();
}
#[test]
fn future_available_training_data_is_rejected() {
    assert!(temporal_holdout(&[row("a", "train", 1, 11), row("b", "test", 12, 12)], 10).is_err());
}
#[test]
fn same_match_across_splits_is_rejected() {
    assert!(temporal_holdout(&[row("a", "train", 1, 2), row("a", "test", 11, 12)], 10).is_err());
}
#[test]
fn holdout_must_follow_cutoff() {
    assert!(temporal_holdout(&[row("a", "train", 1, 2), row("b", "test", 9, 12)], 10).is_err());
}
#[test]
fn empty_holdout_is_not_success() {
    assert!(temporal_holdout(&[], 10).is_err());
}
