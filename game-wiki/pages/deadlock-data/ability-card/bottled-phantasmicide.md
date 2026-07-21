---
title: "Bottled Phantasmicide"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_trapper_poisonjar"
canonical_name: "Bottled Phantasmicide"
snapshot_id: 39907
source_document_id: 7071
payload_hash: "3d505bcc78707cdbcc76a2bd25e1819b2054f566d4b4d726cf1f6dfeade43a24"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.435053+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Bottled Phantasmicide

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_trapper_poisonjar`
- Snapshot ID: `39907`
- Source-Dokument: `7071`
- Kurzinfo: Bottled Phantasmicide aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 30
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 8
  },
  "DescKey": "ability_trapper_poisonjar_desc",
  "HeroKey": "hero_trapper",
  "HeroName": "Trapper",
  "Info1": {
    "Alt": [
      {
        "Key": "InitialRadius",
        "Name": "Initial Radius",
        "Type": "distance",
        "Value": 6
      },
      {
        "Key": "RadiusPerSecond",
        "Name": "Radius Growth",
        "Type": "distance",
        "Value": 0.25
      }
    ],
    "DescKey": "ability_trapper_poisonjar_desc",
    "Main": {
      "Props": [
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Scale": {
            "Type": "spirit",
            "Value": 0.279
          },
          "Type": "slow",
          "Value": 25
        },
        {
          "Key": "AbilityDuration",
          "Name": "Duration",
          "Type": "duration",
          "Value": 8
        }
      ]
    }
  },
  "Key": "ability_trapper_poisonjar",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Bottled Phantasmicide",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "Height": {
      "Name": null,
      "Value": 2
    },
    "TickRate": {
      "Name": null,
      "Value": 0.25
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "SlowPercent": 20
    },
    {
      "AbilityDuration": 4
    },
    {
      "DescKey": "ability_trapper_poisonjar_t3_desc",
      "TechArmorDamageReduction": -25
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
      "hero_key": "hero_trapper",
      "hero_name": "Trapper",
      "lookup": "bottled phantasmicide",
      "name": "Bottled Phantasmicide",
      "type": "ability"
    }
  ]
}
````
