#!/usr/bin/env bash
# Local/CI-only contract tests. No production DSN, TCP listener, secrets, or systemd.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PG_BIN="${BRAIN_TEST_PG_BIN:-$(pg_config --bindir)}"
CARGO="${BRAIN_TEST_CARGO:-cargo}"
CLUSTER="$ROOT/.core-test-pg"
if "$PG_BIN/pg_ctl" -D "$CLUSTER" status >/dev/null 2>&1; then
  echo 'Refusing to reuse or stop an already-running cluster.' >&2
  exit 1
fi
if [[ ! -f "$CLUSTER/PG_VERSION" ]]; then
  "$PG_BIN/initdb" -D "$CLUSTER" --auth=trust --username=brain_core_test --no-locale --encoding=UTF8
fi
# The only server started here is the dedicated worktree cluster, Unix socket only.
"$PG_BIN/pg_ctl" -D "$CLUSTER" -l "$CLUSTER/server.log" \
  -o "-c listen_addresses='' -k $CLUSTER -p 55439 -c max_connections=12 -c shared_buffers=16MB" -w start
cleanup() { "$PG_BIN/pg_ctl" -D "$CLUSTER" -m fast -w stop; }
trap cleanup EXIT
cd "$ROOT/rust"
BRAIN_CORE_TEST_PG_SOCKET="$CLUSTER" "$CARGO" test --locked -p brain-storage --test core_store \
  postgres_atomicity_fences_release_and_restart_contract -- --ignored --exact --nocapture
