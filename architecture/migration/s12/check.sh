#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "")"
cargo fmt --all -- --check
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
