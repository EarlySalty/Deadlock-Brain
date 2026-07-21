---
title: "Charged Shot"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_charged_shot"
canonical_name: "Charged Shot"
snapshot_id: 39416
source_document_id: 7070
payload_hash: "7b2e3663af9355d8d943d742244dbbf5aee67fa7089d868fc02bfa89fb8ef1bb"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.267228+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Charged Shot

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_charged_shot`
- Snapshot ID: `39416`
- Source-Dokument: `7070`
- Kurzinfo: Charged Shot aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.5,
  "AbilityChannelTime": 9999,
  "AbilityCharges": 1,
  "AbilityCooldown": 17.0,
  "AbilityCooldownBetweenCharge": 4,
  "AbilityUnitTargetLimit": 1,
  "AirSpeedMax": 4.1,
  "AutoChannelModifier": {
    "Class": "IntrinsicBase",
    "Subclass": "IntrinsicBase"
  },
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorChannelled",
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "CameraHeightOffset": 20,
  "CameraHorizontalOffset": 15,
  "ChannelMoveSpeed": 1.5,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.0
    },
    "Value": 80
  },
  "FallSpeedMax": 1.524,
  "IsDisabled": false,
  "Key": "ability_charged_shot",
  "Name": "Charged Shot",
  "TechCleaveExpireTime": 0.2,
  "Upgrades": [
    {
      "AbilityCharges": 1
    },
    {
      "Damage": 54.0
    },
    {
      "AbilityCooldownBetweenCharge": -3,
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.0
        },
        "Value": 0
      }
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
    "card_name": "Charged Shot",
    "hero_key": "hero_orion",
    "hero_name": "Grey Talon",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_orion",
      "hero_name": "Grey Talon",
      "lookup": "charged shot",
      "name": "Charged Shot",
      "type": "ability"
    }
  ]
}
````
