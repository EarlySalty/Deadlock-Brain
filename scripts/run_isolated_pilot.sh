#!/usr/bin/env bash
set -euo pipefail
printf 'The historical 5446 pilot is disabled: it used service restarts and environment passwords. Run scripts/test_brain_serve.sh for the disposable peer-auth process harness.\n' >&2
exit 2
