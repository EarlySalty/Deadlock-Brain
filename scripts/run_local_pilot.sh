#!/usr/bin/env bash
# Local-only pilot. Dedicated Unix-socket scratch cluster, no TCP, no production DSN, no secrets.
# BRAIN_PILOT_ROOT must contain public/ and internal/ with locally approved documents outside Git.
set -uo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PG_BIN="${BRAIN_TEST_PG_BIN:-$(pg_config --bindir)}"
CARGO="${BRAIN_TEST_CARGO:-cargo}"
CLUSTER="$ROOT/.core-test-pg"
REPORT="${BRAIN_PILOT_REPORT:-$ROOT/.core-test-logs/pilot}"
: "${BRAIN_PILOT_ROOT:?BRAIN_PILOT_ROOT with public/ and internal/ required}"
case "$BRAIN_PILOT_ROOT" in "$ROOT"/*) echo 'Pilot data must live outside the repository.' >&2; exit 2 ;; esac
if "$PG_BIN/pg_ctl" -D "$CLUSTER" status >/dev/null 2>&1; then
  echo 'Refusing to reuse or stop an already-running cluster.' >&2
  exit 1
fi
if [[ ! -f "$CLUSTER/PG_VERSION" ]]; then
  "$PG_BIN/initdb" -D "$CLUSTER" --auth=trust --username=brain_core_test --no-locale --encoding=UTF8 >/dev/null
fi
rm -rf "$REPORT"
mkdir -p "$REPORT"
export BRAIN_CORE_TEST_PG_SOCKET="$CLUSTER" BRAIN_PILOT_REPORT="$REPORT" BRAIN_PILOT_ROOT
export http_proxy=http://127.0.0.1:9 https_proxy=http://127.0.0.1:9 HTTP_PROXY=http://127.0.0.1:9 HTTPS_PROXY=http://127.0.0.1:9
export no_proxy=127.0.0.1,localhost NO_PROXY=127.0.0.1,localhost
start() {
  "$PG_BIN/pg_ctl" -D "$CLUSTER" -l "$CLUSTER/server.log" \
    -o "-c listen_addresses='' -k $CLUSTER -p 55439 -c max_connections=12 -c shared_buffers=16MB" -w start >>"$REPORT/pg.log" 2>&1
}
start || { echo 'scratch cluster did not start' >&2; exit 1; }
trap '"$PG_BIN/pg_ctl" -D "$CLUSTER" -m fast -w stop >>"$REPORT/pg.log" 2>&1' EXIT
for db in pilot_main pilot_rebuild; do
  "$PG_BIN/dropdb" -h "$CLUSTER" -p 55439 -U brain_core_test --if-exists "$db"
  "$PG_BIN/createdb" -h "$CLUSTER" -p 55439 -U brain_core_test "$db"
done
FAILED=0
phase() {
  local database="$1" test="$2" label="$3"
  (cd "$ROOT/rust" && "$CARGO" test --locked -p brain-api --test local_pilot --no-run) >"$REPORT/$label.build.log" 2>&1
  (cd "$ROOT/rust" && BRAIN_PILOT_DATABASE="$database" "$CARGO" test --locked -p brain-api --test local_pilot "$test" -- --ignored --exact --nocapture) >"$REPORT/$label.log" 2>&1
  local result=$?
  printf '%s\t%s\n' "$label" "$result" >>"$REPORT/summary.tsv"
  ((result == 0)) || FAILED=1
}
printf 'phase\texit_code\n' >"$REPORT/summary.tsv"
phase pilot_main pilot_phase_ingest ingest
"$PG_BIN/pg_ctl" -D "$CLUSTER" -m immediate -w stop >>"$REPORT/pg.log" 2>&1
start || { echo 'scratch cluster did not restart after the forced stop' >&2; exit 1; }
BRAIN_PILOT_VARIANT=default phase pilot_main pilot_phase_after_restart after_restart_default
if [[ -n "${BRAIN_PILOT_DIAGNOSTIC_MAX_INPUT_TOKENS:-}" ]]; then
  BRAIN_PILOT_VARIANT=diagnostic BRAIN_PILOT_RETRIEVAL_LIMIT="${BRAIN_PILOT_DIAGNOSTIC_LIMIT:-1}" \
    BRAIN_PILOT_MAX_INPUT_TOKENS="$BRAIN_PILOT_DIAGNOSTIC_MAX_INPUT_TOKENS" \
    phase pilot_main pilot_phase_after_restart after_restart_diagnostic
fi
phase pilot_rebuild pilot_phase_empty_rebuild rebuild
cat "$REPORT/summary.tsv"
exit "$FAILED"
