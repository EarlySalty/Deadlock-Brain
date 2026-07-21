---
title: "Rallying Charge"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_bookworm_knightcharge"
canonical_name: "Rallying Charge"
snapshot_id: 39410
source_document_id: 7070
payload_hash: "0460479a5ac93eda3803055f368f5ba3856b39b22bf0c2d3b8f0c62c10b4b5da"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.254923+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Rallying Charge

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_bookworm_knightcharge`
- Snapshot ID: `39410`
- Source-Dokument: `7070`
- Kurzinfo: Rallying Charge aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": 600,
  "AbilityChannelTime": 0.7,
  "AbilityCooldown": 220,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 13,
  "AbilityUnitTargetLimit": 1,
  "AirDrag": 0.8,
  "AllyHeight": 20,
  "AllyRadius": 4,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectilePassThroughWorld",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorRefundHalfCooldownOnChannelInterrupt"
  ],
  "BonusMoveSpeed": 5,
  "BuffDuration": 9,
  "BuffModifier": {
    "Class": "BookwormKnightchargeBuff",
    "Subclass": "Buff"
  },
  "CancelCooldownRefundPercentage": 50,
  "ChannelMoveSpeed": 1.3,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.0
    },
    "Value": 125
  },
  "FallSpeedMax": 20,
  "GravityAcceleration": -1900,
  "GroundStickHeight": 0.05,
  "HealAmount": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.6
    },
    "Value": 125
  },
  "IsDisabled": false,
  "Key": "ability_bookworm_knightcharge",
  "KnightBonusPerWave": -99,
  "KnightChargeHeight": 3.5,
  "KnightChargeWidth": 1.7,
  "KnightCount": 5,
  "KnightCountInFirstWave": 5,
  "KnightJumpSpeed": 900,
  "KnightMaxFallHeight": -35,
  "KnightMaxJumpHeight": 30,
  "KnightNavForwardDistance": 8,
  "KnightNavSearchDistance": 10,
  "KnightPositionSpread": 1.8,
  "KnightPositionStagger": -4,
  "KnightWhiskerLength": 300,
  "KnightWhiskerSide": 50,
  "KnightWhiskerStrength": 0.2,
  "MaxAmp": 100,
  "MaxAmpDistance": 250,
  "Name": "Rallying Charge",
  "StunDuration": 1.0,
  "TargetFindingDelay": 0.04,
  "TossBackSpeed": 100,
  "TossUpSpeed": 600,
  "Upgrades": [
    {
      "HealAmount": 150
    },
    {
      "AbilityCooldown": -45,
      "KnightCount": 4,
      "KnightCountInFirstWave": 4
    },
    {
      "Damage": 160.0,
      "MaxAmp": 70,
      "StunDuration": 0.5
    }
  ],
  "WaveCount": 2,
  "WavePositionStagger": -15,
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
    "card_name": "Rallying Charge",
    "hero_key": "hero_bookworm",
    "hero_name": "Paige",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_bookworm",
      "hero_name": "Paige",
      "lookup": "rallying charge",
      "name": "Rallying Charge",
      "type": "ability"
    }
  ]
}
````
