---
title: "Sticky Bomb"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_sticky_bomb"
canonical_name: "Sticky Bomb"
snapshot_id: 39757
source_document_id: 7071
payload_hash: "5e528be3401089906f9918034a0fbadb5cb97c4418b65c9f287a227d3336a0ef"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.144334+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Sticky Bomb

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_sticky_bomb`
- Snapshot ID: `39757`
- Source-Dokument: `7071`
- Kurzinfo: Sticky Bomb aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 6
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 18.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 3.5
  },
  "DescKey": "citadel_ability_sticky_bomb_desc",
  "Duration": {
    "KillCheckWindow": {
      "Name": null,
      "Type": "duration",
      "Value": 10.0
    }
  },
  "HeroKey": "hero_bebop",
  "HeroName": "Bebop",
  "Info1": {
    "Alt": [],
    "DescKey": "citadel_ability_sticky_bomb_desc",
    "Main": {
      "Props": [
        {
          "Key": "FuseTime",
          "Name": "Fuse Time",
          "Type": "duration",
          "Value": 3.5
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.5
          },
          "Title": "On Hit:",
          "Type": "tech_damage",
          "Value": 85
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "DebuffDuration",
          "Name": "Debuff Duration",
          "Title": "On Attach:",
          "Type": "duration",
          "Value": 0
        }
      ]
    }
  },
  "Info3": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "BonusDamagePctPerPlayerHit",
          "Name": "Sticky Bomb Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.0015
          },
          "Title": "On Hero Hit:",
          "Type": "damage",
          "Value": 1.0
        },
        {
          "Key": "BonusDamagePctPerPlayerKilled",
          "Name": "Sticky Bomb Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.01
          },
          "Title": "On Hero Kill:",
          "Type": "damage",
          "Value": 2.5
        }
      ]
    }
  },
  "Key": "citadel_ability_sticky_bomb",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Sticky Bomb",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "OnHitDiminish": {
      "Name": null,
      "Value": 60
    },
    "OnKillDiminish": {
      "Name": null,
      "Value": 7
    },
    "SelfDamagePercent": {
      "Name": null,
      "Value": 20
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 8
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCooldown": -8
    },
    {
      "Damage": 85
    },
    {
      "DescKey": "citadel_ability_sticky_bomb_t3_desc",
      "MovementSpeedBonus": 5,
      "MovementSpeedBonusDuration": 6,
      "StatusResistancePercent": 25
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
      "hero_key": "hero_bebop",
      "hero_name": "Bebop",
      "lookup": "sticky bomb",
      "name": "Sticky Bomb",
      "type": "ability"
    }
  ]
}
````
