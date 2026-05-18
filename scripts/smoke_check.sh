#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

export PYTHONDONTWRITEBYTECODE=1
export PYTHONPATH=src
TMP_DIR="${TMPDIR:-/tmp}/deadlock_brain_smoke_${USER:-user}_$$"
mkdir -p "$TMP_DIR"

python3 -m py_compile $(find src -name '*.py')

python3 -m deadlock_brain.cli status >"$TMP_DIR/status.txt"
python3 -m deadlock_brain.cli context Stalker --limit-events 5 >"$TMP_DIR/context.json"
python3 -m deadlock_brain.cli timeline Backstabber --limit-events 20 >"$TMP_DIR/timeline.json"
python3 -m deadlock_brain.cli review Indomitable --limit-events 20 >"$TMP_DIR/review.json"
python3 -m deadlock_brain.cli lineage Stalker >"$TMP_DIR/lineage.json"
python3 -m deadlock_brain.cli legacy --limit 20 >"$TMP_DIR/legacy.json"
python3 -m deadlock_brain.cli item Refresher >"$TMP_DIR/item_refresher.json"
python3 -m deadlock_brain.cli build "Mo & Krill" >"$TMP_DIR/build_mo_krill.json"
python3 -m deadlock_brain.cli analysis save-review Stalker >"$TMP_DIR/analysis_save.json"
python3 -m deadlock_brain.cli analysis run-minimax Stalker --dry-run >"$TMP_DIR/minimax_dry_run.json"
python3 -m deadlock_brain.cli analysis list --limit 5 >"$TMP_DIR/analysis_list.json"
python3 -m deadlock_brain.cli player list-matches --limit 5 >"$TMP_DIR/player_matches.json"
python3 -m deadlock_brain.cli quality >"$TMP_DIR/quality.json"

python3 -m json.tool "$TMP_DIR/context.json" >/dev/null
python3 -m json.tool "$TMP_DIR/timeline.json" >/dev/null
python3 -m json.tool "$TMP_DIR/review.json" >/dev/null
python3 -m json.tool "$TMP_DIR/lineage.json" >/dev/null
python3 -m json.tool "$TMP_DIR/legacy.json" >/dev/null
python3 -m json.tool "$TMP_DIR/item_refresher.json" >/dev/null
python3 -m json.tool "$TMP_DIR/build_mo_krill.json" >/dev/null
python3 -m json.tool "$TMP_DIR/analysis_save.json" >/dev/null
python3 -m json.tool "$TMP_DIR/minimax_dry_run.json" >/dev/null
python3 -m json.tool "$TMP_DIR/analysis_list.json" >/dev/null
python3 -m json.tool "$TMP_DIR/player_matches.json" >/dev/null
python3 -m json.tool "$TMP_DIR/quality.json" >/dev/null

echo "Smoke check ok"
