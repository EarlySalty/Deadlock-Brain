#!/usr/bin/env bash
# Ausschließlich gespeicherte Eingaben, keine DB, keine KI, keine Veröffentlichung.
set -uo pipefail
umask 077
OUT=/home/nathanael/.local/share/deadlock-brain/releases/20260921-final
BIN=/home/nathanael/.cache/deadlock-brain-final-20260921/debug/examples/family_evaluation
INPUT="$OUT/family-input-current.json"
"$BIN" plan "$INPUT" "$OUT/family-plan-before-refresh.json" >"$OUT/family-plan-before-refresh.log" 2>&1
result=$?
printf '%s\n' "$result" >"$OUT/family-plan-before-refresh.exit"
if [ "$result" -ne 0 ]; then exit "$result"; fi
"$BIN" holdout "$INPUT" "$OUT/family-holdout-before-refresh.json" >"$OUT/family-holdout-before-refresh.log" 2>&1
result=$?
printf '%s\n' "$result" >"$OUT/family-holdout-before-refresh.exit"
exit "$result"
