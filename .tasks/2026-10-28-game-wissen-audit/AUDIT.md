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
