---
title: "Air Drop"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_tengu_airlift"
canonical_name: "Air Drop"
snapshot_id: 39906
source_document_id: 7071
payload_hash: "194cc896f8bf075b799f8474fc31b7f52c1cb17bfb148e10cee29e9896c82659"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.433333+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Air Drop

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_tengu_airlift`
- Snapshot ID: `39906`
- Source-Dokument: `7071`
- Kurzinfo: Air Drop aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

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
    "Value": 22
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 100.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 21.0
  },
  "Damage": {
    "AllyOutgoingDamagePercent": {
      "Name": null,
      "Type": "damage",
      "Value": -20
    }
  },
  "DescKey": "citadel_ability_tengu_airlift_desc",
  "Duration": {
    "AllyCastDelay": {
      "Name": null,
      "Type": "duration",
      "Value": 0.1
    }
  },
  "HeroKey": "hero_tengu",
  "HeroName": "Ivy",
  "Info1": {
    "Alt": [
      {
        "Key": "OnLandDamageRadius",
        "Name": "Landing Radius",
        "Type": "distance",
        "Value": 20
      },
      {
        "Key": "BuffDuration",
        "Name": "Buff Duration",
        "Type": "duration",
        "Value": 8
      },
      {
        "Key": "SlowPercent",
        "Name": "Move Speed",
        "Type": "slow",
        "Value": 0
      },
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 0
      },
      {
        "Key": "BulletArmorReduction",
        "Name": "Bullet Resist",
        "Type": "bullet_armor_down",
        "Value": 0
      },
      {
        "Key": "BulletArmorReductionDuration",
        "Name": "Duration",
        "Type": "duration",
        "Value": 0
      },
      {
        "Key": "SilenceDuration",
        "Name": "Silence Duration",
        "Type": "duration",
        "Value": 0
      }
    ],
    "DescKey": "citadel_ability_tengu_airlift_desc",
    "Main": {
      "Props": [
        {
          "Key": "ExplodeDamage",
          "Name": "Explode Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.7
          },
          "Type": "tech_damage",
          "Value": 115.0
        },
        {
          "Key": "AirDropOutgoingDamagePercent",
          "Name": "Outgoing Damage Bonus",
          "Type": "damage",
          "Value": 20
        },
        {
          "Key": "AirDropBulletShield",
          "Name": "Barrier",
          "Scale": {
            "Type": "spirit",
            "Value": 0
          },
          "Type": "combat_barrier",
          "Value": 0
        }
      ]
    }
  },
  "Key": "citadel_ability_tengu_airlift",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Air Drop",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CooldownReductionPctOnOthers": {
      "Name": null,
      "Value": 30
    },
    "InterruptCooldown": {
      "Name": "Interrupt Cooldown",
      "Value": 3.5
    },
    "OnLandDamageRadiusStart": {
      "Name": null,
      "Value": 16
    },
    "SilenceBombSpeed": {
      "Name": null,
      "Value": 12
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AirDropBulletShield": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.7
        },
        "Value": 300
      },
      "DescKey": "citadel_ability_tengu_airlift_t1_desc"
    },
    {
      "DebuffDuration": 3,
      "DescKey": "citadel_ability_tengu_airlift_t2_desc",
      "SlowPercent": 40
    },
    {
      "AirDropBulletShield": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.0
        },
        "Value": 0
      },
      "DescKey": "citadel_ability_tengu_airlift_t3_desc",
      "ExplodeDamage": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.5
        },
        "Value": 0
      },
      "SilenceDuration": 3
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
      "hero_key": "hero_tengu",
      "hero_name": "Ivy",
      "lookup": "air drop",
      "name": "Air Drop",
      "type": "ability"
    }
  ]
}
````
