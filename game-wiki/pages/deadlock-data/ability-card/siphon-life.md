---
title: "Siphon Life"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_bull_heal"
canonical_name: "Siphon Life"
snapshot_id: 39752
source_document_id: 7071
payload_hash: "25718fc4eb6e24cca97da6d9a3e277fbecdf57ec635fab6157f2b09bcbc5be97"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.133920+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Siphon Life

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_bull_heal`
- Snapshot ID: `39752`
- Source-Dokument: `7071`
- Kurzinfo: Siphon Life aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 42.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 4
  },
  "DescKey": "citadel_ability_bull_heal_desc",
  "HeroKey": "hero_atlas",
  "HeroName": "Abrams",
  "Info1": {
    "Alt": [
      {
        "Key": "NonHeroHealingFactor",
        "Name": "Lifesteal vs Non-Heroes",
        "Type": "healing",
        "Value": 35
      }
    ],
    "DescKey": "citadel_ability_bull_heal_desc",
    "Main": {
      "Props": [
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.6
          },
          "Type": "tech_damage",
          "Value": 22
        },
        {
          "Key": "AbilityDuration",
          "Name": "Duration",
          "Type": "duration",
          "Value": 4
        },
        {
          "Key": "HealingFactor",
          "Name": "Lifesteal",
          "Title": "On Hit:",
          "Type": "healing",
          "Value": 70
        }
      ]
    }
  },
  "Key": "citadel_ability_bull_heal",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Siphon Life",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "TickRate": {
      "Name": null,
      "Value": 0.25
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 8
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": -20.0
    },
    {
      "AbilityDuration": 2
    },
    {
      "DPS": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.12
        },
        "Value": 18
      },
      "DescKey": "citadel_ability_bull_heal_t3_desc",
      "Radius": 2
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
      "hero_key": "hero_atlas",
      "hero_name": "Abrams",
      "lookup": "siphon life",
      "name": "Siphon Life",
      "type": "ability"
    }
  ]
}
````
