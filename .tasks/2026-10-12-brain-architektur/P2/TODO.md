# P2 continuation under the P1 compiler freeze

Recorded 2026-10-12. This is open work, not package acceptance or a live handover.

## Saved package

Parent worktree: `/home/nathanael/.worktrees/Deadlock-Brain-p2-items-abilities`.
Branch: `feat/brain-p2-items-abilities-20261012`, pushed to origin.
Coupled code checkpoint: `2fd36868bb8feeb8e41aeab721233513de60caac`.
Latest documentation checkpoint before this file: `d805474209f490c2e4b36848203d2e0bcc810183`.

The coupled checkpoint contains original tools/cards plus corrected consumer `a1edfdf24fad2c2e9150e64558a57e54d313c94e` and essential-card producer `19f5876bfc527727cae8531c17cb4c0679b13074`. Their ancestry was actually checked with successful `git merge-base --is-ancestor` calls. Component proofs and their limitations are in `CHECKPOINTS.md`; the parent dependency baseline is in `BASELINE.md`. No final combined build, gate, main merge, deployment, production job or timer activation is claimed.

## Binding hold

User directive 08:12: P1 live has top priority. Start no new cargo-slot test, clippy or build jobs until P1 is live AND the orchestrator explicitly releases slots. Already running jobs may finish. Both fresh fixers acknowledged zero running or queued compiler jobs when informed. Read-only review, scoped source/SQL work, notes and formatting may continue; do not substitute compiler-free test execution for the requested reading-only validation hold. Do not ask P1 for additional interfaces.

A post-directive fetch still showed P1 interface `3a0740be` beneath documentation `b4f3c32c`, origin/main `edf503b2`, and current release `ef0418d5`. These observations are not a P1 live proof. Adopt the ordered runner/item-kind checkpoint P1 actually ships; reserve only P2 version 3. Never change P1's entity kinds or invent a separate runner.

The stop hook lists these session-owned open branches. Its suggested individual main merges must not bypass this hold or publish known-defective original slices. Keep the original branches/worktrees until final integrated acceptance and actual main ancestry permit cleanup. Do not modify a foreign main checkout or delete unmerged work to silence the hook.

## Isolated non-build fixes

- `fix/p2-assets-replay-proofs`: current-published-raw extractor replay before HTTP, and exactly-once leaf receipt proof for every fact/edge. Ownership: Assets root/storage/extraction and their concrete contract tests. Interim code report says replay never inserts a new raw object and reuses fencing/publication; this is not executed proof.
- `fix/p2-semantic-source-selection`: known provenance-only selection keys, full winning condition retained, unknown/genuine conditions kept distinct. Ownership: the own still-unapplied P2 SQL and its schema contracts. A same-source collision conservatively disables normalization across the entire ambiguous semantic equivalence class. Single-use windows retain entity/stat partition keys, but index/filter pushdown and timing remain unverified.

Fresh fixers report to this parent. Do not duplicate live writers or read their raw transcripts. Preserve coherent local checkpoints, with exact unexecuted checks disclosed. Neither fix is accepted merely because code or new tests exist.

## Resume after explicit release

1. Adopt P1's actual published checkpoint by merge commit. Integrate the completed fresh fixes into the coupled package, never only the original defective Assets branch. Delegate only narrowly owned migration version-3 registration and integration adjustments. Shared extractor version becomes the then-current origin/main value plus one; actual tool-count assertions follow the merged registry.
2. Run one sequential cargo-slot validation lane under the documented shared-build policy: pinned 1.97.1, locked release, three jobs, sccache, remapped paths and no target copies. Execute matching build/clippy/format checks and affected existing suites. Run the actual positive P1 item/edge ingestion, complete public EN/DE replay, changed-extractor raw-only replay/idempotency, prover mutations, source-priority and actual producer-to-tool contracts. Record precise commands/counts and distinguish skips from executed bodies.
3. Complete independent Rust and coupled intent review on the final package. The earlier Rust-review execution failed on invocation errors and is not approval. Load `rolle-merge-schleuse`, run the local merge gate against the actual updated integration base, fix confirmed findings with fresh contexts and require ALLOW. Only the responsible parent integrator may publish the final verified integration to main.
4. Deploy verified origin/main through brain-release after P1 live. Rebuild/apply the real ordered migrator before dependent code. Prove the system job's actual cgroup-v1 peak below 300 MB and an unchanged second run with zero new raw objects AND facts. Prove normal `/v1/answer` Toxic Bullets and Haze/Wraith acceptance and matching `ops.answer_trace` get_item/compare without deadlock_data, actual four-by-twelve serialized ToolResult at most 16,384 bytes, original-value receipts and genuine indexed item/compare EXPLAIN plans without planner tricks. Activate the hourly system timer only after the real successful job evidence.
5. Supply the live PID/exe/journal/binary/function proof and real URL/API location. Verify each own branch is actually an ancestor of origin/main, then clean up only those own merged branches/worktrees. Any PR created or worked on must be linked to this thread, but none currently exists.
