# P5 continuation, 2026-10-13

Branch: `feat/brain-p5-docs-20261012-v2`.
Verified source checkpoint: `686250c7c1e2b3a5df6d75fdf2af2cec47eba69e`, pushed to origin.
Primary worktree: `/home/nathanael/.worktrees/Deadlock-Brain-p5-docs`.
Full chronology and commands: [AUFTRAG.md](AUFTRAG.md).

## Current state

The pending sparse-registry merge is finished at `4d213212`. The verified Docs fixer and expressly approved Core 3a checkpoint `982645cf325952b56c782fe690f6e23a5a76bf17` are merged without rebase at `686250c7`. All Rust files match worker final `86b52dcb58db789bf883d37ec643874784e53e18` exactly. Registry versions are 1, 5 and 7; P1r owns version 7. No migration body or frozen production core was changed.

Parent verification `bu2dbo7pt`: formatter, storage/serve all-target check and strict Clippy pass; real native Docs suite 23 passed, 0 failed, 0 ignored, 0 filtered; shared service::pg_tools suite 4 passed, 0 failed, 0 ignored, 87 filtered. The 500 ms search deadline remains unchanged. Duplicate naked positive words are compacted at the shared selection path; phrases, OR, negation and punctuation retain their original input. Earlier unexplained short-query timeouts are recorded, not classified as flakes.

Independent Rust review completed with one confirmed HIGH finding at pg_docs.rs:377-392. Rust's alphanumeric guard accepts `or²`, but PG16 interprets it as OR followed by operand `²`. Input `alpha or² omega beta alpha` is compacted to `alpha or² omega beta`; against text `² omega beta`, the original fails membership while the compacted query matches, in both simple and German vectors (rank0 becomes0.1). The current verified-test checkpoint therefore remains semantically defective despite 27 passing tests. Fresh isolated fixer `a2a497c3491682e4b` owns only the guard and concrete real-PG regression in pg_docs.rs/tests, starting from55b67439. Limit rewriting to demonstrably plain PG tokens, preserve uncertain syntax, deadlines and all original request/query bounds. Do not duplicate this active fixer. Parent owns AUFTRAG/TODO; worker may create a new uniquely named round-evidence file only.

## Gate status

No ALLOW. Worker gate initially BLOCKed the old failed-run fixture. Approved Core 3a fixes that exact contract conflict and the shared suite now passes. Final worker gate and one technical retry both exited 2 without a verdict: bwrap/unshare could not allocate memory while creating namespaces. Preserve logs `b2hkdctt5.output` and the first final attempt under the session task-log directory. Do not reroll an unchanged substantive BLOCK or bypass sandbox/gate controls. A later simple user-namespace diagnostic succeeded; that is not proof that the complete gate sandbox recovered.

## Open acceptance and decisions

1. Canonical/public versus scoped internal Evidence remains undecided. Current Docs Prose/numeric receipts do not match the shared canonical formatter or trace resolver. Never relabel internal Docs Public or decorate canonical Evidence. Public tool receipts also need canonical-result binding after the boundary is decided.
2. Actual sections reside in frozen core `canon.fact`; `serve.doc_section` is a view. Owned generated stored vectors plus GIN in migration 5 are approved, but the Docs-limited INSERT/UPDATE trigger exception needed to populate that owned table is unanswered. No core hook, trigger, poller or post-ingest refresh was added. Current source-fact GIN does not accelerate reconstructed serving search.
3. Docs runtime composition is absent from the forwarded serving core. Fixture construction is not production reachability. The responsible owner must forward the permitted composition change.
4. Combined ingestion peak including Git/Postgres children, real find_entity("Baba") alongside ingested Docs, independent whole-final-diff acceptance and live function remain unproven. The separate Store proof is preserved but unmerged because the extractor/test budget is 799 lines; the current pure extractor plus contracts uses 798. The 16 real corpus rejections are recorded and were not repaired out of scope.

## Resume safely

Receive the active fresh fixer's verified branch and inspect its concrete before/after PG proof. Resolve only verified owned findings with a fresh fixer when required. Obtain genuine decisions or forwarded owner checkpoints for the open boundaries before implementing them. Run matching checks and the canonical gate on the final changed source when the reviewer sandbox is operational. Hand over the branch and exact SHA to P1r; do not independently merge main, deploy or cut over Discord.

Brain maintenance and dl-knowledge stay stopped. Do not touch ai-coach, foreign worktrees/builds/caches or production secrets/data. Bots WIP remains preserved at `3c28ce2583d1d06a78d91966a454d60b6e9d6fc0`; Discord remains unchanged until the replacement Brain is actually live.

TESTNACHWEIS[TW-1]: 27 passed, 0 ignored | Baseline: 1 rot
MERGEPROTOKOLL[MS-1]: 3 Git-Schritte einzeln | Anläufe: 0 | Gate: inherited final exit 2 twice, no verdict

The merge protocol counts parent registry commit, worker/Core3a feature merge and feature push. It is not a main merge or a new gate attempt.
