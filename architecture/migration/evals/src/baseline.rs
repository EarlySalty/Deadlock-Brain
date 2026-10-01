use super::*;
use std::{
    hint::black_box,
    time::{Instant, SystemTime, UNIX_EPOCH},
};
// Reuse the real, dependency-free population metric implementation unchanged.
// rustfmt must not recurse into a module owned by S05.
#[rustfmt::skip]
#[path="../../../../rust/crates/dbrain-population/src/metrics.rs"]
mod population_metrics;
const SOURCE: &[u8] = include_bytes!("../../../../rust/crates/dbrain-population/src/metrics.rs");
const REPETITIONS: usize = 5;
const ITERATIONS: usize = 5000;
fn cpu_model() -> Option<String> {
    fs::read_to_string("/proc/cpuinfo")
        .ok()?
        .lines()
        .find_map(|line| {
            let (key, value) = line.split_once(':')?;
            (key.trim() == "model name").then(|| value.trim().to_owned())
        })
}
fn proc_kib(path: &str, key: &str) -> Option<u64> {
    fs::read_to_string(path).ok()?.lines().find_map(|line| {
        let (name, value) = line.split_once(':')?;
        if name == key {
            value.split_whitespace().next()?.parse().ok()
        } else {
            None
        }
    })
}
fn golden() -> bool {
    let build = [1, 2, 3, 4, 5, 6];
    let player = [1, 2, 3, 7, 8, 9];
    let population = [(1, 0.0), (2, 1.0), (3, 2.0)];
    let reverse = [(1, 2.0), (2, 1.0), (3, 0.0)];
    (population_metrics::jaccard_at(6, &build, &player) - 1.0 / 3.0).abs() < 1e-12
        && (population_metrics::kendall_tau(&[1, 2, 3], &population) - 1.0).abs() < 1e-12
        && (population_metrics::kendall_tau(&[1, 2, 3], &reverse) + 1.0).abs() < 1e-12
        && population_metrics::jaccard_at(6, &[], &[]).is_nan()
        && population_metrics::kendall_tau(&[1], &[(1, 0.0)]).is_nan()
}
pub fn measure() -> Result<Value, String> {
    if !golden() {
        return Err("existing population metric disagrees with exact fixture".into());
    }
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let candidate = git_text(&root, &["rev-parse", "HEAD"])?;
    let dirty = !git_text(
        &root,
        &[
            "status",
            "--porcelain",
            "--",
            "architecture/migration/evals",
            "rust/crates/dbrain-population/src/metrics.rs",
        ],
    )?
    .is_empty();
    let build = [1, 2, 3, 4, 5, 6];
    let player = [1, 2, 3, 7, 8, 9];
    let population = [(1, 0.0), (2, 1.0), (3, 2.0)];
    // One explicit unmeasured process warmup. No claim about database/cache coldness.
    for _ in 0..ITERATIONS {
        black_box(population_metrics::jaccard_at(
            6,
            black_box(&build),
            black_box(&player),
        ));
    }
    let mut samples = vec![];
    for repetition in 0..REPETITIONS {
        let started = Instant::now();
        for _ in 0..ITERATIONS {
            black_box(population_metrics::jaccard_at(
                6,
                black_box(&build),
                black_box(&player),
            ));
            black_box(population_metrics::kendall_tau(
                black_box(&build),
                black_box(&population),
            ));
        }
        samples.push(json!({"repetition":repetition,"iterations":ITERATIONS,"function_calls":ITERATIONS*2,"elapsed_us":started.elapsed().as_secs_f64()*1_000_000.0}));
    }
    let per_pair_ms = samples
        .iter()
        .map(|s| s["elapsed_us"].as_f64().unwrap() / ITERATIONS as f64 / 1000.0)
        .collect::<Vec<_>>();
    let hardware = json!({"cpu":cpu_model(),"available_parallelism":std::thread::available_parallelism().ok().map(usize::from),"ram_kib":proc_kib("/proc/meminfo","MemTotal"),"os":std::env::consts::OS,"arch":std::env::consts::ARCH,"storage":null,"network":null,"container_limits":null});
    let binary = std::env::current_exe().map_err(|_| "binary path unavailable")?;
    Ok(json!({
        "schema_version":1,"kind":"stage_fixture","case_id":"DOMAIN_POPULATION_METRICS",
        "measured_at_unix_seconds":SystemTime::now().duration_since(UNIX_EPOCH).map_err(|_|"clock before epoch")?.as_secs(),
        "candidate_commit":candidate,"measurement_tool_dirty":dirty,"tool_version":env!("CARGO_PKG_VERSION"),
        "toolchain":env!("S10_RUSTC_VERSION"),"binary_sha256":digest(&fs::read(binary).map_err(|_|"binary read failed")?),
        "source_path":"rust/crates/dbrain-population/src/metrics.rs","source_sha256":digest(SOURCE),
        "dataset":"fixed integer ordering/overlap fixtures v1","seed":null,"warmup_batches":1,"cache_state":"process_warm",
        "hardware_fingerprint":digest(&serde_json::to_vec(&hardware).map_err(|_|"hardware serialization failed")?),"hardware":hardware,
        "raw_samples":samples,"batch_mean_pair_latency_ms":distribution(&per_pair_ms)?,
        "process_peak_ram_kib":proc_kib("/proc/self/status","VmHWM"),"quality_reference_passed":true,
        "e2e_latency_ms":null,"legacy_baseline":null,"speedup":null,"ingest_lag_seconds":null,
        "cpu_seconds":null,"io_bytes":null,"provider_cost":null,"release_approved":false
    }))
}
