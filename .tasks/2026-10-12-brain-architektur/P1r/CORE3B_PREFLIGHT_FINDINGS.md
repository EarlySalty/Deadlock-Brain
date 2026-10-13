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

Fresh test-only fixer `a9eb6b272d6e7df75`, workflow `wf_aa85b40e-f33`, task `wxj4had8t`, owns only migrations_tests.rs from74bcb9ff. Extract focused <=50-line private helpers preserving all assertions, SQL/bindings, mutation/restoration cases and physical nullable/publication/admin probes. No production edits, new tests or broader suite; fresh scoped review follows. Parent has not integrated74bcb9ff yet.

The orchestrator's separate duplicate-JSON ingest request from P9 ef62ecdf is owned by `aa5265444d730dd69`, workflow `wf_3d709ae4-e23`, task `w1k8wvbs8`, in disjoint receipt-parser/storage-test files. Details and combined acceptance are in TODO.md; P2 slot mapping remains outside P1r.

Core3b remains unpublished. Own local gate count remains one BLOCK and zero ALLOW. No main merge, deployment, production ingestion, timer activation or live answer acceptance is claimed.
