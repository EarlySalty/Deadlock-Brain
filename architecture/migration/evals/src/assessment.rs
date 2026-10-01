use crate::*;
use std::collections::BTreeSet;
#[path = "preconditions.rs"]
mod preconditions;

pub fn assess(
    p: &Profile,
    catalog: &Catalog,
    run: &Run,
    expected_commit: &str,
    profile_hash: &str,
    dataset_hash: &str,
) -> Assessment {
    let mut blockers = vec![];
    let mut failures = vec![];
    preconditions::validate(
        p,
        run,
        expected_commit,
        profile_hash,
        dataset_hash,
        &mut blockers,
        &mut failures,
    );
    require(!run.samples.is_empty(), &mut blockers, "no samples");
    // Bound loops even when the input has an invalid repetition count.
    if (3..=1000).contains(&p.workload.repetitions) && p.workload.cache_states.len() == 2 {
        let keys: BTreeSet<_> = run
            .samples
            .iter()
            .map(|s| (&s.case_id, s.repetition, &s.cache_state))
            .collect();
        for case in &catalog.cases {
            let complete = (0..p.workload.repetitions).all(|rep| {
                p.workload
                    .cache_states
                    .iter()
                    .all(|cache| keys.contains(&(&case.id, rep, cache)))
            });
            require(
                complete,
                &mut blockers,
                "mandatory case/repetition/cache evidence missing",
            );
        }
    }
    for sample in &run.samples {
        if let Some(case) = catalog.cases.iter().find(|c| c.id == sample.case_id) {
            require(
                sample.outcome == case.expected,
                &mut failures,
                "unexpected outcome in required case",
            );
            for id in &case.checks {
                match sample.checks.iter().find(|check| &check.id == id) {
                    None => blockers.push("required assertion missing".into()),
                    Some(check) => {
                        require(
                            check.passed.is_some(),
                            &mut blockers,
                            "assertion unmeasured",
                        );
                        require(
                            check.passed != Some(false),
                            &mut failures,
                            "required assertion failed",
                        );
                        require(
                            evidence(&check.evidence_sha256),
                            &mut blockers,
                            "assertion evidence absent",
                        );
                    }
                }
            }
        }
        require(
            !sample.stages.is_empty(),
            &mut blockers,
            "stage timings absent",
        );
    }
    let summary = match summarize(catalog, run) {
        Ok(summary) => {
            require(
                summary.tokens.is_some(),
                &mut blockers,
                "token accounting incomplete",
            );
            require(
                summary.cost_per_accepted_answer.is_some(),
                &mut blockers,
                "full answer/embedding/Jev/shadow/retry costs absent",
            );
            for (name, measured, limit) in [
                ("p95", summary.end_to_end_ms.p95, p.acceptance.p95_ms),
                ("p99", summary.end_to_end_ms.p99, p.acceptance.p99_ms),
                (
                    "error rate",
                    summary.error_rate,
                    p.acceptance.error_rate_max,
                ),
                ("RAM", run.resources.peak_ram_mib, p.acceptance.ram_mib_max),
                (
                    "cost",
                    summary.cost_per_accepted_answer,
                    p.acceptance.cost_per_accepted_answer_max,
                ),
                (
                    "ingest lag",
                    run.resources.ingest_lag_seconds,
                    p.acceptance.ingest_lag_seconds_max,
                ),
            ] {
                if let (Some(actual), Some(max)) = (measured, limit) {
                    require(
                        actual <= max,
                        &mut failures,
                        &format!("{name} threshold exceeded"),
                    );
                }
            }
            Some(summary)
        }
        Err(error) => {
            failures.push(error);
            None
        }
    };
    blockers.sort();
    blockers.dedup();
    failures.sort();
    failures.dedup();
    let status = if !failures.is_empty() {
        "fail"
    } else if !blockers.is_empty() {
        "blocked"
    } else {
        "pass"
    };
    Assessment {
        status: status.into(),
        release_approved: false,
        blockers,
        failures,
        summary,
    }
}
