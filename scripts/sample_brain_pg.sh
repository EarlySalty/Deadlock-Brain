#!/usr/bin/env bash
# Tastet pg_stat_activity und Prozessressourcen der Brain-Instanz ab, bis die Stop-Datei existiert.
set -uo pipefail
OUT=${1:?Ausgabedatei}
STOP=${2:?Stop-Datei}
DB=${3:-brain_pilot}
PSQL=(sudo -n -u deadlock-brain-pg /usr/lib/postgresql/16/bin/psql -X -At -F'|' -h /run/deadlock-brain-postgresql -p 5446 -d postgres)
printf 'ts_ms|total|service|service_active|service_app_brain_serve|service_app_other|ingest|max_connections\n' > "$OUT"
while [[ ! -e $STOP ]]; do
  row=$("${PSQL[@]}" -c "SELECT (extract(epoch FROM clock_timestamp())*1000)::bigint, count(*), count(*) FILTER (WHERE usename='brain_service'), count(*) FILTER (WHERE usename='brain_service' AND state='active'), count(*) FILTER (WHERE usename='brain_service' AND application_name='brain-serve'), count(*) FILTER (WHERE usename='brain_service' AND application_name<>'brain-serve'), count(*) FILTER (WHERE usename='brain_ingest'), current_setting('max_connections') FROM pg_stat_activity WHERE backend_type='client backend' AND datname='$DB'" 2>/dev/null)
  [[ -n $row ]] && echo "$row" >> "$OUT"
  sleep 0.05
done
