---
title: "Rain of Arrows"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_power_jump"
canonical_name: "Rain of Arrows"
snapshot_id: 39868
source_document_id: 7071
payload_hash: "e440082d4920cec2baa78a43afeb3cc3d891482f4162ada45b392ba9e7f6eb6b"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.361280+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Rain of Arrows

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_power_jump`
- Snapshot ID: `39868`
- Source-Dokument: `7071`
- Kurzinfo: Rain of Arrows aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 25.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 4
  },
  "DescKey": "ability_power_jump_desc",
  "HeroKey": "hero_orion",
  "HeroName": "Grey Talon",
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
        "Key": "BulletLifestealPercent",
        "Name": "Bullet Lifesteal",
        "Type": "healing",
        "Value": 0
      },
      {
        "Key": "TechLifestealPercent",
        "Name": "Spirit Lifesteal",
        "Type": "healing",
        "Value": 0
      },
      {
        "Key": "EvasionPercent",
        "Name": "Bullet Evasion",
        "Value": 0
      }
    ],
    "DescKey": "ability_power_jump_desc",
    "Main": {
      "Props": [
        {
          "Key": "BulletSplitShot",
          "Name": "Weapon Multishot",
          "Type": "fire_rate",
          "Value": 5
        },
        {
          "Key": "WeaponDamageBonus",
          "Name": "Weapon Damage",
          "Type": "bullet_damage",
          "Value": 3
        }
      ]
    }
  },
  "Key": "ability_power_jump",
  "Name": "Rain of Arrows",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AirMoveIncreasePercent": {
      "Name": "Air Jump/Dash Distance",
      "Value": 25
    },
    "AirSpeedMax": {
      "Name": null,
      "Value": 6.64
    },
    "AltJumpSpeed": {
      "Name": null,
      "Value": 12
    },
    "FallSpeedMax": {
      "Name": null,
      "Value": 0.635
    },
    "FxRadius": {
      "Name": null,
      "Value": 4
    },
    "JumpPitch": {
      "Name": null,
      "Value": -60
    },
    "JumpSpeed": {
      "Name": null,
      "Value": 27.5
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "DescKey": "ability_power_jump_t1_desc",
      "SlowDuration": 1.5,
      "SlowPercent": 30,
      "WeaponDamageBonus": 3
    },
    {
      "AbilityCooldown": -12.0
    },
    {
      "BulletLifestealPercent": 30,
      "DescKey": "ability_power_jump_t3_desc",
      "EvasionPercent": 30,
      "TechLifestealPercent": 30
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
      "hero_key": "hero_orion",
      "hero_name": "Grey Talon",
      "lookup": "rain of arrows",
      "name": "Rain of Arrows",
      "type": "ability"
    }
  ]
}
````
