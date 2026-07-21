---
title: "Ava"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_nano_catform"
canonical_name: "Ava"
snapshot_id: 39857
source_document_id: 7071
payload_hash: "e0c620a4ea3e44aeedd8b0f79fda60541715d3441d6b6c1172caae7ccb20872f"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.340242+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Ava

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_nano_catform`
- Snapshot ID: `39857`
- Source-Dokument: `7071`
- Kurzinfo: Ava aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 30
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_nano_catform_desc",
  "HeroKey": "hero_nano",
  "HeroName": "Calico",
  "Info1": {
    "Alt": [
      {
        "Key": "SpeedBuildDuration",
        "Type": "duration",
        "Value": 4
      },
      {
        "Key": "InterruptCooldown",
        "Name": "Interrupt Cooldown",
        "Type": "duration",
        "Value": 6
      },
      {
        "Key": "HealthRegen",
        "Name": "Health Regen",
        "Type": "healing",
        "Value": 0
      }
    ],
    "DescKey": "ability_nano_catform_desc",
    "Main": {
      "Props": [
        {
          "Key": "MinBonusMoveSpeedPercent",
          "Name": "Min Move Speed",
          "Type": "move_speed",
          "Value": 30
        },
        {
          "Key": "MaxBonusMoveSpeedPercent",
          "Name": "Max Move Speed",
          "Type": "move_speed",
          "Value": 65
        },
        {
          "Key": "BuffDuration",
          "Name": "Buff Duration",
          "Type": "duration",
          "Value": 15
        }
      ]
    }
  },
  "Key": "ability_nano_catform",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Ava",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CatFormDamageDealtReduction": {
      "Name": null,
      "Value": -100
    },
    "EnemyDamageSpeedPenalty": {
      "Name": null,
      "Value": 65
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "BuffDuration": 15
    },
    {
      "DescKey": "ability_nano_catform_t2_desc",
      "HealthRegen": 15,
      "MaxBonusMoveSpeedPercent": 45
    },
    {
      "DamageAmpBuildDuration": 10,
      "DamageAmpDuration": 6,
      "DescKey": "ability_nano_catform_t3_desc",
      "OutgoingDamagePercent": 20
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
      "hero_key": "hero_nano",
      "hero_name": "Calico",
      "lookup": "ava",
      "name": "Ava",
      "type": "ability"
    }
  ]
}
````
