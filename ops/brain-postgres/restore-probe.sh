#!/usr/bin/env bash
# Stellt ein Brain-Backup in einer frischen, temporären Wegwerf-Instanz (eigener Socket,
# Port 5447) wieder her und vergleicht Inhalte, Hashes, ACLs und Kernzustand mit der
# laufenden Brain-Instanz. Weder DL-Main noch die laufende Brain-Instanz werden verändert.
set -euo pipefail
[[ $EUID -eq 0 ]] || { echo "als root ausführen" >&2; exit 1; }
BACKUP=${1:?Backup-Verzeichnis}
PG_BIN=/usr/lib/postgresql/16/bin
LIVE_SOCKET=/run/deadlock-brain-postgresql
LIVE_PORT=5446
PROBE_PORT=5447
REPORT=${2:-$BACKUP/restore-probe}
as_pg() { runuser -u deadlock-brain-pg -- "$@"; }
( cd "$BACKUP" && sha256sum --quiet -c SHA256SUMS )
PROBE=$(mktemp -d /var/lib/deadlock-brain/restore-probe.XXXXXX)
chown deadlock-brain-pg:deadlock-brain-pg "$PROBE"; chmod 0700 "$PROBE"
cleanup() {
  as_pg "$PG_BIN/pg_ctl" -D "$PROBE/data" -m fast -w stop >/dev/null 2>&1 || true
  [[ $PROBE == /var/lib/deadlock-brain/restore-probe.* ]] && rm -rf -- "$PROBE"
}
trap cleanup EXIT
as_pg "$PG_BIN/initdb" -D "$PROBE/data" --username=deadlock-brain-pg --auth-local=peer \
  --auth-host=reject --encoding=UTF8 --locale=C.UTF-8 --data-checksums >/dev/null
as_pg "$PG_BIN/pg_ctl" -D "$PROBE/data" -l "$PROBE/server.log" \
  -o "-c listen_addresses='' -k $PROBE -p $PROBE_PORT -c max_connections=10" -w start >/dev/null
probe() { as_pg "$PG_BIN/psql" -X -q -At -h "$PROBE" -p "$PROBE_PORT" "$@"; }
live() { as_pg "$PG_BIN/psql" -X -q -At -h "$LIVE_SOCKET" -p "$LIVE_PORT" "$@"; }
probe -d postgres < "$BACKUP/globals.sql" > "$PROBE/globals.out" 2>&1 || true
install -d -m 0700 -o deadlock-brain-pg -g deadlock-brain-pg "$REPORT"
fingerprint() {
  cat <<'SQL'
SET TIME ZONE 'UTC';
SET extra_float_digits = 1;
SELECT 'db|' || current_database() || '|' || pg_get_userbyid(datdba) || '|' || coalesce(datacl::text, '') FROM pg_database WHERE datname = current_database();
SELECT 'schema|' || nspname || '|' || pg_get_userbyid(nspowner) || '|' || coalesce(nspacl::text, '')
FROM pg_namespace WHERE nspname !~ '^(pg_|information_schema)' ORDER BY nspname;
SELECT 'acl|' || n.nspname || '.' || c.relname || '|' || c.relkind::text || '|' || pg_get_userbyid(c.relowner) || '|' || coalesce(c.relacl::text, '')
FROM pg_class c JOIN pg_namespace n ON n.oid = c.relnamespace
WHERE n.nspname IN ('brain', 'brain_legacy') ORDER BY 1;
SELECT format('SELECT %L || ''|'' || count(*) || ''|'' || md5(coalesce(string_agg(t::text, E''\n'' ORDER BY t::text), '''')) FROM %I.%I t;', 'rows|' || n.nspname || '.' || c.relname, n.nspname, c.relname)
FROM pg_class c JOIN pg_namespace n ON n.oid = c.relnamespace
WHERE n.nspname IN ('brain', 'brain_legacy') AND c.relkind IN ('r', 'p') ORDER BY n.nspname, c.relname \gexec
SELECT 'core|schema_version|' || schema_version || '|' || store_contract FROM brain.core_schema_version;
SELECT 'core|tombstone_revisions|' || count(*) FILTER (WHERE tombstone) || '|' || count(*) FROM brain.source_record_revisions;
SELECT 'core|tombstone_heads|' || count(*) FILTER (WHERE tombstone) || '|' || count(*) FROM brain.source_record_heads;
SELECT 'core|head_visibility|' || coalesce(string_agg(v || '=' || n, ',' ORDER BY v), '') FROM (SELECT record_json->>'visibility' AS v, count(*) AS n FROM brain.source_record_heads GROUP BY 1) x;
SELECT 'core|releases|' || coalesce(string_agg(release_id || '@' || knowledge_version, ',' ORDER BY release_id), '') FROM brain.corpus_releases_v1;
SELECT 'core|checkpoints|' || coalesce(string_agg(source_id || '#' || generation, ',' ORDER BY source_id), '') FROM brain.source_checkpoints_v1;
SELECT 'core|conversation_owners|' || count(*) FROM brain.conversation_owners_v1;
SQL
}
status=0
for dump in "$BACKUP"/*.dump; do
  db=$(basename "$dump" .dump)
  as_pg "$PG_BIN/pg_restore" --create --exit-on-error -h "$PROBE" -p "$PROBE_PORT" -d postgres "$dump"
  fingerprint | live -d "$db" > "$REPORT/$db.live.txt"
  fingerprint | probe -d "$db" > "$REPORT/$db.restored.txt"
  if cmp -s "$REPORT/$db.live.txt" "$REPORT/$db.restored.txt"; then
    echo "RESTORE_EQUAL db=$db lines=$(wc -l < "$REPORT/$db.live.txt")"
  else
    echo "RESTORE_DIFFERS db=$db" >&2
    diff "$REPORT/$db.live.txt" "$REPORT/$db.restored.txt" | head -20 >&2 || true
    status=1
  fi
done
chown -R deadlock-brain-pg:deadlock-brain-pg "$REPORT"
exit "$status"
