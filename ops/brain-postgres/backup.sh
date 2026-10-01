#!/usr/bin/env bash
# Eigenes Backup der Brain-Instanz, unabhängig vom DL-Main-Cluster.
# Optional: backup.sh <Ziel> [Anzahl] [PostgreSQL-bin] [Datenbank ...]
# Ohne Argumente gelten die bestehenden Werte der System-Unit.
set -euo pipefail
export LC_ALL=C
PG_BIN=${3-/usr/lib/postgresql/16/bin}
SOCKET=/run/deadlock-brain-postgresql
PORT=5446
TARGET=${1-/var/backups/deadlock-brain/postgresql}
KEEP=${2-14}
# Bound arithmetic before creating files or installing a destructive cleanup trap.
if [[ ! $KEEP =~ ^[1-9][0-9]*$ || ${#KEEP} -gt 9 ]]; then
  printf '%s\n' 'Backup-Anzahl muss eine positive Ganzzahl bis 999999999 sein.' >&2
  exit 1
fi
if [[ $PG_BIN != /* || ! -d $PG_BIN ]]; then
  printf '%s\n' 'PostgreSQL-bin muss ein vorhandener absoluter Pfad sein.' >&2
  exit 1
fi
databases=(brain)
if (( $# > 3 )); then databases=("${@:4}"); fi
declare -A database_names=()
for db in "${databases[@]}"; do
  if [[ ! $db =~ ^[A-Za-z0-9_][A-Za-z0-9_-]{0,62}$ || -n ${database_names[$db]+present} ]]; then
    printf '%s\n' 'Ungültiger oder doppelter Datenbankname.' >&2
    exit 1
  fi
  database_names[$db]=1
done
while [[ $TARGET == */ && $TARGET != / ]]; do TARGET=${TARGET%/}; done
if [[ ! -d $TARGET || -L $TARGET || $TARGET == / ]]; then
  printf '%s\n' 'Backup target must be an existing, non-symlink directory' >&2
  exit 1
fi
# Anchor all mutations to the opened directory. $PWD preserves embedded/trailing newlines;
# command substitution of pwd/realpath would not. No pathname is parsed as text.
[[ $TARGET == /* ]] || TARGET="./$TARGET"
cd -P -- "$TARGET"
TARGET=$PWD
# Lock the opened directory inode, avoiding a mutable lockfile or pathname race.
# Keep ownership through backup creation, publication and the entire rotation.
exec 9< .
if ! flock --exclusive --nonblock 9; then
  printf '%s\n' 'Für dieses Ziel läuft bereits ein Backup oder eine Restore-Prüfung.' >&2
  exit 1
fi
umask 077
stamp=$(date -u +%Y%m%dT%H%M%SZ)
[[ $stamp =~ ^[0-9]{8}T[0-9]{6}Z$ ]]
work="./.brain-$stamp.partial"
complete="./brain-$stamp"
[[ ! -e $complete && ! -L $complete ]]
mkdir -- "$work"
cleanup() {
  local status=$?
  trap - EXIT
  # EOF rolls the exporting transaction back, including after a failed dump or
  # fingerprint. Reap it before releasing the directory lock or deleting files.
  if [[ -n ${snapshot_in-} ]]; then exec {snapshot_in}>&-; fi
  if [[ -n ${snapshot_out-} ]]; then exec {snapshot_out}<&-; fi
  if [[ -n ${snapshot_pid-} ]]; then
    if wait "$snapshot_pid"; then :; else status=1; fi
  fi
  rm -rf -- "$work"
  exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
sql() { "$PG_BIN/psql" -X -q -At -v ON_ERROR_STOP=1 -h "$SOCKET" -p "$PORT" --no-password "$@"; }

# Keep the exact, versioned query with the backup. Restore uses this query, not a
# potentially changed implementation or live database. Like globals/dumps it is
# executable SQL from a trusted backup, not a safe format for untrusted input.
printf '1\n' > "$work/fingerprint_version.txt"
cat > "$work/fingerprint.sql" <<'SQL'
SET search_path = pg_catalog;
SET TIME ZONE 'UTC';
SET DateStyle = 'ISO, YMD';
SET IntervalStyle = 'postgres';
SET extra_float_digits = 1;
SET bytea_output = 'hex';
SELECT 'db|' || current_database() || '|' || pg_get_userbyid(datdba) || '|' || coalesce(datacl::text, '') FROM pg_database WHERE datname = current_database();
SELECT 'schema|' || nspname || '|' || pg_get_userbyid(nspowner) || '|' || coalesce(nspacl::text, '')
FROM pg_namespace WHERE nspname !~ '^(pg_|information_schema)' ORDER BY nspname COLLATE "C";
SELECT 'acl|' || n.nspname || '.' || c.relname || '|' || c.relkind::text || '|' || pg_get_userbyid(c.relowner) || '|' || coalesce(c.relacl::text, '')
FROM pg_class c JOIN pg_namespace n ON n.oid = c.relnamespace
WHERE n.nspname IN ('brain', 'brain_legacy') ORDER BY n.nspname COLLATE "C", c.relname COLLATE "C";
SELECT format('SELECT %L || ''|'' || count(*) || ''|'' || md5(coalesce(string_agg(t::text, E''\n'' ORDER BY t::text COLLATE "C"), '''')) FROM %I.%I t;', 'rows|' || n.nspname || '.' || c.relname, n.nspname, c.relname)
FROM pg_class c JOIN pg_namespace n ON n.oid = c.relnamespace
WHERE n.nspname IN ('brain', 'brain_legacy') AND c.relkind IN ('r', 'p') ORDER BY n.nspname COLLATE "C", c.relname COLLATE "C" \gexec
SELECT 'core|schema_version|' || schema_version || '|' || store_contract FROM brain.core_schema_version;
SELECT 'core|tombstone_revisions|' || count(*) FILTER (WHERE tombstone) || '|' || count(*) FROM brain.source_record_revisions;
SELECT 'core|tombstone_heads|' || count(*) FILTER (WHERE tombstone) || '|' || count(*) FROM brain.source_record_heads;
SELECT 'core|head_visibility|' || coalesce(string_agg(v || '=' || n, ',' ORDER BY v COLLATE "C"), '') FROM (SELECT record_json->>'visibility' AS v, count(*) AS n FROM brain.source_record_heads GROUP BY 1) x;
SELECT 'core|releases|' || coalesce(string_agg(release_id || '@' || knowledge_version, ',' ORDER BY release_id COLLATE "C"), '') FROM brain.corpus_releases_v1;
SELECT 'core|checkpoints|' || coalesce(string_agg(source_id || '#' || generation, ',' ORDER BY source_id COLLATE "C"), '') FROM brain.source_checkpoints_v1;
SELECT 'core|conversation_owners|' || count(*) FROM brain.conversation_owners_v1;
SQL
for db in "${databases[@]}"; do
  # Both readers import this still-open REPEATABLE READ snapshot. pg_dump alone
  # would be consistent, but an independent later fingerprint would not be.
  coproc SNAPSHOT { sql -d "$db"; }
  snapshot_pid=$!
  exporter_in=${SNAPSHOT[1]}
  exporter_out=${SNAPSHOT[0]}
  exec {snapshot_in}>&"$exporter_in"
  exec {snapshot_out}<&"$exporter_out"
  exec {exporter_in}>&-
  exec {exporter_out}<&-
  printf 'BEGIN ISOLATION LEVEL REPEATABLE READ READ ONLY; SELECT pg_export_snapshot();\n' >&"$snapshot_in"
  IFS= read -r snapshot <&"$snapshot_out"
  [[ $snapshot =~ ^[0-9A-Fa-f]+-[0-9A-Fa-f]+-[0-9]+$ ]]
  "$PG_BIN/pg_dump" -h "$SOCKET" -p "$PORT" -d "$db" --snapshot="$snapshot" --create --format=custom --compress=6 \
    --no-password --file "$work/$db.dump"
  sql -d "$db" --single-transaction \
    -c "SET TRANSACTION ISOLATION LEVEL REPEATABLE READ, READ ONLY; SET TRANSACTION SNAPSHOT '$snapshot';" \
    --file "$work/fingerprint.sql" > "$work/$db.fingerprint.txt"
  [[ -s $work/$db.fingerprint.txt ]]
  printf 'ROLLBACK;\n' >&"$snapshot_in"
  exec {snapshot_in}>&-
  unset snapshot_in
  exec {snapshot_out}<&-
  unset snapshot_out
  wait "$snapshot_pid"
  unset snapshot_pid
  "$PG_BIN/pg_restore" --list "$work/$db.dump" > "$work/$db.toc"
done
"$PG_BIN/pg_dumpall" -h "$SOCKET" -p "$PORT" --globals-only --no-role-passwords \
  --no-password > "$work/globals.sql"
# The compatibility marker also comes from the first backed-up database's
# snapshot (brain by default), never from a separate, later live query.
awk -F '|' '$1 == "core" && $2 == "schema_version" { print $3 " " $4 }' \
  "$work/${databases[0]}.fingerprint.txt" > "$work/schema_version.txt"
[[ -s $work/schema_version.txt ]]
( cd "$work" && sha256sum -- *.dump *.toc *.fingerprint.txt fingerprint.sql fingerprint_version.txt globals.sql schema_version.txt > SHA256SUMS )
# Never nest a partial backup in an existing destination (including a symlink).
mv -T --no-clobber -- "$work" "$complete"
[[ ! -e $work && ! -L $work ]]
trap - EXIT INT TERM

shopt -s nullglob
completed_backup() {
  local candidate=$1 file name line dumps=0 fingerprints=0 version=0
  local -A expected=() seen=()
  [[ ${candidate#./} =~ ^brain-[0-9]{8}T[0-9]{6}Z$ && -d $candidate && ! -L $candidate ]] || return 1
  for name in SHA256SUMS globals.sql schema_version.txt; do
    [[ -f "$candidate/$name" && ! -L "$candidate/$name" ]] || return 1
  done
  [[ -s "$candidate/SHA256SUMS" ]] || return 1
  if [[ -f $candidate/fingerprint_version.txt && ! -L $candidate/fingerprint_version.txt ]]; then
    cmp -s "$candidate/fingerprint_version.txt" <(printf '1\n') || return 1
    [[ -s $candidate/fingerprint.sql ]] || return 1
    version=1
  fi
  for file in "$candidate"/* "$candidate"/.[!.]* "$candidate"/..?*; do
    [[ -f $file && ! -L $file ]] || return 1
    name=${file##*/}
    case "$name" in
      SHA256SUMS) continue ;;
      globals.sql|schema_version.txt) ;;
      fingerprint.sql|fingerprint_version.txt) (( version == 1 )) || return 1 ;;
      *.fingerprint.txt)
        (( version == 1 )) || return 1
        [[ ${name%.fingerprint.txt} =~ ^[A-Za-z0-9_][A-Za-z0-9_-]{0,62}$ && -f ${file%.fingerprint.txt}.dump && -s $file ]] || return 1
        fingerprints=$((fingerprints + 1)) ;;
      *.dump)
        [[ ${name%.dump} =~ ^[A-Za-z0-9_][A-Za-z0-9_-]{0,62}$ && -f ${file%.dump}.toc ]] || return 1
        dumps=$((dumps + 1)) ;;
      *.toc)
        [[ ${name%.toc} =~ ^[A-Za-z0-9_][A-Za-z0-9_-]{0,62}$ && -f ${file%.toc}.dump ]] || return 1 ;;
      *) return 1 ;;
    esac
    expected[$name]=1
  done
  (( dumps > 0 && (version == 0 || fingerprints == dumps) )) || return 1
  # Recognize only complete manifests with exact, local artifact names. This
  # checks coverage without repeatedly hashing large dumps during retention.
  while IFS= read -r line || [[ -n $line ]]; do
    [[ $line =~ ^[0-9a-f]{64}\ \ ([A-Za-z0-9_.-]+)$ ]] || return 1
    name=${BASH_REMATCH[1]}
    [[ -n ${expected[$name]+present} && -z ${seen[$name]+present} ]] || return 1
    seen[$name]=1
  done < "$candidate/SHA256SUMS"
  (( ${#seen[@]} == ${#expected[@]} ))
}
# C-locale glob order is timestamp order. Only recognized completed backups count.
backups=()
for candidate in ./brain-*; do
  if completed_backup "$candidate"; then backups+=("$candidate"); fi
done
remove=$((${#backups[@]} - KEEP))
for candidate in "${backups[@]}"; do
  ((remove > 0)) || break
  # Preserve the just-created backup even after a wall-clock correction.
  [[ $candidate != "$complete" ]] || continue
  if completed_backup "$candidate"; then
    rm -rf -- "$candidate"
    remove=$((remove - 1))
  fi
done
printf '%s\n' "$TARGET/brain-$stamp"
