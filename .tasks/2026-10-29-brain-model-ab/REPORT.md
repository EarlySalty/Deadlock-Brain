# Brain answer model A/B: Haiku 5.5 vs Opus 5.5 vs Sonnet 5.5

Date: 2026-10-29. Status: complete, 75 of 75 items graded for all three models. Report only; live `brain_answer` stayed on `claude-haiku-5-5` throughout.

## Setup

- Three isolated brain-serve instances (`brain-ab-haiku`, `brain-ab-opus`, `brain-ab-sonnet`, ports 18891 to 18893) on temporary gateway slots `brain_answer_ab_*` via a temporary `claude_plan_ab` provider. Live brain-serve on 8788 untouched.
- Question set (`questions.json`, 75 items): 44 probes (F, G, O, I, H series) plus 31 real member questions from the last 7 days of `answer_trace` (T series), including the three known failures T10 (60/40 three-turn conversation), T15 (Walkers and resists), T16 (Parry).
- All three models ran each item in parallel, items interleaved (`ab_run.py`).
- run1 (09:35): items 1 to 25 valid; from item 26 the plan usage limit hit (`kind="limit" reason="usage_limit"`), so items 26 to 75 were rerun in run2 (12:32). `run1-valid.jsonl` holds the 25 valid rows.
- Grading against game data and `brain.patch_changes` (dl-brain `patch_history`), in `grades.json`. Grades: pass, partial (correct but incomplete, or a correct decline of part of the question), fail (wrong or missing core fact), silent (`insufficient_evidence` or a brain-side `unavailable`).
- `unavailable` rows in run2 (Opus H04, Sonnet O07 and I06) are brain-side tool failures ("Werkzeugantwort konnte nicht sicher abgeschlossen werden"), not quota; graded as silent.

## Results per model

| Metric | Haiku 5.5 (live) | Opus 5.5 | Sonnet 5.5 |
|---|---|---|---|
| Pass | 37 (49%) | **59 (79%)** | 45 (60%) |
| Partial | 21 | 3 | 12 |
| Fail | 7 | 6 | **4** |
| Silent | 10 | **7** | 14 |
| Silent rate | 13.3% | **9.3%** | 18.7% |
| Wrong facts | 5 | 6 | **4** |
| Latency median | 23.3 s | **19.4 s** | 26.0 s |
| Latency p90 | 40.6 s | **39.1 s** | 47.7 s |
| Provider rounds mean (total) | 3.01 (226) | **2.68 (201)** | 2.83 (212) |
| Gate rejections (tool rounds rejected) | 29 | **12** | 25 |
| Input tokens | 5.82 M | **4.21 M** | 6.84 M |
| Output tokens | 222.7 k | **66.4 k** | 85.5 k |
| API-equivalent cost, 75 questions | **$0.69** | $18.17 | $14.54 |
| API-equivalent cost per question | **$0.009** | $0.242 | $0.194 |

Cost uses list prices per MTok (Haiku 5.5 $0.10/$0.50, Sonnet 5.5 $2/$10, Opus 5.5 $4/$20), no cache discount, so it is an upper bound. Plan quota use is not directly measurable: the gateway exposes no per-request quota meter. The only observation is that run1 exhausted the plan window after about 75 parallel calls (25 items x 3 models) on top of normal traffic, which Opus and Sonnet dominate by token volume.

## Known failures

| Item | Haiku | Opus | Sonnet |
|---|---|---|---|
| T10 60/40 three-turn | partial (no 2025-11-21 step) | pass, full chain 60/40, 65/35, 60/40, 50/50 | partial (no 2025-11-21 step) |
| T15 Walkers and resists | fail, still no resist values | pass | pass |
| T16 Parry specs | fail, only cooldown | pass (97 s latency) | partial |

## Side-by-side examples

**T10** "War das nicht mal 60/40? Dachte das wäre immer noch so" (after two trooper soul questions)
- Haiku: "In der Lanephase liegt das Verhältnis bei 50/50 ... Früher war es 60/40 ... Im Update vom 21.11.2024 wurde es auf 65/35 geändert. Wann genau von 65/35 auf 50/50 gewech..." Correct now-state, gap in history.
- Opus: "Bis zum 11-21-2024 Update war die Trooper-Belohnung 60/40 ..., danach 65/35. Mit dem 11-21-2025 Update wurde der Anteil im Seelenorb ... von 50% auf 40% gesenkt, also wieder 60/40. Seit dem Minor Update 06-30-2026 liegt der Orb-Anteil wieder bei 50%." Full and correct.
- Sonnet: correct 2024 and 2026 steps, skips the 2025-11-21 return to 60/40.

**G08** "Respawnt man länger, wenn man reicher ist als das gegnerische Team?" (truth: yes, +6 to +22 s, since 12-29-2025)
- Haiku: "hängt die Respawnzeit an der Spielzeit und nicht am Goldstand." Wrong.
- Opus: "Ja. Seit dem 12-29-2025 Update ... Die Strafe steigt von +6s auf +22s ... (15 bis 30% höher) ... (10min bis 25min)." Correct.
- Sonnet: "Einen Zuschlag für einen Souls-Vorsprung ... sehe ich nicht." Wrong.

**O07 / T24** Shrine HP (truth: 5000/10000 since 03-06-2026)
- Haiku: silent on O07, declines HP on T24.
- Opus: "7000" on both, the stale 12-16-2025 value. Opus's most repeated wrong fact.
- Sonnet: unavailable on O07, declines HP on T24.

## Error patterns

- Shared by all three: O06 claims all Walkers have 6000 HP (actual 6000/9000/12000 by lane stage); all three silent on T05 (Mystic Burst follow-up). These point to retrieval or data gaps, not the model.
- Opus wrong facts lean towards stale patch values (Shrine 7000, urn deposit 5 s; deposit is instant since 2026-06-30). It answers more, so it also states more outdated numbers. Haiku and Sonnet decline the same questions instead.
- Haiku's main weakness is partial answers: it finds the base value but not the patch history or the second half of a question (21 partials vs 3 for Opus).
- Sonnet has the highest silent rate and the slowest p90, so it is not a good middle ground.

## Stale probe expectations corrected during grading

- G14: Golden Goose Egg is 80/min since 2026-09-16.
- F03, O08: urn spawns at 10/15/20/25 again since the 2026-06-30 rework; deposit is instant.
- O01: growth 195 = 230 x 0.85.

## Recommendation

Opus 5.5 is clearly the best answer model: 79% pass vs 49% for Haiku, the lowest silent rate, the fewest provider rounds and gate rejections, and the fastest median latency. It also fixes all three known failures. It costs about 26x Haiku per question at list price ($0.24 vs $0.009), and its typical wrong fact is a stale patch value, which retrieval and freshness work should fix whatever model is used.

Suggested next step (needs approval, as it is a model and cost change): run Opus for `brain_answer` on the plan for real member traffic in shadow mode or for a limited window, with a quota watch, because 75 parallel A/B calls already hit the plan limit once. If quota or cost rules out Opus, keep Haiku. Sonnet is slower, more often silent and only slightly better than Haiku, so it does not justify a switch.

## Files

- `questions.json`, `ab_run.py`, `ab_metrics.py`, `ab_dump.py`, `ab_summary.py`
- `run1.jsonl`/`run1.log` (rows 1 to 25 valid, also as `run1-valid.jsonl`), `run2.jsonl`/`run2.log`/`run2.ids`/`run2.start`
- `run1-valid-enriched.jsonl`, `run2-enriched.jsonl` (rounds and tokens per row)
- `grades.json`

Temporary gateway slots and the provider were removed and dl-ai-gateway restarted (PID 322846 to 909974, 0 error lines, `brain_answer` model `claude-haiku-5-5`, live 8788 `/readyz` 200).
