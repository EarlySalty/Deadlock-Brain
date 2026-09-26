#!/usr/bin/env bash
# Kopiert die Brain-eigenen Alttabellen aus DL-Main (DB deadlock, Schema brain) als
# schreibgeschütztes Archivschema brain_legacy in die Brain-Instanz. DL-Main wird nur
# gelesen (ein REPEATABLE-READ-Snapshot); nichts wird dort geändert oder gelöscht.
set -euo pipefail
[[ $EUID -eq 0 ]] || { echo "als root ausführen" >&2; exit 1; }
PG_BIN=/usr/lib/postgresql/16/bin
SOCKET=/run/deadlock-brain-postgresql
PORT=5446
EXCLUDED=(patch_changes feeder_runs plan_items plan_items_verworfen plan_runs)
stamp=$(date -u +%Y%m%dT%H%M%SZ)
OUT=/var/backups/deadlock-brain/postgresql/legacy-import-$stamp
install -d -m 0700 -o deadlock-brain-pg -g deadlock-brain-pg "$OUT"
main() { runuser -u postgres -- "$PG_BIN/psql" -X -q -At -v ON_ERROR_STOP=1 -d deadlock "$@"; }
brain() { runuser -u deadlock-brain-pg -- "$PG_BIN/psql" -X -q -At -v ON_ERROR_STOP=1 -h "$SOCKET" -p "$PORT" "$@"; }
excluded_sql=$(printf "'%s'," "${EXCLUDED[@]}"); excluded_sql=${excluded_sql%,}
hash_query() {
  local schema=$1
  cat <<SQL
SET TIME ZONE 'UTC';
SET extra_float_digits = 1;
SELECT format('SELECT %L || ''|'' || count(*) || ''|'' || md5(coalesce(string_agg(t::text, E''\\n'' ORDER BY t::text), '''')) FROM %I.%I t;', c.relname, '$schema', c.relname)
FROM pg_class c JOIN pg_namespace n ON n.oid = c.relnamespace
WHERE n.nspname = '$schema' AND c.relkind IN ('r','p') AND c.relname NOT IN ($excluded_sql)
ORDER BY c.relname \gexec
SQL
}

coproc SNAP { runuser -u postgres -- "$PG_BIN/psql" -X -q -At -v ON_ERROR_STOP=1 -d deadlock; }
echo "BEGIN ISOLATION LEVEL REPEATABLE READ READ ONLY; SELECT pg_export_snapshot();" >&"${SNAP[1]}"
read -r snapshot <&"${SNAP[0]}"
exclude_args=(); for t in "${EXCLUDED[@]}"; do exclude_args+=(--exclude-table="brain.$t"); done
runuser -u postgres -- "$PG_BIN/pg_dump" -d deadlock --snapshot="$snapshot" --schema=brain \
  "${exclude_args[@]}" --no-owner --no-acl --format=custom > "$OUT/dl-main-brain.dump"
{ hash_query brain; echo "SELECT '__END__';"; } >&"${SNAP[1]}"
: > "$OUT/source-hashes.txt"
while read -r line <&"${SNAP[0]}"; do [[ $line == __END__ ]] && break; echo "$line" >> "$OUT/source-hashes.txt"; done
echo "COMMIT;" >&"${SNAP[1]}"; exec {SNAP[1]}>&-; wait "$SNAP_PID" || true
main -c "SELECT pg_get_viewdef('brain.patch_changes'::regclass, true)" > "$OUT/patch_changes.viewdef.sql"

brain -d postgres -c "DROP DATABASE IF EXISTS brain_legacy_stage" -c "CREATE DATABASE brain_legacy_stage TEMPLATE template0"
runuser -u deadlock-brain-pg -- "$PG_BIN/pg_restore" --list "$OUT/dl-main-brain.dump" > "$OUT/dl-main-brain.toc"
runuser -u deadlock-brain-pg -- "$PG_BIN/pg_restore" --file=- --no-owner --no-acl "$OUT/dl-main-brain.dump" \
  | sed -e 's/public\.gen_random_uuid()/pg_catalog.gen_random_uuid()/g' \
  | runuser -u deadlock-brain-pg -- "$PG_BIN/psql" -X -q -v ON_ERROR_STOP=1 -h "$SOCKET" -p "$PORT" -d brain_legacy_stage >/dev/null
brain -d brain_legacy_stage -c "ALTER SCHEMA brain RENAME TO brain_legacy"
runuser -u deadlock-brain-pg -- "$PG_BIN/pg_dump" -h "$SOCKET" -p "$PORT" -d brain_legacy_stage \
  --schema=brain_legacy --no-owner --no-acl --format=plain \
  | runuser -u deadlock-brain-pg -- "$PG_BIN/psql" -X -q -v ON_ERROR_STOP=1 -h "$SOCKET" -p "$PORT" -U brain_migrate -d brain -1 >/dev/null
brain -d postgres -c "DROP DATABASE brain_legacy_stage"
brain -U brain_migrate -d brain -c "COMMENT ON SCHEMA brain_legacy IS 'Archivkopie der Brain-Alttabellen aus DL-Main ($stamp), nur lesend'"
{ hash_query brain_legacy; } | brain -U brain_migrate -d brain > "$OUT/target-hashes.txt"
chown deadlock-brain-pg:deadlock-brain-pg "$OUT"/*; chmod 0600 "$OUT"/*
if diff -q "$OUT/source-hashes.txt" "$OUT/target-hashes.txt" >/dev/null; then
  echo "HASHES_EQUAL tables=$(wc -l < "$OUT/source-hashes.txt") dir=$OUT"
else
  echo "HASHES_DIFFER dir=$OUT" >&2
  diff "$OUT/source-hashes.txt" "$OUT/target-hashes.txt" >&2 || true
  exit 1
fi
