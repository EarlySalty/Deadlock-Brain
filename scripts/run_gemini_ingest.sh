#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

# One-time setup: open normal Brave with this profile and log in manually.
# Google blocks automated login; unattended cron must run headful under xvfb-run.
# Example:
#   /opt/brave.com/brave/brave --user-data-dir="$PWD/data/gemini_profile" --password-store=basic https://gemini.google.com/app

LIMIT="${GEMINI_INGEST_LIMIT:-${1:-5}}"
LOGFILE="${GEMINI_INGEST_LOG:-data/gemini_ingest.log}"
BIN="${GEMINI_INGEST_BIN:-./rust/target/release/deadlock-brain-yt}"

mkdir -p data
mkdir -p "$(dirname "$LOGFILE")"

if [[ ! -x "$BIN" ]]; then
  echo "deadlock-brain-yt release binary missing: $BIN" >&2
  exit 1
fi

{
  printf '[%s] start gemini ingest limit=%s\n' "$(date --iso-8601=seconds)" "$LIMIT"
  set +e
  xvfb-run -a "$BIN" ingest --limit "$LIMIT"
  status=$?
  set -e
  printf '[%s] done gemini ingest status=%s\n' "$(date --iso-8601=seconds)" "$status"
  exit "$status"
} >> "$LOGFILE" 2>&1
