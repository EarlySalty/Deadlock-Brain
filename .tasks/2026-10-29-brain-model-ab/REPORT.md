# Brain answer model A/B: Haiku 5.5 vs Opus 5.5 vs Sonnet 5.5

Status: PARTIAL. 25 of 75 items graded. Rerun of the 50 items lost to the plan usage limit is in progress (run2.jsonl).

## Partial result (F01-F05, G01-G16, O01-O04)

| Model | Pass | Partial | Fail | Silent | Wrong facts | Median latency | p90 latency |
|---|---|---|---|---|---|---|---|
| Haiku 5.5 | 12 | 4 | 5 | 4 | 4 | 22.1 s | 32.4 s |
| Opus 5.5 | 22 | 1 | 1 | 1 | 1 | 17.9 s | 28.3 s |
| Sonnet 5.5 | 17 | 2 | 2 | 4 | 2 | 23.4 s | 31.1 s |

## Run notes

- run1 started 2026-10-29 09:35 local. From O05 onward (item 26) every call for all three models returned `unavailable`; the gateway logged `kind="limit" reason="usage_limit"` on claude_plan_ab. Those 50 items are rerun in run2 starting 12:32.
- Live `brain_answer` stays on Haiku; A/B traffic runs on temporary slots `brain_answer_ab_*` and three separate brain-serve instances.

## How to continue

1. Wait for run2.jsonl to reach 50 rows.
2. `python3 -I ab_dump.py run2.jsonl <ids>` and grade into grades.json.
3. `python3 -I ab_metrics.py run1.jsonl "2026-10-29 09:30"` and the same for run2 for rounds and tokens.
4. `python3 -I ab_summary.py grades.json run1.jsonl run2.jsonl`.
5. Stop brain-ab-* units, restore gateway config from `~/.local/state/brain-model-ab/dl-ai-gateway.toml.orig`, restart dl-ai-gateway.
