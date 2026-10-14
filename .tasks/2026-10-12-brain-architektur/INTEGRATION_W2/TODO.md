# Wave-2 integration handover

Scope: merge the four exact approved commits in order on `integrate/brain-wave2-20261014`, verify and push each green merge. This worker does not run the final gate, update main, build a release or deploy.

## Approved checklist

- [x] P3 `10502e6d7a8d6b3c8fd6e31bf0169ba29b3e6985`: automatic merge without conflicts; check, strict all-target Clippy and fmt exit 0; matching tests 182 passed, 0 failed, 0 ignored, 131 filtered.
- [ ] E1 `0e1aa497bf9d35db083253ba66acfb817a733b91`.
- [ ] P2 `9dc39a74324c5ac7f6c88f66d4c28f6510565112`.
- [ ] P4 `35a54ce3465c02ca57bfa2b542481cee6ea2d7ab`.

Actual starting main registry: `[1,7,8,9,10]`. Approved additions: P3 `[2]`, P2 `[3,11,13]`, P4 `[4,23]`. E1's approved tree has no GameFacts migration or version 27 registration; no migration is invented from its reservation. Expected final registry: `[1,2,3,4,7,8,9,10,11,13,23]`. No registered collision was found and no SQL was renumbered.

## Conflict resolutions

P3: none. Subsequent resolutions are recorded here as the merges complete.

## Subsequent integrator obligations

- After the final forwarded ALLOW or explicit close, run the combined matching tests and local gate, then perform the authorized main merge and single release/deployment through the existing wrapper. Verify exact live release SHA, executable/process changes, journal errors and endpoint content before cleanup.
- P3: real Timer ingest/repeat, grounded answer with canonical receipts and `ops.answer_trace`, actual job unit/cgroup peak and timer-key access. Offline index/RSS evidence is not live-unit proof; unsupported Soul Urn first spawn must remain evidence-free.
- P2: normal `/v1/answer` for Toxic Bullets cost/effects, Haze/Wraith weapon comparison and positive hero+slot 4 ultimate selection with canonical receipts; inspect tool trace without the unwanted `deadlock_data` path. Check actual Assets cgroup peak below 300 MB and controlled timer activation. Retain the two unexplained historical `Unavailable` outcomes as an operational risk for combined/live checks.
- P4: productive patch_history with FindEntity/GetHero, cold/warm numeric traces and matching receipts; verify feed ingest/repeat raw delta 0, oversized-row skip/log/count and later-post continuation, publication/quarantine revalidation and historical_patch_unavailable, serialized bounds and readonly/timeout restrictions. Verify actual unit ownership/schedule; do not start brain-maintenance or dl-knowledge.
- E1: compare final live gold-set results against `.tasks/2026-10-12-brain-architektur/E1/baseline-20261013.json` and `.txt`. Baseline is one live run re-scored offline, not a second run: 125 cases, answerable 11/101 answered and 6/101 correct, 6/11 precision, 11/11 cited, non-answerable 20/24 compliant. Preserve the source-check caveats in E1/TODO.md.
- Nonblocking P4 follow-up: `rust/crates/brain-job/src/patch_extract.rs:368` may merge consecutive bullets at a lowercase continuation. Do not treat it as an integration blocker.

Evidence: `/home/nathanael/.local/state/brain-wave2-proof-20261014/`, including approved task snapshots, identity/registry checks and per-stage exact commands, exits and test counts. PostgreSQL tests use the existing private `ScratchPg` helper with a unique temporary directory/socket and automatic stop; no production DSN or test container is used by this worker. No inherited test-failure claim is made.
