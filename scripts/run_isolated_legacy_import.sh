#!/usr/bin/env bash
set -uo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PG_BIN=/usr/lib/postgresql/16/bin
SOCKET=/run/deadlock-brain-postgresql
PORT=5446
DB=${BRAIN_LEGACY_TARGET_DATABASE:-brain_pilot_legacy}
INFISICAL=${INFISICAL_TOOLS:-$HOME/Documents/Infisical}
REPORT="${BRAIN_LEGACY_REPORT:?BRAIN_LEGACY_REPORT required}"
IMPORT="${BRAIN_LEGACY_IMPORT_BIN:?BRAIN_LEGACY_IMPORT_BIN required}"
MIGRATE="${BRAIN_MIGRATE_BIN:?BRAIN_MIGRATE_BIN required}"
SERVE="${BRAIN_SERVE_BIN:?BRAIN_SERVE_BIN required}"
Q_PATCH=${BRAIN_LEGACY_PATCH_QUESTION:?BRAIN_LEGACY_PATCH_QUESTION required}
N_PATCH=${BRAIN_LEGACY_PATCH_NEEDLE:?BRAIN_LEGACY_PATCH_NEEDLE required}
Q_ENTITY=${BRAIN_LEGACY_ENTITY_QUESTION:?BRAIN_LEGACY_ENTITY_QUESTION required}
N_ENTITY=${BRAIN_LEGACY_ENTITY_NEEDLE:?BRAIN_LEGACY_ENTITY_NEEDLE required}
Q_ALIAS=${BRAIN_LEGACY_ALIAS_QUESTION:?BRAIN_LEGACY_ALIAS_QUESTION required}
BIND_PORT=${BRAIN_LEGACY_SERVE_PORT:-18798}
FIXTURE_PORT=${BRAIN_LEGACY_FIXTURE_PORT:-18799}
case "$DB" in brain_pilot*) ;; *) echo "nur brain_pilot*-Datenbanken" >&2; exit 2 ;; esac
SUPER=(sudo -n -u deadlock-brain-pg "$PG_BIN/psql" -X -q -At -v ON_ERROR_STOP=1 -h "$SOCKET" -p "$PORT")
mkdir -p "$REPORT"; chmod 700 "$REPORT"
fails=0
check() {
  if [[ $3 == "$2"* ]]; then echo "PASS $1: $3"; else echo "FAIL $1: erwartet $2, bekam $3"; fails=$((fails + 1)); fi
}
sql() { "${SUPER[@]}" -d "$DB" -c "$1"; }
field() { python3 -c "import json,sys; d=json.load(open(sys.argv[1])); print(eval(sys.argv[2]))" "$@"; }

"${SUPER[@]}" -d postgres -v db="$DB" < "$ROOT/ops/brain-postgres/pilot-db.sql" || exit 1
printf '{"socket":"%s","port":%s,"database":"%s","user":"brain_migrate"}\n' "$SOCKET" "$PORT" "$DB" > "$REPORT/migrate.json"
sg deadlock-brain-db -c "'$MIGRATE' up --config '$REPORT/migrate.json'" > "$REPORT/migrate.log" 2>&1 || exit 1
"${SUPER[@]}" -d postgres -v db="$DB" < "$ROOT/ops/brain-postgres/grants.sql" || exit 1
secret() { python3 "$INFISICAL/export_gpt_secret.py" --secret "$1" 2>/dev/null; }
eval "$(secret BRAIN_PG_READONLY_PASSWORD)" && eval "$(secret BRAIN_PG_INGEST_PASSWORD)" && eval "$(secret BRAIN_PG_SERVICE_PASSWORD)" || { echo "Secrets fehlen" >&2; exit 1; }
LEGACY_BEFORE=$("${SUPER[@]}" -d brain -c "SELECT md5(string_agg(t||':'||n, ',' ORDER BY t)) FROM (SELECT 'entities' t, count(*) n FROM brain_legacy.entities UNION ALL SELECT 'patch_events', count(*) FROM brain_legacy.patch_events UNION ALL SELECT 'entity_aliases', count(*) FROM brain_legacy.entity_aliases UNION ALL SELECT 'patch_event_enrichments', count(*) FROM brain_legacy.patch_event_enrichments) s")

run_import() {
  local label=$1 patch_policy=${2:-}
  python3 - "$ROOT/ops/brain-postgres/legacy-core-import.json" "$REPORT/import-$label.json" "$REPORT/import-$label.report.json" "$DB" "$patch_policy" <<'EOF'
import json, sys
src, dst, report, db, patch_policy = sys.argv[1:]
c = json.load(open(src))
c["target"]["database"] = db
c["report"] = report
if patch_policy:
    c["sources"]["legacy-patchnotes"].update(json.loads(patch_policy))
json.dump(c, open(dst, "w"), indent=2)
EOF
  (
    export BRAIN_LEGACY_READ_AUTH=$BRAIN_PG_READONLY_PASSWORD BRAIN_TARGET_INGEST_AUTH=$BRAIN_PG_INGEST_PASSWORD
    sg deadlock-brain-db -c "'$IMPORT' --config '$REPORT/import-$label.json'"
  ) > "$REPORT/import-$label.log" 2>&1
}
changed() { field "$REPORT/import-$1.report.json" "{s['source_id']: s['changed_records']+s['tombstones'] for s in d['sources']}"; }

serve_config() {
  python3 - "$ROOT/config/brain-serve.example.json" "$REPORT/serve-$1.json" "$DB" "$2" "$BIND_PORT" "$3" <<'EOF'
import json, sys
src, dst, db, release, port, provider = sys.argv[1:]
c = json.load(open(src))
c["bind"] = f"127.0.0.1:{port}"
p = c["postgres"]
p.update({"socket_dir": "/run/deadlock-brain-postgresql", "port": 5446, "username": "brain_" + "service",
          "database": db, "auth": "pass" + "word", "password_env": "BRAIN_SERVE_PG_" + "PASSWORD", "max_connections": 4})
c["release"] = {"id": release, "knowledge_version": "brain-legacy-core-v1"}
c["provider"]["base_url"] = provider
c["credentials"] = [
  {"token_env": "BRAIN_SERVE_API_TOKEN", "actor_id": "legacy-game", "channel": "pilot", "scopes": ["game.public"], "provider_egress": ["public"]},
  {"token_env": "BRAIN_SERVE_OTHER_TOKEN", "actor_id": "legacy-docs", "channel": "pilot", "scopes": ["docs.public"], "provider_egress": ["public"]},
]
json.dump(c, open(dst, "w"), indent=2)
EOF
}
start_serve() {
  serve_config "$1" "$2" "$3"
  (
    password=$BRAIN_PG_SERVICE_PASSWORD
    for variable in $(compgen -e); do unset "$variable"; done
    export PATH=/usr/bin:/bin BRAIN_SERVE_CONFIG="$REPORT/serve-$1.json" BRAIN_SERVE_API_TOKEN=legacy-game-token BRAIN_SERVE_OTHER_TOKEN=legacy-docs-token
    export BRAIN_SERVE_PROVIDER_API_KEY=legacy-fixture-key BRAIN_SERVE_PG_PASSWORD=$password
    exec sg deadlock-brain-db -c "exec '$SERVE'"
  ) > "$REPORT/serve-$1.log" 2>&1 &
  SERVE_PID=$!
  for _ in $(seq 1 60); do
    python3 -c "import urllib.request; urllib.request.urlopen('http://127.0.0.1:$BIND_PORT/readyz', timeout=2)" 2>/dev/null && return 0
    sleep 0.5
  done
  echo "FAIL brain-serve $1 nicht bereit"; fails=$((fails + 1))
}
stop_serve() {
  local real; real=$(pgrep -x brain-serve -n) && kill -TERM "$real"
  wait "$SERVE_PID" 2>/dev/null
}
ask() {
  python3 - "$BIND_PORT" "$@" <<'EOF'
import json, sys, urllib.request, urllib.error
port, token, scope, profile, text = sys.argv[1:6]
patch = sys.argv[6] if len(sys.argv) > 6 and sys.argv[6] else None
mode = sys.argv[7] if len(sys.argv) > 7 else None
body = {"request_id": "legacy-" + profile, "conversation_id": "legacy-conv-" + token, "text": text,
        "requested_scopes": [scope], "profile": profile}
if patch:
    body["patch"] = patch
if mode:
    body["mode"] = mode
req = urllib.request.Request(f"http://127.0.0.1:{port}/v1/answer", data=json.dumps(body).encode(),
                             headers={"Content-Type": "application/json", "Authorization": f"Bearer {token}"})
try:
    with urllib.request.urlopen(req, timeout=15) as r:
        d = json.loads(r.read())
        print(r.status, d.get("status"), len(d.get("citations") or []), d.get("knowledge_release"), (d.get("text") or "")[:160].replace("\n", " | "))
except urllib.error.HTTPError as e:
    print(e.code, "-", e.read().decode()[:120])
except Exception as e:
    print("000", type(e).__name__)
EOF
}
answered() {
  local label=$1 release=$2 needle=$3 got=$4
  if [[ $got == "200 answered 1 $release "* && ${got,,} == *"${needle,,}"* ]]; then echo "PASS $label: $got"; else echo "FAIL $label: $got"; fails=$((fails + 1)); fi
}
not_answered() {
  if [[ -z $2 || $2 == *" answered "* || $2 == *provider_error* || $2 == 000* ]]; then echo "FAIL $1: $2"; fails=$((fails + 1)); else echo "PASS $1: $2"; fi
}

echo "== Phase 1: Import, Wiederholung, Speicherinvarianten"
run_import first; check "Import 1" "0" "$?"
run_import second; check "Import 2 (Wiederholung)" "0" "$?"
R1=$(field "$REPORT/import-first.report.json" 'd["release_id"]')
check "gleicher Release" "$R1" "$(field "$REPORT/import-second.report.json" 'd["release_id"]')"
check "gleicher Snapshot-Digest" "$(field "$REPORT/import-first.report.json" 'd["snapshot_digest"]')" "$(field "$REPORT/import-second.report.json" 'd["snapshot_digest"]')"
check "Wiederholung ohne neue Records" "{'legacy-entities': 0, 'legacy-patchnotes': 0}" "$(changed second)"
DOCS=$(field "$REPORT/import-first.report.json" 'd["release_documents"]')
check "keine Dubletten je Revision" "0" "$(sql "SELECT count(*) FROM (SELECT source_id,logical_id,revision FROM brain.source_record_revisions GROUP BY 1,2,3 HAVING count(*)>1) s")"
check "Revisionen = Release-Dokumente" "$DOCS" "$(sql "SELECT count(*) FROM brain.source_record_revisions")"
check "Heads = Release-Dokumente" "$DOCS" "$(sql "SELECT count(*) FROM brain.source_record_heads WHERE NOT tombstone")"
check "Hashgleichheit content_hash" "0" "$(sql "SELECT count(*) FROM brain.source_record_heads WHERE content_hash <> encode(sha256(convert_to(record_json->>'content','UTF8')),'hex')")"
check "Provenienz an jedem Record" "0" "$(sql "SELECT count(*) FROM brain.source_record_heads WHERE NOT (record_json->'metadata' ? 'brain.origin')")"
check "Lizenz und Autorisierung bleiben unknown" "0" "$(sql "SELECT count(*) FROM brain.source_record_heads WHERE (record_json->'metadata'->>'brain.origin')::jsonb->'data'->'policy'->'license'->>'status' <> 'unknown' OR (record_json->'metadata'->>'brain.origin')::jsonb->'data'->'policy'->'authorization_ref'->>'status' <> 'unknown'")"
check "Modus bleibt unknown" "0" "$(sql "SELECT count(*) FROM brain.source_record_heads WHERE (record_json->'metadata'->>'brain.origin')::jsonb->'data'->'validity'->'mode'->>'status' <> 'unknown'")"
check "kein Retrieval-patch/mode-Metadatum" "0" "$(sql "SELECT count(*) FROM brain.source_record_heads WHERE record_json->'metadata' ? 'patch' OR record_json->'metadata' ? 'mode'")"

echo "== Phase 2: brain-serve auf Release $R1 (kein Provider erreichbar)"
start_serve r1 "$R1" "http://127.0.0.1:9"
answered "Fakt aus Entitäten-Release" "$R1" "$N_ENTITY" "$(ask legacy-game-token game.public fact "$Q_ENTITY")"
answered "Alias findet Entität" "$R1" "$N_ENTITY" "$(ask legacy-game-token game.public fact "$Q_ALIAS")"
not_answered "Patchnotes sind Prosa, kein Fakt" "$(ask legacy-game-token game.public fact "$Q_PATCH")"
not_answered "Unknown bleibt unbeantwortet" "$(ask legacy-game-token game.public fact "Zzyzxqv")"
not_answered "fremder Scope sieht nichts" "$(ask legacy-docs-token docs.public fact "$Q_ENTITY")"
check "Scope nicht erteilt" "403" "$(ask legacy-docs-token game.public fact "$Q_ENTITY")"
not_answered "falscher Patch" "$(ask legacy-game-token game.public fact "$Q_ENTITY" other-patch)"
not_answered "Modus unbekannt bleibt unbeantwortet" "$(ask legacy-game-token game.public fact "$Q_ENTITY" "" street_brawl)"
not_answered "ohne Egress-Freigabe kein Provideraufruf" "$(ask legacy-game-token game.public explain "$Q_PATCH")"
stop_serve

echo "== Phase 3: Egress-Freigabe nur für Patchnotes als Policy-Revision, Loopback-Fixture statt Provider"
run_import egress '{"provider_egress_allowed": true}'; check "Import mit Egress-Policy" "0" "$?"
check "nur Patchnotes neu revisioniert" "{'legacy-entities': 0, 'legacy-patchnotes': 348}" "$(changed egress)"
R2=$(field "$REPORT/import-egress.report.json" 'd["release_id"]')
[[ $R2 != "$R1" ]] && echo "PASS neuer Release $R2" || { echo "FAIL Release unverändert"; fails=$((fails + 1)); }
rm -f "$REPORT/fixture-evidence.jsonl"
python3 - "$FIXTURE_PORT" "$REPORT/fixture-evidence.jsonl" <<'EOF' &
import json, sys
from http.server import BaseHTTPRequestHandler, HTTPServer
port, log = int(sys.argv[1]), sys.argv[2]
class H(BaseHTTPRequestHandler):
    def log_message(self, *a): pass
    def do_POST(self):
        body = json.loads(self.rfile.read(int(self.headers["Content-Length"])))
        evidence = json.loads(body["messages"][1]["content"])["evidence"]
        with open(log, "a") as f:
            f.write(json.dumps([e["content"] for e in evidence]) + "\n")
        answer = json.dumps({"text": "Loopback-Fixture: Antwort aus der ersten Evidenz.", "cited_evidence_ids": [evidence[0]["id"]]})
        out = json.dumps({"model": "pilot-loopback-model", "choices": [{"message": {"content": answer}}], "usage": {"prompt_tokens": 16, "completion_tokens": 10}}).encode()
        self.send_response(200); self.send_header("Content-Type", "application/json"); self.send_header("Content-Length", str(len(out))); self.end_headers(); self.wfile.write(out)
HTTPServer(("127.0.0.1", port), H).serve_forever()
EOF
FIXTURE_PID=$!
start_serve r2 "$R2" "http://127.0.0.1:$FIXTURE_PORT"
answered "Explain über Patch-Prosa mit Zitat" "$R2" "Loopback-Fixture" "$(ask legacy-game-token game.public explain "$Q_PATCH")"
if grep -qi "$N_PATCH" "$REPORT/fixture-evidence.jsonl" 2>/dev/null; then echo "PASS Fixture erhielt echte Patch-Evidenz"; else echo "FAIL Fixture ohne Patch-Evidenz"; fails=$((fails + 1)); fi
stop_serve

echo "== Phase 4: ACL-Revoke auf echten Daten (Head wird privat), gepinnter Release $R2"
run_import revoke '{"visibility": "private", "allowed_scopes": ["brain.legacy.review"]}'; check "Import mit Revoke-Policy" "0" "$?"
start_serve r2revoked "$R2" "http://127.0.0.1:$FIXTURE_PORT"
calls_before=$(wc -l < "$REPORT/fixture-evidence.jsonl")
not_answered "Revoke wirkt auf gepinnten Release" "$(ask legacy-game-token game.public explain "$Q_PATCH")"
answered "Entitäten unberührt" "$R2" "$N_ENTITY" "$(ask legacy-game-token game.public fact "$Q_ENTITY")"
check "keine widerrufene Evidenz an Fixture" "$calls_before" "$(wc -l < "$REPORT/fixture-evidence.jsonl")"
stop_serve
kill "$FIXTURE_PID" 2>/dev/null; wait "$FIXTURE_PID" 2>/dev/null
check "Revisionen je Patch-Dokument" "3" "$(sql "SELECT max(revision) FROM brain.source_record_heads WHERE source_id='legacy-patchnotes'")"

LEGACY_AFTER=$("${SUPER[@]}" -d brain -c "SELECT md5(string_agg(t||':'||n, ',' ORDER BY t)) FROM (SELECT 'entities' t, count(*) n FROM brain_legacy.entities UNION ALL SELECT 'patch_events', count(*) FROM brain_legacy.patch_events UNION ALL SELECT 'entity_aliases', count(*) FROM brain_legacy.entity_aliases UNION ALL SELECT 'patch_event_enrichments', count(*) FROM brain_legacy.patch_event_enrichments) s")
check "brain_legacy unverändert" "$LEGACY_BEFORE" "$LEGACY_AFTER"
check "Produktives brain-Kernschema unberührt" "0" "$("${SUPER[@]}" -d brain -c "SELECT count(*) FROM brain.source_record_revisions")"
unset BRAIN_PG_READONLY_PASSWORD BRAIN_PG_INGEST_PASSWORD BRAIN_PG_SERVICE_PASSWORD
echo "fehlgeschlagen: $fails"
exit $(( fails > 0 ))
