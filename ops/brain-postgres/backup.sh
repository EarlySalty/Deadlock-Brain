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
for db in "${databases[@]}"; do
  if [[ ! $db =~ ^[A-Za-z0-9_][A-Za-z0-9_-]{0,62}$ ]]; then
    printf '%s\n' 'Ungültiger Datenbankname.' >&2
    exit 1
  fi
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
  printf '%s\n' 'Für dieses Ziel läuft bereits ein Backup.' >&2
  exit 1
fi
umask 077
stamp=$(date -u +%Y%m%dT%H%M%SZ)
[[ $stamp =~ ^[0-9]{8}T[0-9]{6}Z$ ]]
work="./.brain-$stamp.partial"
complete="./brain-$stamp"
[[ ! -e $complete && ! -L $complete ]]
mkdir -- "$work"
trap 'rm -rf -- "$work"' EXIT
for db in "${databases[@]}"; do
  "$PG_BIN/pg_dump" -h "$SOCKET" -p "$PORT" -d "$db" --create --format=custom --compress=6 \
    --no-password --file "$work/$db.dump"
  "$PG_BIN/pg_restore" --list "$work/$db.dump" > "$work/$db.toc"
done
"$PG_BIN/pg_dumpall" -h "$SOCKET" -p "$PORT" --globals-only --no-role-passwords \
  --no-password > "$work/globals.sql"
"$PG_BIN/psql" -X -A -t -v ON_ERROR_STOP=1 -h "$SOCKET" -p "$PORT" -d brain --no-password -c \
  "SELECT schema_version || ' ' || store_contract FROM brain.core_schema_version" > "$work/schema_version.txt"
( cd "$work" && sha256sum -- *.dump *.toc globals.sql schema_version.txt > SHA256SUMS )
# Never nest a partial backup in an existing destination (including a symlink).
mv -T --no-clobber -- "$work" "$complete"
[[ ! -e $work && ! -L $work ]]
trap - EXIT

shopt -s nullglob
completed_backup() {
  local candidate=$1 file name dumps=0
  [[ ${candidate#./} =~ ^brain-[0-9]{8}T[0-9]{6}Z$ && -d $candidate && ! -L $candidate ]] || return 1
  for name in SHA256SUMS globals.sql schema_version.txt; do
    [[ -f "$candidate/$name" && ! -L "$candidate/$name" ]] || return 1
  done
  [[ -s "$candidate/SHA256SUMS" ]] || return 1
  for file in "$candidate"/* "$candidate"/.[!.]* "$candidate"/..?*; do
    [[ -f $file && ! -L $file ]] || return 1
    name=${file##*/}
    case "$name" in
      SHA256SUMS|globals.sql|schema_version.txt) ;;
      *.dump)
        [[ ${name%.dump} =~ ^[A-Za-z0-9_][A-Za-z0-9_-]{0,62}$ && -f ${file%.dump}.toc ]] || return 1
        dumps=$((dumps + 1)) ;;
      *.toc)
        [[ ${name%.toc} =~ ^[A-Za-z0-9_][A-Za-z0-9_-]{0,62}$ && -f ${file%.toc}.dump ]] || return 1 ;;
      *) return 1 ;;
    esac
  done
  ((dumps > 0))
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
