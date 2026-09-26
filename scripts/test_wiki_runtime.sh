#!/usr/bin/env bash
# C5 offline fixtures + real scratch PostgreSQL + operator CLI smoke test.
# No live wiki requests, application credentials, production database or systemd.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CARGO="${BRAIN_TEST_CARGO:-$(command -v cargo)}"
PG_BIN="${BRAIN_TEST_PG_BIN:-$(pg_config --bindir)}"
if [[ $(id -u) == 0 ]]; then echo 'Run as an unprivileged user (initdb refuses root).' >&2; exit 2; fi
WORK="$(mktemp -d /tmp/brain-c5-pg.XXXXXX)"
chmod 700 "$WORK"
printf 'C5_SCRATCH_ONLY\n' >"$WORK/C5_SCRATCH_ONLY"
mkdir -m 700 "$WORK/artifacts" "$WORK/home"
started=0
cleanup() {
  if [[ "$started" == 1 ]]; then "$PG_BIN/pg_ctl" -D "$WORK/pg" -m fast -w stop >/dev/null; fi
  printf 'C5 scratch evidence (server stopped): %s\n' "$WORK"
}
trap cleanup EXIT
# Each postgres command below targets this freshly-created cluster explicitly.
# Never read .pgpass or application environment variables.
export PGPASSFILE="$WORK/nonexistent-pgpass"
unset DATABASE_URL PGHOST PGPORT PGUSER PGPASSWORD PGDATABASE PGSERVICE PGSERVICEFILE PGOPTIONS
"$PG_BIN/initdb" -D "$WORK/pg" --auth=trust --username=brain_wiki_c5 --no-locale --encoding=UTF8 >"$WORK/initdb.log"
"$PG_BIN/pg_ctl" -D "$WORK/pg" -l "$WORK/server.log" \
  -o "-c listen_addresses='' -k $WORK -p 55441 -c max_connections=12 -c shared_buffers=16MB" -w start
started=1
"$PG_BIN/createdb" -h "$WORK" -p 55441 -U brain_wiki_c5 brain_wiki_c5
# Isolate HOME and all application credentials. Preserve only explicit tool/cache
# locations, not any service configuration. No Infisical invocation is needed.
CLEAN=(env -i "PATH=$PATH" "HOME=$WORK/home" "CARGO_HOME=${CARGO_HOME:-$HOME/.cargo}" "RUSTUP_HOME=${RUSTUP_HOME:-$HOME/.rustup}"
  "SQLX_OFFLINE=true" "CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS:-2}" "CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-$ROOT/rust/target}"
  "BRAIN_C5_TEST_SOCKET=$WORK" "BRAIN_C5_TEST_OUTPUT=$WORK/artifacts")
cd "$ROOT/rust"
"${CLEAN[@]}" "$CARGO" test --locked -p brain-contracts -p brain-storage -p dbrain-s12-wiki-probe -p dbrain-sources
"${CLEAN[@]}" "$CARGO" test --locked -p dbrain-sources --test wiki_runtime \
  scratch_raw_ir_facts_release_delta_reparse_and_acl -- --ignored --exact --nocapture
"${CLEAN[@]}" "$CARGO" run --locked -p dbrain-sources --bin brain-wiki-pilot -- plan \
  "$WORK/artifacts/fixture.capture.json" "$WORK/artifacts/fixture.mapping.json" "$WORK/artifacts/fixture.release.json" "$WORK/cli-plan"
"${CLEAN[@]}" "$CARGO" run --locked -p dbrain-sources --bin brain-wiki-pilot -- stage \
  "$WORK/artifacts/fixture.capture.json" "$WORK/artifacts/fixture.mapping.json" "$WORK/artifacts/fixture.release.json" "$WORK" "$WORK/cli-stage"
echo 'C5 offline mocks, scratch DB and CLI: PASS (no live capture performed)'
