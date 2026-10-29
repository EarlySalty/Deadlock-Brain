# Patch ledger inventory, 2026-10-29

Sources checked: `patchnotes.changelog_posts` (central), canon patch entities in the Brain DB, the full forum crawl of the official patch threads (95 threads, 131 Yoshi posts, via Moli), Steam news for app 1422450, and the legacy `brain.patch_changes` view.

## Before and after

| | before | after |
|---|---|---|
| source posts (`patchnotes.changelog_posts`) | 137 (max id 297) | 149 (max id 309) |
| canon patch entities | 137 | 149 (ingest run 4101 accepted 12) |
| ledger patches | none | 134: 16 major, 50 gameplay, 68 hotfix |
| duplicate posts | 9, unflagged, shown as separate patches | 9, flagged `duplicate`, hidden in patch tools |
| non-patch posts | 6, shown as patches | 6, flagged `excluded` |
| `list_patches` | missing | ledger list, oldest to newest |
| `patch_history` order | newest first | oldest to newest, rows carry `ledger_id` and `ledger_released_at` |

First patch: ledger 80, "05-03-2024 Update", 2024-05-03T20:02:54Z. Last patch: ledger 294, "Minor Update - 10-14-2026", 2026-10-15T03:11:58Z.

## Missing patches (imported)

All 12 were imported through the Patchnotes bot `reconcile` path, additively, with no Discord dispatch or translation queue. Brain ingest then picked them up through the normal brain-job path.

| id | title | released (UTC) | source |
|---|---|---|---|
| 298 | Map Rework Update | 2025-02-25 21:49 | Steam news |
| 299 | Shop Rework Update | 2025-05-08 19:33 | Steam news |
| 300 | 08-01-2025 Update | 2025-08-01 18:48 | forum post 139997 |
| 301 | 08-08-2025 Update | 2025-08-08 19:07 | forum post 141214 |
| 302 | Six New Heroes | 2025-08-18 20:42 | Steam news |
| 303 | 08-22-2025 Update | 2025-08-23 02:02 | forum post 145970 |
| 304 | 08-28-2025 Update | 2025-08-29 00:49 | forum post 149166 |
| 305 | 08-29-2025 Update | 2025-08-30 00:36 | forum post 149696 |
| 306 | 10-03-2025 Update | 2025-10-03 20:56 | forum post 159495 |
| 307 | 10-05-2025 Update | 2025-10-05 18:05 | forum post 160063 |
| 308 | Minor Update - 05-25-2026 | 2026-05-28 16:28 | Steam news |
| 309 | 09-30-2026 Update | 2026-09-30 22:31 | forum post 316496 |

Every other forum post and Steam patch announcement was already in the source.

## Duplicates

Each pair is one patch published twice (a forum post and its Steam mirror, or a re-post). The ledger keeps one row. The Steam post is primary, the release date is the earliest post, and the other post is marked `duplicate`.

| ledger id | label | duplicate post |
|---|---|---|
| 26 | 12-17-2024 Update | 270 |
| 298 | Map Rework Update | 18 |
| 299 | Shop Rework Update | 12 |
| 302 | Six New Heroes | 4 |
| 259 | Gameplay Update - 03-06-2026 | 260 |
| 269 | Gameplay Update - 04-30-2026 | 268 |
| 287 | City Never Sleeps | 286 |
| 289 | Minor Update - 10-05-2026 | 291 |
| 294 | Minor Update - 10-14-2026 | 295 |

## Excluded posts (not patches)

These six posts are event or hero teasers with no balance notes, and they have no https patch source. They stay in canon with disposition `excluded` and are not ledger patches:

| post | title |
|---|---|
| 288 | Listen up, Crumbums! Your King… |
| 290 | Mind the Birds! |
| 292 | Make Your Move. |
| 293 | Fairfax Industries Employee Sp… |
| 296 | Paint the Town |
| 297 | Shh... This Won't Hurt a Bit. |

## Wrong dates

- **Source dates.** All 136 pre-import source posts that have an independent timestamp matched the forum or Steam publish time to the second. No source date needed a fix.
- **Duplicate pairs.** Here the date error was structural: the same patch appeared under two dates. The ledger now uses the earliest post. In some pairs a forum mirror was posted hours after the Steam announcement; in others the Steam post came after the forum post.
- **Legacy `brain.patch_events`.** These rows carry midnight dates such as `2024-05-03 00:00`, title dates instead of publish times, and akamai mirror URLs. One case: legacy "06-11-2026 Update" is dated 06-11 while the patch went out 2026-06-12 UTC. That data is legacy only. The Brain patch tools read canon plus the ledger, never `brain.patch_events`.
- **Post 308.** It is titled "Minor Update - 05-25-2026" but Steam published it 2026-05-28 16:28 UTC. The ledger uses the publish time, as for every other post.

## patch_changes parent linkage

**Canonical (`serve.p4_patch_change`, used by `patch_history`).**
- 2025 change rows across 43 patch posts, and every row has a parent ledger post.
- 1985 rows sit on 41 primary posts.
- 40 rows sit on duplicate posts 291 and 295. Their primaries 289 and 294 hold the same 20 changes each, so these rows are hidden as duplicates.
- 0 rows sit on excluded posts, and 0 are orphans.

**Legacy `brain.patch_changes`: 20461 rows, 59 title/date groups.**

| link method | rows |
|---|---|
| source URL | 14260 |
| exact title | 5368 |
| title date or patch date | 682 |
| total linked to a ledger patch | 20310 |

151 rows have no ledger parent, for these reasons:

| rows | reason |
|---|---|
| 66 | Hero Lab sub-notes (11-08/11-24/11-26-2024, 02-20-2025). These are experimental-mode notes inside a patch thread, not separate patches. |
| 23 | Hero reveal teasers ("Drifter Emerges…", "Old Gods, New Blood", "Rem Enters…", "Graves Prefers The Dead", "The Curse Beckons for Silver", "Introducing The Dazzling Celeste", "Apollo - A Cut Above", "The Doorman, at your Service"). These are announcements, not patches. |
| 4 | Lines parsed from excluded teaser posts 290 and 292. |
| 87 | "06-11-2026 Update", dated 06-11 in legacy. It is the same Steam post as ledger "Minor Update - 06-11-2026" (released 2026-06-12 UTC, same Steam id 1835236783562074), so it is a legacy duplicate and has a parent through the URL. |

The legacy view also double-counts six patches under two titles with the same URL: 05-08-2025/Shop Rework, 08-18-2025/Six New Heroes, 06-04, 06-11, 06-30 and 07-01-2026. It also holds a one-row junk entry titled "t". The legacy view is not used by brain-serve.

## Date spot checks (22 patches)

Ledger date compared with an independent timestamp: the forum post time from the crawl, or the Steam news time. For new imports, the source is the crawled post itself.

| ledger id | label | kind | ledger released_at | independent timestamp | source |
|---|---|---|---|---|---|
| 80 | 05-03-2024 Update | gameplay | 2024-05-03T20:02:54Z | 2024-05-03T13:02:54-07:00 | forum |
| 75 | 05-23-2024 Update | gameplay | 2024-05-23T21:39:49Z | 2024-05-23T14:39:49-07:00 | forum |
| 65 | 06-23-2024 Update | hotfix | 2024-06-23T23:23:36Z | 2024-06-23T16:23:36-07:00 | forum |
| 55 | 08-18-2024 Update | hotfix | 2024-08-19T05:46:34Z | 2024-08-18T22:46:34-07:00 | forum |
| 45 | 09-27-2024 Update | hotfix | 2024-09-28T00:33:18Z | 2024-09-27T17:33:18-07:00 | forum |
| 35 | 10-27-2024 Update | hotfix | 2024-10-28T01:43:19Z | 2024-10-27T18:43:19-07:00 | forum |
| **29** | **11-21-2024 Update (orb)** | major | 2024-11-21T23:21:49Z | 2024-11-21T15:21:49-08:00 | forum |
| 25 | 12-21-2024 Update | gameplay | 2024-12-21T09:54:51Z | 2024-12-21T01:54:51-08:00 | forum |
| 298 | Map Rework Update | major | 2025-02-25T21:49:19Z | Steam news 530965072572320687 | steam |
| 15 | 03-19-2025 Update | hotfix | 2025-03-20T02:36:14Z | 2025-03-19T19:36:14-07:00 | forum |
| 5 | 07-29-2025 Update | gameplay | 2025-07-29T22:22:52Z | 2025-07-29T15:22:52-07:00 | forum |
| 300 | 08-01-2025 Update | hotfix | 2025-08-01T18:48:14Z | forum post 139997 | forum |
| 302 | Six New Heroes | gameplay | 2025-08-18T20:42:20Z | Steam news 669466707009471267 | steam |
| 307 | 10-05-2025 Update | hotfix | 2025-10-05T18:05:47Z | forum post 160063 | forum |
| **241** | **11-21-2025 Update (orb)** | major | 2025-11-21T22:53:12Z | 2025-11-21T14:53:12-08:00 | forum |
| 249 | 12-29-2025 Update | gameplay | 2025-12-30T06:33:52Z | 2025-12-29T22:33:52-08:00 | forum |
| 259 | Gameplay Update - 03-06-2026 | major | 2026-03-06T21:37:00Z | 2026-03-06T21:37:00Z | steam |
| 272 | Gameplay Update - 05-22-2026 | major | 2026-05-22T21:51:02Z | 2026-05-22T21:51:02Z | steam |
| **278** | **Minor Update - 06-30-2026 (orb)** | gameplay | 2026-06-30T17:22:14Z | 2026-06-30T17:22:14Z | steam |
| 281 | Minor Update - 07-28-2026 | gameplay | 2026-07-28T20:24:35Z | 2026-07-28T20:24:35Z | steam |
| 309 | 09-30-2026 Update | hotfix | 2026-09-30T22:31:50Z | forum post 316496 | forum |
| 294 | Minor Update - 10-14-2026 | gameplay | 2026-10-15T03:11:58Z | 2026-10-15T03:11:58Z | steam |

All 22 match. Every other pre-import post (136 in total) was compared the same way, with 0 mismatches.
