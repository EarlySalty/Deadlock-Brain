---
title: "Rocket Launcher"
entity_type: "ability"
source: "deadlock_data"
external_id: "rutger_rocket"
canonical_name: "Rocket Launcher"
snapshot_id: 39704
source_document_id: 7070
payload_hash: "42d1ecb84c863f768b3f9d68c2a06b9e6e2a731e3d514d53762a3459a0d5bcb1"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.999753+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Rocket Launcher

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `rutger_rocket`
- Snapshot ID: `39704`
- Source-Dokument: `7070`
- Kurzinfo: Rocket Launcher aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCharges": 1,
  "AbilityCooldown": 17.0,
  "AbilityCooldownBetweenCharge": 1,
  "AbilityUnitTargetLimit": 1,
  "AirSpeedMax": 3.81,
  "AutoChannelModifier": {
    "Class": "IntrinsicBase",
    "Subclass": "IntrinsicBase"
  },
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDontTriggerSpellBlock",
    "BehaviorDisplaysDamageImpact"
  ],
  "CameraHeightOffset": 20,
  "CameraHorizontalOffset": 15,
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 120
  },
  "FallSpeedMax": 1.524,
  "ImpactRadius": 5,
  "IsDisabled": false,
  "Key": "rutger_rocket",
  "LaunchMaxSpeed": 600,
  "LaunchMinSpeed": 525,
  "LaunchMinVerticalAmount": 0.2,
  "LaunchVerticalBias": 0.75,
  "Name": "Rocket Launcher",
  "SelfDamagePercent": 50,
  "SelfLaunchPercent": 175,
  "TechCleaveExpireTime": 0.2,
  "Upgrades": [
    {
      "AbilityCharges": 1
    },
    {
      "Damage": 80
    },
    {
      "AbilityCooldown": -7.5
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_rutger",
      "hero_name": "Rutger",
      "lookup": "rocket launcher",
      "name": "Rocket Launcher",
      "type": "ability"
    }
  ]
}
````
