# P2: Items and abilities, ingestion v2

## Goal and boundary

Extend the P1r ingestion core with receipt-backed Assets item and ability facts, item relations, canonical cards, and `get_item`, `get_ability`, `compare`. Preserve the existing answer path and legacy tools. Do not import the stopped P1 runner, store, leases, pending states, or recovery machinery.

Branch: `feat/brain-p2-items-abilities-20261012-v2`.
Base: `origin/main` at `edf503b280777524093307a97fabbb1bdba83d7c`.
Reference only: `origin/feat/brain-p2-items-abilities-20261012` at `4d2fa43c0f32b02f4836756d86e7cdc10db75aa6`.
Integration boundary: this P2 session publishes a verified gate-ready feature branch; it does not integrate main or deploy. The dedicated P1r-F team now owns P1r's gate, main merge, release and live proof; the old P1r thread is stopped and receives no questions. P2 main integration remains an orchestrator-controlled handover.

## Binding inputs

- `../CONTRACT_INGEST_V2.md` and `../REDESIGN-INGEST.md` in the domain task directory are authoritative.
- `../BRIEFING_WELLE2_GEMEINSAM.md` supplies the runtime tool contract; its old ingestion lifecycle is superseded.
- Approved checkpoints, merge without rebasing: Core 2b `9f75588ddee83422e6a7755865e4a5c44fa2e33d`, executor `e8a1ef45`, Core 3 `19dec0c96d4765cff9a9a5dd37f5ad2a84bbe7d5` and Core 3a `982645cf325952b56c782fe690f6e23a5a76bf17`. Receipts follow the exact shared Core 3 canonical formatter/resolver contract. Core 3a changes shared fixtures and P1r notes, not production code or migrations. Never rebase published P2 work.
- Final orchestrator allocation at 04:55 on 2026-10-14: P1r-F is reported live on `origin/main` at `46c75d67aac0b60c279881f7089a9e9204faea3b`, using versions 1/2/7/8/9/10. The former 11-20 reservation is released. P2 keeps published v3/v11/v13 unchanged; no renumbering. Version 21 remains P2-owned for any necessary additive Core fix. Merge main and the published round-4 repair without rebasing or force pushing; preserve applied migration bytes. Core 3b will not be published and P1r-F answers no Core questions. P2 hands over an ALLOW feature branch, without main integration, release build or deploy.
- Core 2 is routed by the main orchestrator: applied-object idempotence with rejected-object retry, streamed multi-object sources, `RAW_BYTES`, `validate_numeric_lexeme`, and `kv_path`. Do not duplicate these changes or ask for them again.
- Every gamefacts tool and serve view reads only `serve.current_entity`, `serve.current_alias`, `serve.current_fact`, `serve.current_relation`, or views built on them. Never read `canon.*` or `raw.*` in tools or serve views; P9 quarantine is inherited through `serve.published_receipt`.
- Extraction has no database or catalog lookup. Cross-package references are emitted by normalized key or alias text, resolved at read time, and hidden while unresolved. The approved representation keeps conditions and units in P2 qualifier sidecars and Serve projections without changing the frozen Fact model. Do not silently omit qualifiers or encode JSON as scalar text.
- Package verification follows the orchestrator's 2026-10-14 02:45 user decision: merge origin/main, matching cargo check, strict Clippy, formatter and matching tests, then the local gate. No package binary or release builds, brain-release runs, or private-CLI/binary proofs requiring a release build. The integrator combines all ALLOW packages, runs one combined test run and gate, builds the release once and deploys once with one live proof. Deferred runtime/binary evidence is listed under `Final live checks` in TODO.md. All compiler checks use the current `cargo-slot` routing and Rust build rules.

## Approved slot mapping, 2026-10-13

The orchestrator assigns slot mapping to P2; P1r does not supply it. Extract explicit `items.signature1` through `items.signature4` from the existing Assets Heroes endpoint through a P2-owned registered extractor. Publish one `hero has_ability ability` relation per explicit field with a scalar `slot` qualifier (1, 2, 3, 4; 4 denotes ultimate) and a canonical relation receipt. Resolve both ends through Core identity using source identifiers, not display names or list order. Ability references use source `class_name` to the already published P2 ability entity; hero identity must match what P1r actually publishes. Unresolved references are rejected per input item with a reason, never invented.

Read these mappings through an additive P2 Serve projection. Do not modify the P1r Hero extractor, frozen hero profile or Core types. Verify actual captured input with 41 heroes and 164 mappings, positive selector execution, unresolved/ambiguous refusal, exact receipts, stale-run isolation and unchanged replay. Continue to an actual local gate ALLOW. Core 3b will not be published. The orchestrator's 2026-10-14 02:25 message authorizes immediate origin/main uptake and matching checks/gate; preserve published history without rebasing or force pushing. P1r-F is reported live by the orchestrator's 2026-10-14 04:55 decision; P2 main integration remains unauthorized.

## Ownership

- Extractor worker: P2 Assets extractor and focused lexical, collision, oversize, and poison-run tests. Pure `RawObject` to `Outcome`; central `limits::validate` before writes. The orchestrator's 2026-10-13 clarification supersedes the aggregate size limit: each production file must stay below or at 800 lines, split along responsibilities; test files and static fixtures are exempt. No blanket exception, replacement core, store, or guessed core row types.
- Cards worker: P2 typed contracts, item/ability/compare serving modules, focused serving tests, and additive version-3 migration/views/indexes/grants. Reuse only P2-owned semantics from the reference branch. No copied constraint guards or alterations of the core migration.
- Parent integration worker, after the actual core arrives: minimal append-only shared wiring, source dispatch, ordered migration registration, actual core compatibility, timer contract and integrated verification. One writer per shared path.

## Acceptance

1. The real per-object runner records two poisoned raw objects as rejected and ends succeeded. Unchanged capture adds zero raw objects and facts; lexical value hashes preserve original JSON quotes and escapes.
2. Item and ability cards carry genuine receipt identity and explicit omission status. Four entities by twelve stats, including the complete serialized ToolResult, remain within 16,384 bytes. Read-only `brain_tool`, 500 ms, parameterized SQL and explicit limits remain enforced.
3. Actual item/compare query plans use indexed entity/stat lookup without planner settings that force the result. The real Assets job cgroup-v1 peak is below 300 MB.
4. The normal answer endpoint handles Toxic Bullets cost/effect and Haze/Wraith weapon comparison. `ops.answer_trace` shows the new tools without `deadlock_data` for those requests.
5. A fresh reviewer reads the complete integrated diff against the redesign before the local merge gate. P2 hands over its ALLOW branch without main integration or deployment. The integrator merges ALLOW packages into one integration branch, runs one combined test run and gate, builds one release from the verified integration state and deploys once with one live proof. Existing authorized deployment wrappers remain mandatory; P2 does not invoke them.

P2 remains a published feature-branch handover. The genuine 2026-10-14 04:55 decision releases the prior migration-allocation blocker and approves unchanged P2 v3/v11/v13. Proposed replacement v22/v23 are superseded, not used. Version 21 remains reserved for a necessary additive Core fix. P1r-F's live status is reported by the orchestrator, not independently verified by P2. Package main integration and deployment remain unauthorized.
