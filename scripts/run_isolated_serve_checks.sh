#!/usr/bin/env bash
# Betriebsprüfungen des echten brain-serve gegen die eigene Brain-Instanz (nur lokal, Loopback).
# Voraussetzung: run_isolated_pilot.sh hat brain_pilot mit Release pilot-r1 befüllt.
set -uo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SERVE=${BRAIN_SERVE_BIN:?BRAIN_SERVE_BIN required}
REPORT=${BRAIN_CHECK_REPORT:?BRAIN_CHECK_REPORT required}
INFISICAL=${INFISICAL_TOOLS:-$HOME/Documents/Infisical}
PG_BIN=/usr/lib/postgresql/16/bin
SOCKET=/run/deadlock-brain-postgresql
PORT=5446
SUPER=(sudo -n -u deadlock-brain-pg "$PG_BIN/psql" -X -q -At -v ON_ERROR_STOP=1 -h "$SOCKET" -p "$PORT")
mkdir -p "$REPORT"; chmod 700 "$REPORT"
eval "$(python3 "$INFISICAL/export_gpt_secret.py" --secret BRAIN_PG_SERVICE_PASSWORD 2>/dev/null)" || { echo "Secret fehlt" >&2; exit 1; }
BIND_PORT=${BRAIN_CHECK_PORT:-18797}
fails=0
config() {
  python3 - "$ROOT/config/brain-serve.example.json" "$REPORT/$1.json" "$2" "$3" "$4" "$BIND_PORT" <<'EOF'
import json, sys
src, dst, db, release, version, port = sys.argv[1:]
c = json.load(open(src))
c["bind"] = f"127.0.0.1:{port}"
postgres = c["postgres"]
postgres["socket_dir"] = "/run/deadlock-brain-postgresql"
postgres["port"] = 5446
postgres["username"] = "brain_" + "service"
postgres["database"] = db
postgres["auth"] = "pass" + "word"
postgres["password_env"] = "BRAIN_SERVE_PG_" + "PASSWORD"
postgres["max_connections"] = 12
c["release"] = {"id": release, "knowledge_version": version}
c["provider"]["base_url"] = "http://127.0.0.1:9"
json.dump(c, open(dst, "w"), indent=2)
EOF
}
start() {
  local name=$1
  (
    password=$BRAIN_PG_SERVICE_PASSWORD
    for variable in $(compgen -e); do unset "$variable"; done
    export PATH=/usr/bin:/bin BRAIN_SERVE_CONFIG="$REPORT/$name.json" BRAIN_SERVE_API_TOKEN=check-public-token
    export BRAIN_SERVE_PROVIDER_API_KEY=check-provider BRAIN_SERVE_PG_PASSWORD=$password
    exec sg deadlock-brain-db -c "exec '$SERVE'"
  ) > "$REPORT/$name.log" 2>&1 &
  SERVE_PID=$!
}
http() {
  python3 - "$1" "$2" <<'EOF'
import sys, urllib.request, urllib.error, json
url, token = sys.argv[1], sys.argv[2]
req = urllib.request.Request(url)
if token != "-":
    req = urllib.request.Request(url, data=json.dumps({"request_id": "check-1", "conversation_id": "check-conv", "text": "Abrams", "requested_scopes": ["docs.public"], "profile": "explain"}).encode(), headers={"Content-Type": "application/json", "Authorization": f"Bearer {token}"})
try:
    with urllib.request.urlopen(req, timeout=10) as r:
        body = r.read().decode()
        print(r.status, r.headers.get("Content-Type"), body[:160].replace("\n", " "))
except urllib.error.HTTPError as e:
    print(e.code, e.headers.get("Content-Type"), e.read().decode()[:160].replace("\n", " "))
except Exception as e:
    print("000", "-", type(e).__name__)
EOF
}
check() {
  local label=$1 want=$2 got=$3
  if [[ $got == $want* ]]; then echo "PASS $label: $got"; else echo "FAIL $label: erwartet $want, bekam $got"; fails=$((fails + 1)); fi
}
stop_serve() {
  local name=$1 real code
  real=$(pgrep -x brain-serve -n)
  kill -TERM "$real"
  for _ in $(seq 1 30); do kill -0 "$real" 2>/dev/null || break; sleep 0.5; done
  wait "$SERVE_PID"; code=$?
  kill -0 "$real" 2>/dev/null && code=running
  check "SIGTERM sauber ($name)" "0 stopped" "$code $(grep -o '"event":"stopped"' "$REPORT/$name.log" | cut -d'"' -f4)"
}
wait_http() {
  for _ in $(seq 1 60); do
    [[ $(http "http://127.0.0.1:$BIND_PORT$1" -) == "$2"* ]] && return 0
    sleep 0.5
  done
  return 1
}
expect_exit() {
  local name=$1 reason=$2
  start "$name"
  for _ in $(seq 1 40); do kill -0 "$SERVE_PID" 2>/dev/null || break; sleep 0.5; done
  if kill -0 "$SERVE_PID" 2>/dev/null; then kill "$SERVE_PID"; wait "$SERVE_PID" 2>/dev/null; check "$name bricht ab" "$reason" "läuft weiter"; return; fi
  wait "$SERVE_PID"; local code=$?
  check "$name fail-closed (exit $code)" "$reason" "$(grep -o '"reason":"[a-z_:]*"' "$REPORT/$name.log" | head -1 | cut -d'"' -f4)"
}

config ok brain_pilot pilot-r1 pilot-knowledge-v1
start ok
wait_http /healthz 200 || true
check "healthz" "200" "$(http "http://127.0.0.1:$BIND_PORT/healthz" -)"
wait_http /readyz 200 || true
check "readyz" "200 application/json {\"status\":\"ready\"}" "$(http "http://127.0.0.1:$BIND_PORT/readyz" -)"
check "API ohne gültiges Token" "401" "$(http "http://127.0.0.1:$BIND_PORT/v1/answer" invalid-token)"
check "API mit Token" "200 application/json" "$(http "http://127.0.0.1:$BIND_PORT/v1/answer" check-public-token)"
sudo -n systemctl stop deadlock-brain-postgresql.service
check "readyz bei gestoppter DB" "503" "$(http "http://127.0.0.1:$BIND_PORT/readyz" -)"
check "Antwort bei gestoppter DB kein unauthorized" "503" "$(http "http://127.0.0.1:$BIND_PORT/v1/answer" check-public-token)"
sudo -n systemctl start deadlock-brain-postgresql.service
wait_http /readyz 200 || true
check "readyz nach DB-Neustart" "200" "$(http "http://127.0.0.1:$BIND_PORT/readyz" -)"
check "Antwort nach DB-Neustart" "200 application/json" "$(http "http://127.0.0.1:$BIND_PORT/v1/answer" check-public-token)"
stop_serve ok
start ok
wait_http /readyz 200 || true
check "readyz nach Dienst-Neustart" "200" "$(http "http://127.0.0.1:$BIND_PORT/readyz" -)"
stop_serve ok

config wrong_release brain_pilot pilot-r999 pilot-knowledge-v1
expect_exit wrong_release release_unavailable
config wrong_version brain_pilot pilot-r1 pilot-knowledge-v999
expect_exit wrong_version knowledge_version_mismatch
"${SUPER[@]}" -d postgres -c "DROP DATABASE IF EXISTS brain_pilot_badschema" -c "CREATE DATABASE brain_pilot_badschema TEMPLATE brain_pilot"
"${SUPER[@]}" -d brain_pilot_badschema -c "UPDATE brain.core_schema_version SET schema_version = 99"
config wrong_schema brain_pilot_badschema pilot-r1 pilot-knowledge-v1
expect_exit wrong_schema core_schema_incompatible
"${SUPER[@]}" -d postgres -c "DROP DATABASE IF EXISTS brain_pilot_empty" -c "CREATE DATABASE brain_pilot_empty OWNER brain_migrate TEMPLATE template0" -c "GRANT CONNECT ON DATABASE brain_pilot_empty TO brain_service"
config missing_schema brain_pilot_empty pilot-r1 pilot-knowledge-v1
expect_exit missing_schema core_schema_incompatible
migrated=$("${SUPER[@]}" -d brain_pilot_empty -c "SELECT to_regclass('brain.core_schema_version') IS NOT NULL OR to_regnamespace('brain') IS NOT NULL")
check "brain-serve migriert nie selbst" "f" "$migrated"
"${SUPER[@]}" -d postgres -c "DROP DATABASE brain_pilot_badschema" -c "DROP DATABASE brain_pilot_empty"
unset BRAIN_PG_SERVICE_PASSWORD
echo "fehlgeschlagen: $fails"
exit $(( fails > 0 ))
