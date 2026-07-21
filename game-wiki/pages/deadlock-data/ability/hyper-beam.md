---
title: "Hyper Beam"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_bebop_laser_beam"
canonical_name: "Hyper Beam"
snapshot_id: 39596
source_document_id: 7070
payload_hash: "d6437d0431e9db8b0cd5a72679d2349f1f76cef1fdd67ddd7bda2fe51816a1d9"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.730864+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Hyper Beam

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_bebop_laser_beam`
- Snapshot ID: `39596`
- Source-Dokument: `7070`
- Kurzinfo: Hyper Beam aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 1.0,
  "AbilityChannelTime": 11,
  "AbilityCooldown": 120.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AirSpeedMax": 70,
  "BeamCloseDamagePercent": 75,
  "BeamCloseRadius": 5.0,
  "BeamEndRadius": 4.0,
  "BeamLength": 70,
  "BeamWidth": 2.9,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "ChannelMoveSpeed": 1.8,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 2.511
    },
    "Value": 160
  },
  "FallSpeedMax": 1,
  "GroundDashReductionPercent": -40,
  "Interval": 0.1,
  "IsDisabled": false,
  "Key": "citadel_ability_bebop_laser_beam",
  "Name": "Hyper Beam",
  "RestrictionModifier": {
    "Class": "SlowBase",
    "Subclass": "BebopLaserSlow"
  },
  "SlowPercent": 25,
  "SlowTargetDuration": 0.5,
  "TrackingSpeed": 55,
  "Upgrades": [
    {
      "AbilityCooldown": -20.0
    },
    {
      "DPS": 108.0
    },
    {
      "BeamLifesteal": 65,
      "BeamLifestealNonHeroPercent": 20
    }
  ],
  "ZoomBias": 0.5,
  "ZoomTime": 0.1,
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
    "card_name": "Hyper Beam",
    "hero_key": "hero_bebop",
    "hero_name": "Bebop",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_bebop",
      "hero_name": "Bebop",
      "lookup": "hyper beam",
      "name": "Hyper Beam",
      "type": "ability"
    }
  ]
}
````
