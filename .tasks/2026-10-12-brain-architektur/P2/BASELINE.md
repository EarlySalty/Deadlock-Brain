# P2 pre-change baseline

Measured on 2026-10-12, before integrating any P2 implementation. The parent incorporated P1 checkpoint `3a0740be` at merge commit `07afdf15` while the compiler job was queued, so this is not a baseline of the original `a808f5a4` checkout.

## Command

```bash
CARGO_INCREMENTAL=0 \
RUSTFLAGS=--remap-path-prefix=/home/nathanael=/brain-operator \
RUSTC_WRAPPER=/home/nathanael/.cargo/bin/sccache \
SCCACHE_DIR=/home/nathanael/.cache/sccache \
cargo-slot test \
  --manifest-path /home/nathanael/.worktrees/Deadlock-Brain-p2-items-abilities/rust/Cargo.toml \
  --locked --release --jobs 3 \
  -p brain-contracts \
  -p brain-storage \
  -p brain-feeds \
  -p brain-kernel \
  -p brain-serve \
  > /tmp/brain-p2-20261012-proofs/baseline-tests.log 2>&1
```

## Result

TESTNACHWEIS[TW-1]: 453 passed, 19 ignored | Baseline: 0 rot

Exit code 0. Across 39 test-result blocks: 453 passed, 0 failed, 19 ignored, 0 measured, 0 filtered out. No P2 production-path or live evidence is inferred from this baseline. `brain-job` was not included.

The initial remote attempt failed to load the external `dl-token-secrets` path required by `brain-legacy-import`. `cargo-slot` fell back to the local build and waited for an available compiler slot. The local release test build completed in 13m 16s and the suites ran. The remote manifest diagnostic is an environment failure, not a failed source test. No shared builder, lock, cache server or foreign process was interrupted.

Ignored tests remain explicitly unverified by this default run. The existing `brain-storage/tests/gamefacts_schema.rs` contract did run against its private PostgreSQL 16 cluster, with no environment-based skip, and passed in 9.48s. Relevant P2 PostgreSQL integration and authorization/bounds contracts still require separate actual execution with their documented fixtures or isolated test database; do not count skipped test bodies as live proof.
