#!/usr/bin/env bash
# Pilot gegen die eigene Brain-Instanz (Port 5446) in einer frischen Datenbank brain_pilot.
# Schema nur über brain-migrate (Peer), Ingest als brain_ingest, brain-serve als brain_service.
# Passwörter kommen aus Infisical und landen nur in der Umgebung der Testprozesse.
set -uo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PG_BIN=/usr/lib/postgresql/16/bin
SOCKET=/run/deadlock-brain-postgresql
PORT=5446
DB=${BRAIN_PILOT_DATABASE:-brain_pilot}
CARGO="${BRAIN_TEST_CARGO:-cargo}"
INFISICAL=${INFISICAL_TOOLS:-$HOME/Documents/Infisical}
REPORT="${BRAIN_PILOT_REPORT:?BRAIN_PILOT_REPORT required}"
: "${BRAIN_PILOT_ROOT:?BRAIN_PILOT_ROOT with public/ and internal/ required}"
: "${BRAIN_MIGRATE_BIN:?BRAIN_MIGRATE_BIN required}"
case "$DB" in brain_pilot*) ;; *) echo "nur brain_pilot*-Datenbanken" >&2; exit 2 ;; esac
SUPER=(sudo -n -u deadlock-brain-pg "$PG_BIN/psql" -X -q -v ON_ERROR_STOP=1 -h "$SOCKET" -p "$PORT")
mkdir -p "$REPORT"
"${SUPER[@]}" -d postgres -v db="$DB" < "$ROOT/ops/brain-postgres/pilot-db.sql" || exit 1
printf '{"socket":"%s","port":%s,"database":"%s","user":"brain_migrate"}\n' "$SOCKET" "$PORT" "$DB" > "$REPORT/migrate.json"
sg deadlock-brain-db -c "'$BRAIN_MIGRATE_BIN' up --config '$REPORT/migrate.json'" > "$REPORT/migrate.log" 2>&1 || exit 1
"${SUPER[@]}" -d postgres -v db="$DB" < "$ROOT/ops/brain-postgres/grants.sql" || exit 1
secret() { python3 "$INFISICAL/export_gpt_secret.py" --secret "$1" 2>/dev/null; }
eval "$(secret BRAIN_PG_INGEST_PASSWORD)" && eval "$(secret BRAIN_PG_SERVICE_PASSWORD)" || { echo "Secrets fehlen" >&2; exit 1; }
export BRAIN_PILOT_TARGET=isolated BRAIN_PILOT_DATABASE="$DB" BRAIN_PILOT_REPORT="$REPORT" BRAIN_PILOT_ROOT
export BRAIN_PILOT_INGEST_PASSWORD=$BRAIN_PG_INGEST_PASSWORD BRAIN_PILOT_SERVICE_PASSWORD=$BRAIN_PG_SERVICE_PASSWORD
unset BRAIN_PG_INGEST_PASSWORD BRAIN_PG_SERVICE_PASSWORD
export http_proxy=http://127.0.0.1:9 https_proxy=http://127.0.0.1:9 HTTP_PROXY=http://127.0.0.1:9 HTTPS_PROXY=http://127.0.0.1:9
export no_proxy=127.0.0.1,localhost NO_PROXY=127.0.0.1,localhost
(cd "$ROOT/rust" && "$CARGO" test --locked -p brain-serve --test local_pilot --no-run) > "$REPORT/build.log" 2>&1 || exit 1
FAILED=0
phase() {
  local test=$1 label=$2
  (cd "$ROOT/rust" && sg deadlock-brain-db -c "'$CARGO' test --locked -p brain-serve --test local_pilot $test -- --ignored --exact --nocapture") > "$REPORT/$label.log" 2>&1
  local result=$?
  printf '%s\t%s\n' "$label" "$result" >> "$REPORT/summary.tsv"
  ((result == 0)) || FAILED=1
}
printf 'phase\texit_code\n' > "$REPORT/summary.tsv"
phase pilot_phase_ingest ingest
sudo -n systemctl restart deadlock-brain-postgresql.service
for _ in $(seq 1 30); do "$PG_BIN/pg_isready" -q -h "$SOCKET" -p "$PORT" && break; sleep 1; done
BRAIN_PILOT_VARIANT=default phase pilot_phase_after_restart after_restart_default
cat "$REPORT/summary.tsv"
exit "$FAILED"
