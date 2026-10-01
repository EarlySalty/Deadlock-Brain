#!/usr/bin/env bash
# Reproduce the integrated client / PR27 quality / cutover suites without runtime credentials.
set -uo pipefail
ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)"
MODE="${1:-client}"
case "$MODE" in client|quality|cutover) ;; *) echo 'Usage: check_consumer_offline.sh client|quality|cutover' >&2; exit 64 ;; esac
LOGS="$ROOT/.consumer-ci-reports/$MODE"
mkdir -p "$LOGS/test-home"
printf 'check\texit_code\n' > "$LOGS/results.tsv"
CARGO_CACHE="${CARGO_HOME:-$HOME/.cargo}"
RUSTUP_CACHE="${RUSTUP_HOME:-$HOME/.rustup}"
FAILED=0
run() {
  local name="$1"; shift
  local result=0
  (cd "$ROOT" && env -i PATH="$PATH" HOME="$LOGS/test-home" CARGO_HOME="$CARGO_CACHE" RUSTUP_HOME="$RUSTUP_CACHE" \
    CARGO_BUILD_JOBS=2 CARGO_NET_OFFLINE=true SQLX_OFFLINE=true LC_ALL=C.UTF-8 TZ=UTC "$@") > "$LOGS/$name.log" 2>&1 || result=$?
  printf '%s\t%s\n' "$name" "$result" >> "$LOGS/results.tsv"
  printf '%s %s exit=%s\n' "$MODE" "$name" "$result"
  if ((result != 0)); then FAILED=1; tail -n 50 "$LOGS/$name.log"; fi
}
{ git -C "$ROOT" rev-parse HEAD; cargo --version; rustc --version; uname -srm; } > "$LOGS/provenance.txt"
case "$MODE" in
  client)
    run fmt cargo fmt --manifest-path rust/Cargo.toml -p brain-client -- --check
    run test cargo test --manifest-path rust/Cargo.toml -p brain-client --locked --offline
    run clippy cargo clippy --manifest-path rust/Cargo.toml -p brain-client --all-targets --locked --offline -- -D warnings
    ;;
  quality|cutover)
    if [[ "$MODE" == quality ]]; then MANIFEST=architecture/migration/evals/Cargo.toml; else MANIFEST=infra/cutover/runtime-audit/Cargo.toml; fi
    run fmt cargo fmt --manifest-path "$MANIFEST" -- --check
    run test cargo test --manifest-path "$MANIFEST" --all-targets --locked --offline
    run clippy cargo clippy --manifest-path "$MANIFEST" --all-targets --locked --offline -- -D warnings
    ;;
esac
exit "$FAILED"
