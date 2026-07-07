#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

BRAIN_BIN="${DEADLOCK_BRAIN_BIN:-$PWD/rust/target/release/deadlock-brain}"

if [[ ! -x "$BRAIN_BIN" ]]; then
  echo "deadlock-brain release binary missing: $BRAIN_BIN" >&2
  exit 1
fi

"$BRAIN_BIN" pull assets
"$BRAIN_BIN" pull patchnotes
"$BRAIN_BIN" refresh-sheet
"$BRAIN_BIN" status
