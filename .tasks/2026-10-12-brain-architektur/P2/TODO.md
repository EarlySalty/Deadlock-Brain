# P2 paused continuation

## Binding pause

Explicit user decision on 2026-10-12 at 16:45: P1 is stopped and its ingestion design will be rethought. P2 depends on P1 and is paused. Start no workers, fixers, compiler jobs, tests, ingestion, gate, deployment or automatic restart. Keep the branch and worktree. Resume only with the new briefing after the redesign decision, not on the former 13:50 estimate or the earlier slot-release condition.

This TODO is explicitly requested by the user for the pause, not a quota-exhaustion claim.

## Preserved checkout

- Worktree: `/home/nathanael/.worktrees/Deadlock-Brain-p2-items-abilities`.
- Branch: `feat/brain-p2-items-abilities-20261012`.
- Last pushed source/documentation checkpoint before this pause record: `26dd23ef7c0dbf2305ede77d0ef0f6d10edc2fba`. The pause commit changes documentation only; obtain its final SHA from the thread or `git log -1`.
- No active native workers were listed at pause. The earlier workers/reviews had completed or were stopped; none was restarted. Foreign T3 sessions are untouched.
- This session performed no P2 main merge, merge-gate execution, production migration, deployment, ingestion, service restart or timer activation. The package is not accepted or live.

## Retained source, not a redesign contract

P1 checkpoint `3ef22729188f435f070e6ef8182a90a28df6659c` is an ancestor of the package. P2 appends only migration 3 to the published ordered registry `[1,3]`; the P1 runner and v1 SQL remain unchanged. No later P1 checkpoint was deliberately chased. Do not assume this interface survives the redesign.

Source integration `c0670afb8c7666abebc561f104cd0ed04ae02c35` includes ordered registration `2bf240d88c5e0e50c0ba0c3aad2b07e8c01e8582`, Assets replay/prover fix `192c2e4e5c488982f51b90f287272b9573631e95` and semantic selection fix `37ea7c37834963bb2ab45b8e75fde5f35cb6cb09`. Formatting-only correction `4a9dc01d8350474fcb1fb232c516f5d5ee1ee1b1` enters through `b696daec77aa6bf0fba61ea5949c24af5e8bb8b1`.

The source contains bounded Assets ingestion, receipt-backed item/ability assertions and item relations, canon cards, and `get_item`/`get_ability`/`compare`. The original corrected producer/consumer checkpoints are recorded in `CHECKPOINTS.md`. Shared extractor version remains inherited `p1-v2`, pending any future approved reconciliation. Keep all original and fixer branches/worktrees during this pause.

## Evidence and limits

Parent pinned workspace formatting actually passed at `b696daec`:

```sh
RUSTUP_TOOLCHAIN=1.97.1 /home/nathanael/.cargo/bin/cargo fmt --manifest-path /home/nathanael/.worktrees/Deadlock-Brain-p2-items-abilities/rust/Cargo.toml --all -- --check
```

Exit 0; log `/tmp/brain-p2-integrated-fmt.XPbfne6B.log`. Subsequent changes were documentation only. The final coherent source has no fresh build, clippy, test or PostgreSQL result. Older component runs and the measured baseline in `CHECKPOINTS.md`/`BASELINE.md` are not verification of this final state.

Independent reading-only contract research retained no static contradiction in the new replay/prover slice. It is not runtime proof, mandatory Rust review or gate approval. The specialized Rust reviewer stopped before commands because its mandatory compiler phase conflicted with the hold. Parent supplemental scripted literal comparisons were hook-blocked before execution; no independent scripted result is claimed.

Public source fixtures still existed when this pause was recorded: `/tmp/brain-p2-public-pair.5gz51bxr/items-en.json` and `items-de.json`, 6,157,215 and 6,311,063 bytes. Source downloads were bracketed by the actual `/v1/assets/steam-info` endpoint reporting client 6774. EN SHA256 `31c96630cba83b9fdbbedf62bb76a70cc3334667f05d2694c58d1d73791fe596`; DE `a15e62045e4a592db62a63b8c854059c0ad046172f1ad02e91334db4a010d7b1`. These are temporary public bytes, not successful extraction, receipt, database or live evidence. Verify their availability later; do not use the separate initial `KmZx38Br` preparation as a version-bracketed pair.

## Continuation after a new briefing

1. Read the new briefing and actual approved P1 design first. Reassess retained P2 contracts, migrations, source publication, receipts, kinds and serving against it. Fetch/reconcile the newly approved main only when work is restarted. Do not blindly apply the old migration, resume old ingestion or create an alternative P1 runner. Never edit an applied migration.
2. If the retained design is still applicable and validation is explicitly permitted, run the affected build, formatter, linter and relevant existing suites plus the concrete new contracts. The installed cargo-slot rewrites ordinary requested release checks to `verify`; report the actual profile and use the existing build wrapper, shared sccache and artifacts. Reconcile real tool count and monotone extractor version only against the then-current approved base.
3. Execute actual populated-P1/item-edge/card and GetItem producer/consumer contracts; raw-only extractor replay, noncanonical-locator rejection, lexical receipt checks and unchanged repeat with zero new raw objects/facts. Check essential retention against combined Gamefiles plus EN/DE, not English-only synthetic projection sizes. Four heroes by twelve stats must preserve all identities/cell status and keep the complete serialized ToolResult at or below 16,384 bytes.
4. Run actual item/compare EXPLAIN under the real reader timeout. The composite/window filter-pushdown concern remains PLAUSIBLE, not a proven scan-count or timing defect. Confirm it before assigning a fresh fix. Complete formal Rust review and independent goal acceptance, then the local merge gate on the final SHA.
5. Main integration, release install, live answers/traces, real system-job cgroup-v1 peak below 300 MB and timer activation require the new prerequisite/permission decision. `brain-serve` restarts only through `brain-release install`. Cleanup only after the eventual main/live completion, never to satisfy a stop hook during this authorized pause.

Deferred gate entry, located but not executed:

```sh
/home/nathanael/.venvs/gpt-workers/bin/python /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/Deadlock-Brain-p2-items-abilities --base main --head HEAD
```

No ALLOW/BLOCK result exists. Self-settle this thread after the pause commit is pushed and its SHA reported; do not create or message another T3 thread.
