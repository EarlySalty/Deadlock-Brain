---
title: "Radiant Daggers"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_unicorn_luminousstrike"
canonical_name: "Radiant Daggers"
snapshot_id: 39552
source_document_id: 7070
payload_hash: "49471c3b8c32711dd5785297c782cb504a7b7eca9d1963f6ae821e25515dddbf"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.617331+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Radiant Daggers

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_unicorn_luminousstrike`
- Snapshot ID: `39552`
- Source-Dokument: `7070`
- Kurzinfo: Radiant Daggers aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": 30,
  "AbilityCharges": 1,
  "AbilityCooldown": 33,
  "AbilityCooldownBetweenCharge": 2,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorAlwaysPreviewRadius",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectilePassThroughWorld",
    "BehaviorCanSetQuickCast"
  ],
  "BuffDelay": 0.75,
  "BuffDuration": 30,
  "BuffMaxStacks": 6,
  "BuffModifier": {
    "Class": "UnicornLuminousstrikeBuff",
    "Subclass": "LuminousStrikeBuff"
  },
  "ChannelMoveSpeed": -1,
  "ClimbHeight": 50.0,
  "ExplosionInterval": 0.7,
  "ExplosionRadius": 8,
  "ImpactDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.63
    },
    "Value": 55
  },
  "IsDisabled": false,
  "Key": "ability_unicorn_luminousstrike",
  "MagicIncreasePerStack": 8,
  "Name": "Radiant Daggers",
  "PostExplosionDuration": 0.8,
  "PreExplosionDuration": 1.4,
  "TickRate": 0.5,
  "Upgrades": [
    {
      "AbilityCharges": 2
    },
    {
      "AbilityCooldown": -22,
      "ImpactDamage": 80
    },
    {
      "FireRatePerStack": 9,
      "MagicIncreasePerStack": 3
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
    "card_name": "Radiant Daggers",
    "hero_key": "hero_unicorn",
    "hero_name": "Celeste",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_unicorn",
      "hero_name": "Celeste",
      "lookup": "radiant daggers",
      "name": "Radiant Daggers",
      "type": "ability"
    }
  ]
}
````
