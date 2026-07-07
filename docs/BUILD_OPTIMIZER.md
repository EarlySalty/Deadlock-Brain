# Build Optimizer

`deadlock_brain.build_optimizer` creates a deterministic first-pass build
context for heroes. It is not a final coaching brain yet; it is the scoring
layer that prevents model-only guesses.

## CLI

```bash
./rust/target/release/deadlock-brain item Refresher --pretty
./rust/target/release/deadlock-brain build "Mo & Krill" --pretty
```

## Inputs

- Deadlock Assets API item payloads: cost, tier, slot, active/passive state,
  description, properties, scaling hints, components.
- Deadlock Assets API hero payloads: `cost_bonuses`, `purchase_bonuses`,
  `item_draft_bucketing`, role/playstyle, starting stats and level-up stats.
- Deadlock Assets API ability payloads for the hero's four signatures:
  cooldowns, ranges, durations, scaling stats, control effects and upgrades.
- Google Sheet hero stats through the existing review context.
- Patch timeline signals through the existing review context.
- Cached Deadlock Wiki economy/mechanic pages: `The Curiosity Shop`, `Item`,
  `Souls`, `Level`, `Ability`, `Status Effects`, `Crowd Control`,
  `Damage Resistance`, `Weapon Damage`, `Stats`, and `Mechanics`. Wiki pages
  are pulled manually one at a time and are not bulk-crawled.

## Current Scoring Shape

The scorer separates:

- early economy/lane items
- core impact items
- situational counter items
- late/luxury items
- expensive items that need justification

Each item also gets semantic archetypes from the API payload:
`lane_farm`, `lane_trade`, `kill_setup`, `core_scaling`, `counter`, `support`,
`luxury`, and `active_burden`. These tags make the recommendation auditable:
plain resist side stats do not make an item a counter item, while explicit
counter mechanics and named counter items stay situational.

The important 4800 souls shop-bonus threshold is modeled explicitly per shop
category. The output includes example routes to reach 4800 in HP, Spirit and
Gun categories.

The hero understanding section is printed before the item list. It shows the
inferred gameplan, damage plan (`weapon`, `spirit`, `hybrid`, `utility`),
priority scaling stats, ability role tags, key ability values, lane priorities
and the build implications. If this section is wrong, the item list should not
be trusted yet.

Damage-plan inference is based on concrete ability damage properties and weapon
hooks instead of treating every generic `ETechPower` field as a Spirit build
signal. This prevents gun heroes such as `Haze` from being misread as Spirit
heroes while still allowing real hybrids such as `Infernus`.

For Mo & Krill, expensive cooldown-reset items such as `Refresher` are
penalized unless a clear data-backed reason exists. This keeps the system from
recommending high-cost items just because they are generically powerful.
