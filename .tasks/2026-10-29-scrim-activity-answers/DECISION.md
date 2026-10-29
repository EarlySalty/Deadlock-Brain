# Scrim activity answers: blocked on a product decision

Question (operator DM): "Was ist so in der letzten Zeit bei den Scrims von uns und den Teams passiert"

## Existing paths

- Brain `ServerKnowledge` tool (`brain-serve/src/discord_live_tools.rs`) calls dl-bot `/mcp/public` `public_server_facts` with the requester's Discord permissions. Gated by `allow_discord_reads`.
- dl-bot sets `allow_discord_reads = is_public` (`dl-bot/src/modglue.rs`, `answer_discord_event`). DMs and private channels get `false`.
- That is decision K (2026-10-08, `.tasks/2026-10-08-brain-private-read/AUFTRAG.md`): private channel, thread and DM requests must not read Discord messages of other people, including tool and evidence paths. Only own question, game/server/docs knowledge and own invite status.
- Requester-bound precedent: `self_invite_status` (dl-bot `mcp/self_invite.rs`, Brain `brain_contracts::invite`). Exact-phrase gated, single fact.

## Data, measured 2026-10-29

- `scrim.teams`: 4 teams (DB names "Team 1".."Team 4"; Discord channels support-shiv, urn-runners, return-fire-department, tetrahex), roles 1521163175318388847 / ...233245794465 / ...300480483498 / ...334211338391.
- `scrim.matches`: 2 rows, both `planned` for "Sa 04.07.2026", never updated. `selected_match_results`: no winners. `match_result_refs`: 0. `match_requests`: 0.
- `scrim.lagebild_snapshots`: last `ok` 2026-10-02 (teams 1, 4) and 2026-10-15 (teams 2, 3, 175 chars, empty). Every run since is `error / ai_generation_failed` (Fireworks HTTP 412, provider billing; not changed). Lagebild is shown only in the staff dashboard.
- Real recent activity exists only as human posts: `#announcement-scrims` (schedules, winners, match IDs; last post 2026-09-13) and `#scrim-channel` (discussion, last 2026-10-26). Both deny @everyone VIEW; visible to the scrim role 1520849762851618817, team roles and staff roles.
- Operator (662995601738170389) has the scrim role and staff roles, but none of the 4 team roles and no `scrim.participants` row. "uns" does not resolve to a team under "own-team details only for members of that team".

## Why it stops

Any answer with real recent scrim facts must read human messages in role-gated channels from a DM. That reverses decision K. The structured DB alone yields only two stale July matches and a Lagebild from 2026-10-02 that says no dates or results exist.

## Options

A. Structured only (no decision change). New requester-bound dl-bot tool `scrim_activity` over `scrim.*` plus own-team latest ok Lagebild, allowed in DMs. Honest but thin: today it returns "no recorded scrims since July", and nothing for the operator (no team).
B. Announcement exception. Same tool also parses `#announcement-scrims` posts into team/winner/match-id/date facts without author, only if the requester can VIEW that channel. Narrow exception to K for one curated results channel. Gives real results (latest 2026-09-13).
C. Lift K for requester-visible channels in DMs (`allow_discord_reads = true` for DMs, reads stay limited to channels the requester can see). Broadest, would also cover `#scrim-channel` and own team channels; DM answer is private to the requester, but it reverses the 2026-10-08 decision.
D. Fix the source: record schedules and results in `scrim.matches` (scrim join flow / result entry), then A becomes sufficient.

Also needed for any option: how a staff member without a team role ("uns") should be treated (no own team vs. caster/organizer view of all teams).

Recommendation: B now plus D as the lasting fix; treat staff without team role as having no own team.
