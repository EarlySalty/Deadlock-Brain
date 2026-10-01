#!/usr/bin/env bash
# Review/CI verification. All application credentials and the normal user HOME are excluded.
set -uo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CARGO="${BRAIN_TEST_CARGO:-cargo}"
CARGO_CACHE="${CARGO_HOME:-$HOME/.cargo}"
RUSTUP_CACHE="${RUSTUP_HOME:-$HOME/.rustup}"
if (( $# < 2 )); then
  printf 'Usage: check_brain_core.sh <bootstrap|core|release|postgres|all> <absolute-existing-target-dir> [--peer EXPLICIT_HEAD]...\n' >&2
  exit 2
fi
MODE="$1"
TARGET_DIR="$2"
shift 2
PEER_REFS=()
while (($#)); do
  if [[ "$MODE" != all || "$1" != --peer || $# -lt 2 ]]; then
    printf 'Peer heads are accepted only by all as --peer EXPLICIT_HEAD.\n' >&2
    exit 2
  fi
  PEER_REFS+=("$2")
  shift 2
done
if [[ "$TARGET_DIR" != /* || ! -d "$TARGET_DIR" ]]; then
  printf 'Target cache must be an existing absolute directory: %s\n' "$TARGET_DIR" >&2
  exit 2
fi
case "$MODE" in
  bootstrap|core|release|postgres|all) ;;
  *) printf 'Unknown verification mode: %s\n' "$MODE" >&2; exit 2 ;;
esac
LOGS="$ROOT/.core-test-logs"
mkdir -p "$LOGS/test-home"
RESULTS="$LOGS/results-$MODE.tsv"
printf 'check\texit_code\n' > "$RESULTS"
FAILED=0
run_check() {
  local name="$1"; shift
  printf 'RUN %s\n' "$name"
  (
    cd "$ROOT/rust" || exit
    env -i PATH="$PATH" HOME="$LOGS/test-home" CARGO_HOME="$CARGO_CACHE" RUSTUP_HOME="$RUSTUP_CACHE" \
      CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR="$TARGET_DIR" BRAIN_TEST_CARGO="$CARGO" LC_ALL=C.UTF-8 TZ=UTC "$@"
  ) > "$LOGS/$name.log" 2>&1
  local result=$?
  printf '%s\t%s\n' "$name" "$result" >> "$RESULTS"
  printf 'DONE %s exit=%s\n' "$name" "$result"
  if ((result != 0)); then FAILED=1; tail -n 60 "$LOGS/$name.log"; fi
}
case "$MODE" in
  bootstrap)
    run_check bootstrap "$CARGO" +1.97.1 check -p brain-api --all-targets --locked --offline --jobs 1 --target-dir "$TARGET_DIR"
    ;;
  core)
    run_check core "$CARGO" +1.97.1 test --locked --offline --jobs 1 --target-dir "$TARGET_DIR" -p brain-contracts -p brain-policy -p brain-storage -p brain-ingestion \
      -p brain-providers -p brain-jev -p brain-kernel -p brain-api -p brain-client -p dbrain-retrieval -p dbrain-reasoner
    ;;
  release)
    run_check release "$CARGO" +1.97.1 build --workspace --release --locked --offline --jobs 1 --target-dir "$TARGET_DIR"
    ;;
  postgres)
    run_check postgres bash "$ROOT/scripts/test_brain_core_postgres.sh" "$TARGET_DIR"
    ;;
  all)
    MIGRATION_ARGS=(--base origin/main --head HEAD)
    for peer_ref in "${PEER_REFS[@]}"; do MIGRATION_ARGS+=(--peer "$peer_ref"); done
    run_check migration-guard bash "$ROOT/scripts/ci/run_migration_guard.sh" "${MIGRATION_ARGS[@]}"
    run_check backup-shell-syntax bash -c "bash -n '$ROOT/ops/brain-postgres/backup.sh' && bash -n '$ROOT/ops/brain-postgres/restore-probe.sh' && bash -n '$ROOT/scripts/test_brain_backup.sh' && bash -n '$ROOT/scripts/test_brain_restore.sh'"
    run_check backup-shellcheck shellcheck "$ROOT/ops/brain-postgres/backup.sh" "$ROOT/ops/brain-postgres/restore-probe.sh" "$ROOT/scripts/test_brain_backup.sh" "$ROOT/scripts/test_brain_restore.sh"
    run_check backup-regressions bash "$ROOT/scripts/test_brain_backup.sh"
    run_check restore-regressions bash "$ROOT/scripts/test_brain_restore.sh"
    run_check fmt "$CARGO" +1.97.1 fmt --all -- --check
    run_check clippy "$CARGO" +1.97.1 clippy --workspace --all-targets --locked --offline --jobs 1 --target-dir "$TARGET_DIR" -- -D warnings
    run_check test "$CARGO" +1.97.1 test --workspace --locked --offline --jobs 1 --target-dir "$TARGET_DIR"
    run_check release "$CARGO" +1.97.1 build --workspace --release --locked --offline --jobs 1 --target-dir "$TARGET_DIR"
    run_check postgres bash "$ROOT/scripts/test_brain_core_postgres.sh" "$TARGET_DIR"
    ;;
  *) printf 'Usage: check_brain_core.sh <bootstrap|core|release|postgres|all> <absolute-existing-target-dir>\n' >&2; exit 2 ;;
esac
exit "$FAILED"
