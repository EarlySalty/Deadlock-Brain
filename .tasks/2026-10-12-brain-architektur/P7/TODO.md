# P7 Steckbrief cut

State: site renders hero and item pages from serve views, site comments retired (table kept for P8), legacy maintenance entry points frozen, migration 30 grants the site role.

Done:
- fmt, strict all-target clippy for brain-maintenance and deadlock-brain.
- Site tests against ScratchPg green, Baba visual proof in /home/nathanael/.local/state/brain-wave3-proof-20261015/P7/.
- Legacy retirement test cut at the frozen refresh entry point: refresh must be rejected and leave the HTML unchanged.

Open:
- Five ignored brain-maintenance tests need the fixed legacy cluster at /tmp/brain-maintenance-test-20261002 (not running); credential_transfer_child is a child helper and fails when run standalone. Both independent of P7; P8 removes the legacy paths.
- Unit retirement of brain-maintenance and live cutover belong to the integrator.
