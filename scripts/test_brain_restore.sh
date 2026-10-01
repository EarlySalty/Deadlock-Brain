#!/usr/bin/env bash
# Real PostgreSQL 16, private Unix sockets, synthetic data, no root/live defaults.
set -euo pipefail
umask 077
script_dir=$(dirname -- "${BASH_SOURCE[0]}")
[[ $script_dir == /* ]] || script_dir="./$script_dir"
repo=$(cd -P -- "$script_dir/.." && pwd)
cases=(missing_table globals_error bootstrap bootstrap_duplicate report_retention later_writes content_difference snapshot_concurrency restore_sql_error legacy checksum report_paths source_offline manifest_coverage multiple_databases special_paths)
if (( $# == 0 )); then
  status=0
  for test_case in "${cases[@]}"; do
    if bash "$repo/scripts/test_brain_restore.sh" "$test_case"; then :; else status=1; fi
  done
  exit "$status"
fi
[[ $# == 1 && " ${cases[*]} " == *" $1 "* ]] || { echo 'Unknown test case' >&2; exit 2; }
(( EUID != 0 )) || { echo 'Run as an unprivileged user; NEVER use root/live defaults.' >&2; exit 2; }
PG_BIN=/usr/lib/postgresql/16/bin
[[ $("$PG_BIN/pg_config" --version) == 'PostgreSQL 16.'* ]] || { echo 'PostgreSQL 16 required' >&2; exit 2; }
root=$(mktemp -d /tmp/brain-restore-test.XXXXXX)
source_pg="$root/source"
bin="$root/bin"
mkdir -- "$bin" "$root/backups" "$root/probes" "$root/reports"
pg_user=$(id -un)
clean=(env -i "PATH=$bin:/usr/bin:/bin" "HOME=$root" "LC_ALL=C")
fail() { printf 'FAIL: %s: %s\n' "$1" "${2:-assertion failed}" >&2; exit 1; }
cleanup() {
  local status=$? data
  trap - EXIT
  if [[ -n ${backup_pid-} ]]; then
    printf 'continue\ncontinue\n' >&7
    if wait "$backup_pid"; then :; else status=1; fi
  fi
  shopt -s nullglob
  for data in "$source_pg" "$root"/probes/restore-probe.*/data; do
    if [[ -f $data/postmaster.pid ]]; then
      if ! "${clean[@]}" "$PG_BIN/pg_ctl" -D "$data" -m fast -w stop > "$root/stop.log" 2>&1; then
        echo "FAIL: private cluster did not stop: $data" >&2
        exit 1
      fi
    fi
  done
  if (( status == 0 )); then
    rm -rf -- "$root"
  else
    printf 'Private test logs (clusters stopped): %s\n' "$root" >&2
    for data in backup.err probe.err; do
      [[ ! -f $root/$data ]] || tail -25 -- "$root/$data" >&2
    done
  fi
  exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

# All clients are real PG16 binaries. The existing PG-bin argument routes the
# fixed backup endpoint to this private fixture. No new environment knobs.
# Unexpected sockets are rejected rather than ever reaching a production socket.
{
  printf '#!/usr/bin/env bash\nset -euo pipefail\n'
  printf 'root=%q\nreal=%q\nuser=%q\n' "$root" "$PG_BIN" "$pg_user"
  cat <<'WRAPPER'
name=${0##*/}
args=()
host= port= db=
while (($#)); do
  case "$1" in
    -h)
      host=$2
      if [[ $host == /run/deadlock-brain-postgresql ]]; then host="$root/source"; fi
      [[ $host == "$root/source" || $host == "$root/probes"/restore-probe.* ]] || { echo 'Non-private socket rejected' >&2; exit 90; }
      args+=(-h "$host"); shift 2 ;;
    -p)
      port=$2
      [[ $port != 5446 ]] || port=55448
      args+=(-p "$port"); shift 2 ;;
    -d) db=$2; args+=("$1" "$2"); shift 2 ;;
    *) args+=("$1"); shift ;;
  esac
done
pause() {
  printf '%s\n' "$1" > "$root/ready"
  read -r _ < "$root/release"
}
if [[ $name == pg_dump && -f $root/pause-dump ]]; then pause before; fi
"$real/$name" "${args[@]}"
if [[ $name == pg_dump && -f $root/pause-dump ]]; then pause after; fi
if [[ $name == pg_restore && $db == postgres && -f $root/change-restored ]]; then
  [[ $host == "$root/probes"/restore-probe.* ]] || exit 91
  "$real/psql" -X -qAt -v ON_ERROR_STOP=1 --no-password -U "$user" -h "$host" -p "$port" -d brain \
    -c "UPDATE brain_legacy.facts SET payload='changed only in restore' WHERE id=1"
fi
WRAPPER
} > "$bin/client"
chmod +x "$bin/client"
for name in pg_dump pg_dumpall pg_restore psql; do ln -s client "$bin/$name"; done
for name in initdb pg_ctl; do ln -s "$PG_BIN/$name" "$bin/$name"; done
cat > "$bin/date" <<'WRAPPER'
#!/usr/bin/env bash
cat "${0%/*}/stamp"
WRAPPER
chmod +x "$bin/date"
stamp_number=0
make_backup() {
  local target=${1:-$root/backups} keep=${2:-14} status
  stamp_number=$((stamp_number + 1))
  printf -v stamp '20300101T00%02d00Z' "$stamp_number"
  printf '%s\n' "$stamp" > "$bin/stamp"
  BACKUP="$target/brain-$stamp"
  if "${clean[@]}" bash "$repo/ops/brain-postgres/backup.sh" "$target" "$keep" "$bin" > "$root/backup.out" 2> "$root/backup.err"; then status=0; else status=$?; fi
  return "$status"
}
restore() {
  "${clean[@]}" bash "$repo/ops/brain-postgres/restore-probe.sh" "$BACKUP" "${1-}" "$bin" "$root/probes" > "$root/probe.out" 2> "$root/probe.err"
}
source_sql() {
  "${clean[@]}" "$PG_BIN/psql" -X -qAt -v ON_ERROR_STOP=1 --no-password -U "$pg_user" -h "$source_pg" -p 55448 -d brain "$@"
}
reseal() {
  (cd -- "$BACKUP"; shopt -s nullglob; files=(*); keep=(); for file in "${files[@]}"; do [[ $file == SHA256SUMS ]] || keep+=("$file"); done; sha256sum -- "${keep[@]}" > SHA256SUMS)
}
assert_no_equal() { if grep -q RESTORE_EQUAL "$root/probe.out"; then fail "$1" 'unproven RESTORE_EQUAL'; fi; }

"${clean[@]}" "$PG_BIN/initdb" -D "$source_pg" --username="$pg_user" --auth-local=peer --auth-host=reject --encoding=UTF8 --locale=C.UTF-8 > "$root/initdb.log"
"${clean[@]}" "$PG_BIN/pg_ctl" -D "$source_pg" -l "$root/source.log" -o "-c listen_addresses='' -k $source_pg -p 55448 -c max_connections=12 -c shared_buffers=16MB" -w start > "$root/start.log"
"${clean[@]}" "$PG_BIN/psql" -X -qAt -v ON_ERROR_STOP=1 --no-password -U "$pg_user" -h "$source_pg" -p 55448 -d postgres > "$root/fixture.log" <<'SQL'
CREATE ROLE fixture_reader NOLOGIN;
CREATE DATABASE brain;
\connect brain
CREATE SCHEMA brain;
CREATE SCHEMA brain_legacy;
CREATE TABLE brain.core_schema_version(schema_version integer, store_contract text);
INSERT INTO brain.core_schema_version VALUES (2, 'brain.store.v2');
CREATE TABLE brain.source_record_revisions(id text, tombstone boolean);
INSERT INTO brain.source_record_revisions VALUES ('old', true), ('new', false);
CREATE TABLE brain.source_record_heads(id text, tombstone boolean, record_json jsonb);
INSERT INTO brain.source_record_heads VALUES ('new', false, '{"visibility":"private"}');
CREATE TABLE brain.corpus_releases_v1(release_id text, knowledge_version text);
INSERT INTO brain.corpus_releases_v1 VALUES ('release-1', 'knowledge-1');
CREATE TABLE brain.source_checkpoints_v1(source_id text, generation bigint);
INSERT INTO brain.source_checkpoints_v1 VALUES ('source-1', 1);
CREATE TABLE brain.conversation_owners_v1(conversation_id text, owner_id text);
INSERT INTO brain.conversation_owners_v1 VALUES ('conversation-1', 'owner-1');
CREATE TABLE brain_legacy.facts(id integer PRIMARY KEY, payload text);
INSERT INTO brain_legacy.facts VALUES (1, 'original');
GRANT USAGE ON SCHEMA brain_legacy TO fixture_reader;
GRANT SELECT ON brain_legacy.facts TO fixture_reader;
SQL

case "$1" in
  missing_table)
    make_backup
    old=$BACKUP
    source_sql -c 'DROP TABLE brain.conversation_owners_v1'
    if make_backup "$root/backups" 1; then fail "$1" 'backup accepted a missing fingerprint table'; fi
    [[ -f $old/SHA256SUMS && ! -e $BACKUP ]] || fail "$1" 'failed fingerprint published or rotated a backup'
    ;;
  globals_error)
    make_backup
    printf '\nCREATE ROLE unexpected_duplicate;\nCREATE ROLE unexpected_duplicate;\n' >> "$BACKUP/globals.sql"
    reseal
    if restore "$root/reports/globals"; then fail "$1" 'unexpected globals error was ignored'; fi
    assert_no_equal "$1"
    ;;
  bootstrap)
    source_sql -c "ALTER ROLE \"$pg_user\" CONNECTION LIMIT 11"
    make_backup
    restore "$root/reports/bootstrap"
    grep -q 'RESTORE_EQUAL db=brain' "$root/probe.out" || fail "$1" 'valid bootstrap restore failed'
    grep -q BOOTSTRAP_ROLE_REUSED "$root/reports/bootstrap/globals.out" || fail "$1" 'bootstrap conflict was not handled explicitly'
    if grep -q ERROR "$root/reports/bootstrap/globals.out"; then fail "$1" 'globals SQL errors were suppressed'; fi
    ;;
  bootstrap_duplicate)
    make_backup
    source_sql -c "SELECT format('CREATE ROLE %I;', current_user)" >> "$BACKUP/globals.sql"
    reseal
    if restore "$root/reports/duplicate"; then fail "$1" 'a second bootstrap CREATE was ignored'; fi
    assert_no_equal "$1"
    ;;
  report_retention)
    make_backup
    old=$BACKUP
    restore
    make_backup "$root/backups" 1
    [[ ! -e $old ]] || fail "$1" 'restore report prevented retention of the old backup'
    [[ -f $BACKUP/SHA256SUMS ]] || fail "$1" 'new backup missing'
    find "$root/backups/restore-reports" -name brain.restored.txt -print -quit | grep -q . || fail "$1" 'report missing outside completed backups'
    ;;
  later_writes)
    make_backup
    source_sql -c "UPDATE brain_legacy.facts SET payload='written after backup'; INSERT INTO brain.conversation_owners_v1 VALUES ('later', 'owner-2')"
    restore "$root/reports/later"
    grep -q 'RESTORE_EQUAL db=brain' "$root/probe.out" || fail "$1" 'restore was compared to later live state'
    cmp "$BACKUP/brain.fingerprint.txt" "$root/reports/later/brain.restored.txt"
    ;;
  content_difference)
    make_backup
    touch "$root/change-restored"
    if restore "$root/reports/diff"; then fail "$1" 'changed restored content accepted'; fi
    assert_no_equal "$1"
    grep -q 'RESTORE_DIFFERS db=brain' "$root/probe.err" || fail "$1" 'missing content difference diagnostic'
    ;;
  snapshot_concurrency)
    mkfifo "$root/ready" "$root/release"
    exec 8<> "$root/ready"
    exec 7<> "$root/release"
    touch "$root/pause-dump"
    # The wrapper pauses just before pg_dump imports the exported snapshot and
    # again after the dump, before the fingerprint is read. Neither race is timed.
    make_backup &
    backup_pid=$!
    read -r -t 15 stage <&8
    [[ $stage == before ]] || fail "$1" 'no pre-dump barrier'
    source_sql -c "UPDATE brain_legacy.facts SET payload='before dump'; INSERT INTO brain_legacy.facts VALUES (2, 'new before dump')"
    printf 'continue\n' >&7
    read -r -t 15 stage <&8
    [[ $stage == after ]] || fail "$1" 'no post-dump barrier'
    source_sql -c "UPDATE brain_legacy.facts SET payload='after dump'; INSERT INTO brain_legacy.facts VALUES (3, 'new after dump')"
    printf 'continue\n' >&7
    wait "$backup_pid"
    unset backup_pid
    BACKUP="$root/backups/brain-$(cat "$bin/stamp")"
    grep -q '^rows|brain_legacy.facts|1|' "$BACKUP/brain.fingerprint.txt" || fail "$1" 'fingerprint did not retain the exported snapshot'
    restore "$root/reports/snapshot"
    grep -q 'RESTORE_EQUAL db=brain' "$root/probe.out" || fail "$1" 'dump and fingerprint diverged under concurrent writes'
    ;;
  restore_sql_error)
    make_backup
    printf '\nSELECT * FROM brain.deliberately_missing;\n' >> "$BACKUP/fingerprint.sql"
    reseal
    if restore "$root/reports/sql"; then fail "$1" 'fingerprint SQL error was ignored'; else status=$?; fi
    [[ $status == 3 ]] || fail "$1" "expected psql ON_ERROR_STOP exit 3, got $status"
    assert_no_equal "$1"
    ;;
  legacy)
    make_backup
    rm -f -- "$BACKUP/fingerprint.sql" "$BACKUP/fingerprint_version.txt" "$BACKUP"/*.fingerprint.txt
    reseal
    if restore "$root/reports/legacy"; then fail "$1" 'legacy backup received unproven equality'; else status=$?; fi
    [[ $status == 2 ]] || fail "$1" "expected explicit unverified status 2, got $status"
    grep -q RESTORE_UNVERIFIED "$root/probe.err" || fail "$1" 'missing legacy diagnostic'
    assert_no_equal "$1"
    ;;
  checksum)
    make_backup
    printf 'tampered\n' >> "$BACKUP/brain.fingerprint.txt"
    if restore "$root/reports/checksum"; then fail "$1" 'unchecksummed fingerprint was accepted'; fi
    assert_no_equal "$1"
    ;;
  report_paths)
    make_backup
    if restore "$BACKUP/nested"; then fail "$1" 'report allowed inside backup'; fi
    [[ ! -e $BACKUP/nested ]] || fail "$1" 'rejected report changed backup'
    ln -s "$BACKUP" "$root/backup-link"
    if restore "$root/backup-link/nested"; then fail "$1" 'symlink report escaped path validation'; fi
    [[ ! -e $BACKUP/nested ]] || fail "$1" 'symlink report changed backup'
    mkdir -p "$root/cdpath/work" "$root/cdpath/elsewhere/report"
    printf protected > "$root/cdpath/elsewhere/report/sentinel"
    (
      cd -- "$root/cdpath/work"
      env -i "PATH=$bin:/usr/bin:/bin" "HOME=$root" "LC_ALL=C" "CDPATH=$root/cdpath/elsewhere" \
        bash "$repo/ops/brain-postgres/restore-probe.sh" "$BACKUP" report "$bin" "$root/probes" > "$root/probe.out" 2> "$root/probe.err"
    )
    [[ -f $root/cdpath/work/report/brain.restored.txt && -f $root/cdpath/elsewhere/report/sentinel ]] || fail "$1" 'CDPATH redirected the report'
    ;;
  source_offline)
    make_backup
    "${clean[@]}" "$PG_BIN/pg_ctl" -D "$source_pg" -m fast -w stop > "$root/source-stopped.log"
    restore "$root/reports/offline"
    grep -q 'RESTORE_EQUAL db=brain' "$root/probe.out" || fail "$1" 'restore still requires the source database'
    ;;
  manifest_coverage)
    make_backup
    sed -i '/  brain.fingerprint.txt$/d' "$BACKUP/SHA256SUMS"
    if restore "$root/reports/manifest"; then fail "$1" 'manifest omitted the reference fingerprint'; fi
    assert_no_equal "$1"
    [[ ! -e $root/reports/manifest ]] || fail "$1" 'manifest rejected only after creating a report'
    ;;
  multiple_databases)
    source_sql -d postgres -c 'CREATE DATABASE brain_pilot TEMPLATE brain'
    printf '20300101T000100Z\n' > "$bin/stamp"
    "${clean[@]}" bash "$repo/ops/brain-postgres/backup.sh" "$root/backups" 14 "$bin" brain brain_pilot > "$root/backup.out" 2> "$root/backup.err"
    BACKUP="$root/backups/brain-20300101T000100Z"
    source_sql -d brain_pilot -c "INSERT INTO brain_legacy.facts VALUES (2, 'later pilot data')"
    restore "$root/reports/multiple"
    [[ $(grep -c '^RESTORE_EQUAL db=' "$root/probe.out") == 2 ]] || fail "$1" 'not all databases verified'
    cmp "$BACKUP/brain.fingerprint.txt" "$root/reports/multiple/brain.restored.txt"
    cmp "$BACKUP/brain_pilot.fingerprint.txt" "$root/reports/multiple/brain_pilot.restored.txt"
    ;;
  special_paths)
    target="$root/backup 'quoted' "\"$'\n'
    report="$root/reports/report 'quoted' "\"$'\n'
    mkdir -- "$target"
    make_backup "$target"
    restore "$report"
    cmp "$BACKUP/brain.fingerprint.txt" "$report/brain.restored.txt"
    # Reusing the explicit report path must still work.
    restore "$report"
    cmp "$BACKUP/brain.fingerprint.txt" "$report/brain.restored.txt"
    printf 'protected\n' > "$root/sentinel"
    rm -- "$report/globals.out"
    ln -- "$root/sentinel" "$report/globals.out"
    if restore "$report"; then fail "$1" 'hardlinked report artifact was accepted'; fi
    [[ $(cat "$root/sentinel") == protected ]] || fail "$1" 'report changed an outside hardlink target'
    rm -- "$report/globals.out"
    ln -s -- "$root/sentinel" "$report/globals.out"
    if restore "$report"; then fail "$1" 'symlinked report artifact was accepted'; fi
    [[ $(cat "$root/sentinel") == protected ]] || fail "$1" 'report changed an outside symlink target'
    ;;
esac
printf 'PASS: restore %s (private PostgreSQL 16)\n' "$1"
