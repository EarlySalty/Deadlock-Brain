---
title: "Assassinate"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_hornet_snipe"
canonical_name: "Assassinate"
snapshot_id: 39830
source_document_id: 7071
payload_hash: "4bd3007b3c023a3843acc266c4ee9560cd6428f646c8490efed0ce34e5df59ce"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.285455+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Assassinate

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_hornet_snipe`
- Snapshot ID: `39830`
- Source-Dokument: `7071`
- Kurzinfo: Assassinate aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 55.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 2.5
  },
  "DescKey": "citadel_ability_hornet_snipe_desc",
  "HeroKey": "hero_hornet",
  "HeroName": "Vindicta",
  "Info1": {
    "Alt": [
      {
        "Key": "TimeToFullCharge",
        "Name": "Full Charge Time",
        "Type": "duration",
        "Value": 1.0
      },
      {
        "Key": "MinChargeDamagePercent",
        "Name": "No Charge Damage",
        "Value": 50
      },
      {
        "Key": "HeadshotBonus",
        "Name": "Headshot Damage",
        "Type": "bullet_damage",
        "Value": 20
      }
    ],
    "DescKey": "citadel_ability_hornet_snipe_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.93
          },
          "Type": "tech_damage",
          "Value": 90
        },
        {
          "Key": "LowHealthEnemyDamageBonus",
          "Name": "Max Bonus Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 2.3
          },
          "Type": "tech_damage",
          "Value": 90
        },
        {
          "Key": "WeaponDamageBonusPerKill",
          "Name": "Weapon Damage Per Kill",
          "Type": "bullet_damage",
          "Value": 6
        }
      ]
    }
  },
  "Key": "citadel_ability_hornet_snipe",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Assassinate",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BonusGoldOnKill": {
      "Name": "Bonus Souls Per Assassination",
      "Value": 250
    },
    "LowHealthEnemyThresholdPct": {
      "Name": "Low Health Threshold",
      "Value": 50
    },
    "MaxSoundDistance": {
      "Name": null,
      "Value": 2000
    },
    "MoveSpeed": {
      "Name": null,
      "Value": 4
    },
    "ShotRadius": {
      "Name": null,
      "Value": 4.0
    },
    "ViewPunch": {
      "Name": null,
      "Value": 2.5
    }
  },
  "Range": {
    "Range": {
      "Name": "Range",
      "Type": "distance",
      "Value": 1000
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCooldown": -15.0
    },
    {
      "LowHealthEnemyDamageBonus": 80
    },
    {
      "WeaponDamageBonusPerKill": 4
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
      "hero_key": "hero_hornet",
      "hero_name": "Vindicta",
      "lookup": "assassinate",
      "name": "Assassinate",
      "type": "ability"
    }
  ]
}
````
