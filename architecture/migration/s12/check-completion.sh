#!/usr/bin/env bash
# Offline verification only. Historical reports/ are immutable evidence.
set -euo pipefail
ROOT=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../../.." && pwd -P)
cd -- "$ROOT"
mkdir -p -- "$ROOT/rust/target"
REPORTS=$(mktemp -d "$ROOT/rust/target/wiki-completion.XXXXXX")
mkdir -- "$REPORTS/home"
printf 'Wiki completion artifacts: %s\n' "$REPORTS"
printf 'name\texit_code\tlog\targv\n' > "$REPORTS/completion-results.tsv"
printf 'complete=false\n' > "$REPORTS/status.txt"
# Allowlist only build-tool settings. No database/provider credentials reach tests.
BUILD_ENV=(env -i "PATH=$PATH" "HOME=$REPORTS/home"
    "CARGO_HOME=${CARGO_HOME:-$HOME/.cargo}" "RUSTUP_HOME=${RUSTUP_HOME:-$HOME/.rustup}"
    "SQLX_OFFLINE=true" "CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS:-2}")
if [[ -n ${RUSTUP_TOOLCHAIN:-} ]]; then BUILD_ENV+=("RUSTUP_TOOLCHAIN=$RUSTUP_TOOLCHAIN"); fi
if [[ -n ${CARGO_TARGET_DIR:-} ]]; then BUILD_ENV+=("CARGO_TARGET_DIR=$CARGO_TARGET_DIR"); fi
{
    printf 'SQLX_OFFLINE=true\ncredential_environment=excluded\n'
    "${BUILD_ENV[@]}" rustc --version
    "${BUILD_ENV[@]}" cargo --version
    git rev-parse HEAD
} > "$REPORTS/environment.txt"
failed=0
run_step() {
    local label=$1 code=0 arg
    shift
    printf 'RUN %s\n' "$label"
    "${BUILD_ENV[@]}" "$@" > "$REPORTS/completion-$label.log" 2>&1 || code=$?
    printf '%s\t%s\t%s\t' "$label" "$code" "completion-$label.log" >> "$REPORTS/completion-results.tsv"
    for arg in "$@"; do printf '%q ' "$arg" >> "$REPORTS/completion-results.tsv"; done
    printf '\n' >> "$REPORTS/completion-results.tsv"
    printf 'EXIT %s %s\n' "$code" "$label"
    if (( code != 0 )); then
        failed=1
        tail -c 8000 -- "$REPORTS/completion-$label.log"
    fi
}
run_step offline-suite bash architecture/migration/s12/check.sh
run_step contracts-source-tests cargo test --manifest-path rust/Cargo.toml -p brain-contracts -p dbrain-sources --all-targets --locked --offline -j2
run_step contracts-source-clippy cargo clippy --manifest-path rust/Cargo.toml -p brain-contracts -p dbrain-sources --all-targets --no-deps --locked --offline -- -D warnings
run_step contracts-source-release cargo build --manifest-path rust/Cargo.toml -p brain-contracts -p dbrain-sources --release --locked --offline -j2
run_step retained-hero-tests cargo test --manifest-path rust/Cargo.toml -p dbrain-retrieval --lib --locked --offline -j2 hero_dossier
run_step workspace-all-targets cargo check --manifest-path rust/Cargo.toml --workspace --all-targets --locked --offline -j2
run_step changed-source-format rustfmt --check --edition 2021 --config skip_children=true rust/crates/dbrain-sources/src/lib.rs rust/crates/dbrain-sources/src/wiki_capture_io.rs rust/crates/dbrain-sources/tests/wiki_knowledge_contract.rs
run_step contracts-format cargo fmt --manifest-path rust/Cargo.toml -p brain-contracts -- --check
run_step diff-whitespace git diff --check
# Retain source identity alongside exact commands and exit codes, without
# overwriting versioned Golden files or recording credentials/absolute runtime paths.
git ls-files -z architecture/migration/s12 rust/crates/brain-contracts rust/crates/dbrain-sources rust/Cargo.toml rust/Cargo.lock |
    while IFS= read -r -d '' path; do
        case "$path" in architecture/migration/s12/reports/*) continue;; esac
        [[ -f $path ]] && sha256sum -- "$path"
    done > "$REPORTS/source-sha256.txt"
printf 'complete=true\nsuccess=%s\n' "$((1 - failed))" > "$REPORTS/status.txt"
printf 'Wiki completion artifacts: %s\n' "$REPORTS"
exit "$failed"
