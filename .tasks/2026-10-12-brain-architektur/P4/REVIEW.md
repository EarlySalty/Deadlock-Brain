# P4 independent review follow-up

## Actual second full gate, published126f6f39

Command: `python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/Deadlock-Brain-p4-patches --base origin/main --head feat/brain-p4-patches-20261012-v2`. Fresh baselineedf503b2, head126f6f397580491ee4fec8edc40e84a2a782c5c0. Taskbxr8zwqe2 exited1 with actual gpt-6.1-sol BLOCK: Current-state reads and publication guards remain incorrect; valid replay can publish stale data. Log `/tmp/p4-core3a-gate-20261013.log`, SHA256cc7059a769ab6502ca774d816ac942fbe91142b7a0e9f88893f4ce61a838981f.

Fresh causal round2 verification confirmed all three blocking claims. Parent independently read real role-boundary/cap results (3 passed/0 failed/0 ignored/0 filtered), byte-identical retained coherence validation7/7, and artifact hashes. Exact commands/repair contracts are in GATE_ROUND2_FIXER.md. These probes prove defects, not successful acceptance; no Core or applied migration was edited. All five receipt-writer and all five coherent-reader twins are covered.

Confirmed blocking claims:
1. `scripts/migrations/2026-10-13-brain-gamefacts-run-local-v7.sql:17`, with direct fact INSERT grant at v1:147 and publication at v1:117: mismatched failed valid_from_run/succeeded receipt can insert a visible fact and conflict replacement can withdraw it with an unpublished receipt. Both insertion and replacement require protection.
2. `brain-storage/src/gamefacts/hero_profile.sql:22,55,71` plus exact/fuzzy aliases `brain-serve/src/pg_tools.rs:18,26`: old names/stats/weapon/aliases remain under a newer identity without raw/run/extractor coherence. This original finding survived unchanged owner production in Core3a.
3. `scripts/migrations/2026-10-12-brain-gamefacts-v1.sql:55`:4096-receipt cap accumulates across applications of the same raw/run/extractor; same-run A,B,A can reject finalA and publishB. Entity/alias/fact/relation/empty-set receipt-writer twins need verification.

Nits: owner service.rs:623 still lacks productive PgPatchToolPort composition; gate omitted units::install context, so ops/brain-release/src/main.rs:136 new unit install coverage requires actual implementation inspection. Fresh isolated round2 fixer completed exact cause/twin verification using synthetic scratch PG and unchanged Core inspection. Both nits are confirmed statically:service.rs lacks the wrapper, units.rs:9,90,128 omits all three job units and gamefacts.rs does not install them. No installer or service was run. The additive Core repair must come from P1r; frozen production and applied migrations remain untouched. No reduced-diff review, alternate-model BLOCK reroll, relaxed grants or marker/slot bypass.

MERGEPROTOKOLL[MS-1]: 8 Git-Schritte einzeln | Anläufe: 2 inhaltliche Reviews | Gate: gpt-6.1-sol BLOCK, Exit1,3 owner/Core findings

## Final verified adapter on Core3a

Parent independently verified final artifacts/logs and transferred the exact tested P4 fixture correction. Full artifact SHA256731d3f0c2245fadc056b70c4b70776a5e398600493d42ba2a5f35b3d1219beb4; incremental SHA256c30bfee467c00856ad3e744d39f62e750ce506a96fc3d107b5e441cc21b19beb. All four P4 files, five mechanical resolutions and the owner test are cmp-identical to the tested final worker. Final standalone15/0/0/0 and combined port11/0/0/0 passed; integrated formatter/whitespace and final worker Clippy exit0. Owner failed-run fixture is closed on982; actual kernel/cold/warm traces and complete canonical result sealing are green. Commands/log hashes and retained earlier red attempts are in RUNTIME_VERIFICATION.txt.

The fixture no longer assumes three full content-addressed rows fit in one execution: real serving projection proves all three, bounded independent requests prove every original-byte receipt, complete reconstructed sealed execution21740 bytes/two-row prefix19714 exceed16384 while actual final14407 fits. Two-row case16625→14611 is similarly explicit. Original omission-only quarantine2→1, selected row/evidence equality, all validation purposes and byte-identical restoration remain asserted. Only own scratch table statistics are calibrated; no view/index/planner/production budget/timeout change or receipt consolidation. Actual path peak485813µs and all earlier timeouts are disclosed; time variance is not claimed eliminated.

The remaining original owner coherence finding is unchanged. Green tests do not erase it; final gate must review the complete published candidate rather than only the fixture delta. No ALLOW or live claim.

## Core3 causal verification and remaining owner boundary

Approved Core3a982645cf supplies only the owner test/notes correction. Parent normal composition preserves all own files and the prior five mechanical resolutions. The requested service::pg_tools suite on982 actually passed4/0 failed/0 ignored with98 unrelated lib filters, --include-ignored --nocapture --test-threads=1, local exit0. Evidence `/tmp/p4-core3a-service-pg-tools-20261013.log` proves a genuine failed ingest has stored facts but no published receipts/profile. The prior terminal-status-fixture failure is closed; production coherence finding remains because production/migrations are unchanged. Final P4 fixture/runtime checks and gate are not yet green.

Approved normal composition of19dec0c9 fixes original gate findings1/2: fresh scratch-PG proof checks the catalog preflight under brain_ingest without expanding grants and allows A after B instead of treating historical success as current application. v1/v4/v7 were actually applied. Original finding3 survives: hero_profile identity/stat/weapon joins and exact/fuzzy alias joins expose superseded omitted fields/aliases. P4's coherent joins exclude the corresponding stale rows. Evidence directory `/tmp/p4-gate-r1-pg-LKliLN/` retains the SQL scripts and red/green comparison logs; scratch server stopped. Frozen owner queries are not changed by P4.

Before approved Core3a the composed runtime also had an actual owner-fixture failure: `brain-serve/src/pg_tools_tests.rs:803` updates a succeeded published run to failed, but approved v7 rejects that mutation with23514. Actual combined port target10 passed/1 failed/0 ignored. Correct owner fixture should assert terminal immutability, then quarantine through its scratch published_receipt view rather than weaken v7. This is not an environmental failure or a silently excluded test. Both owner boundaries have been reported to the orchestrator; no unapproved owner patch is inferred.

P4 formatter adaptation is transferred from the fresh fixer's exact four files with `git apply` after a clean applicability check. Artifact `/tmp/p4-gate-r1-pg-LKliLN/p4-canonical-adapter.patch`, SHA256 `aff08bc5ea50ba9b7c40e372bb95824ecc86ef2862f9e3d2fbb32f03e0a0a614`. Actual file comparison and formatter/diff checks passed. It reuses public CanonicalReceipt/format_receipt, remaps only authoritative citation fields, measures the complete execution with actual content IDs and seals the final bounded ToolResult. Frozen canonical formatter, owner tools/trace and hero SQL are byte-identical to approved19dec0c9; only approved additive P4 registrations remain in shared lib/registry/migration registry. Worker runtime proof includes actual owner/P4 kernel publication and cold/warm numeric trace IDs with centrally derived fixture budget54851.

Worker standalone P4 target waited600s for cargo-slot and ran0 tests. The single parent integrated target actually ran15 tests remotely:13 passed/2 failed/0 ignored/0 filtered, exit101. `pg_patch_tools.rs:1185` expected3 ranged rows but received1; :713 expected2 but received1. New-format complete-execution bounding is the suspected cause, not an environment flake. Parent stopped its redundant local fallback after preserving this actual red proof. Fresh fixer is correcting only the four P4-owned files, with the unchanged16384-byte contract and the full three-post omission/quarantine property retained; no assertion is simply weakened or filtered. Log `/tmp/p4-core3-parent-patch-tools-tests-20261013.log`. Parent normal merge remains open until a coherent verified checkpoint exists. Gate verdict remains the original BLOCK; no ALLOW, main/deploy or service insertion is claimed.

## Actual local gate on0a79e1e4, 2026-10-13

Command: `python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/Deadlock-Brain-p4-patches --base origin/main --head feat/brain-p4-patches-20261012-v2`. Task bk2qhgspo exited1 with an actual gpt-6.1-sol BLOCK, not a tool startup failure. Source fix has no new finding in this verdict; full acceptance is blocked by three owner/Core findings:

1. `brain-job/src/main.rs:58`: ingest schema preflight requires serve access, but v1 grants brain_ingest raw/canon/ops access, so the configured ingest role cannot reach object processing.
2. `brain-job/src/lib.rs:84`: historical successful application satisfies was_applied even after newer content; A→B→A skips the last A and leaves B published.
3. `brain-storage/src/gamefacts/hero_profile.sql:22,55,71` and `brain-serve/src/pg_tools.rs:22,30`: identity aliases, stats, weapon and exact/fuzzy aliases lack the object/run/extractor coherence guards, retaining data omitted from a newer successful object.

Nits retain the owner service insertion at service.rs:623 and incomplete deadlock-data extractor handoff at registry.rs:3. Fresh isolated fixer a477e3abbfdfb2d36 is verifying causal findings and ownership. Frozen owner files remain protected; no workaround, relaxed grants or different review baseline will be used to conceal BLOCK.

The preceding --base main attempt exited2 without a verdict: local main was stale cf02c9a0 while published origin/main remained edf503b2. Its4.8M-character input included unrelated already-released history. The actual BLOCK above uses the full published release baseline after fetch, without changing main or gate protection.

## Oversized-post disposition, 2026-10-13

The latest orchestrator instruction supersedes the earlier requirement to wait for a Core source-item/report interface. P4 now skips oversized rows before the iterator, logs each ID/reason/actual byte size and counts them; Core-level reporting is deferred until after P1r is live. The new single regression uses real scratch PostgreSQL and Core ingestion, then captures actual JSON stderr from that same test process: two skipped posts have exact IDs/reason/UTF-8 byte sizes and cumulative counts, the source summary reports scanned4/emitted2/skipped2, later posts are accepted and raw storage contains only paths2/4. Source tests passed6 with no failures or ignored tests; measured client peak254627840 bytes and raw repeat delta0. The orchestrator-approved source policy is implemented and the matching executable build, check, Clippy and formatter exit0. Fresh affected tests total18 passed/0 failed/0 ignored, with2 disclosed unrelated filters. Commands and logs are in `SOURCE_HANDOFF.md`. The fresh independent gate verdict is pending. This is not yet independent acceptance of the new code.

## Earlier independent review: published parent1628ba07, 2026-10-13

Fresh read-only reviewer `a3f637df6130c3776` confirmed the actual integrated parent after approved owner merge and runtime fix67ae30ad. Shared receipt serialization and complete canonical result binding are fixed. Real PostgreSQL and the actual owner/kernel path prove receipt equality; omission-only quarantine changes reject stale seals for Provider, Publication and Cache. Final affected checks: 246 passed, 0 failed, 0 ignored, 0 filtered; formatter/compiler/Clippy exit 0. Evidence: `RUNTIME_VERIFICATION.txt`. Source/schema artifacts and owner service code were unchanged by the runtime fix.

WIRKUNGSPRUEFUNG[WP-1]: 1 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 3/3 geprüft

1. Oversized-post continuation/reporting remains unimplemented. The orchestrator accepted the 8-MiB cap without streaming raw, but requires a failed item with reason and actual size visible in the run report, followed by processing of later posts. `patch_source.rs:337-340` still produces a fatal source error; Core `lib.rs:75` propagates it. RunReport currently contains run_id/counts, and Store.reject requires a captured raw object ID. The missing owner-controlled nonfatal source-item/report interface is the concrete dependency. Fake raw bytes, a loader DB writer, silent skip and logging alone do not satisfy the condition.

Owner service insertion and the documented receipt checkpoint remain handoff boundaries. No gate ALLOW, main integration, deployment or live acceptance is claimed.

## Earlier review history

The following findings describe earlier checkpoints; current dispositions are recorded above.

Fresh read-only reviewer examined frozen Core2 `9f75588d`, parent production state `9192e226`, final source `5685f7e7` and schema `c4962120`. Component hashes matched commit blobs. No Cargo reruns or service changes were made by the reviewer.

WIRKUNGSPRUEFUNG[WP-1]: 3 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 3/3 geprüft

## 1. Explicit transport policy needs owner acceptance

Source `patch_source.rs` rejects rows above its configured title/url/raw_content transport ceiling, at most 8 MiB, before capture. Such an iterator error fails the run and prevents later rows from being read. This is an openly documented deviation from an unrestricted all-objects/raw-before-rejection interpretation of the ingestion contract, not proof of arbitrary complete raw capture. The independent review confirmed it; within-cap poison and 300-MB checks do not erase the boundary.

Decision needed from the orchestrator: explicitly accept the capped transport failure policy for this package, or have P1r publish a streaming raw-capture contract that can persist oversized content without a complete in-memory vector. Raising the cap alone cannot reconcile unbounded raw objects with a fixed process-memory requirement. Preferred bounded scope is explicit acceptance of the documented policy, based on the measured current feed size, without claiming isolation beyond the transport boundary.

Details and measured limits: `SOURCE_HANDOFF.md`.

## 2. Complete result binding is missing from P4 revalidation

At `rust/crates/brain-serve/src/pg_patch_tools.rs:96-112`, revalidation compares only the selected evidence vector. For three visible statements and limit 1, quarantining the receipt of an omitted statement can leave the selected result row and receipts unchanged while changing omitted_count from 2 to 1. The stale result then validates successfully.

Twin: observed owner `pg_tools.rs:329-335,545` binds complete FindEntity/GetHero results and compares the full dependency, using the central request digest protocol in `tools.rs`. P4's adapter correctly routes to the P4 checker, so the adapter does not fix this defect.

Required owner-coordinated fix: extend that single central result-binding protocol specifically to CanonicalPatchHistory; bind the complete result when constructing the P4 dependency, remove the seal for fresh execution and compare the complete new dependency. Preserve the existing legacy PatchHistory pin contract. Do not create a private result cache, a second digest API, fabricated evidence or remove useful omission metadata to conceal the race. At review time the owner checkpoint was not approved for merging, so no frozen contract edit was made. Subsequently the orchestrator explicitly approved merging e8a1ef45 and designated P1r's receipt format canonical. A fresh isolated P4 worker is composing that checkpoint and extending its existing result-binding protocol only for P4's canonical subtype. This decision is not a completed fix or acceptance of the transport policy.

## 3. Adapter fixture incompatibility is fixed in the coherent candidate

The old parent fixture referenced the deleted v2 migration, applied P4 manually after the migration registry and emitted numeric-only patch_change keys. Verified component integration `9118b765` fixes all three in `brain-serve/tests/pg_patch_port.rs`: registered migration [1,4] once, explicit rerun/idempotence checks, reference-fingerprint keys and extractor version 2. It leaves source/schema production artifacts byte-identical to their verified component commits.

Combined affected tests passed 23/23, no failures or ignored tests, with the unrelated Core2 cache fixture explicitly filtered. Formatter, check and both scoped Clippy runs passed. Parent independently checked actual log markers and byte equality before transfer. This resolves the fixture finding, not the two outstanding contract issues above.

## Existing owner integration blocker

The independent review retained the previously confirmed shared Receipt-Evidence serialization conflict and missing owner service insertion. Details: `RUNTIME_HANDOFF.md`. No evidence-ID renaming, weakened kernel guard, invented historical hero values or direct canon/raw tool access is permitted.

## Acceptance status

Pure extraction budget, within-cap object isolation, measured bounded memory, repeat raw delta 0, additive sparse v4, quarantine-safe reference indexing and bounded composed P4 execution are verified component behaviors. Fresh read-only confirmation of actual parent `647b1524aff80da68d4a28181ba05827f8825472` matched source, schema and combined worker blobs; no unreviewed production or unintended frozen-core change was found. Fixture issue 3 is closed. Issues 1 and 2 remain unresolved; the later authorized receipt/digest fix is still pending. No final gate ALLOW, main integration, deployment or live acceptance is claimed.

WIRKUNGSPRUEFUNG[WP-1]: 2 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 3/3 geprüft
