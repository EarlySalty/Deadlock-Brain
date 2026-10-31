# Patch knowledge in the Deadlock Brain: assessment and plan (2026-10-31)

## Question
Is the current setup fit to carry all patch data from 2024 to 2026 as high-quality knowledge, and how do we get today's Halloween 2026 + Gameplay Update into the Brain with gameplay and interaction understanding?

## Measured state (brain DB, 2026-10-31)
- patchnotes.changelog_posts (central DB): 150 posts, 2024-05-03 to 2026-10-30.
- ops.patch_ledger: 134 patches (2024: 57, 2025: 40, 2026: 37), kinds major 17, gameplay 50, hotfix 67. ops.patch_ledger_post: 134 primary, 10 duplicate, 6 excluded (October 2026 hero reveal posts without date in title and under 20 change lines).
- canon.patch_line: 11,243 lines, every ledger patch has lines. Unresolved: general 3,028 of 3,028, objective 548 of 548, item 42, hero 6, ability 0. Structured old/new on 3,935 lines (35 percent).
- serve.patch_line_text: 20,257 rows. list_patches, patch_history and get_mechanic read these legacy fact lines by regex; canon.patch_line and serve.patch_change_v2 are not read by serving.
- 1,469 tie-break corrections from the Haiku/Opus relink run exist under .tasks/2026-10-29-brain-patch-ledger/import/ and were not applied.
- Worktree ~/.worktrees/brain-objective-entities: uncommitted objective entity implementation (migration v58, objectives.json, objectives.rs, 11 files, 361 lines).
- Docs channel: Deadlock-Docs public/*.html, hourly brain-job@deadlock-docs, search_docs with 1,500 chars per section. Public pages are served to members without an audit entry (helden-tipps.html has 2 sections in serve.doc_section and no berichte entry); the audit entry only gates the website export.

## Verdict
The import is complete; nothing from 2024 to 2026 needs re-importing. Quality fails on three layers: objective and general lines have no entity, serving does not read the resolved canon table, and there is no synthesis layer. Haiku interprets single raw lines at query time and cannot supply game understanding by itself.

## Done today
- Hotfix fix/midboss-regex: `mid ?boss` became `mid[ -]?boss` in brain-serve mechanic_history.rs (include and Respawn exclude) plus `drop[ -]?off channel` for Urn. Merged as 1a273fc6, released via brain-release build/verify/install, brain-serve restarted 01:27 CET. Live check: "Wie hat sich der Mid-Boss über die letzten Patches verändert?" now answers with the dated history (2024-06-06, 2024-07-23, 2025-11-21, 2026-10-30) instead of failing.
- Halloween digest: Deadlock-Docs public/deadlock-patches/halloween-2026-gameplay-update.html, 41 sections under 1,400 chars, every number from the official patch text, interpretations marked Einschätzung, claims file next to it. Written by Opus 5.5 at high effort, gated, merged to main (see Deadlock-Docs history).

## Plan for next week
See NEXT-WEEK-PROMPT.md in this folder. Order: P1 entities and resolution, P2 entity timeline as single serving source, P3 Opus-high synthesis (entity dossiers, patch digests, game model page), P4 Haiku prompt and tool strategy, P5 probe set and nightly regression guard.
