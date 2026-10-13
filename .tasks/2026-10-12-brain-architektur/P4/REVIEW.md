# P4 independent review follow-up

Fresh read-only reviewer examined frozen Core2 `9f75588d`, parent production state `9192e226`, final source `5685f7e7` and schema `c4962120`. Component hashes matched commit blobs. No Cargo reruns or service changes were made by the reviewer.

WIRKUNGSPRUEFUNG[WP-1]: 3 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 3/3 geprüft

## 1. Explicit transport policy needs owner acceptance

Source `patch_source.rs` rejects rows above its configured title/url/raw_content transport ceiling, at most 8 MiB, before capture. Such an iterator error fails the run and prevents later rows from being read. This is an openly documented deviation from an unrestricted all-objects/raw-before-rejection interpretation of the ingestion contract, not proof of arbitrary complete raw capture. The independent review confirmed it; within-cap poison and 300-MB checks do not erase the boundary.

Decision needed from the orchestrator: explicitly accept the capped transport failure policy for this package, or have P1r publish a streaming raw-capture contract that can persist oversized content without a complete in-memory vector. Raising the cap alone cannot reconcile unbounded raw objects with a fixed process-memory requirement. Preferred bounded scope is explicit acceptance of the documented policy, based on the measured current feed size, without claiming isolation beyond the transport boundary.

Details and measured limits: `SOURCE_HANDOFF.md`.

## 2. Complete result binding is missing from P4 revalidation

At `rust/crates/brain-serve/src/pg_patch_tools.rs:96-112`, revalidation compares only the selected evidence vector. For three visible statements and limit 1, quarantining the receipt of an omitted statement can leave the selected result row and receipts unchanged while changing omitted_count from 2 to 1. The stale result then validates successfully.

Twin: observed owner `pg_tools.rs:329-335,545` binds complete FindEntity/GetHero results and compares the full dependency, using the central request digest protocol in `tools.rs`. P4's adapter correctly routes to the P4 checker, so the adapter does not fix this defect.

Required owner-coordinated fix: extend that single central result-binding protocol specifically to CanonicalPatchHistory; bind the complete result when constructing the P4 dependency, remove the seal for fresh execution and compare the complete new dependency. Preserve the existing legacy PatchHistory pin contract. Do not create a private result cache, a second digest API, fabricated evidence or remove useful omission metadata to conceal the race. The observed owner checkpoint is not yet approved for merging, so no frozen contract edit is made here.

## 3. Adapter fixture incompatibility is fixed in the coherent candidate

The old parent fixture referenced the deleted v2 migration, applied P4 manually after the migration registry and emitted numeric-only patch_change keys. Verified component integration `9118b765` fixes all three in `brain-serve/tests/pg_patch_port.rs`: registered migration [1,4] once, explicit rerun/idempotence checks, reference-fingerprint keys and extractor version 2. It leaves source/schema production artifacts byte-identical to their verified component commits.

Combined affected tests passed 23/23, no failures or ignored tests, with the unrelated Core2 cache fixture explicitly filtered. Formatter, check and both scoped Clippy runs passed. Parent independently checked actual log markers and byte equality before transfer. This resolves the fixture finding, not the two outstanding contract issues above.

## Existing owner integration blocker

The independent review retained the previously confirmed shared Receipt-Evidence serialization conflict and missing owner service insertion. Details: `RUNTIME_HANDOFF.md`. No evidence-ID renaming, weakened kernel guard, invented historical hero values or direct canon/raw tool access is permitted.

## Acceptance status

Pure extraction budget, within-cap object isolation, measured bounded memory, repeat raw delta 0, additive sparse v4, quarantine-safe reference indexing and bounded composed P4 execution are verified component behaviors. No final gate ALLOW, main integration, deployment or live acceptance is claimed. Final integrated artifact comparison is still required. P1r/orchestrator must resolve the owner boundaries before completion.
