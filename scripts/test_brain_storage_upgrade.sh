#!/usr/bin/env bash
# C11: disposable cluster, Unix socket only. No existing cluster, DSN or production role.
set -euo pipefail
umask 077
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if (( $# < 1 || $# > 2 )); then
  printf 'Usage: test_brain_storage_upgrade.sh [cargo-executable] <absolute-existing-target-dir>\n' >&2
  exit 2
fi
if (( $# == 2 )); then
  CARGO="$1"
  TARGET_DIR="$2"
else
  CARGO="$HOME/.cargo/bin/cargo"
  TARGET_DIR="$1"
fi
if [[ "$TARGET_DIR" != /* || ! -d "$TARGET_DIR" ]]; then
  printf 'Target cache must be an existing absolute directory: %s\n' "$TARGET_DIR" >&2
  exit 2
fi
if ! command -v "$CARGO" >/dev/null 2>&1; then
  printf 'Cargo executable not found: %s\n' "$CARGO" >&2
  exit 2
fi
PG_BIN="$(pg_config --bindir)"
for binary in initdb pg_ctl pg_dump pg_restore; do
  test -x "$PG_BIN/$binary"
done
if (( EUID == 0 )); then
  echo 'Run as a non-root test user; this script never uses sudo or a system cluster.' >&2
  exit 2
fi
# No ambient PostgreSQL credentials/configuration; all connection coordinates below are explicit.
unset DATABASE_URL PGHOST PGHOSTADDR PGPORT PGUSER PGDATABASE PGPASSWORD PGPASSFILE PGSERVICE PGSERVICEFILE PGOPTIONS
SCRATCH="$(mktemp -d /tmp/brain-c11.XXXXXXXXXX)"
readonly SCRATCH
PG_ENV=(env -i "PATH=$PATH" "HOME=$SCRATCH")
STARTED=false
cleanup() {
  local status=$?
  trap - EXIT
  if [[ "$STARTED" == true && -f "$SCRATCH/data/postmaster.pid" ]]; then
    if ! "${PG_ENV[@]}" "$PG_BIN/pg_ctl" -D "$SCRATCH/data" -m fast -w stop; then
      echo "Scratch shutdown failed; retained $SCRATCH for inspection." >&2
      exit 1
    fi
  fi
  if (( status != 0 )) && [[ -f "$SCRATCH/server.log" ]]; then
    tail -n 30 "$SCRATCH/server.log" >&2
  fi
  # Only the directory allocated by mktemp in this invocation; never a supplied path.
  rm -rf -- "$SCRATCH"
  exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
mkdir "$SCRATCH/socket" "$SCRATCH/home"
printf 'brain-c11-scratch-v1\n' > "$SCRATCH/marker"
printf '%s\n' "$PG_BIN" > "$SCRATCH/pg-bin"
"${PG_ENV[@]}" "$PG_BIN/initdb" -D "$SCRATCH/data" --username=brain_c11_owner \
  --auth-local=peer --auth-host=reject --no-locale --encoding=UTF8 >/dev/null
OS_USER="$(id -un)"
if [[ ! "$OS_USER" =~ ^[a-z_][a-z0-9_-]*$ ]]; then
  printf 'Unsupported local account name.\n' >&2
  exit 2
fi
printf 'brain_scratch %s brain_c11_owner\nbrain_scratch %s brain_c11_runtime\n' "$OS_USER" "$OS_USER" > "$SCRATCH/data/pg_ident.conf"
printf 'local all all peer map=brain_scratch\nhost all all all reject\n' > "$SCRATCH/data/pg_hba.conf"
STARTED=true
"${PG_ENV[@]}" "$PG_BIN/pg_ctl" -D "$SCRATCH/data" -l "$SCRATCH/server.log" \
  -o "-c listen_addresses='' -k $SCRATCH/socket -p 55441 -c max_connections=24 -c shared_buffers=16MB" -w start
env -i "PATH=$PATH" "HOME=$SCRATCH/home" "CARGO_HOME=${CARGO_HOME:-$HOME/.cargo}" \
  "RUSTUP_HOME=${RUSTUP_HOME:-$HOME/.rustup}" \
  "CARGO_BUILD_JOBS=1" "SQLX_OFFLINE=true" "BRAIN_C11_SCRATCH=$SCRATCH" \
  "$CARGO" +1.97.1 test --manifest-path "$ROOT/rust/Cargo.toml" \
  --locked --offline --jobs 1 --target-dir "$TARGET_DIR" -p brain-storage --test storage_upgrade \
  v1_upgrade_restore_and_least_privilege -- --ignored --exact --nocapture
