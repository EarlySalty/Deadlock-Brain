---
title: "Arctic Beam"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_icebeam"
canonical_name: "Arctic Beam"
snapshot_id: 39463
source_document_id: 7070
payload_hash: "e77fe031c665eca836afb044961b1e85ce9090bd1c5547911f383a0870e9edf4"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.389298+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Arctic Beam

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_icebeam`
- Snapshot ID: `39463`
- Source-Dokument: `7070`
- Kurzinfo: Arctic Beam aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 28.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5.0,
  "AbilityUnitTargetLimit": 1,
  "BeamSplit": {
    "Scale": {
      "Type": "range",
      "Value": 0.0
    },
    "Value": 0
  },
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDontAimFacingEnemy",
    "BehaviorRequireAbilityButtonToCancel"
  ],
  "BuildupModifier": {
    "BuildUpDecayDelay": 0.4,
    "Class": "IcebeamStackingSlow",
    "EnabledStateMask": [
      "Slowed"
    ],
    "StatusEffectPriority": 50,
    "Subclass": "IcebeamStackingSlow"
  },
  "CameraDistance": 250,
  "ChannelMoveSpeed": -1,
  "ChannelSlowPercent": 8,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.38
    },
    "Value": 45.0
  },
  "IceBeamBuildupProcDuration": 2,
  "IceBeamModifier": {
    "Class": "Base",
    "Subclass": "Icebeaming"
  },
  "IsDisabled": false,
  "Key": "ability_icebeam",
  "MaxFireRateSlowPercent": 20,
  "MaxGroundDashReductionPercent": -20,
  "MaxSlowPercent": 20,
  "MaxSlowTime": 2.0,
  "MinSlowPercent": 30,
  "Name": "Arctic Beam",
  "PathLength": 25,
  "PathWidth": 1.1,
  "SlowDuration": 0.6,
  "TickRate": 0.1,
  "Upgrades": [
    {
      "MaxFireRateSlowPercent": 25,
      "MaxSlowPercent": 25
    },
    {
      "DPS": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.6
        },
        "Value": 20
      }
    },
    {
      "AbilityCooldown": -13,
      "BeamSplit": {
        "Scale": {
          "Type": "range",
          "Value": 0.93
        },
        "Value": 10
      },
      "BeamSplitCount": 2
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
    "card_name": "Arctic Beam",
    "hero_key": "hero_kelvin",
    "hero_name": "Kelvin",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_kelvin",
      "hero_name": "Kelvin",
      "lookup": "arctic beam",
      "name": "Arctic Beam",
      "type": "ability"
    }
  ]
}
````
