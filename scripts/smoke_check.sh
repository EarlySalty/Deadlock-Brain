#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

TMP_DIR="${TMPDIR:-/tmp}/deadlock_brain_smoke_${USER:-user}_$$"
mkdir -p "$TMP_DIR"
BRAIN_BIN="${DEADLOCK_BRAIN_BIN:-$ROOT_DIR/rust/target/release/deadlock-brain}"

if [[ ! -x "$BRAIN_BIN" ]]; then
  echo "deadlock-brain Release Binary fehlt: $BRAIN_BIN" >&2
  exit 1
fi
if ! command -v jq >/dev/null 2>&1; then
  echo "jq fehlt für die JSON Smoke Prüfung." >&2
  exit 1
fi

"$BRAIN_BIN" status >"$TMP_DIR/status.txt"
"$BRAIN_BIN" context Stalker --limit-events 5 >"$TMP_DIR/context.json"
"$BRAIN_BIN" timeline Backstabber --limit-events 20 >"$TMP_DIR/timeline.json"
"$BRAIN_BIN" review Indomitable --limit-events 20 >"$TMP_DIR/review.json"
"$BRAIN_BIN" lineage Stalker >"$TMP_DIR/lineage.json"
"$BRAIN_BIN" legacy --limit 20 >"$TMP_DIR/legacy.json"
"$BRAIN_BIN" item Refresher >"$TMP_DIR/item_refresher.json"
"$BRAIN_BIN" build "Mo & Krill" >"$TMP_DIR/build_mo_krill.json"
"$BRAIN_BIN" analysis save-review Stalker >"$TMP_DIR/analysis_save.json"
"$BRAIN_BIN" analysis run-fireworks Stalker --dry-run >"$TMP_DIR/fireworks_dry_run.json"
"$BRAIN_BIN" analysis list --limit 5 >"$TMP_DIR/analysis_list.json"
"$BRAIN_BIN" player list-matches --limit 5 >"$TMP_DIR/player_matches.json"
"$BRAIN_BIN" quality >"$TMP_DIR/quality.json"

for file in "$TMP_DIR"/*.json; do
  jq empty "$file"
done

echo "Smoke Check erfolgreich"
