#!/usr/bin/env bash
# Eigenes Backup der Brain-Instanz, unabhängig vom DL-Main-Cluster.
set -euo pipefail
PG_BIN=/usr/lib/postgresql/16/bin
SOCKET=/run/deadlock-brain-postgresql
PORT=5446
TARGET=${BRAIN_BACKUP_DIR:-/var/backups/deadlock-brain/postgresql}
KEEP=${BRAIN_BACKUP_KEEP:-14}
umask 077
stamp=$(date -u +%Y%m%dT%H%M%SZ)
work="$TARGET/.brain-$stamp.partial"
mkdir -- "$work"
trap 'rm -rf -- "$work"' EXIT
DATABASES=${BRAIN_BACKUP_DATABASES:-brain}
for db in $DATABASES; do
  "$PG_BIN/pg_dump" -h "$SOCKET" -p "$PORT" -d "$db" --create --format=custom --compress=6 \
    --no-password --file "$work/$db.dump"
  "$PG_BIN/pg_restore" --list "$work/$db.dump" > "$work/$db.toc"
done
"$PG_BIN/pg_dumpall" -h "$SOCKET" -p "$PORT" --globals-only --no-role-passwords \
  --no-password > "$work/globals.sql"
"$PG_BIN/psql" -X -A -t -h "$SOCKET" -p "$PORT" -d brain --no-password -c \
  "SELECT schema_version || ' ' || store_contract FROM brain.core_schema_version" > "$work/schema_version.txt"
( cd "$work" && sha256sum -- *.dump *.toc globals.sql schema_version.txt > SHA256SUMS )
mv -- "$work" "$TARGET/brain-$stamp"
trap - EXIT
ls -1d "$TARGET"/brain-*/ 2>/dev/null | sort | head -n -"$KEEP" | xargs -r rm -rf --
echo "$TARGET/brain-$stamp"
