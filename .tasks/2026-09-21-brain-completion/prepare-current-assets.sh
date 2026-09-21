#!/usr/bin/env bash
# One isolated evaluation: current assets, unchanged source observations, no production writes.
set -Eeuo pipefail
umask 077
ROOT=/home/nathanael/.worktrees/brain-release-completion-20260921
OUT=${1:-/home/nathanael/.local/share/deadlock-brain/releases/20260921-continuation/current}
TARGET=/home/nathanael/.cache/deadlock-brain-final-20260921
TASK=$ROOT/.tasks/2026-09-21-brain-completion
mkdir -p "$OUT/runtime-root"
phase=workspace
trap 'result=$?; printf "%s\n" "$result" >"$OUT/prepare.exit"; printf "%s\n" "$phase" >"$OUT/prepare.last-phase"; unset DEADLOCK_CENTRAL_DSN source_dsn test_dsn base_dsn query_suffix DEADLOCK_BRAIN_ROOT; exit "$result"' EXIT
bash "$TASK/verify-rust.sh" "$OUT"
phase=debug-build
cd "$ROOT/rust"
export SQLX_OFFLINE=true
/home/nathanael/.cargo/bin/cargo build --locked --workspace --bins --examples -j 2 --target-dir "$TARGET" >"$OUT/debug-build.log" 2>&1
printf '0\n' >"$OUT/debug-build.exit"
phase=authorized-access
eval "$(/home/naniadm/Documents/Infisical/export_gpt_secret.py --secret DEADLOCK_CENTRAL_DSN)"
[[ -n "${DEADLOCK_CENTRAL_DSN:-}" ]]
source_dsn=$DEADLOCK_CENTRAL_DSN
case "$source_dsn" in postgresql://*|postgres://*) ;; *) exit 2 ;; esac
export DEADLOCK_BRAIN_ROOT="$OUT/runtime-root"
phase=read-only-family-freeze
"$TARGET/debug/examples/family_evaluation" freeze "$OUT/family-input-current.json" 'Warden,Infernus,Vindicta,Lady Geist,Abrams,Viscous,Shiv,Haze' >"$OUT/family-freeze.log" 2>/dev/null
printf '0\n' >"$OUT/family-freeze.exit"
phase=create-isolated-assets
name="brain_assets_eval_20260921_$(date +%H%M%S)_$$"
[[ "$name" =~ ^brain_assets_eval_[0-9_]+$ ]]
psql --dbname="$source_dsn" -X -w -v ON_ERROR_STOP=1 -c "CREATE DATABASE $name" >/dev/null 2>/dev/null
printf '%s\n' "$name" >"$OUT/owned-asset-database.txt"
base_dsn=${source_dsn%%\?*}; base_dsn=${base_dsn%/*}
query_suffix=
if [[ "$source_dsn" == *\?* ]]; then query_suffix="?${source_dsn#*\?}"; fi
test_dsn="$base_dsn/$name$query_suffix"
actual=$(psql --dbname="$test_dsn" -X -w -Atqc 'SELECT current_database()' 2>/dev/null)
[[ "$actual" == "$name" ]]
psql --dbname="$test_dsn" -X -w -v ON_ERROR_STOP=1 -f "$TASK/asset-scratch-schema.sql" >/dev/null 2>/dev/null
phase=preserve-wiki-item-cards
psql --dbname="$source_dsn" -X -w -q -v ON_ERROR_STOP=1 -c "COPY (SELECT DISTINCT ON (external_id) source, entity_type, external_id, canonical_name, payload_hash, payload, fetched_at FROM brain.entity_snapshots WHERE source='deadlock_data' AND entity_type='item_card' ORDER BY external_id,fetched_at DESC,id DESC) TO STDOUT" 2>/dev/null |
  psql --dbname="$test_dsn" -X -w -q -v ON_ERROR_STOP=1 -c 'COPY brain.entity_snapshots(source,entity_type,external_id,canonical_name,payload_hash,payload,fetched_at) FROM STDIN' >/dev/null 2>/dev/null
unset source_dsn base_dsn query_suffix
export DEADLOCK_CENTRAL_DSN="$test_dsn"
unset test_dsn
phase=official-asset-import
"$TARGET/debug/deadlock-brain" pull assets >"$OUT/assets-refresh.json" 2>/dev/null
printf '0\n' >"$OUT/assets-refresh.exit"
phase=shared-catalog-classification
"$TARGET/debug/examples/scratch_asset_catalog" >"$OUT/scratch-catalog.log" 2>/dev/null
phase=read-only-asset-rebase
"$TARGET/debug/examples/family_evaluation" rebase-assets "$OUT/family-input-current.json" "$OUT/family-input-current-assets.json" >"$OUT/family-rebase.log" 2>/dev/null
unset DEADLOCK_CENTRAL_DSN DEADLOCK_BRAIN_ROOT
phase=inspect-frozen-input
"$TARGET/debug/examples/family_evaluation" inspect "$OUT/family-input-current-assets.json" >"$OUT/family-input-summary.log" 2>&1
phase=prepared
printf 'CURRENT_ASSETS_PREPARED: isolated import and read-only model rebase passed.\n'
