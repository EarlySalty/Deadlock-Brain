---
title: "Tail Whack"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_werewolf_cripplingslash"
canonical_name: "Tail Whack"
snapshot_id: 39941
source_document_id: 7071
payload_hash: "653b20f8a47cdbdedcbd153e98de19c32b9f40f7ba343eac3c384a76f2189e83"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.501511+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Tail Whack

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_cripplingslash`
- Snapshot ID: `39941`
- Source-Dokument: `7071`
- Kurzinfo: Tail Whack aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.15
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 18
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_werewolf_cripplingslash_desc",
  "Duration": {
    "SlowDuration": {
      "Name": "Slow Duration",
      "Type": "duration",
      "Value": 2.0
    }
  },
  "HeroKey": "hero_werewolf_transformed",
  "HeroName": "Silver (Transformed)",
  "Info1": {
    "Alt": [
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Value": 0
      }
    ],
    "DescKey": "ability_werewolf_cripplingslash_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.5
          },
          "Type": "tech_damage",
          "Value": 45
        },
        {
          "Key": "DisarmDuration",
          "Name": "Disarm Duration",
          "StatusEffect": "Disarmed",
          "Title": "On Hit:",
          "Value": 2.0
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Title": "On Hit:",
          "Type": "slow",
          "Value": 30
        },
        {
          "Key": "FireRateSlow",
          "Name": "Fire Rate",
          "Title": "On Hit:",
          "Type": "fire_rate",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_werewolf_cripplingslash",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Tail Whack",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.2
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "LeftForce": {
      "Name": null,
      "Value": 400
    },
    "PushForce": {
      "Name": null,
      "Value": 300
    },
    "SlashHeight": {
      "Name": null,
      "Value": 3
    },
    "SlashRadius": {
      "Name": "Slash Radius",
      "Value": 10
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "Damage": 25
    },
    {
      "SlowPercent": 40
    },
    {
      "DescKey": "ability_werewolf_cripplingslash_t3_desc",
      "DisarmDuration": 1.5,
      "SlowDuration": 1.5
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
      "hero_key": "hero_werewolf_transformed",
      "hero_name": "Silver (Transformed)",
      "lookup": "tail whack",
      "name": "Tail Whack",
      "type": "ability"
    }
  ]
}
````
