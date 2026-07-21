---
title: "Spectral Wall"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_fissure_wall"
canonical_name: "Spectral Wall"
snapshot_id: 39609
source_document_id: 7070
payload_hash: "3628dfd87b2e0baf5ebbc791a6e165a7e230b4718c6ba2a2e69ad47b448832bd"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.765254+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Spectral Wall

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_fissure_wall`
- Snapshot ID: `39609`
- Source-Dokument: `7070`
- Kurzinfo: Spectral Wall aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": 50,
  "AbilityCooldown": 50.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorPreventBotUsage"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.731203
    },
    "Value": 60
  },
  "IsDisabled": false,
  "Key": "citadel_ability_fissure_wall",
  "MinRange": 5,
  "Name": "Spectral Wall",
  "NumWallSegments": 8,
  "PushForce": 175,
  "SegmentEmitTime": 0.1,
  "SlowDuration": 2.5,
  "SlowModifier": {
    "Class": "SlowBase",
    "Subclass": "Slow"
  },
  "SlowPercent": 20,
  "TimeBetweenSegments": 0.035,
  "TimeToMaxDistance": 1.8,
  "Upgrades": [
    {
      "BonusDamagePercent": 20,
      "DebuffDuration": 7
    },
    {
      "AbilityCooldown": -20.0,
      "AbilityDuration": 2
    },
    {
      "CreateTurrets": 2,
      "SlowPercent": 30,
      "TurretLifeTime": 8
    }
  ],
  "WallImpactRange": 5,
  "WallModifier": {
    "Class": "FissureWall",
    "DebuffModifier": {
      "Class": "BonusDamagePercent",
      "Subclass": "BonusDamagePercent"
    },
    "EnemyVisionModifier": {
      "Class": "Base",
      "EnabledStateMask": [
        "GlowThroughWallsToEnemy"
      ],
      "Subclass": "EnemyVision"
    },
    "SentryDistanceFromWall": 80,
    "SlowModifier": {
      "Class": "SlowBase",
      "Subclass": "Slow"
    },
    "Subclass": "FissureWall"
  },
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
    "card_name": "Spectral Wall",
    "hero_key": "hero_forge",
    "hero_name": "McGinnis",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_forge",
      "hero_name": "McGinnis",
      "lookup": "spectral wall",
      "name": "Spectral Wall",
      "type": "ability"
    }
  ]
}
````
