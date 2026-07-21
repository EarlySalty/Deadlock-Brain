---
title: "Crimson Slash"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_healing_slash"
canonical_name: "Crimson Slash"
snapshot_id: 39953
source_document_id: 7071
payload_hash: "9abf96cb3b87752760d29c0294744ceb44995522caeaa936b93a7a616090968e"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.525050+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Crimson Slash

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_healing_slash`
- Snapshot ID: `39953`
- Source-Dokument: `7071`
- Kurzinfo: Crimson Slash aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.3
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 16
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_healing_slash_desc",
  "Duration": {
    "BuffDuration": {
      "Name": "Buff Duration",
      "Scale": {
        "Type": "duration",
        "Value": 1.0
      },
      "Type": "duration",
      "Value": 0
    }
  },
  "HeroKey": "hero_yamato",
  "HeroName": "Yamato",
  "Info1": {
    "Alt": [
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 4
      }
    ],
    "DescKey": "citadel_ability_healing_slash_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.37
          },
          "Type": "tech_damage",
          "Value": 55
        },
        {
          "Key": "HealFixedHealth",
          "Name": "Heal on hero hit",
          "Scale": {
            "Type": "spirit",
            "Value": 1.035871
          },
          "Type": "healing",
          "Value": 55
        },
        {
          "Key": "FireRateSlow",
          "Name": "Fire Rate",
          "Type": "bullet_damage",
          "Value": 30
        }
      ]
    }
  },
  "Key": "citadel_ability_healing_slash",
  "Name": "Crimson Slash",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.4
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 13
  },
  "Slot": "3",
  "Upgrades": [
    {
      "BuffDuration": 4,
      "BuffMeleeDamage": 30,
      "DescKey": "citadel_ability_healing_slash_t1_desc"
    },
    {
      "DescKey": "citadel_ability_healing_slash_t2_desc",
      "HealMaxHealth": 6
    },
    {
      "AbilityCooldown": -10,
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.6
        },
        "Value": 0
      },
      "DescKey": "citadel_ability_healing_slash_t3_desc"
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
      "lookup": "crimson slash",
      "name": "Crimson Slash",
      "type": "ability"
    }
  ]
}
````
