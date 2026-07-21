---
title: "Singularity"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_self_vacuum"
canonical_name: "Singularity"
snapshot_id: 39786
source_document_id: 7071
payload_hash: "b82601e49b920f5ae519aa65e183f57aa9fe68021bb28d0f53542e3adde6fbd8"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.201540+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Singularity

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_self_vacuum`
- Snapshot ID: `39786`
- Source-Dokument: `7071`
- Kurzinfo: Singularity aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

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
    "Value": 2.75
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 265.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_self_vacuum_desc",
  "HeroKey": "hero_dynamo",
  "HeroName": "Dynamo",
  "Info1": {
    "Alt": [
      {
        "Key": "VacuumRadius",
        "Name": "Singularity Radius",
        "Type": "distance",
        "Value": 7
      },
      {
        "Key": "AbilityChannelTime",
        "Name": "Channel Duration",
        "Type": "cast",
        "Value": 2.75
      }
    ],
    "DescKey": "citadel_ability_self_vacuum_desc",
    "Main": {
      "Props": [
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.28
          },
          "Type": "tech_damage",
          "Value": 75
        },
        {
          "Key": "DPSPercentHealth",
          "Name": "Max Health as Damage",
          "Type": "tech_damage",
          "Value": 0.0
        }
      ]
    }
  },
  "Key": "citadel_ability_self_vacuum",
  "Name": "Singularity",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CameraDistance": {
      "Name": null,
      "Value": 400
    },
    "Speed": {
      "Name": null,
      "Value": 5.08
    },
    "TickRate": {
      "Name": null,
      "Value": 0.25
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
      "VacuumRadius": 2
    },
    {
      "AbilityChannelTime": 0.75
    },
    {
      "DPSPercentHealth": 6
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
      "hero_key": "hero_dynamo",
      "hero_name": "Dynamo",
      "lookup": "singularity",
      "name": "Singularity",
      "type": "ability"
    }
  ]
}
````
