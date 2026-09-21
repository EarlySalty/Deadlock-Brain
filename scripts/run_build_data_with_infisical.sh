#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="${DEADLOCK_BRAIN_ROOT:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
LOAD_INFISICAL="${LOAD_INFISICAL:-1}"
BRAIN_BIN="${DEADLOCK_BRAIN_BIN:-$ROOT_DIR/rust/target/release/deadlock-brain}"
SECRET_EXEC="${DEADLOCK_BRAIN_SECRET_EXEC_BIN:-$ROOT_DIR/rust/target/release/deadlock-brain-secret-exec}"

export DEADLOCK_BRAIN_ROOT="$ROOT_DIR"
cd "$ROOT_DIR"

if [[ ! -x "$BRAIN_BIN" ]]; then
  echo "deadlock-brain Release Binary fehlt: $BRAIN_BIN" >&2
  exit 1
fi
if [[ "$LOAD_INFISICAL" == "1" || "$LOAD_INFISICAL" == "true" ]] && [[ ! -x "$SECRET_EXEC" ]]; then
  echo "deadlock-brain-secret-exec Release Binary fehlt: $SECRET_EXEC" >&2
  exit 1
fi

run_brain() {
  if [[ "$LOAD_INFISICAL" == "1" || "$LOAD_INFISICAL" == "true" ]]; then
    "$SECRET_EXEC" -- "$BRAIN_BIN" "$@"
  else
    "$BRAIN_BIN" "$@"
  fi
}

run_brain pull build-data --hero all
run_brain population sync --matches 2000 \
  || echo "Population Sync fehlgeschlagen, der bestehende Build Data Stand bleibt erhalten." >&2
run_brain population stats \
  || echo "Population Stats fehlgeschlagen, die Aggregate wurden nicht aktualisiert." >&2
