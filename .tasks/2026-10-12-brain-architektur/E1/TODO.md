# E1 gold set: state

Branch `feat/brain-e1-gold-set-20261013`, worktree `/home/nathanael/.worktrees/Deadlock-Brain-e1-gold-set`.

## Done

- Gold file `rust/crates/brain-eval/gold/e1-gold-v1.json` (schema `brain.e1-gold.v1`), 125 cases: hero 30, item 27, timer 12, patch 16, docs 10, guide 8, unanswerable 22. Facts verified against the 2026-10-13 snapshot of deadlock-api assets, Steam patch posts and the deployed docs pages.
- Format documented in `rust/crates/brain-eval/README.md`.
- Runner crate `rust/crates/brain-eval` (bin `brain-eval`), one appended workspace member line.

## Open

- Baseline run against live brain-serve (127.0.0.1:8788, docs-client token via Infisical `BRAIN_SERVE_DOCS_PUBLIC_TOKEN`, scope `bot.public`), report under `E1/baseline-20261013.json` plus `E1/baseline-20261013.txt`.
- Gate `gate_hook.py --review`, hand over the branch at ALLOW.

## Notes

- Soul Urn first spawn has no source in deadlock-api assets; both urn cases expect `insufficient_evidence` and carry a note.
- Only 10 docs pages are live; docs questions use those pages only. No hero guide pages are live, so guide cases check the hero's real ability names.
