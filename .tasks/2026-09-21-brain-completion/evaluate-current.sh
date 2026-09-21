#!/usr/bin/env bash
# Existing planner only; no network, credentials, writes to Central, or relaxed gates.
set -Eeuo pipefail
umask 077
BASE=${1:-/home/nathanael/.local/share/deadlock-brain/releases/20260921-continuation/current}
OUT=$BASE/evaluation
BIN=/home/nathanael/.cache/deadlock-brain-final-20260921/debug/examples/family_evaluation
mkdir "$OUT"
phase=unchanged-holdout-inputs
trap 'result=$?; printf "%s\n" "$result" >"$OUT/evaluation.exit"; printf "%s\n" "$phase" >"$OUT/evaluation.last-phase"; exit "$result"' EXIT
cmp --silent <(jq -cS '{config,heroes:[.heroes[]|{meta,events}]}' "$BASE/family-input-current.json") <(jq -cS '{config,heroes:[.heroes[]|{meta,events}]}' "$BASE/family-input-current-assets.json")
printf 'Population, authors, events and configuration are unchanged.\n' >"$OUT/input-contract.log"
sha256sum "$BIN" "$BASE/family-input-current.json" "$BASE/family-input-current-assets.json" >"$OUT/inputs.sha256"
phase=plan
"$BIN" plan "$BASE/family-input-current-assets.json" "$OUT/plan.json" >"$OUT/plan.log" 2>&1
printf '0\n' >"$OUT/plan.exit"
phase=holdout
"$BIN" holdout "$BASE/family-input-current-assets.json" "$OUT/holdout.json" >"$OUT/holdout.log" 2>&1
printf '0\n' >"$OUT/holdout.exit"
phase=three-replays
"$BIN" replay "$BASE/family-input-current-assets.json" "$OUT/replays" >"$OUT/replays.log" 2>&1
phase=verified
