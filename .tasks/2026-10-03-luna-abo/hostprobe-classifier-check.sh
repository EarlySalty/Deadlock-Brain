#!/usr/bin/env bash
set -euo pipefail
source "${BASH_SOURCE[0]%/*}/hostprobe.sh"

allow=0
block=0
expect_allow() {
    brain_metadata_arguments_allowed "$@" || exit 1
    allow=$((allow + 1))
}
expect_block() {
    if brain_metadata_arguments_allowed "$@"; then
        exit 1
    fi
    block=$((block + 1))
}

expect_allow cargo metadata --format-version 1 --no-deps --manifest-path /fixture/Cargo.toml
expect_allow /fixture/bin/cargo metadata --format-version 1 --no-deps --manifest-path /Cargo.toml
expect_allow cargo metadata --format-version 1 --no-deps --manifest-path '/fixture mit Abstand/Cargo.toml'
expect_block cargo metadata --format-version 1 --no-deps --manifest-path fixture/Cargo.toml
expect_block cargo metadata --format-version 1 --no-deps --manifest-path /fixture/other.toml
expect_block cargo metadata --format-version 1 --no-deps --manifest-path /fixture/Cargo.toml --offline
expect_block cargo metadata --format-version 1 --no-deps --manifest-path /fixture/Cargo.toml --no-deps
expect_block cargo metadata --format-version 1 --no-deps --manifest-path
expect_block cargo metadata --format-version 1 --manifest-path /fixture/Cargo.toml
expect_block cargo metadata --format-version 2 --no-deps --manifest-path /fixture/Cargo.toml
expect_block cargo metadata --no-deps --format-version 1 --manifest-path /fixture/Cargo.toml
expect_block other metadata --format-version 1 --no-deps --manifest-path /fixture/Cargo.toml
expect_block cargo build --format-version 1 --no-deps --manifest-path /fixture/Cargo.toml
expect_block cargo test --format-version 1 --no-deps --manifest-path /fixture/Cargo.toml
expect_block cargo check --format-version 1 --no-deps --manifest-path /fixture/Cargo.toml
expect_block cargo clippy --format-version 1 --no-deps --manifest-path /fixture/Cargo.toml
expect_block cargo rustc --format-version 1 --no-deps --manifest-path /fixture/Cargo.toml
expect_block cargo rustdoc --format-version 1 --no-deps --manifest-path /fixture/Cargo.toml
expect_block
declare -a nul_arguments=()
mapfile -d '' -t nul_arguments < <(printf '%s\0' cargo metadata --format-version 1 --no-deps --manifest-path '/fixture mit Abstand/Cargo.toml')
expect_allow "${nul_arguments[@]}"
if brain_cargo_metadata_process_allowed 999999999; then
    exit 1
fi
block=$((block + 1))
printf 'Synthetische Klassifikation: %s Allowfälle, %s Blockfälle, bestanden.\n' "$allow" "$block"
