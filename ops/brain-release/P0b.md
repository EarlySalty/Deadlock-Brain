# P0b enforced system-unit memory caps

Goal: enforce memory limits on the OpenVZ cgroup-v1 host without changing application code, maintenance, timers, Postgres or bots. Acceptance requires the live cgroup limit and readable peak for each service, inactive and disabled old user services, successful cited answer and site HTTP 200, and no new error-priority journal entries.

`brain-release install` and `rollback` install the three embedded templates under `/etc/systemd/system`, install the embedded normal Infisical configuration, reload the system manager, enable and restart system services through `sudo -n systemctl --system`, and check their enforced cgroup-v1 limits. The existing deploy lock covers this sequence. Old user base units are disabled and stopped before configuration changes or system-service startup. Their files remain for rollback until the separate live acceptance passes; remove only these three base files afterward and reload the user manager. A service-install failure is reported as a failed install; release-pointer recovery does not claim to recover service configuration.

| System service | MemoryMax | Expected bytes |
| --- | --- | --- |
| brain-serve.service | 512M | 536870912 |
| deadlock-brain-site.service | 256M | 268435456 |
| brain-browser-consent.service | 64M | 67108864 |

All units use `User=nathanael` and `Group=nathanael`. Systemd supplies HOME for that account. Absolute operator paths replace `%h`, which would denote root's home in system units. No extra HOME or XDG environment setting is required: serve and site receive explicit configuration paths; site uses peer-authenticated Postgres at `/run/deadlock-brain-postgresql` port 5446; supplementary account groups remain available. Consent receives its existing absolute accessibility-bus address from `/opt/deadlock-brain/browser-consent/brain-browser-consent.json` and does not discover a session bus through XDG_RUNTIME_DIR. No new ProtectHome or filesystem restrictions are added. No compiler cache is required at runtime.

The old site's optional legacy EnvironmentFile and user-manager failure notifier are not carried into the new system unit. The site uses its explicit peer configuration, not an exported secret environment; the notifier is a user-manager unit, not a system-manager dependency. Its old drop-ins remain on disk for rollback. Caddy and the session bus are untouched.

The normal Infisical config points to `/run/credentials/brain-serve.service/infisical-token`. Serve's ExecStartPost compares this literal to `${CREDENTIALS_DIRECTORY}/infisical-token` and checks regular-file presence without reading credentials. Live verification must additionally compare the installed normal JSON path with the path emitted by ExecStartPost. LoadCredential continues using the existing secret-manager-managed credential source. No token copies or secret-export environment files are created.

## Switch and rollback

Before installing, verify no `brain-release` process exists and retain the three current user base units plus normal `/etc/deadlock-brain/infisical-serve.json` under `/home/nathanael/.local/state/brain-p0b-rollback-20261012`. Retain an operator-readable metadata-only backup of the latter. Install the checked release helper through the existing root-owned installation path. Use `plan`, `build`, `verify`, and privileged `install` on the clean worktree at verified origin/main. The installer stops old services before starting new ones.

Rollback sequence, usable both before and after old base-file removal:

```bash
sudo -n systemctl disable --now brain-serve.service deadlock-brain-site.service brain-browser-consent.service
sudo -n install -o root -g root -m 0644 /home/nathanael/.local/state/brain-p0b-rollback-20261012/infisical-serve.json /etc/deadlock-brain/infisical-serve.json
install -m 0644 /home/nathanael/.local/state/brain-p0b-rollback-20261012/brain-serve.service /home/nathanael/.config/systemd/user/brain-serve.service
install -m 0644 /home/nathanael/.local/state/brain-p0b-rollback-20261012/deadlock-brain-site.service /home/nathanael/.config/systemd/user/deadlock-brain-site.service
install -m 0644 /home/nathanael/.local/state/brain-p0b-rollback-20261012/brain-browser-consent.service /home/nathanael/.config/systemd/user/brain-browser-consent.service
systemctl --user daemon-reload
systemctl --user enable --now brain-serve.service deadlock-brain-site.service brain-browser-consent.service
```

This reverts enforcement to the known previous user-unit state, not to enforced memory caps. If the release itself fails, use the retained previous helper and its pointer rollback before this sequence. Do not use the new helper's rollback to restore user units: it intentionally restarts system units.

## Caller inventory and later jobs

Repository search found no operational shell health/deploy caller for these three units. `rust/crates/brain-maintenance/src/integration/activation.rs` still invokes `systemctl --user restart` for the configured serve unit. Maintenance and its user-disablement drop-in remain unchanged and inactive; P7 must change or retire that activation before maintenance can run. The historical `architecture/migration/CUTOVER_RUNBOOK.md` has user-manager inspection commands and is outside this package's write area.

For every future Brain job, install its package-owned template in `/etc/systemd/system`, never in the user manager. Use `User=nathanael`, explicit absolute config/credential-source paths, `LoadCredential`, `MemoryAccounting=yes` and a measured `MemoryMax`; bind its timer to the system service and enable it through `sudo -n systemctl`. Stop and disable a matching old user timer and service before enabling the system timer. Prove `/sys/fs/cgroup/memory/system.slice/<instance>/memory.limit_in_bytes` and `memory.max_usage_in_bytes` on a real execution. P1 owns `ops/brain-job@.service` and `ops/brain-job@.timer`; P0b never creates or moves those files, even if P1 merges during this package.

P1 can install its additional units without editing the P0b helper. After its own gate, merge, release installation and verification, use this explicit install path from P1's clean verified origin/main worktree. Hold the shared deploy lock and ensure no release build or install is running. Do not enable a timer until its service has passed the package's real-execution proof.

```bash
sudo -n flock --exclusive /opt/deadlock-brain/.deploy.lock /usr/bin/install -o root -g root -m 0644 ops/brain-job@.service ops/brain-job@.timer /etc/systemd/system/
sudo -n systemctl daemon-reload
sudo -n systemctl start brain-job@ingest-gamefiles.service
sudo -n systemctl enable --now brain-job@ingest-gamefiles.timer
```

If matching user units already exist, disable and stop those instances before installation. The explicit privileged install uses the independently reviewed package files; the root-owned P0b helper does not accept arbitrary unit text or invoke arbitrary caller-selected commands. Its future releases preserve P1's additional system units because it installs only its three named services.
