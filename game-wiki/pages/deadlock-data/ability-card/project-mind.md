---
title: "Project Mind"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_projectmind"
canonical_name: "Project Mind"
snapshot_id: 39944
source_document_id: 7071
payload_hash: "755e247a335ae3271d1e8cb8054bda93d0133421154c04501ec65a731aa482d1"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.506967+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Project Mind

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_projectmind`
- Snapshot ID: `39944`
- Source-Dokument: `7071`
- Kurzinfo: Project Mind aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.75
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 25
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 46.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_projectmind_desc",
  "HeroKey": "hero_wraith",
  "HeroName": "Wraith",
  "Info1": {
    "Alt": [],
    "DescKey": "citadel_ability_projectmind_desc",
    "Main": {}
  },
  "Info2": {
    "Alt": [
      {
        "Key": "BarrierDuration",
        "Name": "Barrier Duration",
        "Type": "duration",
        "Value": 0
      }
    ],
    "Main": {
      "Props": [
        {
          "Key": "AbilityCastRange",
          "Name": "Cast Range",
          "Title": "Barrier:",
          "Type": "range",
          "Value": 25
        },
        {
          "Key": "CombatBarrier",
          "Name": "Barrier",
          "Scale": {
            "Type": "spirit",
            "Value": 0.0
          },
          "Title": "Barrier:",
          "Type": "bullet_armor_up",
          "Value": 0
        }
      ]
    }
  },
  "Key": "citadel_ability_projectmind",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 5.1
    }
  },
  "Name": "Project Mind",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CameraDistance": {
      "Name": null,
      "Value": 250
    },
    "TrailInterval": {
      "Name": null,
      "Value": 0.1
    }
  },
  "Slot": "2",
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
      },
      "DescKey": "citadel_ability_projectmind_t2_desc"
    },
    {
      "AbilityCooldown": -32.0
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
      "hero_key": "hero_wraith",
      "hero_name": "Wraith",
      "lookup": "project mind",
      "name": "Project Mind",
      "type": "ability"
    }
  ]
}
````
