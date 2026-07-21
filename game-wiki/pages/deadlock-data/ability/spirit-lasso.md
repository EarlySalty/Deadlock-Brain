---
title: "Spirit Lasso"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_gravity_lasso"
canonical_name: "Spirit Lasso"
snapshot_id: 39452
source_document_id: 7070
payload_hash: "019bce376b390070492d3f73bcc834821f216503c842a28ffcdde3fb06c65fd1"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.358957+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Spirit Lasso

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_gravity_lasso`
- Snapshot ID: `39452`
- Source-Dokument: `7070`
- Kurzinfo: Spirit Lasso aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.5,
  "AbilityCastRange": 20,
  "AbilityCooldown": 130,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 2.25,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorCleaveDisabled",
    "BehaviorShowCastRangeAsSatSphereWhileCasting",
    "BehaviorDontInterruptSlideOnCast",
    "BehaviorInterruptMeleeOnCast"
  ],
  "BouncePadExtendDuration": 1.0,
  "CameraPreviewDistance": 200,
  "CameraPreviewOffset": 25,
  "CameraPreviewSpeed": 0.6,
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.93
    },
    "Value": 80
  },
  "ExtraTargetConeAngle": 60,
  "ExtraTargetHorizontalOffset": 30,
  "FollowDampingFactor": 8,
  "FollowDistance": 60,
  "GrabExtraTargetsRadiusMult": 2,
  "IsDisabled": false,
  "Key": "ability_gravity_lasso",
  "LassoTargetMaxSpeed": 55,
  "LiftHeight": 7,
  "LiftHorizontal": -30,
  "LiftInitialDelay": 0.5,
  "LiftInitialRisingSpeed": 100,
  "LiftInitialVelocityStart": 500,
  "Name": "Spirit Lasso",
  "Upgrades": [
    {
      "Damage": 80
    },
    {
      "AbilityDuration": 0.75
    },
    {
      "AbilityCooldown": -40
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
    "card_name": "Spirit Lasso",
    "hero_key": "hero_astro",
    "hero_name": "Holliday",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_astro",
      "hero_name": "Holliday",
      "lookup": "spirit lasso",
      "name": "Spirit Lasso",
      "type": "ability"
    }
  ]
}
````
