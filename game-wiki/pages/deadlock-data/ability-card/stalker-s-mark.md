---
title: "Stalker's Mark"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "drifter_shadow_mark"
canonical_name: "Stalker's Mark"
snapshot_id: 39780
source_document_id: 7071
payload_hash: "4cd9278c21d4a1aa83bebd6fea98ee1c15d3e4c7a4d6a31da059e99889c1c44f"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.190732+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Stalker's Mark

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `drifter_shadow_mark`
- Snapshot ID: `39780`
- Source-Dokument: `7071`
- Kurzinfo: Stalker's Mark aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.15
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 20
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
  "Damage": {
    "TeleportDamage": {
      "Name": "Blink Damage",
      "Scale": {
        "Type": "spirit",
        "Value": 0.5
      },
      "Type": "tech_damage",
      "Value": 0
    }
  },
  "DescKey": "drifter_shadow_mark_desc",
  "HeroKey": "hero_drifter",
  "HeroName": "Drifter",
  "Info1": {
    "Alt": [
      {
        "Key": "AbilityCharges",
        "Name": "Charges",
        "Type": "cast",
        "Value": 0
      },
      {
        "Key": "BulletResistReduction",
        "Name": "Bullet Resist",
        "Type": "bullet_armor_down",
        "Value": 0
      },
      {
        "Key": "AbilityCooldownBetweenCharge",
        "Name": "Charge Delay",
        "Type": "charge_cooldown",
        "Value": -1.0
      }
    ],
    "DescKey": "drifter_shadow_mark_desc",
    "Main": {
      "Props": [
        {
          "Key": "DotHealthPercent",
          "Name": "Bleed Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.015
          },
          "Type": "tech_damage",
          "Value": 2.0
        },
        {
          "Key": "AbilityDuration",
          "Name": "Duration",
          "Type": "duration",
          "Value": 5
        },
        {
          "Key": "HealAmpReceivePenaltyPercent",
          "Name": "Healing Reduction",
          "Value": 0
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "BonusFireRate",
          "Name": "Fire Rate",
          "Title": "After ambushing:",
          "Type": "fire_rate",
          "Value": 0
        }
      ]
    }
  },
  "Key": "drifter_shadow_mark",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Stalker's Mark",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AirDrag": {
      "Name": null,
      "Value": 3
    },
    "FallSpeedMax": {
      "Name": null,
      "Value": 0.3
    },
    "TeleportBackOffsetFromTarget": {
      "Name": null,
      "Value": 135
    },
    "TickRate": {
      "Name": null,
      "Value": 0.5
    },
    "VerticalDrag": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "BulletResistReduction": -8,
      "DescKey": "drifter_shadow_mark_t1_desc"
    },
    {
      "AbilityCooldown": -8,
      "AbilityDuration": 3,
      "DescKey": "drifter_shadow_mark_t2_desc"
    },
    {
      "DescKey": "drifter_shadow_mark_t3_desc",
      "DotHealthPercent": 2.0,
      "HealAmpReceivePenaltyPercent": -40,
      "HealAmpRegenPenaltyPercent": -40
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
      "hero_key": "hero_drifter",
      "hero_name": "Drifter",
      "lookup": "stalker's mark",
      "name": "Stalker's Mark",
      "type": "ability"
    }
  ]
}
````
