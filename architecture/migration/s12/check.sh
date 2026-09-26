#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")"
ROOT="$(cd ../../.. && pwd)"
MANIFEST="$ROOT/rust/Cargo.toml"
cargo fmt --manifest-path "$MANIFEST" --package dbrain-s12-wiki-probe -- --check
cargo clippy --manifest-path "$MANIFEST" -p dbrain-s12-wiki-probe --all-targets --locked --offline -- -D warnings
cargo test --manifest-path "$MANIFEST" -p dbrain-s12-wiki-probe --all-targets --locked --offline
cargo build --manifest-path "$MANIFEST" -p dbrain-s12-wiki-probe --release --locked --offline
# cargo run respects CARGO_TARGET_DIR instead of guessing the artifact directory.
cargo run --quiet --manifest-path "$MANIFEST" -p dbrain-s12-wiki-probe --release --locked --offline -- analyze fixtures/pilot.capture.json >/dev/null
cargo run --quiet --manifest-path "$MANIFEST" -p dbrain-s12-wiki-probe --release --locked --offline -- delta fixtures/pilot.capture.json fixtures/pilot.capture.json >/dev/null
status=0
cargo run --quiet --manifest-path "$MANIFEST" -p dbrain-s12-wiki-probe --release --locked --offline -- analyze fixtures/pilot.capture.json --require-production-ready >/dev/null || status=$?
if [[ "$status" -ne 3 ]]; then
  printf 'Expected explicit blocked readiness (3), got %s\n' "$status" >&2
  exit 1
fi
printf 'S12 offline checks passed; production readiness remains blocked.\n'
