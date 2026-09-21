#!/usr/bin/env bash
# Sequenzielle DB-Verträge mit gesonderten Abschlusscodes.
set -uo pipefail
umask 077
TASK=/home/nathanael/.worktrees/brain-final-integration-20260921/.tasks/2026-09-21-brain-final
OUT=/home/nathanael/.local/share/deadlock-brain/releases/20260921-final
mkdir -p "$OUT"
export SQLX_OFFLINE=true
bash "$TASK/verify-postgres.sh" >"$OUT/postgres-test.log" 2>&1
result=$?
printf '%s\n' "$result" >"$OUT/postgres-test.exit"
if [ "$result" -ne 0 ]; then exit "$result"; fi
bash "$TASK/verify-reasoner-postgres.sh" >"$OUT/reasoner-postgres-test.log" 2>&1
result=$?
printf '%s\n' "$result" >"$OUT/reasoner-postgres-test.exit"
exit "$result"
