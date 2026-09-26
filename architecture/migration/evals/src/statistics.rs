use crate::*;
use std::collections::{BTreeMap, BTreeSet};

pub fn distribution(values: &[f64]) -> Result<Distribution, String> {
    if !values.iter().all(|v| nonnegative(*v)) {
        return Err("nonfinite or negative measurement".into());
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    let n = sorted.len();
    if n == 0 {
        return Ok(Distribution {
            count: 0,
            p50: None,
            p95: None,
            p99: None,
            mean: None,
            stddev: None,
        });
    }
    let mean = sorted.iter().map(|v| v / n as f64).sum::<f64>();
    let stddev = (sorted
        .iter()
        .map(|v| (v - mean).powi(2) / n as f64)
        .sum::<f64>())
    .sqrt();
    if !mean.is_finite() || !stddev.is_finite() {
        return Err("measurement arithmetic overflow".into());
    }
    let percentile = |p: f64| Some(sorted[((p * n as f64).ceil() as usize).saturating_sub(1)]);
    Ok(Distribution {
        count: n,
        p50: percentile(0.50),
        p95: percentile(0.95),
        p99: percentile(0.99),
        mean: Some(mean),
        stddev: Some(stddev),
    })
}
fn sample_cost(costs: &Costs) -> Result<Option<f64>, String> {
    let parts = [
        costs.answer,
        costs.embedding,
        costs.jev,
        costs.shadow,
        costs.retries,
    ];
    if parts.iter().flatten().any(|v| !nonnegative(*v)) {
        return Err("invalid cost component".into());
    }
    let sum: Option<f64> = parts.into_iter().sum();
    if sum.is_some_and(|v| !v.is_finite()) {
        return Err("cost overflow".into());
    }
    Ok(sum)
}
pub fn summarize(catalog: &Catalog, run: &Run) -> Result<Summary, String> {
    validate_catalog(catalog)?;
    if run.schema_version != 1 {
        return Err("unknown run schema".into());
    }
    if run.measurement_finished_us <= run.measurement_started_us {
        return Err("invalid measurement window".into());
    }
    if run.offered_requests != run.samples.len() as u64 {
        return Err("offered requests missing from raw samples".into());
    }
    let mut ids = BTreeSet::new();
    let mut keys = BTreeSet::new();
    let mut latency = vec![];
    let mut queue = vec![];
    let mut stages: BTreeMap<String, Vec<f64>> = BTreeMap::new();
    let mut total_cost = Some(0.0);
    let mut total_tokens = Some(0_u64);
    let mut success = 0;
    for sample in &run.samples {
        if !identifier(&sample.id)
            || !ids.insert(&sample.id)
            || !keys.insert((&sample.case_id, sample.repetition, &sample.cache_state))
        {
            return Err("duplicate or invalid sample".into());
        }
        let case = catalog
            .cases
            .iter()
            .find(|c| c.id == sample.case_id)
            .ok_or("unknown sample case")?;
        if sample.repetition >= run.workload.repetitions
            || !run.workload.cache_states.contains(&sample.cache_state)
        {
            return Err("sample outside repetition/cache profile".into());
        }
        if sample.scheduled_us < run.measurement_started_us
            || sample.scheduled_us > sample.started_us
            || sample.started_us > sample.finished_us
            || sample.finished_us > run.measurement_finished_us
        {
            return Err("invalid sample timestamp".into());
        }
        if ![
            "answer",
            "abstain",
            "deny",
            "degraded",
            "quarantine",
            "error",
            "timeout",
            "cancelled",
            "dropped",
        ]
        .contains(&sample.outcome.as_str())
        {
            return Err("unknown sample outcome".into());
        }
        let check_ids: BTreeSet<_> = sample.checks.iter().map(|c| &c.id).collect();
        if check_ids.len() != sample.checks.len()
            || sample.checks.iter().any(|c| !case.checks.contains(&c.id))
        {
            return Err("duplicate or unknown assertion".into());
        }
        let mut stage_ids = BTreeSet::new();
        for stage in &sample.stages {
            if !identifier(&stage.name)
                || !stage_ids.insert(&stage.name)
                || stage.elapsed_us > sample.finished_us - sample.started_us
            {
                return Err("invalid stage measurement".into());
            }
            stages
                .entry(stage.name.clone())
                .or_default()
                .push(stage.elapsed_us as f64 / 1000.0);
        }
        // Scheduled arrival includes queue time, unlike dispatch-based measurements.
        latency.push((sample.finished_us - sample.scheduled_us) as f64 / 1000.0);
        queue.push((sample.started_us - sample.scheduled_us) as f64 / 1000.0);
        success += usize::from(accepted(case, sample));
        total_tokens = total_tokens.and_then(|sum| sample.tokens.and_then(|t| sum.checked_add(t)));
        total_cost = total_cost
            .zip(sample_cost(&sample.costs)?)
            .map(|(a, b)| a + b);
        if total_cost.is_some_and(|v| !v.is_finite()) {
            return Err("total cost overflow".into());
        }
    }
    let n = run.samples.len();
    let duration = (run.measurement_finished_us - run.measurement_started_us) as f64 / 1_000_000.0;
    Ok(Summary {
        requests: n,
        accepted: success,
        error_rate: (n > 0).then(|| (n - success) as f64 / n as f64),
        end_to_end_ms: distribution(&latency)?,
        queue_ms: distribution(&queue)?,
        successful_requests_per_second: (n > 0).then_some(success as f64 / duration),
        tokens: if n > 0 { total_tokens } else { None },
        cost_per_accepted_answer: if success > 0 {
            total_cost.map(|c| c / success as f64)
        } else {
            None
        },
        stages_ms: stages
            .into_iter()
            .map(|(k, v)| Ok((k, distribution(&v)?)))
            .collect::<Result<_, String>>()?,
    })
}
