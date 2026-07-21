---
title: "Overload"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_wrecker_garbage_suck"
canonical_name: "Overload"
snapshot_id: 39667
source_document_id: 7070
payload_hash: "09bf67107e88d526b8c9369d1fc2d03c8abe1f53d309643940ad4e2aa183698a"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.902441+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Overload

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_wrecker_garbage_suck`
- Snapshot ID: `39667`
- Source-Dokument: `7070`
- Kurzinfo: Overload aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityChannelTime": 3,
  "AbilityCooldown": 130,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BaseDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.93
    },
    "Value": 75
  },
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorExclusiveUse",
    "BehaviorCastableWhileBusy",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "CameraDistance": 800,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.194988
    },
    "Value": 0
  },
  "DamagePerSecond": {
    "Scale": {
      "Type": "spirit",
      "Value": 2.604
    },
    "Value": 100
  },
  "GarbageAuraModifier": {
    "Class": "Garbageaura",
    "ProvidedByAura": {
      "Class": "Base",
      "OuterSpeedScale": 10.0,
      "StatusEffectPriority": 60,
      "Subclass": "GarbageauraTarget"
    },
    "StatusEffectPriority": 0,
    "Subclass": "Garbageaura"
  },
  "GarbageRadius": 12,
  "IsDisabled": false,
  "Key": "citadel_ability_wrecker_garbage_suck",
  "Name": "Overload",
  "SlowPercent": 50,
  "Speed": 5.08,
  "TickRate": 1,
  "TossAngle": 45,
  "TossSpeed": 8.89,
  "Upgrades": [
    {
      "GarbageRadius": 2
    },
    {
      "AbilityCooldown": -35
    },
    {
      "BaseDamage": 100,
      "DamagePerSecond": 50
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
    "card_name": "Overload",
    "hero_key": "hero_wrecker",
    "hero_name": "Wrecker",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_wrecker",
      "hero_name": "Wrecker",
      "lookup": "overload",
      "name": "Overload",
      "type": "ability"
    }
  ]
}
````
