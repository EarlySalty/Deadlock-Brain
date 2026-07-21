---
title: "Overload"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_wrecker_garbage_suck"
canonical_name: "Overload"
snapshot_id: 39950
source_document_id: 7071
payload_hash: "5def247d3a3ce91e974080b6050c61977a21c6d91f836986b1f1befc8ecde4af"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.518366+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Overload

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_wrecker_garbage_suck`
- Snapshot ID: `39950`
- Source-Dokument: `7071`
- Kurzinfo: Overload aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 3
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 130
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "Damage": {
    "DPS": {
      "Name": "Damage Per Second",
      "Scale": {
        "Type": "spirit",
        "Value": 0.194988
      },
      "Type": "tech_damage",
      "Value": 0
    }
  },
  "DescKey": "citadel_ability_wrecker_garbage_suck_desc",
  "HeroKey": "hero_wrecker",
  "HeroName": "Wrecker",
  "Info1": {
    "Alt": [
      {
        "Key": "GarbageRadius",
        "Name": "Overload Radius",
        "Type": "distance",
        "Value": 12
      },
      {
        "Key": "SlowPercent",
        "Name": "Move Speed",
        "Type": "slow",
        "Value": 50
      }
    ],
    "DescKey": "citadel_ability_wrecker_garbage_suck_desc",
    "Main": {
      "Props": [
        {
          "Key": "BaseDamage",
          "Name": "Base Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.93
          },
          "Type": "tech_damage",
          "Value": 75
        },
        {
          "Key": "DamagePerSecond",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 2.604
          },
          "Type": "tech_damage",
          "Value": 100
        },
        {
          "Key": "AbilityChannelTime",
          "Name": "Channel Duration",
          "Type": "cast",
          "Value": 3
        }
      ]
    }
  },
  "Key": "citadel_ability_wrecker_garbage_suck",
  "Name": "Overload",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CameraDistance": {
      "Name": null,
      "Value": 800
    },
    "Speed": {
      "Name": null,
      "Value": 5.08
    },
    "TickRate": {
      "Name": null,
      "Value": 1
    },
    "TossAngle": {
      "Name": null,
      "Value": 45
    },
    "TossSpeed": {
      "Name": null,
      "Value": 8.89
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "GarbageRadius": 2
    },
    {
      "AbilityCooldown": -35
    },
    {
      "BaseDamage": 100,
      "DamagePerSecond": 50
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
      "hero_key": "hero_wrecker",
      "hero_name": "Wrecker",
      "lookup": "overload",
      "name": "Overload",
      "type": "ability"
    }
  ]
}
````
