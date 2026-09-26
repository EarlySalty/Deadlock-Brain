#!/usr/bin/env bash
# Prüft Rechte und Trennung der Brain-Instanz. Ändert nichts dauerhaft: jede
# Schreibprobe läuft in einer Transaktion mit ROLLBACK. Gibt keine Secrets aus.
set -uo pipefail
INFISICAL=${INFISICAL_TOOLS:-$HOME/Documents/Infisical}
PSQL=/usr/lib/postgresql/16/bin/psql
SOCKET=/run/deadlock-brain-postgresql
PORT=5446
fails=0
load() {
  local exported
  exported=$(python3 "$INFISICAL/export_gpt_secret.py" --secret "$1" 2>/dev/null) || { echo "Secret $1 fehlt" >&2; exit 2; }
  eval "$exported"
}
load BRAIN_PG_SERVICE_PASSWORD
load BRAIN_PG_INGEST_PASSWORD
load BRAIN_PG_READONLY_PASSWORD
declare -A PW=([brain_service]=$BRAIN_PG_SERVICE_PASSWORD [brain_ingest]=$BRAIN_PG_INGEST_PASSWORD [brain_readonly]=$BRAIN_PG_READONLY_PASSWORD)
unset BRAIN_PG_SERVICE_PASSWORD BRAIN_PG_INGEST_PASSWORD BRAIN_PG_READONLY_PASSWORD

run() {
  local role=$1 host=$2 port=$3 db=$4 sql=$5
  PGPASSWORD=${PW[$role]} PGCONNECT_TIMEOUT=3 sg deadlock-brain-db -c \
    "$PSQL -X -q -At -v ON_ERROR_STOP=1 -h '$host' -p $port -U $role -d $db" <<<"$sql" 2>&1
}
expect() {
  local want=$1 label=$2 role=$3 host=$4 port=$5 db=$6 sql=$7 out code
  out=$(run "$role" "$host" "$port" "$db" "$sql"); code=$?
  if { [[ $want == ok ]] && (( code == 0 )); } || { [[ $want == deny ]] && (( code != 0 )); }; then
    printf 'PASS %-4s %-15s %s\n' "$want" "$role" "$label"
  else
    printf 'FAIL %-4s %-15s %s (exit %s: %s)\n' "$want" "$role" "$label" "$code" "$(head -c 200 <<<"$out" | tr '\n' ' ')"
    fails=$((fails + 1))
  fi
}

for role in brain_service brain_ingest brain_readonly; do
  expect deny "CREATE TABLE"      $role $SOCKET $PORT brain "CREATE TABLE brain.probe_ddl(id int);"
  expect deny "ALTER TABLE"       $role $SOCKET $PORT brain "ALTER TABLE brain.corpus_releases_v1 ADD COLUMN probe int;"
  expect deny "DROP TABLE"        $role $SOCKET $PORT brain "DROP TABLE brain.corpus_releases_v1;"
  expect deny "TRUNCATE"          $role $SOCKET $PORT brain "TRUNCATE brain.source_record_revisions;"
  expect deny "CREATE SCHEMA"     $role $SOCKET $PORT brain "CREATE SCHEMA probe_schema;"
  expect deny "CREATE in public"  $role $SOCKET $PORT brain "CREATE TABLE public.probe_ddl(id int);"
  expect deny "TEMP TABLE"        $role $SOCKET $PORT brain "CREATE TEMP TABLE probe_tmp(id int);"
  expect deny "CREATE EXTENSION"  $role $SOCKET $PORT brain "CREATE EXTENSION IF NOT EXISTS postgres_fdw;"
  expect deny "CREATE ROLE"       $role $SOCKET $PORT brain "CREATE ROLE probe_role;"
  expect deny "Versionsmarker"    $role $SOCKET $PORT brain "BEGIN; UPDATE brain.core_schema_version SET schema_version = schema_version; ROLLBACK;"
  expect deny "DB postgres"       $role $SOCKET $PORT postgres "SELECT 1;"
  expect deny "DL-Main Socket"    $role /var/run/postgresql 5432 deadlock "SELECT 1;"
  expect deny "DL-Main TCP"       $role 127.0.0.1 5432 deadlock "SELECT 1;"
  expect ok   "SELECT Releases"   $role $SOCKET $PORT brain "SELECT count(*) FROM brain.corpus_releases_v1;"
done
expect ok   "Owner-Insert"        brain_service  $SOCKET $PORT brain "BEGIN; INSERT INTO brain.conversation_owners_v1(conversation_id, actor_id) VALUES ('probe-conv','probe-actor'); ROLLBACK;"
expect deny "Owner-Delete"        brain_service  $SOCKET $PORT brain "BEGIN; DELETE FROM brain.conversation_owners_v1; ROLLBACK;"
expect deny "Release schreiben"   brain_service  $SOCKET $PORT brain "BEGIN; DELETE FROM brain.corpus_releases_v1; ROLLBACK;"
expect deny "Revision schreiben"  brain_service  $SOCKET $PORT brain "BEGIN; UPDATE brain.source_record_revisions SET revision = revision; ROLLBACK;"
expect ok   "Job-Update"          brain_ingest   $SOCKET $PORT brain "BEGIN; UPDATE brain.source_jobs_v1 SET state = state; ROLLBACK;"
expect ok   "Checkpoint-Update"   brain_ingest   $SOCKET $PORT brain "BEGIN; UPDATE brain.source_checkpoints_v1 SET generation = generation; ROLLBACK;"
expect deny "Revision löschen"    brain_ingest   $SOCKET $PORT brain "BEGIN; DELETE FROM brain.source_record_revisions; ROLLBACK;"
expect deny "Owner lesen"         brain_ingest   $SOCKET $PORT brain "SELECT count(*) FROM brain.conversation_owners_v1;"
expect deny "Schreiben"           brain_readonly $SOCKET $PORT brain "BEGIN; UPDATE brain.source_jobs_v1 SET state = state; ROLLBACK;"

privileges=$(run brain_service $SOCKET $PORT brain "SELECT has_schema_privilege(current_user,'brain','CREATE') OR has_database_privilege(current_user,current_database(),'CREATE') OR has_database_privilege(current_user,current_database(),'TEMP') OR pg_has_role(current_user,'brain_migrate','MEMBER') OR (SELECT rolsuper OR rolcreatedb OR rolcreaterole FROM pg_roles WHERE rolname=current_user) OR EXISTS (SELECT FROM pg_namespace WHERE nspowner = (SELECT oid FROM pg_roles WHERE rolname=current_user)) OR EXISTS (SELECT FROM pg_database WHERE datdba = (SELECT oid FROM pg_roles WHERE rolname=current_user));")
[[ $privileges == f ]] && echo "PASS brain_service ohne DDL-, Owner- und Rollenrechte" || { echo "FAIL brain_service Privilegien: $privileges"; fails=$((fails + 1)); }

bridge=$(sudo -n -u deadlock-brain-pg $PSQL -X -At -h $SOCKET -p $PORT -d brain -c "SELECT (SELECT string_agg(extname, ',' ORDER BY extname) FROM pg_extension) || '|' || (SELECT count(*) FROM pg_foreign_server) || '|' || (SELECT count(*) FROM pg_foreign_data_wrapper) || '|' || (SELECT count(*) FROM pg_user_mappings);")
[[ $bridge == "plpgsql|0|0|0" ]] && echo "PASS keine FDW/dblink-Brücke ($bridge)" || { echo "FAIL Erweiterungen/FDW: $bridge"; fails=$((fails + 1)); }

main_roles=$(sudo -n -u postgres $PSQL -X -At -c "SELECT count(*) FROM pg_roles WHERE rolname IN ('brain_migrate','brain_ingest','brain_service','brain_readonly','deadlock-brain-pg');")
[[ $main_roles == 0 ]] && echo "PASS DL-Main kennt keine Brain-Rolle" || { echo "FAIL DL-Main hat $main_roles Brain-Rollen"; fails=$((fails + 1)); }

echo "fehlgeschlagen: $fails"
exit $(( fails > 0 ))
