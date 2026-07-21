---
title: "Slither"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_viper_snakedash"
canonical_name: "Slither"
snapshot_id: 39925
source_document_id: 7071
payload_hash: "8f17ba425c389faf6af42ce20425ff13a2521057ca29fca9b04f96c74a3b1faa"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.471125+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Slither

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_viper_snakedash`
- Snapshot ID: `39925`
- Source-Dokument: `7071`
- Kurzinfo: Slither aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_viper_snakedash_desc",
  "HeroKey": "hero_viper",
  "HeroName": "Vyper",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_viper_snakedash_desc",
    "Main": {
      "Props": [
        {
          "Key": "SlideScale",
          "Name": "Slide Distance",
          "Type": "move_speed",
          "Value": 15
        },
        {
          "Key": "CombatBarrier",
          "Name": "Barrier",
          "Scale": {
            "Type": "spirit",
            "Value": 0.0
          },
          "Title": "On Slide:",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_viper_snakedash",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Slither",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "SlideScale": 20
    },
    {
      "Stamina": 2
    },
    {
      "AbilityCooldown": 8,
      "BuffDuration": 5,
      "CombatBarrier": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.8
        },
        "Value": 180
      },
      "DescKey": "ability_viper_snakedash_t3_desc"
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
      "hero_key": "hero_viper",
      "hero_name": "Vyper",
      "lookup": "slither",
      "name": "Slither",
      "type": "ability"
    }
  ]
}
````
