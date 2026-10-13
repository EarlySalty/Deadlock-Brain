# P5 continuation, 2026-10-13

Branch: feat/brain-p5-docs-20261012-v2. Resume HEAD: 3fc7f93323133324bf683651f8170b553a5f97ea.
Primary worktree: /home/nathanael/.worktrees/Deadlock-Brain-p5-docs.

Preserved the previous AUFTRAG.md changes. Committed parser and scoped contract helpers are reused; their reported checks are not repeated as baseline work. No other uncommitted code or P5 worker checkpoint needs salvage.

BESTAND[BS-1]: teilweise | Fundort: rust/crates/brain-job/src/docs_extract.rs:362 | Anknüpfung: existing verified HTML parser, docs_tools bounded outputs and policy helpers, published P1r Store.

Remaining: adapt pure parser to the agreed doc_page/chunk wire format within 800 lines including tests; implement migration 5 with coherent serve-only projection and indexed FTS; add production reader/wiring and streaming Git source. Then run real Store poison, two-run idempotence, scope/quarantine/receipt proofs, relevant checks and independent review.

Core dependency resolved: orchestrator forwarded `9f75588ddee83422e6a7755865e4a5c44fa2e33d` on 2026-10-13, superseding e3d96fa6. Merged and pushed at `70b55acb`; Cargo.lock conflict retained both P1r and P5 dependency additions, locked metadata check passes. Real PG tool executor/find_entity/get_hero await their separate checkpoint. Core stays frozen except approved P5 registration lines.

Integration and deployment belong to P1r under resume rules. Do not deploy or enable jobs here. Brain maintenance and dl-knowledge stay stopped. Discord switch remains deferred until the Brain replacement is live; Bots WIP worktree stays preserved.

Next step: implement owned adapter, projection/reader and source against approved contracts, merge forwarded core 2/2b, run missing real checks, obtain branch gate ALLOW and hand over SHA.

09:40 CEST preparation: parent added unregistered docs_source.rs with lazy commit-pinned scoped HTML iteration and a cache lock held through blob consumption. Added concrete two-test Git source contract. Added append-only SearchDocs tool variant/typed dispatch plus concrete no-game-context dependency contract in brain-contracts. Formatting passes via rustfmt +1.97.1. Source test command did not run: cargo-slot exhausted its ten-minute background limit while all local slots were occupied. Ionos root has 464 GB used and zero free; automatic remote workspace creation fails, launcher falls back locally. No foreign remote workspaces were deleted. Latest check state: typed registration's first ten-minute cargo-slot job also expired while queued and did not run. Stopped the queued local-only combined check deliberately before merging core 2b, avoiding a check against a changing worktree. Orchestrator freed 39 GB on ionos at 09:50; restarted parent contract/source/clippy checks remote-first on approved core with a 30-minute timeout. Parent checks completed on approved core: `cargo-slot test --manifest-path rust/Cargo.toml --locked --release --jobs 3 -p brain-contracts --test docs_tool_registration` (1 passed, 0 ignored); same flags with `-p brain-job --test docs_source_contract` (2 passed, 0 ignored); `cargo-slot clippy --manifest-path rust/Cargo.toml --locked --release --jobs 3 -p brain-job -p brain-contracts --all-targets -- -D warnings` (exit 0). All ran remotely in the verify profile. Source and typed contract are verified intermediate code, not yet production-wired; real Docs ingestion/reader/gate/live proofs remain open. TESTNACHWEIS[TW-1]: 3 passed, 0 ignored | Baseline: 0 rot. TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 0 belegt | Senke: P5 checkpoint commit.

No substitute core. Adapter now retains the original malformed mutation table and reports 799 formatted lines; both workers were instructed to merge parent `70b55acb` and perform real checks.

Native workers: adapter ae162b25f19468573, own worktree /home/nathanael/repos/Deadlock-Brain/.claude/worktrees/agent-ae162b25f19468573; projection/reader a7e5f8a63e0e2a15a, own worktree /home/nathanael/repos/Deadlock-Brain/.claude/worktrees/agent-a7e5f8a63e0e2a15a. Both inherit Sol, disjoint owned writes and no extra delegation. Adapter reports 799 formatted lines, original exact byte ranges and poison/chunk cases prepared; compilation awaits forwarded core2b. No worker result is claimed verified yet.

Preserved Bots worktree is clean on feat/bots-p5-brain-answer-only-20261012 at 3c28ce25. Its existing large WIP diff is not assumed gate-ready; no checks/deploy/switch were done during resume.
