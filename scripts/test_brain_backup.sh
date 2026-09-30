#!/usr/bin/env bash
# Destructive operations are confined to a fresh temporary root. PostgreSQL is NEVER contacted.
set -euo pipefail
repo=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
root=$(mktemp -d)
trap 'rm -rf -- "$root"' EXIT
mkdir -- "$root/stubs"
cat > "$root/stubs/pg-stub" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
name=${0##*/}
if [[ -f "$(dirname "$0")/fail-$name" ]]; then
  # psql reports SQL errors with status 0 unless ON_ERROR_STOP is configured.
  if [[ $name == psql && " $* " != *' ON_ERROR_STOP=1 '* ]]; then exit 0; fi
  exit 42
fi
case "$name" in
  pg_dump)
    while (($#)); do
      if [[ $1 == --file ]]; then printf 'isolated dump\n' > "$2"; exit 0; fi
      shift
    done
    exit 43 ;;
  pg_restore) printf 'isolated table of contents\n' ;;
  pg_dumpall) printf 'isolated globals\n' ;;
  psql) printf '2 brain.store.v2\n' ;;
  *) exit 44 ;;
esac
STUB
cat > "$root/stubs/date" <<'STUB'
#!/usr/bin/env bash
cat "$(dirname "$0")/date-value"
STUB
printf '%s\n' '20300101T000000Z' > "$root/stubs/date-value"
chmod +x -- "$root/stubs/pg-stub" "$root/stubs/date"
for binary in pg_dump pg_restore pg_dumpall psql; do
  ln -s -- pg-stub "$root/stubs/$binary"
done
export PATH="$root/stubs:$PATH"
cd -- "$root"

completed() {
  local path=$1
  mkdir -- "$path"
  printf 'dump\n' > "$path/brain.dump"
  printf 'toc\n' > "$path/brain.toc"
  printf 'globals\n' > "$path/globals.sql"
  printf 'schema\n' > "$path/schema_version.txt"
  (cd -- "$path" && sha256sum -- brain.dump brain.toc globals.sql schema_version.txt > SHA256SUMS)
}
run_backup() {
  bash "$repo/ops/brain-postgres/backup.sh" "$1" "$2" "$root/stubs" "${@:3}" > "$root/output" 2> "$root/error"
}
check() {
  [[ -f $1 ]] || { printf 'FAIL: missing protected file: %q\n' "$1" >&2; exit 1; }
}

# Include a prefix directory that whitespace-based xargs would accidentally delete.
for variant in space special; do
  base="$root/$variant"
  mkdir -- "$base" "$base/backup"
  printf sentinel > "$base/backup/sentinel"
  if [[ $variant == space ]]; then target="$base/backup dir"; else target="$base/backup "\'\"$'\n'"dir"; fi
  mkdir -- "$target"
  for day in 01 02 03; do completed "$target/brain-202001${day}T000000Z"; done
  mkdir -- "$target/.brain-20200104T000000Z.partial" "$target/brain-unexpected" "$target/brain-20200105T000000Z"
  printf partial > "$target/.brain-20200104T000000Z.partial/sentinel"
  printf unexpected > "$target/brain-unexpected/sentinel"
  printf incomplete > "$target/brain-20200105T000000Z/sentinel"
  ln -s -- "$base/backup" "$target/brain-20200106T000000Z"
  completed "$target/brain-20200107T000000Z"
  rm -- "$target/brain-20200107T000000Z/brain.dump"
  ln -s -- "$base/backup/sentinel" "$target/brain-20200107T000000Z/brain.dump"
  completed "$target/brain-20200108T000000Z"
  rm -- "$target/brain-20200108T000000Z/brain.toc"
  run_backup "$target" 2
  check "$base/backup/sentinel"
  check "$target/brain-20300101T000000Z/SHA256SUMS"
  check "$target/brain-20200103T000000Z/SHA256SUMS"
  [[ ! -e "$target/brain-20200101T000000Z" && ! -e "$target/brain-20200102T000000Z" ]]
  check "$target/.brain-20200104T000000Z.partial/sentinel"
  check "$target/brain-unexpected/sentinel"
  check "$target/brain-20200105T000000Z/sentinel"
  [[ -L "$target/brain-20200106T000000Z" ]]
  [[ -L "$target/brain-20200107T000000Z/brain.dump" && -d "$target/brain-20200108T000000Z" ]]
  printf 'PASS: path %s, retention, outside sentinel, partials, unexpected entries and symlinks\n' "$variant"
done

for keep in 0 -1 abc 1.5 +1 '' 999999999999999999999999999; do
  target="$root/invalid-${keep:-empty}"
  mkdir -- "$target"
  completed "$target/brain-20200101T000000Z"
  if run_backup "$target" "$keep"; then printf 'FAIL: accepted KEEP=%q\n' "$keep" >&2; exit 1; fi
  check "$target/brain-20200101T000000Z/SHA256SUMS"
  [[ ! -e "$target/brain-20300101T000000Z" && ! -e "$target/.brain-20300101T000000Z.partial" ]]
done
printf 'PASS: invalid KEEP values fail before backup or rotation\n'

for old in 0 1 3; do
  target="$root/count-$old"
  mkdir -- "$target"
  for ((i=1; i<=old; i++)); do completed "$target/brain-2020010${i}T000000Z"; done
  run_backup "$target" 2
  check "$target/brain-20300101T000000Z/SHA256SUMS"
  shopt -s nullglob
  backups=("$target"/brain-*)
  expected=$((old + 1)); ((expected <= 2)) || expected=2
  [[ ${#backups[@]} == "$expected" ]]
done
printf 'PASS: fewer, exactly and more completed backups than KEEP\n'

for failure in pg_dump pg_restore pg_dumpall psql; do
  target="$root/failure-$failure"
  mkdir -- "$target"
  for day in 01 02 03; do completed "$target/brain-202001${day}T000000Z"; done
  touch "$root/stubs/fail-$failure"
  if run_backup "$target" 1; then printf 'FAIL: backup failure ignored\n' >&2; exit 1; fi
  rm -- "$root/stubs/fail-$failure"
  for day in 01 02 03; do check "$target/brain-202001${day}T000000Z/SHA256SUMS"; done
  [[ ! -e "$target/brain-20300101T000000Z" && ! -e "$target/.brain-20300101T000000Z.partial" ]]
done
printf 'PASS: failed dump, restore validation, globals or schema never rotate backups\n'

# Invalid database identifiers must not escape the partial directory via a dump filename.
target="$root/database-path"
mkdir -- "$target"
if run_backup "$target" 1 '../escape'; then exit 1; fi
[[ ! -e "$target/escape.dump" && ! -e "$root/escape.dump" ]]
printf 'PASS: database names cannot become path traversal\n'

target="$root/backward clock"
mkdir -- "$target"
for day in 01 02 03; do completed "$target/brain-202001${day}T000000Z"; done
printf '%s\n' '19991231T000000Z' > "$root/stubs/date-value"
run_backup "$target" 1
check "$target/brain-19991231T000000Z/SHA256SUMS"
backups=("$target"/brain-*)
[[ ${#backups[@]} == 1 ]]
printf 'PASS: backward clock preserves the new backup and honors KEEP\n'
