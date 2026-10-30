#!/bin/bash
set -euo pipefail

repo=${BRAIN_GUARD_REPO:-/home/nathanael/repos/Deadlock-Brain}
root=${BRAIN_GUARD_STATE:-/home/nathanael/.local/state/brain-probe/deploy}
current=/opt/deadlock-brain/current
harness=.tasks/2026-10-29-brain-data-coverage
retry_wait=${BRAIN_GUARD_RETRY_WAIT:-300}

mkdir -p "$root"
exec 9>"$root/.lock"
flock 9

live_sha() {
  basename "$(readlink -f "$current")"
}

serving() {
  local sha=$1 pid exe
  curl -fsS -o /dev/null --max-time 5 http://127.0.0.1:8788/readyz || return 1
  pid=$(systemctl show brain-serve -p ExecMainPID --value)
  exe=$(readlink "/proc/$pid/exe" 2>/dev/null || true)
  [[ $exe == /opt/deadlock-brain/releases/$sha/* ]]
}

wait_serving() {
  local sha=$1
  for _ in $(seq 60); do
    serving "$sha" && return 0
    sleep 5
  done
  echo "brain-serve did not come up on $sha" >&2
  return 1
}

outage_ids() {
  python3 -I -c '
import json, sys
ids = []
for line in open(sys.argv[1]):
    if line.strip():
        row = json.loads(line)
        status = str(row.get("final_status") or "")
        if status in ("unavailable", "client_error") or status.startswith("http_"):
            ids.append(row["id"])
print(",".join(ids))
' "$1"
}

merge_retry() {
  python3 -I -c '
import json, sys
retry = {}
for line in open(sys.argv[2]):
    if line.strip():
        row = json.loads(line)
        retry[row["id"]] = line.rstrip("\n")
out = []
for line in open(sys.argv[1]):
    if line.strip():
        out.append(retry.get(json.loads(line)["id"], line.rstrip("\n")))
open(sys.argv[1], "w").write("\n".join(out) + "\n")
' "$1" "$2"
}

main() {
  local sha previous dir tree ids status
  sha=$(live_sha)
  previous=$(cat "$root/last-sha" 2>/dev/null || true)
  if [ "$sha" = "$previous" ] && [ "${1:-}" != "--force" ]; then
    echo "guard already ran for $sha"
    return 0
  fi
  wait_serving "$sha"
  dir="$root/$(date -u +%Y%m%dT%H%M%SZ)-${sha:0:8}"
  tree="$dir/tree"
  mkdir -p "$tree"
  git -C "$repo" fetch -q origin
  git -C "$repo" archive "$sha" "$harness" .tasks/2026-10-29-patch-history-answers | tar -x -C "$tree"
  bash "$tree/$harness/release_check.sh" run "$dir" >/dev/null
  cp "$dir/run.jsonl" "$dir/after.jsonl"
  ids=$(outage_ids "$dir/after.jsonl")
  if [ -n "$ids" ]; then
    echo "retrying unavailable probes after ${retry_wait}s: $ids"
    sleep "$retry_wait"
    wait_serving "$sha"
    PROBE_ONLY=$ids bash "$tree/$harness/release_check.sh" retry "$dir" >/dev/null
    merge_retry "$dir/after.jsonl" "$dir/retry.jsonl"
  fi
  [ -f "$root/baseline.jsonl" ] && cp "$root/baseline.jsonl" "$dir/before.jsonl"
  status=0
  python3 -I "$tree/$harness/compare.py" "$dir/before.jsonl" "$dir/after.jsonl" > "$dir/diff.txt" || status=$?
  cat "$dir/diff.txt"
  echo "$sha" > "$root/last-sha"
  if [ "$status" = 0 ]; then
    cp "$dir/after.jsonl" "$root/baseline.jsonl"
    echo "$sha" > "$root/baseline-sha"
  fi
  printf '{"sha":"%s","previous":"%s","dir":"%s","status":%s}\n' "$sha" "$previous" "$dir" "$status" >> "$root/history.jsonl"
  case $status in
    0) echo "guard ok for $sha" ;;
    1) echo "guard regression on $sha; roll back with: sudo -n -- /usr/local/libexec/brain-release rollback $(cat "$root/baseline-sha" 2>/dev/null || echo '<previous sha>')" >&2 ;;
    *) echo "guard incomplete on $sha: provider unavailable" >&2 ;;
  esac
  rm -rf "$tree"
  return "$status"
}

main "$@"
