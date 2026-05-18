#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."
export PYTHONPATH="${PYTHONPATH:-}:$PWD/src"

python3 -m deadlock_brain.cli pull assets
python3 -m deadlock_brain.cli pull patchnotes
python3 -m deadlock_brain.cli refresh-sheet
python3 -m deadlock_brain.cli normalize entities
python3 -m deadlock_brain.cli normalize sheet-stats
python3 -m deadlock_brain.cli normalize sheet-tabs
python3 -m deadlock_brain.cli status

