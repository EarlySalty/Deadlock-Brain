---
title: "Crawling Plague"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_trapper_spiderwave"
canonical_name: "Crawling Plague"
snapshot_id: 39547
source_document_id: 7070
payload_hash: "2f781f5380fe74930053c87f1a3a2cbbb013411dc1fcf3333d6f07685cf3e2af"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.603249+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Crawling Plague

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_trapper_spiderwave`
- Snapshot ID: `39547`
- Source-Dokument: `7070`
- Kurzinfo: Crawling Plague aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.6,
  "AbilityCooldown": 160,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCanSetQuickCast"
  ],
  "ChannelMoveSpeed": 1.3,
  "IsDisabled": false,
  "Key": "ability_trapper_spiderwave",
  "Name": "Crawling Plague",
  "Radius": 3.5,
  "SpiderArmingTime": 0.5,
  "SpiderChaseVelocity": 400,
  "SpiderClimbHeight": 0.3,
  "SpiderCount": 5,
  "SpiderDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.744
    },
    "Value": 140
  },
  "SpiderDistAboveGround": 0.1,
  "SpiderExplodeRadius": 3,
  "SpiderFloatDownRate": 8,
  "SpiderGravity": 1,
  "SpiderLifetime": 25,
  "SpiderRandomPositionRadius": 4,
  "SpiderSearchRadius": 2,
  "SpiderTickRate": 0.3,
  "SpiritReducedPerStack": 5,
  "SpiritResReducedPerStack": 5,
  "SpiritStealDebuffModifier": {
    "Class": "TrapperStealspiritDebuff",
    "Subclass": "TrapperStealspiritDebuff"
  },
  "SpiritStealDuration": 10,
  "SpreadAngle": 30,
  "SpreadDistance": 900,
  "Upgrades": [
    {
      "AbilityCooldown": -45
    },
    {
      "SpiritReducedPerStack": 3,
      "SpiritResReducedPerStack": 3
    },
    {
      "SpiderCount": 5,
      "SpreadDistance": 900
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-data.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_card": {
    "card_name": "Crawling Plague",
    "hero_key": "hero_trapper",
    "hero_name": "Trapper",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_trapper",
      "hero_name": "Trapper",
      "lookup": "crawling plague",
      "name": "Crawling Plague",
      "type": "ability"
    }
  ]
}
````
