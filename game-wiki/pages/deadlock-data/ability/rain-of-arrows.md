---
title: "Rain of Arrows"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_power_jump"
canonical_name: "Rain of Arrows"
snapshot_id: 39507
source_document_id: 7070
payload_hash: "0a52b375ffdc66c27c69c672fd6d07e83384ccbdc6ebac08580a5c9b3aad4a6d"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.502815+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Rain of Arrows

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_power_jump`
- Snapshot ID: `39507`
- Source-Dokument: `7070`
- Kurzinfo: Rain of Arrows aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 25.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 4,
  "AbilityUnitTargetLimit": 1,
  "AirMoveIncreasePercent": 25,
  "AirSpeedMax": 6.64,
  "AltJumpSpeed": 12,
  "BehaviourBits": [
    "BehaviorInputDirectional2d",
    "BehaviorNoTarget",
    "BehaviorAllowAltCast",
    "BehaviorMovement",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BulletSplitShot": 5,
  "FallSpeedMax": 0.635,
  "FxRadius": 4,
  "InAirModifier": {
    "Class": "AirRaid",
    "EnabledStateMask": [
      "DisableAirSpreadPenalty",
      "UnlimitedAirDashes"
    ],
    "SlowModifier": {
      "Class": "SlowBase",
      "Subclass": "SlowBase"
    },
    "Subclass": "AirRaid"
  },
  "IsDisabled": false,
  "JumpPitch": -60,
  "JumpSpeed": 27.5,
  "Key": "ability_power_jump",
  "Name": "Rain of Arrows",
  "PowerJumpModifier": {
    "AirDrag": 2.0,
    "Class": "PowerJump",
    "EnabledStateMask": [
      "AbilityMovement",
      "ZiplineDisabled",
      "JumpDisabled"
    ],
    "Subclass": "PowerJump",
    "VerticalCameraOffset": 20.0,
    "VerticalCameraOffsetLerpTime": 0.4
  },
  "Upgrades": [
    {
      "SlowDuration": 1.5,
      "SlowPercent": 30,
      "WeaponDamageBonus": 3
    },
    {
      "AbilityCooldown": -12.0
    },
    {
      "BulletLifestealPercent": 30,
      "EvasionPercent": 30,
      "TechLifestealPercent": 30
    }
  ],
  "WeaponDamageBonus": 3,
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
    "card_name": "Rain of Arrows",
    "hero_key": "hero_orion",
    "hero_name": "Grey Talon",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_orion",
      "hero_name": "Grey Talon",
      "lookup": "rain of arrows",
      "name": "Rain of Arrows",
      "type": "ability"
    }
  ]
}
````
