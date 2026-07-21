---
title: "Wrecking Ball"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_wrecking_ball"
canonical_name: "Wrecking Ball"
snapshot_id: 39589
source_document_id: 7070
payload_hash: "65cfb5a1b40272ff71b0de702c43f640759d7dc43411f98e05547c5cf53fb1bd"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.710221+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Wrecking Ball

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_wrecking_ball`
- Snapshot ID: `39589`
- Source-Dokument: `7070`
- Kurzinfo: Wrecking Ball aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 1.4,
  "AbilityCastRange": 50,
  "AbilityCooldown": 31,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6,
  "AbilityUnitTargetLimit": 1,
  "AutoThrowModifier": {
    "Class": "WreckingBallAutoThrow",
    "Subclass": "WreckingBallAutoThrow"
  },
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDontTriggerSpellBlock",
    "BehaviorProjectileFiredAsBullet"
  ],
  "ChannelMoveSpeed": 1.3,
  "IsDisabled": false,
  "Key": "ability_wrecking_ball",
  "MinSpeed": 80,
  "MoveSpeedLimit": 6,
  "Name": "Wrecking Ball",
  "StunDuration": 1,
  "TechCleaveExpireTime": 0.4,
  "Upgrades": [
    {
      "AbilityCooldown": -7.5
    },
    {
      "WreckingBallDamage": 80
    },
    {
      "MoveSpeedLimit": 4
    }
  ],
  "WreckingBallDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.302
    },
    "Value": 150
  },
  "WreckingBallPushForce": 1500,
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
  }
}
````
