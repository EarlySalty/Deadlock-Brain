#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

export PYTHONDONTWRITEBYTECODE=1

CONFIG_FILE="${INFISICAL_CONFIG_FILE:-/home/naniadm/.config/deadlock-bots/infisical.env}"
LOAD_INFISICAL="${LOAD_INFISICAL:-1}"
LIMIT="${LIMIT:-5}"
IMPORT_LIMIT_PER_HERO="${IMPORT_LIMIT_PER_HERO:-10}"
LANGUAGE="${LANGUAGE:-0}"
DRY_RUN="${DRY_RUN:-0}"
DELAY_SECONDS="${DELAY_SECONDS:-2}"
ANALYSIS_TIMEOUT_SECONDS="${ANALYSIS_TIMEOUT_SECONDS:-600}"
BRAIN_BIN="${DEADLOCK_BRAIN_BIN:-$PWD/rust/target/release/deadlock-brain}"
PYTHON_BIN="${DEADLOCK_BRAIN_PYTHON:-python3}"

if [[ -x "$PWD/.venv/bin/python" ]]; then
  PYTHON_BIN="$PWD/.venv/bin/python"
fi

if [[ ! -x "$BRAIN_BIN" ]]; then
  echo "deadlock-brain release binary missing: $BRAIN_BIN" >&2
  exit 1
fi

if [[ "$LOAD_INFISICAL" == "1" || "$LOAD_INFISICAL" == "true" ]]; then
  if [[ -f "$CONFIG_FILE" ]]; then
    set -a
    source "$CONFIG_FILE"
    set +a
    if INFISICAL_EXPORT="$("$PYTHON_BIN" scripts/export_infisical_env.py --format shell)"; then
      eval "$INFISICAL_EXPORT"
    else
      echo "Infisical secrets could not be loaded for Deadlock Brain." >&2
      exit 1
    fi
  else
    echo "Infisical config not found: $CONFIG_FILE" >&2
    exit 1
  fi
fi

hero_args=()
if [[ -n "${HERO:-}" ]]; then
  hero_args=(--hero "$HERO")
fi

dry_args=()
if [[ "$DRY_RUN" == "1" || "$DRY_RUN" == "true" ]]; then
  dry_args=(--dry-run)
fi

"$BRAIN_BIN" learn import-steam-builds \
  --pretty \
  --language "$LANGUAGE" \
  --limit-per-hero "$IMPORT_LIMIT_PER_HERO" \
  "${hero_args[@]}"

analyze_cmd=(
  "$BRAIN_BIN" learn analyze-next
  --pretty \
  --limit "$LIMIT" \
  --delay-seconds "$DELAY_SECONDS" \
  "${hero_args[@]}" \
  "${dry_args[@]}"
)

if [[ "$ANALYSIS_TIMEOUT_SECONDS" == "0" ]]; then
  "${analyze_cmd[@]}"
else
  timeout "$ANALYSIS_TIMEOUT_SECONDS" "${analyze_cmd[@]}"
fi
