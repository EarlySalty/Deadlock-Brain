---
title: "Revelation"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "operative_revelation"
canonical_name: "Revelation"
snapshot_id: 39866
source_document_id: 7071
payload_hash: "01cd76b71e9367c0aa7031725732a35a6204eb3ad6c212094f9838e7035158a7"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.357234+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Revelation

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `operative_revelation`
- Snapshot ID: `39866`
- Source-Dokument: `7071`
- Kurzinfo: Revelation aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.3
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 90.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 6
  },
  "DescKey": "operative_revelation_desc",
  "HeroKey": "hero_operative",
  "HeroName": "Raven",
  "Info1": {
    "Alt": [
      {
        "Key": "SlowPercent",
        "Name": "Move Speed",
        "Type": "slow",
        "Value": 25
      },
      {
        "Key": "TimeBeforeCursed",
        "Name": "Time Until Cursed",
        "Type": "duration",
        "Value": 2
      }
    ],
    "DescKey": "operative_revelation_desc",
    "Main": {
      "Props": [
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.651
          },
          "Type": "tech_damage",
          "Value": 50
        },
        {
          "Key": "Radius",
          "Name": "Radius",
          "Type": "distance",
          "Value": 15
        },
        {
          "Key": "CurseDuration",
          "Name": "Curse Duration",
          "Type": "duration",
          "Value": 3
        }
      ]
    }
  },
  "Key": "operative_revelation",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 4.2
    }
  },
  "Name": "Revelation",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "GroundDashReductionPercent": {
      "Name": "Dash Distance",
      "Value": -20
    },
    "MaxCameraAngleForSeeing": {
      "Name": null,
      "Value": 180
    },
    "MoveSpeedReduction": {
      "Name": null,
      "Value": 20
    },
    "TickRate": {
      "Name": null,
      "Value": 0.25
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 15
  },
  "Slot": "4",
  "Upgrades": [
    {
      "Radius": 5
    },
    {
      "AbilityCooldown": -25.0
    },
    {
      "DPS": 50
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
      "hero_key": "hero_operative",
      "hero_name": "Raven",
      "lookup": "revelation",
      "name": "Revelation",
      "type": "ability"
    }
  ]
}
````
