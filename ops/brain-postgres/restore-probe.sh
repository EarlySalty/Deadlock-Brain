#!/usr/bin/env bash
# Prüft ein vertrauenswürdiges Backup ausschließlich gegen seine mitgesicherten
# Fingerprints. Die Wegwerf-Instanz hat einen privaten Socket und keinen TCP-Port.
# restore-probe.sh <Backup> [Bericht] [PostgreSQL-bin] [Scratch-Elternverzeichnis]
# Die ersten beiden Argumente und die bisherigen Produktionsdefaults bleiben gültig.
set -euo pipefail
export LC_ALL=C
umask 077
fail() { printf '%s\n' "$*" >&2; exit 1; }
(( $# >= 1 && $# <= 4 )) || fail 'Aufruf: restore-probe.sh <Backup> [Bericht] [PostgreSQL-bin] [Scratch-Elternverzeichnis]'
# Non-root callers must explicitly supply both test paths. Never silently switch
# an ordinary production invocation to a test cluster or inherited PG* settings.
[[ $EUID -eq 0 || $# -eq 4 ]] || fail 'Als root ausführen oder beide isolierten Testpfade angeben.'
PG_BIN=${3-/usr/lib/postgresql/16/bin}
SCRATCH_PARENT=${4-/var/lib/deadlock-brain}
PG_USER=deadlock-brain-pg
if (( EUID != 0 )); then PG_USER=$(id -un); fi
PROBE_PORT=5447
[[ $PG_BIN == /* && -d $PG_BIN ]] || fail 'PostgreSQL-bin muss ein vorhandener absoluter Pfad sein.'
# pg_ctl passes -o through a shell. Restrict the new scratch-parent argument so
# socket options cannot inject shell syntax; backup/report paths remain arbitrary.
[[ $SCRATCH_PARENT =~ ^/[A-Za-z0-9_./-]+$ && -d $SCRATCH_PARENT && ! -L $SCRATCH_PARENT ]] || fail 'Scratch-Elternverzeichnis muss ein vorhandener absoluter Pfad ohne Sonderzeichen sein.'
[[ $PG_USER =~ ^[a-z_][a-z0-9_-]*$ ]] || fail 'Ungültiger lokaler PostgreSQL-Benutzer.'
cd -P -- .
START_DIR=$PWD
BACKUP=$1
while [[ $BACKUP == */ && $BACKUP != / ]]; do BACKUP=${BACKUP%/}; done
[[ -d $BACKUP && ! -L $BACKUP && $BACKUP != / ]] || fail 'Backup muss ein vorhandenes Verzeichnis ohne Symlink sein.'
[[ $BACKUP == /* ]] || BACKUP="$START_DIR/$BACKUP"
cd -P -- "$BACKUP"
BACKUP=$PWD
# A probe and retention cooperate on the same parent-directory inode. No backup
# may disappear between manifest validation, restore and fingerprint comparison.
exec 9< ..
flock --shared --nonblock 9 || fail 'Für dieses Ziel läuft bereits ein Backup.'
shopt -s nullglob
files=(* .[!.]* ..?*)
dumps=()
declare -A expected=() seen=()
for file in "${files[@]}"; do
  [[ -f $file && ! -L $file ]] || fail "Ungültiger Backup-Eintrag: $file"
  case "$file" in
    SHA256SUMS) continue ;;
    globals.sql|schema_version.txt|fingerprint.sql|fingerprint_version.txt) ;;
    *.fingerprint.txt)
      [[ ${file%.fingerprint.txt} =~ ^[A-Za-z0-9_][A-Za-z0-9_-]{0,62}$ && -f ${file%.fingerprint.txt}.dump ]] || fail 'Verwaister Fingerprint.' ;;
    *.dump)
      [[ ${file%.dump} =~ ^[A-Za-z0-9_][A-Za-z0-9_-]{0,62}$ && -f ${file%.dump}.toc ]] || fail 'Ungültiger Dump oder fehlendes Inhaltsverzeichnis.'
      dumps+=("$file") ;;
    *.toc)
      [[ ${file%.toc} =~ ^[A-Za-z0-9_][A-Za-z0-9_-]{0,62}$ && -f ${file%.toc}.dump ]] || fail 'Verwaistes Inhaltsverzeichnis.' ;;
    *) fail "Unbekannter Backup-Eintrag: $file" ;;
  esac
  expected[$file]=1
done
[[ ${#dumps[@]} -gt 0 && -s SHA256SUMS && -f globals.sql && -f schema_version.txt ]] || fail 'Unvollständiges Backup.'
# Validate exact local filenames, uniqueness and coverage BEFORE sha256sum -c.
# Otherwise a forged/partial manifest could read outside the backup or omit the
# very fingerprint used as evidence of equality.
while IFS= read -r line || [[ -n $line ]]; do
  [[ $line =~ ^[0-9a-f]{64}\ \ ([A-Za-z0-9_.-]+)$ ]] || fail 'Ungültige Prüfsummenzeile.'
  file=${BASH_REMATCH[1]}
  [[ -n ${expected[$file]+present} && -z ${seen[$file]+present} ]] || fail 'Prüfsummen enthalten fremde oder doppelte Dateien.'
  seen[$file]=1
done < SHA256SUMS
(( ${#seen[@]} == ${#expected[@]} )) || fail 'Prüfsummen decken nicht alle Backup-Dateien ab.'
sha256sum --quiet --strict -c SHA256SUMS
if [[ ! -f fingerprint_version.txt || ! -f fingerprint.sql ]]; then
  printf '%s\n' 'RESTORE_UNVERIFIED: Backup ohne Snapshot-Fingerprints; kein belegbares RESTORE_EQUAL. Neues Backup erforderlich.' >&2
  exit 2
fi
cmp -s fingerprint_version.txt <(printf '1\n') || fail 'Nicht unterstützte Fingerprint-Version.'
[[ -s fingerprint.sql ]] || fail 'Leere Fingerprint-Abfrage.'
for dump in "${dumps[@]}"; do
  [[ -s ${dump%.dump}.fingerprint.txt ]] || fail 'Fehlender oder leerer Backup-Fingerprint.'
done

# Resolve NUL-terminated paths to preserve embedded and trailing newlines and to
# reject reports reached through symlinks into this or another completed backup.
IFS= read -r -d '' SCRATCH_PARENT < <(realpath -e -z -- "$SCRATCH_PARENT")
[[ $SCRATCH_PARENT =~ ^/[A-Za-z0-9_./-]+$ ]] || fail 'Unsicherer aufgelöster Scratch-Pfad.'
outside_backups() {
  local path=$1 cursor=$1
  [[ $path != "$BACKUP" && $path != "$BACKUP"/* ]] || return 1
  while [[ $cursor != / ]]; do
    [[ ! ${cursor##*/} =~ ^brain-[0-9]{8}T[0-9]{6}Z$ ]] || return 1
    cursor=${cursor%/*}
    [[ -n $cursor ]] || cursor=/
  done
}
outside_backups "$SCRATCH_PARENT" || fail 'Scratch-Instanz darf nicht in einem Backup liegen.'
if [[ -n ${2-} ]]; then
  REPORT=$2
  while [[ $REPORT == */ && $REPORT != / ]]; do REPORT=${REPORT%/}; done
  [[ $REPORT == /* ]] || REPORT="$START_DIR/$REPORT"
  [[ ! -L $REPORT ]] || fail 'Berichtsverzeichnis darf kein Symlink sein.'
  IFS= read -r -d '' REPORT < <(realpath -m -z -- "$REPORT")
  outside_backups "$REPORT" || fail 'Bericht darf nicht in einem Backup liegen.'
  [[ $REPORT != / && $REPORT != "$SCRATCH_PARENT" && $BACKUP != "$REPORT"/* ]] || fail 'Unsicheres Berichtsverzeichnis.'
  mkdir -p -- "$REPORT"
else
  [[ ! -L $BACKUP/../restore-reports ]] || fail 'Standard-Berichtsverzeichnis darf kein Symlink sein.'
  IFS= read -r -d '' REPORT_PARENT < <(realpath -m -z -- "$BACKUP/../restore-reports")
  outside_backups "$REPORT_PARENT" || fail 'Unsicheres Standard-Berichtsverzeichnis.'
  mkdir -p -- "$REPORT_PARENT"
  REPORT=$(mktemp -d -- "$REPORT_PARENT/${BACKUP##*/}.XXXXXX")
fi
[[ -d $REPORT && ! -L $REPORT ]] || fail 'Ungültiges Berichtsverzeichnis.'
exec 8< "$REPORT"
flock --exclusive --nonblock 8 || fail 'Berichtsverzeichnis wird bereits verwendet.'
# Preserve the optional existing report-directory call, but never follow an
# artifact symlink or recursively chown/delete unrelated caller-owned content.
report_files=(globals.out server.log)
for dump in "${dumps[@]}"; do
  db=${dump%.dump}
  report_files+=("$db.backup.txt" "$db.restored.txt" "$db.restore.out" "$db.diff")
done
for file in "${report_files[@]}"; do
  [[ ! -L $REPORT/$file && ( ! -e $REPORT/$file || -f $REPORT/$file ) ]] || fail 'Unsichere vorhandene Berichtsdatei.'
  if [[ -f $REPORT/$file ]]; then
    [[ $(stat -c %h -- "$REPORT/$file") == 1 ]] || fail 'Berichtsdatei darf keine weiteren Hardlinks haben.'
  fi
done
for file in "${report_files[@]}"; do
  # Replace only this invocation's known artifacts; do not leave stale successful
  # fingerprints behind when a repeated probe fails early during globals restore.
  if [[ -f $REPORT/$file ]]; then rm -- "$REPORT/$file"; fi
done
printf 'RESTORE_REPORT dir=%s\n' "$REPORT"

as_pg() {
  # Do not let the private postgres daemon inherit either directory lock.
  if (( EUID == 0 )); then runuser -u "$PG_USER" -- "$@" 9<&- 8<&-; else "$@" 9<&- 8<&-; fi
}
PROBE=$(mktemp -d -- "$SCRATCH_PARENT/restore-probe.XXXXXX")
# shellcheck disable=SC2317 # Invoked indirectly by the EXIT trap.
cleanup() {
  local status=$?
  trap - EXIT
  if [[ -f $PROBE/data/postmaster.pid ]]; then
    if ! as_pg "$PG_BIN/pg_ctl" -D "$PROBE/data" -m fast -w stop >/dev/null; then
      printf 'Wegwerf-Instanz konnte nicht gestoppt werden; bleibt erhalten: %s\n' "$PROBE" >&2
      exit 1
    fi
  fi
  if [[ -f $PROBE/server.log ]]; then
    if ! cp -- "$PROBE/server.log" "$REPORT/server.log"; then status=1; fi
  fi
  if (( EUID == 0 )); then
    for file in "${report_files[@]}"; do
      if [[ -f $REPORT/$file ]]; then
        if ! chown --no-dereference "$PG_USER:$PG_USER" "$REPORT/$file"; then status=1; fi
      fi
    done
  fi
  [[ $PROBE == "$SCRATCH_PARENT"/restore-probe.* ]] || exit 1
  rm -rf -- "$PROBE"
  if (( status != 0 )); then printf 'RESTORE_FAILED status=%s report=%s\n' "$status" "$REPORT" >&2; fi
  exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
if (( EUID == 0 )); then chown "$PG_USER:$PG_USER" "$PROBE"; fi
as_pg "$PG_BIN/initdb" -D "$PROBE/data" --username="$PG_USER" --auth-local=peer \
  --auth-host=reject --encoding=UTF8 --locale=C.UTF-8 --data-checksums >/dev/null
as_pg "$PG_BIN/pg_ctl" -D "$PROBE/data" -l "$PROBE/server.log" \
  -o "-c listen_addresses='' -k $PROBE -p $PROBE_PORT -c max_connections=10 -c shared_buffers=16MB" -w start >/dev/null
probe() { as_pg "$PG_BIN/psql" -X -q -At -v ON_ERROR_STOP=1 --no-password -U "$PG_USER" -h "$PROBE" -p "$PROBE_PORT" "$@"; }

# initdb has created exactly this bootstrap role. Skip at most ONE exact CREATE
# ROLE statement in pg_dumpall's format. Its ALTER ROLE, grants, other roles and
# any subsequent duplicate CREATE are still executed with ON_ERROR_STOP.
bootstrap_create=$(probe -d postgres -c "SELECT format('CREATE ROLE %I;', current_user)")
skipped=0
: > "$REPORT/globals.out"
while IFS= read -r line || [[ -n $line ]]; do
  if [[ $line == "$bootstrap_create" && $skipped == 0 ]]; then
    printf 'BOOTSTRAP_ROLE_REUSED role=%s\n' "$PG_USER" >> "$REPORT/globals.out"
    skipped=1
  else
    printf '%s\n' "$line"
  fi
done < "$BACKUP/globals.sql" > "$PROBE/globals.sql"
probe -d postgres < "$PROBE/globals.sql" >> "$REPORT/globals.out" 2>&1

status=0
for dump in "${dumps[@]}"; do
  db=${dump%.dump}
  as_pg "$PG_BIN/pg_restore" --create --exit-on-error --no-password -U "$PG_USER" -h "$PROBE" -p "$PROBE_PORT" -d postgres \
    "$BACKUP/$dump" > "$REPORT/$db.restore.out" 2>&1
  cp -- "$BACKUP/$db.fingerprint.txt" "$REPORT/$db.backup.txt"
  probe -d "$db" --single-transaction --file "$BACKUP/fingerprint.sql" > "$REPORT/$db.restored.txt"
  if cmp -s "$REPORT/$db.backup.txt" "$REPORT/$db.restored.txt"; then
    : > "$REPORT/$db.diff"
    printf 'RESTORE_EQUAL db=%s lines=%s\n' "$db" "$(wc -l < "$REPORT/$db.backup.txt")"
  else
    printf 'RESTORE_DIFFERS db=%s\n' "$db" >&2
    if diff -- "$REPORT/$db.backup.txt" "$REPORT/$db.restored.txt" > "$REPORT/$db.diff"; then :; else
      diff_status=$?
      (( diff_status == 1 )) || exit "$diff_status"
    fi
    head -20 -- "$REPORT/$db.diff" >&2
    status=1
  fi
done
exit "$status"
