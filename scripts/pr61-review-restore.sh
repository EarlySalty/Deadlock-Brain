#!/usr/bin/env bash
# Fault injection against the PR's exact fingerprint function on an owned empty scratch DB.
# Does NOT execute restore-probe.sh or connect to production.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LOGS="$ROOT/.core-test-logs"
mkdir -p "$LOGS"
PG_BIN="$(pg_config --bindir)"
WORK="$(mktemp -d /tmp/pr61-restore.XXXXXX)"
cleanup() {
  "$PG_BIN/pg_ctl" -D "$WORK/data" -m immediate -w stop >/dev/null 2>&1 || true
  rm -rf -- "$WORK"
}
trap cleanup EXIT
"$PG_BIN/initdb" -D "$WORK/data" --username="$(id -un)" --auth-local=peer --auth-host=reject --no-sync > "$WORK/init.log" 2>&1
"$PG_BIN/pg_ctl" -D "$WORK/data" -l "$WORK/server.log" -o "-c listen_addresses='' -k $WORK -p 55449 -c max_connections=10 -c shared_buffers=16MB" -w start > "$WORK/start.log" 2>&1
sed -n '/^fingerprint() {$/,/^}$/p' "$ROOT/ops/brain-postgres/restore-probe.sh" > "$WORK/fingerprint.sh"
source "$WORK/fingerprint.sh"
set +e
fingerprint | "$PG_BIN/psql" -X -q -At -h "$WORK" -p 55449 -d postgres > "$WORK/old.txt" 2> "$WORK/old.err"
old=$?
fingerprint | "$PG_BIN/psql" -X -q -At -v ON_ERROR_STOP=1 -h "$WORK" -p 55449 -d postgres > "$WORK/strict.txt" 2> "$WORK/strict.err"
strict=$?
set -e
{
  printf 'PR61_RESTORE_SQL_ERROR: baseline_exit=%s, strict_exit=%s, baseline_sql_errors=%s\n' "$old" "$strict" "$(grep -c '^ERROR:' "$WORK/old.err")"
  cat "$WORK/old.err"
} | tee "$LOGS/pr61-restore.log"
printf 'pr61-restore\t%s\n' "$((old == 0 ? 1 : 0))" >> "$LOGS/pr61-restore-result.tsv"
# The required behavior is a nonzero exit for a failed fingerprint query.
[[ $old -ne 0 ]]
