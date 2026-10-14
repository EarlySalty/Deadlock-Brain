# Wave-2 integration handover

Boundary: integrate exact approved commits in the order below, verify and push each green merge. No main update, gate, release, deployment, service restart or timer activation in this phase.

## Approved checklist

- [x] P3 `10502e6d7a8d6b3c8fd6e31bf0169ba29b3e6985`: merged without content conflicts; check, strict all-target Clippy, fmt and matching tests pass (182 passed, 0 failed/ignored, 131 filtered). Proof: `/home/nathanael/.local/state/brain-wave2-proof-20261014/P3-results.json`.
- [ ] E1 `0e1aa497bf9d35db083253ba66acfb817a733b91`.
- [ ] P2 `9dc39a74324c5ac7f6c88f66d4c28f6510565112`.
- [ ] P4 `35a54ce3465c02ca57bfa2b542481cee6ea2d7ab`.

Main registry at `46c75d67`: 1/7/8/9/10. Approved additions: P3 2; P2 3/11/13; P4 historical 4 plus reader-coherence 23. E1 has no migration in the approved object; allocation 27 is unused. Expected union: 1/2/3/4/7/8/9/10/11/13/23. Preserve SQL and stop on duplicate registered versions.

## Conflict resolutions

P3: none. Main core files are retained except the approved package delta. The first merge invocation reported an existing MERGE_HEAD; inspection confirmed the exact approved P3 SHA, unchanged starting HEAD and no unresolved entries. Completed same-session background verification is retained without repeating its test suites.

## Parent acceptance and live obligations

After final forwarded ALLOW or explicit close: one combined matching test run, one local gate on the combined SHA, authorized main integration, one release build and deployment through the existing wrapper. Verify released SHA, running executables, journals and endpoint behaviour before cleanup.

- P3: real timer ingest/repeat and canonical receipts; normal answer plus trace; successful job/cgroup peak and unit ownership. No invented Soul Urn first-spawn fact.
- P2: Toxic Bullets cost/effects, Haze/Wraith weapon comparison and positive hero slot-4 ultimate via the normal answer path; canonical receipts and answer traces without unwanted `deadlock_data`; Assets cgroup peak below 300 MB and controlled schedule activation. Two earlier package Unavailable outcomes remain unexplained, so check them in combined/live acceptance.
- P4: productive patch_history composition and cold/warm traces; configured read-only feed ingest/repeat raw delta 0; oversized-row skip/log/count and later-post continuation; publication/quarantine revalidation and historical_patch_unavailable; unchanged serialized bound and readonly/timeout enforcement. Verify assigned job schedule without starting brain-maintenance or dl-knowledge.
- E1: preserve the single pre-cut live baseline and offline rescore evidence in `E1/baseline-20261013.json` and `.txt`; run the agreed post-release evaluation against the deployed combined release, not another pre-cut live baseline.

Nonblocking follow-ups: P4 `rust/crates/brain-job/src/patch_extract.rs:368` merges consecutive bullets at lowercase continuation. Coordinator preflight reports `deadlock-brain-postgresql-backup.service` already failed before wave 2; out of scope, not a regression. That preflight saw release `46c75d67`, listening ports 8788/8087 and no Brain job timers; verify current state again at rollout.
