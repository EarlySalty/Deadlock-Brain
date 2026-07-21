---
title: "Stake"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_hornet_chain"
canonical_name: "Stake"
snapshot_id: 39827
source_document_id: 7071
payload_hash: "dc64e4020936224245e2de5b5cb0ef55bf2501fe8e51e924eb58881329c70b55"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.280246+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Stake

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_hornet_chain`
- Snapshot ID: `39827`
- Source-Dokument: `7071`
- Kurzinfo: Stake aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

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
    "Value": 40.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_hornet_chain_desc",
  "HeroKey": "hero_hornet",
  "HeroName": "Vindicta",
  "Info1": {
    "Alt": [
      {
        "Key": "ChainLength",
        "Name": "Tether Length",
        "Type": "distance",
        "Value": 9
      },
      {
        "Key": "SlowPercent",
        "Name": "Move Speed",
        "Type": "slow",
        "Value": 40
      }
    ],
    "DescKey": "citadel_ability_hornet_chain_desc",
    "Main": {
      "Props": [
        {
          "Key": "ChainDuration",
          "Name": "Tether Duration",
          "Type": "duration",
          "Value": 1.75
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.5
          },
          "Type": "tech_damage",
          "Value": 40
        },
        {
          "Key": "CaptureRadius",
          "Name": "Capture Radius",
          "Type": "distance",
          "Value": 9
        }
      ]
    }
  },
  "Key": "citadel_ability_hornet_chain",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Stake",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "EnemyDragSpeed": {
      "Name": null,
      "Value": 25.4
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "Damage": 65
    },
    {
      "AbilityCooldown": -22.0
    },
    {
      "CaptureRadius": 2,
      "ChainDuration": 0.75,
      "DescKey": "citadel_ability_hornet_chain_t3_desc"
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
      "lookup": "stake",
      "name": "Stake",
      "type": "ability"
    }
  ]
}
````
