# P1 final numeric storage-bound correction

## Goal and exact defect

The required whole-diff reviewer completed its read against c19d5b26 and all gate-one-to-four findings. Seven dispositions are resolved/deferred; the gate-two deterministic target wedge reopens through numeric storage width. This is a pre-gate correction, not an actual fifth gate: gate five has not run and remains the final permitted gate round.

Read /tmp/p1-complete-review-20261012-0om208yw/review-result.json and numeric-storage-proof.json. Source 1e64 passes extraction/proof because its lexeme is short, then numeric storage expands to 65 characters and violates fact_value_num_check (SQLSTATE 23514). 1e-64 expands to 66. Ordinary SQL error leaves the immutable target partial, so the next invocation retries it before fetching a corrected source. Both Wiki JSON MaxHealth and GameTracking EMaxHealth/m_strValue numeric branches are affected.

Fix the originating extractor's numeric storage-width/domain validation before inserts, preserving the existing SQL bound. Unsupported source values must reach existing InvalidTarget and terminal failed/no-publication/ops failure. Use the existing separate-rerun/full-snapshot path. Do not widen SQL, blanket-classify SQL/database failures, add recovery, change supported lexemes/hashes, bump extractor version or add a numeric library. Reuse existing numeric parsing/validation where possible. Check directly analogous exponent/sign/scale boundaries so the same storage mismatch does not remain. Preserve supported values exactly.

## Ownership and proof

Only write /home/nathanael/.worktrees/Deadlock-Brain-p1-numeric-fix-20261012, detached c19d5b26dcdffedfe878799f6c72a69a59b3db38. Minimum extract.rs and concrete existing numeric/recovery fixture modules. Schema/SQL, storage lifecycle, serving, contracts, release helper, units, rules and all foreign/parent trees are read-only. No worker commit, push, gate, deploy, agent or T3 thread. Parent owns exact-patch integration and actual gate five. No additional whole-diff review or optional polish.

Real owned scratch Git/Postgres regression for both sources: unsupported numeric source revision becomes terminal failed, ops failure recorded, no publication/head movement, audit retained; a separate corrected revision succeeds, unchanged follow-up adds zero raw objects. Ordinary transient interruption remains partial/resumable. Verify supported numeric boundaries against actual PostgreSQL representation and original proof hashes, not just a self-authored expectation. Add only this concrete regression. Preserve earlier canonical metadata/localization/upgrade-tier/identity/publication fixes.

Use LIVE /home/nathanael/.local/bin/cargo-slot, never the historical Documents/claude-config/bin source script. Matching formatter, strict all-target Clippy, optimized check/build and affected existing tests; locked, jobs 3, shared sccache, CARGO_INCREMENTAL=0, RUSTFLAGS=--remap-path-prefix=/home/nathanael=/brain-operator, RUSTC_WRAPPER=/home/nathanael/.cargo/bin/sccache, SCCACHE_DIR=/home/nathanael/.cache/sccache. Default remote routing; local only for actual scratch PG/public-fixture dependence. Explicit cargo fmt package selection. No repeated unaffected/full-repo suites or competing production release.

## Rules and handover

English internal work and result. German only product text. No code comments, production unwrap, needless unsafe, em dashes, secret/credential-file reads, environment dumps, unredacted production DB output or private external data. Postgres only. Graphify/code-suche first using the existing shared graph if local graph absent; no full extraction. Browser only Moli with guide first if needed. Never Sonnet. Queried fixer order is sol, grok, opus55, astra; inherit authorized native gpt-6.1-sol with no model/effort change.

Save a complete patch, base/verified source hashes, exact commands, actual full logs/counts and independent PG boundary proof in a fresh /tmp/p1-numeric-fix-121638e5-* directory. Return promptly after verification with FIXED/BLOCK, paths, counts and open points. No packaging/audit loop or ALLOW/main/live claim. Parent session 121638e5-3cc5-4f1c-870e-ccf3f7d0895b; main orchestrator 89e002a7-758e-449d-9968-eec29a019a78. Native return only, no SendMessage to active actors or user questions.
