---
title: "ability_charged_bomb"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_charged_bomb"
canonical_name: "ability_charged_bomb"
snapshot_id: 39414
source_document_id: 7070
payload_hash: "23e82c6ec33a7c987cd09b4fca962f5dd14b1fa99a2de661698debfbd32284ef"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.264221+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# ability_charged_bomb

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_charged_bomb`
- Snapshot ID: `39414`
- Source-Dokument: `7070`
- Kurzinfo: ability_charged_bomb aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 10.5,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "BlastJumpVelocity": 25,
  "BlastJumpVelocityCrouch": 30,
  "BlastJumpVelocityGround": 20,
  "ChannelMoveSpeed": -1,
  "ChargeBombModifier": {
    "Class": "ChargedBomb",
    "Subclass": "ChargedBomb"
  },
  "IsDisabled": false,
  "Key": "ability_charged_bomb",
  "MaxChargeTime": 2.0,
  "MaxDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.16064
    },
    "Value": 100
  },
  "Name": null,
  "Radius": 7,
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
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
    "card_name": "Unknown(ability_charged_bomb)",
    "hero_key": "hero_bomber",
    "hero_name": "Bomber",
    "slot": "1"
  }
}
````
