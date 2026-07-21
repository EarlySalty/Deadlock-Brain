---
title: "Frozen Shelter"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_ice_dome"
canonical_name: "Frozen Shelter"
snapshot_id: 39838
source_document_id: 7071
payload_hash: "9aa6c83f1af6a45375d9fac361345e82f6de9c4d1eca49b8787106ffbf17f96b"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.301188+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Frozen Shelter

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_ice_dome`
- Snapshot ID: `39838`
- Source-Dokument: `7071`
- Kurzinfo: Frozen Shelter aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

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
    "Value": 8
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 195
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
  "DescKey": "ability_ice_dome_desc",
  "HeroKey": "hero_kelvin",
  "HeroName": "Kelvin",
  "Info1": {
    "Alt": [
      {
        "Key": "SlowPercent",
        "Name": "Move Speed",
        "Type": "slow",
        "Value": 35
      },
      {
        "Key": "MaxHealthRegen",
        "Name": "Max Health Heal",
        "Type": "healing",
        "Value": 0
      }
    ],
    "DescKey": "ability_ice_dome_desc",
    "Main": {
      "Props": [
        {
          "Key": "BonusHealthRegen",
          "Name": "Health Regen",
          "Scale": {
            "Type": "spirit",
            "Value": 0.0
          },
          "Type": "healing",
          "Value": 90
        },
        {
          "Key": "FireRateSlow",
          "Name": "Fire Rate",
          "Type": "fire_rate",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_ice_dome",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Frozen Shelter",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BlockerScaleFactor": {
      "Name": null,
      "Value": 115
    },
    "EnemyDragSpeed": {
      "Name": null,
      "Value": 25.4
    },
    "GrowTime": {
      "Name": null,
      "Value": 0.2
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 10
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCooldown": -20
    },
    {
      "AbilityDuration": 1.5
    },
    {
      "BonusHealthRegen": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.0
        },
        "Value": 65
      },
      "DescKey": "ability_ice_dome_t3_desc",
      "PurgeOnCast": 1
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
      "hero_key": "hero_kelvin",
      "hero_name": "Kelvin",
      "lookup": "frozen shelter",
      "name": "Frozen Shelter",
      "type": "ability"
    }
  ]
}
````
