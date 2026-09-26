#!/usr/bin/env bash
# 600 Requests je Stufe (8, dann 16, dann 32 Worker) gegen brain-serve auf der eigenen Brain-Instanz.
# Stoppt nach der ersten Stufe mit "too many clients", Fehlklassifikation oder Pilotfehler.
set -uo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BASE=${BRAIN_LOAD_REPORT:?BRAIN_LOAD_REPORT required}
CPU=/sys/fs/cgroup/cpu,cpuacct/system.slice/deadlock-brain-postgresql.service/cpuacct.usage
MEM=/sys/fs/cgroup/memory/system.slice/deadlock-brain-postgresql.service
LOG_DIR=/var/log/deadlock-brain/postgresql
for workers in ${BRAIN_LOAD_STEPS:-8 16 32}; do
  R="$BASE/w$workers"
  rm -rf "$R"; mkdir -p "$R"
  stop="$R/.stop"
  log_lines=$(sudo -n sh -c "cat $LOG_DIR/postgresql-*.log" | wc -l)
  cpu0=$(cat "$CPU")
  sudo -n sh -c "echo 0 > $MEM/memory.max_usage_in_bytes" 2>/dev/null || true
  "$ROOT/scripts/sample_brain_pg.sh" "$R/pg_activity.psv" "$stop" &
  sampler=$!
  BRAIN_PILOT_REPORT="$R/pilot" BRAIN_PILOT_LOAD_REQUESTS=600 BRAIN_PILOT_LOAD_WORKERS=$workers \
    "$ROOT/scripts/run_isolated_pilot.sh" > "$R/pilot.out" 2>&1
  pilot=$?
  touch "$stop"; wait "$sampler"
  cpu1=$(cat "$CPU")
  sudo -n sh -c "cat $LOG_DIR/postgresql-*.log" | tail -n +"$((log_lines + 1))" > "$R/pg-server.log"
  python3 - "$R" "$workers" "$pilot" "$cpu0" "$cpu1" "$(cat "$MEM/memory.max_usage_in_bytes" 2>/dev/null || echo 0)" <<'EOF'
import json, sys, pathlib
r, workers, pilot, cpu0, cpu1, mem = sys.argv[1:]
r = pathlib.Path(r)
rows = [l.split('|') for l in (r / 'pg_activity.psv').read_text().splitlines()[1:] if l.count('|') == 7]
col = lambda i: max((int(x[i]) for x in rows), default=0)
log = (r / 'pg-server.log').read_text(errors='replace')
report = json.loads((r / 'pilot' / 'after_restart_default.json').read_text()) if (r / 'pilot' / 'after_restart_default.json').exists() else {}
load = report.get('load') or {}
summary = {
  'workers': int(workers), 'pilot_exit': int(pilot), 'samples': len(rows),
  'peak_client_backends': col(1), 'peak_brain_service': col(2), 'peak_brain_service_active': col(3),
  'peak_pool_connections_brain_serve': col(4), 'peak_reader_connections_other_app': col(5),
  'max_connections': col(7),
  'new_brain_service_connections': log.count('user=brain_service database=brain_pilot'),
  'too_many_clients': log.count('too many clients'),
  'connection_limit_exceeded': log.count('too many connections for role'),
  'pg_cpu_seconds': (int(cpu1) - int(cpu0) if int(cpu1) >= int(cpu0) else int(cpu1)) / 1e9, 'pg_memory_peak_bytes': int(mem),
  'load': load, 'failed_cases': report.get('failed'),
}
(r / 'summary.json').write_text(json.dumps(summary, indent=2))
print(json.dumps({k: v for k, v in summary.items() if k != 'load'} | {'statuses': load.get('statuses'), 'p50_ms': load.get('p50_ms'), 'p95_ms': load.get('p95_ms'), 'p99_ms': load.get('p99_ms'), 'rps': load.get('throughput_rps'), 'serve_process': load.get('process_after_load')}))
EOF
  if (( pilot != 0 )) || grep -q 'too many clients' "$R/pg-server.log"; then
    echo "Stufe $workers nicht bestanden, keine höhere Stufe" >&2
    exit 1
  fi
done
