#!/usr/bin/env bash
# Legt die eigene Brain-PostgreSQL-Instanz an (idempotent, als root).
# Berührt weder den DL-Main-Cluster (16/main, Port 5432) noch dessen Dateien.
set -euo pipefail
[[ $EUID -eq 0 ]] || { echo "als root ausführen" >&2; exit 1; }
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PG_BIN=/usr/lib/postgresql/16/bin
OS_USER=deadlock-brain-pg
CLIENT_GROUP=deadlock-brain-db
PGDATA=/var/lib/deadlock-brain/postgresql
CONF=/etc/deadlock-brain/postgresql
LOGS=/var/log/deadlock-brain/postgresql
BACKUPS=/var/backups/deadlock-brain/postgresql
PORT=5446
MAIN_DATA=/var/lib/postgresql/16/main

getent group "$CLIENT_GROUP" >/dev/null || groupadd --system "$CLIENT_GROUP"
if ! id "$OS_USER" >/dev/null 2>&1; then
  useradd --system --user-group --home-dir /var/lib/deadlock-brain --no-create-home \
    --shell /usr/sbin/nologin --comment "Deadlock Brain PostgreSQL" "$OS_USER"
fi
usermod -a -G "$CLIENT_GROUP" "$OS_USER"
for member in ${BRAIN_DB_CLIENTS:-nathanael}; do usermod -a -G "$CLIENT_GROUP" "$member"; done

install -d -m 0755 -o root -g root /var/lib/deadlock-brain /var/log/deadlock-brain /var/backups/deadlock-brain /etc/deadlock-brain
install -d -m 0700 -o "$OS_USER" -g "$OS_USER" "$PGDATA" "$LOGS" "$BACKUPS"
install -d -m 0750 -o root -g "$OS_USER" "$CONF"

[[ "$(realpath "$PGDATA")" != "$(realpath "$MAIN_DATA")" ]] || { echo "PGDATA kollidiert mit DL-Main" >&2; exit 1; }
if ss -Hlx | grep -q "\.s\.PGSQL\.$PORT\b"; then
  systemctl is-active --quiet deadlock-brain-postgresql || { echo "Port $PORT belegt" >&2; exit 1; }
fi

if [[ ! -s "$PGDATA/PG_VERSION" ]]; then
  runuser -u "$OS_USER" -- "$PG_BIN/initdb" -D "$PGDATA" --username="$OS_USER" \
    --auth-local=peer --auth-host=reject --encoding=UTF8 --locale=C.UTF-8 --data-checksums >/dev/null
  rm -f "$PGDATA/postgresql.conf" "$PGDATA/pg_hba.conf" "$PGDATA/pg_ident.conf"
fi

for f in postgresql.conf pg_hba.conf pg_ident.conf; do
  install -m 0640 -o root -g "$OS_USER" "$HERE/$f" "$CONF/$f"
done
install -m 0644 -o root -g root "$HERE/deadlock-brain-postgresql.service" /etc/systemd/system/
install -m 0644 -o root -g root "$HERE/deadlock-brain-postgresql-backup.service" /etc/systemd/system/
install -m 0644 -o root -g root "$HERE/deadlock-brain-postgresql-backup.timer" /etc/systemd/system/
install -d -m 0755 /usr/local/lib/deadlock-brain
install -m 0755 -o root -g root "$HERE/backup.sh" /usr/local/lib/deadlock-brain/postgresql-backup.sh
systemctl daemon-reload
systemctl enable --now deadlock-brain-postgresql.service
systemctl reload deadlock-brain-postgresql.service
for _ in $(seq 1 30); do
  "$PG_BIN/pg_isready" -q -h /run/deadlock-brain-postgresql -p "$PORT" && break
  sleep 1
done
runuser -u "$OS_USER" -- "$PG_BIN/psql" -X -q -v ON_ERROR_STOP=1 -h /run/deadlock-brain-postgresql -p "$PORT" -d postgres < "$HERE/roles.sql"
runuser -u "$OS_USER" -- "$PG_BIN/psql" -X -q -v ON_ERROR_STOP=1 -h /run/deadlock-brain-postgresql -p "$PORT" -d postgres -v db=brain < "$HERE/grants.sql" 2>/dev/null || echo "grants.sql erst nach brain-migrate up anwendbar"
systemctl enable --now deadlock-brain-postgresql-backup.timer
echo "brain-postgresql bereit: Socket /run/deadlock-brain-postgresql, Port $PORT"
