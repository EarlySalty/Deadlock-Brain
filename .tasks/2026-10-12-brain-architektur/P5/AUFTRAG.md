# P5: docs ingestion and scoped search

Goal: guide and how-to answers use receipt-backed Deadlock-Docs sections from Postgres full-text search, and dl-bot calls only `/v1/answer`. Missing evidence remains silent in the bot.

Integrator: this package sub-orchestrator. Routing: main orchestrator thread `89e002a7-758e-449d-9968-eec29a019a78`. No additional T3 threads.

Brain worktree: `/home/nathanael/.worktrees/Deadlock-Brain-p5-docs`, branch `feat/brain-p5-docs-20261012`, initial HEAD `a808f5a492b306d63a2a1a42b533dd1aeaf2d290`, initially clean. Bot worktree: `/home/nathanael/.worktrees/Deadlock-Bots-p5-brain-answer-only`, branch `feat/bots-p5-brain-answer-only-20261012`, initially clean.

Write boundaries: new P5 ingestion/tool modules, one additive gamefacts migration, append-only registrations in the common contract, package-owned job timer configuration, P5 task notes, and the isolated bot adapter change. No pre-retrieval removal, old tool removal, content edits, model/provider changes, site/maintenance work, deletion, or changes to foreign work.

Dependencies: common wave-2 contract and design under `.tasks/2026-10-12-brain-architektur`; P1 must be live before integration. Merge published P1 updates, never rebase them. Brain goes live before bot integration. P1 job checkpoint `3a0740be219fb4aceb304e8020501dcd7281fbd7` was merged as `e0f03de4b71152f7776a2d7f8d659fea9ee86194` and pushed. `P0b.md` is present on `origin/main`, not at initial P1 HEAD; its live system-unit contract has been read from `origin/main`. P1 will publish ordered gamefacts migrations; P5 owns version 5. Until that checkpoint, the P5 migration stays standalone without a separate migration runner.

Authorized scope mapping, corrected by the orchestrator: `docs.public` or `bot.public` reads public sections only; existing `second-brain`, channel `internal`, scope `second_brain.internal` reads internal sections. No docs-client grant or provider egress changes. Internal access remains provider-free. The existing internal route uses ReleaseRetriever, so an additive internal adapter must be evaluated rather than assuming tools are reachable there.

Additional ownership approved by the orchestrator: narrowly extend `brain-contracts/src/public_api.rs` to preserve validated HTTPS public Docs section URLs in the existing citation label. No new API field, query credentials, internal URLs or changes to other source labels. The FAQ source links to `/docs/deadlock-helden/billy.html`, but a live GET to that path returned 404 on 2026-10-12. A truthful live citation target still requires verification. The measured older local Docs commit has no public Baba page; inspect the fetched source before making a final content-gap claim.

Acceptance: a live early-game Baba answer cites a Docs section URL; dl-bot makes no request to port 8896; public credentials cannot access internal sections; ingestion peak stays below 300 MB with enforced system-unit memory limits; unchanged second ingest writes zero raw objects; existing affected tests retain their measured baseline. New contract tests only for receipt correctness, bounded output, and enforced scopes.

Start proof: session runtime identifies the Claude Code harness and `gpt-6.1-sol[1m]`; UltraCode is active by runtime reminder. Native Workflow research and baseline tasks launched successfully. Models inherit the approved session model; no Sonnet or extra provider selection.

Integration: disjoint native workers in isolated worktrees, independent reviews, affected formatter/linter/build/tests, local merge gate on the exact integration HEAD, serialized release plan/build/verify/install through existing wrappers after P1 live, then scoped live proofs and the bot's separate gate/release path. Built, reviewed, merged, and live are reported separately. Release installation owns serve restart.
