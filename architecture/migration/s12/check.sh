#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")"
# Scope formatting to this package: --all would also traverse the shared runtime
# workspace through brain-contracts and inspect unrelated historical files.
cargo fmt --package dbrain-s12-wiki-probe -- --check
cargo clippy --all-targets --locked --offline -- -D warnings
cargo test --all-targets --locked --offline
cargo build --release --locked --offline
./target/release/dbrain-s12-wiki-probe analyze fixtures/pilot.capture.json >/dev/null
./target/release/dbrain-s12-wiki-probe delta fixtures/pilot.capture.json fixtures/pilot.capture.json >/dev/null
status=0
./target/release/dbrain-s12-wiki-probe analyze fixtures/pilot.capture.json --require-production-ready >/dev/null || status=$?
if [[ "$status" -ne 3 ]]; then
  printf 'Expected explicit blocked readiness (3), got %s\n' "$status" >&2
  exit 1
fi
printf 'S12 offline checks passed; production readiness remains blocked.\n'
