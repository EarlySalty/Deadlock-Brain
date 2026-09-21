#!/usr/bin/env bash
# Neue, eindeutig benannte Testdatenbanken. Keine Produktivmigration.
set -euo pipefail
umask 077
cd /home/nathanael/.worktrees/brain-final-integration-20260921
OUT=/home/nathanael/.local/share/deadlock-brain/releases/20260921-final
TARGET=/home/nathanael/.cache/deadlock-brain-final-20260921
mkdir -p "$OUT"
eval "$(/home/naniadm/Documents/Infisical/export_gpt_secret.py --secret DEADLOCK_CENTRAL_DSN)"
trap 'unset DEADLOCK_CENTRAL_DSN source_dsn base_dsn query_suffix test_dsn PGDATABASE' EXIT
: "${DEADLOCK_CENTRAL_DSN:?Autorisierter Datenbankzugang fehlt}"
source_dsn=$DEADLOCK_CENTRAL_DSN
base_dsn=${source_dsn%%\?*}
base_dsn=${base_dsn%/*}
query_suffix=
if [[ "$source_dsn" == *\?* ]]; then query_suffix="?${source_dsn#*\?}"; fi
PREFIX="brain_final_$(date +%Y%m%d_%H%M%S)_$$"
FIX=tests/patch-understanding/schema-fixture.sql
OLD=scripts/migrations/2026-09-18-patch-evidence.sql
NEW=scripts/migrations/2026-09-18-patch-evidence-followup.sql
NEW2=scripts/migrations/2026-09-18-patch-evidence-followup2.sql
EXIST=tests/patch-understanding/schema-assertions.sql
HIST=tests/patch-understanding/followup-history-assertions.sql
CAP=tests/patch-understanding/followup-caption-assertions.sql
IDENT=tests/patch-understanding/followup-identity-assertions.sql
R4=tests/patch-understanding/followup2-r4-assertions.sql

create_db() {
  local name="${PREFIX}_$1"
  [[ "$name" =~ ^brain_final_[a-z0-9_]+$ ]] || exit 2
  if ! psql --dbname="$source_dsn" -X -w -v ON_ERROR_STOP=1 -c "CREATE DATABASE $name" >/dev/null 2>/dev/null; then
    printf 'Neue Scratch-Datenbank %s konnte nicht angelegt werden; kein Überschreiben.\n' "$name" >&2
    exit 1
  fi
  printf '%s\n' "$name" >>"$OUT/owned-test-databases.txt"
  test_dsn="$base_dsn/$name$query_suffix"
  local actual
  actual=$(psql --dbname="$test_dsn" -X -w -Atqc 'SELECT current_database()' 2>/dev/null) || exit 1
  [[ "$actual" == "$name" ]] || { printf 'Scratch-Zielprüfung fehlgeschlagen.\n' >&2; exit 1; }
}
run_sql() {
  if ! psql --dbname="$test_dsn" -X -w -v ON_ERROR_STOP=1 -f "$1" >/dev/null 2>/dev/null; then
    printf 'SQL-Test fehlgeschlagen: %s\n' "$1" >&2
    exit 1
  fi
}
count_revisions() {
  psql --dbname="$test_dsn" -X -w -Atqc 'SELECT count(*) FROM brain.patch_evidence_revisions' 2>/dev/null
}
expect_fail() {
  if psql --dbname="$test_dsn" -X -w -v ON_ERROR_STOP=1 -f "$1" >/dev/null 2>/dev/null; then
    printf 'Gegenprobe war unerwartet grün: %s\n' "$1" >&2; exit 1
  fi
  printf 'Gegenprobe erwartungsgemäß rot: %s\n' "$1"
}

create_db base
run_sql "$FIX"; run_sql "$OLD"; run_sql "$EXIST"
before=$(count_revisions); run_sql "$OLD"; after=$(count_revisions)
[[ "$before" == "$after" ]]
printf 'Baseline und ursprüngliche Migration idempotent: bestanden.\n'
for pair in "histold:$HIST" "capold:$CAP" "identold:$IDENT" "r4old:$R4"; do
  create_db "${pair%%:*}"
  run_sql "$FIX"; run_sql "$OLD"
  if [[ "$pair" == r4old:* ]]; then run_sql "$NEW"; fi
  expect_fail "${pair#*:}"
done
for pair in "existing:$EXIST" "history:$HIST" "caption:$CAP" "identity:$IDENT" "r4:$R4"; do
  create_db "full_${pair%%:*}"
  run_sql "$FIX"; run_sql "$OLD"; run_sql "$NEW"; run_sql "$NEW2"; run_sql "${pair#*:}"
  printf 'Vollständige Migrationsfolge bestanden: %s\n' "${pair%%:*}"
done
create_db idem
run_sql "$FIX"; run_sql "$OLD"; run_sql "$NEW"; run_sql "$NEW2"
before=$(count_revisions); run_sql "$NEW"; run_sql "$NEW2"; after=$(count_revisions)
[[ "$before" == "$after" ]]
printf 'Folgemigrationen idempotent: bestanden.\n'

create_db yt
run_sql tests/yt-contract/scratch-schema.sql
(
  cd rust
  export DEADLOCK_CENTRAL_DSN="$test_dsn"
  /home/nathanael/.cargo/bin/cargo test --locked -p deadlock-brain-yt --bin deadlock-brain-yt -j 2 --target-dir "$TARGET" -- --ignored --test-threads=1 \
    queue::tests::select_next_videos_and_mark_status_round_trip_pg \
    transcript_claims::tests::ingest_inserts_maps_verdicts_and_is_idempotent_pg \
    transcript_claims::tests::prepare_selects_untreated_verbal_video_then_excludes_it_pg \
    claims::tests::query_claims_filters_by_entity_prompt_version_and_model_pg \
    claims::tests::query_claims_excludes_revalidation_flagged_videos_pg
)
create_db caption
run_sql tests/patch-understanding/caption-scratch-schema.sql
run_sql "$OLD"; run_sql "$NEW"; run_sql "$NEW2"
(
  cd rust
  export DEADLOCK_CENTRAL_DSN="$test_dsn"
  /home/nathanael/.cargo/bin/cargo test --locked -p deadlock-brain-yt --bin deadlock-brain-yt -j 2 --target-dir "$TARGET" -- --ignored --exact transcripts::tests::save_transcript_is_idempotent_and_sets_ready_pg
)
printf 'POSTGRES_VERIFIED: SQL-Gegenproben, Migrationsfolgen und sechs Rust-Datenbanktests bestanden.\n'
