---
title: "Project Mind"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_projectmind"
canonical_name: "Project Mind"
snapshot_id: 39634
source_document_id: 7070
payload_hash: "bfe4a8e201ce01f84c6d361806aff170483f2c1d50c4068d49f05340db45b4e8"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.825225+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Project Mind

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_projectmind`
- Snapshot ID: `39634`
- Source-Dokument: `7070`
- Kurzinfo: Project Mind aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.75,
  "AbilityCastRange": 25,
  "AbilityCooldown": 46.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorPreventBotUsage",
    "BehaviorMovement",
    "BehaviorCanSetQuickCast"
  ],
  "CameraDistance": 250,
  "ChannelMoveSpeed": 5.1,
  "CombatBarrier": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 0
  },
  "IsDisabled": false,
  "Key": "citadel_ability_projectmind",
  "Name": "Project Mind",
  "ProjectMindModifier": {
    "Class": "CitadelProjectmind",
    "ShieldModifier": {
      "Class": "WraithProjectMindShield",
      "Subclass": "WraithProjectMindShield"
    },
    "Subclass": "CitadelProjectmind"
  },
  "TrailInterval": 0.1,
  "Upgrades": [
    {
      "AbilityCastRange": 15
    },
    {
      "BarrierDuration": 5,
      "CombatBarrier": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.7
        },
        "Value": 300
      }
    },
    {
      "AbilityCooldown": -32.0
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
    "card_name": "Project Mind",
    "hero_key": "hero_wraith",
    "hero_name": "Wraith",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_wraith",
      "hero_name": "Wraith",
      "lookup": "project mind",
      "name": "Project Mind",
      "type": "ability"
    }
  ]
}
````
