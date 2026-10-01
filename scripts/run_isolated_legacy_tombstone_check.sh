#!/usr/bin/env bash
set -euo pipefail
printf 'The historical legacy tombstone runner is disabled: it imports a stored copy of community data. Run scripts/test_brain_serve.sh for a synthetic CLI tombstone and revoke check.\n' >&2
exit 2
