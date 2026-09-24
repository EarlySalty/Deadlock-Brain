#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
status=0
cargo audit --file rust/Cargo.lock --deny unsound --deny yanked || status=1
cargo deny --manifest-path rust/Cargo.toml --locked check || status=1
exit "$status"
