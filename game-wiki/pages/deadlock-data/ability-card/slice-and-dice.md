---
title: "Slice and Dice"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_shiv_dash"
canonical_name: "Slice and Dice"
snapshot_id: 39884
source_document_id: 7071
payload_hash: "67786f6be63a18b5b356f64e3073dfd1c5a6e424aace45d9c29c5951f067c074"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.391759+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Slice and Dice

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_shiv_dash`
- Snapshot ID: `39884`
- Source-Dokument: `7071`
- Kurzinfo: Slice and Dice aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.25
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 16.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_shiv_dash_desc",
  "HeroKey": "hero_shiv",
  "HeroName": "Shiv",
  "Info1": {
    "Alt": [
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 14
      }
    ],
    "DescKey": "citadel_ability_shiv_dash_desc",
    "Main": {
      "Props": [
        {
          "Key": "ImpactDamage",
          "Name": "Impact Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.4415
          },
          "Type": "tech_damage",
          "Value": 75
        },
        {
          "Key": "DashRange",
          "Name": "Time Window",
          "Type": "distance",
          "Value": 12
        },
        {
          "Key": "TechArmorDamageReduction",
          "Name": "Spirit Resist",
          "Type": "tech_armor_down",
          "Value": -6
        }
      ]
    }
  },
  "Info2": {
    "Alt": [
      {
        "Key": "CooldownReductionOnHitNonHero",
        "Name": "Cooldown Reduced on Non-Hero Hit",
        "Type": "cooldown",
        "Value": 0
      }
    ],
    "Main": {
      "Props": [
        {
          "Key": "CooldownReductionOnHit",
          "Name": "Cooldown Reduced on Hero Hit",
          "Title": "On hit",
          "Type": "cooldown",
          "Value": 0
        },
        {
          "Key": "MaxCooldownReductionsFromHits",
          "Name": "Max Cooldown Reduction",
          "Title": "On hit",
          "Value": 0
        }
      ]
    },
    "RequiresUpgradeIndex": 2
  },
  "Info3": {
    "Alt": [],
    "DescKey": "citadel_ability_shiv_dash_ult_desc",
    "Main": {}
  },
  "Key": "citadel_ability_shiv_dash",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Slice and Dice",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.2
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CameraDistance": {
      "Name": null,
      "Value": 250
    },
    "DashAngleThreshold": {
      "Name": null,
      "Value": 89
    },
    "DashRadius": {
      "Name": "Radius",
      "Value": 2.5
    },
    "DashSpeed": {
      "Name": "Dash Speed",
      "Value": 60.96
    },
    "MoveSpeedPenaltyMaxSpeed": {
      "Name": null,
      "Value": 200
    },
    "SideMoveSpeedReduction": {
      "Name": null,
      "Value": -100
    },
    "TechCleaveExpireTime": {
      "Name": null,
      "Value": 0.35
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCooldown": -6
    },
    {
      "DashRange": 2,
      "DescKey": "citadel_ability_shiv_dash_t2_desc",
      "TechArmorDamageReduction": -6
    },
    {
      "CooldownReductionOnHit": 2,
      "CooldownReductionOnHitNonHero": 1,
      "DescKey": "citadel_ability_shiv_dash_t3_desc",
      "ImpactDamage": 50,
      "MaxCooldownReductionsFromHits": 8
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
      "hero_key": "hero_shiv",
      "hero_name": "Shiv",
      "lookup": "slice and dice",
      "name": "Slice and Dice",
      "type": "ability"
    }
  ]
}
````
