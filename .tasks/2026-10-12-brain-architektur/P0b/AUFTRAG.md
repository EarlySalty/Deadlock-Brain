# P0b enforced memory caps

Owner: leaf worker in `/home/nathanael/.worktrees/Deadlock-Brain-p0b-system-units`, branch `fix/brain-p0b-system-units-20261012`. Client: orchestrator thread `89e002a7-758e-449d-9968-eec29a019a78`. This worker is authorized to integrate and deploy P0b after the local merge gate allows it.

Goal: enforce 512M, 256M and 64M caps for serve, site and consent through system units. Preserve shared executable paths, explicit operator configuration, LoadCredential, Postgres peer access and consent accessibility-bus access. Disable and stop matching user units during cutover; retain rollback copies until live acceptance. No changes to Rust application code, maintenance, legacy timers, databases, bots or historic release folders.

Acceptance: all three system services active with exact cgroup-v1 limit and readable peak; matching user services disabled or absent; normal Infisical credential path equals the unit-emitted credential path and exists without reading it; synthetic `/v1/answer` answered with citation; site `/site/steckbriefe` HTTP 200; error-priority journal empty since switch; release install restarts system services.

Procedure and rollback: `ops/brain-release/P0b.md`. Operational evidence: `/home/nathanael/.local/state/brain-p0b-proof-20261012`. P1 owns its job templates even if they appear on origin/main; P0b leaves them untouched and documents a separate explicit install path that needs no P0b helper-code change.
