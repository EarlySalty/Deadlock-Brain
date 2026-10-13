# P9 Sol continuation, 2026-10-13

Assigned worktree: `/home/nathanael/.worktrees/Deadlock-Brain-p9-receipt-audit`.
Published branch: `feat/brain-p9-receipt-audit-20261012-v2`.
HEAD and origin: `8c6812a5f8af83d9c0471f8fa7e071cd8db08fa0`, merge parents exactly `00013430` and authorized core 2b `9f75588d`.

## Salvage and integration

`fix/p9-review-shadow` had no commits beyond `00013430`. Its three dirty files were copied by diff from the old agent worktree, which remains unchanged. The abandoned changes were incomplete: independent SQL review found same-run rank ties and late publication from an older run could still expose historical evidence.

Fresh fixer `ae7598eee9dffa83a` produced and pushed `8c6812a5` on `fix/brain-p9-boundary-ae7598`. It preserves the small audit functions, separates direct/shadow provenance, uses full `(ingest_run_id,id)` rank and a persistent serving exclusion boundary, tests both historical-evidence defects through the real Store, adopts core RAW_BYTES/kv_path/byte_ranges parity, and registers migration 6 through the normal sparse migration path. The parent adopted its exact merge commit by fast-forward and pushed the assigned branch. The parent's interrupted core merge was aborted only after preserving its own preexisting salvage in `/tmp/p9-sol-salvage-before-ff.DNFGjE.patch`; only those backed-up own files were cleared before fast-forward. No foreign worktree was changed or deleted.

## Verification and gate

Rust 1.97.1, strict all-target Clippy and formatter passed. Tests: 5 brain-job passed, 1 cache test ignored; 11 bounded parser/kernel passed; 2 migration regression tests passed. Total 18 passed, 1 ignored. Logs: `/tmp/p9-ae7598-fmt-197.log`, `/tmp/p9-ae7598-clippy-197.log`, `/tmp/p9-ae7598-job-test-197.log`, `/tmp/p9-ae7598-kernel-test-197.log`, `/tmp/p9-ae7598-migration-test-197.log`. Explicit job build passed under Rust 1.97.1, exit 0, log `/tmp/p9-ae7598-build-197.log`. No failed test was classified as preexisting.

Local gate round 1 is BLOCK, exact output `/tmp/p9-ae7598-gate.log`:
- Frozen core `GameFactsStore::was_applied`, gamefacts.rs:222, treats any historical successful raw/extractor pair as current. Source A -> B -> A skips final A and keeps B served. The shared skip is brain-job/lib.rs:83. This affects all canonical serving types and must be routed to P1r; P9 has no authority to modify these core files.
- P9 SQL quarantine filters at 63 and 81 exclude `object_rejected`, leaving compatible raw/locator/hash/parser rejection receipts visible. This is in P9 scope and is being fixed in a fresh round.
- Nonblocking unit nit: audit service lacks the existing ingest template's LoadCredential despite sharing Config::pool and possible Infisical authentication. Round 2 may add the existing managed bootstrap credential to the P9-owned unit, never read or create token files.

Fresh fixer `a406544e9dc9c2bc6` is active in its isolated worktree at base `8c6812a5`. Owned writes are the P9 SQL migration, real PostgreSQL test and P9 service only. It must quarantine compatible object rejections, preserve unsupported-kind skipping and historical/newer evidence behavior, run matching checks once, commit/push and re-gate. Frozen core is read-only. No further agents or T3 threads are authorized inside a fixer.

## Continue

Incorporate round 2's verified branch, inspect its acceptance evidence and publish the assigned branch. Route the A -> B -> A gate blocker to P1r and merge only an explicitly authorized corrective checkpoint. Then rerun affected checks and the exact local gate. Only P1r integrates main and deploys. No ALLOW, main merge, deployment, timer activation or production cgroup/time proof has been claimed.

Environment: put `/home/nathanael/.cargo/bin` first in PATH; the session otherwise resolves system Cargo 1.75. Initial remote builds could not write Git config because ionos was full; the orchestrator repaired capacity at 09:50. Normal remote-first cargo-slot works again. Do not delete remote workspaces, stop foreign builds or manipulate shared locks. The first Rust reviewer returned no assessment while validation was queued; the independent SQL reviewer established the two actual defects and the runner refactoring parity.
