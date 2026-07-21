---
title: "Shadow Transformation"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_infinity_slash"
canonical_name: "Shadow Transformation"
snapshot_id: 39954
source_document_id: 7071
payload_hash: "4b1a275560e9aeb09cca3964585858cbf3bea8b28f0646c226e5c2bfe23d809a"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.526857+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Shadow Transformation

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_infinity_slash`
- Snapshot ID: `39954`
- Source-Dokument: `7071`
- Kurzinfo: Shadow Transformation aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "duration",
    "Value": 1.5
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 150.0
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
  "DescKey": "citadel_ability_infinity_slash_desc",
  "HeroKey": "hero_yamato",
  "HeroName": "Yamato",
  "Info1": {
    "Alt": [
      {
        "Key": "BulletResist",
        "Name": "Bullet Resist",
        "Type": "bullet_armor_up",
        "Value": 30
      },
      {
        "Key": "TechResist",
        "Name": "Spirit Resist",
        "Type": "tech_armor_up",
        "Value": 30
      },
      {
        "Key": "ShadowFormDurationOnKill",
        "Name": "Duration On Kill",
        "Type": "duration",
        "Value": 2.0
      },
      {
        "Key": "MaxHealthHealOnCast",
        "Type": "healing",
        "Value": 0
      }
    ],
    "DescKey": "citadel_ability_infinity_slash_desc",
    "Main": {
      "Props": [
        {
          "Key": "AbilityDuration",
          "Name": "Duration",
          "Type": "duration",
          "Value": 5
        },
        {
          "Key": "AbilitySpeedPct",
          "Name": "Ability Speed",
          "Type": "cast",
          "Value": 60
        },
        {
          "Key": "MaxHealthRegen",
          "Name": "Max Health Heal",
          "Type": "healing",
          "Value": 15
        }
      ]
    }
  },
  "Key": "citadel_ability_infinity_slash",
  "Name": "Shadow Transformation",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "WeaponDamageBonus": 7
    },
    {
      "AbilityCooldown": -20,
      "BonusMoveSpeed": 4,
      "DescKey": "citadel_ability_infinity_slash_t2_desc"
    },
    {
      "AbilityDuration": 3.0,
      "BulletResist": 30,
      "DescKey": "citadel_ability_infinity_slash_t3_desc",
      "TechResist": 30
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
      "lookup": "shadow transformation",
      "name": "Shadow Transformation",
      "type": "ability"
    }
  ]
}
````
