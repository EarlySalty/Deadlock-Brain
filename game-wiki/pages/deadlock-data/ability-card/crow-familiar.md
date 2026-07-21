---
title: "Crow Familiar"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_hornet_sting"
canonical_name: "Crow Familiar"
snapshot_id: 39829
source_document_id: 7071
payload_hash: "ea774a1ab9b94ff86a61f5b89ddc9b600981f5f7d16e8371ff70466b7bae8460"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.283665+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Crow Familiar

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_hornet_sting`
- Snapshot ID: `39829`
- Source-Dokument: `7071`
- Kurzinfo: Crow Familiar aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 32.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_hornet_sting_desc",
  "HeroKey": "hero_hornet",
  "HeroName": "Vindicta",
  "Info1": {
    "Alt": [
      {
        "Key": "HealAmpReceivePenaltyPercent",
        "Name": "Healing Reduction",
        "Value": 0
      },
      {
        "Key": "BulletResistReduction",
        "Name": "Bullet Resist",
        "Type": "bullet_armor_down",
        "Value": -6
      },
      {
        "Key": "TechArmorDamageReduction",
        "Type": "tech_armor_down",
        "Value": -6
      }
    ],
    "DescKey": "citadel_ability_hornet_sting_desc",
    "Main": {
      "Props": [
        {
          "Key": "ImpactDamage",
          "Name": "Impact Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.744
          },
          "Type": "tech_damage",
          "Value": 40
        },
        {
          "Key": "DotHealthPercent",
          "Name": "Bleed Damage",
          "Type": "tech_damage",
          "Value": 2.2
        },
        {
          "Key": "DebuffDuration",
          "Name": "Debuff Duration",
          "Type": "duration",
          "Value": 5
        }
      ]
    }
  },
  "Key": "citadel_ability_hornet_sting",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Crow Familiar",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.2
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "TickRate": {
      "Name": null,
      "Value": 1.0
    },
    "VisualSplashRadius": {
      "Name": null,
      "Value": 4
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "DescKey": "citadel_ability_hornet_sting_t1_desc",
      "HealAmpReceivePenaltyPercent": -35,
      "HealAmpRegenPenaltyPercent": -35
    },
    {
      "AbilityCooldown": -16.0,
      "DescKey": "citadel_ability_hornet_sting_t2_desc",
      "DotHealthPercent": 0.5
    },
    {
      "BulletResistReduction": -8,
      "DebuffDuration": 2,
      "DescKey": "citadel_ability_hornet_sting_t3_desc",
      "TechArmorDamageReduction": -8
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
      "hero_key": "hero_hornet",
      "hero_name": "Vindicta",
      "lookup": "crow familiar",
      "name": "Crow Familiar",
      "type": "ability"
    }
  ]
}
````
