# Hero identity, answer status and build publication

The shared rule is `brain_contracts::hero_status::hero_status`. Hero identity and release status are separate. Development and disabled heroes remain available to answers, entity profiles, wiki retrieval and game tools. Only publication requires `is_released_hero`.

## Rule

A catalog hero has a positive API ID, a stable class identifier in the `hero_` namespace with a nonempty suffix, and a resolved display name distinct from that identifier. The namespace prevents items and abilities from receiving hero status. This rejects unresolved test entries without a hero-name allowlist or denylist. Sources without a catalog match remain answerable when their `Key` uses that same hero namespace, they explicitly mark development or Hero Labs, and they provide at least one named bound ability. This retains Airheart and Tokamak. An absent entry with neither that development evidence nor a catalog identity is not promoted into an independent hero. This excludes Generic Person, TargetDummy and the uncatalogued transformed Silver form from the roster; their original source data is retained.

Status precedence:

1. Explicit `InHeroLabs` or `in_hero_labs`: `hero_labs`.
2. A real catalog hero with `player_selectable=true`, `disabled=false`, and no prerelease or assigned-player restriction: `released`, even when `in_development=true`.
3. Other real heroes: `in_development`.

Missing or malformed playability flags cannot grant release. Explicit contradictory deadlock-data selectable/disabled flags also prevent release. Optional restriction flags may be absent, but explicit null or nonboolean values prevent release. No identity decision uses a specific hero name. Unreleased heroes may be inspected, ranked or calculated, but may not have builds published. Answers mention a nonreleased status briefly, for example “laut Spieldaten noch in Entwicklung”.

Deadlock-data import binds compact catalog observations to the verified assets mirror for the same client version. The derived snapshot records the catalog source run, source document and original hash under `_hero_catalog.receipt`; raw source originals remain unchanged. Missing verified catalog data fails the import explicitly. Normal maintenance reimports snapshots and rebuilds normalized entities and wiki pages; no manual database rewrite is required.

The final publication validator resolves `hero_id` against the current verified English catalog, requires exactly one released match, and checks items from that same complete mirror run before queue insertion or CLI publication. Development, Hero Labs, unresolved, missing and duplicate hero identities are refused without publishing.

## Measurement: 2026-10-11

Brain sources: complete assets mirror run 761, client 6766; deadlock-data revision `2a26e26535a390c309e300e2c7b2763fa5ad7abd`, client 6766. All 65 current English catalog rows use the `hero_` class namespace. Current complete mirror membership is used, not the union of historical snapshots. That distinction matters: historical assets contain Airheart and Tokamak, but the current mirror does not.

All rows below have deadlock-data `InDevelopment=true` and `InHeroLabs=false`. `S/D` means selectable/disabled. Named abilities count localized names in `BoundAbilities`, not merely placeholder keys.

| Source key | DD S/D | Catalog ID | API S/D | Named abilities | Answer status |
| --- | --- | --- | --- | --- | --- |
| hero_airheart | true/true | absent | unavailable | 2 | in_development |
| hero_baba | true/false | 88 | true/false | 4 | released |
| hero_boho | true/true | 68 | false/true | 0 | in_development |
| hero_bomber | true/true | 56 | false/true | 0 | in_development |
| hero_cadence | true/true | 54 | false/true | 4 | in_development |
| hero_druid | true/true | 73 | false/true | 0 | in_development |
| hero_fortuna | true/true | 75 | false/true | 0 | in_development |
| hero_genericperson | true/false | absent | unavailable | 0 | excluded placeholder |
| hero_graf | true/true | 74 | false/true | 0 | in_development |
| hero_gunslinger | true/true | 38 | false/true | 3 | in_development |
| hero_opera | true/true | 82 | false/true | 0 | in_development |
| hero_operative | true/true | 62 | false/true | 4 | in_development |
| hero_rutger | true/true | 49 | false/true | 4 | in_development |
| hero_shieldguy | true/true | 57 | false/true | 0 | in_development |
| hero_skyrunner | true/true | 70 | false/true | 3 | in_development |
| hero_slork | true/true | 53 | false/true | 4 | in_development |
| hero_swan | true/true | 71 | false/true | 2 | in_development |
| hero_testhero | true/true | 83 | false/true | 0 | excluded unresolved name |
| hero_thumper | true/true | 51 | false/true | 0 | in_development |
| hero_tokamak | false/true | absent | unavailable | 4 | in_development |
| hero_trapper | true/true | 61 | false/true | 4 | in_development |
| hero_vandal | true/true | 59 | false/true | 0 | in_development |
| hero_wrecker | true/true | 48 | false/true | 4 | in_development |
| hero_yakuza | true/true | 39 | false/true | 4 | in_development |

`hero_testhero` has API name `hero_testhero`, an unresolved class identifier. Its current deadlock-data name is empty and it has no named bound abilities. Generic Person and TargetDummy have no current catalog membership and no named abilities. Silver's transformed source entry has abilities, but no separate catalog identity and no development/Labs flag; it remains a source form, not another hero in the roster.

The local GameTracking mirror is client 6746 (2026-10-04), older than both Brain sources. It has the development and disabled flags for Generic Person and the 19 disabled catalog heroes; Generic Person is enabled, the others disabled. Baba, Airheart and Tokamak are absent at that revision. Selectable and Labs properties are not explicit in those tracking blocks, so no false default is inferred. Tracking cannot decide Baba's release and is not used for this classification.

The local Deadlock API source-document mirror has match-specific records, not a recent complete hero-stat census. Match absence is therefore not a release signal. Patch-note membership is also unnecessary once the complete version-bound catalog separates the entries; no patch-note absence is treated as evidence that a hero does not exist.
