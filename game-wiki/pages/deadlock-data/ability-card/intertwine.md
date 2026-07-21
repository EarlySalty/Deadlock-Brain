---
title: "Intertwine"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_boho_damageshare"
canonical_name: "Intertwine"
snapshot_id: 39762
source_document_id: 7071
payload_hash: "ccb83b345702810eba89ffb23a06b11033d666108eb2163cdade32933776bab7"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.155586+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Intertwine

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_boho_damageshare`
- Snapshot ID: `39762`
- Source-Dokument: `7071`
- Kurzinfo: Intertwine aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 26.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 5
  },
  "DescKey": "ability_boho_damageshare_desc",
  "HeroKey": "hero_boho",
  "HeroName": "Boho",
  "Info1": {
    "Alt": [
      {
        "Key": "MaxLinks",
        "Name": "Max Links",
        "Type": "cast",
        "Value": 6
      }
    ],
    "DescKey": "ability_boho_damageshare_desc",
    "Main": {
      "Props": [
        {
          "Key": "DamageShareRadius",
          "Name": "Link Distance",
          "Type": "distance",
          "Value": 8
        },
        {
          "Key": "DamageSharePercentage",
          "Name": "Linked Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.186
          },
          "Type": "tech_damage",
          "Value": 30
        }
      ]
    }
  },
  "Key": "ability_boho_damageshare",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Intertwine",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "DebuffDuration": {
      "Name": "Debuff Duration",
      "Value": 6
    },
    "LinkDuration": {
      "Name": null,
      "Value": 0.5
    },
    "TickRate": {
      "Name": null,
      "Value": 0.25
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "DamageShareRadius": 3
    },
    {
      "AbilityDuration": 3
    },
    {
      "DamageSharePercentage": 22.5
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
      "hero_key": "hero_boho",
      "hero_name": "Boho",
      "lookup": "intertwine",
      "name": "Intertwine",
      "type": "ability"
    }
  ]
}
````
