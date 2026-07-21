---
title: "Hellfire Salvo"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_gunslinger_demon_carbine"
canonical_name: "Hellfire Salvo"
snapshot_id: 39454
source_document_id: 7070
payload_hash: "d9fe10ca4f65c127a5972b40b0eef412e73a2f0213ba44c2191795ea21f31f15"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.364513+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Hellfire Salvo

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_gunslinger_demon_carbine`
- Snapshot ID: `39454`
- Source-Dokument: `7070`
- Kurzinfo: Hellfire Salvo aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 30.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BaseBulletDamage": {
    "Scale": {
      "Type": "damage",
      "Value": 1.0
    },
    "Value": 5
  },
  "BehaviourBits": [
    "BehaviorDontBreakInvisibility",
    "BehaviorDontInterruptSprint",
    "BehaviorCastableWhileBusy",
    "BehaviorDontInterruptMeleeOnCast",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "BonusBulletSpeed": 100,
  "BulletRadiusOverride": 13.7,
  "BulletTimeScale": 0.01,
  "ChannelMoveSpeed": -1,
  "ChargingModifier": {
    "Class": "GunslingerDemonCarbine",
    "Subclass": "GunslingerDemonCarbine"
  },
  "DebuffModifier": {
    "Class": "ChronoKineticCarbineSlow",
    "StatusEffectPriority": 50,
    "Subclass": "ChronoKineticCarbineSlow"
  },
  "DemonShotCount": 3,
  "HeadshotBonus": 15,
  "IsDisabled": false,
  "Key": "ability_gunslinger_demon_carbine",
  "MaxChargeDuration": 2.5,
  "MoveSpeedWhileShootingPenaltyReduction": 100,
  "Name": "Hellfire Salvo",
  "ProcDamagePercentage": 400,
  "SpeedChange": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0914
    },
    "Value": 25
  },
  "Upgrades": [
    {
      "AbilityCooldown": -15
    }
  ],
  "WeaponReadyDuration": 5,
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
