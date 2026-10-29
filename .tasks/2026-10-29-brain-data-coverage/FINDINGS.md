# Brain data coverage findings (2026-10-29)

Rule applied to every field: a value reaches answers only when its meaning is confirmed by patch notes, the docs corpus or an unambiguous number with a clear unit. Field and modifier names alone are not evidence.

## Fixed systematically

- Objective NPCs (Guardian, Walker, Patron, Base Guardian, Shrine) now carry HP steps, bullet resist, nearby-enemy resist, ally aura radius and resists, and backdoor resist with its per-hero reduction. Each row has a German meaning with an explicit sign ("nimmt so viel Prozent weniger Schaden").
- The Guardian/Walker rows `PlayerDamageResistPct` -25 and `TrooperDamageResistPct` 42.5 (trace 1357) are no longer extracted.
- New mechanic `core_abilities`: parry stun 2.75 s, +25 % damage taken, cooldown 4.5 s, dash cost 1, down-dash 0.5, wall jump 0.5 with the first one free, wall slide speed 600. Parry rules for troopers, objectives, Guardian, debuff resistance and urn carrier come from patch notes.
- `get_mechanic` drops rows without a verified meaning for objectives and core abilities instead of listing raw keys.
- Super trooper bounty meaning (0.85 = 15 % less) and shrine-down damage scale (+60 %, patch 2026-03-06) corrected; the unverified `healthscale` meaning removed.

## Meaning unverified (kept out of answers)

| Field | Value in data | Why excluded |
|---|---|---|
| `PlayerDamageResistPct`, `TrooperDamageResistPct` (Guardian, Walker) | -25 / 42.5 | Sign and target unclear, no patch note |
| Trooper `VS*` resists | various | No patch note on direction |
| Walker time-decaying resist field | | Decay meaning not documented |
| `BackdoorProtection` | 65 / 85 | Overlaps with the patch-confirmed backdoor resist, relation unknown |
| `HealthPerSecondRegen`, `BackDoorProtectionRange` | | No unit confirmed |
| `RangedArmorModifier` (`RangeMin` 30.48) | | Unclear what is reduced |
| `ParryBossVictimCalm`, `ParryBossVictimNoMelee` | | Names only; must not be read as "boss calm" or "melee disabled" |
| `ParryMoveSpeed` | 50 | Unit and owner unknown |
| Slide cooldown, dash duration, zipline values | 0.85 / 0.6 / | No confirmed unit or meaning |
| Mantle and jump fields | | Not documented |
| Patron growth and regen | | Not documented |
| `PlayerDPS`, `TrooperDPS` (objectives) | | Damage model behind it unknown |
| Super trooper `HealthScale` | 1.4 | Not confirmed by a patch note |
| Hero light and heavy melee damage | | Scaling rules not confirmed |
| Raw rows of other mechanics without a MEANINGS entry | | Still listed by key in `get_mechanic`; next sweep should cover them |

## Stale data

- Urn timer: deadlock-data says 600 s / 300 s, patch 2026-05-28 says spawns at 12/18/24/30 min. Not used as a probe expectation. Needs a patch-ledger override or an upstream fix.

## Probe expectations to watch

- D27 (dash cost "1") and D40 are weak because "1" appears in many answers; read the answer text.
- M02 (Patron) expects the match-end explanation; Patron soul bounty was removed on 2024-10-24.

## Tool result cap

- Unfiltered `get_mechanic objectives` returned 56 verified rows, about 21.6 KB, above the 16 KB tool result cap. `trim_row` dropped the last rows, which were the Walker and Guardian field rows, so W01 failed live on 0b3914bc although the data was correct. Fix: optional `objective` filter (guardian, walker, patron, base_guardian, shrine); each objective now fits with all of its rows (guard test `every_live_objective_fits_one_tool_result_with_all_its_rows`). An unfiltered call is still trimmed; the result stays marked `truncated` only for MAX_ROWS, not for byte trimming.

## Open review NITs

- Parry rules from patch notes are hardcoded in the `get_mechanic` tool description instead of coming from the patch ledger.
- "stamina" and "Ausdauer" collide with the item stat of the same name in search.
- `compare.py` compares only probe ids present in both runs; dropped probes are not reported.
- `verified_only` drops unverified rows without a count, so the model cannot see how many were filtered.
