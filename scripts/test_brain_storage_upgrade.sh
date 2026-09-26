#!/usr/bin/env bash
# C11: disposable cluster, Unix socket only. No existing cluster, DSN or production role.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if (( $# > 1 )); then
  echo 'Usage: test_brain_storage_upgrade.sh [cargo-executable]' >&2
  exit 2
fi
CARGO="${1:-cargo}"
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
STARTED=false
cleanup() {
  local status=$?
  trap - EXIT
  if [[ "$STARTED" == true && -f "$SCRATCH/data/postmaster.pid" ]]; then
    if ! "$PG_BIN/pg_ctl" -D "$SCRATCH/data" -m fast -w stop; then
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
mkdir "$SCRATCH/socket"
printf 'brain-c11-scratch-v1\n' > "$SCRATCH/marker"
printf '%s\n' "$PG_BIN" > "$SCRATCH/pg-bin"
"$PG_BIN/initdb" -D "$SCRATCH/data" --username=brain_c11_owner \
  --auth-local=trust --auth-host=reject --no-locale --encoding=UTF8 >/dev/null
STARTED=true
"$PG_BIN/pg_ctl" -D "$SCRATCH/data" -l "$SCRATCH/server.log" \
  -o "-c listen_addresses='' -k $SCRATCH/socket -p 55441 -c max_connections=24 -c shared_buffers=16MB" -w start
BRAIN_C11_SCRATCH="$SCRATCH" "$CARGO" test --manifest-path "$ROOT/rust/Cargo.toml" \
  --locked --offline --jobs 2 -p brain-storage --test storage_upgrade \
  v1_upgrade_restore_and_least_privilege -- --ignored --exact --nocapture
