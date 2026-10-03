#!/bin/bash
set -u -o pipefail
export PATH=/home/nathanael/.cargo/bin:/usr/local/bin:/usr/bin:/bin
export CARGO_BUILD_JOBS=2
export CARGO_TARGET_DIR=/home/nathanael/.worktrees/brain-fertig-q/rust/target
export SQLX_OFFLINE=true
unset BRAIN_CORE_TEST_PG_SOCKET BRAIN_PILOT_TARGET BRAIN_PILOT_DATABASE BRAIN_PILOT_ROOT BRAIN_PILOT_REPORT BRAIN_PILOT_INGEST_PASSWORD BRAIN_PILOT_SERVICE_PASSWORD
exec >/home/nathanael/.worktrees/brain-fertig-q/.q10-verify.log 2>&1
printf 'WRAPPER_PID=%s\n' "$$"
date -u '+START=%FT%TZ'
child=''
cleanup() {
    if [[ -n "$child" ]]; then
        kill -TERM -- "-$child" 2>/dev/null || true
        wait "$child" 2>/dev/null || true
    fi
    exec 9>&-
    exec 8>&-
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM HUP
printf 'WAIT_HOST_LOCK\n'
exec 8>/home/nathanael/Documents/.tasks/2026-10-02-offene-branches/locks/host-checks.lock
flock -x 8
printf 'WAIT_CARGO_LOCK\n'
exec 9>/tmp/deadlock-cargo-release.lock
flock -x 9
printf 'LOCKS_ACQUIRED\n'
stat -Lc 'FD8 inode=%i device=%d' /proc/$$/fd/8
stat -Lc 'PATH8 inode=%i device=%d' /home/nathanael/Documents/.tasks/2026-10-02-offene-branches/locks/host-checks.lock
stat -Lc 'FD9 inode=%i device=%d' /proc/$$/fd/9
stat -Lc 'PATH9 inode=%i device=%d' /tmp/deadlock-cargo-release.lock
flock -n /home/nathanael/Documents/.tasks/2026-10-02-offene-branches/locks/host-checks.lock true
counter8=$?
flock -n /tmp/deadlock-cargo-release.lock true
counter9=$?
printf 'COUNTER8_EXIT=%s COUNTER9_EXIT=%s\n' "$counter8" "$counter9"
if [[ "$counter8" -ne 1 || "$counter9" -ne 1 ]]; then exit 76; fi
probe() {
    /usr/local/bin/node <<'JS'
const fs = require('fs'), cp = require('child_process');
const rows = cp.execFileSync('ps', ['-eo', 'pid=,ppid=,stat=,comm='], {encoding:'utf8'}).trim().split('\n');
let busy = false;
for (const row of rows) {
    const [pid, ppid, state, name] = row.trim().split(/\s+/);
    if (state.startsWith('Z') || !['cargo','rustc','clippy-driver','rustdoc'].includes(name)) continue;
    let classification = 'blocking';
    if (name === 'cargo') {
        try {
            const a = fs.readFileSync('/proc/'+pid+'/stat','utf8');
            const args = fs.readFileSync('/proc/'+pid+'/cmdline').toString().split('\0');
            if (args.at(-1) === '') args.pop();
            const b = fs.readFileSync('/proc/'+pid+'/stat','utf8');
            const stat = x => x.slice(x.lastIndexOf(')')+2).trim().split(/\s+/);
            const sa = stat(a), sb = stat(b);
            const stable = sa[1] === ppid && sb[1] === ppid && sa[19] === sb[19];
            if (stable && args.length === 7 && args[1] === 'metadata' && args[2] === '--format-version' && args[3] === '1' && args[4] === '--no-deps' && args[5] === '--manifest-path' && args[6].startsWith('/') && args[6].endsWith('/Cargo.toml')) classification = 'pure-metadata';
        } catch (_) {}
    }
    console.log('PROBE pid='+pid+' ppid='+ppid+' name='+name+' class='+classification);
    if (classification !== 'pure-metadata') busy = true;
}
console.log('PROBE_EXIT='+(busy ? 75 : 0));
process.exit(busy ? 75 : 0);
JS
}
while true; do
    date -u '+PROBE_TIME=%FT%TZ'
    probe
    probe_exit=$?
    if [[ "$probe_exit" -eq 0 ]]; then break; fi
    if [[ "$probe_exit" -ne 75 ]]; then exit "$probe_exit"; fi
    sleep 30
done
free -m
df -h /home/nathanael/.worktrees/brain-fertig-q/rust/target
/home/nathanael/.cargo/bin/cargo +1.97.1 --version
/home/nathanael/.cargo/bin/rustc +1.97.1 --version
git -C /home/nathanael/.worktrees/brain-fertig-q rev-parse HEAD
binding() {
    git -C /home/nathanael/.worktrees/brain-fertig-q ls-files -z --cached --others --exclude-standard -- rust config | sort -zu | xargs -0 sha256sum
}
binding >/home/nathanael/.worktrees/brain-fertig-q/.q10-source-before.sha256
binding_exit=$?
printf 'SOURCE_BEFORE_EXIT=%s\n' "$binding_exit"
if [[ "$binding_exit" -ne 0 ]]; then exit "$binding_exit"; fi
all_exit=0
run() {
    label="$1"
    shift
    printf '\nCOMMAND[%s]: ' "$label"
    printf '%q ' "$@"
    printf '\n'
    date -u '+COMMAND_START=%FT%TZ'
    /usr/bin/setsid "$@" >/home/nathanael/.worktrees/brain-fertig-q/.q10-"$label".log 2>&1 &
    child=$!
    printf 'CHILD_PID=%s\n' "$child"
    wait "$child"
    result=$?
    child=''
    printf 'EXIT[%s]=%s\n' "$label" "$result"
    date -u '+COMMAND_END=%FT%TZ'
    if [[ "$result" -ne 0 ]]; then all_exit=1; fi
}
run consumer-libs /home/nathanael/.cargo/bin/cargo +1.97.1 test --manifest-path /home/nathanael/.worktrees/brain-fertig-q/rust/Cargo.toml --locked --offline --no-fail-fast -p brain-api -p brain-client -p brain-serve --lib -- --include-ignored
run consumer-http /home/nathanael/.cargo/bin/cargo +1.97.1 test --manifest-path /home/nathanael/.worktrees/brain-fertig-q/rust/Cargo.toml --locked --offline --no-fail-fast -p brain-api -p brain-client --test '*' -- --include-ignored
run serve-process /home/nathanael/.cargo/bin/cargo +1.97.1 test --manifest-path /home/nathanael/.worktrees/brain-fertig-q/rust/Cargo.toml --locked --offline --no-fail-fast -p brain-serve --test process --test process_e2e --test local_pilot -- --include-ignored
run consumer-docs /home/nathanael/.cargo/bin/cargo +1.97.1 test --manifest-path /home/nathanael/.worktrees/brain-fertig-q/rust/Cargo.toml --locked --offline --no-fail-fast -p brain-api -p brain-client -p brain-serve --doc -- --include-ignored
run consumer-check /home/nathanael/.cargo/bin/cargo +1.97.1 check --manifest-path /home/nathanael/.worktrees/brain-fertig-q/rust/Cargo.toml --locked --offline -p brain-api -p brain-client -p brain-serve --all-targets
run consumer-build /home/nathanael/.cargo/bin/cargo +1.97.1 build --manifest-path /home/nathanael/.worktrees/brain-fertig-q/rust/Cargo.toml --locked --offline -p brain-serve --bin brain-serve
run consumer-clippy /home/nathanael/.cargo/bin/cargo +1.97.1 clippy --manifest-path /home/nathanael/.worktrees/brain-fertig-q/rust/Cargo.toml --locked --offline -p brain-api -p brain-client -p brain-serve --all-targets -- -D warnings
run provider-tests /home/nathanael/.cargo/bin/cargo +1.97.1 test --manifest-path /home/nathanael/.worktrees/brain-fertig-q/rust/Cargo.toml --locked --offline --no-fail-fast -p brain-providers -- --include-ignored
run provider-clippy /home/nathanael/.cargo/bin/cargo +1.97.1 clippy --manifest-path /home/nathanael/.worktrees/brain-fertig-q/rust/Cargo.toml --locked --offline -p brain-providers --all-targets -- -D warnings
run fmt /home/nathanael/.cargo/bin/rustfmt +1.97.1 --edition 2021 --check --config skip_children=true /home/nathanael/.worktrees/brain-fertig-q/rust/crates/brain-api/src/audit.rs /home/nathanael/.worktrees/brain-fertig-q/rust/crates/brain-api/src/http.rs /home/nathanael/.worktrees/brain-fertig-q/rust/crates/brain-api/src/internal.rs /home/nathanael/.worktrees/brain-fertig-q/rust/crates/brain-api/src/lib.rs /home/nathanael/.worktrees/brain-fertig-q/rust/crates/brain-api/tests/request_audit.rs /home/nathanael/.worktrees/brain-fertig-q/rust/crates/brain-serve/src/audit.rs /home/nathanael/.worktrees/brain-fertig-q/rust/crates/brain-serve/src/main.rs /home/nathanael/.worktrees/brain-fertig-q/rust/crates/brain-serve/src/lib.rs /home/nathanael/.worktrees/brain-fertig-q/rust/crates/brain-serve/src/config/tests.rs /home/nathanael/.worktrees/brain-fertig-q/rust/crates/brain-serve/src/secrets.rs /home/nathanael/.worktrees/brain-fertig-q/rust/crates/brain-providers/src/lib.rs /home/nathanael/.worktrees/brain-fertig-q/rust/crates/brain-providers/src/transport.rs /home/nathanael/.worktrees/brain-fertig-q/rust/crates/brain-providers/src/tests/fireworks.rs
binding >/home/nathanael/.worktrees/brain-fertig-q/.q10-source-after.sha256
binding_exit=$?
printf 'SOURCE_AFTER_EXIT=%s\n' "$binding_exit"
if [[ "$binding_exit" -ne 0 ]]; then all_exit=1; fi
cmp /home/nathanael/.worktrees/brain-fertig-q/.q10-source-before.sha256 /home/nathanael/.worktrees/brain-fertig-q/.q10-source-after.sha256
source_exit=$?
printf 'SOURCE_COMPARE_EXIT=%s\n' "$source_exit"
if [[ "$source_exit" -ne 0 ]]; then all_exit=1; fi
exec 9>&-
exec 8>&-
printf 'LOCKS_RELEASED\n'
date -u '+END=%FT%TZ'
printf 'WRAPPER_EXIT=%s\n' "$all_exit"
exit "$all_exit"
