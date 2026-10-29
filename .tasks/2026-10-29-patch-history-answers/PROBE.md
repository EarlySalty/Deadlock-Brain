# Patch-history answer probe, run 1 (2026-10-29)

44 probes (49 turns) against brain-serve `POST /v1/answer` on 127.0.0.1:8788, sent like the Discord bot does (`dl-bot` token, scope `bot.public`, `x-discord-read-access: disabled`). Follow-ups use the `{"query", "user_questions"}` envelope. Calls ran one after another with a 5 s pause between them. Brain state: main before the `fix/patch-history-answers` branch.

## Rerun

```
cd .tasks/2026-10-29-patch-history-answers
eval "$(/home/nathanael/Documents/Infisical/export_claude_secret.py --secret DISCORD_BRAIN_CLIENT_TOKEN)" && python3 -I run_probes.py probes.json /tmp/run2.jsonl; unset DISCORD_BRAIN_CLIENT_TOKEN
python3 -I enrich.py /tmp/run2.jsonl "-2h" > results-run2.jsonl
```

Pass a comma list of IDs as a third argument to run only some probes. `auto_value_hit` is a substring heuristic and gives false positives (O06, O12), so read every answer before grading it. `enrich.py` adds the journal `reasons` and the `ops.answer_trace` tool calls for each turn.

## Headline findings

- `patch_history` was called **0 times in 49 turns**. Item answers carry the history in `get_item`, so they pass. Mechanics, objectives, timers and abilities have no history path.
- `value_gate_rejected` fired in 26 turns. It turns "was it X before?" questions into silence (G16, O07, H02, H04), or into hedges like "den Wert finde ich nicht" even when the current value is correct.
- Follow-up turns call **no tools at all** and lose the topic in 5 of 5 cases.
- Mechanic/timer data is stale in places: the urn timer still says 10 min + 5 min, but the 2026-05-28 patch changed it to 12/18/24/30.

## Counts

| Verdict | Count |
|---|---|
| Pass | 24 |
| Partial (value right, framing wrong or hedged) | 4 |
| Fail | 16 |

| Fail category | Count | Probes |
|---|---|---|
| Follow-up topic lost | 5 | F01, F02, F03, F04, F05 |
| Value gate rejected (silent) | 4 | G16, O07, H02, H04 |
| Data missing / stale | 4 | O06, O08, O09, H05 |
| Wrong answer | 2 | G08, G13 |
| Patch history not retrieved | 1 | O12 |

All 16 failures also share the root cause "patch history not retrieved".

Latency over all 49 turns: median 25.0 s, p90 50.7 s, max 58.3 s.

## Results

| ID | Status | ms | Verdict | Category | Note |
|---|---|---|---|---|---|
| F01 | answered | 45252 | FAIL | follow-up topic lost | Reference case. Turn 1 correctly says 50/50. Turn 2 made no tool call, hit the value gate, talked about lane split shares and said "ob das früher anders war, kann ich nicht sagen". It never mentioned 40% (2025-11-21) or the return to 50% (2026-06-30). |
| F02 | answered | 22866 | FAIL | follow-up topic lost | Turn 2 "Hatten die nicht mal 5500?" was answered with item cost tiers. |
| F03 | answered | 57884 | FAIL | follow-up topic lost | Turn 1 is stale: "ab Minute 10, alle 5 Minuten". Turn 2 drifted to the mid boss respawn. |
| F04 | answered | 31307 | FAIL | follow-up topic lost | Turn 1 already named the fire rate removal. Turn 2 picked `upgrade_fire_rate_aura` and said it had no history. |
| F05 | insufficient_evidence | 20120 | FAIL | follow-up topic lost | Turn 2 "war das früher mehr?" returned invalid_envelope and no tools. Expected 116+1.16/min before 2026-06-30. |
| G01 | answered | 32353 | PASS | | 50% correct, no history given. |
| G02 | answered | 27959 | PASS | | |
| G03 | answered | 14788 | PASS | | |
| G04 | answered | 12551 | PASS | | |
| G05 | answered | 36872 | PASS | | Says "nur aktueller Stand, kein Patch-Vergleich". |
| G06 | answered | 24348 | PASS | | |
| G07 | answered | 17855 | PASS | | |
| G08 | answered | 17938 | FAIL | wrong answer | Denies any net worth effect on respawn. Patch 2025-12-29 added +6 to +22 s. |
| G09 | answered | 13148 | PASS | | |
| G10 | answered | 28039 | PARTIAL | | Finds 25.0, then hedges "lässt sich nicht eindeutig bestätigen, Stand ohne bestätigten Patch". |
| G11 | answered | 13022 | PASS | | |
| G12 | answered | 30639 | PARTIAL | | Gives 28.3% per player (= 85% total) and calls the 90% claim "zu hoch". Never says 85% efficiency. |
| G13 | answered | 22940 | FAIL | wrong answer | Confirms the stale "150 minimum" from the beginner guide. Since 2026-06-30 it is 50 + 5/min. |
| G14 | answered | 37374 | PARTIAL | | Says 80 now, "was 90, lowered 09-16-2026". patch_changes has no such 09-16 row (only 75 -> 90 on 2026-03-06). The history claim is unverified. |
| G15 | answered | 13566 | PASS | | |
| G16 | insufficient_evidence | 58335 | FAIL | value gate rejected | Six tool calls, then silence. Expected 80 since 2025-11-21. |
| O01 | answered | 38721 | PASS | | 13000, but says "deinen Wert finde ich nicht". |
| O02 | answered | 22289 | PASS | | |
| O03 | answered | 26198 | PASS | | 3675 = 3500 + 5%. |
| O04 | answered | 39187 | PASS | | 1375 = 1250 + 10%. |
| O05 | answered | 15492 | PASS | | 4000. |
| O06 | answered | 12922 | FAIL | data missing | Says all walkers have 6000 HP. Since 2026-03-06 it is 6000/9000/12000 (auto hit was a false positive). |
| O07 | insufficient_evidence | 44828 | FAIL | value gate rejected | Shrine HP 5000/10000 not delivered. |
| O08 | insufficient_evidence | 17307 | FAIL | data missing / stale | `get_timer` still holds 10 + 5 min, which contradicts the user's old value; model_insufficient. |
| O09 | answered | 49267 | FAIL | data missing | No deposit timers (5/10/15 s). Repeats the stale 600/300 s urn timer. |
| O10 | answered | 25047 | PARTIAL | | 180 s correct, but "die Angabe in Minuten kann ich so nicht bestätigen" instead of "no, 3 min". |
| O11 | answered | 33976 | PASS | | |
| O12 | answered | 40418 | FAIL | patch history not retrieved | "keine gesicherte Angabe" on crystals (3 since 2025-10-02). Auto hit was a false positive. |
| I01 | answered | 21852 | PASS | | With patch date. |
| I02 | answered | 37599 | PASS | | With patch date. |
| I03 | answered | 23069 | PASS | | Full history chain. |
| I04 | answered | 19329 | PASS | | |
| I05 | answered | 26668 | PASS | | |
| I06 | answered | 54678 | PASS | | Says 60 -> 65. |
| H01 | answered | 24107 | PASS | | No history. |
| H02 | insufficient_evidence | 56991 | FAIL | value gate rejected | Wraith heart heal 60 (2026-09-16) not delivered. |
| H03 | answered | 18807 | PASS | | 3 stacks. |
| H04 | insufficient_evidence | 50719 | FAIL | value gate rejected | Geist Life Drain 100% (2026-07-28) not delivered. |
| H05 | answered | 51244 | FAIL | data missing | Storm Cloud expand time 3.5 s not found. Ability data lacks it, and patch_history was not called. |

Full answers, tools and gate reasons for each turn are in `results-run1.jsonl`.
