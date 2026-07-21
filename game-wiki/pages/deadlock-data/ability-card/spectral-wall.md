---
title: "Spectral Wall"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_fissure_wall"
canonical_name: "Spectral Wall"
snapshot_id: 39797
source_document_id: 7071
payload_hash: "2f88a7ed36c6e449f372f3828d7b957e3cb58b04d711f8ccf7f4c899699c8aff"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.223692+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Spectral Wall

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_fissure_wall`
- Snapshot ID: `39797`
- Source-Dokument: `7071`
- Kurzinfo: Spectral Wall aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 50
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 50.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 6.0
  },
  "DescKey": "citadel_ability_fissure_wall_desc",
  "HeroKey": "hero_forge",
  "HeroName": "McGinnis",
  "Info1": {
    "Alt": [
      {
        "Key": "MinRange",
        "Name": "Minimum Range",
        "Type": "distance",
        "Value": 5
      },
      {
        "Key": "WallImpactRange",
        "Name": "Impact Range",
        "Type": "distance",
        "Value": 5
      },
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Type": "duration",
        "Value": 2.5
      },
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 0
      }
    ],
    "DescKey": "citadel_ability_fissure_wall_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.731203
          },
          "Type": "tech_damage",
          "Value": 60
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Type": "slow",
          "Value": 20
        },
        {
          "Key": "WallStunDuration",
          "Name": "Stun Duration",
          "StatusEffect": "Stun",
          "Type": "duration",
          "Value": 0
        }
      ]
    }
  },
  "Key": "citadel_ability_fissure_wall",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Spectral Wall",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "NumWallSegments": {
      "Name": null,
      "Value": 8
    },
    "PushForce": {
      "Name": null,
      "Value": 175
    },
    "SegmentEmitTime": {
      "Name": null,
      "Value": 0.1
    },
    "TimeBetweenSegments": {
      "Name": null,
      "Value": 0.035
    },
    "TimeToMaxDistance": {
      "Name": null,
      "Value": 1.8
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "BonusDamagePercent": 20,
      "DebuffDuration": 7,
      "DescKey": "citadel_ability_fissure_wall_t1_desc"
    },
    {
      "AbilityCooldown": -20.0,
      "AbilityDuration": 2,
      "DescKey": "citadel_ability_fissure_wall_t2_desc"
    },
    {
      "CreateTurrets": 2,
      "DescKey": "citadel_ability_fissure_wall_t3_desc",
      "SlowPercent": 30,
      "TurretLifeTime": 8
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
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
      "hero_key": "hero_forge",
      "hero_name": "McGinnis",
      "lookup": "spectral wall",
      "name": "Spectral Wall",
      "type": "ability"
    }
  ]
}
````
