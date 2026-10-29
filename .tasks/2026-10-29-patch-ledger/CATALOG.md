# Canonical entity catalog draft (2026-10-29)

## Existing canon entities (reused)
hero 67 (incl. internal), ability 436 (signature 244, ultimate 80, melee 64, innate 46), item 250 (77 disabled), weapon 95 via assets, rule 19, timer 7, patch 137.
Renames: items from `item-lineage`; hero and ability renames from "renamed" patch lines into canon.alias.

## Objectives (npc-data keys)
| entity | data key |
|---|---|
| objective_guardian | npc_boss_tier1 |
| objective_walker | npc_boss_tier2 (+ alt_, _weak) |
| objective_patron | npc_boss_tier3 |
| objective_shrine | npc_barrack_boss (+ _amber, _sapphire) |
| objective_base_guardian | base guardian npc |
| objective_mid_boss | mid boss npc; rejuvenator via rule_rejuvenator__generic_data (RejuvParams) |
| objective_urn | rule_urn__convars |
| objective_sinners_sacrifice | neutral_sinners_sacrifice |
| objective_soul_boxes | BreakableSpawnTimeDesc, golden statues, citadel_idol_orbs_to_spawn |

## New mechanic entities
| entity | current data anchor | legacy general lines |
|---|---|---|
| mechanic_soul_orbs (orb share, deny) | citadel_trooper_instant_gold_ratio_laning / _postlaning = 0.5, citadel_orb_allow_deny, citadel_deny_orb_scale, citadel_secure_gold_percent | 103 |
| mechanic_trooper_souls | citadel_trooper_gold_reward, _bonus_per_minute, TrooperKillGoldShareFrac | 23 |
| npc_troopers (incl. medic, super troopers) | trooper_* npc keys, citadel_super_trooper_gold_mult | 286 |
| npc_neutrals (denizens) | neutral_trooper_* GoldReward | 157 |
| mechanic_hero_kill_bounty (assist, comeback, streak) | HeroKillGoldShareFrac, citadel_player_gold_comeback_killer_fraction | 103 |
| mechanic_souls_general | soul economy lines without a narrower target | 112 |
| mechanic_parry | citadel_ability_melee_parry (cooldown 4.5, stun 2.75, damage taken 25, move speed 50) | 18 |
| mechanic_melee (light, heavy) | melee abilities, citadel_bounty_allow_melee_autoclaim | 37 |
| mechanic_respawn | rule_respawn__convars, citadel_player_spawn_time_max_respawn_time = 90 | 73 |
| mechanic_movement (sprint, stamina, dash, slide, jump, mantle) | convars + hero base stats | 299 |
| mechanic_zipline | zipline convars, trooper_zipline_container | 136 |
| mechanic_teleporter (vents) | map convars | 120 |
| mechanic_powerups | powerup convars | 100 |
| mechanic_shop (secret shop, sell, quickbuy, flex slots, item tiers) | shop convars | 186 |
| mechanic_weapon_general (ammo, reload, falloff, headshots, gun damage) | weapon convars | 480 |
| mechanic_spirit_general (spirit power scaling, resist) | generic-data scaling | 255 |
| mechanic_ability_points (AP, power increases, investment bonuses) | soul-unlock-data | 28 |
| map (lanes, buildings, districts, bridges) | map entity with lane children | 450 |
| mode_street_brawl | rule_street_brawl__generic_data | |
| mode_hideout (sandbox, practice, bots, tutorial) | | 99 |
| mode_hero_labs | | Hero Lab groups |
| system_matchmaking (ranks, queue, party, comms penalties) | | 110 |
| client_interface (UI, HUD, audio, visuals, camera, settings) | no game value, text-only history | 696 |
| client_stability (crash, performance, network) | text-only | 48 |
| client_bugfix | fallback only for fixes with no game subject | 279 |

## Linking rule
Every line gets one primary entity. Priority: hero ability > item > hero > weapon > objective > mechanic > mode/system > client. A line naming several entities gets one row per entity. client_* entities carry text-only history, so the unlinked rest should be media references and empty fragments, each listed with a reason.

## Sizing check on the legacy general bucket
About 5,100 of 7,019 lines match a target by keyword alone; most of the other 1,900 name an item, ability or hero and belong to existing entities. The extractor still decides per line.
