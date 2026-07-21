---
title: "Swish"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_boho_doublehit"
canonical_name: "Swish"
snapshot_id: 39761
source_document_id: 7071
payload_hash: "8348ae4ae062db60d182c0f296f81720aaa50c304a093ac22b404db75ec3ba53"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.153040+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Swish

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_boho_doublehit`
- Snapshot ID: `39761`
- Source-Dokument: `7071`
- Kurzinfo: Swish aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.1
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 11
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 14
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_boho_doublehit_desc",
  "HeroKey": "hero_boho",
  "HeroName": "Boho",
  "Info1": {
    "Alt": [
      {
        "Key": "BuffDuration",
        "Name": "Buff Duration",
        "Type": "move_speed",
        "Value": 4
      },
      {
        "Key": "CombatBarrierPerStack",
        "Scale": {
          "Type": "spirit",
          "Value": 0.2
        },
        "Type": "health",
        "Value": 20
      }
    ],
    "DescKey": "ability_boho_doublehit_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.6
          },
          "Type": "tech_damage",
          "Value": 50
        },
        {
          "Key": "CombatBarrier",
          "Name": "Barrier",
          "Scale": {
            "Type": "spirit",
            "Value": 0.8
          },
          "Title": "On Hero Hit:",
          "Type": "health",
          "Value": 80
        },
        {
          "Key": "BonusMoveSpeed",
          "Name": "Move Speed",
          "Title": "On Hero Hit:",
          "Type": "move_speed",
          "Value": 2
        }
      ]
    }
  },
  "Key": "ability_boho_doublehit",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Swish",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.7
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 16
    },
    "MaxStacks": {
      "Name": "Max Stacks",
      "Value": 6
    },
    "TargetingConeAngle": {
      "Name": "Attack Angle",
      "Value": 100
    },
    "TimeBetweenAttacks": {
      "Name": null,
      "Value": 0.35
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "Damage": 18.0
    },
    {
      "BonusMoveSpeed": 2
    },
    {
      "CombatBarrier": 80
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
      "lookup": "swish",
      "name": "Swish",
      "type": "ability"
    }
  ]
}
````
