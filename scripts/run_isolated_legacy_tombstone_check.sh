#!/usr/bin/env bash
# Echter Legacy-Tombstone-Test auf einem frischen, isolierten Unix-Socket-Cluster.
# Die vorhandene Legacy-Kopie wird nur gelesen. Kein Infisical, kein Passwort,
# kein DSN aus der Umgebung und kein Zugriff auf die laufende Brain-Datenbank.
set -euo pipefail
umask 077

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PG_BIN=/usr/lib/postgresql/16/bin
DUMP="$HOME/.local/share/deadlock-brain/legacy-tombstone-20260926/legacy-schema.sql"
IMPORT="$ROOT/rust/target/debug/brain-legacy-import"
MIGRATE="$ROOT/rust/target/debug/brain-migrate"
DB_SRC=brain_pilot_tombstone_src
DB_TGT=brain_pilot_tombstone_tgt
PORT=55446

for binary in initdb pg_ctl psql; do
  test -x "$PG_BIN/$binary"
done
test -s "$DUMP"
test -x "$IMPORT"
test -x "$MIGRATE"
command -v jq >/dev/null

SCRATCH="$(mktemp -d /tmp/brain-tombstone.XXXXXXXXXX)"
readonly SCRATCH
SOCKET="$SCRATCH/socket"
REPORT="$SCRATCH/report"
mkdir "$SOCKET" "$REPORT"
printf 'brain-tombstone-scratch-v1\n' > "$SCRATCH/marker"

cleanup() {
  status=$?
  trap - EXIT
  if test -f "$SCRATCH/data/postmaster.pid"; then
    if ! env -i "$PG_BIN/pg_ctl" -D "$SCRATCH/data" -m fast -w stop >/dev/null; then
      echo "Scratch-Postgres konnte nicht beendet werden; $SCRATCH bleibt zur Prüfung erhalten." >&2
      exit 1
    fi
  fi
  if [[ "$SCRATCH" == /tmp/brain-tombstone.* && -f "$SCRATCH/marker" ]]; then
    rm -rf -- "$SCRATCH"
  fi
  exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

# Ein eigener Cluster ohne TCP und ohne Host-Authentisierung. Es gibt keine
# Verbindung zum produktiven Brain-Postgres-Socket.
env -i "$PG_BIN/initdb" -D "$SCRATCH/data" --username=brain_migrate \
  --auth-local=trust --auth-host=reject --no-locale --encoding=UTF8 > "$SCRATCH/initdb.log"
env -i "$PG_BIN/pg_ctl" -D "$SCRATCH/data" -l "$SCRATCH/server.log" \
  -o "-c listen_addresses='' -k $SOCKET -p $PORT -c max_connections=12 -c shared_buffers=32MB" -w start >/dev/null

psql_admin() {
  env -i "$PG_BIN/psql" -X -q -At -v ON_ERROR_STOP=1 \
    -h "$SOCKET" -p "$PORT" -U brain_migrate -d postgres -c "$1"
}
sql_src() {
  env -i "$PG_BIN/psql" -X -q -At -v ON_ERROR_STOP=1 \
    -h "$SOCKET" -p "$PORT" -U brain_migrate -d "$DB_SRC" -c "$1"
}
sql_tgt() {
  env -i "$PG_BIN/psql" -X -q -At -v ON_ERROR_STOP=1 \
    -h "$SOCKET" -p "$PORT" -U brain_migrate -d "$DB_TGT" -c "$1"
}

# ACL-Anweisungen im vorhandenen Dump referenzieren brain_readonly.
psql_admin "CREATE ROLE brain_readonly NOLOGIN" >/dev/null
psql_admin "CREATE DATABASE $DB_SRC OWNER brain_migrate" >/dev/null
psql_admin "CREATE DATABASE $DB_TGT OWNER brain_migrate" >/dev/null
env -i "$PG_BIN/psql" -X -q -v ON_ERROR_STOP=1 \
  -h "$SOCKET" -p "$PORT" -U brain_migrate -d "$DB_SRC" \
  < "$DUMP" > "$REPORT/restore.log" 2>&1

printf '{"socket":"%s","port":%s,"database":"%s","user":"brain_migrate"}\n' \
  "$SOCKET" "$PORT" "$DB_TGT" > "$REPORT/migrate.json"
env -i "$MIGRATE" up --config "$REPORT/migrate.json" > "$REPORT/migrate.log" 2>&1

# Ein Nonsecret-Config-File steuert beide Verbindungen. Die Kindprozesse
# bekommen mit env -i weder PGPASSWORD noch andere ambient Credentials.
jq --arg socket "$SOCKET" --argjson port "$PORT" \
   --arg source "$DB_SRC" --arg target "$DB_TGT" \
   '.legacy.socket=$socket | .legacy.port=$port | .legacy.database=$source |
    .legacy.username="brain_migrate" | del(.legacy.auth_env) |
    .target.socket=$socket | .target.port=$port | .target.database=$target |
    .target.username="brain_migrate" | del(.target.auth_env)' \
  "$ROOT/ops/brain-postgres/legacy-core-import.json" > "$REPORT/import-template.json"

run_import() {
  label=$1
  jq --arg report "$REPORT/import-$label.report.json" '.report=$report' \
    "$REPORT/import-template.json" > "$REPORT/import-$label.json"
  if env -i "$IMPORT" --config "$REPORT/import-$label.json" \
      > "$REPORT/import-$label.log" 2>&1; then
    return 0
  else
    rc=$?
    if (( rc == 64 )); then
      # Config ist secretfrei; nur die Parser-Kategorie ausgeben.
      sed -n '/^config:/p;/^usage:/p' "$REPORT/import-$label.log" >&2
    fi
    return "$rc"
  fi
}
changed() {
  jq -r --arg source "$2" \
    '.sources[] | select(.source_id==$source) | (.changed_records + .tombstones)' \
    "$REPORT/import-$1.report.json"
}
check() {
  label=$1 expected=$2 actual=$3
  if [[ "$actual" == "$expected" ]]; then
    echo "PASS $label: $actual"
  else
    echo "FAIL $label: erwartet $expected, bekam $actual" >&2
    fails=$((fails + 1))
  fi
}
fails=0

echo '== T1: Basisimport im isolierten Klon'
run_import base
check 'Basisdokumente = 1253' 1253 \
  "$(jq -r '.release_documents' "$REPORT/import-base.report.json")"

echo '== T2: Entfernte Dokumente erzeugen Tombstones'
WARDEN_ID="$(sql_src "SELECT id FROM brain_legacy.entities WHERE canonical_name='Warden' AND entity_type='hero' LIMIT 1")"
[[ "$WARDEN_ID" =~ ^[0-9]+$ ]]
sql_src "DELETE FROM brain_legacy.entity_aliases WHERE entity_id=$WARDEN_ID" >/dev/null
sql_src "DELETE FROM brain_legacy.entities WHERE id=$WARDEN_ID" >/dev/null
sql_src "DELETE FROM brain_legacy.patch_event_enrichments WHERE patch_event_id IN (SELECT id FROM brain_legacy.patch_events WHERE patch_external_id IN (SELECT DISTINCT patch_external_id FROM brain_legacy.patch_events WHERE raw_line ILIKE '%projectical range and speed reduced by 30%'))" >/dev/null
sql_src "DELETE FROM brain_legacy.patch_events WHERE patch_external_id IN (SELECT DISTINCT patch_external_id FROM brain_legacy.patch_events WHERE raw_line ILIKE '%projectical range and speed reduced by 30%')" >/dev/null
run_import removed
check 'ein Entity-Tombstone' 1 "$(changed removed legacy-entities)"
check 'ein Patch-Tombstone' 1 "$(changed removed legacy-patchnotes)"
check 'Entity-Tombstone im Kopf' 1 "$(sql_tgt "SELECT count(*) FROM brain.source_record_heads WHERE source_id='legacy-entities' AND tombstone")"
check 'Patch-Tombstone im Kopf' 1 "$(sql_tgt "SELECT count(*) FROM brain.source_record_heads WHERE source_id='legacy-patchnotes' AND tombstone")"
check 'Tombstone hat leeren Inhalt' '' "$(sql_tgt "SELECT record_json->>'content' FROM brain.source_record_heads WHERE source_id='legacy-entities' AND tombstone")"
check 'Tombstone revisioniert auf 2' 2 "$(sql_tgt "SELECT max(revision) FROM brain.source_record_heads WHERE source_id='legacy-entities' AND tombstone")"

echo '== T3: Wiederholung bleibt idempotent'
run_import removed2
check 'Wiederholung ohne neue Entity-Records' 0 "$(changed removed2 legacy-entities)"
check 'Wiederholung ohne neue Patch-Records' 0 "$(changed removed2 legacy-patchnotes)"

echo '== T4: Leere Entity-Quelle failt closed'
HEADS_BEFORE="$(sql_tgt 'SELECT count(*) FROM brain.source_record_heads')"
sql_src 'TRUNCATE brain_legacy.entity_aliases, brain_legacy.entities CASCADE' >/dev/null
if run_import empty; then
  echo 'FAIL leerer Import wurde angenommen' >&2
  fails=$((fails + 1))
elif grep -Fq 'empty read never tombstones an existing source' "$REPORT/import-empty.log"; then
  echo 'PASS leerer Import schlägt wegen leerer Quelle fehl'
else
  echo 'FAIL leerer Import scheiterte aus anderem Grund' >&2
  fails=$((fails + 1))
fi
check 'Heads unverändert' "$HEADS_BEFORE" "$(sql_tgt 'SELECT count(*) FROM brain.source_record_heads')"
check 'keine weiteren Entity-Tombstones' 1 "$(sql_tgt "SELECT count(*) FROM brain.source_record_heads WHERE tombstone AND source_id='legacy-entities'")"

echo '== T5: Herkunft eindeutig'
check 'Heads eindeutig je Quelle und logical_id' 0 "$(sql_tgt 'SELECT count(*) FROM (SELECT source_id, logical_id FROM brain.source_record_heads GROUP BY 1,2 HAVING count(*) > 1) x')"
check 'Origin-Quelle passt' 0 "$(sql_tgt "SELECT count(*) FROM brain.source_record_heads WHERE NOT tombstone AND (record_json->'metadata'->>'brain.origin')::jsonb->'data'->'identity'->>'source_id' IS DISTINCT FROM source_id")"
check 'Origin-logical_id passt' 0 "$(sql_tgt "SELECT count(*) FROM brain.source_record_heads WHERE NOT tombstone AND (record_json->'metadata'->>'brain.origin')::jsonb->'data'->'identity'->>'logical_id' IS DISTINCT FROM logical_id")"

echo "fehlgeschlagen: $fails"
exit "$((fails > 0))"
