---
title: "Stone Form"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_tengu_stone_form"
canonical_name: "Stone Form"
snapshot_id: 39653
source_document_id: 7070
payload_hash: "f54e8b253dfc6b91a68bd405f3905a91eb8653719c9311e579217b7848ae6473"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.870472+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Stone Form

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_tengu_stone_form`
- Snapshot ID: `39653`
- Source-Dokument: `7070`
- Kurzinfo: Stone Form aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.25,
  "AbilityCooldown": 40.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 3,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorCastableWhileBusy",
    "BehaviorInterruptMeleeOnCast",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.6
    },
    "Value": 75
  },
  "DampingFactor": 0.25,
  "DragModifier": {
    "Class": "ChargeDragEnemy",
    "ForceDistScale": 10,
    "ForwardOffset": 0,
    "Subclass": "ChargeDragEnemy",
    "VerticalOffset": -200
  },
  "IsDisabled": false,
  "Key": "citadel_ability_tengu_stone_form",
  "LiftHeight": 180,
  "LiftTime": 1.0,
  "MaxHealthRegen": 6,
  "MoveSpeedMax": 8,
  "Name": "Stone Form",
  "Radius": 6,
  "StunDuration": 0.75,
  "Upgrades": [
    {
      "MaxHealthRegen": 7.0
    },
    {
      "AbilityCooldown": -25
    },
    {
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.7
        },
        "Value": 0
      },
      "StunDuration": 1
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
    "card_name": "Stone Form",
    "hero_key": "hero_tengu",
    "hero_name": "Ivy",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_tengu",
      "hero_name": "Ivy",
      "lookup": "stone form",
      "name": "Stone Form",
      "type": "ability"
    }
  ]
}
````
