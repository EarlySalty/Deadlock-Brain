#!/usr/bin/env bash
# Read installed production state without deploying a candidate or persisting builds.
set -uo pipefail
umask 077
ROOT=/home/nathanael/.worktrees/brain-release-completion-20260921
OUT=/home/nathanael/.local/share/deadlock-brain/releases/20260921-continuation/live-read-only
INSTALLED=/home/nathanael/.local/share/deadlock-brain/wiki-refresh/deadlock-brain
CANDIDATE=/home/nathanael/.cache/deadlock-brain-final-20260921/debug
WIKI=/home/nathanael/.local/share/deadlock-brain/game-wiki/current
mkdir -p "$OUT/runtime-root"
export DEADLOCK_BRAIN_ROOT="$OUT/runtime-root"
trap 'unset DEADLOCK_CENTRAL_DSN DEADLOCK_BRAIN_ROOT' EXIT
eval "$(/home/naniadm/Documents/Infisical/export_gpt_secret.py --secret DEADLOCK_CENTRAL_DSN)"
[[ -n "${DEADLOCK_CENTRAL_DSN:-}" ]] || exit 2
readlink -f "$INSTALLED" >"$OUT/installed-path.txt"
sha256sum "$INSTALLED" "$CANDIDATE/deadlock-brain-mcp" >"$OUT/binary-sha256.txt"
date --iso-8601=seconds >"$OUT/checked-at.txt"
run_json() {
    local label=$1; shift
    timeout 240 "$@" >"$OUT/$label.json" 2>/dev/null
    local result=$?
    if [[ "$result" == 0 ]]; then jq -e 'type == "object" and length > 0' "$OUT/$label.json" >/dev/null 2>/dev/null; result=$?; fi
    printf '%s\n' "$result" >"$OUT/$label.exit"
}
run_json installed-hero "$INSTALLED" ask-context 'Welche Fähigkeiten hat Warden und was ist seine Rolle?' --game-wiki-dir "$WIKI" --pretty
run_json installed-scaling "$INSTALLED" ask-context 'Wie skaliert Wardens Alchemical Flask mit Spirit und welche Effekte hat die Fähigkeit?' --game-wiki-dir "$WIKI" --pretty
run_json installed-warden-build "$INSTALLED" reason build Warden --no-ai --no-persist --json
run_json installed-warden-backtest "$INSTALLED" reason backtest --hero Warden --no-persist --json
timeout 60 "$CANDIDATE/deadlock-brain-mcp" <"$ROOT/.tasks/2026-09-21-brain-completion/mcp-smoke-input.jsonl" >"$OUT/candidate-mcp.jsonl" 2>/dev/null
result=$?
if [[ "$result" == 0 ]]; then
  jq -se 'length == 4 and all(.[]; .error == null and .result != null and .result.isError != true) and (.[1].result.tools | length == 5) and (.[0].result.serverInfo.name == "dl-brain")' "$OUT/candidate-mcp.jsonl" >/dev/null 2>/dev/null
  result=$?
fi
printf '%s\n' "$result" >"$OUT/candidate-mcp.exit"
unset DEADLOCK_CENTRAL_DSN
systemctl --user show deadlock-brain-wiki-refresh.service deadlock-brain-build-data.service deadlock-brain-sheet-sync.service deadlock-brain-youtube-learning.service --property=Id,LoadState,ActiveState,SubState,Result,ExecMainStatus,ExecMainStartTimestamp,ExecMainExitTimestamp,NRestarts,FragmentPath >"$OUT/unit-state.txt"
systemctl --user show deadlock-brain-wiki-refresh.timer --property=Id,ActiveState,SubState,LastTriggerUSec,NextElapseUSecRealtime >"$OUT/wiki-timer.txt"
printf '0\n' >"$OUT/collection.exit"
