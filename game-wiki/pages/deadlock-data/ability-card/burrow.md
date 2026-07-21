---
title: "Burrow"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_burrow"
canonical_name: "Burrow"
snapshot_id: 39840
source_document_id: 7071
payload_hash: "8a4daf837a851189a7ae673f913643c1ca95a09bb60d2f8f53f504ad941b1444"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.305770+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Burrow

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_burrow`
- Snapshot ID: `39840`
- Source-Dokument: `7071`
- Kurzinfo: Burrow aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 1
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 5
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 40.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_burrow_desc",
  "Duration": {
    "SpeedLostDuration": {
      "Name": null,
      "Type": "duration",
      "Value": 1
    },
    "SpinSlowDuration": {
      "Name": null,
      "Type": "duration",
      "Value": 0.3
    }
  },
  "HeroKey": "hero_krill",
  "HeroName": "Mo & Krill",
  "Info1": {
    "Alt": [
      {
        "Key": "BulletResist",
        "Name": "Bullet Resist",
        "Type": "bullet_armor_up",
        "Value": 60
      },
      {
        "Key": "TechResist",
        "Name": "Spirit Resist",
        "Type": "tech_armor_up",
        "Value": 30
      }
    ],
    "DescKey": "ability_burrow_desc",
    "Main": {
      "Props": [
        {
          "Key": "BonusMoveSpeed",
          "Name": "Move Speed",
          "Type": "move_speed",
          "Value": 5
        },
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 1.488
          },
          "Type": "tech_damage",
          "Value": 75
        },
        {
          "Key": "SpinDuration",
          "Name": "Spin Duration",
          "Type": "duration",
          "Value": 1.5
        },
        {
          "Key": "TossDuration",
          "Name": "Duration",
          "StatusEffect": "Displacement",
          "Value": 1
        }
      ]
    }
  },
  "Key": "ability_burrow",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Burrow",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "EnemyDamageSpeedPenalty": {
      "Name": null,
      "Value": 0.5
    },
    "SpinSlowPercent": {
      "Name": null,
      "Value": 10
    },
    "TickRate": {
      "Name": null,
      "Value": 0.1
    },
    "UpForce": {
      "Name": null,
      "Value": 250
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 5
  },
  "Slot": "2",
  "Upgrades": [
    {
      "DPS": 50,
      "DescKey": "ability_burrow_t1_desc"
    },
    {
      "AbilityChannelTime": 4,
      "DescKey": "ability_burrow_t2_desc",
      "Radius": 2
    },
    {
      "AbilityCooldown": -20.0,
      "BonusMoveSpeed": 4,
      "DescKey": "ability_burrow_t3_desc"
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
      "hero_key": "hero_krill",
      "hero_name": "Mo & Krill",
      "lookup": "burrow",
      "name": "Burrow",
      "type": "ability"
    }
  ]
}
````
