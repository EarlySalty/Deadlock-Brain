---
title: "Spectral Silk"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_trapper_fear"
canonical_name: "Spectral Silk"
snapshot_id: 39543
source_document_id: 7070
payload_hash: "7f8ff2eda3ff9a324a33787bc9f4d1b8cd44024e51c8fe9e5206290950d1fbf5"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.591707+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Spectral Silk

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_trapper_fear`
- Snapshot ID: `39543`
- Source-Dokument: `7070`
- Kurzinfo: Spectral Silk aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.3,
  "AbilityUnitTargetLimit": 1,
  "AutoIntrinsicModifiers": [
    {
      "BuildUpModifier": {
        "BuildUpDecayDelay": 3.0,
        "Class": "CitadelBaseBuildup",
        "Subclass": "FearBuildup"
      },
      "BuildupProcModifier": {
        "Class": "TrapperImmobilize",
        "EnabledStateMask": [
          "Immobilized",
          "GlowThroughWallsToEnemy",
          "GlowThroughWallsToProvider",
          "GlowToProvider"
        ],
        "StatusEffectPriority": 0,
        "Subclass": "TrapperImmobilize"
      },
      "Class": "FearWatcher",
      "Subclass": "FearWatcher"
    }
  ],
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectileFiredAsBullet"
  ],
  "BuildUpDuration": 15,
  "BuildupProcDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.488
    },
    "Value": 120
  },
  "BuildupProcDuration": 2,
  "BuildupSpiritDamageThreshold": 200,
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_trapper_fear",
  "Name": "Spectral Silk",
  "SlowPercent": 50,
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
  }
}
````
