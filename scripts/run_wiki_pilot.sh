#!/usr/bin/env bash
# Operator-only C5 pilot. This script is NEVER invoked by CI with --allow-network.
# Both modes stage into a fresh Unix-socket scratch cluster, then stop that cluster.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CARGO="${BRAIN_TEST_CARGO:-$(command -v cargo)}"
PG_BIN="${BRAIN_TEST_PG_BIN:-$(pg_config --bindir)}"
case "${1:-}" in
  --allow-network)
    [[ $# == 3 ]] || { echo 'Usage: run_wiki_pilot.sh --allow-network CONFIG.json NEW_OUTPUT_DIR' >&2; exit 2; }
    MODE=capture; CONFIG="$(realpath "$2")"; OUT="$(realpath -m "$3")" ;;
  --from-capture)
    [[ $# == 5 ]] || { echo 'Usage: run_wiki_pilot.sh --from-capture CAPTURE.json MAPPING.json RELEASE_REQUEST.json NEW_OUTPUT_DIR' >&2; exit 2; }
    MODE=offline; CAPTURE="$(realpath "$2")"; MAPPING="$(realpath "$3")"; RELEASE="$(realpath "$4")"; OUT="$(realpath -m "$5")" ;;
  *) echo 'Explicit --allow-network or --from-capture required. No discovery or DB access performed.' >&2; exit 2 ;;
esac
[[ $(id -u) != 0 && ! -e "$OUT" ]] || { echo 'Use an unprivileged user and a new output directory.' >&2; exit 2; }
umask 077
mkdir -p "$OUT"
WORK="$(mktemp -d /tmp/brain-c5-pg.XXXXXX)"
printf 'C5_SCRATCH_ONLY\n' >"$WORK/C5_SCRATCH_ONLY"
mkdir -m 700 "$WORK/home"
started=0
cleanup() {
  if [[ "$started" == 1 ]]; then "$PG_BIN/pg_ctl" -D "$WORK/pg" -m fast -w stop >"$OUT/postgres-stop.log"; fi
  printf 'Capture/IR/receipt: %s\nScratch server stopped; cluster evidence: %s\n' "$OUT" "$WORK"
}
trap cleanup EXIT
unset DATABASE_URL PGHOST PGPORT PGUSER PGPASSWORD PGDATABASE PGSERVICE PGSERVICEFILE PGOPTIONS
export PGPASSFILE="$WORK/nonexistent-pgpass"
CLEAN=(env -i "PATH=$PATH" "HOME=$WORK/home" "CARGO_HOME=${CARGO_HOME:-$HOME/.cargo}" "RUSTUP_HOME=${RUSTUP_HOME:-$HOME/.rustup}"
  "SQLX_OFFLINE=true" "CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS:-2}" "CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-$ROOT/rust/target}")
cd "$ROOT/rust"
"${CLEAN[@]}" "$CARGO" build --locked -p dbrain-sources --bin brain-wiki-pilot
BIN="${CARGO_TARGET_DIR:-$ROOT/rust/target}/debug/brain-wiki-pilot"
if [[ "$MODE" == capture ]]; then
  "${CLEAN[@]}" "$BIN" capture "$CONFIG" "$OUT/capture" --allow-network
  CAPTURE="$OUT/capture/capture.json"; MAPPING="$OUT/capture/mapping.json"; RELEASE="$OUT/capture/release-request.json"
fi
"$PG_BIN/initdb" -D "$WORK/pg" --auth=trust --username=brain_wiki_c5 --no-locale --encoding=UTF8 >"$OUT/initdb.log"
"$PG_BIN/pg_ctl" -D "$WORK/pg" -l "$OUT/postgres.log" \
  -o "-c listen_addresses='' -k $WORK -p 55441 -c max_connections=12 -c shared_buffers=16MB" -w start
started=1
"$PG_BIN/createdb" -h "$WORK" -p 55441 -U brain_wiki_c5 brain_wiki_c5
"${CLEAN[@]}" "$BIN" stage "$CAPTURE" "$MAPPING" "$RELEASE" "$WORK" "$OUT/staged"
