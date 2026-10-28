# Game knowledge audit 2026-10-28

Trigger: "wie viele Souls gibt ne Trooper wave bei min 16" returned InsufficientEvidence (tool calls only find_entity and search_docs).

Method: 50 German member questions through `/tmp/docs-refresh/ask.sh` against live brain-serve (release 2bb225de, origin/main). Raw answers in `before.jsonl`. Ground truth from `brain.source_record_heads` (deadlock-wiki/deadlock-data `convars.json`, `generic-data.json`, `npc-data.json`, `soul-unlock-data.json`, `street-brawl-data.json`) and canon facts.

Result before: **9 of 50 correct**, 4 partial, 37 failed (21 insufficient_evidence, 2 unavailable, 14 answered but wrong or non-answers).

## Root cause classes

- **A, data in PG raw store, not projected to canon, no tool**: troopers, denizens, objectives (guardian, walker, base guardians, shrines, patron), mid boss, kill bounty, respawn ramps, share fractions, deny split, item tier prices, urn (crate_* convars), breakables, level and ability point table, street brawl. `canon.entity` only has hero, item, ability, timer, patch, doc_page, meta_stats, lineage.
- **B, value gate rejects derived numbers**: per-minute scaling, wave totals, split per player, respawn at minute X. Every number must appear in a cited tool row; nothing computes them.
- **C, data not in PG**: wave composition beyond squad size, map layout (lanes, teleporters), game mode list.
- **D, ability stat missing or not named**: afterburn DPS.
- **E, routing or entity resolution**: model asks back or matches items instead of the game concept.
- **F, tool round failed (unavailable)**: cause not visible from the session (no journal access).

## Matrix

| # | Question | Status | Verdict | Class | Ground truth |
|---|---|---|---|---|---|
| 1 | Souls Trooper wave min 16 | insufficient | fail | A+B | citadel_trooper_gold_reward 100 +2/min, squad_size 4 |
| 2 | Souls einzelner Trooper Anfang | insufficient | fail | A | citadel_trooper_gold_reward 100 (wiki table says 116 total, semantics unconfirmed) |
| 3 | Trooper pro Welle | insufficient | fail | A (+C) | citadel_trooper_squad_size 4; full composition not in PG |
| 4 | Prozent ohne Orb | insufficient | fail | A | instant_gold_ratio 0.5 laning/postlaning |
| 5 | Denien | insufficient | fail | A | deny_denier_percentage 1, denied 0 |
| 6 | Zu dritt auf Lane | answered | fail (no number) | A+B | TrooperKillGoldShareFrac 1.0/0.54/0.36 |
| 7 | Jungle camp souls | insufficient | fail | A+B | neutral weak 41, normal 68, strong 181, +1.08 %/min |
| 8 | Jungle spawn | insufficient | fail | A | neutral spawn convars (overrides -1), timer data partial |
| 9 | Souls Walker | answered | fail (no number) | A | Tier2GoldKill 3675 |
| 10 | HP Walker | answered | fail (asks back) | A+E | npc_boss_tier2 6000 |
| 11 | Guardian vs Walker | answered | fail (items) | A+E | tier1 5500 HP / tier2 6000 HP |
| 12 | Souls Guardian | answered | wrong (Guardian Ward 1600) | A+E | Tier1GoldKill 1375 |
| 13 | Base guardians fallen | insufficient | fail | A | BaseGuardiansGoldKill 1000 |
| 14 | Shrines | insufficient | fail | A | ShrinesGoldKill 2000 |
| 15 | HP Patron | answered | fail (abilities) | A+E | npc_boss_tier3 12000 |
| 16 | Mid boss spawn | answered | partial | B | timer mid_boss 420 |
| 17 | Rejuvenator gives | answered | partial (only timers) | A | TrooperHealthMult, PlayerRespawnMult, buff 180 s |
| 18 | Rejuvenator duration | insufficient | fail | E | timer rejuvenator 180 exists |
| 19 | Urn | insufficient | fail | A | citadel_idol_* |
| 20 | Urn first spawn | insufficient | fail | A (+C) | wiki 10 min; not in serve.timer |
| 21 | Respawn min 20 | answered | fail (soul box) | A+B | min_ramp_2 38 s / max_ramp_1 38 s at 1200 s |
| 22 | Respawn Anfang | insufficient | fail | A+B | min_ramp_1 8 s at 300 s |
| 23 | Souls Level 10 | insufficient | fail | A | soul-unlock-data RequiredSouls table |
| 24 | Ability points when | insufficient | fail | A | soul-unlock-data AbilityPoints rows |
| 25 | AP Upgrade cost | insufficient | fail | A (+C) | wiki 1/2/5; not in projected data |
| 26 | Tier 3 cost | answered | fail | A | ItemPricePerTier 3200 |
| 27 | Item tiers | insufficient | fail | A | 800/1600/3200/6400 |
| 28 | Headshot Booster | answered | correct | | |
| 29 | High Velocity Mag | answered | correct | | |
| 30 | Extra Stamina | answered | correct | | |
| 31 | Improved Cooldown | answered | correct | | |
| 32 | Boundless Spirit cost | answered | correct | | 6400 |
| 33 | Haze HP start | answered | partial (hedged) | B | |
| 34 | Lady Geist ult | answered | correct | | |
| 35 | Infernus afterburn dmg | answered | fail | D | |
| 36 | Seven ult upgrades | unavailable | fail | F | |
| 37 | Power increase | answered | fail | A | soul-unlock-data PowerIncrease rows |
| 38 | Bullet dmg per level | insufficient | fail | A+B | hero level scaling |
| 39 | Zipline speed | answered | fail (asks back) | E | Zipline Boost ability facts exist |
| 40 | Teleporter | answered | fail | C | map data |
| 41 | Lanes | answered | fail | C | map data |
| 42 | Game modes | answered | wrong (Discord LFG modes) | C | |
| 43 | Street brawl | unavailable | fail | A+F | street-brawl-data |
| 44 | Last patch | answered | correct | | |
| 45 | Haze nerf last patch | answered | correct | | |
| 46 | Sinners sacrifice | answered | partial | A | neutral_sinners_sacrifice 155 |
| 47 | Souls per kill | answered | fail | A+B | gold_reward_min 200 / max 2200 over 2400 s |
| 48 | Breakables / crates | insufficient | fail | A | BreakableSpawnTimeDesc and breakable reward data; crate_* convars are the Urn, not breakables |
| 49 | Mid boss respawn | answered | correct | | 420 s |
| 50 | Urn cash-in bonus | insufficient | fail | A | citadel_idol_* |

## After, release 02662955

Method: same 50 questions through `run.sh` against live brain-serve on release 02662955 after migration v48 and one `brain-job@deadlock-data` ingest (run 3106, 14 mechanics, 315 rule rows). The first parallel run hit provider HTTP 503 and an open provider circuit on 32 questions with no tool call; those 33 non-answered questions were asked again one at a time. Raw answers in `after.jsonl`.

Result after: **30 of 50 correct**, 8 partial, 12 failed (was 9 / 4 / 37).

| # | Question | Status | Verdict | Class | Note |
|---|---|---|---|---|---|
| 1 | Souls Trooper wave min 16 | answered | correct | | 528 = 4 x 132, 66 instant per trooper |
| 2 | Souls einzelner Trooper Anfang | answered | correct | | 100 +2/min (wiki 116 unresolved) |
| 3 | Trooper pro Welle | answered | correct | | squad size 4 |
| 4 | Prozent ohne Orb | answered | correct | | 0.5 |
| 5 | Denien | answered | fail | E | deny rows not used, lists trooper HP |
| 6 | Zu dritt auf Lane | answered | correct | | 36 % |
| 7 | Jungle camp souls | answered | partial | B | base 41/68/181, skips per-minute growth |
| 8 | Jungle spawn | answered | correct | | 120/85, 300/290, 480/335 s |
| 9 | Souls Walker | answered | correct | | 3675 |
| 10 | HP Walker | answered | correct | | 6000 |
| 11 | Guardian vs Walker | answered | correct | | 5500/1375 vs 6000/3675 |
| 12 | Souls Guardian | answered | correct | | 1375 |
| 13 | Base guardians fallen | answered | partial | C | 1000 souls, game effect not in data |
| 14 | Shrines | answered | partial | C | 2000 souls, count not in data |
| 15 | HP Patron | answered | correct | | 12000 |
| 16 | Mid boss spawn | answered | correct | | initial delay 0, interval 420 s |
| 17 | Rejuvenator gives | answered | correct | | 180 s buff, multipliers per spawn |
| 18 | Rejuvenator duration | answered | correct | | 180 s |
| 19 | Urn | answered | fail | E | crate_*/idol_* rows not recognised as Urn |
| 20 | Urn first spawn | insufficient | fail | E | crate_spawn_initial_delay 600 served but not used |
| 21 | Respawn min 20 | answered | correct | | 38 s |
| 22 | Respawn Anfang | answered | partial | B | cites 8 s ramp, does not commit |
| 23 | Souls Level 10 | answered | correct | | 4600 (index 0 = level 1) |
| 24 | Ability points when | answered | correct | | |
| 25 | AP Upgrade cost | answered | fail | C | 1/2/5 wiki only |
| 26 | Tier 3 cost | answered | correct | | 3200 |
| 27 | Item tiers | answered | correct | | 800/1600/3200/6400 |
| 28 | Headshot Booster | answered | correct | | |
| 29 | High Velocity Mag | answered | correct | | |
| 30 | Extra Stamina | answered | correct | | |
| 31 | Improved Cooldown | answered | correct | | |
| 32 | Boundless Spirit cost | answered | correct | | 6400 |
| 33 | Haze HP start | answered | correct | | 730 |
| 34 | Lady Geist ult | answered | correct | | |
| 35 | Infernus afterburn dmg | answered | fail | D | no DPS stat |
| 36 | Seven ult upgrades | answered | partial | D | two of three upgrades |
| 37 | Power increase | answered | partial | C | flag present, effect not in data |
| 38 | Bullet dmg per level | insufficient | fail | D | hero level scaling not served |
| 39 | Zipline speed | answered | fail | C | no zipline speed in data |
| 40 | Teleporter | answered | fail | C | map data |
| 41 | Lanes | insufficient | fail | C | map data |
| 42 | Game modes | answered | wrong | C | Discord LFG modes |
| 43 | Street brawl | answered | correct | | rounds, souls, AP, respawn |
| 44 | Last patch | answered | correct | | |
| 45 | Haze nerf last patch | insufficient | fail | F | correct before; patch path variance |
| 46 | Sinners sacrifice | answered | correct | | |
| 47 | Souls per kill | answered | partial | B | 200 to 2200 range, no value at a time |
| 48 | Breakables / crates | answered | partial | C | spawn times, no reward data |
| 49 | Mid boss respawn | answered | correct | | 420 s |
| 50 | Urn cash-in bonus | answered | fail | E | idol_cashin_buff_count 4 not explained |

## Round 2, release daa885b4

Changes: get_mechanic derives reference minutes 0/10/20/30 when no minute is given (lane_troopers only minute 0 to stay inside one tool result), hero kill bounty over game time, urn first spawn and respawn in minutes, wave instant/orb split per trooper and per whole wave, condition.meaning for urn, idol and deny rows, card ordering keeps English description and tier texts, hero extractor v4 serves LevelScaling as boon_* facts (340 facts, 67 heroes), tool pool sized from postgres.max_connections (brain_readonly limit 17). Method: all 50 questions one at a time through `ask.sh` after deploy and one `brain-job@deadlock-data` ingest. Raw answers in `round2.jsonl`.

Result: **36 of 50 correct**, 9 partial, 5 failed (round 1: 30 / 8 / 12).

| # | Question | Status | Verdict | Class | Note |
|---|---|---|---|---|---|
| 1 | Souls Trooper wave min 16 | answered | correct | | 132 per trooper, 528 per wave, 264 instant / 264 orb per wave |
| 2 | Souls einzelner Trooper Anfang | answered | partial | E | 100, 50 instant / 50 orb right; claims +2/min starts after minute 8 (laning_gold_rules_end_time), formula applies it from minute 0 |
| 3 | Trooper pro Welle | answered | correct | | 4 |
| 4 | Prozent ohne Orb | answered | correct | | 0.5 |
| 5 | Denien | answered | correct | | denier 100 %, denied 0, 50 per trooper at minute 0 |
| 6 | Zu dritt auf Lane | answered | correct | | 36 % |
| 7 | Jungle camp souls | answered | correct | | 41/68/181 plus values at 10/20/30 |
| 8 | Jungle spawn | answered | correct | | |
| 9 | Souls Walker | answered | correct | | 3675 |
| 10 | HP Walker | answered | correct | | 6000 |
| 11 | Guardian vs Walker | answered | correct | | |
| 12 | Souls Guardian | answered | correct | | 1375 |
| 13 | Base guardians fallen | answered | partial | C | 4000 HP, 1000 souls; game effect not in data |
| 14 | Shrines | answered | partial | C | 2000 souls; count not in data |
| 15 | HP Patron | answered | correct | | 12000 |
| 16 | Mid boss spawn | answered | correct | | |
| 17 | Rejuvenator gives | answered | correct | | |
| 18 | Rejuvenator duration | answered | correct | | 180 s |
| 19 | Urn | answered | partial | E | 10 min first spawn, every 5 min, 250 + 70/min right; calls delivery_overtime_bonus 50 an unconditional bonus |
| 20 | Urn first spawn | answered | correct | | 600 s |
| 21 | Respawn min 20 | answered | correct | | 38 s |
| 22 | Respawn Anfang | answered | partial | B | "zwischen 8 und 38": min and max ramps are derived as two curves; source has no semantics for min/max ramp |
| 23 | Souls Level 10 | answered | correct | | 4600 |
| 24 | Ability points when | answered | correct | | |
| 25 | AP Upgrade cost | answered | fail | C | 1/2/5 wiki only, declines correctly |
| 26 | Tier 3 cost | answered | correct | | 3200 |
| 27 | Item tiers | answered | correct | | |
| 28 | Headshot Booster | answered | correct | | |
| 29 | High Velocity Mag | answered | correct | | |
| 30 | Extra Stamina | answered | correct | | |
| 31 | Improved Cooldown | answered | correct | | |
| 32 | Boundless Spirit cost | answered | correct | | 6400 |
| 33 | Haze HP start | answered | correct | | 730, +33 per boon |
| 34 | Lady Geist ult | answered | correct | | |
| 35 | Infernus afterburn dmg | answered | correct | | 14 DPS, 0.66 spirit scaling |
| 36 | Seven ult upgrades | answered | correct | | all three upgrades |
| 37 | Power increase | answered | partial | E | boon_* facts exist per hero, model does not connect Power Increase to them |
| 38 | Bullet dmg per level | answered | partial | E | real boon value, but picks Violet instead of asking for the hero |
| 39 | Zipline speed | answered | fail | E | ZipSpeedInner/Outer exist on citadel_ability_zip_line, not found |
| 40 | Teleporter | answered | fail | C | locations wiki only, declines correctly |
| 41 | Lanes | answered | fail | C | LaneInfo in generic-data not served |
| 42 | Game modes | answered | fail | C | mode list wiki only, mixes Discord LFG modes |
| 43 | Street brawl | answered | correct | | |
| 44 | Last patch | answered | correct | | |
| 45 | Haze nerf last patch | answered | correct | | |
| 46 | Sinners sacrifice | answered | correct | | 155 + 1.08 %/min |
| 47 | Souls per kill | answered | correct | | 200, 700, 1200, 1700 at 0/10/20/30 |
| 48 | Breakables / crates | answered | partial | C | spawn times, no reward data |
| 49 | Mid boss respawn | answered | correct | | 420 s |
| 50 | Urn cash-in bonus | answered | partial | E | reward right; calls delivery_overtime_bonus 50 the cash-in bonus, idol_cashin_buff_count 4 unused |

Tool pool: no "not safely readable" error in this run or in a 4-parallel live check (4 of 4 answered); the single-connection pool was the cause (tool_pool had max_connections(1) while brain-serve runs up to max_connections parallel tool rounds).

### Class C sources

| Item | Needed source | Upstream deadlock-data | Wiki only |
|---|---|---|---|
| Zipline speed | `ability-data` citadel_ability_zip_line ZipSpeedInner 700, ZipSpeedOuter 830, ZipAcc, LatchSpeed | yes, already in PG | no; findability (class E) |
| Map: lanes | `generic-data` LaneInfo (lane names, colors, 4 used lanes) | yes, not projected | no |
| Map: teleporters | positions | no (TeleporterParams has only effects and sounds) | yes |
| Map: districts | `generic-data` MapDistrictLocalization, MiniMapOffsets | yes, not projected | no |
| Game modes | `generic-data` StreetBrawl, KothParams; `street-brawl-data`; a mode list | parameters yes, list no | list yes |
| AP upgrade cost | per-tier cost 1/2/5 | no (`soul-unlock-data` has only when AP are granted) | yes |
