# O1 maintenance ownership and cutover

## Existing producers

All six legacy timers were enabled but inactive. Their `ActiveExitTimestamp` is 2026-10-09 08:31:08 CEST. The user journal records an explicit stop for each at that timestamp. This was an uncompleted cutover, not six independent timer failures. No automatic reactivation of the old producers is safe.

| Legacy timer | Output and last legacy result | Maintenance replacement | Replacement proof before retirement |
| --- | --- | --- | --- |
| deadlock-brain-build-data | Assets and all-hero build snapshots; success 2026-10-09 03:48:32 CEST; old b7289d11 runtime | `rust/crates/brain-maintenance/src/integration/data_jobs.rs:23`, AssetsBuildData: current installed Rust CLI, daily, aggregate timeout 60 minutes | Required: maintenance_data_jobs_v1.last_success_at after deployment |
| deadlock-brain-patchnotes-sync | Legacy patchnote DB sync; success 2026-10-09 08:28:52 CEST; old shell/binary path | `rust/crates/brain-maintenance/src/integration/runner.rs:703`, patch_feed mirror and canonical patchnotes ingestion | Completed maintenance status 2026-10-11 18:47:55 UTC: 132 source posts, newest post 292, 132 live canonical documents, status up_to_date |
| deadlock-brain-sheet-sync | Sheet tabs and stats, then legacy analysis/enrichment; success 2026-10-09 07:29:34 CEST; Python | `rust/crates/brain-maintenance/src/integration/data_jobs.rs:27`, SheetSync: Rust refresh-sheet imports and normalizes every tab, every 4 hours, timeout 10 minutes | Required: maintenance_data_jobs_v1.last_success_at after deployment. Paid legacy analysis/enrichment is not silently rescheduled |
| deadlock-brain-wiki-refresh | Deadlock-data import and legacy Markdown cards; success 2026-10-09 06:35:33 CEST; fixed old binary | `rust/crates/brain-maintenance/src/integration/runner.rs:794`, refresh_entity_profiles and `rust/crates/brain-maintenance/src/integration/entity_profiles.rs:210`, refresh_git_knowledge: pinned original game documents and derived profiles. Canonical raw cache is explicitly bound to the same source pin | Required: successful import/cache proof on current upstream client. The replacement serves typed source documents, not the old Markdown directory |
| deadlock-brain-youtube-learning | Curated video discovery, transcripts, classification and Gemini claims; failed 2026-10-09 04:39:51 CEST, missing config file | None in O1. Missing config/gemini-browser.json in immutable release; legacy config has learning_enabled=false; the implementation starts Brave and uses direct Gemini | Keep stopped and unchanged. O1Y is a separate external-AI/browser product decision |
| dl-brain-feeder | Weekly aggregate digest and plans into Second-Brain Wiki; success 2026-10-04 19:00:31 CEST | None in O1. Existing producer is Deadlock-Bots Rust bin/dl-brain-feeder (99 KB plus dependencies and deployment) | Keep stopped and unchanged. O1F is queued separately |

The existing patchnotes unit has no tracked deployment definition in this repository. Its safe command/schedule history is retained below. Remove the legacy active definitions for the other replacement-backed rows only after successful replacement proof. Do not retire YouTube or feeder.

Patchnotes history: service ExecStart=/home/naniadm/.local/bin/deadlock-brain-patchnotes-sync.sh; timer OnBootSec=2min, OnUnitActiveSec=5min, Persistent=true, WantedBy=timers.target. No credential configuration is copied into this record.

## Why the policy error occurred

`brain-storage/src/pg_maintenance.rs`, lock_maintenance_provider_policy, rejected both a changed access policy and a different repository commit with the same message. In the live failing local-563766 and local-5b69ba jobs, the policies were equal. Only source_sha differed: a15df92b331ad206504c61a5099afbb983409e9d versus registered 7b0f0e645f026214f28b99dddbb18bd477bc2819.

The local jobs came from an earlier reviewed public-doc import which is no longer in runtime.local_imports. Normal ticks created later successful jobs and advanced their registration. The abandoned local jobs remained claimable. The old provider guard incorrectly required equality of the entire repository SHA even when the registered source paths had no changes. The diff of the five registered Deadlock-Docs source paths between those two commits is empty.

The fix has two parts:

1. The provider guard holds the registration row lock and checks the exact access policy and source identity. Both the registered commit and job's pinned evidence must still match the current relevant source paths. An unrelated commit is allowed; relevant changes, revoked policy, changed document basis and invalid evidence still prevent dispatch. The original strict storage guard remains available for callers requiring exact SHA equality.
2. Normal ticks reconcile abandoned local jobs only when the import is no longer retained and a newer same-target, same-policy job succeeded. Live leases and publication proofs are untouched. The old checkpoint, error and history remain; superseded_by points to the successful replacement. This does not rewrite a policy or manually edit a DB job.

No provider, model, reasoning effort or cost setting changes.

The internal Deadlock-Docs target `internal/public-candidates/docs.html` has provider_egress_allowed=false and no code-only migration authorization. Its latest job currently has matching registry policy and SHA, so it is not the same drift case as the abandoned local jobs. Automatic provider stages end as failed with error_code=POLICY_BLOCKED and blocked_reason=PROVIDER_EGRESS_DENIED. They release the lease and are not retried or added to isolated_errors. Reviewed local imports and explicitly authorized code-only migrations retain their existing paths. Confirm that terminal state live after deployment; do not enable external egress to make the review succeed.

## Branch verification

The existing release suites passed with 174 passed, 0 failed and 15 ignored. The isolated storage queue contract separately passed once with 0 ignored. The ignored terminal-policy, reconciliation, scheduler and provider-dispatch regressions are still required; these suite results do not prove those paths or production operation.

The suite command used CARGO_INCREMENTAL=0, RUSTFLAGS=--remap-path-prefix=/home/nathanael=/brain-operator, RUSTC_WRAPPER=/home/nathanael/.cargo/bin/sccache, SCCACHE_DIR=/home/nathanael/.cache/sccache and BRAIN_MAINTENANCE_TEST_PG_SOCKET=/tmp/brain-o1-tests.NI4H1P/.maintenance-test-pg:

```text
cargo-slot test --manifest-path rust/Cargo.toml --locked --release --jobs 3 -p brain-maintenance -p brain-storage
```

The additive migration executed on that owned test cluster. The scheduler grants permit brain_ingest SELECT/INSERT/UPDATE and brain_readonly SELECT, without granting brain_readonly INSERT. A rolled-back probe verified that an alias table without an id column does not fail sequence lookup. Owned Rust formatter checks and `git diff --check` passed. Production configuration, services and legacy timers have not been changed.

TESTNACHWEIS[TW-1]: 174 passed, 15 ignored | Baseline: not measured; no pre-existing failure claim

## Deployment in the granted merge turn

Data jobs default to an empty list. The example schedules are inactive until added to the live runtime configuration in the granted turn.

1. Rebase, run affected checks and obtain gate ALLOW. Send READY TO MERGE O1. No brain-serve restart before the turn.
2. Use the existing brain-release plan/build/verify/install wrapper on verified origin/main. Apply the additive 2026-10-11-brain-maintenance-data-jobs-v5.sql migration as the existing migration owner. The migration grants only the normalized-entity writes already needed by the registered canonical import; live brain_ingest previously lacked those rights.
3. Add data_jobs from data-jobs.example.json to /etc/deadlock-brain/maintenance-runtime.json. Set canonical_raw_dir on the deadlock-wiki-deadlock-data entry, not on GameTracking-Deadlock, to /home/nathanael/.local/share/deadlock-brain/raw. Preserve all other runtime values. Keep the existing credential FD/Infisical configuration. These are normal configuration values, not secrets.
4. Run the normal maintenance service. Its persistent PG schedule prevents duplicate work, uses a fencing token and bounded subprocesses, records last_success_at and retries failures on the existing retry delay. It only executes the installed Rust data commands through the existing Rust Infisical wrapper. Raw subprocess output is not logged or persisted; the result stores command names and output hashes.
5. Verify the current upstream commit and ClientVersion against the canonical cache and imported source documents. Check local-563766, local-5b69ba and the Deadlock-Docs review job in DB status. Require successful replacement timestamps for assets/build-data and sheet. Disable the replacement-backed legacy timers and remove their active tracked deploy definitions, preserving their Git history. Confirm brain-maintenance.timer still has a next run and no duplicate legacy timer is active.

Operational success is separate from branch tests. A paused YouTube command is not a successful learning run. An isolated_errors entry is not a clean import. Build-data completion is not inferred from an assets-only success.
