---
title: "Spectral Assistant"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_magician_cloneturret"
canonical_name: "Spectral Assistant"
snapshot_id: 39475
source_document_id: 7070
payload_hash: "768383d3442ebb4aeb772c03ec7dc0c7255743383106ce7a8f4820afbe9a306d"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.429102+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Spectral Assistant

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_magician_cloneturret`
- Snapshot ID: `39475`
- Source-Dokument: `7070`
- Kurzinfo: Spectral Assistant aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCastRange": 15,
  "AbilityCooldown": 40,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorAlwaysPreviewRadius",
    "BehaviorPreventBotUsage",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BuffModifier": {
    "Class": "Base",
    "Duration": -1.0,
    "Subclass": "CloneturretBuffModifier"
  },
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.36
    },
    "Value": 15
  },
  "IsDisabled": false,
  "Key": "ability_magician_cloneturret",
  "LeashRadius": 20,
  "Name": "Spectral Assistant",
  "TotalSwaps": 2,
  "TurretBulletTargetAngle": 20,
  "TurretBulletTargetRadius": 500,
  "TurretBulletVerticalOffset": 2,
  "Upgrades": [
    {
      "AbilityCooldown": -10
    },
    {
      "AbilityCastRange": 5,
      "AbilityDuration": 7,
      "LeashRadius": 5
    },
    {
      "BonusFireRate": 60,
      "Damage": 12.6
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
    "card_name": "Spectral Assistant",
    "hero_key": "hero_magician",
    "hero_name": "Sinclair",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_magician",
      "hero_name": "Sinclair",
      "lookup": "spectral assistant",
      "name": "Spectral Assistant",
      "type": "ability"
    }
  ]
}
````
