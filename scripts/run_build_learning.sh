#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

LOAD_INFISICAL="${LOAD_INFISICAL:-1}"
LIMIT="${LIMIT:-5}"
IMPORT_LIMIT_PER_HERO="${IMPORT_LIMIT_PER_HERO:-10}"
LANGUAGE="${LANGUAGE:-0}"
DRY_RUN="${DRY_RUN:-0}"
DELAY_SECONDS="${DELAY_SECONDS:-2}"
ANALYSIS_TIMEOUT_SECONDS="${ANALYSIS_TIMEOUT_SECONDS:-600}"
BRAIN_BIN="${DEADLOCK_BRAIN_BIN:-$ROOT_DIR/rust/target/release/deadlock-brain}"
SECRET_EXEC="${DEADLOCK_BRAIN_SECRET_EXEC_BIN:-$ROOT_DIR/rust/target/release/deadlock-brain-secret-exec}"

export DEADLOCK_BRAIN_ROOT="$ROOT_DIR"

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

hero_args=()
if [[ -n "${HERO:-}" ]]; then
  hero_args=(--hero "$HERO")
fi

dry_args=()
if [[ "$DRY_RUN" == "1" || "$DRY_RUN" == "true" ]]; then
  dry_args=(--dry-run)
fi

"${BRAIN_CMD[@]}" learn import-steam-builds \
  --pretty \
  --language "$LANGUAGE" \
  --limit-per-hero "$IMPORT_LIMIT_PER_HERO" \
  "${hero_args[@]}"

analyze_args=(
  learn analyze-next
  --pretty
  --limit "$LIMIT"
  --delay-seconds "$DELAY_SECONDS"
  "${hero_args[@]}"
  "${dry_args[@]}"
)

if [[ "$ANALYSIS_TIMEOUT_SECONDS" == "0" ]]; then
  "${BRAIN_CMD[@]}" "${analyze_args[@]}"
else
  timeout "$ANALYSIS_TIMEOUT_SECONDS" "${BRAIN_CMD[@]}" "${analyze_args[@]}"
fi
