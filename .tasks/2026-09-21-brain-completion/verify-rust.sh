#!/usr/bin/env bash
set -uo pipefail
umask 077
ROOT=/home/nathanael/.worktrees/brain-release-completion-20260921
OUT=${1:-/home/nathanael/.local/share/deadlock-brain/releases/20260921-continuation}
TARGET=/home/nathanael/.cache/deadlock-brain-final-20260921
mkdir -p "$OUT"
cd "$ROOT/rust" || exit 1
export SQLX_OFFLINE=true
/home/nathanael/.cargo/bin/cargo test --locked --workspace --all-targets -j 2 --target-dir "$TARGET" >"$OUT/workspace-test.log" 2>&1
result=$?
printf '%s\n' "$result" >"$OUT/workspace-test.exit"
if [ "$result" -ne 0 ]; then exit "$result"; fi
/home/nathanael/.cargo/bin/cargo clippy --locked --workspace --all-targets -j 2 --target-dir "$TARGET" >"$OUT/workspace-clippy.log" 2>&1
result=$?
printf '%s\n' "$result" >"$OUT/workspace-clippy.exit"
exit "$result"
