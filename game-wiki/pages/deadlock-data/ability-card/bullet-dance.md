---
title: "Bullet Dance"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_bullet_flurry"
canonical_name: "Bullet Dance"
snapshot_id: 39826
source_document_id: 7071
payload_hash: "df50e1d97557798befc28964c734aca7d87d187e66ffa0b8e46cd15c127b46c4"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.278435+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Bullet Dance

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_bullet_flurry`
- Snapshot ID: `39826`
- Source-Dokument: `7071`
- Kurzinfo: Bullet Dance aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.4
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 165.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Scale": {
      "Type": "spirit",
      "Value": 0.03
    },
    "Type": "duration",
    "Value": 3.5
  },
  "DescKey": "ability_bullet_flurry_desc",
  "HeroKey": "hero_haze",
  "HeroName": "Haze",
  "Info1": {
    "Alt": [
      {
        "Key": "EvasionPercent",
        "Name": "Bullet Evasion",
        "Value": 30
      },
      {
        "Key": "ChannelMoveSpeed",
        "Name": "Channel Move Speed",
        "Type": "move_speed",
        "Value": 4
      }
    ],
    "DescKey": "ability_bullet_flurry_desc",
    "Main": {
      "Props": [
        {
          "Key": "BonusFireRate",
          "Name": "Fire Rate",
          "Type": "fire_rate",
          "Value": 25
        },
        {
          "Key": "WeaponDamageBonus",
          "Name": "Weapon Damage",
          "Type": "bullet_damage",
          "Value": 7
        },
        {
          "Key": "TargetsPerTick",
          "Name": "Targets Hit Per Shot",
          "Type": "radius",
          "Value": 1
        }
      ]
    }
  },
  "Key": "ability_bullet_flurry",
  "Name": "Bullet Dance",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "OverrideBulletRadius": {
      "Name": null,
      "Value": 10
    },
    "ProcChance": {
      "Name": "Proc Chance",
      "Value": 100
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 16
  },
  "Range": {
    "RadiusMin": {
      "Name": null,
      "Type": "distance",
      "Value": 0.75
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "WeaponDamageBonus": 7
    },
    {
      "BonusFireRate": 10,
      "ChannelMoveSpeed": 3,
      "DescKey": "ability_bullet_flurry_t2_desc"
    },
    {
      "AbilityCooldown": -65,
      "DescKey": "ability_bullet_flurry_t3_desc",
      "EvasionPercent": 40.0
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
      "hero_key": "hero_haze",
      "hero_name": "Haze",
      "lookup": "bullet dance",
      "name": "Bullet Dance",
      "type": "ability"
    }
  ]
}
````
