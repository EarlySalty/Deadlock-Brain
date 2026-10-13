# P4 gate round2 causal verification

## Scope and disposition

Actual gate on published code126f6f397580491ee4fec8edc40e84a2a782c5c0 against origin/main edf503b280777524093307a97fabbb1bdba83d7c: gpt-6.1-sol BLOCK, taskbxr8zwqe2, exit1. No further gate was run. Fresh native fixer used its own worktree `/home/nathanael/repos/Deadlock-Brain/.claude/worktrees/agent-a70de76e5a6a76d45`, branch worktree-agent-a70de76e5a6a76d45, clean HEAD126f6f39 and unchanged merge parents. Parent independently read the real result logs and verified artifact hashes. Three blocking causes and two nits are confirmed. No frozen production, applied migration or service file was changed.

## 1. Published canonical data can be inserted and withdrawn

Locations: scripts/migrations/2026-10-13-brain-gamefacts-run-local-v7.sql:17, v1.sql:117,147. Real own disposable PostgreSQL with migrations1/4/7, actual brain_ingest role without superuser/migration-role authority:

- Number/text/bool mismatched inserts: failed valid_from_run plus succeeded receipt appear published3/3.
- Subsequent write_run_fact with an unpublished receipt withdraws the published row3/3.
- With an older published comparison fact, visible number reverts41→10.
- Controls reject a published destination run with23514 and direct UPDATE with42501.
- Late receipt plus entity/identity/alias/relation INSERT under a succeeded run remains possible; four writer twins accepted and the three corresponding current views expose new rows.

Minimal owner repair contract: fact valid_from_run and receipt ingest_run_id must match; protect both INSERT and conflict replacement against already published data. Enforce terminal-publication immutability centrally across receipt/entity/identity/alias/fact/relation and marker write paths. Do not equate raw.object capture run with a new receipt run: legitimate previously captured raw-object reuse must remain valid.

## 2. Superseded profile fields and aliases remain current

Reader twins: brain-storage/src/gamefacts/hero_profile.sql:22,55,71, and exact/fuzzy aliases brain-serve/src/pg_tools.rs:18,26. The retained synthetic real-PG proof is reused rather than rerun. Current rendered v1/v4/v7 are byte-identical to the actual prior proof, exact/fuzzy query texts match after parameter substitution, and owner readers are unchanged versus approved982645cf. Seven retained-result checks passed.

Current identityB still exposes omitted name_de/armor/weapon fromA; both exact/fuzzy reads expose a removed alias once. Coherent comparison queries expose0 removed aliases and0 omitted facts. Retained seed was brain_core_test/migration admin; this is reader-coherence proof, not a brain_ingest privilege-boundary test.

Minimal owner repair contract: one coherent current snapshot selection with raw/run/extractor identity for all five readers. Preserve original rendered migration inputs/checksums; use an additive migration and approved owner query changes rather than editing an applied SQL file or the old embedded hero SQL.

## 3. Receipt cap accumulates across repeated applications

Location: scripts/migrations/2026-10-12-brain-gamefacts-v1.sql:55. Actual checkpoint Rust ingest runner/store on own real scratch PG, role brain_ingest:

Same-run[A,B,A], each valid application4096 receipts, yields2 accepted/1 rejected/0 ignored, status succeeded. FinalA returns sql_write/23514 at transient raw/run/extractor group8192. Rollback leaves4096 receipts perA/B and4095/4095 current facts fromB; raw objects2. All five receipt-writer paths are confirmed: entity/alias/fact/relation first4096 accepted, repeat rejected23514; empty marker4096 accepted after4095 prepared receipts, marker4097 rejected. Counter-control application4097 is correctly rejected by Rust as too_many_rows. These are concrete rejected valid applications, not hypothetical cumulative growth.

Minimal owner repair contract: enforce unchanged4096 per actual application, including empty marker, rather than lifetime raw/run/extractor receipt aggregate. Preserve old receipt identities/history and legitimate reruns. Post-fix[A,B,A] must be3 accepted/0 rejected/0 ignored andA current. Do not fix by raising the cap or discarding prior receipts.

## Two confirmed owner integration nits

- brain-serve/src/service.rs:623 constructs PgTools<deadlock_data::Tools> without PgPatchToolPort. Actual combined tests explicitly wrap it; no production wiring claim is made.
- ops/brain-release/src/units.rs:9,90,128 covers Serve/Site/Browser-Consent but not brain-job@.service, brain-job@.timer or brain-job@deadlock_patchnotes_db.timer. gamefacts.rs does not add installation. Static implementation inspection only; no installer or service was executed. Responsible integrator must cover installation/verified activation at the assigned rollout boundary, not this P4 worker.

## Exact reproduction and evidence

Probe sources and independent locked temporary admin/test workspace: `/tmp/p4-round2-a70de76e5a6a-probes/`. Application crates are exact checkpoint path dependencies; this temporary Cargo.lock is separate from the repository lock. It is not a repo dependency update or new production tool/store. Normal cargo-slot selected local execution because of absolute path dependencies; no slot/remote marker/cache bypass or release build.

```text
/home/nathanael/.local/bin/cargo-slot +1.97.1 test --manifest-path /tmp/p4-round2-a70de76e5a6a-probes/Cargo.toml --locked --jobs 3 --test causal -- --include-ignored --nocapture --test-threads=1
python3 -I /tmp/p4-round2-a70de76e5a6a-probes/retained_proof.py
/home/nathanael/.local/bin/cargo-slot +1.97.1 clippy --manifest-path /tmp/p4-round2-a70de76e5a6a-probes/Cargo.toml --locked --jobs 3 --test causal -- -D warnings
/home/nathanael/.cargo/bin/cargo +1.97.1 fmt --manifest-path /tmp/p4-round2-a70de76e5a6a-probes/Cargo.toml -- --check
```

Actual3 passed/0 failed/0 ignored/0 filtered,61.22s, exit0; formatter and Clippy exit0. These tests prove defects, not a green application acceptance baseline. No accepted broad suite was repeated.

Artifact SHA256:
- causal.rs:cf1d09af56b2355f87ce60a2c8a51cabfe2f29cfaad16e92141b5886db884342
- Cargo.lock:4921972412a0952f9120df49ce6866cb8bb6dc05f68dd0775acace3f284d2840
- tests.log:8621f1d7adefbed985276d2e6b6cd06e406ef30fe9c28bf464518e4e5f931c58
- retained_proof.log:1c8a98e8ac03014517e31ec6108f045f613af0f554693d42fd1ddfb82331f79e
- clippy.log:d2441e43b06f24296b87e2c57edf5815dd881145125e7c682e974e4a02b10bcf

Retained reader proof directory `/tmp/p4-gate-r1-pg-LKliLN/` is unchanged. No production feed or secrets accessed, no raw DB contents reported, no external AI/STT/user data action or service change. Original source skip/log/count and canonical-format output/memory proofs remain valid.

WIRKUNGSPRUEFUNG[WP-1]: 5 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 0/0 geprüft
TESTNACHWEIS[TW-1]: 3 passed, 0 ignored | Baseline: 0 rot
MERGEPROTOKOLL[MS-1]: 1 Git-Schritte einzeln | Anläufe: 0 | Gate: vorliegend gpt-6.1-sol BLOCK

## Required owner checkpoint

P1r must publish an approved additive Core repair covering the three confirmed causes, with actual role-boundary/coherent-reader/application-cap proof and no changed applied migrations or receipt formatter. Parent can then normally merge it, rerun only affected checks and the same full gate; a new actual BLOCK requires a fresh fixer. Only content rounds1/2 have been used. P1r remains sole main integrator/deployer; no ALLOW/main/live/worktree cleanup is claimed.
