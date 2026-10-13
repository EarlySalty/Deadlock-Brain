# P5 continuation, 2026-10-13

Branch: feat/brain-p5-docs-20261012-v2. Resume HEAD: 3fc7f93323133324bf683651f8170b553a5f97ea.
Primary worktree: /home/nathanael/.worktrees/Deadlock-Brain-p5-docs.

Preserved the previous AUFTRAG.md changes. Committed parser and scoped contract helpers are reused; their reported checks are not repeated as baseline work. No other uncommitted code or P5 worker checkpoint needs salvage.

BESTAND[BS-1]: teilweise | Fundort: rust/crates/brain-job/src/docs_extract.rs:362 | Anknüpfung: existing verified HTML parser, docs_tools bounded outputs and policy helpers, published P1r Store.

Remaining: adapt pure parser to the agreed doc_page/chunk wire format within 800 lines including tests; implement migration 5 with coherent serve-only projection and indexed FTS; add production reader/wiring and streaming Git source. Then run real Store poison, two-run idempotence, scope/quarantine/receipt proofs, relevant checks and independent review.

Core dependency: remote P1r provides e3d96fa6 (core 2) and 9f75588d (byte_ranges), inspected read-only. AUFTRAG forbids importing unforwarded later checkpoints, so merge awaits the orchestrator's forwarding. No substitute core implementation.

Integration and deployment belong to P1r under resume rules. Do not deploy or enable jobs here. Brain maintenance and dl-knowledge stay stopped. Discord switch remains deferred until the Brain replacement is live; Bots WIP worktree stays preserved.

Next step: implement owned adapter, projection/reader and source against approved contracts, merge forwarded core 2/2b, run missing real checks, obtain branch gate ALLOW and hand over SHA.
