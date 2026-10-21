# P7 Steckbrief cut

State: site renders hero and item pages from serve views, site comments retired (table kept for P8), legacy maintenance entry points frozen, migration 30 grants the site role.

Done:
- fmt, strict all-target clippy for brain-maintenance and deadlock-brain.
- Site tests against ScratchPg green (fresh run log site-test.log), Baba visual proof baba.png in /home/nathanael/.local/state/brain-wave3-proof-20261015/P7/.
- Legacy compare artifact URLs (/brain/site/compare/<64hex> and /chart.svg) stay served through serve.read_compare_artifact_v1/v2 wrappers from migration 30, executable only by brain_site, read-only under the 500 ms guard; the site role keeps no brain-schema access.
- Legacy retirement test cut at the frozen refresh entry point: refresh must be rejected and leave the HTML unchanged.

Open:
- P8: link producers must switch, or legacy artifact tables must be kept/migrated before drop.
- Five ignored brain-maintenance tests (four in runner_tests, workflow postgres_revocation_after_scan_blocks_actual_provider_dispatch) need the fixed legacy cluster at /tmp/brain-maintenance-test-20261002, which is not running. They fail identically on base d6077a8e, evidence base-d6077a8e-maintenance-legacy-cluster.log in the proof dir. credential_transfer_child is a child helper and fails when run standalone. Both independent of P7; P8 removes the legacy paths.
- Integrator live check before enabling the wrappers: SELECT has_function_privilege('brain_migrate','brain.read_compare_artifact_v1(text)','EXECUTE') and the same for v2 must be true, otherwise legacy compare links answer 503.
- Unit retirement of brain-maintenance and live cutover belong to the integrator.
