#!/usr/bin/env bash
set -euo pipefail
language=$1
database=$2
sarif=$3
test -s "$database/src.zip"
case "$language" in
  rust) pattern='rust/crates/.*\.rs$' ;;
  python) pattern='(src/deadlock_brain|mcp|scripts|rust/python_worker)/.*\.py$' ;;
  *) exit 1 ;;
esac
count=$(unzip -Z1 "$database/src.zip" | grep -Ec "$pattern")
test "$count" -gt 0
printf '%s: %s extracted repository source files\n' "$language" "$count"
jq -e -f scripts/ci/sarif.jq "$sarif"
