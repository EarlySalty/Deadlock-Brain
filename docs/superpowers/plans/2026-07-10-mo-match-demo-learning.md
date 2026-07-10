# Mo & Krill Match Demo Learning Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Import every Mo & Krill match of `kaptennn`, extract evidence from one real demo, and produce a validated full report that cannot promote Brain knowledge before human calibration.

**Architecture:** Extend the existing Deadlock API source and player-learning path. Raw API responses remain in `source_documents`; match rows, demo evidence, and human reviews use `entity_snapshots`; generated reports continue to use `player_match_decision_notes`. The first tracer uses the hosted NDJSON demo-query API and existing Fireworks client, with no new service, database table, dependency, or local demo parser.

**Tech Stack:** Rust 2021, `reqwest` through `deadlock_brain_core::http`, `serde_json`, `sqlx`/PostgreSQL, `clap`, Fireworks OpenAI-compatible chat completions.

## Global Constraints

- Work only on `feat/mo-match-demo-learning`; preserve `data/last_patchnotes_sync_quality.json` unchanged and unstaged.
- Ignore unrelated migration and cutover documents; implement only this plan.
- Account ID is `281768392`, SteamID64 is `76561198242034120`, and Mo & Krill hero ID is `18`.
- Catalog every returned Mo & Krill match without result or patch filtering.
- Every analyzed decision must cite an evidence ID; deterministic match-header fields retain their source metadata. Interpretation and evaluation are separate fields.
- Store reports as `calibration_pending`; do not write active Brain rules, builds, or coaching knowledge.
- TDD for every non-trivial branch. No new crate or third-party dependency.
- After each task: focused tests, `cargo fmt --all -- --check`, own diff review, commit with the required co-author trailer, and push.

---

### Task 1: JSON POST in the existing HTTP client

**Files:**
- Modify: `rust/crates/deadlock-brain-core/src/http.rs`

**Interfaces:**
- Consumes: existing `HttpClient`, `HttpGetOptions`, `HttpResult`, and retry policy.
- Produces: `HttpClient::post_json<T: serde::Serialize>(&self, url: &str, body: &T, options: HttpGetOptions) -> Result<HttpResult>`.

- [ ] **Step 1: Write the failing local-server test**

Add a `#[cfg(test)]` module that binds `TcpListener` to `127.0.0.1:0`, accepts one request, and asserts both `POST /query` and the exact body `{"match_id":92242282,"format":"ndjson"}` before returning `202 application/json`.

```rust
#[test]
fn post_json_sends_body_and_reads_response() {
    let body = serde_json::json!({"match_id": 92242282_u64, "format": "ndjson"});
    let result = client.post_json(&url, &body, HttpGetOptions::default()).unwrap();
    assert_eq!(result.json::<serde_json::Value>().unwrap()["status"], "queued");
}
```

- [ ] **Step 2: Run the red test**

Run: `cargo test -p deadlock-brain-core post_json_sends_body_and_reads_response`

Expected: compile failure because `post_json` does not exist.

- [ ] **Step 3: Implement one shared request/retry path**

Keep GET caching unchanged. Add `post_json`, and factor the existing retry loop so GET and POST share status/transport retry behavior. POST must set `Content-Type: application/json`, use the supplied timeout/headers, and must not write to the GET cache.

```rust
pub fn post_json<T: Serialize>(
    &self,
    url: &str,
    body: &T,
    options: HttpGetOptions,
) -> Result<HttpResult> {
    let body = serde_json::to_vec(body)?;
    self.fetch_with_retry(url, &options, || {
        self.client.post(url).header(CONTENT_TYPE, "application/json").body(body.clone())
    })
}
```

- [ ] **Step 4: Run focused checks**

Run: `cargo test -p deadlock-brain-core post_json_sends_body_and_reads_response`

Expected: one passing test.

- [ ] **Step 5: Commit and push**

Commit: `feat: add retried JSON POST requests`

---

### Task 2: Catalog all Mo & Krill matches

**Files:**
- Modify: `rust/crates/dbrain-sources/src/deadlock_api.rs`
- Modify: `rust/crates/dbrain-sources/src/lib.rs`
- Modify: `rust/crates/deadlock-brain/src/main.rs`

**Interfaces:**
- Consumes: `SourceStore`, `HttpClient`, and `https://api.deadlock-api.com/v1/players/{account_id}/match-history`.
- Produces:

```rust
pub struct PullPlayerMatchHistoryOptions {
    pub account_id: String,
    pub hero_id: Option<u32>,
    pub cache_ttl_seconds: u64,
}

pub async fn pull_player_match_history(
    raw_dir: &Path,
    http: &HttpClient,
    options: PullPlayerMatchHistoryOptions,
) -> Result<Value>;
```

- [ ] **Step 1: Write parser tests before network code**

Use an inline history array containing a Mo win, a Mo loss from another timestamp, and another hero. Assert `filter_match_history(&payload, Some(18))` returns both Mo rows in source order and preserves `match_result`, `start_time`, and `team_abandoned`.

- [ ] **Step 2: Run the red parser test**

Run: `cargo test -p dbrain-sources deadlock_api::tests::filters_every_requested_hero_match`

Expected: compile failure because `filter_match_history` does not exist.

- [ ] **Step 3: Implement fetch and persistence**

Validate a numeric non-empty account ID. Store one raw source document with external ID `player-match-history:{account_id}`. Store each filtered row as:

```text
source        = deadlock_api
entity_type   = deadlock_api_player_match
external_id   = {account_id}:{match_id}
canonical_name= {match_id}
```

Add `_deadlock_brain` metadata containing `account_id`, `match_id`, `hero_id`, and the source URL. Return counts for API rows and stored rows.

- [ ] **Step 4: Add the CLI command**

Add `player sync-matches <account_id> --hero-id <id>`, defaulting `--hero-id` to `18`, with `--cache-ttl-seconds` and `--pretty`. Wire it directly to `pull_player_match_history` using existing settings and HTTP construction.

- [ ] **Step 5: Verify focused tests and CLI parsing**

Run: `cargo test -p dbrain-sources deadlock_api::tests`

Run: `cargo test -p deadlock-brain --bin deadlock-brain`

Expected: all selected tests pass.

- [ ] **Step 6: Commit and push**

Commit: `feat: catalog player matches from Deadlock API`

---

### Task 3: Fetch and persist versioned demo evidence

**Files:**
- Modify: `rust/crates/dbrain-sources/src/deadlock_api.rs`
- Modify: `rust/crates/dbrain-sources/src/lib.rs`
- Modify: `rust/crates/deadlock-brain/src/main.rs`

**Interfaces:**
- Consumes: Task 1 `post_json`, Task 2 match snapshots, demo endpoints `/v1/matches/demo/query` and `/v1/matches/demo/query/{job_id}`.
- Produces:

```rust
pub const DEMO_QUERY_VERSION: &str = "mo_full_report_v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DemoJobState { Queued, Running, Done, Failed }

pub struct DemoJobStatus {
    pub state: DemoJobState,
    pub result_url: Option<String>,
    pub error: Option<String>,
}

pub struct DemoQuery {
    pub name: &'static str,
    pub sql: String,
}

pub struct PullDemoEvidenceOptions {
    pub account_id: String,
    pub match_id: String,
    pub hero_id: u32,
    pub steam_id64: u64,
    pub poll_interval_seconds: u64,
    pub timeout_seconds: u64,
}

pub async fn pull_demo_evidence(
    raw_dir: &Path,
    http: &HttpClient,
    options: PullDemoEvidenceOptions,
) -> Result<Value>;

fn parse_demo_job_status(raw: &str) -> Result<DemoJobStatus>;
fn normalize_demo_rows(query_name: &str, raw: &str, match_id: &str) -> Result<Vec<Value>>;
fn demo_query_bundle(hero_id: u32, steam_id64: u64) -> Vec<DemoQuery>;
```

- [ ] **Step 1: Write failing pure tests**

Cover these contracts with inline JSON/NDJSON:

```rust
assert_eq!(parse_demo_job_status(r#"{"status":"done","result_url":"https://x/result.ndjson"}"#)?.status, DemoJobState::Done);
assert_eq!(normalize_demo_rows("combat", "{\"tick\":10}\n", "92242282")?[0]["evidence_id"], "combat:92242282:000001");
assert!(demo_query_bundle(18, 76561198242034120).iter().all(|query| query.sql.contains('"')));
```

Also test `failed` status preserves the API error and that malformed NDJSON fails rather than dropping a line.

- [ ] **Step 2: Run the red tests**

Run: `cargo test -p dbrain-sources deadlock_api::tests::demo_`

Expected: compile failures for the missing demo functions.

- [ ] **Step 3: Implement the minimal query bundle**

Use NDJSON and exactly three named queries:

1. `player_state`: target controller and Mo pawn deltas, including tick, entity index, Steam ID, pawn handle, lane, hero ID, health, level, net worth, K/D/A, last hits, coarse cell position, and ability/item vectors.
2. `target_combat`: Damage, HeroKilled, AbilityInterrupted, ImportantAbilityUsed, and StaminaConsumed rows involving the target pawn.
3. `economy_objectives`: item purchases, ability upgrades, currency changes, objective masks, boss kills, game-over, and team rewards.

Every camel-case demo column is double quoted. Numeric union columns are explicitly cast to `BIGINT` or `DOUBLE`; missing union fields use typed `CAST(NULL AS ...)` expressions.

- [ ] **Step 4: Implement submit, poll, download, and storage**

For each query: POST `{match_id, query, format:"ndjson"}`, poll until `done` or `failed`, fetch `result_url`, write the untouched NDJSON as a source document, normalize every non-empty row with `evidence_id`, `query_name`, `query_version`, and `match_id`, then persist one combined snapshot:

```text
source       = deadlock_api
entity_type  = deadlock_api_demo_evidence
external_id  = {account_id}:{match_id}:{query_version}
```

On HTTP 404/no demo, return a visible unavailable error and create no evidence snapshot. On timeout or failed job, preserve the job error in `source_runs` and do not mark the match analyzed.

- [ ] **Step 5: Add `player fetch-demo`**

Arguments: `<account_id> <match_id>`, defaults `--hero-id 18`, `--steam-id64 76561198242034120`, `--poll-interval-seconds 5`, `--timeout-seconds 900`, and optional `--pretty`.

- [ ] **Step 6: Run focused checks**

Run: `cargo test -p dbrain-sources deadlock_api::tests`

Run: `cargo test -p deadlock-brain --bin deadlock-brain`

Expected: all selected tests pass.

- [ ] **Step 7: Commit and push**

Commit: `feat: extract match evidence from demo queries`

---

### Task 4: Generate and validate the full report

**Files:**
- Create: `rust/crates/dbrain-learn/src/match_demo_learning.rs`
- Modify: `rust/crates/dbrain-learn/src/lib.rs`
- Modify: `rust/crates/deadlock-brain/src/main.rs`

**Interfaces:**
- Consumes: `deadlock_api_player_match`, `deadlock_api_match_metadata`, `deadlock_api_demo_evidence`, patch events, asset resolution, and existing `MiniMaxClient`.
- Produces:

```rust
pub const DEMO_REPORT_PROMPT_VERSION: &str = "mo_full_report_de_v1";

pub struct DemoAnalyzeMatchOptions {
    pub account_id: String,
    pub match_id: String,
    pub config: MiniMaxConfig,
    pub dry_run: bool,
    pub include_request: bool,
}

pub async fn demo_analyze_match(pool: &PgPool, options: DemoAnalyzeMatchOptions) -> Result<Value>;
pub fn validate_demo_report(report: &Value, evidence_ids: &BTreeSet<String>) -> Result<()>;
pub fn render_demo_report(report: &Value) -> Result<String>;
```

- [ ] **Step 1: Write failing validator and renderer tests**

Use one minimal valid report with metadata, all phase keys, one decision, one hypothesis, and one data gap. The decision must contain `observed_state`, `action`, `effects`, `interpreted_intent`, `evaluation`, `alternative`, `confidence`, and `evidence_ids`.

Assert a known evidence ID passes, an invented ID fails, a missing interpretation/evaluation separation fails, and rendered Markdown includes the match ID, decision timestamp, evaluation, alternative, and evidence ID.

- [ ] **Step 2: Run the red report tests**

Run: `cargo test -p dbrain-learn match_demo_learning::tests`

Expected: compile failure because the module does not exist.

- [ ] **Step 3: Build the deterministic context**

Load the latest snapshots by exact external ID. Resolve the active patch as the newest distinct `patch_events.posted_at` not later than the match start; mark it `derived_from_patch_timeline`, and use explicit `unknown` when no patch exists. Include compact match metadata, all normalized evidence rows, static Mo mechanics, and prior human correction rules. Never label aggregate build data as exact match evidence.

- [ ] **Step 4: Build a JSON-only Fireworks request**

The system message requires German prose while preserving English game names. The user prompt includes the exact report shape and says every factual, interpretive, or evaluative decision field must cite existing evidence IDs; unsupported claims belong in `data_gaps`, not the report body.

- [ ] **Step 5: Validate before persistence**

Parse the model text as one JSON object, run `validate_demo_report`, render Markdown deterministically, and only then insert `player_match_decision_notes` with prompt version `mo_full_report_de_v1`, full report in `insights`, Markdown in `result_text`, and status `calibration_pending`. `--dry-run` stores `context_ready` and performs no model call.

- [ ] **Step 6: Add `player analyze-demo-match`**

Mirror existing model/tuning flags and `--dry-run`. JSON output includes note ID, report JSON, rendered report, model usage, patch provenance, and evidence count. `--pretty` prints the full Markdown report rather than a summary.

- [ ] **Step 7: Run focused checks**

Run: `cargo test -p dbrain-learn match_demo_learning::tests`

Run: `cargo test -p deadlock-brain --bin deadlock-brain`

Expected: all selected tests pass.

- [ ] **Step 8: Commit and push**

Commit: `feat: produce evidence-backed full match reports`

---

### Task 5: Persist human review, document, and prove one live report

**Files:**
- Modify: `rust/crates/dbrain-learn/src/match_demo_learning.rs`
- Modify: `rust/crates/dbrain-learn/src/lib.rs`
- Modify: `rust/crates/deadlock-brain/src/main.rs`
- Create: `docs/MATCH_DEMO_LEARNING.md`
- Modify: `CHANGELOG.md`

**Interfaces:**
- Consumes: Task 4 report note and decision IDs.
- Produces:

```rust
pub async fn save_demo_report_review(
    pool: &PgPool,
    note_id: i64,
    review: &Value,
) -> Result<Value>;

pub async fn demo_calibration_status(pool: &PgPool, account_id: &str) -> Result<Value>;
pub fn validate_demo_report_review(report: &Value, review: &Value) -> Result<Value>;
```

- [ ] **Step 1: Write failing review validation tests**

The accepted review shape is:

```json
{
  "reviewed_decision_ids": ["D1"],
  "corrections": [
    {
      "decision_id": "D1",
      "label": "falsch",
      "comment": "Die Alternative ignoriert den gegnerischen Cooldown.",
      "replacement": "Rueckzug bis der Cooldown abgelaufen ist."
    }
  ]
}
```

Accept only `falsch`, `unbelegt`, `Kontext fehlt`, and `wichtige Entscheidung uebersehen`. Reject unknown decision IDs, empty reviewed lists, duplicate corrections, and an empty comment.

- [ ] **Step 2: Run the red tests**

Run: `cargo test -p dbrain-learn match_demo_learning::tests::review_`

Expected: compile failure for missing review validation.

- [ ] **Step 3: Store review snapshots and calculate gate status**

Insert `source='human_review'`, `entity_type='player_match_report_review'`, external ID `{note_id}:{payload_hash}`, and the complete review plus critical-error and correction-rate fields. Calibration status considers reports in review time order, requires at least five reviewed reports, and passes only when the last three have no critical error and correction rate below `0.10` each.

- [ ] **Step 4: Add review CLI commands**

Add:

```text
player review-demo-report <note_id> --file <review.json> [--pretty]
player demo-calibration-status <account_id> [--pretty]
```

Read the review file with `fs::read_to_string`; do not accept free-form shell JSON.

- [ ] **Step 5: Add operational documentation and changelog**

Document the exact three-command flow (`sync-matches`, `fetch-demo`, `analyze-demo-match`), dry-run, review-file format, gate behavior, API rate limit, and why Steam fallback is deferred until report validation. Add a top changelog entry in the repository's existing format: bad static match learning -> real demo evidence plus review gate -> current calibration-pending behavior.

- [ ] **Step 6: Run the full Rust gate**

Run: `cargo fmt --all -- --check`

Run: `cargo test --workspace`

Run: `cargo clippy --workspace --all-targets -- -D warnings`

Run: `cargo build --release --workspace`

Expected: all commands exit zero.

- [ ] **Step 7: Run the live tracer**

Using secrets only through `/home/naniadm/Documents/Infisical/export_claude_secret.py`, run:

```bash
./rust/target/release/deadlock-brain player sync-matches 281768392 --hero-id 18 --pretty
./rust/target/release/deadlock-brain player fetch-demo 281768392 92242282 --hero-id 18 --steam-id64 76561198242034120 --pretty
./rust/target/release/deadlock-brain player analyze-demo-match 281768392 92242282 --pretty
```

If demo `92242282` is unavailable, select the newest catalogued Mo match with a downloadable demo and record that match ID in the verification output. Verify in PostgreSQL with muted/aggregate output only that the history snapshots, demo-evidence snapshot, and one `calibration_pending` note exist for the same match.

- [ ] **Step 8: Own review, commit, and push**

Commit: `feat: calibrate match reports with human review`

- [ ] **Step 9: Merge, build, and live proof**

Run the repository gate, merge to `main`, push, rebuild release artifacts, and rerun the same report command from `main`. Deadlock Brain has no long-running service in this tracer, so service restart is not applicable; the live proof is the real API/DB/report path.
