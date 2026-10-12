# P4 patches, ingestion v2

Restarted 2026-10-12 17:41 CEST by the main orchestrator. Branch `feat/brain-p4-patches-20261012-v2` was created in the existing isolated worktree from fresh `origin/main`, base `edf503b280777524093307a97fabbb1bdba83d7c`. Old branch `feat/brain-p4-patches-20261012` and its pushed WIP `9619960212aa00321b89854858ab818e4e2a199c` remain references, not a merge base.

## Goal and binding design

Implement P4 against `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-12-brain-architektur/REDESIGN-INGEST.md` and `CONTRACT_INGEST_V2.md` in that folder. Only own extraction logic/tests, canonical patch views/migration and tools are carried forward. The cancelled P1 runner, its Store/leases/resume/tombstones/custom publication guards and the old P4 writer are not copied or used.

P1r owns the new brain-job core. Its thread is `f1064599`, branch `feat/brain-p1r-hero-slice-20261012`; the orchestrator will provide its published core SHA. Work against the documented `RawObject`, `Extractor`, `Outcome`, `FactSet`, `Rejection`, `limits::validate` and registry names until then. Do not invent a shadow core or pretend undocumented FactSet row layouts are published interfaces. Merge the announced checkpoint when it arrives; no published-history rebase.

Extractor and its tests together must remain below 800 lines. Extraction is pure: no DB access or writes, complete Accepted/Rejected/Ignored outcomes, shared central validation before any write, explicit normalized-key collision rejection. Required poison proof: one oversized value and one colliding key are rejected independently; the real core run finishes succeeded. Raw capture and per-object rollback/rejection handling belong to P1r, not P4.

Preserve official raw bytes/receipts, existing proven entity binding, no inferred source-revision-to-patch mapping, bounded canonical numeric patch_history and optional historical hero behavior. Runtime tools use brain_tool, 500 ms timeout, bounded queries, complete citations and serialized result under 16 KB. No source-data repair, provider changes, deferred ability/item/timer patch interfaces, old timer restart or unrelated service effects.

## Ownership and execution

P4 parent owns this note, selective artifact integration and shared append-only registrations. Native executing workers use their own isolated worktrees and disjoint files. Active source workflow: `wo6ptqyuv / wf_99e78d62-197`, with separate extraction and serving/schema coders; exact isolated paths and actual results will be recorded on handoff. Extraction worker owns its new pure extractor module/tests only. Serving/schema worker owns P4 migration/views, patch tools/tests and only the P4 contract additions. No full copies of old shared/P1 files and no old runner registration. Current main has no brain-job crate; its manifest/core arrives from P1r rather than being reconstructed here.

Build slots are explicitly reopened. All Cargo compiler checks use cargo-slot with the required release/locked profile and existing shared artifacts/cache. Remote-first behavior comes from the current cargo-slot wrapper; do not disable it or build another wrapper. DB tests stay local when their required DB environment is set. No competing release builds, main merge, deployment or timer enablement before P1r is live.

A fresh independent reviewer reads the complete integrated diff against both redesign documents before the gate. Local gate ALLOW and independent intent acceptance precede integration, which follows P1r live and the existing ALLOW order. Built, reviewed, merged and live remain separate claims.

## Schedule and routing

ETA ready for gate: 21:00 CEST on 2026-10-12, conditional on P1r publishing its core by 19:15 and being live before the gate starts. Core FactSet row layout, generic source registration and receipt/schema details are checkpoint integration dependencies, not permission to recreate the cancelled runner.

Main orchestrator: `89e002a7-758e-449d-9968-eec29a019a78`. Existing native P4 session: `83582205-a6d2-460c-9655-90e0cf147f71`. No extra T3 threads or session-to-session coordination. Worker/fixer model role from current pyramide.json is gpt-6.1-sol; native jobs inherit the selected session model and never use Sonnet. Regular status cadence is 30 minutes; no duplicate writers or reactivation of old paused workers.

Saved references: extraction `b954a5334e3047598ef082232762b1d0f5ee26f9`, schema `b1a1f6b6f0cb42dde0fa1445ed9777dab862f8cf`, tools `b7118233d64434f3384d3e54a8240f7991d32568`, feed `7ec537e37b1c78aa950242bd3c2ea119cc170cfe`. Read them only for reusable P4 behavior; the feed/store WIP is not the v2 writer implementation. Prior literal-Baba fixtures do not prove a balance transition. No v2 build, poison-run, gate or live proof has yet been executed.
