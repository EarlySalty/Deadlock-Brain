---
title: "Kinetic Carbine"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_chrono_kinetic_carbine"
canonical_name: "Kinetic Carbine"
snapshot_id: 39773
source_document_id: 7071
payload_hash: "efd92e90b6a78f4094d0861307b2065a312703e2b1adf6690bfe60396eaa3a5f"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.176769+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Kinetic Carbine

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_chrono_kinetic_carbine`
- Snapshot ID: `39773`
- Source-Dokument: `7071`
- Kurzinfo: Kinetic Carbine aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 28.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_chrono_kinetic_carbine_desc",
  "Duration": {
    "MaxChargeDuration": {
      "Name": "Full Charge Time",
      "Type": "duration",
      "Value": 2.5
    },
    "MinSlowDuration": {
      "Name": "Min Time-Stop",
      "Type": "duration",
      "Value": 0.25
    }
  },
  "HeroKey": "hero_chrono",
  "HeroName": "Paradox",
  "Info1": {
    "Alt": [
      {
        "Key": "MinBonusBulletDamage",
        "Name": "Min Damage",
        "Scale": {
          "Type": "weapon_power",
          "Value": 25
        },
        "Type": "tech_damage",
        "Value": 5
      },
      {
        "Key": "SpeedBoostDuration",
        "Name": "Charge Hold Duration",
        "Type": "duration",
        "Value": 3.5
      },
      {
        "Key": "HeadshotBonus",
        "Name": "Headshot Damage",
        "Type": "tech_damage",
        "Value": 14
      }
    ],
    "DescKey": "citadel_ability_chrono_kinetic_carbine_desc",
    "Main": {
      "Props": [
        {
          "Key": "MaxBonusBulletDamage",
          "Name": "Max Damage",
          "Scale": {
            "Type": "weapon_power",
            "Value": 125
          },
          "Type": "tech_damage",
          "Value": 5
        },
        {
          "Key": "MaxSlowDuration",
          "Name": "Max Time-Stop",
          "Type": "duration",
          "Value": 0.4
        },
        {
          "Key": "SpeedChange",
          "Name": "Bonus Speed",
          "Scale": {
            "Type": "spirit",
            "Value": 0.13
          },
          "Type": "move_speed",
          "Value": 25
        }
      ]
    }
  },
  "Key": "citadel_ability_chrono_kinetic_carbine",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Kinetic Carbine",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AirMoveIncreasePercent": {
      "Name": "Air Jump/Dash Distance",
      "Value": 20
    },
    "BonusBulletSpeed": {
      "Name": null,
      "Value": 100
    },
    "BulletRadiusOverride": {
      "Name": null,
      "Value": 16.0
    },
    "BulletTimeScale": {
      "Name": null,
      "Value": 0.01
    },
    "MoveSpeedWhileShootingPenaltyReduction": {
      "Name": null,
      "Value": 100
    },
    "ProjectileTimeScale": {
      "Name": null,
      "Value": 0.01
    },
    "ShotCount": {
      "Name": null,
      "Value": 1
    },
    "TimeScaleDebuff": {
      "Name": null,
      "Value": 90
    },
    "TimeWarpRadius": {
      "Name": null,
      "Value": 5
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "MaxSlowDuration": 0.4
    },
    {
      "AbilityCooldown": -12,
      "DescKey": "citadel_ability_chrono_kinetic_carbine_t2_desc",
      "SpeedChange": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.06
        },
        "Value": 0
      }
    },
    {
      "DescKey": "citadel_ability_chrono_kinetic_carbine_t3_desc",
      "MaxBonusBulletDamage": {
        "Scale": {
          "Type": "weapon_power",
          "Value": 55
        },
        "Value": 0
      },
      "MinBonusBulletDamage": {
        "Scale": {
          "Type": "weapon_power",
          "Value": 55
        },
        "Value": 0
      },
      "SpeedBoostDuration": 2
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
      "lookup": "kinetic carbine",
      "name": "Kinetic Carbine",
      "type": "ability"
    }
  ]
}
````
