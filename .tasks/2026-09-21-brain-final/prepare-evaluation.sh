#!/usr/bin/env bash
# Nur Debug-Binaries und lesende, konsistente Eingaben für die Messung.
set -uo pipefail
umask 077
ROOT=/home/nathanael/.worktrees/brain-final-integration-20260921
OUT=/home/nathanael/.local/share/deadlock-brain/releases/20260921-final
TARGET=/home/nathanael/.cache/deadlock-brain-final-20260921
mkdir -p "$OUT"
cd "$ROOT/rust"
export SQLX_OFFLINE=true
/home/nathanael/.cargo/bin/cargo build --locked --workspace --bins --examples -j 2 --target-dir "$TARGET" >"$OUT/debug-build.log" 2>&1
result=$?
printf '%s\n' "$result" >"$OUT/debug-build.exit"
if [ "$result" -ne 0 ]; then exit "$result"; fi
cd "$ROOT"
eval "$(/home/naniadm/Documents/Infisical/export_gpt_secret.py --secret DEADLOCK_CENTRAL_DSN)"
trap 'unset DEADLOCK_CENTRAL_DSN' EXIT
if [ -z "${DEADLOCK_CENTRAL_DSN:-}" ]; then printf 'Autorisierter Datenbankzugang fehlt.\n' >&2; exit 1; fi
"$TARGET/debug/examples/family_evaluation" freeze "$OUT/family-input-current.json" 'Warden,Infernus,Vindicta,Lady Geist,Abrams,Viscous,Shiv,Haze' >"$OUT/family-freeze.log" 2>&1
result=$?
unset DEADLOCK_CENTRAL_DSN
printf '%s\n' "$result" >"$OUT/family-freeze.exit"
if [ "$result" -ne 0 ]; then exit "$result"; fi
"$TARGET/debug/examples/family_evaluation" inspect "$OUT/family-input-current.json" >"$OUT/family-input-summary.log" 2>&1
result=$?
printf '%s\n' "$result" >"$OUT/family-input-summary.exit"
exit "$result"
