#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LOAD_INFISICAL="${LOAD_INFISICAL:-1}"
BRAIN_BIN="${DEADLOCK_BRAIN_BIN:-$ROOT_DIR/rust/target/release/deadlock-brain}"
SECRET_EXEC="${DEADLOCK_BRAIN_SECRET_EXEC_BIN:-$ROOT_DIR/rust/target/release/deadlock-brain-secret-exec}"

export DEADLOCK_BRAIN_ROOT="$ROOT_DIR"
cd "$ROOT_DIR"

if [[ ! -x "$BRAIN_BIN" ]]; then
  echo "deadlock-brain Release Binary fehlt: $BRAIN_BIN" >&2
  exit 1
fi

BRAIN_CMD=("$BRAIN_BIN")
if [[ "$LOAD_INFISICAL" == "1" || "$LOAD_INFISICAL" == "true" ]]; then
  if [[ ! -x "$SECRET_EXEC" ]]; then
    echo "deadlock-brain-secret-exec Release Binary fehlt: $SECRET_EXEC" >&2
    exit 1
  fi
  BRAIN_CMD=("$SECRET_EXEC" -- "$BRAIN_BIN")
fi

"${BRAIN_CMD[@]}" refresh-sheet
"${BRAIN_CMD[@]}" learn analyze-next --limit 20
"${BRAIN_CMD[@]}" enrich patch-impact --limit 50
"${BRAIN_CMD[@]}" enrich meta-trends