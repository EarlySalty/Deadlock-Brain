# P4 runtime handoff

## Final tested Core3a composition

The approved982 normal composition now includes byte-identical transfer of the fresh fixer's final four P4 adapter/fixture files. Parent read actual markers and verified final patch/file hashes and equality; integrated formatter and whitespace checks passed. Standalone15/0/0/0, combined owner/P4 port11/0/0/0 and final Clippy are green. Parent shared service::pg_tools suite is separately4/0/0 with98 filters; four tests overlap the combined target. The previously failing owner fixture is closed on982. Actual cross-tool canonical Evidence equality, kernel publication and numeric cold/warm PostgreSQL traces passed with fixture-only derived budget54851.

Content-addressed complete results enforce the unchanged16384-byte bound: two visible statements reconstruct to16625 bytes and are correctly bounded to14611; three real serving rows reconstruct to21740 bytes, two-row prefix19714, final14407. Every original receipt and all omission-only quarantine/revalidation properties remain checked. Test-only ANALYZE uses genuine disposable tables; no production SQL/view/index/provider/limit change. Actual complete tool peak485813µs, earlier timeouts and their unresolved timing variance are disclosed in RUNTIME_VERIFICATION.txt.

Verified branch commit/push and final gate are next. Original owner profile/alias coherence remains an open finding. Owner service insertion remains a handoff boundary. No ALLOW, main merge, deployment or service activation is claimed.

## Approved Core3a update, 2026-10-13

Orchestrator approved982645cf325952b56c782fe690f6e23a5a76bf17; diff from19dec0c9 is only owner pg_tools_tests.rs and P1r notes. Parent's normal pending merge now targets982, after losslessly preserving its own P4 changes. The authorized owner test now creates a genuine failed unpublished ingest fixture and tolerates additive tool registrations. Production Core, migration7, canonical receipt format and the remaining coherence finding are unchanged. Requested service::pg_tools suite actually completed with --include-ignored --nocapture --test-threads=1:4 passed/0 failed/0 ignored/98 unrelated filters, exit0 locally. Genuine failed fixture remains absent from published_receipt and hero_profile. Log `/tmp/p4-core3a-service-pg-tools-20261013.log`; formatter/whitespace checks passed. This resolves the owner fixture failure on982 but is not a final P4/runtime or gate verdict.

## Current Core3 composition, 2026-10-13

Approved owner checkpoint19dec0c96d4765cff9a9a5dd37f5ad2a84bbe7d5 supersedes the earlier numeric-ID receipt format below. Parent normal merge is still open. The canonical contract is now `docs/gamefacts-tool-receipts.md` and public `brain_contracts::canonical_receipt::{CanonicalReceipt,format_receipt}`; evidence/citation IDs are content-addressed, while raw_receipt_id remains a numeric SQL/trace key. P4 must not serialize or hash a separate receipt format.

Fresh fixer adapted the four P4-owned adapter/fixture files, transferred byte-identically to the parent. Production conversion validates authoritative serving metadata, calls format_receipt, remaps all displayed authoritative citations, bounds the complete serialized execution strictly below16384 bytes and seals the final ToolResult using the existing central protocol. Read-only repeatable-read, brain_tool,500ms,LIMIT20 and legacy pinning remain unchanged. There is no production model/budget/provider change.

Actual worker runtime target10 passed/1 failed/0 ignored. The actual owner/P4 kernel publication loop and cold/warm PostgreSQL traces passed with derived fixture input budget54851; shared Evidence equality and numeric trace IDs are verified. The failure is frozen owner pg_tools_tests.rs:803 mutating a succeeded run to failed, correctly rejected by v7 with23514. Separate standalone worker target waited600s and ran0 tests. Parent integrated target subsequently ran13 passed/2 failed/0 ignored/0 filtered remotely: ranged fixture assertions at pg_patch_tools.rs:1185/713 expected3/2 rows but received1/1 after new-format bounding. Parent stopped the redundant local fallback; fresh fixer is correcting P4 fixture geometry while retaining the unchanged output bound and omission-only quarantine proof. No green full-runtime claim is made.

Original gate owner coherence finding also survives Core3/v7 for superseded hero fields and exact/fuzzy aliases. Both corrections require an authorized P1r checkpoint; P4 does not edit frozen owner files. Owner service insertion below remains a boundary, not a completed deployment. Gate remains BLOCK, no main integration or live acceptance.

## Historical e8 checkpoint

Observed owner checkpoint: `e8a1ef45788faeb988464bf44bf8f0932f12c09c` on `feat/brain-p1r-hero-slice-20261012`, 2026-10-13.

The orchestrator explicitly approved merging `e8a1ef45` on 2026-10-13, merge not rebase, and decided that the P1r core receipt format is canonical. Approved owner merge2e854ea7 and P4 runtime fix67ae30ad are normally merged and published through parent1628ba07. P4 now emits the owner format exactly and binds complete canonical results through the central digest protocol. P1r's documented receipt checkpoint and productive service insertion remain outstanding. P1r remains the main integrator and service owner. `origin/main` was `edf503b280777524093307a97fabbb1bdba83d7c` at the original inspection; this is not a current live-state claim.

## Registration at the owner boundary

At owner `rust/crates/brain-serve/src/service.rs:623-637`, wrap the first delegate argument to `pg_tools::PgTools::new` in:

```rust
crate::pg_patch_port::PgPatchToolPort::new(
    delegate,
    gamefacts_pool.clone(),
    tokio::runtime::Handle::current(),
)
.map_err(|_| Error::ReaderUnavailable)?
```

The intended composition is `TracedTools<PgTools<PgPatchToolPort<deadlock_data::Tools<ReleaseToolExecutionPort>>>>`. The reader pool already exists at line 590; clone it before its move into PgTools. This is an owner-controlled service insertion, not an available append-only registry entry. Do not create a second pool or dispatcher.

## Cross-tool receipt conflict resolved at67ae30ad

Before the fix, owner `pg_tools.rs:563-595` and P4 `pg_patch_tools.rs:331-388` created different `Evidence` objects for the same receipt ID:

| Field | P1r | P4 |
| --- | --- | --- |
| evidence_id | Decimal receipt ID | Same decimal receipt ID |
| source_id | `canon.source/<source>` | `<source>` |
| content_sha256 in content | Missing | Present |

P4 includes hero entity, alias and optional matching-stat receipts in patch results. These overlap the receipts already returned by find_entity/get_hero. `brain-kernel/src/execution.rs:171-177` rejects different Evidence values for the same ID with `InvalidResponse("Beleg-ID ist mehrdeutig")`. The reviewer report was independently checked against the published owner blob and P4 source.

The implemented fix emits the owner's canonical receipt-evidence serialization and source identity exactly, without additional content fields or different JSON construction. Raw object integrity, stored hashes and immutable receipt identity remain unchanged. The kernel equality guard is unchanged and receipt IDs are not renamed. The real PostgreSQL/kernel test exercises actual FindEntity/GetHero and PgPatchToolPort and proves byte-identical alias/entity/stat evidence. Commands, logs and hashes: `RUNTIME_VERIFICATION.txt`. This proof does not substitute for the outstanding owner service insertion or receipt-contract documentation.

## Merge and dependency invariants

Owner `tools.rs` adds FindEntity/GetHero and result digests. Preserve P4's CanonicalPatchHistory types, parser and pin-free exception when merging the sibling checkpoint. Legacy PatchHistory must not become pin-free.

Fix67ae30ad extends the existing central result-binding protocol specifically to CanonicalPatchHistory. P4 binds the complete result after truncation, removes the seal with `without_canonical_result` before fresh execution and compares the complete new dependency. The selected evidence can remain identical while omitted_count changes; the real three-post quarantine regression rejects that stale result for Provider, Publication and Cache and accepts the fresh dependency. Legacy PatchHistory pinning remains unchanged; no private result cache or second digest API was added.

GetHeroRequest already includes `patch: Option<String>` in the owner checkpoint. `pg_tools.rs:160-174` rejects non-current historical requests before SQL with `historical_patch_unavailable` and no evidence. Preserve that behavior until fact-to-patch assignments are actually evidenced. Original patch change statements do not establish historical hero profiles.

TracedTools records generic tool names and positive receipt IDs. It can capture patch_history with no trace change if it stays outside the P4 wrapper. None of these read-only findings is a fresh build, gate, runtime integration or live proof.
