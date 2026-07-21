---
title: "Shoulder Charge"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_bull_charge"
canonical_name: "Shoulder Charge"
snapshot_id: 39597
source_document_id: 7070
payload_hash: "a1d2fcd8244e8ff1788b9ac1d488aba658362ea73312e6c1e043d34add1549b5"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.733395+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Shoulder Charge

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_bull_charge`
- Snapshot ID: `39597`
- Source-Dokument: `7070`
- Kurzinfo: Shoulder Charge aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 33.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 1.4,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDontTriggerPostCastOnCastComplete",
    "BehaviorMovement",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "CameraTurnRateMax": 200,
  "ChannelMoveSpeed": -1,
  "ChargeDragVerticalOffset": 30,
  "ChargeRadius": 2.2,
  "ChargeSpeedMax": 30,
  "CollidePlayersStopTime": 0.3,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.4
    },
    "Value": 30
  },
  "IsDisabled": false,
  "Key": "citadel_ability_bull_charge",
  "ModifierBullCharging": {
    "Class": "CitadelBullCharging",
    "EnabledStateMask": [
      "AbilityMovement",
      "MantleDisabled",
      "MeleeDisabled",
      "SlidingDisabled",
      "DuckingDisabled",
      "ForceCanParry"
    ],
    "Subclass": "CitadelBullCharging"
  },
  "ModifierChargeDragEnemy": {
    "Class": "ChargeDragEnemy",
    "EnabledStateMask": [
      "AbilityMovementDebuff",
      "IgnorePortals"
    ],
    "ForwardOffset": 120,
    "Subclass": "ChargeDragEnemy",
    "VerticalOffset": 30
  },
  "ModifierTossAirControlLockout": {
    "Class": "Base",
    "Duration": 1.0,
    "EnabledStateMask": [
      "MovementAbilityRestricted"
    ],
    "Subclass": "AirControlLockout"
  },
  "ModifierWeaponPowerIncrease": {
    "Class": "Base",
    "Subclass": "ShoulderChargeBuff"
  },
  "Name": "Shoulder Charge",
  "SideMoveSpeedReduction": -65,
  "SlowModifier": {
    "Class": "SlowBase",
    "Subclass": "WallSlamSlow"
  },
  "SpeedInitial": 18.75,
  "StunDuration": 0.3,
  "TossUpMagnitude": 0.5,
  "TurnRateMax": 140,
  "Upgrades": [
    {
      "SlowDuration": 3,
      "SlowPercent": 40
    },
    {
      "StunDuration": 0.8
    },
    {
      "AbilityCooldown": -18,
      "WeaponDamageBonus": 1.5,
      "WeaponPowerIncreaseDuration": 6
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
    "card_name": "Shoulder Charge",
    "hero_key": "hero_atlas",
    "hero_name": "Abrams",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_atlas",
      "hero_name": "Abrams",
      "lookup": "shoulder charge",
      "name": "Shoulder Charge",
      "type": "ability"
    }
  ]
}
````
