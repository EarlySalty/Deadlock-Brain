---
title: "Haunting Spectres"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_necro_haunt"
canonical_name: "Haunting Spectres"
snapshot_id: 39497
source_document_id: 7070
payload_hash: "813da49207faf2bc89fe92c0480567be1c647762e6ee1575b3172a79d8ee20e3"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.481788+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Haunting Spectres

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_necro_haunt`
- Snapshot ID: `39497`
- Source-Dokument: `7070`
- Kurzinfo: Haunting Spectres aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 35,
  "AbilityCooldown": 22,
  "AbilityDuration": 5,
  "AbilityPostCastDuration": 0.2,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectilePassThroughWorld",
    "BehaviorAllowSelfCast",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorAllowAltCast",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorTriggerCancelMashProtectionOnCast",
    "BehaviorCanSetQuickCast"
  ],
  "BonusMoveSpeed": 3,
  "BuffModifier": {
    "Class": "NecroHauntingspirits",
    "Subclass": "Buff"
  },
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.3
    },
    "Value": 25
  },
  "DebuffDuration": 3,
  "DebuffModifier": {
    "Class": "DiminishingSlow",
    "Subclass": "Slow"
  },
  "DetonationRange": 1,
  "HauntCount": 3,
  "HoverRadius": 0.75,
  "IsDisabled": false,
  "Key": "ability_necro_haunt",
  "Name": "Haunting Spectres",
  "PhysicsCurlNoiseFrequency": 0.005,
  "PhysicsCurlNoiseStrength": 0.6,
  "PhysicsDamperStrength": 5,
  "PhysicsSpinPerSecond": 180,
  "PhysicsSpringStrength": 150,
  "SlowPercent": 30,
  "SpawnRadius": 0.2,
  "TargetSearchDelayOnSuccess": 0.3,
  "TargetSearchInitialDelay": 0.7,
  "TargetSearchRadius": 6,
  "TargetSearchRadiusVsHeroes": 15,
  "TargetSearchTick": 0.1,
  "TickRate": 0.1,
  "Upgrades": [
    {
      "Damage": 16
    },
    {
      "FireRateSlow": 30
    },
    {
      "HauntCount": 3
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
