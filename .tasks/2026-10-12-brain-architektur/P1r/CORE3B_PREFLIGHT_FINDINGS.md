# Core3b remaining preflight findings

Verified source: `3c2f2e37f4cdf6c83ea4fe2f09a157a1813b581f`, clean V10 worker branch `worktree-wf_335cc3d1-8d4-1`.

## Independent evidence recovered after reviewer termination

Database reviewer `a7ebe68ae1c7bd329` executed 36 tool calls and produced private-PG artifacts before its final provider request was rejected with API400. No final reviewer verdict was returned. Do not retry or change model/provider to bypass that refusal. Existing local evidence remains valid input for independent adjudication.

Evidence: `/tmp/brain-core3b-db-review-EU0AhmLy/probe.log`, stronger rerun `/tmp/brain-core3b-db-review-EU0AhmLy/recheck.log`, reproduction source `probe.py`. The rerun used actual PostgreSQL16 and `session_user=current_user=brain_ingest`, nonsuperuser and without BYPASSRLS. Normal V8/V9/V10 controls passed: 23 rejected published writes, 18 lawful failed-run transactions, five genuine conflict paths, 18 observed publication-lock waits across six tables, V9 4096/4097 limits, historical raw reuse, new references on old entities and intentional privileged receipt corruption.

Both owned clusters are stopped, confirmed independently by `pg_ctl status` exit3 at `/tmp/brain-core3b-db-review-EU0AhmLy/data` and `/tmp/brain-core3b-db-review-EU0AhmLy/recheck/data`. Data directories are retained with the evidence. No production operations occurred.

## Confirmed remaining gaps

1. **V8 function-body drift is not detected.** Replacing `canon.guard_fact_integrity()` with a body returning NEW retained every currently audited property. All seven exact current preflight SQL checks and frozen checksums passed. Actual ingest LOGIN then inserted a visible published fact. Compare the exact signed V8 function body and complete expected function/trigger metadata.
2. **V7 run-trigger predicate drift is not detected.** Replacing the inherited publication trigger with `WHEN(false)` retained its current audited properties. Preflight and checksums passed. Actual ingest LOGIN withdrew a succeeded run, appended all canonical row kinds and republished. Require the exact unconditional publication trigger, status-column/event binding and signed V7 function body.
3. **Reference-table column UPDATE grants are not detected.** Individually granting `UPDATE(receipt_id)` on entity, entity_identity, alias and relation passed all current preflight checks. Actual ingest LOGIN changed published references, withdrawing serving rows. Audit forbidden mutation privileges, including column-level grants, across the canonical tables. Keep legitimate INSERT/SELECT, the scoped V7 function and intentional administrator-only receipt UPDATE evidence probes.

These are schema-drift/preflight defects, not failures of the correctly installed V10 normal INSERT guards. Frozen V1/V7/V8/V9/V10 SQL must not be edited. Fix the Rust preflight and targeted existing metadata-regression tests. No new public API, broader grants or blanket admin receipt UPDATE prohibition.

## Ordered ownership

Preflight decomposition writer `abcb0fe3fd50aeaab` and reviewer `a2f955c15495a91ea` in `wf_42f07e77-403` completed clean `e2701a5c7efe384d4468c8dd7f36dbda1ff60a37`. Eleven tests and matching checks passed independently; all 15 SQL literals remained byte-identical. Parent integrated V10 plus the refactor as `25949841db7e290296f62e22051e48edaeb358b8` with source parity verified.

Signed-body/ACL fixer `a9400505b71534b08`, workflow `wf_8248d105-58f`, task `w3yaaqoqx`, completed clean `c4f0372ec6b96526a0ecccf1fe47642ffe91cbb9`, only migrations.rs/migrations_tests.rs. All three original gaps close. Author and fresh reviewer `aaf7dc85e4e1dec07` each ran20 actual tests/check/strict Clippy/fmt, including139 intentional V7/V8/V10/ACL fault configurations. Frozen SQL unchanged; no parent integration/publication yet.

## Execution-attribute follow-up

Fresh independent PG16 proof `/tmp/brain-review-c4f037-proof.log` confirms two additional false accepts against c4:

1. V7 `canon.write_run_fact(... ) STRICT` passes all six exact audit predicates but silently skips normal calls containing nullable fact fields. Successful call leaves2facts instead of3; the same arguments write3 after restoring CALLED ON NULL INPUT.
2. V8 `canon.guard_fact_integrity() STABLE` passes preflight but lawful INSERT fails0A000 because its SELECT FOR UPDATE requires volatile execution.

All15 other independent metadata negatives reject, legitimate old raw/new refs and administrator-only receipt corruption remain legal, and the reviewer confirms its own private PG stopped with no lasting fixture changes. Review BLOCK is scoped code review, not a second parent gate.

Execution-attribute fixer `a0e16c2aa7cc16ad9`, workflow `wf_6cd1b9e2-5de`, task `wy4l9cn8n`, completed clean `74bcb9ff3c4442f8bdd093646b77fb245aee6160`. All six V7/V8/V9/V10 functions now require their non-STRICT/VOLATILE/PARALLEL UNSAFE/non-LEAKPROOF/support-hook0 defaults. Author and fresh reviewer `aaeea59bd6012c8e9` each passed21 actual tests/check/strict Clippy/fmt, including42 rejected/restored mutations and three genuinely written/published nullable facts. Original defects independently reproduced before the fix; all normal guards/caps/races, sparse registries, historical references and admin receipt allowance pass. No runtime or safety issue remains; the only scoped BLOCK is the new145-line test. Production helpers remain small, frozen SQL unchanged,1499 job lines preserved, own private probe stopped. Exact logs `/tmp/brain-review-74bcb9ff*`.

Test-only fixer `a9eb6b272d6e7df75`, workflow `wf_aa85b40e-f33`, task `wxj4had8t`, completed clean `532721bd1f58cac09e96d91fab1accb07a33fb11`. Fresh reviewer `a4fb206b3b0394630` APPROVES: every changed function<=47lines, mechanically expanded semantics/SQL/bindings/assertions/order/lifecycle unchanged,10 actual migration tests/check/strict Clippy/absolute fmt each passed independently. All42 mutations, three nullable facts and four admin changes/rollback remain verified. Proof `/tmp/brain-test-decomposition-review-proof.log`. Parent integrated these signed guard/attribute/test fixes as `07392a8e8e21357a49169210feb3e0915d3cf40c` with exact rust/scripts/ops parity verified by exit0 against532721bd. All confirmed preflight runtime/style findings are closed; matching final combined binaries/CLI still await JSON acceptance.

The orchestrator's separate duplicate-JSON ingest request from P9 ef62ecdf is owned by `aa5265444d730dd69`, workflow `wf_3d709ae4-e23`, task `w1k8wvbs8`, in disjoint receipt-parser/storage-test files. Details and combined acceptance are in TODO.md; P2 slot mapping remains outside P1r.

## Confirmed recognized-JSON follow-up

The original independent JSON reviewer `a02d36ecf1b6c962c` completed its resumed same review on f4 and returned BLOCK. `{"x":1,"x":2,"\ud800":0}` passes RawValue recognition, but decoding the surrogate key aborts indexing and `index.scan().is_err()` returns `Ok(None)`. Raw-bytes, empty-set and byte-range paths then publish bad data under actual PG16 brain_ingest. Common runner first3/1/0 and repeat0/1/3 retained the invalid fact and empty success marker. Proof `/tmp/wf_6cd1-json-f4b2-fallback-verified.log`, source `/tmp/wf_6cd1-json-probes/fallback.rs`. Ordinary duplicates and legal controls pass; near-cap stress0.740s/87452KiB. Changed functions59/73/185lines also violate the rubric. Review completed29 passing executions with matching check/Clippy/fmt; passing controls do not waive these defects.

Fresh fixer `aff249cd34fe1684e`, workflow `wf_27cb0c46-c3f`, task `wwwury9w1`, owns only receipt-parser/Core2 tests in `/home/nathanael/repos/Deadlock-Brain/.claude/worktrees/wf_27cb0c46-c3f-1`, branch `fix-json-index-closed-wf27`, basef4. Preserve genuine initial non-JSON fallback, fail closed after recognized JSON cannot be indexed, retain raw/per-item reason and healthy continuation, and extract focused <=50-line helpers without losing assertions. Fresh independent review follows. Signed metadata source is off-limits. No final handover yet.

Two additional reviews were incomplete solely from setup:599 fmt omitted --all; c7 supplied the Hero cache root instead of /kv3. Parent correct absolute formatter passes; parent full job12 and corrected storage21 tests pass with0failed0ignored. Evidence `/tmp/p1r-json-f4-parent-{job,storage-corrected}.log`. Earlier parent storage omission failed1cache test; the corrected command includes explicit env.BRAIN_CORE2_KV3_REPOSITORY.value for `/tmp/brain-core2-cache-wf-7f1dca25-SbrUuw/kv3` and force=true, with --include-ignored. This is not JSON fix acceptance.

Core3b remains unpublished. Own local gate count remains one BLOCK and zero ALLOW. No main merge, deployment, production ingestion, timer activation or live answer acceptance is claimed.
