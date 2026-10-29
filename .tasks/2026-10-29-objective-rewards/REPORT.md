# Objective rewards and misspelled objective names

Live miss: "Was bringt mir ein Shiren und ein Base Guardien wenn ich die kille" resolved "Shiren" to Shiv (pg_trgm similarity 0.333) and answered only 1000 souls plus the Base Guardian HP.

## Commits on main

- 16b10ba7 FindEntity maps objective words to their mechanic rule entities with an edit-distance term table (`brain-serve/src/mechanic_terms.rs`) when no exact match and no strong fuzzy suggestion (similarity below 0.5) exists. It returns `match: "mechanic"` plus `mechanics`. get_mechanic explains every ObjectiveParams reward row; the objectives rule gains the Shrine super trooper buff and the objective convars.
- 3656bcc6 mechanics extractor version 2, so already ingested raw objects are re-extracted; the mechanic match honours `limit`.
- 52939614 the mechanic fallback also runs when the model passes a `kind` such as `hero`; Patron phase 1 reward 0 is named as 0 souls.
- 303be552 Super Trooper DamageScale is no longer presented as a percentage (the source gives `m_flDamageScale = 60.0` without a unit).
- 7fa2e4ca concept rows are ordered by MEANINGS position, so reward rows survive the 16 KB tool result trimmer ahead of generic boss stat rows.

Deployed release: 7fa2e4ca50ec404676f4db9c9dc02f426422bbdb. brain-serve and deadlock-brain-site active. Ingest run 3528 of `brain-job@deadlock-data` published the new rows.

## Data source of the bonuses

All values come from the deadlock-data game files, read through `get_mechanic objectives`:

- `generic-data.json` ObjectiveParams: Tier1GoldKill 1375 (Guardian), Tier2GoldKill 3675 (Walker), BaseGuardiansGoldKill 1000, ShrinesGoldKill 2000, PatronPhase1GoldKill 0, all `*GoldOrbs` 0, NearPlayerSplitPct 25.0.
- `convars`: `citadel_no_orbs_on_objective_kill` true, `citadel_super_trooper_gold_mult` 0.85.
- `npc-data.json` troopers `ShrinesDownBuffModifier`: HealthScale 1.4, DamageScale 60.0, state MODIFIER_STATE_SUPER_TROOPER.
- `npc-data.json` boss rows: HP and combat stats of Guardian, Walker, Patron, Base Guardian.

## Not in the data

- A zipline speed or zipline trooper bonus after objective kills. Only patch notes mention it (2026-03-06: base guardian pair spawns a zipline boosted trooper wave).
- How NearPlayerSplitPct splits souls between the team and nearby players.
- The unit of DamageScale 60.0.
- Shrine invulnerability rules and Patron behaviour (patch notes only).

## Live answers (7fa2e4ca)

- Exact question: "Mit Shiren meinst du wohl den Shrine. Wenn du einen Shrine zerstörst, bekommst du 2000 Seelen. Für einen Base Guardian gibt es 1000 Seelen. Bei beiden entstehen keine Seelenorbs. Spieler in der Nähe des Ziels bekommen laut Daten 25.0 Prozent der Objective-Seelen, wie genau Team und Nahspieler aufgeteilt werden, steht aber nicht in den Daten. Zerstörte Shrines verstärken außerdem die Super Trooper, deren Leben laut Daten mit dem Faktor 1.4 skaliert wird."
- Guardian 1375, Walker 3675, Patron phase 1 0 souls with 12000 HP per phase, Shiv 830 HP: all answered.

## Open

- Patch note lines (zipline wave) are not attached to objective answers; patch retrieval belongs to the patch history work.
- A stale `mechanic_lane_troopers / citadel_super_trooper_gold_mult` serve row remains next to the new objectives row; serve publication does not drop stats that left a rule.
- No scratch PG test for FindEntity returning `match: "mechanic"`; the resolver and ordering are covered by unit tests.
