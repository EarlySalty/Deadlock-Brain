#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CONFIG_FILE="${INFISICAL_CONFIG_FILE:-/home/naniadm/.config/deadlock-bots/infisical.env}"
LOAD_INFISICAL="${LOAD_INFISICAL:-1}"
INFISICAL_RETRY_DELAY="${INFISICAL_RETRY_DELAY:-5}"
INFISICAL_MAX_ATTEMPTS="${INFISICAL_MAX_ATTEMPTS:-0}"

cd "$ROOT_DIR"

export PYTHONDONTWRITEBYTECODE=1
export PYTHONPATH="${PYTHONPATH:-$ROOT_DIR/src}"
export PYTHONUNBUFFERED=1

if [[ "$LOAD_INFISICAL" == "1" || "$LOAD_INFISICAL" == "true" ]]; then
  if [[ ! -f "$CONFIG_FILE" ]]; then
    echo "Infisical config not found: $CONFIG_FILE" >&2
    exit 1
  fi

  set -a
  source "$CONFIG_FILE"
  set +a

  attempt=0
  while true; do
    if INFISICAL_EXPORT="$(python3 scripts/export_infisical_env.py --format shell)"; then
      eval "$INFISICAL_EXPORT"
      break
    fi

    attempt=$((attempt + 1))
    if [[ "$INFISICAL_MAX_ATTEMPTS" -gt 0 && "$attempt" -ge "$INFISICAL_MAX_ATTEMPTS" ]]; then
      echo "Infisical secrets could not be loaded after $attempt attempt(s)." >&2
      exit 1
    fi

    echo "Infisical not ready for Deadlock Brain sheet sync, retrying in ${INFISICAL_RETRY_DELAY}s (attempt $attempt)." >&2
    sleep "$INFISICAL_RETRY_DELAY"
  done
fi

python3 -m deadlock_brain.cli refresh-sheet
python3 -m deadlock_brain.cli normalize entities --rebuild
python3 -m deadlock_brain.cli normalize sheet-stats --rebuild
python3 -m deadlock_brain.cli normalize sheet-tabs --rebuild
python3 -m deadlock_brain.cli learn analyze-next --limit 20
python3 -m deadlock_brain.cli enrich patch-impact --limit 50
python3 -m deadlock_brain.cli enrich meta-trends
