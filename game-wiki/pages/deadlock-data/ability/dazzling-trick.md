---
title: "Dazzling Trick"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_unicorn_prismaticguard"
canonical_name: "Dazzling Trick"
snapshot_id: 39553
source_document_id: 7070
payload_hash: "00a55a42719d359b16d37fc7031fa9939970a1cbbbc50ea83045e0b74388cf3a"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.619721+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Dazzling Trick

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_unicorn_prismaticguard`
- Snapshot ID: `39553`
- Source-Dokument: `7070`
- Kurzinfo: Dazzling Trick aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 32,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BarrierDamagePercentage": 50,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectilePassThroughWorld"
  ],
  "BuffDuration": 4,
  "BuffModifier": {
    "Class": "UnicornPrismaticGuard",
    "Subclass": "UnicornPrismaticGuardBuff",
    "VerticalBoost": 0.5
  },
  "ChannelMoveSpeed": -1,
  "CombatBarrier": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.8
    },
    "Value": 100
  },
  "DebuffDuration": 1.75,
  "DebuffModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "Silenced"
    ],
    "Subclass": "UnicornPrismaticGuardDebuff"
  },
  "ExplodeRadius": 14,
  "IsDisabled": false,
  "Key": "ability_unicorn_prismaticguard",
  "MaxLifetime": 4,
  "Name": "Dazzling Trick",
  "Upgrades": [
    {
      "BonusMoveSpeed": 3.5
    },
    {
      "CombatBarrier": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.7
        },
        "Value": 80
      }
    },
    {
      "AbilityCooldown": -18,
      "DebuffDuration": 1.5
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
    "card_name": "Dazzling Trick",
    "hero_key": "hero_unicorn",
    "hero_name": "Celeste",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_unicorn",
      "hero_name": "Celeste",
      "lookup": "dazzling trick",
      "name": "Dazzling Trick",
      "type": "ability"
    }
  ]
}
````
