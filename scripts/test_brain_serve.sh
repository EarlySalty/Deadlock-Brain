#!/usr/bin/env bash
set -euo pipefail
umask 077
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PG_BIN="${BRAIN_TEST_PG_BIN:-$(pg_config --bindir)}"
CARGO="${BRAIN_TEST_CARGO:-$HOME/.cargo/bin/cargo}"
if (( EUID == 0 )); then
  printf 'Run as an unprivileged user.\n' >&2
  exit 2
fi
for binary in initdb pg_ctl createdb; do
  test -x "$PG_BIN/$binary"
done
SCRATCH="$(mktemp -d "${TMPDIR:-/tmp}/brain-serve-tests.XXXXXX")"
CLUSTER="$SCRATCH/.core-test-pg"
PG_ENV=(env -i "PATH=$PATH" "HOME=$SCRATCH")
STARTED=0
cleanup() {
  local status=$?
  trap - EXIT
  if (( STARTED == 1 )); then
    if ! "${PG_ENV[@]}" "$PG_BIN/pg_ctl" -D "$CLUSTER" -m fast -w stop >/dev/null; then
      printf 'Scratch PostgreSQL could not be stopped: %s\n' "$SCRATCH" >&2
      exit 1
    fi
  fi
  if (( status != 0 )); then
    printf 'Scratch PostgreSQL log: %s\n' "$SCRATCH/postgres.log" >&2
  else
    rm -rf -- "$SCRATCH"
  fi
  exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
"${PG_ENV[@]}" "$PG_BIN/initdb" -D "$CLUSTER" --auth-local=peer --auth-host=reject --username=brain_core_test --no-locale --encoding=UTF8 > "$SCRATCH/initdb.log"
OS_USER="$(id -un)"
if [[ ! "$OS_USER" =~ ^[a-z_][a-z0-9_-]*$ ]]; then
  printf 'Unsupported local account name.\n' >&2
  exit 2
fi
printf 'brain_scratch %s brain_core_test\nbrain_scratch %s brain_serve_fixture\n' "$OS_USER" "$OS_USER" > "$CLUSTER/pg_ident.conf"
printf 'local all all peer map=brain_scratch\nhost all all all reject\n' > "$CLUSTER/pg_hba.conf"
"${PG_ENV[@]}" "$PG_BIN/pg_ctl" -D "$CLUSTER" -l "$SCRATCH/postgres.log" \
  -o "-c listen_addresses='' -k $CLUSTER -p 55439 -c max_connections=12 -c shared_buffers=16MB" -w start
STARTED=1
"${PG_ENV[@]}" "$PG_BIN/createdb" -h "$CLUSTER" -p 55439 -U brain_core_test brain_serve_test
"${PG_ENV[@]}" "$PG_BIN/createdb" -h "$CLUSTER" -p 55439 -U brain_core_test brain_legacy_test
cd "$ROOT/rust"
env -i "PATH=$PATH" "HOME=$SCRATCH" "CARGO_HOME=${CARGO_HOME:-$HOME/.cargo}" \
  "RUSTUP_HOME=${RUSTUP_HOME:-$HOME/.rustup}" "CARGO_TARGET_DIR=$ROOT/rust/target" \
  "CARGO_BUILD_JOBS=2" "SQLX_OFFLINE=true" "BRAIN_CORE_TEST_PG_SOCKET=$CLUSTER" \
  "$CARGO" test --locked --offline --jobs 2 -p brain-serve --test process_e2e \
  binary_loopback_health_readiness_shutdown_and_no_fallback -- --ignored --exact --nocapture
env -i "PATH=$PATH" "HOME=$SCRATCH" "CARGO_HOME=${CARGO_HOME:-$HOME/.cargo}" \
  "RUSTUP_HOME=${RUSTUP_HOME:-$HOME/.rustup}" "CARGO_TARGET_DIR=$ROOT/rust/target" \
  "CARGO_BUILD_JOBS=2" "SQLX_OFFLINE=true" "BRAIN_CORE_TEST_PG_SOCKET=$CLUSTER" \
  "$CARGO" test --locked --offline --jobs 2 -p brain-legacy-import --lib \
  scratch_import_release_tombstone_and_revoke -- --ignored --nocapture
if grep -Eq 'too many clients already|too many connections for role' "$SCRATCH/postgres.log"; then
  printf 'Scratch PostgreSQL exceeded its connection limit.\n' >&2
  exit 1
fi
