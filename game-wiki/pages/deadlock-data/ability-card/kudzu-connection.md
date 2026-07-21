---
title: "Kudzu Connection"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_tangotether"
canonical_name: "Kudzu Connection"
snapshot_id: 39904
source_document_id: 7071
payload_hash: "68b655a0b7e1787a9f9b27584a75e12bceda2fcef81304dc2e5f717f6edeb6ad"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.429643+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Kudzu Connection

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_tangotether`
- Snapshot ID: `39904`
- Source-Dokument: `7071`
- Kurzinfo: Kudzu Connection aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 16
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 37.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 12
  },
  "DescKey": "citadel_ability_tangotether_desc",
  "HeroKey": "hero_tengu",
  "HeroName": "Ivy",
  "Info1": {
    "Alt": [
      {
        "Key": "MoveSpeedBonus",
        "Name": "Move Speed",
        "Type": "move_speed",
        "Value": 0
      },
      {
        "Key": "TotalTetherTargets",
        "Name": "Tether Count",
        "Value": 1
      }
    ],
    "DescKey": "citadel_ability_tangotether_desc",
    "Main": {
      "Props": [
        {
          "Key": "BonusFireRate",
          "Name": "Fire Rate",
          "Scale": {
            "Type": "spirit",
            "Value": 0.18
          },
          "Type": "fire_rate",
          "Value": 10
        },
        {
          "Key": "BulletLifestealPercent",
          "Name": "Bullet Lifesteal",
          "Scale": {
            "Type": "spirit",
            "Value": 0.15
          },
          "Type": "healing",
          "Value": 15
        },
        {
          "Key": "TetherSharedHealPct",
          "Name": "Replicated Healing",
          "Scale": {
            "Type": "power_increase",
            "Value": 0.85
          },
          "Type": "healing",
          "Value": 35
        }
      ]
    }
  },
  "Key": "citadel_ability_tangotether",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    },
    "MoveWhileShootingSpeedPenaltyReductionPercent": {
      "Name": "move speed penalty while shooting reduction",
      "Type": "move_speed",
      "Value": 100
    },
    "MoveWhileZoomedSpeedPenaltyReductionPercent": {
      "Name": null,
      "Type": "move_speed",
      "Value": 100
    }
  },
  "Name": "Kudzu Connection",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "HealingPerGlub": {
      "Name": null,
      "Value": 20
    },
    "TickRate": {
      "Name": null,
      "Value": 0.1
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "MoveSpeedBonus": 2
    },
    {
      "BonusFireRate": 8,
      "BulletLifestealPercent": 8,
      "DescKey": "citadel_ability_tangotether_t2_desc"
    },
    {
      "AbilityCooldown": -37,
      "AbilityDuration": -13,
      "DescKey": "citadel_ability_tangotether_t3_desc"
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
      "lookup": "kudzu connection",
      "name": "Kudzu Connection",
      "type": "ability"
    }
  ]
}
````
