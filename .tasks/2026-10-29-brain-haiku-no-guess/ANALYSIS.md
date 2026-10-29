# Brain on Haiku: why it guesses, cause classes

Model stays claude-haiku-5-5 (slot brain_answer). Isolated serve instance `brain-ng-haiku` on 18894, 75-item set from `.tasks/2026-10-29-brain-data-coverage`.

Classes:
- (a) data missing or wrong
- (b) fact returned but buried, ambiguous or needed arithmetic
- (c) retrieval or ranking missed it
- (d) should have been silence but Haiku guessed
- (i) infrastructure: timeout or revalidation drift, not the model

## Runs

| Run | Code | answered | insufficient | unavailable | pass (graded) | wrong facts |
|---|---|---|---|---|---|---|
| base | 45786eef | 65 | 8 | 2 | ~52 | 3 |
| p1 | + 9f0aca68 | 64 | 10 | 1 | ~55 | 1 |
| p2 | + ad775a98 | 70 | 4 | 1 | ~59 | 1 |
| p3 | + fed5095b | 71 | 3 | 1 | ~65 | 1 to 2 |

## Per item (non-passing in at least one run)

| Item | Symptom | Class | Cause | Fix |
|---|---|---|---|---|
| G06 G07 G08 G12 G13 O05 T14 | silent or missing dates | (c) | mechanic rows had no dated patch lines | 9f0aca68 attaches dated patch_note_change rows to every mechanic |
| G11 I06 H04 G03 O09 | correct answer discarded | (b)/(d) gate | Haiku repeated the premise number from the question, value gate rejected it | ad775a98 repair prompt names the numbers that came from the question |
| G09 F05 | missing 20 s from minute 35 | (b) | per-minute interval row only, patch rows trimmed by size cap | fed5095b wave schedule row, numeric patch lines ranked first |
| G15 | no Medic Trooper HP | (c) | patch line trimmed / excluded | fed5095b |
| G03 G10 | answer dropped at final validation | (i) | optional history read timed out (500 ms) on revalidation, result differed, "Veröffentlichte Spielwerte haben sich geändert" | 94e6f71a retry history read once |
| T10 T13 O09 O11 H02 | unavailable or silent | (i) | patch_history text search reads all current lines, hits 2000 ms statement timeout under parallel load | open: package patch-search-speed |
| F02 | wrong: answers Guardian 5500 for Base Guardian | (c) | follow-up resolves objective to guardian, ignores "Base Guardian" in history | open: package follow-up-resolution |
| F04 | asks which item | (c) | follow-up searches "Fire Rate" instead of Echo Shard from history | open: package follow-up-resolution |
| O09 | no deposit timer | (c) | deposit line 2026-05-26 ranks past the 12-line cap; search used "delivery" | open: package mechanic-history-relevance |
| T16 | parry partial | (c) | core_abilities history lines capped, trooper-stun line missing | open: package mechanic-history-relevance |
| H04 | no heal share | (a)/(c) | 100 % damage-to-heal only in patch notes, get_ability has no dated lines | open: package ability-history |
| H05 | no expansion time | (a)/(c) | 3.5 s only in patch notes | open: package ability-history |
| O12 | crystal count | (a) | only in patch notes; passes since p1 via history | 9f0aca68 |
| T03 | Shiv HP not fetched | (c) | find_entity kind=item error, model gave up before get_hero; passes in p3 | variance |
| F03 O08 | urn 10/15/20 | none | reference is stale: Minor Update 06-30-2026 restored 10/15/20/25 | none, data correct |
| T15 T18 | Walker bullet resist 35 vs reference 25 | none | game files say 35 (npc_boss_tier2 intrinsic modifier), patch line says 15 to 25 | none, data is current source |
| T05 T09 | silent | none | reference expects silence or clarification | none |
