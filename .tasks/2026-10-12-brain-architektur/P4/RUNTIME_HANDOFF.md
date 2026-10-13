# P4 runtime handoff

Observed owner checkpoint: `e8a1ef45788faeb988464bf44bf8f0932f12c09c` on `feat/brain-p1r-hero-slice-20261012`, 2026-10-13.

The orchestrator explicitly approved merging `e8a1ef45` on 2026-10-13, merge not rebase, and decided that the P1r core receipt format is canonical. P4 must adapt to it exactly; P1r will document and publish its receipt checkpoint. A fresh isolated P4 worker is composing the approved owner code and adapting P4 receipts/result binding. P1r remains the main integrator and service owner. `origin/main` was still `edf503b280777524093307a97fabbb1bdba83d7c` at the original inspection.

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

## Confirmed cross-tool receipt conflict

Owner `pg_tools.rs:563-595` and P4 `pg_patch_tools.rs:331-388` create different `Evidence` objects for the same receipt ID:

| Field | P1r | P4 |
| --- | --- | --- |
| evidence_id | Decimal receipt ID | Same decimal receipt ID |
| source_id | `canon.source/<source>` | `<source>` |
| content_sha256 in content | Missing | Present |

P4 includes hero entity, alias and optional matching-stat receipts in patch results. These overlap the receipts already returned by find_entity/get_hero. `brain-kernel/src/execution.rs:171-177` rejects different Evidence values for the same ID with `InvalidResponse("Beleg-ID ist mehrdeutig")`. The reviewer report was independently checked against the published owner blob and P4 source.

Required fix under the explicit decision: P4 emits the owner's canonical receipt-evidence serialization and source identity exactly, without additional content fields or different JSON construction. Raw object integrity, stored hashes and immutable receipt identity remain unchanged. Do not relax the kernel equality guard or rename receipt IDs to hide the conflict. After owner contract publication, prove the actual combined find_entity/get_hero/patch_history path with real PostgreSQL and the kernel tool loop.

## Merge and dependency invariants

Owner `tools.rs` adds FindEntity/GetHero and result digests. Preserve P4's CanonicalPatchHistory types, parser and pin-free exception when merging the sibling checkpoint. Legacy PatchHistory must not become pin-free.

The owner digest contract presently applies only to FindEntity/GetHero. P4 remains receipt-only revalidated. If the owner extends the shared digest contract to canonical patch_history, use subrequest-specific result binding, `without_canonical_result` before fresh execution and full dependency comparison. Do not call P4's existing equality check at `pg_patch_tools.rs:57` with a sealed request.

GetHeroRequest already includes `patch: Option<String>` in the owner checkpoint. `pg_tools.rs:160-174` rejects non-current historical requests before SQL with `historical_patch_unavailable` and no evidence. Preserve that behavior until fact-to-patch assignments are actually evidenced. Original patch change statements do not establish historical hero profiles.

TracedTools records generic tool names and positive receipt IDs. It can capture patch_history with no trace change if it stays outside the P4 wrapper. None of these read-only findings is a fresh build, gate, runtime integration or live proof.
