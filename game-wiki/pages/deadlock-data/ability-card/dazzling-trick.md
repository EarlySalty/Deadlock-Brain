---
title: "Dazzling Trick"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_unicorn_prismaticguard"
canonical_name: "Dazzling Trick"
snapshot_id: 39912
source_document_id: 7071
payload_hash: "400311946c6188dbc52a372e64d6e48ea0b9b7495a5689def619404663179f6b"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.446110+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Dazzling Trick

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_unicorn_prismaticguard`
- Snapshot ID: `39912`
- Source-Dokument: `7071`
- Kurzinfo: Dazzling Trick aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 32
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_unicorn_prismaticguard_desc",
  "HeroKey": "hero_unicorn",
  "HeroName": "Celeste",
  "Info1": {
    "Alt": [
      {
        "Key": "BuffDuration",
        "Name": "Buff Duration",
        "Type": "duration",
        "Value": 4
      }
    ],
    "DescKey": "ability_unicorn_prismaticguard_desc",
    "Main": {
      "Props": [
        {
          "Key": "CombatBarrier",
          "Name": "Barrier",
          "Scale": {
            "Type": "spirit",
            "Value": 0.8
          },
          "Type": "bullet_armor_up",
          "Value": 100
        },
        {
          "Key": "BonusMoveSpeed",
          "Name": "Move Speed",
          "Type": "move_speed",
          "Value": 0
        }
      ]
    }
  },
  "Info2": {
    "Alt": [
      {
        "Key": "ExplodeRadius",
        "Name": "Explosion Radius",
        "Type": "distance",
        "Value": 14
      }
    ],
    "Main": {
      "Props": [
        {
          "Key": "DebuffDuration",
          "Name": "Debuff Duration",
          "Title": "On Explode",
          "Type": "duration",
          "Value": 1.75
        },
        {
          "Key": "BarrierDamagePercentage",
          "Name": "Barrier Damage",
          "Title": "On Explode",
          "Type": "tech_damage",
          "Value": 50
        }
      ]
    }
  },
  "Key": "ability_unicorn_prismaticguard",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Dazzling Trick",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "MaxLifetime": {
      "Name": "Lifetime",
      "Value": 4
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "BonusMoveSpeed": 3.5
    },
    {
      "CombatBarrier": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.7
        },
        "Value": 80
      },
      "DescKey": "ability_unicorn_prismaticguard_t2_desc"
    },
    {
      "AbilityCooldown": -18,
      "DebuffDuration": 1.5,
      "DescKey": "ability_unicorn_prismaticguard_t3_desc"
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
      "hero_key": "hero_unicorn",
      "hero_name": "Celeste",
      "lookup": "dazzling trick",
      "name": "Dazzling Trick",
      "type": "ability"
    }
  ]
}
````
