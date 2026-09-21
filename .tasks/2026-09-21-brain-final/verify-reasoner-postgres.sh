#!/usr/bin/env bash
# Destruktive Fixtures nur in einer neu angelegten, reservierten Testdatenbank.
set -euo pipefail
umask 077
cd /home/nathanael/.worktrees/brain-final-integration-20260921
OUT=/home/nathanael/.local/share/deadlock-brain/releases/20260921-final
eval "$(/home/naniadm/Documents/Infisical/export_gpt_secret.py --secret DEADLOCK_CENTRAL_DSN)"
trap 'unset DEADLOCK_CENTRAL_DSN REASONER_SCRATCH_DSN base_dsn query_suffix' EXIT
: "${DEADLOCK_CENTRAL_DSN:?Autorisierter Datenbankzugang fehlt}"
name="reasoner_a_fix_20260921_$(date +%H%M%S)_$$"
if ! psql --dbname="$DEADLOCK_CENTRAL_DSN" -X -w -v ON_ERROR_STOP=1 -c "CREATE DATABASE $name" >/dev/null 2>/dev/null; then
  printf 'Scratch-Datenbank konnte nicht neu angelegt werden; kein Überschreiben.\n' >&2; exit 1
fi
printf '%s\n' "$name" >>"$OUT/owned-test-databases.txt"
base_dsn=${DEADLOCK_CENTRAL_DSN%%\?*}
base_dsn=${base_dsn%/*}
query_suffix=
if [[ "$DEADLOCK_CENTRAL_DSN" == *\?* ]]; then query_suffix="?${DEADLOCK_CENTRAL_DSN#*\?}"; fi
export REASONER_SCRATCH_DSN="$base_dsn/$name$query_suffix"
actual=$(psql --dbname="$REASONER_SCRATCH_DSN" -X -w -Atqc 'SELECT current_database()' 2>/dev/null)
[[ "$actual" == "$name" ]] || exit 1
unset DEADLOCK_CENTRAL_DSN base_dsn query_suffix
cd rust
/home/nathanael/.cargo/bin/cargo test --locked -p dbrain-reasoner --lib -j 2 --target-dir /home/nathanael/.cache/deadlock-brain-final-20260921 -- \
  --ignored --test-threads=1 \
  --skip fix_e_live_warden_evidence \
  --skip loads_warden_and_reference_items_from_real_snapshot \
  --skip scores_warden_reference_items_from_real_snapshot
