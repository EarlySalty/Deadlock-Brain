---
title: "Consume"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_wrecker_salvage"
canonical_name: "Consume"
snapshot_id: 39948
source_document_id: 7071
payload_hash: "8988cd08724d5afd855e541a02d6234509dc1bc1398dfcd12c8c29100a6b0e5b"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.514757+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Consume

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_wrecker_salvage`
- Snapshot ID: `39948`
- Source-Dokument: `7071`
- Kurzinfo: Consume aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 15
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 12.5
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_wrecker_salvage_desc",
  "HeroKey": "hero_wrecker",
  "HeroName": "Wrecker",
  "Info1": {
    "Alt": [
      {
        "Key": "SalvageDuration",
        "Name": "Max Duration",
        "Type": "duration",
        "Value": 4
      },
      {
        "Key": "MaxRange",
        "Name": "Max Tether Range",
        "Type": "distance",
        "Value": 20
      }
    ],
    "DescKey": "ability_wrecker_salvage_desc",
    "Main": {
      "Props": [
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.974938
          },
          "Type": "tech_damage",
          "Value": 60
        },
        {
          "Key": "SalvageBonus_FireRate",
          "Name": "Fire Rate per bonus",
          "Type": "bullet_damage",
          "Value": 0
        },
        {
          "Key": "ConsumeHealPercentage",
          "Name": "Consume Heal Percentage",
          "Type": "healing",
          "Value": 50
        }
      ]
    }
  },
  "Key": "ability_wrecker_salvage",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 3.8
    }
  },
  "Name": "Consume",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "TickInterval": {
      "Name": null,
      "Value": 0.25
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "ConsumeHealPercentage": 25
    },
    {
      "DPS": 40
    },
    {
      "AbilityUnitTargetLimit": 2,
      "DescKey": "ability_wrecker_salvage_t3_desc"
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
      "lookup": "consume",
      "name": "Consume",
      "type": "ability"
    }
  ]
}
````
