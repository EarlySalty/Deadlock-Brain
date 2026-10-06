#!/usr/bin/env bash
# A durable verification service avoids the terminal worker's execution limit.
set -uo pipefail
umask 077
OUT=/home/nathanael/.local/share/deadlock-brain/releases/20260921-continuation/roundtrip/evaluation
BIN=/home/nathanael/.cache/deadlock-brain-final-20260921/debug/examples/family_evaluation
INPUT=/home/nathanael/.local/share/deadlock-brain/releases/20260921-continuation/roundtrip/family-input-current-assets.json
sha256sum "$BIN" "$INPUT" >"$OUT/durable-inputs.sha256"
"$BIN" replay "$INPUT" "$OUT/replays-durable" >"$OUT/replays-durable.log" 2>&1
result=$?
printf '%s\n' "$result" >"$OUT/replays-durable.exit"
exit "$result"
