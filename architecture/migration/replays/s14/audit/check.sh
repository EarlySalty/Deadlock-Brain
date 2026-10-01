#!/usr/bin/env bash
set -euo pipefail
here="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
cargo_bin="${CARGO:-cargo}"
manifest="$here/Cargo.toml"
"$cargo_bin" fmt --manifest-path "$manifest" -- --check
"$cargo_bin" clippy --manifest-path "$manifest" --all-targets --locked --offline -- -D warnings
"$cargo_bin" test --manifest-path "$manifest" --locked --offline
"$cargo_bin" build --manifest-path "$manifest" --release --locked --offline
# The current real corpus is intentionally not approved/populated. A blocked
# metadata report is required here, never treated as a replay integration pass.
set +e
"$cargo_bin" run --manifest-path "$manifest" --release --locked --offline -- \
  "$here/../CORPUS.tsv" "$here/../CAPABILITIES.tsv"
status=$?
set -e
if [[ "$status" != 2 ]]; then
  printf 'Expected blocked corpus (exit 2); got %s\n' "$status" >&2
  exit 1
fi
printf 'S14 preparation checks passed; real replay integration remains blocked.\n'
