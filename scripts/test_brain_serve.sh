#!/usr/bin/env bash
# Disposable, Unix-socket-only PostgreSQL and real brain-serve process tests.
# This does not use DATABASE_URL, production data, Infisical, or systemd.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PG_BIN="${BRAIN_TEST_PG_BIN:-$(pg_config --bindir)}"
CARGO="${BRAIN_TEST_CARGO:-cargo}"
SCRATCH="$(mktemp -d "${TMPDIR:-/tmp}/brain-serve-tests.XXXXXX")"
CLUSTER="$SCRATCH/.core-test-pg"
STARTED=0
cleanup() {
  if [[ "$STARTED" == 1 ]] && "$PG_BIN/pg_ctl" -D "$CLUSTER" status >/dev/null 2>&1; then
    "$PG_BIN/pg_ctl" -D "$CLUSTER" -m fast -w stop || return
  fi
  # Delete only the directory made by this invocation, never an operator-supplied path.
  rm -rf -- "$SCRATCH"
}
trap cleanup EXIT
"$PG_BIN/initdb" -D "$CLUSTER" --auth=trust --username=brain_core_test --no-locale --encoding=UTF8 >/dev/null
# This rule exercises the password environment port over a local socket (no TCP/TLS bypass).
{ printf 'local brain_serve_test brain_serve_fixture scram-sha-256\n'; cat "$CLUSTER/pg_hba.conf"; } > "$SCRATCH/pg_hba.conf"
mv "$SCRATCH/pg_hba.conf" "$CLUSTER/pg_hba.conf"
STARTED=1
"$PG_BIN/pg_ctl" -D "$CLUSTER" -l "$SCRATCH/postgres.log" \
  -o "-c listen_addresses='' -k $CLUSTER -p 55439 -c max_connections=12 -c shared_buffers=16MB" -w start
"$PG_BIN/createdb" -h "$CLUSTER" -p 55439 -U brain_core_test brain_serve_test
cd "$ROOT/rust"
SQLX_OFFLINE=true BRAIN_CORE_TEST_PG_SOCKET="$CLUSTER" "$CARGO" test --locked -p brain-serve --test process_e2e \
  binary_loopback_health_readiness_shutdown_and_no_fallback -- --ignored --exact --nocapture
if grep -Fq "too many clients already" "$SCRATCH/postgres.log"; then
  printf '%s\n' "scratch PostgreSQL exceeded max_connections=12" >&2
  exit 1
fi
