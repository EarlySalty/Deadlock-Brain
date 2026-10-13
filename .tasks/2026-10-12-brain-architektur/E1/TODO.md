# E1 gold set: state

Branch `feat/brain-e1-gold-set-20261013`, worktree `/home/nathanael/.worktrees/Deadlock-Brain-e1-gold-set`.

## Done

- Gold file `rust/crates/brain-eval/gold/e1-gold-v1.json` (schema `brain.e1-gold.v1`, sha256 `bdfcf6c2...60e7bd`), 125 cases: hero 30, item 27, timer 12, patch 16, docs 10, guide 8, unanswerable 22 (DE 68, EN 57). Facts verified against the 2026-10-13 snapshot of deadlock-api assets, Steam patch posts and the deployed docs pages.
- Format documented in `rust/crates/brain-eval/README.md`.
- Runner crate `rust/crates/brain-eval` (bin `brain-eval`), one appended workspace member line. clippy `-D warnings`, 4 unit tests and `cargo fmt --check` green.
- Baseline before the cut, one run on 2026-10-13 against live `brain-serve.service` (127.0.0.1:8788, docs-client token from Infisical `BRAIN_SERVE_DOCS_PUBLIC_TOKEN`, scope `bot.public`, knowledge release `maintenance-bf5a6d0d...`): `baseline-20261013.json` and `baseline-20261013.txt`.
  - answered rate 10.9% (11/101), correctness 5.0% (5/101), answered precision 45.5% (5/11), citation rate 100% (11/11).
  - non-answerable compliance 83.3% (20/24); 2 answered (unanswerable-001 unknown hero answered with a refusal text, timer-012 Soul Urn "10:00").
  - statuses: answered 13, insufficient_evidence 84, unauthorized_evidence 18, unavailable 10. All 28 unauthorized/unavailable carry "Werkzeugantwort konnte nicht sicher abgeschlossen werden."

## Open

- Gate `gate_hook.py --review`, hand over the branch at ALLOW.

## Notes

- Soul Urn first spawn has no source in the deadlock-api assets snapshot; both urn cases expect `insufficient_evidence`. The live brain answered "10:00" from its own game-data revision 330a775 for the EN case; P3 should confirm or refute that source before the gold expectation changes.
- The live brain named Infernus' first ability "Catalyst"; the 2026-10-13 deadlock-api snapshot says "Napalm". The gold file follows the API.
- Only 10 docs pages are live; docs questions use those pages only. No hero guide pages are live, so guide cases check the hero's real ability names.
