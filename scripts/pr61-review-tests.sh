#!/usr/bin/env bash
# Isolated PR review evidence only. No secrets, production DBs, writes to branches or deployment.
set -uo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TARGET="${1:?existing target cache}"
CARGO=/home/nathanael/.cargo/bin/cargo
CACHE=/home/nathanael/.cargo
RUSTUP=/home/nathanael/.rustup
LOGS="$ROOT/.core-test-logs"
mkdir -p "$LOGS/test-home"
printf 'check\texit_code\n' > "$LOGS/pr61-results.tsv"
run() {
    local name="$1"; shift
    ( cd "$ROOT/rust"; env -i PATH="/home/nathanael/.cargo/bin:/usr/local/bin:/usr/bin:/bin" HOME="$LOGS/test-home" CARGO_HOME="$CACHE" RUSTUP_HOME="$RUSTUP" CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR="$TARGET" BRAIN_TEST_CARGO="$CARGO" LC_ALL=C.UTF-8 TZ=UTC "$@" ) > "$LOGS/$name.log" 2>&1
    local result=$?
    printf '%s\t%s\n' "$name" "$result" >> "$LOGS/pr61-results.tsv"
    printf '%s exit=%s\n' "$name" "$result"
}
run pr61-negatives "$CARGO" +1.97.1 test --locked --offline --jobs 1 --target-dir "$TARGET" -p brain-api --test pr61_review --test pr61_pg_deadline --no-fail-fast -- --nocapture
run pr61-serve bash "$ROOT/scripts/test_brain_serve.sh" "$TARGET"
