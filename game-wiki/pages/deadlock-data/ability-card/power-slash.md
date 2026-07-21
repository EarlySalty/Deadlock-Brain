---
title: "Power Slash"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_power_slash"
canonical_name: "Power Slash"
snapshot_id: 39951
source_document_id: 7071
payload_hash: "2f380b4f9986e0e356e72abecc183134f2dc6d528be3bde61b311e3ba4fd7945"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.520432+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Power Slash

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_power_slash`
- Snapshot ID: `39951`
- Source-Dokument: `7071`
- Kurzinfo: Power Slash aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 1.4
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 12
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "Damage": {
    "MediumChargeDamagePct": {
      "Name": "Medium Charge Dmg",
      "Type": "tech_damage",
      "Value": 50
    },
    "ShortChargeDamagePct": {
      "Name": "Short Charge Dmg",
      "Type": "tech_damage",
      "Value": 30
    }
  },
  "DescKey": "citadel_ability_power_slash_desc",
  "HeroKey": "hero_yamato",
  "HeroName": "Yamato",
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
        "Key": "BulletResist",
        "Name": "Bullet Resist",
        "Type": "bullet_armor_up",
        "Value": 60
      }
    ],
    "DescKey": "citadel_ability_power_slash_desc",
    "Main": {
      "Props": [
        {
          "Key": "FullChargeDamage",
          "Name": "Full Charge Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.85
          },
          "Type": "tech_damage",
          "Value": 145
        },
        {
          "Key": "SlashLength",
          "Name": "Slash Length",
          "Type": "distance",
          "Value": 22
        }
      ]
    }
  },
  "Key": "citadel_ability_power_slash",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Power Slash",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.2
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "FallSpeedMax": {
      "Name": null,
      "Value": 5
    },
    "PowerUpStages": {
      "Name": null,
      "Value": 3
    },
    "SlashCollisionRadius": {
      "Name": null,
      "Value": 4
    },
    "SlashRadius": {
      "Name": "Slash Radius",
      "Type": "",
      "Value": 41
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "DescKey": "citadel_ability_power_slash_t1_desc",
      "SlowDuration": 3,
      "SlowPercent": 40
    },
    {
      "AbilityCooldown": -4,
      "DescKey": "citadel_ability_power_slash_t2_desc"
    },
    {
      "DescKey": "citadel_ability_power_slash_t3_desc",
      "FullChargeDamage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.5
        },
        "Value": 150
      },
      "SlashLength": 8
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
      "lookup": "power slash",
      "name": "Power Slash",
      "type": "ability"
    }
  ]
}
````
