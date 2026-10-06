status: completed_code_only_review
Date: 2026-10-03

WIRKUNGSPRUEFUNG[WP-1]: 1 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 0/0 geprüft

# Independent local recheck of package B

Result: one verified remaining allocation-order defect. No approval for the complete tree: `game_files.rs` changed during this review. This report is a reading-based code review, not compiler, test, real-data, import, integration, or merge approval.

Reviewer used the inherited GPT 6.1 Sol model, with no model override or delegation. CONTRACT.md, BRIEFING-B.md, AN_BEREICHE.md through point 18, and the four findings in REVIEW.md were read before the code assessment. No network, Steam, Git, compiler, test, fmt, clippy, or second waiting check was invoked. The existing waiting compiler check was left alone. Only this report was written; no status event or shared register was changed.

The context-mode execution tool was permission-denied in this session. Read-only native file and search tools were used instead; permissions were not changed. Graphify was queried before code searches. Its returned graph did not locate the new uncommitted B modules, so the expressly assigned paths were read directly and the twins were checked with Grep.

All code paths below are relative to `/home/nathanael/.worktrees/brain-wiki-spielwissen-b/`. The branch and base HEAD stated in the briefing were not independently queried because Git operations were forbidden.

## 1. Open verified finding

### P2, F6: VPK payload capacity is allocated before the payload range is validated

Location: `rust/crates/dbrain-sources/src/game_files/vpk.rs:190-195`, with the late range checks at lines 218-229. Source SHA-256: `bff4f705b6ae326fe5a9f5c58afe7f2c73623692eb192900541fea364c656ed7`, unchanged at both review hash checks.

Concrete input, specified without executing a test:

* A 51-byte VPK v1 `pak01_dir.vpk`, signature `0x55aa1234`, version 1, tree size 39, and no embedded payload.
* Tree strings: extension `txt`, directory `scripts`, filename `items`, each NUL terminated.
* Its one record has CRC 0, preload length 0, archive index `0x7fff`, offset 0, length `4294967295`, terminator `0xffff`, then the three NUL strings closing the name, directory, and extension scopes.
* Otherwise valid options, on the current 64-bit Linux host, with `max_file_bytes = 4294967296`.

The directory reader accepts this structurally complete tree and records `embedded_bytes = 0`. The caller's configured input-size guard allows the declared resource length. `read_resource` then attempts `Vec::with_capacity(4294967295)` before noticing that the declared payload end exceeds the zero available embedded bytes. Thus a 51-byte invalid package requests almost 4 GiB of capacity before a deterministic bounds rejection. No Budget charge occurs at that allocation. The effect proved here is the allocation attempt, not measured resident memory or an observed crash. If allocation fails, this infallible allocation path can terminate before the caller records the normal inventory gap. A smaller configured limit reduces the requested capacity but does not fix the validation order.

Caller binding: in the initially read `game_files.rs`, lines 235-246 perform the size guard and call; the targeted reread of the changed file places the same guard and call at lines 241-252. `validate_options` allows the above nonzero limit below `usize::MAX`, initially lines 84-85 and subsequently lines 90-91.

Twin search, Grep-backed:

* Embedded VPK and sibling VPK payloads both enter the same allocation at `vpk.rs:194`. For sibling payloads even the file lookup and opened-file size check follow the reservation. Both are affected.
* The later `resize` at `vpk.rs:232` is after range validation, but cannot undo the earlier capacity reservation.
* VPK tree bytes at lines 87-90 are charged before allocation. Repeated directory/name/preload expansion at lines 101-131 is charged before construction. These paths are unremarkable for this defect.
* Loose files use a bounded `Read::take` and a separate growth check, initially `game_files.rs:307-324`. They do not reserve capacity from an unchecked VPK length field.

Required correction: validate the declared range against the actual held payload file before reserving or copying bytes; enforce the applicable allocation budget at that point and return a recoverable error that becomes a visible inventory gap. Keep the held-directory protection for sibling archives. Increasing a limit does not address this defect.

## 2. Recheck of the four original findings

These are code-level judgments about the inspected paths. They do not approve the subsequently changed complete tree.

1. Original allocation amplification: the long-parent-key and repeated-prefix cases are addressed in the inspected KV, KV3, JSON, fact-construction, and VPK directory paths. `budget.rs:15-24` uses checked arithmetic and cumulative charges. KV charges token storage, key/prefix expansion, leaf copies, and inherited conditions before allocation at `kv.rs:59-77,89-92,130-188`. KV3 charges flags, object paths, array paths, leaf fields, and tokens before their copies at `kv3.rs:64-67,83-98,112-162,198-265`. JSON charges decoded strings, object keys, child paths, array paths, and leaf fields at `json_text.rs:124-150,165-202`. Fact assembly has its own preallocation accounting, initially `game_files.rs:506-519,543-556`, reread at lines 512-525 and 549-562. Parser or fact-budget failure discards all structured facts and retains the complete document content with an explicit parse status. VPK shared-path amplification has both directory accounting and a whole-archive virtual-path preflight before resource emission. The remaining payload-allocation defect is finding 1 above.
2. Original canonical-resource mismatch: addressed by the inspected explicit layout mapping, initially `game_files.rs:451-473`, reread beginning at line 457. Only loose files with an expressly selected `gametracking-pak01-dir` or `gametracking-citadel-pak01-dir` layout map the exact `game/citadel/pak01_dir/` and `game/core/pak01_dir/` prefixes to their resource roots. Default/resource_paths layouts, VPK resources, and unrelated `other/pak01_dir/` prefixes are not rewritten. A loose Citadel or Core resource therefore matches the virtual path produced from the corresponding VPK in the same game-root layout. `source_locator`, title, and `original_relative_path` retain the presentation path; canonical metadata and IDs use the resource path.
3. Original JSON duplicate-key loss: addressed. `json_text.rs:135-145` detects duplicates using decoded keys, including alternate Unicode-escaped spellings of the same key. Input `{"damage":12,"damage":13}` returns `duplicate_json_keys_preserved_as_text`. The document writer preserves the whole raw text, while extraction emits no partial facts. Nested duplicates also abort the complete structured parse; equal keys in different objects remain valid. KV and KV3 retain repeated properties through per-scope occurrence numbers instead of applying JSON's fallback.
4. Original ancestor replacement race: addressed in the inspected traversal. `anchored.rs:56-71` opens each root component relative to a held directory handle. Directory descent uses another held handle; loose files and VPK siblings are opened through that handle's `/proc/self/fd/<fd>` location. Leaf opens use Linux O_NOFOLLOW, O_NONBLOCK, and regular-file checks; directory opens additionally require O_DIRECTORY. Embedded VPK reads and hashing use the held VPK file. Replacing a visible ancestor with a symlink does not redirect these held handles. No race test was run, and this is not a guarantee of an immutable snapshot against in-place file writes.

## 3. Independently checked numeric preservation

The original GameTracking `abilities.vdata` was read through a targeted search. The exact lexeme `340282346638528859811704183484516925440.0` occurs five times, including line 143695 as `m_flMaxLagCompensation`. Its local source path is:

`/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/b/gametracking-4c6431ccdb816d2911bbbaa4335169cc34140386/files/game/citadel/pak01_dir/scripts/abilities.vdata`

Original-file SHA-256: `6719482b2a06a79b2965a3cba61cb803bbf80fb03615a940fa0754a8da3883ca`, unchanged between the two source-evidence hash checks.

Independent code trace for `{ value = 340282346638528859811704183484516925440.0 }` in KV3 and `{"value":340282346638528859811704183484516925440.0}` in JSON:

* `json_text.rs:8-45` validates numeric grammar using bytes, without float conversion.
* The decimal point bypasses both integer conversions at lines 49-55. Lines 57-60 return the exact original text as a JSON string and mark it `source_numeric_lexeme`.
* KV3 and JSON retain the same source lexeme in their Leaf values. Fact assembly preserves both `value` and `qualifiers.source_lexeme`, plus `numeric_representation`.

The same reasoning covers `18446744073709551616`, `0.12345678901234567890123456789`, `1e999`, and `-0`. In-range i64/u64 integers use exact numeric values and still retain their original lexemes. Ordinary source strings remain distinguishable through their source lexemes and lack of a numeric marker. KV values remain source strings with escape bytes preserved. No float parse was found in these production extraction paths.

This verifies the conversion rule and the presence of the real source value. It does not establish that the entire abilities file parses successfully or that a regenerated JSONL artifact contains those five facts. That requires the pending real-data run.

## 4. IDs, provenance, errors, and additional unremarkable paths

* Document IDs are `game:<declared app_id>:<canonical normalized resource path>` and do not contain the revision. The app ID is supplied, never guessed. Unknown versions are content-hash based; Git source revisions are kept separate from build/manifest fields.
* Distinct normalized resource paths produce distinct document IDs. KV and KV3 escape `~` and `/`, add occurrence indices, and preserve array order. JSON uses escaped object keys and array indices. These inspected fact constructors produce distinct IDs for separate accepted leaves. No random or revision-dependent fact ID is used.
* The exporter does not enforce global uniqueness if the same canonical resource is present twice, for example in two representations or containers. Such entries identify the same resource by design. Output-wide duplicate/conflict handling still needs the actual source validator and C's importer; no output-wide uniqueness proof is claimed here.
* Original decoded source text is not shortened on parser fallback. UTF-16 decoding is strict. Original-file hashes and decoded-content hashes are separate. Units remain unknown, type flags and conditions remain source evidence, and `gameplay_execution_verified` stays false. Binary asset names do not create invented mechanics. Redistribution remains false.
* A directory-tree or virtual-path budget error records the package gap before that package emits resources. Per-resource payload bounds, missing sibling files, CRC failures, input-size violations, and decoding failures have explicit inventory reasons/gaps. Earlier successful resources may remain in the output, with later failures visible in the inventory. Fatal I/O/output errors propagate as Err. The capacity failure in finding 1 can bypass this ordinary gap route.
* The harness validator checks loose original files, hashes, exact decoded content, duplicate document/fact IDs, inventory counts, and declared source revision. It resolves each locator as a regular file below the source root. It is a validator for the two pinned loose sources, not proof of a real VPK download or payload extraction. Its numeric check applies when a fact value is numeric; lexical string preservation is established by the inspected extractor rule, not by a completed validator run.
* External service paths inspected or invoked: 0. Steam license/access/download operations remain exclusively with D.

## 5. Source hash binding and instability

Production sources were hashed at 04:38:39 UTC, 04:46:08 UTC, and finally 04:49:23 UTC. All production and harness hashes at the final check matched the second check. The final binding therefore includes the explicit child-module path attributes, with no further observed source changes. SHA-256 values below are relative to `rust/crates/dbrain-sources/src/`:

| File | Initial hash | Second hash |
| --- | --- | --- |
| game_files.rs | 4c740bf2c5ca2601ecfaec9d01cca6a8fbf17390ebe564f20006e2f2e381573c | 8ca3b514ac2fd4fc0301193114f381560db1322de851ffcb2f7c842f23e45fac |
| game_files/anchored.rs | e6bd4e5221d8806e85f043dc90a116db87848f38a45bad57dd37047858e09474 | unchanged |
| game_files/budget.rs | d18572db271be846a57462fd0232054c0a4c8e9d23d24fb89047f0b75c20ea66 | unchanged |
| game_files/json_text.rs | 83ed6b1fea173e1a630563d56f271111da51dfe8c5715db8a4c700becf2a8d5b | unchanged |
| game_files/kv.rs | 8fa75c5a183d89c6fda516e87c418ca3eb33c242f0c9ca8d8bae8398eeca9aaf | unchanged |
| game_files/kv3.rs | 33f8535652b6b48d1c3a3510ef0cc0deee3ef453eefd8034c111df9b08efbf82 | unchanged |
| game_files/vpk.rs | bff4f705b6ae326fe5a9f5c58afe7f2c73623692eb192900541fea364c656ed7 | unchanged |

The targeted reread showed explicit `#[path = "game_files/..."]` attributes before the six child modules and checked the relevant caller, provenance, and fact-construction paths again. This does not convert an unstable-tree review into approval. The unchanged VPK hash independently binds the open finding.

Harness hashes were first captured at 04:42:22 UTC and were unchanged at 04:46:08 UTC:

* `pruefharness-b/Cargo.toml`: `e264280262af7cd0f89d29d76c695ef2a2e20eff377a8e0b0fb9488fecf6ee62`
* `pruefharness-b/src/main.rs`: `36ffe9a5f0be2fe5127870ec4b19498ea195168c3ef61181258a0b593e2ea6e7`
* `pruefharness-b/src/bin/validate-jsonl.rs`: `eb464a39bfd5e2593289b137f7806675141cbb05bc44d6b28a54afb255b1d6a6`

## 6. Separate verification blockers

Compiler/test evidence: none generated or verified by this reviewer. The coordinator reported that the first actual standalone compile failed with E0583 and exit 1 because bare child-module declarations resolved under src/. The explicit child-module path attributes are present in the final hash-bound state. The coordinator reports a sequential rerun of the existing wrapper, not a concurrent second check. Neither that report nor the targeted code reread proves compiler success; the compile log and rerun were not independently inspected here.

Real-data evidence: no regenerated extraction/validation of either pinned source was run here. The source lexeme/hash observation above is narrower than a complete extraction proof. No real VPK package or Steam download was verified.

Integration/merge evidence: none. Shared manifests and module registration belong to C. The open VPK allocation-order finding needs correction, and a stable reviewed source set is required before a later approval. C retains import, integrated gate, merge, deploy, and live verification ownership.
