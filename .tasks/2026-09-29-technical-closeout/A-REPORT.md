status: erledigt
Datum: 2026-09-29

# Package A handoff

Base: `305df2d36ec7b5d0513d6c0769051b41538d6a1b` (`origin/migration/rust-integration`). Implementation commits: `a03af9d6d95ef4a3284e9a0fe8e16d2179506c5e`, `5cd9970996889e7f16d3a32682566d80dc8aa071`. Branch: `integration/pre-g5-finalize-20260929`. No merge, deploy, consumer enablement, production data request, or real demo download.

## Delivered and observed behavior

- `rust/crates/brain-feeds/src/deadlock_match.rs` binds canonical match and account IDs to the exact metadata request, validates the pinned API response, and emits a private `SourceRecordV2` with `account:<id>` scope, raw-body hash, schema/version, locator, observation time, parser revision, and source origin. The body hash participates in content revision. The adapter emits checkpointed updates and tombstones, supports an idempotent retry, and constructs a new immutable `CorpusRelease` from store pins. Demo evidence requires both IDs in every raw row; missing or mismatched identity is rejected, not silently supplied from the request.
- `rust/crates/brain-feeds/src/bin/brain-match-ingest.rs` is an `ingest|revoke --config <private-json>` runner. It uses a bounded request against the canonical Deadlock API for ingest, checks an optional configured raw hash against the received bytes, and commits through the ordinary leased `DocumentStorePort`. Revocation tombstones match and any present demo source, publishes a release, and reads it back. Config must be a private regular file; PostgreSQL is a Unix socket with no password environment variables. No new ClickHouse or central DSN.
- `rust/crates/dbrain-sources/src/analytics_runtime.rs` uses the existing bounded HTTP client for Meta and Population on the canonical API origin, with fixture-only loopback, exact query and origin validation, two attempts, timeout, 1 MiB, 256 raw rows, 31-day window, hero/item filtering, pinned OpenAPI response schema, raw hash, observed time, and locator. `brain-serve` optionally exposes authenticated `POST /v1/analytics/observation` with `analytics.internal`, four concurrent lookup slots, request bounds, a configured release patch/window, and no analytics database fallback. Existing configs keep analytics disabled until configured.
- The fixture PostgreSQL run created a peer-authenticated, TCP-disabled disposable cluster and exercised commit, release readback through `LocalPgReader`, account-scoped `ReleaseRetriever` evidence, denied provider egress, unchanged retry, CLI revoke, historical-release ACL invalidation, and repeat revoke. The cluster was stopped and removed after the test.

## Verification on the implementation head

Working directory for Cargo commands: `rust/`. All Cargo invocations used `/home/nathanael/.cargo/bin/cargo +stable`, `--locked --offline`, and `CARGO_BUILD_JOBS=2` except formatting.

| Command | Exit | Evidence |
| --- | ---: | --- |
| `cargo +stable fmt --all -- --check` | 0 | `A-fmt-post-review.log` |
| `CARGO_BUILD_JOBS=2 cargo +stable check --workspace --all-targets --locked --offline` | 0 | `A-check-post-review.log` |
| `CARGO_BUILD_JOBS=2 cargo +stable clippy -p brain-feeds -p dbrain-sources -p brain-serve --all-targets --locked --offline -- -D warnings` | 0 | `A-clippy-post-review.log` |
| `CARGO_BUILD_JOBS=2 cargo +stable test --locked --offline -p brain-feeds -p dbrain-sources -p brain-serve` | 0 | `A-tests-post-review.log`: 159 passed, 0 failed, 13 ignored, 0 filtered across 18 groups |
| `CARGO_BUILD_JOBS=2 cargo +stable test --locked --offline -p brain-feeds --bin brain-match-ingest` | 0 | `A-cli-tests.log`: 1 passed, 0 failed, 0 ignored; also included in the later targeted run |
| `BRAIN_MATCH_TEST_PG_SOCKET=<disposable-socket> BRAIN_MATCH_TEST_PG_PORT=55479 CARGO_BUILD_JOBS=2 cargo +stable test --locked --offline -p brain-feeds --test match_store postgres_match_commit_release_readback_replay_and_revoke -- --ignored --exact --nocapture` | 0 | `A-pg-test-final-verified.log`: 1 passed, 0 failed, 0 ignored, 1 filtered |
| `python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/brain-pre-g5-finalize-20260929 --base origin/migration/rust-integration --head HEAD` | 1 then 0 | `A-gate-review.log` found two blockers and one nit; fixes in `5cd9970`; `A-gate-review-2.log` says ALLOW. This self-run gate is not the independent orchestrator review. |

The PostgreSQL setup used `initdb --auth-local=peer --auth-host=reject`, `pg_ctl` with `listen_addresses=''` and `-k <disposable-socket>`, and `createdb` over that socket. No password DSN was read or passed. The first attempts at this ignored test failed: the synchronous reader was called from a Tokio task and an unauthorized retrieval assertion expected an empty result instead of a permission error. The test was corrected, rerun with explicit Cargo exit-code checking, and passed on the final implementation head. Earlier success messages produced by a shell wrapper were not test evidence.

TESTNACHWEIS[TW-1]: 159 passed, 13 ignored | Baseline: nicht erhoben

## Boundaries and remaining review

- The checked OpenAPI contract has no patch parameter or patch field on either analytics endpoint. A configured patch selects the `brain-serve` release context and approved time window but does **not** prove that upstream rows belong to that game patch. Responses explicitly report `patch_membership: "unverified"` and never label the data with a verified patch. Patch-specific consumption must remain blocked until upstream attestation or an independently reviewed mapping exists.
- The full live OpenAPI document hash observed read-only on 2026-09-29 differed from the pinned fixture due to moving default timestamps on several analytics endpoints. The consumed parameter names, required fields, and relevant response schemas matched the fixture. No production match or player data was queried. A future consumed-schema change fails closed.
- Match ingest HTTP against a live player match was not run. Fixtures validate the adapter; the isolated PostgreSQL run exercises the real store and revoke CLI. The analytics loopback fixture exercises real bounded HTTP; it is not a production API call. The authenticated route has config and authorization tests, not a live end-to-end service call.
- Independent review belongs to the orchestrator before integration. No main merge or deploy is authorized for this package.
