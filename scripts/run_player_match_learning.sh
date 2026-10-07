#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

LOAD_INFISICAL="${LOAD_INFISICAL:-1}"
PLAYERS_FROM_LEADERBOARD="${PLAYERS_FROM_LEADERBOARD:-3}"
MATCHES_PER_PLAYER="${MATCHES_PER_PLAYER:-3}"
ANALYZE_LIMIT="${ANALYZE_LIMIT:-5}"
DELAY_SECONDS="${DELAY_SECONDS:-2}"
CACHE_TTL_SECONDS="${CACHE_TTL_SECONDS:-21600}"
INCLUDE_MATCH_DETAILS="${INCLUDE_MATCH_DETAILS:-1}"
INCLUDE_BUILD_ANALYSIS="${INCLUDE_BUILD_ANALYSIS:-1}"
DRY_RUN="${DRY_RUN:-0}"
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

"${BRAIN_CMD[@]}" pull statlocker \
  --kind leaderboard-player-matches \
  --players-from-leaderboard "$PLAYERS_FROM_LEADERBOARD" \
  --matches-per-player "$MATCHES_PER_PLAYER" \
  --delay-seconds "$DELAY_SECONDS" \
  --cache-ttl-seconds "$CACHE_TTL_SECONDS" \
  "${detail_args[@]}" \
  "${build_args[@]}"

analyze_args=(
  player analyze-next
  --pretty
  --limit "$ANALYZE_LIMIT"
  --delay-seconds "$DELAY_SECONDS"
  "${account_args[@]}"
  "${dry_args[@]}"
)

if [[ "$ANALYSIS_TIMEOUT_SECONDS" == "0" ]]; then
  "${BRAIN_CMD[@]}" "${analyze_args[@]}"
else
  timeout "$ANALYSIS_TIMEOUT_SECONDS" "${BRAIN_CMD[@]}" "${analyze_args[@]}"
fi
