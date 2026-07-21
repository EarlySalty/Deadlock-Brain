---
title: "Flying Slash"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_flying_strike"
canonical_name: "Flying Slash"
snapshot_id: 39952
source_document_id: 7071
payload_hash: "cb8b13aa80949f69b54fb2aa7c55bafc7a25b89e05b806a3d9dda35d287a93da"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.522938+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Flying Slash

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_flying_strike`
- Snapshot ID: `39952`
- Source-Dokument: `7071`
- Kurzinfo: Flying Slash aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 28
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 36.0
  },
  "DescKey": "citadel_ability_flying_strike_desc",
  "HeroKey": "hero_yamato",
  "HeroName": "Yamato",
  "Info1": {
    "Alt": [
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Type": "duration",
        "Value": 2.5
      }
    ],
    "DescKey": "citadel_ability_flying_strike_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "melee",
            "Value": 1.0
          },
          "Type": "melee_damage",
          "Value": 0
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Type": "slow",
          "Value": 50
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "SpiritBonus",
          "Name": "Bonus Spirit",
          "Title": "Upon reaching target:",
          "Type": "tech_damage",
          "Value": 0.0
        },
        {
          "Key": "BuffDuration",
          "Name": "Buff Duration",
          "Title": "Upon reaching target:",
          "Type": "duration",
          "Value": 0
        }
      ]
    },
    "RequiresUpgradeIndex": 1
  },
  "Key": "citadel_ability_flying_strike",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Flying Slash",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.2
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCooldown": -18
    },
    {
      "BuffDuration": 6,
      "DescKey": "citadel_ability_flying_strike_t2_desc",
      "SpiritBonus": 40
    },
    {
      "AbilityCastRange": 15,
      "AbilityCharges": 3,
      "AbilityCooldownBetweenCharge": 3,
      "CanGrappleAllyHeroes": 1,
      "DescKey": "citadel_ability_flying_strike_t3_desc"
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
      "hero_key": "hero_yamato",
      "hero_name": "Yamato",
      "lookup": "flying slash",
      "name": "Flying Slash",
      "type": "ability"
    }
  ]
}
````
