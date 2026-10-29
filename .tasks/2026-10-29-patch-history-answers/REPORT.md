# Report 2026-10-29

## Deployed

- 73c0fced: patch_note extraction (v6), patch_history text search, DM follow-up context, value gate for patch dates and split complements, no retry on repeated rejections.
- 8e94654d: search terms keep two-digit numbers (60/40) and match singular/plural, `brain_patch_history` journal event with call shape only (no text).
- Both installed via ops/brain-release; brain-serve active, `/opt/deadlock-brain/current` at 8e94654d.
- First brain-job@deadlock_patchnotes_db run after deploy (09:10) wrote 515 `patch_note_*` facts, including 11-21-2025 "Amount of souls in the flying orb ... reduced from 50% to 40%" and 06-30-2026 "Trooper bounty ratio in the deniable flying orb increased from 40% to 50%".
- SEARCH latency live: 0.9 to 1.2 s per call.

## Live replay

The prior two turns are reconstructions of the incident, not the original messages.

| Question | Before | After |
| --- | --- | --- |
| Wie viele Seelen gibt ein Trooper in Minute 0/1? | answered | answered, 100 / 102, 13 s |
| Und wie viel davon landet im Orb? | answered | answered, 50 %, 16 to 19 s |
| War das nicht mal 60/40? (follow-up) | insufficient_evidence after about 55 s, silent | answered in 28 to 55 s, but incomplete |

Follow-up answers after deploy correctly state the current 50/50 split, the 11-21-2025 rework and the 11-21-2024 kill/orb change from 60/40 to 65/35.
None of five runs mentions the 40 % orb share from 21.11.2025 to 30.06.2026. One run misattributed the 60 % to soul sharing.

## Acceptance

Not met. Timeout and silence are fixed. The answer content is still incomplete.

Cause found via the new journal event: the model calls patch_history with three text terms, which matches the example in the tool description (trooper orb souls).
For those terms both 40 % lines are in the 20 returned rows (ranks about 13 to 16), behind long multi-change lines with more term hits. The model does not use them.
Each answer also re-runs the search during revalidation (up to 11 calls of about 1 s per answer).

## Open

1. Ranking or presentation so the two orb-ratio lines lead the result, for example fewer and shorter rows or a chronological layout for change lines with old and new values. Note facts currently carry no old/new values for these lines.
2. Alternatively, expose the orb-ratio history directly in get_mechanic lane_troopers, which the model always calls for this topic.
3. Revalidation repeats the full SEARCH; caching per request would cut about 10 s.
4. Gate NITs from round 3: short German date labels, schema allows text with entity, rejected_numbers reset after tool rounds, silent note drop on row limit, invented split notation for single percentages.
