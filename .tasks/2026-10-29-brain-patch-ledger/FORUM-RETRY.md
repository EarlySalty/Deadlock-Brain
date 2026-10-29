# Forum retry: 28 rate-blocked patch threads

Date: 2026-10-29. Closes the "Not verified" gap in `.tasks/2026-10-29-patch-ledger/INGEST.md`.

## Crawl status

- The earlier crawl state (`~/.local/state/brain-patch-ledger/scratch/forum_posts.json`, retry log `crawl_retry.log`) already holds all 95 non-meta threads from `forum_threads.txt`. The 28 previously blocked threads were fetched in a retry at 22:30, which confirms the INVENTORY claim of a full 95-thread crawl.
- To check that retry on its own, all 28 threads were fetched again with Moli (`moli fetch --block-private-networks`), 15 s between requests, with backoff from 20 s on empty or failed pages. None were blocked, and no thread has more than one page. Every post count matches the earlier retry log.
- All posts in these threads are by Yoshi. There are no non-developer replies.
- Matching against `patchnotes.changelog_posts` (149 rows, max id 309): first by forum post id in the source url, then by thread url for the first post, then by exact UTC publish second.

## Result

| thread | posts found | in source (forum post id -> source id) | missing | date (UTC, forum timestamp) |
|---|---|---|---|---|
| `threads/09-27-2024-update.33816/` | 1 | 60608->45 | - | 2024-09-28 00:33:18 |
| `threads/09-29-2024-update.34541/` | 1 | 61923->44 | - | 2024-09-29 19:38:55 |
| `threads/09-29-2026.166726/` | 4 | 316496->309, 322289->291, 330973->295 | 316411 (Steam link mirror, content = source 287, see below) | 2026-09-30 21:34:38<br>2026-09-30 22:31:50<br>2026-10-05 23:00:12<br>2026-10-15 03:12:00 |
| `threads/10-02-2024-update.35439/` | 1 | 63608->43 | - | 2024-10-03 00:18:25 |
| `threads/10-02-2025-update.84332/` | 3 | 159109->2, 159495->306, 160063->307 | - | 2025-10-02 20:03:05<br>2025-10-03 20:56:48<br>2025-10-05 18:05:47 |
| `threads/10-07-2024-update.36441/` | 1 | 66320->42 | - | 2024-10-07 19:45:09 |
| `threads/10-10-2024-update.36958/` | 1 | 67710->41 | - | 2024-10-10 20:24:45 |
| `threads/10-11-2024-update.37641/` | 1 | 69195->40 | - | 2024-10-11 23:54:46 |
| `threads/10-15-2024-update.38925/` | 1 | 72096->39 | - | 2024-10-16 00:07:44 |
| `threads/10-18-2024-update-2.39693/` | 1 | 73947->37 | - | 2024-10-18 23:30:16 |
| `threads/10-18-2024-update.39630/` | 1 | 73808->38 | - | 2024-10-18 19:48:11 |
| `threads/10-24-2024-update.40951/` | 1 | 77465->36 | - | 2024-10-24 19:39:08 |
| `threads/10-24-2025-update.87198/` | 1 | 164694->1 | - | 2025-10-24 23:54:51 |
| `threads/10-27-2024-update.42492/` | 1 | 80343->35 | - | 2024-10-28 01:43:19 |
| `threads/10-29-2024-update.42985/` | 1 | 81437->34 | - | 2024-10-29 18:20:09 |
| `threads/11-01-2024-update.43705/` | 1 | 83212->33 | - | 2024-11-01 23:12:33 |
| `threads/11-07-2024-update.44786/` | 1 | 85843->32 | - | 2024-11-07 21:31:34 |
| `threads/11-10-2024-update.45689/` | 1 | 87538->31 | - | 2024-11-10 16:19:27 |
| `threads/11-13-2024-update.46391/` | 1 | 88971->30 | - | 2024-11-14 01:10:08 |
| `threads/11-21-2024-update.47476/` | 1 | 91397->29 | - | 2024-11-21 23:21:49 |
| `threads/11-21-2025-update.90383/` | 6 | 171245->241, 171913->243, 172057->244, 172338->242, 173347->245, 174987->246 | - | 2025-11-21 22:53:12<br>2025-11-23 05:46:02<br>2025-11-23 17:11:20<br>2025-11-24 05:34:45<br>2025-11-27 19:32:53<br>2025-12-04 00:34:27 |
| `threads/11-29-2024-update.49470/` | 1 | 95229->28 | - | 2024-11-29 17:59:16 |
| `threads/12-06-2024-update.50599/` | 1 | 97940->27 | - | 2024-12-06 20:05:10 |
| `threads/12-16-2025-update.93983/` | 2 | 177936->247, 178389->248 | - | 2025-12-16 18:33:38<br>2025-12-17 20:33:46 |
| `threads/12-17-2024-update.52008/` | 1 | 100680->26 | - | 2024-12-17 20:53:43 |
| `threads/12-21-2024-update.52206/` | 1 | 101159->25 | - | 2024-12-21 09:54:51 |
| `threads/12-29-2025-update.95233/` | 4 | 180519->249, 180763->250, 185824->251, 185965->252 | - | 2025-12-30 06:33:52<br>2025-12-31 06:38:21<br>2026-01-20 21:58:55<br>2026-01-21 02:10:58 |
| `threads/12-31-2024-update.52769/` | 1 | 102292->24 | - | 2024-12-31 08:52:42 |

Totals: 28 threads, 42 Yoshi posts, 41 matched to a source row by id, url or timestamp. The one unmatched post is listed below.

## Unmatched post

| url | timestamp (UTC) | thread title | content | verdict |
|---|---|---|---|---|
| https://forums.playdeadlock.com/posts/316411/ | 2026-09-30 21:34:38 | 09-29-2026 | Link card only: "Deadlock - City Never Sleeps - Steam News", no patch notes | Not a missing patch. It mirrors the Steam announcement already in source as 287 (primary) and 286 (duplicate), published 2026-09-29 20:25:11. Nothing to import. |

## Missing posts

None. No import through the Patchnotes bot reconcile path is needed.

Side note: source 291 and 295 are the forum mirrors of hotfixes 289 and 294. INVENTORY.md already marks them `duplicate`.

