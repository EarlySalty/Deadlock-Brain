# Runner fix after P6 (fix/brain-serve-runner-20261023)

Base: integrate/brain-wave2-20261014 @ bfee3a6d. Resolves the slice-10 gate BLOCK (35d5c529..14fc488c).

## Done

- `scripts/test_brain_serve.sh` runs the existing `process_e2e` test `tools_only_process_needs_no_corpus_release_and_refuses_unknown_heroes` by exact name without `--ignored`. Every cargo test step goes through `run_test`, which fails when the summed `test result: ok. N passed` count is 0 and prints `runner_result <step> passed=N` at the end.
- Other runners checked: every named test in `test_brain_core_postgres.sh`, `test_brain_storage_upgrade.sh`, `test_wiki_runtime.sh` and `run_local_pilot.sh` still exists and is `#[ignore]` as invoked. No stale names.
- `ApiService::new` uses `config.release.id` again (the value before P6). `brain-candidate-activate` rewrites `release` pins in the serve config, so answers must carry that id; a hard-coded value would report a release that `/readyz` does not report.
- `process_e2e` asserts again at process level: v99 schema refusal and empty DB without migration (exit 1, `core_schema_incompatible`, no bind, DB unchanged), warm schema marker gives 503 and recovers, DB outage gives 503 with healthz 200 and recovers, scope 403 and unknown token 401 without provider calls, in-flight drain on SIGTERM, credential-free logs, and `knowledge_release` equal to the configured release.

## Proof

`/home/nathanael/.local/state/brain-wave2-proof-20261014/runner-fix/`
