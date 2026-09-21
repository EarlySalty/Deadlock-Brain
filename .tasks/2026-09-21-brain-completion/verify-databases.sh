#!/usr/bin/env bash
# Reuse the isolated DB contracts. Raw connection/test errors are never logged.
set -Eeuo pipefail
umask 077
export BRAIN_TEST_ROOT=/home/nathanael/.worktrees/brain-release-completion-20260921
export BRAIN_TEST_OUT=/home/nathanael/.local/share/deadlock-brain/releases/20260921-continuation/database-contracts
export SQLX_OFFLINE=true
mkdir -p "$BRAIN_TEST_OUT"
phase=postgres
trap 'result=$?; printf "%s\n" "$result" >"$BRAIN_TEST_OUT/contracts.exit"; printf "%s\n" "$phase" >"$BRAIN_TEST_OUT/contracts.last-phase"; unset DEADLOCK_CENTRAL_DSN REASONER_SCRATCH_DSN; exit "$result"' EXIT
bash "$BRAIN_TEST_ROOT/.tasks/2026-09-21-brain-final/verify-postgres.sh" >/dev/null 2>/dev/null
printf '0\n' >"$BRAIN_TEST_OUT/postgres.exit"
printf 'SQL-Migrationsfolgen, Idempotenz, rote Gegenproben und sechs YouTube-/Caption-Tests bestanden.\n' >"$BRAIN_TEST_OUT/verified.log"
phase=reasoner
bash "$BRAIN_TEST_ROOT/.tasks/2026-09-21-brain-final/verify-reasoner-postgres.sh" >/dev/null 2>/dev/null
printf '0\n' >"$BRAIN_TEST_OUT/reasoner.exit"
printf 'Isolierte Reasoner-Postgres-Verträge bestanden.\n' >>"$BRAIN_TEST_OUT/verified.log"
phase=verified
