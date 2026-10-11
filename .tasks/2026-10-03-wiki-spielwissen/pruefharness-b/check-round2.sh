#!/usr/bin/env bash
set -euo pipefail
root=/home/nathanael/.worktrees/brain-wiki-spielwissen-b
harness="$root/.tasks/2026-10-03-wiki-spielwissen/pruefharness-b"
proof_root=/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/b/round3-proof
mkdir -p "$proof_root"
artifacts=$(mktemp -d "$proof_root/run-XXXXXXXX")
exec > "$artifacts/check-b3-run.log" 2>&1
printf 'RUN_START pid=%s utc=%s\n' "$$" "$(date -u +%FT%TZ)"
exec 8>/home/nathanael/Documents/.tasks/2026-10-02-offene-branches/locks/host-checks.lock
flock -x 8
exec 9>/tmp/deadlock-cargo-release.lock
flock -x 9
trap 'exec 9>&-; exec 8>&-; printf "LOCK_RELEASE both released on exit\n"' EXIT
wait_for_compilers() {
    while ! node -e '
const fs=require("fs"),cp=require("child_process");
const processes=cp.execFileSync("ps",["-eo","pid=,ppid=,stat=,comm="],{encoding:"utf8"}).split("\n").map(l=>l.trim().split(/\s+/)).filter(a=>a.length>=4&&!a[2].startsWith("Z")&&["cargo","rustc","clippy-driver","rustdoc"].includes(a[3]));
const busy=processes.filter(a=>{
if(a[3]!=="cargo")return true;
try{const argv=fs.readFileSync(`/proc/${a[0]}/cmdline`,"utf8").split("\0").filter(Boolean);return !(argv.length===7&&argv[1]==="metadata"&&argv[2]==="--format-version"&&argv[3]==="1"&&argv[4]==="--no-deps"&&argv[5]==="--manifest-path"&&argv[6].startsWith("/")&&argv[6].endsWith("/Cargo.toml"));}catch{return true;}
});
console.log(JSON.stringify({compiler_probe:busy.map(a=>({pid:a[0],ppid:a[1],name:a[3]}))}));process.exit(busy.length?75:0);
'; do sleep 30; done
}
run_step() {
    local name="$1" result started="$SECONDS"
    shift
    printf 'STEP_START %s utc=%s\n' "$name" "$(date -u +%FT%TZ)"
    if "$@"; then result=0; else result=$?; fi
    printf 'STEP_EXIT %s %s duration_seconds=%s\n' "$name" "$result" "$((SECONDS-started))"
    return "$result"
}
free -m
df -h "$root"
if flock -n /home/nathanael/Documents/.tasks/2026-10-02-offene-branches/locks/host-checks.lock true; then exit 76; fi
if flock -n /tmp/deadlock-cargo-release.lock true; then exit 76; fi
printf 'LOCK_PROOF both held fd8=%s fd9=%s\n' "$(readlink /proc/$$/fd/8)" "$(readlink /proc/$$/fd/9)"
printf 'MESSGRENZE Linux-Prozesspeak VmHWM in KiB und monotone Dauer je Git-Exportlauf; keine globale Host-RAM-Grenze, keine Steam-/VPK-Datenabnahme.\n'
sources=("$root/rust/crates/dbrain-sources/src/game_files.rs" "$root"/rust/crates/dbrain-sources/src/game_files/*.rs "$harness/src/main.rs" "$harness/src/bin/validate-jsonl.rs")
sha256sum "${sources[@]}" "$harness/check-round2.sh" > "$artifacts/source-before.sha256"
compiler=/home/nathanael/.rustup/toolchains/1.97.1-x86_64-unknown-linux-gnu/bin/rustc
clippy=/home/nathanael/.rustup/toolchains/1.97.1-x86_64-unknown-linux-gnu/bin/clippy-driver
formatter=/home/nathanael/.rustup/toolchains/1.97.1-x86_64-unknown-linux-gnu/bin/rustfmt
deps=/home/nathanael/repos/Deadlock-Brain/rust/target/debug/deps
common=(--edition=2021 -L "dependency=$deps" --extern "serde=$deps/libserde-267052e612033a95.rlib" --extern "serde_json=$deps/libserde_json-e851daea22c6858d.rlib" --extern "sha2=$deps/libsha2-a6ffd98be9b784e2.rlib")
quality=0
run_step fmt_check "$formatter" --check --edition=2021 --config skip_children=true "${sources[@]}" || quality=1
wait_for_compilers
run_step compile_extractor_tests "$compiler" "${common[@]}" --test "$harness/src/main.rs" --crate-name b_game_files_tests --extern "tempfile=$deps/libtempfile-3681fa5dacf21087.rlib" -o "$artifacts/b-game-files-tests"
run_step extractor_tests "$artifacts/b-game-files-tests" --include-ignored --test-threads=2
bounded_test() {
    (ulimit -v 131072; "$artifacts/b-game-files-tests" game_files::vpk::tests::tiny_packages_with_huge_declared_payloads_report_bounds_gaps --exact --include-ignored --test-threads=2)
}
run_step bounded_51_byte_vpk_test bounded_test
wait_for_compilers
run_step compile_extractor "$compiler" "${common[@]}" "$harness/src/main.rs" --crate-name b_game_files_extract -o "$artifacts/b-game-files-extract"
wait_for_compilers
run_step compile_validator_tests "$compiler" "${common[@]}" --test "$harness/src/bin/validate-jsonl.rs" --crate-name b_validate_jsonl_tests -o "$artifacts/b-validate-jsonl-tests"
run_step validator_tests "$artifacts/b-validate-jsonl-tests" --include-ignored --test-threads=2
wait_for_compilers
run_step compile_validator "$compiler" "${common[@]}" "$harness/src/bin/validate-jsonl.rs" --crate-name b_validate_jsonl -o "$artifacts/b-validate-jsonl"
wait_for_compilers
run_step clippy_extractor_tests "$clippy" "${common[@]}" --test "$harness/src/main.rs" --crate-name b_game_files_tests --extern "tempfile=$deps/libtempfile-3681fa5dacf21087.rlib" --emit=metadata -o "$artifacts/clippy-extractor-tests.rmeta" -D warnings || quality=1
wait_for_compilers
run_step clippy_extractor "$clippy" "${common[@]}" "$harness/src/main.rs" --crate-name b_game_files_extract --emit=metadata -o "$artifacts/clippy-extractor.rmeta" -D warnings || quality=1
wait_for_compilers
run_step clippy_validator_tests "$clippy" "${common[@]}" --test "$harness/src/bin/validate-jsonl.rs" --crate-name b_validate_jsonl_tests --emit=metadata -o "$artifacts/clippy-validator-tests.rmeta" -D warnings || quality=1
wait_for_compilers
run_step clippy_validator "$clippy" "${common[@]}" "$harness/src/bin/validate-jsonl.rs" --crate-name b_validate_jsonl --emit=metadata -o "$artifacts/clippy-validator.rmeta" -D warnings || quality=1
source_base=/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/b
for source in gametracking-4c6431ccdb816d2911bbbaa4335169cc34140386 deadlock-data-0d46cdecfccf77adec16aac01af6d30173e0ebb8; do
    run_step "extract_$source" "$artifacts/b-game-files-extract" "$source_base/$source/options.json" "$artifacts/$source.jsonl" "$artifacts/$source.inventory.json" 2> "$artifacts/$source.extract.resources.log"
    run_step "validate_$source" "$artifacts/b-validate-jsonl" "$source_base/$source/options.json" "$artifacts/$source.jsonl" "$artifacts/$source.inventory.json" > "$artifacts/$source.validation.log" 2> "$artifacts/$source.validate.resources.log"
    printf 'VALIDATION_LOG %s\n' "$artifacts/$source.validation.log"
done
sha256sum "${sources[@]}" "$harness/check-round2.sh" > "$artifacts/source-after.sha256"
run_step source_hash_binding cmp "$artifacts/source-before.sha256" "$artifacts/source-after.sha256"
sha256sum "$artifacts"/*.jsonl "$artifacts"/*.inventory.json "$artifacts"/*.validation.log "$artifacts"/*.resources.log > "$artifacts/artifacts.sha256"
run_step artifact_hash_binding sha256sum --check "$artifacts/artifacts.sha256"
exec 9>&-
exec 8>&-
trap - EXIT
printf 'LOCK_RELEASE both released\n'
printf 'RUN_EXIT %s utc=%s duration_seconds=%s\n' "$quality" "$(date -u +%FT%TZ)" "$SECONDS"
exit "$quality"
