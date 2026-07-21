---
title: "Flight"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_hornet_leap"
canonical_name: "Flight"
snapshot_id: 39616
source_document_id: 7070
payload_hash: "c027458461c3d4a17a02383fb8f3c17c66d2df51b89377a2a99728297348e6a9"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.782932+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Flight

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_hornet_leap`
- Snapshot ID: `39616`
- Source-Dokument: `7070`
- Kurzinfo: Flight aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityChannelTime": 0.2,
  "AbilityCooldown": 42.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 13,
  "AbilityUnitTargetLimit": 1,
  "AirSideMoveSpeedPercentage": -35,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorMovement",
    "BehaviorRequireAbilityButtonToCancel",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "FlyingItemCastRange": 50,
  "IsDisabled": false,
  "JumpVelocity": 1000,
  "Key": "citadel_ability_hornet_leap",
  "KillCheckModifier": {
    "Class": "Base",
    "Duration": 1.0,
    "Subclass": "KillcheckModifier"
  },
  "LeapModifier": {
    "Class": "HornetLeap",
    "EnabledStateMask": [
      "AbilityMovement",
      "JumpDisabled",
      "DisableAirSpreadPenalty",
      "UnlimitedAirDashes",
      "ZiplineDisabled"
    ],
    "Subclass": "HornetLeap"
  },
  "MagicDamagePerBullet": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.18
    },
    "Value": 10
  },
  "MaxFlyHeight": 1720,
  "MinVelocityZ": -20.0,
  "Name": "Flight",
  "Upgrades": [
    {
      "BonusClipSizePercent": 50
    },
    {
      "AbilityDuration": 10
    },
    {
      "MagicDamagePerBullet": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.1
        },
        "Value": 10
      },
      "RefreshOnKill": 1
    }
  ],
  "WeaponRecoilReduction": 40,
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
    "card_name": "Flight",
    "hero_key": "hero_hornet",
    "hero_name": "Vindicta",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_hornet",
      "hero_name": "Vindicta",
      "lookup": "flight",
      "name": "Flight",
      "type": "ability"
    }
  ]
}
````
