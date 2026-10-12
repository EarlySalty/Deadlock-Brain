# P2 implementation checkpoints

These are component handovers, not package acceptance or live proof.

## Tools

Commit `7b9bd3ec1c6bce5426b44195c210bc8017d552b2`, clean worker checkout at `/home/nathanael/repos/Deadlock-Brain/.claude/worktrees/wf_61813c32-183-1`.

Worker-owned files are `brain-contracts/src/asset_tools.rs`, append-only `tools.rs`, `brain-serve/src/pg_asset_tools.rs`, `pg_asset_tools_tests.rs`, append-only `pg_tools.rs`, and the narrow existing definition-count assertion in `pg_tools_tests.rs`.

Parent inspected the actual log markers:

- Release `brain-serve` build: completed in 10m 33s, `cargo-slot: EXIT=0`, `/tmp/p2-tool-build-wf_61813c32.log`.
- Release clippy on `brain-contracts` and `brain-serve`, all targets, `-D warnings`: local fallback completed in 9m 02s, `cargo-slot: EXIT=0`, `/tmp/p2-tool-clippy-wf_61813c32.log`. Its initial remote manifest failure was an environment failure, not a source failure.
- Release library tests: 65 contracts and 86 serve tests passed, 8 ignored, `cargo-slot: EXIT=0`, `/tmp/p2-tool-final-tests-wf_61813c32.log`. Relevant pre-P2 baseline was 65 plus 81 passed, 8 ignored. The five additional tests protect typed inputs, receipts, bounds, pin independence and authorization.

The worker reported successful formatting and diff checks. No P2 SQL tool path had run at handover. The parent then identified a real card-producer/consumer shape mismatch, so the coupled component is not accepted despite those green library tests. A fresh isolated fixer is handling the wrapped-card receipt validation/trimming contract and verifying selected-relation source priority. Original source worktrees stay unchanged for independent review.

## Cards and migration

Commit `639a36d41863bb4db7714d6602a063c51e7490cc`, clean worker checkout at `/home/nathanael/repos/Deadlock-Brain/.claude/worktrees/wf_dc2ccbb4-293-1`.

Only the standalone P2 migration and `brain-storage/tests/gamefacts_assets_schema.rs` were committed. The worker reported two passing PostgreSQL tests, successful build and formatter, with clippy and P1 integration still outstanding. The parent has not yet independently recovered the full matching command/proof log, so these counts are worker-reported, not a new parent execution.

The test explicitly skips positive item edges/cards when the real P1 kind constraint does not yet allow items and prints `ITEM_INTEGRATION_PREREQUISITE`. That prerequisite remains unproven until the actual announced P1 checkpoint is merged and the real tests rerun. The migration deliberately has no independent marker/runner and does not modify entity kinds. Before/after relation constraints and the card interface are in `GATE-NOTES.md`.

## Independent reviews

The committed database review and its fresh adversarial verification are complete. They confirmed that the unconditional 12-fact prefix drops required evidence: public Toxic Bullets returns only 12 of 31 mapped English facts, excluding effect description, bleed percent/duration and healing penalty; Sleep Dagger loses tier-1/tier-2 upgrade evidence. The review also executed the existing private-PostgreSQL P2 schema test binary without compilation: 1 passed, 0 failed, 0 ignored. The pending real P1 item-kind widening remained explicitly unverified; the separate public item-card reproduction used test-only kinds. A fresh isolated database fixer now owns essential-evidence ordering/bounds and its concrete contract tests. No original worker source is changed.

Separate committed Rust-contract and authorization reviews are complete. Authorization review and its fresh adversarial verifier confirmed both wrapped-card rejection and the slot query's bypass of `selected_relation_p2`. The Rust review found the same defects, but its mandatory-check phase stopped on incorrectly invoked formatter commands and canceled queued checks; its verifier did not complete substantive verification. This dimension is not a clean approval. No extra product defect was established by those invocation failures.

The tool fixer checkpoint `1faf41bb` actually passed 153 library tests with 8 ignored, including a real PostgreSQL wrapped-ability/selected-slot/receipt-revalidation contract. Its first new test run failed because the test delegate inherited a deliberately fail-closed revalidation default; the fixture was corrected, without changing production helpers. The subsequent build passed. Clippy then failed on duplicate loading of the shared scratch fixture. Final clean correction `a1edfdf24fad2c2e9150e64558a57e54d313c94e` reuses the existing fixture through narrowly authorized visibility changes and fixes only two proven P2-introduced shared-registration formatting deviations. Parent inspected final log markers: all-target clippy exit 0, focused actual PostgreSQL/tool tests 10 passed/0 failed/0 ignored/86 filtered, standalone SQL test 1 passed/0 failed/0 ignored with its positive-item prerequisite explicit. Full pinned workspace formatting and diff checks passed. The 153-test full library run and release build preceded the later fixture/whitespace changes; they are not falsely reported as a fresh full run at the final commit.

## Essential-card correction

Verified clean fix commit `19f5876bfc527727cae8531c17cb4c0679b13074` on `worktree-agent-a1766cc7fae48fe86`. Only the P2 migration, its contract test and a public selected-record fixture changed. The unconditional 12-row prefix is removed; required cost/descriptions/upgrades/gameplay values precede defaults and redundant metadata. Dynamic fact/card bounds also account for the complete wrapped ToolResult, unique receipt IDs and the existing worst allowed escaped call ID. Existing relation rules, source priority, privileges, lineage and locking remain unchanged.

Parent verified final log markers: 4 passed, 0 failed, 0 ignored, 0 filtered; clippy and release library build exit 0. Proofs are `/tmp/brain-p2-card-selection-a1766cc7/final-{postgres-tests,clippy,build}.log`. Accurate public-data fixtures use the actual 64-character payload revision and realistic internal IDs. Measured wrapped results were 15,456 bytes for Sleep Dagger, 15,303 for the largest essential ability case and 11,115 for Toxic Bullets. All designated essential assertions remain, with omissions explicitly counted. Earlier short-revision size measurements are superseded. Receipts are synthetic fixture rows, not a lexical-hash or production-ingestion proof. Positive item coverage uses explicitly labeled test-only kinds until the real P1 checkpoint.

## Assets job

Clean commit `8bec75f8364a5fe4a78e383107d2181113e19aab` on `worktree-wf_f37b6fbd-be8-1`, ten owned files. Adds the sibling Assets ingestion/extraction/storage modules, narrow public parser/CLI registration, one-step shared extractor version bump and hourly system timer instance. It does not modify P1 store behavior or P1 unit templates. The job's numeric identity IDs remain separate from canonical PostgreSQL IDs. Source revisions are the byte payload SHA256.

Worker reported release build, formatter and final clippy success, 26 passed/0 failed/3 ignored plus actual public EN/DE extraction/proof. Matching full command/log recovery and an identical-flag job baseline remain unverified by the parent. Published pre-P2 P1 interface proof reports 23 passed/0 ignored with its public fixture execution, but this is not falsely presented as the same default test invocation. The current parent baseline did not include brain-job. Actual item/edge ingestion, integrated source-priority and production memory/timer proofs remain pending. Independent committed receipt/HTTP and PostgreSQL-ingestion reviews and fresh adversarial verifiers completed. They executed the retained contract/PostgreSQL binaries without compiling; the security dimension also explicitly ran public EN/DE extraction/proof (18,377/18,369 facts and 134 edges per language). No critical security issue was found. Three concrete defects block acceptance: extractor-version replay depends on an upstream fetch despite the raw-only backfill requirement; the prover can skip container/noncanonical record locators; provenance-only differences in equivalent qualified conditions bypass serving source priority. Each has a verified static counterexample, not an executed negative regression. Fresh isolated fixers will own the Assets replay/prover and coupled serving selection separately. The original commits remain unchanged. Exact semantic qualifiers must still remain distinct; neither stored conditions nor P1 extraction may be rewritten to make sources agree.

The committed Assets extraction SHA256 is `bc3561100a9b224a0ba2fa9b238478bcbeafdcaab33c4d1a5489b7f92d521553`, matching the essential-card fixer's inspected mapping. No P2 production migration, main merge, deployment or timer activation has occurred.

## Coupled package checkpoint and compiler hold

Merge commit `2fd36868bb8feeb8e41aeab721233513de60caac` incorporates the corrected tool consumer `a1edfdf2` and essential-card producer `19f5876b` together into the P2 package branch, without rebase. It is pushed to `origin/feat/brain-p2-items-abilities-20261012`, not main. The worker ancestry also carries the existing P0/P0b/site prerequisites; these are inherited dependency commits, not new P2-owned changes. Parent pinned workspace formatting passed after this merge. No new combined compiler/test run or merge-gate approval is claimed.

User directive 08:12 on 2026-10-12 freezes all new cargo-slot build, clippy and test jobs until P1 is live and the orchestrator explicitly releases slots. Both fresh fixers acknowledged that no compiler process or queue had started. Assets replay/prover work is isolated on `fix/p2-assets-replay-proofs`; qualified-source selection is isolated on `fix/p2-semantic-source-selection`. Both have now returned clean, parent-pushed source checkpoints, with all compiler/runtime validation explicitly deferred. Exact SHAs, read-only review boundaries, missing public fixtures, the installed wrapper's actual verification profile and continuation steps are recorded in `AUFTRAG.md`. Neither candidate is independently accepted or merged into the coupled parent. A fresh reading-only review completed with a PLAUSIBLE composite/window predicate-pushdown concern, not an executed scan/timing finding; its precise boundary and conditional next step are in `AUFTRAG.md`. No further P1 interface requests are needed; adopt the checkpoint P1 ships. This freeze supersedes earlier per-worker compiler-lane permissions.

The resumed implementation agents lacked their originally required `StructuredOutput` tool and returned plain-text handovers. Their actual commits and build/test logs, not missing-format assertions, are the evidence. The new tool review workflow intentionally uses plain-text reports without a StructuredOutput dependency.
