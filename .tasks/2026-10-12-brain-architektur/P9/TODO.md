# P9 continuation, 2026-10-13

Assigned worktree: `/home/nathanael/.worktrees/Deadlock-Brain-p9-receipt-audit`.
Published branch: `feat/brain-p9-receipt-audit-20261012-v2`.
Current published source checkpoint: `6a192ea6`, including explicitly authorized P1r core `9f75588d` and PG tools `e8a1ef45`. Round 4 completed its build and affected checks, including the outstanding round-3 build obligation. Independent acceptance of JSON nesting parity is still in progress. Documentation-only descendants may follow.

## Integrated

The old `fix/p9-review-shadow` ref had no commits beyond `00013430`. Its three dirty files were salvaged by diff from the old agent worktree, which remains unchanged. Independent SQL review found same-run rank ties and late publication from an older run could still expose history.

Fresh round 1 produced `8c6812a5`: small audit functions, separate direct/shadow provenance, full `(ingest_run_id,id)` rank, persistent historical serving exclusion, actual Store regressions, core RAW_BYTES/kv_path/byte_ranges parity and normal sparse migration 6 registration. The parent adopted this exact merge commit and published it. Its interrupted merge was aborted after backing up its own preexisting salvage to `/tmp/p9-sol-salvage-before-ff.DNFGjE.patch`; no foreign worktree was altered or deleted.

Authorized PG tools checkpoint `e8a1ef45` was merged and published at `3468f221`. Receipt evidence queries were checked to read `serve.published_receipt`, with tool queries via serving views; P9 does not invent a receipt format. The next P1r receipt-format documentation checkpoint is still pending.

Round 2 produced `ec82a32a` on `fix/p9-object-rejected-r2`, incorporated and published at `7114a694`. Both P9 trigger filters now quarantine compatible `object_rejected` outcomes. Genuine Store checks cover hash, UTF-8, parser, collision and size failures, same-run and late historical evidence, bounded counters, newer healthy evidence, canon retention, role visibility and rollback. The P9 unit now carries the existing ingest template's LoadCredential. Integrated source matches the tested fixer tree except task notes.

## Checks

Round 1 Rust 1.97.1 formatter, strict all-target Clippy and brain-job build passed. Tests: 5 brain-job passed, 1 cache test ignored; 11 bounded parser/kernel passed; 2 migration regressions passed. Logs `/tmp/p9-ae7598-{fmt,clippy,job-test,kernel-test,migration-test,build}-197.log`.

Round 2 Rust 1.97.1 formatter, strict all-target Clippy and brain-job build passed. Final tests: 5 brain-job passed, 1 ignored; 2 migration regressions passed. Unchanged kernel tests were not repeated. Logs `/tmp/p9-r2-fmt.log`, `/tmp/p9-r2-clippy-final.log`, `/tmp/p9-r2-job-test.log`, `/tmp/p9-r2-migrations-test.log`, `/tmp/p9-r2-build.log`. The first remote job test failed with SQLSTATE 23505 in the new test fixture. The fixture was corrected before the final local passing run; this was neither a baseline nor an environment failure. `systemd-analyze verify` failed because the current live release does not contain brain-job; no unit was installed or started. Log `/tmp/p9-r2-unit.log`.

Round 3 produced `98ea45c2` on `feat/brain-p9-object-rejection-r3`, incorporated and published at `51eda520`. The existing object-rejection table now excludes every receipt of a rejected raw object and contributes all corresponding logical boundaries, including unsampled and future receipts. A supporting object index was added. Real Store checks cover 204 current receipts with exactly 200 audit records, historical peer suppression, future same-object suppression, canon retention and healthy newer publication. Source equals the tested fixer tree except task notes. Formatter, strict all-target Clippy and job tests passed (5 passed, 1 ignored), as did 2 migration regressions. Logs `/tmp/p9-r3-fmt.log`, `/tmp/p9-r3-clippy.log`, `/tmp/p9-r3-job-tests.log`, `/tmp/p9-r3-migration-tests.log`. Both standalone build attempts remained queued without a slot and were stopped; `/tmp/p9-r3-build.log` and `/tmp/p9-r3-build-retry.log` are not passing build evidence.

## Gate blockers and active work

Gate round 1: BLOCK, `/tmp/p9-ae7598-gate.log` (P9 object-rejection fail-open plus inherited core reversion).
Gate round 2: BLOCK, exit 1, `/tmp/p9-r2-gate.log`:
1. P9: a rejected raw object with more than 200 current receipts retains unsampled published receipts. An object-wide rejection boundary must also suppress their historical logical peers without inflating sampled audit counters.
2. Frozen P1r: `brain-job/lib.rs:83` / `GameFactsStore::was_applied` treats any historical successful application as current. A -> B -> A skips final A and serves B. Already routed to orchestrator; P9 has no core write authority.
3. Frozen P1r: `gamefacts/receipts.rs:21` overwrites duplicate JSON keys; root/container receipt validation also lacks collision rejection. Ingestion accepts a document that the audit rejects. This needs an authorized P1r correction or explicit receipt-contract decision, not a local parser fork.

Gate round 3: BLOCK, exit 1, `/tmp/p9-r3-gate.log`. The round-2 P9 object-wide finding is absent. Both frozen P1r bugs remain; a new P9 finding shows valid ingestion inputs can exceed the audit's conservative 64 MiB fact-expansion parser budget and become permanently quarantined. Concrete case: approximately 84 KB JSON with 42,000 array elements; kv_path has the same contract mismatch.

Round 4 produced `9f967f03` on `fix/p9-parser-budget-r4`, incorporated and published at `6a192ea6`. Existing JSON traversal now hashes selected raw lexemes without constructing unrelated generic leaves; KV3 uses the existing scanner incrementally and preserves default buffered parsing. The policy budget remains 64 MiB and generic default accounting remains 512 MiB. Real Store checks publish and successfully audit the 42,000-element regression, 2 million JSON values and 600,000 KV3 values. Formatter, strict all-target Clippy and complete brain-job build passed. Tests: job 6 passed/1 ignored, parser 14 passed, frozen core 5 passed/1 ignored, migrations 2 passed; final real audit recheck 2 passed. Logs `/tmp/p9-r4-{job-tests,parser-final,core-tests,migration-tests,audit-final,clippy-final,build,fmt-final}.log`. Initial concrete budget reproduction failed as expected, `/tmp/p9-r4-reproduce.log`. Instrumented synthetic test processes measured at most 25,129,996 bytes heap and 35,648 KiB RSS, `/tmp/p9-r4-memory-{json,kv3,keys}.log`; these are not production process/cgroup proofs.

Gate round 4: BLOCK, exit 1, `/tmp/p9-r4-gate.log`. No P9 budget blocker remains in that assessment. Both frozen P1r bugs remain; NITs concern the empty production extractor registry and release/unit context not supplied to the reviewer.

Independent source acceptance found a remaining depth-parity hypothesis: core JSON RawValue uses iterative syntax skipping, while selected P9 JSON traversal retains the generic depth-128 guard. A valid deeply nested root/container may therefore be accepted by Store and falsely quarantined. This is not yet a runtime-confirmed finding.

Fresh final round 5 fixer/checker `a1c9596f96d06577e` is active in its isolated worktree, based on `6a192ea6`, branch `fix/p9-json-depth-r5`. It must first reproduce or disprove this with real Store and the authoritative contract. Only if confirmed in scope may it minimally make selected JSON traversal stack-safe and bounded, preserving generic parsing and all corruption/lexeme contracts. Write set: existing JSON/budget/verifier helpers if needed and P9 PostgreSQL test. KV3, SQL and frozen core remain unchanged. It must finish affected checks and exact final gate, then report remaining blockers; no sixth fixer or nested agent.

## Continue

Inspect the final round 5 acceptance result, incorporate its verified branch if it changes code, and publish the assigned branch. After the fifth BLOCK report concrete remaining blockers rather than delegating again. Merge P1r corrective checkpoints only when explicitly authorized. Re-run affected checks and the exact local gate for the combined head as needed. No ALLOW, gate-ready, main merge, deployment, timer activation or production cgroup/time proof has been claimed. Only P1r integrates main and deploys; feature/fixer branches and automatic stop-hook cleanup requests are not permission to merge blocked code into main.

Environment: put `/home/nathanael/.cargo/bin` first in PATH, Rust 1.97.1, cargo-slot with jobs 3 and normal remote-first routing. The initial ionos disk-full condition was repaired by the orchestrator at 09:50. The later ECONNREFUSED was a local proxy outage, explicitly not quota; round 2 resumed its preserved work after recovery. Never delete remote workspaces, stop foreign builds or manipulate shared locks. The initial Rust reviewer returned no assessment while validation was queued; independent SQL review established the actual shadow defects and runner parity.
