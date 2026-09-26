#!/usr/bin/env bash
# Setzt SCRAM-Passwörter der Brain-Rollen aus Infisical. Fehlt ein Secret, wird es
# erzeugt und in Infisical angelegt. Kein Wert erscheint in Ausgabe, Datei oder Serverlog.
set -euo pipefail
INFISICAL=${INFISICAL_TOOLS:-$HOME/Documents/Infisical}
PSQL=(sudo -n -u deadlock-brain-pg /usr/lib/postgresql/16/bin/psql -X -q -v ON_ERROR_STOP=1
      -h /run/deadlock-brain-postgresql -p 5446 -d postgres)
declare -A SECRETS=(
  [brain_ingest]=BRAIN_PG_INGEST_PASSWORD
  [brain_service]=BRAIN_PG_SERVICE_PASSWORD
  [brain_readonly]=BRAIN_PG_READONLY_PASSWORD
)
for role in "${!SECRETS[@]}"; do
  name=${SECRETS[$role]}
  value=""
  if exported=$(python3 "$INFISICAL/export_gpt_secret.py" --secret "$name" 2>/dev/null); then
    eval "$exported"
    value=${!name}
    unset "$name"
  fi
  if [[ -z $value ]]; then
    value=$(openssl rand -hex 32)
    printf '%s' "$value" | python3 "$INFISICAL/update_gpt_secret.py" --secret "$name" --anlegen >/dev/null
    echo "$name in Infisical angelegt"
  fi
  [[ $value =~ ^[A-Za-z0-9]{32,}$ ]] || { echo "$name hat ein unerwartetes Format" >&2; exit 1; }
  printf "SET log_statement = 'none';\nSET log_min_duration_statement = -1;\nALTER ROLE %s PASSWORD '%s';\n" \
    "$role" "$value" | "${PSQL[@]}"
  echo "$role: Passwort gesetzt (Quelle $name)"
done
