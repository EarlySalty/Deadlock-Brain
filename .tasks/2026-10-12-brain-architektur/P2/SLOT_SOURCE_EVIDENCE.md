# P2 slot source diagnosis, 2026-10-13

## Result

The public Assets Heroes response contains explicit named mappings under `items.signature1` through `items.signature4`. The existing published P2 source adapter imports Items only, so these mappings are not currently extracted or published. This narrows the blocker: source evidence exists, but the published ingestion and Serve contract do not carry it. The previous isolated fixer correctly established that an adapter/view-only repair cannot invent a mapping from the currently published data; that finding does not establish absence of source evidence.

## Capture and verification

- Existing endpoint declaration: `rust/crates/dbrain-sources/src/assets_api.rs:21`, `heroes`, `/v1/assets/heroes?only_active=true`.
- Read-only public request: `https://api.deadlock-api.com/v1/assets/heroes?only_active=true&language=english`.
- Capture: `/tmp/p2-hero-slot-source-4bbdfa7a-HXX3Ep/heroes-en.json`, 1,334,748 bytes, SHA-256 `5c163d19407aa683a6ebbae6e79d58885d71b3af3c0343e28cf0ec2e7fc75c31`.
- Download used a new empty directory, a 60-second timeout and an 8,388,608-byte bound. JSON inspection used `python3 -I` from the assigned trusted worktree, with capture paths passed as arguments.
- The response contains 41 active heroes and 164 explicit signature values, each a string. Every value matched an Ability `class_name` in the independently captured Items snapshot `/tmp/p2-public-asset-4bbdfa7a-PGvHG8/items-en.json`; no missing values or unresolved class names were observed in this pair of captures.
- Each of `signature1`, `signature2` and `signature3` matched 41 `ability_type=signature` records. `signature4` matched 41 `ability_type=ultimate` records. These are explicit field names, not inferred list positions.

These independent HTTP captures are diagnostic snapshots, not an atomic version-pinned dataset and not published canonical receipts. Their temporary paths do not guarantee durable availability. No production ingestion, database mutation or service change was performed.

## Concrete source mappings

| Hero | Source hero ID | Source field | Source ability class name | Matched source ability ID |
| --- | --- | --- | --- | --- |
| Wraith | 7 | `items.signature1` | `citadel_ability_card_toss` | 1999680326 |
| Wraith | 7 | `items.signature2` | `citadel_ability_projectmind` | 4147641675 |
| Wraith | 7 | `items.signature3` | `citadel_ability_wraith_rapidfire` | 1842576017 |
| Wraith | 7 | `items.signature4` | `citadel_ability_psychic_lift` | 2981692841 |
| Haze | 13 | `items.signature1` | `ability_sleep_dagger` | 2948410412 |
| Haze | 13 | `items.signature2` | `ability_smoke_bomb` | 2414191464 |
| Haze | 13 | `items.signature3` | `ability_stacking_damage` | 1080948381 |
| Haze | 13 | `items.signature4` | `ability_bullet_flurry` | 731943444 |

Source IDs above are not canonical PostgreSQL entity IDs. Cross-source hero identity, ability resolution and the slot-to-Fact/qualifier publication contract still require an explicit owner-approved definition. No class-name heuristic or default list ordering was implemented.

## Remaining boundary

The frozen Core Relation has no slot field, and the frozen Core hero profile publishes an unavailable abilities section. Repair therefore needs ingestion of the evidenced mapping and a P2-owned Serve projection rather than merely changing the current adapter's availability check. Original P2 scope includes Assets Ability facts and relations; whether and how this Heroes-response mapping is assigned to P2 must be resolved with the orchestrator, without modifying the P1r Hero extractor or frozen Core.

Recommendation: approve a narrowly scoped P2 Assets mapping extraction from the explicit signature fields, define the scalar Fact/qualifier and canonical identity contract, and read it through an additive P2 Serve projection. Preserve existing selector behaviour until that contract is implemented and verified. This is a proposal, not an approval or implementation.

The substantive gate BLOCK remains unchanged. The separate package-size exception is still unapproved. No new fixer, gate retry, main merge, deployment or handover delivery was performed during this diagnosis.
