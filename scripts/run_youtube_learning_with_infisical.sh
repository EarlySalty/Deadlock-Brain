#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CONFIG_FILE="${INFISICAL_CONFIG_FILE:-/home/naniadm/.config/deadlock-bots/infisical.conf}"
LOAD_INFISICAL="${LOAD_INFISICAL:-1}"
INFISICAL_RETRY_DELAY="${INFISICAL_RETRY_DELAY:-5}"
INFISICAL_MAX_ATTEMPTS="${INFISICAL_MAX_ATTEMPTS:-0}"

YOUTUBE_CONFIG="${YOUTUBE_CONFIG:-config/youtube_feeds.json}"
DISCOVER_LIMIT="${YOUTUBE_DISCOVER_LIMIT:-50}"
ANALYZE_LIMIT="${YOUTUBE_ANALYZE_LIMIT:-5}"
TRANSCRIPT_FETCH_LIMIT="${YOUTUBE_TRANSCRIPT_FETCH_LIMIT:-20}"
FETCH_TRANSCRIPTS="${YOUTUBE_FETCH_TRANSCRIPTS:-1}"
YOUTUBE_BIN="${DEADLOCK_BRAIN_YT_BIN:-$ROOT_DIR/rust/target/release/deadlock-brain-yt}"

cd "$ROOT_DIR"

PYTHON_BIN="${DEADLOCK_BRAIN_PYTHON:-python3}"
if [[ -x "$ROOT_DIR/.venv/bin/python" ]]; then
  PYTHON_BIN="$ROOT_DIR/.venv/bin/python"
fi

export PYTHONDONTWRITEBYTECODE=1
export PYTHONUNBUFFERED=1

if [[ ! -x "$YOUTUBE_BIN" ]]; then
  echo "deadlock-brain-yt release binary missing: $YOUTUBE_BIN" >&2
  exit 1
fi

if [[ "$LOAD_INFISICAL" == "1" || "$LOAD_INFISICAL" == "true" ]]; then
  if [[ ! -f "$CONFIG_FILE" ]]; then
    echo "Infisical config not found: $CONFIG_FILE" >&2
    exit 1
  fi

  set -a
  source "$CONFIG_FILE"
  set +a

  if [[ -z "${INFISICAL_SERVICE_TOKEN:-}" ]]; then
    for token_file in "${CREDENTIALS_DIRECTORY:-/nonexistent}/infisical-token" \
                      "${INFISICAL_TOKEN_FILE:-/home/naniadm/.config/infisical-tokens/infisical-token-bots}"; do
      if [[ -f "$token_file" ]]; then
        INFISICAL_SERVICE_TOKEN="$(<"$token_file")"
        export INFISICAL_SERVICE_TOKEN
        break
      fi
    done
  fi

  attempt=0
  while true; do
    if INFISICAL_EXPORT="$("$PYTHON_BIN" scripts/export_infisical_env.py --format shell)"; then
      eval "$INFISICAL_EXPORT"
      break
    fi

    attempt=$((attempt + 1))
    if [[ "$INFISICAL_MAX_ATTEMPTS" -gt 0 && "$attempt" -ge "$INFISICAL_MAX_ATTEMPTS" ]]; then
      echo "Infisical secrets could not be loaded after $attempt attempt(s)." >&2
      exit 1
    fi

    echo "Infisical not ready for Deadlock Brain YouTube learning, retrying in ${INFISICAL_RETRY_DELAY}s (attempt $attempt)." >&2
    sleep "$INFISICAL_RETRY_DELAY"
  done
fi

if [[ -z "${DEADLOCK_CENTRAL_DSN:-}" ]]; then
  echo "DEADLOCK_CENTRAL_DSN is not set." >&2
  exit 1
fi

ARGS=(
  auto-learn
  --config "$YOUTUBE_CONFIG" \
  --discover-limit "$DISCOVER_LIMIT" \
  --analyze-limit "$ANALYZE_LIMIT" \
  --transcript-fetch-limit "$TRANSCRIPT_FETCH_LIMIT"
)

if [[ "$FETCH_TRANSCRIPTS" == "0" || "$FETCH_TRANSCRIPTS" == "false" ]]; then
  ARGS+=(--no-fetch-transcripts)
fi

"$YOUTUBE_BIN" "${ARGS[@]}"
