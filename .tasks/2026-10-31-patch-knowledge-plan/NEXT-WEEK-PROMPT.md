# Brain patch knowledge: entity timelines and real game understanding

## Goal
Every Deadlock entity (hero, ability, item, objective, map mechanic) gets a complete, ordered change timeline from 2024-05 to today, and the answer model (Haiku) answers patch and gameplay questions correctly and with game understanding. "Correct" means: every number and date in an answer comes from a patch line with a citation, and the answer explains what the change means in play.

Acceptance: a probe set of 60 questions (20 hero/ability, 15 item, 15 objective/map, 10 cross-entity "what changed about sustain / comeback / Mid-Boss since X") answered through POST /v1/answer with at least 54 graded correct by an Opus-high grader against gold answers, zero invented numbers, and every answer citing patch ids. Measured before and after, numbers in the report.

## Facts (measured 2026-10-31, do not re-derive)
- Central DB patchnotes.changelog_posts: 150 posts, 2024-05-03 to 2026-10-30. Brain ops.patch_ledger: 134 patches (57/40/37 per year), ops.patch_ledger_post: 134 primary, 10 duplicate, 6 excluded (hero reveal posts of October 2026). Coverage is complete; nothing needs re-importing.
- canon.patch_line: 11,243 lines. entity_type ability 3,922 (0 unresolved), item 2,679 (42 unresolved), hero 1,066 (6 unresolved), objective 548 (all unresolved, no entity kind exists), general 3,028 (all unresolved by design). Structured old/new on 3,935 lines only.
- Serving (brain-serve: list_patches, patch_history, get_mechanic) reads the old per-entity fact lines (serve.patch_line_text, 20,257 rows) by regex, never canon.patch_line or serve.patch_change_v2. Mid-Boss regex `mid ?boss` misses the hyphenated spelling (hotfix branch fix/midboss-regex).
- 1,469 Haiku/Opus tie-break corrections exist under .tasks/2026-10-29-brain-patch-ledger/import/ and were never applied (`brain-job patch-line-import <tsv>`).
- Worktree ~/.worktrees/brain-objective-entities holds an uncommitted objective entity implementation (migration v58, objectives.json, objectives.rs, 11 files). Finish it, do not rebuild.
- Docs path works as knowledge channel: Deadlock-Docs public/*.html, hourly brain-job@deadlock-docs, search_docs returns 1,500 chars per section. Pilot page: public/deadlock-patches/halloween-2026-gameplay-update.html.
- Answer model: Haiku through dl_ai_gateway slot brain_answer. Tools: find_entity, get_hero, get_item, get_ability, get_timer, get_mechanic, list_patches, patch_history, search_docs, get_meta_stats, compare. Traces in ops.answer_trace.

## Why Haiku answers are weak today
Haiku is a query-time narrator. It gets 6 to 20 raw lines per tool call, no order, no old/new, no context, and has to invent the interpretation itself. Game understanding cannot come from Haiku at query time. It must be precomputed by a strong model and served as structured data, so Haiku only has to read, order and phrase.

## Work packages (in this order, each merged to main through gate-review before the next starts)

### P1 Entities for everything that changes
- Finish brain-objective-entities: kind `objective` for Mid-Boss, Rejuvenator, Urn, Trooper, Walker, Guardian, Base Guardian, Patron, Shrine, Neutral Camp, Sinner's Sacrifice, Breakables, Souls/Comeback, Lane, Map (list from objectives.json, extend from the 548 objective lines and the top subjects of the 3,028 general lines).
- Resolve all objective lines and the general lines that name a mechanic. Remaining general lines get entity `patch` (the patch itself) so nothing is orphaned. Target: unresolved lines under 300, each listed with reason.
- Apply the 1,469 tie-break corrections. Add structured old/new extraction for the "from X to Y", "X -> Y", "increased/reduced by", "now …" forms; target structured share from 35 percent to over 70 percent, measured on canon.patch_line.

### P2 Entity timeline as the single serving source
- New table or view `serve.entity_timeline` (entity_id, ability_id, released_at, ledger_id, patch_label, patch_kind, section, stat_name, change_type, old_value, new_value, raw_line, line_id), ordered, one row per change, fed from canon.patch_line. Rebuild on every patchnotes ingest.
- Switch list_patches, patch_history and get_mechanic to this source. patch_history returns the timeline in date order, oldest first, grouped by stat, with old/new columns, always with the patch label and date, default window last 12 months, parameter for full history. get_mechanic gets a `history` field from the same source. No regex scoping any more.
- Tool output must be compact and uniform: `2026-10-30 Halloween 2026 + Gameplay Update | Mid-Boss | HP | 12500 -> 14500 | line 4543-17`. Haiku reads it like a table.

### P3 Synthesis layer written by Opus high (the game understanding)
- Per entity: a dossier `entity_note` (German, 600 to 1,200 chars): what the entity does, its role in the game, the 3 to 6 most important changes with dates, current state, typical interactions. Source: timeline rows plus current facts. Every sentence that carries a number references line ids. Stored in canon as facts (stat_key `note_role`, `note_history`, `note_interactions`) and served by get_hero/get_item/get_ability/get_mechanic. Start with the 60 most-asked entities (count from ops.answer_trace), then all.
- Per patch: a digest page like the Halloween pilot for the 17 major and 50 gameplay patches; hotfixes grouped per month. Same Docs page contract, sections under 1,400 chars, claims file per page.
- One game model page (Docs public/): phases of a match, soul economy and comeback, objectives in order, sustain vs anti-heal, stealth, stamina, what a nerf of X usually means for Y. Written once by Opus high, 15 to 25 sections, cited to current facts where numbers appear. This is what gives Haiku its "game understanding" through search_docs.
- Writer prompt rules: numbers only from timeline rows, interpretation marked "Einschätzung", no outside memory, no em dashes, real umlauts, English entity names.

### P4 Haiku answer prompt and tool strategy
- Kernel prompt additions: always call patch_history or get_mechanic before stating any change; order changes by date; name patch label and date for every number; when asked "why" or "what does it mean", use the entity note first and the game model page second; never generalize from one line; when a stat has several changes, show the path (old -> mid -> new) not only the last; if no row exists, say so instead of guessing.
- Answer shape for patch questions: one sentence current state, then the timeline (max 8 rows), then one or two sentences of consequence from the dossier, then citations.
- Few-shot: 6 gold answers (hero nerf path, item rework, objective change, cross-entity theme, "no change" case, ambiguous entity asking back) added to the prompt or to a docs page that the kernel loads.

### P5 Measurement
- Probe set of 60 questions with gold answers written from the timeline (not by Haiku). Stored in the Brain repo under .tasks/<date>-patch-quality/probes.tsv.
- Runner: `brain-job answer-probe` style, through /v1/answer with the discord bot.public contract, one run before P1 and one after each package. Opus-high grader with a fixed rubric: numbers correct, dates correct, order correct, consequence plausible and grounded, citations present, no invention. Report the table per package.
- Regression guard: add the 60 probes to the nightly answer-audit so quality cannot silently drop again.

## Rules for the executing agents
Rust only for production code, no code comments, no .unwrap() in production paths, persistence in Postgres with new migrations only, AI calls through the central provider, no new model names. Opus 5.5 medium for implementation, Opus 5.5 high only for the synthesis writers and the grader. Fresh worktree per package under ~/.worktrees/, gate-review against origin/main, integrator merges, brain-release build/verify/install for brain-serve and brain-job, live check through /v1/answer with the three baseline questions (Mid-Boss and Rejuvenator in the Halloween patch, Shadow Weave rework, lifesteal and Healbane changes) plus the probe set. Report only merges, live state, blockers and the measured probe numbers.
