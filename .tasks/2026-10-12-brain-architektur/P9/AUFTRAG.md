# P9 receipt audit

Goal: daily bounded verification of approximately 200 canonical receipts against their stored raw bytes; quarantine mismatching facts and exclude them from serving. Record results in ops. No ingestion data-quality fixes or other packages' changes.

Acceptance: one deliberately corrupted receipt in an isolated PostgreSQL database quarantines exactly its fact and removes it from serve results; the production system-unit run finishes below 120 seconds and 200 MB cgroup-v1 peak. Run relevant existing tests before and after implementation, formatting, Clippy, matching build and independent review. Integrate only after P1 is live and local gate ALLOW.

Initial inspection on 2026-10-12 found no committed brain-job crate at a808f5a492b306d63a2a1a42b533dd1aeaf2d290. The orchestrator instructed P9 to wait for the locator contract while implementing the additive quarantine and scratch projection fixture. P1 published 3a0740be219fb4aceb304e8020501dcd7281fbd7, integrated with an explicit merge commit. Use its public extract::Assertion, extract_with_identities and prove contracts, including lexeme hashing and occurrence-aware kv_pointer. No parallel locator parser.

The base has only one embedded gamefacts SQL file, applied when ops.gamefacts_schema_version is absent; it does not yet have an ordered package migration list or migration ledger. P9 must append its own embedded migration registration and additive application/check path, preserving historical migration bytes and P1 schema version 1.

P0b.md is absent at the initial P1 checkpoint and exists on fetched origin/main ef0418d5. Its system-unit install contract was read from git without changing another worktree. P9 owns standalone system service/timer instances, not P1's templates. Live deployment remains blocked until P1 live acceptance.

The standalone additive migration creates ops.receipt_audit and canon.receipt_quarantine. Only the security-definer audit-result trigger writes quarantine; ingest has no direct quarantine INSERT, UPDATE or DELETE permission. A mismatch refreshes the existing hero projection in the same serialized transaction. Current fact/alias/relation views exclude quarantined receipts. Receipt RLS makes the existing tool hydration fail closed for quarantined identity citations without modifying P1's serving implementation. Additional audited_entity/audited_hero_profile views filter identity and ability citations directly for future serve consumers. Historical canon rows are preserved.

A socket-only scratch PostgreSQL 16 run passed on 2026-10-12: /tmp/brain-p9-sql-proof.log, cluster provenance /tmp/brain-p9-sql-root.txt; the owned cluster was stopped after the run. This is a SQL projection/RLS/grant/trigger proof with deliberately seeded audit outcomes, not an executed brain-job proof. Independent database review found no concrete defect in the migration or added policies; it confirmed the existing tool's missing-receipt gate. Cargo baseline and Rust projection test were queued through cargo-slot but had not obtained slots at this checkpoint.

The orchestrator reserves gamefacts migration version 6 for P9. Keep the migration standalone until P1 publishes its ordered migration registration checkpoint. P9 reuses the committed locator verifier; its Rust runtime draft, real-job scratch acceptance test and daily timer are not included in the SQL-only verified checkpoint.

Client: orchestrator thread 89e002a7-758e-449d-9968-eec29a019a78. Worktree: /home/nathanael/.worktrees/Deadlock-Brain-p9-receipt-audit. Branch: feat/brain-p9-receipt-audit-20261012.
