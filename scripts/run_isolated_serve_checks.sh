#!/usr/bin/env bash
set -euo pipefail
printf 'The historical system-service check is disabled: it stops the 5446 PostgreSQL service. Run scripts/test_brain_serve.sh for isolated readiness, outage and recovery checks.\n' >&2
exit 2
