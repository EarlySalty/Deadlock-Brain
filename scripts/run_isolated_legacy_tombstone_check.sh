#!/usr/bin/env bash
set -euo pipefail
printf 'The historical legacy tombstone runner is disabled: it imports a stored copy of community data. Use the fixture-backed brain-legacy-import tests.\n' >&2
exit 2
