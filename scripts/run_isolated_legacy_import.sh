#!/usr/bin/env bash
set -euo pipefail
printf 'The historical legacy import is disabled: it reads the existing Brain database and passes service passwords via the environment. Use the fixture-backed brain-legacy-import tests.\n' >&2
exit 2
