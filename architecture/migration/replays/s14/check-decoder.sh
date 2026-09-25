#!/usr/bin/env bash
set -euo pipefail
here="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd -- "$here/../../../.." && pwd)"
cargo_bin="${CARGO:-cargo}"
manifest="$root/rust/Cargo.toml"
export SQLX_OFFLINE=true
"$cargo_bin" fmt --manifest-path "$manifest" -p dbrain-replay -- --check
"${RUSTFMT:-rustfmt}" --edition 2021 --check "$root/rust/crates/deadlock-brain-core/src/replay.rs"
"$cargo_bin" clippy --manifest-path "$manifest" -p dbrain-replay --all-targets --locked --offline -j2 -- -D warnings
"$cargo_bin" test --manifest-path "$manifest" -p dbrain-replay --locked --offline -j2
"$cargo_bin" test --manifest-path "$manifest" -p dbrain-replay --release --locked --offline -j2
# Retain the original independent inventory gate. Its empty real corpus MUST stay blocked.
bash "$here/audit/check.sh"
# This project contains authored generator code, never a public recorded replay corpus.
tracked_replays=$(git -C "$root" ls-files -- '*.dem' '*.dem.bz2' '*.dem.gz' '*.dem.zst')
if [[ -n "$tracked_replays" ]]; then
  printf 'Recorded replay artifacts must not be committed.\n' >&2
  exit 1
fi
printf 'Synthetic decoder verification passed. Real replay validation is NOT established.\n'
