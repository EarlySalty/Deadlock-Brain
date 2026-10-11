# P0: one release, one config, no Python

Goal: every retained Brain unit uses the same release, systemd credentials and an explicit memory cap. Preserve maintenance activation until P7.

Write boundary: ops/brain-release, maintenance unit templates, the listed host units, the obsolete serve configuration and these package notes. Maintenance application logic, legacy timers and databases are excluded.

Acceptance: one live serve configuration without an ExecStart drop-in; shared release binaries; credential loading without shell or Python wrappers; memory limits; answered live request; port 18768 closed; successful maintenance activation after the change.

## Scope blockers measured on 2026-10-12

- HEAD: 17a8e22227688742672e072aa4911ba4d119e9ff, branch fix/brain-p0-one-release-20261012.
- current resolves to /opt/deadlock-brain/releases/17a8e22227688742672e072aa4911ba4d119e9ff; maintenance-current resolves to the matching maintenance-releases directory.
- brain-maintenance.timer is disabled and inactive. brain-maintenance.service is inactive with ConditionResult=no. Its 99-user-disabled-20261012.conf requires /home/nathanael/.config/deadlock-brain/ENABLE-brain-maintenance; that marker is absent. This is a pre-existing explicit disablement, not permission to re-enable maintenance.
- The maintained source in rust/crates/brain-maintenance/src/integration/entity_profiles.rs:619 opens /proc/self/fd/5 when transferring the credential to brain-candidate-activate. The current bash ExecStart creates this FD. In-process Infisical loading supports CREDENTIALS_DIRECTORY in rust/crates/deadlock-brain-core/src/pg_secrets.rs:164, but does not retain an FD5. deadlock-brain-secret-exec fetches secrets and execs; it also does not create FD5. Replacing the wrapper alone breaks candidate activation.
- The proxy on 127.0.0.1:18768 is listening with no established socket observed. No matching consumer reference was found in the inspected ops, config or maintenance sources.
- Six legacy timers remain inactive despite their existing enabled state. They were not changed.

## Orchestrator waiver on 2026-10-12

Responsible orchestrator: thread 89e002a7-758e-449d-9968-eec29a019a78. No Rust maintenance change and no maintenance tick are authorized. The existing 99-user-disabled-20261012.conf and the maintenance ExecStart with its FD5 shell wrapper stay untouched. Only MemoryAccounting=yes and MemoryMax are added to maintenance. Disabled maintenance is excluded from the enabled-unit credential and ExecStart acceptance; wrapper removal moves to P7. Its post-change activation proof is waived. The live serve configuration remains at the writable path already named by activation.

Continue the shared-release change, enabled-unit credentials and memory caps, orphan proxy shutdown, gate, merge, deployment and live answer proof. The six stopped legacy timers remain untouched.

## Enabled serve credential blocker

The prepared direct serve template must not be installed yet. `brain-serve` uses `dl_token_secrets::values`, not the core credential loader cited in the initial investigation. Its resolved dependency is `/home/nathanael/.worktrees/Deadlock-Bots/rust/crates/dl-token-secrets/src/lib.rs`; `load_credential` at line 246 only accepts an existing FD or an absolute configured credential path. It does not inspect `CREDENTIALS_DIRECTORY`. The existing normal Infisical config only selects FD5. `deadlock-brain-secret-exec` exports an environment snapshot but does not provide the FD or private snapshot that serve reads, so that alone does not solve startup.

A harmless transient systemd probe running only `/usr/bin/test -f /proc/self/fd/5` established that `OpenFile` cannot open the unit's LoadCredential path before credentials are prepared. Its journal recorded only the missing path. No credential contents were read or output by the probe.

Approved scope extension on 2026-10-12: `/etc/deadlock-brain/infisical-serve.json`, a normal non-secret configuration mirrored under ops/brain-release, with `credential_path` set to `/run/user/1000/credentials/brain-serve.service/infisical-token`. Reuse all existing transport/project settings; change no model, credential values, Rust application code, shared library or maintenance path. Point only the serve unit's existing `--infisical-config` flag at that configuration. This keeps the sole live brain-serve.json unchanged. Runtime proof must compare the configured path to the unit's CREDENTIALS_DIRECTORY plus infisical-token and print only the path and existence, never content. Back up the previous serve unit and drop-in before restarting so a daemon-reload and rollback restart restore the previous state if readyz fails.

The shared-release alias change and memory-capped unit templates are prepared. Rust formatting and systemd template verification pass. A synthetic Discord baseline request for `Wann spawnt die Urne?` returned HTTP 200, application/json, status answered and one citation. The relevant Rust suite remained queued behind occupied Cargo slots and was stopped before execution when this scope blocker was established. The separate lint/build queue was also stopped. No green Rust test/build or merge is claimed. The read-only reviewer was stopped without a completed verdict. Production service/config/timer state is unchanged.

## Prepared checkpoint

The approved normal credential-path configuration and direct serve unit are now prepared together. The previous serve base unit and ExecStart drop-in are retained at `/home/nathanael/.local/state/brain-p0-rollback-20261012`. A fresh independent Opus reviewer accepted the static package after an isolated fixer corrected one stale journal-recovery test assertion; that auxiliary worktree and branch were removed after importing the exact fix. Formatting and systemd unit syntax checks pass. Production remains unchanged. Relevant helper tests, Clippy and the production helper build are queued sequentially through cargo-slot with central artifacts at `/home/nathanael/.cache/rust-build/brain-release-p0-20261012`; occupied shared slots and an unrelated release build delay execution. This checkpoint does not claim build, test, gate, merge or deployment completion.

## Memory-enforcement waiver

On 2026-10-12 the main orchestrator confirmed that this host is OpenVZ with cgroup v1. MemoryMax in systemctl --user units is recorded, not enforced; MemoryPeak remains unset. P1's independent probe configured 8 MiB, touched 32 MiB, reached 44,416 KiB RSS and exited successfully. Its evidence is `/home/nathanael/.worktrees/Deadlock-Brain-p1-baba-pg/.tasks/2026-10-12-brain-architektur/P1/MEMORY-PROOF.md`. P0 must finish the approved unit configuration and live answer proof without claiming memory enforcement. P0b, a separate package after this merge, moves services to system units with User=nathanael. P0 must not change unit scope, controllers or host virtualization.

## Deferred removal inventory

- Historical maintenance-releases directories and old releases beyond retained rollback versions.
- ~/.local/share/deadlock-brain/releases/25c6ed6-*.
- build-data-runtime-b7289d11.
- wiki-refresh/deadlock-brain legacy symlink.
- scripts/export_infisical_env.py and the three stopped legacy secret wrappers.
- ~/.local/share/deadlock-brain/wiki-refresh/infisical.json.

These are listed from LUECKE rows 1 and 2, not deleted or certified unused.
