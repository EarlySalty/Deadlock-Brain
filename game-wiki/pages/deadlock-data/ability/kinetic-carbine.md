---
title: "Kinetic Carbine"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_chrono_kinetic_carbine"
canonical_name: "Kinetic Carbine"
snapshot_id: 39603
source_document_id: 7070
payload_hash: "214bcc106b43c8f989afb0c67c84e70eb3933af5977d8252d62fece2725d4f54"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.750693+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Kinetic Carbine

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_chrono_kinetic_carbine`
- Snapshot ID: `39603`
- Source-Dokument: `7070`
- Kurzinfo: Kinetic Carbine aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 28.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AirMoveIncreasePercent": 20,
  "BehaviourBits": [
    "BehaviorDontBreakInvisibility",
    "BehaviorDontInterruptSprint",
    "BehaviorCastableWhileBusy",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "BonusBulletSpeed": 100,
  "BulletRadiusOverride": 16.0,
  "BulletTimeScale": 0.01,
  "ChannelMoveSpeed": -1,
  "ChargingModifier": {
    "Class": "ChronoKineticCarbine",
    "Subclass": "ChronoKineticCarbine"
  },
  "DebuffModifier": {
    "Class": "ChronoKineticCarbineSlow",
    "StatusEffectPriority": 50,
    "Subclass": "ChronoKineticCarbineSlow"
  },
  "HeadshotBonus": 14,
  "IsDisabled": false,
  "Key": "citadel_ability_chrono_kinetic_carbine",
  "MaxBonusBulletDamage": {
    "Scale": {
      "Type": "weapon_power",
      "Value": 125
    },
    "Value": 5
  },
  "MaxChargeDuration": 2.5,
  "MaxSlowDuration": 0.4,
  "MinBonusBulletDamage": {
    "Scale": {
      "Type": "weapon_power",
      "Value": 25
    },
    "Value": 5
  },
  "MinSlowDuration": 0.25,
  "MoveSpeedWhileShootingPenaltyReduction": 100,
  "Name": "Kinetic Carbine",
  "ProjectileTimeScale": 0.01,
  "ShotCount": 1,
  "SpeedBoostDuration": 3.5,
  "SpeedChange": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.13
    },
    "Value": 25
  },
  "TimeScaleDebuff": 90,
  "TimeWarpRadius": 5,
  "Upgrades": [
    {
      "MaxSlowDuration": 0.4
    },
    {
      "AbilityCooldown": -12,
      "SpeedChange": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.06
        },
        "Value": 0
      }
    },
    {
      "MaxBonusBulletDamage": {
        "Scale": {
          "Type": "weapon_power",
          "Value": 55
        },
        "Value": 0
      },
      "MinBonusBulletDamage": {
        "Scale": {
          "Type": "weapon_power",
          "Value": 55
        },
        "Value": 0
      },
      "SpeedBoostDuration": 2
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
    "card_name": "Kinetic Carbine",
    "hero_key": "hero_chrono",
    "hero_name": "Paradox",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_chrono",
      "hero_name": "Paradox",
      "lookup": "kinetic carbine",
      "name": "Kinetic Carbine",
      "type": "ability"
    }
  ]
}
````
