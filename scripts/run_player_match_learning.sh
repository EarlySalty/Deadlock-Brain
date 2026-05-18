#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

export PYTHONDONTWRITEBYTECODE=1
export PYTHONPATH="${PYTHONPATH:-src}"

CONFIG_FILE="${INFISICAL_CONFIG_FILE:-/home/naniadm/.config/deadlock-bots/infisical.env}"
LOAD_INFISICAL="${LOAD_INFISICAL:-1}"
PLAYERS_FROM_LEADERBOARD="${PLAYERS_FROM_LEADERBOARD:-3}"
MATCHES_PER_PLAYER="${MATCHES_PER_PLAYER:-3}"
ANALYZE_LIMIT="${ANALYZE_LIMIT:-5}"
MATCH_METADATA_LIMIT="${MATCH_METADATA_LIMIT:-$ANALYZE_LIMIT}"
DELAY_SECONDS="${DELAY_SECONDS:-2}"
CACHE_TTL_SECONDS="${CACHE_TTL_SECONDS:-21600}"
INCLUDE_MATCH_DETAILS="${INCLUDE_MATCH_DETAILS:-1}"
INCLUDE_BUILD_ANALYSIS="${INCLUDE_BUILD_ANALYSIS:-1}"
INCLUDE_DEADLOCK_API_MATCH_METADATA="${INCLUDE_DEADLOCK_API_MATCH_METADATA:-1}"
DRY_RUN="${DRY_RUN:-0}"
ANALYSIS_TIMEOUT_SECONDS="${ANALYSIS_TIMEOUT_SECONDS:-600}"

if [[ "$LOAD_INFISICAL" == "1" || "$LOAD_INFISICAL" == "true" ]]; then
  if [[ -f "$CONFIG_FILE" ]]; then
    set -a
    source "$CONFIG_FILE"
    set +a
    if INFISICAL_EXPORT="$(python3 scripts/export_infisical_env.py --format shell)"; then
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

detail_args=()
if [[ "$INCLUDE_MATCH_DETAILS" == "1" || "$INCLUDE_MATCH_DETAILS" == "true" ]]; then
  detail_args=(--include-match-details)
fi

build_args=()
if [[ "$INCLUDE_BUILD_ANALYSIS" == "1" || "$INCLUDE_BUILD_ANALYSIS" == "true" ]]; then
  build_args=(--include-build-analysis)
fi

account_args=()
if [[ -n "${ACCOUNT_ID:-}" ]]; then
  account_args=(--account-id "$ACCOUNT_ID")
fi

dry_args=()
if [[ "$DRY_RUN" == "1" || "$DRY_RUN" == "true" ]]; then
  dry_args=(--dry-run)
fi

python3 -m deadlock_brain.cli pull statlocker \
  --kind leaderboard-player-matches \
  --players-from-leaderboard "$PLAYERS_FROM_LEADERBOARD" \
  --matches-per-player "$MATCHES_PER_PLAYER" \
  --delay-seconds "$DELAY_SECONDS" \
  --cache-ttl-seconds "$CACHE_TTL_SECONDS" \
  "${detail_args[@]}" \
  "${build_args[@]}"

if [[ "$INCLUDE_DEADLOCK_API_MATCH_METADATA" == "1" || "$INCLUDE_DEADLOCK_API_MATCH_METADATA" == "true" ]]; then
  deadlock_match_args=()
  deadlock_account_args=()
  while IFS=$'\t' read -r account_id match_id; do
    [[ -z "$account_id" || -z "$match_id" ]] && continue
    deadlock_account_args+=(--account-id "$account_id")
    deadlock_match_args+=(--match-id "$match_id")
  done < <(
    MATCH_METADATA_LIMIT="$MATCH_METADATA_LIMIT" ACCOUNT_ID="${ACCOUNT_ID:-}" python3 - <<'PY'
import os
import sqlite3

from deadlock_brain.config import load_settings

settings = load_settings()
limit = max(1, min(int(os.getenv("MATCH_METADATA_LIMIT", "5")), 100))
account_filter = os.getenv("ACCOUNT_ID", "").strip()
conn = sqlite3.connect(settings.db_path)
params = []
sql = """
SELECT external_id
FROM entity_snapshots
WHERE source='statlocker' AND entity_type='statlocker_player_match'
"""
if account_filter:
    sql += " AND external_id LIKE ?"
    params.append(f"{account_filter}:%")
sql += " ORDER BY fetched_at DESC, id DESC LIMIT ?"
params.append(limit)
seen = set()
for (external_id,) in conn.execute(sql, params):
    account_id, _, match_id = str(external_id or "").partition(":")
    if not account_id or not match_id or (account_id, match_id) in seen:
        continue
    seen.add((account_id, match_id))
    print(f"{account_id}\t{match_id}")
PY
  )
  if [[ "${#deadlock_match_args[@]}" -gt 0 ]]; then
    python3 -m deadlock_brain.cli pull deadlock-api \
      "${deadlock_match_args[@]}" \
      "${deadlock_account_args[@]}" \
      --cache-ttl-seconds "$CACHE_TTL_SECONDS"
  fi
fi

analyze_cmd=(
  python3 -m deadlock_brain.cli player analyze-next
  --pretty \
  --limit "$ANALYZE_LIMIT" \
  --delay-seconds "$DELAY_SECONDS" \
  "${account_args[@]}" \
  "${dry_args[@]}"
)

if [[ "$ANALYSIS_TIMEOUT_SECONDS" == "0" ]]; then
  "${analyze_cmd[@]}"
else
  timeout "$ANALYSIS_TIMEOUT_SECONDS" "${analyze_cmd[@]}"
fi
