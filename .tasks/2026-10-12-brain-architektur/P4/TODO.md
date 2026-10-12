# P4 continuation point

Opus takeover 2026-10-12 20:00 CEST after Codex thread ff1b2d74 hit its weekly limit.

## Done and verified on branch feat/brain-p4-patches-20261012-v2

- 4fcf6bad: pure extractor `brain-job/src/patch_extract.rs` plus contract tests, 796 lines. patch_extract_contract 9/9 passed, including the real Store poison run on scratch Postgres (value_too_large and key_collision rejected, run succeeded, following run free).
- 68cfc1cc: serve-only projection SQL (`serve.p4_patch_catalog`, `serve.p4_patch_change`, `serve.p4_patch_receipt`) and canonical patch_history tool. No canon/raw reads. patch_history_canonical 5/5, brain-serve lib pg_patch 9/9, pg_patch_tools scratch-PG integration 2/2.
- 1b7e586b: extractor registered in registry.rs; clippy -D warnings clean for brain-job.
- Sources: wf_89554428-49f-1 (extractor) and wf_89554428-49f-2 (serving). Older drafts in wf_4be1e21f-3e7-1/-2, wf_99e78d62-197-1/-2 and p4-pure-extract-20261012 superseded, left untouched.

## Open

1. Core 2 SHA: merge when forwarded, then add the patchnotes multi-object source loader with one sources.rs line.
2. Migration v4 registration: `migrations.rs` enforces contiguous versions, so v4 can only be appended after P3=2 and P2=3 are on main. Rename file to the `-v1` naming at integration.
3. Acceptance gaps to check: index on (entity_id, patch_id) and under 5 ms, optional `patch` parameter on get_hero, ingest peak memory and second-run zero raw objects.
4. Fresh full-diff reviewer against REDESIGN-INGEST.md and CONTRACT_INGEST_V2.md, then gate after P1r live.
