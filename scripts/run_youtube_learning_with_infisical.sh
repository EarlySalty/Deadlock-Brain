#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CONFIG_FILE="${INFISICAL_CONFIG_FILE:-/home/naniadm/.config/deadlock-bots/infisical.env}"
LOAD_INFISICAL="${LOAD_INFISICAL:-1}"
INFISICAL_RETRY_DELAY="${INFISICAL_RETRY_DELAY:-5}"
INFISICAL_MAX_ATTEMPTS="${INFISICAL_MAX_ATTEMPTS:-0}"

YOUTUBE_CONFIG="${YOUTUBE_CONFIG:-config/youtube_feeds.json}"
TRANSCRIPT_DIR="${YOUTUBE_TRANSCRIPT_DIR:-data/youtube_transcripts}"
DISCOVER_LIMIT="${YOUTUBE_DISCOVER_LIMIT:-50}"
ANALYZE_LIMIT="${YOUTUBE_ANALYZE_LIMIT:-5}"
TRANSCRIPT_FETCH_LIMIT="${YOUTUBE_TRANSCRIPT_FETCH_LIMIT:-20}"
SUBTITLE_LANGUAGES="${YOUTUBE_SUBTITLE_LANGUAGES:-original}"
CACHE_TTL_SECONDS="${YOUTUBE_CACHE_TTL_SECONDS:-21600}"
DELAY_SECONDS="${YOUTUBE_DELAY_SECONDS:-1}"
FETCH_TRANSCRIPTS="${YOUTUBE_FETCH_TRANSCRIPTS:-1}"
LOCAL_TRANSCRIBE="${YOUTUBE_LOCAL_TRANSCRIBE:-1}"
LOCAL_TRANSCRIBE_LIMIT="${YOUTUBE_LOCAL_TRANSCRIBE_LIMIT:-1}"
AUDIO_DIR="${YOUTUBE_AUDIO_DIR:-data/youtube_audio}"
ASR_MODEL_SIZE="${YOUTUBE_ASR_MODEL_SIZE:-base}"
ASR_DEVICE="${YOUTUBE_ASR_DEVICE:-auto}"
ASR_COMPUTE_TYPE="${YOUTUBE_ASR_COMPUTE_TYPE:-int8}"
KEEP_AUDIO="${YOUTUBE_KEEP_AUDIO:-0}"

cd "$ROOT_DIR"

PYTHON_BIN="${DEADLOCK_BRAIN_PYTHON:-python3}"
if [[ -x "$ROOT_DIR/.venv/bin/python" ]]; then
  PYTHON_BIN="$ROOT_DIR/.venv/bin/python"
fi

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

ARGS=(
  youtube auto-learn
  --config "$YOUTUBE_CONFIG" \
  --transcript-dir "$TRANSCRIPT_DIR" \
  --discover-limit "$DISCOVER_LIMIT" \
  --analyze-limit "$ANALYZE_LIMIT" \
  --transcript-fetch-limit "$TRANSCRIPT_FETCH_LIMIT" \
  --languages "$SUBTITLE_LANGUAGES" \
  --local-transcribe-limit "$LOCAL_TRANSCRIBE_LIMIT" \
  --audio-dir "$AUDIO_DIR" \
  --asr-model-size "$ASR_MODEL_SIZE" \
  --asr-device "$ASR_DEVICE" \
  --asr-compute-type "$ASR_COMPUTE_TYPE" \
  --cache-ttl-seconds "$CACHE_TTL_SECONDS" \
  --delay-seconds "$DELAY_SECONDS"
)

if [[ "$FETCH_TRANSCRIPTS" == "0" || "$FETCH_TRANSCRIPTS" == "false" ]]; then
  ARGS+=(--no-fetch-transcripts)
fi

if [[ "$LOCAL_TRANSCRIBE" == "1" || "$LOCAL_TRANSCRIBE" == "true" ]]; then
  ARGS+=(--local-transcribe)
fi

if [[ "$KEEP_AUDIO" == "1" || "$KEEP_AUDIO" == "true" ]]; then
  ARGS+=(--keep-audio)
fi

"$PYTHON_BIN" -m deadlock_brain.cli "${ARGS[@]}"
