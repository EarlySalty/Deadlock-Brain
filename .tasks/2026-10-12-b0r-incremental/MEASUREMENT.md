# B0R targeted incremental measurement

This branch is measurement evidence only, not an application integration candidate. Base source: ef0418d5. Worktree: /home/nathanael/.worktrees/Deadlock-Brain-b0r-profile-measure. Owner: B0R session 3d53b662-0b9d-4b53-881a-e2c48f42956a.

The single source-line change passes the same token string directly to Sha256::digest instead of calling as_bytes first. It preserves the input bytes and API. No service, production database, deployed artifact, migration or foreign worktree is changed.

Command:

```text
/home/nathanael/.worktrees/claude-config-build-speed/bin/cargo-slot +1.97.1 test --manifest-path /home/nathanael/.worktrees/Deadlock-Brain-b0r-profile-measure/rust/Cargo.toml --locked --jobs 4 -p brain-policy --lib
```

| Run | Wall seconds | Cargo seconds | Recompilation | Result |
| --- | ---: | ---: | --- | --- |
| Initial targeted setup | 17.342 | 13.44 | Targeted feature-set dependencies and brain-policy | 6 passed, 0 ignored |
| Unchanged warm baseline | 3.874 | 0.08 | None reported | 6 passed, 0 ignored |
| One source-line change | 3.622 | 0.35 | Only brain-policy reported | 6 passed, 0 ignored |

All three jobs ran on ionos with jobs=4 using the same remote workspace 173350f76360a10f and verify artifact path. The first two used slot 1; the changed run used slot 2. No cache was deleted. The setup run is not a cold full-workspace benchmark. The small wall-time difference is not claimed as a compiler speedup: SSH and synchronization dominate these runs. The changed Cargo run is the actual rebuild proof.

Formatter and package-only Clippy with -D warnings passed, task b7xc6virx. Logs and monotonic wall timing files: /tmp/b0r-profile.f2pfCi/brain-incremental-{unchanged,warm-before,one-line}.{log,time}. Client RSS in those files is not compiler RAM.

Commit-only reuse proof remains pending. No full workspace test suite was run for this measurement.

TESTNACHWEIS[TW-1]: 6 passed, 0 ignored | Baseline: 0 rot in the unchanged targeted run
