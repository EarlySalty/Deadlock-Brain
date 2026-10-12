# P2: Items and abilities, ingestion v2

## Goal and boundary

Extend the P1r ingestion core with receipt-backed Assets item and ability facts, item relations, canonical cards, and `get_item`, `get_ability`, `compare`. Preserve the existing answer path and legacy tools. Do not import the stopped P1 runner, store, leases, pending states, or recovery machinery.

Branch: `feat/brain-p2-items-abilities-20261012-v2`.
Base: `origin/main` at `edf503b280777524093307a97fabbb1bdba83d7c`.
Reference only: `origin/feat/brain-p2-items-abilities-20261012` at `4d2fa43c0f32b02f4836756d86e7cdc10db75aa6`.
Integrator: this P2 session, after P1r is live and the local gate allows the final SHA.

## Binding inputs

- `../CONTRACT_INGEST_V2.md` and `../REDESIGN-INGEST.md` in the domain task directory are authoritative.
- `../BRIEFING_WELLE2_GEMEINSAM.md` supplies the runtime tool contract; its old ingestion lifecycle is superseded.
- Published core: `4cc0cf5d1843897b7519214cb2117e1aa8f848ed`, merged at `e2e91e5a92884a7be7e37f1c1d29d1107011d6d2`. Actual API names win over abbreviated contract stand-ins. No later P1r merge until its SHA is explicitly forwarded; never rebase published P2 work.
- Core files and migration v1 are frozen. Only the package module declaration in `brain-job/src/lib.rs`, extractor registration, source registration and reserved migration entry may be added. Other core needs go to the main orchestrator with exact signatures.
- Core 2 is routed by the main orchestrator: applied-object idempotence with rejected-object retry, streamed multi-object sources, `RAW_BYTES`, `validate_numeric_lexeme`, and `kv_path`. Do not duplicate these changes or ask for them again.
- Every gamefacts tool and serve view reads only `serve.current_entity`, `serve.current_alias`, `serve.current_fact`, `serve.current_relation`, or views built on them. Never read `canon.*` or `raw.*` in tools or serve views; P9 quarantine is inherited through `serve.published_receipt`.
- Extraction has no database or catalog lookup. Cross-package references are emitted by normalized key or alias text, resolved at read time, and hidden while unresolved. The qualified-fact representation for conditions and units remains an open core contract; do not silently omit it or encode JSON as scalar text.
- All compiler checks use the current `cargo-slot` routing and Rust build rules.

## Ownership

- Extractor worker: P2 Assets extractor and focused lexical, collision, oversize, and poison-run tests only. Pure `RawObject` to `Outcome`; central `limits::validate` before writes; extractor and tests together below 800 lines. No replacement core, store, or guessed core row types.
- Cards worker: P2 typed contracts, item/ability/compare serving modules, focused serving tests, and additive version-3 migration/views/indexes/grants. Reuse only P2-owned semantics from the reference branch. No copied constraint guards or alterations of the core migration.
- Parent integration worker, after the actual core arrives: minimal append-only shared wiring, source dispatch, ordered migration registration, actual core compatibility, timer contract and integrated verification. One writer per shared path.

## Acceptance

1. The real per-object runner records two poisoned raw objects as rejected and ends succeeded. Unchanged capture adds zero raw objects and facts; lexical value hashes preserve original JSON quotes and escapes.
2. Item and ability cards carry genuine receipt identity and explicit omission status. Four entities by twelve stats, including the complete serialized ToolResult, remain within 16,384 bytes. Read-only `brain_tool`, 500 ms, parameterized SQL and explicit limits remain enforced.
3. Actual item/compare query plans use indexed entity/stat lookup without planner settings that force the result. The real Assets job cgroup-v1 peak is below 300 MB.
4. The normal answer endpoint handles Toxic Bullets cost/effect and Haze/Wraith weapon comparison. `ops.answer_trace` shows the new tools without `deadlock_data` for those requests.
5. A fresh reviewer reads the complete integrated diff against the redesign before the local merge gate. No integration or deployment before P1r-live. Release uses verified `origin/main` through `brain-release install`; prove the affected live function before cleanup.

Ready-for-gate target: 20:30 CEST, conditional on the P1r core around 19:15 CEST. Work records stay on the feature branch, not in a documentation-only main commit.
