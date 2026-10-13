# P4 continuation point

Sol resumed 2026-10-13. Current published checkpoint: `158cc695` on `feat/brain-p4-patches-20261012-v2`.

## Resume progress 2026-10-13

- `64fc6527` merged published P1r Core 2 and Core 2b through `9f75588d`; sparse migration registry and streaming sources are available. Dependency/module merge conflicts preserved both sides. Formatter passed. The standalone lib check was stopped before acquiring a slot; compiler-backed checks will run on the integrated loader/migration state.
- `158cc695` added the package-owned five-minute `brain-job@deadlock_patchnotes_db.timer`; systemd calendar and unit verification passed. No timer/service was activated.
- Live feed catalog checked read-only through Infisical: 132 posts, 132 dated, 746135 raw-content bytes total, largest 64006 bytes, three posts containing Baba. Columns verified: id bigint; title/url/raw_content/translated_content text; posted_at timestamptz. No credentials or raw DB contents printed.
- Active isolated workers: source loader `agent-ae1a42b9495225b9d`; additive migration `agent-a852aea1944696bb6`. Both worktrees are under `/home/nathanael/repos/Deadlock-Brain/.claude/worktrees/`. Independent source acceptance reviewer `ad9ea46b6c54d7f38` is read-only.
- Build issue: ionos `/dev/vda1` has zero free bytes and fails Git mirror config.lock creation. cargo-slot falls back to local slots, currently occupied. An earlier two-minute background check was killed while waiting, not a test failure. No foreign artifacts or processes were touched.
- P1r hero tool is absent on the published Core 2 checkpoint. The schema worker confirmed that an ordinary `(hero entity_id, patch_id)` index is impossible on current stored rows: `canon.fact.entity_id` is the patch, while hero binding is computed from current published aliases by a lateral aggregate. No shadow materialization/refresh writer was added. Pending orchestrator decision: accept equivalent indexed patch-fact/alias access with measured latency, or authorize a new persisted projection/refresh contract. Historical hero state additionally needs evidenced patch assignments and the owning P1r interface. Neither acceptance point is claimed complete.

## Previously done and verified


- 4fcf6bad: pure extractor `brain-job/src/patch_extract.rs` plus contract tests, 796 lines. patch_extract_contract 9/9 passed, including the real Store poison run on scratch Postgres (value_too_large and key_collision rejected, run succeeded, following run free).
- 68cfc1cc: serve-only projection SQL (`serve.p4_patch_catalog`, `serve.p4_patch_change`, `serve.p4_patch_receipt`) and canonical patch_history tool. No canon/raw reads. patch_history_canonical 5/5, brain-serve lib pg_patch 9/9, pg_patch_tools scratch-PG integration 2/2.
- 1b7e586b: extractor registered in registry.rs; clippy -D warnings clean for brain-job.
- Sources: wf_89554428-49f-1 (extractor) and wf_89554428-49f-2 (serving). Older drafts in wf_4be1e21f-3e7-1/-2, wf_99e78d62-197-1/-2 and p4-pure-extract-20261012 superseded, left untouched.

## Open

1. Integrate the streamed patchnotes loader from the isolated source worker, then verify real-core idempotence and peak memory.
2. Integrate the additive v4 migration registration and nullable patch-boundary columns from the schema worker; the sparse core registry now permits v4 independently of v2/v3.
3. Resolve remaining acceptance: indexed query under 5 ms and optional `patch` on get_hero when its owning P1r interface is published. No invented patch-to-commit mapping.
4. Finish independent full-diff review against REDESIGN-INGEST.md and CONTRACT_INGEST_V2.md, fix concrete findings, then gate after P1r live. Only the responsible integrator merges/deploys.

