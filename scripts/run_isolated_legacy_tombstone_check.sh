#!/usr/bin/env bash
# Zusatzpruefungen auf echten Daten: Tombstones bei entfernten Dokumenten,
# leerer Import toetboardet nicht, Herkunft eindeutig.
# Quelle: brain_pilot_tombstone_src (Legacy-Kopie), Ziel: brain_pilot_tombstone_tgt.
set -uo pipefail
ROOT=/home/nathanael/.worktrees/brain-g5-pre-integration
PG_BIN=/usr/lib/postgresql/16/bin
SOCKET=/run/deadlock-brain-postgresql
PORT=5446
DB_SRC=brain_pilot_tombstone_src
DB_TGT=brain_pilot_tombstone_tgt
INFISICAL=$HOME/Documents/Infisical
IMPORT=$ROOT/rust/target/debug/brain-legacy-import
REPORT=$HOME/.local/share/deadlock-brain/legacy-tombstone-20260926
mkdir -p "$REPORT"; chmod 700 "$REPORT"
fails=0
check() { if [[ $3 == "$2"* ]]; then echo "PASS $1: $3"; else echo "FAIL $1: erwartet $2, bekam $3"; fails=$((fails+1)); fi }
sql() { sudo -n -u deadlock-brain-pg "$PG_BIN/psql" -X -q -At -v ON_ERROR_STOP=1 -h "$SOCKET" -p "$PORT" -d "$DB_TGT" -c "$1"; }
sql_src() { sudo -n -u deadlock-brain-pg "$PG_BIN/psql" -X -q -At -v ON_ERROR_STOP=1 -h "$SOCKET" -p "$PORT" -d "$DB_SRC" -c "$1"; }
field() { python3 -c "import json,sys; d=json.load(open(sys.argv[1])); print(eval(sys.argv[2]))" "$@"; }

# 1) Zwei Wegwerf-DBs: Quelle (Legacy-Kopie) und Ziel (Kernschema)
PGP=(sudo -n -u deadlock-brain-pg "$PG_BIN/psql" -X -q -v ON_ERROR_STOP=1 -h "$SOCKET" -p "$PORT")
"${PGP[@]}" -d postgres -v db="$DB_SRC" < "$ROOT/ops/brain-postgres/pilot-db.sql" || exit 1
"${PGP[@]}" -d postgres -v db="$DB_TGT" < "$ROOT/ops/brain-postgres/pilot-db.sql" || exit 1
sudo -n -u deadlock-brain-pg "$PG_BIN/pg_dump" -h "$SOCKET" -p "$PORT" -d brain -n brain_legacy > "$REPORT/legacy-schema.sql" || exit 1
sudo -n -u deadlock-brain-pg "$PG_BIN/psql" -X -q -v ON_ERROR_STOP=1 -h "$SOCKET" -p "$PORT" -d "$DB_SRC" < "$REPORT/legacy-schema.sql" || exit 1
printf '{"socket":"%s","port":%s,"database":"%s","user":"brain_migrate"}\n' "$SOCKET" "$PORT" "$DB_TGT" > "$REPORT/migrate.json"
sg deadlock-brain-db -c "'$ROOT/rust/target/debug/brain-migrate' up --config '$REPORT/migrate.json'" > "$REPORT/migrate.log" 2>&1 || exit 1
# Quelle braucht das Kernschema nur fuer grants.sql (brain_readonly auf brain_legacy)
printf '{"socket":"%s","port":%s,"database":"%s","user":"brain_migrate"}\n' "$SOCKET" "$PORT" "$DB_SRC" > "$REPORT/migrate-src.json"
sg deadlock-brain-db -c "'$ROOT/rust/target/debug/brain-migrate' up --config '$REPORT/migrate-src.json'" > "$REPORT/migrate-src.log" 2>&1 || exit 1
"${PGP[@]}" -d postgres -v db="$DB_SRC" < "$ROOT/ops/brain-postgres/grants.sql" || exit 1
"${PGP[@]}" -d postgres -v db="$DB_TGT" < "$ROOT/ops/brain-postgres/grants.sql" || exit 1

# 2) Import-Konfiguration: getrennte Quelle und Ziel
python3 - "$ROOT/ops/brain-postgres/legacy-core-import.json" "$REPORT/import.json" <<EOF
import json, sys
c = json.load(open(sys.argv[1]))
c["legacy"]["database"] = "$DB_SRC"
c["target"]["database"] = "$DB_TGT"
c["report"] = sys.argv[1].replace("import.json", "import.report.json")
json.dump(c, open(sys.argv[2], "w"), indent=2)
EOF
run_import() {  # $1 = report-suffix
  python3 - "$REPORT/import.json" "$REPORT/config-$1.json" "${REPORT}/import-$1.report.json" <<'EOF'
import json, sys
c = json.load(open(sys.argv[1]))
c["report"] = sys.argv[3]
json.dump(c, open(sys.argv[2], "w"), indent=2)
EOF
  sg deadlock-brain-db -c "'$IMPORT' --config '$REPORT/config-$1.json'" > "$REPORT/import-$1.log" 2>&1
  echo $?
}
changed() { field "$REPORT/import-$1.report.json" "{s['source_id']: s['changed_records']+s['tombstones'] for s in d['sources']}"; }

secret() { python3 "$INFISICAL/export_gpt_secret.py" --secret "$1" 2>/dev/null; }
eval "$(secret BRAIN_PG_READONLY_PASSWORD)" && eval "$(secret BRAIN_PG_INGEST_PASSWORD)" || exit 1
export BRAIN_LEGACY_READ_AUTH=$BRAIN_PG_READONLY_PASSWORD BRAIN_TARGET_INGEST_AUTH=$BRAIN_PG_INGEST_PASSWORD

echo "== T1: Basisimport im Klon"
check "Basisimport" "0" "$(run_import base)"
BASE_DOCS=$(field "$REPORT/import-base.report.json" 'd["release_documents"]')
check "Basisdokumente = 1253" "1253" "$BASE_DOCS"

echo "== T2: Entfernte Dokumente erzeugen Tombstones"
WARDEN_ID=$(sql_src "SELECT id FROM brain_legacy.entities WHERE canonical_name='Warden' AND entity_type='hero' LIMIT 1")
sql_src "DELETE FROM brain_legacy.entity_aliases WHERE entity_id=$WARDEN_ID" > /dev/null
sql_src "DELETE FROM brain_legacy.entities WHERE id=$WARDEN_ID" > /dev/null
sql_src "DELETE FROM brain_legacy.patch_event_enrichments WHERE patch_event_id IN (SELECT id FROM brain_legacy.patch_events WHERE patch_external_id IN (SELECT DISTINCT patch_external_id FROM brain_legacy.patch_events WHERE raw_line ILIKE '%projectical range and speed reduced by 30%'))" > /dev/null
sql_src "DELETE FROM brain_legacy.patch_events WHERE patch_external_id IN (SELECT DISTINCT patch_external_id FROM brain_legacy.patch_events WHERE raw_line ILIKE '%projectical range and speed reduced by 30%')" > /dev/null
check "Import nach Loeschung" "0" "$(run_import removed)"
check "genau 1 Entity-Tombstone + 1 Patch-Tombstone" "{'legacy-entities': 1, 'legacy-patchnotes': 1}" "$(changed removed)"
check "Entity-Tombstone im Kopf" "1" "$(sql "SELECT count(*) FROM brain.source_record_heads WHERE source_id='legacy-entities' AND tombstone")"
check "Patch-Tombstone im Kopf" "1" "$(sql "SELECT count(*) FROM brain.source_record_heads WHERE source_id='legacy-patchnotes' AND tombstone")"
check "Tombstone hat leeren Inhalt" "" "$(sql "SELECT record_json->>'content' FROM brain.source_record_heads WHERE source_id='legacy-entities' AND tombstone")"
check "Tombstone revisioniert auf 2" "2" "$(sql "SELECT max(revision) FROM brain.source_record_heads WHERE source_id='legacy-entities' AND tombstone")"

echo "== T3: Wiederholung nach Tombstone bleibt idempotent"
check "Wiederholung ohne Aenderung" "0" "$(run_import removed2)"
check "Wiederholung ohne neue Records" "{'legacy-entities': 0, 'legacy-patchnotes': 0}" "$(changed removed2)"

echo "== T4: Leerere Quelle failt closed und toetboardet nicht"
HEADS_BEFORE=$(sql "SELECT count(*) FROM brain.source_record_heads")
sql_src "TRUNCATE brain_legacy.entity_aliases, brain_legacy.entities CASCADE" > /dev/null
if run_import empty >/dev/null 2>&1; then rc=0; else rc=$?; fi
check "leerer Import schlaegt fehl" "ne0" "ne$rc"
check "Heads unveraendert" "$HEADS_BEFORE" "$(sql "SELECT count(*) FROM brain.source_record_heads")"
check "keine weiteren Entity-Tombstones" "1" "$(sql "SELECT count(*) FROM brain.source_record_heads WHERE tombstone AND source_id='legacy-entities'")"

echo "== T5: Herkunft eindeutig"
check "Heads eindeutig je (source_id, logical_id)" "0" "$(sql "SELECT count(*) FROM (SELECT source_id, logical_id FROM brain.source_record_heads GROUP BY 1,2 HAVING count(*) > 1) x")"
check "Origin-Identitaet passt zur Quelle (lebende Heads)" "0" "$(sql "SELECT count(*) FROM brain.source_record_heads WHERE NOT tombstone AND (record_json->'metadata'->>'brain.origin')::jsonb->'data'->'identity'->>'source_id' IS DISTINCT FROM source_id")"
check "Origin logical_id passt (lebende Heads)" "0" "$(sql "SELECT count(*) FROM brain.source_record_heads WHERE NOT tombstone AND (record_json->'metadata'->>'brain.origin')::jsonb->'data'->'identity'->>'logical_id' IS DISTINCT FROM logical_id")"

unset BRAIN_LEGACY_READ_AUTH BRAIN_TARGET_INGEST_AUTH
echo "fehlgeschlagen: $fails"
exit $(( fails > 0 ))
