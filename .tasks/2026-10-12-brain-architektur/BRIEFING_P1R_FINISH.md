# P1r FINISH: take the hero slice to main and live

Rules: /home/nathanael/.claude/CLAUDE.md. Work in English; German only for text players see. Role: package sub-orchestrator (Teil-Orchestrator) with native Claude Code subagents. You own exactly one outcome: P1r merged to main, released, and answering live. Nothing else.

## Why you exist

The previous P1r session (thread 08284667, branch `feat/brain-p1r-hero-slice-20261012`, HEAD 301d1cc6) finished the code but has spent the last three hours in a reviewer cascade (fixer, independent reviewer, combined verifier, whole-diff critic, reviewers ending INCOMPLETE on wrong fixture paths). The user stopped that session and started you. Four other packages wait on this merge. The user's words: focus only on P1r, get it done and reviewed.

## Starting point

- Worktree: `/home/nathanael/.worktrees/Deadlock-Brain-p1r-finish`, branch `feat/brain-p1r-finish-20261013`, HEAD 301d1cc6 (= old P1r head, pushed to origin).
- Pending JSON fix: local branch `fix-json-index-closed-wf27` (a6d56642, two commits on top of 07392a8e). It makes recognized JSON index errors fail closed (duplicate keys, invalid escapes) instead of silently returning None. Tests passed per the old session's notes (`/tmp/wf27-json-*.log`). Merge it first.
- Old session's own state and findings: `.tasks/2026-10-12-brain-architektur/P1r/TODO.md`, `CORE3B_PREFLIGHT_FINDINGS.md`, `AUFTRAG.md` (original scope and acceptance), `../CONTRACT_INGEST_V2.md`, `../REDESIGN-INGEST.md`, `../PAKETE.md` section P1.
- Old P1 gate logs: `/home/nathanael/.local/state/brain-p1-proof-20261012-121638e5-Dr2MKo/all-five-gates/round{1..5}.log`.
- Fixtures: `/tmp/p1r-hero-a314-fixture-GInzyzvn/hero-data.json`, KV3 cache `/tmp/brain-core2-cache-wf-7f1dca25-SbrUuw/kv3`. Pass as `--config 'env.BRAIN_HERO_FIXTURE.value="..."' --config 'env.BRAIN_HERO_FIXTURE.force=true'` and the same for `BRAIN_CORE2_KV3_REPOSITORY`. Give every subagent these literal paths; a reviewer that runs tests without them is your setup error, not a finding.
- Toolchain: cargo only via `/home/nathanael/.local/bin/cargo-slot +1.97.1 ...` (see `/home/nathanael/Documents/claude-config/machines/v50671/RUST-BUILDS.md`). Check disk before release builds.

## Acceptance (unchanged from AUFTRAG.md and PAKETE.md P1)

1. `brain-job ingest-gamefiles` (deadlock-data hero JSON) on the real Brain DB: 67 heroes, 845 facts, 979 receipts; unchanged rerun writes zero raw objects, facts and receipts; peak RSS < 300 MB (cgroup).
2. Tools `find_entity` and `get_hero` registered in brain-serve, reading only `serve.*` with role `brain_tool`; `EXPLAIN` of `get_hero` is an index scan < 5 ms.
3. Two live answers through `/v1/answer` ("Baba Steckbrief" and one other hero) with status `answered`, citations resolving to receipts, `ops.answer_trace` showing `find_entity` then `get_hero`, no `deadlock_data` call, no git open in serve.
4. Migration applied through the release hook; brain-serve `readyz` 200; clean error journal since install.
5. Local `gate_hook.py --review` ALLOW (skill `rolle-merge-schleuse`), merge to main, push, release via brain-release, live proof written to `~/.local/state/brain-p1r-proof-20261013/LIVE.md`.

## How to work (binding)

- Plan: (a) merge the JSON fix, (b) run the full matching suite once yourself (migration, storage, job, serve PG tests, check, strict clippy, fmt), (c) ONE fresh independent whole-diff reviewer (native subagent, Opus or Sol, with the literal fixture configs and the five old gate logs) with a hard brief: confirmed defects with repro only, no style, no "INCOMPLETE" exits; if its setup fails it fixes the setup and continues, (d) fix confirmed defects with a fresh fixer, (e) gate, (f) merge, release, live proof.
- Max one reviewer and one fixer per round. Do not spawn a verifier, a critic and a reviewer for the same diff. If a subagent dies or stalls 20 minutes, replace it, do not wait.
- Gate BLOCK: fresh fixer per round, at most five rounds, then report a concrete blocker to the user with branch, SHA, options and recommendation.
- Do NOT publish core checkpoints for other packages, do NOT touch P2 to P9 branches or worktrees, do NOT answer their questions. Merge to main is the checkpoint; they rebase afterwards. If a change in shared files is needed to pass the gate, make it, note it in `P1r/TODO.md`, move on.
- Do not widen scope: no abilities, items, localization, upgrades, second source, recovery. If the whole-diff review finds something outside the hero slice, record it in `P1r/TODO.md` as follow-up and continue.
- No code comments. Functions small. No `.unwrap()` in production paths. Existing tests must pass; add tests only for a confirmed defect.
- Secrets only via Infisical helper or systemd credentials; never print them. Note: `ps -eo args` printed bearer tokens into a transcript yesterday; use `ps -eo pid,etime,comm`.
- Reporting: no interim messages. Report once at done (branch, merge SHA, release SHA, proof file paths, the two live answers) or once when truly blocked. Then `python3 ~/Documents/tools/t3-thread.py settle --selbst`.

Routing: direct principal is the user's main session (Fable). Main orchestrator thread 958d03f8-4b01-466d-87e7-6b0f2ebf14d4 is informed and will not hand you other work. Architectural blockers end a turn with `FRAGE AN ORCHESTRATOR:` plus branch, SHA, options, recommendation.
