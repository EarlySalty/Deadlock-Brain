#!/usr/bin/env bash
# Review/CI verification. All application credentials and the normal user HOME are excluded.
set -uo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CARGO="${BRAIN_TEST_CARGO:-cargo}"
CARGO_CACHE="${CARGO_HOME:-$HOME/.cargo}"
RUSTUP_CACHE="${RUSTUP_HOME:-$HOME/.rustup}"
LOGS="$ROOT/.core-test-logs"
mkdir -p "$LOGS/test-home"
MODE="${1:-all}"
RESULTS="$LOGS/results-$MODE.tsv"
printf 'check\texit_code\n' > "$RESULTS"
FAILED=0
run_check() {
  local name="$1"; shift
  printf 'RUN %s\n' "$name"
  (
    cd "$ROOT/rust"
    env -i PATH="$PATH" HOME="$LOGS/test-home" CARGO_HOME="$CARGO_CACHE" RUSTUP_HOME="$RUSTUP_CACHE" \
      CARGO_BUILD_JOBS=2 BRAIN_TEST_CARGO="$CARGO" LC_ALL=C.UTF-8 TZ=UTC "$@"
  ) > "$LOGS/$name.log" 2>&1
  local result=$?
  printf '%s\t%s\n' "$name" "$result" >> "$RESULTS"
  printf 'DONE %s exit=%s\n' "$name" "$result"
  if ((result != 0)); then FAILED=1; tail -n 60 "$LOGS/$name.log"; fi
}
case "$MODE" in
  bootstrap)
    run_check bootstrap "$CARGO" check -p brain-api --all-targets
    ;;
  core)
    run_check core "$CARGO" test --locked -p brain-contracts -p brain-policy -p brain-storage -p brain-ingestion \
      -p brain-providers -p brain-jev -p brain-kernel -p brain-api -p brain-client -p dbrain-retrieval -p dbrain-reasoner
    ;;
  all)
    run_check fmt "$CARGO" fmt --all -- --check
    run_check clippy "$CARGO" clippy --workspace --all-targets --locked -- -D warnings
    run_check test "$CARGO" test --workspace --locked
    run_check release "$CARGO" build --workspace --release --locked
    run_check postgres bash "$ROOT/scripts/test_brain_core_postgres.sh"
    ;;
  *) echo 'Usage: check_brain_core.sh [bootstrap|core|all]' >&2; exit 2 ;;
esac
exit "$FAILED"
