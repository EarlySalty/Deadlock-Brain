# P1r: hero slice with simple ingestion

Goal: answer "Baba Steckbrief" and one other hero live through `/v1/answer`, using PostgreSQL `find_entity` and `get_hero` with citations to raw objects and no `deadlock_data` call.

Boundary: one game-file source, heroes and required aliases/card facts only. No abilities, items, localization, upgrades, second source, lease, resume, tombstones, recovery, or wave-2 edits. `brain-job` stays below 1,500 lines including tests. Follow `REDESIGN-INGEST.md` and existing repository working rules.

Owned paths: `rust/crates/brain-job`, gamefacts portions of `brain-storage`, hero tool modules and their registration, one new gamefacts migration set, `ops/brain-job@` templates, and this directory. Reuse the ordered migration registry and release hook where missing. Any required ownership extension is routed to the main orchestrator before edits.

Checkout: `/home/nathanael/.worktrees/Deadlock-Brain-p1r-hero-slice`; branch `feat/brain-p1r-hero-slice-20261012`; starting HEAD `edf503b280777524093307a97fabbb1bdba83d7c`; initially clean. P1 and wave-2 worktrees remain unchanged. The package sub-orchestrator owns integration, gate, release, and live verification. The main orchestrator owns the central register.

Contracts: raw captures bytes, source revision and content hash without content validation; unchanged hashes skip raw writes. Pure per-object extraction validates every storage limit and detects normalized-key collisions before writes. Each canonical write is transactional and an object failure records a rejection without blocking later objects or runs. Runs have terminal results with accepted/rejected counts, never pending lifecycle state.

Evidence: existing-test baseline and matching build/fmt/clippy/test checks; actual-source inspection; scratch-PostgreSQL poison test with exactly two reasoned rejections and an unblocked next run; two live cited answers and answer traces; successful ingest and zero-new-raw rerun; cgroup peak below 300 MB; readyz HTTP 200; empty error-priority journal since install.

Workflow: native Claude Code workflows explicitly enabled by the active harness. Read-only reconnaissance uses independent source, prior-failure, and reusable-interface readers. Productive workers receive disjoint file sets in isolated worktrees. A fresh whole-diff reviewer checks all five P1 gate logs and redesign contracts before the local gate. Fresh fixers handle BLOCK rounds, at most five rounds.

Routing: package P1r, main orchestrator `89e002a7-758e-449d-9968-eec29a019a78`. No extra T3 threads or user questions. Architectural or ownership blockers end a turn with `FRAGE AN ORCHESTRATOR:` and branch, SHA, worktree, options and recommendation. Built, merged and live are reported separately. Final completion includes branch, merge/release SHAs and proof locations.
