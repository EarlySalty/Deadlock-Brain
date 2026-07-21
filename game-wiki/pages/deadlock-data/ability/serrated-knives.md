---
title: "Serrated Knives"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_shiv_dagger"
canonical_name: "Serrated Knives"
snapshot_id: 39639
source_document_id: 7070
payload_hash: "dc6c0d87c1212b9f1ee275c49e3797fa4adb35e422cb4e77bd339ecf79567dab"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.836910+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Serrated Knives

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_shiv_dagger`
- Snapshot ID: `39639`
- Source-Dokument: `7070`
- Kurzinfo: Serrated Knives aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AOERadius": 10,
  "AbilityChannelTime": 0.2,
  "AbilityCharges": 2,
  "AbilityCooldown": 16,
  "AbilityCooldownBetweenCharge": 2,
  "AbilityPostCastDuration": 0.3,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCleaveDisabled",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BleedDPSPerStack": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.13
    },
    "Value": 10.0
  },
  "BleedDuration": 5,
  "BleedTickRate": 1,
  "ChannelMoveSpeed": -1,
  "DamageDebuffModifier": {
    "Class": "ShivThrownShivDamageDebuff",
    "Subclass": "ShivThrownShivDamageDebuff"
  },
  "ImpactDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 0
  },
  "IsDisabled": false,
  "Key": "citadel_ability_shiv_dagger",
  "MovementSlow": 35,
  "Name": "Serrated Knives",
  "RicochetCount": 1,
  "SlowDebuffModifier": {
    "Class": "ShivThrownShivSlowDebuff",
    "Subclass": "ShivThrownShivSlowDebuff"
  },
  "Upgrades": [
    {
      "BleedDuration": 2
    },
    {
      "AbilityCharges": 2
    },
    {
      "BleedDPSPerStack": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.09
        },
        "Value": 12
      }
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
    "card_name": "Serrated Knives",
    "hero_key": "hero_shiv",
    "hero_name": "Shiv",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_shiv",
      "hero_name": "Shiv",
      "lookup": "serrated knives",
      "name": "Serrated Knives",
      "type": "ability"
    }
  ]
}
````
