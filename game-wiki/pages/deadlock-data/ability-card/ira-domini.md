---
title: "Ira Domini"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_priest_weaponswap"
canonical_name: "Ira Domini"
snapshot_id: 39874
source_document_id: 7071
payload_hash: "f05cc4adbae10b24691410ab67f86158581cbd1bef79a724ff8665c933c48aa0"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.372611+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Ira Domini

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_priest_weaponswap`
- Snapshot ID: `39874`
- Source-Dokument: `7071`
- Kurzinfo: Ira Domini aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.3
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 160
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 15
  },
  "DescKey": "ability_priest_weaponswap_desc",
  "HeroKey": "hero_priest",
  "HeroName": "Venator",
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
        "Value": 0
      }
    ],
    "DescKey": "ability_priest_weaponswap_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "weapon_damage_increase",
            "Value": 1.5
          },
          "Type": "bullet_damage",
          "Value": 120
        },
        {
          "Key": "BonusDamage",
          "Name": "Bonus Damage",
          "Scale": {
            "Type": "power_increase",
            "Value": 3.0
          },
          "Title": "While Blessed:",
          "Type": "tech_damage",
          "Value": 100
        },
        {
          "Key": "ExecuteThreshold",
          "Name": "Execute Threshold",
          "Title": "While Blessed:",
          "Type": "damage",
          "Value": 8
        }
      ]
    }
  },
  "Key": "ability_priest_weaponswap",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Ira Domini",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BonusAmpToVampire": {
      "Name": null,
      "Value": 5
    },
    "ExplodeRadius": {
      "Name": "Explosion Radius",
      "Value": 0.2
    },
    "PushForce": {
      "Name": null,
      "Value": 500
    },
    "StakeCount": {
      "Name": "Total Stakes",
      "Value": 3
    },
    "SwapEndDelay": {
      "Name": null,
      "Value": 0.6
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "BonusMoveSpeed": 1.2,
      "DescKey": "ability_priest_weaponswap_t1_desc"
    },
    {
      "AbilityCooldown": -15,
      "BonusDamage": 65,
      "DescKey": "ability_priest_weaponswap_t2_desc"
    },
    {
      "AllStakesBlessed": 1,
      "DescKey": "ability_priest_weaponswap_t3_desc"
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
      "hero_key": "hero_priest",
      "hero_name": "Venator",
      "lookup": "ira domini",
      "name": "Ira Domini",
      "type": "ability"
    }
  ]
}
````
