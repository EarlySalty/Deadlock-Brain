#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)
SOURCE="$ROOT/scripts/ci/migration_guard.rs"
RUSTUP_CACHE=${RUSTUP_HOME:-$HOME/.rustup}
TOOLCHAINS=("$RUSTUP_CACHE"/toolchains/1.97.1-*/bin)
if ((${#TOOLCHAINS[@]} != 1)) || [[ ! -x "${TOOLCHAINS[0]}/rustc" || ! -x "${TOOLCHAINS[0]}/rustfmt" ]]; then
  printf 'Expected exactly one installed Rust 1.97.1 toolchain under %s/toolchains\n' "$RUSTUP_CACHE" >&2
  exit 2
fi
RUSTC="${TOOLCHAINS[0]}/rustc"
RUSTFMT="${TOOLCHAINS[0]}/rustfmt"
BASE_REF=origin/main
HEAD_REF=HEAD
PEER_REFS=()

usage() {
  printf 'Usage: %s [--base REF] [--head REF] [--peer EXPLICIT_HEAD]...\n' "${0##*/}" >&2
}

while (($#)); do
  case "$1" in
    --base)
      (($# >= 2)) || { usage; exit 2; }
      BASE_REF=$2
      shift 2
      ;;
    --head)
      (($# >= 2)) || { usage; exit 2; }
      HEAD_REF=$2
      shift 2
      ;;
    --peer)
      (($# >= 2)) || { usage; exit 2; }
      PEER_REFS+=("$2")
      shift 2
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      usage
      exit 2
      ;;
  esac
done

[[ -x "$RUSTC" && -x "$RUSTFMT" ]] || {
  printf 'Rustup proxies not found under %s/.cargo/bin\n' "$HOME" >&2
  exit 2
}
"$RUSTC" --version | rg -q '^rustc 1\.97\.1 '
"$RUSTFMT" --edition 2021 --check "$SOURCE"

WORK=$(mktemp -d /tmp/brain-migration-guard.XXXXXXXX)
trap 'rm -rf -- "$WORK"' EXIT
TEST_BIN="$WORK/migration-guard-tests"
GUARD_BIN="$WORK/migration-guard"
"$RUSTC" --edition=2021 -Dwarnings --test "$SOURCE" -o "$TEST_BIN"
"$TEST_BIN"
"$RUSTC" --edition=2021 -Dwarnings "$SOURCE" -o "$GUARD_BIN"

GUARD_ARGS=(--base "$BASE_REF" --head "$HEAD_REF")
for peer_ref in "${PEER_REFS[@]}"; do GUARD_ARGS+=(--peer "$peer_ref"); done
(cd -- "$ROOT" && "$GUARD_BIN" "${GUARD_ARGS[@]}")

init_case() {
  local name=$1
  CASE_DIR="$WORK/$name"
  mkdir -p "$CASE_DIR/scripts/migrations" "$CASE_DIR/tests/fixtures"
  git -C "$CASE_DIR" init -q
  git -C "$CASE_DIR" checkout -q -b main
  git -C "$CASE_DIR" config user.name 'Migration Guard Test'
  git -C "$CASE_DIR" config user.email 'migration-guard@example.invalid'
  printf 'CREATE TABLE IF NOT EXISTS brain.applied (id integer);\n' > "$CASE_DIR/scripts/migrations/001-applied.sql"
  printf 'CREATE TABLE IF NOT EXISTS brain.fixture_only (id integer);\n' > "$CASE_DIR/tests/fixtures/schema.sql"
  git -C "$CASE_DIR" add scripts/migrations/001-applied.sql tests/fixtures/schema.sql
  git -C "$CASE_DIR" commit -qm baseline
  git -C "$CASE_DIR" tag base
}

commit_case() {
  git -C "$CASE_DIR" add -A
  git -C "$CASE_DIR" commit -qm candidate
}

expect_failure() {
  local label=$1 expected=$2
  shift 2
  local output="$WORK/$label.log"
  if (cd -- "$CASE_DIR" && "$GUARD_BIN" --base base --head HEAD "$@") > "$output" 2>&1; then
    printf 'FAIL: %s unexpectedly passed\n' "$label" >&2
    cat "$output" >&2
    exit 1
  fi
  if ! rg -Fq "$expected" "$output"; then
    printf 'FAIL: %s failed for the wrong reason\n' "$label" >&2
    cat "$output" >&2
    exit 1
  fi
  printf 'PASS: %s\n' "$label"
}

expect_success() {
  local label=$1
  if ! (cd -- "$CASE_DIR" && "$GUARD_BIN" --base base --head HEAD) > "$WORK/$label.log" 2>&1; then
    printf 'FAIL: %s unexpectedly failed\n' "$label" >&2
    cat "$WORK/$label.log" >&2
    exit 1
  fi
  printf 'PASS: %s\n' "$label"
}

init_case modified
printf 'CREATE TABLE IF NOT EXISTS brain.applied (id bigint);\n' > "$CASE_DIR/scripts/migrations/001-applied.sql"
commit_case
expect_failure modified 'modifies applied migration'

init_case renamed
mv "$CASE_DIR/scripts/migrations/001-applied.sql" "$CASE_DIR/scripts/migrations/002-renamed.sql"
commit_case
expect_failure renamed 'deletes or renames applied migration'

init_case deleted
git -C "$CASE_DIR" rm -q scripts/migrations/001-applied.sql
commit_case
expect_failure deleted 'deletes or renames applied migration'

init_case mode_changed
chmod +x "$CASE_DIR/scripts/migrations/001-applied.sql"
commit_case
expect_failure mode_changed 'modifies applied migration'

init_case table_conflict
printf 'CREATE TABLE IF NOT EXISTS brain.applied (id bigint);\n' > "$CASE_DIR/scripts/migrations/002-new.sql"
commit_case
expect_failure table_conflict 'conflicts with applied table brain.applied'

init_case identical_idempotent
printf 'CREATE TABLE IF NOT EXISTS brain.applied (id integer);\n' > "$CASE_DIR/scripts/migrations/002-new.sql"
commit_case
expect_success identical_idempotent

init_case fixture_ignored
printf 'CREATE TABLE IF NOT EXISTS brain.fixture_only (id integer);\n' > "$CASE_DIR/scripts/migrations/002-new.sql"
commit_case
expect_success fixture_ignored

init_case peer_conflict
  git -C "$CASE_DIR" checkout -q -b pr-a base
printf 'CREATE TABLE IF NOT EXISTS brain.parallel (id integer);\n' > "$CASE_DIR/scripts/migrations/002-a.sql"
git -C "$CASE_DIR" add scripts/migrations/002-a.sql
git -C "$CASE_DIR" commit -qm pr-a
  git -C "$CASE_DIR" checkout -q -b pr-b base
printf 'CREATE TABLE IF NOT EXISTS brain.parallel (id bigint);\n' > "$CASE_DIR/scripts/migrations/003-b.sql"
git -C "$CASE_DIR" add scripts/migrations/003-b.sql
git -C "$CASE_DIR" commit -qm pr-b
expect_failure peer_conflict 'migration table conflict' --peer pr-a

init_case missing_peer
expect_failure missing_peer 'Needed a single revision' --peer no-such-peer

init_case unsupported
printf 'CREATE TABLE brain.new_table AS SELECT 1;\n' > "$CASE_DIR/scripts/migrations/002-new.sql"
commit_case
expect_failure unsupported 'unsupported CREATE TABLE form'

init_case symlink
printf 'CREATE TABLE IF NOT EXISTS brain.symlinked (id integer);\n' > "$WORK/symlink-target.sql"
ln -s "$WORK/symlink-target.sql" "$CASE_DIR/scripts/migrations/002-new.sql"
git -C "$CASE_DIR" add scripts/migrations/002-new.sql
git -C "$CASE_DIR" commit -qm symlink
expect_failure symlink 'unexpected migration tree entry'

printf 'Local migration guard and temp-Git regressions passed.\n'
