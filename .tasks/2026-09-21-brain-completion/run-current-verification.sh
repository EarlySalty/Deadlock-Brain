#!/usr/bin/env bash
set -euo pipefail
TASK=/home/nathanael/.worktrees/brain-release-completion-20260921/.tasks/2026-09-21-brain-completion
OUT=/home/nathanael/.local/share/deadlock-brain/releases/20260921-continuation/roundtrip
bash "$TASK/prepare-current-assets.sh" "$OUT"
bash "$TASK/evaluate-current.sh" "$OUT"
