# Patch completeness and ingest list (2026-10-29, HOLD scope)

Sources compared: raw `deadlock_patchnotes_db` (137 posts, ids 1-297, 81-240 never issued), legacy `brain.patch_events` (240 patch groups, frozen at 2026-10-09), official forum changelog (95 threads, 67 crawled before nginx rate block), Steam announcements.

## Findings
- No public patch notes before 2024-05-03 (forum thread 427 is the first). Pre-release builds had no public notes.
- Nothing official after 2026-10-20. Id 297 is a lore post.
- 84 of 137 raw posts have whitespace-collapsed bodies (one line, items joined by " - " and "[ Section ]"). Line structure must be rebuilt before line coverage can be checked.
- About 2,050 legacy lines are subject-less continuations ("Reduced from 300 to 250") that need the parent header or previous line as context.

## Ingest: present in raw, missing from legacy
| raw id | date | lines | title |
|---|---|---|---|
| 294 | 2026-10-14 | 27 | Minor Update - 10-14-2026 |
| 295 | 2026-10-15 | 27 | 09-29-2026 (forum mirror of 294) |
| 297 | 2026-10-20 | 9 | Nurse Harrow lore post: hero-added event only |
| 288, 293, 296 | Oct 2026 | 2 each | announcements, no change lines |
| 282 | 2026-07-30 | | Matchmaking Update |

## Ingest: present only in legacy patch_events (raw has a stub or nothing)
| date | legacy lines | title | raw state |
|---|---|---|---|
| 2025-02-25 | 581 | Map Rework Update (Steam) | stub (359 chars) |
| 2025-05-08 | 999+ | Shop Rework Update (Steam) | stub (174 chars) |
| 2024-11-07 | 398 | Steam copy | forum full text present |
| 2024-12-06 | 419 | Steam copy | forum full text present |
| 2025-11-21 | 357 | Steam copy | forum + 5 hotfix posts present |
| 2026-05-25 | 22 | Minor Update - 05-25-2026 | missing |
| 2024-10 to 2025-02 | 3-40 each | Hero Lab changelogs (10 groups, source `other`) | missing |
| 2025-07/08/10 | | forum hotfix posts 139997, 141214, 145970, 149166, 149696, 159495, 160063 | missing |
| 2025-01 to 2026-02 | 1-7 each | hero release announcements (Victor, Paige, Billy, Apollo, Rem, Graves, Silver, Venator, Celeste, Doorman, Drifter, six new heroes) | missing; become hero-added events |

## Steam vs forum duplicates
2026 Steam "Minor/Gameplay Update" posts mirror forum threads (03-06, 04-30, 05-22, 05-28, 05-31, 06-04, 06-11, 06-30, 07-01, 07-09, 07-28, 08-12, 08-22, 09-16). One canonical patch per date; the second copy only fills lines the first lacks.

## Not verified
28 forum threads (Sep to Dec 2024, 10-02-2025, 10-24-2025, 11-21-2025, 12-16-2025, 12-29-2025, 09-29-2026) are still rate-blocked. Their first posts are in raw; only extra hotfix replies could be missing. Retry with Moli later.
