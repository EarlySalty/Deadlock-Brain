use crate::*;
use std::collections::BTreeSet;

pub(crate) fn validate(
    p: &Profile,
    run: &Run,
    expected_commit: &str,
    profile_hash: &str,
    dataset_hash: &str,
    b: &mut Vec<String>,
    f: &mut Vec<String>,
) {
    require(p.schema_version == 1, f, "unknown profile schema");
    require(
        p.approved && present(&p.approval_reference),
        b,
        "profile/SLOs not independently approved",
    );
    require(
        valid_hash(expected_commit, 40)
            && p.versions.candidate_commit.as_deref() == Some(expected_commit)
            && run.versions.candidate_commit.as_deref() == Some(expected_commit),
        f,
        "candidate commit mismatch",
    );
    require(
        valid_hash(profile_hash, 64) && run.profile_sha256 == profile_hash,
        f,
        "profile hash mismatch",
    );
    require(
        valid_hash(dataset_hash, 64)
            && p.dataset_sha256 == dataset_hash
            && run.dataset_sha256 == dataset_hash,
        f,
        "dataset hash mismatch",
    );
    require(
        p.versions == run.versions,
        f,
        "source/model/contract/knowledge versions differ",
    );
    for (name, value) in [
        ("baseline", &p.versions.baseline_commit),
        ("candidate", &p.versions.candidate_commit),
        ("contract", &p.versions.contract),
        ("schema", &p.versions.db_schema),
        ("knowledge", &p.versions.knowledge),
        ("corpus", &p.versions.corpus_sha256),
        ("answer model", &p.versions.answer_model),
        ("embedding", &p.versions.embedding),
        ("search", &p.versions.search),
        ("Jev", &p.versions.jev),
    ] {
        require(present(value), b, &format!("missing pinned {name}"));
    }
    require(
        p.versions
            .baseline_commit
            .as_ref()
            .is_some_and(|v| valid_hash(v, 40)),
        b,
        "baseline commit invalid",
    );
    require(
        evidence(&p.versions.corpus_sha256),
        b,
        "corpus digest missing",
    );
    require(
        present(&p.hardware_fingerprint)
            && p.hardware_fingerprint.as_deref() == Some(&run.hardware_fingerprint),
        b,
        "hardware missing or mismatched",
    );
    for (name, value) in [
        ("G0", &p.g0_evidence_sha256),
        ("G1", &p.g1_evidence_sha256),
        ("G2", &p.g2_evidence_sha256),
        ("G3", &p.g3_evidence_sha256),
    ] {
        require(evidence(value), b, &format!("{name} evidence absent"));
    }
    require(
        run.kind == "real_e2e",
        b,
        "unit/mock/stage evidence cannot satisfy E2E acceptance",
    );
    require(
        ["A", "B", "C", "D"].contains(&run.variant.as_str()),
        f,
        "unknown comparison variant",
    );
    if ["A", "B", "C"].contains(&run.variant.as_str()) {
        require(
            run.versions.jev.as_deref() == Some("disabled"),
            f,
            "Jev enabled outside isolated D variant",
        );
    } else if run.variant == "D" {
        require(
            present(&run.versions.jev) && run.versions.jev.as_deref() != Some("disabled"),
            b,
            "D has no pinned Jev function",
        );
    }
    require(
        ["independent_human", "deterministic_reference"].contains(&run.evaluator.as_str()),
        b,
        "independent reference labels absent",
    );
    require(present(&run.label_revision), b, "label revision absent");
    require(
        evidence(&run.raw_artifact_sha256),
        b,
        "raw artifact digest absent",
    );
    require(p.workload == run.workload, f, "workload mismatch");
    require(
        (3..=1000).contains(&p.workload.repetitions),
        b,
        "at least three bounded repetitions required",
    );
    let caches: BTreeSet<_> = p.workload.cache_states.iter().map(String::as_str).collect();
    require(
        caches == BTreeSet::from(["cold", "warm"]) && p.workload.cache_states.len() == 2,
        b,
        "separate cold/warm measurements required",
    );
    require(
        p.workload
            .target_arrivals_per_second
            .is_some_and(|v| v.is_finite() && v > 0.0),
        b,
        "target arrival rate missing",
    );
    require(
        p.workload.target_concurrency.is_some_and(|v| v > 0),
        b,
        "target concurrency missing",
    );
    require(
        p.workload.duration_seconds.is_some_and(|v| v > 0),
        b,
        "measurement duration missing",
    );
    if let Some(seconds) = p.workload.duration_seconds {
        require(
            run.measurement_finished_us
                .checked_sub(run.measurement_started_us)
                .is_some_and(|elapsed| elapsed >= u64::from(seconds) * 1_000_000),
            b,
            "measurement shorter than approved duration",
        );
        if let Some(rate) = p.workload.target_arrivals_per_second {
            let minimum = (rate * f64::from(seconds)).ceil();
            require(
                minimum.is_finite()
                    && minimum > 0.0
                    && minimum < u64::MAX as f64
                    && run.offered_requests >= minimum as u64,
                b,
                "offered request count below approved load or invalid target",
            );
        }
    }
    if let (Some(p95), Some(p99)) = (p.acceptance.p95_ms, p.acceptance.p99_ms) {
        require(p95 <= p99, b, "p95 threshold exceeds p99 threshold");
    }
    require(
        p.workload.background_load.as_deref() == Some("mixed"),
        b,
        "mixed ingestion/replay/embedding load not evidenced",
    );
    for (name, value, interval) in [
        ("p95", p.acceptance.p95_ms, false),
        ("p99", p.acceptance.p99_ms, false),
        ("error rate", p.acceptance.error_rate_max, true),
        ("RAM", p.acceptance.ram_mib_max, false),
        ("cost", p.acceptance.cost_per_accepted_answer_max, false),
        ("ingest lag", p.acceptance.ingest_lag_seconds_max, false),
        (
            "recall margin",
            p.acceptance.recall_noninferiority_margin,
            true,
        ),
        (
            "answer margin",
            p.acceptance.answer_quality_noninferiority_margin,
            true,
        ),
        ("RPO", p.acceptance.rpo_seconds, false),
        ("RTO", p.acceptance.rto_seconds, false),
    ] {
        require(
            value.is_some_and(|v| nonnegative(v) && (!interval || v <= 1.0)),
            b,
            &format!("missing or invalid {name} threshold"),
        );
    }
    for (name, value) in [
        ("RAM", run.resources.peak_ram_mib),
        ("CPU", run.resources.cpu_seconds),
        ("ingest lag", run.resources.ingest_lag_seconds),
        ("rebuild", run.resources.rebuild_seconds),
    ] {
        require(
            value.is_some_and(nonnegative),
            b,
            &format!("missing or invalid {name} measurement"),
        );
    }
    require(
        run.resources.io_read_bytes.is_some() && run.resources.io_write_bytes.is_some(),
        b,
        "IO measurements absent",
    );
    require(
        run.resources.max_queue_depth.is_some(),
        b,
        "queue growth measurement absent",
    );
    require(
        run.resources.recovered_after_overload == Some(true),
        b,
        "overload recovery not verified",
    );
}
