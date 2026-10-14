# P1r hero profile followthrough contract

Owner: `fix/p1r-profile-followthrough-20261014`, based on `f948a44b28f85e97e9109231b329721e718c2f6f`.

Goal: distinguish identity lookup from hero profile evidence in the shared native and compatible provider bridge. A successful identity lookup must guide hero profile requests toward the offered `get_hero` tool using the resolved canonical `entity_id`, before a final answer. Identity-only questions remain answerable without a profile lookup. Ambiguous and suggested identities retain their existing handling.

Boundary: `rust/crates/brain-contracts/src/provider_input.rs` and its existing tests. Preserve selected canonical citation IDs, hashes, tool result binding, grounding and API contracts. No service activation changes, live actions, release builds, extra AI calls or expanded game-data scope.

Local acceptance: formatter, affected all-target check, strict Clippy and existing relevant tests; additionally verify lookup versus profile reminders, exact canonical wire binding, error handling and unavailable profile tools across both wire formats and quality modes. Check commands and complete logs belong outside source under `/home/nathanael/.local/state/brain-p1r-proof-20261013/profile-fixer-*`.

Integration acceptance belongs to parent `502084f3-21be-4f02-9192-47dd41e1ac34`: real Baba and Bebop answers must contain the known complete profile data, resolvable selected canonical citations and a trace beginning with `find_entity` followed by `get_hero`, without legacy data or Git retrieval. A name-only answered response does not satisfy acceptance. Local wire tests cannot prove the real model effect.

Handover boundary: verified source commit and own branch push only. Parent owns gate, integration, deployment and live acceptance. P1r completion is outside this task.
