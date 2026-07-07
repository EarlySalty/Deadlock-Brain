# sqlx-Offline-Cache (`rust/.sqlx/`)

Das Async-Fundament des PG-Cutovers (Spec `docs/specs/2026-07-04-pg-cutover.md`,
Phase 3.0). Compile-geprüfte `sqlx::query!`/`query_as!`-Makros werden gegen ein
gespeichertes Schema-Abbild in `rust/.sqlx/` validiert, damit
`SQLX_OFFLINE=true`-Builds **ohne** laufende Postgres grün sind. Der Cache liegt
unter Versionskontrolle und muss nach jeder Änderung an einer compile-geprüften
Query neu erzeugt werden.

## Wann neu generieren?

Immer wenn eine `query!`/`query_as!`/`query_scalar!`-Query hinzukommt, sich ihr
SQL ändert oder sich das `brain.*`-Schema ändert. Andernfalls schlägt der
Offline-Build mit „no cached data for this query" fehl.

## Regenerieren (Wegwerf-Postgres, nie die Live-DB)

```bash
# 1) Wegwerf-Postgres starten
docker run -d --name dbrain-sqlx-scratch \
  -e POSTGRES_PASSWORD=x -p 55432:5432 postgres:16
# auf Bereitschaft warten
until docker exec dbrain-sqlx-scratch pg_isready -U postgres; do sleep 1; done

# 2) brain.*-Schema einspielen (Reihenfolge einhalten)
#    Quelle: Deadlock-Bots/rust/crates/dl-central-db/migrations
MIG=/home/naniadm/Documents/Deadlock-Bots/rust/crates/dl-central-db/migrations
PSQL="docker exec -i dbrain-sqlx-scratch psql -U postgres -v ON_ERROR_STOP=1 -q"
$PSQL < "$MIG/0012_brain_knowledge_timeline.sql"
$PSQL < "$MIG/0013_brain_insight_records.sql"
# Rest der brain.*-Tabellen liegt (Stand 2026-07) noch auf einem Branch:
git -C /home/naniadm/Documents/Deadlock-Bots show \
  origin/feature/brain-pg-ingestion-tables:rust/crates/dl-central-db/migrations/2026070410_brain_ingestion_tables.sql \
  | $PSQL
# Sobald 2026070410 auf main gemergt ist, stattdessen direkt aus $MIG einspielen.

# 3) Cache erzeugen (schreibt rust/.sqlx/)
cd /home/naniadm/Documents/Deadlock-Brain/rust
DATABASE_URL='postgres://postgres:x@127.0.0.1:55432/postgres?sslmode=disable' \
  cargo sqlx prepare --workspace

# 4) Verifizieren: Offline-Build ohne DB
unset DATABASE_URL
SQLX_OFFLINE=true cargo build --workspace
SQLX_OFFLINE=true cargo clippy --workspace --all-targets

# 5) Aufräumen
docker rm -f dbrain-sqlx-scratch
```

Danach `rust/.sqlx/` mitcommitten. Für einen Funktionstest mit echten Daten
einen frischen Dump der zentralen Postgres in die Wegwerf-PG einspielen — nie
gegen die Live-DB testen.

## Hinweise

- `sqlx-cli` installieren: `cargo install sqlx-cli --no-default-features --features postgres,rustls`.
- Der Cache braucht nur das **Schema**, keine Daten (Prepare beschreibt Queries).
- DSN nie loggen/committen; `DEADLOCK_CENTRAL_DSN` zeigt in CI/Prod auf die
  zentrale Postgres, hier ausschließlich auf die Wegwerf-Instanz.
