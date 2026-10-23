# Alias EXACT audit fix

State: implemented on fix/brain-alias-exact-audit-20261023, tests, fmt and clippy green.
Root cause: DISTINCT ON serve.current_* views evaluated fully when joined on non-key columns, so
serve.receipt_is_published ran per row once audit rows exist.
Fix: keyed lateral lookups in find_entity and migration 35 (serve.timer, p4_patch_catalog, p4_patch_change).
Proof: /home/nathanael/.local/state/brain-wave2-proof-20261014/alias-exact-fix/TIMINGS.md
Open: integrator merge after gate ALLOW.
