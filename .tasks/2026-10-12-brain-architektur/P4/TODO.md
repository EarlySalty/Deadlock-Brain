# P4 continuation point

Sol resumed 2026-10-13. Current verified P4 code checkpoint: `0a79e1e4` on `feat/brain-p4-patches-20261012-v2`. Approved owner e8a1ef45 and runtime fix67ae30ad are retained through parent1628ba07;0a79e1e4 adds the verified orchestrator-approved source skip/log/count fix. Runtime proof246 passed and fresh source/ingest/extractor proof18 passed are recorded separately. Documentation HEAD may advance independently.

## Resume progress 2026-10-13

- `64fc6527` merged published P1r Core 2 and Core 2b through `9f75588d`; sparse migration registry and streaming sources are available. Dependency/module merge conflicts preserved both sides. Formatter passed. The standalone lib check was stopped before acquiring a slot; compiler-backed checks will run on the integrated loader/migration state.
- `158cc695` added the package-owned five-minute `brain-job@deadlock_patchnotes_db.timer`; systemd calendar and unit verification passed. No timer/service was activated.
- Live feed catalog checked read-only through Infisical: 132 posts, 132 dated, 746135 raw-content bytes total, largest 64006 bytes, three posts containing Baba. Columns verified: id bigint; title/url/raw_content/translated_content text; posted_at timestamptz. No credentials or raw DB contents printed.
- Source, schema and adapter component workers completed; their verified artifacts and Git ancestry are retained. Runtime fixer `aa59d1688bbf86f50` completed approved owner merge2e854ea7 and receipt/result fix67ae30ad, normally merged into parent1628ba07. Fresh reviewer `a3f637df6130c3776` confirmed this actual parent: receipt mismatch and omission-result revalidation are fixed; one unresolved finding remains, oversized-post continuation/reporting. No executing worker is pending.
- Build issue resolved by orchestrator 2026-10-13 09:50 CEST: 39 GB freed on ionos and normal remote-first cargo-slot restored. Earlier checks stopped while waiting ran zero tests; do not count them as failures or proof. No worker may delete anything under ~/cargo-remote.
- Core 2 `9f75588d` explicitly approved by the orchestrator. Independent source review found missing runtime patch_history execution/dependency registration. Package-owned `PgPatchToolPort::new(delegate, reader_pool, runtime_handle)` is published at `5be26287`, selectively integrated from worker `2def24a5` in isolated `agent-a7c768fa3a2060c71`. Its real scratch-PG tests passed 4/4 with no ignored tests; check, formatter and Clippy passed. Evidence: `/tmp/p4-patch-port-test-final-a7c768fa.log`. These checks used the old numeric-key P4 projection; fixtures and integrated proof must follow the new fingerprint migration. Shared service/dispatcher files remain read-only.
- The orchestrator explicitly approved `e8a1ef45788faeb988464bf44bf8f0932f12c09c` on 2026-10-13 (merge, no rebase) and decided the P1r core receipt format is canonical. P4 now emits it exactly at fix67ae30ad, integrated through parent1628ba07. P1r's documented receipt checkpoint and productive service insertion remain handoff boundaries; no current values may be substituted for historical hero values.
- Earlier runtime compatibility inspection confirmed a cross-tool blocker: P1r and P4 serialized the same hero receipts with different source IDs and different content metadata, so the kernel rejected duplicate evidence IDs. Fix67ae30ad resolves this without an equality-guard bypass, citation renaming or loss of raw content hash. Owner service insertion remains recorded in `P4/RUNTIME_HANDOFF.md`. Historical get_hero has an optional patch argument in the owner checkpoint and honestly rejects non-current requests without evidence. No new historical profile implementation is inferred.
- Both source and schema workers stopped on API ECONNREFUSED and were resumed in their existing isolated contexts after state inspection. No reported pending check is counted as passed. Source worker was instructed not to block affected P4 checks on the unrelated ignored Core2 hero-cache test; that optional attempt lacked BRAIN_CORE2_HERO_REPOSITORY. Already accepted Core2 proof is not rerun.
- Index decision approved by orchestrator 2026-10-13: equivalent index-backed access with alias resolution at read time, no persisted projection or refresh contract. Acceptance is measured p95 under 5 ms on scratch Postgres with realistic row counts and recorded EXPLAIN. The schema worker owns compatible P4 view/index optimization. Its first one-change probe showed 455.766 ms planning and 5.133 ms execution with default planner, confirming a performance gap, not acceptance. Historical hero state still needs evidenced patch assignments and the owning P1r interface.

- Schema handoff `c49621202a8c9097dbfb7eb6b3d26865995d6bb0` is clean and published on its own worker branch. Relevant proof independently read: upgrade 1/1, sparse registry 2/2, serving 3/3, all with no ignored tests; fmt/check/clippy exit 0. Measurement artifact hashes match the worker notes. Prepared CHANGES p95 is 4.264 ms on the disclosed synthetic fixture; instrumented EXPLAIN sum p95 is 5.725 ms, not a whole-tool or production claim. The parent will integrate this together with extractor version 2, not publish incompatible numeric-key extraction against the new migration.
- Source handoff `5685f7e72a5cde9fafb748d0565a2286146544d9` is clean and published on its own worker branch. The unchanged worst-case memory contract is now satisfied: client VmHWM 258748416 bytes against 300000000, four 8-MiB rows with maximal escaping and backpressure, raw objects 4 to 4. Separate 132-post repeat also has raw delta 0. Earlier real failures at 359378944 and 309030912 bytes were fixed by demand-driven FETCH, removal of envelope queueing/prefetch and vector overcapacity, not a higher threshold. Affected tests 18 passed/1 existing ignored/0 failed, formatter/check/clippy/build exit 0; parent read actual log markers. Exact evidence, configuration and the explicit transport boundary are in `P4/SOURCE_HANDOFF.md`.
- Verified coherent integration candidate `9118b765f7078e72fd5cf9a44373cc877e7d5ca7` combines the exact source/schema artifacts with corrected port fixtures (registered v4 once, fingerprint keys and version 2). Combined affected checks passed 23 tests, no failures or ignored tests, with one unrelated Core2 cache fixture explicitly filtered; formatter/check/two scoped Clippy runs passed. Combined client RSS was 242405376 bytes with raw delta 0. Parent read the actual logs and verified byte equality of transferred Rust/migration/performance artifacts. No owner checkpoint or service change is included.
- Initial independent review found three issues, recorded historically in `P4/REVIEW.md`. Obsolete fixtures were fixed by the coherent candidate; central complete-result binding and shared receipt serialization were fixed at67ae30ad and independently confirmed on parent1628ba07. The orchestrator accepted the transport cap conditionally, but oversized-post continuation/reporting is still missing. Current review retains that single code finding plus owner handoff boundaries. Neither a green component suite nor the stop hook authorizes main integration or a gate/live claim.

## Previously done and verified


- 4fcf6bad: pure extractor `brain-job/src/patch_extract.rs` plus contract tests, 796 lines. patch_extract_contract 9/9 passed, including the real Store poison run on scratch Postgres (value_too_large and key_collision rejected, run succeeded, following run free).
- 68cfc1cc: serve-only projection SQL (`serve.p4_patch_catalog`, `serve.p4_patch_change`, `serve.p4_patch_receipt`) and canonical patch_history tool. No canon/raw reads. patch_history_canonical 5/5, brain-serve lib pg_patch 9/9, pg_patch_tools scratch-PG integration 2/2.
- 1b7e586b: extractor registered in registry.rs; clippy -D warnings clean for brain-job.
- Sources: wf_89554428-49f-1 (extractor) and wf_89554428-49f-2 (serving). Older drafts in wf_4be1e21f-3e7-1/-2, wf_99e78d62-197-1/-2 and p4-pure-extract-20261012 superseded, left untouched.

## Current verified runtime result

Runtime fix67ae30ad is normally merged and published through parent1628ba07. The actual owner FindEntity/GetHero and P4 adapter emit byte-identical overlapping receipts in a real PostgreSQL/kernel loop. CanonicalPatchHistory uses the existing central complete-result digest; omission-only quarantine changes reject stale dependencies for Provider, Publication and Cache. Legacy pinning and the complete 16384-byte bound remain intact. Final affected runtime checks: 246 passed, 0 failed, 0 ignored, 0 filtered; formatter/compiler/Clippy exit 0. Commands, actual logs, hashes and earlier failed attempts are in `RUNTIME_VERIFICATION.txt`. The fresh reviewer confirmed the published parent's tested code equality and these fixes. This is not a productive service, gate or live claim.

## Merge-gate startup blocker, 2026-10-13

The stop hook requested completion of the parent and four contained worker branches. The assigned package rules still reserve main/deploy for P1r and prohibit deleting the old worker worktrees. Refreshed origin/main remains edf503b280777524093307a97fabbb1bdba83d7c; the published P1r candidate remains e8a1ef45788faeb988464bf44bf8f0932f12c09c.

Actual required gate command, run twice (initial attempt plus one startup retry):

```text
python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/Deadlock-Brain-p4-patches --base main --head feat/brain-p4-patches-20261012-v2
```

Both attempts exited 2 without a reviewer verdict. gpt-6.1-sol failed while bwrap mounted the review output; claude-opus-5-5 failed at unshare. Both reported `Cannot allocate memory`. Task IDs: bi48r795i and bl6gkl7gd. Output files are retained under `/tmp/claude-1000/-home-nathanael--worktrees-Deadlock-Brain-p4-patches/58c3ac6f-3049-4932-93a8-78e1ab4d2a1e/tasks/`. This is no ALLOW and no code BLOCK. Read-only diagnostics showed approximately 12 GB available memory, no effective user-slice memory cap, 92 mounts against 100000 and high user/mount namespace limits; the precise sandbox allocation cause is not established. No protection bypass, foreign-process termination, gate change, merge or branch deletion was attempted.

The independent oversized-post source/report finding remains unresolved regardless of gate startup. Continue with the actual P1r source-item/report interface, targeted source fix and required regression; restore normal review sandbox startup before obtaining ALLOW.

MERGEPROTOKOLL[MS-1]: 0 Git-Schritte einzeln | Anläufe: 2 | Gate: kein Urteil, Exit 2, bwrap/unshare Cannot allocate memory

## Latest orchestrator decision, 2026-10-13

The orchestrator confirmed normal bwrap sandbox startup is restored after transient host memory pressure and requested another gate attempt on14188bcf or later. The source policy is explicitly revised: skip oversized posts inside P4 before the Core iterator, emit one structured log per post with ID/reason/byte size and count them; prove following valid posts are ingested. Core-level nonfatal item reporting is deferred until after P1r is live. This supersedes the earlier P1r report-interface prerequisite and the earlier prohibition on a log-based interim report. Frozen Core stays unchanged.

The scoped source fix is verified: SQL returns only size metadata for oversized rows; one demand request scans past skipped rows until the next bounded object; per-post JSON diagnostics include the cumulative skipped count, and successful source completion emits scanned/emitted/skipped counts. No fabricated raw object or new persistence path is introduced. Affected tests18 passed/0 failed/0 ignored, with2 disclosed unrelated filters. Check, Clippy, formatter and executable build exit0. The actual log/count/Core continuation proof and commands are in `SOURCE_HANDOFF.md`; new client peak254627840 bytes and raw repeat delta0. Extractor/contracts remain799 lines.

## Actual gate BLOCK after source completion

Normal review sandbox startup is restored. The --base main attempt on0a79e1e4 failed without a verdict because local main cf02c9a0 was stale and included4.8M characters of unrelated already-released history. The subsequent required gate uses the full fetched published baseline origin/main edf503b2, without changing any main checkout or gate setting. Task bk2qhgspo exited1: `[gpt-6.1-sol] BLOCK: Ingest preflight fails, and current-state publication can remain stale.`

Three blocking findings affect frozen P1r files: ingest-role preflight requires an ungranted serve schema; historical was_applied skips A→B→A restoration; hero/alias readers retain fields and aliases from superseded objects without object/run/extractor coherence. Exact locations and nits are in `REVIEW.md`. Fresh isolated fixer a477e3abbfdfb2d36 is verifying the actual causes and scope. No ALLOW, alternate-model reroll, weakened grant or source workaround is claimed.

MERGEPROTOKOLL[MS-1]: 0 Git-Schritte einzeln | Anläufe: 1 inhaltliches Review | Gate: gpt-6.1-sol BLOCK, Exit1,3 owner/Core findings

## Approved Core3 checkpoint

The orchestrator approved normal merge of19dec0c96d4765cff9a9a5dd37f5ad2a84bbe7d5 from feat/brain-p1r-hero-slice-20261012. It supplies schema/serve readiness, canonical receipt documentation, A→B→A publication repair and the registered deadlock-data hero extractor with real CLI/idempotence proof. Additive core migration7 is reserved for P1r; P4 retains4. Core files remain frozen. Parent will preserve P4 source/contract/projection code, merge without rebase, check exact receipt compatibility and rerun affected checks and the same gate. Fresh fixer a477e3abbfdfb2d36 has the approved checkpoint for read-only verification of the actual BLOCK.

## Open

1. Normally merge approved Core3, preserve both reserved migrations4/7 and the canonical P4 result contract, then receive the fresh fixer's compatibility/causal report. Verify source and actual combined owner/tool behavior before rerunning the same gate. Do not conceal remaining owner bugs by reviewing only the latest source delta.
2. Coordinate P1r's owner-controlled service wrapper insertion. Optional get_hero.patch must retain historical_patch_unavailable without evidenced assignments.
3. Hand over the gate-ready branch according to the assigned P1r live/integration order. Only the responsible integrator merges/deploys; no source activation or live acceptance is claimed.

