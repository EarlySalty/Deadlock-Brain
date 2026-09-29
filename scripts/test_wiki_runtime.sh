#!/usr/bin/env bash
# C5 offline fixtures + real scratch PostgreSQL + operator CLI smoke test.
# No live wiki requests, application credentials, production database or systemd.
set -euo pipefail
umask 077
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CARGO="${BRAIN_TEST_CARGO:-$HOME/.cargo/bin/cargo}"
PG_BIN="${BRAIN_TEST_PG_BIN:-$(pg_config --bindir)}"
if [[ $(id -u) == 0 ]]; then echo 'Run as an unprivileged user (initdb refuses root).' >&2; exit 2; fi
WORK="$(mktemp -d /tmp/brain-c5-pg.XXXXXX)"
chmod 700 "$WORK"
printf 'C5_SCRATCH_ONLY\n' >"$WORK/C5_SCRATCH_ONLY"
mkdir -m 700 "$WORK/artifacts" "$WORK/home"
PG_ENV=(env -i "PATH=$PATH" "HOME=$WORK/home" "PGPASSFILE=$WORK/nonexistent-pgpass")
started=0
cleanup() {
  local status=$?
  trap - EXIT
  if [[ "$started" == 1 ]]; then
    if ! "${PG_ENV[@]}" "$PG_BIN/pg_ctl" -D "$WORK/pg" -m fast -w stop >/dev/null; then
      printf 'Scratch PostgreSQL could not be stopped: %s\n' "$WORK" >&2
      exit 1
    fi
  fi
  if (( status == 0 )); then
    rm -rf -- "$WORK"
  else
    printf 'C5 scratch evidence (server stopped): %s\n' "$WORK" >&2
  fi
  exit "$status"
}
trap cleanup EXIT
# Each postgres command below targets this freshly-created cluster explicitly.
# Never read .pgpass or application environment variables.
export PGPASSFILE="$WORK/nonexistent-pgpass"
unset DATABASE_URL PGHOST PGPORT PGUSER PGPASSWORD PGDATABASE PGSERVICE PGSERVICEFILE PGOPTIONS
"${PG_ENV[@]}" "$PG_BIN/initdb" -D "$WORK/pg" --auth-local=peer --auth-host=reject --username=brain_wiki_c5 --no-locale --encoding=UTF8 >"$WORK/initdb.log"
OS_USER="$(id -un)"
if [[ ! "$OS_USER" =~ ^[a-z_][a-z0-9_-]*$ ]]; then
  printf 'Unsupported local account name.\n' >&2
  exit 2
fi
printf 'brain_scratch %s brain_wiki_c5\n' "$OS_USER" > "$WORK/pg/pg_ident.conf"
printf 'local all all peer map=brain_scratch\nhost all all all reject\n' > "$WORK/pg/pg_hba.conf"
"${PG_ENV[@]}" "$PG_BIN/pg_ctl" -D "$WORK/pg" -l "$WORK/server.log" \
  -o "-c listen_addresses='' -k $WORK -p 55441 -c max_connections=12 -c shared_buffers=16MB" -w start
started=1
"${PG_ENV[@]}" "$PG_BIN/createdb" -h "$WORK" -p 55441 -U brain_wiki_c5 brain_wiki_c5
# Isolate HOME and all application credentials. Preserve only explicit tool/cache
# locations, not any service configuration. No Infisical invocation is needed.
CLEAN=(env -i "PATH=$PATH" "HOME=$WORK/home" "CARGO_HOME=${CARGO_HOME:-$HOME/.cargo}" "RUSTUP_HOME=${RUSTUP_HOME:-$HOME/.rustup}"
  "SQLX_OFFLINE=true" "CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS:-2}" "CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-$ROOT/rust/target}"
  "BRAIN_C5_TEST_SOCKET=$WORK" "BRAIN_C5_TEST_OUTPUT=$WORK/artifacts")
cd "$ROOT/rust"
"${CLEAN[@]}" "$CARGO" test --locked --offline --jobs 2 -p brain-contracts -p brain-storage -p dbrain-s12-wiki-probe -p dbrain-sources
"${CLEAN[@]}" "$CARGO" test --locked --offline --jobs 2 -p dbrain-sources --test wiki_runtime \
  scratch_raw_ir_facts_release_delta_reparse_and_acl -- --ignored --exact --nocapture
"${CLEAN[@]}" "$CARGO" run --locked --offline --jobs 2 -p dbrain-sources --bin brain-wiki-pilot -- plan \
  "$WORK/artifacts/fixture.capture.json" "$WORK/artifacts/fixture.mapping.json" "$WORK/artifacts/fixture.release.json" "$WORK/cli-plan"
"${CLEAN[@]}" "$CARGO" run --locked --offline --jobs 2 -p dbrain-sources --bin brain-wiki-pilot -- stage \
  "$WORK/artifacts/fixture.capture.json" "$WORK/artifacts/fixture.mapping.json" "$WORK/artifacts/fixture.release.json" "$WORK" "$WORK/cli-stage"
echo 'C5 offline mocks, scratch DB and CLI: PASS (no live capture performed)'
