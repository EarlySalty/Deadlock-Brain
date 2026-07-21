---
title: "Love Bites"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_vampirebat_lovebites"
canonical_name: "Love Bites"
snapshot_id: 39917
source_document_id: 7071
payload_hash: "56aaa3b0696b8cfecee98388e2c93d63d646c75101ae18f5888dc1a957f7228f"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.455414+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Love Bites

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_vampirebat_lovebites`
- Snapshot ID: `39917`
- Source-Dokument: `7071`
- Kurzinfo: Love Bites aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "Cooldown": {
    "PerTargetCooldown": {
      "Name": null,
      "Type": "cooldown",
      "Value": 10
    }
  },
  "DescKey": "ability_vampirebat_lovebites_desc",
  "HeroKey": "hero_vampirebat",
  "HeroName": "Mina",
  "Info1": {
    "Alt": [
      {
        "Key": "SlowPercent",
        "Name": "Move Speed",
        "Type": "slow",
        "Value": 0
      },
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Type": "duration",
        "Value": 0
      },
      {
        "Key": "BuffDuration",
        "Name": "Buff Duration",
        "Type": "duration",
        "Value": 0
      }
    ],
    "DescKey": "ability_vampirebat_lovebites_desc",
    "Main": {
      "Props": [
        {
          "Key": "MagicDamagePerBullet",
          "Name": "Spirit Damage Per Bullet",
          "Scale": {
            "Type": "spirit",
            "Value": 0.09
          },
          "Type": "tech_damage",
          "Value": 4
        },
        {
          "Key": "BonusDamage",
          "Name": "Bonus Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.85
          },
          "Title": "On Proc:",
          "Type": "tech_damage",
          "Value": 45
        },
        {
          "Key": "BonusFireRate",
          "Name": "Fire Rate",
          "Title": "On Proc:",
          "Type": "fire_rate",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_vampirebat_lovebites",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Love Bites",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BuildUpDuration": {
      "Name": null,
      "Value": 5
    },
    "BuildUpHeadshotBonus": {
      "Name": null,
      "Value": 1.5
    },
    "BuildUpPerBat": {
      "Name": null,
      "Value": 20
    },
    "BuildUpPerDagger": {
      "Name": null,
      "Value": 30
    },
    "BuildUpPerShot": {
      "Name": null,
      "Value": 18.4
    },
    "EffectivenessVolumeScaleMax": {
      "Name": null,
      "Value": 1.0
    },
    "EffectivenessVolumeScaleMin": {
      "Name": null,
      "Value": 0.5
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "DescKey": "ability_vampirebat_lovebites_t1_desc",
      "SlowDuration": 3,
      "SlowPercent": 30
    },
    {
      "BonusDamage": 45,
      "DescKey": "ability_vampirebat_lovebites_t2_desc",
      "MagicDamagePerBullet": 3.0
    },
    {
      "BonusFireRate": 25,
      "BuffDuration": 5,
      "DescKey": "ability_vampirebat_lovebites_t3_desc",
      "PerTargetCooldown": -5
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
      "hero_key": "hero_vampirebat",
      "hero_name": "Mina",
      "lookup": "love bites",
      "name": "Love Bites",
      "type": "ability"
    }
  ]
}
````
