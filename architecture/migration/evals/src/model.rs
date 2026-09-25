use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Catalog {
    pub schema_version: u32,
    pub cases: Vec<Case>,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Case {
    pub id: String,
    pub class: String,
    pub scenario: String,
    pub owner: String,
    pub expected: String,
    pub checks: Vec<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Versions {
    pub baseline_commit: Option<String>,
    pub candidate_commit: Option<String>,
    pub contract: Option<String>,
    pub db_schema: Option<String>,
    pub knowledge: Option<String>,
    pub corpus_sha256: Option<String>,
    pub answer_model: Option<String>,
    pub embedding: Option<String>,
    pub search: Option<String>,
    pub jev: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Workload {
    pub seed: u64,
    pub repetitions: u32,
    pub cache_states: Vec<String>,
    pub target_arrivals_per_second: Option<f64>,
    pub target_concurrency: Option<u32>,
    pub duration_seconds: Option<u32>,
    pub background_load: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Acceptance {
    pub p95_ms: Option<f64>,
    pub p99_ms: Option<f64>,
    pub error_rate_max: Option<f64>,
    pub ram_mib_max: Option<f64>,
    pub cost_per_accepted_answer_max: Option<f64>,
    pub ingest_lag_seconds_max: Option<f64>,
    pub recall_noninferiority_margin: Option<f64>,
    pub answer_quality_noninferiority_margin: Option<f64>,
    pub rpo_seconds: Option<f64>,
    pub rto_seconds: Option<f64>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Profile {
    pub schema_version: u32,
    pub approved: bool,
    pub approval_reference: Option<String>,
    pub versions: Versions,
    pub dataset_sha256: String,
    pub hardware_fingerprint: Option<String>,
    pub workload: Workload,
    pub acceptance: Acceptance,
    pub g0_evidence_sha256: Option<String>,
    pub g1_evidence_sha256: Option<String>,
    pub g2_evidence_sha256: Option<String>,
    pub g3_evidence_sha256: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Check {
    pub id: String,
    pub passed: Option<bool>,
    pub evidence_sha256: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Costs {
    pub answer: Option<f64>,
    pub embedding: Option<f64>,
    pub jev: Option<f64>,
    pub shadow: Option<f64>,
    pub retries: Option<f64>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage {
    pub name: String,
    pub elapsed_us: u64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Sample {
    pub id: String,
    pub case_id: String,
    pub repetition: u32,
    pub cache_state: String,
    pub scheduled_us: u64,
    pub started_us: u64,
    pub finished_us: u64,
    pub outcome: String,
    pub checks: Vec<Check>,
    pub stages: Vec<Stage>,
    pub tokens: Option<u64>,
    pub costs: Costs,
}
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Resources {
    pub peak_ram_mib: Option<f64>,
    pub cpu_seconds: Option<f64>,
    pub io_read_bytes: Option<u64>,
    pub io_write_bytes: Option<u64>,
    pub ingest_lag_seconds: Option<f64>,
    pub rebuild_seconds: Option<f64>,
    pub max_queue_depth: Option<u64>,
    pub recovered_after_overload: Option<bool>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Run {
    pub schema_version: u32,
    pub kind: String,
    pub variant: String,
    pub versions: Versions,
    pub hardware_fingerprint: String,
    pub profile_sha256: String,
    pub dataset_sha256: String,
    pub evaluator: String,
    pub label_revision: Option<String>,
    pub workload: Workload,
    pub measurement_started_us: u64,
    pub measurement_finished_us: u64,
    pub offered_requests: u64,
    pub raw_artifact_sha256: Option<String>,
    pub resources: Resources,
    pub samples: Vec<Sample>,
}
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Distribution {
    pub count: usize,
    pub p50: Option<f64>,
    pub p95: Option<f64>,
    pub p99: Option<f64>,
    pub mean: Option<f64>,
    pub stddev: Option<f64>,
}
#[derive(Clone, Debug, Serialize)]
pub struct Summary {
    pub requests: usize,
    pub accepted: usize,
    pub error_rate: Option<f64>,
    pub end_to_end_ms: Distribution,
    pub queue_ms: Distribution,
    pub successful_requests_per_second: Option<f64>,
    pub tokens: Option<u64>,
    pub cost_per_accepted_answer: Option<f64>,
    pub stages_ms: std::collections::BTreeMap<String, Distribution>,
}
#[derive(Debug, Serialize)]
pub struct Assessment {
    pub status: String,
    // NEVER authorize a merge or cutover; this is only an assessment proposal.
    pub release_approved: bool,
    pub blockers: Vec<String>,
    pub failures: Vec<String>,
    pub summary: Option<Summary>,
}
