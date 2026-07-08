#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

export PYTHONDONTWRITEBYTECODE=1
TMP_DIR="${TMPDIR:-/tmp}/deadlock_brain_smoke_${USER:-user}_$$"
mkdir -p "$TMP_DIR"
BRAIN_BIN="${DEADLOCK_BRAIN_BIN:-$PWD/rust/target/release/deadlock-brain}"

if [[ ! -x "$BRAIN_BIN" ]]; then
  echo "deadlock-brain release binary missing: $BRAIN_BIN" >&2
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

python3 -m json.tool "$TMP_DIR/context.json" >/dev/null
python3 -m json.tool "$TMP_DIR/timeline.json" >/dev/null
python3 -m json.tool "$TMP_DIR/review.json" >/dev/null
python3 -m json.tool "$TMP_DIR/lineage.json" >/dev/null
python3 -m json.tool "$TMP_DIR/legacy.json" >/dev/null
python3 -m json.tool "$TMP_DIR/item_refresher.json" >/dev/null
python3 -m json.tool "$TMP_DIR/build_mo_krill.json" >/dev/null
python3 -m json.tool "$TMP_DIR/analysis_save.json" >/dev/null
python3 -m json.tool "$TMP_DIR/fireworks_dry_run.json" >/dev/null
python3 -m json.tool "$TMP_DIR/analysis_list.json" >/dev/null
python3 -m json.tool "$TMP_DIR/player_matches.json" >/dev/null
python3 -m json.tool "$TMP_DIR/quality.json" >/dev/null

echo "Smoke check ok"
