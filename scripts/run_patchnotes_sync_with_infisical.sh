#!/usr/bin/env bash
set -euo pipefail

DEFAULT_ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ROOT_DIR="${DEADLOCK_BRAIN_ROOT:-$DEFAULT_ROOT_DIR}"
CONFIG_FILE="${INFISICAL_CONFIG_FILE:-/home/naniadm/.config/deadlock-bots/infisical.env}"
LOAD_INFISICAL="${LOAD_INFISICAL:-1}"
INFISICAL_RETRY_DELAY="${INFISICAL_RETRY_DELAY:-5}"
INFISICAL_MAX_ATTEMPTS="${INFISICAL_MAX_ATTEMPTS:-0}"

BRAIN_BIN="${DEADLOCK_BRAIN_BIN:-$ROOT_DIR/rust/target/release/deadlock-brain}"
BRAIN_DB="${DEADLOCK_BRAIN_DB_PATH:-$ROOT_DIR/data/deadlock_brain.sqlite3}"
PATCH_DB="${DEADLOCK_DB_PATH:-/home/naniadm/Documents/Deadlock-Bots/data/deadlock.sqlite3}"
BACKUP_DIR="${DEADLOCK_BRAIN_BACKUP_DIR:-$ROOT_DIR/data/backups}"
FORCE="${DEADLOCK_BRAIN_PATCH_SYNC_FORCE:-0}"

cd "$ROOT_DIR"

PYTHON_BIN="${DEADLOCK_BRAIN_PYTHON:-python3}"
if [[ -x "$ROOT_DIR/.venv/bin/python" ]]; then
  PYTHON_BIN="$ROOT_DIR/.venv/bin/python"
fi

export PYTHONDONTWRITEBYTECODE=1
export PYTHONPATH="${PYTHONPATH:-$ROOT_DIR/src}"
export PYTHONUNBUFFERED=1

if [[ "$LOAD_INFISICAL" == "1" || "$LOAD_INFISICAL" == "true" ]]; then
  if [[ ! -f "$CONFIG_FILE" ]]; then
    echo "Infisical config not found: $CONFIG_FILE" >&2
    exit 1
  fi

  set -a
  source "$CONFIG_FILE"
  set +a

  attempt=0
  while true; do
    if INFISICAL_EXPORT="$("$PYTHON_BIN" scripts/export_infisical_env.py --format shell)"; then
      eval "$INFISICAL_EXPORT"
      break
    fi

    attempt=$((attempt + 1))
    if [[ "$INFISICAL_MAX_ATTEMPTS" -gt 0 && "$attempt" -ge "$INFISICAL_MAX_ATTEMPTS" ]]; then
      echo "Infisical secrets could not be loaded after $attempt attempt(s)." >&2
      exit 1
    fi

    echo "Infisical not ready for Deadlock Brain patchnotes sync, retrying in ${INFISICAL_RETRY_DELAY}s (attempt $attempt)." >&2
    sleep "$INFISICAL_RETRY_DELAY"
  done
fi

if [[ ! -x "$BRAIN_BIN" ]]; then
  echo "Brain binary not executable: $BRAIN_BIN" >&2
  exit 1
fi
if [[ ! -f "$BRAIN_DB" ]]; then
  echo "Brain DB not found: $BRAIN_DB" >&2
  exit 1
fi
if [[ ! -f "$PATCH_DB" ]]; then
  echo "Patchnotes DB not found: $PATCH_DB" >&2
  exit 1
fi

latest_patch_id="$(
  "$PYTHON_BIN" - "$PATCH_DB" <<'PY'
import sqlite3
import sys

conn = sqlite3.connect(sys.argv[1])
try:
    row = conn.execute(
        "SELECT COALESCE(MAX(id), 0) FROM changelog_posts "
        "WHERE raw_content IS NOT NULL AND raw_content != ''"
    ).fetchone()
    print(int(row[0] or 0))
finally:
    conn.close()
PY
)"

latest_brain_patch_id="$(
  "$PYTHON_BIN" - "$BRAIN_DB" <<'PY'
import sqlite3
import sys

conn = sqlite3.connect(sys.argv[1])
try:
    row = conn.execute(
        "SELECT COALESCE(MAX(CAST(json_extract(payload_json,'$.id') AS INTEGER)), 0) "
        "FROM entity_snapshots "
        "WHERE source='deadlock_patchnotes_db' AND entity_type='patchnote'"
    ).fetchone()
    print(int(row[0] or 0))
finally:
    conn.close()
PY
)"

if [[ "$FORCE" != "1" && "$FORCE" != "true" && "$latest_patch_id" -le "$latest_brain_patch_id" ]]; then
  echo "Deadlock Brain patchnotes sync: no new patchnotes (patch_db=$latest_patch_id brain=$latest_brain_patch_id)"
  exit 0
fi

mkdir -p "$BACKUP_DIR"
backup_path="$BACKUP_DIR/deadlock_brain.pre-patchnotes-sync.$(date +%Y%m%d-%H%M%S).sqlite3"
cp -a "$BRAIN_DB" "$backup_path"
echo "Deadlock Brain patchnotes sync: backup=$backup_path patch_db=$latest_patch_id brain_before=$latest_brain_patch_id"

# Trusted game data first: this updates current hero/item/ability facts from original data.
"$BRAIN_BIN" --db "$BRAIN_DB" pull deadlock-data
"$BRAIN_BIN" --db "$BRAIN_DB" pull assets --kind items --kind heroes
"$BRAIN_BIN" --db "$BRAIN_DB" normalize entities --rebuild

# Patchnotes are imported from changelog_posts.raw_content. translated_content is not parsed
# unless a legacy row has no original text at all.
"$BRAIN_BIN" --db "$BRAIN_DB" pull patchnotes --db-path "$PATCH_DB"
"$BRAIN_BIN" --db "$BRAIN_DB" parse patchnotes
"$BRAIN_BIN" --db "$BRAIN_DB" enrich patch-events
"$BRAIN_BIN" --db "$BRAIN_DB" enrich lineage
"$BRAIN_BIN" --db "$BRAIN_DB" enrich legacy-entities
"$BRAIN_BIN" --db "$BRAIN_DB" normalize resolve-gaps
"$BRAIN_BIN" --db "$BRAIN_DB" quality > "$ROOT_DIR/data/last_patchnotes_sync_quality.json"

echo "Deadlock Brain patchnotes sync: done"
