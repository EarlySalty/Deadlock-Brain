#!/bin/bash
set -euo pipefail
mode=${1:?usage: release_check.sh before|after <state-dir> [previous-sha]}
state=${2:?state dir}
here=$(cd "$(dirname "$0")" && pwd)
harness="$here/../2026-10-29-patch-history-answers"
mkdir -p "$state"
out="$state/$mode.jsonl"
: > "$out"
started=$(date '+%Y-%m-%d %H:%M:%S')
eval "$(/home/nathanael/Documents/Infisical/export_claude_secret.py --secret DISCORD_BRAIN_CLIENT_TOKEN)"
PROBE_PREFIX=probe-bdc- python3 -I "$harness/run_probes.py" "$here/probes.json" "$out" ${PROBE_ONLY:+"$PROBE_ONLY"}
unset DISCORD_BRAIN_CLIENT_TOKEN
PROBE_PREFIX=probe-bdc- python3 -I "$harness/enrich.py" "$out" "$started" > "$state/$mode.enriched.jsonl" 2> "$state/$mode.cost.json"
cat "$state/$mode.cost.json"
if [ "$mode" = after ]; then
  if ! python3 -I "$here/compare.py" "$state/before.jsonl" "$out"; then
    echo "probe regression; roll back with: sudo -n -- /usr/local/libexec/brain-release rollback ${3:-<previous sha>}" >&2
    exit 1
  fi
fi
