#!/usr/bin/env bash
set -euo pipefail
printf 'The historical legacy import is disabled: it reads the existing Brain database and passes service passwords via the environment. Run scripts/test_brain_serve.sh for a synthetic legacy archive and import CLI check.\n' >&2
exit 2
