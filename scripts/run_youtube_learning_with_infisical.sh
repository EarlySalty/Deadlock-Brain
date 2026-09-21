#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DEPLOY_PREFLIGHT="${DEPLOY_PREFLIGHT:-$HOME/Documents/Admin-Scripts/deploy-preflight.sh}"
_parent_comm="$(ps -o comm= -p "$PPID" 2>/dev/null || true)"
if [[ "$_parent_comm" == "systemd" ]]; then
  if [[ ! -x "$DEPLOY_PREFLIGHT" ]]; then
    echo "FEHLER: deploy-preflight fehlt unter systemd Start: $DEPLOY_PREFLIGHT" >&2
    exit 1
  fi
  DEPLOY_PREFLIGHT_SYSTEMD_PARENT=1 "$DEPLOY_PREFLIGHT" "$ROOT_DIR" main "deadlock-brain-yt"
elif [[ -x "$DEPLOY_PREFLIGHT" ]]; then
  "$DEPLOY_PREFLIGHT" "$ROOT_DIR" main "deadlock-brain-yt"
fi

LOAD_INFISICAL="${LOAD_INFISICAL:-1}"
YOUTUBE_CONFIG="${YOUTUBE_CONFIG:-config/youtube_feeds.json}"
DISCOVER_LIMIT="${YOUTUBE_DISCOVER_LIMIT:-50}"
ANALYZE_LIMIT="${YOUTUBE_ANALYZE_LIMIT:-5}"
TRANSCRIPT_FETCH_LIMIT="${YOUTUBE_TRANSCRIPT_FETCH_LIMIT:-20}"
FETCH_TRANSCRIPTS="${YOUTUBE_FETCH_TRANSCRIPTS:-1}"
YOUTUBE_BIN="${DEADLOCK_BRAIN_YT_BIN:-$ROOT_DIR/rust/target/release/deadlock-brain-yt}"
SECRET_EXEC="${DEADLOCK_BRAIN_SECRET_EXEC_BIN:-$ROOT_DIR/rust/target/release/deadlock-brain-secret-exec}"
ENABLE_YOUTUBE_SYNC="${DEADLOCK_BRAIN_ENABLE_YOUTUBE_SYNC:-0}"

export DEADLOCK_BRAIN_ROOT="$ROOT_DIR"
cd "$ROOT_DIR"

if [[ "$ENABLE_YOUTUBE_SYNC" != "1" && "$ENABLE_YOUTUBE_SYNC" != "true" ]]; then
  echo "Deadlock Brain YouTube Learning ist deaktiviert."
  exit 0
fi
if [[ ! -x "$YOUTUBE_BIN" ]]; then
  echo "deadlock-brain-yt Release Binary fehlt: $YOUTUBE_BIN" >&2
  exit 1
fi

YOUTUBE_CMD=("$YOUTUBE_BIN")
if [[ "$LOAD_INFISICAL" == "1" || "$LOAD_INFISICAL" == "true" ]]; then
  if [[ ! -x "$SECRET_EXEC" ]]; then
    echo "deadlock-brain-secret-exec Release Binary fehlt: $SECRET_EXEC" >&2
    exit 1
  fi
  YOUTUBE_CMD=("$SECRET_EXEC" -- "$YOUTUBE_BIN")
fi

args=(
  auto-learn
  --config "$YOUTUBE_CONFIG"
  --discover-limit "$DISCOVER_LIMIT"
  --analyze-limit "$ANALYZE_LIMIT"
  --transcript-fetch-limit "$TRANSCRIPT_FETCH_LIMIT"
)
if [[ "$FETCH_TRANSCRIPTS" == "0" || "$FETCH_TRANSCRIPTS" == "false" ]]; then
  args+=(--no-fetch-transcripts)
fi

"${YOUTUBE_CMD[@]}" "${args[@]}"
