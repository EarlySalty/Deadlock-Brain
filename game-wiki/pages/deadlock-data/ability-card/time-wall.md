---
title: "Time Wall"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_chrono_time_wall"
canonical_name: "Time Wall"
snapshot_id: 39772
source_document_id: 7071
payload_hash: "5740d073a735ce6aad71e444ef5c87d5ff0c864b342189ef182134f128ab9bb6"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.175008+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Time Wall

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_chrono_time_wall`
- Snapshot ID: `39772`
- Source-Dokument: `7071`
- Kurzinfo: Time Wall aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 5.08
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 25.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 5.5
  },
  "DescKey": "citadel_ability_chrono_time_wall_desc",
  "HeroKey": "hero_chrono",
  "HeroName": "Paradox",
  "Info1": {
    "Alt": [
      {
        "Key": "TimeWallWidth",
        "Name": "Wall Width",
        "Type": "range",
        "Value": 8
      },
      {
        "Key": "TimeWallHeight",
        "Name": "Wall Height",
        "Value": 4
      },
      {
        "Key": "TimeWallTimeScaleFriendly",
        "Name": "Ally Bullet Speed",
        "Value": 2
      }
    ],
    "DescKey": "citadel_ability_chrono_time_wall_desc",
    "Main": {
      "Props": [
        {
          "Key": "DebuffDuration",
          "Name": "Debuff Duration",
          "Value": 0
        },
        {
          "Key": "MovementSlowPct",
          "Name": "Movement Slow",
          "Type": "slow",
          "Value": 80
        },
        {
          "Key": "TimeScaleDuration",
          "Name": "Time Stop Duration",
          "Type": "duration",
          "Value": 0.5
        },
        {
          "Key": "FriendlyBulletDamageBonus",
          "Name": "Ally Weapon Damage",
          "Type": "bullet_damage",
          "Value": 30
        }
      ]
    }
  },
  "Key": "citadel_ability_chrono_time_wall",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Time Wall",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AuraEffectDuration": {
      "Name": null,
      "Value": 2
    },
    "TimeWallDepth": {
      "Name": null,
      "Value": 0.5
    },
    "TimeWallDepthVisualScale": {
      "Name": null,
      "Value": 0.16
    },
    "TimeWallFormationTime": {
      "Name": null,
      "Value": 0.5
    },
    "TimeWallTimeScale": {
      "Name": null,
      "Value": 0.0001
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityDuration": 3.5,
      "DescKey": "citadel_ability_chrono_time_wall_t1_desc",
      "TimeWallHeight": 1,
      "TimeWallWidth": 3
    },
    {
      "DebuffDuration": 2.3,
      "DescKey": "citadel_ability_chrono_time_wall_t2_desc",
      "FriendlyBulletDamageBonus": 35
    },
    {
      "AbilityCharges": 3,
      "AbilityCooldownBetweenCharge": 2,
      "DescKey": "citadel_ability_chrono_time_wall_t3_desc"
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
      "hero_key": "hero_chrono",
      "hero_name": "Paradox",
      "lookup": "time wall",
      "name": "Time Wall",
      "type": "ability"
    }
  ]
}
````
